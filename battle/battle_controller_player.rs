//! Translated from `src/battle_controller_player.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sPlayerBufferCommands sTargetIdentities sUnused
#[allow(unused_imports)]
use crate::data::battle_controller_player::*;

unsafe extern "C" {
    static mut gAbsentBattlerFlags: u8;
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
    static mut gBattleMons: u8;
    static mut gBattleMoves: u8;
    static mut gBattleOutcome: u8;
    static mut gBattlePalaceMoveSelectionRngValue: u8;
    static mut gBattlePartyCurrentOrder: u8;
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
    static mut gBattlersCount: u8;
    static mut gBitTable: u8;
    static mut gDisableStructs: u8;
    static mut gDisplayedStringBattle: u8;
    static mut gDoingBattleAnim: u8;
    static mut gExperienceTables: u8;
    static mut gHealthboxSpriteIds: u8;
    static mut gIntroSlideFlags: u8;
    static mut gLinkPlayers: u8;
    static mut gMPlayInfo_BGM: u8;
    static mut gMain: u8;
    static mut gMoveNames: u8;
    static mut gMoveSelectionCursor: u8;
    static mut gMultiUsePlayerCursor: u8;
    static mut gMultiuseSpriteTemplate: u8;
    static mut gNumberOfMovesToChoose: u8;
    static mut gPaletteFade: u8;
    static mut gPartnerTrainerId: u8;
    static mut gPartyMenuUseExitCallback: u8;
    static mut gPlayerDpadHoldFrames: u8;
    static mut gPlayerParty: u8;
    static mut gPlayerPartyLostHP: u8;
    static mut gPreBattleCallback1: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRngValue: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSelectedMonPartyId: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSpeciesInfo: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gText_BattleMenu: u8;
    static mut gText_BattleSwitchWhich: u8;
    static mut gText_BattleYesNoChoice: u8;
    static mut gText_LinkStandby: u8;
    static mut gText_MoveInterfacePP: u8;
    static mut gText_MoveInterfaceType: u8;
    static mut gText_WhatWillPkmnDo: u8;
    static mut gTrainerBackPicCoords: u8;
    static mut gTrainerBackPicPaletteTable: u8;
    static mut gTrainerFrontPicCoords: u8;
    static mut gTrainerFrontPicPaletteTable: u8;
    static mut gTransformedPersonalities: u8;
    static mut gTypeNames: u8;
    static mut gUnusedControllerStruct: u8;
    static mut gWeatherMoveAnim: u8;
    static mut gWirelessCommType: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn BattleArena_DeductSkillPoints(a0: u8, a1: u16);
    fn BattleCreateYesNoCursorAt(a0: u8);
    fn BattleDestroyYesNoCursorAt(a0: u8);
    fn BattleGfxSfxDummy2(a0: u16);
    fn BattleGfxSfxDummy3(a0: u8);
    fn BattleLoadPlayerMonSpriteGfx(a0: *mut u8, a1: u8);
    fn BattleMainCB2();
    fn BattlePutTextOnWindow(a0: *mut u8, a1: u8);
    fn BattleStopLowHpSound();
    fn BattleStringExpandPlaceholdersToDisplayedString(a0: *mut u8) -> u32;
    fn BattleTv_ClearExplosionFaintCause();
    fn BattleTv_SetDataBasedOnAnimation(a0: u8);
    fn BattleTv_SetDataBasedOnMove(a0: u16, a1: u16, a2: *mut u8);
    fn BattleTv_SetDataBasedOnString(a0: u16);
    fn BeginFastPaletteFade(a0: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BtlController_EmitChosenMonReturnValue(a0: u8, a1: u8, a2: *mut u8);
    fn BtlController_EmitDataTransfer(a0: u8, a1: u16, a2: *mut u8);
    fn BtlController_EmitOneReturnValue(a0: u8, a1: u16);
    fn BtlController_EmitOneReturnValue_Duplicate(a0: u8, a1: u16);
    fn BtlController_EmitTwoReturnValues(a0: u8, a1: u8, a2: u16);
    fn BufferStringBattle(a0: u16);
    fn CB2_BagMenuFromBattle();
    fn CB2_InitEndLinkBattle();
    fn CalculateMonStats(a0: *mut u8);
    fn ChooseMoveAndTargetInBattlePalace() -> u16;
    fn ClearTemporarySpeciesSpriteData(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyAllBattleSpritesInvisibilities();
    fn CopyBattleSpriteInvisibility(a0: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CountAliveMonsInBattle(a0: u8) -> u8;
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut u8)>) -> u8;
    fn CreatePartyStatusSummarySprites(a0: u8, a1: *mut u8, a2: u8, a3: u8) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressTrainerBackPic(a0: u16, a1: u8);
    fn DecompressTrainerFrontPic(a0: u16, a1: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DoBounceEffect(a0: u8, a1: u8, a2: i8, a3: i8);
    fn DoHitAnimHealthboxEffect(a0: u8);
    fn DoMoveAnim(a0: u16);
    fn DoPokeballSendOutAnimation(a0: i16, a1: u8) -> u8;
    fn EndBounceEffect(a0: u8, a1: u8);
    fn FadeOutMapMusic(a0: u8);
    fn FreeAllWindowBuffers();
    fn FreeOamMatrix(a0: u8);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteDefault_Y(a0: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetDefaultMoveTarget(a0: u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn GetSpritePaletteTagByPaletteNum(a0: u8) -> u16;
    fn HandleBattleWindow(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn HandleIntroSlide(a0: u8);
    fn HandleLowHpMusicChange(a0: *mut u8, a1: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitAndLaunchChosenStatusAnimation(a0: u8, a1: u32);
    fn InitAndLaunchSpecialAnimation(a0: u8, a1: u8, a2: u8, a3: u8);
    fn IsBattleSEPlaying(a0: u8) -> u8;
    fn IsBattlerSpritePresent(a0: u8) -> u8;
    fn IsCryPlayingOrClearCrySongs() -> u8;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsDoubleBattle() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn IsMoveWithoutAnimation(a0: u16, a1: u8) -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LoadBattleBarGfx(a0: u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn MoveBattleBar(a0: u8, a1: u8, a2: u8, a3: u8) -> i32;
    fn OpenPartyMenuInBattle(a0: u8);
    fn PlayBGM(a0: u16);
    fn PlayCry_ByMode(a0: u16, a1: i8, a2: u8);
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PlayerGenderToFrontTrainerPicId(a0: u8) -> u16;
    fn PrepareBufferDataTransferLink(a0: u8, a1: u16, a2: *mut u8);
    fn RecordedBattle_RecordAllBattlerData(a0: *mut u8);
    fn ReshowBattleScreenAfterMenu();
    fn ReshowBattleScreenDummy();
    fn SetBattleBarStruct(a0: u8, a1: u8, a2: i32, a3: i32, a4: i32);
    fn SetBattlerSpriteAffineMode(a0: u8);
    fn SetCloseLinkCallback();
    fn SetHealthboxSpriteInvisible(a0: u8);
    fn SetHealthboxSpriteVisible(a0: u8);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetMultiuseSpriteTemplateToTrainerBack(a0: u16, a1: u8);
    fn SetMultiuseSpriteTemplateToTrainerFront(a0: u16, a1: u8);
    fn SetPPNumbersPaletteInMoveSelection();
    fn SetSpritePrimaryCoordsFromSecondaryCoords(a0: *mut u8);
    fn SpriteCB_FaintSlideAnim(a0: *mut u8);
    fn SpriteCB_HideAsMoveTarget(a0: *mut u8);
    fn SpriteCB_ShowAsMoveTarget(a0: *mut u8);
    fn SpriteCB_TrainerSlideIn(a0: *mut u8);
    fn SpriteCB_WaitForBattlerBallReleaseAnim(a0: *mut u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartHealthboxSlideIn(a0: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn SwapHpBarsWithHpText();
    fn TaskDummy(a0: u8);
    fn Task_HidePartyStatusSummary(a0: u8);
    fn TryHandleLaunchBattleTableAnimation(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn TryPutLinkBattleTvShowOnAir();
    fn TrySetBehindSubstituteSpriteBit(a0: u8, a1: u16);
    fn TryShinyAnimation(a0: u8, a1: *mut u8);
    fn UpdateHealthboxAttribute(a0: u8, a1: *mut u8, a2: u8);
    fn UpdateHpTextInHealthbox(a0: u8, a1: i16, a2: u8);
    fn m4aMPlayContinue(a0: *mut u8);
    fn m4aMPlayVolumeControl(a0: *mut u8, a1: u16, a2: u16);
    fn m4aSongNumStop(a0: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleControllerDummy() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetControllerToPlayer() {
    unsafe {
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(PlayerBufferRunCommand));
        ((&raw mut gDoingBattleAnim).cast::<u8>()).write(0u8);
        ((&raw mut gPlayerDpadHoldFrames).cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn PlayerBufferExecCompleted() {
    unsafe {
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(PlayerBufferRunCommand));
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
pub(crate) unsafe extern "C" fn PlayerBufferRunCommand() {
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
                (((((&raw const sPlayerBufferCommands)
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
                PlayerBufferExecCompleted();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnBankSpritePosX_0() {
    unsafe {
        if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(36)
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn HandleInputChooseAction() {
    unsafe {
        let mut itemId: u16 = (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
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
        DoBounceEffect(
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            1u8,
            7i8,
            1i8,
        );
        DoBounceEffect(
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            0u8,
            7i8,
            1i8,
        );
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 240i32)
            != 0)
            && (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19)).read())
                as i32)
                == 2i32)
        {
            let __p1 = (&raw mut gPlayerDpadHoldFrames).cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            ((&raw mut gPlayerDpadHoldFrames).cast::<u8>()).write(0u8);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            'l1: {
                let __sw2 = (((((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32);
                if __sw2 == 0i32 {
                    BtlController_EmitTwoReturnValues(1u8, 0u8, 0u16);
                    break 'l1;
                }
                if __sw2 == 1i32 {
                    BtlController_EmitTwoReturnValues(1u8, 1u8, 0u16);
                    break 'l1;
                }
                if __sw2 == 2i32 {
                    BtlController_EmitTwoReturnValues(1u8, 2u8, 0u16);
                    break 'l1;
                }
                if __sw2 == 3i32 {
                    BtlController_EmitTwoReturnValues(1u8, 3u8, 0u16);
                    break 'l1;
                }
            }
            PlayerBufferExecCompleted();
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
                    let __p3 = ((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
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
                        let __p4 = ((&raw mut gActionSelectionCursor).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            );
                        (__p4).write((((((__p4).read()) as i32) ^ 1i32) as u8));
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
                            let __p5 = ((&raw mut gActionSelectionCursor).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                );
                            (__p5).write((((((__p5).read()) as i32) ^ 2i32) as u8));
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
                                let __p6 = ((&raw mut gActionSelectionCursor).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    );
                                (__p6).write((((((__p6).read()) as i32) ^ 2i32) as u8));
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
                        } else {
                            if (((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 2i32)
                                != 0)
                                || (((((&raw mut gPlayerDpadHoldFrames).cast::<u8>()).read())
                                    as i32)
                                    > 59i32)
                            {
                                if ((((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32)
                                    != 0)
                                    && (((GetBattlerPosition(
                                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                    )) as i32)
                                        == 2i32))
                                    && (!((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read())
                                        as u32)
                                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                            .wrapping_offset(
                                                ((GetBattlerAtPosition(0u8)) as i32) as isize,
                                            ))
                                        .read())
                                        != 0)))
                                    && (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                        & 64u32)
                                        != 0))
                                {
                                    if (((((((&raw mut gBattleBufferA).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 512,
                                        ))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        == 1i32
                                    {
                                        if ((itemId) as i32) <= 12i32 {
                                            AddBagItem(itemId, 1u16);
                                        } else {
                                            return;
                                        }
                                    }
                                    PlaySE(5u16);
                                    BtlController_EmitTwoReturnValues(1u8, 12u8, 0u16);
                                    PlayerBufferExecCompleted();
                                }
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(46)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 8i32)
                                    != 0
                                {
                                    SwapHpBarsWithHpText();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UnusedEndBounceEffect() {
    unsafe {
        EndBounceEffect(((&raw mut gActiveBattler).cast::<u8>()).read(), 1u8);
        EndBounceEffect(((&raw mut gActiveBattler).cast::<u8>()).read(), 0u8);
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(HandleInputChooseTarget));
    }
}
pub(crate) unsafe extern "C" fn HandleInputChooseTarget() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut identities = crate::ffi::Align4([0u8; 4]);
        crate::c::memcpy(
            (&raw mut identities).cast::<u8>(),
            ((&raw const sTargetIdentities).cast::<u8>().cast_mut()).cast::<u8>(),
            crate::c::div_u32(4u32, 1u32),
        );
        DoBounceEffect(
            ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
            1u8,
            15i8,
            1i8,
        );
        i = 0i32;
        if ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32) != 0i32 {
            'l1: loop {
                'l2: {
                    if i != ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) {
                        EndBounceEffect(((i) as u8), 1u8);
                    }
                    i = (i).wrapping_add(1);
                }
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
            }
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 240i32)
            != 0)
            && (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19)).read())
                as i32)
                == 2i32)
        {
            let __p1 = (&raw mut gPlayerDpadHoldFrames).cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            ((&raw mut gPlayerDpadHoldFrames).cast::<u8>()).write(0u8);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_HideAsMoveTarget));
            BtlController_EmitTwoReturnValues(
                1u8,
                10u8,
                (((((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32)
                    | (((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) << 8))
                    as u16),
            );
            EndBounceEffect(((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(), 1u8);
            PlayerBufferExecCompleted();
        } else {
            if (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0)
                || (((((&raw mut gPlayerDpadHoldFrames).cast::<u8>()).read()) as i32) > 59i32)
            {
                PlaySE(5u16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_HideAsMoveTarget));
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(Some(HandleInputChooseMove));
                DoBounceEffect(
                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                    1u8,
                    7i8,
                    1i8,
                );
                DoBounceEffect(
                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                    0u8,
                    7i8,
                    1i8,
                );
                EndBounceEffect(((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(), 1u8);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 96i32)
                    != 0
                {
                    PlaySE(5u16);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_HideAsMoveTarget));
                    'l3: loop {
                        'l4: {
                            let mut currSelIdentity: u8 = GetBattlerPosition(
                                ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                            );
                            {
                                i = 0i32;
                                'l5: loop {
                                    if !(i < 4i32) {
                                        break 'l5;
                                    }
                                    'l6: {
                                        if ((currSelIdentity) as i32)
                                            == (((((&raw mut identities).cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                        {
                                            break 'l5;
                                        }
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                            'l7: loop {
                                'l8: {
                                    if {
                                        let __t2 = (i).wrapping_sub(1);
                                        i = __t2;
                                        __t2
                                    } < 0i32
                                    {
                                        i = 3i32;
                                    }
                                    ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).write(
                                        GetBattlerAtPosition(
                                            (((&raw mut identities).cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                            .read(),
                                        ),
                                    );
                                }
                                if !(((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read())
                                    as i32)
                                    == ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                                {
                                    break 'l7;
                                }
                            }
                            i = 0i32;
                            'l9: {
                                let __sw3 = ((GetBattlerPosition(
                                    ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                )) as i32);
                                if __sw3 == 0i32 || __sw3 == 2i32 {
                                    if ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        != ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read())
                                            as i32)
                                    {
                                        i = (i).wrapping_add(1);
                                    } else {
                                        if (((((((&raw mut gBattleMoves).cast::<u8>())
                                            .wrapping_offset(
                                                ((GetMonData2(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((((&raw mut gBattlerPartyIndexes)
                                                                .cast::<u16>())
                                                            .cast::<u16>())
                                                            .wrapping_offset(
                                                                ((((&raw mut gActiveBattler)
                                                                    .cast::<u8>())
                                                                .read())
                                                                    as i32)
                                                                    as isize,
                                                            ))
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 100,
                                                        ),
                                                    (13i32).wrapping_add(
                                                        (((((&raw mut gMoveSelectionCursor)
                                                            .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .read())
                                                            as i32),
                                                    ),
                                                ))
                                                    as i32)
                                                    as isize
                                                    * 12,
                                            ))
                                        .wrapping_add(6))
                                        .read())
                                            as i32)
                                            & 2i32)
                                            != 0
                                        {
                                            i = (i).wrapping_add(1);
                                        }
                                    }
                                    break 'l9;
                                }
                                if __sw3 == 1i32 || __sw3 == 3i32 {
                                    i = (i).wrapping_add(1);
                                    break 'l9;
                                }
                            }
                            if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read())
                                            as i32)
                                            as isize,
                                    ))
                                .read())
                                != 0
                            {
                                i = 0i32;
                            }
                        }
                        if !(i == 0i32) {
                            break 'l3;
                        }
                    }
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_ShowAsMoveTarget));
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 144i32)
                        != 0
                    {
                        PlaySE(5u16);
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_HideAsMoveTarget));
                        'l10: loop {
                            'l11: {
                                let mut currSelIdentity: u8 = GetBattlerPosition(
                                    ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                );
                                {
                                    i = 0i32;
                                    'l12: loop {
                                        if !(i < 4i32) {
                                            break 'l12;
                                        }
                                        'l13: {
                                            if ((currSelIdentity) as i32)
                                                == (((((&raw mut identities).cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                            {
                                                break 'l12;
                                            }
                                        }
                                        i = (i).wrapping_add(1);
                                    }
                                }
                                'l14: loop {
                                    'l15: {
                                        if {
                                            let __t4 = (i).wrapping_add(1);
                                            i = __t4;
                                            __t4
                                        } > 3i32
                                        {
                                            i = 0i32;
                                        }
                                        ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).write(
                                            GetBattlerAtPosition(
                                                (((&raw mut identities).cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                .read(),
                                            ),
                                        );
                                    }
                                    if !(((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read())
                                        as i32)
                                        == ((((&raw mut gBattlersCount).cast::<u8>()).read())
                                            as i32))
                                    {
                                        break 'l14;
                                    }
                                }
                                i = 0i32;
                                'l16: {
                                    let __sw5 = ((GetBattlerPosition(
                                        ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                    )) as i32);
                                    if __sw5 == 0i32 || __sw5 == 2i32 {
                                        if ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                            as i32)
                                            != ((((&raw mut gMultiUsePlayerCursor).cast::<u8>())
                                                .read())
                                                as i32)
                                        {
                                            i = (i).wrapping_add(1);
                                        } else {
                                            if (((((((&raw mut gBattleMoves).cast::<u8>())
                                                .wrapping_offset(
                                                    ((GetMonData2(
                                                        ((&raw mut gPlayerParty).cast::<u8>())
                                                            .wrapping_offset(
                                                            ((((((&raw mut gBattlerPartyIndexes)
                                                                .cast::<u16>())
                                                            .cast::<u16>())
                                                            .wrapping_offset(
                                                                ((((&raw mut gActiveBattler)
                                                                    .cast::<u8>())
                                                                .read())
                                                                    as i32)
                                                                    as isize,
                                                            ))
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 100,
                                                        ),
                                                        (13i32).wrapping_add(
                                                            (((((&raw mut gMoveSelectionCursor)
                                                                .cast::<u8>())
                                                            .wrapping_offset(
                                                                ((((&raw mut gActiveBattler)
                                                                    .cast::<u8>())
                                                                .read())
                                                                    as i32)
                                                                    as isize,
                                                            ))
                                                            .read())
                                                                as i32),
                                                        ),
                                                    ))
                                                        as i32)
                                                        as isize
                                                        * 12,
                                                ))
                                            .wrapping_add(6))
                                            .read())
                                                as i32)
                                                & 2i32)
                                                != 0
                                            {
                                                i = (i).wrapping_add(1);
                                            }
                                        }
                                        break 'l16;
                                    }
                                    if __sw5 == 1i32 || __sw5 == 3i32 {
                                        i = (i).wrapping_add(1);
                                        break 'l16;
                                    }
                                }
                                if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                                    & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gMultiUsePlayerCursor).cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read())
                                    != 0
                                {
                                    i = 0i32;
                                }
                            }
                            if !(i == 0i32) {
                                break 'l10;
                            }
                        }
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_ShowAsMoveTarget));
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandleInputChooseMove() {
    unsafe {
        let mut canSelectTarget: u32 = 0u32;
        let mut moveInfo: *mut u8 = ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(4);
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 240i32)
            != 0)
            && (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19)).read())
                as i32)
                == 2i32)
        {
            let __p1 = (&raw mut gPlayerDpadHoldFrames).cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            ((&raw mut gPlayerDpadHoldFrames).cast::<u8>()).write(0u8);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            let mut moveTarget: u8 = 0u8;
            PlaySE(5u16);
            if (((((moveInfo).cast::<u16>()).wrapping_offset(
                (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize,
            ))
            .read()) as i32)
                == 174i32
            {
                if ((((((moveInfo).wrapping_add(18)).cast::<u8>()).read()) as i32) != 7i32)
                    && (((((((moveInfo).wrapping_add(18)).cast::<u8>()).wrapping_offset(1)).read())
                        as i32)
                        != 7i32)
                {
                    moveTarget = 16u8;
                } else {
                    moveTarget = 0u8;
                }
            } else {
                moveTarget = ((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                    (((((moveInfo).cast::<u16>()).wrapping_offset(
                        (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 12,
                ))
                .wrapping_add(6))
                .read();
            }
            if (((moveTarget) as i32) & 16i32) != 0 {
                ((&raw mut gMultiUsePlayerCursor).cast::<u8>())
                    .write(((&raw mut gActiveBattler).cast::<u8>()).read());
            } else {
                ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).write(GetBattlerAtPosition(
                    (((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()))
                        as i32)
                        & 1i32)
                        ^ 1i32) as u8),
                ));
            }
            if !(((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read())
                != 0)
            {
                if ((((moveTarget) as i32) & 2i32) != 0)
                    && (!(((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read())
                        != 0))
                {
                    canSelectTarget = (canSelectTarget).wrapping_add(1);
                }
            } else {
                if !((((moveTarget) as i32) & 125i32) != 0) {
                    canSelectTarget = (canSelectTarget).wrapping_add(1);
                }
                if ((((((moveInfo).wrapping_add(8)).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize,
                ))
                .read()) as i32)
                    == 0i32
                {
                    canSelectTarget = 0u32;
                } else {
                    if (!((((moveTarget) as i32) & 18i32) != 0))
                        && (((CountAliveMonsInBattle(0u8)) as i32) <= 1i32)
                    {
                        ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).write(
                            GetDefaultMoveTarget(((&raw mut gActiveBattler).cast::<u8>()).read()),
                        );
                        canSelectTarget = 0u32;
                    }
                }
            }
            if !((canSelectTarget) != 0) {
                BtlController_EmitTwoReturnValues(
                    1u8,
                    10u8,
                    (((((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        | (((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) << 8))
                        as u16),
                );
                PlayerBufferExecCompleted();
            } else {
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(Some(HandleInputChooseTarget));
                if (((moveTarget) as i32) & 18i32) != 0 {
                    ((&raw mut gMultiUsePlayerCursor).cast::<u8>())
                        .write(((&raw mut gActiveBattler).cast::<u8>()).read());
                } else {
                    if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                            .wrapping_offset(((GetBattlerAtPosition(1u8)) as i32) as isize))
                        .read())
                        != 0
                    {
                        ((&raw mut gMultiUsePlayerCursor).cast::<u8>())
                            .write(GetBattlerAtPosition(3u8));
                    } else {
                        ((&raw mut gMultiUsePlayerCursor).cast::<u8>())
                            .write(GetBattlerAtPosition(1u8));
                    }
                }
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_ShowAsMoveTarget));
            }
        } else {
            if (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0)
                || (((((&raw mut gPlayerDpadHoldFrames).cast::<u8>()).read()) as i32) > 59i32)
            {
                PlaySE(5u16);
                BtlController_EmitTwoReturnValues(1u8, 10u8, 65535u16);
                PlayerBufferExecCompleted();
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 32i32)
                    != 0
                {
                    if ((((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        MoveSelectionDestroyCursorAt(
                            (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read(),
                        );
                        let __p2 = ((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        );
                        (__p2).write((((((__p2).read()) as i32) ^ 1i32) as u8));
                        PlaySE(5u16);
                        MoveSelectionCreateCursorAt(
                            (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read(),
                            0u8,
                        );
                        MoveSelectionDisplayPPNumber();
                        MoveSelectionDisplayMoveType();
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 16i32)
                        != 0
                    {
                        if (!(((((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            & 1i32)
                            != 0))
                            && (((((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32)
                                ^ 1i32)
                                < ((((&raw mut gNumberOfMovesToChoose).cast::<u8>()).read())
                                    as i32))
                        {
                            MoveSelectionDestroyCursorAt(
                                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                                .read(),
                            );
                            let __p3 = ((&raw mut gMoveSelectionCursor).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                );
                            (__p3).write((((((__p3).read()) as i32) ^ 1i32) as u8));
                            PlaySE(5u16);
                            MoveSelectionCreateCursorAt(
                                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                                .read(),
                                0u8,
                            );
                            MoveSelectionDisplayPPNumber();
                            MoveSelectionDisplayMoveType();
                        }
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 64i32)
                            != 0
                        {
                            if ((((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read()) as i32)
                                & 2i32)
                                != 0
                            {
                                MoveSelectionDestroyCursorAt(
                                    (((&raw mut gMoveSelectionCursor).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read(),
                                );
                                let __p4 = ((&raw mut gMoveSelectionCursor).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    );
                                (__p4).write((((((__p4).read()) as i32) ^ 2i32) as u8));
                                PlaySE(5u16);
                                MoveSelectionCreateCursorAt(
                                    (((&raw mut gMoveSelectionCursor).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read(),
                                    0u8,
                                );
                                MoveSelectionDisplayPPNumber();
                                MoveSelectionDisplayMoveType();
                            }
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 128i32)
                                != 0
                            {
                                if (!(((((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()) as i32)
                                    & 2i32)
                                    != 0))
                                    && (((((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()) as i32)
                                        ^ 2i32)
                                        < ((((&raw mut gNumberOfMovesToChoose).cast::<u8>()).read())
                                            as i32))
                                {
                                    MoveSelectionDestroyCursorAt(
                                        (((&raw mut gMoveSelectionCursor).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read(),
                                    );
                                    let __p5 = ((&raw mut gMoveSelectionCursor).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        );
                                    (__p5).write((((((__p5).read()) as i32) ^ 2i32) as u8));
                                    PlaySE(5u16);
                                    MoveSelectionCreateCursorAt(
                                        (((&raw mut gMoveSelectionCursor).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read(),
                                        0u8,
                                    );
                                    MoveSelectionDisplayPPNumber();
                                    MoveSelectionDisplayMoveType();
                                }
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(46)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 4i32)
                                    != 0
                                {
                                    if (((((&raw mut gNumberOfMovesToChoose).cast::<u8>()).read())
                                        as i32)
                                        > 1i32)
                                        && (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                            & 2u32)
                                            != 0))
                                    {
                                        MoveSelectionCreateCursorAt(
                                            (((&raw mut gMoveSelectionCursor).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler).cast::<u8>())
                                                        .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                            .read(),
                                            29u8,
                                        );
                                        if (((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read()) as i32)
                                            != 0i32
                                        {
                                            ((&raw mut gMultiUsePlayerCursor).cast::<u8>())
                                                .write(0u8);
                                        } else {
                                            ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).write(
                                                (((((((&raw mut gMoveSelectionCursor)
                                                    .cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler).cast::<u8>())
                                                        .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .read())
                                                    as i32)
                                                    .wrapping_add(1i32))
                                                    as u8),
                                            );
                                        }
                                        MoveSelectionCreateCursorAt(
                                            ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                            27u8,
                                        );
                                        BattlePutTextOnWindow(
                                            (&raw mut gText_BattleSwitchWhich).cast::<u8>(),
                                            11u8,
                                        );
                                        ((((&raw mut gBattlerControllerFuncs)
                                            .cast::<Option<unsafe extern "C" fn()>>())
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                        .write(Some(HandleMoveSwitching));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandleMoveInputUnused() -> u32 {
    unsafe {
        let mut var: u32 = 0u32;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            var = 1u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            PlaySE(5u16);
            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(320u16);
            var = 255u32;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0)
            && (((((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                & 1i32)
                != 0)
        {
            MoveSelectionDestroyCursorAt(
                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
            );
            let __p1 = ((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            );
            (__p1).write((((((__p1).read()) as i32) ^ 1i32) as u8));
            PlaySE(5u16);
            MoveSelectionCreateCursorAt(
                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
                0u8,
            );
        }
        if ((((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0)
            && (!(((((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                & 1i32)
                != 0)))
            && (((((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                ^ 1i32)
                < ((((&raw mut gNumberOfMovesToChoose).cast::<u8>()).read()) as i32))
        {
            MoveSelectionDestroyCursorAt(
                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
            );
            let __p2 = ((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            );
            (__p2).write((((((__p2).read()) as i32) ^ 1i32) as u8));
            PlaySE(5u16);
            MoveSelectionCreateCursorAt(
                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
                0u8,
            );
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0)
            && (((((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                & 2i32)
                != 0)
        {
            MoveSelectionDestroyCursorAt(
                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
            );
            let __p3 = ((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            );
            (__p3).write((((((__p3).read()) as i32) ^ 2i32) as u8));
            PlaySE(5u16);
            MoveSelectionCreateCursorAt(
                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
                0u8,
            );
        }
        if ((((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0)
            && (!(((((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                & 2i32)
                != 0)))
            && (((((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                ^ 2i32)
                < ((((&raw mut gNumberOfMovesToChoose).cast::<u8>()).read()) as i32))
        {
            MoveSelectionDestroyCursorAt(
                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
            );
            let __p4 = ((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            );
            (__p4).write((((((__p4).read()) as i32) ^ 2i32) as u8));
            PlaySE(5u16);
            MoveSelectionCreateCursorAt(
                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
                0u8,
            );
        }
        return var;
    }
}
pub(crate) unsafe extern "C" fn HandleMoveSwitching() {
    unsafe {
        let mut perMovePPBonuses = crate::ffi::Align4([0u8; 4]);
        let mut moveStruct = crate::ffi::Align4([0u8; 20]);
        let mut totalPPBonuses: u8 = 0u8;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 5i32)
            != 0
        {
            PlaySE(5u16);
            if (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                != ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
            {
                let mut moveInfo: *mut u8 = ((((&raw mut gBattleBufferA).cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                .cast::<u8>())
                .wrapping_offset(4);
                let mut i: i32 = 0i32;
                i = (((((moveInfo).cast::<u16>()).wrapping_offset(
                    (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize,
                ))
                .read()) as i32);
                (((moveInfo).cast::<u16>()).wrapping_offset(
                    (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize,
                ))
                .write(
                    (((moveInfo).cast::<u16>()).wrapping_offset(
                        ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read(),
                );
                (((moveInfo).cast::<u16>()).wrapping_offset(
                    ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(((i) as u16));
                i = ((((((moveInfo).wrapping_add(8)).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize,
                ))
                .read()) as i32);
                ((((moveInfo).wrapping_add(8)).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize,
                ))
                .write(
                    ((((moveInfo).wrapping_add(8)).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read(),
                );
                ((((moveInfo).wrapping_add(8)).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(((i) as u8));
                i = ((((((moveInfo).wrapping_add(12)).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize,
                ))
                .read()) as i32);
                ((((moveInfo).wrapping_add(12)).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize,
                ))
                .write(
                    ((((moveInfo).wrapping_add(12)).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read(),
                );
                ((((moveInfo).wrapping_add(12)).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(((i) as u8));
                if (((crate::c::bf_read(
                    (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 28,
                    ))
                    .wrapping_add(24),
                    4,
                    4,
                    false,
                ) as u8) as u32)
                    & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                        (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32) as isize,
                    ))
                    .read())
                    != 0
                {
                    crate::c::bf_write(
                        (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(24),
                        4,
                        4,
                        ((((crate::c::bf_read(
                            (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(24),
                            4,
                            4,
                            false,
                        ) as u8) as u32)
                            & !(((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset(
                                    (((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()) as i32) as isize,
                                ))
                            .read())) as u8) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(24),
                        4,
                        4,
                        ((((crate::c::bf_read(
                            (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(24),
                            4,
                            4,
                            false,
                        ) as u8) as u32)
                            | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset(
                                    ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read())
                                        as i32) as isize,
                                ))
                            .read()) as u8) as i32,
                    );
                }
                MoveSelectionDisplayMoveNames();
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i < 4i32) {
                            break 'l1;
                        }
                        'l2: {
                            (((&raw mut perMovePPBonuses).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .write(
                                ((crate::c::shr_i32(
                                    (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(59))
                                    .read()) as i32)
                                        & crate::c::shl_i32(
                                            3i32,
                                            (((i).wrapping_mul(2i32)) as u32),
                                        )),
                                    (((i).wrapping_mul(2i32)) as u32),
                                )) as u8),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                totalPPBonuses = (((&raw mut perMovePPBonuses).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize,
                ))
                .read();
                (((&raw mut perMovePPBonuses).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize,
                ))
                .write(
                    (((&raw mut perMovePPBonuses).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read(),
                );
                (((&raw mut perMovePPBonuses).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(totalPPBonuses);
                totalPPBonuses = 0u8;
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < 4i32) {
                            break 'l3;
                        }
                        'l4: {
                            totalPPBonuses = ((((totalPPBonuses) as i32)
                                | crate::c::shl_i32(
                                    (((((&raw mut perMovePPBonuses).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as i32),
                                    (((i).wrapping_mul(2i32)) as u32),
                                )) as u8);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(59))
                .write(totalPPBonuses);
                {
                    i = 0i32;
                    'l5: loop {
                        if !(i < 4i32) {
                            break 'l5;
                        }
                        'l6: {
                            ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(12))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(
                                (((moveInfo).cast::<u16>()).wrapping_offset((i) as isize)).read(),
                            );
                            ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(36))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((moveInfo).wrapping_add(8)).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if !((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(80)
                .cast::<u32>())
                .read()
                    & 2097152u32)
                    != 0)
                {
                    {
                        i = 0i32;
                        'l7: loop {
                            if !(i < 4i32) {
                                break 'l7;
                            }
                            'l8: {
                                ((((&raw mut moveStruct).cast::<u8>()).cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                .write(
                                    ((GetMonData2(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .read())
                                                as i32)
                                                as isize
                                                * 100,
                                        ),
                                        (13i32).wrapping_add(i),
                                    )) as u16),
                                );
                                (((((&raw mut moveStruct).cast::<u8>()).wrapping_add(8))
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .write(
                                    ((GetMonData2(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .read())
                                                as i32)
                                                as isize
                                                * 100,
                                        ),
                                        (17i32).wrapping_add(i),
                                    )) as u8),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    totalPPBonuses = ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        21i32,
                    )) as u8);
                    {
                        i = 0i32;
                        'l9: loop {
                            if !(i < 4i32) {
                                break 'l9;
                            }
                            'l10: {
                                (((&raw mut perMovePPBonuses).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .write(
                                    ((crate::c::shr_i32(
                                        (((totalPPBonuses) as i32)
                                            & crate::c::shl_i32(
                                                3i32,
                                                (((i).wrapping_mul(2i32)) as u32),
                                            )),
                                        (((i).wrapping_mul(2i32)) as u32),
                                    )) as u8),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    i = ((((((&raw mut moveStruct).cast::<u8>()).cast::<u16>()).wrapping_offset(
                        (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32);
                    ((((&raw mut moveStruct).cast::<u8>()).cast::<u16>()).wrapping_offset(
                        (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32) as isize,
                    ))
                    .write(
                        ((((&raw mut moveStruct).cast::<u8>()).cast::<u16>()).wrapping_offset(
                            ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                        .read(),
                    );
                    ((((&raw mut moveStruct).cast::<u8>()).cast::<u16>()).wrapping_offset(
                        ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .write(((i) as u16));
                    i = (((((((&raw mut moveStruct).cast::<u8>()).wrapping_add(8)).cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read()) as i32) as isize,
                        ))
                    .read()) as i32);
                    (((((&raw mut moveStruct).cast::<u8>()).wrapping_add(8)).cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read()) as i32) as isize,
                        ))
                    .write(
                        (((((&raw mut moveStruct).cast::<u8>()).wrapping_add(8)).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read(),
                    );
                    (((((&raw mut moveStruct).cast::<u8>()).wrapping_add(8)).cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                    .write(((i) as u8));
                    totalPPBonuses = (((&raw mut perMovePPBonuses).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32) as isize,
                    ))
                    .read();
                    (((&raw mut perMovePPBonuses).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32) as isize,
                    ))
                    .write(
                        (((&raw mut perMovePPBonuses).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                        .read(),
                    );
                    (((&raw mut perMovePPBonuses).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .write(totalPPBonuses);
                    totalPPBonuses = 0u8;
                    {
                        i = 0i32;
                        'l11: loop {
                            if !(i < 4i32) {
                                break 'l11;
                            }
                            'l12: {
                                totalPPBonuses = ((((totalPPBonuses) as i32)
                                    | crate::c::shl_i32(
                                        (((((&raw mut perMovePPBonuses).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read()) as i32),
                                        (((i).wrapping_mul(2i32)) as u32),
                                    )) as u8);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    {
                        i = 0i32;
                        'l13: loop {
                            if !(i < 4i32) {
                                break 'l13;
                            }
                            'l14: {
                                SetMonData(
                                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read()) as i32)
                                            as isize
                                            * 100,
                                    ),
                                    (13i32).wrapping_add(i),
                                    ((((&raw mut moveStruct).cast::<u8>()).cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                    .cast::<u8>(),
                                );
                                SetMonData(
                                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read()) as i32)
                                            as isize
                                            * 100,
                                    ),
                                    (17i32).wrapping_add(i),
                                    ((((&raw mut moveStruct).cast::<u8>()).wrapping_add(8))
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        21i32,
                        &raw mut totalPPBonuses,
                    );
                }
            }
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(HandleInputChooseMove));
            (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .write(((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read());
            MoveSelectionCreateCursorAt(
                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
                0u8,
            );
            MoveSelectionDisplayPPString();
            MoveSelectionDisplayPPNumber();
            MoveSelectionDisplayMoveType();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 6i32)
                != 0
            {
                PlaySE(5u16);
                MoveSelectionDestroyCursorAt(
                    ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                );
                MoveSelectionCreateCursorAt(
                    (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read(),
                    0u8,
                );
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(Some(HandleInputChooseMove));
                MoveSelectionDisplayPPString();
                MoveSelectionDisplayPPNumber();
                MoveSelectionDisplayMoveType();
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 32i32)
                    != 0
                {
                    if (((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) & 1i32)
                        != 0
                    {
                        if ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                            == (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read()) as i32)
                        {
                            MoveSelectionCreateCursorAt(
                                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                                .read(),
                                29u8,
                            );
                        } else {
                            MoveSelectionDestroyCursorAt(
                                ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                            );
                        }
                        let __p1 = (&raw mut gMultiUsePlayerCursor).cast::<u8>();
                        (__p1).write((((((__p1).read()) as i32) ^ 1i32) as u8));
                        PlaySE(5u16);
                        if ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                            == (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                            .read()) as i32)
                        {
                            MoveSelectionCreateCursorAt(
                                ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                0u8,
                            );
                        } else {
                            MoveSelectionCreateCursorAt(
                                ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                27u8,
                            );
                        }
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 16i32)
                        != 0
                    {
                        if (!((((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                            & 1i32)
                            != 0))
                            && ((((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                ^ 1i32)
                                < ((((&raw mut gNumberOfMovesToChoose).cast::<u8>()).read())
                                    as i32))
                        {
                            if ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                == (((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()) as i32)
                            {
                                MoveSelectionCreateCursorAt(
                                    (((&raw mut gMoveSelectionCursor).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read(),
                                    29u8,
                                );
                            } else {
                                MoveSelectionDestroyCursorAt(
                                    ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                );
                            }
                            let __p2 = (&raw mut gMultiUsePlayerCursor).cast::<u8>();
                            (__p2).write((((((__p2).read()) as i32) ^ 1i32) as u8));
                            PlaySE(5u16);
                            if ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                == (((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                .read()) as i32)
                            {
                                MoveSelectionCreateCursorAt(
                                    ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                    0u8,
                                );
                            } else {
                                MoveSelectionCreateCursorAt(
                                    ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                    27u8,
                                );
                            }
                        }
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 64i32)
                            != 0
                        {
                            if (((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                & 2i32)
                                != 0
                            {
                                if ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                    == (((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()) as i32)
                                {
                                    MoveSelectionCreateCursorAt(
                                        (((&raw mut gMoveSelectionCursor).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read(),
                                        29u8,
                                    );
                                } else {
                                    MoveSelectionDestroyCursorAt(
                                        ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                    );
                                }
                                let __p3 = (&raw mut gMultiUsePlayerCursor).cast::<u8>();
                                (__p3).write((((((__p3).read()) as i32) ^ 2i32) as u8));
                                PlaySE(5u16);
                                if ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32)
                                    == (((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()) as i32)
                                {
                                    MoveSelectionCreateCursorAt(
                                        ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                        0u8,
                                    );
                                } else {
                                    MoveSelectionCreateCursorAt(
                                        ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                        27u8,
                                    );
                                }
                            }
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 128i32)
                                != 0
                            {
                                if (!((((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read())
                                    as i32)
                                    & 2i32)
                                    != 0))
                                    && ((((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read())
                                        as i32)
                                        ^ 2i32)
                                        < ((((&raw mut gNumberOfMovesToChoose).cast::<u8>()).read())
                                            as i32))
                                {
                                    if ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read())
                                        as i32)
                                        == (((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read()) as i32)
                                    {
                                        MoveSelectionCreateCursorAt(
                                            (((&raw mut gMoveSelectionCursor).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler).cast::<u8>())
                                                        .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                            .read(),
                                            29u8,
                                        );
                                    } else {
                                        MoveSelectionDestroyCursorAt(
                                            ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                        );
                                    }
                                    let __p4 = (&raw mut gMultiUsePlayerCursor).cast::<u8>();
                                    (__p4).write((((((__p4).read()) as i32) ^ 2i32) as u8));
                                    PlaySE(5u16);
                                    if ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read())
                                        as i32)
                                        == (((((&raw mut gMoveSelectionCursor).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read()) as i32)
                                    {
                                        MoveSelectionCreateCursorAt(
                                            ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                            0u8,
                                        );
                                    } else {
                                        MoveSelectionCreateCursorAt(
                                            ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read(),
                                            27u8,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetLinkBattleEndCallbacks() {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) == 0i32 {
            if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 0i32 {
                m4aSongNumStop(90u16);
                crate::c::bf_write(
                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                    1,
                    1,
                    (0u8) as i32,
                );
                (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(
                    ((&raw mut gPreBattleCallback1).cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                );
                SetMainCallback2(Some(CB2_InitEndLinkBattle));
                if ((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32) == 1i32 {
                    TryPutLinkBattleTvShowOnAir();
                }
                FreeAllWindowBuffers();
            }
        } else {
            if (IsLinkTaskFinished()) != 0 {
                m4aSongNumStop(90u16);
                crate::c::bf_write(
                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                    1,
                    1,
                    (0u8) as i32,
                );
                (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(
                    ((&raw mut gPreBattleCallback1).cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                );
                SetMainCallback2(Some(CB2_InitEndLinkBattle));
                if ((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32) == 1i32 {
                    TryPutLinkBattleTvShowOnAir();
                }
                FreeAllWindowBuffers();
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattleEndCallbacks() {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
                if (IsLinkTaskFinished()) != 0 {
                    if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) == 0i32 {
                        SetCloseLinkCallback();
                    } else {
                        SetLinkStandbyCallback();
                    }
                    ((((&raw mut gBattlerControllerFuncs)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .write(Some(SetLinkBattleEndCallbacks));
                }
            } else {
                m4aSongNumStop(90u16);
                crate::c::bf_write(
                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                    1,
                    1,
                    (0u8) as i32,
                );
                (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(
                    ((&raw mut gPreBattleCallback1).cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
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
            PlayerBufferExecCompleted();
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
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn FreeTrainerSpriteAfterSlide() {
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
            BattleGfxSfxDummy3(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read(),
            );
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
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn Intro_DelayAndEnd() {
    unsafe {
        if (({
            let __p1 = ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(9);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 255i32
        {
            (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(9))
            .write(0u8);
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn Intro_WaitForShinyAnimAndHealthbox() {
    unsafe {
        let mut healthboxAnimDone: u8 = 0u8;
        if (!((IsDoubleBattle()) != 0))
            || (((IsDoubleBattle()) != 0)
                && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0))
        {
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
                healthboxAnimDone = 1u8;
            }
        } else {
            if (core::mem::transmute::<_, usize>(
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
            ) == (SpriteCallbackDummy as *const () as usize))
                && (core::mem::transmute::<_, usize>(
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32)
                                as isize,
                        ))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) == (SpriteCallbackDummy as *const () as usize))
            {
                healthboxAnimDone = 1u8;
            }
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
            if (IsDoubleBattle()) != 0 {
                HandleLowHpMusicChange(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    ((((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
                );
            }
            (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(9))
            .write(3u8);
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(Intro_DelayAndEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn Intro_TryShinyAnimShowHealthbox() {
    unsafe {
        let mut bgmRestored: u32 = 0u32;
        let mut battlerAnimsDone: u32 = 0u32;
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
        if (!((crate::c::bf_read(
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
                != 0))
        {
            if !((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(1),
                7,
                1,
                false,
            ) as u8)
                != 0)
            {
                if ((IsDoubleBattle()) != 0)
                    && (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0))
                {
                    UpdateHealthboxAttribute(
                        (((&raw mut gHealthboxSpriteIds).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32)
                                as isize,
                        ))
                        .read(),
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        ^ 2i32) as isize,
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
            }
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(1),
                7,
                1,
                (1u8) as i32,
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
            .wrapping_add(1),
            6,
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
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(1),
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
                .wrapping_add(1),
                6,
                1,
                false,
            ) as u8)
                != 0)))
            && (!((IsCryPlayingOrClearCrySongs()) != 0))
        {
            if !((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(1),
                5,
                1,
                false,
            ) as u8)
                != 0)
            {
                if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0)
                    && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0)
                {
                    m4aMPlayContinue((&raw mut gMPlayInfo_BGM).cast::<u8>());
                } else {
                    m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 256u16);
                }
            }
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(1),
                5,
                1,
                (1u8) as i32,
            );
            bgmRestored = 1u32;
        }
        if (!((IsDoubleBattle()) != 0))
            || (((IsDoubleBattle()) != 0)
                && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0))
        {
            if (core::mem::transmute::<_, usize>(
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
            ) == (SpriteCallbackDummy as *const () as usize))
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
                battlerAnimsDone = 1u32;
            }
        } else {
            if (((core::mem::transmute::<_, usize>(
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
            ) == (SpriteCallbackDummy as *const () as usize))
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
                ) == (SpriteCallbackDummy as *const () as usize)))
                && (core::mem::transmute::<_, usize>(
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattleControllerData).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32)
                                as isize,
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
                            (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 2i32)
                                as isize,
                        ))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) == (SpriteCallbackDummy as *const () as usize))
            {
                battlerAnimsDone = 1u32;
            }
        }
        if ((bgmRestored) != 0) && ((battlerAnimsDone) != 0) {
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
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(1),
                5,
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
                7,
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
pub(crate) unsafe extern "C" fn SwitchIn_CleanShinyAnimShowSubstitute() {
    unsafe {
        if ((core::mem::transmute::<_, usize>(
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
        ) == (SpriteCallbackDummy as *const () as usize))
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
            CopyBattleSpriteInvisibility(((&raw mut gActiveBattler).cast::<u8>()).read());
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
            FreeSpriteTilesByTag(10233u16);
            FreeSpritePaletteByTag(10233u16);
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
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(SwitchIn_HandleSoundAndEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn SwitchIn_HandleSoundAndEnd() {
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
            6,
            1,
            false,
        ) as u8)
            != 0))
            && (!((IsCryPlayingOrClearCrySongs()) != 0))
        {
            m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 256u16);
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
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn SwitchIn_TryShinyAnimShowHealthbox() {
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
        if (core::mem::transmute::<_, usize>(
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
        ) == (SpriteCallbackDummy as *const () as usize))
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
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(SwitchIn_CleanShinyAnimShowSubstitute));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_PlayerController_RestoreBgmAfterCry(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((IsCryPlayingOrClearCrySongs()) != 0) {
            m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 256u16);
            DestroyTask(taskId);
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
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnInactiveTextPrinter() {
    unsafe {
        if !((IsTextPrinterActive(0u8)) != 0) {
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_GiveExpToMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut monId: u32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8) as u32);
        let mut battler: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        let mut gainedExp: i16 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read();
        if (((IsDoubleBattle()) as i32) == 1i32)
            || (monId
                != ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as u32))
        {
            let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((monId) as i32) as isize * 100);
            let mut species: u16 = ((GetMonData2(mon, 11i32)) as u16);
            let mut level: u8 = ((GetMonData2(mon, 56i32)) as u8);
            let mut currExp: u32 = GetMonData2(mon, 25i32);
            let mut nextLvlExp: u32 = (((((&raw mut gExperienceTables).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(19))
                    .read()) as i32) as isize
                        * 404,
                ))
            .cast::<u32>())
            .wrapping_offset((((level) as i32).wrapping_add(1i32)) as isize))
            .read();
            if (currExp).wrapping_add(((gainedExp) as u32)) >= nextLvlExp {
                let mut savedActiveBattler: u8 = 0u8;
                SetMonData(mon, 25i32, (&raw mut nextLvlExp).cast::<u8>());
                CalculateMonStats(mon);
                gainedExp = ((((gainedExp) as u32).wrapping_sub((nextLvlExp).wrapping_sub(currExp)))
                    as i16);
                savedActiveBattler = ((&raw mut gActiveBattler).cast::<u8>()).read();
                ((&raw mut gActiveBattler).cast::<u8>()).write(battler);
                BtlController_EmitTwoReturnValues(1u8, 11u8, ((gainedExp) as u16));
                ((&raw mut gActiveBattler).cast::<u8>()).write(savedActiveBattler);
                if (((IsDoubleBattle()) as i32) == 1i32)
                    && (((((monId) as u16) as i32)
                        == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32))
                        || ((((monId) as u16) as i32)
                            == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset((((battler) as i32) ^ 2i32) as isize))
                            .read()) as i32)))
                {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LaunchLvlUpAnim));
                } else {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(DestroyExpTaskAndCompleteOnInactiveTextPrinter));
                }
            } else {
                currExp = (currExp).wrapping_add(((gainedExp) as u32));
                SetMonData(mon, 25i32, (&raw mut currExp).cast::<u8>());
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(((battler) as i32) as isize))
                .write(Some(CompleteOnInactiveTextPrinter));
                DestroyTask(taskId);
            }
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_PrepareToGiveExpWithExpBar));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrepareToGiveExpWithExpBar(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut monIndex: u8 = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        let mut gainedExp: i32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32);
        let mut battler: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>())
            .wrapping_offset(((monIndex) as i32) as isize * 100);
        let mut level: u8 = ((GetMonData2(mon, 56i32)) as u8);
        let mut species: u16 = ((GetMonData2(mon, 11i32)) as u16);
        let mut exp: u32 = GetMonData2(mon, 25i32);
        let mut currLvlExp: u32 = (((((&raw mut gExperienceTables).cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gSpeciesInfo).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .wrapping_add(19))
                .read()) as i32) as isize
                    * 404,
            ))
        .cast::<u32>())
        .wrapping_offset(((level) as i32) as isize))
        .read();
        let mut expToNextLvl: u32 = 0u32;
        exp = (exp).wrapping_sub(currLvlExp);
        expToNextLvl = ((((((&raw mut gExperienceTables).cast::<u8>()).wrapping_offset(
            ((((((&raw mut gSpeciesInfo).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(19))
            .read()) as i32) as isize
                * 404,
        ))
        .cast::<u32>())
        .wrapping_offset((((level) as i32).wrapping_add(1i32)) as isize))
        .read())
        .wrapping_sub(currLvlExp);
        SetBattleBarStruct(
            battler,
            (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read(),
            ((expToNextLvl) as i32),
            ((exp) as i32),
            (gainedExp).wrapping_neg(),
        );
        PlaySE(33u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_GiveExpWithExpBar));
    }
}
pub(crate) unsafe extern "C" fn Task_GiveExpWithExpBar(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .read()) as i32)
            < 13i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            let mut monId: u8 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u8);
            let mut gainedExp: i16 = ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read();
            let mut battler: u8 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as u8);
            let mut newExpPoints: i16 = 0i16;
            newExpPoints = ((MoveBattleBar(
                battler,
                (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read(),
                1u8,
                0u8,
            )) as i16);
            SetHealthboxSpriteVisible(
                (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read(),
            );
            if ((newExpPoints) as i32) == (-1i32) {
                let mut level: u8 = 0u8;
                let mut currExp: i32 = 0i32;
                let mut species: u16 = 0u16;
                let mut expOnNextLvl: i32 = 0i32;
                m4aSongNumStop(33u16);
                level = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    56i32,
                )) as u8);
                currExp = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    25i32,
                )) as i32);
                species = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    11i32,
                )) as u16);
                expOnNextLvl = (((((((&raw mut gExperienceTables).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(19))
                    .read()) as i32) as isize
                        * 404,
                ))
                .cast::<u32>())
                .wrapping_offset((((level) as i32).wrapping_add(1i32)) as isize))
                .read()) as i32);
                if (currExp).wrapping_add(((gainedExp) as i32)) >= expOnNextLvl {
                    let mut savedActiveBattler: u8 = 0u8;
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        25i32,
                        (&raw mut expOnNextLvl).cast::<u8>(),
                    );
                    CalculateMonStats(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                    );
                    gainedExp = ((((gainedExp) as i32)
                        .wrapping_sub((expOnNextLvl).wrapping_sub(currExp)))
                        as i16);
                    savedActiveBattler = ((&raw mut gActiveBattler).cast::<u8>()).read();
                    ((&raw mut gActiveBattler).cast::<u8>()).write(battler);
                    BtlController_EmitTwoReturnValues(1u8, 11u8, ((gainedExp) as u16));
                    ((&raw mut gActiveBattler).cast::<u8>()).write(savedActiveBattler);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LaunchLvlUpAnim));
                } else {
                    currExp = (currExp).wrapping_add(((gainedExp) as i32));
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 100),
                        25i32,
                        (&raw mut currExp).cast::<u8>(),
                    );
                    ((((&raw mut gBattlerControllerFuncs)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                    .wrapping_offset(((battler) as i32) as isize))
                    .write(Some(CompleteOnInactiveTextPrinter));
                    DestroyTask(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LaunchLvlUpAnim(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut battler: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        let mut monIndex: u8 = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        if (((IsDoubleBattle()) as i32) == 1i32)
            && (((monIndex) as i32)
                == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset((((battler) as i32) ^ 2i32) as isize))
                .read()) as i32))
        {
            battler = ((((battler) as i32) ^ 2i32) as u8);
        }
        InitAndLaunchSpecialAnimation(battler, battler, battler, 0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_UpdateLvlInHealthbox));
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateLvlInHealthbox(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut battler: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        if !((crate::c::bf_read(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 12))
            .wrapping_add(0),
            6,
            1,
            false,
        ) as u8)
            != 0)
        {
            let mut monIndex: u8 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u8);
            GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((monIndex) as i32) as isize * 100),
                56i32,
            );
            if (((IsDoubleBattle()) as i32) == 1i32)
                && (((monIndex) as i32)
                    == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset((((battler) as i32) ^ 2i32) as isize))
                    .read()) as i32))
            {
                UpdateHealthboxAttribute(
                    (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                        .wrapping_offset((((battler) as i32) ^ 2i32) as isize))
                    .read(),
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monIndex) as i32) as isize * 100),
                    0u8,
                );
            } else {
                UpdateHealthboxAttribute(
                    (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read(),
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monIndex) as i32) as isize * 100),
                    0u8,
                );
            }
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(DestroyExpTaskAndCompleteOnInactiveTextPrinter));
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyExpTaskAndCompleteOnInactiveTextPrinter(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut monIndex: u8 = 0u8;
        let mut battler: u8 = 0u8;
        monIndex = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((monIndex) as i32) as isize * 100),
            56i32,
        );
        battler = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((battler) as i32) as isize))
        .write(Some(CompleteOnInactiveTextPrinter));
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn FreeMonSpriteAfterFaintAnim() {
    unsafe {
        if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .read()) as i32)
            .wrapping_add(
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32),
            )
            > 160i32
        {
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
            BattleGfxSfxDummy2(species);
            FreeOamMatrix(
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
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
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn FreeMonSpriteAfterSwitchOutAnim() {
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
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnInactiveTextPrinter2() {
    unsafe {
        if !((IsTextPrinterActive(0u8)) != 0) {
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn OpenPartyMenuToChooseMon() {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            let mut caseId: u8 = 0u8;
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(WaitForMonSelection));
            caseId = (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattleControllerData).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u8);
            DestroyTask(
                (((&raw mut gBattleControllerData).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
            );
            FreeAllWindowBuffers();
            OpenPartyMenuInBattle(caseId);
        }
    }
}
pub(crate) unsafe extern "C" fn WaitForMonSelection() {
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
            if ((((&raw mut gPartyMenuUseExitCallback).cast::<u8>()).read()) as i32) == 1i32 {
                BtlController_EmitChosenMonReturnValue(
                    1u8,
                    ((&raw mut gSelectedMonPartyId).cast::<u8>()).read(),
                    (&raw mut gBattlePartyCurrentOrder).cast::<u8>(),
                );
            } else {
                BtlController_EmitChosenMonReturnValue(1u8, 6u8, core::ptr::null_mut());
            }
            if ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                & 15i32)
                == 1i32
            {
                PrintLinkStandbyMsg();
            }
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn OpenBagAndChooseItem() {
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
            .write(Some(CompleteWhenChoseItem));
            ReshowBattleScreenDummy();
            FreeAllWindowBuffers();
            CB2_BagMenuFromBattle();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteWhenChoseItem() {
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
            PlayerBufferExecCompleted();
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
            PlayerBufferExecCompleted();
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
            PlayerBufferExecCompleted();
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
pub(crate) unsafe extern "C" fn PlayerHandleYesNoInput() {
    unsafe {
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0)
            && (((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) != 0i32)
        {
            PlaySE(5u16);
            BattleDestroyYesNoCursorAt(((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read());
            ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).write(0u8);
            BattleCreateYesNoCursorAt(0u8);
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0)
            && (((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) == 0i32)
        {
            PlaySE(5u16);
            BattleDestroyYesNoCursorAt(((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read());
            ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).write(1u8);
            BattleCreateYesNoCursorAt(1u8);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            HandleBattleWindow(24u8, 8u8, 29u8, 13u8, 1u8);
            PlaySE(5u16);
            if ((((&raw mut gMultiUsePlayerCursor).cast::<u8>()).read()) as i32) != 0i32 {
                BtlController_EmitTwoReturnValues(1u8, 14u8, 0u16);
            } else {
                BtlController_EmitTwoReturnValues(1u8, 13u8, 0u16);
            }
            PlayerBufferExecCompleted();
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            HandleBattleWindow(24u8, 8u8, 29u8, 13u8, 1u8);
            PlaySE(5u16);
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn MoveSelectionDisplayMoveNames() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut moveInfo: *mut u8 = ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(4);
        ((&raw mut gNumberOfMovesToChoose).cast::<u8>()).write(0u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    MoveSelectionDestroyCursorAt(((i) as u8));
                    StringCopy(
                        (&raw mut gDisplayedStringBattle).cast::<u8>(),
                        (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                            (((((moveInfo).cast::<u16>()).wrapping_offset((i) as isize)).read())
                                as i32) as isize
                                * 13,
                        ))
                        .cast::<u8>(),
                    );
                    BattlePutTextOnWindow(
                        (&raw mut gDisplayedStringBattle).cast::<u8>(),
                        (((i).wrapping_add(3i32)) as u8),
                    );
                    if (((((moveInfo).cast::<u16>()).wrapping_offset((i) as isize)).read()) as i32)
                        != 0i32
                    {
                        let __p1 = (&raw mut gNumberOfMovesToChoose).cast::<u8>();
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MoveSelectionDisplayPPString() {
    unsafe {
        StringCopy(
            (&raw mut gDisplayedStringBattle).cast::<u8>(),
            (&raw mut gText_MoveInterfacePP).cast::<u8>(),
        );
        BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 7u8);
    }
}
pub(crate) unsafe extern "C" fn MoveSelectionDisplayPPNumber() {
    unsafe {
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        let mut moveInfo: *mut u8 = core::ptr::null_mut();
        if (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(2))
        .read()) as i32)
            == 1i32
        {
            return;
        }
        SetPPNumbersPaletteInMoveSelection();
        moveInfo = ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(4);
        txtPtr = ConvertIntToDecimalStringN(
            (&raw mut gDisplayedStringBattle).cast::<u8>(),
            ((((((moveInfo).wrapping_add(8)).cast::<u8>()).wrapping_offset(
                (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize,
            ))
            .read()) as i32),
            1i32,
            2u8,
        );
        ({
            let __t1 = txtPtr;
            txtPtr = (txtPtr).wrapping_offset(1);
            __t1
        })
        .write(186u8);
        ConvertIntToDecimalStringN(
            txtPtr,
            ((((((moveInfo).wrapping_add(12)).cast::<u8>()).wrapping_offset(
                (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize,
            ))
            .read()) as i32),
            1i32,
            2u8,
        );
        BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 9u8);
    }
}
pub(crate) unsafe extern "C" fn MoveSelectionDisplayMoveType() {
    unsafe {
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        let mut moveInfo: *mut u8 = ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(4);
        txtPtr = StringCopy(
            (&raw mut gDisplayedStringBattle).cast::<u8>(),
            (&raw mut gText_MoveInterfaceType).cast::<u8>(),
        );
        ({
            let __t1 = txtPtr;
            txtPtr = (txtPtr).wrapping_offset(1);
            __t1
        })
        .write(252u8);
        ({
            let __t2 = txtPtr;
            txtPtr = (txtPtr).wrapping_offset(1);
            __t2
        })
        .write(6u8);
        ({
            let __t3 = txtPtr;
            txtPtr = (txtPtr).wrapping_offset(1);
            __t3
        })
        .write(1u8);
        StringCopy(
            txtPtr,
            (((&raw mut gTypeNames).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                    (((((moveInfo).cast::<u16>()).wrapping_offset(
                        (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 12,
                ))
                .wrapping_add(2))
                .read()) as i32) as isize
                    * 7,
            ))
            .cast::<u8>(),
        );
        BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 10u8);
    }
}
pub(crate) unsafe extern "C" fn MoveSelectionCreateCursorAt(cursorPosition: u8, baseTileNum: u8) {
    unsafe {
        let mut cursorPosition = cursorPosition;
        let mut baseTileNum = baseTileNum;
        let mut src = crate::ffi::Align4([0u8; 4]);
        ((&raw mut src).cast::<u16>()).write(((((baseTileNum) as i32).wrapping_add(1i32)) as u16));
        (((&raw mut src).cast::<u16>()).wrapping_offset(1))
            .write(((((baseTileNum) as i32).wrapping_add(2i32)) as u16));
        CopyToBgTilemapBufferRect_ChangePalette(
            0u8,
            ((&raw mut src).cast::<u16>()).cast::<u8>(),
            ((((9i32).wrapping_mul((((cursorPosition) as i32) & 1i32))).wrapping_add(1i32)) as u8),
            (((55i32).wrapping_add((((cursorPosition) as i32) & 2i32))) as u8),
            1u8,
            2u8,
            17u8,
        );
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn MoveSelectionDestroyCursorAt(cursorPosition: u8) {
    unsafe {
        let mut cursorPosition = cursorPosition;
        let mut src = crate::ffi::Align4([0u8; 4]);
        ((&raw mut src).cast::<u16>()).write(4118u16);
        (((&raw mut src).cast::<u16>()).wrapping_offset(1)).write(4118u16);
        CopyToBgTilemapBufferRect_ChangePalette(
            0u8,
            ((&raw mut src).cast::<u16>()).cast::<u8>(),
            ((((9i32).wrapping_mul((((cursorPosition) as i32) & 1i32))).wrapping_add(1i32)) as u8),
            (((55i32).wrapping_add((((cursorPosition) as i32) & 2i32))) as u8),
            1u8,
            2u8,
            17u8,
        );
        CopyBgTilemapBufferToVram(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ActionSelectionCreateCursorAt(cursorPosition: u8, baseTileNum: u8) {
    unsafe {
        let mut cursorPosition = cursorPosition;
        let mut baseTileNum = baseTileNum;
        let mut src = crate::ffi::Align4([0u8; 4]);
        ((&raw mut src).cast::<u16>()).write(1u16);
        (((&raw mut src).cast::<u16>()).wrapping_offset(1)).write(2u16);
        CopyToBgTilemapBufferRect_ChangePalette(
            0u8,
            ((&raw mut src).cast::<u16>()).cast::<u8>(),
            ((((7i32).wrapping_mul((((cursorPosition) as i32) & 1i32))).wrapping_add(16i32)) as u8),
            (((35i32).wrapping_add((((cursorPosition) as i32) & 2i32))) as u8),
            1u8,
            2u8,
            17u8,
        );
        CopyBgTilemapBufferToVram(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ActionSelectionDestroyCursorAt(cursorPosition: u8) {
    unsafe {
        let mut cursorPosition = cursorPosition;
        let mut src = crate::ffi::Align4([0u8; 4]);
        ((&raw mut src).cast::<u16>()).write(4118u16);
        (((&raw mut src).cast::<u16>()).wrapping_offset(1)).write(4118u16);
        CopyToBgTilemapBufferRect_ChangePalette(
            0u8,
            ((&raw mut src).cast::<u16>()).cast::<u8>(),
            ((((7i32).wrapping_mul((((cursorPosition) as i32) & 1i32))).wrapping_add(16i32)) as u8),
            (((35i32).wrapping_add((((cursorPosition) as i32) & 2i32))) as u8),
            1u8,
            2u8,
            17u8,
        );
        CopyBgTilemapBufferToVram(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_SetUpReshowBattleScreenAfterMenu() {
    unsafe {
        SetMainCallback2(Some(ReshowBattleScreenAfterMenu));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_SetUpReshowBattleScreenAfterMenu2() {
    unsafe {
        SetMainCallback2(Some(ReshowBattleScreenAfterMenu));
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
            PlayerBufferExecCompleted();
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
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn PrintLinkStandbyMsg() {
    unsafe {
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
            BattlePutTextOnWindow((&raw mut gText_LinkStandby).cast::<u8>(), 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleGetMonData() {
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
            size = (size).wrapping_add(CopyPlayerMonData(
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
                            size = (size).wrapping_add(CopyPlayerMonData(
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
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CopyPlayerMonData(monId: u8, dst: *mut u8) -> u32 {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerHandleGetRawMonData() {
    unsafe {
        let mut battleMon = crate::ffi::Align4([0u8; 88]);
        let mut src: *mut u8 = (((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 100,
        ))
        .wrapping_offset(
            (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32) as isize,
        );
        let mut dst: *mut u8 = ((&raw mut battleMon).cast::<u8>()).wrapping_offset(
            (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32) as isize,
        );
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((dst).wrapping_offset(((i) as i32) as isize))
                        .write(((src).wrapping_offset(((i) as i32) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        BtlController_EmitDataTransfer(
            1u8,
            (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read()) as u16),
            dst,
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleSetMonData() {
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
            SetPlayerMonData(
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
                            SetPlayerMonData(i);
                        }
                        monToCheck = ((((monToCheck) as i32) >> 1) as u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SetPlayerMonData(monId: u8) {
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
pub(crate) unsafe extern "C" fn PlayerHandleSetRawMonData() {
    unsafe {
        let mut dst: *mut u8 = (((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 100,
        ))
        .wrapping_offset(
            (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32) as isize,
        );
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((dst).wrapping_offset(((i) as i32) as isize)).write(
                        (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 512,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(((3i32).wrapping_add(((i) as i32))) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleLoadMonSprite() {
    unsafe {
        BattleLoadPlayerMonSpriteGfx(
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
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(CompleteOnBankSpritePosX_0));
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleSwitchInAnim() {
    unsafe {
        ClearTemporarySpeciesSpriteData(
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read(),
        );
        ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(
            (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as u16),
        );
        BattleLoadPlayerMonSpriteGfx(
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
        (((&raw mut gActionSelectionCursor).cast::<u8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(0u8);
        (((&raw mut gMoveSelectionCursor).cast::<u8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(0u8);
        StartSendOutAnim(
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read(),
        );
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(SwitchIn_TryShinyAnimShowHealthbox));
    }
}
pub(crate) unsafe extern "C" fn StartSendOutAnim(battler: u8, dontClearSubstituteBit: u8) {
    unsafe {
        let mut battler = battler;
        let mut dontClearSubstituteBit = dontClearSubstituteBit;
        let mut species: u16 = 0u16;
        ClearTemporarySpeciesSpriteData(battler, dontClearSubstituteBit);
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
pub(crate) unsafe extern "C" fn PlayerHandleReturnMonToBall() {
    unsafe {
        if !(((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read())
            != 0)
        {
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
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn DoSwitchOutAnimation() {
    unsafe {
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
                    (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(4))
                    .write(0u8);
                    InitAndLaunchSpecialAnimation(
                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                        1u8,
                    );
                    ((((&raw mut gBattlerControllerFuncs)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .write(Some(FreeMonSpriteAfterSwitchOutAnim));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleDrawTrainerPic() {
    unsafe {
        let mut xPos: i16 = 0i16;
        let mut yPos: i16 = 0i16;
        let mut trainerPicId: u32 = 0u32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
            if ((((((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
            .cast::<u16>())
            .read()) as i32)
                & 255i32)
                == 4i32)
                || ((((((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                .cast::<u16>())
                .read()) as i32)
                    & 255i32)
                    == 5i32)
            {
                trainerPicId = ((((((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                .wrapping_add(19))
                .read()) as i32)
                    .wrapping_add(2i32)) as u32);
            } else {
                if ((((((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                .cast::<u16>())
                .read()) as i32)
                    & 255i32)
                    == 2i32)
                    || ((((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                    .cast::<u16>())
                    .read()) as i32)
                        & 255i32)
                        == 1i32)
                {
                    trainerPicId = ((((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                    .wrapping_add(19))
                    .read()) as i32)
                        .wrapping_add(4i32)) as u32);
                } else {
                    trainerPicId = ((((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                    .wrapping_add(19))
                    .read()) as i32)
                        .wrapping_add(0i32)) as u32);
                }
            }
        } else {
            trainerPicId = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(8))
            .read()) as u32);
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
            if (((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32)
                & 2i32)
                != 0i32
            {
                xPos = 90i16;
            } else {
                xPos = 32i16;
            }
            if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4194304u32) != 0)
                && (((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32) != 3075i32)
            {
                xPos = 90i16;
                yPos = (((((8i32).wrapping_sub(
                    (((((&raw mut gTrainerFrontPicCoords).cast::<u8>())
                        .wrapping_offset(((trainerPicId) as i32) as isize * 4))
                    .read()) as i32),
                ))
                .wrapping_mul(4i32))
                .wrapping_add(80i32)) as i16);
            } else {
                yPos = (((((8i32).wrapping_sub(
                    (((((&raw mut gTrainerBackPicCoords).cast::<u8>())
                        .wrapping_offset(((trainerPicId) as i32) as isize * 4))
                    .read()) as i32),
                ))
                .wrapping_mul(4i32))
                .wrapping_add(80i32)) as i16);
            }
        } else {
            xPos = 80i16;
            yPos = (((((8i32).wrapping_sub(
                (((((&raw mut gTrainerBackPicCoords).cast::<u8>())
                    .wrapping_offset(((trainerPicId) as i32) as isize * 4))
                .read()) as i32),
            ))
            .wrapping_mul(4i32))
            .wrapping_add(80i32)) as i16);
        }
        if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4194304u32) != 0)
            && (((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32) != 3075i32)
        {
            trainerPicId = ((PlayerGenderToFrontTrainerPicId(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read(),
            )) as u32);
            DecompressTrainerFrontPic(
                ((trainerPicId) as u16),
                ((&raw mut gActiveBattler).cast::<u8>()).read(),
            );
            SetMultiuseSpriteTemplateToTrainerFront(
                ((trainerPicId) as u16),
                GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()),
            );
            (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .write(CreateSprite(
                (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
                xPos,
                yPos,
                GetBattlerSpriteSubpriority(((&raw mut gActiveBattler).cast::<u8>()).read()),
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
                ((IndexOfSpritePaletteTag(
                    ((((&raw mut gTrainerFrontPicPaletteTable).cast::<u8>())
                        .wrapping_offset(((trainerPicId) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read(),
                )) as u16) as i32,
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
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(48i16);
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
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(1),
                0,
                2,
                (0u32) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(63),
                0,
                1,
                (1u16) as i32,
            );
        } else {
            DecompressTrainerBackPic(
                ((trainerPicId) as u16),
                ((&raw mut gActiveBattler).cast::<u8>()).read(),
            );
            SetMultiuseSpriteTemplateToTrainerBack(
                ((trainerPicId) as u16),
                GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()),
            );
            (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .write(CreateSprite(
                (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
                xPos,
                yPos,
                GetBattlerSpriteSubpriority(((&raw mut gActiveBattler).cast::<u8>()).read()),
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
        }
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(CompleteOnBattlerSpriteCallbackDummy));
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleTrainerSlide() {
    unsafe {
        let mut trainerPicId: u32 = 0u32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
            if ((((((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
            .cast::<u16>())
            .read()) as i32)
                & 255i32)
                == 4i32)
                || ((((((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                .cast::<u16>())
                .read()) as i32)
                    & 255i32)
                    == 5i32)
            {
                trainerPicId = ((((((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                .wrapping_add(19))
                .read()) as i32)
                    .wrapping_add(2i32)) as u32);
            } else {
                if ((((((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                .cast::<u16>())
                .read()) as i32)
                    & 255i32)
                    == 2i32)
                    || ((((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                    .cast::<u16>())
                    .read()) as i32)
                        & 255i32)
                        == 1i32)
                {
                    trainerPicId = ((((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                    .wrapping_add(19))
                    .read()) as i32)
                        .wrapping_add(4i32)) as u32);
                } else {
                    trainerPicId = ((((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                    .wrapping_add(19))
                    .read()) as i32)
                        .wrapping_add(0i32)) as u32);
                }
            }
        } else {
            trainerPicId = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(8))
            .read()) as i32)
                .wrapping_add(0i32)) as u32);
        }
        DecompressTrainerBackPic(
            ((trainerPicId) as u16),
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
        );
        SetMultiuseSpriteTemplateToTrainerBack(
            ((trainerPicId) as u16),
            GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()),
        );
        (((&raw mut gBattlerSpriteIds).cast::<u8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(CreateSprite(
            (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
            80i16,
            (((((8i32).wrapping_sub(
                (((((&raw mut gTrainerBackPicCoords).cast::<u8>())
                    .wrapping_offset(((trainerPicId) as i32) as isize * 4))
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
pub(crate) unsafe extern "C" fn PlayerHandleTrainerSlideBack() {
    unsafe {
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
        StoreSpriteCallbackInData6(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ),
            Some(SpriteCallbackDummy),
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
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(FreeTrainerSpriteAfterSlide));
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleFaintAnimation() {
    unsafe {
        if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
        ))
        .wrapping_add(4))
        .read()) as i32)
            == 0i32
        {
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
            let __p1 = ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(4);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
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
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(4))
                .write(0u8);
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
                PlaySE12WithPanning(16u16, (-64i8));
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
                .write(5i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_FaintSlideAnim));
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(Some(FreeMonSpriteAfterFaintAnim));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandlePaletteFade() {
    unsafe {
        BeginNormalPaletteFade(4294967295u32, 2i8, 0u8, 16u8, 0u16);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleSuccessBallThrowAnim() {
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
            3u8,
        );
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(CompleteOnSpecialAnimDone));
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleBallThrowAnim() {
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
            3u8,
        );
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(CompleteOnSpecialAnimDone));
    }
}
pub(crate) unsafe extern "C" fn PlayerHandlePause() {
    unsafe {
        let mut timer: u8 = (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read();
        'l1: loop {
            if !(((timer) as i32) != 0i32) {
                break 'l1;
            }
            timer = (timer).wrapping_sub(1);
        }
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleMoveAnimation() {
    unsafe {
        if !((IsBattleSEPlaying(((&raw mut gActiveBattler).cast::<u8>()).read())) != 0) {
            let mut r#move: u16 = (((((((((&raw mut gBattleBufferA).cast::<u8>())
                .wrapping_offset(
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
            ((((&raw mut gTransformedPersonalities).cast::<u32>()).cast::<u32>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .write(
                ((((&raw mut gAnimDisableStructPtr).cast::<*mut u8>()).read()).cast::<u32>())
                    .read(),
            );
            if (IsMoveWithoutAnimation(r#move, ((&raw mut gAnimMoveTurn).cast::<u8>()).read())) != 0
            {
                PlayerBufferExecCompleted();
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
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(Some(PlayerDoMoveAnimation));
                BattleTv_SetDataBasedOnMove(
                    r#move,
                    ((&raw mut gWeatherMoveAnim).cast::<u16>()).read(),
                    ((&raw mut gAnimDisableStructPtr).cast::<*mut u8>()).read(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerDoMoveAnimation() {
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
        let mut multihit: u8 = (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(11))
        .read();
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
                if ((crate::c::bf_read(
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
                    != 0)
                    && (!((crate::c::bf_read(
                        ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 4,
                        ))
                        .wrapping_add(0),
                        3,
                        1,
                        false,
                    ) as u16)
                        != 0))
                {
                    crate::c::bf_write(
                        ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 4,
                        ))
                        .wrapping_add(0),
                        3,
                        1,
                        (1u16) as i32,
                    );
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
                    if ((crate::c::bf_read(
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
                        != 0)
                        && (((multihit) as i32) < 2i32)
                    {
                        InitAndLaunchSpecialAnimation(
                            ((&raw mut gActiveBattler).cast::<u8>()).read(),
                            ((&raw mut gActiveBattler).cast::<u8>()).read(),
                            ((&raw mut gActiveBattler).cast::<u8>()).read(),
                            6u8,
                        );
                        crate::c::bf_write(
                            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 4,
                            ))
                            .wrapping_add(0),
                            3,
                            1,
                            (0u16) as i32,
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
                    PlayerBufferExecCompleted();
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandlePrintString() {
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
        .write(Some(CompleteOnInactiveTextPrinter2));
        BattleTv_SetDataBasedOnString((stringId).read());
        BattleArena_DeductSkillPoints(
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            (stringId).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn PlayerHandlePrintSelectionString() {
    unsafe {
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
            PlayerHandlePrintString();
        } else {
            PlayerBufferExecCompleted();
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
pub(crate) unsafe extern "C" fn PlayerHandleChooseAction() {
    unsafe {
        let mut i: i32 = 0i32;
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(HandleChooseActionAfterDma3));
        BattleTv_ClearExplosionFaintCause();
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
            (&raw mut gText_WhatWillPkmnDo).cast::<u8>(),
        );
        BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 1u8);
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleYesNoBox() {
    unsafe {
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
            HandleBattleWindow(24u8, 8u8, 29u8, 13u8, 0u8);
            BattlePutTextOnWindow((&raw mut gText_BattleYesNoChoice).cast::<u8>(), 12u8);
            ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).write(1u8);
            BattleCreateYesNoCursorAt(1u8);
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(PlayerHandleYesNoInput));
        } else {
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn HandleChooseMoveAfterDma3() {
    unsafe {
        if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(320u16);
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(HandleInputChooseMove));
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerChooseMoveInBattlePalace() {
    unsafe {
        if (({
            let __p1 = (((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(664))
                .cast::<i8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            ((&raw mut gBattlePalaceMoveSelectionRngValue).cast::<u32>())
                .write(((&raw mut gRngValue).cast::<u32>()).read());
            BtlController_EmitTwoReturnValues(1u8, 10u8, ChooseMoveAndTargetInBattlePalace());
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleChooseMove() {
    unsafe {
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 131072u32) != 0 {
            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(664))
                .cast::<i8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(8i8);
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(PlayerChooseMoveInBattlePalace));
        } else {
            InitMoveSelectionsVarsAndStrings();
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(HandleChooseMoveAfterDma3));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMoveSelectionsVarsAndStrings() {
    unsafe {
        MoveSelectionDisplayMoveNames();
        ((&raw mut gMultiUsePlayerCursor).cast::<u8>()).write(255u8);
        MoveSelectionCreateCursorAt(
            (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
            0u8,
        );
        MoveSelectionDisplayPPString();
        MoveSelectionDisplayPPNumber();
        MoveSelectionDisplayMoveType();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleChooseItem() {
    unsafe {
        let mut i: i32 = 0i32;
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(OpenBagAndChooseItem));
        ((&raw mut gBattlerInMenuId).cast::<u8>())
            .write(((&raw mut gActiveBattler).cast::<u8>()).read());
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(3u32, 1u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut gBattlePartyCurrentOrder).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 512,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(((1i32).wrapping_add(i)) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleChoosePokemon() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(3u32, 1u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut gBattlePartyCurrentOrder).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 512,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(((4i32).wrapping_add(i)) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 262144u32) != 0)
            && (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                & 15i32)
                != 2i32)
        {
            BtlController_EmitChosenMonReturnValue(
                1u8,
                ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32)
                    .wrapping_add(1i32)) as u8),
                (&raw mut gBattlePartyCurrentOrder).cast::<u8>(),
            );
            PlayerBufferExecCompleted();
        } else {
            (((&raw mut gBattleControllerData).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .write(CreateTask(Some(TaskDummy), 255u8));
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattleControllerData).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .write(
                (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    & 15i32) as i16),
            );
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(73)).write(
                (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    >> 4) as u8),
            );
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(139)).write(
                (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(2))
                .read(),
            );
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(176)).write(
                (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(3))
                .read(),
            );
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(OpenPartyMenuToChooseMon));
            ((&raw mut gBattlerInMenuId).cast::<u8>())
                .write(((&raw mut gActiveBattler).cast::<u8>()).read());
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleCmd23() {
    unsafe {
        BattleStopLowHpSound();
        BeginNormalPaletteFade(4294967295u32, 2i8, 0u8, 16u8, 0u16);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleHealthBarUpdate() {
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
        if ((hpVal) as i32) > 0i32 {
            let __p1 = (&raw mut gPlayerPartyLostHP).cast::<u32>();
            (__p1).write(((__p1).read()).wrapping_add(((hpVal) as u32)));
        }
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
pub(crate) unsafe extern "C" fn PlayerHandleExpUpdate() {
    unsafe {
        let mut monId: u8 = (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read();
        if GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((monId) as i32) as isize * 100),
            56i32,
        ) >= 100u32
        {
            PlayerBufferExecCompleted();
        } else {
            let mut expPointsToGive: i16 = 0i16;
            let mut taskId: u8 = 0u8;
            LoadBattleBarGfx(1u8);
            GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((monId) as i32) as isize * 100),
                11i32,
            );
            expPointsToGive = (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32)
                | (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(2))
                .wrapping_offset(1))
                .read()) as i32)
                    << 8)) as i16);
            taskId = CreateTask(Some(Task_GiveExpToMon), 10u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(((monId) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(expPointsToGive);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i16));
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(BattleControllerDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleStatusIconUpdate() {
    unsafe {
        if !((IsBattleSEPlaying(((&raw mut gActiveBattler).cast::<u8>()).read())) != 0) {
            let mut battler: u8 = 0u8;
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
                9u8,
            );
            battler = ((&raw mut gActiveBattler).cast::<u8>()).read();
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(0),
                4,
                1,
                (0u8) as i32,
            );
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(CompleteOnFinishedStatusAnimation));
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleStatusAnimation() {
    unsafe {
        if !((IsBattleSEPlaying(((&raw mut gActiveBattler).cast::<u8>()).read())) != 0) {
            InitAndLaunchChosenStatusAnimation(
                (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(1))
                .read(),
                (((((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
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
                        << 8))
                    | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(4))
                    .read()) as i32)
                        << 16))
                    | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(5))
                    .read()) as i32)
                        << 24)) as u32),
            );
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(CompleteOnFinishedStatusAnimation));
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleStatusXor() {
    unsafe {
        let mut val: u8 = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32) as isize
                    * 100,
            ),
            55i32,
        ) ^ (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read()) as u32)) as u8);
        SetMonData(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                .read()) as i32) as isize
                    * 100,
            ),
            55i32,
            &raw mut val,
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleDataTransfer() {
    unsafe {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleDMA3Transfer() {
    unsafe {
        let mut dstArg: u32 = (((((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
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
                << 8))
            | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(3))
            .read()) as i32)
                << 16))
            | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(4))
            .read()) as i32)
                << 24)) as u32);
        let mut sizeArg: u16 = (((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(5))
        .read()) as i32)
            | ((((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(6))
            .read()) as i32)
                << 8)) as u16);
        {
            let mut _src: *mut u8 = ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(7);
            let mut _dest: *mut u8 = ((dstArg) as usize as *mut u8);
            let mut _size: u32 = ((sizeArg) as u32);
            'l1: loop {
                if !((1i32) != 0) {
                    break 'l1;
                }
                if _size <= 4096u32 {
                    'l2: loop {
                        'l3: {
                            'l4: loop {
                                'l5: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            ((_dest) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (2147483648u32
                                                | crate::c::div_u32(
                                                    _size,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                )),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l2;
                        }
                    }
                    break 'l1;
                }
                'l6: loop {
                    'l7: {
                        'l8: loop {
                            'l9: {
                                {
                                    let mut dmaRegs: *mut u32 =
                                        ((67109076i32) as usize as *mut u32);
                                    crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(1),
                                        ((_dest) as usize as u32),
                                    );
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(2),
                                        (((-2147483648i32)
                                            | crate::c::div_i32(
                                                4096i32,
                                                crate::c::div_i32(16i32, 8i32),
                                            )) as u32),
                                    );
                                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l8;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l6;
                    }
                }
                _src = (_src).wrapping_offset(4096);
                _dest = (_dest).wrapping_offset(4096);
                _size = (_size).wrapping_sub(4096u32);
            }
        }
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandlePlayBGM() {
    unsafe {
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
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleCmd32() {
    unsafe {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleTwoReturnValues() {
    unsafe {
        BtlController_EmitTwoReturnValues(1u8, 0u8, 0u16);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleChosenMonReturnValue() {
    unsafe {
        BtlController_EmitChosenMonReturnValue(1u8, 0u8, core::ptr::null_mut());
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleOneReturnValue() {
    unsafe {
        BtlController_EmitOneReturnValue(1u8, 0u16);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleOneReturnValue_Duplicate() {
    unsafe {
        BtlController_EmitOneReturnValue_Duplicate(1u8, 0u16);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleClearUnkVar() {
    unsafe {
        crate::c::bf_write(
            ((&raw mut gUnusedControllerStruct).cast::<u8>()).wrapping_add(0),
            0,
            7,
            (0u8) as i32,
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleSetUnkVar() {
    unsafe {
        crate::c::bf_write(
            ((&raw mut gUnusedControllerStruct).cast::<u8>()).wrapping_add(0),
            0,
            7,
            ((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32,
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleClearUnkFlag() {
    unsafe {
        crate::c::bf_write(
            ((&raw mut gUnusedControllerStruct).cast::<u8>()).wrapping_add(0),
            7,
            1,
            (0u8) as i32,
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleToggleUnkFlag() {
    unsafe {
        crate::c::bf_write(
            ((&raw mut gUnusedControllerStruct).cast::<u8>()).wrapping_add(0),
            7,
            1,
            ((((crate::c::bf_read(
                ((&raw mut gUnusedControllerStruct).cast::<u8>()).wrapping_add(0),
                7,
                1,
                false,
            ) as u8) as i32)
                ^ 1i32) as u8) as i32,
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleHitAnimation() {
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
            PlayerBufferExecCompleted();
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
pub(crate) unsafe extern "C" fn PlayerHandleCantSwitch() {
    unsafe {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandlePlaySE() {
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
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandlePlayFanfareOrBGM() {
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
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleFaintingCry() {
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
        PlayCry_ByMode(species, (-25i8), 5u8);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleIntroSlide() {
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
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleIntroTrainerBallThrow() {
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
            ((((&raw mut gTrainerBackPicPaletteTable).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                    as i32) as isize
                    * 8,
            ))
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_FreePlayerSpriteLoadMonSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u8);
        FreeSpriteOamMatrix(sprite);
        FreeSpritePaletteByTag(GetSpritePaletteTagByPaletteNum(
            ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as u8),
        ));
        DestroySprite(sprite);
        BattleLoadPlayerMonSpriteGfx(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 100,
            ),
            battler,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 68,
            ),
            0u8,
        );
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
            let mut savedActiveBattler: u8 = ((&raw mut gActiveBattler).cast::<u8>()).read();
            ((&raw mut gActiveBattler).cast::<u8>()).write(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8),
            );
            if (!((IsDoubleBattle()) != 0))
                || ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0)
            {
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
                StartSendOutAnim(((&raw mut gActiveBattler).cast::<u8>()).read(), 0u8);
            } else {
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
                StartSendOutAnim(((&raw mut gActiveBattler).cast::<u8>()).read(), 0u8);
                let __p2 = (&raw mut gActiveBattler).cast::<u8>();
                (__p2).write((((((__p2).read()) as i32) ^ 2i32) as u8));
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
                BattleLoadPlayerMonSpriteGfx(
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
                StartSendOutAnim(((&raw mut gActiveBattler).cast::<u8>()).read(), 0u8);
                let __p3 = (&raw mut gActiveBattler).cast::<u8>();
                (__p3).write((((((__p3).read()) as i32) ^ 2i32) as u8));
            }
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(Intro_TryShinyAnimShowHealthbox));
            ((&raw mut gActiveBattler).cast::<u8>()).write(savedActiveBattler);
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleDrawPartyStatusSummary() {
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
            PlayerBufferExecCompleted();
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
            (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(5))
            .write(0u8);
            if (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32)
                != 0i32
            {
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(5))
                .write(93u8);
            }
            ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(Some(EndDrawPartyStatusSummary));
        }
    }
}
pub(crate) unsafe extern "C" fn EndDrawPartyStatusSummary() {
    unsafe {
        if (({
            let __p1 = ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(5);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            > 92i32
        {
            (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(5))
            .write(0u8);
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleHidePartyStatusSummary() {
    unsafe {
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
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleEndBounceEffect() {
    unsafe {
        EndBounceEffect(((&raw mut gActiveBattler).cast::<u8>()).read(), 1u8);
        EndBounceEffect(((&raw mut gActiveBattler).cast::<u8>()).read(), 0u8);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleSpriteInvisibility() {
    unsafe {
        if (IsBattlerSpritePresent(((&raw mut gActiveBattler).cast::<u8>()).read())) != 0 {
            crate::c::bf_write(
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
                (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as u16) as i32,
            );
            CopyBattleSpriteInvisibility(((&raw mut gActiveBattler).cast::<u8>()).read());
        }
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleBattleAnimation() {
    unsafe {
        if !((IsBattleSEPlaying(((&raw mut gActiveBattler).cast::<u8>()).read())) != 0) {
            let mut animationId: u8 = (((((&raw mut gBattleBufferA).cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
                ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read();
            let mut argument: u16 = (((((((((&raw mut gBattleBufferA).cast::<u8>())
                .wrapping_offset(
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
                PlayerBufferExecCompleted();
            } else {
                ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(Some(CompleteOnFinishedBattleAnimation));
            }
            BattleTv_SetDataBasedOnAnimation(animationId);
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleLinkStandbyMsg() {
    unsafe {
        RecordedBattle_RecordAllBattlerData(
            ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2),
        );
        'l1: {
            let __sw1 = (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                PrintLinkStandbyMsg();
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                EndBounceEffect(((&raw mut gActiveBattler).cast::<u8>()).read(), 1u8);
                EndBounceEffect(((&raw mut gActiveBattler).cast::<u8>()).read(), 0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                PrintLinkStandbyMsg();
                break 'l1;
            }
        }
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleResetActionMoveSelection() {
    unsafe {
        'l1: {
            let __sw1 = (((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32);
            if __sw1 == 0i32 {
                (((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(0u8);
                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                (((&raw mut gActionSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(0u8);
                break 'l1;
            }
        }
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleEndLinkBattle() {
    unsafe {
        RecordedBattle_RecordAllBattlerData(
            ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(4),
        );
        ((&raw mut gBattleOutcome).cast::<u8>()).write(
            (((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read(),
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            3,
            1,
            ((((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32,
        );
        FadeOutMapMusic(5u8);
        BeginFastPaletteFade(3u8);
        PlayerBufferExecCompleted();
        ((((&raw mut gBattlerControllerFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .write(Some(SetBattleEndCallbacks));
    }
}
pub(crate) unsafe extern "C" fn PlayerCmdEnd() {
    unsafe {}
}
