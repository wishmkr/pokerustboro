//! Translated from `src/battle_controller_safari.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sSafariBufferCommands

static sSafariBufferCommands: Table<CArray<Option<unsafe extern "C" fn()>, 57>> =
    Table((&raw const crate::data::battle_controller_safari::sSafariBufferCommands).cast());

unsafe extern "C" {
    static mut gActionSelectionCursor: CArray<u8, 4>;
    static mut gActiveBattler: u8;
    static mut gBattleBufferA: CArray<CArray<u8, 512>, 4>;
    static mut gBattleControllerExecFlags: u32;
    static mut gBattleOutcome: u8;
    static mut gBattleSpritesDataPtr: *mut BattleSpriteData;
    static mut gBattleTypeFlags: u32;
    static mut gBattle_BG0_X: u16;
    static mut gBattle_BG0_Y: u16;
    static mut gBattlerControllerFuncs: CArray<Option<unsafe extern "C" fn()>, 4>;
    static mut gBattlerInMenuId: u8;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gBattlerSpriteIds: CArray<u8, 4>;
    static gBitTable: CArray<u32, 0>;
    static mut gDisplayedStringBattle: CArray<u8, 300>;
    static mut gDoingBattleAnim: u8;
    static mut gHealthboxSpriteIds: CArray<u8, 4>;
    static mut gIntroSlideFlags: u16;
    static mut gMain: Main;
    static mut gMultiuseSpriteTemplate: SpriteTemplate;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPreBattleCallback1: Option<unsafe extern "C" fn()>;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_ItemId: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static gText_SafariZoneMenu: CArray<u8, 0>;
    static gText_WhatWillPkmnDo2: CArray<u8, 0>;
    static gTrainerBackPicCoords: CArray<MonCoords, 0>;
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
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DecompressTrainerBackPic(a0: u16, a1: u8);
    fn FadeOutMapMusic(a0: u8);
    fn FreeAllWindowBuffers();
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
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
    fn SpriteCB_TrainerSlideIn(a0: *mut Sprite);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartHealthboxSlideIn(a0: u8);
    fn TryHandleLaunchBattleTableAnimation(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn UpdateHealthboxAttribute(a0: u8, a1: *mut Pokemon, a2: u8);
}

pub(crate) unsafe extern "C" fn SpriteCB_Null4() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetControllerToSafari() {
    gBattlerControllerFuncs[gActiveBattler] = Some(SafariBufferRunCommand);
}
pub(crate) unsafe extern "C" fn SafariBufferRunCommand() {
    if gBattleControllerExecFlags & gBitTable[gActiveBattler] != 0 {
        if gBattleBufferA[gActiveBattler][0] < 57 {
            sSafariBufferCommands[gBattleBufferA[gActiveBattler][0]].unwrap_unchecked()();
        } else {
            SafariBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn HandleInputChooseAction() {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        match gActionSelectionCursor[gActiveBattler] {
            0 => {
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SAFARI_BALL, 0);
            }
            1 => {
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SAFARI_POKEBLOCK, 0);
            }
            2 => {
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SAFARI_GO_NEAR, 0);
            }
            3 => {
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_SAFARI_RUN, 0);
            }
            _ => {}
        }
        SafariBufferExecCompleted();
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
    }
}
pub(crate) unsafe extern "C" fn CompleteOnBattlerSpriteCallbackDummy() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnInactiveTextPrinter() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnHealthboxSpriteCallbackDummy() {
    if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariSetBattleEndCallbacks() {
    if gPaletteFade.active() == 0 {
        gMain.set_inBattle(FALSE);
        gMain.callback1 = gPreBattleCallback1;
        SetMainCallback2(gMain.savedCallback);
    }
}
pub(crate) unsafe extern "C" fn CompleteOnSpecialAnimDone() {
    if gDoingBattleAnim == 0
        || (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0
    {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariOpenPokeblockCase() {
    if gPaletteFade.active() == 0 {
        gBattlerControllerFuncs[gActiveBattler] = Some(CompleteWhenChosePokeblock);
        FreeAllWindowBuffers();
        OpenPokeblockCaseInBattle();
    }
}
pub(crate) unsafe extern "C" fn CompleteWhenChosePokeblock() {
    if gMain.callback2 == Some(BattleMainCB2 as unsafe extern "C" fn())
        && gPaletteFade.active() == 0
    {
        BtlController_EmitOneReturnValue(B_COMM_TO_ENGINE, gSpecialVar_ItemId);
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnFinishedBattleAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animFromTableActive() == 0 {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariBufferExecCompleted() {
    gBattlerControllerFuncs[gActiveBattler] = Some(SafariBufferRunCommand);
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        let mut playerId: u8 = GetMultiplayerId();
        PrepareBufferDataTransferLink(B_COMM_CONTROLLER_IS_DONE, 4, &raw mut playerId);
        gBattleBufferA[gActiveBattler][0] = CONTROLLER_TERMINATOR_NOP;
    } else {
        gBattleControllerExecFlags &= !gBitTable[gActiveBattler];
    }
}
pub(crate) unsafe extern "C" fn CompleteOnFinishedStatusAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).statusAnimActive() == 0 {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn SafariHandleGetMonData() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleGetRawMonData() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleSetMonData() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleSetRawMonData() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleLoadMonSprite() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleSwitchInAnim() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleReturnMonToBall() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleDrawTrainerPic() {
    DecompressTrainerBackPic((*gSaveBlock2Ptr).playerGender as u16, gActiveBattler);
    SetMultiuseSpriteTemplateToTrainerBack(
        (*gSaveBlock2Ptr).playerGender as u16,
        GetBattlerPosition(gActiveBattler),
    );
    gBattlerSpriteIds[gActiveBattler] = CreateSprite(
        &raw mut gMultiuseSpriteTemplate,
        80,
        (8 - gTrainerBackPicCoords[(*gSaveBlock2Ptr).playerGender].size as i16) * 4 + 80,
        30,
    );
    gSprites[gBattlerSpriteIds[gActiveBattler]]
        .oam
        .set_paletteNum(gActiveBattler as u16);
    gSprites[gBattlerSpriteIds[gActiveBattler]].x2 = DISPLAY_WIDTH as i16;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = -2;
    gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(SpriteCB_TrainerSlideIn);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnBattlerSpriteCallbackDummy);
}
pub(crate) unsafe extern "C" fn SafariHandleTrainerSlide() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleTrainerSlideBack() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleFaintAnimation() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandlePaletteFade() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleSuccessBallThrowAnim() {
    (*(*gBattleSpritesDataPtr).animationData).ballThrowCaseId = BALL_3_SHAKES_SUCCESS;
    gDoingBattleAnim = TRUE;
    InitAndLaunchSpecialAnimation(
        gActiveBattler,
        gActiveBattler,
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        B_ANIM_BALL_THROW_WITH_TRAINER,
    );
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnSpecialAnimDone);
}
pub(crate) unsafe extern "C" fn SafariHandleBallThrowAnim() {
    let mut ballThrowCaseId: u8 = gBattleBufferA[gActiveBattler][1];
    (*(*gBattleSpritesDataPtr).animationData).ballThrowCaseId = ballThrowCaseId;
    gDoingBattleAnim = TRUE;
    InitAndLaunchSpecialAnimation(
        gActiveBattler,
        gActiveBattler,
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        B_ANIM_BALL_THROW_WITH_TRAINER,
    );
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnSpecialAnimDone);
}
pub(crate) unsafe extern "C" fn SafariHandlePause() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleMoveAnimation() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandlePrintString() {
    let mut stringId: *mut u16 = null_mut();
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    stringId = &raw mut gBattleBufferA[gActiveBattler][2] as *mut u16;
    BufferStringBattle(*stringId);
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnInactiveTextPrinter);
}
pub(crate) unsafe extern "C" fn SafariHandlePrintSelectionString() {
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        SafariHandlePrintString();
    } else {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn HandleChooseActionAfterDma3() {
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        gBattle_BG0_X = 0;
        gBattle_BG0_Y = DISPLAY_HEIGHT;
        gBattlerControllerFuncs[gActiveBattler] = Some(HandleInputChooseAction);
    }
}
pub(crate) unsafe extern "C" fn SafariHandleChooseAction() {
    let mut i: i32 = 0;
    gBattlerControllerFuncs[gActiveBattler] = Some(HandleChooseActionAfterDma3);
    BattlePutTextOnWindow(gText_SafariZoneMenu.as_ptr().cast_mut(), B_WIN_ACTION_MENU);
    i = 0;
    while i < 4 {
        ActionSelectionDestroyCursorAt(i as u8);
        i += 1;
    }
    ActionSelectionCreateCursorAt(gActionSelectionCursor[gActiveBattler], 0);
    BattleStringExpandPlaceholdersToDisplayedString(gText_WhatWillPkmnDo2.as_ptr().cast_mut());
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_ACTION_PROMPT);
}
pub(crate) unsafe extern "C" fn SafariHandleYesNoBox() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleChooseMove() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleChooseItem() {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
    gBattlerControllerFuncs[gActiveBattler] = Some(SafariOpenPokeblockCase);
    gBattlerInMenuId = gActiveBattler;
}
pub(crate) unsafe extern "C" fn SafariHandleChoosePokemon() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleCmd23() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleHealthBarUpdate() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleExpUpdate() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleStatusIconUpdate() {
    UpdateHealthboxAttribute(
        gHealthboxSpriteIds[gActiveBattler],
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        HEALTHBOX_SAFARI_BALLS_TEXT,
    );
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleStatusAnimation() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleStatusXor() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleDataTransfer() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleDMA3Transfer() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandlePlayBGM() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleCmd32() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleTwoReturnValues() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleChosenMonReturnValue() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleOneReturnValue() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleOneReturnValue_Duplicate() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleClearUnkVar() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleSetUnkVar() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleClearUnkFlag() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleToggleUnkFlag() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleHitAnimation() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleCantSwitch() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandlePlaySE() {
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
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandlePlayFanfareOrBGM() {
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
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleFaintingCry() {
    let mut species: u16 = GetMonData2(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        MON_DATA_SPECIES,
    ) as u16;
    PlayCry_Normal(species, 25);
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleIntroSlide() {
    HandleIntroSlide(gBattleBufferA[gActiveBattler][1]);
    gIntroSlideFlags |= 1;
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleIntroTrainerBallThrow() {
    UpdateHealthboxAttribute(
        gHealthboxSpriteIds[gActiveBattler],
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        HEALTHBOX_SAFARI_ALL_TEXT,
    );
    StartHealthboxSlideIn(gActiveBattler);
    SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler]);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnHealthboxSpriteCallbackDummy);
}
pub(crate) unsafe extern "C" fn SafariHandleDrawPartyStatusSummary() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleHidePartyStatusSummary() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleEndBounceEffect() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleSpriteInvisibility() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleBattleAnimation() {
    let mut animationId: u8 = gBattleBufferA[gActiveBattler][1];
    let mut argument: u16 =
        gBattleBufferA[gActiveBattler][2] as u16 | (gBattleBufferA[gActiveBattler][3] as u16) << 8;
    if TryHandleLaunchBattleTableAnimation(
        gActiveBattler,
        gActiveBattler,
        gActiveBattler,
        animationId,
        argument,
    ) != 0
    {
        SafariBufferExecCompleted();
    } else {
        gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedBattleAnimation);
    }
}
pub(crate) unsafe extern "C" fn SafariHandleLinkStandbyMsg() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleResetActionMoveSelection() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SafariHandleEndLinkBattle() {
    gBattleOutcome = gBattleBufferA[gActiveBattler][1];
    FadeOutMapMusic(5);
    BeginFastPaletteFade(3);
    SafariBufferExecCompleted();
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 && gBattleTypeFlags & BATTLE_TYPE_IS_MASTER == 0 {
        gBattlerControllerFuncs[gActiveBattler] = Some(SafariSetBattleEndCallbacks);
    }
}
pub(crate) unsafe extern "C" fn SafariCmdEnd() {}
