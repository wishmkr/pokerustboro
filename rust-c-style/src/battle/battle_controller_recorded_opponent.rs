//! Translated from `src/battle_controller_recorded_opponent.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sRecordedOpponentBufferCommands

static sRecordedOpponentBufferCommands: Table<CArray<Option<unsafe extern "C" fn()>, 57>> = Table(
    (&raw const crate::data::battle_controller_recorded_opponent::sRecordedOpponentBufferCommands)
        .cast(),
);

unsafe extern "C" {
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
    static mut gBattleOutcome: u8;
    static mut gBattleSpritesDataPtr: *mut BattleSpriteData;
    static mut gBattleStruct: *mut BattleStruct;
    static mut gBattleTypeFlags: u32;
    static mut gBattle_BG0_X: u16;
    static mut gBattle_BG0_Y: u16;
    static mut gBattlerControllerFuncs: CArray<Option<unsafe extern "C" fn()>, 4>;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gBattlerSpriteIds: CArray<u8, 4>;
    static mut gBattlerStatusSummaryTaskId: CArray<u8, 4>;
    static gBitTable: CArray<u32, 0>;
    static mut gDisplayedStringBattle: CArray<u8, 300>;
    static mut gDoingBattleAnim: u8;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gHealthboxSpriteIds: CArray<u8, 4>;
    static mut gIntroSlideFlags: u16;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gMPlayInfo_BGM: MusicPlayerInfo;
    static mut gMultiuseSpriteTemplate: SpriteTemplate;
    static mut gRecordedBattleMultiplayerId: u8;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static mut gTrainerBattleOpponent_A: u16;
    static mut gTrainerBattleOpponent_B: u16;
    static gTrainerFrontPicCoords: CArray<MonCoords, 0>;
    static gTrainerFrontPicPaletteTable: CArray<CompressedSpritePalette, 0>;
    static mut gTransformedPersonalities: CArray<u32, 4>;
    static mut gUnusedControllerStruct: UnusedControllerStruct;
    static mut gWeatherMoveAnim: u16;
    fn BattleLoadOpponentMonSpriteGfx(a0: *mut Pokemon, a1: u8);
    fn BattlePutTextOnWindow(a0: *mut u8, a1: u8);
    fn BattleStopLowHpSound();
    fn BeginFastPaletteFade(a0: u8);
    fn BtlController_EmitChosenMonReturnValue(a0: u8, a1: u8, a2: *mut u8);
    fn BtlController_EmitDataTransfer(a0: u8, a1: u16, a2: *mut c_void);
    fn BtlController_EmitTwoReturnValues(a0: u8, a1: u8, a2: u16);
    fn BufferStringBattle(a0: u16);
    fn ChooseMoveAndTargetInBattlePalace() -> u16;
    fn ClearTemporarySpeciesSpriteData(a0: u8, a1: u8);
    fn CopyAllBattleSpritesInvisibilities();
    fn CopyBattleSpriteInvisibility(a0: u8);
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut Sprite)>) -> u8;
    fn CreatePartyStatusSummarySprites(a0: u8, a1: *mut HpAndStatus, a2: u8, a3: u8) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressTrainerFrontPic(a0: u16, a1: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DoHitAnimHealthboxEffect(a0: u8);
    fn DoMoveAnim(a0: u16);
    fn DoPokeballSendOutAnimation(a0: i16, a1: u8) -> u8;
    fn FadeOutMapMusic(a0: u8);
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTrainerFrontPicPalette(a0: u16);
    fn GetActiveBattlerLinkPlayerGender() -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteDefault_Y(a0: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetFrontierTrainerFrontSpriteId(a0: u16) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn GetUnionRoomTrainerPic() -> u16;
    fn HandleIntroSlide(a0: u8);
    fn HideBattlerShadowSprite(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitAndLaunchChosenStatusAnimation(a0: u8, a1: u32);
    fn InitAndLaunchSpecialAnimation(a0: u8, a1: u8, a2: u8, a3: u8);
    fn IsBattleSEPlaying(a0: u8) -> u8;
    fn IsBattlerSpritePresent(a0: u8) -> u8;
    fn IsCryPlayingOrClearCrySongs() -> u8;
    fn IsDoubleBattle() -> u8;
    fn IsMoveWithoutAnimation(a0: u16, a1: u8) -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LoadBattleBarGfx(a0: u8);
    fn MoveBattleBar(a0: u8, a1: u8, a2: u8, a3: u8) -> i32;
    fn PlayBGM(a0: u16);
    fn PlayCry_ByMode(a0: u16, a1: i8, a2: u8);
    fn PlayFanfare(a0: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PlayerGenderToFrontTrainerPicId(a0: u8) -> u16;
    fn PrepareBufferDataTransferLink(a0: u8, a1: u16, a2: *mut u8);
    fn RecordedBattle_GetBattlerAction(a0: u8) -> u8;
    fn SetBattleBarStruct(a0: u8, a1: u8, a2: i32, a3: i32, a4: i32);
    fn SetBattleEndCallbacks();
    fn SetBattlerShadowSpriteCallback(a0: u8, a1: u16);
    fn SetBattlerSpriteAffineMode(a0: u8);
    fn SetHealthboxSpriteInvisible(a0: u8);
    fn SetHealthboxSpriteVisible(a0: u8);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetMultiuseSpriteTemplateToTrainerBack(a0: u16, a1: u8);
    fn SetSpritePrimaryCoordsFromSecondaryCoords(a0: *mut Sprite);
    fn SpriteCB_FaintOpponentMon(a0: *mut Sprite);
    fn SpriteCB_TrainerSlideIn(a0: *mut Sprite);
    fn SpriteCB_WaitForBattlerBallReleaseAnim(a0: *mut Sprite);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn SpriteCallbackDummy_2(a0: *mut Sprite);
    fn StartAnimLinearTranslation(a0: *mut Sprite);
    fn StartHealthboxSlideIn(a0: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_HidePartyStatusSummary(a0: u8);
    fn TryHandleLaunchBattleTableAnimation(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn TrySetBehindSubstituteSpriteBit(a0: u8, a1: u16);
    fn TryShinyAnimation(a0: u8, a1: *mut Pokemon);
    fn UpdateHealthboxAttribute(a0: u8, a1: *mut Pokemon, a2: u8);
    fn UpdateHpTextInHealthbox(a0: u8, a1: i16, a2: u8);
    fn m4aMPlayContinue(a0: *mut MusicPlayerInfo);
    fn m4aMPlayVolumeControl(a0: *mut MusicPlayerInfo, a1: u16, a2: u16);
}

pub(crate) unsafe extern "C" fn RecordedOpponentDummy() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetControllerToRecordedOpponent() {
    gBattlerControllerFuncs[gActiveBattler] = Some(RecordedOpponentBufferRunCommand);
}
pub(crate) unsafe extern "C" fn RecordedOpponentBufferRunCommand() {
    if gBattleControllerExecFlags & gBitTable[gActiveBattler] != 0 {
        if gBattleBufferA[gActiveBattler][0] < 57 {
            sRecordedOpponentBufferCommands[gBattleBufferA[gActiveBattler][0]].unwrap_unchecked()();
        } else {
            RecordedOpponentBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnBattlerSpriteCallbackDummy() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        RecordedOpponentBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnBankSpriteCallbackDummy2() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        RecordedOpponentBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn FreeTrainerSpriteAfterSlide() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        FreeTrainerFrontPicPalette(gSprites[gBattlerSpriteIds[gActiveBattler]].oam.affineParam);
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        RecordedOpponentBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn Intro_DelayAndEnd() {
    if ({
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay -= 1;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay
    }) == 255
    {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).introEndDelay = 0;
        RecordedOpponentBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn Intro_WaitForShinyAnimAndHealthbox() {
    let mut healthboxAnimDone: u8 = FALSE;
    if IsDoubleBattle() == 0 || IsDoubleBattle() != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            && gSprites[gBattlerSpriteIds[gActiveBattler]].animEnded() != 0
        {
            healthboxAnimDone = TRUE;
        }
    } else {
        if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            && gSprites[gHealthboxSpriteIds[gActiveBattler as i32 ^ 2]].callback
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            && gSprites[gBattlerSpriteIds[gActiveBattler]].animEnded() != 0
            && gSprites[gBattlerSpriteIds[gActiveBattler as i32 ^ 2]].animEnded() != 0
        {
            healthboxAnimDone = TRUE;
        }
    }
    if healthboxAnimDone != 0 {
        if GetBattlerPosition(gActiveBattler) == B_POSITION_OPPONENT_LEFT {
            if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).finishedShinyMonAnim()
                == 0
            {
                return;
            }
            if (*(*gBattleSpritesDataPtr)
                .healthBoxesData
                .at(gActiveBattler as i32 ^ 2))
            .finishedShinyMonAnim()
                == 0
            {
                return;
            }
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
            &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
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
            &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler as i32 ^ 2]],
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
                    &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler as i32 ^ 2]],
                    HEALTHBOX_ALL,
                );
                StartHealthboxSlideIn(gActiveBattler ^ 2);
                SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler as i32 ^ 2]);
            }
            UpdateHealthboxAttribute(
                gHealthboxSpriteIds[gActiveBattler],
                &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
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
                if GetBattlerPosition(gActiveBattler) == B_POSITION_OPPONENT_LEFT {
                    m4aMPlayContinue(&raw mut gMPlayInfo_BGM);
                }
            } else {
                m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
            }
        }
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_bgmRestored(TRUE);
        bgmRestored = TRUE as u32;
    }
    if IsDoubleBattle() == 0 {
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
            SetBattlerShadowSpriteCallback(
                gActiveBattler ^ 2,
                GetMonData2(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler as i32 ^ 2]],
                    MON_DATA_SPECIES,
                ) as u16,
            );
        }
        DestroySprite(&raw mut gSprites[gBattleControllerData[gActiveBattler]]);
        SetBattlerShadowSpriteCallback(
            gActiveBattler,
            GetMonData2(
                &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
                MON_DATA_SPECIES,
            ) as u16,
        );
        (*(*gBattleSpritesDataPtr).animationData).set_introAnimActive(FALSE);
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_bgmRestored(FALSE);
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_healthboxSlideInStarted(FALSE);
        gBattlerControllerFuncs[gActiveBattler] = Some(Intro_WaitForShinyAnimAndHealthbox);
    }
}
pub(crate) unsafe extern "C" fn TryShinyAnimAfterMonAnim() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
        && gSprites[gBattlerSpriteIds[gActiveBattler]].x2 == 0
    {
        if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).triedShinyMonAnim() == 0 {
            TryShinyAnimation(
                gActiveBattler,
                &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
            );
        } else {
            if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).finishedShinyMonAnim()
                != 0
            {
                (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
                    .set_triedShinyMonAnim(FALSE);
                (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
                    .set_finishedShinyMonAnim(FALSE);
                FreeSpriteTilesByTag(ANIM_TAG_GOLD_STARS);
                FreeSpritePaletteByTag(ANIM_TAG_GOLD_STARS);
                RecordedOpponentBufferExecCompleted();
            }
        }
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
        RecordedOpponentBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn HideHealthboxAfterMonFaint() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].inUse() == 0 {
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        RecordedOpponentBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn FreeMonSpriteAfterSwitchOutAnim() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0 {
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        HideBattlerShadowSprite(gActiveBattler);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        RecordedOpponentBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnInactiveTextPrinter() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        RecordedOpponentBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn DoHitAnimBlinkSpriteEffect() {
    let mut spriteId: u8 = gBattlerSpriteIds[gActiveBattler];
    if gSprites[spriteId].data[1] == 32 {
        gSprites[spriteId].data[1] = 0;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gDoingBattleAnim = FALSE;
        RecordedOpponentBufferExecCompleted();
    } else {
        if gSprites[spriteId].data[1] % 4 == 0 {
            gSprites[spriteId].set_invisible(gSprites[spriteId].invisible() ^ 1);
        }
        gSprites[spriteId].data[1] += 1;
    }
}
pub(crate) unsafe extern "C" fn SwitchIn_ShowSubstitute() {
    if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
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
        if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            || gSprites[gBattlerSpriteIds[gActiveBattler]].callback
                == Some(SpriteCallbackDummy_2 as unsafe extern "C" fn(*mut Sprite))
        {
            m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
            RecordedOpponentBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn SwitchIn_ShowHealthbox() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).finishedShinyMonAnim() != 0
        && gSprites[gBattlerSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_triedShinyMonAnim(FALSE);
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_finishedShinyMonAnim(FALSE);
        FreeSpriteTilesByTag(ANIM_TAG_GOLD_STARS);
        FreeSpritePaletteByTag(ANIM_TAG_GOLD_STARS);
        StartSpriteAnim(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]], 0);
        UpdateHealthboxAttribute(
            gHealthboxSpriteIds[gActiveBattler],
            &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
            HEALTHBOX_ALL,
        );
        StartHealthboxSlideIn(gActiveBattler);
        SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler]);
        CopyBattleSpriteInvisibility(gActiveBattler);
        gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_ShowSubstitute);
    }
}
pub(crate) unsafe extern "C" fn SwitchIn_TryShinyAnim() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).ballAnimActive() == 0
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).triedShinyMonAnim() == 0
    {
        TryShinyAnimation(
            gActiveBattler,
            &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
        );
    }
    if gSprites[gBattleControllerData[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
        && (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).ballAnimActive() == 0
    {
        DestroySprite(&raw mut gSprites[gBattleControllerData[gActiveBattler]]);
        SetBattlerShadowSpriteCallback(
            gActiveBattler,
            GetMonData2(
                &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
                MON_DATA_SPECIES,
            ) as u16,
        );
        gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_ShowHealthbox);
    }
}
pub(crate) unsafe extern "C" fn CompleteOnFinishedStatusAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).statusAnimActive() == 0 {
        RecordedOpponentBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnFinishedBattleAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animFromTableActive() == 0 {
        RecordedOpponentBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn RecordedOpponentBufferExecCompleted() {
    gBattlerControllerFuncs[gActiveBattler] = Some(RecordedOpponentBufferRunCommand);
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        let mut playerId: u8 = GetMultiplayerId();
        PrepareBufferDataTransferLink(B_COMM_CONTROLLER_IS_DONE, 4, &raw mut playerId);
        gBattleBufferA[gActiveBattler][0] = CONTROLLER_TERMINATOR_NOP;
    } else {
        gBattleControllerExecFlags &= !gBitTable[gActiveBattler];
    }
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleGetMonData() {
    let mut monData: CArray<u8, 256> = zeroed();
    let mut size: u32 = 0;
    let mut monToCheck: u8 = 0;
    let mut i: i32 = 0;
    if gBattleBufferA[gActiveBattler][2] == 0 {
        size += CopyRecordedOpponentMonData(
            gBattlerPartyIndexes[gActiveBattler] as u8,
            monData.as_mut_ptr(),
        );
    } else {
        monToCheck = gBattleBufferA[gActiveBattler][2];
        i = 0;
        while i < PARTY_SIZE {
            if monToCheck as i32 & 1 != 0 {
                size += CopyRecordedOpponentMonData(i as u8, monData.as_mut_ptr().at(size));
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
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn CopyRecordedOpponentMonData(monId: u8, mut dst: *mut u8) -> u32 {
    let mut battleMon: BattlePokemon = zeroed();
    let mut moveData: MovePPInfo = zeroed();
    let mut nickname: CArray<u8, 20> = zeroed();
    let mut src: *mut u8 = null_mut();
    let mut data16: i16 = 0;
    let mut data32: u32 = 0;
    let mut size: i32 = 0;
    match gBattleBufferA[gActiveBattler][1] {
        REQUEST_ALL_BATTLE => {
            battleMon.species = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPECIES) as u16;
            battleMon.item = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_HELD_ITEM) as u16;
            size = 0;
            while size < MAX_MON_MOVES {
                battleMon.moves[size] =
                    GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_MOVE1 + size) as u16;
                battleMon.pp[size] =
                    GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_PP1 + size) as u8;
                size += 1;
            }
            battleMon.ppBonuses =
                GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_PP_BONUSES) as u8;
            battleMon.friendship =
                GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_FRIENDSHIP) as u8;
            battleMon.experience = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_EXP);
            battleMon.set_hpIV(GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_HP_IV));
            battleMon.set_attackIV(GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_ATK_IV));
            battleMon.set_defenseIV(GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_DEF_IV));
            battleMon.set_speedIV(GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPEED_IV));
            battleMon.set_spAttackIV(GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPATK_IV));
            battleMon.set_spDefenseIV(GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPDEF_IV));
            battleMon.personality = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_PERSONALITY);
            battleMon.status1 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_STATUS);
            battleMon.level = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_LEVEL) as u8;
            battleMon.hp = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_HP) as u16;
            battleMon.maxHP = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_MAX_HP) as u16;
            battleMon.attack = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_ATK) as u16;
            battleMon.defense = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_DEF) as u16;
            battleMon.speed = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPEED) as u16;
            battleMon.spAttack = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPATK) as u16;
            battleMon.spDefense = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPDEF) as u16;
            battleMon.set_isEgg(GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_IS_EGG));
            battleMon.set_abilityNum(GetMonData2(
                &raw mut gEnemyParty[monId],
                MON_DATA_ABILITY_NUM,
            ));
            battleMon.otId = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_OT_ID);
            GetMonData3(
                &raw mut gEnemyParty[monId],
                MON_DATA_NICKNAME,
                nickname.as_mut_ptr(),
            );
            StringCopy_Nickname(battleMon.nickname.as_mut_ptr(), nickname.as_mut_ptr());
            GetMonData3(
                &raw mut gEnemyParty[monId],
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
            data16 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPECIES) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_HELDITEM_BATTLE => {
            data16 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_HELD_ITEM) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_MOVES_PP_BATTLE => {
            size = 0;
            while size < MAX_MON_MOVES {
                moveData.moves[size] =
                    GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_MOVE1 + size) as u16;
                moveData.pp[size] =
                    GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_PP1 + size) as u8;
                size += 1;
            }
            moveData.ppBonuses =
                GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_PP_BONUSES) as u8;
            src = &raw mut moveData as *mut u8;
            size = 0;
            while size < 16 {
                *dst.at(size) = *src.at(size);
                size += 1;
            }
        }
        4 | REQUEST_MOVE2_BATTLE | REQUEST_MOVE3_BATTLE | REQUEST_MOVE4_BATTLE => {
            data16 = GetMonData2(
                &raw mut gEnemyParty[monId],
                MON_DATA_MOVE1 + gBattleBufferA[gActiveBattler][1] as i32 - REQUEST_MOVE1_BATTLE,
            ) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_PP_DATA_BATTLE => {
            size = 0;
            while size < MAX_MON_MOVES {
                *dst.at(size) = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_PP1 + size) as u8;
                size += 1;
            }
            *dst.at(size) = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_PP_BONUSES) as u8;
            size += 1;
        }
        REQUEST_PPMOVE1_BATTLE
        | REQUEST_PPMOVE2_BATTLE
        | REQUEST_PPMOVE3_BATTLE
        | REQUEST_PPMOVE4_BATTLE => {
            *dst = GetMonData2(
                &raw mut gEnemyParty[monId],
                MON_DATA_PP1 + gBattleBufferA[gActiveBattler][1] as i32
                    - REQUEST_PPMOVE1_BATTLE as i32,
            ) as u8;
            size = 1;
        }
        REQUEST_OTID_BATTLE => {
            data32 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_OT_ID);
            *dst = data32 as u8 & 0x000000FF;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_EXP_BATTLE => {
            data32 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_EXP);
            *dst = data32 as u8 & 0x000000FF;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            size = 3;
        }
        REQUEST_HP_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_HP_EV) as u8;
            size = 1;
        }
        REQUEST_ATK_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_ATK_EV) as u8;
            size = 1;
        }
        REQUEST_DEF_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_DEF_EV) as u8;
            size = 1;
        }
        REQUEST_SPEED_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPEED_EV) as u8;
            size = 1;
        }
        REQUEST_SPATK_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPATK_EV) as u8;
            size = 1;
        }
        REQUEST_SPDEF_EV_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPDEF_EV) as u8;
            size = 1;
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_FRIENDSHIP) as u8;
            size = 1;
        }
        REQUEST_POKERUS_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_POKERUS) as u8;
            size = 1;
        }
        REQUEST_MET_LOCATION_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_MET_LOCATION) as u8;
            size = 1;
        }
        REQUEST_MET_LEVEL_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_MET_LEVEL) as u8;
            size = 1;
        }
        REQUEST_MET_GAME_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_MET_GAME) as u8;
            size = 1;
        }
        REQUEST_POKEBALL_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_POKEBALL) as u8;
            size = 1;
        }
        REQUEST_ALL_IVS_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_HP_IV) as u8;
            *dst.at(1) = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_ATK_IV) as u8;
            *dst.at(2) = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_DEF_IV) as u8;
            *dst.at(3) = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPEED_IV) as u8;
            *dst.at(4) = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPATK_IV) as u8;
            *dst.at(5) = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPDEF_IV) as u8;
            size = 6;
        }
        REQUEST_HP_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_HP_IV) as u8;
            size = 1;
        }
        REQUEST_ATK_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_ATK_IV) as u8;
            size = 1;
        }
        REQUEST_DEF_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_DEF_IV) as u8;
            size = 1;
        }
        REQUEST_SPEED_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPEED_IV) as u8;
            size = 1;
        }
        REQUEST_SPATK_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPATK_IV) as u8;
            size = 1;
        }
        REQUEST_SPDEF_IV_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPDEF_IV) as u8;
            size = 1;
        }
        REQUEST_PERSONALITY_BATTLE => {
            data32 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_PERSONALITY);
            *dst = data32 as u8 & 0x000000FF;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            *dst.at(3) = ((data32 & 0xFF000000) >> 24) as u8;
            size = 4;
        }
        REQUEST_CHECKSUM_BATTLE => {
            data16 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_CHECKSUM) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_STATUS_BATTLE => {
            data32 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_STATUS);
            *dst = data32 as u8 & 0x000000FF;
            *dst.at(1) = ((data32 & 0x0000FF00) >> 8) as u8;
            *dst.at(2) = ((data32 & 0x00FF0000) >> 16) as u8;
            *dst.at(3) = ((data32 & 0xFF000000) >> 24) as u8;
            size = 4;
        }
        REQUEST_LEVEL_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_LEVEL) as u8;
            size = 1;
        }
        REQUEST_HP_BATTLE => {
            data16 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_HP) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_MAX_HP_BATTLE => {
            data16 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_MAX_HP) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_ATK_BATTLE => {
            data16 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_ATK) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_DEF_BATTLE => {
            data16 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_DEF) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPEED_BATTLE => {
            data16 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPEED) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPATK_BATTLE => {
            data16 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPATK) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_SPDEF_BATTLE => {
            data16 = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SPDEF) as i16;
            *dst = data16 as u8;
            *dst.at(1) = (data16 >> 8) as u8;
            size = 2;
        }
        REQUEST_COOL_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_COOL) as u8;
            size = 1;
        }
        REQUEST_BEAUTY_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_BEAUTY) as u8;
            size = 1;
        }
        REQUEST_CUTE_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_CUTE) as u8;
            size = 1;
        }
        REQUEST_SMART_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SMART) as u8;
            size = 1;
        }
        REQUEST_TOUGH_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_TOUGH) as u8;
            size = 1;
        }
        REQUEST_SHEEN_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SHEEN) as u8;
            size = 1;
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_COOL_RIBBON) as u8;
            size = 1;
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_BEAUTY_RIBBON) as u8;
            size = 1;
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_CUTE_RIBBON) as u8;
            size = 1;
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_SMART_RIBBON) as u8;
            size = 1;
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            *dst = GetMonData2(&raw mut gEnemyParty[monId], MON_DATA_TOUGH_RIBBON) as u8;
            size = 1;
        }
        _ => {}
    }
    return size as u32;
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleGetRawMonData() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleSetMonData() {
    let mut monToCheck: u8 = 0;
    let mut i: u8 = 0;
    if gBattleBufferA[gActiveBattler][2] == 0 {
        SetRecordedOpponentMonData(gBattlerPartyIndexes[gActiveBattler] as u8);
    } else {
        monToCheck = gBattleBufferA[gActiveBattler][2];
        i = 0;
        while i < PARTY_SIZE as u8 {
            if monToCheck as i32 & 1 != 0 {
                SetRecordedOpponentMonData(i);
            }
            monToCheck >>= 1;
            i += 1;
        }
    }
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SetRecordedOpponentMonData(monId: u8) {
    let mut battlePokemon: *mut BattlePokemon =
        &raw mut gBattleBufferA[gActiveBattler][3] as *mut BattlePokemon;
    let mut moveData: *mut MovePPInfo =
        &raw mut gBattleBufferA[gActiveBattler][3] as *mut MovePPInfo;
    let mut i: i32 = 0;
    match gBattleBufferA[gActiveBattler][1] {
        REQUEST_ALL_BATTLE => {
            let mut iv: u8 = 0;
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPECIES,
                &raw mut (*battlePokemon).species as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_HELD_ITEM,
                &raw mut (*battlePokemon).item as *mut c_void,
            );
            i = 0;
            while i < MAX_MON_MOVES {
                SetMonData(
                    &raw mut gEnemyParty[monId],
                    MON_DATA_MOVE1 + i,
                    &raw mut (*battlePokemon).moves[i] as *mut c_void,
                );
                SetMonData(
                    &raw mut gEnemyParty[monId],
                    MON_DATA_PP1 + i,
                    &raw mut (*battlePokemon).pp[i] as *mut c_void,
                );
                i += 1;
            }
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_PP_BONUSES,
                &raw mut (*battlePokemon).ppBonuses as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_FRIENDSHIP,
                &raw mut (*battlePokemon).friendship as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_EXP,
                &raw mut (*battlePokemon).experience as *mut c_void,
            );
            iv = (*battlePokemon).hpIV() as u8;
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_HP_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).attackIV() as u8;
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_ATK_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).defenseIV() as u8;
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_DEF_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).speedIV() as u8;
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPEED_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).spAttackIV() as u8;
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPATK_IV,
                &raw mut iv as *mut c_void,
            );
            iv = (*battlePokemon).spDefenseIV() as u8;
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPDEF_IV,
                &raw mut iv as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_PERSONALITY,
                &raw mut (*battlePokemon).personality as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_STATUS,
                &raw mut (*battlePokemon).status1 as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_LEVEL,
                &raw mut (*battlePokemon).level as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_HP,
                &raw mut (*battlePokemon).hp as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_MAX_HP,
                &raw mut (*battlePokemon).maxHP as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_ATK,
                &raw mut (*battlePokemon).attack as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_DEF,
                &raw mut (*battlePokemon).defense as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPEED,
                &raw mut (*battlePokemon).speed as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPATK,
                &raw mut (*battlePokemon).spAttack as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPDEF,
                &raw mut (*battlePokemon).spDefense as *mut c_void,
            );
        }
        REQUEST_SPECIES_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPECIES,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_HELDITEM_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_HELD_ITEM,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MOVES_PP_BATTLE => {
            i = 0;
            while i < MAX_MON_MOVES {
                SetMonData(
                    &raw mut gEnemyParty[monId],
                    MON_DATA_MOVE1 + i,
                    &raw mut (*moveData).moves[i] as *mut c_void,
                );
                SetMonData(
                    &raw mut gEnemyParty[monId],
                    MON_DATA_PP1 + i,
                    &raw mut (*moveData).pp[i] as *mut c_void,
                );
                i += 1;
            }
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_PP_BONUSES,
                &raw mut (*moveData).ppBonuses as *mut c_void,
            );
        }
        4 | REQUEST_MOVE2_BATTLE | REQUEST_MOVE3_BATTLE | REQUEST_MOVE4_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_MOVE1 + gBattleBufferA[gActiveBattler][1] as i32 - REQUEST_MOVE1_BATTLE,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_PP_DATA_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_PP1,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_PP2,
                &raw mut gBattleBufferA[gActiveBattler][4] as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_PP3,
                &raw mut gBattleBufferA[gActiveBattler][5] as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_PP4,
                &raw mut gBattleBufferA[gActiveBattler][6] as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_PP_BONUSES,
                &raw mut gBattleBufferA[gActiveBattler][7] as *mut c_void,
            );
        }
        REQUEST_PPMOVE1_BATTLE
        | REQUEST_PPMOVE2_BATTLE
        | REQUEST_PPMOVE3_BATTLE
        | REQUEST_PPMOVE4_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_PP1 + gBattleBufferA[gActiveBattler][1] as i32
                    - REQUEST_PPMOVE1_BATTLE as i32,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_OTID_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_OT_ID,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_EXP_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_EXP,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_HP_EV_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_HP_EV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ATK_EV_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_ATK_EV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_DEF_EV_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_DEF_EV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_EV_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPEED_EV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_EV_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPATK_EV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_EV_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPDEF_EV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_FRIENDSHIP_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_FRIENDSHIP,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_POKERUS_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_POKERUS,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MET_LOCATION_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_MET_LOCATION,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MET_LEVEL_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_MET_LEVEL,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MET_GAME_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_MET_GAME,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_POKEBALL_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_POKEBALL,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ALL_IVS_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_HP_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_ATK_IV,
                &raw mut gBattleBufferA[gActiveBattler][4] as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_DEF_IV,
                &raw mut gBattleBufferA[gActiveBattler][5] as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPEED_IV,
                &raw mut gBattleBufferA[gActiveBattler][6] as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPATK_IV,
                &raw mut gBattleBufferA[gActiveBattler][7] as *mut c_void,
            );
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPDEF_IV,
                &raw mut gBattleBufferA[gActiveBattler][8] as *mut c_void,
            );
        }
        REQUEST_HP_IV_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_HP_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ATK_IV_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_ATK_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_DEF_IV_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_DEF_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_IV_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPEED_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_IV_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPATK_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_IV_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPDEF_IV,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_PERSONALITY_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_PERSONALITY,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_CHECKSUM_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_CHECKSUM,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_STATUS_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_STATUS,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_LEVEL_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_LEVEL,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_HP_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_HP,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_MAX_HP_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_MAX_HP,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_ATK_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_ATK,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_DEF_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_DEF,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPEED_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPEED,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPATK_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPATK,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SPDEF_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SPDEF,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_COOL_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_COOL,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_BEAUTY,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_CUTE,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SMART_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SMART,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_TOUGH,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SHEEN_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SHEEN,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_COOL_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_COOL_RIBBON,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_BEAUTY_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_BEAUTY_RIBBON,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_CUTE_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_CUTE_RIBBON,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_SMART_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_SMART_RIBBON,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        REQUEST_TOUGH_RIBBON_BATTLE => {
            SetMonData(
                &raw mut gEnemyParty[monId],
                MON_DATA_TOUGH_RIBBON,
                &raw mut gBattleBufferA[gActiveBattler][3] as *mut c_void,
            );
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleSetRawMonData() {
    let mut dst: *mut u8 = (&raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]] as *mut u8)
        .at(gBattleBufferA[gActiveBattler][1]);
    let mut i: u8 = 0;
    i = 0;
    while i < gBattleBufferA[gActiveBattler][2] {
        *dst.at(i) = gBattleBufferA[gActiveBattler][3 + i as i32];
        i += 1;
    }
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleLoadMonSprite() {
    let mut species: u16 = GetMonData2(
        &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
        MON_DATA_SPECIES,
    ) as u16;
    BattleLoadOpponentMonSpriteGfx(
        &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
        gActiveBattler,
    );
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
    SetBattlerShadowSpriteCallback(
        gActiveBattler,
        GetMonData2(
            &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_SPECIES,
        ) as u16,
    );
    gBattlerControllerFuncs[gActiveBattler] = Some(TryShinyAnimAfterMonAnim);
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleSwitchInAnim() {
    gBattlerPartyIndexes[gActiveBattler] = gBattleBufferA[gActiveBattler][1] as u16;
    StartSendOutAnim(gActiveBattler, gBattleBufferA[gActiveBattler][2]);
    gBattlerControllerFuncs[gActiveBattler] = Some(SwitchIn_TryShinyAnim);
}
pub(crate) unsafe extern "C" fn StartSendOutAnim(battler: u8, dontClearSubstituteBit: u8) {
    let mut species: u16 = 0;
    ClearTemporarySpeciesSpriteData(battler, dontClearSubstituteBit);
    gBattlerPartyIndexes[battler] = gBattleBufferA[battler][1] as u16;
    species = GetMonData2(
        &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
        MON_DATA_SPECIES,
    ) as u16;
    gBattleControllerData[battler] =
        CreateInvisibleSpriteWithCallback(Some(SpriteCB_WaitForBattlerBallReleaseAnim));
    BattleLoadOpponentMonSpriteGfx(&raw mut gEnemyParty[gBattlerPartyIndexes[battler]], battler);
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
        DoPokeballSendOutAnimation(0, POKEBALL_OPPONENT_SENDOUT) as i16;
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleReturnMonToBall() {
    if gBattleBufferA[gActiveBattler][1] == 0 {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
        gBattlerControllerFuncs[gActiveBattler] = Some(DoSwitchOutAnimation);
    } else {
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        HideBattlerShadowSprite(gActiveBattler);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        RecordedOpponentBufferExecCompleted();
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
                    B_ANIM_SWITCH_OUT_OPPONENT_MON,
                );
                gBattlerControllerFuncs[gActiveBattler] = Some(FreeMonSpriteAfterSwitchOutAnim);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleDrawTrainerPic() {
    let mut xPos: i16 = 0;
    let mut trainerPicId: u32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if GetBattlerPosition(gActiveBattler) as i32 & BIT_FLANK as i32 != 0 {
            xPos = 152;
        } else {
            xPos = 200;
        }
        if gBattleTypeFlags & BATTLE_TYPE_BATTLE_TOWER != 0 {
            if gActiveBattler == 1 {
                trainerPicId = GetFrontierTrainerFrontSpriteId(gTrainerBattleOpponent_A) as u32;
            } else {
                trainerPicId = GetFrontierTrainerFrontSpriteId(gTrainerBattleOpponent_B) as u32;
            }
        } else {
            trainerPicId =
                PlayerGenderToFrontTrainerPicId(GetActiveBattlerLinkPlayerGender()) as u32;
        }
    } else {
        xPos = 176;
        if gTrainerBattleOpponent_A == TRAINER_UNION_ROOM {
            trainerPicId = GetUnionRoomTrainerPic() as u32;
        } else {
            trainerPicId = PlayerGenderToFrontTrainerPicId(
                gLinkPlayers[gRecordedBattleMultiplayerId as i32 ^ BIT_SIDE as i32].gender,
            ) as u32;
        }
    }
    DecompressTrainerFrontPic(trainerPicId as u16, gActiveBattler);
    SetMultiuseSpriteTemplateToTrainerBack(trainerPicId as u16, GetBattlerPosition(gActiveBattler));
    gBattlerSpriteIds[gActiveBattler] = CreateSprite(
        &raw mut gMultiuseSpriteTemplate,
        xPos,
        (8 - gTrainerFrontPicCoords[trainerPicId].size as i16) * 4 + 40,
        GetBattlerSpriteSubpriority(gActiveBattler),
    );
    gSprites[gBattlerSpriteIds[gActiveBattler]].x2 = -240;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = 2;
    gSprites[gBattlerSpriteIds[gActiveBattler]]
        .oam
        .set_paletteNum(
            IndexOfSpritePaletteTag(gTrainerFrontPicPaletteTable[trainerPicId].tag) as u16,
        );
    gSprites[gBattlerSpriteIds[gActiveBattler]].oam.affineParam = trainerPicId as u16;
    gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(SpriteCB_TrainerSlideIn);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnBattlerSpriteCallbackDummy);
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleTrainerSlide() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleTrainerSlideBack() {
    SetSpritePrimaryCoordsFromSecondaryCoords(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = 35;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[2] = 280;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[4] =
        gSprites[gBattlerSpriteIds[gActiveBattler]].y;
    gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(
        &raw mut gSprites[gBattlerSpriteIds[gActiveBattler]],
        Some(SpriteCallbackDummy),
    );
    gBattlerControllerFuncs[gActiveBattler] = Some(FreeTrainerSpriteAfterSlide);
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleFaintAnimation() {
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
            PlaySE12WithPanning(SE_FAINT, SOUND_PAN_TARGET);
            gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(SpriteCB_FaintOpponentMon);
            gBattlerControllerFuncs[gActiveBattler] = Some(HideHealthboxAfterMonFaint);
        }
    }
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandlePaletteFade() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleSuccessBallThrowAnim() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleBallThrowAnim() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandlePause() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleMoveAnimation() {
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
            RecordedOpponentBufferExecCompleted();
        } else {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
            gBattlerControllerFuncs[gActiveBattler] = Some(RecordedOpponentDoMoveAnimation);
        }
    }
}
pub(crate) unsafe extern "C" fn RecordedOpponentDoMoveAnimation() {
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
                RecordedOpponentBufferExecCompleted();
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandlePrintString() {
    let mut stringId: *mut u16 = null_mut();
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    stringId = &raw mut gBattleBufferA[gActiveBattler][2] as *mut u16;
    BufferStringBattle(*stringId);
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnInactiveTextPrinter);
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandlePrintSelectionString() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleChooseAction() {
    BtlController_EmitTwoReturnValues(
        B_COMM_TO_ENGINE,
        RecordedBattle_GetBattlerAction(gActiveBattler),
        0,
    );
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleYesNoBox() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleChooseMove() {
    if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0 {
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            B_ACTION_EXEC_SCRIPT,
            ChooseMoveAndTargetInBattlePalace(),
        );
    } else {
        let mut moveIndex: u8 = RecordedBattle_GetBattlerAction(gActiveBattler);
        let mut target: u8 = RecordedBattle_GetBattlerAction(gActiveBattler);
        BtlController_EmitTwoReturnValues(
            B_COMM_TO_ENGINE,
            B_ACTION_EXEC_SCRIPT,
            moveIndex as u16 | (target as u16) << 8,
        );
    }
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleChooseItem() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleChoosePokemon() {
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
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleCmd23() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleHealthBarUpdate() {
    let mut hpVal: i16 = 0;
    LoadBattleBarGfx(0);
    hpVal =
        gBattleBufferA[gActiveBattler][2] as i16 | (gBattleBufferA[gActiveBattler][3] as i16) << 8;
    if hpVal != INSTANT_HP_BAR_DROP {
        let mut maxHP: u32 = GetMonData2(
            &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_MAX_HP,
        );
        let mut curHP: u32 = GetMonData2(
            &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
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
            &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
            MON_DATA_MAX_HP,
        );
        SetBattleBarStruct(
            gActiveBattler,
            gHealthboxSpriteIds[gActiveBattler],
            maxHP as i32,
            0,
            hpVal as i32,
        );
    }
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnHealthbarDone);
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleExpUpdate() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleStatusIconUpdate() {
    if IsBattleSEPlaying(gActiveBattler) == 0 {
        let mut battler: u8 = 0;
        UpdateHealthboxAttribute(
            gHealthboxSpriteIds[gActiveBattler],
            &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
            HEALTHBOX_STATUS_ICON,
        );
        battler = gActiveBattler;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_statusAnimActive(0);
        gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedStatusAnimation);
    }
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleStatusAnimation() {
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
pub(crate) unsafe extern "C" fn RecordedOpponentHandleStatusXor() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleDataTransfer() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleDMA3Transfer() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandlePlayBGM() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleCmd32() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleTwoReturnValues() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleChosenMonReturnValue() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleOneReturnValue() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleOneReturnValue_Duplicate() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleClearUnkVar() {
    gUnusedControllerStruct.set_unk(0);
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleSetUnkVar() {
    gUnusedControllerStruct.set_unk(gBattleBufferA[gActiveBattler][1]);
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleClearUnkFlag() {
    gUnusedControllerStruct.set_flag(0);
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleToggleUnkFlag() {
    gUnusedControllerStruct.set_flag(gUnusedControllerStruct.flag() ^ 1);
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleHitAnimation() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].invisible() == TRUE as u16 {
        RecordedOpponentBufferExecCompleted();
    } else {
        gDoingBattleAnim = TRUE;
        gSprites[gBattlerSpriteIds[gActiveBattler]].data[1] = 0;
        DoHitAnimHealthboxEffect(gActiveBattler);
        gBattlerControllerFuncs[gActiveBattler] = Some(DoHitAnimBlinkSpriteEffect);
    }
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleCantSwitch() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandlePlaySE() {
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
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandlePlayFanfareOrBGM() {
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
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleFaintingCry() {
    let mut species: u16 = GetMonData2(
        &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
        MON_DATA_SPECIES,
    ) as u16;
    PlayCry_ByMode(species, 25, CRY_MODE_FAINT);
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleIntroSlide() {
    HandleIntroSlide(gBattleBufferA[gActiveBattler][1]);
    gIntroSlideFlags |= 1;
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleIntroTrainerBallThrow() {
    let mut taskId: u8 = 0;
    SetSpritePrimaryCoordsFromSecondaryCoords(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[0] = 35;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[2] = 280;
    gSprites[gBattlerSpriteIds[gActiveBattler]].data[4] =
        gSprites[gBattlerSpriteIds[gActiveBattler]].y;
    gSprites[gBattlerSpriteIds[gActiveBattler]].callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(
        &raw mut gSprites[gBattlerSpriteIds[gActiveBattler]],
        Some(SpriteCB_FreeOpponentSprite),
    );
    taskId = CreateTask(Some(Task_StartSendOutAnim), 5);
    gTasks[taskId].data[0] = gActiveBattler as i16;
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusSummaryShown() != 0
    {
        gTasks[gBattlerStatusSummaryTaskId[gActiveBattler]].func =
            Some(Task_HidePartyStatusSummary);
    }
    (*(*gBattleSpritesDataPtr).animationData).set_introAnimActive(TRUE);
    gBattlerControllerFuncs[gActiveBattler] = Some(RecordedOpponentDummy);
}
pub(crate) unsafe extern "C" fn Task_StartSendOutAnim(taskId: u8) {
    let mut savedActiveBank: u8 = gActiveBattler;
    gActiveBattler = gTasks[taskId].data[0] as u8;
    if IsDoubleBattle() == 0 || gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        gBattleBufferA[gActiveBattler][1] = gBattlerPartyIndexes[gActiveBattler] as u8;
        StartSendOutAnim(gActiveBattler, FALSE);
    } else {
        gBattleBufferA[gActiveBattler][1] = gBattlerPartyIndexes[gActiveBattler] as u8;
        StartSendOutAnim(gActiveBattler, FALSE);
        gActiveBattler ^= BIT_FLANK;
        gBattleBufferA[gActiveBattler][1] = gBattlerPartyIndexes[gActiveBattler] as u8;
        StartSendOutAnim(gActiveBattler, FALSE);
        gActiveBattler ^= BIT_FLANK;
    }
    gBattlerControllerFuncs[gActiveBattler] = Some(Intro_TryShinyAnimShowHealthbox);
    gActiveBattler = savedActiveBank;
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn SpriteCB_FreeOpponentSprite(sprite: *mut Sprite) {
    FreeTrainerFrontPicPalette((*sprite).oam.affineParam);
    FreeSpriteOamMatrix(sprite);
    DestroySprite(sprite);
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleDrawPartyStatusSummary() {
    if gBattleBufferA[gActiveBattler][1] != 0 && GetBattlerSide(gActiveBattler) == 0 {
        RecordedOpponentBufferExecCompleted();
    } else {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_partyStatusSummaryShown(1);
        if gBattleBufferA[gActiveBattler][2] != 0 {
            if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
                .opponentDrawPartyStatusSummaryDelay()
                < 2
            {
                (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
                    .set_opponentDrawPartyStatusSummaryDelay(
                        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
                            .opponentDrawPartyStatusSummaryDelay()
                            + 1,
                    );
                return;
            } else {
                (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
                    .set_opponentDrawPartyStatusSummaryDelay(0);
            }
        }
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
        RecordedOpponentBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleHidePartyStatusSummary() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).partyStatusSummaryShown() != 0
    {
        gTasks[gBattlerStatusSummaryTaskId[gActiveBattler]].func =
            Some(Task_HidePartyStatusSummary);
    }
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleEndBounceEffect() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleSpriteInvisibility() {
    if IsBattlerSpritePresent(gActiveBattler) != 0 {
        gSprites[gBattlerSpriteIds[gActiveBattler]]
            .set_invisible(gBattleBufferA[gActiveBattler][1] as u16);
        CopyBattleSpriteInvisibility(gActiveBattler);
    }
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleBattleAnimation() {
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
            RecordedOpponentBufferExecCompleted();
        } else {
            gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedBattleAnimation);
        }
    }
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleLinkStandbyMsg() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleResetActionMoveSelection() {
    RecordedOpponentBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn RecordedOpponentHandleEndLinkBattle() {
    if gBattleBufferA[gActiveBattler][1] == B_OUTCOME_DREW {
        gBattleOutcome = gBattleBufferA[gActiveBattler][1];
    } else {
        gBattleOutcome = gBattleBufferA[gActiveBattler][1] ^ B_OUTCOME_DREW;
    }
    FadeOutMapMusic(5);
    BeginFastPaletteFade(3);
    RecordedOpponentBufferExecCompleted();
    gBattlerControllerFuncs[gActiveBattler] = Some(SetBattleEndCallbacks);
}
pub(crate) unsafe extern "C" fn RecordedOpponentCmdEnd() {}
