//! Translated from `src/battle_controller_wally.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sWallyBufferCommands
#[allow(unused_imports)]
use crate::data::battle_controller_wally::*;

unsafe extern "C" {
    static mut gActionSelectionCursor: u8;
    static mut gActiveBattler: u8;
    static mut gAnimDisableStructPtr: u8;
    static mut gAnimFriendship: u8;
    static mut gAnimMoveDmg: u8;
    static mut gAnimMovePower: u8;
    static mut gAnimMoveTurn: u8;
    static mut gAnimScriptActive: u8;
    static mut gAnimScriptCallback: u8;
    static mut gBattleBufferA: u8;
    static mut gBattleControllerData: u8;
    static mut gBattleControllerExecFlags: u8;
    static mut gBattleMonForms: u8;
    static mut gBattleOutcome: u8;
    static mut gBattleSpritesDataPtr: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattle_BG0_X: u8;
    static mut gBattle_BG0_Y: u8;
    static mut gBattlerControllerFuncs: u8;
    static mut gBattlerInMenuId: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gBattlerStatusSummaryTaskId: u8;
    static mut gBitTable: u8;
    static mut gDisplayedStringBattle: u8;
    static mut gDoingBattleAnim: u8;
    static mut gHealthboxSpriteIds: u8;
    static mut gIntroSlideFlags: u8;
    static mut gMain: u8;
    static mut gMultiuseSpriteTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gText_BattleMenu: u8;
    static mut gText_WhatWillWallyDo: u8;
    static mut gTrainerBackPicCoords: u8;
    static mut gTrainerBackPicPaletteTable: u8;
    static mut gTransformedPersonalities: u8;
    static mut gWeatherMoveAnim: u8;
    fn ActionSelectionCreateCursorAt(a0: u8, a1: u8);
    fn ActionSelectionDestroyCursorAt(a0: u8);
    fn AllocSpritePalette(a0: u16) -> u8;
    fn BattleControllerDummy();
    fn BattleMainCB2();
    fn BattlePutTextOnWindow(a0: *mut u8, a1: u8);
    fn BattleStopLowHpSound();
    fn BattleStringExpandPlaceholdersToDisplayedString(a0: *mut u8) -> u32;
    fn BeginFastPaletteFade(a0: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BtlController_EmitDataTransfer(a0: u8, a1: u16, a2: *mut u8);
    fn BtlController_EmitOneReturnValue(a0: u8, a1: u16);
    fn BtlController_EmitTwoReturnValues(a0: u8, a1: u8, a2: u16);
    fn BufferStringBattle(a0: u16);
    fn CopyAllBattleSpritesInvisibilities();
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut u8)>) -> u8;
    fn CreatePartyStatusSummarySprites(a0: u8, a1: *mut u8, a2: u8, a3: u8) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressTrainerBackPic(a0: u16, a1: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DoHitAnimHealthboxEffect(a0: u8);
    fn DoMoveAnim(a0: u16);
    fn DoPokeballSendOutAnimation(a0: i16, a1: u8) -> u8;
    fn DoWallyTutorialBagMenu();
    fn FadeOutMapMusic(a0: u8);
    fn FreeAllWindowBuffers();
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteDefault_Y(a0: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn HandleIntroSlide(a0: u8);
    fn HandleLowHpMusicChange(a0: *mut u8, a1: u8);
    fn InitAndLaunchSpecialAnimation(a0: u8, a1: u8, a2: u8, a3: u8);
    fn InitMoveSelectionsVarsAndStrings();
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsDoubleBattle() -> u8;
    fn IsMoveWithoutAnimation(a0: u16, a1: u8) -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LoadBattleBarGfx(a0: u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn MoveBattleBar(a0: u8, a1: u8, a2: u8, a3: u8) -> i32;
    fn PlayBGM(a0: u16);
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn PlayerHandleGetRawMonData();
    fn PrepareBufferDataTransferLink(a0: u8, a1: u16, a2: *mut u8);
    fn ReshowBattleScreenDummy();
    fn SetBattleBarStruct(a0: u8, a1: u8, a2: i32, a3: i32, a4: i32);
    fn SetBattleEndCallbacks();
    fn SetBattlerSpriteAffineMode(a0: u8);
    fn SetHealthboxSpriteInvisible(a0: u8);
    fn SetHealthboxSpriteVisible(a0: u8);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetMultiuseSpriteTemplateToTrainerBack(a0: u16, a1: u8);
    fn SetSpritePrimaryCoordsFromSecondaryCoords(a0: *mut u8);
    fn SpriteCB_FreePlayerSpriteLoadMonSprite(a0: *mut u8);
    fn SpriteCB_TrainerSlideIn(a0: *mut u8);
    fn SpriteCB_WaitForBattlerBallReleaseAnim(a0: *mut u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartHealthboxSlideIn(a0: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_HidePartyStatusSummary(a0: u8);
    fn Task_PlayerController_RestoreBgmAfterCry(a0: u8);
    fn TryHandleLaunchBattleTableAnimation(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn TrySetBehindSubstituteSpriteBit(a0: u8, a1: u16);
    fn TryShinyAnimation(a0: u8, a1: *mut u8);
    fn UpdateHealthboxAttribute(a0: u8, a1: *mut u8, a2: u8);
    fn UpdateHpTextInHealthbox(a0: u8, a1: i16, a2: u8);
}

pub(crate) unsafe extern "C" fn SpriteCB_Null7() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetControllerToWally() {
    unsafe {
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(WallyBufferRunCommand));
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(148)).write(0u8);
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(149)).write(0u8);
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(150)).write(0u8);
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(151)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn WallyBufferRunCommand() {
    unsafe {
        if (((&raw mut gBattleControllerExecFlags).cast::<u32>()).read()
            & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read())
            != 0
        {
            if ((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .read()) as u32)
                < crate::c::div_u32(228u32, 4u32)
            {
                (((((&raw const sWallyBufferCommands)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(
                    ((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .read()) as i32) as isize,
                ))
                .read())
                .unwrap_unchecked()();
            } else {
                WallyBufferExecCompleted();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WallyHandleActions() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(148))
                .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(150))
                    .write(64u8);
                let __p2 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(148);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if (({
                    let __p3 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(150);
                    let __t4 = ((__p3).read()).wrapping_sub(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == 0i32
                {
                    PlaySE(5u16);
                    BtlController_EmitTwoReturnValues(1u8, 0u8, 0u16);
                    WallyBufferExecCompleted();
                    let __p5 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(148);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(149))
                        .write(0u8);
                    ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(150))
                        .write(64u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (({
                    let __p6 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(150);
                    let __t7 = ((__p6).read()).wrapping_sub(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    == 0i32
                {
                    PlaySE(5u16);
                    BtlController_EmitTwoReturnValues(1u8, 0u8, 0u16);
                    WallyBufferExecCompleted();
                    let __p8 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(148);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                    ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(149))
                        .write(0u8);
                    ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(150))
                        .write(64u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (({
                    let __p9 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(150);
                    let __t10 = ((__p9).read()).wrapping_sub(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    == 0i32
                {
                    BtlController_EmitTwoReturnValues(1u8, 9u8, 0u16);
                    WallyBufferExecCompleted();
                    let __p11 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(148);
                    (__p11).write(((__p11).read()).wrapping_add(1));
                    ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(149))
                        .write(0u8);
                    ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(150))
                        .write(64u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if (({
                    let __p12 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(150);
                    let __t13 = ((__p12).read()).wrapping_sub(1);
                    (__p12).write(__t13);
                    __t13
                }) as i32)
                    == 0i32
                {
                    PlaySE(5u16);
                    ActionSelectionDestroyCursorAt(0u8);
                    ActionSelectionCreateCursorAt(1u8, 0u8);
                    ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(150))
                        .write(64u8);
                    let __p14 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(148);
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                if (({
                    let __p15 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(150);
                    let __t16 = ((__p15).read()).wrapping_sub(1);
                    (__p15).write(__t16);
                    __t16
                }) as i32)
                    == 0i32
                {
                    PlaySE(5u16);
                    BtlController_EmitTwoReturnValues(1u8, 1u8, 0u16);
                    WallyBufferExecCompleted();
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnBattlerSpriteCallbackDummy() {
    unsafe {
        if core::mem::transmute::<_, usize>(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) == (SpriteCallbackDummy as *const () as usize)
        {
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnInactiveTextPrinter() {
    unsafe {
        if !((IsTextPrinterActive(0u8)) != 0) {
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnFinishedAnimation() {
    unsafe {
        if !((((&raw mut gDoingBattleAnim).cast::<u8>()).read()) != 0) {
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn OpenBagAfterPaletteFade() {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(CompleteOnChosenItem));
            ReshowBattleScreenDummy();
            FreeAllWindowBuffers();
            DoWallyTutorialBagMenu();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnChosenItem() {
    unsafe {
        if (core::mem::transmute::<_, usize>(
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        ) == (BattleMainCB2 as *const () as usize))
            && (!((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0))
        {
            BtlController_EmitOneReturnValue(
                1u8,
                ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            );
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn Intro_TryShinyAnimShowHealthbox() {
    unsafe {
        if (!((crate::c::bf_read(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(0),
            7,
            1,
            false,
        ) as u8)
            != 0))
            && (!((crate::c::bf_read(
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
                false,
            ) as u8)
                != 0))
        {
            TryShinyAnimation(
                ((&raw mut gActiveBattler).cast::<u8>()).read(),
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32) as isize
                        * 100,
                ),
            );
        }
        if (!((crate::c::bf_read(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32) as isize * 12,
            ))
            .wrapping_add(0),
            7,
            1,
            false,
        ) as u8)
            != 0))
            && (!((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32) as isize
                        * 12,
                ))
                .wrapping_add(0),
                3,
                1,
                false,
            ) as u8)
                != 0))
        {
            TryShinyAnimation(
                ((((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32)
                                as isize,
                        ))
                    .read()) as i32) as isize
                        * 100,
                ),
            );
        }
        if (((!((crate::c::bf_read(
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
            false,
        ) as u8)
            != 0))
            && (!((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32) as isize
                        * 12,
                ))
                .wrapping_add(0),
                3,
                1,
                false,
            ) as u8)
                != 0)))
            && (core::mem::transmute::<_, usize>(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattleControllerData).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .read(),
            ) == (SpriteCallbackDummy as *const () as usize)))
            && (core::mem::transmute::<_, usize>(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .read(),
            ) == (SpriteCallbackDummy as *const () as usize))
        {
            if ((IsDoubleBattle()) != 0)
                && (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0))
            {
                DestroySprite(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattleControllerData).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32)
                                as isize,
                        ))
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                UpdateHealthboxAttribute(
                    (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32)
                            as isize,
                    ))
                    .read(),
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    0u8,
                );
                StartHealthboxSlideIn(
                    ((((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
                );
                SetHealthboxSpriteVisible(
                    (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32)
                            as isize,
                    ))
                    .read(),
                );
            }
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattleControllerData).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            UpdateHealthboxAttribute(
                (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32) as isize
                        * 100,
                ),
                0u8,
            );
            StartHealthboxSlideIn(((&raw mut gActiveBattler).cast::<u8>()).read());
            SetHealthboxSpriteVisible(
                (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
            );
            crate::c::bf_write(
                (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(9),
                0,
                1,
                (0u8) as i32,
            );
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(Intro_WaitForShinyAnimAndHealthbox));
        }
    }
}
pub(crate) unsafe extern "C" fn Intro_WaitForShinyAnimAndHealthbox() {
    unsafe {
        let mut healthboxAnimDone: u32 = 0u32;
        if core::mem::transmute::<_, usize>(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) == (SpriteCallbackDummy as *const () as usize)
        {
            healthboxAnimDone = 1u32;
        }
        if (((healthboxAnimDone) != 0)
            && ((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(1),
                0,
                1,
                false,
            ) as u8)
                != 0))
            && ((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32) as isize
                        * 12,
                ))
                .wrapping_add(1),
                0,
                1,
                false,
            ) as u8)
                != 0)
        {
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(0),
                7,
                1,
                (0u8) as i32,
            );
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(1),
                0,
                1,
                (0u8) as i32,
            );
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32) as isize
                        * 12,
                ))
                .wrapping_add(0),
                7,
                1,
                (0u8) as i32,
            );
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32) as isize
                        * 12,
                ))
                .wrapping_add(1),
                0,
                1,
                (0u8) as i32,
            );
            FreeSpriteTilesByTag(10233u16);
            FreeSpritePaletteByTag(10233u16);
            CreateTask(Some(Task_PlayerController_RestoreBgmAfterCry), 10u8);
            HandleLowHpMusicChange(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32) as isize
                        * 100,
                ),
                ((&raw mut gActiveBattler).cast::<u8>()).read(),
            );
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnHealthbarDone() {
    unsafe {
        let mut hpValue: i16 = ((MoveBattleBar(
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
            0u8,
            0u8,
        )) as i16);
        SetHealthboxSpriteVisible(
            (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        if ((hpValue) as i32) != (-1i32) {
            UpdateHpTextInHealthbox(
                (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
                hpValue,
                0u8,
            );
        } else {
            HandleLowHpMusicChange(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32) as isize
                        * 100,
                ),
                ((&raw mut gActiveBattler).cast::<u8>()).read(),
            );
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn DoHitAnimBlinkSpriteEffect() {
    unsafe {
        let mut spriteId: u8 = (((&raw mut gBattlerSpriteIds).cast::<u8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .read();
        if ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 32i32
        {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            ((&raw mut gDoingBattleAnim).cast::<u8>()).write(0u8);
            WallyBufferExecCompleted();
        } else {
            if crate::c::rem_i32(
                ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32),
                4i32,
            ) == 0i32
            {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(62),
                    2,
                    1,
                    ((((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        false,
                    ) as u16) as i32)
                        ^ 1i32) as u16) as i32,
                );
            }
            let __p1 = (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn DoSwitchOutAnimation() {
    unsafe {
        if !((crate::c::bf_read(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(0),
            6,
            1,
            false,
        ) as u8)
            != 0)
        {
            FreeSpriteOamMatrix(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            SetHealthboxSpriteInvisible(
                (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
            );
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnBankSpriteCallbackDummy2() {
    unsafe {
        if core::mem::transmute::<_, usize>(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) == (SpriteCallbackDummy as *const () as usize)
        {
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnFinishedBattleAnimation() {
    unsafe {
        if !((crate::c::bf_read(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(0),
            5,
            1,
            false,
        ) as u8)
            != 0)
        {
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn WallyBufferExecCompleted() {
    unsafe {
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(WallyBufferRunCommand));
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
            let mut playerId: u8 = GetMultiplayerId();
            PrepareBufferDataTransferLink(2u8, 4u16, &raw mut playerId);
            ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .write(56u8);
        } else {
            let __p1 = (&raw mut gBattleControllerExecFlags).cast::<u32>();
            (__p1).write(
                ((__p1).read()
                    & !(((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read())),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnFinishedStatusAnimation() {
    unsafe {
        if !((crate::c::bf_read(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(0),
            4,
            1,
            false,
        ) as u8)
            != 0)
        {
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn WallyHandleGetMonData() {
    unsafe {
        let mut monData = crate::ffi::Align4([0u8; 256]);
        let mut size: u32 = 0u32;
        let mut monToCheck: u8 = 0u8;
        let mut i: i32 = 0i32;
        if (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(2))
        .read()) as i32)
            == 0i32
        {
            size = (size).wrapping_add(CopyWallyMonData(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as u8),
                (&raw mut monData).cast::<u8>(),
            ));
        } else {
            monToCheck = (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read();
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((monToCheck) as i32) & 1i32) != 0 {
                            size = (size).wrapping_add(CopyWallyMonData(
                                ((i) as u8),
                                ((&raw mut monData).cast::<u8>())
                                    .wrapping_offset(((size) as i32) as isize),
                            ));
                        }
                        monToCheck = ((((monToCheck) as i32) >> 1) as u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        BtlController_EmitDataTransfer(1u8, ((size) as u16), (&raw mut monData).cast::<u8>());
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CopyWallyMonData(monId: u8, dst: *mut u8) -> u32 {
    unsafe {
        let mut monId = monId;
        let mut dst = dst;
        let mut battleMon = crate::ffi::Align4([0u8; 88]);
        let mut moveData = crate::ffi::Align4([0u8; 16]);
        let mut nickname = crate::ffi::Align4([0u8; 20]);
        let mut src: *mut u8 = core::ptr::null_mut();
        let mut data16: i16 = 0i16;
        let mut data32: u32 = 0u32;
        let mut size: i32 = 0i32;
        'l1: {
            let __sw1 = (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32);
            if __sw1 == 0i32 {
                (((&raw mut battleMon).cast::<u8>()).cast::<u16>()).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        11i32,
                    )) as u16),
                );
                (((&raw mut battleMon).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        12i32,
                    )) as u16),
                );
                {
                    size = 0i32;
                    'l2: loop {
                        if !(size < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            (((((&raw mut battleMon).cast::<u8>()).wrapping_add(12))
                                .cast::<u16>())
                            .wrapping_offset((size) as isize))
                            .write(
                                ((GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((monId) as i32) as isize * 100),
                                    (13i32).wrapping_add(size),
                                )) as u16),
                            );
                            (((((&raw mut battleMon).cast::<u8>()).wrapping_add(36)).cast::<u8>())
                                .wrapping_offset((size) as isize))
                            .write(
                                ((GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((monId) as i32) as isize * 100),
                                    (17i32).wrapping_add(size),
                                )) as u8),
                            );
                        }
                        size = (size).wrapping_add(1);
                    }
                }
                (((&raw mut battleMon).cast::<u8>()).wrapping_add(59)).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        21i32,
                    )) as u8),
                );
                (((&raw mut battleMon).cast::<u8>()).wrapping_add(43)).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        32i32,
                    )) as u8),
                );
                (((&raw mut battleMon).cast::<u8>())
                    .wrapping_add(68)
                    .cast::<u32>())
                .write(GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    25i32,
                ));
                crate::c::bf_write(
                    ((&raw mut battleMon).cast::<u8>()).wrapping_add(20),
                    0,
                    5,
                    (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        39i32,
                    )) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut battleMon).cast::<u8>()).wrapping_add(20),
                    5,
                    5,
                    (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        40i32,
                    )) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut battleMon).cast::<u8>()).wrapping_add(21),
                    2,
                    5,
                    (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        41i32,
                    )) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut battleMon).cast::<u8>()).wrapping_add(21),
                    7,
                    5,
                    (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        42i32,
                    )) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut battleMon).cast::<u8>()).wrapping_add(22),
                    4,
                    5,
                    (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        43i32,
                    )) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut battleMon).cast::<u8>()).wrapping_add(23),
                    1,
                    5,
                    (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        44i32,
                    )) as i32,
                );
                (((&raw mut battleMon).cast::<u8>())
                    .wrapping_add(72)
                    .cast::<u32>())
                .write(GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    0i32,
                ));
                (((&raw mut battleMon).cast::<u8>())
                    .wrapping_add(76)
                    .cast::<u32>())
                .write(GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    55i32,
                ));
                (((&raw mut battleMon).cast::<u8>()).wrapping_add(42)).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        56i32,
                    )) as u8),
                );
                (((&raw mut battleMon).cast::<u8>())
                    .wrapping_add(40)
                    .cast::<u16>())
                .write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        57i32,
                    )) as u16),
                );
                (((&raw mut battleMon).cast::<u8>())
                    .wrapping_add(44)
                    .cast::<u16>())
                .write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        58i32,
                    )) as u16),
                );
                (((&raw mut battleMon).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        59i32,
                    )) as u16),
                );
                (((&raw mut battleMon).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>())
                .write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        60i32,
                    )) as u16),
                );
                (((&raw mut battleMon).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        61i32,
                    )) as u16),
                );
                (((&raw mut battleMon).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<u16>())
                .write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        62i32,
                    )) as u16),
                );
                (((&raw mut battleMon).cast::<u8>())
                    .wrapping_add(10)
                    .cast::<u16>())
                .write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        63i32,
                    )) as u16),
                );
                crate::c::bf_write(
                    ((&raw mut battleMon).cast::<u8>()).wrapping_add(23),
                    6,
                    1,
                    (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        45i32,
                    )) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut battleMon).cast::<u8>()).wrapping_add(23),
                    7,
                    1,
                    (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        46i32,
                    )) as i32,
                );
                (((&raw mut battleMon).cast::<u8>())
                    .wrapping_add(84)
                    .cast::<u32>())
                .write(GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    1i32,
                ));
                GetMonData3(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    2i32,
                    (&raw mut nickname).cast::<u8>(),
                );
                StringCopy_Nickname(
                    (((&raw mut battleMon).cast::<u8>()).wrapping_add(48)).cast::<u8>(),
                    (&raw mut nickname).cast::<u8>(),
                );
                GetMonData3(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    7i32,
                    (((&raw mut battleMon).cast::<u8>()).wrapping_add(60)).cast::<u8>(),
                );
                src = (&raw mut battleMon).cast::<u8>();
                {
                    size = 0i32;
                    'l4: loop {
                        if !(((size) as u32) < 88u32) {
                            break 'l4;
                        }
                        'l5: {
                            ((dst).wrapping_offset((size) as isize))
                                .write(((src).wrapping_offset((size) as isize)).read());
                        }
                        size = (size).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                data16 = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    11i32,
                )) as i16);
                (dst).write(((data16) as u8));
                ((dst).wrapping_offset(1)).write(((((data16) as i32) >> 8) as u8));
                size = 2i32;
                break 'l1;
            }
            if __sw1 == 2i32 {
                data16 = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    12i32,
                )) as i16);
                (dst).write(((data16) as u8));
                ((dst).wrapping_offset(1)).write(((((data16) as i32) >> 8) as u8));
                size = 2i32;
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    size = 0i32;
                    'l6: loop {
                        if !(size < 4i32) {
                            break 'l6;
                        }
                        'l7: {
                            ((((&raw mut moveData).cast::<u8>()).cast::<u16>())
                                .wrapping_offset((size) as isize))
                            .write(
                                ((GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((monId) as i32) as isize * 100),
                                    (13i32).wrapping_add(size),
                                )) as u16),
                            );
                            (((((&raw mut moveData).cast::<u8>()).wrapping_add(8)).cast::<u8>())
                                .wrapping_offset((size) as isize))
                            .write(
                                ((GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((monId) as i32) as isize * 100),
                                    (17i32).wrapping_add(size),
                                )) as u8),
                            );
                        }
                        size = (size).wrapping_add(1);
                    }
                }
                (((&raw mut moveData).cast::<u8>()).wrapping_add(12)).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        21i32,
                    )) as u8),
                );
                src = (&raw mut moveData).cast::<u8>();
                {
                    size = 0i32;
                    'l8: loop {
                        if !(((size) as u32) < 16u32) {
                            break 'l8;
                        }
                        'l9: {
                            ((dst).wrapping_offset((size) as isize))
                                .write(((src).wrapping_offset((size) as isize)).read());
                        }
                        size = (size).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 || __sw1 == 5i32 || __sw1 == 6i32 || __sw1 == 7i32 {
                data16 = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    ((13i32).wrapping_add(
                        (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 512,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    ))
                    .wrapping_sub(4i32),
                )) as i16);
                (dst).write(((data16) as u8));
                ((dst).wrapping_offset(1)).write(((((data16) as i32) >> 8) as u8));
                size = 2i32;
                break 'l1;
            }
            if __sw1 == 8i32 {
                {
                    size = 0i32;
                    'l10: loop {
                        if !(size < 4i32) {
                            break 'l10;
                        }
                        'l11: {
                            ((dst).wrapping_offset((size) as isize)).write(
                                ((GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((monId) as i32) as isize * 100),
                                    (17i32).wrapping_add(size),
                                )) as u8),
                            );
                        }
                        size = (size).wrapping_add(1);
                    }
                }
                ((dst).wrapping_offset((size) as isize)).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        21i32,
                    )) as u8),
                );
                size = (size).wrapping_add(1);
                break 'l1;
            }
            if __sw1 == 9i32 || __sw1 == 10i32 || __sw1 == 11i32 || __sw1 == 12i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        ((17i32).wrapping_add(
                            (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 512,
                            ))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32),
                        ))
                        .wrapping_sub(9i32),
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 17i32 {
                data32 = GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    1i32,
                );
                (dst).write(((data32 & 255u32) as u8));
                ((dst).wrapping_offset(1)).write((((data32 & 65280u32) >> 8) as u8));
                ((dst).wrapping_offset(2)).write((((data32 & 16711680u32) >> 16) as u8));
                size = 3i32;
                break 'l1;
            }
            if __sw1 == 18i32 {
                data32 = GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    25i32,
                );
                (dst).write(((data32 & 255u32) as u8));
                ((dst).wrapping_offset(1)).write((((data32 & 65280u32) >> 8) as u8));
                ((dst).wrapping_offset(2)).write((((data32 & 16711680u32) >> 16) as u8));
                size = 3i32;
                break 'l1;
            }
            if __sw1 == 19i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        26i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 20i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        27i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 21i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        28i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 22i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        29i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 23i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        30i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 24i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        31i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 25i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        32i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 26i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        34i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 27i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        35i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 28i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        36i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 29i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        37i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 30i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        38i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 31i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        39i32,
                    )) as u8),
                );
                ((dst).wrapping_offset(1)).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        40i32,
                    )) as u8),
                );
                ((dst).wrapping_offset(2)).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        41i32,
                    )) as u8),
                );
                ((dst).wrapping_offset(3)).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        42i32,
                    )) as u8),
                );
                ((dst).wrapping_offset(4)).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        43i32,
                    )) as u8),
                );
                ((dst).wrapping_offset(5)).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        44i32,
                    )) as u8),
                );
                size = 6i32;
                break 'l1;
            }
            if __sw1 == 32i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        39i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 33i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        40i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 34i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        41i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 35i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        42i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 36i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        43i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 37i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        44i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 38i32 {
                data32 = GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    0i32,
                );
                (dst).write(((data32 & 255u32) as u8));
                ((dst).wrapping_offset(1)).write((((data32 & 65280u32) >> 8) as u8));
                ((dst).wrapping_offset(2)).write((((data32 & 16711680u32) >> 16) as u8));
                ((dst).wrapping_offset(3)).write((((data32 & 4278190080u32) >> 24) as u8));
                size = 4i32;
                break 'l1;
            }
            if __sw1 == 39i32 {
                data16 = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    9i32,
                )) as i16);
                (dst).write(((data16) as u8));
                ((dst).wrapping_offset(1)).write(((((data16) as i32) >> 8) as u8));
                size = 2i32;
                break 'l1;
            }
            if __sw1 == 40i32 {
                data32 = GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    55i32,
                );
                (dst).write(((data32 & 255u32) as u8));
                ((dst).wrapping_offset(1)).write((((data32 & 65280u32) >> 8) as u8));
                ((dst).wrapping_offset(2)).write((((data32 & 16711680u32) >> 16) as u8));
                ((dst).wrapping_offset(3)).write((((data32 & 4278190080u32) >> 24) as u8));
                size = 4i32;
                break 'l1;
            }
            if __sw1 == 41i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        56i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 42i32 {
                data16 = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    57i32,
                )) as i16);
                (dst).write(((data16) as u8));
                ((dst).wrapping_offset(1)).write(((((data16) as i32) >> 8) as u8));
                size = 2i32;
                break 'l1;
            }
            if __sw1 == 43i32 {
                data16 = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    58i32,
                )) as i16);
                (dst).write(((data16) as u8));
                ((dst).wrapping_offset(1)).write(((((data16) as i32) >> 8) as u8));
                size = 2i32;
                break 'l1;
            }
            if __sw1 == 44i32 {
                data16 = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    59i32,
                )) as i16);
                (dst).write(((data16) as u8));
                ((dst).wrapping_offset(1)).write(((((data16) as i32) >> 8) as u8));
                size = 2i32;
                break 'l1;
            }
            if __sw1 == 45i32 {
                data16 = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    60i32,
                )) as i16);
                (dst).write(((data16) as u8));
                ((dst).wrapping_offset(1)).write(((((data16) as i32) >> 8) as u8));
                size = 2i32;
                break 'l1;
            }
            if __sw1 == 46i32 {
                data16 = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    61i32,
                )) as i16);
                (dst).write(((data16) as u8));
                ((dst).wrapping_offset(1)).write(((((data16) as i32) >> 8) as u8));
                size = 2i32;
                break 'l1;
            }
            if __sw1 == 47i32 {
                data16 = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    62i32,
                )) as i16);
                (dst).write(((data16) as u8));
                ((dst).wrapping_offset(1)).write(((((data16) as i32) >> 8) as u8));
                size = 2i32;
                break 'l1;
            }
            if __sw1 == 48i32 {
                data16 = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    63i32,
                )) as i16);
                (dst).write(((data16) as u8));
                ((dst).wrapping_offset(1)).write(((((data16) as i32) >> 8) as u8));
                size = 2i32;
                break 'l1;
            }
            if __sw1 == 49i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        22i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 50i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        23i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 51i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        24i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 52i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        33i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 53i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        47i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 54i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        48i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 55i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        50i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 56i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        51i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 57i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        52i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 58i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        53i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
            if __sw1 == 59i32 {
                (dst).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        54i32,
                    )) as u8),
                );
                size = 1i32;
                break 'l1;
            }
        }
        return ((size) as u32);
    }
}
pub(crate) unsafe extern "C" fn WallyHandleGetRawMonData() {
    unsafe {
        PlayerHandleGetRawMonData();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleSetMonData() {
    unsafe {
        let mut monToCheck: u8 = 0u8;
        let mut i: u8 = 0u8;
        if (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(2))
        .read()) as i32)
            == 0i32
        {
            SetWallyMonData(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as u8),
            );
        } else {
            monToCheck = (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read();
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((monToCheck) as i32) & 1i32) != 0 {
                            SetWallyMonData(i);
                        }
                        monToCheck = ((((monToCheck) as i32) >> 1) as u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SetWallyMonData(monId: u8) {
    unsafe {
        let mut monId = monId;
        let mut battlePokemon: *mut u8 = ((((&raw mut gBattleBufferA).cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
        .cast::<u8>())
        .wrapping_offset(3);
        let mut moveData: *mut u8 = ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(3);
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32);
            if __sw1 == 0i32 {
                {
                    let mut iv: u8 = 0u8;
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        11i32,
                        ((battlePokemon).cast::<u16>()).cast::<u8>(),
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        12i32,
                        ((battlePokemon).wrapping_add(46).cast::<u16>()).cast::<u8>(),
                    );
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 4i32) {
                                break 'l2;
                            }
                            'l3: {
                                SetMonData(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((monId) as i32) as isize * 100),
                                    (13i32).wrapping_add(i),
                                    ((((battlePokemon).wrapping_add(12)).cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                    .cast::<u8>(),
                                );
                                SetMonData(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((monId) as i32) as isize * 100),
                                    (17i32).wrapping_add(i),
                                    (((battlePokemon).wrapping_add(36)).cast::<u8>())
                                        .wrapping_offset((i) as isize),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        21i32,
                        (battlePokemon).wrapping_add(59),
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        32i32,
                        (battlePokemon).wrapping_add(43),
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        25i32,
                        ((battlePokemon).wrapping_add(68).cast::<u32>()).cast::<u8>(),
                    );
                    iv = ((crate::c::bf_read((battlePokemon).wrapping_add(20), 0, 5, false) as u32)
                        as u8);
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        39i32,
                        &raw mut iv,
                    );
                    iv = ((crate::c::bf_read((battlePokemon).wrapping_add(20), 5, 5, false) as u32)
                        as u8);
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        40i32,
                        &raw mut iv,
                    );
                    iv = ((crate::c::bf_read((battlePokemon).wrapping_add(21), 2, 5, false) as u32)
                        as u8);
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        41i32,
                        &raw mut iv,
                    );
                    iv = ((crate::c::bf_read((battlePokemon).wrapping_add(21), 7, 5, false) as u32)
                        as u8);
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        42i32,
                        &raw mut iv,
                    );
                    iv = ((crate::c::bf_read((battlePokemon).wrapping_add(22), 4, 5, false) as u32)
                        as u8);
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        43i32,
                        &raw mut iv,
                    );
                    iv = ((crate::c::bf_read((battlePokemon).wrapping_add(23), 1, 5, false) as u32)
                        as u8);
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        44i32,
                        &raw mut iv,
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        0i32,
                        ((battlePokemon).wrapping_add(72).cast::<u32>()).cast::<u8>(),
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        55i32,
                        ((battlePokemon).wrapping_add(76).cast::<u32>()).cast::<u8>(),
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        56i32,
                        (battlePokemon).wrapping_add(42),
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        57i32,
                        ((battlePokemon).wrapping_add(40).cast::<u16>()).cast::<u8>(),
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        58i32,
                        ((battlePokemon).wrapping_add(44).cast::<u16>()).cast::<u8>(),
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        59i32,
                        ((battlePokemon).wrapping_add(2).cast::<u16>()).cast::<u8>(),
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        60i32,
                        ((battlePokemon).wrapping_add(4).cast::<u16>()).cast::<u8>(),
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        61i32,
                        ((battlePokemon).wrapping_add(6).cast::<u16>()).cast::<u8>(),
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        62i32,
                        ((battlePokemon).wrapping_add(8).cast::<u16>()).cast::<u8>(),
                    );
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        63i32,
                        ((battlePokemon).wrapping_add(10).cast::<u16>()).cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    11i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    12i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i < 4i32) {
                            break 'l4;
                        }
                        'l5: {
                            SetMonData(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((monId) as i32) as isize * 100),
                                (13i32).wrapping_add(i),
                                (((moveData).cast::<u16>()).wrapping_offset((i) as isize))
                                    .cast::<u8>(),
                            );
                            SetMonData(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((monId) as i32) as isize * 100),
                                (17i32).wrapping_add(i),
                                (((moveData).wrapping_add(8)).cast::<u8>())
                                    .wrapping_offset((i) as isize),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    21i32,
                    (moveData).wrapping_add(12),
                );
                break 'l1;
            }
            if __sw1 == 4i32 || __sw1 == 5i32 || __sw1 == 6i32 || __sw1 == 7i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    ((13i32).wrapping_add(
                        (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 512,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    ))
                    .wrapping_sub(4i32),
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 8i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    17i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    18i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(4),
                );
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    19i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(5),
                );
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    20i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(6),
                );
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    21i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(7),
                );
                break 'l1;
            }
            if __sw1 == 9i32 || __sw1 == 10i32 || __sw1 == 11i32 || __sw1 == 12i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    ((17i32).wrapping_add(
                        (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 512,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    ))
                    .wrapping_sub(9i32),
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 17i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    1i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 18i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    25i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 19i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    26i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 20i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    27i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 21i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    28i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 22i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    29i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 23i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    30i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 24i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    31i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 25i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    32i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 26i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    34i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 27i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    35i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 28i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    36i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 29i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    37i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 30i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    38i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 31i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    39i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    40i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(4),
                );
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    41i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(5),
                );
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    42i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(6),
                );
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    43i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(7),
                );
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    44i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(8),
                );
                break 'l1;
            }
            if __sw1 == 32i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    39i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 33i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    40i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 34i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    41i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 35i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    42i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 36i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    43i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 37i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    44i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 38i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    0i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 39i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    9i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 40i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    55i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 41i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    56i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 42i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    57i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 43i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    58i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 44i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    59i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 45i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    60i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 46i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    61i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 47i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    62i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 48i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    63i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 49i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    22i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 50i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    23i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 51i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    24i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 52i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    33i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 53i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    47i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 54i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    48i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 55i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    50i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 56i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    51i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 57i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    52i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 58i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    53i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
            if __sw1 == 59i32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    54i32,
                    ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(3),
                );
                break 'l1;
            }
        }
        HandleLowHpMusicChange(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32) as isize
                    * 100,
            ),
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn WallyHandleSetRawMonData() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleLoadMonSprite() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleSwitchInAnim() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleReturnMonToBall() {
    unsafe {
        if (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read()) as i32)
            == 0i32
        {
            InitAndLaunchSpecialAnimation(
                ((&raw mut gActiveBattler).cast::<u8>()).read(),
                ((&raw mut gActiveBattler).cast::<u8>()).read(),
                ((&raw mut gActiveBattler).cast::<u8>()).read(),
                1u8,
            );
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(DoSwitchOutAnimation));
        } else {
            FreeSpriteOamMatrix(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            SetHealthboxSpriteInvisible(
                (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
            );
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn WallyHandleDrawTrainerPic() {
    unsafe {
        DecompressTrainerBackPic(6u16, ((&raw mut gActiveBattler).cast::<u8>()).read());
        SetMultiuseSpriteTemplateToTrainerBack(
            6u16,
            GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()),
        );
        (((&raw mut gBattlerSpriteIds).cast::<u8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(CreateSprite(
            (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
            80i16,
            (((80i32).wrapping_add((4i32).wrapping_mul((8i32).wrapping_sub(
                (((((&raw mut gTrainerBackPicCoords).cast::<u8>()).wrapping_offset(24)).read())
                    as i32),
            )))) as i16),
            30u8,
        ));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            4,
            4,
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(36)
        .cast::<i16>())
        .write(240i16);
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .write((-2i16));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_TrainerSlideIn));
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(CompleteOnBattlerSpriteCallbackDummy));
    }
}
pub(crate) unsafe extern "C" fn WallyHandleTrainerSlide() {
    unsafe {
        DecompressTrainerBackPic(6u16, ((&raw mut gActiveBattler).cast::<u8>()).read());
        SetMultiuseSpriteTemplateToTrainerBack(
            6u16,
            GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()),
        );
        (((&raw mut gBattlerSpriteIds).cast::<u8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(CreateSprite(
            (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
            80i16,
            (((80i32).wrapping_add((4i32).wrapping_mul((8i32).wrapping_sub(
                (((((&raw mut gTrainerBackPicCoords).cast::<u8>()).wrapping_offset(24)).read())
                    as i32),
            )))) as i16),
            30u8,
        ));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            4,
            4,
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(36)
        .cast::<i16>())
        .write((-96i16));
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .write(2i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_TrainerSlideIn));
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(CompleteOnBankSpriteCallbackDummy2));
    }
}
pub(crate) unsafe extern "C" fn WallyHandleTrainerSlideBack() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleFaintAnimation() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandlePaletteFade() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleSuccessBallThrowAnim() {
    unsafe {
        ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8))
        .write(4u8);
        ((&raw mut gDoingBattleAnim).cast::<u8>()).write(1u8);
        InitAndLaunchSpecialAnimation(
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            GetBattlerAtPosition(1u8),
            4u8,
        );
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(CompleteOnFinishedAnimation));
    }
}
pub(crate) unsafe extern "C" fn WallyHandleBallThrowAnim() {
    unsafe {
        let mut ballThrowCaseId: u8 = (((((&raw mut gBattleBufferA).cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read();
        ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8))
        .write(ballThrowCaseId);
        ((&raw mut gDoingBattleAnim).cast::<u8>()).write(1u8);
        InitAndLaunchSpecialAnimation(
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            GetBattlerAtPosition(1u8),
            4u8,
        );
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(CompleteOnFinishedAnimation));
    }
}
pub(crate) unsafe extern "C" fn WallyHandlePause() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleMoveAnimation() {
    unsafe {
        let mut r#move: u16 = (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32)
                << 8)) as u16);
        ((&raw mut gAnimMoveTurn).cast::<u8>()).write(
            (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(3))
            .read(),
        );
        ((&raw mut gAnimMovePower).cast::<u16>()).write(
            (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(4))
            .read()) as i32)
                | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(5))
                .read()) as i32)
                    << 8)) as u16),
        );
        ((&raw mut gAnimMoveDmg).cast::<i32>()).write(
            ((((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(6))
            .read()) as i32)
                | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(7))
                .read()) as i32)
                    << 8))
                | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(8))
                .read()) as i32)
                    << 16))
                | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(9))
                .read()) as i32)
                    << 24)),
        );
        ((&raw mut gAnimFriendship).cast::<u8>()).write(
            (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(10))
            .read(),
        );
        ((&raw mut gWeatherMoveAnim).cast::<u16>()).write(
            (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(12))
            .read()) as i32)
                | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(13))
                .read()) as i32)
                    << 8)) as u16),
        );
        ((&raw mut gAnimDisableStructPtr).cast::<*mut u8>()).write(
            ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(16),
        );
        ((((&raw mut gTransformedPersonalities).cast::<u32>()).cast::<u32>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(
            ((((&raw mut gAnimDisableStructPtr).cast::<*mut u8>()).read()).cast::<u32>()).read(),
        );
        if (IsMoveWithoutAnimation(r#move, ((&raw mut gAnimMoveTurn).cast::<u8>()).read())) != 0 {
            WallyBufferExecCompleted();
        } else {
            (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(4))
            .write(0u8);
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(WallyDoMoveAnimation));
        }
    }
}
pub(crate) unsafe extern "C" fn WallyDoMoveAnimation() {
    unsafe {
        let mut r#move: u16 = (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read()) as i32)
            | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32)
                << 8)) as u16);
        'l1: {
            let __sw1 = (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(4))
            .read()) as i32);
            if __sw1 == 0i32 {
                if (crate::c::bf_read(
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 4,
                    ))
                    .wrapping_add(0),
                    2,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    InitAndLaunchSpecialAnimation(
                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                        5u8,
                    );
                }
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(4))
                .write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(0),
                    6,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    SetBattlerSpriteAffineMode(0u8);
                    DoMoveAnim(r#move);
                    (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(4))
                    .write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((&raw mut gAnimScriptCallback).cast::<Option<unsafe extern "C" fn()>>()).read())
                    .unwrap_unchecked()();
                if !((((&raw mut gAnimScriptActive).cast::<u8>()).read()) != 0) {
                    SetBattlerSpriteAffineMode(1u8);
                    if (crate::c::bf_read(
                        ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 4,
                        ))
                        .wrapping_add(0),
                        2,
                        1,
                        false,
                    ) as u16)
                        != 0
                    {
                        InitAndLaunchSpecialAnimation(
                            ((&raw mut gActiveBattler).cast::<u8>()).read(),
                            ((&raw mut gActiveBattler).cast::<u8>()).read(),
                            ((&raw mut gActiveBattler).cast::<u8>()).read(),
                            6u8,
                        );
                    }
                    (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(4))
                    .write(3u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((crate::c::bf_read(
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(0),
                    6,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    CopyAllBattleSpritesInvisibilities();
                    TrySetBehindSubstituteSpriteBit(
                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                        (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 512,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 512,
                            ))
                            .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32)
                                << 8)) as u16),
                    );
                    (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(4))
                    .write(0u8);
                    WallyBufferExecCompleted();
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WallyHandlePrintString() {
    unsafe {
        let mut stringId: *mut u16 = core::ptr::null_mut();
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        stringId = (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(2))
        .cast::<u16>();
        BufferStringBattle((stringId).read());
        BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 0u8);
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(CompleteOnInactiveTextPrinter));
    }
}
pub(crate) unsafe extern "C" fn WallyHandlePrintSelectionString() {
    unsafe {
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
            WallyHandlePrintString();
        } else {
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn HandleChooseActionAfterDma3() {
    unsafe {
        if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(160u16);
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(WallyHandleActions));
        }
    }
}
pub(crate) unsafe extern "C" fn WallyHandleChooseAction() {
    unsafe {
        let mut i: i32 = 0i32;
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(HandleChooseActionAfterDma3));
        BattlePutTextOnWindow((&raw mut gText_BattleMenu).cast::<u8>(), 2u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ActionSelectionDestroyCursorAt(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
        ActionSelectionCreateCursorAt(
            (((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
            0u8,
        );
        BattleStringExpandPlaceholdersToDisplayedString(
            (&raw mut gText_WhatWillWallyDo).cast::<u8>(),
        );
        BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 1u8);
    }
}
pub(crate) unsafe extern "C" fn WallyHandleYesNoBox() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleChooseMove() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(149))
                .read()) as i32);
            if __sw1 == 0i32 {
                InitMoveSelectionsVarsAndStrings();
                let __p2 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(149);
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(151))
                    .write(80u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
                    ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(320u16);
                    let __p3 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(149);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p4 =
                        (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(151);
                    let __t5 = ((__p4).read()).wrapping_sub(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    == 0i32
                {
                    PlaySE(5u16);
                    BtlController_EmitTwoReturnValues(1u8, 10u8, 256u16);
                    WallyBufferExecCompleted();
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WallyHandleChooseItem() {
    unsafe {
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(OpenBagAfterPaletteFade));
        ((&raw mut gBattlerInMenuId).cast::<u8>())
            .write(((&raw mut gActiveBattler).cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn WallyHandleChoosePokemon() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleCmd23() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleHealthBarUpdate() {
    unsafe {
        let mut hpVal: i16 = 0i16;
        LoadBattleBarGfx(0u8);
        hpVal = (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(2))
        .read()) as i32)
            | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(3))
            .read()) as i32)
                << 8)) as i16);
        if ((hpVal) as i32) != 32767i32 {
            let mut maxHP: u32 = GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32) as isize
                        * 100,
                ),
                58i32,
            );
            let mut curHP: u32 = GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32) as isize
                        * 100,
                ),
                57i32,
            );
            SetBattleBarStruct(
                ((&raw mut gActiveBattler).cast::<u8>()).read(),
                (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
                ((maxHP) as i32),
                ((curHP) as i32),
                ((hpVal) as i32),
            );
        } else {
            let mut maxHP: u32 = GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32) as isize
                        * 100,
                ),
                58i32,
            );
            SetBattleBarStruct(
                ((&raw mut gActiveBattler).cast::<u8>()).read(),
                (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
                ((maxHP) as i32),
                0i32,
                ((hpVal) as i32),
            );
            UpdateHpTextInHealthbox(
                (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
                0i16,
                0u8,
            );
        }
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(CompleteOnHealthbarDone));
    }
}
pub(crate) unsafe extern "C" fn WallyHandleExpUpdate() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleStatusIconUpdate() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleStatusAnimation() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleStatusXor() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleDataTransfer() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleDMA3Transfer() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandlePlayBGM() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleCmd32() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleTwoReturnValues() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleChosenMonReturnValue() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleOneReturnValue() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleOneReturnValue_Duplicate() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleClearUnkVar() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleSetUnkVar() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleClearUnkFlag() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleToggleUnkFlag() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleHitAnimation() {
    unsafe {
        if ((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            false,
        ) as u16) as i32)
            == 1i32
        {
            WallyBufferExecCompleted();
        } else {
            ((&raw mut gDoingBattleAnim).cast::<u8>()).write(1u8);
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
            DoHitAnimHealthboxEffect(((&raw mut gActiveBattler).cast::<u8>()).read());
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(DoHitAnimBlinkSpriteEffect));
        }
    }
}
pub(crate) unsafe extern "C" fn WallyHandleCantSwitch() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandlePlaySE() {
    unsafe {
        PlaySE(
            (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(2))
                .read()) as i32)
                    << 8)) as u16),
        );
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandlePlayFanfareOrBGM() {
    unsafe {
        if ((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(3))
        .read())
            != 0
        {
            BattleStopLowHpSound();
            PlayBGM(
                (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 8)) as u16),
            );
        } else {
            PlayFanfare(
                (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 8)) as u16),
            );
        }
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleFaintingCry() {
    unsafe {
        let mut species: u16 = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32) as isize
                    * 100,
            ),
            11i32,
        )) as u16);
        PlayCry_Normal(species, 25i8);
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleIntroSlide() {
    unsafe {
        HandleIntroSlide(
            (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read(),
        );
        let __p1 = (&raw mut gIntroSlideFlags).cast::<u16>();
        (__p1).write((((((__p1).read()) as i32) | 1i32) as u16));
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleIntroTrainerBallThrow() {
    unsafe {
        let mut paletteNum: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        SetSpritePrimaryCoordsFromSecondaryCoords(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .write(50i16);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write((-40i16));
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read(),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i16));
        StoreSpriteCallbackInData6(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ),
            Some(SpriteCB_FreePlayerSpriteLoadMonSprite),
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ),
            1u8,
        );
        paletteNum = AllocSpritePalette(55032u16);
        LoadCompressedPalette(
            ((((&raw mut gTrainerBackPicPaletteTable).cast::<u8>()).wrapping_offset(48))
                .cast::<*mut u32>())
            .read(),
            (((256i32).wrapping_add(((paletteNum) as i32).wrapping_mul(16i32))) as u16),
            32u16,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            4,
            4,
            ((paletteNum) as u16) as i32,
        );
        taskId = CreateTask(Some(Task_StartSendOutAnim), 5u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i16));
        if (crate::c::bf_read(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(0),
            0,
            1,
            false,
        ) as u8)
            != 0
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerStatusSummaryTaskId).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 40,
            ))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HidePartyStatusSummary));
        }
        crate::c::bf_write(
            (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(9),
            0,
            1,
            (1u8) as i32,
        );
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(BattleControllerDummy));
    }
}
pub(crate) unsafe extern "C" fn StartSendOutAnim(battler: u8) {
    unsafe {
        let mut battler = battler;
        let mut species: u16 = 0u16;
        (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read())
        .wrapping_offset(((battler) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .write(0u16);
        ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((battler) as i32) as isize))
        .write(
            (((((((&raw mut gBattleBufferA).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 512))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as u16),
        );
        species = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 100,
            ),
            11i32,
        )) as u16);
        (((&raw mut gBattleControllerData).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize))
        .write(CreateInvisibleSpriteWithCallback(Some(
            SpriteCB_WaitForBattlerBallReleaseAnim,
        )));
        SetMultiuseSpriteTemplateToPokemon(species, GetBattlerPosition(battler));
        (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(((battler) as i32) as isize))
            .write(CreateSprite(
                (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
                ((GetBattlerSpriteCoord(battler, 2u8)) as i16),
                ((GetBattlerSpriteDefault_Y(battler)) as i16),
                GetBattlerSpriteSubpriority(battler),
            ));
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattleControllerData).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattleControllerData).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((battler) as i16));
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .write(((battler) as i16));
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((species) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            4,
            4,
            ((battler) as u16) as i32,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 68,
            ),
            (((&raw mut gBattleMonForms).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read(),
        );
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
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattleControllerData).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .write(((DoPokeballSendOutAnimation(0i16, 255u8)) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_StartSendOutAnim(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            < 31i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            let mut savedActiveBank: u8 = ((&raw mut gActiveBattler).cast::<u8>()).read();
            ((&raw mut gActiveBattler).cast::<u8>()).write(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
            );
            (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .write(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as u8),
            );
            StartSendOutAnim(((&raw mut gActiveBattler).cast::<u8>()).read());
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(Intro_TryShinyAnimShowHealthbox));
            ((&raw mut gActiveBattler).cast::<u8>()).write(savedActiveBank);
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn WallyHandleDrawPartyStatusSummary() {
    unsafe {
        if ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read()) as i32)
            != 0i32)
            && (((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32)
        {
            WallyBufferExecCompleted();
        } else {
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(0),
                0,
                1,
                (1u8) as i32,
            );
            (((&raw mut gBattlerStatusSummaryTaskId).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .write(CreatePartyStatusSummarySprites(
                ((&raw mut gActiveBattler).cast::<u8>()).read(),
                ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(4),
                (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(1))
                .read(),
                (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(2))
                .read(),
            ));
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn WallyHandleHidePartyStatusSummary() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleEndBounceEffect() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleSpriteInvisibility() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleBattleAnimation() {
    unsafe {
        let mut animationId: u8 = (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read();
        let mut argument: u16 = (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(2))
        .read()) as i32)
            | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(3))
            .read()) as i32)
                << 8)) as u16);
        if (TryHandleLaunchBattleTableAnimation(
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            animationId,
            argument,
        )) != 0
        {
            WallyBufferExecCompleted();
        } else {
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(CompleteOnFinishedBattleAnimation));
        }
    }
}
pub(crate) unsafe extern "C" fn WallyHandleLinkStandbyMsg() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleResetActionMoveSelection() {
    unsafe {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleEndLinkBattle() {
    unsafe {
        ((&raw mut gBattleOutcome).cast::<u8>()).write(
            (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read(),
        );
        FadeOutMapMusic(5u8);
        BeginFastPaletteFade(3u8);
        WallyBufferExecCompleted();
        if (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4u32) != 0))
            && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0)
        {
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(SetBattleEndCallbacks));
        }
    }
}
pub(crate) unsafe extern "C" fn WallyCmdEnd() {
    unsafe {}
}
