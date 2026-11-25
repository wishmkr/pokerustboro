//! Translated from `src/battle_controller_opponent.c` by tools/rustport/c2rs.py.
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
    clippy::eq_op,
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    unused_assignments
)]

use crate::agb_main::gMain;
use crate::battle_ai_script_commands::{BattleAI_ChooseMoveOrAction, BattleAI_SetupAIData};
use crate::battle_ai_switch_items::{AI_TrySwitchOrUseItem, GetMostSuitableMonToSwitchInto};
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
    BtlController_EmitOneReturnValue, BtlController_EmitTwoReturnValues,
    PrepareBufferDataTransferLink, gUnusedControllerStruct,
};
use crate::battle_gfx_sfx_util::{
    BattleLoadOpponentMonSpriteGfx, BattleStopLowHpSound, ChooseMoveAndTargetInBattlePalace,
    ClearTemporarySpeciesSpriteData, CopyAllBattleSpritesInvisibilities,
    CopyBattleSpriteInvisibility, DecompressTrainerFrontPic, FreeTrainerFrontPicPalette,
    HideBattlerShadowSprite, InitAndLaunchChosenStatusAnimation, InitAndLaunchSpecialAnimation,
    IsBattleSEPlaying, IsMoveWithoutAnimation, LoadBattleBarGfx, SetBattlerShadowSpriteCallback,
    SetBattlerSpriteAffineMode, SpriteCB_TrainerSlideIn, SpriteCB_WaitForBattlerBallReleaseAnim,
    TryHandleLaunchBattleTableAnimation, TrySetBehindSubstituteSpriteBit,
};
use crate::battle_interface::{
    CreatePartyStatusSummarySprites, MoveBattleBar, SetBattleBarStruct,
    SetHealthboxSpriteInvisible, SetHealthboxSpriteVisible, Task_HidePartyStatusSummary,
    UpdateHealthboxAttribute, UpdateHpTextInHealthbox,
};
use crate::battle_intro::HandleIntroSlide;
use crate::battle_main::{
    SpriteCB_FaintOpponentMon, SpriteCallbackDummy_2, gAbsentBattlerFlags, gActiveBattler,
    gBattle_BG0_X, gBattle_BG0_Y, gBattleControllerExecFlags,
    gBattleControllerOpponentFlankHealthboxData, gBattleControllerOpponentHealthboxData,
    gBattleSpritesDataPtr, gBattleStruct, gBattleTypeFlags, gBattlerControllerFuncs,
    gBattlerTarget, gDoingBattleAnim, gIntroSlideFlags, gPreBattleCallback1,
    gTransformedPersonalities,
};
use crate::battle_main::{
    gBattleBufferA, gBattleControllerData, gBattleMonForms, gBattlerPartyIndexes,
    gBattlerSpriteIds, gBattlerStatusSummaryTaskId, gDisplayedStringBattle, gHealthboxSpriteIds,
};
use crate::battle_message::{BattlePutTextOnWindow, BufferStringBattle};
use crate::battle_setup::{gTrainerBattleOpponent_A, gTrainerBattleOpponent_B};
use crate::battle_tower::{GetEreaderTrainerFrontSpriteId, GetFrontierTrainerFrontSpriteId};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::frontier_util::GetFrontierBrainTrainerPicIndex;
use crate::link::GetMultiplayerId;
use crate::m4a::{gMPlayInfo_BGM, m4aMPlayContinue, m4aMPlayVolumeControl};
use crate::pokeball::{
    DoHitAnimHealthboxEffect, DoPokeballSendOutAnimation, StartHealthboxSlideIn,
};
use crate::pokemon::{
    GetMonData2, GetMonData3, GetSecretBaseTrainerPicIndex, SetMonData,
    SetMultiuseSpriteTemplateToPokemon, SetMultiuseSpriteTemplateToTrainerBack, gEnemyParty,
    gMultiuseSpriteTemplate,
};
use crate::random::Random;
use crate::sound::{
    IsCryPlayingOrClearCrySongs, PlayBGM, PlayCry_ByMode, PlayFanfare, PlaySE12WithPanning,
};
use crate::sprite::gSprites;
use crate::sprite::{FreeSpritePaletteByTag, FreeSpriteTilesByTag, IndexOfSpritePaletteTag};
use crate::task::DestroyTask;
use crate::task::{task_get, task_set, task_set_func};
use crate::text::IsTextPrinterActive;
use crate::trainer_hill::GetTrainerHillTrainerFrontSpriteId;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
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
/// `StringCopy_Nickname` with this module's view of its types.
#[inline]
unsafe fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy_Nickname(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const sSpeedX: usize = 0;
// Data tables (translate with cdata.py): sOpponentBufferCommands sUnused

static sOpponentBufferCommands: Table<CArray<Option<unsafe fn()>, 57>> =
    Table((&raw const crate::data::battle_controller_opponent::sOpponentBufferCommands).cast());

/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) fn OpponentDummy() {}
pub fn SetControllerToOpponent() {
    {
        let gl_i0 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i0] = Some(OpponentBufferRunCommand))
    };
}
pub(crate) unsafe fn OpponentBufferRunCommand() {
    if gBattleControllerExecFlags.get()
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
        != 0
    {
        if ({
            let gl_i1 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i1][0])
        }) < 57
        {
            sOpponentBufferCommands[{
                let gl_i2 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i2][0])
            }]
            .unwrap_unchecked()();
        } else {
            OpponentBufferExecCompleted();
        }
    }
}
pub(crate) unsafe fn CompleteOnBattlerSpriteCallbackDummy() {
    if (*gSprites.as_ptr())[{
        let gl_i3 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i3])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        OpponentBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnBankSpriteCallbackDummy2() {
    if (*gSprites.as_ptr())[{
        let gl_i4 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i4])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        OpponentBufferExecCompleted();
    }
}
pub(crate) unsafe fn FreeTrainerSpriteAfterSlide() {
    if (*gSprites.as_ptr())[{
        let gl_i5 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i5])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        FreeTrainerFrontPicPalette(
            (*gSprites.as_ptr())[{
                let gl_i6 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i6])
            }]
            .oam
            .affineParam,
        );
        FreeSpriteOamMatrix(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i7 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i7])
            }],
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i8 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i8])
            }],
        );
        OpponentBufferExecCompleted();
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
        OpponentBufferExecCompleted();
    }
}
pub(crate) unsafe fn Intro_WaitForShinyAnimAndHealthbox() {
    let mut healthboxAnimDone: u8 = FALSE;
    let mut twoMons: u8 = 0;
    if IsDoubleBattle() == 0
        || (IsDoubleBattle() != 0 && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
            || gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0)
    {
        if (*gSprites.as_ptr())[{
            let gl_i9 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i9])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        {
            healthboxAnimDone = TRUE;
        }
        twoMons = FALSE;
    } else {
        if (*gSprites.as_ptr())[{
            let gl_i10 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i10])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            && (*gSprites.as_ptr())[{
                let gl_i11 = gActiveBattler.get() as i32 ^ 2;
                crate::gl!(gHealthboxSpriteIds[gl_i11])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        {
            healthboxAnimDone = TRUE;
        }
        twoMons = TRUE;
    }
    crate::gl!(
        gBattleControllerOpponentHealthboxData = (*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get())
    );
    crate::gl!(
        gBattleControllerOpponentFlankHealthboxData = (*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2)
    );
    if healthboxAnimDone != 0 {
        if twoMons == TRUE {
            if (*(*crate::gl!(gBattleSpritesDataPtr))
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
            } else {
                return;
            }
        } else if (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .finishedShinyMonAnim()
            != 0
        {
            if GetBattlerPosition(gActiveBattler.get()) == 3 {
                if (*(*crate::gl!(gBattleSpritesDataPtr))
                    .healthBoxesData
                    .at(gActiveBattler.get() as i32 ^ 2))
                .triedShinyMonAnim()
                    == 0
                    && (*(*crate::gl!(gBattleSpritesDataPtr))
                        .healthBoxesData
                        .at(gActiveBattler.get() as i32 ^ 2))
                    .finishedShinyMonAnim()
                        == 0
                {
                    FreeSpriteTilesByTag(ANIM_TAG_GOLD_STARS);
                    FreeSpritePaletteByTag(ANIM_TAG_GOLD_STARS);
                } else {
                    return;
                }
            }
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .set_triedShinyMonAnim(FALSE);
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .set_finishedShinyMonAnim(FALSE);
        } else {
            return;
        }
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .introEndDelay = 3;
        {
            let gl_i12 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i12] = Some(Intro_DelayAndEnd))
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
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .finishedShinyMonAnim()
            == 0
    {
        TryShinyAnimation(
            gActiveBattler.get(),
            &raw mut (*gEnemyParty.as_ptr())[{
                let gl_i13 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i13])
            }],
        );
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS == 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI == 0
        && IsDoubleBattle() != 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .triedShinyMonAnim()
            == 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .ballAnimActive()
            == 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get() as i32 ^ 2))
        .finishedShinyMonAnim()
            == 0
    {
        TryShinyAnimation(
            gActiveBattler.get() ^ 2,
            &raw mut (*gEnemyParty.as_ptr())[{
                let gl_i14 = gActiveBattler.get() as i32 ^ 2;
                crate::gl!(gBattlerPartyIndexes[gl_i14])
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
                        let gl_i15 = gActiveBattler.get() as i32 ^ 2;
                        crate::gl!(gHealthboxSpriteIds[gl_i15])
                    },
                    &raw mut (*gEnemyParty.as_ptr())[{
                        let gl_i16 = gActiveBattler.get() as i32 ^ 2;
                        crate::gl!(gBattlerPartyIndexes[gl_i16])
                    }],
                    HEALTHBOX_ALL,
                );
                StartHealthboxSlideIn(gActiveBattler.get() ^ 2);
                SetHealthboxSpriteVisible({
                    let gl_i17 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gHealthboxSpriteIds[gl_i17])
                });
            }
            UpdateHealthboxAttribute(
                {
                    let gl_i18 = gActiveBattler.get();
                    crate::gl!(gHealthboxSpriteIds[gl_i18])
                },
                &raw mut (*gEnemyParty.as_ptr())[{
                    let gl_i19 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i19])
                }],
                HEALTHBOX_ALL,
            );
            StartHealthboxSlideIn(gActiveBattler.get());
            SetHealthboxSpriteVisible({
                let gl_i20 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i20])
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
                if GetBattlerPosition(gActiveBattler.get()) == 1 {
                    m4aMPlayContinue(crate::gl!(&raw gMPlayInfo_BGM));
                }
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
            let gl_i21 = gActiveBattler.get();
            crate::gl!(gBattleControllerData[gl_i21])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            && (*gSprites.as_ptr())[{
                let gl_i22 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i22])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        {
            battlerAnimsDone = TRUE as u32;
        }
    } else {
        if (*gSprites.as_ptr())[{
            let gl_i23 = gActiveBattler.get();
            crate::gl!(gBattleControllerData[gl_i23])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            && (*gSprites.as_ptr())[{
                let gl_i24 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i24])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            && (*gSprites.as_ptr())[{
                let gl_i25 = gActiveBattler.get() as i32 ^ 2;
                crate::gl!(gBattleControllerData[gl_i25])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            && (*gSprites.as_ptr())[{
                let gl_i26 = gActiveBattler.get() as i32 ^ 2;
                crate::gl!(gBattlerSpriteIds[gl_i26])
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
                    let gl_i27 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gBattleControllerData[gl_i27])
                }],
            );
            SetBattlerShadowSpriteCallback(
                gActiveBattler.get() ^ 2,
                GetMonData2(
                    &raw mut (*gEnemyParty.as_ptr())[{
                        let gl_i28 = gActiveBattler.get() as i32 ^ 2;
                        crate::gl!(gBattlerPartyIndexes[gl_i28])
                    }],
                    MON_DATA_SPECIES,
                ) as u16,
            );
        }
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i29 = gActiveBattler.get();
                crate::gl!(gBattleControllerData[gl_i29])
            }],
        );
        SetBattlerShadowSpriteCallback(
            gActiveBattler.get(),
            GetMonData2(
                &raw mut (*gEnemyParty.as_ptr())[{
                    let gl_i30 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i30])
                }],
                MON_DATA_SPECIES,
            ) as u16,
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
            let gl_i31 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i31] = Some(Intro_WaitForShinyAnimAndHealthbox))
        };
    }
}
pub(crate) unsafe fn TryShinyAnimAfterMonAnim() {
    if (*gSprites.as_ptr())[{
        let gl_i32 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i32])
    }]
    .x2 == 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .triedShinyMonAnim()
            == 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .finishedShinyMonAnim()
            == 0
    {
        TryShinyAnimation(
            gActiveBattler.get(),
            &raw mut (*gEnemyParty.as_ptr())[{
                let gl_i33 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i33])
            }],
        );
    }
    if (*gSprites.as_ptr())[{
        let gl_i34 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i34])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
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
        FreeSpriteTilesByTag(ANIM_TAG_GOLD_STARS);
        FreeSpritePaletteByTag(ANIM_TAG_GOLD_STARS);
        OpponentBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnHealthbarDone() {
    let hpValue: i16 = MoveBattleBar(
        gActiveBattler.get(),
        {
            let gl_i35 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i35])
        },
        HEALTH_BAR,
        0,
    ) as i16;
    SetHealthboxSpriteVisible({
        let gl_i36 = gActiveBattler.get();
        crate::gl!(gHealthboxSpriteIds[gl_i36])
    });
    if hpValue != -1 {
        UpdateHpTextInHealthbox(
            {
                let gl_i37 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i37])
            },
            hpValue,
            HP_CURRENT,
        );
    } else {
        OpponentBufferExecCompleted();
    }
}
pub(crate) unsafe fn HideHealthboxAfterMonFaint() {
    if (*gSprites.as_ptr())[{
        let gl_i38 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i38])
    }]
    .inUse()
        == 0
    {
        SetHealthboxSpriteInvisible({
            let gl_i39 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i39])
        });
        OpponentBufferExecCompleted();
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
                let gl_i40 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i40])
            }],
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i41 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i41])
            }],
        );
        HideBattlerShadowSprite(gActiveBattler.get());
        SetHealthboxSpriteInvisible({
            let gl_i42 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i42])
        });
        OpponentBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnInactiveTextPrinter() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        OpponentBufferExecCompleted();
    }
}
pub(crate) unsafe fn DoHitAnimBlinkSpriteEffect() {
    let spriteId: u8 = {
        let gl_i43 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i43])
    };
    if crate::gl!(gSprites[spriteId].data[1]) == 32 {
        crate::gl!(gSprites[spriteId].data[1] = 0);
        (*gSprites.as_ptr())[spriteId].set_invisible(FALSE as u16);
        gDoingBattleAnim.set(FALSE);
        OpponentBufferExecCompleted();
    } else {
        if crate::gl!(gSprites[spriteId].data[1]) % 4 == 0 {
            (*gSprites.as_ptr())[spriteId]
                .set_invisible((*gSprites.as_ptr())[spriteId].invisible() ^ 1);
        }
        crate::gl!(gSprites[spriteId].data[1] += 1);
    }
}
pub(crate) unsafe fn SwitchIn_ShowSubstitute() {
    if (*gSprites.as_ptr())[{
        let gl_i44 = gActiveBattler.get();
        crate::gl!(gHealthboxSpriteIds[gl_i44])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
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
                B_ANIM_MON_TO_SUBSTITUTE,
            );
        }
        {
            let gl_i45 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i45] = Some(SwitchIn_HandleSoundAndEnd))
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
        && ((*gSprites.as_ptr())[{
            let gl_i46 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i46])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            || (*gSprites.as_ptr())[{
                let gl_i47 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i47])
            }]
            .callback
                == Some(SpriteCallbackDummy_2 as unsafe fn(*mut Sprite)))
    {
        m4aMPlayVolumeControl(crate::gl!(&raw gMPlayInfo_BGM), TRACKS_ALL, 0x100);
        OpponentBufferExecCompleted();
    }
}
pub(crate) unsafe fn SwitchIn_ShowHealthbox() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .finishedShinyMonAnim()
        != 0
        && (*gSprites.as_ptr())[{
            let gl_i48 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i48])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
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
        StartSpriteAnim(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i49 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i49])
            }],
            0,
        );
        UpdateHealthboxAttribute(
            {
                let gl_i50 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i50])
            },
            &raw mut (*gEnemyParty.as_ptr())[{
                let gl_i51 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i51])
            }],
            HEALTHBOX_ALL,
        );
        StartHealthboxSlideIn(gActiveBattler.get());
        SetHealthboxSpriteVisible({
            let gl_i52 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i52])
        });
        CopyBattleSpriteInvisibility(gActiveBattler.get());
        {
            let gl_i53 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i53] = Some(SwitchIn_ShowSubstitute))
        };
    }
}
pub(crate) unsafe fn SwitchIn_TryShinyAnim() {
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
            &raw mut (*gEnemyParty.as_ptr())[{
                let gl_i54 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i54])
            }],
        );
    }
    if (*gSprites.as_ptr())[{
        let gl_i55 = gActiveBattler.get();
        crate::gl!(gBattleControllerData[gl_i55])
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
                let gl_i56 = gActiveBattler.get();
                crate::gl!(gBattleControllerData[gl_i56])
            }],
        );
        SetBattlerShadowSpriteCallback(
            gActiveBattler.get(),
            GetMonData2(
                &raw mut (*gEnemyParty.as_ptr())[{
                    let gl_i57 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i57])
                }],
                MON_DATA_SPECIES,
            ) as u16,
        );
        {
            let gl_i58 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i58] = Some(SwitchIn_ShowHealthbox))
        };
    }
}
pub(crate) unsafe fn CompleteOnFinishedStatusAnimation() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .statusAnimActive()
        == 0
    {
        OpponentBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnFinishedBattleAnimation() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .animFromTableActive()
        == 0
    {
        OpponentBufferExecCompleted();
    }
}
unsafe fn OpponentBufferExecCompleted() {
    {
        let gl_i59 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i59] = Some(OpponentBufferRunCommand))
    };
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        let mut playerId: u8 = GetMultiplayerId();
        PrepareBufferDataTransferLink(B_COMM_CONTROLLER_IS_DONE, 4, &raw mut playerId);
        {
            let gl_i60 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i60][0] = CONTROLLER_TERMINATOR_NOP)
        };
    } else {
        gBattleControllerExecFlags
            .set(gBattleControllerExecFlags.get() & (!gBitTable[gActiveBattler.get()]));
    }
}
pub(crate) unsafe fn OpponentHandleGetMonData() {
    let mut monData: CArray<u8, 256> = zeroed();
    let mut size: u32 = 0;
    let mut monToCheck: u8 = 0;
    if ({
        let gl_i61 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i61][2])
    }) == 0
    {
        size += GetOpponentMonData(
            ({
                let gl_i62 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i62])
            }) as u8,
            monData.as_mut_ptr(),
        );
    } else {
        monToCheck = {
            let gl_i63 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i63][2])
        };
        for i in 0..PARTY_SIZE {
            if monToCheck as i32 & 1 != 0 {
                size += GetOpponentMonData(i as u8, monData.as_mut_ptr().at(size));
            }
            monToCheck >>= 1;
        }
    }
    BtlController_EmitDataTransfer(
        B_COMM_TO_ENGINE,
        size as u16,
        monData.as_mut_ptr() as *mut c_void,
    );
    OpponentBufferExecCompleted();
}
unsafe fn GetOpponentMonData(monId: u8, dst: *mut u8) -> u32 {
    let mut battleMon: BattlePokemon = zeroed();
    let mut moveData: MovePPInfo = zeroed();
    let mut nickname: CArray<u8, 20> = zeroed();
    let mut src: *mut u8 = null_mut();
    let mut data16: i16 = 0;
    let mut data32: u32 = 0;
    let mut size: i32 = 0;
    match {
        let gl_i64 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i64][1])
    } {
        REQUEST_ALL_BATTLE => {
            battleMon.species =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPECIES) as u16;
            battleMon.item =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_HELD_ITEM) as u16;
            for size in 0..MAX_MON_MOVES {
                battleMon.moves[size] =
                    GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_MOVE1 + size) as u16;
                battleMon.pp[size] =
                    GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_PP1 + size) as u8;
            }
            battleMon.ppBonuses =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_PP_BONUSES) as u8;
            battleMon.friendship =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_FRIENDSHIP) as u8;
            battleMon.experience = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_EXP);
            battleMon.set_hpIV(GetMonData2(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_HP_IV,
            ));
            battleMon.set_attackIV(GetMonData2(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_ATK_IV,
            ));
            battleMon.set_defenseIV(GetMonData2(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_DEF_IV,
            ));
            battleMon.set_speedIV(GetMonData2(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPEED_IV,
            ));
            battleMon.set_spAttackIV(GetMonData2(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPATK_IV,
            ));
            battleMon.set_spDefenseIV(GetMonData2(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPDEF_IV,
            ));
            battleMon.personality =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_PERSONALITY);
            battleMon.status1 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_STATUS);
            battleMon.level =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_LEVEL) as u8;
            battleMon.hp = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_HP) as u16;
            battleMon.maxHP =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_MAX_HP) as u16;
            battleMon.attack =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_ATK) as u16;
            battleMon.defense =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_DEF) as u16;
            battleMon.speed =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPEED) as u16;
            battleMon.spAttack =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPATK) as u16;
            battleMon.spDefense =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPDEF) as u16;
            battleMon.set_isEgg(GetMonData2(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_IS_EGG,
            ));
            battleMon.set_abilityNum(GetMonData2(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_ABILITY_NUM,
            ));
            battleMon.otId = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_OT_ID);
            GetMonData3(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_NICKNAME,
                nickname.as_mut_ptr(),
            );
            StringCopy_Nickname(battleMon.nickname.as_mut_ptr(), nickname.as_mut_ptr());
            GetMonData3(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_OT_NAME,
                battleMon.otName.as_mut_ptr(),
            );
            src = &raw mut battleMon as *mut u8;
            for size in 0..88i32 {
                *dst.at(size) = *src.at(size);
            }
        }
        REQUEST_SPECIES_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPECIES) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_HELDITEM_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_HELD_ITEM) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_MOVES_PP_BATTLE => {
            for size in 0..MAX_MON_MOVES {
                moveData.moves[size] =
                    GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_MOVE1 + size) as u16;
                moveData.pp[size] =
                    GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_PP1 + size) as u8;
            }
            moveData.ppBonuses =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_PP_BONUSES) as u8;
            src = &raw mut moveData as *mut u8;
            for size in 0..16i32 {
                *dst.at(size) = *src.at(size);
            }
        }
        4 | REQUEST_MOVE2_BATTLE | REQUEST_MOVE3_BATTLE | REQUEST_MOVE4_BATTLE => {
            data16 = GetMonData2(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_MOVE1
                    + ({
                        let gl_i65 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i65][1])
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
                    GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_PP1 + size) as u8;
                size += 1;
            }
            *dst.at(size) =
                GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_PP_BONUSES) as u8;
            size += 1;
        }
        REQUEST_PPMOVE1_BATTLE
        | REQUEST_PPMOVE2_BATTLE
        | REQUEST_PPMOVE3_BATTLE
        | REQUEST_PPMOVE4_BATTLE => {
            *dst = GetMonData2(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_PP1
                    + ({
                        let gl_i66 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i66][1])
                    }) as i32
                    - REQUEST_PPMOVE1_BATTLE as i32,
            ) as u8;
            size = 1;
        }
        REQUEST_OTID_BATTLE => {
            data32 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_OT_ID);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_EXP_BATTLE => {
            data32 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_EXP);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_HP_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_HP_EV) as u8;
            size = 1;
        }
        REQUEST_ATK_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_ATK_EV) as u8;
            size = 1;
        }
        REQUEST_DEF_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_DEF_EV) as u8;
            size = 1;
        }
        REQUEST_SPEED_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPEED_EV) as u8;
            size = 1;
        }
        REQUEST_SPATK_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPATK_EV) as u8;
            size = 1;
        }
        REQUEST_SPDEF_EV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPDEF_EV) as u8;
            size = 1;
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_FRIENDSHIP) as u8;
            size = 1;
        }
        REQUEST_POKERUS_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_POKERUS) as u8;
            size = 1;
        }
        REQUEST_MET_LOCATION_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_MET_LOCATION) as u8;
            size = 1;
        }
        REQUEST_MET_LEVEL_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_MET_LEVEL) as u8;
            size = 1;
        }
        REQUEST_MET_GAME_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_MET_GAME) as u8;
            size = 1;
        }
        REQUEST_POKEBALL_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_POKEBALL) as u8;
            size = 1;
        }
        REQUEST_ALL_IVS_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_HP_IV) as u8;
            *dst.at(1) = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_ATK_IV) as u8;
            *dst.at(2) = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_DEF_IV) as u8;
            *dst.at(3) = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPEED_IV) as u8;
            *dst.at(4) = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPATK_IV) as u8;
            *dst.at(5) = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPDEF_IV) as u8;
            size = 6;
        }
        REQUEST_HP_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_HP_IV) as u8;
            size = 1;
        }
        REQUEST_ATK_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_ATK_IV) as u8;
            size = 1;
        }
        REQUEST_DEF_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_DEF_IV) as u8;
            size = 1;
        }
        REQUEST_SPEED_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPEED_IV) as u8;
            size = 1;
        }
        REQUEST_SPATK_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPATK_IV) as u8;
            size = 1;
        }
        REQUEST_SPDEF_IV_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPDEF_IV) as u8;
            size = 1;
        }
        REQUEST_PERSONALITY_BATTLE => {
            data32 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_PERSONALITY);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            *dst.at(3) = ((data32 & 0xFF000000) >> 24) as u8;
            size = 4;
        }
        REQUEST_CHECKSUM_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_CHECKSUM) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_STATUS_BATTLE => {
            data32 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_STATUS);
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            *dst.at(3) = ((data32 & 0xFF000000) >> 24) as u8;
            size = 4;
        }
        REQUEST_LEVEL_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_LEVEL) as u8;
            size = 1;
        }
        REQUEST_HP_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_HP) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_MAX_HP_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_MAX_HP) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_ATK_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_ATK) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_DEF_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_DEF) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPEED_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPEED) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPATK_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPATK) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPDEF_BATTLE => {
            data16 = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SPDEF) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_COOL_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_COOL) as u8;
            size = 1;
        }
        REQUEST_BEAUTY_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_BEAUTY) as u8;
            size = 1;
        }
        REQUEST_CUTE_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_CUTE) as u8;
            size = 1;
        }
        REQUEST_SMART_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SMART) as u8;
            size = 1;
        }
        REQUEST_TOUGH_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_TOUGH) as u8;
            size = 1;
        }
        REQUEST_SHEEN_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SHEEN) as u8;
            size = 1;
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_COOL_RIBBON) as u8;
            size = 1;
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_BEAUTY_RIBBON) as u8;
            size = 1;
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_CUTE_RIBBON) as u8;
            size = 1;
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_SMART_RIBBON) as u8;
            size = 1;
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            *dst = GetMonData2(crate::gl!(&raw gEnemyParty[monId]), MON_DATA_TOUGH_RIBBON) as u8;
            size = 1;
        }
        _ => {}
    }
    size as u32
}
pub(crate) unsafe fn OpponentHandleGetRawMonData() {
    let mut battleMon: BattlePokemon = zeroed();
    let src: *mut u8 = (&raw mut (*gEnemyParty.as_ptr())[{
        let gl_i67 = gActiveBattler.get();
        crate::gl!(gBattlerPartyIndexes[gl_i67])
    }] as *mut u8)
        .at({
            let gl_i68 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i68][1])
        });
    let dst: *mut u8 = (&raw mut battleMon as *mut u8).at({
        let gl_i69 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i69][1])
    });
    for i in 0..({
        let gl_i70 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i70][2])
    }) {
        *dst.at(i) = *src.at(i);
    }
    BtlController_EmitDataTransfer(
        B_COMM_TO_ENGINE,
        ({
            let gl_i71 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i71][2])
        }) as u16,
        dst as *mut c_void,
    );
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleSetMonData() {
    let mut monToCheck: u8 = 0;
    if ({
        let gl_i72 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i72][2])
    }) == 0
    {
        SetOpponentMonData(
            ({
                let gl_i73 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i73])
            }) as u8,
        );
    } else {
        monToCheck = {
            let gl_i74 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i74][2])
        };
        for i in 0..(PARTY_SIZE as u8) {
            if monToCheck as i32 & 1 != 0 {
                SetOpponentMonData(i);
            }
            monToCheck >>= 1;
        }
    }
    OpponentBufferExecCompleted();
}
unsafe fn SetOpponentMonData(monId: u8) {
    let battlePokemon: *mut BattlePokemon =
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][3] as *mut BattlePokemon;
    let moveData: *mut MovePPInfo =
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][3] as *mut MovePPInfo;
    match {
        let gl_i75 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i75][1])
    } {
        REQUEST_ALL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPECIES,
                &raw mut (*battlePokemon).species as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_HELD_ITEM,
                &raw mut (*battlePokemon).item as *mut c_void,
            );
            for i in 0..MAX_MON_MOVES {
                SetMonData(
                    crate::gl!(&raw gEnemyParty[monId]),
                    MON_DATA_MOVE1 + i,
                    &raw mut (*battlePokemon).moves[i] as *mut c_void,
                );
                SetMonData(
                    crate::gl!(&raw gEnemyParty[monId]),
                    MON_DATA_PP1 + i,
                    &raw mut (*battlePokemon).pp[i] as *mut c_void,
                );
            }
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_PP_BONUSES,
                &raw mut (*battlePokemon).ppBonuses as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_FRIENDSHIP,
                &raw mut (*battlePokemon).friendship as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_EXP,
                &raw mut (*battlePokemon).experience as *mut c_void,
            );
            let mut iv: u8 = (*battlePokemon).hpIV() as u8;
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_HP_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).attackIV() as u8;
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_ATK_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).defenseIV() as u8;
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_DEF_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).speedIV() as u8;
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPEED_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).spAttackIV() as u8;
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPATK_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).spDefenseIV() as u8;
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPDEF_IV,
                &raw mut iv as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_PERSONALITY,
                &raw mut (*battlePokemon).personality as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_STATUS,
                &raw mut (*battlePokemon).status1 as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_LEVEL,
                &raw mut (*battlePokemon).level as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_HP,
                &raw mut (*battlePokemon).hp as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_MAX_HP,
                &raw mut (*battlePokemon).maxHP as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_ATK,
                &raw mut (*battlePokemon).attack as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_DEF,
                &raw mut (*battlePokemon).defense as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPEED,
                &raw mut (*battlePokemon).speed as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPATK,
                &raw mut (*battlePokemon).spAttack as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPDEF,
                &raw mut (*battlePokemon).spDefense as *mut c_void,
            );
        }
        REQUEST_SPECIES_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPECIES,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_HELDITEM_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_HELD_ITEM,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MOVES_PP_BATTLE => {
            for i in 0..MAX_MON_MOVES {
                SetMonData(
                    crate::gl!(&raw gEnemyParty[monId]),
                    MON_DATA_MOVE1 + i,
                    &raw mut (*moveData).moves[i] as *mut c_void,
                );
                SetMonData(
                    crate::gl!(&raw gEnemyParty[monId]),
                    MON_DATA_PP1 + i,
                    &raw mut (*moveData).pp[i] as *mut c_void,
                );
            }
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_PP_BONUSES,
                &raw mut (*moveData).ppBonuses as *mut c_void,
            );
        }
        4 | REQUEST_MOVE2_BATTLE | REQUEST_MOVE3_BATTLE | REQUEST_MOVE4_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_MOVE1
                    + ({
                        let gl_i76 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i76][1])
                    }) as i32
                    - REQUEST_MOVE1_BATTLE,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_PP_DATA_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_PP1,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_PP2,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][4] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_PP3,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][5] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_PP4,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][6] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
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
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_PP1
                    + ({
                        let gl_i77 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i77][1])
                    }) as i32
                    - REQUEST_PPMOVE1_BATTLE as i32,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_OTID_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_OT_ID,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_EXP_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_EXP,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_HP_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_HP_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_ATK_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_ATK_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_DEF_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_DEF_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPEED_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPATK_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_EV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPDEF_EV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_FRIENDSHIP,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_POKERUS_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_POKERUS,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MET_LOCATION_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_MET_LOCATION,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MET_LEVEL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_MET_LEVEL,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MET_GAME_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_MET_GAME,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_POKEBALL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_POKEBALL,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_ALL_IVS_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_HP_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_ATK_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][4] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_DEF_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][5] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPEED_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][6] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPATK_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][7] as *mut c_void,
            );
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPDEF_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][8] as *mut c_void,
            );
        }
        REQUEST_HP_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_HP_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_ATK_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_ATK_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_DEF_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_DEF_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPEED_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPATK_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_IV_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPDEF_IV,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_PERSONALITY_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_PERSONALITY,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_CHECKSUM_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_CHECKSUM,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_STATUS_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_STATUS,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_LEVEL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_LEVEL,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_HP_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_HP,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_MAX_HP_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_MAX_HP,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_ATK_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_ATK,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_DEF_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_DEF,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPEED,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPATK,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SPDEF,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_COOL_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_COOL,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_BEAUTY,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_CUTE,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SMART_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SMART,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_TOUGH,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SHEEN_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SHEEN,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_COOL_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_BEAUTY_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_CUTE_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_SMART_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            SetMonData(
                crate::gl!(&raw gEnemyParty[monId]),
                MON_DATA_TOUGH_RIBBON,
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][3] as *mut c_void,
            );
        }
        _ => {}
    }
}
pub(crate) unsafe fn OpponentHandleSetRawMonData() {
    let dst: *mut u8 = (&raw mut (*gEnemyParty.as_ptr())[{
        let gl_i78 = gActiveBattler.get();
        crate::gl!(gBattlerPartyIndexes[gl_i78])
    }] as *mut u8)
        .at({
            let gl_i79 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i79][1])
        });
    for i in 0..({
        let gl_i80 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i80][2])
    }) {
        *dst.at(i) = {
            let gl_i81 = gActiveBattler.get();
            let gl_i82 = 3 + i as i32;
            crate::gl!(gBattleBufferA[gl_i81][gl_i82])
        };
    }
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleLoadMonSprite() {
    let species: u16 = GetMonData2(
        &raw mut (*gEnemyParty.as_ptr())[{
            let gl_i83 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i83])
        }],
        MON_DATA_SPECIES,
    ) as u16;
    BattleLoadOpponentMonSpriteGfx(
        &raw mut (*gEnemyParty.as_ptr())[{
            let gl_i84 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i84])
        }],
        gActiveBattler.get(),
    );
    SetMultiuseSpriteTemplateToPokemon(species, GetBattlerPosition(gActiveBattler.get()));
    {
        let gl_v = CreateSprite(
            crate::gl!(&raw gMultiuseSpriteTemplate),
            GetBattlerSpriteCoord(gActiveBattler.get(), BATTLER_COORD_X_2) as i16,
            GetBattlerSpriteDefault_Y(gActiveBattler.get()) as i16,
            GetBattlerSpriteSubpriority(gActiveBattler.get()),
        );
        let gl_i85 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i85] = gl_v)
    };
    (*gSprites.as_ptr())[{
        let gl_i86 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i86])
    }]
    .x2 = -240;
    (*gSprites.as_ptr())[{
        let gl_i87 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i87])
    }]
    .data[0] = gActiveBattler.get() as i16;
    (*gSprites.as_ptr())[{
        let gl_i88 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i88])
    }]
    .data[2] = species as i16;
    (*gSprites.as_ptr())[{
        let gl_i89 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i89])
    }]
    .oam
    .set_paletteNum(gActiveBattler.get() as u16);
    StartSpriteAnim(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i90 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i90])
        }],
        {
            let gl_i91 = gActiveBattler.get();
            crate::gl!(gBattleMonForms[gl_i91])
        },
    );
    SetBattlerShadowSpriteCallback(
        gActiveBattler.get(),
        GetMonData2(
            &raw mut (*gEnemyParty.as_ptr())[{
                let gl_i92 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i92])
            }],
            MON_DATA_SPECIES,
        ) as u16,
    );
    {
        let gl_i93 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i93] = Some(TryShinyAnimAfterMonAnim))
    };
}
pub(crate) unsafe fn OpponentHandleSwitchInAnim() {
    *(*crate::gl!(gBattleStruct))
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler.get()) = PARTY_SIZE as u8;
    {
        let gl_i94 = gActiveBattler.get();
        crate::gl!(
            gBattlerPartyIndexes[gl_i94] = ({
                let gl_i95 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i95][1])
            }) as u16
        )
    };
    StartSendOutAnim(gActiveBattler.get(), {
        let gl_i96 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i96][2])
    });
    {
        let gl_i97 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i97] = Some(SwitchIn_TryShinyAnim))
    };
}
pub(crate) unsafe fn StartSendOutAnim(battler: u8, dontClearSubstituteBit: u8) {
    ClearTemporarySpeciesSpriteData(battler, dontClearSubstituteBit);
    crate::gl!(gBattlerPartyIndexes[battler] = crate::gl!(gBattleBufferA[battler][1]) as u16);
    let species: u16 = GetMonData2(
        &raw mut (*gEnemyParty.as_ptr())[crate::gl!(gBattlerPartyIndexes[battler])],
        MON_DATA_SPECIES,
    ) as u16;
    crate::gl!(
        gBattleControllerData[battler] =
            CreateInvisibleSpriteWithCallback(Some(SpriteCB_WaitForBattlerBallReleaseAnim))
    );
    BattleLoadOpponentMonSpriteGfx(
        &raw mut (*gEnemyParty.as_ptr())[crate::gl!(gBattlerPartyIndexes[battler])],
        battler,
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
    (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])].data[0] = battler as i16;
    (*gSprites.as_ptr())[crate::gl!(gBattlerSpriteIds[battler])].data[2] = species as i16;
    (*gSprites.as_ptr())[crate::gl!(gBattleControllerData[battler])].data[1] =
        crate::gl!(gBattlerSpriteIds[battler]) as i16;
    (*gSprites.as_ptr())[crate::gl!(gBattleControllerData[battler])].data[2] = battler as i16;
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
        DoPokeballSendOutAnimation(0, POKEBALL_OPPONENT_SENDOUT) as i16;
}
pub(crate) unsafe fn OpponentHandleReturnMonToBall() {
    if ({
        let gl_i98 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i98][1])
    }) == 0
    {
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .animationState = 0;
        {
            let gl_i99 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i99] = Some(DoSwitchOutAnimation))
        };
    } else {
        FreeSpriteOamMatrix(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i100 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i100])
            }],
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i101 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i101])
            }],
        );
        HideBattlerShadowSprite(gActiveBattler.get());
        SetHealthboxSpriteInvisible({
            let gl_i102 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i102])
        });
        OpponentBufferExecCompleted();
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
                B_ANIM_SWITCH_OUT_OPPONENT_MON,
            );
            {
                let gl_i103 = gActiveBattler.get();
                crate::gl!(gBattlerControllerFuncs[gl_i103] = Some(FreeMonSpriteAfterSwitchOutAnim))
            };
        }
        _ => {}
    }
}
pub(crate) unsafe fn OpponentHandleDrawTrainerPic() {
    let mut trainerPicId: u32 = 0;
    let mut xPos: i16 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_SECRET_BASE != 0 {
        trainerPicId = GetSecretBaseTrainerPicIndex() as u32;
    } else if gTrainerBattleOpponent_A.get() == TRAINER_FRONTIER_BRAIN {
        trainerPicId = GetFrontierBrainTrainerPicIndex() as u32;
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER_HILL != 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0 {
            if gActiveBattler.get() == 1 {
                trainerPicId =
                    GetTrainerHillTrainerFrontSpriteId(gTrainerBattleOpponent_A.get()) as u32;
            } else {
                trainerPicId =
                    GetTrainerHillTrainerFrontSpriteId(gTrainerBattleOpponent_B.get()) as u32;
            }
        } else {
            trainerPicId =
                GetTrainerHillTrainerFrontSpriteId(gTrainerBattleOpponent_A.get()) as u32;
        }
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_FRONTIER != 0 {
        if gBattleTypeFlags.get() & 0x808000 != 0 {
            if gActiveBattler.get() == 1 {
                trainerPicId =
                    GetFrontierTrainerFrontSpriteId(gTrainerBattleOpponent_A.get()) as u32;
            } else {
                trainerPicId =
                    GetFrontierTrainerFrontSpriteId(gTrainerBattleOpponent_B.get()) as u32;
            }
        } else {
            trainerPicId = GetFrontierTrainerFrontSpriteId(gTrainerBattleOpponent_A.get()) as u32;
        }
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_EREADER_TRAINER != 0 {
        trainerPicId = GetEreaderTrainerFrontSpriteId() as u32;
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0 {
        if gActiveBattler.get() != 1 {
            trainerPicId = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_B.get()]
            .trainerPic as u32;
        } else {
            trainerPicId = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A.get()]
            .trainerPic as u32;
        }
    } else {
        trainerPicId = (*(&raw const crate::data::data_tables::gTrainers)
            .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A.get()]
        .trainerPic as u32;
    }
    if gBattleTypeFlags.get() & 32832 != 0 {
        if GetBattlerPosition(gActiveBattler.get()) as i32 & BIT_FLANK as i32 != 0 {
            xPos = 152;
        } else {
            xPos = 200;
        }
    } else {
        xPos = 176;
    }
    DecompressTrainerFrontPic(trainerPicId as u16, gActiveBattler.get());
    SetMultiuseSpriteTemplateToTrainerBack(
        trainerPicId as u16,
        GetBattlerPosition(gActiveBattler.get()),
    );
    {
        let gl_v = CreateSprite(
            crate::gl!(&raw gMultiuseSpriteTemplate),
            xPos,
            (8 - (*(&raw const crate::data::data_tables::gTrainerFrontPicCoords)
                .cast::<CArray<MonCoords, 0>>())[trainerPicId]
                .size as i16)
                * 4
                + 40,
            GetBattlerSpriteSubpriority(gActiveBattler.get()),
        );
        let gl_i104 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i104] = gl_v)
    };
    (*gSprites.as_ptr())[{
        let gl_i105 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i105])
    }]
    .x2 = -240;
    (*gSprites.as_ptr())[{
        let gl_i106 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i106])
    }]
    .data[sSpeedX] = 2;
    (*gSprites.as_ptr())[{
        let gl_i107 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i107])
    }]
    .oam
    .set_paletteNum(IndexOfSpritePaletteTag(
        (*(&raw const crate::data::data_tables::gTrainerFrontPicPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[trainerPicId]
            .tag,
    ) as u16);
    (*gSprites.as_ptr())[{
        let gl_i108 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i108])
    }]
    .oam
    .affineParam = trainerPicId as u16;
    (*gSprites.as_ptr())[{
        let gl_i109 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i109])
    }]
    .callback = Some(SpriteCB_TrainerSlideIn);
    {
        let gl_i110 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i110] = Some(CompleteOnBattlerSpriteCallbackDummy))
    };
}
pub(crate) unsafe fn OpponentHandleTrainerSlide() {
    let mut trainerPicId: u32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_SECRET_BASE != 0 {
        trainerPicId = GetSecretBaseTrainerPicIndex() as u32;
    } else if gTrainerBattleOpponent_A.get() == TRAINER_FRONTIER_BRAIN {
        trainerPicId = GetFrontierBrainTrainerPicIndex() as u32;
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER_HILL != 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0 {
            if gActiveBattler.get() == 1 {
                trainerPicId =
                    GetTrainerHillTrainerFrontSpriteId(gTrainerBattleOpponent_A.get()) as u32;
            } else {
                trainerPicId =
                    GetTrainerHillTrainerFrontSpriteId(gTrainerBattleOpponent_B.get()) as u32;
            }
        } else {
            trainerPicId =
                GetTrainerHillTrainerFrontSpriteId(gTrainerBattleOpponent_A.get()) as u32;
        }
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_FRONTIER != 0 {
        if gBattleTypeFlags.get() & 0x808000 != 0 {
            if gActiveBattler.get() == 1 {
                trainerPicId =
                    GetFrontierTrainerFrontSpriteId(gTrainerBattleOpponent_A.get()) as u32;
            } else {
                trainerPicId =
                    GetFrontierTrainerFrontSpriteId(gTrainerBattleOpponent_B.get()) as u32;
            }
        } else {
            trainerPicId = GetFrontierTrainerFrontSpriteId(gTrainerBattleOpponent_A.get()) as u32;
        }
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_EREADER_TRAINER != 0 {
        trainerPicId = GetEreaderTrainerFrontSpriteId() as u32;
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0 {
        if gActiveBattler.get() != 1 {
            trainerPicId = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_B.get()]
            .trainerPic as u32;
        } else {
            trainerPicId = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A.get()]
            .trainerPic as u32;
        }
    } else {
        trainerPicId = (*(&raw const crate::data::data_tables::gTrainers)
            .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A.get()]
        .trainerPic as u32;
    }
    DecompressTrainerFrontPic(trainerPicId as u16, gActiveBattler.get());
    SetMultiuseSpriteTemplateToTrainerBack(
        trainerPicId as u16,
        GetBattlerPosition(gActiveBattler.get()),
    );
    {
        let gl_v = CreateSprite(
            crate::gl!(&raw gMultiuseSpriteTemplate),
            176,
            (8 - (*(&raw const crate::data::data_tables::gTrainerFrontPicCoords)
                .cast::<CArray<MonCoords, 0>>())[trainerPicId]
                .size as i16)
                * 4
                + 40,
            0x1E,
        );
        let gl_i111 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i111] = gl_v)
    };
    (*gSprites.as_ptr())[{
        let gl_i112 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i112])
    }]
    .x2 = 96;
    (*gSprites.as_ptr())[{
        let gl_i113 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i113])
    }]
    .x += 32;
    (*gSprites.as_ptr())[{
        let gl_i114 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i114])
    }]
    .data[sSpeedX] = -2;
    (*gSprites.as_ptr())[{
        let gl_i115 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i115])
    }]
    .oam
    .set_paletteNum(IndexOfSpritePaletteTag(
        (*(&raw const crate::data::data_tables::gTrainerFrontPicPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[trainerPicId]
            .tag,
    ) as u16);
    (*gSprites.as_ptr())[{
        let gl_i116 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i116])
    }]
    .oam
    .affineParam = trainerPicId as u16;
    (*gSprites.as_ptr())[{
        let gl_i117 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i117])
    }]
    .callback = Some(SpriteCB_TrainerSlideIn);
    {
        let gl_i118 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i118] = Some(CompleteOnBankSpriteCallbackDummy2))
    };
}
pub(crate) unsafe fn OpponentHandleTrainerSlideBack() {
    SetSpritePrimaryCoordsFromSecondaryCoords(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i119 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i119])
        }],
    );
    (*gSprites.as_ptr())[{
        let gl_i120 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i120])
    }]
    .data[0] = 35;
    (*gSprites.as_ptr())[{
        let gl_i121 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i121])
    }]
    .data[2] = 280;
    (*gSprites.as_ptr())[{
        let gl_i122 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i122])
    }]
    .data[4] = (*gSprites.as_ptr())[{
        let gl_i123 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i123])
    }]
    .y;
    (*gSprites.as_ptr())[{
        let gl_i124 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i124])
    }]
    .callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i125 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i125])
        }],
        Some(SpriteCallbackDummy),
    );
    {
        let gl_i126 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i126] = Some(FreeTrainerSpriteAfterSlide))
    };
}
pub(crate) unsafe fn OpponentHandleFaintAnimation() {
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
            PlaySE12WithPanning(SE_FAINT, SOUND_PAN_TARGET);
            (*gSprites.as_ptr())[{
                let gl_i127 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i127])
            }]
            .callback = Some(SpriteCB_FaintOpponentMon);
            {
                let gl_i128 = gActiveBattler.get();
                crate::gl!(gBattlerControllerFuncs[gl_i128] = Some(HideHealthboxAfterMonFaint))
            };
        }
    }
}
pub(crate) unsafe fn OpponentHandlePaletteFade() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleSuccessBallThrowAnim() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleBallThrow() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandlePause() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleMoveAnimation() {
    if IsBattleSEPlaying(gActiveBattler.get()) == 0 {
        let r#move: u16 = ({
            let gl_i129 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i129][1])
        }) as u16
            | (({
                let gl_i130 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i130][2])
            }) as u16)
                << 8;
        gAnimMoveTurn.set({
            let gl_i131 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i131][3])
        });
        gAnimMovePower.set(
            ({
                let gl_i132 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i132][4])
            }) as u16
                | (({
                    let gl_i133 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i133][5])
                }) as u16)
                    << 8,
        );
        gAnimMoveDmg.set(
            ({
                let gl_i134 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i134][6])
            }) as i32
                | (({
                    let gl_i135 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i135][7])
                }) as i32)
                    << 8
                | (({
                    let gl_i136 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i136][8])
                }) as i32)
                    << 16
                | (({
                    let gl_i137 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i137][9])
                }) as i32)
                    << 24,
        );
        gAnimFriendship.set({
            let gl_i138 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i138][10])
        });
        gWeatherMoveAnim.set(
            ({
                let gl_i139 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i139][12])
            }) as u16
                | (({
                    let gl_i140 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i140][13])
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
            let gl_i141 = gActiveBattler.get();
            crate::gl!(
                gTransformedPersonalities[gl_i141] =
                    (*crate::gl!(gAnimDisableStructPtr)).transformedMonPersonality
            )
        };
        if IsMoveWithoutAnimation(r#move, gAnimMoveTurn.get()) != 0 {
            OpponentBufferExecCompleted();
        } else {
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .animationState = 0;
            {
                let gl_i142 = gActiveBattler.get();
                crate::gl!(gBattlerControllerFuncs[gl_i142] = Some(OpponentDoMoveAnimation))
            };
        }
    }
}
pub(crate) unsafe fn OpponentDoMoveAnimation() {
    let r#move: u16 = ({
        let gl_i143 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i143][1])
    }) as u16
        | (({
            let gl_i144 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i144][2])
        }) as u16)
            << 8;
    let multihit: u8 = {
        let gl_i145 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i145][11])
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
                    let gl_i146 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i146][1])
                }) as u16
                    | (({
                        let gl_i147 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i147][2])
                    }) as u16)
                        << 8,
            );
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .animationState = 0;
            OpponentBufferExecCompleted();
        }
        _ => {}
    }
}
pub(crate) unsafe fn OpponentHandlePrintString() {
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    let stringId: *mut u16 = &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;)
        .cast_const())
    .cast::<CArray<CArray<u8, 512>, 4>>()
    .cast_mut())[gActiveBattler.get()][2] as *mut u16;
    BufferStringBattle(*stringId);
    BattlePutTextOnWindow((*gDisplayedStringBattle.as_ptr()).as_mut_ptr(), B_WIN_MSG);
    {
        let gl_i148 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i148] = Some(CompleteOnInactiveTextPrinter))
    };
    BattleArena_DeductSkillPoints(gActiveBattler.get(), *stringId);
}
pub(crate) unsafe fn OpponentHandlePrintSelectionString() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleChooseAction() {
    AI_TrySwitchOrUseItem();
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleYesNoBox() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleChooseMove() {
    if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 {
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            B_ACTION_EXEC_SCRIPT,
            ChooseMoveAndTargetInBattlePalace(),
        );
        OpponentBufferExecCompleted();
    } else {
        let mut chosenMoveId: u8 = 0;
        let moveInfo: *mut ChooseMoveStruct =
            &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                .cast::<CArray<CArray<u8, 512>, 4>>()
                .cast_mut())[gActiveBattler.get()][4] as *mut ChooseMoveStruct;
        if gBattleTypeFlags.get() & 1176 != 0 {
            BattleAI_SetupAIData(ALL_MOVES_MASK);
            chosenMoveId = BattleAI_ChooseMoveOrAction();
            match chosenMoveId {
                AI_CHOICE_WATCH => {
                    BtlController_EmitTwoReturnValues(
                        B_COMM_TO_ENGINE,
                        B_ACTION_SAFARI_WATCH_CAREFULLY,
                        0,
                    );
                }
                AI_CHOICE_FLEE => {
                    BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_RUN, 0);
                }
                6 => {
                    BtlController_EmitTwoReturnValues(
                        B_COMM_TO_ENGINE,
                        B_ACTION_UNK_15,
                        gBattlerTarget.get() as u16,
                    );
                }
                _ => {
                    if (*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[(*moveInfo).moves[chosenMoveId]]
                        .target as i32
                        & 18
                        != 0
                    {
                        gBattlerTarget.set(gActiveBattler.get());
                    }
                    if (*(&raw const crate::data::pokemon::gBattleMoves)
                        .cast::<CArray<BattleMove, 0>>())[(*moveInfo).moves[chosenMoveId]]
                        .target as i32
                        & MOVE_TARGET_BOTH as i32
                        != 0
                    {
                        gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_PLAYER_LEFT));
                        if gAbsentBattlerFlags.get() as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [gBattlerTarget.get()]
                            != 0
                        {
                            gBattlerTarget.set(GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT));
                        }
                    }
                    BtlController_EmitTwoReturnValues(
                        B_COMM_TO_ENGINE,
                        B_ACTION_EXEC_SCRIPT,
                        chosenMoveId as u16 | (gBattlerTarget.get() as u16) << 8,
                    );
                }
            }
            OpponentBufferExecCompleted();
        } else {
            let mut r#move: u16 = 0;
            loop {
                chosenMoveId = (if 0 != 0 {
                    Random() as i32 % 4
                } else {
                    Random() as i32 & 3
                }) as u8;
                r#move = (*moveInfo).moves[chosenMoveId];
                if r#move != MOVE_NONE {
                    break;
                }
            }
            if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [r#move]
                .target as i32
                & 18
                != 0
            {
                BtlController_EmitTwoReturnValues(
                    B_COMM_TO_ENGINE,
                    B_ACTION_EXEC_SCRIPT,
                    chosenMoveId as u16 | (gActiveBattler.get() as u16) << 8,
                );
            } else if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
                BtlController_EmitTwoReturnValues(
                    B_COMM_TO_ENGINE,
                    B_ACTION_EXEC_SCRIPT,
                    chosenMoveId as u16 | (GetBattlerAtPosition(Random() as u8 & 2) as u16) << 8,
                );
            } else {
                BtlController_EmitTwoReturnValues(
                    B_COMM_TO_ENGINE,
                    B_ACTION_EXEC_SCRIPT,
                    chosenMoveId as u16
                        | (GetBattlerAtPosition(B_POSITION_PLAYER_LEFT) as u16) << 8,
                );
            }
            OpponentBufferExecCompleted();
        }
    }
}
pub(crate) unsafe fn OpponentHandleChooseItem() {
    BtlController_EmitOneReturnValue(
        B_COMM_TO_ENGINE,
        *(*crate::gl!(gBattleStruct))
            .chosenItem
            .as_mut_ptr()
            .at(gActiveBattler.get() as i32 / 2 * 2) as u16,
    );
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleChoosePokemon() {
    let mut chosenMonId: i32 = 0;
    if *(*crate::gl!(gBattleStruct))
        .AI_monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler.get())
        == PARTY_SIZE as u8
    {
        chosenMonId = GetMostSuitableMonToSwitchInto() as i32;
        if chosenMonId == PARTY_SIZE {
            let mut battler1: i32 = 0;
            let mut battler2: i32 = 0;
            let mut firstId: i32 = 0;
            let mut lastId: i32 = 0;
            if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE == 0 {
                battler2 = {
                    battler1 = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT) as i32;
                    battler1
                };
            } else {
                battler1 = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT) as i32;
                battler2 = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT) as i32;
            }
            if gBattleTypeFlags.get() & 0x808000 != 0 {
                if gActiveBattler.get() == 1 {
                    firstId = 0;
                    lastId = 3;
                } else {
                    firstId = 3;
                    lastId = PARTY_SIZE;
                }
            } else {
                firstId = 0;
                lastId = PARTY_SIZE;
            }
            for chosenMonId in firstId..lastId {
                if GetMonData2(crate::gl!(&raw gEnemyParty[chosenMonId]), MON_DATA_HP) != 0
                    && chosenMonId != crate::gl!(gBattlerPartyIndexes[battler1]) as i32
                    && chosenMonId != crate::gl!(gBattlerPartyIndexes[battler2]) as i32
                {
                    break;
                }
            }
        }
    } else {
        chosenMonId = *(*crate::gl!(gBattleStruct))
            .AI_monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler.get()) as i32;
        *(*crate::gl!(gBattleStruct))
            .AI_monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler.get()) = PARTY_SIZE as u8;
    }
    *(*crate::gl!(gBattleStruct))
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler.get()) = chosenMonId as u8;
    BtlController_EmitChosenMonReturnValue(B_COMM_TO_ENGINE, chosenMonId as u8, null_mut());
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleCmd23() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleHealthBarUpdate() {
    LoadBattleBarGfx(0);
    let hpVal: i16 = (({
        let gl_i149 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i149][3])
    }) as i16)
        << 8
        | ({
            let gl_i150 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i150][2])
        }) as i16;
    if hpVal != INSTANT_HP_BAR_DROP {
        let maxHP: u32 = GetMonData2(
            &raw mut (*gEnemyParty.as_ptr())[{
                let gl_i151 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i151])
            }],
            MON_DATA_MAX_HP,
        );
        let curHP: u32 = GetMonData2(
            &raw mut (*gEnemyParty.as_ptr())[{
                let gl_i152 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i152])
            }],
            MON_DATA_HP,
        );
        SetBattleBarStruct(
            gActiveBattler.get(),
            {
                let gl_i153 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i153])
            },
            maxHP as i32,
            curHP as i32,
            hpVal as i32,
        );
    } else {
        let maxHP: u32 = GetMonData2(
            &raw mut (*gEnemyParty.as_ptr())[{
                let gl_i154 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i154])
            }],
            MON_DATA_MAX_HP,
        );
        SetBattleBarStruct(
            gActiveBattler.get(),
            {
                let gl_i155 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i155])
            },
            maxHP as i32,
            0,
            hpVal as i32,
        );
    }
    {
        let gl_i156 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i156] = Some(CompleteOnHealthbarDone))
    };
}
pub(crate) unsafe fn OpponentHandleExpUpdate() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleStatusIconUpdate() {
    if IsBattleSEPlaying(gActiveBattler.get()) == 0 {
        UpdateHealthboxAttribute(
            {
                let gl_i157 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i157])
            },
            &raw mut (*gEnemyParty.as_ptr())[{
                let gl_i158 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i158])
            }],
            HEALTHBOX_STATUS_ICON,
        );
        let battler: u8 = gActiveBattler.get();
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .set_statusAnimActive(0);
        {
            let gl_i159 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i159] = Some(CompleteOnFinishedStatusAnimation))
        };
    }
}
pub(crate) unsafe fn OpponentHandleStatusAnimation() {
    if IsBattleSEPlaying(gActiveBattler.get()) == 0 {
        InitAndLaunchChosenStatusAnimation(
            {
                let gl_i160 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i160][1])
            },
            ({
                let gl_i161 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i161][2])
            }) as u32
                | (({
                    let gl_i162 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i162][3])
                }) as u32)
                    << 8
                | (({
                    let gl_i163 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i163][4])
                }) as u32)
                    << 16
                | (({
                    let gl_i164 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i164][5])
                }) as u32)
                    << 24,
        );
        {
            let gl_i165 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i165] = Some(CompleteOnFinishedStatusAnimation))
        };
    }
}
pub(crate) unsafe fn OpponentHandleStatusXor() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleDataTransfer() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleDMA3Transfer() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandlePlayBGM() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleCmd32() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleTwoReturnValues() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleChosenMonReturnValue() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleOneReturnValue() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleOneReturnValue_Duplicate() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleClearUnkVar() {
    (*gUnusedControllerStruct.as_ptr()).set_unk(0);
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleSetUnkVar() {
    (*gUnusedControllerStruct.as_ptr()).set_unk({
        let gl_i166 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i166][1])
    });
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleClearUnkFlag() {
    (*gUnusedControllerStruct.as_ptr()).set_flag(0);
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleToggleUnkFlag() {
    (*gUnusedControllerStruct.as_ptr()).set_flag((*gUnusedControllerStruct.as_ptr()).flag() ^ 1);
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleHitAnimation() {
    if (*gSprites.as_ptr())[{
        let gl_i167 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i167])
    }]
    .invisible()
        == TRUE as u16
    {
        OpponentBufferExecCompleted();
    } else {
        gDoingBattleAnim.set(TRUE);
        (*gSprites.as_ptr())[{
            let gl_i168 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i168])
        }]
        .data[1] = 0;
        DoHitAnimHealthboxEffect(gActiveBattler.get());
        {
            let gl_i169 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i169] = Some(DoHitAnimBlinkSpriteEffect))
        };
    }
}
pub(crate) unsafe fn OpponentHandleCantSwitch() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandlePlaySE() {
    let mut pan: i8 = 0;
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        pan = SOUND_PAN_ATTACKER;
    } else {
        pan = SOUND_PAN_TARGET;
    }
    PlaySE12WithPanning(
        ({
            let gl_i170 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i170][1])
        }) as u16
            | (({
                let gl_i171 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i171][2])
            }) as u16)
                << 8,
        pan,
    );
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandlePlayFanfareOrBGM() {
    if ({
        let gl_i172 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i172][3])
    }) != 0
    {
        BattleStopLowHpSound();
        PlayBGM(
            ({
                let gl_i173 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i173][1])
            }) as u16
                | (({
                    let gl_i174 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i174][2])
                }) as u16)
                    << 8,
        );
    } else {
        PlayFanfare(
            ({
                let gl_i175 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i175][1])
            }) as u16
                | (({
                    let gl_i176 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i176][2])
                }) as u16)
                    << 8,
        );
    }
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleFaintingCry() {
    let species: u16 = GetMonData2(
        &raw mut (*gEnemyParty.as_ptr())[{
            let gl_i177 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i177])
        }],
        MON_DATA_SPECIES,
    ) as u16;
    PlayCry_ByMode(species, 25, CRY_MODE_FAINT);
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleIntroSlide() {
    HandleIntroSlide({
        let gl_i178 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i178][1])
    });
    gIntroSlideFlags.set(gIntroSlideFlags.get() | (1));
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleIntroTrainerBallThrow() {
    SetSpritePrimaryCoordsFromSecondaryCoords(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i179 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i179])
        }],
    );
    (*gSprites.as_ptr())[{
        let gl_i180 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i180])
    }]
    .data[0] = 35;
    (*gSprites.as_ptr())[{
        let gl_i181 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i181])
    }]
    .data[2] = 280;
    (*gSprites.as_ptr())[{
        let gl_i182 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i182])
    }]
    .data[4] = (*gSprites.as_ptr())[{
        let gl_i183 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i183])
    }]
    .y;
    (*gSprites.as_ptr())[{
        let gl_i184 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i184])
    }]
    .callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i185 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i185])
        }],
        Some(SpriteCB_FreeOpponentSprite),
    );
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
                let gl_i186 = gActiveBattler.get();
                crate::gl!(gBattlerStatusSummaryTaskId[gl_i186])
            },
            Some(Task_HidePartyStatusSummary),
        );
    }
    (*(*crate::gl!(gBattleSpritesDataPtr)).animationData).set_introAnimActive(TRUE);
    {
        let gl_i187 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i187] = Some(OpponentDummy))
    };
}
pub(crate) unsafe fn SpriteCB_FreeOpponentSprite(sprite: *mut Sprite) {
    FreeTrainerFrontPicPalette((*sprite).oam.affineParam);
    FreeSpriteOamMatrix(sprite);
    DestroySprite(sprite);
}
pub(crate) unsafe fn Task_StartSendOutAnim(taskId: u8) {
    let savedActiveBank: u8 = gActiveBattler.get();
    gActiveBattler.set(task_get(taskId, 0) as u8);
    if IsDoubleBattle() == 0 || gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
        {
            let gl_i188 = gActiveBattler.get();
            crate::gl!(
                gBattleBufferA[gl_i188][1] = ({
                    let gl_i189 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i189])
                }) as u8
            )
        };
        StartSendOutAnim(gActiveBattler.get(), FALSE);
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0 {
        {
            let gl_i190 = gActiveBattler.get();
            crate::gl!(
                gBattleBufferA[gl_i190][1] = ({
                    let gl_i191 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i191])
                }) as u8
            )
        };
        StartSendOutAnim(gActiveBattler.get(), FALSE);
    } else {
        {
            let gl_i192 = gActiveBattler.get();
            crate::gl!(
                gBattleBufferA[gl_i192][1] = ({
                    let gl_i193 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i193])
                }) as u8
            )
        };
        StartSendOutAnim(gActiveBattler.get(), FALSE);
        gActiveBattler.set(gActiveBattler.get() ^ (BIT_FLANK));
        {
            let gl_i194 = gActiveBattler.get();
            crate::gl!(
                gBattleBufferA[gl_i194][1] = ({
                    let gl_i195 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i195])
                }) as u8
            )
        };
        StartSendOutAnim(gActiveBattler.get(), FALSE);
        gActiveBattler.set(gActiveBattler.get() ^ (BIT_FLANK));
    }
    {
        let gl_i196 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i196] = Some(Intro_TryShinyAnimShowHealthbox))
    };
    gActiveBattler.set(savedActiveBank);
    DestroyTask(taskId);
}
pub(crate) unsafe fn OpponentHandleDrawPartyStatusSummary() {
    if ({
        let gl_i197 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i197][1])
    }) != 0
        && GetBattlerSide(gActiveBattler.get()) == 0
    {
        OpponentBufferExecCompleted();
    } else {
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .set_partyStatusSummaryShown(1);
        if ({
            let gl_i198 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i198][2])
        }) != 0
        {
            if (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .opponentDrawPartyStatusSummaryDelay()
                < 2
            {
                (*(*crate::gl!(gBattleSpritesDataPtr))
                    .healthBoxesData
                    .at(gActiveBattler.get()))
                .set_opponentDrawPartyStatusSummaryDelay(
                    (*(*crate::gl!(gBattleSpritesDataPtr))
                        .healthBoxesData
                        .at(gActiveBattler.get()))
                    .opponentDrawPartyStatusSummaryDelay()
                        + 1,
                );
                return;
            } else {
                (*(*crate::gl!(gBattleSpritesDataPtr))
                    .healthBoxesData
                    .at(gActiveBattler.get()))
                .set_opponentDrawPartyStatusSummaryDelay(0);
            }
        }
        {
            let gl_v = CreatePartyStatusSummarySprites(
                gActiveBattler.get(),
                &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler.get()][4] as *mut HpAndStatus,
                {
                    let gl_i200 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i200][1])
                },
                {
                    let gl_i201 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i201][2])
                },
            );
            let gl_i199 = gActiveBattler.get();
            crate::gl!(gBattlerStatusSummaryTaskId[gl_i199] = gl_v)
        };
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .partyStatusDelayTimer = 0;
        if ({
            let gl_i202 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i202][2])
        }) != 0
        {
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .partyStatusDelayTimer = 93;
        }
        {
            let gl_i203 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i203] = Some(EndDrawPartyStatusSummary))
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
        OpponentBufferExecCompleted();
    }
}
pub(crate) unsafe fn OpponentHandleHidePartyStatusSummary() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .partyStatusSummaryShown()
        != 0
    {
        task_set_func(
            {
                let gl_i204 = gActiveBattler.get();
                crate::gl!(gBattlerStatusSummaryTaskId[gl_i204])
            },
            Some(Task_HidePartyStatusSummary),
        );
    }
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleEndBounceEffect() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleSpriteInvisibility() {
    if IsBattlerSpritePresent(gActiveBattler.get()) != 0 {
        (*gSprites.as_ptr())[{
            let gl_i205 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i205])
        }]
        .set_invisible(
            ({
                let gl_i206 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i206][1])
            }) as u16,
        );
        CopyBattleSpriteInvisibility(gActiveBattler.get());
    }
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleBattleAnimation() {
    if IsBattleSEPlaying(gActiveBattler.get()) == 0 {
        let animationId: u8 = {
            let gl_i207 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i207][1])
        };
        let argument: u16 = ({
            let gl_i208 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i208][2])
        }) as u16
            | (({
                let gl_i209 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i209][3])
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
            OpponentBufferExecCompleted();
        } else {
            {
                let gl_i210 = gActiveBattler.get();
                crate::gl!(
                    gBattlerControllerFuncs[gl_i210] = Some(CompleteOnFinishedBattleAnimation)
                )
            };
        }
    }
}
pub(crate) unsafe fn OpponentHandleLinkStandbyMsg() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleResetActionMoveSelection() {
    OpponentBufferExecCompleted();
}
pub(crate) unsafe fn OpponentHandleEndLinkBattle() {
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_IS_MASTER == 0
    {
        (*gMain.as_ptr()).set_inBattle(FALSE);
        crate::gl!(gMain.callback1 = crate::gl!(gPreBattleCallback1));
        SetMainCallback2(crate::gl!(gMain.savedCallback));
    }
    OpponentBufferExecCompleted();
}
pub(crate) fn OpponentCmdEnd() {}
