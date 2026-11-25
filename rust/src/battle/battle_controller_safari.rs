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
pub fn SetControllerToSafari() {
    {
        let gl_i0 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i0] = Some(SafariBufferRunCommand))
    };
}
pub(crate) unsafe fn SafariBufferRunCommand() {
    if gBattleControllerExecFlags.get()
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
        != 0
    {
        if ({
            let gl_i1 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i1][0])
        }) < 57
        {
            sSafariBufferCommands[{
                let gl_i2 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i2][0])
            }]
            .unwrap_unchecked()();
        } else {
            SafariBufferExecCompleted();
        }
    }
}
pub(crate) unsafe fn HandleInputChooseAction() {
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        match {
            let gl_i3 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i3])
        } {
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
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_LEFT != 0 {
        if ({
            let gl_i4 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i4])
        }) as i32
            & 1
            != 0
        {
            PlaySE(SE_SELECT);
            ActionSelectionDestroyCursorAt({
                let gl_i5 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i5])
            });
            {
                let gl_i6 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i6] ^= 1)
            };
            ActionSelectionCreateCursorAt(
                {
                    let gl_i7 = gActiveBattler.get();
                    crate::gl!(gActionSelectionCursor[gl_i7])
                },
                0,
            );
        }
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_RIGHT != 0 {
        if ({
            let gl_i8 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i8])
        }) as i32
            & 1
            == 0
        {
            PlaySE(SE_SELECT);
            ActionSelectionDestroyCursorAt({
                let gl_i9 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i9])
            });
            {
                let gl_i10 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i10] ^= 1)
            };
            ActionSelectionCreateCursorAt(
                {
                    let gl_i11 = gActiveBattler.get();
                    crate::gl!(gActionSelectionCursor[gl_i11])
                },
                0,
            );
        }
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0 {
        if ({
            let gl_i12 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i12])
        }) as i32
            & 2
            != 0
        {
            PlaySE(SE_SELECT);
            ActionSelectionDestroyCursorAt({
                let gl_i13 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i13])
            });
            {
                let gl_i14 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i14] ^= 2)
            };
            ActionSelectionCreateCursorAt(
                {
                    let gl_i15 = gActiveBattler.get();
                    crate::gl!(gActionSelectionCursor[gl_i15])
                },
                0,
            );
        }
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_DOWN != 0
        && (*(crate::gl!(&raw @at crate::battle_main::gActionSelectionCursor;).cast_const())
            .cast::<CArray<u8, 4>>()
            .cast_mut())[gActiveBattler.get()] as i32
            & 2
            == 0
    {
        PlaySE(SE_SELECT);
        ActionSelectionDestroyCursorAt({
            let gl_i16 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i16])
        });
        {
            let gl_i17 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i17] ^= 2)
        };
        ActionSelectionCreateCursorAt(
            {
                let gl_i18 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i18])
            },
            0,
        );
    }
}
pub(crate) unsafe fn CompleteOnBattlerSpriteCallbackDummy() {
    if (*gSprites.as_ptr())[{
        let gl_i19 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i19])
    }]
    .callback
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
    if (*gSprites.as_ptr())[{
        let gl_i20 = gActiveBattler.get();
        crate::gl!(gHealthboxSpriteIds[gl_i20])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe fn SafariSetBattleEndCallbacks() {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        (*gMain.as_ptr()).set_inBattle(FALSE);
        crate::gl!(gMain.callback1 = crate::gl!(gPreBattleCallback1));
        SetMainCallback2(crate::gl!(gMain.savedCallback));
    }
}
pub(crate) unsafe fn CompleteOnSpecialAnimDone() {
    if gDoingBattleAnim.get() == 0
        || (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .specialAnimActive()
            == 0
    {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe fn SafariOpenPokeblockCase() {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        {
            let gl_i21 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i21] = Some(CompleteWhenChosePokeblock))
        };
        FreeAllWindowBuffers();
        OpenPokeblockCaseInBattle();
    }
}
pub(crate) unsafe fn CompleteWhenChosePokeblock() {
    if crate::gl!(gMain.callback2) == Some(BattleMainCB2 as unsafe fn())
        && (*gPaletteFade.as_ptr()).active() == 0
    {
        BtlController_EmitOneReturnValue(B_COMM_TO_ENGINE, gSpecialVar_ItemId.get());
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnFinishedBattleAnimation() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .animFromTableActive()
        == 0
    {
        SafariBufferExecCompleted();
    }
}
unsafe fn SafariBufferExecCompleted() {
    {
        let gl_i22 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i22] = Some(SafariBufferRunCommand))
    };
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        let mut playerId: u8 = GetMultiplayerId();
        PrepareBufferDataTransferLink(B_COMM_CONTROLLER_IS_DONE, 4, &raw mut playerId);
        {
            let gl_i23 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i23][0] = CONTROLLER_TERMINATOR_NOP)
        };
    } else {
        gBattleControllerExecFlags
            .set(gBattleControllerExecFlags.get() & (!gBitTable[gActiveBattler.get()]));
    }
}
pub(crate) unsafe fn CompleteOnFinishedStatusAnimation() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .statusAnimActive()
        == 0
    {
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
    DecompressTrainerBackPic(
        (*crate::gl!(gSaveBlock2Ptr)).playerGender as u16,
        gActiveBattler.get(),
    );
    SetMultiuseSpriteTemplateToTrainerBack(
        (*crate::gl!(gSaveBlock2Ptr)).playerGender as u16,
        GetBattlerPosition(gActiveBattler.get()),
    );
    {
        let gl_v = CreateSprite(
            crate::gl!(&raw gMultiuseSpriteTemplate),
            80,
            (8 - (*(&raw const crate::data::data_tables::gTrainerBackPicCoords)
                .cast::<CArray<MonCoords, 0>>())[(*crate::gl!(gSaveBlock2Ptr)).playerGender]
                .size as i16)
                * 4
                + 80,
            30,
        );
        let gl_i24 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i24] = gl_v)
    };
    (*gSprites.as_ptr())[{
        let gl_i25 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i25])
    }]
    .oam
    .set_paletteNum(gActiveBattler.get() as u16);
    (*gSprites.as_ptr())[{
        let gl_i26 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i26])
    }]
    .x2 = DISPLAY_WIDTH as i16;
    (*gSprites.as_ptr())[{
        let gl_i27 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i27])
    }]
    .data[sSpeedX] = -2;
    (*gSprites.as_ptr())[{
        let gl_i28 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i28])
    }]
    .callback = Some(SpriteCB_TrainerSlideIn);
    {
        let gl_i29 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i29] = Some(CompleteOnBattlerSpriteCallbackDummy))
    };
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
    (*(*crate::gl!(gBattleSpritesDataPtr)).animationData).ballThrowCaseId = BALL_3_SHAKES_SUCCESS;
    gDoingBattleAnim.set(TRUE);
    InitAndLaunchSpecialAnimation(
        gActiveBattler.get(),
        gActiveBattler.get(),
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        B_ANIM_BALL_THROW_WITH_TRAINER,
    );
    {
        let gl_i30 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i30] = Some(CompleteOnSpecialAnimDone))
    };
}
pub(crate) unsafe fn SafariHandleBallThrowAnim() {
    let ballThrowCaseId: u8 = {
        let gl_i31 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i31][1])
    };
    (*(*crate::gl!(gBattleSpritesDataPtr)).animationData).ballThrowCaseId = ballThrowCaseId;
    gDoingBattleAnim.set(TRUE);
    InitAndLaunchSpecialAnimation(
        gActiveBattler.get(),
        gActiveBattler.get(),
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        B_ANIM_BALL_THROW_WITH_TRAINER,
    );
    {
        let gl_i32 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i32] = Some(CompleteOnSpecialAnimDone))
    };
}
pub(crate) unsafe fn SafariHandlePause() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleMoveAnimation() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandlePrintString() {
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    let stringId: *mut u16 = &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;)
        .cast_const())
    .cast::<CArray<CArray<u8, 512>, 4>>()
    .cast_mut())[gActiveBattler.get()][2] as *mut u16;
    BufferStringBattle(*stringId);
    BattlePutTextOnWindow((*gDisplayedStringBattle.as_ptr()).as_mut_ptr(), B_WIN_MSG);
    {
        let gl_i33 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i33] = Some(CompleteOnInactiveTextPrinter))
    };
}
pub(crate) unsafe fn SafariHandlePrintSelectionString() {
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        SafariHandlePrintString();
    } else {
        SafariBufferExecCompleted();
    }
}
pub(crate) unsafe fn HandleChooseActionAfterDma3() {
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        gBattle_BG0_X.set(0);
        gBattle_BG0_Y.set(DISPLAY_HEIGHT);
        {
            let gl_i34 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i34] = Some(HandleInputChooseAction))
        };
    }
}
pub(crate) unsafe fn SafariHandleChooseAction() {
    {
        let gl_i35 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i35] = Some(HandleChooseActionAfterDma3))
    };
    BattlePutTextOnWindow(
        (*(&raw const crate::data::battle_message::gText_SafariZoneMenu).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        B_WIN_ACTION_MENU,
    );
    for i in 0..4i32 {
        ActionSelectionDestroyCursorAt(i as u8);
    }
    ActionSelectionCreateCursorAt(
        {
            let gl_i36 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i36])
        },
        0,
    );
    BattleStringExpandPlaceholdersToDisplayedString(
        (*(&raw const crate::data::battle_message::gText_WhatWillPkmnDo2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    BattlePutTextOnWindow(
        (*gDisplayedStringBattle.as_ptr()).as_mut_ptr(),
        B_WIN_ACTION_PROMPT,
    );
}
pub(crate) unsafe fn SafariHandleYesNoBox() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleChooseMove() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleChooseItem() {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
    {
        let gl_i37 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i37] = Some(SafariOpenPokeblockCase))
    };
    gBattlerInMenuId.set(gActiveBattler.get());
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
        {
            let gl_i38 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i38])
        },
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i39 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i39])
        }],
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
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        pan = SOUND_PAN_ATTACKER;
    } else {
        pan = SOUND_PAN_TARGET;
    }
    PlaySE12WithPanning(
        ({
            let gl_i40 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i40][1])
        }) as u16
            | (({
                let gl_i41 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i41][2])
            }) as u16)
                << 8,
        pan,
    );
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandlePlayFanfareOrBGM() {
    if ({
        let gl_i42 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i42][3])
    }) != 0
    {
        BattleStopLowHpSound();
        PlayBGM(
            ({
                let gl_i43 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i43][1])
            }) as u16
                | (({
                    let gl_i44 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i44][2])
                }) as u16)
                    << 8,
        );
    } else {
        PlayFanfare(
            ({
                let gl_i45 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i45][1])
            }) as u16
                | (({
                    let gl_i46 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i46][2])
                }) as u16)
                    << 8,
        );
    }
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleFaintingCry() {
    let species: u16 = GetMonData2(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i47 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i47])
        }],
        MON_DATA_SPECIES,
    ) as u16;
    PlayCry_Normal(species, 25);
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleIntroSlide() {
    HandleIntroSlide({
        let gl_i48 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i48][1])
    });
    gIntroSlideFlags.set(gIntroSlideFlags.get() | (1));
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleIntroTrainerBallThrow() {
    UpdateHealthboxAttribute(
        {
            let gl_i49 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i49])
        },
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i50 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i50])
        }],
        HEALTHBOX_SAFARI_ALL_TEXT,
    );
    StartHealthboxSlideIn(gActiveBattler.get());
    SetHealthboxSpriteVisible({
        let gl_i51 = gActiveBattler.get();
        crate::gl!(gHealthboxSpriteIds[gl_i51])
    });
    {
        let gl_i52 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i52] = Some(CompleteOnHealthboxSpriteCallbackDummy))
    };
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
    let animationId: u8 = {
        let gl_i53 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i53][1])
    };
    let argument: u16 = ({
        let gl_i54 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i54][2])
    }) as u16
        | (({
            let gl_i55 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i55][3])
        }) as u16)
            << 8;
    if TryHandleLaunchBattleTableAnimation(
        gActiveBattler.get(),
        gActiveBattler.get(),
        gActiveBattler.get(),
        animationId,
        argument,
    ) != 0
    {
        SafariBufferExecCompleted();
    } else {
        {
            let gl_i56 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i56] = Some(CompleteOnFinishedBattleAnimation))
        };
    }
}
pub(crate) unsafe fn SafariHandleLinkStandbyMsg() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleResetActionMoveSelection() {
    SafariBufferExecCompleted();
}
pub(crate) unsafe fn SafariHandleEndLinkBattle() {
    gBattleOutcome.set({
        let gl_i57 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i57][1])
    });
    FadeOutMapMusic(5);
    BeginFastPaletteFade(3);
    SafariBufferExecCompleted();
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_IS_MASTER == 0
    {
        {
            let gl_i58 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i58] = Some(SafariSetBattleEndCallbacks))
        };
    }
}
pub(crate) fn SafariCmdEnd() {}
