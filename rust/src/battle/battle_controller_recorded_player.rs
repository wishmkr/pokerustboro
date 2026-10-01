//! Translated from `src/battle_controller_recorded_player.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    clippy::unnecessary_cast,
    unused_assignments
)]

use crate::battle_anim::{
    DoMoveAnim, gAnimDisableStructPtr, gAnimFriendship, gAnimMoveDmg, gAnimMovePower,
    gAnimMoveTurn, gAnimScriptActive, gAnimScriptCallback, gWeatherMoveAnim,
};
use crate::battle_anim_mons::{
    GetBattlerPosition, GetBattlerSide, GetBattlerSpriteCoord, GetBattlerSpriteDefault_Y,
    GetBattlerSpriteSubpriority, IsBattlerSpritePresent, IsDoubleBattle,
    SetSpritePrimaryCoordsFromSecondaryCoords, StartAnimLinearTranslation,
    StoreSpriteCallbackInData6,
};
use crate::battle_anim_throw::TryShinyAnimation;
use crate::battle_controller_player::{
    SetBattleEndCallbacks, SpriteCB_FreePlayerSpriteLoadMonSprite,
    Task_PlayerController_RestoreBgmAfterCry,
};
use crate::battle_controllers::{
    BtlController_EmitChosenMonReturnValue, BtlController_EmitDataTransfer,
    BtlController_EmitTwoReturnValues, PrepareBufferDataTransferLink, gUnusedControllerStruct,
};
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
    SetHealthboxSpriteInvisible, SetHealthboxSpriteVisible, Task_HidePartyStatusSummary,
    UpdateHealthboxAttribute, UpdateHpTextInHealthbox,
};
use crate::battle_intro::HandleIntroSlide;
use crate::battle_main::{
    SpriteCB_FaintSlideAnim, gActiveBattler, gBattle_BG0_X, gBattle_BG0_Y,
    gBattleControllerExecFlags, gBattleOutcome, gBattleSpritesDataPtr, gBattleStruct,
    gBattleTypeFlags, gBattlerControllerFuncs, gBattlersCount, gDoingBattleAnim, gIntroSlideFlags,
    gTransformedPersonalities,
};
use crate::battle_main::{
    gBattleBufferA, gBattleCommunication, gBattleControllerData, gBattleMonForms,
    gBattlerPartyIndexes, gBattlerSpriteIds, gBattlerStatusSummaryTaskId, gDisplayedStringBattle,
    gHealthboxSpriteIds,
};
use crate::battle_message::{BattlePutTextOnWindow, BufferStringBattle};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::link::{GetMultiplayerId, gLinkPlayers};
use crate::load_save::gSaveBlock2Ptr;
use crate::m4a::{gMPlayInfo_BGM, m4aMPlayContinue, m4aMPlayVolumeControl};
use crate::palette::{BeginFastPaletteFade, LoadCompressedPalette};
use crate::pokeball::{
    DoHitAnimHealthboxEffect, DoPokeballSendOutAnimation, StartHealthboxSlideIn,
};
use crate::pokemon::{
    GetBattlerMultiplayerId, GetMonData2, GetMonData3, PlayerGenderToFrontTrainerPicId, SetMonData,
    SetMultiuseSpriteTemplateToPokemon, SetMultiuseSpriteTemplateToTrainerBack,
    SetMultiuseSpriteTemplateToTrainerFront, gMultiuseSpriteTemplate, gPlayerParty,
};
use crate::recorded_battle::{
    GetActiveBattlerLinkPlayerGender, RecordedBattle_GetBattlerAction, gRecordedBattleMultiplayerId,
};
use crate::sound::{
    FadeOutMapMusic, IsCryPlayingOrClearCrySongs, PlayBGM, PlayCry_ByMode, PlayFanfare,
    PlaySE12WithPanning,
};
use crate::sprite::gSprites;
use crate::sprite::{
    AllocSpritePalette, FreeOamMatrix, FreeSpritePaletteByTag, FreeSpriteTilesByTag,
    IndexOfSpritePaletteTag,
};
use crate::task::DestroyTask;
use crate::task::{task_get, task_set, task_set_func};
use crate::text::IsTextPrinterActive;
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
unsafe fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe fn(*mut Sprite)>) -> u8 {
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
const sSpeedY: usize = 2;
// Data tables (translate with cdata.py): sRecordedPlayerBufferCommands

static sRecordedPlayerBufferCommands: Table<CArray<Option<unsafe fn()>, 57>> = Table(
    (&raw const crate::data::battle_controller_recorded_player::sRecordedPlayerBufferCommands)
        .cast(),
);

pub(crate) unsafe fn RecordedPlayerDummy() {}
pub unsafe fn SetControllerToRecordedPlayer() {
    gBattlerControllerFuncs[gActiveBattler] = Some(RecordedPlayerBufferRunCommand);
}
pub(crate) unsafe fn RecordedPlayerBufferRunCommand() {
    if gBattleControllerExecFlags
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler]
        != 0
    {
        if gBattleBufferA[gActiveBattler][0] < 57 {
            sRecordedPlayerBufferCommands[gBattleBufferA[gActiveBattler][0]].unwrap_unchecked()();
        } else {
            RecordedPlayerBufferExecCompleted();
        }
    }
}
pub(crate) unsafe fn CompleteOnBattlerSpriteCallbackDummy() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn FreeTrainerSpriteAfterSlide() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        BattleGfxSfxDummy3(MALE);
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn Intro_DelayAndEnd() {
    if ({
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay -= 1;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay
    }) == 255
    {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay = 0;
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn Intro_WaitForShinyAnimAndHealthbox() {
    let mut healthboxAnimDone: u32 = FALSE as u32;
    if GetBattlerPosition(gActiveBattler) == B_POSITION_PLAYER_LEFT {
        if IsDoubleBattle() == 0
            || IsDoubleBattle() != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
        {
            if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                healthboxAnimDone = TRUE as u32;
            }
        } else {
            if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
                && gSprites[gHealthboxSpriteIds[gActiveBattler as i32 ^ 2]].callback
                    == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                healthboxAnimDone = TRUE as u32;
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
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
                .set_triedShinyMonAnim(FALSE);
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
    } else {
        if IsDoubleBattle() == 0
            || IsDoubleBattle() != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
        {
            if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                healthboxAnimDone = TRUE as u32;
            }
        } else {
            if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
                && gSprites[gHealthboxSpriteIds[gActiveBattler as i32 ^ 2]].callback
                    == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                healthboxAnimDone = TRUE as u32;
            }
        }
        if IsCryPlayingOrClearCrySongs() != 0 {
            healthboxAnimDone = FALSE as u32;
        }
        if healthboxAnimDone != 0 {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay = 3;
            gBattlerControllerFuncs[gActiveBattler] = Some(Intro_DelayAndEnd);
        }
    }
}
pub(crate) unsafe fn Intro_TryShinyAnimShowHealthbox() {
    let mut bgmRestored: u32 = FALSE as u32;
    if GetBattlerPosition(gActiveBattler) == B_POSITION_PLAYER_LEFT {
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
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).healthboxSlideInStarted() != 0
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).waitForCry() == 0
        && (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gActiveBattler as i32 ^ 2))
        .waitForCry()
            == 0
        && IsCryPlayingOrClearCrySongs() == 0
    {
        if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).bgmRestored() == 0 {
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
            {
                if GetBattlerPosition(gActiveBattler) == B_POSITION_PLAYER_LEFT {
                    m4aMPlayContinue(&raw mut gMPlayInfo_BGM);
                }
            } else {
                m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
            }
        }
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_bgmRestored(TRUE);
        bgmRestored = TRUE as u32;
    }
    if bgmRestored != 0
        && gSprites[gBattleControllerData[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && gSprites[gBattlerSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
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
pub(crate) unsafe fn WaitForMonAnimAfterLoad() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].animEnded() != 0
        && gSprites[gBattlerSpriteIds[gActiveBattler]].x2 == 0
    {
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnHealthbarDone() {
    let hpValue: i16 = MoveBattleBar(
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
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn FreeMonSpriteAfterFaintAnim() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].y as i32
        + gSprites[gBattlerSpriteIds[gActiveBattler]].y2 as i32
        > DISPLAY_HEIGHT as i32
    {
        let species: u16 = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_SPECIES,
        ) as u16;
        BattleGfxSfxDummy2(species);
        FreeOamMatrix(gSprites[gBattlerSpriteIds[gActiveBattler]].oam.matrixNum() as u8);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn FreeMonSpriteAfterSwitchOutAnim() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0 {
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnInactiveTextPrinter() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn DoHitAnimBlinkSpriteEffect() {
    let spriteId: u8 = gBattlerSpriteIds[gActiveBattler];
    if gSprites[spriteId].data[1] == 32 {
        gSprites[spriteId].data[1] = 0;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gDoingBattleAnim = FALSE;
        RecordedPlayerBufferExecCompleted();
    } else {
        if gSprites[spriteId].data[1] % 4 == 0 {
            gSprites[spriteId].set_invisible(gSprites[spriteId].invisible() ^ 1);
        }
        gSprites[spriteId].data[1] += 1;
    }
}
pub(crate) unsafe fn SwitchIn_ShowSubstitute() {
    if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        CopyBattleSpriteInvisibility(gActiveBattler);
        if (*(*gBattleSpritesDataPtr).battlerData.at(gActiveBattler)).behindSubstitute() != 0 {
            InitAndLaunchSpecialAnimation(
                gActiveBattler,
                gActiveBattler,
                gActiveBattler,
                B_ANIM_MON_TO_SUBSTITUTE,
            );
        }
        gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_WaitAndEnd);
    }
}
pub(crate) unsafe fn SwitchIn_WaitAndEnd() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0
        && gSprites[gBattlerSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn SwitchIn_ShowHealthbox() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).finishedShinyMonAnim() != 0 {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_triedShinyMonAnim(FALSE);
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_finishedShinyMonAnim(FALSE);
        FreeSpriteTilesByTag(ANIM_TAG_GOLD_STARS);
        FreeSpritePaletteByTag(ANIM_TAG_GOLD_STARS);
        CreateTask(Some(Task_PlayerController_RestoreBgmAfterCry), 10);
        HandleLowHpMusicChange(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            gActiveBattler,
        );
        StartSpriteAnim(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]], 0);
        UpdateHealthboxAttribute(
            gHealthboxSpriteIds[gActiveBattler],
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            HEALTHBOX_ALL,
        );
        StartHealthboxSlideIn(gActiveBattler);
        SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler]);
        gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_ShowSubstitute);
    }
}
pub(crate) unsafe fn SwitchIn_TryShinyAnim() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).triedShinyMonAnim() == 0
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).ballAnimActive() == 0
    {
        TryShinyAnimation(
            gActiveBattler,
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        );
    }
    if gSprites[gBattleControllerData[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).ballAnimActive() == 0
    {
        DestroySprite(&raw mut gSprites[gBattleControllerData[gActiveBattler]]);
        gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_ShowHealthbox);
    }
}
unsafe fn RecordedPlayerBufferExecCompleted() {
    gBattlerControllerFuncs[gActiveBattler] = Some(RecordedPlayerBufferRunCommand);
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
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnFinishedBattleAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animFromTableActive() == 0 {
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn RecordedPlayerHandleGetMonData() {
    let mut monData: CArray<u8, 256> = zeroed();
    let mut size: u32 = 0;
    let mut monToCheck: u8 = 0;
    if gBattleBufferA[gActiveBattler][2] == 0 {
        size += CopyRecordedPlayerMonData(
            gBattlerPartyIndexes[gActiveBattler] as u8,
            monData.as_mut_ptr(),
        );
    } else {
        monToCheck = gBattleBufferA[gActiveBattler][2];
        for i in 0..PARTY_SIZE {
            if monToCheck as i32 & 1 != 0 {
                size += CopyRecordedPlayerMonData(i as u8, monData.as_mut_ptr().at(size));
            }
            monToCheck >>= 1;
        }
    }
    BtlController_EmitDataTransfer(
        B_COMM_TO_ENGINE,
        size as u16,
        monData.as_mut_ptr() as *mut c_void,
    );
    RecordedPlayerBufferExecCompleted();
}
unsafe fn CopyRecordedPlayerMonData(monId: u8, dst: *mut u8) -> u32 {
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
            for size in 0..MAX_MON_MOVES {
                battleMon.moves[size] =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MOVE1 + size) as u16;
                battleMon.pp[size] =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP1 + size) as u8;
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
            for size in 0..88i32 {
                *dst.at(size) = *src.at(size);
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
            for size in 0..MAX_MON_MOVES {
                moveData.moves[size] =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_MOVE1 + size) as u16;
                moveData.pp[size] =
                    GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP1 + size) as u8;
            }
            moveData.ppBonuses =
                GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PP_BONUSES) as u8;
            src = &raw mut moveData as *mut u8;
            for size in 0..16i32 {
                *dst.at(size) = *src.at(size);
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
            *dst = data32 as u8;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_EXP_BATTLE => {
            data32 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_EXP);
            *dst = data32 as u8;
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
            *dst = data32 as u8;
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
            *dst = data32 as u8;
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
    size as u32
}
pub(crate) unsafe fn RecordedPlayerHandleGetRawMonData() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleSetMonData() {
    let mut monToCheck: u8 = 0;
    if gBattleBufferA[gActiveBattler][2] == 0 {
        SetRecordedPlayerMonData(gBattlerPartyIndexes[gActiveBattler] as u8);
    } else {
        monToCheck = gBattleBufferA[gActiveBattler][2];
        for i in 0..(PARTY_SIZE as u8) {
            if monToCheck as i32 & 1 != 0 {
                SetRecordedPlayerMonData(i);
            }
            monToCheck >>= 1;
        }
    }
    RecordedPlayerBufferExecCompleted();
}
unsafe fn SetRecordedPlayerMonData(monId: u8) {
    let battlePokemon: *mut BattlePokemon =
        &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler][3] as *mut BattlePokemon;
    let moveData: *mut MovePPInfo = &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
        .cast::<CArray<CArray<u8, 512>, 4>>()
        .cast_mut())[gActiveBattler][3] as *mut MovePPInfo;
    match gBattleBufferA[gActiveBattler][1] {
        REQUEST_ALL_BATTLE => {
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
            for i in 0..MAX_MON_MOVES {
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
            let mut iv: u8 = (*battlePokemon).hpIV() as u8;
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
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_HELDITEM_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HELD_ITEM,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MOVES_PP_BATTLE => {
            for i in 0..MAX_MON_MOVES {
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
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_PP_DATA_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP1,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP2,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][4] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP3,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][5] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP4,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][6] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PP_BONUSES,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][7] as *mut c_void,
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
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_OTID_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_OT_ID,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_EXP_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_EXP,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_HP_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP_EV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ATK_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK_EV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_DEF_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF_EV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED_EV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK_EV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_EV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF_EV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_FRIENDSHIP,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_POKERUS_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_POKERUS,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MET_LOCATION_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MET_LOCATION,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MET_LEVEL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MET_LEVEL,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MET_GAME_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MET_GAME,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_POKEBALL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_POKEBALL,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ALL_IVS_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][4] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][5] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][6] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][7] as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][8] as *mut c_void,
            );
        }
        REQUEST_HP_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ATK_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_DEF_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_IV_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF_IV,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_PERSONALITY_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_PERSONALITY,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_CHECKSUM_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_CHECKSUM,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_STATUS_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_STATUS,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_LEVEL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_LEVEL,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_HP_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_HP,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MAX_HP_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MAX_HP,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ATK_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_ATK,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_DEF_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_DEF,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPEED,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPATK,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SPDEF,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_COOL_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_COOL,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_BEAUTY,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_CUTE,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SMART_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SMART,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_TOUGH,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SHEEN_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SHEEN,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_COOL_RIBBON,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_BEAUTY_RIBBON,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_CUTE_RIBBON,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_SMART_RIBBON,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_TOUGH_RIBBON,
                &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                    .cast::<CArray<CArray<u8, 512>, 4>>()
                    .cast_mut())[gActiveBattler][3] as *mut c_void,
            );
        }
        _ => {}
    }
    HandleLowHpMusicChange(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        gActiveBattler,
    );
}
pub(crate) unsafe fn RecordedPlayerHandleSetRawMonData() {
    let dst: *mut u8 = (&raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]] as *mut u8)
        .at(gBattleBufferA[gActiveBattler][1]);
    for i in 0..gBattleBufferA[gActiveBattler][2] {
        *dst.at(i) = gBattleBufferA[gActiveBattler][3 + i as i32];
    }
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleLoadMonSprite() {
    BattleLoadPlayerMonSpriteGfx(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        gActiveBattler,
    );
    let species: u16 = GetMonData2(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        MON_DATA_SPECIES,
    ) as u16;
    SetMultiuseSpriteTemplateToPokemon(species, GetBattlerPosition(gActiveBattler));
    gBattlerSpriteIds[gActiveBattler] = CreateSprite(
        &raw mut gMultiuseSpriteTemplate,
        GetBattlerSpriteCoord(gActiveBattler, BATTLER_COORD_X_2) as i16,
        GetBattlerSpriteDefault_Y(gActiveBattler) as i16,
        GetBattlerSpriteSubpriority(gActiveBattler),
    );
    gSprites[gBattlerSpriteIds[gActiveBattler]].x2 = -240;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = gActiveBattler as i16;
    gSprites[gBattlerSpriteIds[gActiveBattler]]
        .oam
        .set_paletteNum(gActiveBattler as u16);
    StartSpriteAnim(
        &raw mut gSprites[gBattlerSpriteIds[gActiveBattler]],
        gBattleMonForms[gActiveBattler],
    );
    gBattlerControllerFuncs[gActiveBattler] = Some(WaitForMonAnimAfterLoad);
}
pub(crate) unsafe fn RecordedPlayerHandleSwitchInAnim() {
    ClearTemporarySpeciesSpriteData(gActiveBattler, gBattleBufferA[gActiveBattler][2]);
    gBattlerPartyIndexes[gActiveBattler] = gBattleBufferA[gActiveBattler][1] as u16;
    BattleLoadPlayerMonSpriteGfx(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        gActiveBattler,
    );
    StartSendOutAnim(gActiveBattler, gBattleBufferA[gActiveBattler][2]);
    gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_TryShinyAnim);
}
pub(crate) unsafe fn StartSendOutAnim(battler: u8, dontClearSubstituteBit: u8) {
    ClearTemporarySpeciesSpriteData(battler, dontClearSubstituteBit);
    gBattlerPartyIndexes[battler] = gBattleBufferA[battler][1] as u16;
    let species: u16 = GetMonData2(
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
pub(crate) unsafe fn RecordedPlayerHandleReturnMonToBall() {
    if gBattleBufferA[gActiveBattler][1] == 0 {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
        gBattlerControllerFuncs[gActiveBattler] = Some(DoSwitchOutAnimation);
    } else {
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn DoSwitchOutAnimation() {
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
        1 if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive()
            == 0 =>
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
        _ => {}
    }
}
pub(crate) unsafe fn RecordedPlayerHandleDrawTrainerPic() {
    let mut xPos: i16 = 0;
    let mut yPos: i16 = 0;
    let mut trainerPicId: u32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
        if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
            trainerPicId = GetActiveBattlerLinkPlayerGender() as u32;
        } else {
            trainerPicId = gLinkPlayers[gRecordedBattleMultiplayerId].gender as u32;
        }
    } else {
        trainerPicId = gLinkPlayers[0].gender as u32;
    }
    if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if GetBattlerPosition(gActiveBattler) as i32 & BIT_FLANK as i32 != 0 {
            xPos = 90;
        } else {
            xPos = 32;
        }
        if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 {
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
    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 {
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
            .set_paletteNum(IndexOfSpritePaletteTag(
                (*(&raw const crate::data::data_tables::gTrainerFrontPicPaletteTable)
                    .cast::<CArray<CompressedSpritePalette, 0>>())[trainerPicId]
                    .tag,
            ) as u16);
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
pub(crate) unsafe fn RecordedPlayerHandleTrainerSlide() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleTrainerSlideBack() {
    SetSpritePrimaryCoordsFromSecondaryCoords(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = 35;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[2] = -40;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[4] =
        gSprites[gBattlerSpriteIds[gActiveBattler]].y;
    gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(
        &raw mut gSprites[gBattlerSpriteIds[gActiveBattler]],
        Some(SpriteCallbackDummy),
    );
    gBattlerControllerFuncs[gActiveBattler] = Some(FreeTrainerSpriteAfterSlide);
}
pub(crate) unsafe fn RecordedPlayerHandleFaintAnimation() {
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
            PlaySE12WithPanning(SE_FAINT, -64);
            gSprites[gBattlerSpriteIds[gActiveBattler]].data[1] = 0;
            gSprites[gBattlerSpriteIds[gActiveBattler]].data[sSpeedY] = 5;
            gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(SpriteCB_FaintSlideAnim);
            gBattlerControllerFuncs[gActiveBattler] = Some(FreeMonSpriteAfterFaintAnim);
        }
    }
}
pub(crate) unsafe fn RecordedPlayerHandlePaletteFade() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleSuccessBallThrowAnim() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleBallThrowAnim() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandlePause() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleMoveAnimation() {
    if IsBattleSEPlaying(gActiveBattler) == 0 {
        let r#move: u16 = gBattleBufferA[gActiveBattler][1] as u16
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
        gAnimDisableStructPtr = &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler][16] as *mut DisableStruct;
        gTransformedPersonalities[gActiveBattler] =
            (*gAnimDisableStructPtr).transformedMonPersonality;
        if IsMoveWithoutAnimation(r#move, gAnimMoveTurn) != 0 {
            RecordedPlayerBufferExecCompleted();
        } else {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
            gBattlerControllerFuncs[gActiveBattler] = Some(RecordedPlayerDoMoveAnimation);
        }
    }
}
pub(crate) unsafe fn RecordedPlayerDoMoveAnimation() {
    let r#move: u16 =
        gBattleBufferA[gActiveBattler][1] as u16 | (gBattleBufferA[gActiveBattler][2] as u16) << 8;
    let multihit: u8 = gBattleBufferA[gActiveBattler][11];
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
        3 if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive()
            == 0 =>
        {
            CopyAllBattleSpritesInvisibilities();
            TrySetBehindSubstituteSpriteBit(
                gActiveBattler,
                gBattleBufferA[gActiveBattler][1] as u16
                    | (gBattleBufferA[gActiveBattler][2] as u16) << 8,
            );
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
            RecordedPlayerBufferExecCompleted();
        }
        _ => {}
    }
}
pub(crate) unsafe fn RecordedPlayerHandlePrintString() {
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    let stringId: *mut u16 = &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
        .cast::<CArray<CArray<u8, 512>, 4>>()
        .cast_mut())[gActiveBattler][2] as *mut u16;
    BufferStringBattle(*stringId);
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnInactiveTextPrinter);
}
pub(crate) unsafe fn RecordedPlayerHandlePrintSelectionString() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn ChooseActionInBattlePalace() {
    if gBattleCommunication[4] as i32 >= gBattlersCount as i32 / 2 {
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            RecordedBattle_GetBattlerAction(gActiveBattler),
            0,
        );
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn RecordedPlayerHandleChooseAction() {
    if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0 {
        gBattlerControllerFuncs[gActiveBattler] = Some(ChooseActionInBattlePalace);
    } else {
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            RecordedBattle_GetBattlerAction(gActiveBattler),
            0,
        );
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn RecordedPlayerHandleYesNoBox() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleChooseMove() {
    if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0 {
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            B_ACTION_EXEC_SCRIPT,
            ChooseMoveAndTargetInBattlePalace(),
        );
    } else {
        let moveIndex: u8 = RecordedBattle_GetBattlerAction(gActiveBattler);
        let target: u8 = RecordedBattle_GetBattlerAction(gActiveBattler);
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            B_ACTION_EXEC_SCRIPT,
            moveIndex as u16 | (target as u16) << 8,
        );
    }
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleChooseItem() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleChoosePokemon() {
    *(*gBattleStruct)
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler) = RecordedBattle_GetBattlerAction(gActiveBattler);
    BtlController_EmitChosenMonReturnValue(
        B_COMM_TO_ENGINE,
        *(*gBattleStruct)
            .monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler),
        null_mut(),
    );
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleCmd23() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleHealthBarUpdate() {
    LoadBattleBarGfx(0);
    let hpVal: i16 =
        gBattleBufferA[gActiveBattler][2] as i16 | (gBattleBufferA[gActiveBattler][3] as i16) << 8;
    if hpVal != INSTANT_HP_BAR_DROP {
        let maxHP: u32 = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_MAX_HP,
        );
        let curHP: u32 = GetMonData2(
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
        let maxHP: u32 = GetMonData2(
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
pub(crate) unsafe fn RecordedPlayerHandleExpUpdate() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleStatusIconUpdate() {
    if IsBattleSEPlaying(gActiveBattler) == 0 {
        UpdateHealthboxAttribute(
            gHealthboxSpriteIds[gActiveBattler],
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            HEALTHBOX_STATUS_ICON,
        );
        let battler: u8 = gActiveBattler;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_statusAnimActive(0);
        gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedStatusAnimation);
    }
}
pub(crate) unsafe fn RecordedPlayerHandleStatusAnimation() {
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
pub(crate) unsafe fn RecordedPlayerHandleStatusXor() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleDataTransfer() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleDMA3Transfer() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandlePlayBGM() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleCmd32() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleTwoReturnValues() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleChosenMonReturnValue() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleOneReturnValue() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleOneReturnValue_Duplicate() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleClearUnkVar() {
    gUnusedControllerStruct.set_unk(0);
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleSetUnkVar() {
    gUnusedControllerStruct.set_unk(gBattleBufferA[gActiveBattler][1]);
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleClearUnkFlag() {
    gUnusedControllerStruct.set_flag(0);
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleToggleUnkFlag() {
    gUnusedControllerStruct.set_flag(gUnusedControllerStruct.flag() ^ 1);
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleHitAnimation() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].invisible() == TRUE as u16 {
        RecordedPlayerBufferExecCompleted();
    } else {
        gDoingBattleAnim = TRUE;
        gSprites[gBattlerSpriteIds[gActiveBattler]].data[1] = 0;
        DoHitAnimHealthboxEffect(gActiveBattler);
        gBattlerControllerFuncs[gActiveBattler] = Some(DoHitAnimBlinkSpriteEffect);
    }
}
pub(crate) unsafe fn RecordedPlayerHandleCantSwitch() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandlePlaySE() {
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
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandlePlayFanfareOrBGM() {
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
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleFaintingCry() {
    let species: u16 = GetMonData2(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        MON_DATA_SPECIES,
    ) as u16;
    PlayCry_ByMode(species, -25, CRY_MODE_FAINT);
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleIntroSlide() {
    HandleIntroSlide(gBattleBufferA[gActiveBattler][1]);
    gIntroSlideFlags |= 1;
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleIntroTrainerBallThrow() {
    let mut trainerPicId: u32 = 0;
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
    let paletteNum: u8 = AllocSpritePalette(0xD6F9);
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
        trainerPicId = gLinkPlayers[GetBattlerMultiplayerId(gActiveBattler as u16)].gender as u32;
    } else {
        trainerPicId = (*gSaveBlock2Ptr).playerGender as u32;
    }
    LoadCompressedPalette(
        (*(&raw const crate::data::data_tables::gTrainerBackPicPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[trainerPicId]
            .data,
        0x100 + paletteNum as u16 * 16,
        32,
    );
    gSprites[gBattlerSpriteIds[gActiveBattler]]
        .oam
        .set_paletteNum(paletteNum as u16);
    let taskId: u8 = CreateTask(Some(Task_StartSendOutAnim), 5);
    task_set(taskId, 0, gActiveBattler as i16);
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusSummaryShown() != 0
    {
        task_set_func(
            gBattlerStatusSummaryTaskId[gActiveBattler],
            Some(Task_HidePartyStatusSummary),
        );
    }
    (*(*gBattleSpritesDataPtr).animationData).set_introAnimActive(TRUE);
    gBattlerControllerFuncs[gActiveBattler] = Some(RecordedPlayerDummy);
}
pub(crate) unsafe fn Task_StartSendOutAnim(taskId: u8) {
    if task_get(taskId, 1) < 24 {
        task_set(taskId, 1, task_get(taskId, 1) + 1);
    } else {
        let savedActiveBank: u8 = gActiveBattler;
        gActiveBattler = task_get(taskId, 0) as u8;
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
        gActiveBattler = savedActiveBank;
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn RecordedPlayerHandleDrawPartyStatusSummary() {
    if gBattleBufferA[gActiveBattler][1] != 0 && GetBattlerSide(gActiveBattler) == 0 {
        RecordedPlayerBufferExecCompleted();
    } else {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_partyStatusSummaryShown(1);
        gBattlerStatusSummaryTaskId[gActiveBattler] = CreatePartyStatusSummarySprites(
            gActiveBattler,
            &raw mut (*(&raw const crate::battle_main::gBattleBufferA)
                .cast::<CArray<CArray<u8, 512>, 4>>()
                .cast_mut())[gActiveBattler][4] as *mut HpAndStatus,
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
pub(crate) unsafe fn EndDrawPartyStatusSummary() {
    if ({
        let t1 =
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer += 1;
        t1
    }) > 92
    {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusDelayTimer = 0;
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn RecordedPlayerHandleHidePartyStatusSummary() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusSummaryShown() != 0
    {
        task_set_func(
            gBattlerStatusSummaryTaskId[gActiveBattler],
            Some(Task_HidePartyStatusSummary),
        );
    }
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleEndBounceEffect() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleSpriteInvisibility() {
    if IsBattlerSpritePresent(gActiveBattler) != 0 {
        gSprites[gBattlerSpriteIds[gActiveBattler]]
            .set_invisible(gBattleBufferA[gActiveBattler][1] as u16);
        CopyBattleSpriteInvisibility(gActiveBattler);
    }
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleBattleAnimation() {
    if IsBattleSEPlaying(gActiveBattler) == 0 {
        let animationId: u8 = gBattleBufferA[gActiveBattler][1];
        let argument: u16 = gBattleBufferA[gActiveBattler][2] as u16
            | (gBattleBufferA[gActiveBattler][3] as u16) << 8;
        if TryHandleLaunchBattleTableAnimation(
            gActiveBattler,
            gActiveBattler,
            gActiveBattler,
            animationId,
            argument,
        ) != 0
        {
            RecordedPlayerBufferExecCompleted();
        } else {
            gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedBattleAnimation);
        }
    }
}
pub(crate) unsafe fn RecordedPlayerHandleLinkStandbyMsg() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleResetActionMoveSelection() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleEndLinkBattle() {
    gBattleOutcome = gBattleBufferA[gActiveBattler][1];
    FadeOutMapMusic(5);
    BeginFastPaletteFade(3);
    RecordedPlayerBufferExecCompleted();
    gBattlerControllerFuncs[gActiveBattler] = Some(SetBattleEndCallbacks);
}
pub(crate) fn RecordedPlayerCmdEnd() {}
