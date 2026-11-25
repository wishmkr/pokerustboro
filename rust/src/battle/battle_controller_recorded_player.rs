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
const sSpeedY: usize = 2;
// Data tables (translate with cdata.py): sRecordedPlayerBufferCommands

static sRecordedPlayerBufferCommands: Table<CArray<Option<unsafe fn()>, 57>> = Table(
    (&raw const crate::data::battle_controller_recorded_player::sRecordedPlayerBufferCommands)
        .cast(),
);

pub(crate) fn RecordedPlayerDummy() {}
pub fn SetControllerToRecordedPlayer() {
    {
        let gl_i0 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i0] = Some(RecordedPlayerBufferRunCommand))
    };
}
pub(crate) unsafe fn RecordedPlayerBufferRunCommand() {
    if gBattleControllerExecFlags.get()
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
        != 0
    {
        if ({
            let gl_i1 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i1][0])
        }) < 57
        {
            sRecordedPlayerBufferCommands[{
                let gl_i2 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i2][0])
            }]
            .unwrap_unchecked()();
        } else {
            RecordedPlayerBufferExecCompleted();
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
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn FreeTrainerSpriteAfterSlide() {
    if (*gSprites.as_ptr())[{
        let gl_i4 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i4])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        BattleGfxSfxDummy3(MALE);
        FreeSpriteOamMatrix(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i5 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i5])
            }],
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i6 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i6])
            }],
        );
        RecordedPlayerBufferExecCompleted();
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
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn Intro_WaitForShinyAnimAndHealthbox() {
    let mut healthboxAnimDone: u32 = FALSE as u32;
    if GetBattlerPosition(gActiveBattler.get()) == B_POSITION_PLAYER_LEFT {
        if IsDoubleBattle() == 0
            || IsDoubleBattle() != 0 && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
        {
            if (*gSprites.as_ptr())[{
                let gl_i7 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i7])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                healthboxAnimDone = TRUE as u32;
            }
        } else {
            if (*gSprites.as_ptr())[{
                let gl_i8 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i8])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
                && (*gSprites.as_ptr())[{
                    let gl_i9 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gHealthboxSpriteIds[gl_i9])
                }]
                .callback
                    == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                healthboxAnimDone = TRUE as u32;
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
                    let gl_i10 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i10])
                }],
                gActiveBattler.get(),
            );
            if IsDoubleBattle() != 0 {
                HandleLowHpMusicChange(
                    &raw mut (*gPlayerParty.as_ptr())[{
                        let gl_i11 = gActiveBattler.get() as i32 ^ 2;
                        crate::gl!(gBattlerPartyIndexes[gl_i11])
                    }],
                    gActiveBattler.get() ^ 2,
                );
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
    } else {
        if IsDoubleBattle() == 0
            || IsDoubleBattle() != 0 && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
        {
            if (*gSprites.as_ptr())[{
                let gl_i13 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i13])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                healthboxAnimDone = TRUE as u32;
            }
        } else {
            if (*gSprites.as_ptr())[{
                let gl_i14 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i14])
            }]
            .callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
                && (*gSprites.as_ptr())[{
                    let gl_i15 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gHealthboxSpriteIds[gl_i15])
                }]
                .callback
                    == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                healthboxAnimDone = TRUE as u32;
            }
        }
        if IsCryPlayingOrClearCrySongs() != 0 {
            healthboxAnimDone = FALSE as u32;
        }
        if healthboxAnimDone != 0 {
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .introEndDelay = 3;
            {
                let gl_i16 = gActiveBattler.get();
                crate::gl!(gBattlerControllerFuncs[gl_i16] = Some(Intro_DelayAndEnd))
            };
        }
    }
}
pub(crate) unsafe fn Intro_TryShinyAnimShowHealthbox() {
    let mut bgmRestored: u32 = FALSE as u32;
    if GetBattlerPosition(gActiveBattler.get()) == B_POSITION_PLAYER_LEFT {
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
                    let gl_i17 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i17])
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
                    let gl_i18 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gBattlerPartyIndexes[gl_i18])
                }],
            );
        }
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
                        let gl_i19 = gActiveBattler.get() as i32 ^ 2;
                        crate::gl!(gHealthboxSpriteIds[gl_i19])
                    },
                    &raw mut (*gPlayerParty.as_ptr())[{
                        let gl_i20 = gActiveBattler.get() as i32 ^ 2;
                        crate::gl!(gBattlerPartyIndexes[gl_i20])
                    }],
                    HEALTHBOX_ALL,
                );
                StartHealthboxSlideIn(gActiveBattler.get() ^ 2);
                SetHealthboxSpriteVisible({
                    let gl_i21 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gHealthboxSpriteIds[gl_i21])
                });
            }
            UpdateHealthboxAttribute(
                {
                    let gl_i22 = gActiveBattler.get();
                    crate::gl!(gHealthboxSpriteIds[gl_i22])
                },
                &raw mut (*gPlayerParty.as_ptr())[{
                    let gl_i23 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i23])
                }],
                HEALTHBOX_ALL,
            );
            StartHealthboxSlideIn(gActiveBattler.get());
            SetHealthboxSpriteVisible({
                let gl_i24 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i24])
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
    .healthboxSlideInStarted()
        != 0
        && (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .waitForCry()
            == 0
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
            if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0
                && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
            {
                if GetBattlerPosition(gActiveBattler.get()) == B_POSITION_PLAYER_LEFT {
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
    if bgmRestored != 0
        && (*gSprites.as_ptr())[{
            let gl_i25 = gActiveBattler.get();
            crate::gl!(gBattleControllerData[gl_i25])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && (*gSprites.as_ptr())[{
            let gl_i26 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i26])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        if IsDoubleBattle() != 0 && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI == 0 {
            DestroySprite(
                &raw mut (*gSprites.as_ptr())[{
                    let gl_i27 = gActiveBattler.get() as i32 ^ 2;
                    crate::gl!(gBattleControllerData[gl_i27])
                }],
            );
        }
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i28 = gActiveBattler.get();
                crate::gl!(gBattleControllerData[gl_i28])
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
            let gl_i29 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i29] = Some(Intro_WaitForShinyAnimAndHealthbox))
        };
    }
}
pub(crate) unsafe fn WaitForMonAnimAfterLoad() {
    if (*gSprites.as_ptr())[{
        let gl_i30 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i30])
    }]
    .animEnded()
        != 0
        && (*gSprites.as_ptr())[{
            let gl_i31 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i31])
        }]
        .x2 == 0
    {
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnHealthbarDone() {
    let hpValue: i16 = MoveBattleBar(
        gActiveBattler.get(),
        {
            let gl_i32 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i32])
        },
        HEALTH_BAR,
        0,
    ) as i16;
    SetHealthboxSpriteVisible({
        let gl_i33 = gActiveBattler.get();
        crate::gl!(gHealthboxSpriteIds[gl_i33])
    });
    if hpValue != -1 {
        UpdateHpTextInHealthbox(
            {
                let gl_i34 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i34])
            },
            hpValue,
            HP_CURRENT,
        );
    } else {
        HandleLowHpMusicChange(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i35 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i35])
            }],
            gActiveBattler.get(),
        );
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn FreeMonSpriteAfterFaintAnim() {
    if (*gSprites.as_ptr())[{
        let gl_i36 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i36])
    }]
    .y as i32
        + (*gSprites.as_ptr())[{
            let gl_i37 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i37])
        }]
        .y2 as i32
        > DISPLAY_HEIGHT as i32
    {
        let species: u16 = GetMonData2(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i38 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i38])
            }],
            MON_DATA_SPECIES,
        ) as u16;
        BattleGfxSfxDummy2(species);
        FreeOamMatrix(
            (*gSprites.as_ptr())[{
                let gl_i39 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i39])
            }]
            .oam
            .matrixNum() as u8,
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i40 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i40])
            }],
        );
        SetHealthboxSpriteInvisible({
            let gl_i41 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i41])
        });
        RecordedPlayerBufferExecCompleted();
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
                let gl_i42 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i42])
            }],
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i43 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i43])
            }],
        );
        SetHealthboxSpriteInvisible({
            let gl_i44 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i44])
        });
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnInactiveTextPrinter() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn DoHitAnimBlinkSpriteEffect() {
    let spriteId: u8 = {
        let gl_i45 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i45])
    };
    if crate::gl!(gSprites[spriteId].data[1]) == 32 {
        crate::gl!(gSprites[spriteId].data[1] = 0);
        (*gSprites.as_ptr())[spriteId].set_invisible(FALSE as u16);
        gDoingBattleAnim.set(FALSE);
        RecordedPlayerBufferExecCompleted();
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
        let gl_i46 = gActiveBattler.get();
        crate::gl!(gHealthboxSpriteIds[gl_i46])
    }]
    .callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        CopyBattleSpriteInvisibility(gActiveBattler.get());
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
            let gl_i47 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i47] = Some(SwitchIn_WaitAndEnd))
        };
    }
}
pub(crate) unsafe fn SwitchIn_WaitAndEnd() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .specialAnimActive()
        == 0
        && (*gSprites.as_ptr())[{
            let gl_i48 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i48])
        }]
        .callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn SwitchIn_ShowHealthbox() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
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
        CreateTask(Some(Task_PlayerController_RestoreBgmAfterCry), 10);
        HandleLowHpMusicChange(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i49 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i49])
            }],
            gActiveBattler.get(),
        );
        StartSpriteAnim(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i50 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i50])
            }],
            0,
        );
        UpdateHealthboxAttribute(
            {
                let gl_i51 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i51])
            },
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i52 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i52])
            }],
            HEALTHBOX_ALL,
        );
        StartHealthboxSlideIn(gActiveBattler.get());
        SetHealthboxSpriteVisible({
            let gl_i53 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i53])
        });
        {
            let gl_i54 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i54] = Some(SwitchIn_ShowSubstitute))
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
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i55 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i55])
            }],
        );
    }
    if (*gSprites.as_ptr())[{
        let gl_i56 = gActiveBattler.get();
        crate::gl!(gBattleControllerData[gl_i56])
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
                let gl_i57 = gActiveBattler.get();
                crate::gl!(gBattleControllerData[gl_i57])
            }],
        );
        {
            let gl_i58 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i58] = Some(SwitchIn_ShowHealthbox))
        };
    }
}
unsafe fn RecordedPlayerBufferExecCompleted() {
    {
        let gl_i59 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i59] = Some(RecordedPlayerBufferRunCommand))
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
pub(crate) unsafe fn CompleteOnFinishedStatusAnimation() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .statusAnimActive()
        == 0
    {
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn CompleteOnFinishedBattleAnimation() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .animFromTableActive()
        == 0
    {
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn RecordedPlayerHandleGetMonData() {
    let mut monData: CArray<u8, 256> = zeroed();
    let mut size: u32 = 0;
    let mut monToCheck: u8 = 0;
    if ({
        let gl_i61 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i61][2])
    }) == 0
    {
        size += CopyRecordedPlayerMonData(
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
    match {
        let gl_i64 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i64][1])
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
                        let gl_i66 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i66][1])
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
pub(crate) unsafe fn RecordedPlayerHandleGetRawMonData() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleSetMonData() {
    let mut monToCheck: u8 = 0;
    if ({
        let gl_i67 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i67][2])
    }) == 0
    {
        SetRecordedPlayerMonData(
            ({
                let gl_i68 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i68])
            }) as u8,
        );
    } else {
        monToCheck = {
            let gl_i69 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i69][2])
        };
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
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][3] as *mut BattlePokemon;
    let moveData: *mut MovePPInfo =
        &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;).cast_const())
            .cast::<CArray<CArray<u8, 512>, 4>>()
            .cast_mut())[gActiveBattler.get()][3] as *mut MovePPInfo;
    match {
        let gl_i70 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i70][1])
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
                        let gl_i71 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i71][1])
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
                        let gl_i72 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i72][1])
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
            let gl_i73 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i73])
        }],
        gActiveBattler.get(),
    );
}
pub(crate) unsafe fn RecordedPlayerHandleSetRawMonData() {
    let dst: *mut u8 = (&raw mut (*gPlayerParty.as_ptr())[{
        let gl_i74 = gActiveBattler.get();
        crate::gl!(gBattlerPartyIndexes[gl_i74])
    }] as *mut u8)
        .at({
            let gl_i75 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i75][1])
        });
    for i in 0..({
        let gl_i76 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i76][2])
    }) {
        *dst.at(i) = {
            let gl_i77 = gActiveBattler.get();
            let gl_i78 = 3 + i as i32;
            crate::gl!(gBattleBufferA[gl_i77][gl_i78])
        };
    }
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleLoadMonSprite() {
    BattleLoadPlayerMonSpriteGfx(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i79 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i79])
        }],
        gActiveBattler.get(),
    );
    let species: u16 = GetMonData2(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i80 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i80])
        }],
        MON_DATA_SPECIES,
    ) as u16;
    SetMultiuseSpriteTemplateToPokemon(species, GetBattlerPosition(gActiveBattler.get()));
    {
        let gl_v = CreateSprite(
            crate::gl!(&raw gMultiuseSpriteTemplate),
            GetBattlerSpriteCoord(gActiveBattler.get(), BATTLER_COORD_X_2) as i16,
            GetBattlerSpriteDefault_Y(gActiveBattler.get()) as i16,
            GetBattlerSpriteSubpriority(gActiveBattler.get()),
        );
        let gl_i81 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i81] = gl_v)
    };
    (*gSprites.as_ptr())[{
        let gl_i82 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i82])
    }]
    .x2 = -240;
    (*gSprites.as_ptr())[{
        let gl_i83 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i83])
    }]
    .data[0] = gActiveBattler.get() as i16;
    (*gSprites.as_ptr())[{
        let gl_i84 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i84])
    }]
    .oam
    .set_paletteNum(gActiveBattler.get() as u16);
    StartSpriteAnim(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i85 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i85])
        }],
        {
            let gl_i86 = gActiveBattler.get();
            crate::gl!(gBattleMonForms[gl_i86])
        },
    );
    {
        let gl_i87 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i87] = Some(WaitForMonAnimAfterLoad))
    };
}
pub(crate) unsafe fn RecordedPlayerHandleSwitchInAnim() {
    ClearTemporarySpeciesSpriteData(gActiveBattler.get(), {
        let gl_i88 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i88][2])
    });
    {
        let gl_i89 = gActiveBattler.get();
        crate::gl!(
            gBattlerPartyIndexes[gl_i89] = ({
                let gl_i90 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i90][1])
            }) as u16
        )
    };
    BattleLoadPlayerMonSpriteGfx(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i91 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i91])
        }],
        gActiveBattler.get(),
    );
    StartSendOutAnim(gActiveBattler.get(), {
        let gl_i92 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i92][2])
    });
    {
        let gl_i93 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i93] = Some(SwitchIn_TryShinyAnim))
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
pub(crate) unsafe fn RecordedPlayerHandleReturnMonToBall() {
    if ({
        let gl_i94 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i94][1])
    }) == 0
    {
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .animationState = 0;
        {
            let gl_i95 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i95] = Some(DoSwitchOutAnimation))
        };
    } else {
        FreeSpriteOamMatrix(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i96 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i96])
            }],
        );
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[{
                let gl_i97 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i97])
            }],
        );
        SetHealthboxSpriteInvisible({
            let gl_i98 = gActiveBattler.get();
            crate::gl!(gHealthboxSpriteIds[gl_i98])
        });
        RecordedPlayerBufferExecCompleted();
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
                let gl_i99 = gActiveBattler.get();
                crate::gl!(gBattlerControllerFuncs[gl_i99] = Some(FreeMonSpriteAfterSwitchOutAnim))
            };
        }
        _ => {}
    }
}
pub(crate) unsafe fn RecordedPlayerHandleDrawTrainerPic() {
    let mut xPos: i16 = 0;
    let mut yPos: i16 = 0;
    let mut trainerPicId: u32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
            trainerPicId = GetActiveBattlerLinkPlayerGender() as u32;
        } else {
            trainerPicId = ({
                let gl_i100 = gRecordedBattleMultiplayerId.get();
                crate::gl!(gLinkPlayers[gl_i100].gender)
            }) as u32;
        }
    } else {
        trainerPicId = crate::gl!(gLinkPlayers[0].gender) as u32;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
        if GetBattlerPosition(gActiveBattler.get()) as i32 & BIT_FLANK as i32 != 0 {
            xPos = 90;
        } else {
            xPos = 32;
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0 {
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
    if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0 {
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
            let gl_i101 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i101] = gl_v)
        };
        (*gSprites.as_ptr())[{
            let gl_i102 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i102])
        }]
        .oam
        .set_paletteNum(IndexOfSpritePaletteTag(
            (*(&raw const crate::data::data_tables::gTrainerFrontPicPaletteTable)
                .cast::<CArray<CompressedSpritePalette, 0>>())[trainerPicId]
                .tag,
        ) as u16);
        (*gSprites.as_ptr())[{
            let gl_i103 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i103])
        }]
        .x2 = DISPLAY_WIDTH as i16;
        (*gSprites.as_ptr())[{
            let gl_i104 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i104])
        }]
        .y2 = 48;
        (*gSprites.as_ptr())[{
            let gl_i105 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i105])
        }]
        .data[0] = -2;
        (*gSprites.as_ptr())[{
            let gl_i106 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i106])
        }]
        .callback = Some(SpriteCB_TrainerSlideIn);
        (*gSprites.as_ptr())[{
            let gl_i107 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i107])
        }]
        .oam
        .set_affineMode(ST_OAM_AFFINE_OFF);
        (*gSprites.as_ptr())[{
            let gl_i108 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i108])
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
            let gl_i109 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i109] = gl_v)
        };
        (*gSprites.as_ptr())[{
            let gl_i110 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i110])
        }]
        .oam
        .set_paletteNum(gActiveBattler.get() as u16);
        (*gSprites.as_ptr())[{
            let gl_i111 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i111])
        }]
        .x2 = DISPLAY_WIDTH as i16;
        (*gSprites.as_ptr())[{
            let gl_i112 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i112])
        }]
        .data[0] = -2;
        (*gSprites.as_ptr())[{
            let gl_i113 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i113])
        }]
        .callback = Some(SpriteCB_TrainerSlideIn);
    }
    {
        let gl_i114 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i114] = Some(CompleteOnBattlerSpriteCallbackDummy))
    };
}
pub(crate) unsafe fn RecordedPlayerHandleTrainerSlide() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleTrainerSlideBack() {
    SetSpritePrimaryCoordsFromSecondaryCoords(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i115 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i115])
        }],
    );
    (*gSprites.as_ptr())[{
        let gl_i116 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i116])
    }]
    .data[0] = 35;
    (*gSprites.as_ptr())[{
        let gl_i117 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i117])
    }]
    .data[2] = -40;
    (*gSprites.as_ptr())[{
        let gl_i118 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i118])
    }]
    .data[4] = (*gSprites.as_ptr())[{
        let gl_i119 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i119])
    }]
    .y;
    (*gSprites.as_ptr())[{
        let gl_i120 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i120])
    }]
    .callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i121 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i121])
        }],
        Some(SpriteCallbackDummy),
    );
    {
        let gl_i122 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i122] = Some(FreeTrainerSpriteAfterSlide))
    };
}
pub(crate) unsafe fn RecordedPlayerHandleFaintAnimation() {
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
                    let gl_i123 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i123])
                }],
                gActiveBattler.get(),
            );
            PlaySE12WithPanning(SE_FAINT, -64);
            (*gSprites.as_ptr())[{
                let gl_i124 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i124])
            }]
            .data[1] = 0;
            (*gSprites.as_ptr())[{
                let gl_i125 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i125])
            }]
            .data[sSpeedY] = 5;
            (*gSprites.as_ptr())[{
                let gl_i126 = gActiveBattler.get();
                crate::gl!(gBattlerSpriteIds[gl_i126])
            }]
            .callback = Some(SpriteCB_FaintSlideAnim);
            {
                let gl_i127 = gActiveBattler.get();
                crate::gl!(gBattlerControllerFuncs[gl_i127] = Some(FreeMonSpriteAfterFaintAnim))
            };
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
    if IsBattleSEPlaying(gActiveBattler.get()) == 0 {
        let r#move: u16 = ({
            let gl_i128 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i128][1])
        }) as u16
            | (({
                let gl_i129 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i129][2])
            }) as u16)
                << 8;
        gAnimMoveTurn.set({
            let gl_i130 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i130][3])
        });
        gAnimMovePower.set(
            ({
                let gl_i131 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i131][4])
            }) as u16
                | (({
                    let gl_i132 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i132][5])
                }) as u16)
                    << 8,
        );
        gAnimMoveDmg.set(
            ({
                let gl_i133 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i133][6])
            }) as i32
                | (({
                    let gl_i134 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i134][7])
                }) as i32)
                    << 8
                | (({
                    let gl_i135 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i135][8])
                }) as i32)
                    << 16
                | (({
                    let gl_i136 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i136][9])
                }) as i32)
                    << 24,
        );
        gAnimFriendship.set({
            let gl_i137 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i137][10])
        });
        gWeatherMoveAnim.set(
            ({
                let gl_i138 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i138][12])
            }) as u16
                | (({
                    let gl_i139 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i139][13])
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
            let gl_i140 = gActiveBattler.get();
            crate::gl!(
                gTransformedPersonalities[gl_i140] =
                    (*crate::gl!(gAnimDisableStructPtr)).transformedMonPersonality
            )
        };
        if IsMoveWithoutAnimation(r#move, gAnimMoveTurn.get()) != 0 {
            RecordedPlayerBufferExecCompleted();
        } else {
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .animationState = 0;
            {
                let gl_i141 = gActiveBattler.get();
                crate::gl!(gBattlerControllerFuncs[gl_i141] = Some(RecordedPlayerDoMoveAnimation))
            };
        }
    }
}
pub(crate) unsafe fn RecordedPlayerDoMoveAnimation() {
    let r#move: u16 = ({
        let gl_i142 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i142][1])
    }) as u16
        | (({
            let gl_i143 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i143][2])
        }) as u16)
            << 8;
    let multihit: u8 = {
        let gl_i144 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i144][11])
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
                    let gl_i145 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i145][1])
                }) as u16
                    | (({
                        let gl_i146 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i146][2])
                    }) as u16)
                        << 8,
            );
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .animationState = 0;
            RecordedPlayerBufferExecCompleted();
        }
        _ => {}
    }
}
pub(crate) unsafe fn RecordedPlayerHandlePrintString() {
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    let stringId: *mut u16 = &raw mut (*(crate::gl!(&raw @at crate::battle_main::gBattleBufferA;)
        .cast_const())
    .cast::<CArray<CArray<u8, 512>, 4>>()
    .cast_mut())[gActiveBattler.get()][2] as *mut u16;
    BufferStringBattle(*stringId);
    BattlePutTextOnWindow((*gDisplayedStringBattle.as_ptr()).as_mut_ptr(), B_WIN_MSG);
    {
        let gl_i147 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i147] = Some(CompleteOnInactiveTextPrinter))
    };
}
pub(crate) unsafe fn RecordedPlayerHandlePrintSelectionString() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn ChooseActionInBattlePalace() {
    if crate::gl!(gBattleCommunication[4]) as i32 >= gBattlersCount.get() as i32 / 2 {
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            RecordedBattle_GetBattlerAction(gActiveBattler.get()),
            0,
        );
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn RecordedPlayerHandleChooseAction() {
    if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 {
        {
            let gl_i148 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i148] = Some(ChooseActionInBattlePalace))
        };
    } else {
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            RecordedBattle_GetBattlerAction(gActiveBattler.get()),
            0,
        );
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn RecordedPlayerHandleYesNoBox() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleChooseMove() {
    if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 {
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            B_ACTION_EXEC_SCRIPT,
            ChooseMoveAndTargetInBattlePalace(),
        );
    } else {
        let moveIndex: u8 = RecordedBattle_GetBattlerAction(gActiveBattler.get());
        let target: u8 = RecordedBattle_GetBattlerAction(gActiveBattler.get());
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
    *(*crate::gl!(gBattleStruct))
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler.get()) = RecordedBattle_GetBattlerAction(gActiveBattler.get());
    BtlController_EmitChosenMonReturnValue(
        B_COMM_TO_ENGINE,
        *(*crate::gl!(gBattleStruct))
            .monToSwitchIntoId
            .as_mut_ptr()
            .at(gActiveBattler.get()),
        null_mut(),
    );
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleCmd23() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleHealthBarUpdate() {
    LoadBattleBarGfx(0);
    let hpVal: i16 = ({
        let gl_i149 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i149][2])
    }) as i16
        | (({
            let gl_i150 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i150][3])
        }) as i16)
            << 8;
    if hpVal != INSTANT_HP_BAR_DROP {
        let maxHP: u32 = GetMonData2(
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i151 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i151])
            }],
            MON_DATA_MAX_HP,
        );
        let curHP: u32 = GetMonData2(
            &raw mut (*gPlayerParty.as_ptr())[{
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
            &raw mut (*gPlayerParty.as_ptr())[{
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
        UpdateHpTextInHealthbox(
            {
                let gl_i156 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i156])
            },
            0,
            HP_CURRENT,
        );
    }
    {
        let gl_i157 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i157] = Some(CompleteOnHealthbarDone))
    };
}
pub(crate) unsafe fn RecordedPlayerHandleExpUpdate() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleStatusIconUpdate() {
    if IsBattleSEPlaying(gActiveBattler.get()) == 0 {
        UpdateHealthboxAttribute(
            {
                let gl_i158 = gActiveBattler.get();
                crate::gl!(gHealthboxSpriteIds[gl_i158])
            },
            &raw mut (*gPlayerParty.as_ptr())[{
                let gl_i159 = gActiveBattler.get();
                crate::gl!(gBattlerPartyIndexes[gl_i159])
            }],
            HEALTHBOX_STATUS_ICON,
        );
        let battler: u8 = gActiveBattler.get();
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .set_statusAnimActive(0);
        {
            let gl_i160 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i160] = Some(CompleteOnFinishedStatusAnimation))
        };
    }
}
pub(crate) unsafe fn RecordedPlayerHandleStatusAnimation() {
    if IsBattleSEPlaying(gActiveBattler.get()) == 0 {
        InitAndLaunchChosenStatusAnimation(
            {
                let gl_i161 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i161][1])
            },
            ({
                let gl_i162 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i162][2])
            }) as u32
                | (({
                    let gl_i163 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i163][3])
                }) as u32)
                    << 8
                | (({
                    let gl_i164 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i164][4])
                }) as u32)
                    << 16
                | (({
                    let gl_i165 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i165][5])
                }) as u32)
                    << 24,
        );
        {
            let gl_i166 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i166] = Some(CompleteOnFinishedStatusAnimation))
        };
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
    (*gUnusedControllerStruct.as_ptr()).set_unk(0);
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleSetUnkVar() {
    (*gUnusedControllerStruct.as_ptr()).set_unk({
        let gl_i167 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i167][1])
    });
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleClearUnkFlag() {
    (*gUnusedControllerStruct.as_ptr()).set_flag(0);
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleToggleUnkFlag() {
    (*gUnusedControllerStruct.as_ptr()).set_flag((*gUnusedControllerStruct.as_ptr()).flag() ^ 1);
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleHitAnimation() {
    if (*gSprites.as_ptr())[{
        let gl_i168 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i168])
    }]
    .invisible()
        == TRUE as u16
    {
        RecordedPlayerBufferExecCompleted();
    } else {
        gDoingBattleAnim.set(TRUE);
        (*gSprites.as_ptr())[{
            let gl_i169 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i169])
        }]
        .data[1] = 0;
        DoHitAnimHealthboxEffect(gActiveBattler.get());
        {
            let gl_i170 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i170] = Some(DoHitAnimBlinkSpriteEffect))
        };
    }
}
pub(crate) unsafe fn RecordedPlayerHandleCantSwitch() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandlePlaySE() {
    let mut pan: i8 = 0;
    if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
        pan = SOUND_PAN_ATTACKER;
    } else {
        pan = SOUND_PAN_TARGET;
    }
    PlaySE12WithPanning(
        ({
            let gl_i171 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i171][1])
        }) as u16
            | (({
                let gl_i172 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i172][2])
            }) as u16)
                << 8,
        pan,
    );
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandlePlayFanfareOrBGM() {
    if ({
        let gl_i173 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i173][3])
    }) != 0
    {
        BattleStopLowHpSound();
        PlayBGM(
            ({
                let gl_i174 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i174][1])
            }) as u16
                | (({
                    let gl_i175 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i175][2])
                }) as u16)
                    << 8,
        );
    } else {
        PlayFanfare(
            ({
                let gl_i176 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i176][1])
            }) as u16
                | (({
                    let gl_i177 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i177][2])
                }) as u16)
                    << 8,
        );
    }
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleFaintingCry() {
    let species: u16 = GetMonData2(
        &raw mut (*gPlayerParty.as_ptr())[{
            let gl_i178 = gActiveBattler.get();
            crate::gl!(gBattlerPartyIndexes[gl_i178])
        }],
        MON_DATA_SPECIES,
    ) as u16;
    PlayCry_ByMode(species, -25, CRY_MODE_FAINT);
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleIntroSlide() {
    HandleIntroSlide({
        let gl_i179 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i179][1])
    });
    gIntroSlideFlags.set(gIntroSlideFlags.get() | (1));
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleIntroTrainerBallThrow() {
    let mut trainerPicId: u32 = 0;
    SetSpritePrimaryCoordsFromSecondaryCoords(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i180 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i180])
        }],
    );
    (*gSprites.as_ptr())[{
        let gl_i181 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i181])
    }]
    .data[0] = 50;
    (*gSprites.as_ptr())[{
        let gl_i182 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i182])
    }]
    .data[2] = -40;
    (*gSprites.as_ptr())[{
        let gl_i183 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i183])
    }]
    .data[4] = (*gSprites.as_ptr())[{
        let gl_i184 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i184])
    }]
    .y;
    (*gSprites.as_ptr())[{
        let gl_i185 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i185])
    }]
    .callback = Some(StartAnimLinearTranslation);
    (*gSprites.as_ptr())[{
        let gl_i186 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i186])
    }]
    .data[5] = gActiveBattler.get() as i16;
    StoreSpriteCallbackInData6(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i187 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i187])
        }],
        Some(SpriteCB_FreePlayerSpriteLoadMonSprite),
    );
    StartSpriteAnim(
        &raw mut (*gSprites.as_ptr())[{
            let gl_i188 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i188])
        }],
        1,
    );
    let paletteNum: u8 = AllocSpritePalette(0xD6F9);
    if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0 {
        trainerPicId = ({
            let gl_i189 = GetBattlerMultiplayerId(gActiveBattler.get() as u16);
            crate::gl!(gLinkPlayers[gl_i189].gender)
        }) as u32;
    } else {
        trainerPicId = (*crate::gl!(gSaveBlock2Ptr)).playerGender as u32;
    }
    LoadCompressedPalette(
        (*(&raw const crate::data::data_tables::gTrainerBackPicPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[trainerPicId]
            .data,
        0x100 + paletteNum as u16 * 16,
        32,
    );
    (*gSprites.as_ptr())[{
        let gl_i190 = gActiveBattler.get();
        crate::gl!(gBattlerSpriteIds[gl_i190])
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
                let gl_i191 = gActiveBattler.get();
                crate::gl!(gBattlerStatusSummaryTaskId[gl_i191])
            },
            Some(Task_HidePartyStatusSummary),
        );
    }
    (*(*crate::gl!(gBattleSpritesDataPtr)).animationData).set_introAnimActive(TRUE);
    {
        let gl_i192 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i192] = Some(RecordedPlayerDummy))
    };
}
pub(crate) unsafe fn Task_StartSendOutAnim(taskId: u8) {
    if task_get(taskId, 1) < 24 {
        task_set(taskId, 1, task_get(taskId, 1) + 1);
    } else {
        let savedActiveBank: u8 = gActiveBattler.get();
        gActiveBattler.set(task_get(taskId, 0) as u8);
        if IsDoubleBattle() == 0 || gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
            {
                let gl_i193 = gActiveBattler.get();
                crate::gl!(
                    gBattleBufferA[gl_i193][1] = ({
                        let gl_i194 = gActiveBattler.get();
                        crate::gl!(gBattlerPartyIndexes[gl_i194])
                    }) as u8
                )
            };
            StartSendOutAnim(gActiveBattler.get(), FALSE);
        } else {
            {
                let gl_i195 = gActiveBattler.get();
                crate::gl!(
                    gBattleBufferA[gl_i195][1] = ({
                        let gl_i196 = gActiveBattler.get();
                        crate::gl!(gBattlerPartyIndexes[gl_i196])
                    }) as u8
                )
            };
            StartSendOutAnim(gActiveBattler.get(), FALSE);
            gActiveBattler.set(gActiveBattler.get() ^ (BIT_FLANK));
            {
                let gl_i197 = gActiveBattler.get();
                crate::gl!(
                    gBattleBufferA[gl_i197][1] = ({
                        let gl_i198 = gActiveBattler.get();
                        crate::gl!(gBattlerPartyIndexes[gl_i198])
                    }) as u8
                )
            };
            BattleLoadPlayerMonSpriteGfx(
                &raw mut (*gPlayerParty.as_ptr())[{
                    let gl_i199 = gActiveBattler.get();
                    crate::gl!(gBattlerPartyIndexes[gl_i199])
                }],
                gActiveBattler.get(),
            );
            StartSendOutAnim(gActiveBattler.get(), FALSE);
            gActiveBattler.set(gActiveBattler.get() ^ (BIT_FLANK));
        }
        {
            let gl_i200 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i200] = Some(Intro_TryShinyAnimShowHealthbox))
        };
        gActiveBattler.set(savedActiveBank);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn RecordedPlayerHandleDrawPartyStatusSummary() {
    if ({
        let gl_i201 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i201][1])
    }) != 0
        && GetBattlerSide(gActiveBattler.get()) == 0
    {
        RecordedPlayerBufferExecCompleted();
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
                    let gl_i203 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i203][1])
                },
                {
                    let gl_i204 = gActiveBattler.get();
                    crate::gl!(gBattleBufferA[gl_i204][2])
                },
            );
            let gl_i202 = gActiveBattler.get();
            crate::gl!(gBattlerStatusSummaryTaskId[gl_i202] = gl_v)
        };
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(gActiveBattler.get()))
        .partyStatusDelayTimer = 0;
        if ({
            let gl_i205 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i205][2])
        }) != 0
        {
            (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(gActiveBattler.get()))
            .partyStatusDelayTimer = 93;
        }
        {
            let gl_i206 = gActiveBattler.get();
            crate::gl!(gBattlerControllerFuncs[gl_i206] = Some(EndDrawPartyStatusSummary))
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
        RecordedPlayerBufferExecCompleted();
    }
}
pub(crate) unsafe fn RecordedPlayerHandleHidePartyStatusSummary() {
    if (*(*crate::gl!(gBattleSpritesDataPtr))
        .healthBoxesData
        .at(gActiveBattler.get()))
    .partyStatusSummaryShown()
        != 0
    {
        task_set_func(
            {
                let gl_i207 = gActiveBattler.get();
                crate::gl!(gBattlerStatusSummaryTaskId[gl_i207])
            },
            Some(Task_HidePartyStatusSummary),
        );
    }
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleEndBounceEffect() {
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleSpriteInvisibility() {
    if IsBattlerSpritePresent(gActiveBattler.get()) != 0 {
        (*gSprites.as_ptr())[{
            let gl_i208 = gActiveBattler.get();
            crate::gl!(gBattlerSpriteIds[gl_i208])
        }]
        .set_invisible(
            ({
                let gl_i209 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i209][1])
            }) as u16,
        );
        CopyBattleSpriteInvisibility(gActiveBattler.get());
    }
    RecordedPlayerBufferExecCompleted();
}
pub(crate) unsafe fn RecordedPlayerHandleBattleAnimation() {
    if IsBattleSEPlaying(gActiveBattler.get()) == 0 {
        let animationId: u8 = {
            let gl_i210 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i210][1])
        };
        let argument: u16 = ({
            let gl_i211 = gActiveBattler.get();
            crate::gl!(gBattleBufferA[gl_i211][2])
        }) as u16
            | (({
                let gl_i212 = gActiveBattler.get();
                crate::gl!(gBattleBufferA[gl_i212][3])
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
            RecordedPlayerBufferExecCompleted();
        } else {
            {
                let gl_i213 = gActiveBattler.get();
                crate::gl!(
                    gBattlerControllerFuncs[gl_i213] = Some(CompleteOnFinishedBattleAnimation)
                )
            };
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
    gBattleOutcome.set({
        let gl_i214 = gActiveBattler.get();
        crate::gl!(gBattleBufferA[gl_i214][1])
    });
    FadeOutMapMusic(5);
    BeginFastPaletteFade(3);
    RecordedPlayerBufferExecCompleted();
    {
        let gl_i215 = gActiveBattler.get();
        crate::gl!(gBattlerControllerFuncs[gl_i215] = Some(SetBattleEndCallbacks))
    };
}
pub(crate) fn RecordedPlayerCmdEnd() {}
