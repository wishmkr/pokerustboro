//! Translated from `src/battle_controller_player.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
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
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sPlayerBufferCommands sTargetIdentities sUnused

static sPlayerBufferCommands: Table<CArray<Option<unsafe extern "C" fn()>, 57>> =
    Table((&raw const crate::data::battle_controller_player::sPlayerBufferCommands).cast());
static sTargetIdentities: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_controller_player::sTargetIdentities).cast());

unsafe extern "C" {
    static mut gAbsentBattlerFlags: u8;
    static mut gActionSelectionCursor: CArray<u8, 4>;
    static mut gActiveBattler: u8;
    static mut gAnimDisableStructPtr: *mut DisableStruct;
    static mut gAnimFriendship: u8;
    static mut gAnimMoveDmg: i32;
    static mut gAnimMovePower: u16;
    static mut gAnimMoveTurn: u8;
    static mut gAnimScriptActive: u8;
    static mut gAnimScriptCallback: Option<unsafe extern "C" fn()>;
    static mut gBattleBufferA: CArray<CArray<u8, 512>, 4>;
    static mut gBattleControllerData: CArray<u8, 4>;
    static mut gBattleControllerExecFlags: u32;
    static mut gBattleMonForms: CArray<u8, 4>;
    static mut gBattleMons: CArray<BattlePokemon, 4>;
    static gBattleMoves: CArray<BattleMove, 0>;
    static mut gBattleOutcome: u8;
    static mut gBattlePalaceMoveSelectionRngValue: u32;
    static mut gBattlePartyCurrentOrder: CArray<u8, 3>;
    static mut gBattleSpritesDataPtr: *mut BattleSpriteData;
    static mut gBattleStruct: *mut BattleStruct;
    static mut gBattleTypeFlags: u32;
    static mut gBattle_BG0_X: u16;
    static mut gBattle_BG0_Y: u16;
    static mut gBattlerControllerFuncs: CArray<Option<unsafe extern "C" fn()>, 4>;
    static mut gBattlerInMenuId: u8;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gBattlerSpriteIds: CArray<u8, 4>;
    static mut gBattlerStatusSummaryTaskId: CArray<u8, 4>;
    static mut gBattlersCount: u8;
    static gBitTable: CArray<u32, 0>;
    static mut gDisableStructs: CArray<DisableStruct, 4>;
    static mut gDisplayedStringBattle: CArray<u8, 300>;
    static mut gDoingBattleAnim: u8;
    static gExperienceTables: CArray<CArray<u32, 101>, 0>;
    static mut gHealthboxSpriteIds: CArray<u8, 4>;
    static mut gIntroSlideFlags: u16;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gMPlayInfo_BGM: MusicPlayerInfo;
    static mut gMain: Main;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static mut gMoveSelectionCursor: CArray<u8, 4>;
    static mut gMultiUsePlayerCursor: u8;
    static mut gMultiuseSpriteTemplate: SpriteTemplate;
    static mut gNumberOfMovesToChoose: u8;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPartnerTrainerId: u16;
    static mut gPartyMenuUseExitCallback: u8;
    static mut gPlayerDpadHoldFrames: u8;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlayerPartyLostHP: u32;
    static mut gPreBattleCallback1: Option<unsafe extern "C" fn()>;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRngValue: u32;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSelectedMonPartyId: u8;
    static mut gSpecialVar_ItemId: u16;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gText_BattleMenu: CArray<u8, 0>;
    static gText_BattleSwitchWhich: CArray<u8, 0>;
    static gText_BattleYesNoChoice: CArray<u8, 0>;
    static gText_LinkStandby: CArray<u8, 0>;
    static gText_MoveInterfacePP: CArray<u8, 0>;
    static gText_MoveInterfaceType: CArray<u8, 0>;
    static gText_WhatWillPkmnDo: CArray<u8, 0>;
    static gTrainerBackPicCoords: CArray<MonCoords, 0>;
    static gTrainerBackPicPaletteTable: CArray<CompressedSpritePalette, 0>;
    static gTrainerFrontPicCoords: CArray<MonCoords, 0>;
    static gTrainerFrontPicPaletteTable: CArray<CompressedSpritePalette, 0>;
    static mut gTransformedPersonalities: CArray<u32, 4>;
    static gTypeNames: CArray<CArray<u8, 7>, 18>;
    static mut gUnusedControllerStruct: UnusedControllerStruct;
    static mut gWeatherMoveAnim: u16;
    static mut gWirelessCommType: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn BattleArena_DeductSkillPoints(a0: u8, a1: u16);
    fn BattleCreateYesNoCursorAt(a0: u8);
    fn BattleDestroyYesNoCursorAt(a0: u8);
    fn BattleGfxSfxDummy2(a0: u16);
    fn BattleGfxSfxDummy3(a0: u8);
    fn BattleLoadPlayerMonSpriteGfx(a0: *mut Pokemon, a1: u8);
    fn BattleMainCB2();
    fn BattlePutTextOnWindow(a0: *mut u8, a1: u8);
    fn BattleStopLowHpSound();
    fn BattleStringExpandPlaceholdersToDisplayedString(a0: *mut u8) -> u32;
    fn BattleTv_ClearExplosionFaintCause();
    fn BattleTv_SetDataBasedOnAnimation(a0: u8);
    fn BattleTv_SetDataBasedOnMove(a0: u16, a1: u16, a2: *mut DisableStruct);
    fn BattleTv_SetDataBasedOnString(a0: u16);
    fn BeginFastPaletteFade(a0: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BtlController_EmitChosenMonReturnValue(a0: u8, a1: u8, a2: *mut u8);
    fn BtlController_EmitDataTransfer(a0: u8, a1: u16, a2: *mut c_void);
    fn BtlController_EmitOneReturnValue(a0: u8, a1: u16);
    fn BtlController_EmitOneReturnValue_Duplicate(a0: u8, a1: u16);
    fn BtlController_EmitTwoReturnValues(a0: u8, a1: u8, a2: u16);
    fn BufferStringBattle(a0: u16);
    fn CB2_BagMenuFromBattle();
    fn CB2_InitEndLinkBattle();
    fn CalculateMonStats(a0: *mut Pokemon);
    fn ChooseMoveAndTargetInBattlePalace() -> u16;
    fn ClearTemporarySpeciesSpriteData(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyAllBattleSpritesInvisibilities();
    fn CopyBattleSpriteInvisibility(a0: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CountAliveMonsInBattle(a0: u8) -> u8;
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut Sprite)>) -> u8;
    fn CreatePartyStatusSummarySprites(a0: u8, a1: *mut HpAndStatus, a2: u8, a3: u8) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressTrainerBackPic(a0: u16, a1: u8);
    fn DecompressTrainerFrontPic(a0: u16, a1: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DoBounceEffect(a0: u8, a1: u8, a2: i8, a3: i8);
    fn DoHitAnimHealthboxEffect(a0: u8);
    fn DoMoveAnim(a0: u16);
    fn DoPokeballSendOutAnimation(a0: i16, a1: u8) -> u8;
    fn EndBounceEffect(a0: u8, a1: u8);
    fn FadeOutMapMusic(a0: u8);
    fn FreeAllWindowBuffers();
    fn FreeOamMatrix(a0: u8);
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteDefault_Y(a0: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetDefaultMoveTarget(a0: u8) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn GetSpritePaletteTagByPaletteNum(a0: u8) -> u16;
    fn HandleBattleWindow(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn HandleIntroSlide(a0: u8);
    fn HandleLowHpMusicChange(a0: *mut Pokemon, a1: u8);
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
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetMultiuseSpriteTemplateToTrainerBack(a0: u16, a1: u8);
    fn SetMultiuseSpriteTemplateToTrainerFront(a0: u16, a1: u8);
    fn SetPPNumbersPaletteInMoveSelection();
    fn SetSpritePrimaryCoordsFromSecondaryCoords(a0: *mut Sprite);
    fn SpriteCB_FaintSlideAnim(a0: *mut Sprite);
    fn SpriteCB_HideAsMoveTarget(a0: *mut Sprite);
    fn SpriteCB_ShowAsMoveTarget(a0: *mut Sprite);
    fn SpriteCB_TrainerSlideIn(a0: *mut Sprite);
    fn SpriteCB_WaitForBattlerBallReleaseAnim(a0: *mut Sprite);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartAnimLinearTranslation(a0: *mut Sprite);
    fn StartHealthboxSlideIn(a0: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn SwapHpBarsWithHpText();
    fn TaskDummy(a0: u8);
    fn Task_HidePartyStatusSummary(a0: u8);
    fn TryHandleLaunchBattleTableAnimation(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn TryPutLinkBattleTvShowOnAir();
    fn TrySetBehindSubstituteSpriteBit(a0: u8, a1: u16);
    fn TryShinyAnimation(a0: u8, a1: *mut Pokemon);
    fn UpdateHealthboxAttribute(a0: u8, a1: *mut Pokemon, a2: u8);
    fn UpdateHpTextInHealthbox(a0: u8, a1: i16, a2: u8);
    fn m4aMPlayContinue(a0: *mut MusicPlayerInfo);
    fn m4aMPlayVolumeControl(a0: *mut MusicPlayerInfo, a1: u16, a2: u16);
    fn m4aSongNumStop(a0: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleControllerDummy() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetControllerToPlayer() {
    gBattlerControllerFuncs[gActiveBattler] = Some(PlayerBufferRunCommand);
    gDoingBattleAnim = FALSE;
    gPlayerDpadHoldFrames = 0;
}
pub(crate) unsafe extern "C" fn PlayerBufferExecCompleted() {
    gBattlerControllerFuncs[gActiveBattler] = Some(PlayerBufferRunCommand);
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        let mut playerId: u8 = GetMultiplayerId();
        PrepareBufferDataTransferLink(B_COMM_CONTROLLER_IS_DONE, 4, &raw mut playerId);
        gBattleBufferA[gActiveBattler][0] = CONTROLLER_TERMINATOR_NOP;
    } else {
        gBattleControllerExecFlags &= !gBitTable[gActiveBattler];
    }
}
pub(crate) unsafe extern "C" fn PlayerBufferRunCommand() {
    if gBattleControllerExecFlags & gBitTable[gActiveBattler] != 0 {
        if gBattleBufferA[gActiveBattler][0] < 57 {
            sPlayerBufferCommands[gBattleBufferA[gActiveBattler][0]].unwrap_unchecked()();
        } else {
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnBankSpritePosX_0() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].x2 == 0 {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn HandleInputChooseAction() {
    let mut itemId: u16 =
        gBattleBufferA[gActiveBattler][2] as u16 | (gBattleBufferA[gActiveBattler][3] as u16) << 8;
    DoBounceEffect(gActiveBattler, 0x1, 7, 1);
    DoBounceEffect(gActiveBattler, BOUNCE_MON, 7, 1);
    if gMain.newAndRepeatedKeys as i32 & DPAD_ANY != 0
        && (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_L_EQUALS_A
    {
        gPlayerDpadHoldFrames += 1;
    } else {
        gPlayerDpadHoldFrames = 0;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        match gActionSelectionCursor[gActiveBattler] {
            0 => {
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, 0, 0);
            }
            1 => {
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_USE_ITEM, 0);
            }
            2 => {
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SWITCH, 0);
            }
            3 => {
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_RUN, 0);
            }
            _ => {}
        }
        PlayerBufferExecCompleted();
    } else if gMain.newKeys as i32 & DPAD_LEFT != 0 {
        if gActionSelectionCursor[gActiveBattler] as i32 & 1 != 0 {
            PlaySE(SE_SELECT);
            ActionSelectionDestroyCursorAt(gActionSelectionCursor[gActiveBattler]);
            gActionSelectionCursor[gActiveBattler] ^= 1;
            ActionSelectionCreateCursorAt(gActionSelectionCursor[gActiveBattler], 0);
        }
    } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
        if gActionSelectionCursor[gActiveBattler] as i32 & 1 == 0 {
            PlaySE(SE_SELECT);
            ActionSelectionDestroyCursorAt(gActionSelectionCursor[gActiveBattler]);
            gActionSelectionCursor[gActiveBattler] ^= 1;
            ActionSelectionCreateCursorAt(gActionSelectionCursor[gActiveBattler], 0);
        }
    } else if gMain.newKeys as i32 & DPAD_UP != 0 {
        if gActionSelectionCursor[gActiveBattler] as i32 & 2 != 0 {
            PlaySE(SE_SELECT);
            ActionSelectionDestroyCursorAt(gActionSelectionCursor[gActiveBattler]);
            gActionSelectionCursor[gActiveBattler] ^= 2;
            ActionSelectionCreateCursorAt(gActionSelectionCursor[gActiveBattler], 0);
        }
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
        if gActionSelectionCursor[gActiveBattler] as i32 & 2 == 0 {
            PlaySE(SE_SELECT);
            ActionSelectionDestroyCursorAt(gActionSelectionCursor[gActiveBattler]);
            gActionSelectionCursor[gActiveBattler] ^= 2;
            ActionSelectionCreateCursorAt(gActionSelectionCursor[gActiveBattler], 0);
        }
    } else if gMain.newKeys as i32 & B_BUTTON != 0 || gPlayerDpadHoldFrames > 59 {
        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
            && GetBattlerPosition(gActiveBattler) == B_POSITION_PLAYER_RIGHT
            && gAbsentBattlerFlags as u32 & gBitTable[GetBattlerAtPosition(B_POSITION_PLAYER_LEFT)]
                == 0
            && gBattleTypeFlags & BATTLE_TYPE_MULTI == 0
        {
            if gBattleBufferA[gActiveBattler][1] == 1 {
                if itemId <= ITEM_PREMIER_BALL {
                    AddBagItem(itemId, 1);
                } else {
                    return;
                }
            }
            PlaySE(SE_SELECT);
            BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_CANCEL_PARTNER, 0);
            PlayerBufferExecCompleted();
        }
    } else if gMain.newKeys as i32 & START_BUTTON != 0 {
        SwapHpBarsWithHpText();
    }
}
pub(crate) unsafe extern "C" fn UnusedEndBounceEffect() {
    EndBounceEffect(gActiveBattler, BOUNCE_HEALTHBOX);
    EndBounceEffect(gActiveBattler, BOUNCE_MON);
    gBattlerControllerFuncs[gActiveBattler] = Some(HandleInputChooseTarget);
}
pub(crate) unsafe extern "C" fn HandleInputChooseTarget() {
    let mut i: i32 = 0;
    let mut identities: CArray<u8, 4> = zeroed();
    memcpy(
        identities.as_mut_ptr(),
        sTargetIdentities.as_ptr().cast_mut(),
        4,
    );
    DoBounceEffect(gMultiUsePlayerCursor, 0x1, 15, 1);
    i = 0;
    if gBattlersCount != 0 {
        loop {
            if i != gMultiUsePlayerCursor as i32 {
                EndBounceEffect(i as u8, BOUNCE_HEALTHBOX);
            }
            i += 1;
            if i >= gBattlersCount as i32 {
                break;
            }
        }
    }
    if gMain.heldKeys as i32 & DPAD_ANY != 0
        && (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_L_EQUALS_A
    {
        gPlayerDpadHoldFrames += 1;
    } else {
        gPlayerDpadHoldFrames = 0;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        gSprites[gBattlerSpriteIds[gMultiUsePlayerCursor]].callback =
            Some(SpriteCB_HideAsMoveTarget);
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            B_ACTION_EXEC_SCRIPT,
            gMoveSelectionCursor[gActiveBattler] as u16 | (gMultiUsePlayerCursor as u16) << 8,
        );
        EndBounceEffect(gMultiUsePlayerCursor, BOUNCE_HEALTHBOX);
        PlayerBufferExecCompleted();
    } else if gMain.newKeys as i32 & B_BUTTON != 0 || gPlayerDpadHoldFrames > 59 {
        PlaySE(SE_SELECT);
        gSprites[gBattlerSpriteIds[gMultiUsePlayerCursor]].callback =
            Some(SpriteCB_HideAsMoveTarget);
        gBattlerControllerFuncs[gActiveBattler] = Some(HandleInputChooseMove);
        DoBounceEffect(gActiveBattler, 0x1, 7, 1);
        DoBounceEffect(gActiveBattler, BOUNCE_MON, 7, 1);
        EndBounceEffect(gMultiUsePlayerCursor, BOUNCE_HEALTHBOX);
    } else if gMain.newKeys as i32 & 96 != 0 {
        PlaySE(SE_SELECT);
        gSprites[gBattlerSpriteIds[gMultiUsePlayerCursor]].callback =
            Some(SpriteCB_HideAsMoveTarget);
        loop {
            let mut currSelIdentity: u8 = GetBattlerPosition(gMultiUsePlayerCursor);
            i = 0;
            while i < MAX_BATTLERS_COUNT as i32 {
                if currSelIdentity == identities[i] {
                    break;
                }
                i += 1;
            }
            loop {
                if ({
                    i -= 1;
                    i
                }) < 0
                {
                    i = 3;
                }
                gMultiUsePlayerCursor = GetBattlerAtPosition(identities[i]);
                if gMultiUsePlayerCursor != gBattlersCount {
                    break;
                }
            }
            i = 0;
            match GetBattlerPosition(gMultiUsePlayerCursor) {
                B_POSITION_PLAYER_LEFT | B_POSITION_PLAYER_RIGHT => {
                    if gActiveBattler != gMultiUsePlayerCursor {
                        i += 1;
                    } else if gBattleMoves[GetMonData2(
                        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                        MON_DATA_MOVE1 + gMoveSelectionCursor[gActiveBattler] as i32,
                    )]
                    .target as i32
                        & MOVE_TARGET_USER_OR_SELECTED as i32
                        != 0
                    {
                        i += 1;
                    }
                }
                B_POSITION_OPPONENT_LEFT | B_POSITION_OPPONENT_RIGHT => {
                    i += 1;
                }
                _ => {}
            }
            if gAbsentBattlerFlags as u32 & gBitTable[gMultiUsePlayerCursor] != 0 {
                i = 0;
            }
            if i != 0 {
                break;
            }
        }
        gSprites[gBattlerSpriteIds[gMultiUsePlayerCursor]].callback =
            Some(SpriteCB_ShowAsMoveTarget);
    } else if gMain.newKeys as i32 & 144 != 0 {
        PlaySE(SE_SELECT);
        gSprites[gBattlerSpriteIds[gMultiUsePlayerCursor]].callback =
            Some(SpriteCB_HideAsMoveTarget);
        loop {
            let mut currSelIdentity: u8 = GetBattlerPosition(gMultiUsePlayerCursor);
            i = 0;
            while i < MAX_BATTLERS_COUNT as i32 {
                if currSelIdentity == identities[i] {
                    break;
                }
                i += 1;
            }
            loop {
                if ({
                    i += 1;
                    i
                }) > 3
                {
                    i = 0;
                }
                gMultiUsePlayerCursor = GetBattlerAtPosition(identities[i]);
                if gMultiUsePlayerCursor != gBattlersCount {
                    break;
                }
            }
            i = 0;
            match GetBattlerPosition(gMultiUsePlayerCursor) {
                B_POSITION_PLAYER_LEFT | B_POSITION_PLAYER_RIGHT => {
                    if gActiveBattler != gMultiUsePlayerCursor {
                        i += 1;
                    } else if gBattleMoves[GetMonData2(
                        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                        MON_DATA_MOVE1 + gMoveSelectionCursor[gActiveBattler] as i32,
                    )]
                    .target as i32
                        & MOVE_TARGET_USER_OR_SELECTED as i32
                        != 0
                    {
                        i += 1;
                    }
                }
                B_POSITION_OPPONENT_LEFT | B_POSITION_OPPONENT_RIGHT => {
                    i += 1;
                }
                _ => {}
            }
            if gAbsentBattlerFlags as u32 & gBitTable[gMultiUsePlayerCursor] != 0 {
                i = 0;
            }
            if i != 0 {
                break;
            }
        }
        gSprites[gBattlerSpriteIds[gMultiUsePlayerCursor]].callback =
            Some(SpriteCB_ShowAsMoveTarget);
    }
}
pub(crate) unsafe extern "C" fn HandleInputChooseMove() {
    let mut canSelectTarget: u32 = FALSE as u32;
    let mut moveInfo: *mut ChooseMoveStruct =
        &raw mut gBattleBufferA[gActiveBattler][4] as *mut ChooseMoveStruct;
    if gMain.heldKeys as i32 & DPAD_ANY != 0
        && (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_L_EQUALS_A
    {
        gPlayerDpadHoldFrames += 1;
    } else {
        gPlayerDpadHoldFrames = 0;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        let mut moveTarget: u8 = 0;
        PlaySE(SE_SELECT);
        if (*moveInfo).moves[gMoveSelectionCursor[gActiveBattler]] == MOVE_CURSE {
            if (*moveInfo).monTypes[0] != TYPE_GHOST && (*moveInfo).monTypes[1] != TYPE_GHOST {
                moveTarget = MOVE_TARGET_USER;
            } else {
                moveTarget = MOVE_TARGET_SELECTED;
            }
        } else {
            moveTarget =
                gBattleMoves[(*moveInfo).moves[gMoveSelectionCursor[gActiveBattler]]].target;
        }
        if moveTarget as i32 & MOVE_TARGET_USER as i32 != 0 {
            gMultiUsePlayerCursor = gActiveBattler;
        } else {
            gMultiUsePlayerCursor =
                GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) & 1 ^ 1);
        }
        if gBattleBufferA[gActiveBattler][1] == 0 {
            if moveTarget as i32 & 2 != 0 && gBattleBufferA[gActiveBattler][2] == 0 {
                canSelectTarget += 1;
            }
        } else {
            if moveTarget as i32 & 125 == 0 {
                canSelectTarget += 1;
            }
            if (*moveInfo).currentPP[gMoveSelectionCursor[gActiveBattler]] == 0 {
                canSelectTarget = FALSE as u32;
            } else if moveTarget as i32 & 18 == 0
                && CountAliveMonsInBattle(BATTLE_ALIVE_EXCEPT_ACTIVE) <= 1
            {
                gMultiUsePlayerCursor = GetDefaultMoveTarget(gActiveBattler);
                canSelectTarget = FALSE as u32;
            }
        }
        if canSelectTarget == 0 {
            BtlController_EmitTwoReturnValues(
                B_COMM_TO_ENGINE,
                B_ACTION_EXEC_SCRIPT,
                gMoveSelectionCursor[gActiveBattler] as u16 | (gMultiUsePlayerCursor as u16) << 8,
            );
            PlayerBufferExecCompleted();
        } else {
            gBattlerControllerFuncs[gActiveBattler] = Some(HandleInputChooseTarget);
            if moveTarget as i32 & 18 != 0 {
                gMultiUsePlayerCursor = gActiveBattler;
            } else if gAbsentBattlerFlags as u32
                & gBitTable[GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT)]
                != 0
            {
                gMultiUsePlayerCursor = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
            } else {
                gMultiUsePlayerCursor = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
            }
            gSprites[gBattlerSpriteIds[gMultiUsePlayerCursor]].callback =
                Some(SpriteCB_ShowAsMoveTarget);
        }
    } else if gMain.newKeys as i32 & B_BUTTON != 0 || gPlayerDpadHoldFrames > 59 {
        PlaySE(SE_SELECT);
        BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_EXEC_SCRIPT, 0xFFFF);
        PlayerBufferExecCompleted();
    } else if gMain.newKeys as i32 & DPAD_LEFT != 0 {
        if gMoveSelectionCursor[gActiveBattler] as i32 & 1 != 0 {
            MoveSelectionDestroyCursorAt(gMoveSelectionCursor[gActiveBattler]);
            gMoveSelectionCursor[gActiveBattler] ^= 1;
            PlaySE(SE_SELECT);
            MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 0);
            MoveSelectionDisplayPPNumber();
            MoveSelectionDisplayMoveType();
        }
    } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
        if gMoveSelectionCursor[gActiveBattler] as i32 & 1 == 0
            && gMoveSelectionCursor[gActiveBattler] as i32 ^ 1 < gNumberOfMovesToChoose as i32
        {
            MoveSelectionDestroyCursorAt(gMoveSelectionCursor[gActiveBattler]);
            gMoveSelectionCursor[gActiveBattler] ^= 1;
            PlaySE(SE_SELECT);
            MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 0);
            MoveSelectionDisplayPPNumber();
            MoveSelectionDisplayMoveType();
        }
    } else if gMain.newKeys as i32 & DPAD_UP != 0 {
        if gMoveSelectionCursor[gActiveBattler] as i32 & 2 != 0 {
            MoveSelectionDestroyCursorAt(gMoveSelectionCursor[gActiveBattler]);
            gMoveSelectionCursor[gActiveBattler] ^= 2;
            PlaySE(SE_SELECT);
            MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 0);
            MoveSelectionDisplayPPNumber();
            MoveSelectionDisplayMoveType();
        }
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
        if gMoveSelectionCursor[gActiveBattler] as i32 & 2 == 0
            && gMoveSelectionCursor[gActiveBattler] as i32 ^ 2 < gNumberOfMovesToChoose as i32
        {
            MoveSelectionDestroyCursorAt(gMoveSelectionCursor[gActiveBattler]);
            gMoveSelectionCursor[gActiveBattler] ^= 2;
            PlaySE(SE_SELECT);
            MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 0);
            MoveSelectionDisplayPPNumber();
            MoveSelectionDisplayMoveType();
        }
    } else if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
        if gNumberOfMovesToChoose > 1 && gBattleTypeFlags & BATTLE_TYPE_LINK == 0 {
            MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 29);
            if gMoveSelectionCursor[gActiveBattler] != 0 {
                gMultiUsePlayerCursor = 0;
            } else {
                gMultiUsePlayerCursor = gMoveSelectionCursor[gActiveBattler] + 1;
            }
            MoveSelectionCreateCursorAt(gMultiUsePlayerCursor, 27);
            BattlePutTextOnWindow(
                gText_BattleSwitchWhich.as_ptr().cast_mut(),
                B_WIN_SWITCH_PROMPT,
            );
            gBattlerControllerFuncs[gActiveBattler] = Some(HandleMoveSwitching);
        }
    }
}
pub(crate) unsafe extern "C" fn HandleMoveInputUnused() -> u32 {
    let mut var: u32 = 0;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        var = 1;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        gBattle_BG0_X = 0;
        gBattle_BG0_Y = 320;
        var = 0xFF;
    }
    if gMain.newKeys as i32 & DPAD_LEFT != 0 && gMoveSelectionCursor[gActiveBattler] as i32 & 1 != 0
    {
        MoveSelectionDestroyCursorAt(gMoveSelectionCursor[gActiveBattler]);
        gMoveSelectionCursor[gActiveBattler] ^= 1;
        PlaySE(SE_SELECT);
        MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 0);
    }
    if gMain.newKeys as i32 & DPAD_RIGHT != 0
        && gMoveSelectionCursor[gActiveBattler] as i32 & 1 == 0
        && gMoveSelectionCursor[gActiveBattler] as i32 ^ 1 < gNumberOfMovesToChoose as i32
    {
        MoveSelectionDestroyCursorAt(gMoveSelectionCursor[gActiveBattler]);
        gMoveSelectionCursor[gActiveBattler] ^= 1;
        PlaySE(SE_SELECT);
        MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 0);
    }
    if gMain.newKeys as i32 & DPAD_UP != 0 && gMoveSelectionCursor[gActiveBattler] as i32 & 2 != 0 {
        MoveSelectionDestroyCursorAt(gMoveSelectionCursor[gActiveBattler]);
        gMoveSelectionCursor[gActiveBattler] ^= 2;
        PlaySE(SE_SELECT);
        MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 0);
    }
    if gMain.newKeys as i32 & DPAD_DOWN != 0
        && gMoveSelectionCursor[gActiveBattler] as i32 & 2 == 0
        && gMoveSelectionCursor[gActiveBattler] as i32 ^ 2 < gNumberOfMovesToChoose as i32
    {
        MoveSelectionDestroyCursorAt(gMoveSelectionCursor[gActiveBattler]);
        gMoveSelectionCursor[gActiveBattler] ^= 2;
        PlaySE(SE_SELECT);
        MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 0);
    }
    return var;
}
pub(crate) unsafe extern "C" fn HandleMoveSwitching() {
    let mut perMovePPBonuses: CArray<u8, 4> = zeroed();
    let mut moveStruct: ChooseMoveStruct = zeroed();
    let mut totalPPBonuses: u8 = 0;
    if gMain.newKeys as i32 & 5 != 0 {
        PlaySE(SE_SELECT);
        if gMoveSelectionCursor[gActiveBattler] != gMultiUsePlayerCursor {
            let mut moveInfo: *mut ChooseMoveStruct =
                &raw mut gBattleBufferA[gActiveBattler][4] as *mut ChooseMoveStruct;
            let mut i: i32 = 0;
            i = (*moveInfo).moves[gMoveSelectionCursor[gActiveBattler]] as i32;
            (*moveInfo).moves[gMoveSelectionCursor[gActiveBattler]] =
                (*moveInfo).moves[gMultiUsePlayerCursor];
            (*moveInfo).moves[gMultiUsePlayerCursor] = i as u16;
            i = (*moveInfo).currentPP[gMoveSelectionCursor[gActiveBattler]] as i32;
            (*moveInfo).currentPP[gMoveSelectionCursor[gActiveBattler]] =
                (*moveInfo).currentPP[gMultiUsePlayerCursor];
            (*moveInfo).currentPP[gMultiUsePlayerCursor] = i as u8;
            i = (*moveInfo).maxPP[gMoveSelectionCursor[gActiveBattler]] as i32;
            (*moveInfo).maxPP[gMoveSelectionCursor[gActiveBattler]] =
                (*moveInfo).maxPP[gMultiUsePlayerCursor];
            (*moveInfo).maxPP[gMultiUsePlayerCursor] = i as u8;
            if gDisableStructs[gActiveBattler].mimickedMoves() as u32
                & gBitTable[gMoveSelectionCursor[gActiveBattler]]
                != 0
            {
                gDisableStructs[gActiveBattler].set_mimickedMoves(
                    gDisableStructs[gActiveBattler].mimickedMoves()
                        & !(gBitTable[gMoveSelectionCursor[gActiveBattler]] as u8),
                );
                gDisableStructs[gActiveBattler].set_mimickedMoves(
                    gDisableStructs[gActiveBattler].mimickedMoves()
                        | gBitTable[gMultiUsePlayerCursor] as u8,
                );
            }
            MoveSelectionDisplayMoveNames();
            i = 0;
            while i < MAX_MON_MOVES {
                perMovePPBonuses[i] = shr_i32(
                    gBattleMons[gActiveBattler].ppBonuses as i32 & shl_i32(3, i as u32 * 2),
                    i as u32 * 2,
                ) as u8;
                i += 1;
            }
            totalPPBonuses = perMovePPBonuses[gMoveSelectionCursor[gActiveBattler]];
            perMovePPBonuses[gMoveSelectionCursor[gActiveBattler]] =
                perMovePPBonuses[gMultiUsePlayerCursor];
            perMovePPBonuses[gMultiUsePlayerCursor] = totalPPBonuses;
            totalPPBonuses = 0;
            i = 0;
            while i < MAX_MON_MOVES {
                totalPPBonuses |= shl_i32(perMovePPBonuses[i] as i32, i as u32 * 2) as u8;
                i += 1;
            }
            gBattleMons[gActiveBattler].ppBonuses = totalPPBonuses;
            i = 0;
            while i < MAX_MON_MOVES {
                gBattleMons[gActiveBattler].moves[i] = (*moveInfo).moves[i];
                gBattleMons[gActiveBattler].pp[i] = (*moveInfo).currentPP[i];
                i += 1;
            }
            if gBattleMons[gActiveBattler].status2 & STATUS2_TRANSFORMED == 0 {
                i = 0;
                while i < MAX_MON_MOVES {
                    moveStruct.moves[i] = GetMonData2(
                        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                        MON_DATA_MOVE1 + i,
                    ) as u16;
                    moveStruct.currentPP[i] = GetMonData2(
                        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                        MON_DATA_PP1 + i,
                    ) as u8;
                    i += 1;
                }
                totalPPBonuses = GetMonData2(
                    &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                    MON_DATA_PP_BONUSES,
                ) as u8;
                i = 0;
                while i < MAX_MON_MOVES {
                    perMovePPBonuses[i] = shr_i32(
                        totalPPBonuses as i32 & shl_i32(3, i as u32 * 2),
                        i as u32 * 2,
                    ) as u8;
                    i += 1;
                }
                i = moveStruct.moves[gMoveSelectionCursor[gActiveBattler]] as i32;
                moveStruct.moves[gMoveSelectionCursor[gActiveBattler]] =
                    moveStruct.moves[gMultiUsePlayerCursor];
                moveStruct.moves[gMultiUsePlayerCursor] = i as u16;
                i = moveStruct.currentPP[gMoveSelectionCursor[gActiveBattler]] as i32;
                moveStruct.currentPP[gMoveSelectionCursor[gActiveBattler]] =
                    moveStruct.currentPP[gMultiUsePlayerCursor];
                moveStruct.currentPP[gMultiUsePlayerCursor] = i as u8;
                totalPPBonuses = perMovePPBonuses[gMoveSelectionCursor[gActiveBattler]];
                perMovePPBonuses[gMoveSelectionCursor[gActiveBattler]] =
                    perMovePPBonuses[gMultiUsePlayerCursor];
                perMovePPBonuses[gMultiUsePlayerCursor] = totalPPBonuses;
                totalPPBonuses = 0;
                i = 0;
                while i < MAX_MON_MOVES {
                    totalPPBonuses |= shl_i32(perMovePPBonuses[i] as i32, i as u32 * 2) as u8;
                    i += 1;
                }
                i = 0;
                while i < MAX_MON_MOVES {
                    SetMonData(
                        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                        MON_DATA_MOVE1 + i,
                        &raw mut moveStruct.moves[i] as *mut c_void,
                    );
                    SetMonData(
                        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                        MON_DATA_PP1 + i,
                        &raw mut moveStruct.currentPP[i] as *mut c_void,
                    );
                    i += 1;
                }
                SetMonData(
                    &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                    MON_DATA_PP_BONUSES,
                    &raw mut totalPPBonuses as *mut c_void,
                );
            }
        }
        gBattlerControllerFuncs[gActiveBattler] = Some(HandleInputChooseMove);
        gMoveSelectionCursor[gActiveBattler] = gMultiUsePlayerCursor;
        MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 0);
        MoveSelectionDisplayPPString();
        MoveSelectionDisplayPPNumber();
        MoveSelectionDisplayMoveType();
    } else if gMain.newKeys as i32 & 6 != 0 {
        PlaySE(SE_SELECT);
        MoveSelectionDestroyCursorAt(gMultiUsePlayerCursor);
        MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 0);
        gBattlerControllerFuncs[gActiveBattler] = Some(HandleInputChooseMove);
        MoveSelectionDisplayPPString();
        MoveSelectionDisplayPPNumber();
        MoveSelectionDisplayMoveType();
    } else if gMain.newKeys as i32 & DPAD_LEFT != 0 {
        if gMultiUsePlayerCursor as i32 & 1 != 0 {
            if gMultiUsePlayerCursor == gMoveSelectionCursor[gActiveBattler] {
                MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 29);
            } else {
                MoveSelectionDestroyCursorAt(gMultiUsePlayerCursor);
            }
            gMultiUsePlayerCursor ^= 1;
            PlaySE(SE_SELECT);
            if gMultiUsePlayerCursor == gMoveSelectionCursor[gActiveBattler] {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor, 0);
            } else {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor, 27);
            }
        }
    } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
        if gMultiUsePlayerCursor as i32 & 1 == 0
            && gMultiUsePlayerCursor as i32 ^ 1 < gNumberOfMovesToChoose as i32
        {
            if gMultiUsePlayerCursor == gMoveSelectionCursor[gActiveBattler] {
                MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 29);
            } else {
                MoveSelectionDestroyCursorAt(gMultiUsePlayerCursor);
            }
            gMultiUsePlayerCursor ^= 1;
            PlaySE(SE_SELECT);
            if gMultiUsePlayerCursor == gMoveSelectionCursor[gActiveBattler] {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor, 0);
            } else {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor, 27);
            }
        }
    } else if gMain.newKeys as i32 & DPAD_UP != 0 {
        if gMultiUsePlayerCursor as i32 & 2 != 0 {
            if gMultiUsePlayerCursor == gMoveSelectionCursor[gActiveBattler] {
                MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 29);
            } else {
                MoveSelectionDestroyCursorAt(gMultiUsePlayerCursor);
            }
            gMultiUsePlayerCursor ^= 2;
            PlaySE(SE_SELECT);
            if gMultiUsePlayerCursor == gMoveSelectionCursor[gActiveBattler] {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor, 0);
            } else {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor, 27);
            }
        }
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
        if gMultiUsePlayerCursor as i32 & 2 == 0
            && gMultiUsePlayerCursor as i32 ^ 2 < gNumberOfMovesToChoose as i32
        {
            if gMultiUsePlayerCursor == gMoveSelectionCursor[gActiveBattler] {
                MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 29);
            } else {
                MoveSelectionDestroyCursorAt(gMultiUsePlayerCursor);
            }
            gMultiUsePlayerCursor ^= 2;
            PlaySE(SE_SELECT);
            if gMultiUsePlayerCursor == gMoveSelectionCursor[gActiveBattler] {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor, 0);
            } else {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor, 27);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetLinkBattleEndCallbacks() {
    if gWirelessCommType == 0 {
        if gReceivedRemoteLinkPlayers == 0 {
            m4aSongNumStop(SE_LOW_HEALTH);
            gMain.set_inBattle(FALSE);
            gMain.callback1 = gPreBattleCallback1;
            SetMainCallback2(Some(CB2_InitEndLinkBattle));
            if gBattleOutcome == B_OUTCOME_WON {
                TryPutLinkBattleTvShowOnAir();
            }
            FreeAllWindowBuffers();
        }
    } else {
        if IsLinkTaskFinished() != 0 {
            m4aSongNumStop(SE_LOW_HEALTH);
            gMain.set_inBattle(FALSE);
            gMain.callback1 = gPreBattleCallback1;
            SetMainCallback2(Some(CB2_InitEndLinkBattle));
            if gBattleOutcome == B_OUTCOME_WON {
                TryPutLinkBattleTvShowOnAir();
            }
            FreeAllWindowBuffers();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattleEndCallbacks() {
    if gPaletteFade.active() == 0 {
        if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
            if IsLinkTaskFinished() != 0 {
                if gWirelessCommType == 0 {
                    SetCloseLinkCallback();
                } else {
                    SetLinkStandbyCallback();
                }
                gBattlerControllerFuncs[gActiveBattler] = Some(SetLinkBattleEndCallbacks);
            }
        } else {
            m4aSongNumStop(SE_LOW_HEALTH);
            gMain.set_inBattle(FALSE);
            gMain.callback1 = gPreBattleCallback1;
            SetMainCallback2(gMain.savedCallback);
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnBattlerSpriteCallbackDummy() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnBankSpriteCallbackDummy2() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn FreeTrainerSpriteAfterSlide() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        BattleGfxSfxDummy3((*gSaveBlock2Ptr).playerGender);
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn Intro_DelayAndEnd() {
    if ({
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay -= 1;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay
    }) == 255
    {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay = 0;
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn Intro_WaitForShinyAnimAndHealthbox() {
    let mut healthboxAnimDone: u8 = FALSE;
    if IsDoubleBattle() == 0 || IsDoubleBattle() != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
        {
            healthboxAnimDone = TRUE;
        }
    } else {
        if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            && gSprites[gHealthboxSpriteIds[gActiveBattler as i32 ^ 2]].callback
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
        {
            healthboxAnimDone = TRUE;
        }
    }
    if healthboxAnimDone != 0
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).finishedShinyMonAnim()
            != 0
        && (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gActiveBattler as i32 ^ 2))
        .finishedShinyMonAnim()
            != 0
    {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_triedShinyMonAnim(FALSE);
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_finishedShinyMonAnim(FALSE);
        (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gActiveBattler as i32 ^ 2))
        .set_triedShinyMonAnim(FALSE);
        (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gActiveBattler as i32 ^ 2))
        .set_finishedShinyMonAnim(FALSE);
        FreeSpriteTilesByTag(ANIM_TAG_GOLD_STARS);
        FreeSpritePaletteByTag(ANIM_TAG_GOLD_STARS);
        HandleLowHpMusicChange(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            gActiveBattler,
        );
        if IsDoubleBattle() != 0 {
            HandleLowHpMusicChange(
                &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler as i32 ^ 2]],
                gActiveBattler ^ 2,
            );
        }
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay = 3;
        gBattlerControllerFuncs[gActiveBattler] = Some(Intro_DelayAndEnd);
    }
}
pub(crate) unsafe extern "C" fn Intro_TryShinyAnimShowHealthbox() {
    let mut bgmRestored: u32 = FALSE as u32;
    let mut battlerAnimsDone: u32 = FALSE as u32;
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).triedShinyMonAnim() == 0
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).ballAnimActive() == 0
    {
        TryShinyAnimation(
            gActiveBattler,
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        );
    }
    if (*(*gBattleSpritesDataPtr)
        .healthBoxesData
        .at(gActiveBattler as i32 ^ 2))
    .triedShinyMonAnim()
        == 0
        && (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gActiveBattler as i32 ^ 2))
        .ballAnimActive()
            == 0
    {
        TryShinyAnimation(
            gActiveBattler ^ 2,
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler as i32 ^ 2]],
        );
    }
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).ballAnimActive() == 0
        && (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gActiveBattler as i32 ^ 2))
        .ballAnimActive()
            == 0
    {
        if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).healthboxSlideInStarted()
            == 0
        {
            if IsDoubleBattle() != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
                UpdateHealthboxAttribute(
                    gHealthboxSpriteIds[gActiveBattler as i32 ^ 2],
                    &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler as i32 ^ 2]],
                    HEALTHBOX_ALL,
                );
                StartHealthboxSlideIn(gActiveBattler ^ 2);
                SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler as i32 ^ 2]);
            }
            UpdateHealthboxAttribute(
                gHealthboxSpriteIds[gActiveBattler],
                &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                HEALTHBOX_ALL,
            );
            StartHealthboxSlideIn(gActiveBattler);
            SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler]);
        }
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_healthboxSlideInStarted(TRUE);
    }
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).waitForCry() == 0
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).healthboxSlideInStarted()
            != 0
        && (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gActiveBattler as i32 ^ 2))
        .waitForCry()
            == 0
        && IsCryPlayingOrClearCrySongs() == 0
    {
        if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).bgmRestored() == 0 {
            if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 && gBattleTypeFlags & BATTLE_TYPE_LINK != 0
            {
                m4aMPlayContinue(&raw mut gMPlayInfo_BGM);
            } else {
                m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
            }
        }
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_bgmRestored(TRUE);
        bgmRestored = TRUE as u32;
    }
    if IsDoubleBattle() == 0 || IsDoubleBattle() != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if gSprites[gBattleControllerData[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            && gSprites[gBattlerSpriteIds[gActiveBattler]].callback
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
        {
            battlerAnimsDone = TRUE as u32;
        }
    } else {
        if gSprites[gBattleControllerData[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            && gSprites[gBattlerSpriteIds[gActiveBattler]].callback
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            && gSprites[gBattleControllerData[gActiveBattler as i32 ^ 2]].callback
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            && gSprites[gBattlerSpriteIds[gActiveBattler as i32 ^ 2]].callback
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
        {
            battlerAnimsDone = TRUE as u32;
        }
    }
    if bgmRestored != 0 && battlerAnimsDone != 0 {
        if IsDoubleBattle() != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
            DestroySprite(&raw mut gSprites[gBattleControllerData[gActiveBattler as i32 ^ 2]]);
        }
        DestroySprite(&raw mut gSprites[gBattleControllerData[gActiveBattler]]);
        (*(*gBattleSpritesDataPtr).animationData).set_introAnimActive(FALSE);
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_bgmRestored(FALSE);
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_healthboxSlideInStarted(FALSE);
        gBattlerControllerFuncs[gActiveBattler] = Some(Intro_WaitForShinyAnimAndHealthbox);
    }
}
pub(crate) unsafe extern "C" fn SwitchIn_CleanShinyAnimShowSubstitute() {
    if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).finishedShinyMonAnim()
            != 0
        && gSprites[gBattlerSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        CopyBattleSpriteInvisibility(gActiveBattler);
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_triedShinyMonAnim(FALSE);
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_finishedShinyMonAnim(FALSE);
        FreeSpriteTilesByTag(ANIM_TAG_GOLD_STARS);
        FreeSpritePaletteByTag(ANIM_TAG_GOLD_STARS);
        if (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).behindSubstitute() != 0 {
            InitAndLaunchSpecialAnimation(
                gActiveBattler,
                gActiveBattler,
                gActiveBattler,
                B_ANIM_MON_TO_SUBSTITUTE,
            );
        }
        gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_HandleSoundAndEnd);
    }
}
pub(crate) unsafe extern "C" fn SwitchIn_HandleSoundAndEnd() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0
        && IsCryPlayingOrClearCrySongs() == 0
    {
        m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
        HandleLowHpMusicChange(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            gActiveBattler,
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SwitchIn_TryShinyAnimShowHealthbox() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).triedShinyMonAnim() == 0
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).ballAnimActive() == 0
    {
        TryShinyAnimation(
            gActiveBattler,
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        );
    }
    if gSprites[gBattleControllerData[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).ballAnimActive() == 0
    {
        DestroySprite(&raw mut gSprites[gBattleControllerData[gActiveBattler]]);
        UpdateHealthboxAttribute(
            gHealthboxSpriteIds[gActiveBattler],
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            HEALTHBOX_ALL,
        );
        StartHealthboxSlideIn(gActiveBattler);
        SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler]);
        gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_CleanShinyAnimShowSubstitute);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_PlayerController_RestoreBgmAfterCry(taskId: u8) {
    if IsCryPlayingOrClearCrySongs() == 0 {
        m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn CompleteOnHealthbarDone() {
    let mut hpValue: i16 = MoveBattleBar(
        gActiveBattler,
        gHealthboxSpriteIds[gActiveBattler],
        HEALTH_BAR,
        0,
    ) as i16;
    SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler]);
    if hpValue != -1 {
        UpdateHpTextInHealthbox(gHealthboxSpriteIds[gActiveBattler], hpValue, HP_CURRENT);
    } else {
        HandleLowHpMusicChange(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            gActiveBattler,
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnInactiveTextPrinter() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn Task_GiveExpToMon(taskId: u8) {
    let mut monId: u32 = gTasks[taskId].data[0] as u8 as u32;
    let mut battler: u8 = gTasks[taskId].data[2] as u8;
    let mut gainedExp: i16 = gTasks[taskId].data[1];
    if IsDoubleBattle() == TRUE || monId != gBattlerPartyIndexes[battler] as u32 {
        let mut mon: *mut Pokemon = &raw mut gPlayerParty[monId];
        let mut species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
        let mut level: u8 = GetMonData2(mon, MON_DATA_LEVEL) as u8;
        let mut currExp: u32 = GetMonData2(mon, MON_DATA_EXP);
        let mut nextLvlExp: u32 =
            gExperienceTables[gSpeciesInfo[species].growthRate][level as i32 + 1];
        if currExp + gainedExp as u32 >= nextLvlExp {
            let mut savedActiveBattler: u8 = 0;
            SetMonData(mon, MON_DATA_EXP, &raw mut nextLvlExp as *mut c_void);
            CalculateMonStats(mon);
            gainedExp -= nextLvlExp as i16 - currExp as i16;
            savedActiveBattler = gActiveBattler;
            gActiveBattler = battler;
            BtlController_EmitTwoReturnValues(
                B_COMM_TO_ENGINE,
                RET_VALUE_LEVELED_UP,
                gainedExp as u16,
            );
            gActiveBattler = savedActiveBattler;
            if IsDoubleBattle() == TRUE
                && (monId as u16 == gBattlerPartyIndexes[battler]
                    || monId as u16 == gBattlerPartyIndexes[battler as i32 ^ 2])
            {
                gTasks[taskId].func = Some(Task_LaunchLvlUpAnim);
            } else {
                gTasks[taskId].func = Some(DestroyExpTaskAndCompleteOnInactiveTextPrinter);
            }
        } else {
            currExp += gainedExp as u32;
            SetMonData(mon, MON_DATA_EXP, &raw mut currExp as *mut c_void);
            gBattlerControllerFuncs[battler] = Some(CompleteOnInactiveTextPrinter);
            DestroyTask(taskId);
        }
    } else {
        gTasks[taskId].func = Some(Task_PrepareToGiveExpWithExpBar);
    }
}
pub(crate) unsafe extern "C" fn Task_PrepareToGiveExpWithExpBar(taskId: u8) {
    let mut monIndex: u8 = gTasks[taskId].data[0] as u8;
    let mut gainedExp: i32 = gTasks[taskId].data[1] as i32;
    let mut battler: u8 = gTasks[taskId].data[2] as u8;
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[monIndex];
    let mut level: u8 = GetMonData2(mon, MON_DATA_LEVEL) as u8;
    let mut species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
    let mut exp: u32 = GetMonData2(mon, MON_DATA_EXP);
    let mut currLvlExp: u32 = gExperienceTables[gSpeciesInfo[species].growthRate][level];
    let mut expToNextLvl: u32 = 0;
    exp -= currLvlExp;
    expToNextLvl =
        gExperienceTables[gSpeciesInfo[species].growthRate][level as i32 + 1] - currLvlExp;
    SetBattleBarStruct(
        battler,
        gHealthboxSpriteIds[battler],
        expToNextLvl as i32,
        exp as i32,
        -gainedExp,
    );
    PlaySE(SE_EXP);
    gTasks[taskId].func = Some(Task_GiveExpWithExpBar);
}
pub(crate) unsafe extern "C" fn Task_GiveExpWithExpBar(taskId: u8) {
    if gTasks[taskId].data[10] < 13 {
        gTasks[taskId].data[10] += 1;
    } else {
        let mut monId: u8 = gTasks[taskId].data[0] as u8;
        let mut gainedExp: i16 = gTasks[taskId].data[1];
        let mut battler: u8 = gTasks[taskId].data[2] as u8;
        let mut newExpPoints: i16 = 0;
        newExpPoints = MoveBattleBar(battler, gHealthboxSpriteIds[battler], EXP_BAR, 0) as i16;
        SetHealthboxSpriteVisible(gHealthboxSpriteIds[battler]);
        if newExpPoints == -1 {
            let mut level: u8 = 0;
            let mut currExp: i32 = 0;
            let mut species: u16 = 0;
            let mut expOnNextLvl: i32 = 0;
            m4aSongNumStop(SE_EXP);
            level = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) as u8;
            currExp = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_EXP) as i32;
            species = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPECIES) as u16;
            expOnNextLvl =
                gExperienceTables[gSpeciesInfo[species].growthRate][level as i32 + 1] as i32;
            if currExp + gainedExp as i32 >= expOnNextLvl {
                let mut savedActiveBattler: u8 = 0;
                SetMonData(
                    &raw mut gPlayerParty[monId],
                    MON_DATA_EXP,
                    &raw mut expOnNextLvl as *mut c_void,
                );
                CalculateMonStats(&raw mut gPlayerParty[monId]);
                gainedExp -= expOnNextLvl as i16 - currExp as i16;
                savedActiveBattler = gActiveBattler;
                gActiveBattler = battler;
                BtlController_EmitTwoReturnValues(
                    B_COMM_TO_ENGINE,
                    RET_VALUE_LEVELED_UP,
                    gainedExp as u16,
                );
                gActiveBattler = savedActiveBattler;
                gTasks[taskId].func = Some(Task_LaunchLvlUpAnim);
            } else {
                currExp += gainedExp as i32;
                SetMonData(
                    &raw mut gPlayerParty[monId],
                    MON_DATA_EXP,
                    &raw mut currExp as *mut c_void,
                );
                gBattlerControllerFuncs[battler] = Some(CompleteOnInactiveTextPrinter);
                DestroyTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LaunchLvlUpAnim(taskId: u8) {
    let mut battler: u8 = gTasks[taskId].data[2] as u8;
    let mut monIndex: u8 = gTasks[taskId].data[0] as u8;
    if IsDoubleBattle() == TRUE && monIndex as u16 == gBattlerPartyIndexes[battler as i32 ^ 2] {
        battler ^= BIT_FLANK;
    }
    InitAndLaunchSpecialAnimation(battler, battler, battler, B_ANIM_LVL_UP);
    gTasks[taskId].func = Some(Task_UpdateLvlInHealthbox);
}
pub(crate) unsafe extern "C" fn Task_UpdateLvlInHealthbox(taskId: u8) {
    let mut battler: u8 = gTasks[taskId].data[2] as u8;
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).specialAnimActive() == 0 {
        let mut monIndex: u8 = gTasks[taskId].data[0] as u8;
        GetMonData2(&raw mut gPlayerParty[monIndex], MON_DATA_LEVEL);
        if IsDoubleBattle() == TRUE && monIndex as u16 == gBattlerPartyIndexes[battler as i32 ^ 2] {
            UpdateHealthboxAttribute(
                gHealthboxSpriteIds[battler as i32 ^ 2],
                &raw mut gPlayerParty[monIndex],
                HEALTHBOX_ALL,
            );
        } else {
            UpdateHealthboxAttribute(
                gHealthboxSpriteIds[battler],
                &raw mut gPlayerParty[monIndex],
                HEALTHBOX_ALL,
            );
        }
        gTasks[taskId].func = Some(DestroyExpTaskAndCompleteOnInactiveTextPrinter);
    }
}
pub(crate) unsafe extern "C" fn DestroyExpTaskAndCompleteOnInactiveTextPrinter(taskId: u8) {
    let mut monIndex: u8 = 0;
    let mut battler: u8 = 0;
    monIndex = gTasks[taskId].data[0] as u8;
    GetMonData2(&raw mut gPlayerParty[monIndex], MON_DATA_LEVEL);
    battler = gTasks[taskId].data[2] as u8;
    gBattlerControllerFuncs[battler] = Some(CompleteOnInactiveTextPrinter);
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn FreeMonSpriteAfterFaintAnim() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].y as i32
        + gSprites[gBattlerSpriteIds[gActiveBattler]].y2 as i32
        > DISPLAY_HEIGHT as i32
    {
        let mut species: u16 = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_SPECIES,
        ) as u16;
        BattleGfxSfxDummy2(species);
        FreeOamMatrix(gSprites[gBattlerSpriteIds[gActiveBattler]].oam.matrixNum() as u8);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn FreeMonSpriteAfterSwitchOutAnim() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0 {
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnInactiveTextPrinter2() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn OpenPartyMenuToChooseMon() {
    if gPaletteFade.active() == 0 {
        let mut caseId: u8 = 0;
        gBattlerControllerFuncs[gActiveBattler] = Some(WaitForMonSelection);
        caseId = gTasks[gBattleControllerData[gActiveBattler]].data[0] as u8;
        DestroyTask(gBattleControllerData[gActiveBattler]);
        FreeAllWindowBuffers();
        OpenPartyMenuInBattle(caseId);
    }
}
pub(crate) unsafe extern "C" fn WaitForMonSelection() {
    if gMain.callback2 == Some(BattleMainCB2 as unsafe extern "C" fn())
        && gPaletteFade.active() == 0
    {
        if gPartyMenuUseExitCallback == TRUE {
            BtlController_EmitChosenMonReturnValue(
                B_COMM_TO_ENGINE,
                gSelectedMonPartyId,
                gBattlePartyCurrentOrder.as_mut_ptr(),
            );
        } else {
            BtlController_EmitChosenMonReturnValue(B_COMM_TO_ENGINE, PARTY_SIZE as u8, null_mut());
        }
        if gBattleBufferA[gActiveBattler][1] as i32 & 0xF == 1 {
            PrintLinkStandbyMsg();
        }
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn OpenBagAndChooseItem() {
    if gPaletteFade.active() == 0 {
        gBattlerControllerFuncs[gActiveBattler] = Some(CompleteWhenChoseItem);
        ReshowBattleScreenDummy();
        FreeAllWindowBuffers();
        CB2_BagMenuFromBattle();
    }
}
pub(crate) unsafe extern "C" fn CompleteWhenChoseItem() {
    if gMain.callback2 == Some(BattleMainCB2 as unsafe extern "C" fn())
        && gPaletteFade.active() == 0
    {
        BtlController_EmitOneReturnValue(B_COMM_TO_ENGINE, gSpecialVar_ItemId);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnSpecialAnimDone() {
    if gDoingBattleAnim == 0
        || (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0
    {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn DoHitAnimBlinkSpriteEffect() {
    let mut spriteId: u8 = gBattlerSpriteIds[gActiveBattler];
    if gSprites[spriteId].data[1] == 32 {
        gSprites[spriteId].data[1] = 0;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gDoingBattleAnim = FALSE;
        PlayerBufferExecCompleted();
    } else {
        if gSprites[spriteId].data[1] % 4 == 0 {
            gSprites[spriteId].set_invisible(gSprites[spriteId].invisible() ^ 1);
        }
        gSprites[spriteId].data[1] += 1;
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleYesNoInput() {
    if gMain.newKeys as i32 & DPAD_UP != 0 && gMultiUsePlayerCursor != 0 {
        PlaySE(SE_SELECT);
        BattleDestroyYesNoCursorAt(gMultiUsePlayerCursor);
        gMultiUsePlayerCursor = 0;
        BattleCreateYesNoCursorAt(0);
    }
    if gMain.newKeys as i32 & DPAD_DOWN != 0 && gMultiUsePlayerCursor == 0 {
        PlaySE(SE_SELECT);
        BattleDestroyYesNoCursorAt(gMultiUsePlayerCursor);
        gMultiUsePlayerCursor = 1;
        BattleCreateYesNoCursorAt(1);
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
        PlaySE(SE_SELECT);
        if gMultiUsePlayerCursor != 0 {
            BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_UNK_14, 0);
        } else {
            BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_NOTHING_FAINTED, 0);
        }
        PlayerBufferExecCompleted();
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
        PlaySE(SE_SELECT);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn MoveSelectionDisplayMoveNames() {
    let mut i: i32 = 0;
    let mut moveInfo: *mut ChooseMoveStruct =
        &raw mut gBattleBufferA[gActiveBattler][4] as *mut ChooseMoveStruct;
    gNumberOfMovesToChoose = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        MoveSelectionDestroyCursorAt(i as u8);
        StringCopy(
            gDisplayedStringBattle.as_mut_ptr(),
            gMoveNames[(*moveInfo).moves[i]].as_ptr().cast_mut(),
        );
        BattlePutTextOnWindow(
            gDisplayedStringBattle.as_mut_ptr(),
            i as u8 + B_WIN_MOVE_NAME_1,
        );
        if (*moveInfo).moves[i] != MOVE_NONE {
            gNumberOfMovesToChoose += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn MoveSelectionDisplayPPString() {
    StringCopy(
        gDisplayedStringBattle.as_mut_ptr(),
        gText_MoveInterfacePP.as_ptr().cast_mut(),
    );
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_PP);
}
pub(crate) unsafe extern "C" fn MoveSelectionDisplayPPNumber() {
    let mut txtPtr: *mut u8 = null_mut();
    let mut moveInfo: *mut ChooseMoveStruct = null_mut();
    if gBattleBufferA[gActiveBattler][2] == TRUE {
        return;
    }
    SetPPNumbersPaletteInMoveSelection();
    moveInfo = &raw mut gBattleBufferA[gActiveBattler][4] as *mut ChooseMoveStruct;
    txtPtr = ConvertIntToDecimalStringN(
        gDisplayedStringBattle.as_mut_ptr(),
        (*moveInfo).currentPP[gMoveSelectionCursor[gActiveBattler]] as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        2,
    );
    *({
        let t1 = txtPtr;
        txtPtr = txtPtr.at(1);
        t1
    }) = CHAR_SLASH;
    ConvertIntToDecimalStringN(
        txtPtr,
        (*moveInfo).maxPP[gMoveSelectionCursor[gActiveBattler]] as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        2,
    );
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_PP_REMAINING);
}
pub(crate) unsafe extern "C" fn MoveSelectionDisplayMoveType() {
    let mut txtPtr: *mut u8 = null_mut();
    let mut moveInfo: *mut ChooseMoveStruct =
        &raw mut gBattleBufferA[gActiveBattler][4] as *mut ChooseMoveStruct;
    txtPtr = StringCopy(
        gDisplayedStringBattle.as_mut_ptr(),
        gText_MoveInterfaceType.as_ptr().cast_mut(),
    );
    *({
        let t1 = txtPtr;
        txtPtr = txtPtr.at(1);
        t1
    }) = EXT_CTRL_CODE_BEGIN;
    *({
        let t2 = txtPtr;
        txtPtr = txtPtr.at(1);
        t2
    }) = EXT_CTRL_CODE_FONT;
    *({
        let t3 = txtPtr;
        txtPtr = txtPtr.at(1);
        t3
    }) = FONT_NORMAL;
    StringCopy(
        txtPtr,
        gTypeNames[gBattleMoves[(*moveInfo).moves[gMoveSelectionCursor[gActiveBattler]]].r#type]
            .as_ptr()
            .cast_mut(),
    );
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MOVE_TYPE);
}
pub(crate) unsafe extern "C" fn MoveSelectionCreateCursorAt(cursorPosition: u8, baseTileNum: u8) {
    let mut src: CArray<u16, 2> = zeroed();
    src[0] = baseTileNum as u16 + 1;
    src[1] = baseTileNum as u16 + 2;
    CopyToBgTilemapBufferRect_ChangePalette(
        0,
        src.as_mut_ptr() as *mut c_void,
        9 * (cursorPosition & 1) + 1,
        55 + (cursorPosition & 2),
        1,
        2,
        0x11,
    );
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn MoveSelectionDestroyCursorAt(cursorPosition: u8) {
    let mut src: CArray<u16, 2> = zeroed();
    src[0] = 0x1016;
    src[1] = 0x1016;
    CopyToBgTilemapBufferRect_ChangePalette(
        0,
        src.as_mut_ptr() as *mut c_void,
        9 * (cursorPosition & 1) + 1,
        55 + (cursorPosition & 2),
        1,
        2,
        0x11,
    );
    CopyBgTilemapBufferToVram(0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ActionSelectionCreateCursorAt(cursorPosition: u8, baseTileNum: u8) {
    let mut src: CArray<u16, 2> = zeroed();
    src[0] = 1;
    src[1] = 2;
    CopyToBgTilemapBufferRect_ChangePalette(
        0,
        src.as_mut_ptr() as *mut c_void,
        7 * (cursorPosition & 1) + 16,
        35 + (cursorPosition & 2),
        1,
        2,
        0x11,
    );
    CopyBgTilemapBufferToVram(0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ActionSelectionDestroyCursorAt(cursorPosition: u8) {
    let mut src: CArray<u16, 2> = zeroed();
    src[0] = 0x1016;
    src[1] = 0x1016;
    CopyToBgTilemapBufferRect_ChangePalette(
        0,
        src.as_mut_ptr() as *mut c_void,
        7 * (cursorPosition & 1) + 16,
        35 + (cursorPosition & 2),
        1,
        2,
        0x11,
    );
    CopyBgTilemapBufferToVram(0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_SetUpReshowBattleScreenAfterMenu() {
    SetMainCallback2(Some(ReshowBattleScreenAfterMenu));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_SetUpReshowBattleScreenAfterMenu2() {
    SetMainCallback2(Some(ReshowBattleScreenAfterMenu));
}
pub(crate) unsafe extern "C" fn CompleteOnFinishedStatusAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).statusAnimActive() == 0 {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnFinishedBattleAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animFromTableActive() == 0 {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PrintLinkStandbyMsg() {
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        gBattle_BG0_X = 0;
        gBattle_BG0_Y = 0;
        BattlePutTextOnWindow(gText_LinkStandby.as_ptr().cast_mut(), B_WIN_MSG);
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleGetMonData() {
    let mut monData: CArray<u8, 256> = zeroed();
    let mut size: u32 = 0;
    let mut monToCheck: u8 = 0;
    let mut i: i32 = 0;
    if gBattleBufferA[gActiveBattler][2] == 0 {
        size += CopyPlayerMonData(
            gBattlerPartyIndexes[gActiveBattler] as u8,
            monData.as_mut_ptr(),
        );
    } else {
        monToCheck = gBattleBufferA[gActiveBattler][2];
        i = 0;
        while i < PARTY_SIZE {
            if monToCheck as i32 & 1 != 0 {
                size += CopyPlayerMonData(i as u8, monData.as_mut_ptr().at(size));
            }
            monToCheck >>= 1;
            i += 1;
        }
    }
    BtlController_EmitDataTransfer(
        B_COMM_TO_ENGINE,
        size as u16,
        monData.as_mut_ptr() as *mut c_void,
    );
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn CopyPlayerMonData(monId: u8, mut dst: *mut u8) -> u32 {
    let mut battleMon: BattlePokemon = zeroed();
    let mut moveData: MovePPInfo = zeroed();
    let mut nickname: CArray<u8, 20> = zeroed();
    let mut src: *mut u8 = null_mut();
    let mut data16: i16 = 0;
    let mut data32: u32 = 0;
    let mut size: i32 = 0;
    match gBattleBufferA[gActiveBattler][1] {
        REQUEST_ALL_BATTLE => {
            battleMon.species = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPECIES) as u16;
            battleMon.item = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HELD_ITEM) as u16;
            size = 0;
            while size < MAX_MON_MOVES {
                battleMon.moves[size] =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MOVE1 + size) as u16;
                battleMon.pp[size] =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP1 + size) as u8;
                size += 1;
            }
            battleMon.ppBonuses =
                GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP_BONUSES) as u8;
            battleMon.friendship =
                GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_FRIENDSHIP) as u8;
            battleMon.experience = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_EXP);
            battleMon.set_hpIV(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP_IV));
            battleMon.set_attackIV(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_ATK_IV));
            battleMon.set_defenseIV(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_DEF_IV));
            battleMon.set_speedIV(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPEED_IV));
            battleMon.set_spAttackIV(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPATK_IV));
            battleMon.set_spDefenseIV(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPDEF_IV));
            battleMon.personality = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PERSONALITY);
            battleMon.status1 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_STATUS);
            battleMon.level = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) as u8;
            battleMon.hp = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP) as u16;
            battleMon.maxHP = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MAX_HP) as u16;
            battleMon.attack = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_ATK) as u16;
            battleMon.defense = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_DEF) as u16;
            battleMon.speed = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPEED) as u16;
            battleMon.spAttack = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPATK) as u16;
            battleMon.spDefense = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPDEF) as u16;
            battleMon.set_isEgg(GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_IS_EGG));
            battleMon.set_abilityNum(GetMonData2(
                &raw mut gPlayerParty[monId],
                MON_DATA_ABILITY_NUM,
            ));
            battleMon.otId = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_OT_ID);
            GetMonData3(
                &raw mut gPlayerParty[monId],
                MON_DATA_NICKNAME,
                nickname.as_mut_ptr(),
            );
            StringCopy_Nickname(battleMon.nickname.as_mut_ptr(), nickname.as_mut_ptr());
            GetMonData3(
                &raw mut gPlayerParty[monId],
                MON_DATA_OT_NAME,
                battleMon.otName.as_mut_ptr(),
            );
            src = &raw mut battleMon as *mut u8;
            size = 0;
            while size < 88 {
                *dst.at(size) = *src.at(size);
                size += 1;
            }
        }
        REQUEST_SPECIES_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPECIES) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_HELDITEM_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HELD_ITEM) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_MOVES_PP_BATTLE => {
            size = 0;
            while size < MAX_MON_MOVES {
                moveData.moves[size] =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MOVE1 + size) as u16;
                moveData.pp[size] =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP1 + size) as u8;
                size += 1;
            }
            moveData.ppBonuses =
                GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP_BONUSES) as u8;
            src = &raw mut moveData as *mut u8;
            size = 0;
            while size < 16 {
                *dst.at(size) = *src.at(size);
                size += 1;
            }
        }
        4 | REQUEST_MOVE2_BATTLE | REQUEST_MOVE3_BATTLE | REQUEST_MOVE4_BATTLE => {
            data16 = GetMonData2(
                &raw mut gPlayerParty[monId],
                MON_DATA_MOVE1 + gBattleBufferA[gActiveBattler][1] as i32 - REQUEST_MOVE1_BATTLE,
            ) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_PP_DATA_BATTLE => {
            size = 0;
            while size < MAX_MON_MOVES {
                *dst.at(size) =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP1 + size) as u8;
                size += 1;
            }
            *dst.at(size) = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP_BONUSES) as u8;
            size += 1;
        }
        REQUEST_PPMOVE1_BATTLE
        | REQUEST_PPMOVE2_BATTLE
        | REQUEST_PPMOVE3_BATTLE
        | REQUEST_PPMOVE4_BATTLE => {
            *dst = GetMonData2(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP1 + gBattleBufferA[gActiveBattler][1] as i32
                    - REQUEST_PPMOVE1_BATTLE as i32,
            ) as u8;
            size = 1;
        }
        REQUEST_OTID_BATTLE => {
            data32 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_OT_ID);
            *dst = data32 as u8 & 0x000000FF;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_EXP_BATTLE => {
            data32 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_EXP);
            *dst = data32 as u8 & 0x000000FF;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_HP_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP_EV) as u8;
            size = 1;
        }
        REQUEST_ATK_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_ATK_EV) as u8;
            size = 1;
        }
        REQUEST_DEF_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_DEF_EV) as u8;
            size = 1;
        }
        REQUEST_SPEED_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPEED_EV) as u8;
            size = 1;
        }
        REQUEST_SPATK_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPATK_EV) as u8;
            size = 1;
        }
        REQUEST_SPDEF_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPDEF_EV) as u8;
            size = 1;
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_FRIENDSHIP) as u8;
            size = 1;
        }
        REQUEST_POKERUS_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_POKERUS) as u8;
            size = 1;
        }
        REQUEST_MET_LOCATION_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MET_LOCATION) as u8;
            size = 1;
        }
        REQUEST_MET_LEVEL_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MET_LEVEL) as u8;
            size = 1;
        }
        REQUEST_MET_GAME_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MET_GAME) as u8;
            size = 1;
        }
        REQUEST_POKEBALL_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_POKEBALL) as u8;
            size = 1;
        }
        REQUEST_ALL_IVS_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP_IV) as u8;
            *dst.at(1) = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_ATK_IV) as u8;
            *dst.at(2) = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_DEF_IV) as u8;
            *dst.at(3) = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPEED_IV) as u8;
            *dst.at(4) = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPATK_IV) as u8;
            *dst.at(5) = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPDEF_IV) as u8;
            size = 6;
        }
        REQUEST_HP_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP_IV) as u8;
            size = 1;
        }
        REQUEST_ATK_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_ATK_IV) as u8;
            size = 1;
        }
        REQUEST_DEF_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_DEF_IV) as u8;
            size = 1;
        }
        REQUEST_SPEED_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPEED_IV) as u8;
            size = 1;
        }
        REQUEST_SPATK_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPATK_IV) as u8;
            size = 1;
        }
        REQUEST_SPDEF_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPDEF_IV) as u8;
            size = 1;
        }
        REQUEST_PERSONALITY_BATTLE => {
            data32 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PERSONALITY);
            *dst = data32 as u8 & 0x000000FF;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            *dst.at(3) = ((data32 & 0xFF000000) >> 24) as u8;
            size = 4;
        }
        REQUEST_CHECKSUM_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_CHECKSUM) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_STATUS_BATTLE => {
            data32 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_STATUS);
            *dst = data32 as u8 & 0x000000FF;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            *dst.at(3) = ((data32 & 0xFF000000) >> 24) as u8;
            size = 4;
        }
        REQUEST_LEVEL_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) as u8;
            size = 1;
        }
        REQUEST_HP_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_MAX_HP_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MAX_HP) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_ATK_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_ATK) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_DEF_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_DEF) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPEED_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPEED) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPATK_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPATK) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPDEF_BATTLE => {
            data16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPDEF) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_COOL_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_COOL) as u8;
            size = 1;
        }
        REQUEST_BEAUTY_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_BEAUTY) as u8;
            size = 1;
        }
        REQUEST_CUTE_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_CUTE) as u8;
            size = 1;
        }
        REQUEST_SMART_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SMART) as u8;
            size = 1;
        }
        REQUEST_TOUGH_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_TOUGH) as u8;
            size = 1;
        }
        REQUEST_SHEEN_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SHEEN) as u8;
            size = 1;
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_COOL_RIBBON) as u8;
            size = 1;
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_BEAUTY_RIBBON) as u8;
            size = 1;
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_CUTE_RIBBON) as u8;
            size = 1;
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SMART_RIBBON) as u8;
            size = 1;
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_TOUGH_RIBBON) as u8;
            size = 1;
        }
        _ => {}
    }
    return size as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerHandleGetRawMonData() {
    let mut battleMon: BattlePokemon = zeroed();
    let mut src: *mut u8 = (&raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]] as *mut u8)
        .at(gBattleBufferA[gActiveBattler][1]);
    let mut dst: *mut u8 = (&raw mut battleMon as *mut u8).at(gBattleBufferA[gActiveBattler][1]);
    let mut i: u8 = 0;
    i = 0;
    while i < gBattleBufferA[gActiveBattler][2] {
        *dst.at(i) = *src.at(i);
        i += 1;
    }
    BtlController_EmitDataTransfer(
        B_COMM_TO_ENGINE,
        gBattleBufferA[gActiveBattler][2] as u16,
        dst as *mut c_void,
    );
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleSetMonData() {
    let mut monToCheck: u8 = 0;
    let mut i: u8 = 0;
    if gBattleBufferA[gActiveBattler][2] == 0 {
        SetPlayerMonData(gBattlerPartyIndexes[gActiveBattler] as u8);
    } else {
        monToCheck = gBattleBufferA[gActiveBattler][2];
        i = 0;
        while i < PARTY_SIZE as u8 {
            if monToCheck as i32 & 1 != 0 {
                SetPlayerMonData(i);
            }
            monToCheck >>= 1;
            i += 1;
        }
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SetPlayerMonData(monId: u8) {
    let mut battlePokemon: *mut BattlePokemon =
        &raw mut gBattleBufferA[gActiveBattler][3] as *mut BattlePokemon;
    let mut moveData: *mut MovePPInfo =
        &raw mut gBattleBufferA[gActiveBattler][3] as *mut MovePPInfo;
    let mut i: i32 = 0;
    match gBattleBufferA[gActiveBattler][1] {
        REQUEST_ALL_BATTLE => {
            let mut iv: u8 = 0;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPECIES,
                &raw mut (*battlePokemon).species as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HELD_ITEM,
                &raw mut (*battlePokemon).item as *mut c_void,
            );
            i = 0;
            while i < MAX_MON_MOVES {
                SetMonData(
                    &raw mut gPlayerParty[monId],
                    MON_DATA_MOVE1 + i,
                    &raw mut (*battlePokemon).moves[i] as *mut c_void,
                );
                SetMonData(
                    &raw mut gPlayerParty[monId],
                    MON_DATA_PP1 + i,
                    &raw mut (*battlePokemon).pp[i] as *mut c_void,
                );
                i += 1;
            }
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP_BONUSES,
                &raw mut (*battlePokemon).ppBonuses as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_FRIENDSHIP,
                &raw mut (*battlePokemon).friendship as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_EXP,
                &raw mut (*battlePokemon).experience as *mut c_void,
            );
            iv = (*battlePokemon).hpIV() as u8;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).attackIV() as u8;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).defenseIV() as u8;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).speedIV() as u8;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).spAttackIV() as u8;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).spDefenseIV() as u8;
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF_IV,
                &raw mut iv as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PERSONALITY,
                &raw mut (*battlePokemon).personality as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_STATUS,
                &raw mut (*battlePokemon).status1 as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_LEVEL,
                &raw mut (*battlePokemon).level as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP,
                &raw mut (*battlePokemon).hp as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MAX_HP,
                &raw mut (*battlePokemon).maxHP as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK,
                &raw mut (*battlePokemon).attack as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF,
                &raw mut (*battlePokemon).defense as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED,
                &raw mut (*battlePokemon).speed as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK,
                &raw mut (*battlePokemon).spAttack as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF,
                &raw mut (*battlePokemon).spDefense as *mut c_void,
            );
        }
        REQUEST_SPECIES_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPECIES,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_HELDITEM_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HELD_ITEM,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MOVES_PP_BATTLE => {
            i = 0;
            while i < MAX_MON_MOVES {
                SetMonData(
                    &raw mut gPlayerParty[monId],
                    MON_DATA_MOVE1 + i,
                    &raw mut (*moveData).moves[i] as *mut c_void,
                );
                SetMonData(
                    &raw mut gPlayerParty[monId],
                    MON_DATA_PP1 + i,
                    &raw mut (*moveData).pp[i] as *mut c_void,
                );
                i += 1;
            }
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP_BONUSES,
                &raw mut (*moveData).ppBonuses as *mut c_void,
            );
        }
        4 | REQUEST_MOVE2_BATTLE | REQUEST_MOVE3_BATTLE | REQUEST_MOVE4_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MOVE1 + gBattleBufferA[gActiveBattler][1] as i32 - REQUEST_MOVE1_BATTLE,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_PP_DATA_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP1,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP2,
                &raw mut gBattleBufferA[gActiveBattler][4] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP3,
                &raw mut gBattleBufferA[gActiveBattler][5] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP4,
                &raw mut gBattleBufferA[gActiveBattler][6] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP_BONUSES,
                &raw mut gBattleBufferA[gActiveBattler][7] as *mut c_void,
            );
        }
        REQUEST_PPMOVE1_BATTLE
        | REQUEST_PPMOVE2_BATTLE
        | REQUEST_PPMOVE3_BATTLE
        | REQUEST_PPMOVE4_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP1 + gBattleBufferA[gActiveBattler][1] as i32
                    - REQUEST_PPMOVE1_BATTLE as i32,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_OTID_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_OT_ID,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_EXP_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_EXP,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_HP_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP_EV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ATK_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK_EV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_DEF_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF_EV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED_EV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK_EV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF_EV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_FRIENDSHIP,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_POKERUS_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_POKERUS,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MET_LOCATION_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MET_LOCATION,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MET_LEVEL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MET_LEVEL,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MET_GAME_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MET_GAME,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_POKEBALL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_POKEBALL,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ALL_IVS_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK_IV,
                &raw mut gBattleBufferA[gActiveBattler][4] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF_IV,
                &raw mut gBattleBufferA[gActiveBattler][5] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED_IV,
                &raw mut gBattleBufferA[gActiveBattler][6] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK_IV,
                &raw mut gBattleBufferA[gActiveBattler][7] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF_IV,
                &raw mut gBattleBufferA[gActiveBattler][8] as *mut c_void,
            );
        }
        REQUEST_HP_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ATK_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_DEF_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_PERSONALITY_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PERSONALITY,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_CHECKSUM_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_CHECKSUM,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_STATUS_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_STATUS,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_LEVEL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_LEVEL,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_HP_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MAX_HP_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MAX_HP,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ATK_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_DEF_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_COOL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_COOL,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_BEAUTY,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_CUTE,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SMART_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SMART,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_TOUGH,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SHEEN_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SHEEN,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_COOL_RIBBON,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_BEAUTY_RIBBON,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_CUTE_RIBBON,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SMART_RIBBON,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_TOUGH_RIBBON,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        _ => {}
    }
    HandleLowHpMusicChange(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        gActiveBattler,
    );
}
pub(crate) unsafe extern "C" fn PlayerHandleSetRawMonData() {
    let mut dst: *mut u8 = (&raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]] as *mut u8)
        .at(gBattleBufferA[gActiveBattler][1]);
    let mut i: u8 = 0;
    i = 0;
    while i < gBattleBufferA[gActiveBattler][2] {
        *dst.at(i) = gBattleBufferA[gActiveBattler][3 + i as i32];
        i += 1;
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleLoadMonSprite() {
    BattleLoadPlayerMonSpriteGfx(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        gActiveBattler,
    );
    gSprites[gBattlerSpriteIds[gActiveBattler]]
        .oam
        .set_paletteNum(gActiveBattler as u16);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnBankSpritePosX_0);
}
pub(crate) unsafe extern "C" fn PlayerHandleSwitchInAnim() {
    ClearTemporarySpeciesSpriteData(gActiveBattler, gBattleBufferA[gActiveBattler][2]);
    gBattlerPartyIndexes[gActiveBattler] = gBattleBufferA[gActiveBattler][1] as u16;
    BattleLoadPlayerMonSpriteGfx(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        gActiveBattler,
    );
    gActionSelectionCursor[gActiveBattler] = 0;
    gMoveSelectionCursor[gActiveBattler] = 0;
    StartSendOutAnim(gActiveBattler, gBattleBufferA[gActiveBattler][2]);
    gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_TryShinyAnimShowHealthbox);
}
pub(crate) unsafe extern "C" fn StartSendOutAnim(battler: u8, dontClearSubstituteBit: u8) {
    let mut species: u16 = 0;
    ClearTemporarySpeciesSpriteData(battler, dontClearSubstituteBit);
    gBattlerPartyIndexes[battler] = gBattleBufferA[battler][1] as u16;
    species = GetMonData2(
        &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
        MON_DATA_SPECIES,
    ) as u16;
    gBattleControllerData[battler] =
        CreateInvisibleSpriteWithCallback(Some(SpriteCB_WaitForBattlerBallReleaseAnim));
    SetMultiuseSpriteTemplateToPokemon(species, GetBattlerPosition(battler));
    gBattlerSpriteIds[battler] = CreateSprite(
        &raw mut gMultiuseSpriteTemplate,
        GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16,
        GetBattlerSpriteDefault_Y(battler) as i16,
        GetBattlerSpriteSubpriority(battler),
    );
    gSprites[gBattleControllerData[battler]].data[1] = gBattlerSpriteIds[battler] as i16;
    gSprites[gBattleControllerData[battler]].data[2] = battler as i16;
    gSprites[gBattlerSpriteIds[battler]].data[0] = battler as i16;
    gSprites[gBattlerSpriteIds[battler]].data[2] = species as i16;
    gSprites[gBattlerSpriteIds[battler]]
        .oam
        .set_paletteNum(battler as u16);
    StartSpriteAnim(
        &raw mut gSprites[gBattlerSpriteIds[battler]],
        gBattleMonForms[battler],
    );
    gSprites[gBattlerSpriteIds[battler]].set_invisible(TRUE as u16);
    gSprites[gBattlerSpriteIds[battler]].callback = Some(SpriteCallbackDummy);
    gSprites[gBattleControllerData[battler]].data[0] =
        DoPokeballSendOutAnimation(0, POKEBALL_PLAYER_SENDOUT) as i16;
}
pub(crate) unsafe extern "C" fn PlayerHandleReturnMonToBall() {
    if gBattleBufferA[gActiveBattler][1] == 0 {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
        gBattlerControllerFuncs[gActiveBattler] = Some(DoSwitchOutAnimation);
    } else {
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn DoSwitchOutAnimation() {
    match (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState {
        0 => {
            if (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).behindSubstitute() != 0 {
                InitAndLaunchSpecialAnimation(
                    gActiveBattler,
                    gActiveBattler,
                    gActiveBattler,
                    B_ANIM_SUBSTITUTE_TO_MON,
                );
            }
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 1;
        }
        1 => {
            if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive()
                == 0
            {
                (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
                InitAndLaunchSpecialAnimation(
                    gActiveBattler,
                    gActiveBattler,
                    gActiveBattler,
                    B_ANIM_SWITCH_OUT_PLAYER_MON,
                );
                gBattlerControllerFuncs[gActiveBattler] = Some(FreeMonSpriteAfterSwitchOutAnim);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleDrawTrainerPic() {
    let mut xPos: i16 = 0;
    let mut yPos: i16 = 0;
    let mut trainerPicId: u32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        if gLinkPlayers[GetMultiplayerId()].version as i32 & 0xFF == VERSION_FIRE_RED
            || gLinkPlayers[GetMultiplayerId()].version as i32 & 0xFF == VERSION_LEAF_GREEN
        {
            trainerPicId = gLinkPlayers[GetMultiplayerId()].gender as u32 + TRAINER_BACK_PIC_RED;
        } else if gLinkPlayers[GetMultiplayerId()].version as i32 & 0xFF == VERSION_RUBY
            || gLinkPlayers[GetMultiplayerId()].version as i32 & 0xFF == VERSION_SAPPHIRE
        {
            trainerPicId = gLinkPlayers[GetMultiplayerId()].gender as u32
                + TRAINER_BACK_PIC_RUBY_SAPPHIRE_BRENDAN;
        } else {
            trainerPicId =
                gLinkPlayers[GetMultiplayerId()].gender as u32 + TRAINER_BACK_PIC_BRENDAN;
        }
    } else {
        trainerPicId = (*gSaveBlock2Ptr).playerGender as u32;
    }
    if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if GetBattlerPosition(gActiveBattler) as i32 & BIT_FLANK as i32 != B_FLANK_LEFT {
            xPos = 90;
        } else {
            xPos = 32;
        }
        if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0
            && gPartnerTrainerId != TRAINER_STEVEN_PARTNER
        {
            xPos = 90;
            yPos = (8 - gTrainerFrontPicCoords[trainerPicId].size as i16) * 4 + 80;
        } else {
            yPos = (8 - gTrainerBackPicCoords[trainerPicId].size as i16) * 4 + 80;
        }
    } else {
        xPos = 80;
        yPos = (8 - gTrainerBackPicCoords[trainerPicId].size as i16) * 4 + 80;
    }
    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0
        && gPartnerTrainerId != TRAINER_STEVEN_PARTNER
    {
        trainerPicId = PlayerGenderToFrontTrainerPicId((*gSaveBlock2Ptr).playerGender) as u32;
        DecompressTrainerFrontPic(trainerPicId as u16, gActiveBattler);
        SetMultiuseSpriteTemplateToTrainerFront(
            trainerPicId as u16,
            GetBattlerPosition(gActiveBattler),
        );
        gBattlerSpriteIds[gActiveBattler] = CreateSprite(
            &raw mut gMultiuseSpriteTemplate,
            xPos,
            yPos,
            GetBattlerSpriteSubpriority(gActiveBattler),
        );
        gSprites[gBattlerSpriteIds[gActiveBattler]]
            .oam
            .set_paletteNum(
                IndexOfSpritePaletteTag(gTrainerFrontPicPaletteTable[trainerPicId].tag) as u16,
            );
        gSprites[gBattlerSpriteIds[gActiveBattler]].x2 = DISPLAY_WIDTH as i16;
        gSprites[gBattlerSpriteIds[gActiveBattler]].y2 = 48;
        gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = -2;
        gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(SpriteCB_TrainerSlideIn);
        gSprites[gBattlerSpriteIds[gActiveBattler]]
            .oam
            .set_affineMode(ST_OAM_AFFINE_OFF);
        gSprites[gBattlerSpriteIds[gActiveBattler]].set_hFlip(1);
    } else {
        DecompressTrainerBackPic(trainerPicId as u16, gActiveBattler);
        SetMultiuseSpriteTemplateToTrainerBack(
            trainerPicId as u16,
            GetBattlerPosition(gActiveBattler),
        );
        gBattlerSpriteIds[gActiveBattler] = CreateSprite(
            &raw mut gMultiuseSpriteTemplate,
            xPos,
            yPos,
            GetBattlerSpriteSubpriority(gActiveBattler),
        );
        gSprites[gBattlerSpriteIds[gActiveBattler]]
            .oam
            .set_paletteNum(gActiveBattler as u16);
        gSprites[gBattlerSpriteIds[gActiveBattler]].x2 = DISPLAY_WIDTH as i16;
        gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = -2;
        gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(SpriteCB_TrainerSlideIn);
    }
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnBattlerSpriteCallbackDummy);
}
pub(crate) unsafe extern "C" fn PlayerHandleTrainerSlide() {
    let mut trainerPicId: u32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        if gLinkPlayers[GetMultiplayerId()].version as i32 & 0xFF == VERSION_FIRE_RED
            || gLinkPlayers[GetMultiplayerId()].version as i32 & 0xFF == VERSION_LEAF_GREEN
        {
            trainerPicId = gLinkPlayers[GetMultiplayerId()].gender as u32 + TRAINER_BACK_PIC_RED;
        } else if gLinkPlayers[GetMultiplayerId()].version as i32 & 0xFF == VERSION_RUBY
            || gLinkPlayers[GetMultiplayerId()].version as i32 & 0xFF == VERSION_SAPPHIRE
        {
            trainerPicId = gLinkPlayers[GetMultiplayerId()].gender as u32
                + TRAINER_BACK_PIC_RUBY_SAPPHIRE_BRENDAN;
        } else {
            trainerPicId =
                gLinkPlayers[GetMultiplayerId()].gender as u32 + TRAINER_BACK_PIC_BRENDAN;
        }
    } else {
        trainerPicId = (*gSaveBlock2Ptr).playerGender as u32 + TRAINER_BACK_PIC_BRENDAN;
    }
    DecompressTrainerBackPic(trainerPicId as u16, gActiveBattler);
    SetMultiuseSpriteTemplateToTrainerBack(trainerPicId as u16, GetBattlerPosition(gActiveBattler));
    gBattlerSpriteIds[gActiveBattler] = CreateSprite(
        &raw mut gMultiuseSpriteTemplate,
        80,
        (8 - gTrainerBackPicCoords[trainerPicId].size as i16) * 4 + 80,
        30,
    );
    gSprites[gBattlerSpriteIds[gActiveBattler]]
        .oam
        .set_paletteNum(gActiveBattler as u16);
    gSprites[gBattlerSpriteIds[gActiveBattler]].x2 = -96;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = 2;
    gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(SpriteCB_TrainerSlideIn);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnBankSpriteCallbackDummy2);
}
pub(crate) unsafe extern "C" fn PlayerHandleTrainerSlideBack() {
    SetSpritePrimaryCoordsFromSecondaryCoords(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = 50;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[2] = -40;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[4] =
        gSprites[gBattlerSpriteIds[gActiveBattler]].y;
    gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(
        &raw mut gSprites[gBattlerSpriteIds[gActiveBattler]],
        Some(SpriteCallbackDummy),
    );
    StartSpriteAnim(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]], 1);
    gBattlerControllerFuncs[gActiveBattler] = Some(FreeTrainerSpriteAfterSlide);
}
pub(crate) unsafe extern "C" fn PlayerHandleFaintAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState == 0 {
        if (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).behindSubstitute() != 0 {
            InitAndLaunchSpecialAnimation(
                gActiveBattler,
                gActiveBattler,
                gActiveBattler,
                B_ANIM_SUBSTITUTE_TO_MON,
            );
        }
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState += 1;
    } else {
        if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0 {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
            HandleLowHpMusicChange(
                &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                gActiveBattler,
            );
            PlaySE12WithPanning(SE_FAINT, SOUND_PAN_ATTACKER);
            gSprites[gBattlerSpriteIds[gActiveBattler]].data[1] = 0;
            gSprites[gBattlerSpriteIds[gActiveBattler]].data[2] = 5;
            gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(SpriteCB_FaintSlideAnim);
            gBattlerControllerFuncs[gActiveBattler] = Some(FreeMonSpriteAfterFaintAnim);
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerHandlePaletteFade() {
    BeginNormalPaletteFade(PALETTES_ALL, 2, 0, 16, 0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleSuccessBallThrowAnim() {
    (*(*gBattleSpritesDataPtr).animationData).ballThrowCaseId = BALL_3_SHAKES_SUCCESS;
    gDoingBattleAnim = TRUE;
    InitAndLaunchSpecialAnimation(
        gActiveBattler,
        gActiveBattler,
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        B_ANIM_BALL_THROW,
    );
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnSpecialAnimDone);
}
pub(crate) unsafe extern "C" fn PlayerHandleBallThrowAnim() {
    let mut ballThrowCaseId: u8 = gBattleBufferA[gActiveBattler][1];
    (*(*gBattleSpritesDataPtr).animationData).ballThrowCaseId = ballThrowCaseId;
    gDoingBattleAnim = TRUE;
    InitAndLaunchSpecialAnimation(
        gActiveBattler,
        gActiveBattler,
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        B_ANIM_BALL_THROW,
    );
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnSpecialAnimDone);
}
pub(crate) unsafe extern "C" fn PlayerHandlePause() {
    let mut timer: u8 = gBattleBufferA[gActiveBattler][1];
    while timer != 0 {
        timer -= 1;
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleMoveAnimation() {
    if IsBattleSEPlaying(gActiveBattler) == 0 {
        let mut r#move: u16 = gBattleBufferA[gActiveBattler][1] as u16
            | (gBattleBufferA[gActiveBattler][2] as u16) << 8;
        gAnimMoveTurn = gBattleBufferA[gActiveBattler][3];
        gAnimMovePower = gBattleBufferA[gActiveBattler][4] as u16
            | (gBattleBufferA[gActiveBattler][5] as u16) << 8;
        gAnimMoveDmg = gBattleBufferA[gActiveBattler][6] as i32
            | (gBattleBufferA[gActiveBattler][7] as i32) << 8
            | (gBattleBufferA[gActiveBattler][8] as i32) << 16
            | (gBattleBufferA[gActiveBattler][9] as i32) << 24;
        gAnimFriendship = gBattleBufferA[gActiveBattler][10];
        gWeatherMoveAnim = gBattleBufferA[gActiveBattler][12] as u16
            | (gBattleBufferA[gActiveBattler][13] as u16) << 8;
        gAnimDisableStructPtr = &raw mut gBattleBufferA[gActiveBattler][16] as *mut DisableStruct;
        gTransformedPersonalities[gActiveBattler] =
            (*gAnimDisableStructPtr).transformedMonPersonality;
        if IsMoveWithoutAnimation(r#move, gAnimMoveTurn) != 0 {
            PlayerBufferExecCompleted();
        } else {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
            gBattlerControllerFuncs[gActiveBattler] = Some(PlayerDoMoveAnimation);
            BattleTv_SetDataBasedOnMove(r#move, gWeatherMoveAnim, gAnimDisableStructPtr);
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerDoMoveAnimation() {
    let mut r#move: u16 =
        gBattleBufferA[gActiveBattler][1] as u16 | (gBattleBufferA[gActiveBattler][2] as u16) << 8;
    let mut multihit: u8 = gBattleBufferA[gActiveBattler][11];
    match (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState {
        0 => {
            if (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).behindSubstitute() != 0
                && (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).flag_x8() == 0
            {
                (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).set_flag_x8(1);
                InitAndLaunchSpecialAnimation(
                    gActiveBattler,
                    gActiveBattler,
                    gActiveBattler,
                    B_ANIM_SUBSTITUTE_TO_MON,
                );
            }
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 1;
        }
        1 => {
            if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive()
                == 0
            {
                SetBattlerSpriteAffineMode(ST_OAM_AFFINE_OFF as u8);
                DoMoveAnim(r#move);
                (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 2;
            }
        }
        2 => {
            gAnimScriptCallback.unwrap_unchecked()();
            if gAnimScriptActive == 0 {
                SetBattlerSpriteAffineMode(ST_OAM_AFFINE_NORMAL as u8);
                if (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).behindSubstitute()
                    != 0
                    && multihit < 2
                {
                    InitAndLaunchSpecialAnimation(
                        gActiveBattler,
                        gActiveBattler,
                        gActiveBattler,
                        B_ANIM_MON_TO_SUBSTITUTE,
                    );
                    (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).set_flag_x8(0);
                }
                (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 3;
            }
        }
        3 => {
            if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive()
                == 0
            {
                CopyAllBattleSpritesInvisibilities();
                TrySetBehindSubstituteSpriteBit(
                    gActiveBattler,
                    gBattleBufferA[gActiveBattler][1] as u16
                        | (gBattleBufferA[gActiveBattler][2] as u16) << 8,
                );
                (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
                PlayerBufferExecCompleted();
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn PlayerHandlePrintString() {
    let mut stringId: *mut u16 = null_mut();
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    stringId = &raw mut gBattleBufferA[gActiveBattler][2] as *mut u16;
    BufferStringBattle(*stringId);
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnInactiveTextPrinter2);
    BattleTv_SetDataBasedOnString(*stringId);
    BattleArena_DeductSkillPoints(gActiveBattler, *stringId);
}
pub(crate) unsafe extern "C" fn PlayerHandlePrintSelectionString() {
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        PlayerHandlePrintString();
    } else {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn HandleChooseActionAfterDma3() {
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        gBattle_BG0_X = 0;
        gBattle_BG0_Y = DISPLAY_HEIGHT;
        gBattlerControllerFuncs[gActiveBattler] = Some(HandleInputChooseAction);
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleChooseAction() {
    let mut i: i32 = 0;
    gBattlerControllerFuncs[gActiveBattler] = Some(HandleChooseActionAfterDma3);
    BattleTv_ClearExplosionFaintCause();
    BattlePutTextOnWindow(gText_BattleMenu.as_ptr().cast_mut(), B_WIN_ACTION_MENU);
    i = 0;
    while i < 4 {
        ActionSelectionDestroyCursorAt(i as u8);
        i += 1;
    }
    ActionSelectionCreateCursorAt(gActionSelectionCursor[gActiveBattler], 0);
    BattleStringExpandPlaceholdersToDisplayedString(gText_WhatWillPkmnDo.as_ptr().cast_mut());
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_ACTION_PROMPT);
}
pub(crate) unsafe extern "C" fn PlayerHandleYesNoBox() {
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        HandleBattleWindow(24, 8, 29, 13, 0);
        BattlePutTextOnWindow(gText_BattleYesNoChoice.as_ptr().cast_mut(), B_WIN_YESNO);
        gMultiUsePlayerCursor = 1;
        BattleCreateYesNoCursorAt(1);
        gBattlerControllerFuncs[gActiveBattler] = Some(PlayerHandleYesNoInput);
    } else {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn HandleChooseMoveAfterDma3() {
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        gBattle_BG0_X = 0;
        gBattle_BG0_Y = 320;
        gBattlerControllerFuncs[gActiveBattler] = Some(HandleInputChooseMove);
    }
}
pub(crate) unsafe extern "C" fn PlayerChooseMoveInBattlePalace() {
    if ({
        *(*gBattleStruct)
            .arenaMindPoints
            .as_mut_ptr()
            .at(gActiveBattler) -= 1;
        *(*gBattleStruct)
            .arenaMindPoints
            .as_mut_ptr()
            .at(gActiveBattler)
    }) == 0
    {
        gBattlePalaceMoveSelectionRngValue = gRngValue;
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            B_ACTION_EXEC_SCRIPT,
            ChooseMoveAndTargetInBattlePalace(),
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleChooseMove() {
    if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0 {
        *(*gBattleStruct)
            .arenaMindPoints
            .as_mut_ptr()
            .at(gActiveBattler) = 8;
        gBattlerControllerFuncs[gActiveBattler] = Some(PlayerChooseMoveInBattlePalace);
    } else {
        InitMoveSelectionsVarsAndStrings();
        gBattlerControllerFuncs[gActiveBattler] = Some(HandleChooseMoveAfterDma3);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMoveSelectionsVarsAndStrings() {
    MoveSelectionDisplayMoveNames();
    gMultiUsePlayerCursor = 0xFF;
    MoveSelectionCreateCursorAt(gMoveSelectionCursor[gActiveBattler], 0);
    MoveSelectionDisplayPPString();
    MoveSelectionDisplayPPNumber();
    MoveSelectionDisplayMoveType();
}
pub(crate) unsafe extern "C" fn PlayerHandleChooseItem() {
    let mut i: i32 = 0;
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
    gBattlerControllerFuncs[gActiveBattler] = Some(OpenBagAndChooseItem);
    gBattlerInMenuId = gActiveBattler;
    i = 0;
    while i < 3 {
        gBattlePartyCurrentOrder[i] = gBattleBufferA[gActiveBattler][1 + i];
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleChoosePokemon() {
    let mut i: i32 = 0;
    i = 0;
    while i < 3 {
        gBattlePartyCurrentOrder[i] = gBattleBufferA[gActiveBattler][4 + i];
        i += 1;
    }
    if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0
        && gBattleBufferA[gActiveBattler][1] as i32 & 0xF != PARTY_ACTION_CANT_SWITCH as i32
    {
        BtlController_EmitChosenMonReturnValue(
            B_COMM_TO_ENGINE,
            gBattlerPartyIndexes[gActiveBattler] as u8 + 1,
            gBattlePartyCurrentOrder.as_mut_ptr(),
        );
        PlayerBufferExecCompleted();
    } else {
        gBattleControllerData[gActiveBattler] = CreateTask(Some(TaskDummy), 0xFF);
        gTasks[gBattleControllerData[gActiveBattler]].data[0] =
            gBattleBufferA[gActiveBattler][1] as i16 & 0xF;
        *(&raw mut (*gBattleStruct).battlerPreventingSwitchout) =
            gBattleBufferA[gActiveBattler][1] >> 4;
        *(&raw mut (*gBattleStruct).prevSelectedPartySlot) = gBattleBufferA[gActiveBattler][2];
        *(&raw mut (*gBattleStruct).abilityPreventingSwitchout) = gBattleBufferA[gActiveBattler][3];
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
        gBattlerControllerFuncs[gActiveBattler] = Some(OpenPartyMenuToChooseMon);
        gBattlerInMenuId = gActiveBattler;
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleCmd23() {
    BattleStopLowHpSound();
    BeginNormalPaletteFade(PALETTES_ALL, 2, 0, 16, 0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleHealthBarUpdate() {
    let mut hpVal: i16 = 0;
    LoadBattleBarGfx(0);
    hpVal =
        gBattleBufferA[gActiveBattler][2] as i16 | (gBattleBufferA[gActiveBattler][3] as i16) << 8;
    if hpVal > 0 {
        gPlayerPartyLostHP += hpVal as u32;
    }
    if hpVal != INSTANT_HP_BAR_DROP {
        let mut maxHP: u32 = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_MAX_HP,
        );
        let mut curHP: u32 = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_HP,
        );
        SetBattleBarStruct(
            gActiveBattler,
            gHealthboxSpriteIds[gActiveBattler],
            maxHP as i32,
            curHP as i32,
            hpVal as i32,
        );
    } else {
        let mut maxHP: u32 = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_MAX_HP,
        );
        SetBattleBarStruct(
            gActiveBattler,
            gHealthboxSpriteIds[gActiveBattler],
            maxHP as i32,
            0,
            hpVal as i32,
        );
        UpdateHpTextInHealthbox(gHealthboxSpriteIds[gActiveBattler], 0, HP_CURRENT);
    }
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnHealthbarDone);
}
pub(crate) unsafe extern "C" fn PlayerHandleExpUpdate() {
    let mut monId: u8 = gBattleBufferA[gActiveBattler][1];
    if GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) >= MAX_LEVEL {
        PlayerBufferExecCompleted();
    } else {
        let mut expPointsToGive: i16 = 0;
        let mut taskId: u8 = 0;
        LoadBattleBarGfx(1);
        GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPECIES);
        expPointsToGive = *(&raw mut gBattleBufferA[gActiveBattler][2]) as i16
            | (*(&raw mut gBattleBufferA[gActiveBattler][2]).at(1) as i16) << 8;
        taskId = CreateTask(Some(Task_GiveExpToMon), 10);
        gTasks[taskId].data[0] = monId as i16;
        gTasks[taskId].data[1] = expPointsToGive;
        gTasks[taskId].data[2] = gActiveBattler as i16;
        gBattlerControllerFuncs[gActiveBattler] = Some(BattleControllerDummy);
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleStatusIconUpdate() {
    if IsBattleSEPlaying(gActiveBattler) == 0 {
        let mut battler: u8 = 0;
        UpdateHealthboxAttribute(
            gHealthboxSpriteIds[gActiveBattler],
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            HEALTHBOX_STATUS_ICON,
        );
        battler = gActiveBattler;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_statusAnimActive(0);
        gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedStatusAnimation);
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleStatusAnimation() {
    if IsBattleSEPlaying(gActiveBattler) == 0 {
        InitAndLaunchChosenStatusAnimation(
            gBattleBufferA[gActiveBattler][1],
            gBattleBufferA[gActiveBattler][2] as u32
                | (gBattleBufferA[gActiveBattler][3] as u32) << 8
                | (gBattleBufferA[gActiveBattler][4] as u32) << 16
                | (gBattleBufferA[gActiveBattler][5] as u32) << 24,
        );
        gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedStatusAnimation);
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleStatusXor() {
    let mut val: u8 = GetMonData2(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        MON_DATA_STATUS,
    ) as u8
        ^ gBattleBufferA[gActiveBattler][1];
    SetMonData(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        MON_DATA_STATUS,
        &raw mut val as *mut c_void,
    );
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleDataTransfer() {
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleDMA3Transfer() {
    let mut dstArg: u32 = gBattleBufferA[gActiveBattler][1] as u32
        | (gBattleBufferA[gActiveBattler][2] as u32) << 8
        | (gBattleBufferA[gActiveBattler][3] as u32) << 16
        | (gBattleBufferA[gActiveBattler][4] as u32) << 24;
    let mut sizeArg: u16 =
        gBattleBufferA[gActiveBattler][5] as u16 | (gBattleBufferA[gActiveBattler][6] as u16) << 8;
    {
        let mut _src: *mut c_void = &raw mut gBattleBufferA[gActiveBattler][7] as *mut c_void;
        let mut _dest: *mut c_void = dstArg as usize as *mut c_void;
        let mut _size: u32 = sizeArg as u32;
        loop {
            if _size <= 0x1000 {
                {
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, _src as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
                break;
            }
            {
                {
                    {
                        let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                        volatile_write(dmaRegs, _src as usize as u32);
                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                        volatile_write(dmaRegs.at(2), 0x80000800);
                        let _ = (dmaRegs.at(2)).read_volatile();
                    }
                }
            }
            _src = (_src as *mut u8).at(4096) as *mut c_void;
            _dest = (_dest as *mut u8).at(4096) as *mut c_void;
            _size -= 0x1000;
        }
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandlePlayBGM() {
    PlayBGM(
        gBattleBufferA[gActiveBattler][1] as u16 | (gBattleBufferA[gActiveBattler][2] as u16) << 8,
    );
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleCmd32() {
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleTwoReturnValues() {
    BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, 0, 0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleChosenMonReturnValue() {
    BtlController_EmitChosenMonReturnValue(B_COMM_TO_ENGINE, 0, null_mut());
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleOneReturnValue() {
    BtlController_EmitOneReturnValue(B_COMM_TO_ENGINE, 0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleOneReturnValue_Duplicate() {
    BtlController_EmitOneReturnValue_Duplicate(B_COMM_TO_ENGINE, 0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleClearUnkVar() {
    gUnusedControllerStruct.set_unk(0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleSetUnkVar() {
    gUnusedControllerStruct.set_unk(gBattleBufferA[gActiveBattler][1]);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleClearUnkFlag() {
    gUnusedControllerStruct.set_flag(0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleToggleUnkFlag() {
    gUnusedControllerStruct.set_flag(gUnusedControllerStruct.flag() ^ 1);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleHitAnimation() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].invisible() == TRUE as u16 {
        PlayerBufferExecCompleted();
    } else {
        gDoingBattleAnim = TRUE;
        gSprites[gBattlerSpriteIds[gActiveBattler]].data[1] = 0;
        DoHitAnimHealthboxEffect(gActiveBattler);
        gBattlerControllerFuncs[gActiveBattler] = Some(DoHitAnimBlinkSpriteEffect);
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleCantSwitch() {
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandlePlaySE() {
    let mut pan: i8 = 0;
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        pan = SOUND_PAN_ATTACKER;
    } else {
        pan = SOUND_PAN_TARGET;
    }
    PlaySE12WithPanning(
        gBattleBufferA[gActiveBattler][1] as u16 | (gBattleBufferA[gActiveBattler][2] as u16) << 8,
        pan,
    );
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandlePlayFanfareOrBGM() {
    if gBattleBufferA[gActiveBattler][3] != 0 {
        BattleStopLowHpSound();
        PlayBGM(
            gBattleBufferA[gActiveBattler][1] as u16
                | (gBattleBufferA[gActiveBattler][2] as u16) << 8,
        );
    } else {
        PlayFanfare(
            gBattleBufferA[gActiveBattler][1] as u16
                | (gBattleBufferA[gActiveBattler][2] as u16) << 8,
        );
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleFaintingCry() {
    let mut species: u16 = GetMonData2(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        MON_DATA_SPECIES,
    ) as u16;
    PlayCry_ByMode(species, -25, CRY_MODE_FAINT);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleIntroSlide() {
    HandleIntroSlide(gBattleBufferA[gActiveBattler][1]);
    gIntroSlideFlags |= 1;
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleIntroTrainerBallThrow() {
    let mut paletteNum: u8 = 0;
    let mut taskId: u8 = 0;
    SetSpritePrimaryCoordsFromSecondaryCoords(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = 50;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[2] = -40;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[4] =
        gSprites[gBattlerSpriteIds[gActiveBattler]].y;
    gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(StartAnimLinearTranslation);
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[5] = gActiveBattler as i16;
    StoreSpriteCallbackInData6(
        &raw mut gSprites[gBattlerSpriteIds[gActiveBattler]],
        Some(SpriteCB_FreePlayerSpriteLoadMonSprite),
    );
    StartSpriteAnim(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]], 1);
    paletteNum = AllocSpritePalette(0xD6F8);
    LoadCompressedPalette(
        gTrainerBackPicPaletteTable[(*gSaveBlock2Ptr).playerGender].data,
        0x100 + paletteNum as u16 * 16,
        32,
    );
    gSprites[gBattlerSpriteIds[gActiveBattler]]
        .oam
        .set_paletteNum(paletteNum as u16);
    taskId = CreateTask(Some(Task_StartSendOutAnim), 5);
    gTasks[taskId].data[0] = gActiveBattler as i16;
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusSummaryShown() != 0
    {
        gTasks[gBattlerStatusSummaryTaskId[gActiveBattler]].func =
            Some(Task_HidePartyStatusSummary);
    }
    (*(*gBattleSpritesDataPtr).animationData).set_introAnimActive(TRUE);
    gBattlerControllerFuncs[gActiveBattler] = Some(BattleControllerDummy);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_FreePlayerSpriteLoadMonSprite(sprite: *mut Sprite) {
    let mut battler: u8 = (*sprite).data[5] as u8;
    FreeSpriteOamMatrix(sprite);
    FreeSpritePaletteByTag(GetSpritePaletteTagByPaletteNum(
        (*sprite).oam.paletteNum() as u8
    ));
    DestroySprite(sprite);
    BattleLoadPlayerMonSpriteGfx(
        &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
        battler,
    );
    StartSpriteAnim(&raw mut gSprites[gBattlerSpriteIds[battler]], 0);
}
pub(crate) unsafe extern "C" fn Task_StartSendOutAnim(taskId: u8) {
    if gTasks[taskId].data[1] < 31 {
        gTasks[taskId].data[1] += 1;
    } else {
        let mut savedActiveBattler: u8 = gActiveBattler;
        gActiveBattler = gTasks[taskId].data[0] as u8;
        if IsDoubleBattle() == 0 || gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
            gBattleBufferA[gActiveBattler][1] = gBattlerPartyIndexes[gActiveBattler] as u8;
            StartSendOutAnim(gActiveBattler, FALSE);
        } else {
            gBattleBufferA[gActiveBattler][1] = gBattlerPartyIndexes[gActiveBattler] as u8;
            StartSendOutAnim(gActiveBattler, FALSE);
            gActiveBattler ^= BIT_FLANK;
            gBattleBufferA[gActiveBattler][1] = gBattlerPartyIndexes[gActiveBattler] as u8;
            BattleLoadPlayerMonSpriteGfx(
                &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                gActiveBattler,
            );
            StartSendOutAnim(gActiveBattler, FALSE);
            gActiveBattler ^= BIT_FLANK;
        }
        gBattlerControllerFuncs[gActiveBattler] = Some(Intro_TryShinyAnimShowHealthbox);
        gActiveBattler = savedActiveBattler;
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleDrawPartyStatusSummary() {
    if gBattleBufferA[gActiveBattler][1] != 0 && GetBattlerSide(gActiveBattler) == 0 {
        PlayerBufferExecCompleted();
    } else {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_partyStatusSummaryShown(1);
        gBattlerStatusSummaryTaskId[gActiveBattler] = CreatePartyStatusSummarySprites(
            gActiveBattler,
            &raw mut gBattleBufferA[gActiveBattler][4] as *mut HpAndStatus,
            gBattleBufferA[gActiveBattler][1],
            gBattleBufferA[gActiveBattler][2],
        );
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer = 0;
        if gBattleBufferA[gActiveBattler][2] != 0 {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer =
                93;
        }
        gBattlerControllerFuncs[gActiveBattler] = Some(EndDrawPartyStatusSummary);
    }
}
pub(crate) unsafe extern "C" fn EndDrawPartyStatusSummary() {
    if ({
        let t1 =
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer += 1;
        t1
    }) > 92
    {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer = 0;
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleHidePartyStatusSummary() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusSummaryShown() != 0
    {
        gTasks[gBattlerStatusSummaryTaskId[gActiveBattler]].func =
            Some(Task_HidePartyStatusSummary);
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleEndBounceEffect() {
    EndBounceEffect(gActiveBattler, BOUNCE_HEALTHBOX);
    EndBounceEffect(gActiveBattler, BOUNCE_MON);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleSpriteInvisibility() {
    if IsBattlerSpritePresent(gActiveBattler) != 0 {
        gSprites[gBattlerSpriteIds[gActiveBattler]]
            .set_invisible(gBattleBufferA[gActiveBattler][1] as u16);
        CopyBattleSpriteInvisibility(gActiveBattler);
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleBattleAnimation() {
    if IsBattleSEPlaying(gActiveBattler) == 0 {
        let mut animationId: u8 = gBattleBufferA[gActiveBattler][1];
        let mut argument: u16 = gBattleBufferA[gActiveBattler][2] as u16
            | (gBattleBufferA[gActiveBattler][3] as u16) << 8;
        if TryHandleLaunchBattleTableAnimation(
            gActiveBattler,
            gActiveBattler,
            gActiveBattler,
            animationId,
            argument,
        ) != 0
        {
            PlayerBufferExecCompleted();
        } else {
            gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedBattleAnimation);
        }
        BattleTv_SetDataBasedOnAnimation(animationId);
    }
}
pub(crate) unsafe extern "C" fn PlayerHandleLinkStandbyMsg() {
    RecordedBattle_RecordAllBattlerData(&raw mut gBattleBufferA[gActiveBattler][2]);
    'l1: {
        let sw1: u8 = gBattleBufferA[gActiveBattler][1];
        let mut fall = false;
        if sw1 == LINK_STANDBY_MSG_STOP_BOUNCE {
            fall = true;
            PrintLinkStandbyMsg();
        }
        if fall || sw1 == LINK_STANDBY_STOP_BOUNCE_ONLY {
            fall = true;
            EndBounceEffect(gActiveBattler, BOUNCE_HEALTHBOX);
            EndBounceEffect(gActiveBattler, BOUNCE_MON);
            break 'l1;
        }
        if sw1 == LINK_STANDBY_MSG_ONLY {
            fall = true;
            PrintLinkStandbyMsg();
            break 'l1;
        }
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleResetActionMoveSelection() {
    match gBattleBufferA[gActiveBattler][1] {
        RESET_ACTION_MOVE_SELECTION => {
            gActionSelectionCursor[gActiveBattler] = 0;
            gMoveSelectionCursor[gActiveBattler] = 0;
        }
        RESET_ACTION_SELECTION => {
            gActionSelectionCursor[gActiveBattler] = 0;
        }
        RESET_MOVE_SELECTION => {
            gMoveSelectionCursor[gActiveBattler] = 0;
        }
        _ => {}
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn PlayerHandleEndLinkBattle() {
    RecordedBattle_RecordAllBattlerData(&raw mut gBattleBufferA[gActiveBattler][4]);
    gBattleOutcome = gBattleBufferA[gActiveBattler][1];
    (*gSaveBlock2Ptr)
        .frontier
        .set_disableRecordBattle(gBattleBufferA[gActiveBattler][2]);
    FadeOutMapMusic(5);
    BeginFastPaletteFade(3);
    PlayerBufferExecCompleted();
    gBattlerControllerFuncs[gActiveBattler] = Some(SetBattleEndCallbacks);
}
pub(crate) unsafe extern "C" fn PlayerCmdEnd() {}
