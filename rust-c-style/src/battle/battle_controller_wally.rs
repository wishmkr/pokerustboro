//! Translated from `src/battle_controller_wally.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sWallyBufferCommands

static sWallyBufferCommands: Table<CArray<Option<unsafe extern "C" fn()>, 57>> =
    Table((&raw const crate::data::battle_controller_wally::sWallyBufferCommands).cast());

unsafe extern "C" {
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
    static mut gBattleOutcome: u8;
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
    static gBitTable: CArray<u32, 0>;
    static mut gDisplayedStringBattle: CArray<u8, 300>;
    static mut gDoingBattleAnim: u8;
    static mut gHealthboxSpriteIds: CArray<u8, 4>;
    static mut gIntroSlideFlags: u16;
    static mut gMain: Main;
    static mut gMultiuseSpriteTemplate: SpriteTemplate;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gSpecialVar_ItemId: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gText_BattleMenu: CArray<u8, 0>;
    static gText_WhatWillWallyDo: CArray<u8, 0>;
    static gTrainerBackPicCoords: CArray<MonCoords, 0>;
    static gTrainerBackPicPaletteTable: CArray<CompressedSpritePalette, 0>;
    static mut gTransformedPersonalities: CArray<u32, 4>;
    static mut gWeatherMoveAnim: u16;
    fn ActionSelectionCreateCursorAt(a0: u8, a1: u8);
    fn ActionSelectionDestroyCursorAt(a0: u8);
    fn AllocSpritePalette(a0: u16) -> u8;
    fn BattleControllerDummy();
    fn BattleMainCB2();
    fn BattlePutTextOnWindow(a0: *mut u8, a1: u8);
    fn BattleStopLowHpSound();
    fn BattleStringExpandPlaceholdersToDisplayedString(a0: *mut u8) -> u32;
    fn BeginFastPaletteFade(a0: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BtlController_EmitDataTransfer(a0: u8, a1: u16, a2: *mut c_void);
    fn BtlController_EmitOneReturnValue(a0: u8, a1: u16);
    fn BtlController_EmitTwoReturnValues(a0: u8, a1: u8, a2: u16);
    fn BufferStringBattle(a0: u16);
    fn CopyAllBattleSpritesInvisibilities();
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut Sprite)>) -> u8;
    fn CreatePartyStatusSummarySprites(a0: u8, a1: *mut HpAndStatus, a2: u8, a3: u8) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressTrainerBackPic(a0: u16, a1: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DoHitAnimHealthboxEffect(a0: u8);
    fn DoMoveAnim(a0: u16);
    fn DoPokeballSendOutAnimation(a0: i16, a1: u8) -> u8;
    fn DoWallyTutorialBagMenu();
    fn FadeOutMapMusic(a0: u8);
    fn FreeAllWindowBuffers();
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteDefault_Y(a0: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn HandleIntroSlide(a0: u8);
    fn HandleLowHpMusicChange(a0: *mut Pokemon, a1: u8);
    fn InitAndLaunchSpecialAnimation(a0: u8, a1: u8, a2: u8, a3: u8);
    fn InitMoveSelectionsVarsAndStrings();
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsDoubleBattle() -> u8;
    fn IsMoveWithoutAnimation(a0: u16, a1: u8) -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LoadBattleBarGfx(a0: u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn MoveBattleBar(a0: u8, a1: u8, a2: u8, a3: u8) -> i32;
    fn PlayBGM(a0: u16);
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn PlayerHandleGetRawMonData();
    fn PrepareBufferDataTransferLink(a0: u8, a1: u16, a2: *mut u8);
    fn ReshowBattleScreenDummy();
    fn SetBattleBarStruct(a0: u8, a1: u8, a2: i32, a3: i32, a4: i32);
    fn SetBattleEndCallbacks();
    fn SetBattlerSpriteAffineMode(a0: u8);
    fn SetHealthboxSpriteInvisible(a0: u8);
    fn SetHealthboxSpriteVisible(a0: u8);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetMultiuseSpriteTemplateToTrainerBack(a0: u16, a1: u8);
    fn SetSpritePrimaryCoordsFromSecondaryCoords(a0: *mut Sprite);
    fn SpriteCB_FreePlayerSpriteLoadMonSprite(a0: *mut Sprite);
    fn SpriteCB_TrainerSlideIn(a0: *mut Sprite);
    fn SpriteCB_WaitForBattlerBallReleaseAnim(a0: *mut Sprite);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartAnimLinearTranslation(a0: *mut Sprite);
    fn StartHealthboxSlideIn(a0: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_HidePartyStatusSummary(a0: u8);
    fn Task_PlayerController_RestoreBgmAfterCry(a0: u8);
    fn TryHandleLaunchBattleTableAnimation(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn TrySetBehindSubstituteSpriteBit(a0: u8, a1: u16);
    fn TryShinyAnimation(a0: u8, a1: *mut Pokemon);
    fn UpdateHealthboxAttribute(a0: u8, a1: *mut Pokemon, a2: u8);
    fn UpdateHpTextInHealthbox(a0: u8, a1: i16, a2: u8);
}

pub(crate) unsafe extern "C" fn SpriteCB_Null7() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetControllerToWally() {
    gBattlerControllerFuncs[gActiveBattler] = Some(WallyBufferRunCommand);
    (*gBattleStruct).wallyBattleState = 0;
    (*gBattleStruct).wallyMovesState = 0;
    (*gBattleStruct).wallyWaitFrames = 0;
    (*gBattleStruct).wallyMoveFrames = 0;
}
pub(crate) unsafe extern "C" fn WallyBufferRunCommand() {
    if gBattleControllerExecFlags & gBitTable[gActiveBattler] != 0 {
        if gBattleBufferA[gActiveBattler][0] < 57 {
            sWallyBufferCommands[gBattleBufferA[gActiveBattler][0]].unwrap_unchecked()();
        } else {
            WallyBufferExecCompleted();
        }
    }
}
pub(crate) unsafe extern "C" fn WallyHandleActions() {
    'l1: {
        let sw1: u8 = (*gBattleStruct).wallyBattleState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*gBattleStruct).wallyWaitFrames = B_WAIT_TIME_LONG;
            (*gBattleStruct).wallyBattleState += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if ({
                (*gBattleStruct).wallyWaitFrames -= 1;
                (*gBattleStruct).wallyWaitFrames
            }) == 0
            {
                PlaySE(SE_SELECT);
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, 0, 0);
                WallyBufferExecCompleted();
                (*gBattleStruct).wallyBattleState += 1;
                (*gBattleStruct).wallyMovesState = 0;
                (*gBattleStruct).wallyWaitFrames = B_WAIT_TIME_LONG;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if ({
                (*gBattleStruct).wallyWaitFrames -= 1;
                (*gBattleStruct).wallyWaitFrames
            }) == 0
            {
                PlaySE(SE_SELECT);
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, 0, 0);
                WallyBufferExecCompleted();
                (*gBattleStruct).wallyBattleState += 1;
                (*gBattleStruct).wallyMovesState = 0;
                (*gBattleStruct).wallyWaitFrames = B_WAIT_TIME_LONG;
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if ({
                (*gBattleStruct).wallyWaitFrames -= 1;
                (*gBattleStruct).wallyWaitFrames
            }) == 0
            {
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_WALLY_THROW, 0);
                WallyBufferExecCompleted();
                (*gBattleStruct).wallyBattleState += 1;
                (*gBattleStruct).wallyMovesState = 0;
                (*gBattleStruct).wallyWaitFrames = B_WAIT_TIME_LONG;
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            if ({
                (*gBattleStruct).wallyWaitFrames -= 1;
                (*gBattleStruct).wallyWaitFrames
            }) == 0
            {
                PlaySE(SE_SELECT);
                ActionSelectionDestroyCursorAt(0);
                ActionSelectionCreateCursorAt(1, 0);
                (*gBattleStruct).wallyWaitFrames = B_WAIT_TIME_LONG;
                (*gBattleStruct).wallyBattleState += 1;
            }
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            if ({
                (*gBattleStruct).wallyWaitFrames -= 1;
                (*gBattleStruct).wallyWaitFrames
            }) == 0
            {
                PlaySE(SE_SELECT);
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_USE_ITEM, 0);
                WallyBufferExecCompleted();
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn CompleteOnBattlerSpriteCallbackDummy() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnInactiveTextPrinter() {
    if IsTextPrinterActive(B_WIN_MSG) == 0 {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnFinishedAnimation() {
    if gDoingBattleAnim == 0 {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn OpenBagAfterPaletteFade() {
    if gPaletteFade.active() == 0 {
        gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnChosenItem);
        ReshowBattleScreenDummy();
        FreeAllWindowBuffers();
        DoWallyTutorialBagMenu();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnChosenItem() {
    if gMain.callback2 == Some(BattleMainCB2 as unsafe extern "C" fn())
        && gPaletteFade.active() == 0
    {
        BtlController_EmitOneReturnValue(B_COMM_TO_ENGINE, gSpecialVar_ItemId);
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn Intro_TryShinyAnimShowHealthbox() {
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
        && gSprites[gBattleControllerData[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
        && gSprites[gBattlerSpriteIds[gActiveBattler]].callback
            == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        if IsDoubleBattle() != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
            DestroySprite(&raw mut gSprites[gBattleControllerData[gActiveBattler as i32 ^ 2]]);
            UpdateHealthboxAttribute(
                gHealthboxSpriteIds[gActiveBattler as i32 ^ 2],
                &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler as i32 ^ 2]],
                HEALTHBOX_ALL,
            );
            StartHealthboxSlideIn(gActiveBattler ^ 2);
            SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler as i32 ^ 2]);
        }
        DestroySprite(&raw mut gSprites[gBattleControllerData[gActiveBattler]]);
        UpdateHealthboxAttribute(
            gHealthboxSpriteIds[gActiveBattler],
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            HEALTHBOX_ALL,
        );
        StartHealthboxSlideIn(gActiveBattler);
        SetHealthboxSpriteVisible(gHealthboxSpriteIds[gActiveBattler]);
        (*(*gBattleSpritesDataPtr).animationData).set_introAnimActive(FALSE);
        gBattlerControllerFuncs[gActiveBattler] = Some(Intro_WaitForShinyAnimAndHealthbox);
    }
}
pub(crate) unsafe extern "C" fn Intro_WaitForShinyAnimAndHealthbox() {
    let mut healthboxAnimDone: u32 = FALSE as u32;
    if gSprites[gHealthboxSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        healthboxAnimDone = TRUE as u32;
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
        CreateTask(Some(Task_PlayerController_RestoreBgmAfterCry), 10);
        HandleLowHpMusicChange(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
            gActiveBattler,
        );
        WallyBufferExecCompleted();
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
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn DoHitAnimBlinkSpriteEffect() {
    let mut spriteId: u8 = gBattlerSpriteIds[gActiveBattler];
    if gSprites[spriteId].data[1] == 32 {
        gSprites[spriteId].data[1] = 0;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gDoingBattleAnim = FALSE;
        WallyBufferExecCompleted();
    } else {
        if gSprites[spriteId].data[1] % 4 == 0 {
            gSprites[spriteId].set_invisible(gSprites[spriteId].invisible() ^ 1);
        }
        gSprites[spriteId].data[1] += 1;
    }
}
pub(crate) unsafe extern "C" fn DoSwitchOutAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).specialAnimActive() == 0 {
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnBankSpriteCallbackDummy2() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn CompleteOnFinishedBattleAnimation() {
    if (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animFromTableActive() == 0 {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyBufferExecCompleted() {
    gBattlerControllerFuncs[gActiveBattler] = Some(WallyBufferRunCommand);
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
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleGetMonData() {
    let mut monData: CArray<u8, 256> = zeroed();
    let mut size: u32 = 0;
    let mut monToCheck: u8 = 0;
    let mut i: i32 = 0;
    if gBattleBufferA[gActiveBattler][2] == 0 {
        size += CopyWallyMonData(
            gBattlerPartyIndexes[gActiveBattler] as u8,
            monData.as_mut_ptr(),
        );
    } else {
        monToCheck = gBattleBufferA[gActiveBattler][2];
        i = 0;
        while i < PARTY_SIZE {
            if monToCheck as i32 & 1 != 0 {
                size += CopyWallyMonData(i as u8, monData.as_mut_ptr().at(size));
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
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn CopyWallyMonData(monId: u8, mut dst: *mut u8) -> u32 {
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
pub(crate) unsafe extern "C" fn WallyHandleGetRawMonData() {
    PlayerHandleGetRawMonData();
}
pub(crate) unsafe extern "C" fn WallyHandleSetMonData() {
    let mut monToCheck: u8 = 0;
    let mut i: u8 = 0;
    if gBattleBufferA[gActiveBattler][2] == 0 {
        SetWallyMonData(gBattlerPartyIndexes[gActiveBattler] as u8);
    } else {
        monToCheck = gBattleBufferA[gActiveBattler][2];
        i = 0;
        while i < PARTY_SIZE as u8 {
            if monToCheck as i32 & 1 != 0 {
                SetWallyMonData(i);
            }
            monToCheck >>= 1;
            i += 1;
        }
    }
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn SetWallyMonData(monId: u8) {
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
pub(crate) unsafe extern "C" fn WallyHandleSetRawMonData() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleLoadMonSprite() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleSwitchInAnim() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleReturnMonToBall() {
    if gBattleBufferA[gActiveBattler][1] == 0 {
        InitAndLaunchSpecialAnimation(
            gActiveBattler,
            gActiveBattler,
            gActiveBattler,
            B_ANIM_SWITCH_OUT_PLAYER_MON,
        );
        gBattlerControllerFuncs[gActiveBattler] = Some(DoSwitchOutAnimation);
    } else {
        FreeSpriteOamMatrix(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[gActiveBattler]]);
        SetHealthboxSpriteInvisible(gHealthboxSpriteIds[gActiveBattler]);
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleDrawTrainerPic() {
    DecompressTrainerBackPic(TRAINER_BACK_PIC_WALLY, gActiveBattler);
    SetMultiuseSpriteTemplateToTrainerBack(
        TRAINER_BACK_PIC_WALLY,
        GetBattlerPosition(gActiveBattler),
    );
    gBattlerSpriteIds[gActiveBattler] = CreateSprite(
        &raw mut gMultiuseSpriteTemplate,
        80,
        80 + 4 * (8 - gTrainerBackPicCoords[6].size as i16),
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
pub(crate) unsafe extern "C" fn WallyHandleTrainerSlide() {
    DecompressTrainerBackPic(TRAINER_BACK_PIC_WALLY, gActiveBattler);
    SetMultiuseSpriteTemplateToTrainerBack(
        TRAINER_BACK_PIC_WALLY,
        GetBattlerPosition(gActiveBattler),
    );
    gBattlerSpriteIds[gActiveBattler] = CreateSprite(
        &raw mut gMultiuseSpriteTemplate,
        80,
        80 + 4 * (8 - gTrainerBackPicCoords[6].size as i16),
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
pub(crate) unsafe extern "C" fn WallyHandleTrainerSlideBack() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleFaintAnimation() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandlePaletteFade() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleSuccessBallThrowAnim() {
    (*(*gBattleSpritesDataPtr).animationData).ballThrowCaseId = BALL_3_SHAKES_SUCCESS;
    gDoingBattleAnim = TRUE;
    InitAndLaunchSpecialAnimation(
        gActiveBattler,
        gActiveBattler,
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        B_ANIM_BALL_THROW_WITH_TRAINER,
    );
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedAnimation);
}
pub(crate) unsafe extern "C" fn WallyHandleBallThrowAnim() {
    let mut ballThrowCaseId: u8 = gBattleBufferA[gActiveBattler][1];
    (*(*gBattleSpritesDataPtr).animationData).ballThrowCaseId = ballThrowCaseId;
    gDoingBattleAnim = TRUE;
    InitAndLaunchSpecialAnimation(
        gActiveBattler,
        gActiveBattler,
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        B_ANIM_BALL_THROW_WITH_TRAINER,
    );
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedAnimation);
}
pub(crate) unsafe extern "C" fn WallyHandlePause() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleMoveAnimation() {
    let mut r#move: u16 =
        gBattleBufferA[gActiveBattler][1] as u16 | (gBattleBufferA[gActiveBattler][2] as u16) << 8;
    gAnimMoveTurn = gBattleBufferA[gActiveBattler][3];
    gAnimMovePower =
        gBattleBufferA[gActiveBattler][4] as u16 | (gBattleBufferA[gActiveBattler][5] as u16) << 8;
    gAnimMoveDmg = gBattleBufferA[gActiveBattler][6] as i32
        | (gBattleBufferA[gActiveBattler][7] as i32) << 8
        | (gBattleBufferA[gActiveBattler][8] as i32) << 16
        | (gBattleBufferA[gActiveBattler][9] as i32) << 24;
    gAnimFriendship = gBattleBufferA[gActiveBattler][10];
    gWeatherMoveAnim = gBattleBufferA[gActiveBattler][12] as u16
        | (gBattleBufferA[gActiveBattler][13] as u16) << 8;
    gAnimDisableStructPtr = &raw mut gBattleBufferA[gActiveBattler][16] as *mut DisableStruct;
    gTransformedPersonalities[gActiveBattler] = (*gAnimDisableStructPtr).transformedMonPersonality;
    if IsMoveWithoutAnimation(r#move, gAnimMoveTurn) != 0 {
        WallyBufferExecCompleted();
    } else {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).animationState = 0;
        gBattlerControllerFuncs[gActiveBattler] = Some(WallyDoMoveAnimation);
    }
}
pub(crate) unsafe extern "C" fn WallyDoMoveAnimation() {
    let mut r#move: u16 =
        gBattleBufferA[gActiveBattler][1] as u16 | (gBattleBufferA[gActiveBattler][2] as u16) << 8;
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
                {
                    InitAndLaunchSpecialAnimation(
                        gActiveBattler,
                        gActiveBattler,
                        gActiveBattler,
                        B_ANIM_MON_TO_SUBSTITUTE,
                    );
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
                WallyBufferExecCompleted();
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn WallyHandlePrintString() {
    let mut stringId: *mut u16 = null_mut();
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    stringId = &raw mut gBattleBufferA[gActiveBattler][2] as *mut u16;
    BufferStringBattle(*stringId);
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
    gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnInactiveTextPrinter);
}
pub(crate) unsafe extern "C" fn WallyHandlePrintSelectionString() {
    if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
        WallyHandlePrintString();
    } else {
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn HandleChooseActionAfterDma3() {
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        gBattle_BG0_X = 0;
        gBattle_BG0_Y = DISPLAY_HEIGHT;
        gBattlerControllerFuncs[gActiveBattler] = Some(WallyHandleActions);
    }
}
pub(crate) unsafe extern "C" fn WallyHandleChooseAction() {
    let mut i: i32 = 0;
    gBattlerControllerFuncs[gActiveBattler] = Some(HandleChooseActionAfterDma3);
    BattlePutTextOnWindow(gText_BattleMenu.as_ptr().cast_mut(), B_WIN_ACTION_MENU);
    i = 0;
    while i < 4 {
        ActionSelectionDestroyCursorAt(i as u8);
        i += 1;
    }
    ActionSelectionCreateCursorAt(gActionSelectionCursor[gActiveBattler], 0);
    BattleStringExpandPlaceholdersToDisplayedString(gText_WhatWillWallyDo.as_ptr().cast_mut());
    BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_ACTION_PROMPT);
}
pub(crate) unsafe extern "C" fn WallyHandleYesNoBox() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleChooseMove() {
    match (*gBattleStruct).wallyMovesState {
        0 => {
            InitMoveSelectionsVarsAndStrings();
            (*gBattleStruct).wallyMovesState += 1;
            (*gBattleStruct).wallyMoveFrames = 80;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                gBattle_BG0_X = 0;
                gBattle_BG0_Y = 320;
                (*gBattleStruct).wallyMovesState += 1;
            }
        }
        2 => {
            if ({
                (*gBattleStruct).wallyMoveFrames -= 1;
                (*gBattleStruct).wallyMoveFrames
            }) == 0
            {
                PlaySE(SE_SELECT);
                BtlController_EmitTwoReturnValues(B_COMM_TO_ENGINE, B_ACTION_EXEC_SCRIPT, 0x100);
                WallyBufferExecCompleted();
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn WallyHandleChooseItem() {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
    gBattlerControllerFuncs[gActiveBattler] = Some(OpenBagAfterPaletteFade);
    gBattlerInMenuId = gActiveBattler;
}
pub(crate) unsafe extern "C" fn WallyHandleChoosePokemon() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleCmd23() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleHealthBarUpdate() {
    let mut hpVal: i16 = 0;
    LoadBattleBarGfx(0);
    hpVal =
        gBattleBufferA[gActiveBattler][2] as i16 | (gBattleBufferA[gActiveBattler][3] as i16) << 8;
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
pub(crate) unsafe extern "C" fn WallyHandleExpUpdate() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleStatusIconUpdate() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleStatusAnimation() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleStatusXor() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleDataTransfer() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleDMA3Transfer() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandlePlayBGM() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleCmd32() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleTwoReturnValues() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleChosenMonReturnValue() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleOneReturnValue() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleOneReturnValue_Duplicate() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleClearUnkVar() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleSetUnkVar() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleClearUnkFlag() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleToggleUnkFlag() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleHitAnimation() {
    if gSprites[gBattlerSpriteIds[gActiveBattler]].invisible() == TRUE as u16 {
        WallyBufferExecCompleted();
    } else {
        gDoingBattleAnim = TRUE;
        gSprites[gBattlerSpriteIds[gActiveBattler]].data[1] = 0;
        DoHitAnimHealthboxEffect(gActiveBattler);
        gBattlerControllerFuncs[gActiveBattler] = Some(DoHitAnimBlinkSpriteEffect);
    }
}
pub(crate) unsafe extern "C" fn WallyHandleCantSwitch() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandlePlaySE() {
    PlaySE(
        gBattleBufferA[gActiveBattler][1] as u16 | (gBattleBufferA[gActiveBattler][2] as u16) << 8,
    );
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandlePlayFanfareOrBGM() {
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
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleFaintingCry() {
    let mut species: u16 = GetMonData2(
        &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
        MON_DATA_SPECIES,
    ) as u16;
    PlayCry_Normal(species, 25);
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleIntroSlide() {
    HandleIntroSlide(gBattleBufferA[gActiveBattler][1]);
    gIntroSlideFlags |= 1;
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleIntroTrainerBallThrow() {
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
        gTrainerBackPicPaletteTable[6].data,
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
pub(crate) unsafe extern "C" fn StartSendOutAnim(battler: u8) {
    let mut species: u16 = 0;
    (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies = 0;
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
pub(crate) unsafe extern "C" fn Task_StartSendOutAnim(taskId: u8) {
    if gTasks[taskId].data[1] < 31 {
        gTasks[taskId].data[1] += 1;
    } else {
        let mut savedActiveBank: u8 = gActiveBattler;
        gActiveBattler = gTasks[taskId].data[0] as u8;
        gBattleBufferA[gActiveBattler][1] = gBattlerPartyIndexes[gActiveBattler] as u8;
        StartSendOutAnim(gActiveBattler);
        gBattlerControllerFuncs[gActiveBattler] = Some(Intro_TryShinyAnimShowHealthbox);
        gActiveBattler = savedActiveBank;
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn WallyHandleDrawPartyStatusSummary() {
    if gBattleBufferA[gActiveBattler][1] != 0 && GetBattlerSide(gActiveBattler) == 0 {
        WallyBufferExecCompleted();
    } else {
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler))
            .set_partyStatusSummaryShown(1);
        gBattlerStatusSummaryTaskId[gActiveBattler] = CreatePartyStatusSummarySprites(
            gActiveBattler,
            &raw mut gBattleBufferA[gActiveBattler][4] as *mut HpAndStatus,
            gBattleBufferA[gActiveBattler][1],
            gBattleBufferA[gActiveBattler][2],
        );
        WallyBufferExecCompleted();
    }
}
pub(crate) unsafe extern "C" fn WallyHandleHidePartyStatusSummary() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleEndBounceEffect() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleSpriteInvisibility() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleBattleAnimation() {
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
        WallyBufferExecCompleted();
    } else {
        gBattlerControllerFuncs[gActiveBattler] = Some(CompleteOnFinishedBattleAnimation);
    }
}
pub(crate) unsafe extern "C" fn WallyHandleLinkStandbyMsg() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleResetActionMoveSelection() {
    WallyBufferExecCompleted();
}
pub(crate) unsafe extern "C" fn WallyHandleEndLinkBattle() {
    gBattleOutcome = gBattleBufferA[gActiveBattler][1];
    FadeOutMapMusic(5);
    BeginFastPaletteFade(3);
    WallyBufferExecCompleted();
    if gBattleTypeFlags & BATTLE_TYPE_IS_MASTER == 0 && gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        gBattlerControllerFuncs[gActiveBattler] = Some(SetBattleEndCallbacks);
    }
}
pub(crate) unsafe extern "C" fn WallyCmdEnd() {}
