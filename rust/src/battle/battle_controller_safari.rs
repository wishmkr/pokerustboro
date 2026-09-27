//! Translated from `src/battle_controller_safari.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sSafariBufferCommands
#[allow(unused_imports)]
use crate::data::battle_controller_safari::*;

unsafe extern "C" {
    static mut gActionSelectionCursor: u8;
    static mut gActiveBattler: u8;
    static mut gBattleBufferA: u8;
    static mut gBattleControllerExecFlags: u8;
    static mut gBattleOutcome: u8;
    static mut gBattleSpritesDataPtr: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattle_BG0_X: u8;
    static mut gBattle_BG0_Y: u8;
    static mut gBattlerControllerFuncs: u8;
    static mut gBattlerInMenuId: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gBitTable: u8;
    static mut gDisplayedStringBattle: u8;
    static mut gDoingBattleAnim: u8;
    static mut gHealthboxSpriteIds: u8;
    static mut gIntroSlideFlags: u8;
    static mut gMain: u8;
    static mut gMultiuseSpriteTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPreBattleCallback1: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSprites: u8;
    static mut gText_SafariZoneMenu: u8;
    static mut gText_WhatWillPkmnDo2: u8;
    static mut gTrainerBackPicCoords: u8;
    fn ActionSelectionCreateCursorAt(a0: u8, a1: u8);
    fn ActionSelectionDestroyCursorAt(a0: u8);
    fn BattleMainCB2();
    fn BattlePutTextOnWindow(a0: *mut u8, a1: u8);
    fn BattleStopLowHpSound();
    fn BattleStringExpandPlaceholdersToDisplayedString(a0: *mut u8) -> u32;
    fn BeginFastPaletteFade(a0: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BtlController_EmitOneReturnValue(a0: u8, a1: u16);
    fn BtlController_EmitTwoReturnValues(a0: u8, a1: u8, a2: u16);
    fn BufferStringBattle(a0: u16);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DecompressTrainerBackPic(a0: u16, a1: u8);
    fn FadeOutMapMusic(a0: u8);
    fn FreeAllWindowBuffers();
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMultiplayerId() -> u8;
    fn HandleIntroSlide(a0: u8);
    fn InitAndLaunchSpecialAnimation(a0: u8, a1: u8, a2: u8, a3: u8);
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn OpenPokeblockCaseInBattle();
    fn PlayBGM(a0: u16);
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PrepareBufferDataTransferLink(a0: u8, a1: u16, a2: *mut u8);
    fn SetHealthboxSpriteVisible(a0: u8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMultiuseSpriteTemplateToTrainerBack(a0: u16, a1: u8);
    fn SpriteCB_TrainerSlideIn(a0: *mut u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartHealthboxSlideIn(a0: u8);
    fn TryHandleLaunchBattleTableAnimation(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn UpdateHealthboxAttribute(a0: u8, a1: *mut u8, a2: u8);
}

pub(crate) unsafe extern "C" fn SpriteCB_Null4() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetControllerToSafari() {
    unsafe {
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(SafariBufferRunCommand));
    }
}
pub(crate) unsafe extern "C" fn SafariBufferRunCommand() {
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
                (((((&raw const sSafariBufferCommands)
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
                SafariBufferExecCompleted();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandleInputChooseAction() {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            'l1: {
                let __sw1 = (((((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32);
                if __sw1 == 0i32 {
                    BtlController_EmitTwoReturnValues(1u8, 5u8, 0u16);
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    BtlController_EmitTwoReturnValues(1u8, 6u8, 0u16);
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    BtlController_EmitTwoReturnValues(1u8, 7u8, 0u16);
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    BtlController_EmitTwoReturnValues(1u8, 8u8, 0u16);
                    break 'l1;
                }
            }
            SafariBufferExecCompleted();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 32i32)
                != 0
            {
                if ((((((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    PlaySE(5u16);
                    ActionSelectionDestroyCursorAt(
                        (((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read(),
                    );
                    let __p2 = ((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    );
                    (__p2).write((((((__p2).read()) as i32) ^ 1i32) as u8));
                    ActionSelectionCreateCursorAt(
                        (((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read(),
                        0u8,
                    );
                }
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 16i32)
                    != 0
                {
                    if !(((((((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        & 1i32)
                        != 0)
                    {
                        PlaySE(5u16);
                        ActionSelectionDestroyCursorAt(
                            (((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read(),
                        );
                        let __p3 = ((&raw mut gActionSelectionCursor).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            );
                        (__p3).write((((((__p3).read()) as i32) ^ 1i32) as u8));
                        ActionSelectionCreateCursorAt(
                            (((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read(),
                            0u8,
                        );
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 64i32)
                        != 0
                    {
                        if ((((((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            & 2i32)
                            != 0
                        {
                            PlaySE(5u16);
                            ActionSelectionDestroyCursorAt(
                                (((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                                .read(),
                            );
                            let __p4 = ((&raw mut gActionSelectionCursor).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                );
                            (__p4).write((((((__p4).read()) as i32) ^ 2i32) as u8));
                            ActionSelectionCreateCursorAt(
                                (((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                                .read(),
                                0u8,
                            );
                        }
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 128i32)
                            != 0
                        {
                            if !(((((((&raw mut gActionSelectionCursor).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32)
                                & 2i32)
                                != 0)
                            {
                                PlaySE(5u16);
                                ActionSelectionDestroyCursorAt(
                                    (((&raw mut gActionSelectionCursor).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read(),
                                );
                                let __p5 = ((&raw mut gActionSelectionCursor).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    );
                                (__p5).write((((((__p5).read()) as i32) ^ 2i32) as u8));
                                ActionSelectionCreateCursorAt(
                                    (((&raw mut gActionSelectionCursor).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read(),
                                    0u8,
                                );
                            }
                        }
                    }
                }
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
            SafariBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnInactiveTextPrinter() {
    unsafe {
        if !((IsTextPrinterActive(0u8)) != 0) {
            SafariBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnHealthboxSpriteCallbackDummy() {
    unsafe {
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
            SafariBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn SafariSetBattleEndCallbacks() {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            crate::c::bf_write(
                ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                1,
                1,
                (0u8) as i32,
            );
            (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(
                ((&raw mut gPreBattleCallback1).cast::<Option<unsafe extern "C" fn()>>()).read(),
            );
            SetMainCallback2(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnSpecialAnimDone() {
    unsafe {
        if (!((((&raw mut gDoingBattleAnim).cast::<u8>()).read()) != 0))
            || (!((crate::c::bf_read(
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
                != 0))
        {
            SafariBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn SafariOpenPokeblockCase() {
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
            .write(Some(CompleteWhenChosePokeblock));
            FreeAllWindowBuffers();
            OpenPokeblockCaseInBattle();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteWhenChosePokeblock() {
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
            SafariBufferExecCompleted();
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
            SafariBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn SafariBufferExecCompleted() {
    unsafe {
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(SafariBufferRunCommand));
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
            SafariBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn SafariHandleGetMonData() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleGetRawMonData() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleSetMonData() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleSetRawMonData() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleLoadMonSprite() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleSwitchInAnim() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleReturnMonToBall() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleDrawTrainerPic() {
    unsafe {
        DecompressTrainerBackPic(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as u16),
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
        );
        SetMultiuseSpriteTemplateToTrainerBack(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as u16),
            GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()),
        );
        (((&raw mut gBattlerSpriteIds).cast::<u8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(CreateSprite(
            (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
            80i16,
            (((((8i32).wrapping_sub(
                (((((&raw mut gTrainerBackPicCoords).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                        .read()) as i32) as isize
                        * 4,
                ))
                .read()) as i32),
            ))
            .wrapping_mul(4i32))
            .wrapping_add(80i32)) as i16),
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
pub(crate) unsafe extern "C" fn SafariHandleTrainerSlide() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleTrainerSlideBack() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleFaintAnimation() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandlePaletteFade() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleSuccessBallThrowAnim() {
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
        .write(Some(CompleteOnSpecialAnimDone));
    }
}
pub(crate) unsafe extern "C" fn SafariHandleBallThrowAnim() {
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
        .write(Some(CompleteOnSpecialAnimDone));
    }
}
pub(crate) unsafe extern "C" fn SafariHandlePause() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleMoveAnimation() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandlePrintString() {
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
pub(crate) unsafe extern "C" fn SafariHandlePrintSelectionString() {
    unsafe {
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
            SafariHandlePrintString();
        } else {
            SafariBufferExecCompleted();
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
            .write(Some(HandleInputChooseAction));
        }
    }
}
pub(crate) unsafe extern "C" fn SafariHandleChooseAction() {
    unsafe {
        let mut i: i32 = 0i32;
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(HandleChooseActionAfterDma3));
        BattlePutTextOnWindow((&raw mut gText_SafariZoneMenu).cast::<u8>(), 2u8);
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
            (&raw mut gText_WhatWillPkmnDo2).cast::<u8>(),
        );
        BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 1u8);
    }
}
pub(crate) unsafe extern "C" fn SafariHandleYesNoBox() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleChooseMove() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleChooseItem() {
    unsafe {
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(SafariOpenPokeblockCase));
        ((&raw mut gBattlerInMenuId).cast::<u8>())
            .write(((&raw mut gActiveBattler).cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn SafariHandleChoosePokemon() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleCmd23() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleHealthBarUpdate() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleExpUpdate() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleStatusIconUpdate() {
    unsafe {
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
            11u8,
        );
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleStatusAnimation() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleStatusXor() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleDataTransfer() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleDMA3Transfer() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandlePlayBGM() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleCmd32() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleTwoReturnValues() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleChosenMonReturnValue() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleOneReturnValue() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleOneReturnValue_Duplicate() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleClearUnkVar() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleSetUnkVar() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleClearUnkFlag() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleToggleUnkFlag() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleHitAnimation() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleCantSwitch() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandlePlaySE() {
    unsafe {
        let mut pan: i8 = 0i8;
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
            pan = (-64i8);
        } else {
            pan = 63i8;
        }
        PlaySE12WithPanning(
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
            pan,
        );
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandlePlayFanfareOrBGM() {
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
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleFaintingCry() {
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
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleIntroSlide() {
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
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleIntroTrainerBallThrow() {
    unsafe {
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
            10u8,
        );
        StartHealthboxSlideIn(((&raw mut gActiveBattler).cast::<u8>()).read());
        SetHealthboxSpriteVisible(
            (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(CompleteOnHealthboxSpriteCallbackDummy));
    }
}
pub(crate) unsafe extern "C" fn SafariHandleDrawPartyStatusSummary() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleHidePartyStatusSummary() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleEndBounceEffect() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleSpriteInvisibility() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleBattleAnimation() {
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
            SafariBufferExecCompleted();
        } else {
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(CompleteOnFinishedBattleAnimation));
        }
    }
}
pub(crate) unsafe extern "C" fn SafariHandleLinkStandbyMsg() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleResetActionMoveSelection() {
    unsafe {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleEndLinkBattle() {
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
        SafariBufferExecCompleted();
        if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0)
            && (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4u32) != 0))
        {
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(SafariSetBattleEndCallbacks));
        }
    }
}
pub(crate) unsafe extern "C" fn SafariCmdEnd() {
    unsafe {}
}
