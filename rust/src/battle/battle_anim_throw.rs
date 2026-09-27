//! Translated from `src/battle_anim_throw.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sCaptureStars sBallParticleSpriteSheets sBallParticlePalettes sAnim_RegularBall sAnim_MasterBall sAnim_NetDiveBall sAnim_NestBall sAnim_LuxuryPremierBall sAnim_UltraRepeatTimerBall sAnims_BallParticles sBallParticleAnimNums sBallParticleAnimationFuncs sBallParticleSpriteTemplates gBallOpenFadeColors gPokeblockSpriteTemplate sAnim_SafariRock sAnims_SafariRock sSafariRockSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_throw::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMonShrinkDuration: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMonShrinkDelta: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMonShrinkDistance: u16 = 0u16;

unsafe extern "C" {
    static mut UnusedLevelupAnimationGfx: u8;
    static mut UnusedLevelupAnimationTilemap: u8;
    static mut gBallSpriteTemplates: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimPaletteTable: u8;
    static mut gBattleAnimPicTable: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattleSpritesDataPtr: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_WIN0H: u8;
    static mut gBattle_WIN0V: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gCureBubblesPal: u8;
    static mut gDoingBattleAnim: u8;
    static mut gEffectBattler: u8;
    static mut gEnemyParty: u8;
    static mut gHealthboxSpriteIds: u8;
    static mut gLastUsedItem: u8;
    static mut gMain: u8;
    static mut gMiniTwinklingStarSpriteTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gWishStarSpriteTemplate: u8;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn AnimLoadCompressedBgGfx(a0: u32, a1: *mut u32, a2: u32);
    fn AnimLoadCompressedBgTilemap(a0: u32, a1: *mut u8);
    fn AnimateSprite(a0: *mut u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn ChangeSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn ClearBehindSubstituteBit(a0: u8);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut u8)>) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndFreeResources(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FreeBallGfx(a0: u8);
    fn FreeOamMatrix(a0: u8);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut u8);
    fn GetBattlePalettesMask(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u32;
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetGpuReg(a0: u8) -> u16;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitAnimArcTranslation(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn LoadBallGfx(a0: u8);
    fn LoadBattleMonGfxAndAnimate(a0: u8, a1: u8, a2: u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePaletteUsingHeap(a0: *mut u8) -> u8;
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut u8) -> u8;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PrepareBattlerSpriteForRotScale(a0: u8, a1: u8);
    fn RequestDma3Fill(a0: i32, a1: *mut u8, a2: u16, a3: u8) -> i16;
    fn ResetBattleAnimBg(a0: u8);
    fn ResetSpriteRotScale(a0: u8);
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBattlerSpriteYOffsetFromYScale(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetSpriteRotScale(a0: u8, a1: i16, a2: i16, a3: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCB_TrainerThrowObject(a0: *mut u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn TaskDummy(a0: u8);
    fn TranslateAnimHorizontalArc(a0: *mut u8) -> u8;
    fn UpdateOamPriorityInAllHealthboxes(a0: u8);
    fn m4aMPlayAllStop();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_UnusedLevelUpHealthBox(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBgData = crate::ffi::Align4([0u8; 16]);
        let mut healthBoxSpriteId: u8 = 0u8;
        let mut battler: u8 = 0u8;
        let mut spriteId1: u8 = 0u8;
        let mut spriteId2: u8 = 0u8;
        let mut spriteId3: u8 = 0u8;
        let mut spriteId4: u8 = 0u8;
        battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
        SetGpuReg(72u8, 16191u16);
        SetGpuReg(74u8, 16189u16);
        SetGpuRegBits(0u8, 32768u16);
        SetGpuReg(80u8, 16194u16);
        SetGpuReg(82u8, 4096u16);
        SetAnimBgAttribute(1u8, 4u8, 0u8);
        SetAnimBgAttribute(1u8, 0u8, 0u8);
        SetAnimBgAttribute(1u8, 1u8, 1u8);
        SetAnimBgAttribute(1u8, 3u8, 1u8);
        healthBoxSpriteId = (((&raw mut gHealthboxSpriteIds).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize))
        .read();
        spriteId1 = ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthBoxSpriteId) as i32) as isize * 68))
        .wrapping_add(6)
        .cast::<u16>())
        .read()) as u8);
        spriteId2 = ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthBoxSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as u8);
        spriteId3 = CreateInvisibleSpriteWithCallback(Some(SpriteCallbackDummy));
        spriteId4 = CreateInvisibleSpriteWithCallback(Some(SpriteCallbackDummy));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthBoxSpriteId) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId1) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId3) as i32) as isize * 68)
            .cast::<crate::c::Rec4<68>>()
            .write_unaligned(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthBoxSpriteId) as i32) as isize * 68)
                    .cast::<crate::c::Rec4<68>>()
                    .read_unaligned(),
            );
        ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId4) as i32) as isize * 68)
            .cast::<crate::c::Rec4<68>>()
            .write_unaligned(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId1) as i32) as isize * 68)
                    .cast::<crate::c::Rec4<68>>()
                    .read_unaligned(),
            );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId3) as i32) as isize * 68))
            .wrapping_add(1),
            2,
            2,
            (2u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId4) as i32) as isize * 68))
            .wrapping_add(1),
            2,
            2,
            (2u32) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId3) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId4) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        GetBattleAnimBg1Data((&raw mut animBgData).cast::<u8>());
        AnimLoadCompressedBgTilemap(
            (((((&raw mut animBgData).cast::<u8>()).wrapping_add(9)).read()) as u32),
            (((&raw mut UnusedLevelupAnimationTilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
        );
        AnimLoadCompressedBgGfx(
            (((((&raw mut animBgData).cast::<u8>()).wrapping_add(9)).read()) as u32),
            ((&raw mut UnusedLevelupAnimationGfx).cast::<u32>()).cast::<u32>(),
            (((((&raw mut animBgData).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as u32),
        );
        LoadCompressedPalette(
            ((&raw mut gCureBubblesPal).cast::<u32>()).cast::<u32>(),
            (((0i32).wrapping_add(
                (((((&raw mut animBgData).cast::<u8>()).wrapping_add(8)).read()) as i32)
                    .wrapping_mul(16i32),
            )) as u16),
            32u16,
        );
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(
            (((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId3) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_neg())
            .wrapping_add(32i32)) as u16),
        );
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(
            (((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId3) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_neg())
            .wrapping_sub(32i32)) as u16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(640i16);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((spriteId3) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((spriteId4) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_UnusedLevelUpHealthBox_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_UnusedLevelUpHealthBox_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId1: u8 = 0u8;
        let mut spriteId2: u8 = 0u8;
        let mut battler: u8 = 0u8;
        battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13);
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
        let __p2 = (&raw mut gBattle_BG1_Y).cast::<u16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(13))
                .read()) as u16) as i32)
                    >> 8),
            )) as u16),
        );
        let __p3 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13);
        (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
        'l1: {
            let __sw4 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32);
            if __sw4 == 0i32 {
                if (({
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t6 = (__p5).read();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    __t6
                }) as i32)
                    > 1i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(0i16);
                    let __p7 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .read()) as i32),
                        ) << 8)
                            | ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .read()) as i32)) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12))
                    .read()) as i32)
                        == 8i32
                    {
                        let __p8 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15);
                        (__p8).write(((__p8).read()).wrapping_add(1));
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
                    .wrapping_offset(10);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    == 30i32
                {
                    let __p11 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15);
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
                    .wrapping_offset(11);
                    let __t13 = (__p12).read();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                    __t13
                }) as i32)
                    > 1i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(0i16);
                    let __p14 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    (__p14).write(((__p14).read()).wrapping_sub(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .read()) as i32),
                        ) << 8)
                            | ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(12))
                            .read()) as i32)) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12))
                    .read()) as i32)
                        == 0i32
                    {
                        ResetBattleAnimBg(0u8);
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
                        DestroySprite(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32) as isize
                                    * 68,
                            ),
                        );
                        SetAnimBgAttribute(1u8, 1u8, 0u8);
                        spriteId1 = ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read()) as u8);
                        spriteId2 = ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as u8);
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            (1u16) as i32,
                        );
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId1) as i32) as isize * 68))
                            .wrapping_add(5),
                            2,
                            2,
                            (1u16) as i32,
                        );
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId2) as i32) as isize * 68))
                            .wrapping_add(5),
                            2,
                            2,
                            (1u16) as i32,
                        );
                        DestroyAnimVisualTask(taskId);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadHealthboxPalsForLevelUp(
    paletteId1: *mut u8,
    paletteId2: *mut u8,
    battler: u8,
) {
    unsafe {
        let mut paletteId1 = paletteId1;
        let mut paletteId2 = paletteId2;
        let mut battler = battler;
        let mut healthBoxSpriteId: u8 = 0u8;
        let mut spriteId1: u8 = 0u8;
        let mut spriteId2: u8 = 0u8;
        let mut offset1: u16 = 0u16;
        let mut offset2: u16 = 0u16;
        healthBoxSpriteId = (((&raw mut gHealthboxSpriteIds).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize))
        .read();
        spriteId1 = ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthBoxSpriteId) as i32) as isize * 68))
        .wrapping_add(6)
        .cast::<u16>())
        .read()) as u8);
        spriteId2 = ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthBoxSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as u8);
        (paletteId1).write(AllocSpritePalette(55049u16));
        (paletteId2).write(AllocSpritePalette(55050u16));
        offset1 = (((256i32).wrapping_add(
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthBoxSpriteId) as i32) as isize * 68))
                .wrapping_add(5),
                4,
                4,
                false,
            ) as u16) as i32)
                .wrapping_mul(16i32),
        )) as u16);
        offset2 = (((256i32).wrapping_add(
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId2) as i32) as isize * 68))
                .wrapping_add(5),
                4,
                4,
                false,
            ) as u16) as i32)
                .wrapping_mul(16i32),
        )) as u16);
        LoadPalette(
            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((offset1) as i32) as isize))
            .cast::<u8>(),
            (((256i32).wrapping_add((((paletteId1).read()) as i32).wrapping_mul(16i32))) as u16),
            32u16,
        );
        LoadPalette(
            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((offset2) as i32) as isize))
            .cast::<u8>(),
            (((256i32).wrapping_add((((paletteId2).read()) as i32).wrapping_mul(16i32))) as u16),
            32u16,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthBoxSpriteId) as i32) as isize * 68))
            .wrapping_add(5),
            4,
            4,
            (((paletteId1).read()) as u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId1) as i32) as isize * 68))
            .wrapping_add(5),
            4,
            4,
            (((paletteId1).read()) as u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(5),
            4,
            4,
            (((paletteId2).read()) as u16) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_LoadHealthboxPalsForLevelUp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut paletteId1: u8 = 0u8;
        let mut paletteId2: u8 = 0u8;
        LoadHealthboxPalsForLevelUp(
            &raw mut paletteId1,
            &raw mut paletteId2,
            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
        );
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn FreeHealthboxPalsForLevelUp(battler: u8) {
    unsafe {
        let mut battler = battler;
        let mut healthBoxSpriteId: u8 = 0u8;
        let mut spriteId1: u8 = 0u8;
        let mut spriteId2: u8 = 0u8;
        let mut paletteId1: u8 = 0u8;
        let mut paletteId2: u8 = 0u8;
        healthBoxSpriteId = (((&raw mut gHealthboxSpriteIds).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize))
        .read();
        spriteId1 = ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthBoxSpriteId) as i32) as isize * 68))
        .wrapping_add(6)
        .cast::<u16>())
        .read()) as u8);
        spriteId2 = ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthBoxSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as u8);
        FreeSpritePaletteByTag(55049u16);
        FreeSpritePaletteByTag(55050u16);
        paletteId1 = IndexOfSpritePaletteTag(55039u16);
        paletteId2 = IndexOfSpritePaletteTag(55044u16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthBoxSpriteId) as i32) as isize * 68))
            .wrapping_add(5),
            4,
            4,
            ((paletteId1) as u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId1) as i32) as isize * 68))
            .wrapping_add(5),
            4,
            4,
            ((paletteId1) as u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(5),
            4,
            4,
            ((paletteId2) as u16) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FreeHealthboxPalsForLevelUp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        FreeHealthboxPalsForLevelUp(((&raw mut gBattleAnimAttacker).cast::<u8>()).read());
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FlashHealthboxOnLevelUp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_FlashHealthboxOnLevelUp_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FlashHealthboxOnLevelUp_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut paletteNum: u8 = 0u8;
        let mut paletteOffset: u32 = 0u32;
        let mut colorOffset: u32 = 0u32;
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (({
            let __p2 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t3 = (__p2).read();
            (__p2).write(((__p2).read()).wrapping_add(1));
            __t3
        }) as i32)
            >= ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read()) as i32)
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            paletteNum = IndexOfSpritePaletteTag(55049u16);
            colorOffset = ((if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32)
                == 0i32
            {
                6i32
            } else {
                2i32
            }) as u32);
            'l1: {
                let __sw4 = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32);
                if __sw4 == 0i32 {
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p5).write((((((__p5).read()) as i32).wrapping_add(2i32)) as i16));
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        > 16i32
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(16i16);
                    }
                    paletteOffset =
                        (((256i32).wrapping_add(((paletteNum) as i32).wrapping_mul(16i32))) as u32);
                    BlendPalette(
                        (((paletteOffset).wrapping_add(colorOffset)) as u16),
                        1u16,
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as u8),
                        32628u16,
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        == 16i32
                    {
                        let __p6 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1);
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                    break 'l1;
                }
                if __sw4 == 1i32 {
                    let __p7 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p7).write((((((__p7).read()) as i32).wrapping_sub(2i32)) as i16));
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        < 0i32
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(0i16);
                    }
                    paletteOffset =
                        (((256i32).wrapping_add(((paletteNum) as i32).wrapping_mul(16i32))) as u32);
                    BlendPalette(
                        (((paletteOffset).wrapping_add(colorOffset)) as u16),
                        1u16,
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as u8),
                        32628u16,
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        == 0i32
                    {
                        DestroyAnimVisualTask(taskId);
                    }
                    break 'l1;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SwitchOutShrinkMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        spriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
        ))
        .read();
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                PrepareBattlerSpriteForRotScale(spriteId, 0u8);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(256i16);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(48i32)) as i16));
                SetSpriteRotScale(
                    spriteId,
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .read(),
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .read(),
                    0u16,
                );
                SetBattlerSpriteYOffsetFromYScale(spriteId);
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    >= 720i32
                {
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetSpriteRotScale(spriteId);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SwitchOutBallEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        let mut ball: u16 = 0u16;
        let mut ballId: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut priority: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        let mut selectedPalettes: u32 = 0u32;
        spriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
        ))
        .read();
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            ball = ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                    .read()) as i32) as isize
                        * 100,
                ),
                38i32,
            )) as u16);
        } else {
            ball = ((GetMonData2(
                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                    .read()) as i32) as isize
                        * 100,
                ),
                38i32,
            )) as u16);
        }
        ballId = ItemIdToBallId(ball);
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                x = GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    0u8,
                );
                y = GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    1u8,
                );
                priority = ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    false,
                ) as u16) as u8);
                subpriority = ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(67))
                .read();
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(
                    ((AnimateBallOpenParticles(
                        x,
                        ((((y) as i32).wrapping_add(32i32)) as u8),
                        priority,
                        subpriority,
                        ballId,
                    )) as i16),
                );
                selectedPalettes = GetBattlePalettesMask(1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .write(
                    ((LaunchBallFadeMonTask(
                        0u8,
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        selectedPalettes,
                        ballId,
                    )) as i16),
                );
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (!((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read())
                    != 0))
                    && (!((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(11))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(4))
                    .read())
                        != 0))
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_LoadBallGfx(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut ballId: u8 = ItemIdToBallId(((&raw mut gLastUsedItem).cast::<u16>()).read());
        LoadBallGfx(ballId);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FreeBallGfx(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut ballId: u8 = ItemIdToBallId(((&raw mut gLastUsedItem).cast::<u16>()).read());
        FreeBallGfx(ballId);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_IsBallBlockedByTrainer(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8))
        .read()) as i32)
            == 5i32
        {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write((-1i16));
        } else {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(0i16);
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemIdToBallId(ballItem: u16) -> u8 {
    unsafe {
        let mut ballItem = ballItem;
        'l1: {
            let __sw1 = ((ballItem) as i32);
            let __matched = __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 4i32;
            if __sw1 == 1i32 {
                return 4u8;
            }
            if __sw1 == 2i32 {
                return 3u8;
            }
            if __sw1 == 3i32 {
                return 1u8;
            }
            if __sw1 == 5i32 {
                return 2u8;
            }
            if __sw1 == 6i32 {
                return 5u8;
            }
            if __sw1 == 7i32 {
                return 6u8;
            }
            if __sw1 == 8i32 {
                return 7u8;
            }
            if __sw1 == 9i32 {
                return 8u8;
            }
            if __sw1 == 10i32 {
                return 9u8;
            }
            if __sw1 == 11i32 {
                return 10u8;
            }
            if __sw1 == 12i32 {
                return 11u8;
            }
            if __sw1 == 4i32 || !__matched {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ThrowBall(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut ballId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        ballId = ItemIdToBallId(((&raw mut gLastUsedItem).cast::<u16>()).read());
        spriteId = CreateSprite(
            ((&raw mut gBallSpriteTemplates).cast::<u8>())
                .wrapping_offset(((ballId) as i32) as isize * 24),
            32i16,
            80i16,
            29u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(34i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                as i32)
                .wrapping_sub(16i32)) as i16),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Ball_Throw));
        crate::c::bf_write(
            (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(9),
            1,
            1,
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                false,
            ) as u16) as u8) as i32,
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((spriteId) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_ThrowBall_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ThrowBall_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        if ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as u16) as i32)
            == 65535i32
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ThrowBall_StandingTrainer(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut ballId: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 512u32) != 0 {
            x = 32i16;
            y = 11i16;
        } else {
            x = 23i16;
            y = 5i16;
        }
        ballId = ItemIdToBallId(((&raw mut gLastUsedItem).cast::<u16>()).read());
        subpriority = ((((GetBattlerSpriteSubpriority(GetBattlerAtPosition(1u8))) as i32)
            .wrapping_add(1i32)) as u8);
        spriteId = CreateSprite(
            ((&raw mut gBallSpriteTemplates).cast::<u8>())
                .wrapping_offset(((ballId) as i32) as isize * 24),
            ((((x) as i32).wrapping_add(32i32)) as i16),
            ((((y) as i32) | 80i32) as i16),
            subpriority,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(34i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                as i32)
                .wrapping_sub(16i32)) as i16),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                .wrapping_offset(((GetBattlerAtPosition(0u8)) as i32) as isize))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_TrainerThrowObject));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((spriteId) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_ThrowBall_StandingTrainer_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ThrowBall_StandingTrainer_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                .wrapping_offset(((GetBattlerAtPosition(0u8)) as i32) as isize))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(43))
        .read()) as i32)
            == 1i32
        {
            PlaySE12WithPanning(61u16, 0i8);
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Ball_Throw));
            CreateTask(Some(Task_PlayerThrow_Wait), 10u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_ThrowBall_Step));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PlayerThrow_Wait(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((GetBattlerAtPosition(0u8)) as i32) as isize))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(63),
            4,
            1,
            false,
        ) as u16)
            != 0
        {
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((GetBattlerAtPosition(0u8)) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ),
                0u8,
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Throw(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut targetX: u16 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u16);
        let mut targetY: u16 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(((targetX) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(((targetY) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write((-40i16));
        InitAnimArcTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Ball_Arc));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Arc(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i32 = 0i32;
        let mut ballId: u8 = 0u8;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            if ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8))
            .read()) as i32)
                == 5i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_Ball_Block));
            } else {
                StartSpriteAnim(sprite, 1u8);
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
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i < 8i32) {
                            break 'l1;
                        }
                        'l2: {
                            ((((sprite).wrapping_add(46)).cast::<i16>())
                                .wrapping_offset((i) as isize))
                            .write(0i16);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_Ball_MonShrink));
                ballId = ItemIdToBallId(((&raw mut gLastUsedItem).cast::<u16>()).read());
                'l3: {
                    let __sw3 = ((ballId) as i32);
                    if (0i32..=11i32).contains(&__sw3) {
                        AnimateBallOpenParticles(
                            ((((sprite).wrapping_add(32).cast::<i16>()).read()) as u8),
                            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                                .wrapping_sub(5i32)) as u8),
                            1u8,
                            28u8,
                            ballId,
                        );
                        LaunchBallFadeMonTask(
                            0u8,
                            ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                            14u32,
                            ballId,
                        );
                        break 'l3;
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_MonShrink(sprite: *mut u8) {
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
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                .write(((CreateTask(Some(TaskDummy), 50u8)) as i16));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Ball_MonShrink_Step));
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
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
pub(crate) unsafe extern "C" fn SpriteCB_Ball_MonShrink_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut spriteId: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        spriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
        ))
        .read();
        taskId = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u8);
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 11i32
        {
            PlaySE(60u16);
        }
        'l1: {
            let __sw3 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw3 == 0i32 || __sw3 == 1i32 || __sw3 == 2i32 || __sw3 == 3i32;
            if __sw3 == 0i32 {
                PrepareBattlerSpriteForRotScale(spriteId, 0u8);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(256i16);
                ((&raw mut gMonShrinkDuration).cast::<u8>().cast::<u32>()).write(28u32);
                ((&raw mut gMonShrinkDistance).cast::<u8>().cast::<u16>()).write(
                    (((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(34)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(38)
                            .cast::<i16>())
                            .read()) as i32),
                        ))
                    .wrapping_sub(
                        ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                            ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32),
                        ),
                    )) as u16),
                );
                ((&raw mut gMonShrinkDelta).cast::<u8>().cast::<u16>()).write(
                    ((crate::c::div_u32(
                        ((((((&raw mut gMonShrinkDistance).cast::<u8>().cast::<u16>()).read())
                            as i32)
                            .wrapping_mul(256i32)) as u32),
                        ((&raw mut gMonShrinkDuration).cast::<u8>().cast::<u32>()).read(),
                    )) as u16),
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(((((&raw mut gMonShrinkDelta).cast::<u8>().cast::<u16>()).read()) as i16));
                let __p4 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw3 == 1i32 {
                let __p5 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10);
                (__p5).write((((((__p5).read()) as i32).wrapping_add(32i32)) as i16));
                SetSpriteRotScale(
                    spriteId,
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .read(),
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .read(),
                    0u16,
                );
                let __p6 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3);
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_add(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as i32),
                    )) as i16),
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(
                    ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32)
                        .wrapping_neg()
                        >> 8) as i16),
                );
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    >= 1152i32
                {
                    let __p7 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw3 == 2i32 {
                ResetSpriteRotScale(spriteId);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                let __p8 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw3 == 3i32 || !__matched {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    > 10i32
                {
                    DestroyTask(taskId);
                    StartSpriteAnim(sprite, 2u8);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_Ball_Bounce));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Bounce(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut phase: i16 = 0i16;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(40i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            phase = 0i16;
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(((Cos(phase, 40i16)) as i32))) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Cos(
                    phase,
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Ball_Bounce_Step));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Bounce_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut lastBounce: u8 = 0u8;
        let mut bounceCount: i16 = 0i16;
        lastBounce = 0u8;
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
                    (((((__p2).read()) as i32).wrapping_add(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                            >> 8)
                            .wrapping_add(4i32),
                    )) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    >= 64i32
                {
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                    (__p3).write((((((__p3).read()) as i32).wrapping_sub(10i32)) as i16));
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p4).write((((((__p4).read()) as i32).wrapping_add(257i32)) as i16));
                    bounceCount = ((((((((sprite).wrapping_add(46)).cast::<i16>())
                        .wrapping_offset(3))
                    .read()) as i32)
                        >> 8) as i16);
                    if ((bounceCount) as i32) == 4i32 {
                        lastBounce = 1u8;
                    }
                    'l2: {
                        let __sw5 = ((bounceCount) as i32);
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
                    (((((__p6).read()) as i32).wrapping_sub(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                            >> 8)
                            .wrapping_add(4i32),
                    )) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    <= 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p7).write((((((__p7).read()) as i32) & (-256i32)) as i16));
                }
                break 'l1;
            }
        }
        if (lastBounce) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            let __p8 = (sprite).wrapping_add(34).cast::<i16>();
            (__p8).write(
                (((((__p8).read()) as i32).wrapping_add(((Cos(64i16, 40i16)) as i32))) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            if ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8))
            .read()) as i32)
                == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_Ball_Release));
            } else {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_Ball_Wobble));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Wobble(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 31i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
            StartSpriteAffineAnim(sprite, 1u8);
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12)
            .cast::<i16>())
            .write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Ball_Wobble_Step));
            PlaySE(23u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Wobble_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut shakes: i8 = 0i8;
        let mut frame: u16 = 0u16;
        'l1: {
            let __sw1 = (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                as i32)
                & 255i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(12)
                .cast::<i16>())
                .read()) as i32)
                    > 255i32
                {
                    let __p2 = (sprite).wrapping_add(36).cast::<i16>();
                    (__p2).write(
                        (((((__p2).read()) as i32).wrapping_add(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                                .read()) as i32),
                        )) as i16),
                    );
                    let __p3 = (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<i16>();
                    (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
                } else {
                    let __p4 = (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<i16>();
                    (__p4).write((((((__p4).read()) as i32).wrapping_add(176i32)) as i16));
                }
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p5).write(((__p5).read()).wrapping_add(1));
                crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (0u8) as i32);
                frame =
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        .wrapping_add(7i32)) as u16);
                if ((frame) as i32) > 14i32 {
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<i16>())
                    .write(0i16);
                    let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (({
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    == 1i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)).wrapping_neg()) as i16));
                    let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p9).write(((__p9).read()).wrapping_add(1));
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
            if __sw1 == 2i32 {
                __fall = true;
                if ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(12)
                .cast::<i16>())
                .read()) as i32)
                    > 255i32
                {
                    let __p10 = (sprite).wrapping_add(36).cast::<i16>();
                    (__p10).write(
                        (((((__p10).read()) as i32).wrapping_add(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                                .read()) as i32),
                        )) as i16),
                    );
                    let __p11 = (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<i16>();
                    (__p11).write((((((__p11).read()) as i32) & 255i32) as i16));
                } else {
                    let __p12 = (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<i16>();
                    (__p12).write((((((__p12).read()) as i32).wrapping_add(176i32)) as i16));
                }
                let __p13 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p13).write(((__p13).read()).wrapping_add(1));
                crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (0u8) as i32);
                frame =
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        .wrapping_add(12i32)) as u16);
                if ((frame) as i32) > 24i32 {
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<i16>())
                    .write(0i16);
                    let __p14 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p14).write(((__p14).read()).wrapping_add(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (({
                    let __p15 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                    let __t16 = (__p15).read();
                    (__p15).write(((__p15).read()).wrapping_add(1));
                    __t16
                }) as i32)
                    < 0i32
                {
                    crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
                    break 'l1;
                }
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        .wrapping_neg()) as i16),
                );
                let __p17 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p17).write(((__p17).read()).wrapping_add(1));
                crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (0u8) as i32);
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    < 0i32
                {
                    ChangeSpriteAffineAnim(sprite, 2u8);
                } else {
                    ChangeSpriteAffineAnim(sprite, 1u8);
                }
            }
            if __fall || __sw1 == 4i32 {
                __fall = true;
                if ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(12)
                .cast::<i16>())
                .read()) as i32)
                    > 255i32
                {
                    let __p18 = (sprite).wrapping_add(36).cast::<i16>();
                    (__p18).write(
                        (((((__p18).read()) as i32).wrapping_add(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                                .read()) as i32),
                        )) as i16),
                    );
                    let __p19 = (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<i16>();
                    (__p19).write((((((__p19).read()) as i32) & 255i32) as i16));
                } else {
                    let __p20 = (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<i16>();
                    (__p20).write((((((__p20).read()) as i32).wrapping_add(176i32)) as i16));
                }
                let __p21 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p21).write(((__p21).read()).wrapping_add(1));
                crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (0u8) as i32);
                frame =
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        .wrapping_add(4i32)) as u16);
                if ((frame) as i32) > 8i32 {
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<i16>())
                    .write(0i16);
                    let __p22 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p22).write(((__p22).read()).wrapping_add(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)).wrapping_neg()) as i16));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                let __p23 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p23).write((((((__p23).read()) as i32).wrapping_add(256i32)) as i16));
                shakes = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32)
                    >> 8) as i8);
                if ((shakes) as i32)
                    == ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(8))
                    .read()) as i32)
                {
                    crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_Ball_Release));
                } else {
                    if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(8))
                    .read()) as i32)
                        == 4i32)
                        && (((shakes) as i32) == 3i32)
                    {
                        ((sprite)
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_Ball_Capture));
                        crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
                    } else {
                        let __p24 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                        (__p24).write(((__p24).read()).wrapping_add(1));
                        crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 || !__matched {
                __fall = true;
                if (({
                    let __p25 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                    let __t26 = ((__p25).read()).wrapping_add(1);
                    (__p25).write(__t26);
                    __t26
                }) as i32)
                    == 31i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    let __p27 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p27).write((((((__p27).read()) as i32) & (-256i32)) as i16));
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
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Release(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 31i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Ball_Release_Step));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Capture(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Ball_Capture_Step));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Capture_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: *mut u8 = (&raw mut gBattleAnimTarget).cast::<u8>();
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            == 40i32
        {
            PlaySE(254u16);
            BlendPalettes(
                ((crate::c::shl_i32(
                    65536i32,
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as u32),
                )) as u32),
                6u8,
                0u16,
            );
            MakeCaptureStars(sprite);
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                == 60i32
            {
                BeginNormalPaletteFade(
                    ((crate::c::shl_i32(
                        65536i32,
                        ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as u32),
                    )) as u32),
                    2i8,
                    6u8,
                    0u8,
                    0u16,
                );
            } else {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    == 95i32
                {
                    ((&raw mut gDoingBattleAnim).cast::<u8>()).write(0u8);
                    UpdateOamPriorityInAllHealthboxes(1u8);
                    m4aMPlayAllStop();
                    PlaySE(531u16);
                } else {
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        == 315i32
                    {
                        FreeOamMatrix(
                            ((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                        .wrapping_offset((((battler).read()) as i32) as isize))
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
                                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                    .wrapping_offset((((battler).read()) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ),
                        );
                        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                        ((sprite)
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_Ball_FadeOut));
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_FadeOut(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut paletteIndex: u8 = 0u8;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                crate::c::bf_write((sprite).wrapping_add(1), 2, 2, (1u32) as i32);
                SetGpuReg(80u8, 16192u16);
                SetGpuReg(82u8, 16u16);
                paletteIndex = IndexOfSpritePaletteTag(
                    ((((sprite).wrapping_add(20).cast::<*mut u8>()).read())
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read(),
                );
                BeginNormalPaletteFade(
                    ((crate::c::shl_i32(
                        1i32,
                        ((((paletteIndex) as i32).wrapping_add(16i32)) as u32),
                    )) as u32),
                    0i8,
                    0u8,
                    16u8,
                    32767u16,
                );
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t4 = (__p3).read();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    __t4
                }) as i32)
                    > 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    SetGpuReg(
                        82u8,
                        (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                            .read()) as i32)
                            << 8)
                            | (16i32).wrapping_sub(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                            )) as u16),
                    );
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        == 16i32
                    {
                        let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                let __p7 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetGpuReg(80u8, 0u16);
                    SetGpuReg(82u8, 0u16);
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(DestroySpriteAfterOneFrame));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DestroySpriteAfterOneFrame(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write((-1i16));
        } else {
            FreeSpriteOamMatrix(sprite);
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn MakeCaptureStars(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: u32 = 0u32;
        let mut subpriority: u8 = 0u8;
        if (((sprite).wrapping_add(67)).read()) != 0 {
            subpriority =
                ((((((sprite).wrapping_add(67)).read()) as i32).wrapping_sub(1i32)) as u8);
        } else {
            subpriority = 0u8;
            ((sprite).wrapping_add(67)).write(1u8);
        }
        LoadBallParticleGfx(4u8);
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(12u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateSprite(
                        (((&raw const sBallParticleSpriteTemplates)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(96),
                        ((sprite).wrapping_add(32).cast::<i16>()).read(),
                        ((sprite).wrapping_add(34).cast::<i16>()).read(),
                        subpriority,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(24i16);
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(
                            ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                                .wrapping_add(
                                    (((((((&raw const sCaptureStars).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .cast::<i8>())
                                    .read()) as i32),
                                )) as i16),
                        );
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(
                            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                                .wrapping_add(
                                    (((((((&raw const sCaptureStars).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(1)
                                    .cast::<i8>())
                                    .read()) as i32),
                                )) as i16),
                        );
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(
                            (((((((&raw const sCaptureStars).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<i8>())
                            .read()) as i16),
                        );
                        InitAnimArcTranslation(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                        );
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_CaptureStar_Flicker));
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            ((((&raw const sBallParticleAnimNums).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(4))
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CaptureStar_Flicker(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            ((!((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0)) as u16)
                as i32,
        );
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Release_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut ballId: u8 = 0u8;
        StartSpriteAnim(sprite, 1u8);
        StartSpriteAffineAnim(sprite, 0u8);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Ball_Release_Wait));
        ballId = ItemIdToBallId(((&raw mut gLastUsedItem).cast::<u16>()).read());
        'l1: {
            let __sw1 = ((ballId) as i32);
            if (0i32..=11i32).contains(&__sw1) {
                AnimateBallOpenParticles(
                    ((((sprite).wrapping_add(32).cast::<i16>()).read()) as u8),
                    ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                        .wrapping_sub(5i32)) as u8),
                    1u8,
                    28u8,
                    ballId,
                );
                LaunchBallFadeMonTask(
                    1u8,
                    ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    14u32,
                    ballId,
                );
                break 'l1;
            }
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        StartSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ),
            1u8,
        );
        AnimateSprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
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
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Release_Wait(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut released: u8 = 0u8;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
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
            StartSpriteAffineAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ),
                0u8,
            );
            released = 1u8;
        } else {
            let __p1 = (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(288i32)) as i16));
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                    ))
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
            && ((released) != 0)
        {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(0i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                ((crate::c::bf_read(
                    (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(9),
                    1,
                    1,
                    false,
                ) as u8) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(DestroySpriteAfterOneFrame));
            ((&raw mut gDoingBattleAnim).cast::<u8>()).write(0u8);
            UpdateOamPriorityInAllHealthboxes(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Block(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i32 = 0i32;
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
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset((i) as isize))
                        .write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Ball_Block_Step));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Ball_Block_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut dy: i16 = (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            .wrapping_add(2048i32)) as i16);
        let mut dx: i16 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .read()) as i32)
            .wrapping_add(1664i32)) as i16);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub((((dx) as i32) >> 8))) as i16));
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add((((dy) as i32) >> 8))) as i16));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(2048i32)
                & 255i32) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_add(1664i32)
                & 255i32) as i16),
        );
        if (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
            > 160i32)
            || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
                < (-8i32))
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(DestroySpriteAfterOneFrame));
            ((&raw mut gDoingBattleAnim).cast::<u8>()).write(0u8);
            UpdateOamPriorityInAllHealthboxes(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadBallParticleGfx(ballId: u8) {
    unsafe {
        let mut ballId = ballId;
        if ((GetSpriteTileStartByTag(
            (((((&raw const sBallParticleSpriteSheets)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((ballId) as i32) as isize * 8))
            .wrapping_add(6)
            .cast::<u16>())
            .read(),
        )) as i32)
            == 65535i32
        {
            LoadCompressedSpriteSheetUsingHeap(
                (((&raw const sBallParticleSpriteSheets)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((ballId) as i32) as isize * 8),
            );
            LoadCompressedSpritePaletteUsingHeap(
                (((&raw const sBallParticlePalettes).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((ballId) as i32) as isize * 8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimateBallOpenParticles(
    x: u8,
    y: u8,
    priority: u8,
    subpriority: u8,
    ballId: u8,
) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut priority = priority;
        let mut subpriority = subpriority;
        let mut ballId = ballId;
        let mut taskId: u8 = 0u8;
        LoadBallParticleGfx(ballId);
        taskId = CreateTask(
            ((((&raw const sBallParticleAnimationFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .wrapping_offset(((ballId) as i32) as isize))
            .read(),
            5u8,
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((x) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((y) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((priority) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((subpriority) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(((ballId) as i16));
        PlaySE(15u16);
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn IncrBallParticleCount() {
    unsafe {
        if (crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0
        {
            let __p1 = (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(10);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn PokeBallOpenParticleAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut priority: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        let mut ballId: u8 = 0u8;
        let mut var0: u8 = 0u8;
        ballId = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as u8);
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            < 16i32
        {
            x = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as u8);
            y = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as u8);
            priority = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as u8);
            subpriority = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8);
            spriteId = CreateSprite(
                (((&raw const sBallParticleSpriteTemplates)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((ballId) as i32) as isize * 24),
                ((x) as i16),
                ((y) as i16),
                subpriority,
            );
            if ((spriteId) as i32) != 64i32 {
                IncrBallParticleCount();
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((((&raw const sBallParticleAnimNums).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((ballId) as i32) as isize))
                    .read(),
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(PokeBallOpenParticleAnimation_Step1));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    ((priority) as u16) as i32,
                );
                var0 = (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8);
                if ((var0) as i32) >= 8i32 {
                    var0 = ((((var0) as i32).wrapping_sub(8i32)) as u8);
                }
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(((((var0) as i32).wrapping_mul(32i32)) as i16));
            }
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 15i32
            {
                if !((crate::c::bf_read(
                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                    1,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(7))
                    .write(1i16);
                }
                DestroyTask(taskId);
                return;
            }
        }
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn PokeBallOpenParticleAnimation_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 0i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(PokeBallOpenParticleAnimation_Step2));
        } else {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn PokeBallOpenParticleAnimation_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 50i32
        {
            DestroyBallOpenAnimationParticle(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn TimerBallOpenParticleAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut priority: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        let mut ballId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        ballId = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as u8);
        x = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        y = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        priority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as u8);
        subpriority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (((&raw const sBallParticleSpriteTemplates)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((ballId) as i32) as isize * 24),
                        ((x) as i16),
                        ((y) as i16),
                        subpriority,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        IncrBallParticleCount();
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            ((((&raw const sBallParticleAnimNums).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((ballId) as i32) as isize))
                            .read(),
                        );
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(FanOutBallOpenParticles_Step1));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(5),
                            2,
                            2,
                            ((priority) as u16) as i32,
                        );
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((((i) as i32).wrapping_mul(32i32)) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(10i16);
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(2i16);
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .write(1i16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0)
        {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(1i16);
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn DiveBallOpenParticleAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut priority: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        let mut ballId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        ballId = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as u8);
        x = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        y = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        priority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as u8);
        subpriority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (((&raw const sBallParticleSpriteTemplates)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((ballId) as i32) as isize * 24),
                        ((x) as i16),
                        ((y) as i16),
                        subpriority,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        IncrBallParticleCount();
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            ((((&raw const sBallParticleAnimNums).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((ballId) as i32) as isize))
                            .read(),
                        );
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(FanOutBallOpenParticles_Step1));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(5),
                            2,
                            2,
                            ((priority) as u16) as i32,
                        );
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((((i) as i32).wrapping_mul(32i32)) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(10i16);
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(1i16);
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .write(2i16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0)
        {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(1i16);
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SafariBallOpenParticleAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut priority: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        let mut ballId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        ballId = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as u8);
        x = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        y = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        priority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as u8);
        subpriority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (((&raw const sBallParticleSpriteTemplates)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((ballId) as i32) as isize * 24),
                        ((x) as i16),
                        ((y) as i16),
                        subpriority,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        IncrBallParticleCount();
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            ((((&raw const sBallParticleAnimNums).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((ballId) as i32) as isize))
                            .read(),
                        );
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(FanOutBallOpenParticles_Step1));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(5),
                            2,
                            2,
                            ((priority) as u16) as i32,
                        );
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((((i) as i32).wrapping_mul(32i32)) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(4i16);
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(1i16);
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .write(1i16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0)
        {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(1i16);
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn UltraBallOpenParticleAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut priority: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        let mut ballId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        ballId = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as u8);
        x = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        y = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        priority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as u8);
        subpriority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (((&raw const sBallParticleSpriteTemplates)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((ballId) as i32) as isize * 24),
                        ((x) as i16),
                        ((y) as i16),
                        subpriority,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        IncrBallParticleCount();
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            ((((&raw const sBallParticleAnimNums).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((ballId) as i32) as isize))
                            .read(),
                        );
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(FanOutBallOpenParticles_Step1));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(5),
                            2,
                            2,
                            ((priority) as u16) as i32,
                        );
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((((i) as i32).wrapping_mul(25i32)) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(5i16);
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(1i16);
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .write(1i16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0)
        {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(1i16);
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn GreatBallOpenParticleAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut priority: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        let mut ballId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .read())
            != 0
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            ballId = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as u8);
            x = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as u8);
            y = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as u8);
            priority = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as u8);
            subpriority = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8);
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 8i32) {
                        break 'l1;
                    }
                    'l2: {
                        spriteId = CreateSprite(
                            (((&raw const sBallParticleSpriteTemplates)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((ballId) as i32) as isize * 24),
                            ((x) as i16),
                            ((y) as i16),
                            subpriority,
                        );
                        if ((spriteId) as i32) != 64i32 {
                            IncrBallParticleCount();
                            StartSpriteAnim(
                                ((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                                ((((&raw const sBallParticleAnimNums).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((ballId) as i32) as isize))
                                .read(),
                            );
                            ((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .write(Some(FanOutBallOpenParticles_Step1));
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(5),
                                2,
                                2,
                                ((priority) as u16) as i32,
                            );
                            (((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .write(((((i) as i32).wrapping_mul(32i32)) as i16));
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .write(8i16);
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(5))
                            .write(2i16);
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .write(2i16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(8i16);
            if (({
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 2i32
            {
                if !((crate::c::bf_read(
                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                    1,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(7))
                    .write(1i16);
                }
                DestroyTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FanOutBallOpenParticles_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
        ));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            ) & 255i32) as i16),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16),
        );
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 51i32
        {
            DestroyBallOpenAnimationParticle(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn RepeatBallOpenParticleAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut priority: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        let mut ballId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        ballId = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as u8);
        x = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        y = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        priority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as u8);
        subpriority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 12i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (((&raw const sBallParticleSpriteTemplates)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((ballId) as i32) as isize * 24),
                        ((x) as i16),
                        ((y) as i16),
                        subpriority,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        IncrBallParticleCount();
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            ((((&raw const sBallParticleAnimNums).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((ballId) as i32) as isize))
                            .read(),
                        );
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(RepeatBallOpenParticleAnimation_Step1));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(5),
                            2,
                            2,
                            ((priority) as u16) as i32,
                        );
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((((i) as i32).wrapping_mul(21i32)) as i16));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0)
        {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(1i16);
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn RepeatBallOpenParticleAnimation_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            Sin(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            ),
        ));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(6i32)
                & 255i32) as i16),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 51i32
        {
            DestroyBallOpenAnimationParticle(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn MasterBallOpenParticleAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut priority: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        let mut ballId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        ballId = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as u8);
        x = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        y = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        priority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as u8);
        subpriority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as u8);
        {
            j = 0u8;
            'l1: loop {
                if !(((j) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        i = 0u8;
                        'l3: loop {
                            if !(((i) as i32) < 8i32) {
                                break 'l3;
                            }
                            'l4: {
                                spriteId = CreateSprite(
                                    (((&raw const sBallParticleSpriteTemplates)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((ballId) as i32) as isize * 24),
                                    ((x) as i16),
                                    ((y) as i16),
                                    subpriority,
                                );
                                if ((spriteId) as i32) != 64i32 {
                                    IncrBallParticleCount();
                                    StartSpriteAnim(
                                        ((&raw mut gSprites).cast::<u8>())
                                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                                        ((((&raw const sBallParticleAnimNums)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((ballId) as i32) as isize))
                                        .read(),
                                    );
                                    ((((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                                    .wrapping_add(28)
                                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                    .write(Some(FanOutBallOpenParticles_Step1));
                                    crate::c::bf_write(
                                        (((&raw mut gSprites).cast::<u8>())
                                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                                        .wrapping_add(5),
                                        2,
                                        2,
                                        ((priority) as u16) as i32,
                                    );
                                    (((((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .write(((((i) as i32).wrapping_mul(32i32)) as i16));
                                    ((((((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .wrapping_offset(4))
                                    .write(8i16);
                                    if ((j) as i32) == 0i32 {
                                        ((((((&raw mut gSprites).cast::<u8>())
                                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                                        .wrapping_add(46))
                                        .cast::<i16>())
                                        .wrapping_offset(5))
                                        .write(2i16);
                                        ((((((&raw mut gSprites).cast::<u8>())
                                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                                        .wrapping_add(46))
                                        .cast::<i16>())
                                        .wrapping_offset(6))
                                        .write(1i16);
                                    } else {
                                        ((((((&raw mut gSprites).cast::<u8>())
                                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                                        .wrapping_add(46))
                                        .cast::<i16>())
                                        .wrapping_offset(5))
                                        .write(1i16);
                                        ((((((&raw mut gSprites).cast::<u8>())
                                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                                        .wrapping_add(46))
                                        .cast::<i16>())
                                        .wrapping_offset(6))
                                        .write(2i16);
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
        if !((crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0)
        {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(1i16);
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn PremierBallOpenParticleAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut priority: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        let mut ballId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        ballId = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as u8);
        x = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        y = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        priority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as u8);
        subpriority = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (((&raw const sBallParticleSpriteTemplates)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((ballId) as i32) as isize * 24),
                        ((x) as i16),
                        ((y) as i16),
                        subpriority,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        IncrBallParticleCount();
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            ((((&raw const sBallParticleAnimNums).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((ballId) as i32) as isize))
                            .read(),
                        );
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(PremierBallOpenParticleAnimation_Step1));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(5),
                            2,
                            2,
                            ((priority) as u16) as i32,
                        );
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((((i) as i32).wrapping_mul(32i32)) as i16));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0)
        {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(1i16);
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn PremierBallOpenParticleAnimation_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            Sin(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) & 63i32) as i16),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            ),
        ));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(10i32)
                & 255i32) as i16),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 51i32
        {
            DestroyBallOpenAnimationParticle(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyBallOpenAnimationParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        if !((crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0)
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                == 1i32
            {
                DestroySpriteAndFreeResources(sprite);
            } else {
                DestroySprite(sprite);
            }
        } else {
            let __p1 = (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(10);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            if ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(10))
            .read()) as i32)
                == 0i32
            {
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i < 12i32) {
                            break 'l1;
                        }
                        'l2: {
                            if ((FuncIsActiveTask(
                                ((((&raw const sBallParticleAnimationFuncs)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            )) as i32)
                                == 1i32
                            {
                                break 'l1;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i == 12i32 {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 12i32) {
                                break 'l3;
                            }
                            'l4: {
                                FreeSpriteTilesByTag(
                                    (((((&raw const sBallParticleSpriteSheets)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 8))
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                    .read(),
                                );
                                FreeSpritePaletteByTag(
                                    (((((&raw const sBallParticlePalettes)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 8))
                                    .wrapping_add(4)
                                    .cast::<u16>())
                                    .read(),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                DestroySprite(sprite);
            } else {
                DestroySprite(sprite);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LaunchBallFadeMonTask(
    unfadeLater: u8,
    spritePalNum: u8,
    selectedPalettes: u32,
    ballId: u8,
) -> u8 {
    unsafe {
        let mut unfadeLater = unfadeLater;
        let mut spritePalNum = spritePalNum;
        let mut selectedPalettes = selectedPalettes;
        let mut ballId = ballId;
        let mut taskId: u8 = 0u8;
        taskId = CreateTask(Some(Task_FadeMon_ToBallColor), 5u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(((ballId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((spritePalNum) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(((selectedPalettes) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .write(((selectedPalettes >> 16) as i16));
        if !((unfadeLater) != 0) {
            BlendPalette(
                (((256i32).wrapping_add(((spritePalNum) as i32).wrapping_mul(16i32))) as u16),
                16u16,
                0u8,
                ((((&raw const gBallOpenFadeColors)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((ballId) as i32) as isize))
                .read(),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(1i16);
        } else {
            BlendPalette(
                (((256i32).wrapping_add(((spritePalNum) as i32).wrapping_mul(16i32))) as u16),
                16u16,
                16u8,
                ((((&raw const gBallOpenFadeColors)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((ballId) as i32) as isize))
                .read(),
            );
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(16i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write((-1i16));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_FadeMon_ToNormal));
        }
        BeginNormalPaletteFade(selectedPalettes, 0i8, 0u8, 16u8, 32767u16);
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_FadeMon_ToBallColor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut ballId: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as u8);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            <= 16i32
        {
            BlendPalette(
                (((256i32).wrapping_add(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32)
                        .wrapping_mul(16i32),
                )) as u16),
                16u16,
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
                ((((&raw const gBallOpenFadeColors)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((ballId) as i32) as isize))
                .read(),
            );
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
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
            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
        } else {
            if !((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0)
            {
                let mut selectedPalettes: u32 = (((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as u16) as i32)
                    | ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .read()) as u16) as i32)
                        << 16)) as u32);
                BeginNormalPaletteFade(selectedPalettes, 0i8, 16u8, 0u8, 32767u16);
                DestroyTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_FadeMon_ToNormal(taskId: u8) {
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
            let mut selectedPalettes: u32 = (((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as u16) as i32)
                | ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .read()) as u16) as i32)
                    << 16)) as u32);
            BeginNormalPaletteFade(selectedPalettes, 0i8, 16u8, 0u8, 32767u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_FadeMon_ToNormal_Step));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_FadeMon_ToNormal_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut ballId: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as u8);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            <= 16i32
        {
            BlendPalette(
                (((256i32).wrapping_add(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32)
                        .wrapping_mul(16i32),
                )) as u16),
                16u16,
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
                ((((&raw const gBallOpenFadeColors)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((ballId) as i32) as isize))
                .read(),
            );
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
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
            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
        } else {
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SwapMonSpriteToFromSubstitute(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        let mut x: u32 = 0u32;
        let mut done: u32 = 0u32;
        spriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
        ))
        .read();
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(1280i32)) as i16));
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    != 0i32
                {
                    let __p3 = (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>();
                    (__p3).write(
                        (((((__p3).read()) as i32).wrapping_add(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .read()) as i32)
                                >> 8),
                        )) as i16),
                    );
                } else {
                    let __p4 = (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>();
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .read()) as i32)
                                >> 8),
                        )) as i16),
                    );
                }
                let __p5 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p5).write((((((__p5).read()) as i32) & 255i32) as i16));
                x = (((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(32)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .read()) as i32),
                    ))
                .wrapping_add(32i32)) as u32);
                if x > 304u32 {
                    let __p6 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                LoadBattleMonGfxAndAnimate(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .read()) as u8),
                    spriteId,
                );
                let __p7 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p8 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p8).write((((((__p8).read()) as i32).wrapping_add(1280i32)) as i16));
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    != 0i32
                {
                    let __p9 = (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>();
                    (__p9).write(
                        (((((__p9).read()) as i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .read()) as i32)
                                >> 8),
                        )) as i16),
                    );
                } else {
                    let __p10 = (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>();
                    (__p10).write(
                        (((((__p10).read()) as i32).wrapping_add(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .read()) as i32)
                                >> 8),
                        )) as i16),
                    );
                }
                let __p11 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p11).write((((((__p11).read()) as i32) & 255i32) as i16));
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    != 0i32
                {
                    if ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .read()) as i32)
                        <= 0i32
                    {
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write(0i16);
                        done = 1u32;
                    }
                } else {
                    if ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .read()) as i32)
                        >= 0i32
                    {
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write(0i16);
                        done = 1u32;
                    }
                }
                if (done) != 0 {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SubstituteFadeToInvisible(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32);
            if __sw1 == 0i32 {
                if ((GetBattlerSpriteBGPriorityRank(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                )) as i32)
                    == 1i32
                {
                    SetGpuReg(80u8, 16194u16);
                } else {
                    SetGpuReg(80u8, 16196u16);
                }
                SetGpuReg(82u8, 16u16);
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    let __t4 = (__p3).read();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    __t4
                }) as i32)
                    > 1i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    let __p5 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    SetGpuReg(
                        82u8,
                        ((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .read()) as i32)
                            << 8)
                            | (16i32).wrapping_sub(
                                (((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .read()) as i32),
                            )) as u16),
                    );
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32)
                        == 16i32
                    {
                        let __p6 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15);
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                spriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read();
                RequestDma3Fill(
                    0i32,
                    ((100728832i32) as usize as *mut u8).wrapping_offset(
                        (((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(4),
                            0,
                            10,
                            false,
                        ) as u16) as i32)
                            .wrapping_mul(crate::c::div_i32(256i32, 8i32)))
                            as isize
                            * 1,
                    ),
                    ((crate::c::div_i32(4096i32, 2i32)) as u16),
                    1u8,
                );
                ClearBehindSubstituteBit(((&raw mut gBattleAnimAttacker).cast::<u8>()).read());
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_IsAttackerBehindSubstitute(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).write(
            ((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize * 4,
                ))
                .wrapping_add(0),
                2,
                1,
                false,
            ) as u16) as i16),
        );
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetTargetToEffectBattler(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gBattleAnimTarget).cast::<u8>())
            .write(((&raw mut gEffectBattler).cast::<u8>()).read());
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryShinyAnimation(battler: u8, mon: *mut u8) {
    unsafe {
        let mut battler = battler;
        let mut mon = mon;
        let mut isShiny: u8 = 0u8;
        let mut otId: u32 = 0u32;
        let mut personality: u32 = 0u32;
        let mut shinyValue: u32 = 0u32;
        let mut taskCirc: u8 = 0u8;
        let mut taskDgnl: u8 = 0u8;
        isShiny = 0u8;
        crate::c::bf_write(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 12))
            .wrapping_add(0),
            7,
            1,
            (1u8) as i32,
        );
        otId = GetMonData2(mon, 1i32);
        personality = GetMonData2(mon, 0i32);
        if (IsBattlerSpriteVisible(battler)) != 0 {
            shinyValue = (((((otId & 4294901760u32) >> 16) ^ (otId & 65535u32))
                ^ ((personality & 4294901760u32) >> 16))
                ^ (personality & 65535u32));
            if shinyValue < 8u32 {
                isShiny = 1u8;
            }
            if (isShiny) != 0 {
                if ((GetSpriteTileStartByTag(10233u16)) as i32) == 65535i32 {
                    LoadCompressedSpriteSheetUsingHeap(
                        ((&raw mut gBattleAnimPicTable).cast::<u8>()).wrapping_offset(1864),
                    );
                    LoadCompressedSpritePaletteUsingHeap(
                        ((&raw mut gBattleAnimPaletteTable).cast::<u8>()).wrapping_offset(1864),
                    );
                }
                taskCirc = CreateTask(Some(Task_ShinyStars), 10u8);
                taskDgnl = CreateTask(Some(Task_ShinyStars), 10u8);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskCirc) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(((battler) as i16));
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskDgnl) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(((battler) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskCirc) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskDgnl) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(1i16);
                return;
            }
        }
        crate::c::bf_write(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 12))
            .wrapping_add(1),
            0,
            1,
            (1u8) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_ShinyStars(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut battler: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        let mut timer: u16 = 0u16;
        let mut starIdx: i16 = 0i16;
        let mut pan: u8 = 0u8;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13))
        .read()) as i32)
            < 60i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(13);
            (__p1).write(((__p1).read()).wrapping_add(1));
            return;
        }
        if (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(10))
        .read())
            != 0
        {
            return;
        }
        timer = (({
            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            let __t3 = (__p2).read();
            (__p2).write(((__p2).read()).wrapping_add(1));
            __t3
        }) as u16);
        if (crate::c::rem_i32(((timer) as i32), 4i32)) != 0 {
            return;
        }
        battler = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        x = GetBattlerSpriteCoord(battler, 0u8);
        y = GetBattlerSpriteCoord(battler, 1u8);
        starIdx = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .read();
        if ((starIdx) as i32) == 0i32 {
            spriteId = CreateSprite(
                (&raw mut gWishStarSpriteTemplate).cast::<u8>(),
                ((x) as i16),
                ((y) as i16),
                5u8,
            );
        } else {
            if (((starIdx) as i32) >= 0i32)
                && (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .read()) as i32)
                    < 4i32)
            {
                spriteId = CreateSprite(
                    (&raw mut gMiniTwinklingStarSpriteTemplate).cast::<u8>(),
                    ((x) as i16),
                    ((y) as i16),
                    5u8,
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
                        .wrapping_add(4i32)) as u16) as i32,
                );
            } else {
                spriteId = CreateSprite(
                    (&raw mut gMiniTwinklingStarSpriteTemplate).cast::<u8>(),
                    ((x) as i16),
                    ((y) as i16),
                    5u8,
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
                        .wrapping_add(5i32)) as u16) as i32,
                );
            }
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 0i32
        {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ShinyStars_Encircle));
        } else {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ShinyStars_Diagonal));
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .write((-32i16));
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .write(32i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read()) as i32)
                == 0i32
            {
                if ((GetBattlerSide(battler)) as i32) == 0i32 {
                    pan = 192u8;
                } else {
                    pan = 63u8;
                }
                PlaySE12WithPanning(102u16, ((pan) as i8));
            }
        }
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        let __p4 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11);
        (__p4).write(((__p4).read()).wrapping_add(1));
        if ((spriteId) as i32) != 64i32 {
            let __p5 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12);
            (__p5).write(((__p5).read()).wrapping_add(1));
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .read()) as i32)
            == 5i32
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ShinyStars_Wait));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShinyStars_Wait(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut battler: u8 = 0u8;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(12))
        .read()) as i32)
            == 0i32
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == 1i32
            {
                battler = (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8);
                crate::c::bf_write(
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 12))
                    .wrapping_add(1),
                    0,
                    1,
                    (1u8) as i32,
                );
            }
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShinyStars_Encircle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            24i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            24i16,
        ));
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(12i32)) as i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            > 255i32
        {
            let __p2 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12);
            (__p2).write(((__p2).read()).wrapping_sub(1));
            FreeSpriteOamMatrix(sprite);
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShinyStars_Diagonal(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32) < 4i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            let __p2 = (sprite).wrapping_add(36).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(5i32)) as i16));
            let __p3 = (sprite).wrapping_add(38).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_sub(5i32)) as i16));
            if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) > 32i32 {
                let __p4 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12);
                (__p4).write(((__p4).read()).wrapping_sub(1));
                FreeSpriteOamMatrix(sprite);
                DestroySprite(sprite);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_LoadPokeblockGfx(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut paletteIndex: u8 = 0u8;
        LoadCompressedSpriteSheetUsingHeap(
            ((&raw mut gBattleAnimPicTable).cast::<u8>()).wrapping_offset(2152),
        );
        LoadCompressedSpritePaletteUsingHeap(
            ((&raw mut gBattleAnimPaletteTable).cast::<u8>()).wrapping_offset(2152),
        );
        paletteIndex = IndexOfSpritePaletteTag(10269u16);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FreePokeblockGfx(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        FreeSpriteTilesByTag(10269u16);
        FreeSpritePaletteByTag(10269u16);
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokeBlock_Throw(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 0u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(30i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((GetBattlerSpriteCoord(GetBattlerAtPosition(1u8), 0u8)) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(GetBattlerAtPosition(1u8), 1u8)) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write((-32i16));
        InitAnimArcTranslation(sprite);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_TrainerThrowObject));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_PokeBlock_LiftArm));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokeBlock_LiftArm(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(43))
        .read()) as i32)
            == 1i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_PokeBlock_Arc));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokeBlock_Arc(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ThrowPokeBlock_Free));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ThrowPokeBlock_Free(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(63),
            4,
            1,
            false,
        ) as u16)
            != 0
        {
            if (({
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > 0i32
            {
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    0u8,
                );
                DestroyAnimSprite(sprite);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetAttackerTargetLeftPos(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gBattleAnimAttacker).cast::<u8>()).write(GetBattlerAtPosition(0u8));
                ((&raw mut gBattleAnimTarget).cast::<u8>()).write(GetBattlerAtPosition(1u8));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut gBattleAnimAttacker).cast::<u8>()).write(GetBattlerAtPosition(1u8));
                ((&raw mut gBattleAnimTarget).cast::<u8>()).write(GetBattlerAtPosition(0u8));
                break 'l1;
            }
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetTrappedMoveAnimId(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u16>())
        .read()) as i32)
            == 83i32
        {
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(1i16);
        } else {
            if ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .cast::<u16>())
            .read()) as i32)
                == 250i32
            {
                (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(2i16);
            } else {
                if ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .cast::<u16>())
                .read()) as i32)
                    == 128i32
                {
                    (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(3i16);
                } else {
                    if ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u16>())
                    .read()) as i32)
                        == 328i32
                    {
                        (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(4i16);
                    } else {
                        (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(0i16);
                    }
                }
            }
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetBattlersFromArg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gBattleAnimAttacker).cast::<u8>()).write(
            ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .cast::<u16>())
            .read()) as u8),
        );
        ((&raw mut gBattleAnimTarget).cast::<u8>()).write(
            ((((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .cast::<u16>())
            .read()) as i32)
                >> 8) as u8),
        );
        DestroyAnimVisualTask(taskId);
    }
}
