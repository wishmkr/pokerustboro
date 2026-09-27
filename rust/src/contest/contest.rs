//! Translated from `src/contest.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sSliderHeartYPositions sNextTurnSpriteYPositions sSpriteSheet_SliderHeart sOam_SliderHeart sAffineAnim_SliderHeart_Normal sAffineAnim_SliderHeart_SpinDisappear sAffineAnim_SliderHeart_SpinAppear sAffineAnims_SliderHeart sSpriteTemplate_SliderHeart sSpriteSheet_NextTurn sSpritePalette_NextTurn sOam_NextTurn sSpriteTemplates_NextTurn sSubsprites_NextTurn sSubspriteTable_NextTurn sSpriteSheet_Faces sOam_Faces sSpriteTemplate_Faces sSpriteSheet_ApplauseMeter sSpritePalette_ApplauseMeter sOam_ApplauseMeter sSpriteTemplate_ApplauseMeter sOam_Judge sSpriteTemplate_Judge sSpriteSheet_Judge sSpriteSheet_JudgeSymbols sSpritePalette_JudgeSymbols sSpriteTemplate_JudgeSpeechBubble sText_Pal gContestEffectDescriptionPointers sUnusedComboMoveNameTexts gContestMoveTypeTextPointers sUnusedAppealResultTexts sRoundResultTexts sAppealResultTexts sContestConditions sInvalidContestMoveNames sContestBgTemplates sContestWindowTemplates gDefaultContestWinners gContestOpponents gPostgameContestOpponentFilter sSpriteSheets_ContestantsTurnBlinkEffect sSpritePalettes_ContestantsTurnBlinkEffect sOam_ContestantsTurnBlinkEffect sAffineAnim_ContestantsTurnBlinkEffect_0 sAffineAnim_ContestantsTurnBlinkEffect_1 sAffineAnims_ContestantsTurnBlinkEffect sSpriteTemplates_ContestantsTurnBlinkEffect sContestExcitementTable
#[allow(unused_imports)]
use crate::data::contest::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestMons: crate::ffi::Align4<[u8; 256]> = crate::ffi::Align4([0; 256]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestMonRound1Points: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestMonTotalPoints: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestMonAppealPointTotals: crate::ffi::Align4<[u8; 8]> =
    crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestMonRound2Points: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestFinalStandings: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestMonPartyIndex: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestPlayerMonIndex: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestantTurnOrder: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkContestFlags: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestLinkLeaderIndex: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSpecialVar_ContestCategory: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSpecialVar_ContestRank: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gNumLinkContestPlayers: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gHighestRibbonRank: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestResources: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sContestBgCopyFlags: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurContestWinner: crate::ffi::Align4<[u8; 32]> = crate::ffi::Align4([0; 32]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurContestWinnerIsForArtist: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurContestWinnerSaveIdx: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gContestRngValue: u32 = 0u32;

unsafe extern "C" {
    static mut gAffineAnims_BattleSpriteContest: u8;
    static mut gAffineAnims_BattleSpriteOpponentSide: u8;
    static mut gAnimFriendship: u8;
    static mut gAnimMoveTurn: u8;
    static mut gAnimScriptActive: u8;
    static mut gAnimScriptCallback: u8;
    static mut gBattleAnimBgTileBuffer: u8;
    static mut gBattleAnimBgTilemapBuffer: u8;
    static mut gBattleMonForms: u8;
    static mut gBattleMoves: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattle_BG0_X: u8;
    static mut gBattle_BG0_Y: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_BG2_X: u8;
    static mut gBattle_BG2_Y: u8;
    static mut gBattle_BG3_X: u8;
    static mut gBattle_BG3_Y: u8;
    static mut gBattle_WIN0H: u8;
    static mut gBattle_WIN0V: u8;
    static mut gBattle_WIN1H: u8;
    static mut gBattle_WIN1V: u8;
    static mut gBattlerAttacker: u8;
    static mut gBattlerPositions: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gBattlerTarget: u8;
    static mut gContest2Pal: u8;
    static mut gContestApplauseMeterGfx: u8;
    static mut gContestAudienceGfx: u8;
    static mut gContestAudienceTilemap: u8;
    static mut gContestCurtainTilemap: u8;
    static mut gContestEffectFuncs: u8;
    static mut gContestEffects: u8;
    static mut gContestInterfaceAudiencePalette: u8;
    static mut gContestInterfaceGfx: u8;
    static mut gContestInterfaceTilemap: u8;
    static mut gContestMoves: u8;
    static mut gContestNextTurnNumbersGfx: u8;
    static mut gContestNextTurnRandomGfx: u8;
    static mut gDisplayedStringBattle: u8;
    static mut gEnableContestDebugging: u8;
    static mut gFieldCallback: u8;
    static mut gHeap: u8;
    static mut gMPlayInfo_SE1: u8;
    static mut gMain: u8;
    static mut gMonBackPicTable: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gMoveNames: u8;
    static mut gMultiuseSpriteTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gRngValue: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpeciesInfo: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gTextFlags: u8;
    static mut gText_AllOutOfAppealTime: u8;
    static mut gText_AppealComboWentOverExcellently: u8;
    static mut gText_AppealComboWentOverVeryWell: u8;
    static mut gText_AppealComboWentOverWell: u8;
    static mut gText_AppealNumButItCantParticipate: u8;
    static mut gText_AppealNumWhichMoveWillBePlayed: u8;
    static mut gText_BDot: u8;
    static mut gText_CDot: u8;
    static mut gText_ColorBlue: u8;
    static mut gText_ColorLightShadowDarkGray: u8;
    static mut gText_ColorTransparent: u8;
    static mut gText_Contest_Anxiety: u8;
    static mut gText_Contest_Fear: u8;
    static mut gText_Contest_Hesitancy: u8;
    static mut gText_Contest_Laziness: u8;
    static mut gText_Contest_Shyness: u8;
    static mut gText_CrowdContinuesToWatchMon: u8;
    static mut gText_JudgeLookedAtMonExpectantly: u8;
    static mut gText_LinkStandby4: u8;
    static mut gText_MonAppealedWithMove: u8;
    static mut gText_MonCantAppealNextTurn: u8;
    static mut gText_MonWasTooNervousToMove: u8;
    static mut gText_MonWasWatchingOthers: u8;
    static mut gText_MonsMoveIsIgnored: u8;
    static mut gText_MonsXDidntGoOverWell: u8;
    static mut gText_MonsXGotTheCrowdGoing: u8;
    static mut gText_MonsXWentOverGreat: u8;
    static mut gText_OneDash: u8;
    static mut gText_RepeatedAppeal: u8;
    static mut gText_Slash: u8;
    fn AddTextPrinter(a0: *mut u8, a1: u8, a2: Option<unsafe extern "C" fn(*mut u8, u16)>) -> u16;
    fn AllocOamMatrix() -> u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AllocateMonSpritesGfx();
    fn AnimateSprite(a0: *mut u8);
    fn AnimateSprites();
    fn AreMovesContestCombo(a0: u16, a1: u16) -> u8;
    fn BeginFastPaletteFade(a0: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn BravoTrainerPokemonProfile_BeforeInterview1(a0: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToField();
    fn ClearBattleAnimationVars();
    fn ClearBattleMonForms();
    fn ContestAI_GetActionToUse() -> u8;
    fn ContestAI_ResetAI(a0: u8);
    fn ContestLiveUpdates_Init(a0: u8);
    fn ContestLiveUpdates_SetLoserData(a0: u8, a1: u8);
    fn ContestLiveUpdates_SetRound2Placing(a0: u8);
    fn ContestLiveUpdates_SetWinnerAppealFlag(a0: u8);
    fn ContestLiveUpdates_SetWinnerMoveUsed(a0: u16);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopySpriteTiles(a0: u8, a1: u8, a2: *mut u8, a3: *mut u16, a4: *mut u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut u8)>) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndFreeResources(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DoMoveAnim(a0: u16);
    fn FillPalette(a0: u16, a1: u16, a2: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeMonSpritesGfx();
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteFinal_Y(a0: u8, a1: u16, a2: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetContestRand() -> u16;
    fn GetGpuReg(a0: u8) -> u16;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetMultiplayerId() -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn HandleLoadSpecialPokePic_2(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsLinkTaskFinished() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LZDecompressVram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RequestDma3Copy(a0: *mut u8, a1: *mut u8, a2: u16, a3: u8) -> i16;
    fn RequestDma3Fill(a0: i32, a1: *mut u8, a2: u16, a3: u8) -> i16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn RunTextPrinters();
    fn ScanlineEffect_Clear();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScriptContext_Enable();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetSubspriteTables(a0: *mut u8, a1: *mut u8);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn StripExtCtrlCodes(a0: *mut u8);
    fn Task_LinkContest_CommunicateAppealsState(a0: u8);
    fn Task_LinkContest_CommunicateFinalStandings(a0: u8);
    fn Task_LinkContest_CommunicateMonIdxs(a0: u8);
    fn Task_LinkContest_CommunicateMoveSelections(a0: u8);
    fn TransferPlttBuffer();
    fn UnlockPlayerFieldControls();
    fn UpdatePaletteFade() -> u8;
    fn WriteSequenceToBgTilemapBuffer(
        a0: u8,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: i16,
    );
    fn m4aMPlayImmInit(a0: *mut u8);
    fn m4aMPlayPitchControl(a0: *mut u8, a1: u16, a2: i16);
}

pub(crate) unsafe extern "C" fn TaskDummy1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetLinkContestBoolean() {
    unsafe {
        ((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn SetupContestGpuRegs() {
    unsafe {
        SetGpuReg(0u8, 64u16);
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
        SetGpuReg(72u8, 16191u16);
        SetGpuReg(74u8, 16191u16);
        SetGpuRegBits(0u8, 32512u16);
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG3_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN1H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN1V).cast::<u16>()).write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadContestBgAfterMoveAnim() {
    unsafe {
        let mut i: i32 = 0i32;
        LZDecompressVram(
            ((&raw mut gContestInterfaceGfx).cast::<u32>()).cast::<u32>(),
            ((100663296i32) as usize as *mut u8),
        );
        LZDecompressVram(
            ((&raw mut gContestAudienceGfx).cast::<u32>()).cast::<u32>(),
            ((100671488i32) as usize as *mut u8),
        );
        CopyToBgTilemapBuffer(
            3u8,
            (((&raw mut gContestAudienceTilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u16,
            0u16,
        );
        CopyBgTilemapBufferToVram(3u8);
        LoadCompressedPalette(
            ((&raw mut gContestInterfaceAudiencePalette).cast::<u32>()).cast::<u32>(),
            0u16,
            512u16,
        );
        LoadContestPalettes();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut contestantWindowId: u32 = (((5i32).wrapping_add(i)) as u32);
                    LoadPalette(
                        ((((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106500))
                            .cast::<u8>())
                        .wrapping_offset(((contestantWindowId) as i32) as isize * 32))
                        .cast::<u16>())
                        .cast::<u8>(),
                        (((0i32).wrapping_add(
                            ((5i32).wrapping_add(
                                ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32),
                            ))
                            .wrapping_mul(16i32),
                        )) as u16),
                        32u16,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitContestInfoBgs() {
    unsafe {
        let mut i: i32 = 0i32;
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sContestBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        SetBgAttribute(3u8, 6u8, 1u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    SetBgTilemapBuffer(
                        ((i) as u8),
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(36))
                        .cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitContestWindows() {
    unsafe {
        InitWindows(((&raw const sContestWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
        if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32) != 0
        {
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                0,
                1,
                (0u8) as i32,
            );
        } else {
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                0,
                1,
                (1u8) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn LoadContestPalettes() {
    unsafe {
        let mut i: i32 = 0i32;
        LoadPalette(
            (((&raw const sText_Pal).cast::<u8>().cast_mut().cast::<u16>()).cast::<u16>())
                .cast::<u8>(),
            240u16,
            32u16,
        );
        SetBackdropFromColor(0u16);
        {
            i = 10i32;
            'l1: loop {
                if !(i < 14i32) {
                    break 'l1;
                }
                'l2: {
                    LoadPalette(
                        ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(241))
                        .cast::<u8>(),
                        (((240i32).wrapping_add(i)) as u16),
                        2u16,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        FillPalette(32319u16, 243u16, 2u16);
    }
}
pub(crate) unsafe extern "C" fn InitContestResources() {
    unsafe {
        let mut i: i32 = 0i32;
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read()
            .cast::<crate::c::Rec4<92>>()
            .write_unaligned({
                let mut __lit1 = crate::ffi::Align4([0u8; 92]);
                (&raw mut __lit1).cast::<u8>().wrapping_add(0).write(0u8);
                (&raw mut __lit1)
                    .cast::<crate::c::Rec4<92>>()
                    .read_unaligned()
            });
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(2))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28)
                    .cast::<crate::c::Rec4<28>>()
                    .write_unaligned({
                        let mut __lit2 = crate::ffi::Align4([0u8; 28]);
                        (&raw mut __lit2)
                            .cast::<u8>()
                            .wrapping_add(0)
                            .cast::<i16>()
                            .write(0i16);
                        (&raw mut __lit2)
                            .cast::<crate::c::Rec4<28>>()
                            .read_unaligned()
                    });
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 4i32) {
                    break 'l5;
                }
                'l6: {
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(11),
                        0,
                        2,
                        (0u8) as i32,
                    );
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(19))
                    .write(255u8);
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(20))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read()
        .cast::<crate::c::Rec4<20>>()
        .write_unaligned({
            let mut __lit3 = crate::ffi::Align4([0u8; 20]);
            (&raw mut __lit3)
                .cast::<crate::c::Rec4<20>>()
                .read_unaligned()
        });
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read()
        .cast::<crate::c::Rec4<68>>()
        .write_unaligned({
            let mut __lit4 = crate::ffi::Align4([0u8; 68]);
            (&raw mut __lit4).cast::<u8>().wrapping_add(0).write(0u8);
            (&raw mut __lit4)
                .cast::<crate::c::Rec4<68>>()
                .read_unaligned()
        });
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<*mut u8>())
        .read()
        .cast::<crate::c::Rec4<4>>()
        .write_unaligned({
            let mut __lit5 = crate::ffi::Align4([0u8; 4]);
            (&raw mut __lit5)
                .cast::<u8>()
                .wrapping_add(0)
                .cast::<i8>()
                .write(0i8);
            (&raw mut __lit5)
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned()
        });
        crate::c::memset(
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read(),
            0i32,
            16u32,
        );
        if !((((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32)
            != 0)
        {
            SortContestants(0u8);
        }
        {
            i = 0i32;
            'l7: loop {
                if !(i < 4i32) {
                    break 'l7;
                }
                'l8: {
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(25))
                    .write(255u8);
                    ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(20))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ApplyNextTurnOrder();
        crate::c::memset(
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .read(),
            0i32,
            64u32,
        );
    }
}
pub(crate) unsafe extern "C" fn AllocContestResources() {
    unsafe {
        ((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(64u32));
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
            .write(AllocZeroed(92u32));
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write(AllocZeroed(112u32));
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .write(AllocZeroed(20u32));
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .write(AllocZeroed(68u32));
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<*mut u8>())
        .write(AllocZeroed(16u32));
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .write(AllocZeroed(16u32));
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<*mut u8>())
        .write(AllocZeroed(20u32));
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .write(AllocZeroed(64u32));
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<*mut u8>())
        .write(AllocZeroed(12u32));
        (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
            .cast::<*mut u8>())
        .write(AllocZeroed(4096u32));
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<*mut u8>())
        .wrapping_offset(1))
        .write(AllocZeroed(4096u32));
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<*mut u8>())
        .wrapping_offset(2))
        .write(AllocZeroed(4096u32));
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<*mut u8>())
        .wrapping_offset(3))
        .write(AllocZeroed(4096u32));
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52)
            .cast::<*mut u8>())
        .write(AllocZeroed(2048u32));
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(56)
            .cast::<*mut u8>())
        .write(AllocZeroed(2048u32));
        ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(60)
            .cast::<*mut u8>())
        .write(AllocZeroed(8192u32));
        ((&raw mut gBattleAnimBgTileBuffer).cast::<*mut u8>()).write(
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(60)
                .cast::<*mut u8>())
            .read(),
        );
        ((&raw mut gBattleAnimBgTilemapBuffer).cast::<*mut u8>()).write(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<*mut u8>())
            .wrapping_offset(1))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn FreeContestResources() {
    unsafe {
        {
            Free(
                ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<*mut u8>())
                .read(),
            );
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<*mut u8>())
                .wrapping_offset(1))
                .read(),
            );
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<*mut u8>())
            .wrapping_offset(1))
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<*mut u8>())
                .wrapping_offset(2))
                .read(),
            );
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<*mut u8>())
            .wrapping_offset(2))
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<*mut u8>())
                .wrapping_offset(3))
                .read(),
            );
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<*mut u8>())
            .wrapping_offset(3))
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(56)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(56)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(60)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                .write(core::ptr::null_mut());
        }
        ((&raw mut gBattleAnimBgTileBuffer).cast::<*mut u8>()).write(core::ptr::null_mut());
        ((&raw mut gBattleAnimBgTilemapBuffer).cast::<*mut u8>()).write(core::ptr::null_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_StartContest() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut sContestBgCopyFlags).cast::<u8>().cast::<u8>()).write(0u8);
                AllocContestResources();
                AllocateMonSpritesGfx();
                {
                    Free(
                        ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read(),
                    );
                    ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                }
                ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                    .write(AllocZeroed(16384u32));
                SetVBlankCallback(None);
                InitContestInfoBgs();
                InitContestWindows();
                SetupContestGpuRegs();
                ScanlineEffect_Clear();
                ResetPaletteFade();
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (1u16) as i32,
                );
                ResetSpriteData();
                ResetTasks();
                FreeAllSpritePalettes();
                ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(4u8);
                (((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).write(0u8);
                ClearBattleMonForms();
                InitContestResources();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (SetupContestGraphics(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(89),
                )) != 0
                {
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(89))
                    .write(0u8);
                    let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetBgForCurtainDrop();
                ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                BeginFastPaletteFade(2u8);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                SetVBlankCallback(Some(VBlankCB_Contest));
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8))
                .write(CreateTask(Some(Task_StartContestWaitFade), 10u8));
                SetMainCallback2(Some(CB2_ContestMain));
                if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32)
                    & 2i32)
                    != 0
                {
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(8u8, 8u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartContestWaitFade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_TryStartLinkContest));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TryStartLinkContest(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32) != 0
        {
            if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 2i32)
                != 0
            {
                'l1: {
                    let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32);
                    let mut __fall = false;
                    if __sw1 == 0i32 {
                        __fall = true;
                        ContestPrintLinkStandby();
                        let __p2 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                    if __fall || __sw1 == 1i32 {
                        __fall = true;
                        if (IsLinkTaskFinished()) != 0 {
                            SetLinkStandbyCallback();
                            let __p3 = ((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>();
                            (__p3).write(((__p3).read()).wrapping_add(1));
                        }
                        return;
                    }
                    if __sw1 == 2i32 {
                        __fall = true;
                        if ((IsLinkTaskFinished()) as i32) != 1i32 {
                            return;
                        }
                        let __p4 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p4).write(((__p4).read()).wrapping_add(1));
                        break 'l1;
                    }
                }
            }
            if !((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0)
            {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                if !((((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32)
                    & 2i32)
                    != 0)
                {
                    ContestPrintLinkStandby();
                }
                CreateTask(Some(Task_CommunicateMonIdxs), 0u8);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(TaskDummy1));
            }
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_WaitToRaiseCurtainAtStart));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CommunicateMonIdxs(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateMonIdxs),
            Some(Task_EndCommunicateMonIdxs),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_EndCommunicateMonIdxs(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(1i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ReadyStartLinkContest));
    }
}
pub(crate) unsafe extern "C" fn Task_ReadyStartLinkContest(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            <= 0i32
        {
            GetMultiplayerId();
            DestroyTask(taskId);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8))
                .read()) as i32) as isize
                    * 40,
            ))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_WaitToRaiseCurtainAtStart));
            ((&raw mut gRngValue).cast::<u32>())
                .write(((&raw mut gContestRngValue).cast::<u8>().cast::<u32>()).read());
        }
    }
}
pub(crate) unsafe extern "C" fn SetupContestGraphics(stateVar: *mut u8) -> u8 {
    unsafe {
        let mut stateVar = stateVar;
        let mut tempPalette1 = crate::ffi::Align4([0u8; 32]);
        let mut tempPalette2 = crate::ffi::Align4([0u8; 32]);
        'l1: {
            let __sw1 = (((stateVar).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            if __sw1 == 0i32 {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (1u16) as i32,
                );
                RequestDma3Fill(0i32, ((100663296i32) as usize as *mut u8), 32768u16, 1u8);
                RequestDma3Fill(
                    0i32,
                    ((100663296i32) as usize as *mut u8).wrapping_offset(32768),
                    32768u16,
                    1u8,
                );
                RequestDma3Fill(
                    0i32,
                    ((100663296i32) as usize as *mut u8).wrapping_offset(65536),
                    32768u16,
                    1u8,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                LZDecompressVram(
                    ((&raw mut gContestInterfaceGfx).cast::<u32>()).cast::<u32>(),
                    ((100663296i32) as usize as *mut u8),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                LZDecompressVram(
                    ((&raw mut gContestAudienceGfx).cast::<u32>()).cast::<u32>(),
                    ((100671488i32) as usize as *mut u8),
                );
                {
                    let mut _src: *mut u8 = ((100671488i32) as usize as *mut u8);
                    let mut _dest: *mut u8 = ((&raw mut gHeap).cast::<u8>()).wrapping_offset(98304);
                    let mut _size: u32 = 8192u32;
                    'l2: loop {
                        if !((1i32) != 0) {
                            break 'l2;
                        }
                        if _size <= 4096u32 {
                            'l3: loop {
                                'l4: {
                                    'l5: loop {
                                        'l6: {
                                            {
                                                let mut dmaRegs: *mut u32 =
                                                    ((67109076i32) as usize as *mut u32);
                                                crate::c::volatile_write(
                                                    dmaRegs,
                                                    ((_src) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(1),
                                                    ((_dest) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(2),
                                                    (2214592512u32
                                                        | crate::c::div_u32(
                                                            _size,
                                                            ((crate::c::div_i32(32i32, 8i32))
                                                                as u32),
                                                        )),
                                                );
                                                let _ =
                                                    ((dmaRegs).wrapping_offset(2)).read_volatile();
                                            }
                                        }
                                        if !((0i32) != 0) {
                                            break 'l5;
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l3;
                                }
                            }
                            break 'l2;
                        }
                        'l7: loop {
                            'l8: {
                                'l9: loop {
                                    'l10: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (((-2080374784i32)
                                                    | crate::c::div_i32(
                                                        4096i32,
                                                        crate::c::div_i32(32i32, 8i32),
                                                    ))
                                                    as u32),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l9;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l7;
                            }
                        }
                        _src = (_src).wrapping_offset(4096);
                        _dest = (_dest).wrapping_offset(4096);
                        _size = (_size).wrapping_sub(4096u32);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                CopyToBgTilemapBuffer(
                    3u8,
                    (((&raw mut gContestAudienceTilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyBgTilemapBufferToVram(3u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                CopyToBgTilemapBuffer(
                    2u8,
                    (((&raw mut gContestInterfaceTilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyBgTilemapBufferToVram(2u8);
                {
                    let mut _src: *mut u8 =
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(36))
                        .cast::<*mut u8>())
                        .wrapping_offset(2))
                        .read();
                    let mut _dest: *mut u8 = ((((&raw mut gHeap).cast::<u8>())
                        .wrapping_offset(106500))
                    .wrapping_add(2560))
                    .cast::<u8>();
                    let mut _size: u32 = 2048u32;
                    'l11: loop {
                        'l12: {
                            'l13: loop {
                                'l14: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            ((_dest) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (2214592512u32
                                                | crate::c::div_u32(
                                                    _size,
                                                    ((crate::c::div_i32(32i32, 8i32)) as u32),
                                                )),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l13;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                LoadCompressedPalette(
                    ((&raw mut gContestInterfaceAudiencePalette).cast::<u32>()).cast::<u32>(),
                    0u16,
                    512u16,
                );
                'l15: loop {
                    'l16: {
                        'l17: loop {
                            'l18: {
                                CpuSet(
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(128))
                                    .cast::<u8>(),
                                    ((&raw mut tempPalette1).cast::<u16>()).cast::<u8>(),
                                    (67108864u32
                                        | (crate::c::div_u32(
                                            32u32,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l17;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l15;
                    }
                }
                'l19: loop {
                    'l20: {
                        'l21: loop {
                            'l22: {
                                CpuSet(
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(
                                            ((0i32).wrapping_add(
                                                ((5i32).wrapping_add(
                                                    ((((&raw mut gContestPlayerMonIndex)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32),
                                                ))
                                                .wrapping_mul(16i32),
                                            )) as isize,
                                        ))
                                    .cast::<u8>(),
                                    ((&raw mut tempPalette2).cast::<u16>()).cast::<u8>(),
                                    (67108864u32
                                        | (crate::c::div_u32(
                                            32u32,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l21;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l19;
                    }
                }
                'l23: loop {
                    'l24: {
                        'l25: loop {
                            'l26: {
                                CpuSet(
                                    ((&raw mut tempPalette2).cast::<u16>()).cast::<u8>(),
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(128))
                                    .cast::<u8>(),
                                    (67108864u32
                                        | (crate::c::div_u32(
                                            32u32,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l25;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l23;
                    }
                }
                'l27: loop {
                    'l28: {
                        'l29: loop {
                            'l30: {
                                CpuSet(
                                    ((&raw mut tempPalette1).cast::<u16>()).cast::<u8>(),
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(
                                            ((0i32).wrapping_add(
                                                ((5i32).wrapping_add(
                                                    ((((&raw mut gContestPlayerMonIndex)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32),
                                                ))
                                                .wrapping_mul(16i32),
                                            )) as isize,
                                        ))
                                    .cast::<u8>(),
                                    (67108864u32
                                        | (crate::c::div_u32(
                                            32u32,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l29;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l27;
                    }
                }
                {
                    let mut _src: *mut u8 =
                        (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).cast::<u8>();
                    let mut _dest: *mut u8 =
                        (((&raw mut gHeap).cast::<u8>()).wrapping_offset(106500)).cast::<u8>();
                    let mut _size: u32 = 512u32;
                    'l31: loop {
                        'l32: {
                            'l33: loop {
                                'l34: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            ((_dest) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (2214592512u32
                                                | crate::c::div_u32(
                                                    _size,
                                                    ((crate::c::div_i32(32i32, 8i32)) as u32),
                                                )),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l33;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l31;
                        }
                    }
                }
                LoadContestPalettes();
                break 'l1;
            }
            if __sw1 == 6i32 {
                DrawContestantWindows();
                FillContestantWindowBgs();
                SwapMoveDescAndContestTilemaps();
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(18))
                .write(CreateJudgeSpeechBubbleSprite());
                CreateSliderHeartSprites();
                CreateNextTurnSprites();
                CreateApplauseMeterSprite();
                CreateJudgeAttentionEyeTask();
                CreateUnusedBlendTask();
                ((&raw mut gBattlerPositions).cast::<u8>()).write(0u8);
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(1)).write(1u8);
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(2)).write(3u8);
                (((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(3)).write(2u8);
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(0u32);
                ((&raw mut gBattlerAttacker).cast::<u8>()).write(2u8);
                ((&raw mut gBattlerTarget).cast::<u8>()).write(3u8);
                (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(CreateJudgeSprite());
                CreateInvisibleBattleTargetSprite();
                CopyBgTilemapBufferToVram(3u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(1u8);
                ShowBg(3u8);
                ShowBg(2u8);
                ShowBg(0u8);
                ShowBg(1u8);
                break 'l1;
            }
            if !__matched {
                (stateVar).write(0u8);
                return 1u8;
            }
        }
        (stateVar).write(((stateVar).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_WaitToRaiseCurtainAtStart(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            7,
            1,
            (0u16) as i32,
        );
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_RaiseCurtainAtStart));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_RaiseCurtainAtStart(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    let __t3 = (__p2).read();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    __t3
                }) as i32)
                    <= 60i32
                {
                    break 'l1;
                }
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(0i16);
                PlaySE12WithPanning(97u16, 0i8);
                let __p4 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p5 = ((&raw mut gBattle_BG1_Y).cast::<u16>()).cast::<i16>();
                (__p5).write((((((__p5).read()) as i32).wrapping_add(7i32)) as i16));
                if (((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i16) as i32) <= 160i32 {
                    break 'l1;
                }
                let __p6 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                UpdateContestantBoxOrder();
                let __p7 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    let mut bg0Cnt: u16 = GetGpuReg(8u8);
                    let mut bg2Cnt: u16 = GetGpuReg(12u8);
                    crate::c::bf_write(
                        ((&raw mut bg0Cnt).cast::<u8>()).wrapping_add(0),
                        0,
                        2,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        ((&raw mut bg2Cnt).cast::<u8>()).wrapping_add(0),
                        0,
                        2,
                        (0u16) as i32,
                    );
                    SetGpuReg(8u8, bg0Cnt);
                    SetGpuReg(12u8, bg2Cnt);
                    SlideApplauseMeterIn();
                    let __p8 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                    break 'l1;
                }
            }
            if __sw1 == 4i32 || !__matched {
                if (crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6),
                    6,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    break 'l1;
                }
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_DisplayAppealNumberText));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ContestMain() {
    unsafe {
        let mut i: i32 = 0i32;
        AnimateSprites();
        RunTasks();
        BuildOamBuffer();
        UpdatePaletteFade();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_i32(
                        ((((&raw mut sContestBgCopyFlags).cast::<u8>().cast::<u8>()).read())
                            as i32),
                        ((i) as u32),
                    ) & 1i32)
                        != 0
                    {
                        CopyBgTilemapBufferToVram(((i) as u8));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut sContestBgCopyFlags).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Contest() {
    unsafe {
        SetGpuReg(16u8, ((&raw mut gBattle_BG0_X).cast::<u16>()).read());
        SetGpuReg(18u8, ((&raw mut gBattle_BG0_Y).cast::<u16>()).read());
        SetGpuReg(20u8, ((&raw mut gBattle_BG1_X).cast::<u16>()).read());
        SetGpuReg(22u8, ((&raw mut gBattle_BG1_Y).cast::<u16>()).read());
        SetGpuReg(24u8, ((&raw mut gBattle_BG2_X).cast::<u16>()).read());
        SetGpuReg(26u8, ((&raw mut gBattle_BG2_Y).cast::<u16>()).read());
        SetGpuReg(28u8, ((&raw mut gBattle_BG3_X).cast::<u16>()).read());
        SetGpuReg(30u8, ((&raw mut gBattle_BG3_Y).cast::<u16>()).read());
        SetGpuReg(64u8, ((&raw mut gBattle_WIN0H).cast::<u16>()).read());
        SetGpuReg(68u8, ((&raw mut gBattle_WIN0V).cast::<u16>()).read());
        SetGpuReg(66u8, ((&raw mut gBattle_WIN1H).cast::<u16>()).read());
        SetGpuReg(70u8, ((&raw mut gBattle_WIN1V).cast::<u16>()).read());
        TransferPlttBuffer();
        LoadOam();
        ProcessSpriteCopyRequests();
        ScanlineEffect_InitHBlankDmaTransfer();
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayAppealNumberText(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
            ContestDebugDoPrint();
            {
                let mut _src: *mut u8 =
                    (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).cast::<u8>();
                let mut _dest: *mut u8 =
                    (((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106500)).wrapping_add(512))
                        .cast::<u16>())
                    .cast::<u8>();
                let mut _size: u32 = 1024u32;
                'l1: loop {
                    'l2: {
                        'l3: loop {
                            'l4: {
                                {
                                    let mut dmaRegs: *mut u32 =
                                        ((67109076i32) as usize as *mut u32);
                                    crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(1),
                                        ((_dest) as usize as u32),
                                    );
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(2),
                                        (2214592512u32
                                            | crate::c::div_u32(
                                                _size,
                                                ((crate::c::div_i32(32i32, 8i32)) as u32),
                                            )),
                                    );
                                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l3;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l1;
                    }
                }
            }
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1))
                .read()) as i32)
                    .wrapping_add(1i32),
                0i32,
                1u8,
            );
            if !((Contest_IsMonsTurnDisabled(
                ((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read(),
            )) != 0)
            {
                StringCopy(
                    (&raw mut gDisplayedStringBattle).cast::<u8>(),
                    (&raw mut gText_AppealNumWhichMoveWillBePlayed).cast::<u8>(),
                );
            } else {
                StringCopy(
                    (&raw mut gDisplayedStringBattle).cast::<u8>(),
                    (&raw mut gText_AppealNumButItCantParticipate).cast::<u8>(),
                );
            }
            ContestClearGeneralTextWindow();
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gDisplayedStringBattle).cast::<u8>(),
            );
            Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if !((Contest_RunTextPrinters()) != 0) {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_TryShowMoveSelectScreen));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TryShowMoveSelectScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0)
            || ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                == 2i32)
        {
            PlaySE(5u16);
            if !((Contest_IsMonsTurnDisabled(
                ((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read(),
            )) != 0)
            {
                SetBottomSliderHeartsInvisibility(1u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ShowMoveSelectScreen));
            } else {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_SelectedMove));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowMoveSelectScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut moveName = crate::ffi::Align4([0u8; 32]);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(160u16);
        ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(160u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut r#move: u16 = (((((((&raw mut gContestMons).cast::<u8>())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read())
                            as i32) as isize
                            * 64,
                    ))
                    .wrapping_add(30))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read();
                    let mut moveNameBuffer: *mut u8 = (&raw mut moveName).cast::<u8>();
                    if ((((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read())
                            as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(8)
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32)
                        && ((IsContestantAllowedToCombo(
                            ((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read(),
                        )) != 0))
                        && ((AreMovesContestCombo(
                            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>())
                                    .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(8)
                            .cast::<u16>())
                            .read(),
                            r#move,
                        )) != 0))
                        && ((crate::c::bf_read(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>())
                                    .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(21),
                            4,
                            1,
                            false,
                        ) as u8)
                            != 0)
                    {
                        moveNameBuffer = StringCopy(
                            (&raw mut moveName).cast::<u8>(),
                            (&raw mut gText_ColorLightShadowDarkGray).cast::<u8>(),
                        );
                    } else {
                        if ((((r#move) as i32) != 0i32)
                            && ((((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>())
                                    .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(8)
                            .cast::<u16>())
                            .read()) as i32)
                                == ((r#move) as i32)))
                            && ((((((&raw mut gContestMoves).cast::<u8>())
                                .wrapping_offset(((r#move) as i32) as isize * 8))
                            .read()) as i32)
                                != 3i32)
                        {
                            moveNameBuffer = StringCopy(
                                (&raw mut moveName).cast::<u8>(),
                                (&raw mut gText_ColorBlue).cast::<u8>(),
                            );
                        }
                    }
                    moveNameBuffer = StringCopy(
                        moveNameBuffer,
                        (((&raw mut gMoveNames).cast::<u8>())
                            .wrapping_offset(((r#move) as i32) as isize * 13))
                        .cast::<u8>(),
                    );
                    FillWindowPixelBuffer(((((i) as i32).wrapping_add(5i32)) as u8), 0u8);
                    Contest_PrintTextToBg0WindowAt(
                        ((((i) as i32).wrapping_add(5i32)) as u32),
                        (&raw mut moveName).cast::<u8>(),
                        5i32,
                        1i32,
                        7i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        DrawMoveSelectArrow(
            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .read()) as i8),
        );
        PrintContestMoveDescription(
            (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize
                    * 64,
            ))
            .wrapping_add(30))
            .cast::<u16>())
            .wrapping_offset(
                (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .read()) as i32) as isize,
            ))
            .read(),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleMoveSelectInput));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleMoveSelectInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut numMoves: u8 = 0u8;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read())
                                as i32) as isize
                                * 64,
                        ))
                    .wrapping_add(30))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        numMoves = (numMoves).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SelectedMove));
        } else {
            'l3: {
                let __sw1 = (((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32);
                if __sw1 == 2i32 {
                    PlaySE(5u16);
                    SetBottomSliderHeartsInvisibility(0u8);
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar1).cast::<u8>(),
                        ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1))
                        .read()) as i32)
                            .wrapping_add(1i32),
                        0i32,
                        1u8,
                    );
                    if !((Contest_IsMonsTurnDisabled(
                        ((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read(),
                    )) != 0)
                    {
                        StringCopy(
                            (&raw mut gDisplayedStringBattle).cast::<u8>(),
                            (&raw mut gText_AppealNumWhichMoveWillBePlayed).cast::<u8>(),
                        );
                    } else {
                        StringCopy(
                            (&raw mut gDisplayedStringBattle).cast::<u8>(),
                            (&raw mut gText_AppealNumButItCantParticipate).cast::<u8>(),
                        );
                    }
                    ContestClearGeneralTextWindow();
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gDisplayedStringBattle).cast::<u8>(),
                    );
                    Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 0u32);
                    ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                    ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_TryShowMoveSelectScreen));
                    break 'l3;
                }
                if __sw1 == 32i32 || __sw1 == 16i32 {
                    break 'l3;
                }
                if __sw1 == 64i32 {
                    EraseMoveSelectArrow(
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .read()) as i8),
                    );
                    if (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .read()) as i32)
                        == 0i32
                    {
                        (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .write(((((numMoves) as i32).wrapping_sub(1i32)) as u8));
                    } else {
                        let __p2 =
                            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<*mut u8>())
                            .read());
                        (__p2).write(((__p2).read()).wrapping_sub(1));
                    }
                    DrawMoveSelectArrow(
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .read()) as i8),
                    );
                    PrintContestMoveDescription(
                        (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>())
                                    .read()) as i32) as isize
                                    * 64,
                            ))
                        .wrapping_add(30))
                        .cast::<u16>())
                        .wrapping_offset(
                            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<*mut u8>())
                            .read())
                            .read()) as i32) as isize,
                        ))
                        .read(),
                    );
                    if ((numMoves) as i32) > 1i32 {
                        PlaySE(5u16);
                    }
                    break 'l3;
                }
                if __sw1 == 128i32 {
                    EraseMoveSelectArrow(
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .read()) as i8),
                    );
                    if (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .read()) as i32)
                        == ((numMoves) as i32).wrapping_sub(1i32)
                    {
                        (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .write(0u8);
                    } else {
                        let __p3 =
                            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<*mut u8>())
                            .read());
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    }
                    DrawMoveSelectArrow(
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .read()) as i8),
                    );
                    PrintContestMoveDescription(
                        (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>())
                                    .read()) as i32) as isize
                                    * 64,
                            ))
                        .wrapping_add(30))
                        .cast::<u16>())
                        .wrapping_offset(
                            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<*mut u8>())
                            .read())
                            .read()) as i32) as isize,
                        ))
                        .read(),
                    );
                    if ((numMoves) as i32) > 1i32 {
                        PlaySE(5u16);
                    }
                    break 'l3;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawMoveSelectArrow(moveIndex: i8) {
    unsafe {
        let mut moveIndex = moveIndex;
        ContestBG_FillBoxWithIncrementingTile(
            2u8,
            55u16,
            0u8,
            (((31i32).wrapping_add(((moveIndex) as i32).wrapping_mul(2i32))) as u8),
            2u8,
            2u8,
            17u8,
            1i16,
        );
    }
}
pub(crate) unsafe extern "C" fn EraseMoveSelectArrow(moveIndex: i8) {
    unsafe {
        let mut moveIndex = moveIndex;
        ContestBG_FillBoxWithIncrementingTile(
            2u8,
            11u16,
            0u8,
            (((31i32).wrapping_add(((moveIndex) as i32).wrapping_mul(2i32))) as u8),
            2u8,
            1u8,
            17u8,
            1i16,
        );
        ContestBG_FillBoxWithIncrementingTile(
            2u8,
            11u16,
            0u8,
            (((32i32).wrapping_add(((moveIndex) as i32).wrapping_mul(2i32))) as u8),
            2u8,
            1u8,
            17u8,
            1i16,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_SelectedMove(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32) != 0
        {
            let mut r#move: u16 =
                GetChosenMove(((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read());
            let mut taskId2: u8 = 0u8;
            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize
                    * 28,
            ))
            .wrapping_add(6)
            .cast::<u16>())
            .write(r#move);
            taskId2 = CreateTask(Some(Task_LinkContest_CommunicateMoveSelections), 0u8);
            SetTaskFuncWithFollowupFunc(
                taskId2,
                Some(Task_LinkContest_CommunicateMoveSelections),
                Some(Task_EndCommunicateMoveSelections),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(TaskDummy1));
            ContestPrintLinkStandby();
            SetBottomSliderHeartsInvisibility(0u8);
        } else {
            GetAllChosenMoves();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HideMoveSelectScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_EndCommunicateMoveSelections(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyTask(taskId);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8))
            .read()) as i32) as isize
                * 40,
        ))
        .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HideMoveSelectScreen));
    }
}
pub(crate) unsafe extern "C" fn Task_HideMoveSelectScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        ContestClearGeneralTextWindow();
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
        SetBottomSliderHeartsInvisibility(0u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    FillWindowPixelBuffer((((5i32).wrapping_add(i)) as u8), 0u8);
                    PutWindowTilemap((((5i32).wrapping_add(i)) as u8));
                    CopyWindowToVram((((5i32).wrapping_add(i)) as u8), 2u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        Contest_SetBgCopyFlags(0u32);
        {
            let mut _src: *mut u8 =
                (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).cast::<u8>();
            let mut _dest: *mut u8 = (((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106500))
                .wrapping_add(1536))
            .cast::<u16>())
            .cast::<u8>();
            let mut _size: u32 = 1024u32;
            'l3: loop {
                'l4: {
                    'l5: loop {
                        'l6: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    ((_dest) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (2214592512u32
                                        | crate::c::div_u32(
                                            _size,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
                                        )),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l5;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l3;
                }
            }
        }
        LoadPalette(
            (((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106500)).wrapping_add(512))
                .cast::<u16>())
            .cast::<u8>(),
            0u16,
            1024u16,
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HideApplauseMeterForAppealStart));
    }
}
pub(crate) unsafe extern "C" fn Task_HideApplauseMeterForAppealStart(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 2i32
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            if (({
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 2i32
            {
                SlideApplauseMeterOut();
                AnimateSliderHearts(1u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_WaitHideApplauseMeterForAppealStart));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitHideApplauseMeterForAppealStart(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (!((crate::c::bf_read(
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(6),
            6,
            1,
            false,
        ) as u16)
            != 0))
            && (!((crate::c::bf_read(
                (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(7),
                1,
                1,
                false,
            ) as u16)
                != 0))
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_AppealSetup));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_AppealSetup(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 19i32
        {
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(16))
            .write(0u8);
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(24)
            .cast::<u32>())
            .write(((&raw mut gRngValue).cast::<u32>()).read());
            if ((((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32)
                != 0)
                && ((IsPlayerLinkLeader()) != 0)
            {
                let mut i: i32 = 0i32;
                {
                    i = 0i32;
                    'l1: loop {
                        if !((i).wrapping_add(
                            ((((&raw mut gNumLinkContestPlayers).cast::<u8>().cast::<u8>()).read())
                                as i32),
                        ) < 4i32)
                        {
                            break 'l1;
                        }
                        'l2: {
                            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                (((((&raw mut gNumLinkContestPlayers).cast::<u8>().cast::<u8>())
                                    .read()) as i32)
                                    .wrapping_add(i)) as isize
                                    * 28,
                            ))
                            .wrapping_add(6)
                            .cast::<u16>())
                            .write(GetChosenMove(
                                ((((((&raw mut gNumLinkContestPlayers).cast::<u8>().cast::<u8>())
                                    .read()) as i32)
                                    .wrapping_add(i)) as u8),
                            ));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_DoAppeals));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DoAppeals(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut contestant: u8 =
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(17))
            .read();
        let mut r3: i8 = 0i8;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ContestDebugDoPrint();
                {
                    i = 0i32;
                    'l2: loop {
                        if !(((((((((&raw mut gContestResources)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(16))
                        .read()) as i32)
                            != (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {}
                        i = (i).wrapping_add(1);
                    }
                }
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .write(((i) as u8));
                contestant = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                    .read())
                .cast::<*mut u8>())
                .read())
                .wrapping_add(17))
                .read();
                if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32)
                    & 1i32)
                    != 0
                {
                    let mut taskId2: u8 = 0u8;
                    crate::c::bf_write(
                        (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(7),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    if (IsPlayerLinkLeader()) != 0 {
                        CalculateAppealMoveImpact(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(17))
                            .read(),
                        );
                    }
                    taskId2 = CreateTask(Some(Task_LinkContest_CommunicateAppealsState), 0u8);
                    SetTaskFuncWithFollowupFunc(
                        taskId2,
                        Some(Task_LinkContest_CommunicateAppealsState),
                        Some(Task_EndWaitForLink),
                    );
                    ContestPrintLinkStandby();
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                } else {
                    CalculateAppealMoveImpact(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read(),
                    );
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                return;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(7),
                    2,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                return;
            }
            if __sw1 == 2i32 {
                SetContestLiveUpdateFlags(contestant);
                ContestDebugPrintBitStrings();
                if (((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(12),
                    1,
                    2,
                    false,
                ) as u8) as i32)
                    != 0i32)
                    || ((crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(11),
                        7,
                        1,
                        false,
                    ) as u8)
                        != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(31i16);
                } else {
                    ContestClearGeneralTextWindow();
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(3i16);
                }
                return;
            }
            if __sw1 == 3i32 {
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i < 4i32) {
                            break 'l4;
                        }
                        'l5: {
                            (((&raw mut gBattleMonForms).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .write(0u8);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                crate::c::memset(
                    ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read(),
                    0i32,
                    20u32,
                );
                SetMoveAnimAttackerData(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read(),
                );
                spriteId = CreateContestantSprite(
                    (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read()) as i32) as isize
                            * 64,
                    ))
                    .cast::<u16>())
                    .read(),
                    (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read()) as i32) as isize
                            * 64,
                    ))
                    .wrapping_add(60)
                    .cast::<u32>())
                    .read(),
                    (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read()) as i32) as isize
                            * 64,
                    ))
                    .wrapping_add(56)
                    .cast::<u32>())
                    .read(),
                    ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read()) as u32),
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(120i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_MonSlideIn));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(((spriteId) as i16));
                (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(spriteId);
                BlinkContestantBox(
                    CreateContestantBoxBlinkSprites(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read(),
                    ),
                    0u8,
                );
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(4i16);
                return;
            }
            if __sw1 == 4i32 {
                spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u8);
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) == (SpriteCallbackDummy as *const () as usize)
                {
                    if !((crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 4))
                        .wrapping_add(2),
                        1,
                        1,
                        false,
                    ) as u8)
                        != 0)
                    {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(5i16);
                    }
                }
                return;
            }
            if __sw1 == 5i32 {
                if (crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(12),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(33i16);
                } else {
                    ContestClearGeneralTextWindow();
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((contestant) as i32) as isize * 64))
                        .wrapping_add(2))
                        .cast::<u8>(),
                    );
                    if (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read()) as i32)
                        < 355i32
                    {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                                (((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((contestant) as i32) as isize * 28))
                                .wrapping_add(6)
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 13,
                            ))
                            .cast::<u8>(),
                        );
                    } else {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            ((((&raw const sInvalidContestMoveNames)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset(
                                (((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((contestant) as i32) as isize * 28))
                                .wrapping_add(10))
                                .read()) as i32) as isize,
                            ))
                            .read(),
                        );
                    }
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_MonAppealedWithMove).cast::<u8>(),
                    );
                    Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(6i16);
                }
                return;
            }
            if __sw1 == 6i32 {
                if !((Contest_RunTextPrinters()) != 0) {
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(90))
                    .write(0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(7i16);
                }
                return;
            }
            if __sw1 == 7i32 {
                {
                    let mut r#move: u16 = SanitizeMove(
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(17))
                            .read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read(),
                    );
                    SetMoveSpecificAnimData(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read(),
                    );
                    SetMoveAnimAttackerData(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read(),
                    );
                    SetMoveTargetPosition(r#move);
                    DoMoveAnim(r#move);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(8i16);
                }
                return;
            }
            if __sw1 == 8i32 {
                (((&raw mut gAnimScriptCallback).cast::<Option<unsafe extern "C" fn()>>()).read())
                    .unwrap_unchecked()();
                if !((((&raw mut gAnimScriptActive).cast::<u8>()).read()) != 0) {
                    ClearMoveAnimData(contestant);
                    if ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(90))
                    .read()) as i32)
                        != 0i32
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .write(0i16);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(9i16);
                    } else {
                        if !((crate::c::bf_read(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 28))
                            .wrapping_add(21),
                            4,
                            1,
                            false,
                        ) as u8)
                            != 0)
                        {
                            StopFlashJudgeAttentionEye(contestant);
                        }
                        DrawUnnervedSymbols();
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(23i16);
                    }
                }
                return;
            }
            if __sw1 == 9i32 {
                if (({
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t3 = (__p2).read();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    __t3
                }) as i32)
                    > 30i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(7i16);
                }
                return;
            }
            if __sw1 == 23i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(0i16);
                if (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(19))
                .read()) as i32)
                    != 255i32
                {
                    PrintAppealMoveResultText(
                        contestant,
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(19))
                        .read(),
                    );
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(19))
                    .write(255u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(24i16);
                } else {
                    if (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(20))
                    .read()) as i32)
                        != 255i32
                    {
                        {
                            i = 0i32;
                            'l6: loop {
                                if !(i < 4i32) {
                                    break 'l6;
                                }
                                'l7: {
                                    if (i != ((contestant) as i32))
                                        && ((((((((((&raw mut gContestResources)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset((i) as isize * 28))
                                        .wrapping_add(19))
                                        .read())
                                            as i32)
                                            != 255i32)
                                    {
                                        break 'l6;
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        if i == 4i32 {
                            PrintAppealMoveResultText(
                                contestant,
                                (((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((contestant) as i32) as isize * 28))
                                .wrapping_add(20))
                                .read(),
                            );
                            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 28))
                            .wrapping_add(20))
                            .write(255u8);
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(24i16);
                        } else {
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(48i16);
                        }
                    } else {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(48i16);
                    }
                }
                return;
            }
            if __sw1 == 24i32 {
                if !((Contest_RunTextPrinters()) != 0) {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(23i16);
                }
                return;
            }
            if __sw1 == 48i32 {
                if ((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(17),
                    0,
                    2,
                    false,
                ) as u8) as i32)
                    == 1i32
                {
                    DoJudgeSpeechBubble(5u8);
                } else {
                    if ((crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(17),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        == 2i32
                    {
                        DoJudgeSpeechBubble(6u8);
                    } else {
                        if ((crate::c::bf_read(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 28))
                            .wrapping_add(17),
                            0,
                            2,
                            false,
                        ) as u8) as i32)
                            == 3i32
                        {
                            DoJudgeSpeechBubble(7u8);
                        } else {
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(47i16);
                            return;
                        }
                    }
                }
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(49i16);
                return;
            }
            if __sw1 == 49i32 {
                if !((crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6),
                    4,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(47i16);
                }
                return;
            }
            if __sw1 == 47i32 {
                ShowHideNextTurnGfx(1u8);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(12i16);
                return;
            }
            if __sw1 == 12i32 {
                UpdateAppealHearts(
                    0i16,
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(2)
                    .cast::<i16>())
                    .read(),
                    contestant,
                );
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(13i16);
                return;
            }
            if __sw1 == 13i32 {
                if !((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(2),
                    2,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(35i16);
                }
                return;
            }
            if __sw1 == 35i32 {
                if ((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(16),
                    4,
                    2,
                    false,
                ) as u8) as i32)
                    == 1i32
                {
                    DoJudgeSpeechBubble(8u8);
                }
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(36i16);
                return;
            }
            if __sw1 == 36i32 {
                if !((crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6),
                    4,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(37i16);
                }
                return;
            }
            if __sw1 == 37i32 {
                if (UpdateConditionStars(contestant, 1u8)) != 0 {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(38i16);
                } else {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(50i16);
                }
                return;
            }
            if __sw1 == 38i32 {
                if (({
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 20i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(50i16);
                }
                return;
            }
            if __sw1 == 50i32 {
                if (DrawStatusSymbol(contestant)) != 0 {
                    PlaySE(99u16);
                }
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(25i16);
                return;
            }
            if __sw1 == 25i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(0i16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(26i16);
                return;
            }
            if __sw1 == 26i32 {
                {
                    let mut j: i32 = 0i32;
                    r3 = 0i8;
                    {
                        i = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32);
                        'l8: loop {
                            if !(i < 4i32) {
                                break 'l8;
                            }
                            'l9: {
                                r3 = 0i8;
                                {
                                    j = 0i32;
                                    'l10: loop {
                                        if !(j < 4i32) {
                                            break 'l10;
                                        }
                                        'l11: {
                                            if ((j != ((contestant) as i32))
                                                && (((((((&raw mut gContestantTurnOrder)
                                                    .cast::<u8>())
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize))
                                                .read())
                                                    as i32)
                                                    == i))
                                                && ((((((((((&raw mut gContestResources)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(4)
                                                .cast::<*mut u8>())
                                                .read())
                                                .wrapping_offset((j) as isize * 28))
                                                .wrapping_add(19))
                                                .read())
                                                    as i32)
                                                    != 255i32)
                                            {
                                                r3 = 1i8;
                                                break 'l10;
                                            }
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                                if (r3) != 0 {
                                    break 'l8;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    if (r3) != 0 {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(
                            ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((j) as isize))
                            .read()) as i16),
                        );
                        PrintAppealMoveResultText(
                            ((j) as u8),
                            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((j) as isize * 28))
                            .wrapping_add(19))
                            .read(),
                        );
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((j) as isize * 28))
                        .wrapping_add(19))
                        .write(255u8);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(27i16);
                    } else {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .write(0i16);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(51i16);
                        DrawStatusSymbols();
                    }
                }
                return;
            }
            if __sw1 == 27i32 {
                if !((Contest_RunTextPrinters()) != 0) {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(28i16);
                }
                return;
            }
            if __sw1 == 28i32 {
                {
                    i = 0i32;
                    'l12: loop {
                        if !(((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            != ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32))
                        {
                            break 'l12;
                        }
                        'l13: {}
                        i = (i).wrapping_add(1);
                    }
                }
                UpdateAppealHearts(
                    (((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(2)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(
                            (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(14))
                            .read()) as i32),
                        )) as i16),
                    (((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(14))
                    .read()) as i32)
                        .wrapping_neg()) as i16),
                    ((i) as u8),
                );
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(29i16);
                return;
            }
            if __sw1 == 29i32 {
                {
                    i = 0i32;
                    'l14: loop {
                        if !(((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            != ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32))
                        {
                            break 'l14;
                        }
                        'l15: {}
                        i = (i).wrapping_add(1);
                    }
                }
                if !((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 4))
                    .wrapping_add(2),
                    2,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(39i16);
                }
                return;
            }
            if __sw1 == 39i32 {
                {
                    i = 0i32;
                    'l16: loop {
                        if !(((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            != ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32))
                        {
                            break 'l16;
                        }
                        'l17: {}
                        i = (i).wrapping_add(1);
                    }
                }
                if (UpdateConditionStars(((i) as u8), 1u8)) != 0 {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(40i16);
                } else {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(30i16);
                }
                return;
            }
            if __sw1 == 40i32 {
                if (({
                    let __p6 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    > 20i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(30i16);
                }
                return;
            }
            if __sw1 == 30i32 {
                {
                    i = 0i32;
                    'l18: loop {
                        if !(i < 4i32) {
                            break 'l18;
                        }
                        'l19: {
                            if ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as i32)
                            {
                                break 'l18;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if (DrawStatusSymbol(((i) as u8))) != 0 {
                    PlaySE(99u16);
                } else {
                    PlaySE(100u16);
                }
                if (crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(21),
                    5,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    StopFlashJudgeAttentionEye(((i) as u8));
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(21),
                        5,
                        1,
                        (0u8) as i32,
                    );
                }
                let __p8 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p8).write(((__p8).read()).wrapping_add(1));
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(26i16);
                return;
            }
            if __sw1 == 51i32 {
                if (({
                    let __p9 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t10 = (__p9).read();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                    __t10
                }) as i32)
                    > 9i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    if (((crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(12),
                        1,
                        2,
                        false,
                    ) as u8) as i32)
                        != 0i32)
                        || ((crate::c::bf_read(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 28))
                            .wrapping_add(17),
                            2,
                            1,
                            false,
                        ) as u8)
                            != 0)
                    {
                        ContestClearGeneralTextWindow();
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((contestant) as i32) as isize * 64))
                            .wrapping_add(2))
                            .cast::<u8>(),
                        );
                        StringExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            (&raw mut gText_MonCantAppealNextTurn).cast::<u8>(),
                        );
                        Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
                    }
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(52i16);
                }
                return;
            }
            if __sw1 == 52i32 {
                if !((Contest_RunTextPrinters()) != 0) {
                    if !((crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(21),
                        6,
                        1,
                        false,
                    ) as u8)
                        != 0)
                    {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(17i16);
                    } else {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(14i16);
                    }
                }
                return;
            }
            if __sw1 == 14i32 {
                {
                    let mut completedCombo: i8 =
                        (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(22))
                        .read()) as i8);
                    if ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(22))
                    .read())
                        != 0
                    {
                        ContestClearGeneralTextWindow();
                        if ((completedCombo) as i32) == 1i32 {
                            Contest_StartTextPrinter(
                                (&raw mut gText_AppealComboWentOverWell).cast::<u8>(),
                                1u32,
                            );
                        } else {
                            if ((completedCombo) as i32) == 2i32 {
                                Contest_StartTextPrinter(
                                    (&raw mut gText_AppealComboWentOverVeryWell).cast::<u8>(),
                                    1u32,
                                );
                            } else {
                                Contest_StartTextPrinter(
                                    (&raw mut gText_AppealComboWentOverExcellently).cast::<u8>(),
                                    1u32,
                                );
                            }
                        }
                        DoJudgeSpeechBubble(3u8);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .write(0i16);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(45i16);
                    } else {
                        ContestClearGeneralTextWindow();
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((contestant) as i32) as isize * 64))
                            .wrapping_add(2))
                            .cast::<u8>(),
                        );
                        StringExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            (&raw mut gText_JudgeLookedAtMonExpectantly).cast::<u8>(),
                        );
                        Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
                        DoJudgeSpeechBubble(2u8);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .write(0i16);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(45i16);
                    }
                    return;
                }
            }
            if __sw1 == 45i32 {
                if !((crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6),
                    4,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    StartStopFlashJudgeAttentionEye(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read(),
                    );
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(15i16);
                }
                return;
            }
            if __sw1 == 15i32 {
                if !((Contest_RunTextPrinters()) != 0) {
                    if (({
                        let __p11 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10);
                        let __t12 = ((__p11).read()).wrapping_add(1);
                        (__p11).write(__t12);
                        __t12
                    }) as i32)
                        > 50i32
                    {
                        if !((crate::c::bf_read(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 28))
                            .wrapping_add(21),
                            4,
                            1,
                            false,
                        ) as u8)
                            != 0)
                        {
                            UpdateAppealHearts(
                                (((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((contestant) as i32) as isize * 28))
                                .wrapping_add(2)
                                .cast::<i16>())
                                .read(),
                                (((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((contestant) as i32) as isize * 28))
                                .wrapping_add(23))
                                .read()) as i16),
                                contestant,
                            );
                            let __p13 = ((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 28))
                            .wrapping_add(2)
                            .cast::<i16>();
                            (__p13).write(
                                (((((__p13).read()) as i32).wrapping_add(
                                    (((((((((&raw mut gContestResources)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((contestant) as i32) as isize * 28))
                                    .wrapping_add(23))
                                    .read()) as i32),
                                )) as i16),
                            );
                        }
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(16i16);
                    }
                }
                return;
            }
            if __sw1 == 16i32 {
                if !((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 4))
                    .wrapping_add(2),
                    2,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(17i16);
                }
                return;
            }
            if __sw1 == 17i32 {
                if (crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(21),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    ContestClearGeneralTextWindow();
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((contestant) as i32) as isize * 64))
                        .wrapping_add(2))
                        .cast::<u8>(),
                    );
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_RepeatedAppeal).cast::<u8>(),
                    );
                    Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    DoJudgeSpeechBubble(0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(46i16);
                } else {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(41i16);
                }
                return;
            }
            if __sw1 == 46i32 {
                if !((crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6),
                    4,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(19i16);
                }
                return;
            }
            if __sw1 == 19i32 {
                if !((Contest_RunTextPrinters()) != 0) {
                    UpdateAppealHearts(
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(2)
                        .cast::<i16>())
                        .read(),
                        (((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(24))
                        .read()) as i32)
                            .wrapping_neg()) as i16),
                        contestant,
                    );
                    let __p14 =
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(2)
                        .cast::<i16>();
                    (__p14).write(
                        (((((__p14).read()) as i32).wrapping_sub(
                            (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 28))
                            .wrapping_add(24))
                            .read()) as i32),
                        )) as i16),
                    );
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(18i16);
                }
                return;
            }
            if __sw1 == 18i32 {
                ContestDebugDoPrint();
                if !((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 4))
                    .wrapping_add(2),
                    2,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    ContestClearGeneralTextWindow();
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(41i16);
                }
                return;
            }
            if __sw1 == 41i32 {
                if ((crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0)
                    && (((contestant) as i32)
                        != ((crate::c::bf_read(
                            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1),
                            1,
                            3,
                            false,
                        ) as u8) as i32))
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(57i16);
                } else {
                    r3 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(16)
                    .cast::<*mut u8>())
                    .read())
                    .cast::<i8>())
                    .read();
                    if (crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(17),
                        4,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        r3 = 1i8;
                        StringCopy(
                            (&raw mut gStringVar3).cast::<u8>(),
                            (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                                (((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((contestant) as i32) as isize * 28))
                                .wrapping_add(6)
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 13,
                            ))
                            .cast::<u8>(),
                        );
                    } else {
                        StringCopy(
                            (&raw mut gStringVar3).cast::<u8>(),
                            ((((&raw const sContestConditions)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    (((&raw mut gContestMoves).cast::<u8>()).wrapping_offset(
                                        (((((((((&raw mut gContestResources)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((contestant) as i32) as isize * 28))
                                        .wrapping_add(6)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 8,
                                    ))
                                    .wrapping_add(1),
                                    0,
                                    3,
                                    false,
                                ) as u8) as i32) as isize,
                            ))
                            .read(),
                        );
                    }
                    if (((r3) as i32) > 0i32)
                        && ((crate::c::bf_read(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 28))
                            .wrapping_add(21),
                            0,
                            1,
                            false,
                        ) as u8)
                            != 0)
                    {
                        r3 = 0i8;
                    }
                    ContestClearGeneralTextWindow();
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((contestant) as i32) as isize * 64))
                        .wrapping_add(2))
                        .cast::<u8>(),
                    );
                    let __p15 = (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(19)
                    .cast::<i8>();
                    (__p15).write((((((__p15).read()) as i32).wrapping_add(((r3) as i32))) as i8));
                    if ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(19)
                    .cast::<i8>())
                    .read()) as i32)
                        < 0i32
                    {
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(19)
                        .cast::<i8>())
                        .write(0i8);
                    }
                    if ((r3) as i32) == 0i32 {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(55i16);
                    } else {
                        if ((r3) as i32) < 0i32 {
                            StringExpandPlaceholders(
                                (&raw mut gStringVar4).cast::<u8>(),
                                (&raw mut gText_MonsXDidntGoOverWell).cast::<u8>(),
                            );
                        } else {
                            if (((r3) as i32) > 0i32)
                                && (((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(19)
                                .cast::<i8>())
                                .read()) as i32)
                                    <= 4i32)
                            {
                                StringExpandPlaceholders(
                                    (&raw mut gStringVar4).cast::<u8>(),
                                    (&raw mut gText_MonsXWentOverGreat).cast::<u8>(),
                                );
                            } else {
                                StringExpandPlaceholders(
                                    (&raw mut gStringVar4).cast::<u8>(),
                                    (&raw mut gText_MonsXGotTheCrowdGoing).cast::<u8>(),
                                );
                            }
                        }
                        Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(11))
                        .write(0i16);
                        if ((r3) as i32) < 0i32 {
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(53i16);
                        } else {
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(54i16);
                        }
                    }
                }
                return;
            }
            if __sw1 == 53i32 {
                'l20: {
                    let __sw16 = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .read()) as i32);
                    if __sw16 == 0i32 {
                        BlendAudienceBackground((-1i8), 1i8);
                        PlayFanfare(391u16);
                        let __p17 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10);
                        (__p17).write(((__p17).read()).wrapping_add(1));
                        break 'l20;
                    }
                    if __sw16 == 1i32 {
                        if (!((crate::c::bf_read(
                            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(7),
                            0,
                            1,
                            false,
                        ) as u16)
                            != 0))
                            && (!((Contest_RunTextPrinters()) != 0))
                        {
                            ShowAndUpdateApplauseMeter((-1i8));
                            let __p18 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(10);
                            (__p18).write(((__p18).read()).wrapping_add(1));
                        }
                        break 'l20;
                    }
                    if __sw16 == 2i32 {
                        if !((crate::c::bf_read(
                            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(6),
                            5,
                            1,
                            false,
                        ) as u16)
                            != 0)
                        {
                            if (({
                                let __p19 = (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(11);
                                let __t20 = (__p19).read();
                                (__p19).write(((__p19).read()).wrapping_add(1));
                                __t20
                            }) as i32)
                                > 29i32
                            {
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(11))
                                .write(0i16);
                                BlendAudienceBackground((-1i8), (-1i8));
                                let __p21 = (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(10);
                                (__p21).write(((__p21).read()).wrapping_add(1));
                            }
                        }
                        break 'l20;
                    }
                    if __sw16 == 3i32 {
                        if !((crate::c::bf_read(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                            7,
                            1,
                            false,
                        ) as u16)
                            != 0)
                        {
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(10))
                            .write(0i16);
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .write(0i16);
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(43i16);
                        }
                        break 'l20;
                    }
                }
                return;
            }
            if __sw1 == 54i32 {
                'l21: {
                    let __sw22 = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .read()) as i32);
                    if __sw22 == 0i32 {
                        if !((Contest_RunTextPrinters()) != 0) {
                            BlendAudienceBackground(1i8, 1i8);
                            let __p23 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(10);
                            (__p23).write(((__p23).read()).wrapping_add(1));
                        }
                        break 'l21;
                    }
                    if __sw22 == 1i32 {
                        if !((crate::c::bf_read(
                            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(7),
                            0,
                            1,
                            false,
                        ) as u16)
                            != 0)
                        {
                            AnimateAudience();
                            PlaySE(223u16);
                            ShowAndUpdateApplauseMeter(1i8);
                            let __p24 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(10);
                            (__p24).write(((__p24).read()).wrapping_add(1));
                        }
                        break 'l21;
                    }
                    if __sw22 == 2i32 {
                        if !((crate::c::bf_read(
                            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(6),
                            5,
                            1,
                            false,
                        ) as u16)
                            != 0)
                        {
                            if (({
                                let __p25 = (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(11);
                                let __t26 = (__p25).read();
                                (__p25).write(((__p25).read()).wrapping_add(1));
                                __t26
                            }) as i32)
                                > 29i32
                            {
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(11))
                                .write(0i16);
                                UpdateAppealHearts(
                                    (((((((&raw mut gContestResources)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((contestant) as i32) as isize * 28))
                                    .wrapping_add(2)
                                    .cast::<i16>())
                                    .read(),
                                    ((((((((&raw mut gContestResources)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(16)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(2)
                                    .cast::<i8>())
                                    .read()) as i16),
                                    contestant,
                                );
                                let __p27 = ((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((contestant) as i32) as isize * 28))
                                .wrapping_add(2)
                                .cast::<i16>();
                                (__p27).write(
                                    (((((__p27).read()) as i32).wrapping_add(
                                        ((((((((&raw mut gContestResources)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(16)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(2)
                                        .cast::<i8>())
                                        .read()) as i32),
                                    )) as i16),
                                );
                                let __p28 = (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(10);
                                (__p28).write(((__p28).read()).wrapping_add(1));
                            }
                        }
                        break 'l21;
                    }
                    if __sw22 == 3i32 {
                        if !((crate::c::bf_read(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 4))
                            .wrapping_add(2),
                            2,
                            1,
                            false,
                        ) as u8)
                            != 0)
                        {
                            if !((crate::c::bf_read(
                                (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(6),
                                7,
                                1,
                                false,
                            ) as u16)
                                != 0)
                            {
                                BlendAudienceBackground(1i8, (-1i8));
                                let __p29 = (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(10);
                                (__p29).write(((__p29).read()).wrapping_add(1));
                            }
                        }
                        break 'l21;
                    }
                    if __sw22 == 4i32 {
                        if !((crate::c::bf_read(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                            7,
                            1,
                            false,
                        ) as u16)
                            != 0)
                        {
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(10))
                            .write(0i16);
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .write(0i16);
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(43i16);
                        }
                        break 'l21;
                    }
                }
                return;
            }
            if __sw1 == 43i32 {
                if !((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 4))
                    .wrapping_add(2),
                    2,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    ContestClearGeneralTextWindow();
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(55i16);
                }
                return;
            }
            if __sw1 == 57i32 {
                ContestClearGeneralTextWindow();
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        ((crate::c::bf_read(
                            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1),
                            1,
                            3,
                            false,
                        ) as u8) as i32) as isize
                            * 64,
                    ))
                    .wrapping_add(2))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((contestant) as i32) as isize * 64))
                    .wrapping_add(2))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 13,
                    ))
                    .cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_CrowdContinuesToWatchMon).cast::<u8>(),
                );
                Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(58i16);
                return;
            }
            if __sw1 == 58i32 {
                if !((Contest_RunTextPrinters()) != 0) {
                    ContestClearGeneralTextWindow();
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_MonsMoveIsIgnored).cast::<u8>(),
                    );
                    Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(59i16);
                }
                return;
            }
            if __sw1 == 59i32 {
                if !((Contest_RunTextPrinters()) != 0) {
                    ContestClearGeneralTextWindow();
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(55i16);
                }
                return;
            }
            if __sw1 == 33i32 {
                if (crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(21),
                    4,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(21),
                        4,
                        1,
                        (0u8) as i32,
                    );
                }
                StartStopFlashJudgeAttentionEye(contestant);
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((contestant) as i32) as isize * 64))
                    .wrapping_add(2))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 13,
                    ))
                    .cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_MonWasTooNervousToMove).cast::<u8>(),
                );
                Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(34i16);
                return;
            }
            if __sw1 == 34i32 {
                if !((Contest_RunTextPrinters()) != 0) {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(55i16);
                }
                return;
            }
            if __sw1 == 55i32 {
                SlideApplauseMeterOut();
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(56i16);
                return;
            }
            if __sw1 == 56i32 {
                if !((crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6),
                    6,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    if ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(19)
                    .cast::<i8>())
                    .read()) as i32)
                        > 4i32
                    {
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(19)
                        .cast::<i8>())
                        .write(0i8);
                        UpdateApplauseMeter();
                    }
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(10i16);
                }
                return;
            }
            if __sw1 == 10i32 {
                spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u8);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_MonSlideOut));
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(11i16);
                return;
            }
            if __sw1 == 11i32 {
                spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u8);
                if (crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(62),
                    2,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    FreeSpriteOamMatrix(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                    );
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                    );
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(20i16);
                }
                return;
            }
            if __sw1 == 20i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(0i16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(21i16);
                return;
            }
            if __sw1 == 31i32 {
                ContestClearGeneralTextWindow();
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((contestant) as i32) as isize * 64))
                    .wrapping_add(2))
                    .cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_MonWasWatchingOthers).cast::<u8>(),
                );
                Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(32i16);
                return;
            }
            if __sw1 == 32i32 {
                if !((Contest_RunTextPrinters()) != 0) {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(21i16);
                }
                return;
            }
            if __sw1 == 21i32 {
                if (({
                    let __p30 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t31 = ((__p30).read()).wrapping_add(1);
                    (__p30).write(__t31);
                    __t31
                }) as i32)
                    > 29i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(22i16);
                }
                return;
            }
            if __sw1 == 22i32 {
                if (({
                    let __p32 = (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(16);
                    let __t33 = ((__p32).read()).wrapping_add(1);
                    (__p32).write(__t33);
                    __t33
                }) as i32)
                    == 4i32
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(0i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_FinishRoundOfAppeals));
                } else {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                }
                return;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_EndWaitForLink(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        crate::c::bf_write(
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(7),
            2,
            1,
            (0u16) as i32,
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonSlideIn(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) != 0i32 {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
        } else {
            if (({
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 31i32
            {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonSlideOut(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(6i32)) as i16));
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
            < (-32i32)
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_FinishRoundOfAppeals(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32)
                    & 1i32)
                    != 0
                {
                    let mut taskId2: u8 = 0u8;
                    crate::c::bf_write(
                        (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(7),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    if (IsPlayerLinkLeader()) != 0 {
                        RankContestants();
                        SetAttentionLevels();
                    }
                    taskId2 = CreateTask(Some(Task_LinkContest_CommunicateAppealsState), 0u8);
                    SetTaskFuncWithFollowupFunc(
                        taskId2,
                        Some(Task_LinkContest_CommunicateAppealsState),
                        Some(Task_EndWaitForLink),
                    );
                    ContestPrintLinkStandby();
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                } else {
                    RankContestants();
                    SetAttentionLevels();
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(7),
                    2,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ReadyUpdateHeartSliders));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReadyUpdateHeartSliders(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ShowHideNextTurnGfx(0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_UpdateHeartSliders));
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateHeartSliders(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 20i32
                {
                    AnimateSliderHearts(2u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(7),
                    1,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    if (({
                        let __p5 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1);
                        let __t6 = ((__p5).read()).wrapping_add(1);
                        (__p5).write(__t6);
                        __t6
                    }) as i32)
                        > 20i32
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(0i16);
                        let __p7 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                UpdateHeartSliders();
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_WaitForHeartSliders));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForHeartSliders(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (SlidersDoneUpdating()) != 0 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_RestorePlttBufferUnfaded));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_RestorePlttBufferUnfaded(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        {
            let mut _src: *mut u8 = (((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106500))
                .wrapping_add(512))
            .cast::<u16>())
            .cast::<u8>();
            let mut _dest: *mut u8 =
                (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).cast::<u8>();
            let mut _size: u32 = 1024u32;
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    ((_dest) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (2214592512u32
                                        | crate::c::div_u32(
                                            _size,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
                                        )),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(2i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_WaitPrintRoundResult));
    }
}
pub(crate) unsafe extern "C" fn Task_WaitPrintRoundResult(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 2i32
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            if (({
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                let __t4 = ((__p3).read()).wrapping_sub(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 0i32
            {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_PrintRoundResultText));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintRoundResultText(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            let mut attention: u8 =
                (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 28,
                ))
                .wrapping_add(26))
                .read();
            ContestClearGeneralTextWindow();
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 64,
                ))
                .wrapping_add(2))
                .cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                ((((&raw const sRoundResultTexts)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(((attention) as i32) as isize))
                .read(),
            );
            Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if !((Contest_RunTextPrinters()) != 0) {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ReUpdateHeartSliders));
                ContestDebugDoPrint();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReUpdateHeartSliders(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            > 29i32
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            UpdateHeartSliders();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_WaitForHeartSlidersAgain));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForHeartSlidersAgain(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (SlidersDoneUpdating()) != 0 {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_DropCurtainAtRoundEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DropCurtainAtRoundEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetBgForCurtainDrop();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_StartDropCurtainAtRoundEnd));
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateContestantBoxOrder(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        UpdateContestantBoxOrder();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_TryStartNextRoundOfAppeals));
    }
}
pub(crate) unsafe extern "C" fn Task_TryStartNextRoundOfAppeals(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut sp0: u16 = 0u16;
        (&raw mut sp0).write_volatile(GetGpuReg(8u8));
        let mut sp2: u16 = 0u16;
        (&raw mut sp2).write_volatile(GetGpuReg(12u8));
        crate::c::bf_write(
            ((&raw mut sp0).cast::<u8>()).wrapping_add(0),
            0,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut sp2).cast::<u8>()).wrapping_add(0),
            0,
            2,
            (0u16) as i32,
        );
        SetGpuReg(8u8, (&raw mut sp0).read_volatile());
        SetGpuReg(12u8, (&raw mut sp2).read_volatile());
        let __p1 = (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1))
        .read()) as i32)
            == 5i32
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_EndAppeals));
        } else {
            SlideApplauseMeterIn();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_StartNewRoundOfAppeals));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartNewRoundOfAppeals(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(6),
            6,
            1,
            false,
        ) as u16)
            != 0)
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_DisplayAppealNumberText));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_EndAppeals(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gContestMonAppealPointTotals)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .wrapping_offset((i) as isize))
                    .write(
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(4)
                        .cast::<i16>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CalculateFinalScores();
        ContestClearGeneralTextWindow();
        if !((((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32)
            != 0)
        {
            BravoTrainerPokemonProfile_BeforeInterview1(
                (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 28,
                ))
                .wrapping_add(8)
                .cast::<u16>())
                .read(),
            );
        } else {
            CalculateContestLiveUpdateData();
            SetConestLiveUpdateTVData();
            ContestDebugPrintBitStrings();
        }
        ((&raw mut gContestRngValue).cast::<u8>().cast::<u32>())
            .write(((&raw mut gRngValue).cast::<u32>()).read());
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_AllOutOfAppealTime).cast::<u8>(),
        );
        Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_WaitForOutOfTimeMsg));
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForOutOfTimeMsg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((Contest_RunTextPrinters()) != 0) {
            SetBgForCurtainDrop();
            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(160u16);
            PlaySE12WithPanning(98u16, 0i8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_DropCurtainAtAppealsEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DropCurtainAtAppealsEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = (&raw mut gBattle_BG1_Y).cast::<u16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(7i32)) as u16));
        if (((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i16) as i32) < 0i32 {
            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        }
        if ((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i32) == 0i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_TryCommunicateFinalStandings));
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TryCommunicateFinalStandings(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            >= 50i32
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32)
                != 0
            {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_CommunicateFinalStandings));
            } else {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ContestReturnToField));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CommunicateFinalStandings(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut taskId2: u8 = CreateTask(Some(Task_LinkContest_CommunicateFinalStandings), 0u8);
        SetTaskFuncWithFollowupFunc(
            taskId2,
            Some(Task_LinkContest_CommunicateFinalStandings),
            Some(Task_EndCommunicateFinalStandings),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(TaskDummy1));
        ContestPrintLinkStandby();
        SetBottomSliderHeartsInvisibility(0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_EndCommunicateFinalStandings(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyTask(taskId);
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(8))
            .read()) as i32) as isize
                * 40,
        ))
        .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ContestReturnToField));
    }
}
pub(crate) unsafe extern "C" fn Task_ContestReturnToField(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyTask(taskId);
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCB_ContestReturnToField));
            FreeAllWindowBuffers();
            FreeContestResources();
            FreeMonSpritesGfx();
            SetMainCallback2(Some(CB2_ReturnToField));
        }
    }
}
pub(crate) unsafe extern "C" fn FieldCB_ContestReturnToField() {
    unsafe {
        UnlockPlayerFieldControls();
        ScriptContext_Enable();
    }
}
pub(crate) unsafe extern "C" fn TryPutPlayerLast() {
    unsafe {
        if !((((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32)
            != 0)
        {
            ((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).write(3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn IsPlayerLinkLeader() -> u8 {
    unsafe {
        if ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
            == ((((&raw mut gContestLinkLeaderIndex).cast::<u8>().cast::<u8>()).read()) as i32)
        {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateContestMonFromParty(partyIndex: u8) {
    unsafe {
        let mut partyIndex = partyIndex;
        let mut name = crate::ffi::Align4([0u8; 20]);
        let mut heldItem: u16 = 0u16;
        let mut cool: i16 = 0i16;
        let mut beauty: i16 = 0i16;
        let mut cute: i16 = 0i16;
        let mut smart: i16 = 0i16;
        let mut tough: i16 = 0i16;
        StringCopy(
            (&raw mut name).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32) != 0
        {
            StripPlayerNameForLinkContest((&raw mut name).cast::<u8>());
        }
        crate::c::memcpy(
            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize
                    * 64,
            ))
            .wrapping_add(13))
            .cast::<u8>(),
            (&raw mut name).cast::<u8>(),
            8u32,
        );
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            == 0i32
        {
            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize
                    * 64,
            ))
            .wrapping_add(21))
            .write(216u8);
        } else {
            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize
                    * 64,
            ))
            .wrapping_add(21))
            .write(217u8);
        }
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(24)
        .cast::<u32>())
        .write(0u32);
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(44))
        .write(0u8);
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .cast::<u16>())
        .write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                11i32,
            )) as u16),
        );
        GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            2i32,
            (&raw mut name).cast::<u8>(),
        );
        StringGet_Nickname((&raw mut name).cast::<u8>());
        if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32) != 0
        {
            StripMonNameForLinkContest(
                (&raw mut name).cast::<u8>(),
                ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((partyIndex) as i32) as isize * 100),
                    3i32,
                )) as i32),
            );
        }
        crate::c::memcpy(
            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize
                    * 64,
            ))
            .wrapping_add(2))
            .cast::<u8>(),
            (&raw mut name).cast::<u8>(),
            11u32,
        );
        StringCopy(
            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize
                    * 64,
            ))
            .wrapping_add(2))
            .cast::<u8>(),
            (&raw mut name).cast::<u8>(),
        );
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(38))
        .write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                22i32,
            )) as u8),
        );
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(39))
        .write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                23i32,
            )) as u8),
        );
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(40))
        .write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                24i32,
            )) as u8),
        );
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(41))
        .write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                33i32,
            )) as u8),
        );
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(42))
        .write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                47i32,
            )) as u8),
        );
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(43))
        .write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                48i32,
            )) as u8),
        );
        ((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                13i32,
            )) as u16),
        );
        (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(1))
        .write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                14i32,
            )) as u16),
        );
        (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(2))
        .write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                15i32,
            )) as u16),
        );
        (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(30))
        .cast::<u16>())
        .wrapping_offset(3))
        .write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                16i32,
            )) as u16),
        );
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(56)
        .cast::<u32>())
        .write(GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            0i32,
        ));
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(60)
        .cast::<u32>())
        .write(GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            1i32,
        ));
        heldItem = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            12i32,
        )) as u16);
        cool = (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(38))
        .read()) as i16);
        beauty = (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(39))
        .read()) as i16);
        cute = (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(40))
        .read()) as i16);
        smart = (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(41))
        .read()) as i16);
        tough = (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(42))
        .read()) as i16);
        if ((heldItem) as i32) == 254i32 {
            cool = ((((cool) as i32).wrapping_add(20i32)) as i16);
        } else {
            if ((heldItem) as i32) == 255i32 {
                beauty = ((((beauty) as i32).wrapping_add(20i32)) as i16);
            } else {
                if ((heldItem) as i32) == 256i32 {
                    cute = ((((cute) as i32).wrapping_add(20i32)) as i16);
                } else {
                    if ((heldItem) as i32) == 257i32 {
                        smart = ((((smart) as i32).wrapping_add(20i32)) as i16);
                    } else {
                        if ((heldItem) as i32) == 258i32 {
                            tough = ((((tough) as i32).wrapping_add(20i32)) as i16);
                        }
                    }
                }
            }
        }
        if ((cool) as i32) > 255i32 {
            cool = 255i16;
        }
        if ((beauty) as i32) > 255i32 {
            beauty = 255i16;
        }
        if ((cute) as i32) > 255i32 {
            cute = 255i16;
        }
        if ((smart) as i32) > 255i32 {
            smart = 255i16;
        }
        if ((tough) as i32) > 255i32 {
            tough = 255i16;
        }
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(38))
        .write(((cool) as u8));
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(39))
        .write(((beauty) as u8));
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(40))
        .write(((cute) as u8));
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(41))
        .write(((smart) as u8));
        (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 64,
        ))
        .wrapping_add(42))
        .write(((tough) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContestants(contestType: u8, rank: u8) {
    unsafe {
        let mut contestType = contestType;
        let mut rank = rank;
        let mut i: i32 = 0i32;
        let mut opponentsCount: u8 = 0u8;
        let mut opponents = crate::ffi::Align4([0u8; 100]);
        let mut allowPostgameContestants: u8 = 0u8;
        let mut filter: *mut u8 = core::ptr::null_mut();
        TryPutPlayerLast();
        if ((FlagGet(2148u16)) != 0)
            && (!((((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32)
                & 1i32)
                != 0))
        {
            allowPostgameContestants = 1u8;
        }
        filter = ((&raw const gPostgameContestOpponentFilter)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>();
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(6144u32, 64u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((rank) as i32)
                        == ((crate::c::bf_read(
                            ((((&raw const gContestOpponents).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 64))
                            .wrapping_add(28),
                            0,
                            2,
                            false,
                        ) as u8) as i32)
                    {
                        if ((allowPostgameContestants) as i32) == 1i32 {
                            if ((((filter).wrapping_offset((i) as isize)).read()) as i32) == 1i32 {
                                break 'l2;
                            }
                        } else {
                            if ((((filter).wrapping_offset((i) as isize)).read()) as i32) == 2i32 {
                                break 'l2;
                            }
                        }
                        if (((contestType) as i32) == 0i32)
                            && ((crate::c::bf_read(
                                ((((&raw const gContestOpponents).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 64))
                                .wrapping_add(28),
                                2,
                                1,
                                false,
                            ) as u8)
                                != 0)
                        {
                            (((&raw mut opponents).cast::<u8>()).wrapping_offset(
                                (({
                                    let __t1 = opponentsCount;
                                    opponentsCount = (opponentsCount).wrapping_add(1);
                                    __t1
                                }) as i32) as isize,
                            ))
                            .write(((i) as u8));
                        } else {
                            if (((contestType) as i32) == 1i32)
                                && ((crate::c::bf_read(
                                    ((((&raw const gContestOpponents).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize * 64))
                                    .wrapping_add(28),
                                    3,
                                    1,
                                    false,
                                ) as u8)
                                    != 0)
                            {
                                (((&raw mut opponents).cast::<u8>()).wrapping_offset(
                                    (({
                                        let __t2 = opponentsCount;
                                        opponentsCount = (opponentsCount).wrapping_add(1);
                                        __t2
                                    }) as i32) as isize,
                                ))
                                .write(((i) as u8));
                            } else {
                                if (((contestType) as i32) == 2i32)
                                    && ((crate::c::bf_read(
                                        ((((&raw const gContestOpponents)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 64))
                                        .wrapping_add(28),
                                        4,
                                        1,
                                        false,
                                    ) as u8)
                                        != 0)
                                {
                                    (((&raw mut opponents).cast::<u8>()).wrapping_offset(
                                        (({
                                            let __t3 = opponentsCount;
                                            opponentsCount = (opponentsCount).wrapping_add(1);
                                            __t3
                                        }) as i32) as isize,
                                    ))
                                    .write(((i) as u8));
                                } else {
                                    if (((contestType) as i32) == 3i32)
                                        && ((crate::c::bf_read(
                                            ((((&raw const gContestOpponents)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 64))
                                            .wrapping_add(28),
                                            5,
                                            1,
                                            false,
                                        ) as u8)
                                            != 0)
                                    {
                                        (((&raw mut opponents).cast::<u8>()).wrapping_offset(
                                            (({
                                                let __t4 = opponentsCount;
                                                opponentsCount = (opponentsCount).wrapping_add(1);
                                                __t4
                                            }) as i32)
                                                as isize,
                                        ))
                                        .write(((i) as u8));
                                    } else {
                                        if (((contestType) as i32) == 4i32)
                                            && ((crate::c::bf_read(
                                                ((((&raw const gContestOpponents)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize * 64))
                                                .wrapping_add(28),
                                                6,
                                                1,
                                                false,
                                            )
                                                as u8)
                                                != 0)
                                        {
                                            (((&raw mut opponents).cast::<u8>()).wrapping_offset(
                                                (({
                                                    let __t5 = opponentsCount;
                                                    opponentsCount =
                                                        (opponentsCount).wrapping_add(1);
                                                    __t5
                                                })
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(((i) as u8));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut opponents).cast::<u8>()).wrapping_offset(((opponentsCount) as i32) as isize))
            .write(255u8);
        {
            i = 0i32;
            'l3: loop {
                if !(i < 3i32) {
                    break 'l3;
                }
                'l4: {
                    let mut rnd: u16 =
                        ((crate::c::rem_i32(((Random()) as i32), ((opponentsCount) as i32)))
                            as u16);
                    let mut j: i32 = 0i32;
                    (((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 64)
                        .cast::<crate::c::Rec4<64>>()
                        .write_unaligned(
                            (((&raw const gContestOpponents).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut opponents).cast::<u8>())
                                        .wrapping_offset(((rnd) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 64,
                                )
                                .cast::<crate::c::Rec4<64>>()
                                .read_unaligned(),
                        );
                    {
                        j = ((rnd) as i32);
                        'l5: loop {
                            if !((((((&raw mut opponents).cast::<u8>())
                                .wrapping_offset((j) as isize))
                            .read()) as i32)
                                != 255i32)
                            {
                                break 'l5;
                            }
                            'l6: {
                                (((&raw mut opponents).cast::<u8>()).wrapping_offset((j) as isize))
                                    .write(
                                        (((&raw mut opponents).cast::<u8>())
                                            .wrapping_offset(((j).wrapping_add(1i32)) as isize))
                                        .read(),
                                    );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    opponentsCount = (opponentsCount).wrapping_sub(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        CreateContestMonFromParty(
            ((&raw mut gContestMonPartyIndex).cast::<u8>().cast::<u8>()).read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLinkAIContestants(contestType: u8, rank: u8, isPostgame: u32) {
    unsafe {
        let mut contestType = contestType;
        let mut rank = rank;
        let mut isPostgame = isPostgame;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut opponentsCount: u8 = 0u8;
        let mut opponents = crate::ffi::Align4([0u8; 100]);
        if ((((&raw mut gNumLinkContestPlayers).cast::<u8>().cast::<u8>()).read()) as i32) == 4i32 {
            return;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(6144u32, 64u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((rank) as i32)
                        != ((crate::c::bf_read(
                            ((((&raw const gContestOpponents).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 64))
                            .wrapping_add(28),
                            0,
                            2,
                            false,
                        ) as u8) as i32)
                    {
                        break 'l2;
                    }
                    if isPostgame == 1u32 {
                        if ((((((&raw const gPostgameContestOpponentFilter)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            == 1i32
                        {
                            break 'l2;
                        }
                    } else {
                        if ((((((&raw const gPostgameContestOpponentFilter)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            == 2i32
                        {
                            break 'l2;
                        }
                    }
                    if (((((((contestType) as i32) == 0i32)
                        && ((crate::c::bf_read(
                            ((((&raw const gContestOpponents).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 64))
                            .wrapping_add(28),
                            2,
                            1,
                            false,
                        ) as u8)
                            != 0))
                        || ((((contestType) as i32) == 1i32)
                            && ((crate::c::bf_read(
                                ((((&raw const gContestOpponents).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 64))
                                .wrapping_add(28),
                                3,
                                1,
                                false,
                            ) as u8)
                                != 0)))
                        || ((((contestType) as i32) == 2i32)
                            && ((crate::c::bf_read(
                                ((((&raw const gContestOpponents).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 64))
                                .wrapping_add(28),
                                4,
                                1,
                                false,
                            ) as u8)
                                != 0)))
                        || ((((contestType) as i32) == 3i32)
                            && ((crate::c::bf_read(
                                ((((&raw const gContestOpponents).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 64))
                                .wrapping_add(28),
                                5,
                                1,
                                false,
                            ) as u8)
                                != 0)))
                        || ((((contestType) as i32) == 4i32)
                            && ((crate::c::bf_read(
                                ((((&raw const gContestOpponents).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 64))
                                .wrapping_add(28),
                                6,
                                1,
                                false,
                            ) as u8)
                                != 0))
                    {
                        (((&raw mut opponents).cast::<u8>()).wrapping_offset(
                            (({
                                let __t1 = opponentsCount;
                                opponentsCount = (opponentsCount).wrapping_add(1);
                                __t1
                            }) as i32) as isize,
                        ))
                        .write(((i) as u8));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut opponents).cast::<u8>()).wrapping_offset(((opponentsCount) as i32) as isize))
            .write(255u8);
        {
            i = 0i32;
            'l3: loop {
                if !(i
                    < (4i32).wrapping_sub(
                        ((((&raw mut gNumLinkContestPlayers).cast::<u8>().cast::<u8>()).read())
                            as i32),
                    ))
                {
                    break 'l3;
                }
                'l4: {
                    let mut rnd: u16 = ((crate::c::rem_i32(
                        ((GetContestRand()) as i32),
                        ((opponentsCount) as i32),
                    )) as u16);
                    (((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset((((((((&raw mut gNumLinkContestPlayers).cast::<u8>().cast::<u8>()).read()) as i32))).wrapping_add(i)) as isize * 64).cast::<crate::c::Rec4<64>>().write_unaligned((((&raw const gContestOpponents).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(((((((&raw mut opponents).cast::<u8>()).wrapping_offset((((rnd) as i32)) as isize)).read()) as i32)) as isize * 64).cast::<crate::c::Rec4<64>>().read_unaligned());
                    StripPlayerNameForLinkContest((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset((((((((&raw mut gNumLinkContestPlayers).cast::<u8>().cast::<u8>()).read()) as i32))).wrapping_add(i)) as isize * 64)).wrapping_add(13)).cast::<u8>());
                    StripMonNameForLinkContest((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>()).wrapping_offset((((((((&raw mut gNumLinkContestPlayers).cast::<u8>().cast::<u8>()).read()) as i32))).wrapping_add(i)) as isize * 64)).wrapping_add(2)).cast::<u8>(), 2i32);
                    {
                        j = ((rnd) as i32);
                        'l5: loop {
                            if !((((((&raw mut opponents).cast::<u8>())
                                .wrapping_offset((j) as isize))
                            .read()) as i32)
                                != 255i32)
                            {
                                break 'l5;
                            }
                            'l6: {
                                (((&raw mut opponents).cast::<u8>()).wrapping_offset((j) as isize))
                                    .write(
                                        (((&raw mut opponents).cast::<u8>())
                                            .wrapping_offset(((j).wrapping_add(1i32)) as isize))
                                        .read(),
                                    );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    opponentsCount = (opponentsCount).wrapping_sub(1);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestEntryEligibility(pkmn: *mut u8) -> u8 {
    unsafe {
        let mut pkmn = pkmn;
        let mut ribbon: u8 = 0u8;
        let mut eligibility: u8 = 0u8;
        if (GetMonData2(pkmn, 45i32)) != 0 {
            return 3u8;
        }
        if GetMonData2(pkmn, 57i32) == 0u32 {
            return 4u8;
        }
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_ContestCategory)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                ribbon = ((GetMonData2(pkmn, 50i32)) as u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ribbon = ((GetMonData2(pkmn, 51i32)) as u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ribbon = ((GetMonData2(pkmn, 52i32)) as u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ribbon = ((GetMonData2(pkmn, 53i32)) as u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ribbon = ((GetMonData2(pkmn, 54i32)) as u8);
                break 'l1;
            }
            if !__matched {
                return 0u8;
            }
        }
        if ((ribbon) as i32)
            > ((((&raw mut gSpecialVar_ContestRank)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
        {
            eligibility = 2u8;
        } else {
            if ((ribbon) as i32)
                >= ((((&raw mut gSpecialVar_ContestRank)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as i32)
            {
                eligibility = 1u8;
            } else {
                eligibility = 0u8;
            }
        }
        return eligibility;
    }
}
pub(crate) unsafe extern "C" fn DrawContestantWindowText() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    FillWindowPixelBuffer(
                        ((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read(),
                        0u8,
                    );
                    PrintContestantTrainerName(((i) as u8));
                    PrintContestantMonName(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Contest_CopyStringWithColor(string: *mut u8, color: u8) -> *mut u8 {
    unsafe {
        let mut string = string;
        let mut color = color;
        let mut ptr: *mut u8 = StringCopy(
            (&raw mut gDisplayedStringBattle).cast::<u8>(),
            (&raw mut gText_ColorTransparent).cast::<u8>(),
        );
        ((ptr).wrapping_offset(-1)).write(color);
        ptr = StringCopy(ptr, string);
        return ptr;
    }
}
pub(crate) unsafe extern "C" fn PrintContestantTrainerName(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        PrintContestantTrainerNameWithColor(
            contestant,
            ((((contestant) as i32).wrapping_add(10i32)) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintContestantTrainerNameWithColor(contestant: u8, color: u8) {
    unsafe {
        let mut contestant = contestant;
        let mut color = color;
        let mut buffer = crate::ffi::Align4([0u8; 32]);
        let mut offset: i32 = 0i32;
        StringCopy(
            (&raw mut buffer).cast::<u8>(),
            (&raw mut gText_Slash).cast::<u8>(),
        );
        StringAppend(
            (&raw mut buffer).cast::<u8>(),
            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((contestant) as i32) as isize * 64))
            .wrapping_add(13))
            .cast::<u8>(),
        );
        Contest_CopyStringWithColor((&raw mut buffer).cast::<u8>(), color);
        offset =
            GetStringRightAlignXOffset(7i32, (&raw mut gDisplayedStringBattle).cast::<u8>(), 96i32);
        if offset > 55i32 {
            offset = 55i32;
        }
        Contest_PrintTextToBg0WindowAt(
            ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((contestant) as i32) as isize))
            .read()) as u32),
            (&raw mut gDisplayedStringBattle).cast::<u8>(),
            offset,
            1i32,
            7i32,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintContestantMonName(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        PrintContestantMonNameWithColor(
            contestant,
            ((((contestant) as i32).wrapping_add(10i32)) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintContestantMonNameWithColor(contestant: u8, color: u8) {
    unsafe {
        let mut contestant = contestant;
        let mut color = color;
        Contest_CopyStringWithColor(
            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((contestant) as i32) as isize * 64))
            .wrapping_add(2))
            .cast::<u8>(),
            color,
        );
        Contest_PrintTextToBg0WindowAt(
            ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((contestant) as i32) as isize))
            .read()) as u32),
            (&raw mut gDisplayedStringBattle).cast::<u8>(),
            5i32,
            1i32,
            7i32,
        );
    }
}
pub(crate) unsafe extern "C" fn CalculateContestantRound1Points(
    who: u8,
    contestCategory: u8,
) -> u16 {
    unsafe {
        let mut who = who;
        let mut contestCategory = contestCategory;
        let mut statMain: u8 = 0u8;
        let mut statSub1: u8 = 0u8;
        let mut statSub2: u8 = 0u8;
        'l1: {
            let __sw1 = ((contestCategory) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                statMain = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(38))
                .read();
                statSub1 = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(42))
                .read();
                statSub2 = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(39))
                .read();
                break 'l1;
            }
            if __sw1 == 1i32 {
                statMain = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(39))
                .read();
                statSub1 = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(38))
                .read();
                statSub2 = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(40))
                .read();
                break 'l1;
            }
            if __sw1 == 2i32 {
                statMain = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(40))
                .read();
                statSub1 = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(39))
                .read();
                statSub2 = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(41))
                .read();
                break 'l1;
            }
            if __sw1 == 3i32 {
                statMain = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(41))
                .read();
                statSub1 = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(40))
                .read();
                statSub2 = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(42))
                .read();
                break 'l1;
            }
            if __sw1 == 4i32 || !__matched {
                statMain = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(42))
                .read();
                statSub1 = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(41))
                .read();
                statSub2 = (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(38))
                .read();
                break 'l1;
            }
        }
        return ((((statMain) as i32).wrapping_add(crate::c::div_i32(
            (((statSub1) as i32).wrapping_add(((statSub2) as i32))).wrapping_add(
                (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize * 64))
                .wrapping_add(43))
                .read()) as i32),
            ),
            2i32,
        ))) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculateRound1Points(contestCategory: u8) {
    unsafe {
        let mut contestCategory = contestCategory;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gContestMonRound1Points)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((CalculateContestantRound1Points(((i) as u8), contestCategory)) as i16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateJudgeSprite() -> u8 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        LoadCompressedSpriteSheet((&raw const sSpriteSheet_Judge).cast::<u8>().cast_mut());
        LoadCompressedPalette(
            ((&raw mut gContest2Pal).cast::<u32>()).cast::<u32>(),
            272u16,
            32u16,
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_Judge).cast::<u8>().cast_mut(),
            112i16,
            36i16,
            30u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            4,
            4,
            (1u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn CreateJudgeSpeechBubbleSprite() -> u8 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_JudgeSymbols)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpritePalette(
            (&raw const sSpritePalette_JudgeSymbols)
                .cast::<u8>()
                .cast_mut(),
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_JudgeSpeechBubble)
                .cast::<u8>()
                .cast_mut(),
            96i16,
            10i16,
            29u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(4),
                0,
                10,
                false,
            ) as u16) as i16),
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn CreateContestantSprite(
    species: u16,
    otId: u32,
    personality: u32,
    index: u32,
) -> u8 {
    unsafe {
        let mut species = species;
        let mut otId = otId;
        let mut personality = personality;
        let mut index = index;
        let mut spriteId: u8 = 0u8;
        species = SanitizeSpecies(species);
        if index == ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as u32)
        {
            HandleLoadSpecialPokePic_2(
                ((&raw mut gMonBackPicTable).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 8),
                (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<*mut u8>())
                .read(),
                ((species) as i32),
                personality,
            );
        } else {
            HandleLoadSpecialPokePic_DontHandleDeoxys(
                ((&raw mut gMonBackPicTable).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 8),
                (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<*mut u8>())
                .read(),
                ((species) as i32),
                personality,
            );
        }
        LoadCompressedPalette(
            GetMonSpritePalFromSpeciesAndPersonality(species, otId, personality),
            288u16,
            32u16,
        );
        SetMultiuseSpriteTemplateToPokemon(species, 0u8);
        spriteId = CreateSprite(
            (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
            112i16,
            ((GetBattlerSpriteFinal_Y(2u8, species, 0u8)) as i16),
            30u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            4,
            4,
            (2u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (2u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(67))
        .write(GetBattlerSpriteSubpriority(2u8));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                4,
                4,
                false,
            ) as u16) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((species) as i16));
        if (IsSpeciesNotUnown(species)) != 0 {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
            .write(
                ((&raw mut gAffineAnims_BattleSpriteContest).cast::<*mut u8>()).cast::<*mut u8>(),
            );
        } else {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
            .write(
                ((&raw mut gAffineAnims_BattleSpriteOpponentSide).cast::<*mut u8>())
                    .cast::<*mut u8>(),
            );
        }
        StartSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            0u8,
        );
        return spriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSpeciesNotUnown(species: u16) -> u8 {
    unsafe {
        let mut species = species;
        if ((species) as i32) == 201i32 {
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SwapMoveDescAndContestTilemaps() {
    unsafe {
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(36))
                            .cast::<*mut u8>())
                            .read(),
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(36))
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(1280),
                            ((0i32
                                | (crate::c::div_i32(640i32, crate::c::div_i32(16i32, 8i32))
                                    & 2097151i32)) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(36))
                            .cast::<*mut u8>())
                            .wrapping_offset(2))
                            .read(),
                            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(36))
                            .cast::<*mut u8>())
                            .wrapping_offset(2))
                            .read())
                            .wrapping_offset(1280),
                            ((0i32
                                | (crate::c::div_i32(640i32, crate::c::div_i32(16i32, 8i32))
                                    & 2097151i32)) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l7;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetMoveEffectSymbolTileOffset(r#move: u16, contestant: u8) -> u16 {
    unsafe {
        let mut r#move = r#move;
        let mut contestant = contestant;
        let mut offset: u16 = 0u16;
        'l1: {
            let __sw1 = (((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
                (((((&raw mut gContestMoves).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 8))
                .read()) as i32) as isize
                    * 4,
            ))
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 8i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 8i32 {
                offset = 36994u16;
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 3i32 {
                offset = 37000u16;
                break 'l1;
            }
            if !__matched {
                offset = 36998u16;
                break 'l1;
            }
        }
        offset = ((((offset) as i32)
            .wrapping_add((36864i32).wrapping_add((((contestant) as i32) << 12))))
            as u16);
        return offset;
    }
}
pub(crate) unsafe extern "C" fn PrintContestMoveDescription(r#move: u16) {
    unsafe {
        let mut r#move = r#move;
        let mut category: u8 = 0u8;
        let mut categoryTile: u16 = 0u16;
        let mut numHearts: u8 = 0u8;
        category = (crate::c::bf_read(
            (((&raw mut gContestMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 8))
            .wrapping_add(1),
            0,
            3,
            false,
        ) as u8);
        if ((category) as i32) == 0i32 {
            categoryTile = 16448u16;
        } else {
            if ((category) as i32) == 1i32 {
                categoryTile = 16453u16;
            } else {
                if ((category) as i32) == 2i32 {
                    categoryTile = 16458u16;
                } else {
                    if ((category) as i32) == 3i32 {
                        categoryTile = 16490u16;
                    } else {
                        categoryTile = 16522u16;
                    }
                }
            }
        }
        ContestBG_FillBoxWithIncrementingTile(0u8, categoryTile, 11u8, 31u8, 5u8, 1u8, 17u8, 1i16);
        ContestBG_FillBoxWithIncrementingTile(
            0u8,
            ((((categoryTile) as i32).wrapping_add(16i32)) as u16),
            11u8,
            32u8,
            5u8,
            1u8,
            17u8,
            1i16,
        );
        if ((((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
            (((((&raw mut gContestMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 8))
            .read()) as i32) as isize
                * 4,
        ))
        .wrapping_add(1))
        .read()) as i32)
            == 255i32
        {
            numHearts = 0u8;
        } else {
            numHearts = ((crate::c::div_i32(
                ((((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gContestMoves).cast::<u8>())
                        .wrapping_offset(((r#move) as i32) as isize * 8))
                    .read()) as i32) as isize
                        * 4,
                ))
                .wrapping_add(1))
                .read()) as i32),
                10i32,
            )) as u8);
        }
        if ((numHearts) as i32) > 8i32 {
            numHearts = 8u8;
        }
        ContestBG_FillBoxWithTile(0u8, 20533u16, 21u8, 31u8, 8u8, 1u8, 17u8);
        ContestBG_FillBoxWithTile(0u8, 20498u16, 21u8, 31u8, numHearts, 1u8, 17u8);
        if ((((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
            (((((&raw mut gContestMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 8))
            .read()) as i32) as isize
                * 4,
        ))
        .wrapping_add(2))
        .read()) as i32)
            == 255i32
        {
            numHearts = 0u8;
        } else {
            numHearts = ((crate::c::div_i32(
                ((((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gContestMoves).cast::<u8>())
                        .wrapping_offset(((r#move) as i32) as isize * 8))
                    .read()) as i32) as isize
                        * 4,
                ))
                .wrapping_add(2))
                .read()) as i32),
                10i32,
            )) as u8);
        }
        if ((numHearts) as i32) > 8i32 {
            numHearts = 8u8;
        }
        ContestBG_FillBoxWithTile(0u8, 20534u16, 21u8, 32u8, 8u8, 1u8, 17u8);
        ContestBG_FillBoxWithTile(0u8, 20500u16, 21u8, 32u8, numHearts, 1u8, 17u8);
        FillWindowPixelBuffer(10u8, 0u8);
        Contest_PrintTextToBg0WindowStd(
            10u32,
            ((((&raw const gContestEffectDescriptionPointers)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                (((((&raw mut gContestMoves).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 8))
                .read()) as i32) as isize,
            ))
            .read(),
        );
        Contest_PrintTextToBg0WindowStd(9u32, (&raw mut gText_Slash).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn DrawMoveEffectSymbol(r#move: u16, contestant: u8) {
    unsafe {
        let mut r#move = r#move;
        let mut contestant = contestant;
        let mut contestantOffset: u8 = (((((((((&raw mut gContestantTurnOrder).cast::<u8>())
            .cast::<u8>())
        .wrapping_offset(((contestant) as i32) as isize))
        .read()) as i32)
            .wrapping_mul(5i32))
        .wrapping_add(2i32)) as u8);
        if (!((Contest_IsMonsTurnDisabled(contestant)) != 0)) && (((r#move) as i32) != 0i32) {
            let mut tile: u16 = GetMoveEffectSymbolTileOffset(r#move, contestant);
            ContestBG_FillBoxWithIncrementingTile(
                0u8,
                tile,
                20u8,
                contestantOffset,
                2u8,
                1u8,
                17u8,
                1i16,
            );
            ContestBG_FillBoxWithIncrementingTile(
                0u8,
                ((((tile) as i32).wrapping_add(16i32)) as u16),
                20u8,
                ((((contestantOffset) as i32).wrapping_add(1i32)) as u8),
                2u8,
                1u8,
                17u8,
                1i16,
            );
        } else {
            ContestBG_FillBoxWithTile(0u8, 0u16, 20u8, contestantOffset, 2u8, 2u8, 17u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DrawMoveEffectSymbols() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    DrawMoveEffectSymbol(
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read(),
                        ((i) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetStarTileOffset() -> u16 {
    unsafe {
        return 8244u16;
    }
}
pub(crate) unsafe extern "C" fn UpdateConditionStars(contestantIdx: u8, resetMod: u8) -> u8 {
    unsafe {
        let mut contestantIdx = contestantIdx;
        let mut resetMod = resetMod;
        let mut contestantOffset: u8 = 0u8;
        let mut numStars: i32 = 0i32;
        if ((crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestantIdx) as i32) as isize * 28))
            .wrapping_add(16),
            4,
            2,
            false,
        ) as u8) as i32)
            == 0i32
        {
            return 0u8;
        }
        contestantOffset = (((((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((contestantIdx) as i32) as isize))
        .read()) as i32)
            .wrapping_mul(5i32))
        .wrapping_add(2i32)) as u8);
        numStars = crate::c::div_i32(
            (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestantIdx) as i32) as isize * 28))
            .wrapping_add(13)
            .cast::<i8>())
            .read()) as i32),
            10i32,
        );
        if ((crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestantIdx) as i32) as isize * 28))
            .wrapping_add(16),
            4,
            2,
            false,
        ) as u8) as i32)
            == 1i32
        {
            ContestBG_FillBoxWithTile(
                0u8,
                GetStarTileOffset(),
                19u8,
                contestantOffset,
                1u8,
                ((numStars) as u8),
                17u8,
            );
            if (resetMod) != 0 {
                PlaySE(91u16);
                crate::c::bf_write(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestantIdx) as i32) as isize * 28))
                    .wrapping_add(16),
                    4,
                    2,
                    (0u8) as i32,
                );
            }
        } else {
            ContestBG_FillBoxWithTile(
                0u8,
                0u16,
                19u8,
                ((((contestantOffset) as i32).wrapping_add(numStars)) as u8),
                1u8,
                (((3i32).wrapping_sub(numStars)) as u8),
                17u8,
            );
            if (resetMod) != 0 {
                PlaySE(38u16);
                crate::c::bf_write(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestantIdx) as i32) as isize * 28))
                    .wrapping_add(16),
                    4,
                    2,
                    (0u8) as i32,
                );
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn DrawConditionStars() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut numStars: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut contestantOffset: u8 =
                        (((((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            .wrapping_mul(5i32))
                        .wrapping_add(2i32)) as u8);
                    let mut starOffset: u16 = GetStarTileOffset();
                    numStars = crate::c::div_i32(
                        (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(13)
                        .cast::<i8>())
                        .read()) as i32),
                        10i32,
                    );
                    ContestBG_FillBoxWithTile(
                        0u8,
                        starOffset,
                        19u8,
                        contestantOffset,
                        1u8,
                        ((numStars) as u8),
                        17u8,
                    );
                    ContestBG_FillBoxWithTile(
                        0u8,
                        0u16,
                        19u8,
                        ((((contestantOffset) as i32).wrapping_add(numStars)) as u8),
                        1u8,
                        (((3i32).wrapping_sub(numStars)) as u8),
                        17u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetStatusSymbolTileOffset(status: u8) -> u16 {
    unsafe {
        let mut status = status;
        let mut offset: u16 = 0u16;
        'l1: {
            let __sw1 = ((status) as i32);
            if __sw1 == 0i32 {
                offset = 128u16;
                break 'l1;
            }
            if __sw1 == 1i32 {
                offset = 132u16;
                break 'l1;
            }
            if __sw1 == 2i32 {
                offset = 134u16;
                break 'l1;
            }
            if __sw1 == 3i32 {
                offset = 136u16;
                break 'l1;
            }
            if __sw1 == 4i32 {
                offset = 130u16;
                break 'l1;
            }
        }
        offset = ((((offset) as i32).wrapping_add(36864i32)) as u16);
        return offset;
    }
}
pub(crate) unsafe extern "C" fn DrawStatusSymbol(contestant: u8) -> u8 {
    unsafe {
        let mut contestant = contestant;
        let mut statused: u8 = 1u8;
        let mut symbolOffset: u16 = 0u16;
        let mut contestantOffset: u8 = (((((((((&raw mut gContestantTurnOrder).cast::<u8>())
            .cast::<u8>())
        .wrapping_offset(((contestant) as i32) as isize))
        .read()) as i32)
            .wrapping_mul(5i32))
        .wrapping_add(2i32)) as u8);
        if ((((crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(16),
            0,
            1,
            false,
        ) as u8)
            != 0)
            || ((crate::c::bf_read(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(16),
                1,
                1,
                false,
            ) as u8)
                != 0))
            || ((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(18))
            .read()) as i32)
                != 0i32))
            || ((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(15))
            .read()) as i32)
                != 0i32)
        {
            symbolOffset = GetStatusSymbolTileOffset(0u8);
        } else {
            if (crate::c::bf_read(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(12),
                0,
                1,
                false,
            ) as u8)
                != 0
            {
                symbolOffset = GetStatusSymbolTileOffset(1u8);
            } else {
                if (((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(12),
                    1,
                    2,
                    false,
                ) as u8) as i32)
                    != 0i32)
                    || ((crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(11),
                        7,
                        1,
                        false,
                    ) as u8)
                        != 0)
                {
                    symbolOffset = GetStatusSymbolTileOffset(2u8);
                } else {
                    statused = 0u8;
                }
            }
        }
        if (statused) != 0 {
            ContestBG_FillBoxWithIncrementingTile(
                0u8,
                symbolOffset,
                20u8,
                contestantOffset,
                2u8,
                1u8,
                17u8,
                1i16,
            );
            ContestBG_FillBoxWithIncrementingTile(
                0u8,
                ((((symbolOffset) as i32).wrapping_add(16i32)) as u16),
                20u8,
                ((((contestantOffset) as i32).wrapping_add(1i32)) as u8),
                2u8,
                1u8,
                17u8,
                1i16,
            );
        } else {
            ContestBG_FillBoxWithTile(0u8, 0u16, 20u8, contestantOffset, 2u8, 2u8, 17u8);
        }
        return statused;
    }
}
pub(crate) unsafe extern "C" fn DrawStatusSymbols() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    DrawStatusSymbol(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ContestClearGeneralTextWindow() {
    unsafe {
        FillWindowPixelBuffer(4u8, 0u8);
        CopyWindowToVram(4u8, 2u8);
        Contest_SetBgCopyFlags(0u32);
    }
}
pub(crate) unsafe extern "C" fn GetChosenMove(contestant: u8) -> u16 {
    unsafe {
        let mut contestant = contestant;
        if (Contest_IsMonsTurnDisabled(contestant)) != 0 {
            return 0u16;
        }
        if ((contestant) as i32)
            == ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
        {
            return (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((contestant) as i32) as isize * 64))
            .wrapping_add(30))
            .cast::<u16>())
            .wrapping_offset(
                (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .read()) as i32) as isize,
            ))
            .read();
        } else {
            let mut moveChoice: u8 = 0u8;
            ContestAI_ResetAI(contestant);
            moveChoice = ContestAI_GetActionToUse();
            return (((((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((contestant) as i32) as isize * 64))
            .wrapping_add(30))
            .cast::<u16>())
            .wrapping_offset(((moveChoice) as i32) as isize))
            .read();
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn GetAllChosenMoves() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .write(GetChosenMove(((i) as u8)));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RankContestants() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut arr = crate::ffi::Align4([0u8; 8]);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let __p1 =
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(4)
                        .cast::<i16>();
                    (__p1).write(
                        (((((__p1).read()) as i32).wrapping_add(
                            (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(2)
                            .cast::<i16>())
                            .read()) as i32),
                        )) as i16),
                    );
                    (((&raw mut arr).cast::<i16>()).wrapping_offset((i) as isize)).write(
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(4)
                        .cast::<i16>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 3i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        j = 3i32;
                        'l5: loop {
                            if !(j > i) {
                                break 'l5;
                            }
                            'l6: {
                                if (((((&raw mut arr).cast::<i16>())
                                    .wrapping_offset(((j).wrapping_sub(1i32)) as isize))
                                .read()) as i32)
                                    < (((((&raw mut arr).cast::<i16>())
                                        .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                {
                                    let mut temp: u16 = 0u16;
                                    {
                                        temp = (((((&raw mut arr).cast::<i16>())
                                            .wrapping_offset((j) as isize))
                                        .read())
                                            as u16);
                                        (((&raw mut arr).cast::<i16>())
                                            .wrapping_offset((j) as isize))
                                        .write(
                                            (((&raw mut arr).cast::<i16>()).wrapping_offset(
                                                ((j).wrapping_sub(1i32)) as isize,
                                            ))
                                            .read(),
                                        );
                                        (((&raw mut arr).cast::<i16>())
                                            .wrapping_offset(((j).wrapping_sub(1i32)) as isize))
                                        .write(((temp) as i16));
                                    }
                                }
                            }
                            j = (j).wrapping_sub(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l7: loop {
                if !(i < 4i32) {
                    break 'l7;
                }
                'l8: {
                    {
                        j = 0i32;
                        'l9: loop {
                            if !(j < 4i32) {
                                break 'l9;
                            }
                            'l10: {
                                if (((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 28))
                                .wrapping_add(4)
                                .cast::<i16>())
                                .read()) as i32)
                                    == (((((&raw mut arr).cast::<i16>())
                                        .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                {
                                    crate::c::bf_write(
                                        ((((((&raw mut gContestResources)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset((i) as isize * 28))
                                        .wrapping_add(11),
                                        0,
                                        2,
                                        ((j) as u8) as i32,
                                    );
                                    break 'l9;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        SortContestants(1u8);
        ApplyNextTurnOrder();
    }
}
pub(crate) unsafe extern "C" fn SetAttentionLevels() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut attentionLevel: u8 = 0u8;
                    if (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read()) as i32)
                        == 0i32
                    {
                        attentionLevel = 5u8;
                    } else {
                        if (((((((((&raw mut gContestResources)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(2)
                        .cast::<i16>())
                        .read()) as i32)
                            <= 0i32
                        {
                            attentionLevel = 0u8;
                        } else {
                            if (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(2)
                            .cast::<i16>())
                            .read()) as i32)
                                < 30i32
                            {
                                attentionLevel = 1u8;
                            } else {
                                if (((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 28))
                                .wrapping_add(2)
                                .cast::<i16>())
                                .read()) as i32)
                                    < 60i32
                                {
                                    attentionLevel = 2u8;
                                } else {
                                    if (((((((((&raw mut gContestResources)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset((i) as isize * 28))
                                    .wrapping_add(2)
                                    .cast::<i16>())
                                    .read()) as i32)
                                        < 80i32
                                    {
                                        attentionLevel = 3u8;
                                    } else {
                                        attentionLevel = 4u8;
                                    }
                                }
                            }
                        }
                    }
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(26))
                    .write(attentionLevel);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ContestantCanUseTurn(contestant: u8) -> u8 {
    unsafe {
        let mut contestant = contestant;
        if (((crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(12),
            1,
            2,
            false,
        ) as u8) as i32)
            != 0i32)
            || ((crate::c::bf_read(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(11),
                7,
                1,
                false,
            ) as u8)
                != 0)
        {
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SetContestantStatusesForNextRound() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(2)
                    .cast::<i16>())
                    .write(0i16);
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .cast::<i16>())
                    .write(0i16);
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(18))
                    .write(0u8);
                    if ((crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(12),
                        1,
                        2,
                        false,
                    ) as u8) as i32)
                        > 0i32
                    {
                        crate::c::bf_write(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(12),
                            1,
                            2,
                            ((crate::c::bf_read(
                                ((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 28))
                                .wrapping_add(12),
                                1,
                                2,
                                false,
                            ) as u8)
                                .wrapping_sub(1)) as i32,
                        );
                    }
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(14))
                    .write(0u8);
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(16),
                        0,
                        1,
                        (0u8) as i32,
                    );
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(15))
                    .write(0u8);
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(16),
                        1,
                        1,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(16),
                        2,
                        1,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(16),
                        3,
                        1,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(12),
                        0,
                        1,
                        (0u8) as i32,
                    );
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(19))
                    .write(255u8);
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(20))
                    .write(255u8);
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(16),
                        4,
                        2,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(21),
                        2,
                        1,
                        (crate::c::bf_read(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(21),
                            0,
                            1,
                            false,
                        ) as u8) as i32,
                    );
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(21),
                        0,
                        1,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(17),
                        0,
                        2,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(17),
                        5,
                        1,
                        (0u8) as i32,
                    );
                    if (crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(17),
                        2,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        crate::c::bf_write(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(12),
                            1,
                            2,
                            (1u8) as i32,
                        );
                        crate::c::bf_write(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(17),
                            2,
                            1,
                            (0u8) as i32,
                        );
                    }
                    if (crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(17),
                        3,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        crate::c::bf_write(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(11),
                            7,
                            1,
                            (1u8) as i32,
                        );
                        crate::c::bf_write(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(17),
                            3,
                            1,
                            (0u8) as i32,
                        );
                    }
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(17),
                        4,
                        1,
                        (0u8) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(8)
                    .cast::<u16>())
                    .write(
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read(),
                    );
                    ((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(28))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1))
                        .read()) as i32) as isize
                            * 8,
                    ))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(
                        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read(),
                    );
                    ((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(68))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(1))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((Contest_GetMoveExcitement(
                            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(6)
                            .cast::<u16>())
                            .read(),
                        )) as u8),
                    );
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::bf_write(
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1),
            0,
            1,
            (0u8) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Contest_IsMonsTurnDisabled(contestant: u8) -> u8 {
    unsafe {
        let mut contestant = contestant;
        if (((crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(12),
            1,
            2,
            false,
        ) as u8) as i32)
            != 0i32)
            || ((crate::c::bf_read(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(11),
                7,
                1,
                false,
            ) as u8)
                != 0)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn CalculateTotalPointsForContestant(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        ((((&raw mut gContestMonRound2Points)
            .cast::<u8>()
            .cast::<i16>())
        .cast::<i16>())
        .wrapping_offset(((contestant) as i32) as isize))
        .write(GetContestantRound2Points(contestant));
        ((((&raw mut gContestMonTotalPoints).cast::<u8>().cast::<i16>()).cast::<i16>())
            .wrapping_offset(((contestant) as i32) as isize))
        .write(
            ((((((((&raw mut gContestMonRound1Points)
                .cast::<u8>()
                .cast::<i16>())
            .cast::<i16>())
            .wrapping_offset(((contestant) as i32) as isize))
            .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gContestMonRound2Points)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .wrapping_offset(((contestant) as i32) as isize))
                    .read()) as i32),
                )) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn CalculateFinalScores() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    CalculateTotalPointsForContestant(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
        DetermineFinalStandings();
    }
}
pub(crate) unsafe extern "C" fn GetContestantRound2Points(contestant: u8) -> i16 {
    unsafe {
        let mut contestant = contestant;
        return ((((((((&raw mut gContestMonAppealPointTotals)
            .cast::<u8>()
            .cast::<i16>())
        .cast::<i16>())
        .wrapping_offset(((contestant) as i32) as isize))
        .read()) as i32)
            .wrapping_mul(2i32)) as i16);
    }
}
pub(crate) unsafe extern "C" fn DetermineFinalStandings() {
    unsafe {
        let mut randomOrdering = crate::ffi::Align4([0u8; 8]);
        (&raw mut randomOrdering)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(0u16);
        let mut standings = crate::ffi::Align4([0u8; 64]);
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut j: i32 = 0i32;
                    (((&raw mut randomOrdering).cast::<u16>()).wrapping_offset((i) as isize))
                        .write(Random());
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < i) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((&raw mut randomOrdering).cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    == (((((&raw mut randomOrdering).cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                {
                                    i = (i).wrapping_sub(1);
                                    break 'l3;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 4i32) {
                    break 'l5;
                }
                'l6: {
                    ((((&raw mut standings).cast::<u8>()).wrapping_offset((i) as isize * 16))
                        .cast::<i32>())
                    .write(
                        ((((((&raw mut gContestMonTotalPoints).cast::<u8>().cast::<i16>())
                            .cast::<i16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32),
                    );
                    ((((&raw mut standings).cast::<u8>()).wrapping_offset((i) as isize * 16))
                        .wrapping_add(4)
                        .cast::<i32>())
                    .write(
                        ((((((&raw mut gContestMonRound1Points)
                            .cast::<u8>()
                            .cast::<i16>())
                        .cast::<i16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32),
                    );
                    ((((&raw mut standings).cast::<u8>()).wrapping_offset((i) as isize * 16))
                        .wrapping_add(8)
                        .cast::<i32>())
                    .write(
                        (((((&raw mut randomOrdering).cast::<u16>()).wrapping_offset((i) as isize))
                            .read()) as i32),
                    );
                    ((((&raw mut standings).cast::<u8>()).wrapping_offset((i) as isize * 16))
                        .wrapping_add(12)
                        .cast::<i32>())
                    .write(i);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l7: loop {
                if !(i < 3i32) {
                    break 'l7;
                }
                'l8: {
                    let mut j: i32 = 0i32;
                    {
                        j = 3i32;
                        'l9: loop {
                            if !(j > i) {
                                break 'l9;
                            }
                            'l10: {
                                if (DidContestantPlaceHigher(
                                    (j).wrapping_sub(1i32),
                                    j,
                                    (&raw mut standings).cast::<u8>(),
                                )) != 0
                                {
                                    let mut temp = crate::ffi::Align4([0u8; 16]);
                                    (((&raw mut temp).cast::<u8>()).cast::<i32>()).write(
                                        ((((&raw mut standings).cast::<u8>()).wrapping_offset(
                                            ((j).wrapping_sub(1i32)) as isize * 16,
                                        ))
                                        .cast::<i32>())
                                        .read(),
                                    );
                                    (((&raw mut temp).cast::<u8>()).wrapping_add(4).cast::<i32>())
                                        .write(
                                            ((((&raw mut standings).cast::<u8>())
                                                .wrapping_offset(
                                                    ((j).wrapping_sub(1i32)) as isize * 16,
                                                ))
                                            .wrapping_add(4)
                                            .cast::<i32>())
                                            .read(),
                                        );
                                    (((&raw mut temp).cast::<u8>()).wrapping_add(8).cast::<i32>())
                                        .write(
                                            ((((&raw mut standings).cast::<u8>())
                                                .wrapping_offset(
                                                    ((j).wrapping_sub(1i32)) as isize * 16,
                                                ))
                                            .wrapping_add(8)
                                            .cast::<i32>())
                                            .read(),
                                        );
                                    (((&raw mut temp).cast::<u8>())
                                        .wrapping_add(12)
                                        .cast::<i32>())
                                    .write(
                                        ((((&raw mut standings).cast::<u8>()).wrapping_offset(
                                            ((j).wrapping_sub(1i32)) as isize * 16,
                                        ))
                                        .wrapping_add(12)
                                        .cast::<i32>())
                                        .read(),
                                    );
                                    ((((&raw mut standings).cast::<u8>())
                                        .wrapping_offset(((j).wrapping_sub(1i32)) as isize * 16))
                                    .cast::<i32>())
                                    .write(
                                        ((((&raw mut standings).cast::<u8>())
                                            .wrapping_offset((j) as isize * 16))
                                        .cast::<i32>())
                                        .read(),
                                    );
                                    ((((&raw mut standings).cast::<u8>())
                                        .wrapping_offset(((j).wrapping_sub(1i32)) as isize * 16))
                                    .wrapping_add(4)
                                    .cast::<i32>())
                                    .write(
                                        ((((&raw mut standings).cast::<u8>())
                                            .wrapping_offset((j) as isize * 16))
                                        .wrapping_add(4)
                                        .cast::<i32>())
                                        .read(),
                                    );
                                    ((((&raw mut standings).cast::<u8>())
                                        .wrapping_offset(((j).wrapping_sub(1i32)) as isize * 16))
                                    .wrapping_add(8)
                                    .cast::<i32>())
                                    .write(
                                        ((((&raw mut standings).cast::<u8>())
                                            .wrapping_offset((j) as isize * 16))
                                        .wrapping_add(8)
                                        .cast::<i32>())
                                        .read(),
                                    );
                                    ((((&raw mut standings).cast::<u8>())
                                        .wrapping_offset(((j).wrapping_sub(1i32)) as isize * 16))
                                    .wrapping_add(12)
                                    .cast::<i32>())
                                    .write(
                                        ((((&raw mut standings).cast::<u8>())
                                            .wrapping_offset((j) as isize * 16))
                                        .wrapping_add(12)
                                        .cast::<i32>())
                                        .read(),
                                    );
                                    ((((&raw mut standings).cast::<u8>())
                                        .wrapping_offset((j) as isize * 16))
                                    .cast::<i32>())
                                    .write((((&raw mut temp).cast::<u8>()).cast::<i32>()).read());
                                    ((((&raw mut standings).cast::<u8>())
                                        .wrapping_offset((j) as isize * 16))
                                    .wrapping_add(4)
                                    .cast::<i32>())
                                    .write(
                                        (((&raw mut temp).cast::<u8>())
                                            .wrapping_add(4)
                                            .cast::<i32>())
                                        .read(),
                                    );
                                    ((((&raw mut standings).cast::<u8>())
                                        .wrapping_offset((j) as isize * 16))
                                    .wrapping_add(8)
                                    .cast::<i32>())
                                    .write(
                                        (((&raw mut temp).cast::<u8>())
                                            .wrapping_add(8)
                                            .cast::<i32>())
                                        .read(),
                                    );
                                    ((((&raw mut standings).cast::<u8>())
                                        .wrapping_offset((j) as isize * 16))
                                    .wrapping_add(12)
                                    .cast::<i32>())
                                    .write(
                                        (((&raw mut temp).cast::<u8>())
                                            .wrapping_add(12)
                                            .cast::<i32>())
                                        .read(),
                                    );
                                }
                            }
                            j = (j).wrapping_sub(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l11: loop {
                if !(i < 4i32) {
                    break 'l11;
                }
                'l12: {
                    ((((&raw mut gContestFinalStandings).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut standings).cast::<u8>())
                                .wrapping_offset((i) as isize * 16))
                            .wrapping_add(12)
                            .cast::<i32>())
                            .read()) as isize,
                        ))
                    .write(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveLinkContestResults() {
    unsafe {
        if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32) != 0
        {
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1572))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gSpecialVar_ContestCategory)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 8,
            ))
            .cast::<u16>())
            .wrapping_offset(
                ((((((&raw mut gContestFinalStandings).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read())
                            as i32) as isize,
                    ))
                .read()) as i32) as isize,
            ))
            .write(
                ((if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1572))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_ContestCategory)
                        .cast::<u8>()
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
                .cast::<u16>())
                .wrapping_offset(
                    ((((((&raw mut gContestFinalStandings).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read())
                                as i32) as isize,
                        ))
                    .read()) as i32) as isize,
                ))
                .read()) as i32)
                    .wrapping_add(1i32)
                    > 9999i32
                {
                    9999i32
                } else {
                    ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1572))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gSpecialVar_ContestCategory)
                            .cast::<u8>()
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 8,
                    ))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((((&raw mut gContestFinalStandings).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>())
                                    .read()) as i32) as isize,
                            ))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        .wrapping_add(1i32)
                }) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DidContestantPlaceHigher(a: i32, b: i32, standings: *mut u8) -> u8 {
    unsafe {
        let mut a = a;
        let mut b = b;
        let mut standings = standings;
        let mut retVal: u8 = 0u8;
        if (((standings).wrapping_offset((a) as isize * 16)).cast::<i32>()).read()
            < (((standings).wrapping_offset((b) as isize * 16)).cast::<i32>()).read()
        {
            retVal = 1u8;
        } else {
            if (((standings).wrapping_offset((a) as isize * 16)).cast::<i32>()).read()
                > (((standings).wrapping_offset((b) as isize * 16)).cast::<i32>()).read()
            {
                retVal = 0u8;
            } else {
                if (((standings).wrapping_offset((a) as isize * 16))
                    .wrapping_add(4)
                    .cast::<i32>())
                .read()
                    < (((standings).wrapping_offset((b) as isize * 16))
                        .wrapping_add(4)
                        .cast::<i32>())
                    .read()
                {
                    retVal = 1u8;
                } else {
                    if (((standings).wrapping_offset((a) as isize * 16))
                        .wrapping_add(4)
                        .cast::<i32>())
                    .read()
                        > (((standings).wrapping_offset((b) as isize * 16))
                            .wrapping_add(4)
                            .cast::<i32>())
                        .read()
                    {
                        retVal = 0u8;
                    } else {
                        if (((standings).wrapping_offset((a) as isize * 16))
                            .wrapping_add(8)
                            .cast::<i32>())
                        .read()
                            < (((standings).wrapping_offset((b) as isize * 16))
                                .wrapping_add(8)
                                .cast::<i32>())
                            .read()
                        {
                            retVal = 1u8;
                        } else {
                            retVal = 0u8;
                        }
                    }
                }
            }
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn ContestPrintLinkStandby() {
    unsafe {
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
        ContestClearGeneralTextWindow();
        Contest_StartTextPrinter((&raw mut gText_LinkStandby4).cast::<u8>(), 0u32);
    }
}
pub(crate) unsafe extern "C" fn FillContestantWindowBgs() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ContestBG_FillBoxWithTile(
                        0u8,
                        0u16,
                        22u8,
                        (((2i32).wrapping_add((i).wrapping_mul(5i32))) as u8),
                        8u8,
                        2u8,
                        17u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetAppealHeartTileOffset(contestant: u8) -> u16 {
    unsafe {
        let mut contestant = contestant;
        let mut offset: u16 = 0u16;
        if ((contestant) as i32) == 0i32 {
            offset = 20497u16;
        } else {
            if ((contestant) as i32) == 1i32 {
                offset = 24593u16;
            } else {
                if ((contestant) as i32) == 2i32 {
                    offset = 28689u16;
                } else {
                    offset = 32785u16;
                }
            }
        }
        return ((((offset) as i32).wrapping_add(1i32)) as u16);
    }
}
pub(crate) unsafe extern "C" fn GetNumHeartsFromAppealPoints(appeal: i16) -> i8 {
    unsafe {
        let mut appeal = appeal;
        let mut hearts: i8 = ((crate::c::div_i32(((appeal) as i32), 10i32)) as i8);
        if ((hearts) as i32) > 16i32 {
            hearts = 16i8;
        } else {
            if ((hearts) as i32) < (-16i32) {
                hearts = (-16i8);
            }
        }
        return hearts;
    }
}
pub(crate) unsafe extern "C" fn UpdateAppealHearts(
    startAppeal: i16,
    appealDelta: i16,
    contestant: u8,
) -> u8 {
    unsafe {
        let mut startAppeal = startAppeal;
        let mut appealDelta = appealDelta;
        let mut contestant = contestant;
        let mut taskId: u8 = 0u8;
        let mut startHearts: i8 = 0i8;
        let mut heartsDelta: i8 = 0i8;
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 4))
            .wrapping_add(2),
            2,
            1,
            (1u8) as i32,
        );
        taskId = CreateTask(Some(Task_UpdateAppealHearts), 20u8);
        startHearts = GetNumHeartsFromAppealPoints(startAppeal);
        heartsDelta = ((((GetNumHeartsFromAppealPoints(
            ((((startAppeal) as i32).wrapping_add(((appealDelta) as i32))) as i16),
        )) as i32)
            .wrapping_sub(((startHearts) as i32))) as i8);
        GetAppealHeartTileOffset(contestant);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(
            ((if ((startHearts) as i32) < 0i32 {
                ((startHearts) as i32).wrapping_neg()
            } else {
                ((startHearts) as i32)
            }) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((heartsDelta) as i16));
        if (((startHearts) as i32) > 0i32)
            || ((((startHearts) as i32) == 0i32) && (((heartsDelta) as i32) > 0i32))
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(1i16);
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write((-1i16));
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((contestant) as i16));
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateAppealHearts(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut contestant: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as u8);
        let mut startHearts: i16 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read();
        let mut heartsDelta: i16 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read();
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 14i32
        {
            let mut heartOffset: u16 = 0u16;
            let mut newNumHearts: u8 = 0u8;
            let mut pitchMod: u8 = 0u8;
            let mut onSecondLine: u8 = 0u8;
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == 0i32
            {
                DestroyTask(taskId);
                crate::c::bf_write(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 4))
                    .wrapping_add(2),
                    2,
                    1,
                    (0u8) as i32,
                );
                return;
            } else {
                if ((startHearts) as i32) == 0i32 {
                    if ((heartsDelta) as i32) < 0i32 {
                        heartOffset = ((((GetAppealHeartTileOffset(contestant)) as i32)
                            .wrapping_add(2i32)) as u16);
                        let __p3 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    } else {
                        heartOffset = GetAppealHeartTileOffset(contestant);
                        let __p4 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1);
                        (__p4).write(((__p4).read()).wrapping_sub(1));
                    }
                    newNumHearts = (({
                        let __p5 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        let __t6 = (__p5).read();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                        __t6
                    }) as u8);
                } else {
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        < 0i32
                    {
                        if ((heartsDelta) as i32) < 0i32 {
                            newNumHearts = (({
                                let __p7 = ((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>();
                                let __t8 = (__p7).read();
                                (__p7).write(((__p7).read()).wrapping_add(1));
                                __t8
                            }) as u8);
                            let __p9 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1);
                            (__p9).write(((__p9).read()).wrapping_add(1));
                            heartOffset = ((((GetAppealHeartTileOffset(contestant)) as i32)
                                .wrapping_add(2i32))
                                as u16);
                        } else {
                            newNumHearts = (({
                                let __p10 = ((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>();
                                let __t11 = ((__p10).read()).wrapping_sub(1);
                                (__p10).write(__t11);
                                __t11
                            }) as u8);
                            heartOffset = 0u16;
                            let __p12 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1);
                            (__p12).write(((__p12).read()).wrapping_sub(1));
                        }
                    } else {
                        if ((heartsDelta) as i32) < 0i32 {
                            newNumHearts = (({
                                let __p13 = ((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>();
                                let __t14 = ((__p13).read()).wrapping_sub(1);
                                (__p13).write(__t14);
                                __t14
                            }) as u8);
                            heartOffset = 0u16;
                            let __p15 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1);
                            (__p15).write(((__p15).read()).wrapping_add(1));
                        } else {
                            newNumHearts = (({
                                let __p16 = ((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>();
                                let __t17 = (__p16).read();
                                (__p16).write(((__p16).read()).wrapping_add(1));
                                __t17
                            }) as u8);
                            let __p18 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1);
                            (__p18).write(((__p18).read()).wrapping_sub(1));
                            heartOffset = GetAppealHeartTileOffset(contestant);
                        }
                    }
                }
            }
            pitchMod = newNumHearts;
            onSecondLine = 0u8;
            if ((newNumHearts) as i32) > 7i32 {
                onSecondLine = 1u8;
                newNumHearts = ((((newNumHearts) as i32).wrapping_sub(8i32)) as u8);
            }
            ContestBG_FillBoxWithTile(
                0u8,
                heartOffset,
                ((((newNumHearts) as i32).wrapping_add(22i32)) as u8),
                ((((((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((contestant) as i32) as isize))
                .read()) as i32)
                    .wrapping_mul(5i32))
                .wrapping_add(2i32))
                .wrapping_add(((onSecondLine) as i32))) as u8),
                1u8,
                1u8,
                17u8,
            );
            if ((heartsDelta) as i32) > 0i32 {
                PlaySE(96u16);
                m4aMPlayImmInit((&raw mut gMPlayInfo_SE1).cast::<u8>());
                m4aMPlayPitchControl(
                    (&raw mut gMPlayInfo_SE1).cast::<u8>(),
                    65535u16,
                    ((((pitchMod) as i32).wrapping_mul(256i32)) as i16),
                );
            } else {
                PlaySE(22u16);
            }
            if ((!((onSecondLine) != 0)) && (((newNumHearts) as i32) == 0i32))
                && (((heartOffset) as i32) == 0i32)
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(
                    ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        .wrapping_neg()) as i16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSliderHeartSprites() {
    unsafe {
        let mut i: i32 = 0i32;
        LoadSpriteSheet(
            (&raw const sSpriteSheet_SliderHeart)
                .cast::<u8>()
                .cast_mut(),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut y: u8 =
                        ((((&raw const sSliderHeartYPositions).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize,
                        ))
                        .read();
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 4))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_SliderHeart)
                            .cast::<u8>()
                            .cast_mut(),
                        180i16,
                        ((y) as i16),
                        1u8,
                    ));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateHeartSlider(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        let mut spriteId: u8 = 0u8;
        let mut slideTarget: i16 = 0i16;
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 4))
            .wrapping_add(2),
            0,
            1,
            (1u8) as i32,
        );
        spriteId = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((contestant) as i32) as isize * 4))
        .read();
        slideTarget = (((crate::c::div_i32(
            (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(4)
            .cast::<i16>())
            .read()) as i32),
            10i32,
        ))
        .wrapping_mul(2i32)) as i16);
        if ((slideTarget) as i32) > 56i32 {
            slideTarget = 56i16;
        } else {
            if ((slideTarget) as i32) < 0i32 {
                slideTarget = 0i16;
            }
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((contestant) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(slideTarget);
        if ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            > ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .read()) as i32)
        {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(1i16);
        } else {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write((-1i16));
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_UpdateHeartSlider));
    }
}
pub(crate) unsafe extern "C" fn UpdateHeartSliders() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    UpdateHeartSlider(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SlidersDoneUpdating() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(2),
                        0,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i == 4i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UpdateHeartSlider(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
        {
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 4,
                ))
                .wrapping_add(2),
                0,
                1,
                (0u8) as i32,
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        } else {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateSliderHeartSpriteYPositions() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 4))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(
                        ((((((&raw const sSliderHeartYPositions).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetBottomSliderHeartsInvisibility(invisible: u8) {
    unsafe {
        let mut invisible = invisible;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        > 1i32
                    {
                        if !((invisible) != 0) {
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 4))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(32)
                            .cast::<i16>())
                            .write(180i16);
                        } else {
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 4))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(32)
                            .cast::<i16>())
                            .write(256i16);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateNextTurnSprites() {
    unsafe {
        let mut i: i32 = 0i32;
        LoadSpritePalette((&raw const sSpritePalette_NextTurn).cast::<u8>().cast_mut());
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    LoadCompressedSpriteSheet(
                        (((&raw const sSpriteSheet_NextTurn).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 8),
                    );
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 4))
                    .wrapping_add(1))
                    .write(CreateSprite(
                        (((&raw const sSpriteTemplates_NextTurn)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 24),
                        204i16,
                        ((((((&raw const sNextTurnSpriteYPositions)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i16),
                        0u8,
                    ));
                    SetSubspriteTables(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(1))
                            .read()) as i32) as isize
                                * 68,
                        ),
                        ((&raw const sSubspriteTable_NextTurn)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(1))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateApplauseMeterSprite() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_ApplauseMeter)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadSpritePalette(
            (&raw const sSpritePalette_ApplauseMeter)
                .cast::<u8>()
                .cast_mut(),
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_ApplauseMeter)
                .cast::<u8>()
                .cast_mut(),
            30i16,
            44i16,
            1u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(88))
        .write(spriteId);
    }
}
pub(crate) unsafe extern "C" fn CreateJudgeAttentionEyeTask() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut taskId: u8 = CreateTask(Some(Task_FlashJudgeAttentionEye), 30u8);
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(13))
        .write(taskId);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset((((i) as i32).wrapping_mul(4i32)) as isize))
                    .write(255i16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartFlashJudgeAttentionEye(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(13))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(((((contestant) as i32).wrapping_mul(4i32)).wrapping_add(0i32)) as isize))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(13))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(((((contestant) as i32).wrapping_mul(4i32)).wrapping_add(1i32)) as isize))
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn StopFlashJudgeAttentionEye(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        let mut taskId: u8 = CreateTask(Some(Task_StopFlashJudgeAttentionEye), 31u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((contestant) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_StopFlashJudgeAttentionEye(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut contestant: u8 = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        if (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(13))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(((((contestant) as i32).wrapping_mul(4i32)).wrapping_add(0i32)) as isize))
        .read()) as i32)
            == 0i32)
            || (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(13))
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(
                ((((contestant) as i32).wrapping_mul(4i32)).wrapping_add(0i32)) as isize,
            ))
            .read()) as i32)
                == 255i32)
        {
            ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(13))
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(
                ((((contestant) as i32).wrapping_mul(4i32)).wrapping_add(0i32)) as isize,
            ))
            .write(255i16);
            ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(13))
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(
                ((((contestant) as i32).wrapping_mul(4i32)).wrapping_add(1i32)) as isize,
            ))
            .write(0i16);
            BlendPalette(
                ((((0i32).wrapping_add(
                    ((5i32).wrapping_add(
                        ((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(20))
                        .cast::<u8>())
                        .wrapping_offset(((contestant) as i32) as isize))
                        .read()) as i32),
                    ))
                    .wrapping_mul(16i32),
                ))
                .wrapping_add(6i32)) as u16),
                2u16,
                0u8,
                19455u16,
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_FlashJudgeAttentionEye(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut offset: u8 = ((((i) as i32).wrapping_mul(4i32)) as u8);
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset((((offset) as i32).wrapping_add(0i32)) as isize))
                    .read()) as i32)
                        != 255i32
                    {
                        if ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset((((offset) as i32).wrapping_add(1i32)) as isize))
                        .read()) as i32)
                            == 0i32
                        {
                            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((offset) as i32).wrapping_add(0i32)) as isize);
                            (__p1).write(((__p1).read()).wrapping_add(1));
                        } else {
                            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((offset) as i32).wrapping_add(0i32)) as isize);
                            (__p2).write(((__p2).read()).wrapping_sub(1));
                        }
                        if (((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset((((offset) as i32).wrapping_add(0i32)) as isize))
                        .read()) as i32)
                            == 16i32)
                            || (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((offset) as i32).wrapping_add(0i32)) as isize))
                            .read()) as i32)
                                == 0i32)
                        {
                            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((offset) as i32).wrapping_add(1i32)) as isize);
                            (__p3).write((((((__p3).read()) as i32) ^ 1i32) as i16));
                        }
                        BlendPalette(
                            ((((0i32).wrapping_add(
                                ((5i32).wrapping_add(
                                    ((((((((((&raw mut gContestResources)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(20))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32),
                                ))
                                .wrapping_mul(16i32),
                            ))
                            .wrapping_add(6i32)) as u16),
                            2u16,
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((offset) as i32).wrapping_add(0i32)) as isize))
                            .read()) as u8),
                            19455u16,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateUnusedBlendTask() {
    unsafe {
        let mut i: i32 = 0i32;
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(14))
        .write(CreateTask(Some(Task_UnusedBlend), 30u8));
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    InitUnusedBlendTaskData(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitUnusedBlendTaskData(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(14))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset((((contestant) as i32).wrapping_mul(4i32)) as isize))
        .write(255i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(14))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(((((contestant) as i32).wrapping_mul(4i32)).wrapping_add(1i32)) as isize))
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn UpdateBlendTaskContestantsData() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    UpdateBlendTaskContestantData(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateBlendTaskContestantData(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        let mut palOffset1: u32 = 0u32;
        let mut palOffset2: u32 = 0u32;
        InitUnusedBlendTaskData(contestant);
        palOffset1 = ((((contestant) as i32).wrapping_add(5i32)) as u32);
        {
            let mut _src: *mut u8 = ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(
                    ((((palOffset1).wrapping_mul(16u32)).wrapping_add(10u32)) as i32) as isize,
                ))
            .cast::<u8>();
            let mut _dest: *mut u8 = ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(
                    ((((palOffset1).wrapping_mul(16u32)).wrapping_add(10u32)) as i32) as isize,
                ))
            .cast::<u8>();
            let mut _size: u32 = 2u32;
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    ((_dest) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (2147483648u32
                                        | crate::c::div_u32(
                                            _size,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        )),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
        palOffset2 = (((((((contestant) as i32).wrapping_add(5i32)).wrapping_mul(16i32))
            .wrapping_add(12i32))
        .wrapping_add(((contestant) as i32))) as u32);
        {
            let mut _src: *mut u8 = ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((palOffset2) as i32) as isize))
            .cast::<u8>();
            let mut _dest: *mut u8 = ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((palOffset2) as i32) as isize))
            .cast::<u8>();
            let mut _size: u32 = 2u32;
            'l5: loop {
                'l6: {
                    'l7: loop {
                        'l8: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    ((_dest) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (2147483648u32
                                        | crate::c::div_u32(
                                            _size,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        )),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l5;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UnusedBlend(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut idx: u8 = ((((i) as i32).wrapping_mul(4i32)) as u8);
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(((idx) as i32) as isize))
                    .read()) as i32)
                        != 255i32
                    {
                        if (({
                            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((idx) as i32).wrapping_add(2i32)) as isize);
                            let __t2 = ((__p1).read()).wrapping_add(1);
                            (__p1).write(__t2);
                            __t2
                        }) as i32)
                            > 2i32
                        {
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((idx) as i32).wrapping_add(2i32)) as isize))
                            .write(0i16);
                            if ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((idx) as i32).wrapping_add(1i32)) as isize))
                            .read()) as i32)
                                == 0i32
                            {
                                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(((idx) as i32) as isize);
                                (__p3).write(((__p3).read()).wrapping_add(1));
                            } else {
                                let __p4 = (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(((idx) as i32) as isize);
                                (__p4).write(((__p4).read()).wrapping_sub(1));
                            }
                            if (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(((idx) as i32) as isize))
                            .read()) as i32)
                                == 16i32)
                                || (((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(((idx) as i32) as isize))
                                .read()) as i32)
                                    == 0i32)
                            {
                                let __p5 = (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset((((idx) as i32).wrapping_add(1i32)) as isize);
                                (__p5).write((((((__p5).read()) as i32) ^ 1i32) as i16));
                            }
                            BlendPalette(
                                ((((0i32).wrapping_add(
                                    ((5i32).wrapping_add(((i) as i32))).wrapping_mul(16i32),
                                ))
                                .wrapping_add(10i32)) as u16),
                                1u16,
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset((((idx) as i32).wrapping_add(0i32)) as isize))
                                .read()) as u8),
                                19455u16,
                            );
                            BlendPalette(
                                (((((0i32).wrapping_add(
                                    ((5i32).wrapping_add(((i) as i32))).wrapping_mul(16i32),
                                ))
                                .wrapping_add(12i32))
                                .wrapping_add(((i) as i32)))
                                    as u16),
                                1u16,
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset((((idx) as i32).wrapping_add(0i32)) as isize))
                                .read()) as u8),
                                19455u16,
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartStopFlashJudgeAttentionEye(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        if (crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(21),
            4,
            1,
            false,
        ) as u8)
            != 0
        {
            StartFlashJudgeAttentionEye(contestant);
        } else {
            StopFlashJudgeAttentionEye(contestant);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateContestantBoxBlinkSprites(contestant: u8) -> u8 {
    unsafe {
        let mut contestant = contestant;
        let mut spriteId1: u8 = 0u8;
        let mut spriteId2: u8 = 0u8;
        let mut x: u8 = (((((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((contestant) as i32) as isize))
        .read()) as i32)
            .wrapping_mul(40i32))
        .wrapping_add(32i32)) as u8);
        LoadCompressedSpriteSheet(
            (((&raw const sSpriteSheets_ContestantsTurnBlinkEffect)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((contestant) as i32) as isize * 8),
        );
        LoadSpritePalette(
            (((&raw const sSpritePalettes_ContestantsTurnBlinkEffect)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((contestant) as i32) as isize * 8),
        );
        spriteId1 = CreateSprite(
            (((&raw const sSpriteTemplates_ContestantsTurnBlinkEffect)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((contestant) as i32) as isize * 24),
            184i16,
            ((x) as i16),
            29u8,
        );
        spriteId2 = CreateSprite(
            (((&raw const sSpriteTemplates_ContestantsTurnBlinkEffect)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((contestant) as i32) as isize * 24),
            248i16,
            ((x) as i16),
            29u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId2) as i32) as isize * 68))
                .wrapping_add(4),
                0,
                10,
                false,
            ) as u16) as i32)
                .wrapping_add(64i32)) as u16) as i32,
        );
        CopySpriteTiles(
            0u8,
            3u8,
            ((100663296i32) as usize as *mut u8),
            ((((100720640i32).wrapping_add(
                (((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((contestant) as i32) as isize))
                .read()) as i32)
                    .wrapping_mul(5i32))
                .wrapping_mul(64i32),
            ))
            .wrapping_add(38i32)) as usize as *mut u16),
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52)
                .cast::<*mut u8>())
            .read(),
        );
        CopySpriteTiles(
            0u8,
            3u8,
            ((100663296i32) as usize as *mut u8),
            ((((100720640i32).wrapping_add(
                (((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((contestant) as i32) as isize))
                .read()) as i32)
                    .wrapping_mul(5i32))
                .wrapping_mul(64i32),
            ))
            .wrapping_add(54i32)) as usize as *mut u16),
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(56)
                .cast::<*mut u8>())
            .read(),
        );
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(52)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(1280),
                                ((83886080i32
                                    | (crate::c::div_i32(768i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(56)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(1280),
                                ((83886080i32
                                    | (crate::c::div_i32(768i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        RequestDma3Copy(
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52)
                .cast::<*mut u8>())
            .read(),
            (((100728832i32).wrapping_add(
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId1) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    false,
                ) as u16) as i32)
                    .wrapping_mul(32i32),
            )) as usize as *mut u8),
            2048u16,
            1u8,
        );
        RequestDma3Copy(
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(56)
                .cast::<*mut u8>())
            .read(),
            (((100728832i32).wrapping_add(
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId2) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    false,
                ) as u16) as i32)
                    .wrapping_mul(32i32),
            )) as usize as *mut u8),
            2048u16,
            1u8,
        );
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(((spriteId2) as i16));
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(((spriteId1) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((contestant) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((contestant) as i16));
        return spriteId1;
    }
}
pub(crate) unsafe extern "C" fn DestroyContestantBoxBlinkSprites(spriteId: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut spriteId2: u8 = (((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as u8);
        FreeSpriteOamMatrix(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId2) as i32) as isize * 68),
        );
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId2) as i32) as isize * 68),
        );
        DestroySpriteAndFreeResources(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
    }
}
pub(crate) unsafe extern "C" fn SetBlendForContestantBoxBlink() {
    unsafe {
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(82u8, 2311u16);
    }
}
pub(crate) unsafe extern "C" fn ResetBlendForContestantBoxBlink() {
    unsafe {
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn BlinkContestantBox(spriteId: u8, b: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut b = b;
        let mut spriteId2: u8 = 0u8;
        SetBlendForContestantBoxBlink();
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 4,
            ))
            .wrapping_add(2),
            1,
            1,
            (1u8) as i32,
        );
        spriteId2 = (((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as u8);
        StartSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            1u8,
        );
        StartSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId2) as i32) as isize * 68),
            1u8,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_BlinkContestantBox));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        if ((b) as i32) == 0i32 {
            PlaySE(101u16);
        } else {
            PlaySE(2u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BlinkContestantBox(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            let mut spriteId2: u8 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
            if (crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId2) as i32) as isize * 68))
                .wrapping_add(63),
                5,
                1,
                false,
            ) as u16)
                != 0
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId2) as i32) as isize * 68))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_EndBlinkContestantBox));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_EndBlinkContestantBox(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize
                    * 4,
            ))
            .wrapping_add(2),
            1,
            1,
            (0u8) as i32,
        );
        DestroyContestantBoxBlinkSprites(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
        );
        ResetBlendForContestantBoxBlink();
    }
}
pub(crate) unsafe extern "C" fn ContestDebugTogglePointTotal() {
    unsafe {
        if (((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).read()) as i32) == 1i32 {
            (((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).write(0u8);
        } else {
            (((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).write(1u8);
        }
        if (((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).read()) as i32) == 0i32 {
            DrawContestantWindowText();
            SwapMoveDescAndContestTilemaps();
        } else {
            ContestDebugDoPrint();
        }
    }
}
pub(crate) unsafe extern "C" fn ContestDebugDoPrint() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut value: i16 = 0i16;
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        let mut text = crate::ffi::Align4([0u8; 8]);
        if !((((&raw mut gEnableContestDebugging).cast::<u8>()).read()) != 0) {
            return;
        }
        'l1: {
            let __sw1 = (((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 3i32 {
                ContestDebugPrintBitStrings();
                break 'l1;
            }
            if !__matched {
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            FillWindowPixelBuffer(i, 0u8);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = 0u8;
                    'l4: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l4;
                        }
                        'l5: {
                            value = (((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(4)
                            .cast::<i16>())
                            .read();
                            txtPtr = (&raw mut text).cast::<u8>();
                            if (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(4)
                            .cast::<i16>())
                            .read()) as i32)
                                < 0i32
                            {
                                value = ((((value) as i32).wrapping_mul((-1i32))) as i16);
                                txtPtr = StringCopy(txtPtr, (&raw mut gText_OneDash).cast::<u8>());
                            }
                            ConvertIntToDecimalStringN(txtPtr, ((value) as i32), 0i32, 4u8);
                            Contest_PrintTextToBg0WindowAt(
                                ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as u32),
                                (&raw mut text).cast::<u8>(),
                                55i32,
                                1i32,
                                7i32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = 0u8;
                    'l6: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l6;
                        }
                        'l7: {
                            value = (((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(2)
                            .cast::<i16>())
                            .read();
                            txtPtr = (&raw mut text).cast::<u8>();
                            if (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(2)
                            .cast::<i16>())
                            .read()) as i32)
                                < 0i32
                            {
                                value = ((((value) as i32).wrapping_mul((-1i32))) as i16);
                                txtPtr = StringCopy(txtPtr, (&raw mut gText_OneDash).cast::<u8>());
                            }
                            ConvertIntToDecimalStringN(txtPtr, ((value) as i32), 0i32, 4u8);
                            Contest_PrintTextToBg0WindowAt(
                                ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as u32),
                                (&raw mut text).cast::<u8>(),
                                5i32,
                                1i32,
                                7i32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                SwapMoveDescAndContestTilemaps();
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SortContestants(useRanking: u8) {
    unsafe {
        let mut useRanking = useRanking;
        let mut scratch = crate::ffi::Align4([0u8; 4]);
        let mut randomOrdering = crate::ffi::Align4([0u8; 8]);
        (&raw mut randomOrdering)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(0u16);
        let mut i: i32 = 0i32;
        let mut v3: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut j: i32 = 0i32;
                    (((&raw mut randomOrdering).cast::<u16>()).wrapping_offset((i) as isize))
                        .write(Random());
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < i) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((&raw mut randomOrdering).cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    == (((((&raw mut randomOrdering).cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                {
                                    i = (i).wrapping_sub(1);
                                    break 'l3;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((useRanking) != 0) {
            {
                i = 0i32;
                'l5: loop {
                    if !(i < 4i32) {
                        break 'l5;
                    }
                    'l6: {
                        ((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .write(((i) as u8));
                        {
                            v3 = 0i32;
                            'l7: loop {
                                if !(v3 < i) {
                                    break 'l7;
                                }
                                'l8: {
                                    if (((((((&raw mut gContestMonRound1Points)
                                        .cast::<u8>()
                                        .cast::<i16>())
                                    .cast::<i16>())
                                    .wrapping_offset(
                                        ((((((&raw mut gContestantTurnOrder).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset((v3) as isize))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32)
                                        < ((((((&raw mut gContestMonRound1Points)
                                            .cast::<u8>()
                                            .cast::<i16>())
                                        .cast::<i16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32))
                                        || ((((((((&raw mut gContestMonRound1Points)
                                            .cast::<u8>()
                                            .cast::<i16>())
                                        .cast::<i16>())
                                        .wrapping_offset(
                                            ((((((&raw mut gContestantTurnOrder).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset((v3) as isize))
                                            .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            == ((((((&raw mut gContestMonRound1Points)
                                                .cast::<u8>()
                                                .cast::<i16>())
                                            .cast::<i16>())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as i32))
                                            && ((((((&raw mut randomOrdering).cast::<u16>())
                                                .wrapping_offset(
                                                    ((((((&raw mut gContestantTurnOrder)
                                                        .cast::<u8>())
                                                    .cast::<u8>())
                                                    .wrapping_offset((v3) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                            .read())
                                                as i32)
                                                < (((((&raw mut randomOrdering).cast::<u16>())
                                                    .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)))
                                    {
                                        let mut j: i32 = 0i32;
                                        {
                                            j = i;
                                            'l9: loop {
                                                if !(j > v3) {
                                                    break 'l9;
                                                }
                                                'l10: {
                                                    ((((&raw mut gContestantTurnOrder)
                                                        .cast::<u8>())
                                                    .cast::<u8>())
                                                    .wrapping_offset((j) as isize))
                                                    .write(
                                                        ((((&raw mut gContestantTurnOrder)
                                                            .cast::<u8>())
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((j).wrapping_sub(1i32)) as isize,
                                                        ))
                                                        .read(),
                                                    );
                                                }
                                                j = (j).wrapping_sub(1);
                                            }
                                        }
                                        ((((&raw mut gContestantTurnOrder).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset((v3) as isize))
                                        .write(((i) as u8));
                                        break 'l7;
                                    }
                                }
                                v3 = (v3).wrapping_add(1);
                            }
                        }
                        if v3 == i {
                            ((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .write(((i) as u8));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            crate::c::memcpy(
                (&raw mut scratch).cast::<u8>(),
                ((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>(),
                4u32,
            );
            {
                i = 0i32;
                'l11: loop {
                    if !(i < 4i32) {
                        break 'l11;
                    }
                    'l12: {
                        ((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                (((((&raw mut scratch).cast::<u8>()).wrapping_offset((i) as isize))
                                    .read()) as i32) as isize,
                            ))
                        .write(((i) as u8));
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            crate::c::memset((&raw mut scratch).cast::<u8>(), 255i32, 4u32);
            {
                i = 0i32;
                'l13: loop {
                    if !(i < 4i32) {
                        break 'l13;
                    }
                    'l14: {
                        let mut j: u8 = (crate::c::bf_read(
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 28))
                            .wrapping_add(11),
                            0,
                            2,
                            false,
                        ) as u8);
                        'l15: loop {
                            if !((1i32) != 0) {
                                break 'l15;
                            }
                            let mut ptr: *mut u8 = ((&raw mut scratch).cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize);
                            if (((ptr).read()) as i32) == 255i32 {
                                (ptr).write(((i) as u8));
                                ((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .write(j);
                                break 'l15;
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l16: loop {
                    if !(i < 3i32) {
                        break 'l16;
                    }
                    'l17: {
                        {
                            v3 = 3i32;
                            'l18: loop {
                                if !(v3 > i) {
                                    break 'l18;
                                }
                                'l19: {
                                    if ((((crate::c::bf_read(
                                        ((((((&raw mut gContestResources)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((v3).wrapping_sub(1i32)) as isize * 28))
                                        .wrapping_add(11),
                                        0,
                                        2,
                                        false,
                                    ) as u8) as i32)
                                        == ((crate::c::bf_read(
                                            ((((((&raw mut gContestResources)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(4)
                                            .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset((v3) as isize * 28))
                                            .wrapping_add(11),
                                            0,
                                            2,
                                            false,
                                        ) as u8)
                                            as i32))
                                        && (((((((&raw mut gContestantTurnOrder).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(((v3).wrapping_sub(1i32)) as isize))
                                        .read())
                                            as i32)
                                            < ((((((&raw mut gContestantTurnOrder).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset((v3) as isize))
                                            .read())
                                                as i32)))
                                        && ((((((&raw mut randomOrdering).cast::<u16>())
                                            .wrapping_offset(((v3).wrapping_sub(1i32)) as isize))
                                        .read())
                                            as i32)
                                            < (((((&raw mut randomOrdering).cast::<u16>())
                                                .wrapping_offset((v3) as isize))
                                            .read())
                                                as i32))
                                    {
                                        let mut temp: u8 = ((((&raw mut gContestantTurnOrder)
                                            .cast::<u8>())
                                        .cast::<u8>())
                                        .wrapping_offset((v3) as isize))
                                        .read();
                                        ((((&raw mut gContestantTurnOrder).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset((v3) as isize))
                                        .write(
                                            ((((&raw mut gContestantTurnOrder).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(((v3).wrapping_sub(1i32)) as isize))
                                            .read(),
                                        );
                                        ((((&raw mut gContestantTurnOrder).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(((v3).wrapping_sub(1i32)) as isize))
                                        .write(temp);
                                    }
                                }
                                v3 = (v3).wrapping_sub(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawContestantWindows() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut windowId: i32 = (i).wrapping_add(5i32);
                    LoadPalette(
                        ((((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106500))
                            .cast::<u8>())
                        .wrapping_offset((windowId) as isize * 32))
                        .cast::<u16>())
                        .cast::<u8>(),
                        (((0i32).wrapping_add(
                            ((5i32).wrapping_add(
                                ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32),
                            ))
                            .wrapping_mul(16i32),
                        )) as u16),
                        32u16,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        DrawContestantWindowText();
    }
}
pub(crate) unsafe extern "C" fn CalculateAppealMoveImpact(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        let mut r#move: u16 = 0u16;
        let mut effect: u8 = 0u8;
        let mut rnd: u8 = 0u8;
        let mut i: i32 = 0i32;
        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((contestant) as i32) as isize * 28))
        .wrapping_add(2)
        .cast::<i16>())
        .write(0i16);
        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((contestant) as i32) as isize * 28))
        .cast::<i16>())
        .write(0i16);
        if !((ContestantCanUseTurn(contestant)) != 0) {
            return;
        }
        r#move = (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((contestant) as i32) as isize * 28))
        .wrapping_add(6)
        .cast::<u16>())
        .read();
        effect = (((&raw mut gContestMoves).cast::<u8>())
            .wrapping_offset(((r#move) as i32) as isize * 8))
        .read();
        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((contestant) as i32) as isize * 28))
        .wrapping_add(10))
        .write(
            (crate::c::bf_read(
                (((&raw mut gContestMoves).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
                .wrapping_add(1),
                0,
                3,
                false,
            ) as u8),
        );
        if ((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((contestant) as i32) as isize * 28))
        .wrapping_add(6)
        .cast::<u16>())
        .read()) as i32)
            == (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(8)
            .cast::<u16>())
            .read()) as i32))
            && ((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(6)
            .cast::<u16>())
            .read()) as i32)
                != 0i32)
        {
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(21),
                0,
                1,
                (1u8) as i32,
            );
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(11),
                4,
                3,
                ((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(11),
                    4,
                    3,
                    false,
                ) as u8)
                    .wrapping_add(1)) as i32,
            );
        } else {
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(11),
                4,
                3,
                (0u8) as i32,
            );
        }
        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((contestant) as i32) as isize * 28))
        .cast::<i16>())
        .write(
            ((((((&raw mut gContestEffects).cast::<u8>())
                .wrapping_offset(((effect) as i32) as isize * 4))
            .wrapping_add(1))
            .read()) as i16),
        );
        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((contestant) as i32) as isize * 28))
        .wrapping_add(2)
        .cast::<i16>())
        .write(
            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .cast::<i16>())
            .read(),
        );
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<i16>())
        .write(
            ((((((&raw mut gContestEffects).cast::<u8>())
                .wrapping_offset(((effect) as i32) as isize * 4))
            .wrapping_add(2))
            .read()) as i16),
        );
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(6)
        .cast::<i16>())
        .write(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4)
            .cast::<i16>())
            .read(),
        );
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(17))
        .write(contestant);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(14))
                    .write(0u8);
                    ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(13))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(21),
            4,
            1,
            false,
        ) as u8)
            != 0)
            && (!((AreMovesContestCombo(
                (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(8)
                .cast::<u16>())
                .read(),
                (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(6)
                .cast::<u16>())
                .read(),
            )) != 0))
        {
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(21),
                4,
                1,
                (0u8) as i32,
            );
        }
        (((((&raw mut gContestEffectFuncs).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((effect) as i32) as isize))
        .read())
        .unwrap_unchecked()();
        if ((crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(16),
            4,
            2,
            false,
        ) as u8) as i32)
            == 1i32
        {
            let __p1 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(2)
            .cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(13)
                    .cast::<i8>())
                    .read()) as i32)
                        .wrapping_sub(10i32),
                )) as i16),
            );
        } else {
            if (crate::c::bf_read(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(17),
                5,
                1,
                false,
            ) as u8)
                != 0
            {
                let __p2 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(2)
                .cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(13)
                        .cast::<i8>())
                        .read()) as i32)
                            .wrapping_mul(3i32),
                    )) as i16),
                );
            } else {
                let __p3 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(2)
                .cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(13)
                        .cast::<i8>())
                        .read()) as i32),
                    )) as i16),
                );
            }
        }
        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((contestant) as i32) as isize * 28))
        .wrapping_add(22))
        .write(0u8);
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(21),
            6,
            1,
            (0u8) as i32,
        );
        if (IsContestantAllowedToCombo(contestant)) != 0 {
            let mut completedCombo: u8 = AreMovesContestCombo(
                (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(8)
                .cast::<u16>())
                .read(),
                (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(6)
                .cast::<u16>())
                .read(),
            );
            if ((completedCombo) != 0)
                && ((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(21),
                    4,
                    1,
                    false,
                ) as u8)
                    != 0)
            {
                (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(22))
                .write(completedCombo);
                crate::c::bf_write(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(21),
                    6,
                    1,
                    (1u8) as i32,
                );
                crate::c::bf_write(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(21),
                    4,
                    1,
                    (0u8) as i32,
                );
                (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(23))
                .write(
                    (((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_mul(
                            (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 28))
                            .wrapping_add(22))
                            .read()) as i32),
                        )) as u8),
                );
                crate::c::bf_write(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(21),
                    3,
                    1,
                    (1u8) as i32,
                );
            } else {
                if ((((((&raw mut gContestMoves).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
                .wrapping_add(2))
                .read()) as i32)
                    != 0i32
                {
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(21),
                        4,
                        1,
                        (1u8) as i32,
                    );
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(21),
                        6,
                        1,
                        (1u8) as i32,
                    );
                } else {
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((contestant) as i32) as isize * 28))
                        .wrapping_add(21),
                        4,
                        1,
                        (0u8) as i32,
                    );
                }
            }
        }
        if (crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(21),
            0,
            1,
            false,
        ) as u8)
            != 0
        {
            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(24))
            .write(
                (((((crate::c::bf_read(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(11),
                    4,
                    3,
                    false,
                ) as u8) as i32)
                    .wrapping_add(1i32))
                .wrapping_mul(10i32)) as u8),
            );
        }
        if (crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(12),
            0,
            1,
            false,
        ) as u8)
            != 0
        {
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(21),
                4,
                1,
                (0u8) as i32,
            );
            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(2)
            .cast::<i16>())
            .write(0i16);
            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .cast::<i16>())
            .write(0i16);
        }
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<*mut u8>())
        .read())
        .cast::<i8>())
        .write(Contest_GetMoveExcitement(
            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(6)
            .cast::<u16>())
            .read(),
        ));
        if (crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(17),
            4,
            1,
            false,
        ) as u8)
            != 0
        {
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .cast::<i8>())
            .write(1i8);
        }
        if ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<*mut u8>())
        .read())
        .cast::<i8>())
        .read()) as i32)
            > 0i32
        {
            if ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(19)
            .cast::<i8>())
            .read()) as i32)
                .wrapping_add(
                    ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<i8>())
                    .read()) as i32),
                )
                > 4i32
            {
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2)
                .cast::<i8>())
                .write(60i8);
            } else {
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2)
                .cast::<i8>())
                .write(10i8);
            }
        } else {
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2)
            .cast::<i8>())
            .write(0i8);
        }
        rnd = ((crate::c::rem_i32(((Random()) as i32), 3i32)) as u8);
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    if i != ((contestant) as i32) {
                        if ((rnd) as i32) == 0i32 {
                            break 'l3;
                        }
                        rnd = (rnd).wrapping_sub(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((contestant) as i32) as isize * 28))
        .wrapping_add(27))
        .write(((i) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContestantEffectStringID(contestant: u8, effectStringId: u8) {
    unsafe {
        let mut contestant = contestant;
        let mut effectStringId = effectStringId;
        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((contestant) as i32) as isize * 28))
        .wrapping_add(19))
        .write(effectStringId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContestantEffectStringID2(contestant: u8, effectStringId: u8) {
    unsafe {
        let mut contestant = contestant;
        let mut effectStringId = effectStringId;
        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((contestant) as i32) as isize * 28))
        .wrapping_add(20))
        .write(effectStringId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetStartledString(contestant: u8, jam: u8) {
    unsafe {
        let mut contestant = contestant;
        let mut jam = jam;
        if ((jam) as i32) >= 60i32 {
            SetContestantEffectStringID(contestant, 53u8);
        } else {
            if ((jam) as i32) >= 40i32 {
                SetContestantEffectStringID(contestant, 52u8);
            } else {
                if ((jam) as i32) >= 30i32 {
                    SetContestantEffectStringID(contestant, 51u8);
                } else {
                    if ((jam) as i32) >= 20i32 {
                        SetContestantEffectStringID(contestant, 50u8);
                    } else {
                        if ((jam) as i32) >= 10i32 {
                            SetContestantEffectStringID(contestant, 49u8);
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintAppealMoveResultText(contestant: u8, stringId: u8) {
    unsafe {
        let mut contestant = contestant;
        let mut stringId = stringId;
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((contestant) as i32) as isize * 64))
            .wrapping_add(2))
            .cast::<u8>(),
        );
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(6)
                .cast::<u16>())
                .read()) as i32) as isize
                    * 13,
            ))
            .cast::<u8>(),
        );
        if ((crate::c::bf_read(
            (((&raw mut gContestMoves).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(17))
                    .read()) as i32) as isize
                        * 28,
                ))
                .wrapping_add(6)
                .cast::<u16>())
                .read()) as i32) as isize
                    * 8,
            ))
            .wrapping_add(1),
            0,
            3,
            false,
        ) as u8) as i32)
            == 0i32
        {
            StringCopy(
                (&raw mut gStringVar3).cast::<u8>(),
                (&raw mut gText_Contest_Shyness).cast::<u8>(),
            );
        } else {
            if ((crate::c::bf_read(
                (((&raw mut gContestMoves).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(17))
                        .read()) as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
                .wrapping_add(1),
                0,
                3,
                false,
            ) as u8) as i32)
                == 1i32
            {
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (&raw mut gText_Contest_Anxiety).cast::<u8>(),
                );
            } else {
                if ((crate::c::bf_read(
                    (((&raw mut gContestMoves).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(17))
                            .read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 8,
                    ))
                    .wrapping_add(1),
                    0,
                    3,
                    false,
                ) as u8) as i32)
                    == 2i32
                {
                    StringCopy(
                        (&raw mut gStringVar3).cast::<u8>(),
                        (&raw mut gText_Contest_Laziness).cast::<u8>(),
                    );
                } else {
                    if ((crate::c::bf_read(
                        (((&raw mut gContestMoves).cast::<u8>()).wrapping_offset(
                            (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(8)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(17))
                                .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(6)
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 8,
                        ))
                        .wrapping_add(1),
                        0,
                        3,
                        false,
                    ) as u8) as i32)
                        == 3i32
                    {
                        StringCopy(
                            (&raw mut gStringVar3).cast::<u8>(),
                            (&raw mut gText_Contest_Hesitancy).cast::<u8>(),
                        );
                    } else {
                        StringCopy(
                            (&raw mut gStringVar3).cast::<u8>(),
                            (&raw mut gText_Contest_Fear).cast::<u8>(),
                        );
                    }
                }
            }
        }
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            ((((&raw const sAppealResultTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((stringId) as i32) as isize))
            .read(),
        );
        ContestClearGeneralTextWindow();
        Contest_StartTextPrinter((&raw mut gStringVar4).cast::<u8>(), 1u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MakeContestantNervous(p: u8) {
    unsafe {
        let mut p = p;
        crate::c::bf_write(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((p) as i32) as isize * 28))
            .wrapping_add(12),
            0,
            1,
            (1u8) as i32,
        );
        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((p) as i32) as isize * 28))
        .wrapping_add(6)
        .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn ApplyNextTurnOrder() {
    unsafe {
        let mut nextContestant: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut newTurnOrder = crate::ffi::Align4([0u8; 4]);
        let mut isContestantOrdered = crate::ffi::Align4([0u8; 4]);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut newTurnOrder).cast::<u8>()).wrapping_offset((i) as isize)).write(
                        ((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read(),
                    );
                    (((&raw mut isContestantOrdered).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        j = 0i32;
                        'l5: loop {
                            if !(j < 4i32) {
                                break 'l5;
                            }
                            'l6: {
                                if (((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((j) as isize * 28))
                                .wrapping_add(25))
                                .read()) as i32)
                                    == i
                                {
                                    (((&raw mut newTurnOrder).cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                    .write(((i) as u8));
                                    (((&raw mut isContestantOrdered).cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                    .write(1u8);
                                    break 'l5;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if j == 4i32 {
                        {
                            j = 0i32;
                            'l7: loop {
                                if !(j < 4i32) {
                                    break 'l7;
                                }
                                'l8: {
                                    if (!(((((&raw mut isContestantOrdered).cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                    .read())
                                        != 0))
                                        && ((((((((((&raw mut gContestResources)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset((j) as isize * 28))
                                        .wrapping_add(25))
                                        .read())
                                            as i32)
                                            == 255i32)
                                    {
                                        nextContestant = ((j) as u8);
                                        j = (j).wrapping_add(1);
                                        break 'l7;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        {
                            'l9: loop {
                                if !(j < 4i32) {
                                    break 'l9;
                                }
                                'l10: {
                                    if ((!(((((&raw mut isContestantOrdered).cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                    .read())
                                        != 0))
                                        && ((((((((((&raw mut gContestResources)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset((j) as isize * 28))
                                        .wrapping_add(25))
                                        .read())
                                            as i32)
                                            == 255i32))
                                        && (((((((&raw mut gContestantTurnOrder).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(((nextContestant) as i32) as isize))
                                        .read())
                                            as i32)
                                            > ((((((&raw mut gContestantTurnOrder).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32))
                                    {
                                        nextContestant = ((j) as u8);
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        (((&raw mut newTurnOrder).cast::<u8>())
                            .wrapping_offset(((nextContestant) as i32) as isize))
                        .write(((i) as u8));
                        (((&raw mut isContestantOrdered).cast::<u8>())
                            .wrapping_offset(((nextContestant) as i32) as isize))
                        .write(1u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l11: loop {
                if !(i < 4i32) {
                    break 'l11;
                }
                'l12: {
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(
                        (((&raw mut newTurnOrder).cast::<u8>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 28))
                    .wrapping_add(25))
                    .write(255u8);
                    crate::c::bf_write(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(16),
                        6,
                        2,
                        (0u8) as i32,
                    );
                    ((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((&raw mut newTurnOrder).cast::<u8>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_JudgeSpeechBubble(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            > 84i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            crate::c::bf_write(
                (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6),
                4,
                1,
                (0u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DoJudgeSpeechBubble(symbolId: u8) {
    unsafe {
        let mut symbolId = symbolId;
        let mut spriteId: u8 =
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(18))
            .read();
        'l1: {
            let __sw1 = ((symbolId) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 8i32
                || __sw1 == 7i32;
            if __sw1 == 0i32 || __sw1 == 1i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    (((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as u16) as i32,
                );
                PlaySE(32u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    (((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(4i32)) as u16) as i32,
                );
                PlaySE(31u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    (((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(8i32)) as u16) as i32,
                );
                PlaySE(31u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    (((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(12i32)) as u16) as i32,
                );
                PlaySE(45u16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    (((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(12i32)) as u16) as i32,
                );
                PlaySE(45u16);
                break 'l1;
            }
            if __sw1 == 6i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    (((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(16i32)) as u16) as i32,
                );
                PlaySE(45u16);
                break 'l1;
            }
            if __sw1 == 8i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    (((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(24i32)) as u16) as i32,
                );
                PlaySE(195u16);
                break 'l1;
            }
            if __sw1 == 7i32 || !__matched {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    (((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(20i32)) as u16) as i32,
                );
                PlaySE(45u16);
                break 'l1;
            }
        }
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_JudgeSpeechBubble));
        crate::c::bf_write(
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(6),
            4,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateApplauseMeter() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    let mut src: *mut u8 = core::ptr::null_mut();
                    if i < ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(19)
                    .cast::<i8>())
                    .read()) as i32)
                    {
                        src =
                            ((&raw mut gContestApplauseMeterGfx).cast::<u8>()).wrapping_offset(64);
                    } else {
                        src = (&raw mut gContestApplauseMeterGfx).cast::<u8>();
                    }
                    'l3: loop {
                        'l4: {
                            'l5: loop {
                                'l6: {
                                    CpuSet(
                                        src,
                                        (((100728832i32).wrapping_add(
                                            ((((crate::c::bf_read(
                                                (((&raw mut gSprites).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((((((&raw mut gContestResources)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(88))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 68,
                                                    ))
                                                .wrapping_add(4),
                                                0,
                                                10,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                                .wrapping_add(17i32))
                                            .wrapping_add(i))
                                            .wrapping_mul(32i32),
                                        )) as usize
                                            as *mut u8),
                                        ((67108864i32
                                            | (crate::c::div_i32(
                                                32i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l5;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                    'l7: loop {
                        'l8: {
                            'l9: loop {
                                'l10: {
                                    CpuSet(
                                        (src).wrapping_offset(32),
                                        (((100728832i32).wrapping_add(
                                            ((((crate::c::bf_read(
                                                (((&raw mut gSprites).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((((((&raw mut gContestResources)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(88))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 68,
                                                    ))
                                                .wrapping_add(4),
                                                0,
                                                10,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                                .wrapping_add(25i32))
                                            .wrapping_add(i))
                                            .wrapping_mul(32i32),
                                        )) as usize
                                            as *mut u8),
                                        ((67108864i32
                                            | (crate::c::div_i32(
                                                32i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l9;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                    if ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(19)
                    .cast::<i8>())
                    .read()) as i32)
                        > 4i32
                    {
                        StartApplauseOverflowAnimation();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Contest_GetMoveExcitement(r#move: u16) -> i8 {
    unsafe {
        let mut r#move = r#move;
        return ((((((&raw const sContestExcitementTable).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gSpecialVar_ContestCategory)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 5,
            ))
        .cast::<i8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                (((&raw mut gContestMoves).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 8))
                .wrapping_add(1),
                0,
                3,
                false,
            ) as u8) as i32) as isize,
        ))
        .read();
    }
}
pub(crate) unsafe extern "C" fn StartApplauseOverflowAnimation() -> u8 {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_ApplauseOverflowAnimation), 10u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(1i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((IndexOfSpritePaletteTag(44002u16)) as i16));
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_ApplauseOverflowAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 1i32
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
                == 0i32
            {
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4);
                (__p3).write(((__p3).read()).wrapping_add(1));
            } else {
                let __p4 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4);
                (__p4).write(((__p4).read()).wrapping_sub(1));
            }
            BlendPalette(
                ((((256i32).wrapping_add(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        .wrapping_mul(16i32),
                ))
                .wrapping_add(8i32)) as u16),
                1u16,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u8),
                32767u16,
            );
            if (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as i32)
                == 0i32)
                || (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32)
                    == 16i32)
            {
                let __p5 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3);
                (__p5).write((((((__p5).read()) as i32) ^ 1i32) as i16));
                if ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(19)
                .cast::<i8>())
                .read()) as i32)
                    < 5i32
                {
                    BlendPalette(
                        ((((256i32).wrapping_add(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read()) as i32)
                                .wrapping_mul(16i32),
                        ))
                        .wrapping_add(8i32)) as u16),
                        1u16,
                        0u8,
                        31u16,
                    );
                    DestroyTask(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SlideApplauseMeterIn() {
    unsafe {
        CreateTask(Some(Task_SlideApplauseMeterIn), 10u8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(88))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(36)
        .cast::<i16>())
        .write((-70i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(88))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(6),
            6,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_SlideApplauseMeterIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(88))
            .read()) as i32) as isize
                * 68,
        );
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(1664i32)) as i16));
        let __p2 = (sprite).wrapping_add(36).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    >> 8),
            )) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(
            ((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32)
                & 255i32) as i16),
        );
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) > 0i32 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        }
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) == 0i32 {
            crate::c::bf_write(
                (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6),
                6,
                1,
                (0u16) as i32,
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SlideApplauseMeterOut() {
    unsafe {
        if ((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(88))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            false,
        ) as u16) as i32)
            == 1i32
        {
            crate::c::bf_write(
                (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6),
                6,
                1,
                (0u16) as i32,
            );
        } else {
            CreateTask(Some(Task_SlideApplauseMeterOut), 10u8);
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(88))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(0i16);
            crate::c::bf_write(
                (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6),
                6,
                1,
                (1u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SlideApplauseMeterOut(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(88))
            .read()) as i32) as isize
                * 68,
        );
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(1664i32)) as i16));
        let __p2 = (sprite).wrapping_add(36).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_sub(
                (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    >> 8),
            )) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(
            ((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32)
                & 255i32) as i16),
        );
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) < (-70i32) {
            ((sprite).wrapping_add(36).cast::<i16>()).write((-70i16));
        }
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) == (-70i32) {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            crate::c::bf_write(
                (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6),
                6,
                1,
                (0u16) as i32,
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ShowAndUpdateApplauseMeter(unused: i8) {
    unsafe {
        let mut unused = unused;
        let mut taskId: u8 = CreateTask(Some(Task_ShowAndUpdateApplauseMeter), 5u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((unused) as i16));
        crate::c::bf_write(
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(6),
            5,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_ShowAndUpdateApplauseMeter(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32);
            if __sw1 == 0i32 {
                SlideApplauseMeterIn();
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6),
                    6,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t5 = (__p4).read();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    __t5
                }) as i32)
                    > 20i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(0i16);
                    UpdateApplauseMeter();
                    crate::c::bf_write(
                        (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(6),
                        5,
                        1,
                        (0u16) as i32,
                    );
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HideApplauseMeterNoAnim() {
    unsafe {
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(88))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(36)
        .cast::<i16>())
        .write(0i16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(88))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn ShowApplauseMeterNoAnim() {
    unsafe {
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(88))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn AnimateAudience() {
    unsafe {
        CreateTask(Some(Task_AnimateAudience), 15u8);
        crate::c::bf_write(
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(6),
            7,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_AnimateAudience(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            > 6i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read()) as i32)
                == 0i32
            {
                RequestDma3Copy(
                    ((&raw mut gHeap).cast::<u8>()).wrapping_offset(102400),
                    ((100671488i32) as usize as *mut u8),
                    4096u16,
                    1u8,
                );
            } else {
                RequestDma3Copy(
                    ((&raw mut gHeap).cast::<u8>()).wrapping_offset(98304),
                    ((100671488i32) as usize as *mut u8),
                    4096u16,
                    1u8,
                );
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12);
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
            let __p4 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11);
            (__p4).write((((((__p4).read()) as i32) ^ 1i32) as i16));
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12))
            .read()) as i32)
                == 9i32
            {
                crate::c::bf_write(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6),
                    7,
                    1,
                    (0u16) as i32,
                );
                DestroyTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BlendAudienceBackground(excitementDir: i8, blendDir: i8) {
    unsafe {
        let mut excitementDir = excitementDir;
        let mut blendDir = blendDir;
        let mut taskId: u8 = CreateTask(Some(Task_BlendAudienceBackground), 10u8);
        let mut blendColor: u16 = 0u16;
        let mut blendCoeff: u8 = 0u8;
        let mut targetBlendCoeff: u8 = 0u8;
        if ((excitementDir) as i32) > 0i32 {
            blendColor = 9086u16;
            if ((blendDir) as i32) > 0i32 {
                blendCoeff = 0u8;
                targetBlendCoeff =
                    ((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(19)
                    .cast::<i8>())
                    .read()) as i32)
                        .wrapping_mul(3i32)) as u8);
            } else {
                blendCoeff =
                    ((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(19)
                    .cast::<i8>())
                    .read()) as i32)
                        .wrapping_mul(3i32)) as u8);
                targetBlendCoeff = 0u8;
            }
        } else {
            blendColor = 0u16;
            if ((blendDir) as i32) > 0i32 {
                blendCoeff = 0u8;
                targetBlendCoeff = 12u8;
            } else {
                blendCoeff = 12u8;
                targetBlendCoeff = 0u8;
            }
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((blendColor) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((blendCoeff) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((blendDir) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((targetBlendCoeff) as i16));
        crate::c::bf_write(
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(7),
            0,
            1,
            (0u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_BlendAudienceBackground(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            >= 0i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                > 0i32
            {
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p3).write(((__p3).read()).wrapping_add(1));
            } else {
                let __p4 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p4).write(((__p4).read()).wrapping_sub(1));
            }
            BlendPalette(
                17u16,
                1u16,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8),
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u16),
            );
            BlendPalette(
                26u16,
                1u16,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8),
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u16),
            );
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
            {
                DestroyTask(taskId);
                crate::c::bf_write(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(7),
                    0,
                    1,
                    (0u16) as i32,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowHideNextTurnGfx(show: u8) {
    unsafe {
        let mut show = show;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(16),
                        6,
                        2,
                        false,
                    ) as u8) as i32)
                        != 0i32)
                        && ((show) != 0)
                    {
                        'l3: loop {
                            'l4: {
                                'l5: loop {
                                    'l6: {
                                        CpuSet(
                                            GetTurnOrderNumberGfx(((i) as u8)),
                                            (((100728832i32).wrapping_add(
                                                (((crate::c::bf_read(
                                                    (((&raw mut gSprites).cast::<u8>())
                                                        .wrapping_offset(
                                                        (((((((((&raw mut gContestResources)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(20)
                                                        .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_offset((i) as isize * 4))
                                                        .wrapping_add(1))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 68,
                                                    ))
                                                    .wrapping_add(4),
                                                    0,
                                                    10,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    .wrapping_add(6i32))
                                                .wrapping_mul(32i32),
                                            )) as usize
                                                as *mut u8),
                                            ((67108864i32
                                                | (crate::c::div_i32(
                                                    32i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l5;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l3;
                            }
                        }
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(1))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(34)
                        .cast::<i16>())
                        .write(
                            ((((((&raw const sNextTurnSpriteYPositions)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i16),
                        );
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 4))
                                .wrapping_add(1))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (0u16) as i32,
                        );
                    } else {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 4))
                                .wrapping_add(1))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetTurnOrderNumberGfx(contestant: u8) -> *mut u8 {
    unsafe {
        let mut contestant = contestant;
        if ((crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(16),
            6,
            2,
            false,
        ) as u8) as i32)
            != 1i32
        {
            return (&raw mut gContestNextTurnRandomGfx).cast::<u8>();
        } else {
            return ((&raw mut gContestNextTurnNumbersGfx).cast::<u8>()).wrapping_offset(
                ((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(25))
                .read()) as i32)
                    .wrapping_mul(32i32)) as isize,
            );
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
pub(crate) unsafe extern "C" fn DrawUnnervedSymbols() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(13))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 0i32)
                        && (!((Contest_IsMonsTurnDisabled(((i) as u8))) != 0))
                    {
                        let mut contestantOffset: u32 =
                            (((((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32)
                                .wrapping_mul(5i32))
                            .wrapping_add(2i32)) as u32);
                        let mut symbolOffset: u16 = GetStatusSymbolTileOffset(3u8);
                        ContestBG_FillBoxWithIncrementingTile(
                            0u8,
                            symbolOffset,
                            20u8,
                            ((contestantOffset) as u8),
                            2u8,
                            1u8,
                            17u8,
                            1i16,
                        );
                        symbolOffset = ((((symbolOffset) as i32).wrapping_add(16i32)) as u16);
                        ContestBG_FillBoxWithIncrementingTile(
                            0u8,
                            symbolOffset,
                            20u8,
                            (((contestantOffset).wrapping_add(1u32)) as u8),
                            2u8,
                            1u8,
                            17u8,
                            1i16,
                        );
                        PlaySE(99u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsContestantAllowedToCombo(contestant: u8) -> u8 {
    unsafe {
        let mut contestant = contestant;
        if ((crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(21),
            0,
            1,
            false,
        ) as u8)
            != 0)
            || ((crate::c::bf_read(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(12),
                0,
                1,
                false,
            ) as u8)
                != 0)
        {
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SetBgForCurtainDrop() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut bg0Cnt: u16 = 0u16;
        let mut bg1Cnt: u16 = 0u16;
        let mut bg2Cnt: u16 = 0u16;
        bg1Cnt = GetGpuReg(10u8);
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(0),
            0,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(1),
            6,
            2,
            (2u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(1),
            5,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(0),
            2,
            2,
            (0u16) as i32,
        );
        SetGpuReg(10u8, bg1Cnt);
        bg0Cnt = GetGpuReg(8u8);
        bg2Cnt = GetGpuReg(12u8);
        crate::c::bf_write(
            ((&raw mut bg0Cnt).cast::<u8>()).wrapping_add(0),
            0,
            2,
            (1u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut bg2Cnt).cast::<u8>()).wrapping_add(0),
            0,
            2,
            (1u16) as i32,
        );
        SetGpuReg(8u8, bg0Cnt);
        SetGpuReg(12u8, bg2Cnt);
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(240u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(160u16);
        SetGpuReg(20u8, ((&raw mut gBattle_BG1_X).cast::<u16>()).read());
        SetGpuReg(22u8, ((&raw mut gBattle_BG1_Y).cast::<u16>()).read());
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(36))
                                .cast::<*mut u8>())
                                .wrapping_offset(1))
                                .read(),
                                ((83886080i32
                                    | (crate::c::div_i32(4096i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        CopyToBgTilemapBuffer(
            1u8,
            (((&raw mut gContestCurtainTilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u16,
            0u16,
        );
        Contest_SetBgCopyFlags(1u32);
        {
            i = 0i32;
            'l5: loop {
                if !(i < 4i32) {
                    break 'l5;
                }
                'l6: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 4))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(1))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        (1u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateContestantBoxOrder() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut bg1Cnt: u16 = 0u16;
        RequestDma3Fill(0i32, ((100696064i32) as usize as *mut u8), 8192u16, 1u8);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(36))
                                .cast::<*mut u8>())
                                .wrapping_offset(1))
                                .read(),
                                ((83886080i32
                                    | (crate::c::div_i32(4096i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        Contest_SetBgCopyFlags(1u32);
        bg1Cnt = GetGpuReg(10u8);
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(0),
            0,
            2,
            (1u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(1),
            6,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(1),
            5,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut bg1Cnt).cast::<u8>()).wrapping_add(0),
            2,
            2,
            (2u16) as i32,
        );
        SetGpuReg(10u8, bg1Cnt);
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        {
            i = 0i32;
            'l5: loop {
                if !(i < 4i32) {
                    break 'l5;
                }
                'l6: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 4))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(1))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        (0u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartDropCurtainAtRoundEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(160u16);
        PlaySE12WithPanning(98u16, 0i8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_UpdateCurtainDropAtRoundEnd));
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateCurtainDropAtRoundEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((({
            let __p1 = (&raw mut gBattle_BG1_Y).cast::<u16>();
            let __v2 = (((((__p1).read()) as i32).wrapping_sub(7i32)) as u16);
            (__p1).write(__v2);
            __v2
        }) as i16) as i32)
            < 0i32
        {
            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        }
        if ((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i32) == 0i32 {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ResetForNextRound));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ResetForNextRound(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(20))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                FillContestantWindowBgs();
                UpdateBlendTaskContestantsData();
                DrawConditionStars();
                DrawContestantWindows();
                ShowHideNextTurnGfx(1u8);
                UpdateSliderHeartSpriteYPositions();
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32)
                    & 1i32)
                    != 0
                {
                    let mut taskId2: u8 = 0u8;
                    crate::c::bf_write(
                        (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(7),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    if (IsPlayerLinkLeader()) != 0 {
                        SetContestantStatusesForNextRound();
                    }
                    taskId2 = CreateTask(Some(Task_LinkContest_CommunicateAppealsState), 0u8);
                    SetTaskFuncWithFollowupFunc(
                        taskId2,
                        Some(Task_LinkContest_CommunicateAppealsState),
                        Some(Task_EndWaitForLink),
                    );
                    ContestPrintLinkStandby();
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                } else {
                    SetContestantStatusesForNextRound();
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(7),
                    2,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                DrawStatusSymbols();
                SwapMoveDescAndContestTilemaps();
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_WaitRaiseCurtainAtRoundEnd));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateRaiseCurtainAtRoundEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((({
            let __p1 = (&raw mut gBattle_BG1_Y).cast::<u16>();
            let __v2 = (((((__p1).read()) as i32).wrapping_add(7i32)) as u16);
            (__p1).write(__v2);
            __v2
        }) as i16) as i32)
            > 160i32
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_UpdateContestantBoxOrder));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitRaiseCurtainAtRoundEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            < 10i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == 0i32
            {
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    == 16i32
                {
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                } else {
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
            } else {
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    == 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(0i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_StartRaiseCurtainAtRoundEnd));
                } else {
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_sub(1));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartRaiseCurtainAtRoundEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            < 10i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
            PlaySE12WithPanning(97u16, 0i8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_UpdateRaiseCurtainAtRoundEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimateSliderHearts(animId: u8) {
    unsafe {
        let mut animId = animId;
        let mut i: i32 = 0i32;
        let mut taskId: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 4))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(3),
                        1,
                        5,
                        ((AllocOamMatrix()) as u32) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 4))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(1),
                        0,
                        2,
                        (1u32) as i32,
                    );
                    StartSpriteAffineAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 4))
                            .read()) as i32) as isize
                                * 68,
                        ),
                        animId,
                    );
                    if ((animId) as i32) == 2i32 {
                        AnimateSprite(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 4))
                                .read()) as i32) as isize
                                    * 68,
                            ),
                        );
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 4))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (0u16) as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        taskId = CreateTask(Some(Task_WaitForSliderHeartAnim), 5u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((animId) as i16));
        crate::c::bf_write(
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(7),
            1,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForSliderHeartAnim(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(63),
            5,
            1,
            false,
        ) as u16)
            != 0
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u8) as i32)
                == 1i32
            {
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i < 4i32) {
                            break 'l1;
                        }
                        'l2: {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut gContestResources)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(20)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset((i) as isize * 4))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(62),
                                2,
                                1,
                                (1u16) as i32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        FreeSpriteOamMatrix(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 4))
                                .read()) as i32) as isize
                                    * 68,
                            ),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            crate::c::bf_write(
                (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(7),
                1,
                1,
                (0u16) as i32,
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SanitizeMove(r#move: u16) -> u16 {
    unsafe {
        let mut r#move = r#move;
        if ((r#move) as i32) >= 355i32 {
            r#move = 1u16;
        }
        return r#move;
    }
}
pub(crate) unsafe extern "C" fn SanitizeSpecies(species: u16) -> u16 {
    unsafe {
        let mut species = species;
        if ((species) as i32) >= 412i32 {
            species = 0u16;
        }
        return species;
    }
}
pub(crate) unsafe extern "C" fn SetMoveSpecificAnimData(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        let mut i: i32 = 0i32;
        let mut r#move: u16 = SanitizeMove(
            (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(6)
            .cast::<u16>())
            .read(),
        );
        let mut species: u16 = SanitizeSpecies(
            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((contestant) as i32) as isize * 64))
            .cast::<u16>())
            .read(),
        );
        let mut targetContestant: u8 = 0u8;
        crate::c::memset(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .cast::<u16>())
            .cast::<u8>(),
            0i32,
            20u32,
        );
        ClearBattleAnimationVars();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut gBattleMonForms).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        'l3: {
            let __sw1 = ((r#move) as i32);
            if __sw1 == 174i32 {
                if ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .wrapping_add(6))
                .cast::<u8>())
                .read()) as i32)
                    == 7i32)
                    || (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(6))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        == 7i32)
                {
                    ((&raw mut gAnimMoveTurn).cast::<u8>()).write(0u8);
                } else {
                    ((&raw mut gAnimMoveTurn).cast::<u8>()).write(1u8);
                }
                break 'l3;
            }
            if __sw1 == 144i32 || __sw1 == 272i32 {
                targetContestant =
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(27))
                    .read();
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2)
                .cast::<u16>())
                .write(SanitizeSpecies(
                    (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((targetContestant) as i32) as isize * 64))
                    .cast::<u16>())
                    .read(),
                ));
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(16)
                .cast::<u32>())
                .write(
                    (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((targetContestant) as i32) as isize * 64))
                    .wrapping_add(56)
                    .cast::<u32>())
                    .read(),
                );
                crate::c::bf_write(
                    (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4),
                    0,
                    1,
                    (1u8) as i32,
                );
                break 'l3;
            }
            if __sw1 == 216i32 {
                ((&raw mut gAnimFriendship).cast::<u8>()).write(255u8);
                break 'l3;
            }
            if __sw1 == 218i32 {
                ((&raw mut gAnimFriendship).cast::<u8>()).write(0u8);
                break 'l3;
            }
            if __sw1 == 76i32 || __sw1 == 13i32 || __sw1 == 130i32 || __sw1 == 143i32 {
                if ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(90))
                .read()) as i32)
                    == 0i32
                {
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(90))
                    .write(2u8);
                    ((&raw mut gAnimMoveTurn).cast::<u8>()).write(0u8);
                } else {
                    ((&raw mut gAnimMoveTurn).cast::<u8>()).write(1u8);
                }
                break 'l3;
            }
        }
        SetBattleTargetSpritePosition();
    }
}
pub(crate) unsafe extern "C" fn ClearMoveAnimData(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        crate::c::memset(
            ((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read(),
            0i32,
            20u32,
        );
        if ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(90))
        .read()) as i32)
            != 0i32
        {
            let __p1 = (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(90);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn SetMoveAnimAttackerData(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(5))
        .write(contestant);
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<*mut u8>())
        .read())
        .cast::<u16>())
        .write(SanitizeSpecies(
            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((contestant) as i32) as isize * 64))
            .cast::<u16>())
            .read(),
        ));
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(8)
        .cast::<u32>())
        .write(
            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((contestant) as i32) as isize * 64))
            .wrapping_add(56)
            .cast::<u32>())
            .read(),
        );
        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(12)
        .cast::<u32>())
        .write(
            (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((contestant) as i32) as isize * 64))
            .wrapping_add(60)
            .cast::<u32>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateInvisibleBattleTargetSprite() {
    unsafe {
        (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(3))
            .write(CreateInvisibleSpriteWithCallback(Some(SpriteCallbackDummy)));
        InitSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        SetBattleTargetSpritePosition();
    }
}
pub(crate) unsafe extern "C" fn SetBattleTargetSpritePosition() {
    unsafe {
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(3)).read()) as i32)
                as isize
                * 68,
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(32).cast::<i16>()).write(((GetBattlerSpriteCoord(3u8, 0u8)) as i16));
        ((sprite).wrapping_add(34).cast::<i16>()).write(((GetBattlerSpriteCoord(3u8, 1u8)) as i16));
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
    }
}
pub(crate) unsafe extern "C" fn SetMoveTargetPosition(r#move: u16) {
    unsafe {
        let mut r#move = r#move;
        'l1: {
            let __sw1 = ((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(6))
            .read()) as i32);
            let __matched = __sw1 == 2i32
                || __sw1 == 16i32
                || __sw1 == 0i32
                || __sw1 == 4i32
                || __sw1 == 8i32
                || __sw1 == 32i32;
            if __sw1 == 2i32 || __sw1 == 16i32 {
                ((&raw mut gBattlerTarget).cast::<u8>()).write(2u8);
                break 'l1;
            }
            if __sw1 == 0i32 || __sw1 == 4i32 || __sw1 == 8i32 || __sw1 == 32i32 || !__matched {
                ((&raw mut gBattlerTarget).cast::<u8>()).write(3u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Contest_PrintTextToBg0WindowStd(windowId: u32, b: *mut u8) {
    unsafe {
        let mut windowId = windowId;
        let mut b = b;
        let mut printerTemplate = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printerTemplate).cast::<u8>()).cast::<*mut u8>()).write(b);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(4)).write(((windowId) as u8));
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(5)).write(1u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(6)).write(0u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(7)).write(1u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(8)).write(0u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(9)).write(1u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(11)).write(0u8);
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (15u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (8u8) as i32,
        );
        AddTextPrinter((&raw mut printerTemplate).cast::<u8>(), 0u8, None);
        PutWindowTilemap(((windowId) as u8));
        Contest_SetBgCopyFlags(0u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Contest_PrintTextToBg0WindowAt(
    windowId: u32,
    currChar: *mut u8,
    x: i32,
    y: i32,
    fontId: i32,
) {
    unsafe {
        let mut windowId = windowId;
        let mut currChar = currChar;
        let mut x = x;
        let mut y = y;
        let mut fontId = fontId;
        let mut printerTemplate = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printerTemplate).cast::<u8>()).cast::<*mut u8>()).write(currChar);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(4)).write(((windowId) as u8));
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(5)).write(((fontId) as u8));
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(6)).write(((x) as u8));
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(7)).write(((y) as u8));
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(8)).write(((x) as u8));
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(9)).write(((y) as u8));
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(11)).write(0u8);
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (15u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (8u8) as i32,
        );
        AddTextPrinter((&raw mut printerTemplate).cast::<u8>(), 0u8, None);
        PutWindowTilemap(((windowId) as u8));
        Contest_SetBgCopyFlags(0u32);
    }
}
pub(crate) unsafe extern "C" fn Contest_StartTextPrinter(currChar: *mut u8, b: u32) {
    unsafe {
        let mut currChar = currChar;
        let mut b = b;
        let mut printerTemplate = crate::ffi::Align4([0u8; 16]);
        let mut speed: u8 = 0u8;
        (((&raw mut printerTemplate).cast::<u8>()).cast::<*mut u8>()).write(currChar);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(4)).write(4u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(5)).write(1u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(6)).write(0u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(7)).write(1u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(8)).write(0u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(9)).write(1u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(11)).write(0u8);
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (1u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (8u8) as i32,
        );
        if !((b) != 0) {
            AddTextPrinter((&raw mut printerTemplate).cast::<u8>(), 0u8, None);
        } else {
            if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32)
                != 0
            {
                speed = 4u8;
            } else {
                speed = GetPlayerTextSpeedDelay();
            }
            AddTextPrinter((&raw mut printerTemplate).cast::<u8>(), speed, None);
        }
        PutWindowTilemap(4u8);
        Contest_SetBgCopyFlags(0u32);
    }
}
pub(crate) unsafe extern "C" fn ContestBG_FillBoxWithIncrementingTile(
    bg: u8,
    firstTileNum: u16,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    paletteSlot: u8,
    tileNumData: i16,
) {
    unsafe {
        let mut bg = bg;
        let mut firstTileNum = firstTileNum;
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        let mut paletteSlot = paletteSlot;
        let mut tileNumData = tileNumData;
        WriteSequenceToBgTilemapBuffer(
            bg,
            firstTileNum,
            x,
            y,
            width,
            height,
            paletteSlot,
            tileNumData,
        );
        Contest_SetBgCopyFlags(((bg) as u32));
    }
}
pub(crate) unsafe extern "C" fn ContestBG_FillBoxWithTile(
    bg: u8,
    firstTileNum: u16,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    paletteSlot: u8,
) {
    unsafe {
        let mut bg = bg;
        let mut firstTileNum = firstTileNum;
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        let mut paletteSlot = paletteSlot;
        ContestBG_FillBoxWithIncrementingTile(
            bg,
            firstTileNum,
            x,
            y,
            width,
            height,
            paletteSlot,
            0i16,
        );
    }
}
pub(crate) unsafe extern "C" fn Contest_RunTextPrinters() -> u32 {
    unsafe {
        RunTextPrinters();
        return ((IsTextPrinterActive(4u8)) as u32);
    }
}
pub(crate) unsafe extern "C" fn Contest_SetBgCopyFlags(flagIndex: u32) {
    unsafe {
        let mut flagIndex = flagIndex;
        let __p1 = (&raw mut sContestBgCopyFlags).cast::<u8>().cast::<u8>();
        (__p1).write((((((__p1).read()) as i32) | crate::c::shl_i32(1i32, flagIndex)) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetContestLinkResults() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1572))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                                .cast::<u16>())
                                .wrapping_offset((j) as isize))
                                .write(0u16);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveContestWinner(rank: u8) -> u8 {
    unsafe {
        let mut rank = rank;
        let mut i: i32 = 0i32;
        let mut captionId: u8 = ((crate::c::rem_i32(((Random()) as i32), 3i32)) as u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gContestFinalStandings).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((rank) as i32) == 255i32)
            && (i
                != ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32))
        {
            return 0u8;
        }
        'l3: {
            let __sw1 = ((((&raw mut gSpecialVar_ContestCategory)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                captionId = ((((captionId) as i32).wrapping_add(0i32)) as u8);
                break 'l3;
            }
            if __sw1 == 1i32 {
                captionId = ((((captionId) as i32).wrapping_add(3i32)) as u8);
                break 'l3;
            }
            if __sw1 == 2i32 {
                captionId = ((((captionId) as i32).wrapping_add(6i32)) as u8);
                break 'l3;
            }
            if __sw1 == 3i32 {
                captionId = ((((captionId) as i32).wrapping_add(9i32)) as u8);
                break 'l3;
            }
            if __sw1 == 4i32 {
                captionId = ((((captionId) as i32).wrapping_add(12i32)) as u8);
                break 'l3;
            }
        }
        if ((rank) as i32) != 254i32 {
            let mut id: u8 = GetContestWinnerSaveIdx(rank, 1u8);
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11920))
                .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 32))
            .cast::<u32>())
            .write(
                (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .wrapping_add(56)
                .cast::<u32>())
                .read(),
            );
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11920))
                .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 32))
            .wrapping_add(8)
            .cast::<u16>())
            .write(
                (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .cast::<u16>())
                .read(),
            );
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11920))
                .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 32))
            .wrapping_add(4)
            .cast::<u32>())
            .write(
                (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .wrapping_add(60)
                .cast::<u32>())
                .read(),
            );
            StringCopy(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11920))
                    .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 32))
                .wrapping_add(11))
                .cast::<u8>(),
                (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .wrapping_add(2))
                .cast::<u8>(),
            );
            StringCopy(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11920))
                    .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 32))
                .wrapping_add(22))
                .cast::<u8>(),
                (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .wrapping_add(13))
                .cast::<u8>(),
            );
            if (((((&raw mut gLinkContestFlags).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32)
                != 0
            {
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11920))
                    .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 32))
                .wrapping_add(30))
                .write(4u8);
            } else {
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11920))
                    .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 32))
                .wrapping_add(30))
                .write(
                    ((((&raw mut gSpecialVar_ContestRank)
                        .cast::<u8>()
                        .cast::<u16>())
                    .read()) as u8),
                );
            }
            if ((rank) as i32) != 255i32 {
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11920))
                    .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 32))
                .wrapping_add(10))
                .write(
                    ((((&raw mut gSpecialVar_ContestCategory)
                        .cast::<u8>()
                        .cast::<u16>())
                    .read()) as u8),
                );
            } else {
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11920))
                    .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 32))
                .wrapping_add(10))
                .write(captionId);
            }
        } else {
            (((&raw mut gCurContestWinner).cast::<u8>()).cast::<u32>()).write(
                (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .wrapping_add(56)
                .cast::<u32>())
                .read(),
            );
            (((&raw mut gCurContestWinner).cast::<u8>())
                .wrapping_add(4)
                .cast::<u32>())
            .write(
                (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .wrapping_add(60)
                .cast::<u32>())
                .read(),
            );
            (((&raw mut gCurContestWinner).cast::<u8>())
                .wrapping_add(8)
                .cast::<u16>())
            .write(
                (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .cast::<u16>())
                .read(),
            );
            StringCopy(
                (((&raw mut gCurContestWinner).cast::<u8>()).wrapping_add(11)).cast::<u8>(),
                (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .wrapping_add(2))
                .cast::<u8>(),
            );
            StringCopy(
                (((&raw mut gCurContestWinner).cast::<u8>()).wrapping_add(22)).cast::<u8>(),
                (((((&raw mut gContestMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .wrapping_add(13))
                .cast::<u8>(),
            );
            (((&raw mut gCurContestWinner).cast::<u8>()).wrapping_add(10)).write(captionId);
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestWinnerSaveIdx(rank: u8, shift: u8) -> u8 {
    unsafe {
        let mut rank = rank;
        let mut shift = shift;
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = ((rank) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 {
                if (shift) != 0 {
                    {
                        i = 5i32;
                        'l2: loop {
                            if !(i > 0i32) {
                                break 'l2;
                            }
                            'l3: {
                                crate::c::memcpy(
                                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(11920))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 32),
                                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(11920))
                                    .cast::<u8>())
                                    .wrapping_offset(((i).wrapping_sub(1i32)) as isize * 32),
                                    32u32,
                                );
                            }
                            i = (i).wrapping_sub(1);
                        }
                    }
                }
                return 0u8;
            }
            if !__matched {
                'l4: {
                    let __sw2 = ((((&raw mut gSpecialVar_ContestCategory)
                        .cast::<u8>()
                        .cast::<u16>())
                    .read()) as i32);
                    let __matched = __sw2 == 0i32
                        || __sw2 == 1i32
                        || __sw2 == 2i32
                        || __sw2 == 3i32
                        || __sw2 == 4i32;
                    if __sw2 == 0i32 {
                        return 8u8;
                    }
                    if __sw2 == 1i32 {
                        return 9u8;
                    }
                    if __sw2 == 2i32 {
                        return 10u8;
                    }
                    if __sw2 == 3i32 {
                        return 11u8;
                    }
                    if __sw2 == 4i32 || !__matched {
                        return 12u8;
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearContestWinnerPicsInContestHall() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11920))
                        .cast::<u8>())
                    .wrapping_offset((i) as isize * 32)
                    .cast::<crate::c::Rec4<32>>()
                    .write_unaligned(
                        (((&raw const gDefaultContestWinners).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 32)
                        .cast::<crate::c::Rec4<32>>()
                        .read_unaligned(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetContestLiveUpdateFlags(contestant: u8) {
    unsafe {
        let mut contestant = contestant;
        let mut i: i32 = 0i32;
        if ((!((crate::c::bf_read(
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1),
            0,
            1,
            false,
        ) as u8)
            != 0))
            && (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .cast::<i8>())
            .read()) as i32)
                > 0i32))
            && (!((crate::c::bf_read(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(21),
                0,
                1,
                false,
            ) as u8)
                != 0))
        {
            let __p1 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 16))
            .wrapping_add(12);
            (__p1).write((((((__p1).read()) as i32) | 1i32) as u8));
            crate::c::bf_write(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 16))
                .wrapping_add(14),
                1,
                1,
                (1u8) as i32,
            );
        }
        if (crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(12),
            0,
            1,
            false,
        ) as u8)
            != 0
        {
            let __p2 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 16))
            .wrapping_add(12);
            (__p2).write((((((__p2).read()) as i32) | 2i32) as u8));
        }
        if ((!((crate::c::bf_read(
            (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1),
            0,
            1,
            false,
        ) as u8)
            != 0))
            && (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .cast::<i8>())
            .read()) as i32)
                != 0i32))
            && (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2)
            .cast::<i8>())
            .read()) as i32)
                == 60i32)
        {
            let __p3 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 16))
            .wrapping_add(12);
            (__p3).write((((((__p3).read()) as i32) | 4i32) as u8));
        }
        if ((crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(21),
            6,
            1,
            false,
        ) as u8)
            != 0)
            && (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(22))
            .read())
                != 0)
        {
            let __p4 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 16))
            .wrapping_add(12);
            (__p4).write((((((__p4).read()) as i32) | 8i32) as u8));
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (i != ((contestant) as i32))
                        && ((((((((((&raw mut gContestResources)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 28))
                        .wrapping_add(14))
                        .read()) as i32)
                            != 0i32)
                    {
                        let __p5 =
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(28)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((contestant) as i32) as isize * 16))
                            .wrapping_add(12);
                        (__p5).write((((((__p5).read()) as i32) | 16i32) as u8));
                        let __p6 =
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(28)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 16))
                            .wrapping_add(12);
                        (__p6).write((((((__p6).read()) as i32) | 64i32) as u8));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(12),
            1,
            2,
            false,
        ) as u8) as i32)
            != 0i32)
            || ((crate::c::bf_read(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(11),
                7,
                1,
                false,
            ) as u8)
                != 0)
        {
            let __p7 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 16))
            .wrapping_add(12);
            (__p7).write((((((__p7).read()) as i32) | 32i32) as u8));
        } else {
            if !((crate::c::bf_read(
                ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 28))
                .wrapping_add(12),
                0,
                1,
                false,
            ) as u8)
                != 0)
            {
                let __p8 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(28)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 16))
                .wrapping_add(12);
                (__p8).write((((((__p8).read()) as i32) | 128i32) as u8));
                crate::c::bf_write(
                    ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(28)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 16))
                    .wrapping_add(14),
                    0,
                    1,
                    (1u8) as i32,
                );
                ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((contestant) as i32) as isize * 16))
                .cast::<u16>())
                .wrapping_offset(
                    ((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(1))
                    .read()) as i32) as isize,
                ))
                .write(
                    (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((contestant) as i32) as isize * 28))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read(),
                );
            }
        }
        if (crate::c::bf_read(
            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 28))
            .wrapping_add(21),
            0,
            1,
            false,
        ) as u8)
            != 0
        {
            let __p9 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 16))
            .wrapping_add(13);
            (__p9).write((((((__p9).read()) as i32) | 2i32) as u8));
        }
        if ((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(19)
        .cast::<i8>())
        .read()) as i32)
            == 4i32)
            && (!((crate::c::bf_read(
                (((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1),
                0,
                1,
                false,
            ) as u8)
                != 0)))
            && (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .cast::<i8>())
            .read()) as i32)
                < 0i32)
        {
            let __p10 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(28)
            .cast::<*mut u8>())
            .read())
            .wrapping_offset(((contestant) as i32) as isize * 16))
            .wrapping_add(13);
            (__p10).write((((((__p10).read()) as i32) | 32i32) as u8));
        }
    }
}
pub(crate) unsafe extern "C" fn CalculateContestLiveUpdateData() {
    unsafe {
        let mut loser: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut notLastInRound1: u32 = 0u32;
        let mut notLastInRound2: u32 = 0u32;
        let mut appealMoves = crate::ffi::Align4([0u8; 12]);
        let mut numMoveUses = crate::ffi::Align4([0u8; 6]);
        let mut moveCandidates = crate::ffi::Align4([0u8; 10]);
        let mut winner: u8 = 0u8;
        let mut mostUses: u8 = 0u8;
        let mut numMoveCandidates: u8 = 0u8;
        loser = 0u8;
        winner = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gContestFinalStandings).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        winner = ((i) as u8);
                    } else {
                        if ((((((&raw mut gContestFinalStandings).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            == 3i32
                        {
                            loser = ((i) as u8);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p1 = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((loser) as i32) as isize * 16))
        .wrapping_add(13);
        (__p1).write((((((__p1).read()) as i32) | 1i32) as u8));
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    if (i != ((winner) as i32))
                        && (((((((&raw mut gContestMonTotalPoints).cast::<u8>().cast::<i16>())
                            .cast::<i16>())
                        .wrapping_offset(((winner) as i32) as isize))
                        .read()) as i32)
                            .wrapping_sub(
                                ((((((&raw mut gContestMonTotalPoints)
                                    .cast::<u8>()
                                    .cast::<i16>())
                                .cast::<i16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32),
                            )
                            <= 50i32)
                    {
                        let __p2 =
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(28)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 16))
                            .wrapping_add(13);
                        (__p2).write((((((__p2).read()) as i32) | 4i32) as u8));
                    }
                    if !((crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(28)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 16))
                        .wrapping_add(14),
                        1,
                        1,
                        false,
                    ) as u8)
                        != 0)
                    {
                        let __p3 =
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(28)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 16))
                            .wrapping_add(13);
                        (__p3).write((((((__p3).read()) as i32) | 8i32) as u8));
                    }
                    {
                        j = 0i32;
                        'l5: loop {
                            if !(j < 4i32) {
                                break 'l5;
                            }
                            'l6: {
                                if ((((((&raw mut gContestMonRound1Points)
                                    .cast::<u8>()
                                    .cast::<i16>())
                                .cast::<i16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    < ((((((&raw mut gContestMonRound1Points)
                                        .cast::<u8>()
                                        .cast::<i16>())
                                    .cast::<i16>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                {
                                    break 'l5;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if (j == 4i32)
                        && (((((((&raw mut gContestFinalStandings).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            != 0i32)
                    {
                        let __p4 =
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(28)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 16))
                            .wrapping_add(13);
                        (__p4).write((((((__p4).read()) as i32) | 16i32) as u8));
                    }
                    notLastInRound1 = 0u32;
                    notLastInRound2 = 0u32;
                    {
                        j = 0i32;
                        'l7: loop {
                            if !(j < 4i32) {
                                break 'l7;
                            }
                            'l8: {
                                if ((((((&raw mut gContestMonRound1Points)
                                    .cast::<u8>()
                                    .cast::<i16>())
                                .cast::<i16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    > ((((((&raw mut gContestMonRound1Points)
                                        .cast::<u8>()
                                        .cast::<i16>())
                                    .cast::<i16>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                {
                                    notLastInRound1 = 1u32;
                                }
                                if ((((((&raw mut gContestMonRound2Points)
                                    .cast::<u8>()
                                    .cast::<i16>())
                                .cast::<i16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    > ((((((&raw mut gContestMonRound2Points)
                                        .cast::<u8>()
                                        .cast::<i16>())
                                    .cast::<i16>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                {
                                    notLastInRound2 = 1u32;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if (!((notLastInRound1) != 0)) && (!((notLastInRound2) != 0)) {
                        let __p5 =
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(28)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 16))
                            .wrapping_add(13);
                        (__p5).write((((((__p5).read()) as i32) | 64i32) as u8));
                    }
                    if !((crate::c::bf_read(
                        ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(28)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 16))
                        .wrapping_add(14),
                        0,
                        1,
                        false,
                    ) as u8)
                        != 0)
                    {
                        let __p6 =
                            ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(28)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 16))
                            .wrapping_add(13);
                        (__p6).write((((((__p6).read()) as i32) | 128i32) as u8));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l9: loop {
                if !(i < 5i32) {
                    break 'l9;
                }
                'l10: {
                    (((&raw mut appealMoves).cast::<u16>()).wrapping_offset((i) as isize))
                        .write(0u16);
                    (((&raw mut numMoveUses).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut appealMoves).cast::<u16>()).wrapping_offset(5)).write(65535u16);
        (((&raw mut numMoveUses).cast::<u8>()).wrapping_offset(5)).write(0u8);
        {
            i = 0i32;
            'l11: loop {
                if !(i < 5i32) {
                    break 'l11;
                }
                'l12: {
                    if ((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((winner) as i32) as isize * 16))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        {
                            j = 0i32;
                            'l13: loop {
                                if !(j < 5i32) {
                                    break 'l13;
                                }
                                'l14: {
                                    if ((((((((((&raw mut gContestResources)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(28)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((winner) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        != (((((&raw mut appealMoves).cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                    {
                                        if (((((&raw mut appealMoves).cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            == 0i32
                                        {
                                            (((&raw mut appealMoves).cast::<u16>())
                                                .wrapping_offset((j) as isize))
                                            .write(
                                                ((((((((&raw mut gContestResources)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(28)
                                                .cast::<*mut u8>())
                                                .read())
                                                .wrapping_offset(
                                                    ((winner) as i32) as isize * 16,
                                                ))
                                                .cast::<u16>())
                                                .wrapping_offset((i) as isize))
                                                .read(),
                                            );
                                            let __p7 = ((&raw mut numMoveUses).cast::<u8>())
                                                .wrapping_offset((j) as isize);
                                            (__p7).write(((__p7).read()).wrapping_add(1));
                                        }
                                    } else {
                                        let __p8 = ((&raw mut numMoveUses).cast::<u8>())
                                            .wrapping_offset((j) as isize);
                                        (__p8).write(((__p8).read()).wrapping_add(1));
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut moveCandidates).cast::<u16>())
            .write(((&raw mut appealMoves).cast::<u16>()).read());
        mostUses = ((&raw mut numMoveUses).cast::<u8>()).read();
        numMoveCandidates = 0u8;
        {
            i = 1i32;
            'l15: loop {
                if !((((((&raw mut appealMoves).cast::<u16>()).wrapping_offset((i) as isize))
                    .read()) as i32)
                    != 65535i32)
                {
                    break 'l15;
                }
                'l16: {
                    if ((mostUses) as i32)
                        < (((((&raw mut numMoveUses).cast::<u8>()).wrapping_offset((i) as isize))
                            .read()) as i32)
                    {
                        ((&raw mut moveCandidates).cast::<u16>()).write(
                            (((&raw mut appealMoves).cast::<u16>()).wrapping_offset((i) as isize))
                                .read(),
                        );
                        mostUses = (((&raw mut numMoveUses).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read();
                        numMoveCandidates = 1u8;
                    } else {
                        if ((mostUses) as i32)
                            == (((((&raw mut numMoveUses).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            (((&raw mut moveCandidates).cast::<u16>())
                                .wrapping_offset(((numMoveCandidates) as i32) as isize))
                            .write(
                                (((&raw mut appealMoves).cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                .read(),
                            );
                            numMoveCandidates = (numMoveCandidates).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((winner) as i32) as isize * 16))
        .wrapping_add(10)
        .cast::<i16>())
        .write(
            (((((&raw mut moveCandidates).cast::<u16>()).wrapping_offset(
                (crate::c::rem_i32(((Random()) as i32), ((numMoveCandidates) as i32))) as isize,
            ))
            .read()) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn SetConestLiveUpdateTVData() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut flags: u32 = 0u32;
        let mut winner: u8 = 0u8;
        let mut round1Placing: u8 = 0u8;
        let mut round2Placing: u8 = 0u8;
        let mut count: u8 = 0u8;
        let mut randAction: u8 = 0u8;
        let mut numLoserCandidates: u8 = 0u8;
        let mut flagId: u8 = 0u8;
        let mut winnerFlag: u16 = 0u16;
        let mut loserFlag: u8 = 0u8;
        let mut loser: u8 = 0u8;
        let mut loserCandidates = crate::ffi::Align4([0u8; 3]);
        if ((((((&raw mut gContestFinalStandings).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize,
        ))
        .read()) as i32)
            != 0i32
        {
            return;
        }
        winner = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gContestFinalStandings).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        winner = ((i) as u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        round1Placing = 0u8;
        round2Placing = 0u8;
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    if ((((((&raw mut gContestMonRound1Points)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .wrapping_offset(((winner) as i32) as isize))
                    .read()) as i32)
                        < ((((((&raw mut gContestMonRound1Points)
                            .cast::<u8>()
                            .cast::<i16>())
                        .cast::<i16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        round1Placing = (round1Placing).wrapping_add(1);
                    }
                    if ((((((&raw mut gContestMonRound2Points)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .wrapping_offset(((winner) as i32) as isize))
                    .read()) as i32)
                        < ((((((&raw mut gContestMonRound2Points)
                            .cast::<u8>()
                            .cast::<i16>())
                        .cast::<i16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        round2Placing = (round2Placing).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        flags = (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((winner) as i32) as isize * 16))
        .wrapping_add(12))
        .read()) as u32);
        count = 0u8;
        {
            i = 0i32;
            'l5: loop {
                if !(i < 8i32) {
                    break 'l5;
                }
                'l6: {
                    if (flags & 1u32) != 0 {
                        count = (count).wrapping_add(1);
                    }
                }
                flags = (flags >> 1);
                i = (i).wrapping_add(1);
            }
        }
        randAction = ((crate::c::rem_i32(((Random()) as i32), ((count) as i32))) as u8);
        flags = (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((winner) as i32) as isize * 16))
        .wrapping_add(12))
        .read()) as u32);
        count = 0u8;
        flagId = 0u8;
        {
            i = 0i32;
            'l7: loop {
                if !(i < 8i32) {
                    break 'l7;
                }
                'l8: {
                    if !((flags & 1u32) != 0) {
                        break 'l8;
                    }
                    if ((randAction) as i32) == ((count) as i32) {
                        break 'l7;
                    }
                    count = (count).wrapping_add(1);
                }
                flags = (flags >> 1);
                flagId = (flagId).wrapping_add(1);
                i = (i).wrapping_add(1);
            }
        }
        winnerFlag = ((crate::c::shl_i32(1i32, ((flagId) as u32))) as u16);
        if ((winner) as i32) == 0i32 {
            ((&raw mut loserCandidates).cast::<u8>()).write(1u8);
            loserFlag = (((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(28)
            .cast::<*mut u8>())
            .read())
            .wrapping_offset(16))
            .wrapping_add(13))
            .read();
            i = 2i32;
        } else {
            ((&raw mut loserCandidates).cast::<u8>()).write(0u8);
            loserFlag = ((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(28)
            .cast::<*mut u8>())
            .read())
            .wrapping_add(13))
            .read();
            i = 1i32;
        }
        numLoserCandidates = 1u8;
        {
            'l9: loop {
                if !(i < 4i32) {
                    break 'l9;
                }
                'l10: {
                    if i != ((winner) as i32) {
                        if ((loserFlag) as i32)
                            < (((((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(28)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 16))
                            .wrapping_add(13))
                            .read()) as i32)
                        {
                            ((&raw mut loserCandidates).cast::<u8>()).write(((i) as u8));
                            loserFlag = (((((((&raw mut gContestResources)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(28)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 16))
                            .wrapping_add(13))
                            .read();
                            numLoserCandidates = 1u8;
                        } else {
                            if ((loserFlag) as i32)
                                == (((((((((&raw mut gContestResources)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(28)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 16))
                                .wrapping_add(13))
                                .read()) as i32)
                            {
                                (((&raw mut loserCandidates).cast::<u8>())
                                    .wrapping_offset(((numLoserCandidates) as i32) as isize))
                                .write(((i) as u8));
                                numLoserCandidates = (numLoserCandidates).wrapping_add(1);
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        loser = (((&raw mut loserCandidates).cast::<u8>()).wrapping_offset(
            (crate::c::rem_i32(((Random()) as i32), ((numLoserCandidates) as i32))) as isize,
        ))
        .read();
        flagId = 128u8;
        {
            i = 0i32;
            'l11: loop {
                if !(i < 8i32) {
                    break 'l11;
                }
                'l12: {
                    loserFlag =
                        (((((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(28)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((loser) as i32) as isize * 16))
                        .wrapping_add(13))
                        .read()) as i32)
                            & ((flagId) as i32)) as u8);
                    if (loserFlag) != 0 {
                        break 'l11;
                    }
                }
                flagId = ((((flagId) as i32) >> 1) as u8);
                i = (i).wrapping_add(1);
            }
        }
        ContestLiveUpdates_Init(round1Placing);
        ContestLiveUpdates_SetRound2Placing(round2Placing);
        ContestLiveUpdates_SetWinnerAppealFlag(((winnerFlag) as u8));
        ContestLiveUpdates_SetWinnerMoveUsed(
            (((((((((&raw mut gContestResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((winner) as i32) as isize * 16))
            .wrapping_add(10)
            .cast::<i16>())
            .read()) as u16),
        );
        ContestLiveUpdates_SetLoserData(loserFlag, loser);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestDebugToggleBitfields(loserFlags: u8) {
    unsafe {
        let mut loserFlags = loserFlags;
        if (((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).read()) as i32) == 0i32 {
            if !((loserFlags) != 0) {
                (((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).write(2u8);
            } else {
                (((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).write(3u8);
            }
        } else {
            (((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).write(0u8);
        }
        if (((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).read()) as i32) == 0i32 {
            DrawContestantWindowText();
            SwapMoveDescAndContestTilemaps();
        } else {
            ContestDebugPrintBitStrings();
        }
    }
}
pub(crate) unsafe extern "C" fn ContestDebugPrintBitStrings() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: i8 = 0i8;
        let mut text1 = crate::ffi::Align4([0u8; 20]);
        let mut text2 = crate::ffi::Align4([0u8; 20]);
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        let mut bits: u32 = 0u32;
        if !((((&raw mut gEnableContestDebugging).cast::<u8>()).read()) != 0) {
            return;
        }
        if ((((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).read()) as i32) != 2i32)
            && ((((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).read()) as i32) != 3i32)
        {
            return;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    FillWindowPixelBuffer(i, 0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((((&raw mut gHeap).cast::<u8>()).wrapping_offset(106496)).read()) as i32) == 2i32 {
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        txtPtr = StringCopy(
                            (&raw mut text1).cast::<u8>(),
                            (&raw mut gText_CDot).cast::<u8>(),
                        );
                        Contest_PrintTextToBg0WindowAt(
                            ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as u32),
                            (&raw mut text1).cast::<u8>(),
                            5i32,
                            1i32,
                            7i32,
                        );
                        bits = (((((((((&raw mut gContestResources)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(28)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 16))
                        .wrapping_add(12))
                        .read()) as u32);
                        {
                            j = 7i8;
                            'l5: loop {
                                if !(((j) as i32) > (-1i32)) {
                                    break 'l5;
                                }
                                'l6: {
                                    txtPtr = ConvertIntToDecimalStringN(
                                        txtPtr,
                                        ((bits & 1u32) as i32),
                                        0i32,
                                        1u8,
                                    );
                                    bits = (bits >> 1);
                                }
                                j = (j).wrapping_sub(1);
                            }
                        }
                        {
                            j = 0i8;
                            'l7: loop {
                                if !(((j) as i32) < 5i32) {
                                    break 'l7;
                                }
                                'l8: {
                                    (((&raw mut text2).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize))
                                    .write(
                                        (((&raw mut text1).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize))
                                        .read(),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        (((&raw mut text2).cast::<u8>()).wrapping_offset(((j) as i32) as isize))
                            .write(255u8);
                        Contest_PrintTextToBg0WindowAt(
                            ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as u32),
                            (&raw mut text2).cast::<u8>(),
                            5i32,
                            1i32,
                            7i32,
                        );
                        Contest_PrintTextToBg0WindowAt(
                            ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as u32),
                            ((&raw mut text1).cast::<u8>()).wrapping_offset(((j) as i32) as isize),
                            55i32,
                            1i32,
                            7i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0u8;
                'l9: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l9;
                    }
                    'l10: {
                        StringCopy(
                            (&raw mut text1).cast::<u8>(),
                            (&raw mut gText_BDot).cast::<u8>(),
                        );
                        bits = (((((((((&raw mut gContestResources)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(28)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 16))
                        .wrapping_add(13))
                        .read()) as u32);
                        txtPtr = ((&raw mut text1).cast::<u8>()).wrapping_offset(2);
                        {
                            j = 7i8;
                            'l11: loop {
                                if !(((j) as i32) > (-1i32)) {
                                    break 'l11;
                                }
                                'l12: {
                                    txtPtr = ConvertIntToDecimalStringN(
                                        txtPtr,
                                        ((bits & 1u32) as i32),
                                        0i32,
                                        1u8,
                                    );
                                    bits = (bits >> 1);
                                }
                                j = (j).wrapping_sub(1);
                            }
                        }
                        {
                            j = 0i8;
                            'l13: loop {
                                if !(((j) as i32) < 5i32) {
                                    break 'l13;
                                }
                                'l14: {
                                    (((&raw mut text2).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize))
                                    .write(
                                        (((&raw mut text1).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize))
                                        .read(),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        (((&raw mut text2).cast::<u8>()).wrapping_offset(((j) as i32) as isize))
                            .write(255u8);
                        Contest_PrintTextToBg0WindowAt(
                            ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as u32),
                            (&raw mut text2).cast::<u8>(),
                            5i32,
                            1i32,
                            7i32,
                        );
                        Contest_PrintTextToBg0WindowAt(
                            ((((((&raw mut gContestantTurnOrder).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as u32),
                            ((&raw mut text1).cast::<u8>()).wrapping_offset(((j) as i32) as isize),
                            55i32,
                            1i32,
                            7i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        SwapMoveDescAndContestTilemaps();
    }
}
pub(crate) unsafe extern "C" fn GetMonNicknameLanguage(nickname: *mut u8) -> u8 {
    unsafe {
        let mut nickname = nickname;
        let mut ret: u8 = 2u8;
        if ((((nickname).read()) as i32) == 252i32)
            && (((((nickname).wrapping_offset(1)).read()) as i32) == 21i32)
        {
            return 2u8;
        }
        if ((StringLength(nickname)) as i32) <= 5i32 {
            'l1: loop {
                if !((((nickname).read()) as i32) != 255i32) {
                    break 'l1;
                }
                if (((((((((((((((((((nickname).read()) as i32) >= 187i32)
                    && ((((nickname).read()) as i32) <= 238i32))
                    || (((((nickname).read()) as i32) >= 161i32)
                        && ((((nickname).read()) as i32) <= 170i32)))
                    || ((((nickname).read()) as i32) == 0i32))
                    || ((((nickname).read()) as i32) == 173i32))
                    || ((((nickname).read()) as i32) == 184i32))
                    || ((((nickname).read()) as i32) == 171i32))
                    || ((((nickname).read()) as i32) == 172i32))
                    || ((((nickname).read()) as i32) == 181i32))
                    || ((((nickname).read()) as i32) == 182i32))
                    || ((((nickname).read()) as i32) == 186i32))
                    || ((((nickname).read()) as i32) == 174i32))
                    || ((((nickname).read()) as i32) == 176i32))
                    || ((((nickname).read()) as i32) == 177i32))
                    || ((((nickname).read()) as i32) == 178i32))
                    || ((((nickname).read()) as i32) == 179i32))
                    || ((((nickname).read()) as i32) == 177i32)
                {
                    nickname = (nickname).wrapping_offset(1);
                } else {
                    ret = 1u8;
                    break 'l1;
                }
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn StripPlayerNameForLinkContest(playerName: *mut u8) {
    unsafe {
        let mut playerName = playerName;
        let mut chr: u8 = ((playerName).wrapping_offset(5)).read();
        ((playerName).wrapping_offset(5)).write(255u8);
        ((playerName).wrapping_offset(7)).write(chr);
    }
}
pub(crate) unsafe extern "C" fn StripMonNameForLinkContest(monName: *mut u8, language: i32) {
    unsafe {
        let mut monName = monName;
        let mut language = language;
        let mut chr: u8 = 0u8;
        StripExtCtrlCodes(monName);
        if language == 1i32 {
            ((monName).wrapping_offset(5)).write(255u8);
            ((monName).wrapping_offset(10)).write(252u8);
        } else {
            chr = ((monName).wrapping_offset(5)).read();
            ((monName).wrapping_offset(5)).write(255u8);
            ((monName).wrapping_offset(10)).write(chr);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StripPlayerAndMonNamesForLinkContest(mon: *mut u8, language: i32) {
    unsafe {
        let mut mon = mon;
        let mut language = language;
        let mut name: *mut u8 = ((mon).wrapping_add(2)).cast::<u8>();
        if language == 1i32 {
            ConvertInternationalString(name, GetMonNicknameLanguage(name));
        } else {
            if ((((name).wrapping_offset(10)).read()) as i32) == 252i32 {
                ConvertInternationalString(name, 1u8);
            } else {
                ((name).wrapping_offset(5)).write(((name).wrapping_offset(10)).read());
                ((name).wrapping_offset(10)).write(255u8);
            }
        }
        name = ((mon).wrapping_add(13)).cast::<u8>();
        if language == 1i32 {
            ((name).wrapping_offset(7)).write(255u8);
            ((name).wrapping_offset(6)).write(((name).wrapping_offset(4)).read());
            ((name).wrapping_offset(5)).write(((name).wrapping_offset(3)).read());
            ((name).wrapping_offset(4)).write(((name).wrapping_offset(2)).read());
            ((name).wrapping_offset(3)).write(((name).wrapping_offset(1)).read());
            ((name).wrapping_offset(2)).write((((mon).wrapping_add(13)).cast::<u8>()).read());
            ((name).wrapping_offset(1)).write(21u8);
            (name).write(252u8);
        } else {
            ((name).wrapping_offset(5)).write(((name).wrapping_offset(7)).read());
            ((name).wrapping_offset(7)).write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn SetBackdropFromColor(color: u16) {
    unsafe {
        let mut color = color;
        FillPalette(color, 0u16, 2u16);
    }
}
