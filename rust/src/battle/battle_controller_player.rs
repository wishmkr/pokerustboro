//! Translated from `src/battle_controller_player.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::battle_anim::{
    DoMoveAnim, gAnimDisableStructPtr, gAnimFriendship, gAnimMoveDmg, gAnimMovePower,
    gAnimMoveTurn, gAnimScriptActive, gAnimScriptCallback, gWeatherMoveAnim,
};
use crate::battle_anim_mons::{
    GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide, GetBattlerSpriteCoord,
    GetBattlerSpriteDefault_Y, GetBattlerSpriteSubpriority, IsBattlerSpritePresent, IsDoubleBattle,
    SetSpritePrimaryCoordsFromSecondaryCoords, StartAnimLinearTranslation,
    StoreSpriteCallbackInData6,
};
use crate::battle_anim_throw::TryShinyAnimation;
use crate::battle_arena::BattleArena_DeductSkillPoints;
use crate::battle_controllers::{
    BtlController_EmitChosenMonReturnValue, BtlController_EmitDataTransfer,
    BtlController_EmitOneReturnValue, BtlController_EmitOneReturnValue_Duplicate,
    BtlController_EmitTwoReturnValues, PrepareBufferDataTransferLink, gUnusedControllerStruct,
};
use crate::battle_dome::gPlayerPartyLostHP;
use crate::battle_gfx_sfx_util::{
    BattleGfxSfxDummy2, BattleGfxSfxDummy3, BattleLoadPlayerMonSpriteGfx, BattleStopLowHpSound,
    ChooseMoveAndTargetInBattlePalace, ClearTemporarySpeciesSpriteData,
    CopyAllBattleSpritesInvisibilities, CopyBattleSpriteInvisibility, DecompressTrainerBackPic,
    DecompressTrainerFrontPic, HandleLowHpMusicChange, InitAndLaunchChosenStatusAnimation,
    InitAndLaunchSpecialAnimation, IsBattleSEPlaying, IsMoveWithoutAnimation, LoadBattleBarGfx,
    SetBattlerSpriteAffineMode, SpriteCB_TrainerSlideIn, SpriteCB_WaitForBattlerBallReleaseAnim,
    TryHandleLaunchBattleTableAnimation, TrySetBehindSubstituteSpriteBit,
};
use crate::battle_interface::{
    CreatePartyStatusSummarySprites, MoveBattleBar, SetBattleBarStruct,
    SetHealthboxSpriteInvisible, SetHealthboxSpriteVisible, SwapHpBarsWithHpText,
    Task_HidePartyStatusSummary, UpdateHealthboxAttribute, UpdateHpTextInHealthbox,
};
use crate::battle_intro::HandleIntroSlide;
use crate::battle_main::{
    BattleMainCB2, CB2_InitEndLinkBattle, DoBounceEffect, EndBounceEffect, SpriteCB_FaintSlideAnim,
    SpriteCB_HideAsMoveTarget, SpriteCB_ShowAsMoveTarget, gAbsentBattlerFlags, gActiveBattler,
    gBattle_BG0_X, gBattle_BG0_Y, gBattleControllerExecFlags, gBattleMons, gBattleOutcome,
    gBattleSpritesDataPtr, gBattleStruct, gBattleTypeFlags, gBattlerControllerFuncs,
    gBattlerInMenuId, gBattlersCount, gDisableStructs, gDoingBattleAnim, gIntroSlideFlags,
    gMultiUsePlayerCursor, gNumberOfMovesToChoose, gPlayerDpadHoldFrames, gPreBattleCallback1,
    gTransformedPersonalities,
};
use crate::battle_main::{
    gActionSelectionCursor, gBattleBufferA, gBattleControllerData, gBattleMonForms,
    gBattlerPartyIndexes, gBattlerSpriteIds, gBattlerStatusSummaryTaskId, gDisplayedStringBattle,
    gHealthboxSpriteIds, gMoveSelectionCursor,
};
use crate::battle_message::{
    BattlePutTextOnWindow, BattleStringExpandPlaceholdersToDisplayedString, BufferStringBattle,
    SetPPNumbersPaletteInMoveSelection,
};
use crate::battle_script_commands::{
    BattleCreateYesNoCursorAt, BattleDestroyYesNoCursorAt, HandleBattleWindow,
};
use crate::battle_setup::gPartnerTrainerId;
use crate::battle_tv::{
    BattleTv_ClearExplosionFaintCause, BattleTv_SetDataBasedOnAnimation,
    BattleTv_SetDataBasedOnMove, BattleTv_SetDataBasedOnString, TryPutLinkBattleTvShowOnAir,
};
use crate::bg::{CopyBgTilemapBufferToVram, IsDma3ManagerBusyWithBgCopy};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::item::AddBagItem;
use crate::item_menu::{CB2_BagMenuFromBattle, gSpecialVar_ItemId};
use crate::link::{
    GetMultiplayerId, IsLinkTaskFinished, SetCloseLinkCallback, SetLinkStandbyCallback,
    gLinkPlayers, gReceivedRemoteLinkPlayers, gWirelessCommType,
};
use crate::load_save::gSaveBlock2Ptr;
use crate::m4a::{gMPlayInfo_BGM, m4aMPlayContinue, m4aMPlayVolumeControl, m4aSongNumStop};
use crate::palette::{
    BeginFastPaletteFade, BeginNormalPaletteFade, LoadCompressedPalette, gPaletteFade,
};
use crate::party_menu::gBattlePartyCurrentOrder;
use crate::party_menu::{OpenPartyMenuInBattle, gPartyMenuUseExitCallback, gSelectedMonPartyId};
use crate::pokeball::{
    DoHitAnimHealthboxEffect, DoPokeballSendOutAnimation, StartHealthboxSlideIn,
};
use crate::pokemon::{
    CalculateMonStats, CountAliveMonsInBattle, GetDefaultMoveTarget, GetMonData2, GetMonData3,
    PlayerGenderToFrontTrainerPicId, SetMonData, SetMultiuseSpriteTemplateToPokemon,
    SetMultiuseSpriteTemplateToTrainerBack, SetMultiuseSpriteTemplateToTrainerFront,
    gMultiuseSpriteTemplate, gPlayerParty,
};
use crate::recorded_battle::{
    RecordedBattle_RecordAllBattlerData, gBattlePalaceMoveSelectionRngValue,
};
use crate::reshow_battle_screen::{ReshowBattleScreenAfterMenu, ReshowBattleScreenDummy};
use crate::sound::{
    FadeOutMapMusic, IsCryPlayingOrClearCrySongs, PlayBGM, PlayCry_ByMode, PlayFanfare, PlaySE,
    PlaySE12WithPanning,
};
use crate::sprite::gSprites;
use crate::sprite::{
    AllocSpritePalette, FreeOamMatrix, FreeSpritePaletteByTag, FreeSpriteTilesByTag,
    GetSpritePaletteTagByPaletteNum, IndexOfSpritePaletteTag,
};
use crate::task::{DestroyTask, TaskDummy};
use crate::task::{task_get, task_set, task_set_func};
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
/// `ConvertIntToDecimalStringN` with this module's view of its types.
#[inline]
unsafe fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8 {
    unsafe { crate::string_util::ConvertIntToDecimalStringN(a0 as _, a1, a2, a3) as *mut u8 }
}
/// `CopyToBgTilemapBufferRect_ChangePalette` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBufferRect_ChangePalette(
    a0: u8,
    a1: *mut c_void,
    a2: u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: u8,
) {
    unsafe {
        crate::bg::CopyToBgTilemapBufferRect_ChangePalette(a0, a1 as _, a2, a3, a4, a5, a6);
    }
}
/// `CreateInvisibleSpriteWithCallback` with this module's view of its types.
#[inline]
fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe fn(*mut Sprite)>) -> u8 {
    unsafe { crate::util::CreateInvisibleSpriteWithCallback(core::mem::transmute(a0)) }
}
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
    }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCopy_Nickname` with this module's view of its types.
#[inline]
unsafe fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy_Nickname(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const tBattlerId: usize = 0;
const tExpTask_monId: usize = 0;
const tExpTask_gainedExp: usize = 1;
const tStartTimer: usize = 1;
const sSpeedY: usize = 2;
const tExpTask_battler: usize = 2;
const sBattlerId: usize = 5;
const tExpTask_frames: usize = 10;
// Data tables (translate with cdata.py): sPlayerBufferCommands sTargetIdentities sUnused

static sPlayerBufferCommands: Table<CArray<Option<unsafe fn()>, 57>> =
    Table((&raw const crate::data::battle_controller_player::sPlayerBufferCommands).cast());
static sTargetIdentities: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_controller_player::sTargetIdentities).cast());

/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub fn BattleControllerDummy() {}
pub unsafe fn SetControllerToPlayer() {
    {
        let gl_i0 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i0] = Some(PlayerBufferRunCommand))
    };
    gDoingBattleAnim.set(FALSE);
    gPlayerDpadHoldFrames.set(0);
}
unsafe fn PlayerBufferExecCompleted() {
    {
        let gl_i1 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i1] = Some(PlayerBufferRunCommand))
    };
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        let mut playerId: u8 = GetMultiplayerId();
        PrepareBufferDataTransferLink(B_COMM_CONTROLLER_IS_DONE, 4, &raw mut playerId);
        {
            let gl_i2 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i2][0] = CONTROLLER_TERMINATOR_NOP)
        };
    } else {
        gBattleControllerExecFlags
            .set(gBattleControllerExecFlags.get() & (!gBitTable[gActiveBattler.get()]));
    }
}
pub(crate) unsafe fn PlayerBufferRunCommand() {
    if gBattleControllerExecFlags.get()
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
        != 0
    {
        if ({
            let gl_i3 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i3][0])
        }) < 57
        {
            sPlayerBufferCommands[{
                let gl_i4 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i4][0])
            }]
            .unwrap_unchecked()();
        } else {
            PlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe fn CompleteOnBankSpritePosX_0() {
    if (*gSprites.as_ptr())[{
        let gl_i5 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i5])
    }]
    .x2 == 0
    {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn HandleInputChooseAction() {
    let itemId: u16 = ({
        let gl_i6 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i6][2])
    }) as u16
        | (({
            let gl_i7 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i7][3])
        }) as u16)
            << 8;
    DoBounceEffect(gActiveBattler.get(), 0x1, 7, 1);
    DoBounceEffect(gActiveBattler.get(), BOUNCE_MON, 7, 1);
    if crate::gl!(gMain.newAndRepeatedKeys) as i32 & DPAD_ANY != 0
        && (*crate::gl!(gSaveBlock2Ptr)).optionsButtonMode == OPTIONS_BUTTON_MODE_L_EQUALS_A
    {
        gPlayerDpadHoldFrames.set(gPlayerDpadHoldFrames.get() + (1));
    } else {
        gPlayerDpadHoldFrames.set(0);
    }
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        match {
            let gl_i8 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i8])
        } {
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
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_LEFT != 0 {
        if ({
            let gl_i9 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i9])
        }) as i32
            & 1
            != 0
        {
            PlaySE(SE_SELECT);
            ActionSelectionDestroyCursorAt({
                let gl_i10 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i10])
            });
            {
                let gl_i11 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i11] ^= 1)
            };
            ActionSelectionCreateCursorAt(
                {
                    let gl_i12 = gActiveBattler.get();
                    crate::gl!(gActionSelectionCursor[gl_i12])
                },
                0,
            );
        }
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_RIGHT != 0 {
        if ({
            let gl_i13 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i13])
        }) as i32
            & 1
            == 0
        {
            PlaySE(SE_SELECT);
            ActionSelectionDestroyCursorAt({
                let gl_i14 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i14])
            });
            {
                let gl_i15 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i15] ^= 1)
            };
            ActionSelectionCreateCursorAt(
                {
                    let gl_i16 = gActiveBattler.get();
                    crate::gl!(gActionSelectionCursor[gl_i16])
                },
                0,
            );
        }
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0 {
        if ({
            let gl_i17 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i17])
        }) as i32
            & 2
            != 0
        {
            PlaySE(SE_SELECT);
            ActionSelectionDestroyCursorAt({
                let gl_i18 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i18])
            });
            {
                let gl_i19 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i19] ^= 2)
            };
            ActionSelectionCreateCursorAt(
                {
                    let gl_i20 = gActiveBattler.get();
                    crate::gl!(gActionSelectionCursor[gl_i20])
                },
                0,
            );
        }
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_DOWN != 0 {
        if ({
            let gl_i21 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i21])
        }) as i32
            & 2
            == 0
        {
            PlaySE(SE_SELECT);
            ActionSelectionDestroyCursorAt({
                let gl_i22 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i22])
            });
            {
                let gl_i23 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i23] ^= 2)
            };
            ActionSelectionCreateCursorAt(
                {
                    let gl_i24 = gActiveBattler.get();
                    crate::gl!(gActionSelectionCursor[gl_i24])
                },
                0,
            );
        }
    } else if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 || gPlayerDpadHoldFrames.get() > 59 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
            && GetBattlerPosition(gActiveBattler.get()) == B_POSITION_PLAYER_RIGHT
            && gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [GetBattlerAtPosition(B_POSITION_PLAYER_LEFT)]
                == 0
            && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI == 0
        {
            if ({
                let gl_i25 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i25][1])
            }) == 1
            {
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
    } else if crate::gl!(gMain.newKeys) as i32 & START_BUTTON != 0 {
        SwapHpBarsWithHpText();
    }
}
unsafe fn UnusedEndBounceEffect() {
    EndBounceEffect(gActiveBattler.get(), BOUNCE_HEALTHBOX);
    EndBounceEffect(gActiveBattler.get(), BOUNCE_MON);
    {
        let gl_i26 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i26] = Some(HandleInputChooseTarget))
    };
}
pub(crate) unsafe fn HandleInputChooseTarget() {
    let mut identities: CArray<u8, 4> = zeroed();
    memcpy(
        identities.as_mut_ptr(),
        sTargetIdentities.as_ptr().cast_mut(),
        4,
    );
    DoBounceEffect(gMultiUsePlayerCursor.get(), 0x1, 15, 1);
    let mut i: i32 = 0;
    if gBattlersCount.get() != 0 {
        loop {
            if i != gMultiUsePlayerCursor.get() as i32 {
                EndBounceEffect(i as u8, BOUNCE_HEALTHBOX);
            }
            i += 1;
            if i >= gBattlersCount.get() as i32 {
                break;
            }
        }
    }
    if crate::gl!(gMain.heldKeys) as i32 & DPAD_ANY != 0
        && (*crate::gl!(gSaveBlock2Ptr)).optionsButtonMode == OPTIONS_BUTTON_MODE_L_EQUALS_A
    {
        gPlayerDpadHoldFrames.set(gPlayerDpadHoldFrames.get() + (1));
    } else {
        gPlayerDpadHoldFrames.set(0);
    }
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        (*gSprites.as_ptr())[{
            let gl_i27 = gMultiUsePlayerCursor.get();
            crate::gl!(gBattlerSpriteIds[gl_i27])
        }]
        .callback = Some(SpriteCB_HideAsMoveTarget);
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            B_ACTION_EXEC_SCRIPT,
            ({
                let gl_i28 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i28])
            }) as u16
                | (gMultiUsePlayerCursor.get() as u16) << 8,
        );
        EndBounceEffect(gMultiUsePlayerCursor.get(), BOUNCE_HEALTHBOX);
        PlayerBufferExecCompleted();
    } else if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 || gPlayerDpadHoldFrames.get() > 59 {
        PlaySE(SE_SELECT);
        (*gSprites.as_ptr())[{
            let gl_i29 = gMultiUsePlayerCursor.get();
            crate::gl!(gBattlerSpriteIds[gl_i29])
        }]
        .callback = Some(SpriteCB_HideAsMoveTarget);
        {
            let gl_i30 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i30] = Some(HandleInputChooseMove))
        };
        DoBounceEffect(gActiveBattler.get(), 0x1, 7, 1);
        DoBounceEffect(gActiveBattler.get(), BOUNCE_MON, 7, 1);
        EndBounceEffect(gMultiUsePlayerCursor.get(), BOUNCE_HEALTHBOX);
    } else if crate::gl!(gMain.newKeys) as i32 & 96 != 0 {
        PlaySE(SE_SELECT);
        (*gSprites.as_ptr())[{
            let gl_i31 = gMultiUsePlayerCursor.get();
            crate::gl!(gBattlerSpriteIds[gl_i31])
        }]
        .callback = Some(SpriteCB_HideAsMoveTarget);
        loop {
            let currSelIdentity: u8 = GetBattlerPosition(gMultiUsePlayerCursor.get());
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
                gMultiUsePlayerCursor.set(GetBattlerAtPosition(identities[i]));
                if gMultiUsePlayerCursor.get() != gBattlersCount.get() {
                    break;
                }
            }
            i = 0;
            match GetBattlerPosition(gMultiUsePlayerCursor.get()) {
                B_POSITION_PLAYER_LEFT | B_POSITION_PLAYER_RIGHT => {
                    if gActiveBattler.get() != gMultiUsePlayerCursor.get() {
                        i += 1;
                    } else if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<
                        BattleMove,
                        0,
                    >>(
                    ))[GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())[{
                            let gl_i32 = gActiveBattler.get();
                            crate::gl!(gBattlerPartyIndexes[gl_i32])
                        }],
                        MON_DATA_MOVE1
                            + ({
                                let gl_i33 = gActiveBattler.get();
                                crate::gl!(gMoveSelectionCursor[gl_i33])
                            }) as i32,
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
            if gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gMultiUsePlayerCursor.get()]
                != 0
            {
                i = 0;
            }
            if i != 0 {
                break;
            }
        }
        (*gSprites.as_ptr())[{
            let gl_i34 = gMultiUsePlayerCursor.get();
            crate::gl!(gBattlerSpriteIds[gl_i34])
        }]
        .callback = Some(SpriteCB_ShowAsMoveTarget);
    } else if crate::gl!(gMain.newKeys) as i32 & 144 != 0 {
        PlaySE(SE_SELECT);
        (*gSprites.as_ptr())[{
            let gl_i35 = gMultiUsePlayerCursor.get();
            crate::gl!(gBattlerSpriteIds[gl_i35])
        }]
        .callback = Some(SpriteCB_HideAsMoveTarget);
        loop {
            let currSelIdentity: u8 = GetBattlerPosition(gMultiUsePlayerCursor.get());
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
                gMultiUsePlayerCursor.set(GetBattlerAtPosition(identities[i]));
                if gMultiUsePlayerCursor.get() != gBattlersCount.get() {
                    break;
                }
            }
            i = 0;
            match GetBattlerPosition(gMultiUsePlayerCursor.get()) {
                B_POSITION_PLAYER_LEFT | B_POSITION_PLAYER_RIGHT => {
                    if gActiveBattler.get() != gMultiUsePlayerCursor.get() {
                        i += 1;
                    } else if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<
                        BattleMove,
                        0,
                    >>(
                    ))[GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())[{
                            let gl_i36 = gActiveBattler.get();
                            crate::gl!(gBattlerPartyIndexes[gl_i36])
                        }],
                        MON_DATA_MOVE1
                            + ({
                                let gl_i37 = gActiveBattler.get();
                                crate::gl!(gMoveSelectionCursor[gl_i37])
                            }) as i32,
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
            if gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [gMultiUsePlayerCursor.get()]
                != 0
            {
                i = 0;
            }
            if i != 0 {
                break;
            }
        }
        (*gSprites.as_ptr())[{
            let gl_i38 = gMultiUsePlayerCursor.get();
            crate::gl!(gBattlerSpriteIds[gl_i38])
        }]
        .callback = Some(SpriteCB_ShowAsMoveTarget);
    }
}
pub(crate) unsafe fn HandleInputChooseMove() {
    let mut canSelectTarget: u32 = FALSE as u32;
    let moveInfo: *mut ChooseMoveStruct =
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][4] as *mut ChooseMoveStruct;
    if crate::gl!(gMain.heldKeys) as i32 & DPAD_ANY != 0
        && (*crate::gl!(gSaveBlock2Ptr)).optionsButtonMode == OPTIONS_BUTTON_MODE_L_EQUALS_A
    {
        gPlayerDpadHoldFrames.set(gPlayerDpadHoldFrames.get() + (1));
    } else {
        gPlayerDpadHoldFrames.set(0);
    }
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        let mut moveTarget: u8 = 0;
        PlaySE(SE_SELECT);
        if (*moveInfo).moves[{
            let gl_i39 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i39])
        }] == MOVE_CURSE
        {
            if (*moveInfo).monTypes[0] != TYPE_GHOST && (*moveInfo).monTypes[1] != TYPE_GHOST {
                moveTarget = MOVE_TARGET_USER;
            } else {
                moveTarget = MOVE_TARGET_SELECTED;
            }
        } else {
            moveTarget = (*(&raw const crate::data::pokemon::gBattleMoves)
                .cast::<CArray<BattleMove, 0>>())[(*moveInfo).moves[{
                let gl_i40 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i40])
            }]]
            .target;
        }
        if moveTarget as i32 & MOVE_TARGET_USER as i32 != 0 {
            gMultiUsePlayerCursor.set(gActiveBattler.get());
        } else {
            gMultiUsePlayerCursor.set(GetBattlerAtPosition(
                GetBattlerPosition(gActiveBattler.get()) & 1 ^ 1,
            ));
        }
        if ({
            let gl_i41 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i41][1])
        }) == 0
        {
            if moveTarget as i32 & 2 != 0
                && (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][2]
                    == 0
            {
                canSelectTarget += 1;
            }
        } else {
            if moveTarget as i32 & 125 == 0 {
                canSelectTarget += 1;
            }
            if (*moveInfo).currentPP[{
                let gl_i42 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i42])
            }] == 0
            {
                canSelectTarget = FALSE as u32;
            } else if moveTarget as i32 & 18 == 0
                && CountAliveMonsInBattle(BATTLE_ALIVE_EXCEPT_ACTIVE) <= 1
            {
                gMultiUsePlayerCursor.set(GetDefaultMoveTarget(gActiveBattler.get()));
                canSelectTarget = FALSE as u32;
            }
        }
        if canSelectTarget == 0 {
            BtlController_EmitTwoReturnValues(
                B_COMM_TO_ENGINE,
                B_ACTION_EXEC_SCRIPT,
                ({
                    let gl_i43 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i43])
                }) as u16
                    | (gMultiUsePlayerCursor.get() as u16) << 8,
            );
            PlayerBufferExecCompleted();
        } else {
            {
                let gl_i44 = gActiveBattler.get();
                crate::gl!(gBattlerControllerFuncs[gl_i44] = Some(HandleInputChooseTarget))
            };
            if moveTarget as i32 & 18 != 0 {
                gMultiUsePlayerCursor.set(gActiveBattler.get());
            } else if gAbsentBattlerFlags.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                    [GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT)]
                != 0
            {
                gMultiUsePlayerCursor.set(GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT));
            } else {
                gMultiUsePlayerCursor.set(GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT));
            }
            (*gSprites.as_ptr())[{
                let gl_i45 = gMultiUsePlayerCursor.get();
                crate::gl!(gBattlerSpriteIds[gl_i45])
            }]
            .callback = Some(SpriteCB_ShowAsMoveTarget);
        }
    } else if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 || gPlayerDpadHoldFrames.get() > 59 {
        PlaySE(SE_SELECT);
        BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_EXEC_SCRIPT, 0xFFFF);
        PlayerBufferExecCompleted();
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_LEFT != 0 {
        if ({
            let gl_i46 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i46])
        }) as i32
            & 1
            != 0
        {
            MoveSelectionDestroyCursorAt({
                let gl_i47 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i47])
            });
            {
                let gl_i48 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i48] ^= 1)
            };
            PlaySE(SE_SELECT);
            MoveSelectionCreateCursorAt(
                {
                    let gl_i49 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i49])
                },
                0,
            );
            MoveSelectionDisplayPPNumber();
            MoveSelectionDisplayMoveType();
        }
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_RIGHT != 0 {
        if ({
            let gl_i50 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i50])
        }) as i32
            & 1
            == 0
            && (*(crate::gl!(&raw @at crate::battle_main::gMoveSelectionCursor;).cast_const())
                .cast::<CArray<u8, 4>>()
                .cast_mut())[gActiveBattler.get()] as i32
                ^ 1
                < gNumberOfMovesToChoose.get() as i32
        {
            MoveSelectionDestroyCursorAt({
                let gl_i51 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i51])
            });
            {
                let gl_i52 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i52] ^= 1)
            };
            PlaySE(SE_SELECT);
            MoveSelectionCreateCursorAt(
                {
                    let gl_i53 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i53])
                },
                0,
            );
            MoveSelectionDisplayPPNumber();
            MoveSelectionDisplayMoveType();
        }
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0 {
        if ({
            let gl_i54 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i54])
        }) as i32
            & 2
            != 0
        {
            MoveSelectionDestroyCursorAt({
                let gl_i55 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i55])
            });
            {
                let gl_i56 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i56] ^= 2)
            };
            PlaySE(SE_SELECT);
            MoveSelectionCreateCursorAt(
                {
                    let gl_i57 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i57])
                },
                0,
            );
            MoveSelectionDisplayPPNumber();
            MoveSelectionDisplayMoveType();
        }
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_DOWN != 0 {
        if ({
            let gl_i58 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i58])
        }) as i32
            & 2
            == 0
            && (*(crate::gl!(&raw @at crate::battle_main::gMoveSelectionCursor;).cast_const())
                .cast::<CArray<u8, 4>>()
                .cast_mut())[gActiveBattler.get()] as i32
                ^ 2
                < gNumberOfMovesToChoose.get() as i32
        {
            MoveSelectionDestroyCursorAt({
                let gl_i59 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i59])
            });
            {
                let gl_i60 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i60] ^= 2)
            };
            PlaySE(SE_SELECT);
            MoveSelectionCreateCursorAt(
                {
                    let gl_i61 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i61])
                },
                0,
            );
            MoveSelectionDisplayPPNumber();
            MoveSelectionDisplayMoveType();
        }
    } else if crate::gl!(gMain.newKeys) as i32 & SELECT_BUTTON != 0
        && gNumberOfMovesToChoose.get() > 1
        && gBattleTypeFlags.get() & BATTLE_TYPE_LINK == 0
    {
        MoveSelectionCreateCursorAt(
            {
                let gl_i62 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i62])
            },
            29,
        );
        if ({
            let gl_i63 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i63])
        }) != 0
        {
            gMultiUsePlayerCursor.set(0);
        } else {
            gMultiUsePlayerCursor.set(
                ({
                    let gl_i64 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i64])
                }) + 1,
            );
        }
        MoveSelectionCreateCursorAt(gMultiUsePlayerCursor.get(), 27);
        BattlePutTextOnWindow(
            (*(&raw const crate::data::battle_message::gText_BattleSwitchWhich)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
            B_WIN_SWITCH_PROMPT,
        );
        {
            let gl_i65 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i65] = Some(HandleMoveSwitching))
        };
    }
}
unsafe fn HandleMoveInputUnused() -> u32 {
    let mut var: u32 = 0;
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        var = 1;
    }
    if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        gBattle_BG0_X.set(0);
        gBattle_BG0_Y.set(320);
        var = 0xFF;
    }
    if crate::gl!(gMain.newKeys) as i32 & DPAD_LEFT != 0
        && (*(crate::gl!(&raw @at crate::battle_main::gMoveSelectionCursor;).cast_const())
            .cast::<CArray<u8, 4>>()
            .cast_mut())[gActiveBattler.get()] as i32
            & 1
            != 0
    {
        MoveSelectionDestroyCursorAt({
            let gl_i66 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i66])
        });
        {
            let gl_i67 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i67] ^= 1)
        };
        PlaySE(SE_SELECT);
        MoveSelectionCreateCursorAt(
            {
                let gl_i68 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i68])
            },
            0,
        );
    }
    if crate::gl!(gMain.newKeys) as i32 & DPAD_RIGHT != 0
        && (*(crate::gl!(&raw @at crate::battle_main::gMoveSelectionCursor;).cast_const())
            .cast::<CArray<u8, 4>>()
            .cast_mut())[gActiveBattler.get()] as i32
            & 1
            == 0
        && (*(crate::gl!(&raw @at crate::battle_main::gMoveSelectionCursor;).cast_const())
            .cast::<CArray<u8, 4>>()
            .cast_mut())[gActiveBattler.get()] as i32
            ^ 1
            < gNumberOfMovesToChoose.get() as i32
    {
        MoveSelectionDestroyCursorAt({
            let gl_i69 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i69])
        });
        {
            let gl_i70 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i70] ^= 1)
        };
        PlaySE(SE_SELECT);
        MoveSelectionCreateCursorAt(
            {
                let gl_i71 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i71])
            },
            0,
        );
    }
    if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0
        && (*(crate::gl!(&raw @at crate::battle_main::gMoveSelectionCursor;).cast_const())
            .cast::<CArray<u8, 4>>()
            .cast_mut())[gActiveBattler.get()] as i32
            & 2
            != 0
    {
        MoveSelectionDestroyCursorAt({
            let gl_i72 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i72])
        });
        {
            let gl_i73 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i73] ^= 2)
        };
        PlaySE(SE_SELECT);
        MoveSelectionCreateCursorAt(
            {
                let gl_i74 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i74])
            },
            0,
        );
    }
    if crate::gl!(gMain.newKeys) as i32 & DPAD_DOWN != 0
        && (*(crate::gl!(&raw @at crate::battle_main::gMoveSelectionCursor;).cast_const())
            .cast::<CArray<u8, 4>>()
            .cast_mut())[gActiveBattler.get()] as i32
            & 2
            == 0
        && (*(crate::gl!(&raw @at crate::battle_main::gMoveSelectionCursor;).cast_const())
            .cast::<CArray<u8, 4>>()
            .cast_mut())[gActiveBattler.get()] as i32
            ^ 2
            < gNumberOfMovesToChoose.get() as i32
    {
        MoveSelectionDestroyCursorAt({
            let gl_i75 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i75])
        });
        {
            let gl_i76 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i76] ^= 2)
        };
        PlaySE(SE_SELECT);
        MoveSelectionCreateCursorAt(
            {
                let gl_i77 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i77])
            },
            0,
        );
    }
    var
}
pub(crate) unsafe fn HandleMoveSwitching() {
    let mut perMovePPBonuses: CArray<u8, 4> = zeroed();
    let mut moveStruct: ChooseMoveStruct = zeroed();
    let mut totalPPBonuses: u8 = 0;
    if crate::gl!(gMain.newKeys) as i32 & 5 != 0 {
        PlaySE(SE_SELECT);
        if ({
            let gl_i78 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i78])
        }) != gMultiUsePlayerCursor.get()
        {
            let moveInfo: *mut ChooseMoveStruct =
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][4] as *mut ChooseMoveStruct;
            let mut i: i32 = (*moveInfo).moves[{
                let gl_i79 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i79])
            }] as i32;
            (*moveInfo).moves[{
                let gl_i80 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i80])
            }] = (*moveInfo).moves[gMultiUsePlayerCursor.get()];
            (*moveInfo).moves[gMultiUsePlayerCursor.get()] = i as u16;
            i = (*moveInfo).currentPP[{
                let gl_i81 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i81])
            }] as i32;
            (*moveInfo).currentPP[{
                let gl_i82 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i82])
            }] = (*moveInfo).currentPP[gMultiUsePlayerCursor.get()];
            (*moveInfo).currentPP[gMultiUsePlayerCursor.get()] = i as u8;
            i = (*moveInfo).maxPP[{
                let gl_i83 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i83])
            }] as i32;
            (*moveInfo).maxPP[{
                let gl_i84 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i84])
            }] = (*moveInfo).maxPP[gMultiUsePlayerCursor.get()];
            (*moveInfo).maxPP[gMultiUsePlayerCursor.get()] = i as u8;
            if (*gDisableStructs.as_ptr())[gActiveBattler.get()].mimickedMoves() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[{
                    let gl_i85 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i85])
                }]
                != 0
            {
                (*gDisableStructs.as_ptr())[gActiveBattler.get()].set_mimickedMoves(
                    (*gDisableStructs.as_ptr())[gActiveBattler.get()].mimickedMoves()
                        & !(gBitTable[{
                            let gl_i86 = gActiveBattler.get();
                            crate::gl!(gMoveSelectionCursor[gl_i86])
                        }] as u8),
                );
                (*gDisableStructs.as_ptr())[gActiveBattler.get()].set_mimickedMoves(
                    (*gDisableStructs.as_ptr())[gActiveBattler.get()].mimickedMoves()
                        | gBitTable[gMultiUsePlayerCursor.get()] as u8,
                );
            }
            MoveSelectionDisplayMoveNames();
            for i in 0..MAX_MON_MOVES {
                perMovePPBonuses[i] = shr_i32(
                    ({
                        let gl_i87 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i87].ppBonuses)
                    }) as i32
                        & shl_i32(3, i as u32 * 2),
                    i as u32 * 2,
                ) as u8;
            }
            totalPPBonuses = perMovePPBonuses[{
                let gl_i88 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i88])
            }];
            perMovePPBonuses[{
                let gl_i89 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i89])
            }] = perMovePPBonuses[gMultiUsePlayerCursor.get()];
            perMovePPBonuses[gMultiUsePlayerCursor.get()] = totalPPBonuses;
            totalPPBonuses = 0;
            for i in 0..MAX_MON_MOVES {
                totalPPBonuses |= shl_i32(perMovePPBonuses[i] as i32, i as u32 * 2) as u8;
            }
            {
                let gl_i90 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i90].ppBonuses = totalPPBonuses)
            };
            i = 0;
            while i < MAX_MON_MOVES {
                {
                    let gl_i91 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i91].moves[i] = (*moveInfo).moves[i])
                };
                {
                    let gl_i92 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i92].pp[i] = (*moveInfo).currentPP[i])
                };
                i += 1;
            }
            if ({
                let gl_i93 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i93].status2)
            }) & STATUS2_TRANSFORMED
                == 0
            {
                for i in 0..MAX_MON_MOVES {
                    moveStruct.moves[i] = GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())[{
                            let gl_i94 = gActiveBattler.get();
                            crate::gl!(gBattlerPartyIndexes[gl_i94])
                        }],
                        MON_DATA_MOVE1 + i,
                    ) as u16;
                    moveStruct.currentPP[i] = GetMonData2(
                        &raw mut (*gPlayerParty.as_ptr())[{
                            let gl_i95 = gActiveBattler.get();
                            crate::gl!(gBattlerPartyIndexes[gl_i95])
                        }],
                        MON_DATA_PP1 + i,
                    ) as u8;
                }
                totalPPBonuses = GetMonData2(
                    &raw mut (*gPlayerParty.as_ptr())[{
                        let gl_i96 = gActiveBattler.get();
                        crate::gl!(gBattlerPartyIndexes[gl_i96])
                    }],
                    MON_DATA_PP_BONUSES,
                ) as u8;
                for i in 0..MAX_MON_MOVES {
                    perMovePPBonuses[i] = shr_i32(
                        totalPPBonuses as i32 & shl_i32(3, i as u32 * 2),
                        i as u32 * 2,
                    ) as u8;
                }
                i = moveStruct.moves[{
                    let gl_i97 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i97])
                }] as i32;
                moveStruct.moves[{
                    let gl_i98 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i98])
                }] = moveStruct.moves[gMultiUsePlayerCursor.get()];
                moveStruct.moves[gMultiUsePlayerCursor.get()] = i as u16;
                i = moveStruct.currentPP[{
                    let gl_i99 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i99])
                }] as i32;
                moveStruct.currentPP[{
                    let gl_i100 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i100])
                }] = moveStruct.currentPP[gMultiUsePlayerCursor.get()];
                moveStruct.currentPP[gMultiUsePlayerCursor.get()] = i as u8;
                totalPPBonuses = perMovePPBonuses[{
                    let gl_i101 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i101])
                }];
                perMovePPBonuses[{
                    let gl_i102 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i102])
                }] = perMovePPBonuses[gMultiUsePlayerCursor.get()];
                perMovePPBonuses[gMultiUsePlayerCursor.get()] = totalPPBonuses;
                totalPPBonuses = 0;
                i = 0;
                while i < MAX_MON_MOVES {
                    totalPPBonuses |= shl_i32(perMovePPBonuses[i] as i32, i as u32 * 2) as u8;
                    i += 1;
                }
                for i in 0..MAX_MON_MOVES {
                    SetMonData(
                        &raw mut (*gPlayerParty.as_ptr())[{
                            let gl_i103 = gActiveBattler.get();
                            crate::gl!(gBattlerPartyIndexes[gl_i103])
                        }],
                        MON_DATA_MOVE1 + i,
                        &raw mut moveStruct.moves[i] as *mut c_void,
                    );
                    SetMonData(
                        &raw mut (*gPlayerParty.as_ptr())[{
                            let gl_i104 = gActiveBattler.get();
                            crate::gl!(gBattlerPartyIndexes[gl_i104])
                        }],
                        MON_DATA_PP1 + i,
                        &raw mut moveStruct.currentPP[i] as *mut c_void,
                    );
                }
                SetMonData(
                    &raw mut (*gPlayerParty.as_ptr())[{
                        let gl_i105 = gActiveBattler.get();
                        crate::gl!(gBattlerPartyIndexes[gl_i105])
                    }],
                    MON_DATA_PP_BONUSES,
                    &raw mut totalPPBonuses as *mut c_void,
                );
            }
        }
        {
            let gl_i106 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i106] = Some(HandleInputChooseMove))
        };
        {
            let gl_i107 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i107] = gMultiUsePlayerCursor.get())
        };
        MoveSelectionCreateCursorAt(
            {
                let gl_i108 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i108])
            },
            0,
        );
        MoveSelectionDisplayPPString();
        MoveSelectionDisplayPPNumber();
        MoveSelectionDisplayMoveType();
    } else if crate::gl!(gMain.newKeys) as i32 & 6 != 0 {
        PlaySE(SE_SELECT);
        MoveSelectionDestroyCursorAt(gMultiUsePlayerCursor.get());
        MoveSelectionCreateCursorAt(
            {
                let gl_i109 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i109])
            },
            0,
        );
        {
            let gl_i110 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i110] = Some(HandleInputChooseMove))
        };
        MoveSelectionDisplayPPString();
        MoveSelectionDisplayPPNumber();
        MoveSelectionDisplayMoveType();
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_LEFT != 0 {
        if gMultiUsePlayerCursor.get() as i32 & 1 != 0 {
            if gMultiUsePlayerCursor.get()
                == ({
                    let gl_i111 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i111])
                })
            {
                MoveSelectionCreateCursorAt(
                    {
                        let gl_i112 = gActiveBattler.get();
                        crate::gl!(gMoveSelectionCursor[gl_i112])
                    },
                    29,
                );
            } else {
                MoveSelectionDestroyCursorAt(gMultiUsePlayerCursor.get());
            }
            gMultiUsePlayerCursor.set(gMultiUsePlayerCursor.get() ^ (1));
            PlaySE(SE_SELECT);
            if gMultiUsePlayerCursor.get()
                == ({
                    let gl_i113 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i113])
                })
            {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor.get(), 0);
            } else {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor.get(), 27);
            }
        }
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_RIGHT != 0 {
        if gMultiUsePlayerCursor.get() as i32 & 1 == 0
            && gMultiUsePlayerCursor.get() as i32 ^ 1 < gNumberOfMovesToChoose.get() as i32
        {
            if gMultiUsePlayerCursor.get()
                == ({
                    let gl_i114 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i114])
                })
            {
                MoveSelectionCreateCursorAt(
                    {
                        let gl_i115 = gActiveBattler.get();
                        crate::gl!(gMoveSelectionCursor[gl_i115])
                    },
                    29,
                );
            } else {
                MoveSelectionDestroyCursorAt(gMultiUsePlayerCursor.get());
            }
            gMultiUsePlayerCursor.set(gMultiUsePlayerCursor.get() ^ (1));
            PlaySE(SE_SELECT);
            if gMultiUsePlayerCursor.get()
                == ({
                    let gl_i116 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i116])
                })
            {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor.get(), 0);
            } else {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor.get(), 27);
            }
        }
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0 {
        if gMultiUsePlayerCursor.get() as i32 & 2 != 0 {
            if gMultiUsePlayerCursor.get()
                == ({
                    let gl_i117 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i117])
                })
            {
                MoveSelectionCreateCursorAt(
                    {
                        let gl_i118 = gActiveBattler.get();
                        crate::gl!(gMoveSelectionCursor[gl_i118])
                    },
                    29,
                );
            } else {
                MoveSelectionDestroyCursorAt(gMultiUsePlayerCursor.get());
            }
            gMultiUsePlayerCursor.set(gMultiUsePlayerCursor.get() ^ (2));
            PlaySE(SE_SELECT);
            if gMultiUsePlayerCursor.get()
                == ({
                    let gl_i119 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i119])
                })
            {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor.get(), 0);
            } else {
                MoveSelectionCreateCursorAt(gMultiUsePlayerCursor.get(), 27);
            }
        }
    } else if crate::gl!(gMain.newKeys) as i32 & DPAD_DOWN != 0
        && gMultiUsePlayerCursor.get() as i32 & 2 == 0
        && gMultiUsePlayerCursor.get() as i32 ^ 2 < gNumberOfMovesToChoose.get() as i32
    {
        if gMultiUsePlayerCursor.get()
            == ({
                let gl_i120 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i120])
            })
        {
            MoveSelectionCreateCursorAt(
                {
                    let gl_i121 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i121])
                },
                29,
            );
        } else {
            MoveSelectionDestroyCursorAt(gMultiUsePlayerCursor.get());
        }
        gMultiUsePlayerCursor.set(gMultiUsePlayerCursor.get() ^ (2));
        PlaySE(SE_SELECT);
        if gMultiUsePlayerCursor.get()
            == ({
                let gl_i122 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i122])
            })
        {
            MoveSelectionCreateCursorAt(gMultiUsePlayerCursor.get(), 0);
        } else {
            MoveSelectionCreateCursorAt(gMultiUsePlayerCursor.get(), 27);
        }
    }
}
pub(crate) unsafe fn SetLinkBattleEndCallbacks() {
    if gWirelessCommType.get() == 0 {
        if gReceivedRemoteLinkPlayers.get() == 0 {
            m4aSongNumStop(SE_LOW_HEALTH);
            (*gMain.as_ptr()).set_inBattle(FALSE);
            crate::gl!(gMain.callback1 = crate::gl!(gPreBattleCallback1));
            SetMainCallback2(Some(CB2_InitEndLinkBattle));
            if gBattleOutcome.get() == B_OUTCOME_WON {
                TryPutLinkBattleTvShowOnAir();
            }
            FreeAllWindowBuffers();
        }
    } else {
        if IsLinkTaskFinished() != 0 {
            m4aSongNumStop(SE_LOW_HEALTH);
            (*gMain.as_ptr()).set_inBattle(FALSE);
            crate::gl!(gMain.callback1 = crate::gl!(gPreBattleCallback1));
            SetMainCallback2(Some(CB2_InitEndLinkBattle));
            if gBattleOutcome.get() == B_OUTCOME_WON {
                TryPutLinkBattleTvShowOnAir();
            }
            FreeAllWindowBuffers();
        }
    }
}
pub unsafe fn SetBattleEndCallbacks() {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
            if IsLinkTaskFinished() != 0 {
                if gWirelessCommType.get() == 0 {
                    SetCloseLinkCallback();
                } else {
                    SetLinkStandbyCallback();
                }
                {
                    let gl_i123 = gActiveBattler.get();
                    crate::gl!(gBattlerControllerFuncs[gl_i123] = Some(SetLinkBattleEndCallbacks))
                };
            }
        } else {
            m4aSongNumStop(SE_LOW_HEALTH);
            (*gMain.as_ptr()).set_inBattle(FALSE);
            crate::gl!(gMain.callback1 = crate::gl!(gPreBattleCallback1));
            SetMainCallback2(crate::gl!(gMain.savedCallback));
        }
    }
}
pub(crate) unsafe fn CompleteOnBattlerSpriteCallbackDummy() {
    if (*gSprites.as_ptr())[{
        let gl_i124 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i124])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnBankSpriteCallbackDummy2() {
    if (*gSprites.as_ptr())[{
        let gl_i125 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i125])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn FreeTrainerSpriteAfterSlide() {
    if (*gSprites.as_ptr())[{
        let gl_i126 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i126])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        BattleGfxSfxDummy3((*crate::gl!(gSaveBlock2Ptr)).playerGender);
        FreeSpriteOamMatrix(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i127 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i127])
            }],
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i128 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i128])
            }],
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn Intro_DelayAndEnd() {
    if ({
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .introEndDelay -= 1;
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .introEndDelay
    }) == 255
    {
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .introEndDelay = 0;
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn Intro_WaitForShinyAnimAndHealthbox() {
    let mut healthboxAnimDone: u8 = FALSE;
    if IsDoubleBattle() == 0
        || IsDoubleBattle() != 0 && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
    {
        if (*gSprites.as_ptr())[{
            let gl_i129 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i129])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        {
            healthboxAnimDone = TRUE;
        }
    } else {
        if (*gSprites.as_ptr())[{
            let gl_i130 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i130])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            && (*gSprites.as_ptr())[{
                let gl_i131 = gActiveBattler.get() as i32 ^ 2;
                crate::gl!(gHealthboxSpriteIds[gl_i131])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        {
            healthboxAnimDone = TRUE;
        }
    }
    if healthboxAnimDone != 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .finishedShinyMonAnim()
            != 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .finishedShinyMonAnim()
            != 0
    {
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_triedShinyMonAnim(FALSE);
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_finishedShinyMonAnim(FALSE);
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .set_triedShinyMonAnim(FALSE);
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .set_finishedShinyMonAnim(FALSE);
        FreeSpriteTilesByTag(ANIM_TAG_GOLD_STARS);
        FreeSpritePaletteByTag(ANIM_TAG_GOLD_STARS);
        HandleLowHpMusicChange(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i132 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i132])
            }],
            gActiveBattler.get(),
        );
        if IsDoubleBattle() != 0 {
            HandleLowHpMusicChange(
                &raw mut (*gPlayerParty.as_ptr())[{
                    let gl_i133 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gBattlerPartyIndexes[gl_i133])
                }],
                gActiveBattler.get() ^ 2,
            );
        }
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .introEndDelay = 3;
        {
            let gl_i134 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i134] = Some(Intro_DelayAndEnd))
        };
    }
}
pub(crate) unsafe fn Intro_TryShinyAnimShowHealthbox() {
    let mut bgmRestored: u32 = FALSE as u32;
    let mut battlerAnimsDone: u32 = FALSE as u32;
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .triedShinyMonAnim()
        == 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .ballAnimActive()
            == 0
    {
        TryShinyAnimation(
            gActiveBattler.get(),
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i135 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i135])
            }],
        );
    }
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get() as i32 ^ 2))
    .triedShinyMonAnim()
        == 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .ballAnimActive()
            == 0
    {
        TryShinyAnimation(
            gActiveBattler.get() ^ 2,
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i136 = gActiveBattler.get() as i32 ^ 2;
                crate::gl!(gBattlerPartyIndexes[gl_i136])
            }],
        );
    }
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .ballAnimActive()
        == 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .ballAnimActive()
            == 0
    {
        if (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .healthboxSlideInStarted()
            == 0
        {
            if IsDoubleBattle() != 0 && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI == 0 {
                UpdateHealthboxAttribute(
                    {
                        let gl_i137 = gActiveBattler.get() as i32 ^ 2;
                        crate::gl!(gHealthboxSpriteIds[gl_i137])
                    },
                    &raw mut (*gPlayerParty.as_ptr())[{
                        let gl_i138 = gActiveBattler.get() as i32 ^ 2;
                        crate::gl!(gBattlerPartyIndexes[gl_i138])
                    }],
                    HEALTHBOX_ALL,
                );
                StartHealthboxSlideIn(gActiveBattler.get() ^ 2);
                SetHealthboxSpriteVisible({
                    let gl_i139 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gHealthboxSpriteIds[gl_i139])
                });
            }
            UpdateHealthboxAttribute(
                {
                    let gl_i140 = gActiveBattler.get();
                    crate::gl!(gHealthboxSpriteIds[gl_i140])
                },
                &raw mut (*gPlayerParty.as_ptr())[{
                    let gl_i141 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i141])
                }],
                HEALTHBOX_ALL,
            );
            StartHealthboxSlideIn(gActiveBattler.get());
            SetHealthboxSpriteVisible({
                let gl_i142 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i142])
            });
        }
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_healthboxSlideInStarted(TRUE);
    }
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .waitForCry()
        == 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .healthboxSlideInStarted()
            != 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .waitForCry()
            == 0
        && IsCryPlayingOrClearCrySongs() == 0
    {
        if (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .bgmRestored()
            == 0
        {
            if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
                && gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0
            {
                m4aMPlayContinue(crate::gl!(&raw gMPlayInfo_BGM));
            } else {
                m4aMPlayVolumeControl(crate::gl!(&raw gMPlayInfo_BGM), TRACKS_ALL, 0x100);
            }
        }
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_bgmRestored(TRUE);
        bgmRestored = TRUE as u32;
    }
    if IsDoubleBattle() == 0
        || IsDoubleBattle() != 0 && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
    {
        if (*gSprites.as_ptr())[{
            let gl_i143 = gActiveBattler.get();
            crate::gl!(gBattleControllerData[gl_i143])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            && (*gSprites.as_ptr())[{
                let gl_i144 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i144])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        {
            battlerAnimsDone = TRUE as u32;
        }
    } else {
        if (*gSprites.as_ptr())[{
            let gl_i145 = gActiveBattler.get();
            crate::gl!(gBattleControllerData[gl_i145])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            && (*gSprites.as_ptr())[{
                let gl_i146 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i146])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            && (*gSprites.as_ptr())[{
                let gl_i147 = gActiveBattler.get() as i32 ^ 2;
                crate::gl!(gBattleControllerData[gl_i147])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            && (*gSprites.as_ptr())[{
                let gl_i148 = gActiveBattler.get() as i32 ^ 2;
                crate::gl!(gBattlerSpriteIds[gl_i148])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        {
            battlerAnimsDone = TRUE as u32;
        }
    }
    if bgmRestored != 0 && battlerAnimsDone != 0 {
        if IsDoubleBattle() != 0 && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI == 0 {
            DestroySprite(
                &raw mut (*gSprites.as_ptr())[{
                    let gl_i149 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gBattleControllerData[gl_i149])
                }],
            );
        }
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i150 = gActiveBattler.get();
                crate::gl!(gBattleControllerData[gl_i150])
            }],
        );
        (*(*crate::gl!(gBattleSpritesDataPtr)).animationData).set_introAnimActive(FALSE);
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_bgmRestored(FALSE);
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_healthboxSlideInStarted(FALSE);
        {
            let gl_i151 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i151] = Some(Intro_WaitForShinyAnimAndHealthbox))
        };
    }
}
pub(crate) unsafe fn SwitchIn_CleanShinyAnimShowSubstitute() {
    if (*gSprites.as_ptr())[{
        let gl_i152 = gActiveBattler.get();
        crate::gl!(gHealthboxSpriteIds[gl_i152])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .finishedShinyMonAnim()
            != 0
        && (*gSprites.as_ptr())[{
            let gl_i153 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i153])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        CopyBattleSpriteInvisibility(gActiveBattler.get());
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_triedShinyMonAnim(FALSE);
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_finishedShinyMonAnim(FALSE);
        FreeSpriteTilesByTag(ANIM_TAG_GOLD_STARS);
        FreeSpritePaletteByTag(ANIM_TAG_GOLD_STARS);
        if (*(*crate::gl!(gBattleSpritesDataPtr))
            .battlerData
            .at(gActiveBattler.get()))
        .behindSubstitute()
            != 0
        {
            InitAndLaunchSpecialAnimation(
                gActiveBattler.get(),
                gActiveBattler.get(),
                gActiveBattler.get(),
                B_ANIM_MON_TO_SUBSTITUTE,
            );
        }
        {
            let gl_i154 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i154] = Some(SwitchIn_HandleSoundAndEnd))
        };
    }
}
pub(crate) unsafe fn SwitchIn_HandleSoundAndEnd() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .specialAnimActive()
        == 0
        && IsCryPlayingOrClearCrySongs() == 0
    {
        m4aMPlayVolumeControl(crate::gl!(&raw gMPlayInfo_BGM), TRACKS_ALL, 0x100);
        HandleLowHpMusicChange(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i155 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i155])
            }],
            gActiveBattler.get(),
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn SwitchIn_TryShinyAnimShowHealthbox() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .triedShinyMonAnim()
        == 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .ballAnimActive()
            == 0
    {
        TryShinyAnimation(
            gActiveBattler.get(),
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i156 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i156])
            }],
        );
    }
    if (*gSprites.as_ptr())[{
        let gl_i157 = gActiveBattler.get();
        crate::gl!(gBattleControllerData[gl_i157])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .ballAnimActive()
            == 0
    {
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i158 = gActiveBattler.get();
                crate::gl!(gBattleControllerData[gl_i158])
            }],
        );
        UpdateHealthboxAttribute(
            {
                let gl_i159 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i159])
            },
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i160 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i160])
            }],
            HEALTHBOX_ALL,
        );
        StartHealthboxSlideIn(gActiveBattler.get());
        SetHealthboxSpriteVisible({
            let gl_i161 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i161])
        });
        {
            let gl_i162 = gActiveBattler.get();
            crate::gl!(
                gBattlerControllerFuncs[gl_i162] = Some(SwitchIn_CleanShinyAnimShowSubstitute)
            )
        };
    }
}
pub unsafe fn Task_PlayerController_RestoreBgmAfterCry(taskId: u8) {
    if IsCryPlayingOrClearCrySongs() == 0 {
        m4aMPlayVolumeControl(crate::gl!(&raw gMPlayInfo_BGM), TRACKS_ALL, 0x100);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn CompleteOnHealthbarDone() {
    let hpValue: i16 = MoveBattleBar(
        gActiveBattler.get(),
        {
            let gl_i163 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i163])
        },
        HEALTH_BAR,
        0,
    ) as i16;
    SetHealthboxSpriteVisible({
        let gl_i164 = gActiveBattler.get();
        crate::gl!(gHealthboxSpriteIds[gl_i164])
    });
    if hpValue != -1 {
        UpdateHpTextInHealthbox(
            {
                let gl_i165 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i165])
            },
            hpValue,
            HP_CURRENT,
        );
    } else {
        HandleLowHpMusicChange(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i166 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i166])
            }],
            gActiveBattler.get(),
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnInactiveTextPrinter() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn Task_GiveExpToMon(taskId: u8) {
    let monId: u32 = task_get(taskId, tExpTask_monId) as u8 as u32;
    let battler: u8 = task_get(taskId, tExpTask_battler) as u8;
    let mut gainedExp: i16 = task_get(taskId, tExpTask_gainedExp);
    if IsDoubleBattle() == TRUE || monId != crate::gl!(gBattlerPartyIndexes[battler]) as u32 {
        let mon: *mut Pokemon = crate::gl!(&raw gPlayerParty[monId]);
        let species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
        let level: u8 = GetMonData2(mon, MON_DATA_LEVEL) as u8;
        let mut currExp: u32 = GetMonData2(mon, MON_DATA_EXP);
        let mut nextLvlExp: u32 = (*(&raw const crate::data::pokemon::gExperienceTables)
            .cast::<CArray<CArray<u32, 101>, 0>>())
            [(*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [species]
                .growthRate][level as i32 + 1];
        if currExp + gainedExp as u32 >= nextLvlExp {
            SetMonData(mon, MON_DATA_EXP, &raw mut nextLvlExp as *mut c_void);
            CalculateMonStats(mon);
            gainedExp -= nextLvlExp as i16 - currExp as i16;
            let savedActiveBattler: u8 = gActiveBattler.get();
            gActiveBattler.set(battler);
            BtlController_EmitTwoReturnValues(
                B_COMM_TO_ENGINE,
                RET_VALUE_LEVELED_UP,
                gainedExp as u16,
            );
            gActiveBattler.set(savedActiveBattler);
            if IsDoubleBattle() == TRUE
                && (monId as u16 == crate::gl!(gBattlerPartyIndexes[battler])
                    || monId as u16
                        == ({
                            let gl_i167 = battler as i32 ^ 2;
                            crate::gl!(gBattlerPartyIndexes[gl_i167])
                        }))
            {
                task_set_func(taskId, Some(Task_LaunchLvlUpAnim));
            } else {
                task_set_func(taskId, Some(DestroyExpTaskAndCompleteOnInactiveTextPrinter));
            }
        } else {
            currExp += gainedExp as u32;
            SetMonData(mon, MON_DATA_EXP, &raw mut currExp as *mut c_void);
            crate::gl!(gBattlerControllerFuncs[battler] = Some(CompleteOnInactiveTextPrinter));
            DestroyTask(taskId);
        }
    } else {
        task_set_func(taskId, Some(Task_PrepareToGiveExpWithExpBar));
    }
}
pub(crate) unsafe fn Task_PrepareToGiveExpWithExpBar(taskId: u8) {
    let monIndex: u8 = task_get(taskId, tExpTask_monId) as u8;
    let gainedExp: i32 = task_get(taskId, tExpTask_gainedExp) as i32;
    let battler: u8 = task_get(taskId, tExpTask_battler) as u8;
    let mon: *mut Pokemon = crate::gl!(&raw gPlayerParty[monIndex]);
    let level: u8 = GetMonData2(mon, MON_DATA_LEVEL) as u8;
    let species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
    let mut exp: u32 = GetMonData2(mon, MON_DATA_EXP);
    let currLvlExp: u32 = (*(&raw const crate::data::pokemon::gExperienceTables)
        .cast::<CArray<CArray<u32, 101>, 0>>())[(*(&raw const crate::data::pokemon::gSpeciesInfo)
        .cast::<CArray<SpeciesInfo, 0>>())[species]
        .growthRate][level];
    exp -= currLvlExp;
    let expToNextLvl: u32 = (*(&raw const crate::data::pokemon::gExperienceTables)
        .cast::<CArray<CArray<u32, 101>, 0>>())[(*(&raw const crate::data::pokemon::gSpeciesInfo)
        .cast::<CArray<SpeciesInfo, 0>>())[species]
        .growthRate][level as i32 + 1]
        - currLvlExp;
    SetBattleBarStruct(
        battler,
        crate::gl!(gHealthboxSpriteIds[battler]),
        expToNextLvl as i32,
        exp as i32,
        -gainedExp,
    );
    PlaySE(SE_EXP);
    task_set_func(taskId, Some(Task_GiveExpWithExpBar));
}
pub(crate) unsafe fn Task_GiveExpWithExpBar(taskId: u8) {
    if task_get(taskId, tExpTask_frames) < 13 {
        task_set(
            taskId,
            tExpTask_frames,
            task_get(taskId, tExpTask_frames) + 1,
        );
    } else {
        let monId: u8 = task_get(taskId, tExpTask_monId) as u8;
        let mut gainedExp: i16 = task_get(taskId, tExpTask_gainedExp);
        let battler: u8 = task_get(taskId, tExpTask_battler) as u8;
        let newExpPoints: i16 = MoveBattleBar(
            battler,
            crate::gl!(gHealthboxSpriteIds[battler]),
            EXP_BAR,
            0,
        ) as i16;
        SetHealthboxSpriteVisible(crate::gl!(gHealthboxSpriteIds[battler]));
        if newExpPoints == -1 {
            m4aSongNumStop(SE_EXP);
            let level: u8 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_LEVEL) as u8;
            let mut currExp: i32 =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_EXP) as i32;
            let species: u16 =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPECIES) as u16;
            let mut expOnNextLvl: i32 = (*(&raw const crate::data::pokemon::gExperienceTables)
                .cast::<CArray<CArray<u32, 101>, 0>>())
                [(*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .growthRate][level as i32 + 1] as i32;
            if currExp + gainedExp as i32 >= expOnNextLvl {
                SetMonData(
                    crate::gl!(&raw gPlayerParty[monId]),
                    MON_DATA_EXP,
                    &raw mut expOnNextLvl as *mut c_void,
                );
                CalculateMonStats(crate::gl!(&raw gPlayerParty[monId]));
                gainedExp -= expOnNextLvl as i16 - currExp as i16;
                let savedActiveBattler: u8 = gActiveBattler.get();
                gActiveBattler.set(battler);
                BtlController_EmitTwoReturnValues(
                    B_COMM_TO_ENGINE,
                    RET_VALUE_LEVELED_UP,
                    gainedExp as u16,
                );
                gActiveBattler.set(savedActiveBattler);
                task_set_func(taskId, Some(Task_LaunchLvlUpAnim));
            } else {
                currExp += gainedExp as i32;
                SetMonData(
                    crate::gl!(&raw gPlayerParty[monId]),
                    MON_DATA_EXP,
                    &raw mut currExp as *mut c_void,
                );
                crate::gl!(gBattlerControllerFuncs[battler] = Some(CompleteOnInactiveTextPrinter));
                DestroyTask(taskId);
            }
        }
    }
}
pub(crate) unsafe fn Task_LaunchLvlUpAnim(taskId: u8) {
    let mut battler: u8 = task_get(taskId, tExpTask_battler) as u8;
    let monIndex: u8 = task_get(taskId, tExpTask_monId) as u8;
    if IsDoubleBattle() == TRUE
        && monIndex as u16
            == ({
                let gl_i168 = battler as i32 ^ 2;
                crate::gl!(gBattlerPartyIndexes[gl_i168])
            })
    {
        battler ^= BIT_FLANK;
    }
    InitAndLaunchSpecialAnimation(battler, battler, battler, B_ANIM_LVL_UP);
    task_set_func(taskId, Some(Task_UpdateLvlInHealthbox));
}
pub(crate) unsafe fn Task_UpdateLvlInHealthbox(taskId: u8) {
    let battler: u8 = task_get(taskId, tExpTask_battler) as u8;
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(battler))
    .specialAnimActive()
        == 0
    {
        let monIndex: u8 = task_get(taskId, tExpTask_monId) as u8;
        GetMonData2(crate::gl!(&raw gPlayerParty[monIndex]), MON_DATA_LEVEL);
        if IsDoubleBattle() == TRUE
            && monIndex as u16
                == ({
                    let gl_i169 = battler as i32 ^ 2;
                    crate::gl!(gBattlerPartyIndexes[gl_i169])
                })
        {
            UpdateHealthboxAttribute(
                {
                    let gl_i170 = battler as i32 ^ 2;
                    crate::gl!(gHealthboxSpriteIds[gl_i170])
                },
                crate::gl!(&raw gPlayerParty[monIndex]),
                HEALTHBOX_ALL,
            );
        } else {
            UpdateHealthboxAttribute(
                crate::gl!(gHealthboxSpriteIds[battler]),
                crate::gl!(&raw gPlayerParty[monIndex]),
                HEALTHBOX_ALL,
            );
        }
        task_set_func(taskId, Some(DestroyExpTaskAndCompleteOnInactiveTextPrinter));
    }
}
pub(crate) unsafe fn DestroyExpTaskAndCompleteOnInactiveTextPrinter(taskId: u8) {
    let monIndex: u8 = task_get(taskId, tExpTask_monId) as u8;
    GetMonData2(crate::gl!(&raw gPlayerParty[monIndex]), MON_DATA_LEVEL);
    let battler: u8 = task_get(taskId, tExpTask_battler) as u8;
    crate::gl!(gBattlerControllerFuncs[battler] = Some(CompleteOnInactiveTextPrinter));
    DestroyTask(taskId);
}
pub(crate) unsafe fn FreeMonSpriteAfterFaintAnim() {
    if (*gSprites.as_ptr())[{
        let gl_i171 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i171])
    }]
    .y as i32
        + (*gSprites.as_ptr())[{
            let gl_i172 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i172])
        }]
        .y2 as i32
        > DISPLAY_HEIGHT as i32
    {
        let species: u16 = GetMonData2(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i173 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i173])
            }],
            MON_DATA_SPECIES,
        ) as u16;
        BattleGfxSfxDummy2(species);
        FreeOamMatrix(
            (*gSprites.as_ptr())[{
                let gl_i174 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i174])
            }]
            .oam
            .matrixNum() as u8,
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i175 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i175])
            }],
        );
        SetHealthboxSpriteInvisible({
            let gl_i176 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i176])
        });
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn FreeMonSpriteAfterSwitchOutAnim() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .specialAnimActive()
        == 0
    {
        FreeSpriteOamMatrix(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i177 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i177])
            }],
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i178 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i178])
            }],
        );
        SetHealthboxSpriteInvisible({
            let gl_i179 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i179])
        });
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnInactiveTextPrinter2() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn OpenPartyMenuToChooseMon() {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        {
            let gl_i180 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i180] = Some(WaitForMonSelection))
        };
        let caseId: u8 = task_get(
            {
                let gl_i181 = gActiveBattler.get();
                crate::gl!(gBattleControllerData[gl_i181])
            },
            0,
        ) as u8;
        DestroyTask({
            let gl_i182 = gActiveBattler.get();
            crate::gl!(gBattleControllerData[gl_i182])
        });
        FreeAllWindowBuffers();
        OpenPartyMenuInBattle(caseId);
    }
}
pub(crate) unsafe fn WaitForMonSelection() {
    if crate::gl!(gMain.callback2) == Some(BattleMainCB2 as unsafe fn())
        && (*gPaletteFade.as_ptr()).active() == 0
    {
        if gPartyMenuUseExitCallback.get() == TRUE {
            BtlController_EmitChosenMonReturnValue(
                B_COMM_TO_ENGINE,
                gSelectedMonPartyId.get(),
                (*gBattlePartyCurrentOrder.as_ptr()).as_mut_ptr(),
            );
        } else {
            BtlController_EmitChosenMonReturnValue(B_COMM_TO_ENGINE, PARTY_SIZE as u8, null_mut());
        }
        if ({
            let gl_i183 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i183][1])
        }) as i32
            & 0xF
            == 1
        {
            PrintLinkStandbyMsg();
        }
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn OpenBagAndChooseItem() {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        {
            let gl_i184 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i184] = Some(CompleteWhenChoseItem))
        };
        ReshowBattleScreenDummy();
        FreeAllWindowBuffers();
        CB2_BagMenuFromBattle();
    }
}
pub(crate) unsafe fn CompleteWhenChoseItem() {
    if crate::gl!(gMain.callback2) == Some(BattleMainCB2 as unsafe fn())
        && (*gPaletteFade.as_ptr()).active() == 0
    {
        BtlController_EmitOneReturnValue(B_COMM_TO_ENGINE, gSpecialVar_ItemId.get());
        PlayerBufferExecCompleted();
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
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn DoHitAnimBlinkSpriteEffect() {
    let spriteId: u8 = {
        let gl_i185 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i185])
    };
    if crate::gl!(gSprites[spriteId].data[1]) == 32 {
        crate::gl!(gSprites[spriteId].data[1] = 0);
        (*gSprites.as_ptr())[spriteId].set_invisible(FALSE as u16);
        gDoingBattleAnim.set(FALSE);
        PlayerBufferExecCompleted();
    } else {
        if crate::gl!(gSprites[spriteId].data[1]) % 4 == 0 {
            (*gSprites.as_ptr())[spriteId]
                .set_invisible((*gSprites.as_ptr())[spriteId].invisible() ^ 1);
        }
        crate::gl!(gSprites[spriteId].data[1] += 1);
    }
}
pub(crate) unsafe fn PlayerHandleYesNoInput() {
    if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0 && gMultiUsePlayerCursor.get() != 0 {
        PlaySE(SE_SELECT);
        BattleDestroyYesNoCursorAt(gMultiUsePlayerCursor.get());
        gMultiUsePlayerCursor.set(0);
        BattleCreateYesNoCursorAt(0);
    }
    if crate::gl!(gMain.newKeys) as i32 & DPAD_DOWN != 0 && gMultiUsePlayerCursor.get() == 0 {
        PlaySE(SE_SELECT);
        BattleDestroyYesNoCursorAt(gMultiUsePlayerCursor.get());
        gMultiUsePlayerCursor.set(1);
        BattleCreateYesNoCursorAt(1);
    }
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
        PlaySE(SE_SELECT);
        if gMultiUsePlayerCursor.get() != 0 {
            BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_UNK_14, 0);
        } else {
            BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_NOTHING_FAINTED, 0);
        }
        PlayerBufferExecCompleted();
    }
    if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
        HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
        PlaySE(SE_SELECT);
        PlayerBufferExecCompleted();
    }
}
unsafe fn MoveSelectionDisplayMoveNames() {
    let moveInfo: *mut ChooseMoveStruct =
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][4] as *mut ChooseMoveStruct;
    gNumberOfMovesToChoose.set(0);
    for i in 0..MAX_MON_MOVES {
        MoveSelectionDestroyCursorAt(i as u8);
        StringCopy(
            (*gDisplayedStringBattle.as_ptr()).as_mut_ptr(),
            (*(&raw const crate::data::data_tables::gMoveNames)
                .cast::<CArray<CArray<u8, 13>, 355>>())[(*moveInfo).moves[i]]
                .as_ptr()
                .cast_mut(),
        );
        BattlePutTextOnWindow(
            (*gDisplayedStringBattle.as_ptr()).as_mut_ptr(),
            i as u8 + B_WIN_MOVE_NAME_1,
        );
        if (*moveInfo).moves[i] != MOVE_NONE {
            gNumberOfMovesToChoose.set(gNumberOfMovesToChoose.get() + (1));
        }
    }
}
unsafe fn MoveSelectionDisplayPPString() {
    StringCopy(
        (*gDisplayedStringBattle.as_ptr()).as_mut_ptr(),
        (*(&raw const crate::data::battle_message::gText_MoveInterfacePP).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    BattlePutTextOnWindow((*gDisplayedStringBattle.as_ptr()).as_mut_ptr(), B_WIN_PP);
}
unsafe fn MoveSelectionDisplayPPNumber() {
    if ({
        let gl_i186 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i186][2])
    }) == TRUE
    {
        return;
    }
    SetPPNumbersPaletteInMoveSelection();
    let moveInfo: *mut ChooseMoveStruct =
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][4] as *mut ChooseMoveStruct;
    let mut txtPtr: *mut u8 = ConvertIntToDecimalStringN(
        (*gDisplayedStringBattle.as_ptr()).as_mut_ptr(),
        (*moveInfo).currentPP[{
            let gl_i187 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i187])
        }] as i32,
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
        (*moveInfo).maxPP[{
            let gl_i188 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i188])
        }] as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        2,
    );
    BattlePutTextOnWindow(
        (*gDisplayedStringBattle.as_ptr()).as_mut_ptr(),
        B_WIN_PP_REMAINING,
    );
}
unsafe fn MoveSelectionDisplayMoveType() {
    let moveInfo: *mut ChooseMoveStruct =
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][4] as *mut ChooseMoveStruct;
    let mut txtPtr: *mut u8 = StringCopy(
        (*gDisplayedStringBattle.as_ptr()).as_mut_ptr(),
        (*(&raw const crate::data::battle_message::gText_MoveInterfaceType)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut(),
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
        (*(&raw const crate::data::battle_main::gTypeNames).cast::<CArray<CArray<u8, 7>, 18>>())
            [(*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [(*moveInfo).moves[{
                    let gl_i189 = gActiveBattler.get();
                    crate::gl!(gMoveSelectionCursor[gl_i189])
                }]]
            .r#type]
            .as_ptr()
            .cast_mut(),
    );
    BattlePutTextOnWindow(
        (*gDisplayedStringBattle.as_ptr()).as_mut_ptr(),
        B_WIN_MOVE_TYPE,
    );
}
unsafe fn MoveSelectionCreateCursorAt(cursorPosition: u8, baseTileNum: u8) {
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
unsafe fn MoveSelectionDestroyCursorAt(cursorPosition: u8) {
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
pub unsafe fn ActionSelectionCreateCursorAt(cursorPosition: u8, baseTileNum: u8) {
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
pub unsafe fn ActionSelectionDestroyCursorAt(cursorPosition: u8) {
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
pub unsafe fn CB2_SetUpReshowBattleScreenAfterMenu() {
    SetMainCallback2(Some(ReshowBattleScreenAfterMenu));
}
pub unsafe fn CB2_SetUpReshowBattleScreenAfterMenu2() {
    SetMainCallback2(Some(ReshowBattleScreenAfterMenu));
}
pub(crate) unsafe fn CompleteOnFinishedStatusAnimation() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .statusAnimActive()
        == 0
    {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnFinishedBattleAnimation() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .animFromTableActive()
        == 0
    {
        PlayerBufferExecCompleted();
    }
}
unsafe fn PrintLinkStandbyMsg() {
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        gBattle_BG0_X.set(0);
        gBattle_BG0_Y.set(0);
        BattlePutTextOnWindow(
            (*(&raw const crate::data::battle_message::gText_LinkStandby).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            B_WIN_MSG,
        );
    }
}
pub(crate) unsafe fn PlayerHandleGetMonData() {
    let mut monData: CArray<u8, 256> = zeroed();
    let mut size: u32 = 0;
    let mut monToCheck: u8 = 0;
    if ({
        let gl_i190 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i190][2])
    }) == 0
    {
        size += CopyPlayerMonData(
            ({
                let gl_i191 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i191])
            }) as u8,
            monData.as_mut_ptr(),
        );
    } else {
        monToCheck = {
            let gl_i192 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i192][2])
        };
        for i in 0..PARTY_SIZE {
            if monToCheck as i32 & 1 != 0 {
                size += CopyPlayerMonData(i as u8, monData.as_mut_ptr().at(size));
            }
            monToCheck >>= 1;
        }
    }
    BtlController_EmitDataTransfer(
        B_COMM_TO_ENGINE,
        size as u16,
        monData.as_mut_ptr() as *mut c_void,
    );
    PlayerBufferExecCompleted();
}
unsafe fn CopyPlayerMonData(monId: u8, dst: *mut u8) -> u32 {
    let mut battleMon: BattlePokemon = zeroed();
    let mut moveData: MovePPInfo = zeroed();
    let mut nickname: CArray<u8, 20> = zeroed();
    let mut src: *mut u8 = null_mut();
    let mut data16: i16 = 0;
    let mut data32: u32 = 0;
    let mut size: i32 = 0;
    match {
        let gl_i193 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i193][1])
    } {
        REQUEST_ALL_BATTLE => {
            battleMon.species =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPECIES) as u16;
            battleMon.item =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HELD_ITEM) as u16;
            for size in 0..MAX_MON_MOVES {
                battleMon.moves[size] =
                    GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MOVE1 + size) as u16;
                battleMon.pp[size] =
                    GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PP1 + size) as u8;
            }
            battleMon.ppBonuses =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PP_BONUSES) as u8;
            battleMon.friendship =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_FRIENDSHIP) as u8;
            battleMon.experience = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_EXP);
            battleMon.set_hpIV(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP_IV,
            ));
            battleMon.set_attackIV(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK_IV,
            ));
            battleMon.set_defenseIV(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF_IV,
            ));
            battleMon.set_speedIV(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED_IV,
            ));
            battleMon.set_spAttackIV(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK_IV,
            ));
            battleMon.set_spDefenseIV(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF_IV,
            ));
            battleMon.personality =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PERSONALITY);
            battleMon.status1 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_STATUS);
            battleMon.level =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_LEVEL) as u8;
            battleMon.hp = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HP) as u16;
            battleMon.maxHP =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MAX_HP) as u16;
            battleMon.attack =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_ATK) as u16;
            battleMon.defense =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_DEF) as u16;
            battleMon.speed =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPEED) as u16;
            battleMon.spAttack =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPATK) as u16;
            battleMon.spDefense =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPDEF) as u16;
            battleMon.set_isEgg(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_IS_EGG,
            ));
            battleMon.set_abilityNum(GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ABILITY_NUM,
            ));
            battleMon.otId = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_OT_ID);
            GetMonData3(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_NICKNAME,
                nickname.as_mut_ptr(),
            );
            StringCopy_Nickname(battleMon.nickname.as_mut_ptr(), nickname.as_mut_ptr());
            GetMonData3(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_OT_NAME,
                battleMon.otName.as_mut_ptr(),
            );
            src = &raw mut battleMon as *mut u8;
            for size in 0..88i32 {
                *dst.at(size) = *src.at(size);
            }
        }
        REQUEST_SPECIES_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPECIES) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_HELDITEM_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HELD_ITEM) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_MOVES_PP_BATTLE => {
            for size in 0..MAX_MON_MOVES {
                moveData.moves[size] =
                    GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MOVE1 + size) as u16;
                moveData.pp[size] =
                    GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PP1 + size) as u8;
            }
            moveData.ppBonuses =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PP_BONUSES) as u8;
            src = &raw mut moveData as *mut u8;
            for size in 0..16i32 {
                *dst.at(size) = *src.at(size);
            }
        }
        4 | REQUEST_MOVE2_BATTLE | REQUEST_MOVE3_BATTLE | REQUEST_MOVE4_BATTLE => {
            data16 = GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MOVE1
                    + ({
                        let gl_i194 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i194][1])
                    }) as i32
                    - REQUEST_MOVE1_BATTLE,
            ) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_PP_DATA_BATTLE => {
            size = 0;
            while size < MAX_MON_MOVES {
                *dst.at(size) =
                    GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PP1 + size) as u8;
                size += 1;
            }
            *dst.at(size) =
                GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PP_BONUSES) as u8;
            size += 1;
        }
        REQUEST_PPMOVE1_BATTLE
        | REQUEST_PPMOVE2_BATTLE
        | REQUEST_PPMOVE3_BATTLE
        | REQUEST_PPMOVE4_BATTLE => {
            *dst = GetMonData2(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP1
                    + ({
                        let gl_i195 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i195][1])
                    }) as i32
                    - REQUEST_PPMOVE1_BATTLE as i32,
            ) as u8;
            size = 1;
        }
        REQUEST_OTID_BATTLE => {
            data32 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_OT_ID);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_EXP_BATTLE => {
            data32 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_EXP);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_HP_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HP_EV) as u8;
            size = 1;
        }
        REQUEST_ATK_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_ATK_EV) as u8;
            size = 1;
        }
        REQUEST_DEF_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_DEF_EV) as u8;
            size = 1;
        }
        REQUEST_SPEED_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPEED_EV) as u8;
            size = 1;
        }
        REQUEST_SPATK_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPATK_EV) as u8;
            size = 1;
        }
        REQUEST_SPDEF_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPDEF_EV) as u8;
            size = 1;
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_FRIENDSHIP) as u8;
            size = 1;
        }
        REQUEST_POKERUS_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_POKERUS) as u8;
            size = 1;
        }
        REQUEST_MET_LOCATION_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MET_LOCATION) as u8;
            size = 1;
        }
        REQUEST_MET_LEVEL_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MET_LEVEL) as u8;
            size = 1;
        }
        REQUEST_MET_GAME_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MET_GAME) as u8;
            size = 1;
        }
        REQUEST_POKEBALL_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_POKEBALL) as u8;
            size = 1;
        }
        REQUEST_ALL_IVS_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HP_IV) as u8;
            *dst.at(1) = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_ATK_IV) as u8;
            *dst.at(2) = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_DEF_IV) as u8;
            *dst.at(3) = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPEED_IV) as u8;
            *dst.at(4) = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPATK_IV) as u8;
            *dst.at(5) = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPDEF_IV) as u8;
            size = 6;
        }
        REQUEST_HP_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HP_IV) as u8;
            size = 1;
        }
        REQUEST_ATK_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_ATK_IV) as u8;
            size = 1;
        }
        REQUEST_DEF_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_DEF_IV) as u8;
            size = 1;
        }
        REQUEST_SPEED_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPEED_IV) as u8;
            size = 1;
        }
        REQUEST_SPATK_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPATK_IV) as u8;
            size = 1;
        }
        REQUEST_SPDEF_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPDEF_IV) as u8;
            size = 1;
        }
        REQUEST_PERSONALITY_BATTLE => {
            data32 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_PERSONALITY);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            *dst.at(3) = ((data32 & 0xFF000000) >> 24) as u8;
            size = 4;
        }
        REQUEST_CHECKSUM_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_CHECKSUM) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_STATUS_BATTLE => {
            data32 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_STATUS);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            *dst.at(3) = ((data32 & 0xFF000000) >> 24) as u8;
            size = 4;
        }
        REQUEST_LEVEL_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_LEVEL) as u8;
            size = 1;
        }
        REQUEST_HP_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_HP) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_MAX_HP_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_MAX_HP) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_ATK_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_ATK) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_DEF_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_DEF) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPEED_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPEED) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPATK_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPATK) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPDEF_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPDEF) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_COOL_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_COOL) as u8;
            size = 1;
        }
        REQUEST_BEAUTY_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_BEAUTY) as u8;
            size = 1;
        }
        REQUEST_CUTE_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_CUTE) as u8;
            size = 1;
        }
        REQUEST_SMART_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SMART) as u8;
            size = 1;
        }
        REQUEST_TOUGH_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_TOUGH) as u8;
            size = 1;
        }
        REQUEST_SHEEN_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SHEEN) as u8;
            size = 1;
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_COOL_RIBBON) as u8;
            size = 1;
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_BEAUTY_RIBBON) as u8;
            size = 1;
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_CUTE_RIBBON) as u8;
            size = 1;
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SMART_RIBBON) as u8;
            size = 1;
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_TOUGH_RIBBON) as u8;
            size = 1;
        }
        _ => {}
    }
    size as u32
}
pub unsafe fn PlayerHandleGetRawMonData() {
    let mut battleMon: BattlePokemon = zeroed();
    let src: *mut u8 = (&raw mut (*gPlayerParty.as_ptr())[{
        let gl_i196 = gActiveBattler.get();
        crate::gl!(gBattlerPartyIndexes[gl_i196])
    }] as *mut u8)
        .at({
            let gl_i197 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i197][1])
        });
    let dst: *mut u8 = (&raw mut battleMon as *mut u8).at({
        let gl_i198 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i198][1])
    });
    for i in 0..({
        let gl_i199 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i199][2])
    }) {
        *dst.at(i) = *src.at(i);
    }
    BtlController_EmitDataTransfer(
        B_COMM_TO_ENGINE,
        ({
            let gl_i200 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i200][2])
        }) as u16,
        dst as *mut c_void,
    );
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleSetMonData() {
    let mut monToCheck: u8 = 0;
    if ({
        let gl_i201 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i201][2])
    }) == 0
    {
        SetPlayerMonData(
            ({
                let gl_i202 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i202])
            }) as u8,
        );
    } else {
        monToCheck = {
            let gl_i203 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i203][2])
        };
        for i in 0..(PARTY_SIZE as u8) {
            if monToCheck as i32 & 1 != 0 {
                SetPlayerMonData(i);
            }
            monToCheck >>= 1;
        }
    }
    PlayerBufferExecCompleted();
}
unsafe fn SetPlayerMonData(monId: u8) {
    let battlePokemon: *mut BattlePokemon =
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][3] as *mut BattlePokemon;
    let moveData: *mut MovePPInfo =
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][3] as *mut MovePPInfo;
    match {
        let gl_i204 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i204][1])
    } {
        REQUEST_ALL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPECIES,
                &raw mut (*battlePokemon).species as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HELD_ITEM,
                &raw mut (*battlePokemon).item as *mut c_void,
            );
            for i in 0..MAX_MON_MOVES {
                SetMonData(
                    crate::gl!(&raw gPlayerParty[monId]),
                    MON_DATA_MOVE1 + i,
                    &raw mut (*battlePokemon).moves[i] as *mut c_void,
                );
                SetMonData(
                    crate::gl!(&raw gPlayerParty[monId]),
                    MON_DATA_PP1 + i,
                    &raw mut (*battlePokemon).pp[i] as *mut c_void,
                );
            }
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP_BONUSES,
                &raw mut (*battlePokemon).ppBonuses as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_FRIENDSHIP,
                &raw mut (*battlePokemon).friendship as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_EXP,
                &raw mut (*battlePokemon).experience as *mut c_void,
            );
            let mut iv: u8 = (*battlePokemon).hpIV() as u8;
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).attackIV() as u8;
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).defenseIV() as u8;
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).speedIV() as u8;
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).spAttackIV() as u8;
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).spDefenseIV() as u8;
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF_IV,
                &raw mut iv as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PERSONALITY,
                &raw mut (*battlePokemon).personality as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_STATUS,
                &raw mut (*battlePokemon).status1 as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_LEVEL,
                &raw mut (*battlePokemon).level as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP,
                &raw mut (*battlePokemon).hp as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MAX_HP,
                &raw mut (*battlePokemon).maxHP as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK,
                &raw mut (*battlePokemon).attack as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF,
                &raw mut (*battlePokemon).defense as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED,
                &raw mut (*battlePokemon).speed as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK,
                &raw mut (*battlePokemon).spAttack as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF,
                &raw mut (*battlePokemon).spDefense as *mut c_void,
            );
        }
        REQUEST_SPECIES_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPECIES,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_HELDITEM_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HELD_ITEM,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MOVES_PP_BATTLE => {
            for i in 0..MAX_MON_MOVES {
                SetMonData(
                    crate::gl!(&raw gPlayerParty[monId]),
                    MON_DATA_MOVE1 + i,
                    &raw mut (*moveData).moves[i] as *mut c_void,
                );
                SetMonData(
                    crate::gl!(&raw gPlayerParty[monId]),
                    MON_DATA_PP1 + i,
                    &raw mut (*moveData).pp[i] as *mut c_void,
                );
            }
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP_BONUSES,
                &raw mut (*moveData).ppBonuses as *mut c_void,
            );
        }
        4 | REQUEST_MOVE2_BATTLE | REQUEST_MOVE3_BATTLE | REQUEST_MOVE4_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MOVE1
                    + ({
                        let gl_i205 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i205][1])
                    }) as i32
                    - REQUEST_MOVE1_BATTLE,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_PP_DATA_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP1,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP2,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][4] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP3,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][5] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP4,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][6] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP_BONUSES,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][7] as *mut c_void,
            );
        }
        REQUEST_PPMOVE1_BATTLE
        | REQUEST_PPMOVE2_BATTLE
        | REQUEST_PPMOVE3_BATTLE
        | REQUEST_PPMOVE4_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PP1
                    + ({
                        let gl_i206 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i206][1])
                    }) as i32
                    - REQUEST_PPMOVE1_BATTLE as i32,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_OTID_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_OT_ID,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_EXP_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_EXP,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_HP_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_ATK_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_DEF_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_FRIENDSHIP,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_POKERUS_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_POKERUS,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MET_LOCATION_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MET_LOCATION,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MET_LEVEL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MET_LEVEL,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MET_GAME_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MET_GAME,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_POKEBALL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_POKEBALL,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_ALL_IVS_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][4] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][5] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][6] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][7] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][8] as *mut c_void,
            );
        }
        REQUEST_HP_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_ATK_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_DEF_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_PERSONALITY_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_PERSONALITY,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_CHECKSUM_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_CHECKSUM,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_STATUS_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_STATUS,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_LEVEL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_LEVEL,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_HP_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_HP,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MAX_HP_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_MAX_HP,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_ATK_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_ATK,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_DEF_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_DEF,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPEED,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPATK,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SPDEF,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_COOL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_COOL,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_BEAUTY,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_CUTE,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SMART_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SMART,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_TOUGH,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SHEEN_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SHEEN,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_COOL_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_BEAUTY_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_CUTE_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_SMART_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gPlayerParty[monId]),
                MON_DATA_TOUGH_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        _ => {}
    }
    HandleLowHpMusicChange(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i207 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i207])
        }],
        gActiveBattler.get(),
    );
}
pub(crate) unsafe fn PlayerHandleSetRawMonData() {
    let dst: *mut u8 = (&raw mut (*gPlayerParty.as_ptr())[{
        let gl_i208 = gActiveBattler.get();
        crate::gl!(gBattlerPartyIndexes[gl_i208])
    }] as *mut u8)
        .at({
            let gl_i209 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i209][1])
        });
    for i in 0..({
        let gl_i210 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i210][2])
    }) {
        *dst.at(i) = {
            let gl_i211 = gActiveBattler.get();
            let gl_i212 = 3 + i as i32;
            crate::gl!(gBattleBufferA[gl_i211][gl_i212])
        };
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleLoadMonSprite() {
    BattleLoadPlayerMonSpriteGfx(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i213 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i213])
        }],
        gActiveBattler.get(),
    );
    (*gSprites.as_ptr())[{
        let gl_i214 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i214])
    }]
    .oam
    .set_paletteNum(gActiveBattler.get() as u16);
    {
        let gl_i215 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i215] = Some(CompleteOnBankSpritePosX_0))
    };
}
pub(crate) unsafe fn PlayerHandleSwitchInAnim() {
    ClearTemporarySpeciesSpriteData(gActiveBattler.get(), {
        let gl_i216 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i216][2])
    });
    {
        let gl_i217 = gActiveBattler.get();
        crate::gl!(
            gBattlerPartyIndexes[gl_i217] = ({
                let gl_i218 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i218][1])
            }) as u16
        )
    };
    BattleLoadPlayerMonSpriteGfx(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i219 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i219])
        }],
        gActiveBattler.get(),
    );
    {
        let gl_i220 = gActiveBattler.get();
        crate::gl!(gActionSelectionCursor[gl_i220] = 0)
    };
    {
        let gl_i221 = gActiveBattler.get();
        crate::gl!(gMoveSelectionCursor[gl_i221] = 0)
    };
    StartSendOutAnim(gActiveBattler.get(), {
        let gl_i222 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i222][2])
    });
    {
        let gl_i223 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i223] = Some(SwitchIn_TryShinyAnimShowHealthbox))
    };
}
pub(crate) unsafe fn StartSendOutAnim(battler: u8, dontClearSubstituteBit: u8) {
    ClearTemporarySpeciesSpriteData(battler, dontClearSubstituteBit);
    crate::gl!(gBattlerPartyIndexes[battler] = crate::gl!(gBattleBufferA[battler][1]) as u16);
    let species: u16 = GetMonData2(
        &raw mut (*gPlayerParty.as_ptr())[crate::gl!(gBattlerPartyIndexes[battler])],
        MON_DATA_SPECIES,
    ) as u16;
    crate::gl!(
        gBattleControllerData[battler] =
            CreateInvisibleSpriteWithCallback(Some(SpriteCB_WaitForBattlerBallReleaseAnim))
    );
    SetMultiuseSpriteTemplateToPokemon(species, GetBattlerPosition(battler));
    crate::gl!(
        gBattlerSpriteIds[battler] = CreateSprite(
            crate::gl!(&raw gMultiuseSpriteTemplate),
            GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16,
            GetBattlerSpriteDefault_Y(battler) as i16,
            GetBattlerSpriteSubpriority(battler),
        )
    );
    (*gSprites.as_ptr())[crate::gl!(gBattleControllerData[battler])].data[1] =
        crate::gl!(gBattlerSpriteIds[battler]) as i16;
    (*gSprites.as_ptr())[crate::gl!(gBattleControllerData[battler])].data[2] = battler as i16;
    (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])].data[0] = battler as i16;
    (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])].data[2] = species as i16;
    (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])]
        .oam
        .set_paletteNum(battler as u16);
    StartSpriteAnim(
        &raw mut (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])],
        crate::gl!(gBattleMonForms[battler]),
    );
    (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])].set_invisible(TRUE as u16);
    (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])].callback =
        Some(SpriteCallbackDummy);
    (*gSprites.as_ptr())[crate::gl!(gBattleControllerData[battler])].data[0] =
        DoPokeballSendOutAnimation(0, POKEBALL_PLAYER_SENDOUT) as i16;
}
pub(crate) unsafe fn PlayerHandleReturnMonToBall() {
    if ({
        let gl_i224 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i224][1])
    }) == 0
    {
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .animationState = 0;
        {
            let gl_i225 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i225] = Some(DoSwitchOutAnimation))
        };
    } else {
        FreeSpriteOamMatrix(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i226 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i226])
            }],
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i227 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i227])
            }],
        );
        SetHealthboxSpriteInvisible({
            let gl_i228 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i228])
        });
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn DoSwitchOutAnimation() {
    match (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .animationState
    {
        0 => {
            if (*(*crate::gl!(gBattleSpritesDataPtr))
                .battlerData
                .at(gActiveBattler.get()))
            .behindSubstitute()
                != 0
            {
                InitAndLaunchSpecialAnimation(
                    gActiveBattler.get(),
                    gActiveBattler.get(),
                    gActiveBattler.get(),
                    B_ANIM_SUBSTITUTE_TO_MON,
                );
            }
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .animationState = 1;
        }
        1 if (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .specialAnimActive()
            == 0 =>
        {
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .animationState = 0;
            InitAndLaunchSpecialAnimation(
                gActiveBattler.get(),
                gActiveBattler.get(),
                gActiveBattler.get(),
                B_ANIM_SWITCH_OUT_PLAYER_MON,
            );
            {
                let gl_i229 = gActiveBattler.get();
                crate::gl!(gBattlerControllerFuncs[gl_i229] = Some(FreeMonSpriteAfterSwitchOutAnim))
            };
        }
        _ => {}
    }
}
pub(crate) unsafe fn PlayerHandleDrawTrainerPic() {
    let mut xPos: i16 = 0;
    let mut yPos: i16 = 0;
    let mut trainerPicId: u32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        if ({
            let gl_i230 = GetMultiplayerId();
            crate::gl!(gLinkPlayers[gl_i230].version)
        }) as i32
            & 0xFF
            == VERSION_FIRE_RED
            || ({
                let gl_i231 = GetMultiplayerId();
                crate::gl!(gLinkPlayers[gl_i231].version)
            }) as i32
                & 0xFF
                == VERSION_LEAF_GREEN
        {
            trainerPicId = ({
                let gl_i232 = GetMultiplayerId();
                crate::gl!(gLinkPlayers[gl_i232].gender)
            }) as u32
                + TRAINER_BACK_PIC_RED;
        } else if ({
            let gl_i233 = GetMultiplayerId();
            crate::gl!(gLinkPlayers[gl_i233].version)
        }) as i32
            & 0xFF
            == VERSION_RUBY
            || ({
                let gl_i234 = GetMultiplayerId();
                crate::gl!(gLinkPlayers[gl_i234].version)
            }) as i32
                & 0xFF
                == VERSION_SAPPHIRE
        {
            trainerPicId = ({
                let gl_i235 = GetMultiplayerId();
                crate::gl!(gLinkPlayers[gl_i235].gender)
            }) as u32
                + TRAINER_BACK_PIC_RUBY_SAPPHIRE_BRENDAN;
        } else {
            trainerPicId = ({
                let gl_i236 = GetMultiplayerId();
                crate::gl!(gLinkPlayers[gl_i236].gender)
            }) as u32
                + TRAINER_BACK_PIC_BRENDAN;
        }
    } else {
        trainerPicId = (*crate::gl!(gSaveBlock2Ptr)).playerGender as u32;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
        if GetBattlerPosition(gActiveBattler.get()) as i32 & BIT_FLANK as i32 != B_FLANK_LEFT {
            xPos = 90;
        } else {
            xPos = 32;
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0
            && gPartnerTrainerId.get() != TRAINER_STEVEN_PARTNER
        {
            xPos = 90;
            yPos = (8
                - (*(&raw const crate::data::data_tables::gTrainerFrontPicCoords).cast::<CArray<
                    MonCoords,
                    0,
                >>(
                ))[trainerPicId]
                    .size as i16)
                * 4
                + 80;
        } else {
            yPos = (8
                - (*(&raw const crate::data::data_tables::gTrainerBackPicCoords).cast::<CArray<
                    MonCoords,
                    0,
                >>(
                ))[trainerPicId]
                    .size as i16)
                * 4
                + 80;
        }
    } else {
        xPos = 80;
        yPos = (8
            - (*(&raw const crate::data::data_tables::gTrainerBackPicCoords)
                .cast::<CArray<MonCoords, 0>>())[trainerPicId]
                .size as i16)
            * 4
            + 80;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0
        && gPartnerTrainerId.get() != TRAINER_STEVEN_PARTNER
    {
        trainerPicId =
            PlayerGenderToFrontTrainerPicId((*crate::gl!(gSaveBlock2Ptr)).playerGender) as u32;
        DecompressTrainerFrontPic(trainerPicId as u16, gActiveBattler.get());
        SetMultiuseSpriteTemplateToTrainerFront(
            trainerPicId as u16,
            GetBattlerPosition(gActiveBattler.get()),
        );
        {
            let gl_v = CreateSprite(
                crate::gl!(&raw gMultiuseSpriteTemplate),
                xPos,
                yPos,
                GetBattlerSpriteSubpriority(gActiveBattler.get()),
            );
            let gl_i237 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i237] = gl_v)
        };
        (*gSprites.as_ptr())[{
            let gl_i238 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i238])
        }]
        .oam
        .set_paletteNum(IndexOfSpritePaletteTag(
            (*(&raw const crate::data::data_tables::gTrainerFrontPicPaletteTable)
                .cast::<CArray<CompressedSpritePalette, 0>>())[trainerPicId]
                .tag,
        ) as u16);
        (*gSprites.as_ptr())[{
            let gl_i239 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i239])
        }]
        .x2 = DISPLAY_WIDTH as i16;
        (*gSprites.as_ptr())[{
            let gl_i240 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i240])
        }]
        .y2 = 48;
        (*gSprites.as_ptr())[{
            let gl_i241 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i241])
        }]
        .data[0] = -2;
        (*gSprites.as_ptr())[{
            let gl_i242 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i242])
        }]
        .callback = Some(SpriteCB_TrainerSlideIn);
        (*gSprites.as_ptr())[{
            let gl_i243 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i243])
        }]
        .oam
        .set_affineMode(ST_OAM_AFFINE_OFF);
        (*gSprites.as_ptr())[{
            let gl_i244 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i244])
        }]
        .set_hFlip(1);
    } else {
        DecompressTrainerBackPic(trainerPicId as u16, gActiveBattler.get());
        SetMultiuseSpriteTemplateToTrainerBack(
            trainerPicId as u16,
            GetBattlerPosition(gActiveBattler.get()),
        );
        {
            let gl_v = CreateSprite(
                crate::gl!(&raw gMultiuseSpriteTemplate),
                xPos,
                yPos,
                GetBattlerSpriteSubpriority(gActiveBattler.get()),
            );
            let gl_i245 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i245] = gl_v)
        };
        (*gSprites.as_ptr())[{
            let gl_i246 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i246])
        }]
        .oam
        .set_paletteNum(gActiveBattler.get() as u16);
        (*gSprites.as_ptr())[{
            let gl_i247 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i247])
        }]
        .x2 = DISPLAY_WIDTH as i16;
        (*gSprites.as_ptr())[{
            let gl_i248 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i248])
        }]
        .data[0] = -2;
        (*gSprites.as_ptr())[{
            let gl_i249 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i249])
        }]
        .callback = Some(SpriteCB_TrainerSlideIn);
    }
    {
        let gl_i250 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i250] = Some(CompleteOnBattlerSpriteCallbackDummy))
    };
}
pub(crate) unsafe fn PlayerHandleTrainerSlide() {
    let mut trainerPicId: u32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        if ({
            let gl_i251 = GetMultiplayerId();
            crate::gl!(gLinkPlayers[gl_i251].version)
        }) as i32
            & 0xFF
            == VERSION_FIRE_RED
            || ({
                let gl_i252 = GetMultiplayerId();
                crate::gl!(gLinkPlayers[gl_i252].version)
            }) as i32
                & 0xFF
                == VERSION_LEAF_GREEN
        {
            trainerPicId = ({
                let gl_i253 = GetMultiplayerId();
                crate::gl!(gLinkPlayers[gl_i253].gender)
            }) as u32
                + TRAINER_BACK_PIC_RED;
        } else if ({
            let gl_i254 = GetMultiplayerId();
            crate::gl!(gLinkPlayers[gl_i254].version)
        }) as i32
            & 0xFF
            == VERSION_RUBY
            || ({
                let gl_i255 = GetMultiplayerId();
                crate::gl!(gLinkPlayers[gl_i255].version)
            }) as i32
                & 0xFF
                == VERSION_SAPPHIRE
        {
            trainerPicId = ({
                let gl_i256 = GetMultiplayerId();
                crate::gl!(gLinkPlayers[gl_i256].gender)
            }) as u32
                + TRAINER_BACK_PIC_RUBY_SAPPHIRE_BRENDAN;
        } else {
            trainerPicId = ({
                let gl_i257 = GetMultiplayerId();
                crate::gl!(gLinkPlayers[gl_i257].gender)
            }) as u32
                + TRAINER_BACK_PIC_BRENDAN;
        }
    } else {
        trainerPicId = (*crate::gl!(gSaveBlock2Ptr)).playerGender as u32 + TRAINER_BACK_PIC_BRENDAN;
    }
    DecompressTrainerBackPic(trainerPicId as u16, gActiveBattler.get());
    SetMultiuseSpriteTemplateToTrainerBack(
        trainerPicId as u16,
        GetBattlerPosition(gActiveBattler.get()),
    );
    {
        let gl_v = CreateSprite(
            crate::gl!(&raw gMultiuseSpriteTemplate),
            80,
            (8 - (*(&raw const crate::data::data_tables::gTrainerBackPicCoords)
                .cast::<CArray<MonCoords, 0>>())[trainerPicId]
                .size as i16)
                * 4
                + 80,
            30,
        );
        let gl_i258 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i258] = gl_v)
    };
    (*gSprites.as_ptr())[{
        let gl_i259 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i259])
    }]
    .oam
    .set_paletteNum(gActiveBattler.get() as u16);
    (*gSprites.as_ptr())[{
        let gl_i260 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i260])
    }]
    .x2 = -96;
    (*gSprites.as_ptr())[{
        let gl_i261 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i261])
    }]
    .data[0] = 2;
    (*gSprites.as_ptr())[{
        let gl_i262 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i262])
    }]
    .callback = Some(SpriteCB_TrainerSlideIn);
    {
        let gl_i263 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i263] = Some(CompleteOnBankSpriteCallbackDummy2))
    };
}
pub(crate) unsafe fn PlayerHandleTrainerSlideBack() {
    SetSpritePrimaryCoordsFromSecondaryCoords(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i264 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i264])
        }],
    );
    (*gSprites.as_ptr())[{
        let gl_i265 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i265])
    }]
    .data[0] = 50;
    (*gSprites.as_ptr())[{
        let gl_i266 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i266])
    }]
    .data[2] = -40;
    (*gSprites.as_ptr())[{
        let gl_i267 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i267])
    }]
    .data[4] = (*gSprites.as_ptr())[{
        let gl_i268 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i268])
    }]
    .y;
    (*gSprites.as_ptr())[{
        let gl_i269 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i269])
    }]
    .callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i270 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i270])
        }],
        Some(SpriteCallbackDummy),
    );
    StartSpriteAnim(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i271 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i271])
        }],
        1,
    );
    {
        let gl_i272 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i272] = Some(FreeTrainerSpriteAfterSlide))
    };
}
pub(crate) unsafe fn PlayerHandleFaintAnimation() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .animationState
        == 0
    {
        if (*(*crate::gl!(gBattleSpritesDataPtr))
            .battlerData
            .at(gActiveBattler.get()))
        .behindSubstitute()
            != 0
        {
            InitAndLaunchSpecialAnimation(
                gActiveBattler.get(),
                gActiveBattler.get(),
                gActiveBattler.get(),
                B_ANIM_SUBSTITUTE_TO_MON,
            );
        }
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .animationState += 1;
    } else {
        if (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .specialAnimActive()
            == 0
        {
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .animationState = 0;
            HandleLowHpMusicChange(
                &raw mut (*gPlayerParty.as_ptr())[{
                    let gl_i273 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i273])
                }],
                gActiveBattler.get(),
            );
            PlaySE12WithPanning(SE_FAINT, SOUND_PAN_ATTACKER);
            (*gSprites.as_ptr())[{
                let gl_i274 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i274])
            }]
            .data[1] = 0;
            (*gSprites.as_ptr())[{
                let gl_i275 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i275])
            }]
            .data[sSpeedY] = 5;
            (*gSprites.as_ptr())[{
                let gl_i276 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i276])
            }]
            .callback = Some(SpriteCB_FaintSlideAnim);
            {
                let gl_i277 = gActiveBattler.get();
                crate::gl!(gBattlerControllerFuncs[gl_i277] = Some(FreeMonSpriteAfterFaintAnim))
            };
        }
    }
}
pub(crate) unsafe fn PlayerHandlePaletteFade() {
    BeginNormalPaletteFade(PALETTES_ALL, 2, 0, 16, 0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleSuccessBallThrowAnim() {
    (*(*crate::gl!(gBattleSpritesDataPtr)).animationData).ballThrowCaseId = BALL_3_SHAKES_SUCCESS;
    gDoingBattleAnim.set(TRUE);
    InitAndLaunchSpecialAnimation(
        gActiveBattler.get(),
        gActiveBattler.get(),
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        B_ANIM_BALL_THROW,
    );
    {
        let gl_i278 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i278] = Some(CompleteOnSpecialAnimDone))
    };
}
pub(crate) unsafe fn PlayerHandleBallThrowAnim() {
    let ballThrowCaseId: u8 = {
        let gl_i279 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i279][1])
    };
    (*(*crate::gl!(gBattleSpritesDataPtr)).animationData).ballThrowCaseId = ballThrowCaseId;
    gDoingBattleAnim.set(TRUE);
    InitAndLaunchSpecialAnimation(
        gActiveBattler.get(),
        gActiveBattler.get(),
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        B_ANIM_BALL_THROW,
    );
    {
        let gl_i280 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i280] = Some(CompleteOnSpecialAnimDone))
    };
}
pub(crate) unsafe fn PlayerHandlePause() {
    let mut timer: u8 = {
        let gl_i281 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i281][1])
    };
    while timer != 0 {
        timer -= 1;
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleMoveAnimation() {
    if IsBattleSEPlaying(gActiveBattler.get()) == 0 {
        let r#move: u16 = ({
            let gl_i282 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i282][1])
        }) as u16
            | (({
                let gl_i283 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i283][2])
            }) as u16)
                << 8;
        gAnimMoveTurn.set({
            let gl_i284 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i284][3])
        });
        gAnimMovePower.set(
            ({
                let gl_i285 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i285][4])
            }) as u16
                | (({
                    let gl_i286 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i286][5])
                }) as u16)
                    << 8,
        );
        gAnimMoveDmg.set(
            ({
                let gl_i287 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i287][6])
            }) as i32
                | (({
                    let gl_i288 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i288][7])
                }) as i32)
                    << 8
                | (({
                    let gl_i289 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i289][8])
                }) as i32)
                    << 16
                | (({
                    let gl_i290 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i290][9])
                }) as i32)
                    << 24,
        );
        gAnimFriendship.set({
            let gl_i291 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i291][10])
        });
        gWeatherMoveAnim.set(
            ({
                let gl_i292 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i292][12])
            }) as u16
                | (({
                    let gl_i293 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i293][13])
                }) as u16)
                    << 8,
        );
        crate::gl!(
            gAnimDisableStructPtr =
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][16] as *mut DisableStruct
        );
        {
            let gl_i294 = gActiveBattler.get();
            crate::gl!(
                gTransformedPersonalities[gl_i294] =
                    (*crate::gl!(gAnimDisableStructPtr)).transformedMonPersonality
            )
        };
        if IsMoveWithoutAnimation(r#move, gAnimMoveTurn.get()) != 0 {
            PlayerBufferExecCompleted();
        } else {
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .animationState = 0;
            {
                let gl_i295 = gActiveBattler.get();
                crate::gl!(gBattlerControllerFuncs[gl_i295] = Some(PlayerDoMoveAnimation))
            };
            BattleTv_SetDataBasedOnMove(
                r#move,
                gWeatherMoveAnim.get(),
                crate::gl!(gAnimDisableStructPtr),
            );
        }
    }
}
pub(crate) unsafe fn PlayerDoMoveAnimation() {
    let r#move: u16 = ({
        let gl_i296 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i296][1])
    }) as u16
        | (({
            let gl_i297 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i297][2])
        }) as u16)
            << 8;
    let multihit: u8 = {
        let gl_i298 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i298][11])
    };
    match (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .animationState
    {
        0 => {
            if (*(*crate::gl!(gBattleSpritesDataPtr))
                .battlerData
                .at(gActiveBattler.get()))
            .behindSubstitute()
                != 0
                && (*(*crate::gl!(gBattleSpritesDataPtr))
                    .battlerData
                    .at(gActiveBattler.get()))
                .flag_x8()
                    == 0
            {
                (*(*crate::gl!(gBattleSpritesDataPtr))
                    .battlerData
                    .at(gActiveBattler.get()))
                .set_flag_x8(1);
                InitAndLaunchSpecialAnimation(
                    gActiveBattler.get(),
                    gActiveBattler.get(),
                    gActiveBattler.get(),
                    B_ANIM_SUBSTITUTE_TO_MON,
                );
            }
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .animationState = 1;
        }
        1 => {
            if (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .specialAnimActive()
                == 0
            {
                SetBattlerSpriteAffineMode(ST_OAM_AFFINE_OFF as u8);
                DoMoveAnim(r#move);
                (*(*crate::gl!(gBattleSpritesDataPtr))
                    .healthBoxesData
                    .at(gActiveBattler.get()))
                .animationState = 2;
            }
        }
        2 => {
            (*gAnimScriptCallback.as_ptr()).unwrap_unchecked()();
            if gAnimScriptActive.get() == 0 {
                SetBattlerSpriteAffineMode(ST_OAM_AFFINE_NORMAL as u8);
                if (*(*crate::gl!(gBattleSpritesDataPtr))
                    .battlerData
                    .at(gActiveBattler.get()))
                .behindSubstitute()
                    != 0
                    && multihit < 2
                {
                    InitAndLaunchSpecialAnimation(
                        gActiveBattler.get(),
                        gActiveBattler.get(),
                        gActiveBattler.get(),
                        B_ANIM_MON_TO_SUBSTITUTE,
                    );
                    (*(*crate::gl!(gBattleSpritesDataPtr))
                        .battlerData
                        .at(gActiveBattler.get()))
                    .set_flag_x8(0);
                }
                (*(*crate::gl!(gBattleSpritesDataPtr))
                    .healthBoxesData
                    .at(gActiveBattler.get()))
                .animationState = 3;
            }
        }
        3 if (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .specialAnimActive()
            == 0 =>
        {
            CopyAllBattleSpritesInvisibilities();
            TrySetBehindSubstituteSpriteBit(
                gActiveBattler.get(),
                ({
                    let gl_i299 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i299][1])
                }) as u16
                    | (({
                        let gl_i300 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i300][2])
                    }) as u16)
                        << 8,
            );
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .animationState = 0;
            PlayerBufferExecCompleted();
        }
        _ => {}
    }
}
pub(crate) unsafe fn PlayerHandlePrintString() {
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    let stringId: *mut u16 = &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;)
        .cast_const())
    .cast::<CArray<CArray<u8, 512>, 4>>()
    .cast_mut())[gActiveBattler.get()][2] as *mut u16;
    BufferStringBattle(*stringId);
    BattlePutTextOnWindow((*gDisplayedStringBattle.as_ptr()).as_mut_ptr(), B_WIN_MSG);
    {
        let gl_i301 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i301] = Some(CompleteOnInactiveTextPrinter2))
    };
    BattleTv_SetDataBasedOnString(*stringId);
    BattleArena_DeductSkillPoints(gActiveBattler.get(), *stringId);
}
pub(crate) unsafe fn PlayerHandlePrintSelectionString() {
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        PlayerHandlePrintString();
    } else {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn HandleChooseActionAfterDma3() {
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        gBattle_BG0_X.set(0);
        gBattle_BG0_Y.set(DISPLAY_HEIGHT);
        {
            let gl_i302 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i302] = Some(HandleInputChooseAction))
        };
    }
}
pub(crate) unsafe fn PlayerHandleChooseAction() {
    {
        let gl_i303 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i303] = Some(HandleChooseActionAfterDma3))
    };
    BattleTv_ClearExplosionFaintCause();
    BattlePutTextOnWindow(
        (*(&raw const crate::data::battle_message::gText_BattleMenu).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        B_WIN_ACTION_MENU,
    );
    for i in 0..4i32 {
        ActionSelectionDestroyCursorAt(i as u8);
    }
    ActionSelectionCreateCursorAt(
        {
            let gl_i304 = gActiveBattler.get();
            crate::gl!(gActionSelectionCursor[gl_i304])
        },
        0,
    );
    BattleStringExpandPlaceholdersToDisplayedString(
        (*(&raw const crate::data::battle_message::gText_WhatWillPkmnDo).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    BattlePutTextOnWindow(
        (*gDisplayedStringBattle.as_ptr()).as_mut_ptr(),
        B_WIN_ACTION_PROMPT,
    );
}
pub(crate) unsafe fn PlayerHandleYesNoBox() {
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        HandleBattleWindow(24, 8, 29, 13, 0);
        BattlePutTextOnWindow(
            (*(&raw const crate::data::battle_message::gText_BattleYesNoChoice)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
            B_WIN_YESNO,
        );
        gMultiUsePlayerCursor.set(1);
        BattleCreateYesNoCursorAt(1);
        {
            let gl_i305 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i305] = Some(PlayerHandleYesNoInput))
        };
    } else {
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn HandleChooseMoveAfterDma3() {
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        gBattle_BG0_X.set(0);
        gBattle_BG0_Y.set(320);
        {
            let gl_i306 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i306] = Some(HandleInputChooseMove))
        };
    }
}
pub(crate) unsafe fn PlayerChooseMoveInBattlePalace() {
    if ({
        *(*crate::gl!(gBattleStruct))
            .arenaMindPoints
            .as_mut_ptr()
            .at(gActiveBattler.get()) -= 1;
        *(*crate::gl!(gBattleStruct))
            .arenaMindPoints
            .as_mut_ptr()
            .at(gActiveBattler.get())
    }) == 0
    {
        gBattlePalaceMoveSelectionRngValue.set(*crate::random::gRngValue.as_ptr().cast::<u32>());
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            B_ACTION_EXEC_SCRIPT,
            ChooseMoveAndTargetInBattlePalace(),
        );
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn PlayerHandleChooseMove() {
    if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 {
        *(*crate::gl!(gBattleStruct))
            .arenaMindPoints
            .as_mut_ptr()
            .at(gActiveBattler.get()) = 8;
        {
            let gl_i307 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i307] = Some(PlayerChooseMoveInBattlePalace))
        };
    } else {
        InitMoveSelectionsVarsAndStrings();
        {
            let gl_i308 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i308] = Some(HandleChooseMoveAfterDma3))
        };
    }
}
pub unsafe fn InitMoveSelectionsVarsAndStrings() {
    MoveSelectionDisplayMoveNames();
    gMultiUsePlayerCursor.set(0xFF);
    MoveSelectionCreateCursorAt(
        {
            let gl_i309 = gActiveBattler.get();
            crate::gl!(gMoveSelectionCursor[gl_i309])
        },
        0,
    );
    MoveSelectionDisplayPPString();
    MoveSelectionDisplayPPNumber();
    MoveSelectionDisplayMoveType();
}
pub(crate) unsafe fn PlayerHandleChooseItem() {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
    {
        let gl_i310 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i310] = Some(OpenBagAndChooseItem))
    };
    gBattlerInMenuId.set(gActiveBattler.get());
    for i in 0..3i32 {
        crate::gl!(
            gBattlePartyCurrentOrder[i] = ({
                let gl_i311 = gActiveBattler.get();
                let gl_i312 = 1 + i;
                crate::gl!(gBattleBufferA[gl_i311][gl_i312])
            })
        );
    }
}
pub(crate) unsafe fn PlayerHandleChoosePokemon() {
    for i in 0..3i32 {
        crate::gl!(
            gBattlePartyCurrentOrder[i] = ({
                let gl_i313 = gActiveBattler.get();
                let gl_i314 = 4 + i;
                crate::gl!(gBattleBufferA[gl_i313][gl_i314])
            })
        );
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_ARENA != 0
        && (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][1] as i32
            & 0xF
            != PARTY_ACTION_CANT_SWITCH as i32
    {
        BtlController_EmitChosenMonReturnValue(
            B_COMM_TO_ENGINE,
            ({
                let gl_i315 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i315])
            }) as u8
                + 1,
            (*gBattlePartyCurrentOrder.as_ptr()).as_mut_ptr(),
        );
        PlayerBufferExecCompleted();
    } else {
        {
            let gl_v = CreateTask(Some(TaskDummy), 0xFF);
            let gl_i316 = gActiveBattler.get();
            crate::gl!(gBattleControllerData[gl_i316] = gl_v)
        };
        task_set(
            {
                let gl_i317 = gActiveBattler.get();
                crate::gl!(gBattleControllerData[gl_i317])
            },
            0,
            ({
                let gl_i318 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i318][1])
            }) as i16
                & 0xF,
        );
        (*crate::gl!(gBattleStruct)).battlerPreventingSwitchout = ({
            let gl_i319 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i319][1])
        }) >> 4;
        (*crate::gl!(gBattleStruct)).prevSelectedPartySlot = {
            let gl_i320 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i320][2])
        };
        (*crate::gl!(gBattleStruct)).abilityPreventingSwitchout = {
            let gl_i321 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i321][3])
        };
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
        {
            let gl_i322 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i322] = Some(OpenPartyMenuToChooseMon))
        };
        gBattlerInMenuId.set(gActiveBattler.get());
    }
}
pub(crate) unsafe fn PlayerHandleCmd23() {
    BattleStopLowHpSound();
    BeginNormalPaletteFade(PALETTES_ALL, 2, 0, 16, 0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleHealthBarUpdate() {
    LoadBattleBarGfx(0);
    let hpVal: i16 = ({
        let gl_i323 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i323][2])
    }) as i16
        | (({
            let gl_i324 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i324][3])
        }) as i16)
            << 8;
    if hpVal > 0 {
        gPlayerPartyLostHP.set(gPlayerPartyLostHP.get() + (hpVal as u32));
    }
    if hpVal != INSTANT_HP_BAR_DROP {
        let maxHP: u32 = GetMonData2(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i325 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i325])
            }],
            MON_DATA_MAX_HP,
        );
        let curHP: u32 = GetMonData2(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i326 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i326])
            }],
            MON_DATA_HP,
        );
        SetBattleBarStruct(
            gActiveBattler.get(),
            {
                let gl_i327 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i327])
            },
            maxHP as i32,
            curHP as i32,
            hpVal as i32,
        );
    } else {
        let maxHP: u32 = GetMonData2(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i328 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i328])
            }],
            MON_DATA_MAX_HP,
        );
        SetBattleBarStruct(
            gActiveBattler.get(),
            {
                let gl_i329 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i329])
            },
            maxHP as i32,
            0,
            hpVal as i32,
        );
        UpdateHpTextInHealthbox(
            {
                let gl_i330 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i330])
            },
            0,
            HP_CURRENT,
        );
    }
    {
        let gl_i331 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i331] = Some(CompleteOnHealthbarDone))
    };
}
pub(crate) unsafe fn PlayerHandleExpUpdate() {
    let monId: u8 = {
        let gl_i332 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i332][1])
    };
    if GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_LEVEL) >= MAX_LEVEL {
        PlayerBufferExecCompleted();
    } else {
        LoadBattleBarGfx(1);
        GetMonData2(crate::gl!(&raw gPlayerParty[monId]), MON_DATA_SPECIES);
        let expPointsToGive: i16 = ({
            let gl_i333 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i333][2])
        }) as i16
            | (*(&raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;)
                .cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][2])
                .at(1) as i16)
                << 8;
        let taskId: u8 = CreateTask(Some(Task_GiveExpToMon), 10);
        task_set(taskId, tExpTask_monId, monId as i16);
        task_set(taskId, tExpTask_gainedExp, expPointsToGive);
        task_set(taskId, tExpTask_battler, gActiveBattler.get() as i16);
        {
            let gl_i334 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i334] = Some(BattleControllerDummy))
        };
    }
}
pub(crate) unsafe fn PlayerHandleStatusIconUpdate() {
    if IsBattleSEPlaying(gActiveBattler.get()) == 0 {
        UpdateHealthboxAttribute(
            {
                let gl_i335 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i335])
            },
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i336 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i336])
            }],
            HEALTHBOX_STATUS_ICON,
        );
        let battler: u8 = gActiveBattler.get();
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .set_statusAnimActive(0);
        {
            let gl_i337 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i337] = Some(CompleteOnFinishedStatusAnimation))
        };
    }
}
pub(crate) unsafe fn PlayerHandleStatusAnimation() {
    if IsBattleSEPlaying(gActiveBattler.get()) == 0 {
        InitAndLaunchChosenStatusAnimation(
            {
                let gl_i338 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i338][1])
            },
            ({
                let gl_i339 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i339][2])
            }) as u32
                | (({
                    let gl_i340 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i340][3])
                }) as u32)
                    << 8
                | (({
                    let gl_i341 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i341][4])
                }) as u32)
                    << 16
                | (({
                    let gl_i342 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i342][5])
                }) as u32)
                    << 24,
        );
        {
            let gl_i343 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i343] = Some(CompleteOnFinishedStatusAnimation))
        };
    }
}
pub(crate) unsafe fn PlayerHandleStatusXor() {
    let mut val: u8 = GetMonData2(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i344 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i344])
        }],
        MON_DATA_STATUS,
    ) as u8
        ^ ({
            let gl_i345 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i345][1])
        });
    SetMonData(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i346 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i346])
        }],
        MON_DATA_STATUS,
        &raw mut val as *mut c_void,
    );
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleDataTransfer() {
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleDMA3Transfer() {
    let dstArg: u32 = ({
        let gl_i347 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i347][1])
    }) as u32
        | (({
            let gl_i348 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i348][2])
        }) as u32)
            << 8
        | (({
            let gl_i349 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i349][3])
        }) as u32)
            << 16
        | (({
            let gl_i350 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i350][4])
        }) as u32)
            << 24;
    let sizeArg: u16 = ({
        let gl_i351 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i351][5])
    }) as u16
        | (({
            let gl_i352 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i352][6])
        }) as u16)
            << 8;
    {
        let mut _src: *mut c_void =
            &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                .cast::<CArray<CArray<u8, 512>, 4>>()
                .cast_mut())[gActiveBattler.get()][7] as *mut c_void;
        let mut _dest: *mut c_void = dstArg as usize as *mut c_void;
        let mut _size: u32 = sizeArg as u32;
        loop {
            if _size <= 0x1000 {
                {
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, _src as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
                break;
            }
            {
                {
                    {
                        let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
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
pub(crate) unsafe fn PlayerHandlePlayBGM() {
    PlayBGM(
        ({
            let gl_i353 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i353][1])
        }) as u16
            | (({
                let gl_i354 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i354][2])
            }) as u16)
                << 8,
    );
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleCmd32() {
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleTwoReturnValues() {
    BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, 0, 0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleChosenMonReturnValue() {
    BtlController_EmitChosenMonReturnValue(B_COMM_TO_ENGINE, 0, null_mut());
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleOneReturnValue() {
    BtlController_EmitOneReturnValue(B_COMM_TO_ENGINE, 0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleOneReturnValue_Duplicate() {
    BtlController_EmitOneReturnValue_Duplicate(B_COMM_TO_ENGINE, 0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleClearUnkVar() {
    (*gUnusedControllerStruct.as_ptr()).set_unk(0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleSetUnkVar() {
    (*gUnusedControllerStruct.as_ptr()).set_unk({
        let gl_i355 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i355][1])
    });
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleClearUnkFlag() {
    (*gUnusedControllerStruct.as_ptr()).set_flag(0);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleToggleUnkFlag() {
    (*gUnusedControllerStruct.as_ptr()).set_flag((*gUnusedControllerStruct.as_ptr()).flag() ^ 1);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleHitAnimation() {
    if (*gSprites.as_ptr())[{
        let gl_i356 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i356])
    }]
    .invisible()
        == TRUE as u16
    {
        PlayerBufferExecCompleted();
    } else {
        gDoingBattleAnim.set(TRUE);
        (*gSprites.as_ptr())[{
            let gl_i357 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i357])
        }]
        .data[1] = 0;
        DoHitAnimHealthboxEffect(gActiveBattler.get());
        {
            let gl_i358 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i358] = Some(DoHitAnimBlinkSpriteEffect))
        };
    }
}
pub(crate) unsafe fn PlayerHandleCantSwitch() {
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandlePlaySE() {
    let mut pan: i8 = 0;
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        pan = SOUND_PAN_ATTACKER;
    } else {
        pan = SOUND_PAN_TARGET;
    }
    PlaySE12WithPanning(
        ({
            let gl_i359 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i359][1])
        }) as u16
            | (({
                let gl_i360 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i360][2])
            }) as u16)
                << 8,
        pan,
    );
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandlePlayFanfareOrBGM() {
    if ({
        let gl_i361 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i361][3])
    }) != 0
    {
        BattleStopLowHpSound();
        PlayBGM(
            ({
                let gl_i362 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i362][1])
            }) as u16
                | (({
                    let gl_i363 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i363][2])
                }) as u16)
                    << 8,
        );
    } else {
        PlayFanfare(
            ({
                let gl_i364 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i364][1])
            }) as u16
                | (({
                    let gl_i365 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i365][2])
                }) as u16)
                    << 8,
        );
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleFaintingCry() {
    let species: u16 = GetMonData2(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i366 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i366])
        }],
        MON_DATA_SPECIES,
    ) as u16;
    PlayCry_ByMode(species, -25, CRY_MODE_FAINT);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleIntroSlide() {
    HandleIntroSlide({
        let gl_i367 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i367][1])
    });
    gIntroSlideFlags.set(gIntroSlideFlags.get() | (1));
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleIntroTrainerBallThrow() {
    SetSpritePrimaryCoordsFromSecondaryCoords(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i368 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i368])
        }],
    );
    (*gSprites.as_ptr())[{
        let gl_i369 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i369])
    }]
    .data[0] = 50;
    (*gSprites.as_ptr())[{
        let gl_i370 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i370])
    }]
    .data[2] = -40;
    (*gSprites.as_ptr())[{
        let gl_i371 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i371])
    }]
    .data[4] = (*gSprites.as_ptr())[{
        let gl_i372 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i372])
    }]
    .y;
    (*gSprites.as_ptr())[{
        let gl_i373 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i373])
    }]
    .callback = Some(StartAnimLinearTranslation);
    (*gSprites.as_ptr())[{
        let gl_i374 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i374])
    }]
    .data[sBattlerId] = gActiveBattler.get() as i16;
    StoreSpriteCallbackInData6(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i375 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i375])
        }],
        Some(SpriteCB_FreePlayerSpriteLoadMonSprite),
    );
    StartSpriteAnim(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i376 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i376])
        }],
        1,
    );
    let paletteNum: u8 = AllocSpritePalette(0xD6F8);
    LoadCompressedPalette(
        (*(&raw const crate::data::data_tables::gTrainerBackPicPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())
            [(*crate::gl!(gSaveBlock2Ptr)).playerGender]
            .data,
        0x100 + paletteNum as u16 * 16,
        32,
    );
    (*gSprites.as_ptr())[{
        let gl_i377 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i377])
    }]
    .oam
    .set_paletteNum(paletteNum as u16);
    let taskId: u8 = CreateTask(Some(Task_StartSendOutAnim), 5);
    task_set(taskId, 0, gActiveBattler.get() as i16);
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .partyStatusSummaryShown()
        != 0
    {
        task_set_func(
            {
                let gl_i378 = gActiveBattler.get();
                crate::gl!(gBattlerStatusSummaryTaskId[gl_i378])
            },
            Some(Task_HidePartyStatusSummary),
        );
    }
    (*(*crate::gl!(gBattleSpritesDataPtr)).animationData).set_introAnimActive(TRUE);
    {
        let gl_i379 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i379] = Some(BattleControllerDummy))
    };
}
pub unsafe fn SpriteCB_FreePlayerSpriteLoadMonSprite(sprite: *mut Sprite) {
    let battler: u8 = (*sprite).data[sBattlerId] as u8;
    FreeSpriteOamMatrix(sprite);
    FreeSpritePaletteByTag(GetSpritePaletteTagByPaletteNum(
        (*sprite).oam.paletteNum() as u8
    ));
    DestroySprite(sprite);
    BattleLoadPlayerMonSpriteGfx(
        &raw mut (*gPlayerParty.as_ptr())[crate::gl!(gBattlerPartyIndexes[battler])],
        battler,
    );
    StartSpriteAnim(
        &raw mut (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])],
        0,
    );
}
pub(crate) unsafe fn Task_StartSendOutAnim(taskId: u8) {
    if task_get(taskId, tStartTimer) < 31 {
        task_set(taskId, tStartTimer, task_get(taskId, tStartTimer) + 1);
    } else {
        let savedActiveBattler: u8 = gActiveBattler.get();
        gActiveBattler.set(task_get(taskId, tBattlerId) as u8);
        if IsDoubleBattle() == 0 || gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
            {
                let gl_i380 = gActiveBattler.get();
                crate::gl!(
                    gBattleBufferA[gl_i380][1] = ({
                        let gl_i381 = gActiveBattler.get();
                        crate::gl!(gBattlerPartyIndexes[gl_i381])
                    }) as u8
                )
            };
            StartSendOutAnim(gActiveBattler.get(), FALSE);
        } else {
            {
                let gl_i382 = gActiveBattler.get();
                crate::gl!(
                    gBattleBufferA[gl_i382][1] = ({
                        let gl_i383 = gActiveBattler.get();
                        crate::gl!(gBattlerPartyIndexes[gl_i383])
                    }) as u8
                )
            };
            StartSendOutAnim(gActiveBattler.get(), FALSE);
            gActiveBattler.set(gActiveBattler.get() ^ (BIT_FLANK));
            {
                let gl_i384 = gActiveBattler.get();
                crate::gl!(
                    gBattleBufferA[gl_i384][1] = ({
                        let gl_i385 = gActiveBattler.get();
                        crate::gl!(gBattlerPartyIndexes[gl_i385])
                    }) as u8
                )
            };
            BattleLoadPlayerMonSpriteGfx(
                &raw mut (*gPlayerParty.as_ptr())[{
                    let gl_i386 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i386])
                }],
                gActiveBattler.get(),
            );
            StartSendOutAnim(gActiveBattler.get(), FALSE);
            gActiveBattler.set(gActiveBattler.get() ^ (BIT_FLANK));
        }
        {
            let gl_i387 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i387] = Some(Intro_TryShinyAnimShowHealthbox))
        };
        gActiveBattler.set(savedActiveBattler);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn PlayerHandleDrawPartyStatusSummary() {
    if ({
        let gl_i388 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i388][1])
    }) != 0
        && GetBattlerSide(gActiveBattler.get()) == 0
    {
        PlayerBufferExecCompleted();
    } else {
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_partyStatusSummaryShown(1);
        {
            let gl_v = CreatePartyStatusSummarySprites(
                gActiveBattler.get(),
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][4] as *mut HpAndStatus,
                {
                    let gl_i390 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i390][1])
                },
                {
                    let gl_i391 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i391][2])
                },
            );
            let gl_i389 = gActiveBattler.get();
            crate::gl!(gBattlerStatusSummaryTaskId[gl_i389] = gl_v)
        };
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .partyStatusDelayTimer = 0;
        if ({
            let gl_i392 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i392][2])
        }) != 0
        {
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .partyStatusDelayTimer = 93;
        }
        {
            let gl_i393 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i393] = Some(EndDrawPartyStatusSummary))
        };
    }
}
pub(crate) unsafe fn EndDrawPartyStatusSummary() {
    if ({
        let t1 = (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .partyStatusDelayTimer;
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .partyStatusDelayTimer += 1;
        t1
    }) > 92
    {
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .partyStatusDelayTimer = 0;
        PlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn PlayerHandleHidePartyStatusSummary() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .partyStatusSummaryShown()
        != 0
    {
        task_set_func(
            {
                let gl_i394 = gActiveBattler.get();
                crate::gl!(gBattlerStatusSummaryTaskId[gl_i394])
            },
            Some(Task_HidePartyStatusSummary),
        );
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleEndBounceEffect() {
    EndBounceEffect(gActiveBattler.get(), BOUNCE_HEALTHBOX);
    EndBounceEffect(gActiveBattler.get(), BOUNCE_MON);
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleSpriteInvisibility() {
    if IsBattlerSpritePresent(gActiveBattler.get()) != 0 {
        (*gSprites.as_ptr())[{
            let gl_i395 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i395])
        }]
        .set_invisible(
            ({
                let gl_i396 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i396][1])
            }) as u16,
        );
        CopyBattleSpriteInvisibility(gActiveBattler.get());
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleBattleAnimation() {
    if IsBattleSEPlaying(gActiveBattler.get()) == 0 {
        let animationId: u8 = {
            let gl_i397 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i397][1])
        };
        let argument: u16 = ({
            let gl_i398 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i398][2])
        }) as u16
            | (({
                let gl_i399 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i399][3])
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
            PlayerBufferExecCompleted();
        } else {
            {
                let gl_i400 = gActiveBattler.get();
                crate::gl!(
                    gBattlerControllerFuncs[gl_i400] = Some(CompleteOnFinishedBattleAnimation)
                )
            };
        }
        BattleTv_SetDataBasedOnAnimation(animationId);
    }
}
pub(crate) unsafe fn PlayerHandleLinkStandbyMsg() {
    RecordedBattle_RecordAllBattlerData(
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][2],
    );
    'l1: {
        let sw1: u8 = {
            let gl_i401 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i401][1])
        };
        let mut fall = false;
        if sw1 == LINK_STANDBY_MSG_STOP_BOUNCE {
            fall = true;
            PrintLinkStandbyMsg();
        }
        if fall || sw1 == LINK_STANDBY_STOP_BOUNCE_ONLY {
            EndBounceEffect(gActiveBattler.get(), BOUNCE_HEALTHBOX);
            EndBounceEffect(gActiveBattler.get(), BOUNCE_MON);
            break 'l1;
        }
        if sw1 == LINK_STANDBY_MSG_ONLY {
            PrintLinkStandbyMsg();
            break 'l1;
        }
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleResetActionMoveSelection() {
    match {
        let gl_i402 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i402][1])
    } {
        RESET_ACTION_MOVE_SELECTION => {
            {
                let gl_i403 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i403] = 0)
            };
            {
                let gl_i404 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i404] = 0)
            };
        }
        RESET_ACTION_SELECTION => {
            {
                let gl_i405 = gActiveBattler.get();
                crate::gl!(gActionSelectionCursor[gl_i405] = 0)
            };
        }
        RESET_MOVE_SELECTION => {
            {
                let gl_i406 = gActiveBattler.get();
                crate::gl!(gMoveSelectionCursor[gl_i406] = 0)
            };
        }
        _ => {}
    }
    PlayerBufferExecCompleted();
}
pub(crate) unsafe fn PlayerHandleEndLinkBattle() {
    RecordedBattle_RecordAllBattlerData(
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][4],
    );
    gBattleOutcome.set({
        let gl_i407 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i407][1])
    });
    (*crate::gl!(gSaveBlock2Ptr))
        .frontier
        .set_disableRecordBattle({
            let gl_i408 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i408][2])
        });
    FadeOutMapMusic(5);
    BeginFastPaletteFade(3);
    PlayerBufferExecCompleted();
    {
        let gl_i409 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i409] = Some(SetBattleEndCallbacks))
    };
}
pub(crate) fn PlayerCmdEnd() {}
