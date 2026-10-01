//! Translated from `src/battle_controller_safari.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    clippy::type_complexity,
    clippy::useless_transmute,
    dead_code,
    unused_assignments
)]

use crate::agb_main::gMain;
use crate::battle_anim_mons::{GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide};
use crate::battle_controller_player::{
    ActionSelectionCreateCursorAt, ActionSelectionDestroyCursorAt,
};
use crate::battle_controllers::{
    BtlController_EmitOneReturnValue, BtlController_EmitTwoReturnValues,
    PrepareBufferDataTransferLink,
};
use crate::battle_gfx_sfx_util::{
    BattleStopLowHpSound, DecompressTrainerBackPic, InitAndLaunchSpecialAnimation,
    SpriteCB_TrainerSlideIn, TryHandleLaunchBattleTableAnimation,
};
use crate::battle_interface::{SetHealthboxSpriteVisible, UpdateHealthboxAttribute};
use crate::battle_intro::HandleIntroSlide;
use crate::battle_main::{
    BattleMainCB2, gActiveBattler, gBattle_BG0_X, gBattle_BG0_Y, gBattleControllerExecFlags,
    gBattleOutcome, gBattleSpritesDataPtr, gBattleTypeFlags, gBattlerControllerFuncs,
    gBattlerInMenuId, gDoingBattleAnim, gIntroSlideFlags, gPreBattleCallback1,
};
use crate::battle_main::{
    gActionSelectionCursor, gBattleBufferA, gBattlerPartyIndexes, gBattlerSpriteIds,
    gDisplayedStringBattle, gHealthboxSpriteIds,
};
use crate::battle_message::{
    BattlePutTextOnWindow, BattleStringExpandPlaceholdersToDisplayedString, BufferStringBattle,
};
use crate::bg::IsDma3ManagerBusyWithBgCopy;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::item_menu::gSpecialVar_ItemId;
use crate::link::GetMultiplayerId;
use crate::load_save::gSaveBlock2Ptr;
use crate::palette::{BeginFastPaletteFade, BeginNormalPaletteFade, gPaletteFade};
use crate::pokeball::StartHealthboxSlideIn;
use crate::pokeblock::OpenPokeblockCaseInBattle;
use crate::pokemon::{
    GetMonData2, SetMultiuseSpriteTemplateToTrainerBack, gMultiuseSpriteTemplate, gPlayerParty,
};
use crate::sound::{
    FadeOutMapMusic, PlayBGM, PlayCry_Normal, PlayFanfare, PlaySE, PlaySE12WithPanning,
};
use crate::sprite::gSprites;
use crate::text::IsTextPrinterActive;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
use crate::window::FreeAllWindowBuffers;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
// The C's names for task and sprite data slots.
const sSpeedX: usize = 0;
// Data tables (translate with cdata.py): sSafariBufferCommands

static sSafariBufferCommands: Table<CArray<Option<unsafe fn()>, 57>> =
    Table((&raw const crate::data::battle_controller_safari::sSafariBufferCommands).cast());

/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

fn SpriteCB_Null4() {}
pub unsafe fn SetControllerToSafari() {
    gBattlerControllerFuncs[gActiveBattler] = Some(SafariBufferRunCommand);
}
pub(crate) unsafe fn SafariBufferRunCommand() {
    if gBattleControllerExecFlags
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler]
        != 0
    {
        if gBattleBufferA[gActiveBattler][0] < 57 {
            sSafariBufferCommands[gBattleBufferA[gActiveBattler][0]].unwrap_unchecked()();
        } else {
            SafariBufferExecCompleted();
        }
    }
}
pub(crate) unsafe fn HandleInputChooseAction() {
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
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0
        && (*(&raw const crate::battle_main::gActionSelectionCursor)
            .cast::<CArray<u8, 4>>()
            .cast_mut())[gActiveBattler] as i32
            & 2
            == 0
    {
        PlaySE(SE_SELECT);
        ActionSelectionDestroyCursorAt(gActionSelectionCursor[gActiveBattler]);
        gActionSelectionCursor[gActiveBattler] ^= 2;
        ActionSelectionCreateCursorAt(gActionSelectionCursor[gActiveBattler], 0);
    }
}
pub(crate) unsafe fn CompleteOnBattlerSpriteCallbackDummy() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnInactiveTextPrinter() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnHealthboxSpriteCallbackDummy() {
    if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe fn SafariSetBattleEndCallbacks() {
    if gPaletteFade.active() == 0 {
        gMain.set_inBattle(FALSE);
        gMain.callback1 = gPreBattleCallback1;
        SetMainCallback2(gMain.savedCallback);
    }
}
pub(crate) unsafe fn CompleteOnSpecialAnimDone() {
    if gDoingBattleAnim == 0
        || (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0
    {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe fn SafariOpenPokeblockCase() {
    if gPaletteFade.active() == 0 {
        gBattlerControllerFuncs[gActiveBattler] = Some(CompleteWhenChosePokeblock);
        FreeAllWindowBuffers();
        OpenPokeblockCaseInBattle();
    }
}
pub(crate) unsafe fn CompleteWhenChosePokeblock() {
    if gMain.callback2 == Some(BattleMainCB2 as unsafe fn()) && gPaletteFade.active() == 0 {
        BtlController_EmitOneReturnValue(B_COMM_TO_ENGINE, gSpecialVar_ItemId);
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnFinishedBattleAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animFromTableActive() == 0 {
        SafariBufferExecCompleted();
    }
}
unsafe fn SafariBufferExecCompleted() {
    gBattlerControllerFuncs[gActiveBattler] = Some(SafariBufferRunCommand);
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        let mut playerId: u8 = GetMultiplayerId();
        PrepareBufferDataTransferLink(B_COMM_CONTROLLER_IS_DONE, 4, &raw mut playerId);
        gBattleBufferA[gActiveBattler][0] = CONTROLLER_TERMINATOR_NOP;
    } else {
        gBattleControllerExecFlags &= !gBitTable[gActiveBattler];
    }
}
pub(crate) unsafe fn CompleteOnFinishedStatusAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).statusAnimActive() == 0 {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe fn SafariHandleGetMonData() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleGetRawMonData() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleSetMonData() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleSetRawMonData() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleLoadMonSprite() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleSwitchInAnim() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleReturnMonToBall() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleDrawTrainerPic() {
    DecompressTrainerBackPic((*gSaveBlock2Ptr).playerGender as u16, gActiveBattler);
    SetMultiuseSpriteTemplateToTrainerBack(
        (*gSaveBlock2Ptr).playerGender as u16,
        GetBattlerPosition(gActiveBattler),
    );
    gBattlerSpriteIds[gActiveBattler] = CreateSprite(
        &raw mut gMultiuseSpriteTemplate,
        80,
        (8 - (*(&raw const crate::data::data_tables::gTrainerBackPicCoords)
            .cast::<CArray<MonCoords, 0>>())[(*gSaveBlock2Ptr).playerGender]
            .size as i16)
            * 4
            + 80,
        30,
    );
    gSprites[gBattlerSpriteIds[gActiveBattler]]
        .oam
        .set_paletteNum(gActiveBattler as u16);
    gSprites[gBattlerSpriteIds[gActiveBattler]].x2 = DISPLAY_WIDTH as i16;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[sSpeedX] = -2;
    gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(SpriteCB_TrainerSlideIn);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnBattlerSpriteCallbackDummy);
}
pub(crate) unsafe fn SafariHandleTrainerSlide() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleTrainerSlideBack() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleFaintAnimation() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandlePaletteFade() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleSuccessBallThrowAnim() {
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
pub(crate) unsafe fn SafariHandleBallThrowAnim() {
    let ballThrowCaseId: u8 = gBattleBufferA[gActiveBattler][1];
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
pub(crate) unsafe fn SafariHandlePause() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleMoveAnimation() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandlePrintString() {
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    let stringId: *mut u16 = &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
        .cast::<CArray<CArray<u8, 512>, 4>>()
        .cast_mut())[gActiveBattler][2] as *mut u16;
    BufferStringBattle(*stringId);
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnInactiveTextPrinter);
}
pub(crate) unsafe fn SafariHandlePrintSelectionString() {
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        SafariHandlePrintString();
    } else {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe fn HandleChooseActionAfterDma3() {
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        gBattle_BG0_X = 0;
        gBattle_BG0_Y = DISPLAY_HEIGHT;
        gBattlerControllerFuncs[gActiveBattler] = Some(HandleInputChooseAction);
    }
}
pub(crate) unsafe fn SafariHandleChooseAction() {
    gBattlerControllerFuncs[gActiveBattler] = Some(HandleChooseActionAfterDma3);
    BattlePutTextOnWindow(
        (*(&raw const crate::data::battle_message::gText_SafariZoneMenu).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        B_WIN_ACTION_MENU,
    );
    for i in 0..4i32 {
        ActionSelectionDestroyCursorAt(i as u8);
    }
    ActionSelectionCreateCursorAt(gActionSelectionCursor[gActiveBattler], 0);
    BattleStringExpandPlaceholdersToDisplayedString(
        (*(&raw const crate::data::battle_message::gText_WhatWillPkmnDo2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_ACTION_PROMPT);
}
pub(crate) unsafe fn SafariHandleYesNoBox() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleChooseMove() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleChooseItem() {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
    gBattlerControllerFuncs[gActiveBattler] = Some(SafariOpenPokeblockCase);
    gBattlerInMenuId = gActiveBattler;
}
pub(crate) unsafe fn SafariHandleChoosePokemon() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleCmd23() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleHealthBarUpdate() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleExpUpdate() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleStatusIconUpdate() {
    UpdateHealthboxAttribute(
        gHealthboxSpriteIds[gActiveBattler],
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        HEALTHBOX_SAFARI_BALLS_TEXT,
    );
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleStatusAnimation() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleStatusXor() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleDataTransfer() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleDMA3Transfer() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandlePlayBGM() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleCmd32() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleTwoReturnValues() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleChosenMonReturnValue() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleOneReturnValue() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleOneReturnValue_Duplicate() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleClearUnkVar() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleSetUnkVar() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleClearUnkFlag() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleToggleUnkFlag() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleHitAnimation() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleCantSwitch() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandlePlaySE() {
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
pub(crate) unsafe fn SafariHandlePlayFanfareOrBGM() {
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
pub(crate) unsafe fn SafariHandleFaintingCry() {
    let species: u16 = GetMonData2(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        MON_DATA_SPECIES,
    ) as u16;
    PlayCry_Normal(species, 25);
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleIntroSlide() {
    HandleIntroSlide(gBattleBufferA[gActiveBattler][1]);
    gIntroSlideFlags |= 1;
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleIntroTrainerBallThrow() {
    UpdateHealthboxAttribute(
        gHealthboxSpriteIds[gActiveBattler],
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        HEALTHBOX_SAFARI_ALL_TEXT,
    );
    StartHealthboxSlideIn(gActiveBattler);
    SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler]);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnHealthboxSpriteCallbackDummy);
}
pub(crate) unsafe fn SafariHandleDrawPartyStatusSummary() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleHidePartyStatusSummary() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleEndBounceEffect() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleSpriteInvisibility() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleBattleAnimation() {
    let animationId: u8 = gBattleBufferA[gActiveBattler][1];
    let argument: u16 =
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
pub(crate) unsafe fn SafariHandleLinkStandbyMsg() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleResetActionMoveSelection() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleEndLinkBattle() {
    gBattleOutcome = gBattleBufferA[gActiveBattler][1];
    FadeOutMapMusic(5);
    BeginFastPaletteFade(3);
    SafariBufferExecCompleted();
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 && gBattleTypeFlags & BATTLE_TYPE_IS_MASTER == 0 {
        gBattlerControllerFuncs[gActiveBattler] = Some(SafariSetBattleEndCallbacks);
    }
}
pub(crate) fn SafariCmdEnd() {}
