//! Translated from `src/battle_dome.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBattleStyleMovePoints sBattleStyleThresholds sUnusedArray sTourneyTreeCursorMovementMap sTourneyTreeBgTemplates sInfoCardBgTemplates sTourneyTreeWindowTemplates sInfoCardWindowTemplates sTourneyTreeScanlineEffectParams sTourneyTreeButtonsSpriteSheet sTourneyTreeButtonsSpritePal sOamData_TourneyTreePokeball sOamData_TourneyTreeCloseButton sOamData_VerticalScrollArrow sOamData_HorizontalScrollArrow sSpriteAnim_TourneyTreePokeballNormal sSpriteAnim_TourneyTreePokeballSelected sSpriteAnimTable_TourneyTreePokeball sTourneyTreePokeballSpriteTemplate sSpriteAnim_TourneyTreeCancelButtonNormal sSpriteAnim_TourneyTreeCancelButtonSelected sSpriteAnimTable_TourneyTreeCancelButton sCancelButtonSpriteTemplate sSpriteAnim_TourneyTreeExitButtonNormal sSpriteAnim_TourneyTreeExitButtonSelected sSpriteAnimTable_TourneyTreeExitButton sExitButtonSpriteTemplate sSpriteAnim_UpArrow sSpriteAnim_DownArrow sSpriteAnim_LeftArrow sSpriteAnim_RightArrow sSpriteAnimTable_VerticalScrollArrow sSpriteAnimTable_HorizontalScrollArrow sHorizontalScrollArrowSpriteTemplate sVerticalScrollArrowSpriteTemplate sTourneyTreeTrainerIds sBattleDomeFunctions sWinStreakFlags sWinStreakMasks sIdToOpponentId sTourneyTreeTrainerOpponentIds sIdToMatchNumber sLastMatchCardNum sTrainerAndRoundToLastMatchCardNum sTournamentIdToPairedTrainerIds sBattleDomePotentialTexts sBattleDomeOpponentStyleTexts sBattleDomeOpponentStatsTexts sInfoTrainerMonX sInfoTrainerMonY sSpeciesNameTextYCoords sStatTextOffsets sBattleDomeMatchNumberTexts sBattleDomeWinTexts sLeftTrainerMonX sLeftTrainerMonY sRightTrainerMonX sRightTrainerMonY sTourneyTreeTrainerIds2 sCompetitorRangeByMatch sTrainerNamePositions sTourneyTreePokeballCoords sLineSectionTrainer1Round1 sLineSectionTrainer1Round2 sLineSectionTrainer1Semifinal sLineSectionTrainer1Final sLineSectionTrainer9Round1 sLineSectionTrainer9Round2 sLineSectionTrainer9Semifinal sLineSectionTrainer9Final sLineSectionTrainer13Round1 sLineSectionTrainer13Round2 sLineSectionTrainer13Semifinal sLineSectionTrainer13Final sLineSectionTrainer5Round1 sLineSectionTrainer5Round2 sLineSectionTrainer5Semifinal sLineSectionTrainer5Final sLineSectionTrainer8Round1 sLineSectionTrainer8Round2 sLineSectionTrainer8Semifinal sLineSectionTrainer8Final sLineSectionTrainer16Round1 sLineSectionTrainer16Round2 sLineSectionTrainer16Semifinal sLineSectionTrainer16Final sLineSectionTrainer12Round1 sLineSectionTrainer12Round2 sLineSectionTrainer12Semifinal sLineSectionTrainer12Final sLineSectionTrainer4Round1 sLineSectionTrainer4Round2 sLineSectionTrainer4Semifinal sLineSectionTrainer4Final sLineSectionTrainer3Round1 sLineSectionTrainer3Round2 sLineSectionTrainer3Semifinal sLineSectionTrainer3Final sLineSectionTrainer11Round1 sLineSectionTrainer11Round2 sLineSectionTrainer11Semifinal sLineSectionTrainer11Final sLineSectionTrainer15Round1 sLineSectionTrainer15Round2 sLineSectionTrainer15Semifinal sLineSectionTrainer15Final sLineSectionTrainer7Round1 sLineSectionTrainer7Round2 sLineSectionTrainer7Semifinal sLineSectionTrainer7Final sLineSectionTrainer6Round1 sLineSectionTrainer6Round2 sLineSectionTrainer6Semifinal sLineSectionTrainer6Final sLineSectionTrainer14Round1 sLineSectionTrainer14Round2 sLineSectionTrainer14Semifinal sLineSectionTrainer14Final sLineSectionTrainer10Round1 sLineSectionTrainer10Round2 sLineSectionTrainer10Semifinal sLineSectionTrainer10Final sLineSectionTrainer2Round1 sLineSectionTrainer2Round2 sLineSectionTrainer2Semifinal sLineSectionTrainer2Final sTourneyTreeLineSections sTourneyTreeLineSectionArrayCounts
#[allow(unused_imports)]
use crate::data::battle_dome::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerPartyLostHP: u32 = 0u32;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlayerPartyMaxHP: u32 = 0u32;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInfoCard: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTilemapBuffer: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBattleFrontierHeldItems: u8;
    static mut gBattleFrontierMons: u8;
    static mut gBattleFrontierTrainers: u8;
    static mut gBattleMoves: u8;
    static mut gBattleOutcome: u8;
    static mut gBattleResults: u8;
    static mut gBattleWindowTextPalette: u8;
    static mut gBattle_BG0_X: u8;
    static mut gBattle_BG0_Y: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_BG2_X: u8;
    static mut gBattle_BG2_Y: u8;
    static mut gBattle_BG3_X: u8;
    static mut gBattle_BG3_Y: u8;
    static mut gBitTable: u8;
    static mut gDisplayedStringBattle: u8;
    static mut gDomeTourneyInfoCardBg_Tilemap: u8;
    static mut gDomeTourneyInfoCard_Gfx: u8;
    static mut gDomeTourneyInfoCard_Tilemap: u8;
    static mut gDomeTourneyLineDown_Tilemap: u8;
    static mut gDomeTourneyLineUp_Tilemap: u8;
    static mut gDomeTourneyLine_Gfx: u8;
    static mut gDomeTourneyMatchCardBg_Pal: u8;
    static mut gDomeTourneyTreeButtons_Pal: u8;
    static mut gDomeTourneyTree_Gfx: u8;
    static mut gDomeTourneyTree_Pal: u8;
    static mut gDomeTourneyTree_Tilemap: u8;
    static mut gEnemyParty: u8;
    static mut gFacilityClassToTrainerClass: u8;
    static mut gFacilityTrainerMons: u8;
    static mut gFacilityTrainers: u8;
    static mut gMain: u8;
    static mut gMoveNames: u8;
    static mut gNatureStatTable: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gRoundsStringTable: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSelectedOrderFromParty: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesInfo: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_BattleTourney: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainerClassNames: u8;
    static mut gTrainers: u8;
    static mut gTypeEffectiveness: u8;
    fn AI_TypeCalc(a0: u16, a1: u16, a2: u8) -> u8;
    fn AddTextPrinter(a0: *mut u8, a1: u8, a2: Option<unsafe extern "C" fn(*mut u8, u16)>) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearSelectedPartyOrder();
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateMonIcon(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut u8)>,
        a2: i16,
        a3: i16,
        a4: u8,
        a5: u32,
        a6: u32,
    ) -> u8;
    fn CreateMonWithEVSpreadNatureOTID(
        a0: *mut u8,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u32,
    );
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTrainerPicSprite(a0: u16, a1: u8, a2: i16, a3: i16, a4: u8, a5: u16) -> u16;
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8);
    fn DestroyTask(a0: u8);
    fn EnableInterrupts(a0: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeAndDestroyMonIconSprite(a0: *mut u8);
    fn FreeAndDestroyTrainerPicSprite(a0: u16) -> u16;
    fn FreeMonIconPalettes();
    fn GetCurrentFacilityWinStreak() -> u32;
    fn GetFrontierBrainMonEvs(a0: u8, a1: u8) -> u8;
    fn GetFrontierBrainMonMove(a0: u8, a1: u8) -> u16;
    fn GetFrontierBrainMonNature(a0: u8) -> u8;
    fn GetFrontierBrainMonSpecies(a0: u8) -> u16;
    fn GetFrontierBrainStatus() -> u8;
    fn GetFrontierOpponentClass(a0: u16) -> u8;
    fn GetFrontierTrainerFrontSpriteId(a0: u16) -> u8;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetNature(a0: *mut u8) -> u8;
    fn GetNatureFromPersonality(a0: u32) -> u8;
    fn GetRandomFrontierMonFromSet(a0: u16) -> u16;
    fn GetRandomScaledFrontierTrainerId(a0: u8, a1: u8) -> u16;
    fn GetStringCenterAlignXOffsetWithLetterSpacing(a0: i32, a1: *mut u8, a2: i32, a3: i32) -> i32;
    fn GetStringWidthDifference(a0: i32, a1: *mut u8, a2: i32, a3: i32) -> i32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadMonIconPalettes();
    fn LoadOam();
    fn ModifyStatByNature(a0: u8, a1: u16, a2: u8) -> u16;
    fn PlaySE(a0: u16);
    fn PlayerGenderToFrontTrainerPicId(a0: u8) -> u16;
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn ReducePlayerPartyToSelectedMons();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn RunTasks();
    fn RunTextPrinters();
    fn SaveGameFrontier();
    fn ScanlineEffect_Clear();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn ScanlineEffect_Stop();
    fn SetBattleFacilityTrainerGfxId(a0: u16, a1: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetDynamicWarp(a0: i32, a1: i8, a2: i8, a3: i8);
    fn SetFacilityPtrsGetLevel() -> u8;
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMonMoveSlot(a0: *mut u8, a1: u16, a2: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdateMonIconFrame(a0: *mut u8) -> u8;
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn ZeroEnemyPartyMons();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallBattleDomeFunction() {
    unsafe {
        (((((&raw const sBattleDomeFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn InitDomeChallenge() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut battleMode: u32 = ((VarGet(16590u16)) as u32);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1628))
        .write(0u8);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .write(0u16);
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            2,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            3,
            1,
            (0u8) as i32,
        );
        if !(((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1680)
            .cast::<u32>())
        .read()
            & ((((((&raw const sWinStreakFlags).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((battleMode) as i32) as isize * 8))
            .cast::<u32>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read())
            != 0)
        {
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1728))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .write(0u16);
        }
        SetDynamicWarp(
            0i32,
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<i8>())
                .read(),
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read(),
            (-1i8),
        );
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn GetDomeData() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut battleMode: u32 = ((VarGet(16590u16)) as u32);
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1728))
                    .cast::<u8>())
                    .wrapping_offset(((battleMode) as i32) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset(((lvlMode) as i32) as isize))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1680)
                    .cast::<u32>())
                    .read()
                        & ((((((&raw const sWinStreakFlags).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((battleMode) as i32) as isize * 8))
                        .cast::<u32>())
                        .wrapping_offset(((lvlMode) as i32) as isize))
                        .read())
                        != 0u32) as u16),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1724),
                        0,
                        1,
                        false,
                    ) as u8) as u16),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1724),
                        1,
                        1,
                        false,
                    ) as u8) as u16),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1724),
                        2,
                        1,
                        false,
                    ) as u8) as u16),
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1724),
                        3,
                        1,
                        false,
                    ) as u8) as u16),
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((VarGet(16590u16)) as i32) == 1i32 {
                    if lvlMode != 0u32 {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1724),
                                5,
                                1,
                                false,
                            ) as u8) as u16),
                        );
                    } else {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1724),
                                4,
                                1,
                                false,
                            ) as u8) as u16),
                        );
                    }
                } else {
                    if lvlMode != 0u32 {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1724),
                                1,
                                1,
                                false,
                            ) as u8) as u16),
                        );
                    } else {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1724),
                                0,
                                1,
                                false,
                            ) as u8) as u16),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((VarGet(16590u16)) as i32) == 1i32 {
                    if lvlMode != 0u32 {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1724),
                                7,
                                1,
                                false,
                            ) as u8) as u16),
                        );
                    } else {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1724),
                                6,
                                1,
                                false,
                            ) as u8) as u16),
                        );
                    }
                } else {
                    if lvlMode != 0u32 {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1724),
                                3,
                                1,
                                false,
                            ) as u8) as u16),
                        );
                    } else {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1724),
                                2,
                                1,
                                false,
                            ) as u8) as u16),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                ClearSelectedPartyOrder();
                ((&raw mut gSelectedOrderFromParty).cast::<u8>()).write(
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1630))
                    .cast::<u16>())
                    .wrapping_offset(3))
                    .read()) as u8),
                );
                (((&raw mut gSelectedOrderFromParty).cast::<u8>()).wrapping_offset(1)).write(
                    (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1630))
                    .cast::<u16>())
                    .wrapping_offset(3))
                    .read()) as i32)
                        >> 8) as u8),
                );
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1726))
                    .read()) as i32)
                        .wrapping_mul(2i32))
                    .wrapping_sub(3i32))
                    .wrapping_add(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1727))
                        .read()) as i32),
                    )) as u16),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetDomeData() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut battleMode: u32 = ((VarGet(16590u16)) as u32);
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1728))
                .cast::<u8>())
                .wrapping_offset(((battleMode) as i32) as isize * 4))
                .cast::<u16>())
                .wrapping_offset(((lvlMode) as i32) as isize))
                .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) != 0 {
                    let __p2 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1680)
                    .cast::<u32>();
                    (__p2).write(
                        ((__p2).read()
                            | ((((((&raw const sWinStreakFlags).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((battleMode) as i32) as isize * 8))
                            .cast::<u32>())
                            .wrapping_offset(((lvlMode) as i32) as isize))
                            .read()),
                    );
                } else {
                    let __p3 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1680)
                    .cast::<u32>();
                    (__p3).write(
                        ((__p3).read()
                            & ((((((&raw const sWinStreakMasks).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((battleMode) as i32) as isize * 8))
                            .cast::<u32>())
                            .wrapping_offset(((lvlMode) as i32) as isize))
                            .read()),
                    );
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1724),
                    0,
                    1,
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1724),
                    1,
                    1,
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1724),
                    2,
                    1,
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1724),
                    3,
                    1,
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((VarGet(16590u16)) as i32) == 1i32 {
                    if lvlMode != 0u32 {
                        crate::c::bf_write(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1724),
                            5,
                            1,
                            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                        );
                    } else {
                        crate::c::bf_write(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1724),
                            4,
                            1,
                            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                        );
                    }
                } else {
                    if lvlMode != 0u32 {
                        crate::c::bf_write(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1724),
                            1,
                            1,
                            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                        );
                    } else {
                        crate::c::bf_write(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1724),
                            0,
                            1,
                            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((VarGet(16590u16)) as i32) == 1i32 {
                    if lvlMode != 0u32 {
                        crate::c::bf_write(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1724),
                            7,
                            1,
                            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                        );
                    } else {
                        crate::c::bf_write(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1724),
                            6,
                            1,
                            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                        );
                    }
                } else {
                    if lvlMode != 0u32 {
                        crate::c::bf_write(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1724),
                            3,
                            1,
                            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                        );
                    } else {
                        crate::c::bf_write(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1724),
                            2,
                            1,
                            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1630))
                .cast::<u16>())
                .wrapping_offset(3))
                .write(
                    ((((((&raw mut gSelectedOrderFromParty).cast::<u8>()).read()) as i32)
                        | ((((((&raw mut gSelectedOrderFromParty).cast::<u8>()).wrapping_offset(1))
                            .read()) as i32)
                            << 8)) as u16),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitDomeTrainers() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut monLevel: i32 = 0i32;
        let mut species = crate::ffi::Align4([0u8; 12]);
        let mut monTypesBits: i32 = 0i32;
        let mut monTypesCount: i32 = 0i32;
        let mut trainerId: i32 = 0i32;
        let mut monId: i32 = 0i32;
        let mut rankingScores: *mut u16 = core::ptr::null_mut();
        let mut statValues: *mut i32 = core::ptr::null_mut();
        let mut ivs: u8 = 0u8;
        ((&raw mut species).cast::<i32>()).write(0i32);
        (((&raw mut species).cast::<i32>()).wrapping_offset(1)).write(0i32);
        (((&raw mut species).cast::<i32>()).wrapping_offset(2)).write(0i32);
        rankingScores = (AllocZeroed(32u32)).cast::<u16>();
        statValues = (AllocZeroed(24u32)).cast::<i32>();
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1726))
        .write(
            ((((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32)
                .wrapping_add(1i32)) as u8),
        );
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1727))
        .write(((((VarGet(16590u16)) as i32).wrapping_add(1i32)) as u8));
        crate::c::bf_write(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1752))
            .cast::<u8>())
            .wrapping_add(0),
            0,
            10,
            (1023u16) as i32,
        );
        crate::c::bf_write(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1752))
            .cast::<u8>())
            .wrapping_add(1),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1752))
            .cast::<u8>())
            .wrapping_add(1),
            3,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1752))
            .cast::<u8>())
            .wrapping_add(1),
            5,
            3,
            (0u16) as i32,
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1816))
                    .cast::<u8>())
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1630))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    .wrapping_sub(1i32)) as isize
                                    * 100,
                            ),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(2224))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 16))
                                .cast::<u16>())
                                .wrapping_offset((j) as isize))
                                .write(
                                    ((GetMonData3(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1630))
                                            .cast::<u16>())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                .wrapping_sub(1i32))
                                                as isize
                                                * 100,
                                        ),
                                        (13i32).wrapping_add(j),
                                        core::ptr::null_mut(),
                                    )) as u16),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    {
                        j = 0i32;
                        'l5: loop {
                            if !(j < 6i32) {
                                break 'l5;
                            }
                            'l6: {
                                ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(2224))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 16))
                                .wrapping_add(8))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .write(
                                    ((GetMonData3(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1630))
                                            .cast::<u16>())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                .wrapping_sub(1i32))
                                                as isize
                                                * 100,
                                        ),
                                        (26i32).wrapping_add(j),
                                        core::ptr::null_mut(),
                                    )) as u8),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2224))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 16))
                    .wrapping_add(14))
                    .write(GetNature(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1630))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize
                                * 100,
                        ),
                    ));
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 1i32;
            'l7: loop {
                if !(i < 16i32) {
                    break 'l7;
                }
                'l8: {
                    if i > 5i32 {
                        'l9: loop {
                            'l10: {
                                trainerId = ((GetRandomScaledFrontierTrainerId(
                                    ((GetCurrentFacilityWinStreak()) as u8),
                                    0u8,
                                )) as i32);
                                {
                                    j = 1i32;
                                    'l11: loop {
                                        if !(j < i) {
                                            break 'l11;
                                        }
                                        'l12: {
                                            if ((crate::c::bf_read(
                                                (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1752))
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize * 4))
                                                .wrapping_add(0),
                                                0,
                                                10,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                                == trainerId
                                            {
                                                break 'l11;
                                            }
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                            }
                            if !(j != i) {
                                break 'l9;
                            }
                        }
                        crate::c::bf_write(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(0),
                            0,
                            10,
                            ((trainerId) as u16) as i32,
                        );
                    } else {
                        'l13: loop {
                            'l14: {
                                trainerId = ((GetRandomScaledFrontierTrainerId(
                                    (((GetCurrentFacilityWinStreak()).wrapping_add(1u32)) as u8),
                                    0u8,
                                )) as i32);
                                {
                                    j = 1i32;
                                    'l15: loop {
                                        if !(j < i) {
                                            break 'l15;
                                        }
                                        'l16: {
                                            if ((crate::c::bf_read(
                                                (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1752))
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize * 4))
                                                .wrapping_add(0),
                                                0,
                                                10,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                                == trainerId
                                            {
                                                break 'l15;
                                            }
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                            }
                            if !(j != i) {
                                break 'l13;
                            }
                        }
                        crate::c::bf_write(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(0),
                            0,
                            10,
                            ((trainerId) as u16) as i32,
                        );
                    }
                    {
                        j = 0i32;
                        'l17: loop {
                            if !(j < 3i32) {
                                break 'l17;
                            }
                            'l18: {
                                'l19: loop {
                                    'l20: {
                                        monId = ((GetRandomFrontierMonFromSet(((trainerId) as u16)))
                                            as i32);
                                        {
                                            k = 0i32;
                                            'l21: loop {
                                                if !(k < j) {
                                                    break 'l21;
                                                }
                                                'l22: {
                                                    let mut alreadySelectedMonId: i32 =
                                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(1612))
                                                        .wrapping_add(1816))
                                                        .cast::<u8>())
                                                        .wrapping_offset((i) as isize * 6))
                                                        .cast::<u16>())
                                                        .wrapping_offset((k) as isize))
                                                        .read())
                                                            as i32);
                                                    if (((alreadySelectedMonId == monId) || (((&raw mut species).cast::<i32>()).read() == ((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset((monId) as isize * 16)).cast::<u16>()).read()) as i32)))) || ((((&raw mut species).cast::<i32>()).wrapping_offset(1)).read() == ((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset((monId) as isize * 16)).cast::<u16>()).read()) as i32)))) || (((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset((alreadySelectedMonId) as isize * 16)).wrapping_add(10)).read()) as i32)) == ((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset((monId) as isize * 16)).wrapping_add(10)).read()) as i32))) {
break 'l21;
}
                                                }
                                                k = (k).wrapping_add(1);
                                            }
                                        }
                                    }
                                    if !(k != j) {
                                        break 'l19;
                                    }
                                }
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1816))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 6))
                                .cast::<u16>())
                                .wrapping_offset((j) as isize))
                                .write(((monId) as u16));
                                (((&raw mut species).cast::<i32>()).wrapping_offset((j) as isize))
                                    .write(
                                        (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                            .read())
                                        .wrapping_offset((monId) as isize * 16))
                                        .cast::<u16>())
                                        .read()) as i32),
                                    );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    crate::c::bf_write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(1),
                        2,
                        1,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(1),
                        3,
                        2,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(1),
                        5,
                        3,
                        (0u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        monTypesBits = 0i32;
        (rankingScores).write(0u16);
        {
            i = 0i32;
            'l23: loop {
                if !(i < 3i32) {
                    break 'l23;
                }
                'l24: {
                    trainerId = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1630))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        .wrapping_sub(1i32);
                    (rankingScores).write(
                        (((((rankingScores).read()) as u32).wrapping_add(GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((trainerId) as isize * 100),
                            59i32,
                            core::ptr::null_mut(),
                        ))) as u16),
                    );
                    (rankingScores).write(
                        (((((rankingScores).read()) as u32).wrapping_add(GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((trainerId) as isize * 100),
                            60i32,
                            core::ptr::null_mut(),
                        ))) as u16),
                    );
                    (rankingScores).write(
                        (((((rankingScores).read()) as u32).wrapping_add(GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((trainerId) as isize * 100),
                            62i32,
                            core::ptr::null_mut(),
                        ))) as u16),
                    );
                    (rankingScores).write(
                        (((((rankingScores).read()) as u32).wrapping_add(GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((trainerId) as isize * 100),
                            63i32,
                            core::ptr::null_mut(),
                        ))) as u16),
                    );
                    (rankingScores).write(
                        (((((rankingScores).read()) as u32).wrapping_add(GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((trainerId) as isize * 100),
                            61i32,
                            core::ptr::null_mut(),
                        ))) as u16),
                    );
                    (rankingScores).write(
                        (((((rankingScores).read()) as u32).wrapping_add(GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((trainerId) as isize * 100),
                            58i32,
                            core::ptr::null_mut(),
                        ))) as u16),
                    );
                    monTypesBits = ((((monTypesBits) as u32)
                        | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                            (((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                                ((GetMonData3(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset((trainerId) as isize * 100),
                                    11i32,
                                    core::ptr::null_mut(),
                                )) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(6))
                            .cast::<u8>())
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32);
                    monTypesBits = ((((monTypesBits) as u32)
                        | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                            ((((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                                ((GetMonData3(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset((trainerId) as isize * 100),
                                    11i32,
                                    core::ptr::null_mut(),
                                )) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(6))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            monTypesCount = 0i32;
            j = 0i32;
            'l25: loop {
                if !(j < 32i32) {
                    break 'l25;
                }
                'l26: {
                    if (monTypesBits & 1i32) != 0 {
                        monTypesCount = (monTypesCount).wrapping_add(1);
                    }
                    monTypesBits = (monTypesBits >> 1);
                }
                j = (j).wrapping_add(1);
            }
        }
        monLevel = ((SetFacilityPtrsGetLevel()) as i32);
        (rankingScores).write(
            (((((rankingScores).read()) as i32).wrapping_add(crate::c::div_i32(
                (monTypesCount).wrapping_mul(monLevel),
                20i32,
            ))) as u16),
        );
        {
            i = 1i32;
            'l27: loop {
                if !(i < 16i32) {
                    break 'l27;
                }
                'l28: {
                    monTypesBits = 0i32;
                    ((rankingScores).wrapping_offset((i) as isize)).write(0u16);
                    ivs = GetDomeTrainerMonIvs(
                        (crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(0),
                            0,
                            10,
                            false,
                        ) as u16),
                    );
                    {
                        j = 0i32;
                        'l29: loop {
                            if !(j < 3i32) {
                                break 'l29;
                            }
                            'l30: {
                                CalcDomeMonStats(
                                    (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(
                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1816))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .cast::<u16>())
                                    .read(),
                                    monLevel,
                                    ((ivs) as i32),
                                    (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(
                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1816))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(11))
                                    .read(),
                                    (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(
                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1816))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(12))
                                    .read(),
                                    statValues,
                                );
                                let __p1 = (rankingScores).wrapping_offset((i) as isize);
                                (__p1).write(
                                    (((((__p1).read()) as i32)
                                        .wrapping_add(((statValues).wrapping_offset(1)).read()))
                                        as u16),
                                );
                                let __p2 = (rankingScores).wrapping_offset((i) as isize);
                                (__p2).write(
                                    (((((__p2).read()) as i32)
                                        .wrapping_add(((statValues).wrapping_offset(2)).read()))
                                        as u16),
                                );
                                let __p3 = (rankingScores).wrapping_offset((i) as isize);
                                (__p3).write(
                                    (((((__p3).read()) as i32)
                                        .wrapping_add(((statValues).wrapping_offset(4)).read()))
                                        as u16),
                                );
                                let __p4 = (rankingScores).wrapping_offset((i) as isize);
                                (__p4).write(
                                    (((((__p4).read()) as i32)
                                        .wrapping_add(((statValues).wrapping_offset(5)).read()))
                                        as u16),
                                );
                                let __p5 = (rankingScores).wrapping_offset((i) as isize);
                                (__p5).write(
                                    (((((__p5).read()) as i32)
                                        .wrapping_add(((statValues).wrapping_offset(3)).read()))
                                        as u16),
                                );
                                let __p6 = (rankingScores).wrapping_offset((i) as isize);
                                (__p6).write(
                                    (((((__p6).read()) as i32).wrapping_add((statValues).read()))
                                        as u16),
                                );
                                monTypesBits = ((((monTypesBits) as u32)
                                    | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            (((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut gFacilityTrainerMons)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_offset(
                                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(1612))
                                                        .wrapping_add(1816))
                                                        .cast::<u8>())
                                                        .wrapping_offset((i) as isize * 6))
                                                        .cast::<u16>())
                                                        .wrapping_offset((j) as isize))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 16,
                                                    ))
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 28,
                                                ))
                                            .wrapping_add(6))
                                            .cast::<u8>())
                                            .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read())
                                    as i32);
                                monTypesBits = ((((monTypesBits) as u32)
                                    | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut gFacilityTrainerMons)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_offset(
                                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(1612))
                                                        .wrapping_add(1816))
                                                        .cast::<u8>())
                                                        .wrapping_offset((i) as isize * 6))
                                                        .cast::<u16>())
                                                        .wrapping_offset((j) as isize))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 16,
                                                    ))
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 28,
                                                ))
                                            .wrapping_add(6))
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read())
                                    as i32);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    {
                        monTypesCount = 0i32;
                        j = 0i32;
                        'l31: loop {
                            if !(j < 32i32) {
                                break 'l31;
                            }
                            'l32: {
                                if (monTypesBits & 1i32) != 0 {
                                    monTypesCount = (monTypesCount).wrapping_add(1);
                                }
                                monTypesBits = (monTypesBits >> 1);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    let __p7 = (rankingScores).wrapping_offset((i) as isize);
                    (__p7).write(
                        (((((__p7).read()) as i32).wrapping_add(crate::c::div_i32(
                            (monTypesCount).wrapping_mul(monLevel),
                            20i32,
                        ))) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l33: loop {
                if !(i < 15i32) {
                    break 'l33;
                }
                'l34: {
                    {
                        j = (i).wrapping_add(1i32);
                        'l35: loop {
                            if !(j < 16i32) {
                                break 'l35;
                            }
                            'l36: {
                                if ((((rankingScores).wrapping_offset((i) as isize)).read()) as i32)
                                    < ((((rankingScores).wrapping_offset((j) as isize)).read())
                                        as i32)
                                {
                                    SwapDomeTrainers(i, j, rankingScores);
                                } else {
                                    if ((((rankingScores).wrapping_offset((i) as isize)).read())
                                        as i32)
                                        == ((((rankingScores).wrapping_offset((j) as isize)).read())
                                            as i32)
                                    {
                                        if ((crate::c::bf_read(
                                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1752))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize * 4))
                                            .wrapping_add(0),
                                            0,
                                            10,
                                            false,
                                        ) as u16)
                                            as i32)
                                            == 1023i32
                                        {
                                            SwapDomeTrainers(i, j, rankingScores);
                                        } else {
                                            if ((crate::c::bf_read(
                                                (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1752))
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize * 4))
                                                .wrapping_add(0),
                                                0,
                                                10,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                                > ((crate::c::bf_read(
                                                    (((((((&raw mut gSaveBlock2Ptr)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(1612))
                                                    .wrapping_add(1752))
                                                    .cast::<u8>())
                                                    .wrapping_offset((j) as isize * 4))
                                                    .wrapping_add(0),
                                                    0,
                                                    10,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                            {
                                                SwapDomeTrainers(i, j, rankingScores);
                                            }
                                        }
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((GetFrontierBrainStatus()) as i32) != 0i32 {
            {
                i = 0i32;
                'l37: loop {
                    if !(i < 16i32) {
                        break 'l37;
                    }
                    'l38: {
                        if ((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(0),
                            0,
                            10,
                            false,
                        ) as u16) as i32)
                            == 1023i32
                        {
                            break 'l37;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (((((((&raw const sTrainerNamePositions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((i) as isize * 2))
            .cast::<u8>())
            .read()) as i32)
                != 0i32
            {
                j = 0i32;
                crate::c::bf_write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1752))
                    .cast::<u8>())
                    .wrapping_offset((j) as isize * 4))
                    .wrapping_add(0),
                    0,
                    10,
                    (1022u16) as i32,
                );
            } else {
                j = 1i32;
                crate::c::bf_write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1752))
                    .cast::<u8>())
                    .wrapping_offset((j) as isize * 4))
                    .wrapping_add(0),
                    0,
                    10,
                    (1022u16) as i32,
                );
            }
            {
                i = 0i32;
                'l39: loop {
                    if !(i < 3i32) {
                        break 'l39;
                    }
                    'l40: {
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1816))
                        .cast::<u8>())
                        .wrapping_offset((j) as isize * 6))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write(GetFrontierBrainMonSpecies(((i) as u8)));
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        Free((rankingScores).cast::<u8>());
        Free((statValues).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn CalcDomeMonStats(
    species: u16,
    level: i32,
    ivs: i32,
    evBits: u8,
    nature: u8,
    stats: *mut i32,
) {
    unsafe {
        let mut species = species;
        let mut level = level;
        let mut ivs = ivs;
        let mut evBits = evBits;
        let mut nature = nature;
        let mut stats = stats;
        let mut i: i32 = 0i32;
        let mut count: i32 = 0i32;
        let mut bits: u8 = 0u8;
        let mut resultingEvs: u16 = 0u16;
        let mut evs = crate::ffi::Align4([0u8; 24]);
        count = 0i32;
        bits = evBits;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (((bits) as i32) & 1i32) != 0 {
                        count = (count).wrapping_add(1);
                    }
                }
                bits = ((((bits) as i32) >> 1) as u8);
                i = (i).wrapping_add(1);
            }
        }
        resultingEvs = ((crate::c::div_i32(510i32, count)) as u16);
        {
            i = 0i32;
            'l3: loop {
                if !(i < 6i32) {
                    break 'l3;
                }
                'l4: {
                    (((&raw mut evs).cast::<i32>()).wrapping_offset((i) as isize)).write(0i32);
                    if (((evBits) as i32) & ((bits) as i32)) != 0 {
                        (((&raw mut evs).cast::<i32>()).wrapping_offset((i) as isize))
                            .write(((resultingEvs) as i32));
                    }
                }
                bits = ((((bits) as i32) << 1) as u8);
                i = (i).wrapping_add(1);
            }
        }
        if ((species) as i32) == 303i32 {
            (stats).write(1i32);
        } else {
            let mut n: i32 = (2i32).wrapping_mul(
                (((((&raw mut gSpeciesInfo).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .read()) as i32),
            );
            (stats).write(
                ((crate::c::div_i32(
                    (((n).wrapping_add(ivs)).wrapping_add(crate::c::div_i32(
                        ((&raw mut evs).cast::<i32>()).read(),
                        4i32,
                    )))
                    .wrapping_mul(level),
                    100i32,
                ))
                .wrapping_add(level))
                .wrapping_add(10i32),
            );
        }
        {
            let mut baseStat: u8 = ((((&raw mut gSpeciesInfo).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(1))
            .read();
            ((stats).wrapping_offset(1)).write(
                (crate::c::div_i32(
                    ((((2i32).wrapping_mul(((baseStat) as i32))).wrapping_add(ivs)).wrapping_add(
                        crate::c::div_i32(
                            (((&raw mut evs).cast::<i32>()).wrapping_offset(1)).read(),
                            4i32,
                        ),
                    ))
                    .wrapping_mul(level),
                    100i32,
                ))
                .wrapping_add(5i32),
            );
            ((stats).wrapping_offset(1)).write(
                (((ModifyStatByNature(nature, ((((stats).wrapping_offset(1)).read()) as u16), 1u8))
                    as u8) as i32),
            );
        }
        {
            let mut baseStat: u8 = ((((&raw mut gSpeciesInfo).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(2))
            .read();
            ((stats).wrapping_offset(2)).write(
                (crate::c::div_i32(
                    ((((2i32).wrapping_mul(((baseStat) as i32))).wrapping_add(ivs)).wrapping_add(
                        crate::c::div_i32(
                            (((&raw mut evs).cast::<i32>()).wrapping_offset(2)).read(),
                            4i32,
                        ),
                    ))
                    .wrapping_mul(level),
                    100i32,
                ))
                .wrapping_add(5i32),
            );
            ((stats).wrapping_offset(2)).write(
                (((ModifyStatByNature(nature, ((((stats).wrapping_offset(2)).read()) as u16), 2u8))
                    as u8) as i32),
            );
        }
        {
            let mut baseStat: u8 = ((((&raw mut gSpeciesInfo).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(3))
            .read();
            ((stats).wrapping_offset(3)).write(
                (crate::c::div_i32(
                    ((((2i32).wrapping_mul(((baseStat) as i32))).wrapping_add(ivs)).wrapping_add(
                        crate::c::div_i32(
                            (((&raw mut evs).cast::<i32>()).wrapping_offset(3)).read(),
                            4i32,
                        ),
                    ))
                    .wrapping_mul(level),
                    100i32,
                ))
                .wrapping_add(5i32),
            );
            ((stats).wrapping_offset(3)).write(
                (((ModifyStatByNature(nature, ((((stats).wrapping_offset(3)).read()) as u16), 3u8))
                    as u8) as i32),
            );
        }
        {
            let mut baseStat: u8 = ((((&raw mut gSpeciesInfo).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(4))
            .read();
            ((stats).wrapping_offset(4)).write(
                (crate::c::div_i32(
                    ((((2i32).wrapping_mul(((baseStat) as i32))).wrapping_add(ivs)).wrapping_add(
                        crate::c::div_i32(
                            (((&raw mut evs).cast::<i32>()).wrapping_offset(4)).read(),
                            4i32,
                        ),
                    ))
                    .wrapping_mul(level),
                    100i32,
                ))
                .wrapping_add(5i32),
            );
            ((stats).wrapping_offset(4)).write(
                (((ModifyStatByNature(nature, ((((stats).wrapping_offset(4)).read()) as u16), 4u8))
                    as u8) as i32),
            );
        }
        {
            let mut baseStat: u8 = ((((&raw mut gSpeciesInfo).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(5))
            .read();
            ((stats).wrapping_offset(5)).write(
                (crate::c::div_i32(
                    ((((2i32).wrapping_mul(((baseStat) as i32))).wrapping_add(ivs)).wrapping_add(
                        crate::c::div_i32(
                            (((&raw mut evs).cast::<i32>()).wrapping_offset(5)).read(),
                            4i32,
                        ),
                    ))
                    .wrapping_mul(level),
                    100i32,
                ))
                .wrapping_add(5i32),
            );
            ((stats).wrapping_offset(5)).write(
                (((ModifyStatByNature(nature, ((((stats).wrapping_offset(5)).read()) as u16), 5u8))
                    as u8) as i32),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SwapDomeTrainers(id1: i32, id2: i32, statsArray: *mut u16) {
    unsafe {
        let mut id1 = id1;
        let mut id2 = id2;
        let mut statsArray = statsArray;
        let mut i: i32 = 0i32;
        let mut temp: u16 = 0u16;
        {
            temp = ((statsArray).wrapping_offset((id1) as isize)).read();
            ((statsArray).wrapping_offset((id1) as isize))
                .write(((statsArray).wrapping_offset((id2) as isize)).read());
            ((statsArray).wrapping_offset((id2) as isize)).write(temp);
        }
        {
            temp = (crate::c::bf_read(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1752))
                .cast::<u8>())
                .wrapping_offset((id1) as isize * 4))
                .wrapping_add(0),
                0,
                10,
                false,
            ) as u16);
            crate::c::bf_write(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1752))
                .cast::<u8>())
                .wrapping_offset((id1) as isize * 4))
                .wrapping_add(0),
                0,
                10,
                (crate::c::bf_read(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1752))
                    .cast::<u8>())
                    .wrapping_offset((id2) as isize * 4))
                    .wrapping_add(0),
                    0,
                    10,
                    false,
                ) as u16) as i32,
            );
            crate::c::bf_write(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1752))
                .cast::<u8>())
                .wrapping_offset((id2) as isize * 4))
                .wrapping_add(0),
                0,
                10,
                (temp) as i32,
            );
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    temp = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1816))
                    .cast::<u8>())
                    .wrapping_offset((id1) as isize * 6))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read();
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1816))
                    .cast::<u8>())
                    .wrapping_offset((id1) as isize * 6))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1816))
                        .cast::<u8>())
                        .wrapping_offset((id2) as isize * 6))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1816))
                    .cast::<u8>())
                    .wrapping_offset((id2) as isize * 6))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(temp);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BufferDomeRoundText() {
    unsafe {
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((&raw mut gRoundsStringTable).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1638)
                        .cast::<u16>())
                    .read()) as i32) as isize,
                ))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn BufferDomeOpponentName() {
    unsafe {
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((&raw mut gRoundsStringTable).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1638)
                        .cast::<u16>())
                    .read()) as i32) as isize,
                ))
            .read(),
        );
        CopyDomeTrainerName(
            (&raw mut gStringVar2).cast::<u8>(),
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn InitDomeOpponentParty() {
    unsafe {
        ((&raw mut gPlayerPartyLostHP).cast::<u8>().cast::<u32>()).write(0u32);
        ((&raw mut sPlayerPartyMaxHP).cast::<u8>().cast::<u32>()).write(GetMonData3(
            (&raw mut gPlayerParty).cast::<u8>(),
            58i32,
            core::ptr::null_mut(),
        ));
        let __p1 = (&raw mut sPlayerPartyMaxHP).cast::<u8>().cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(100),
            58i32,
            core::ptr::null_mut(),
        )));
        CalculatePlayerPartyCount();
        CreateDomeOpponentMons(
            ((TrainerIdToTournamentId(((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()))
                as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateDomeOpponentMon(
    monPartyId: u8,
    tournamentTrainerId: u16,
    tournamentMonId: u8,
    otId: u32,
) {
    unsafe {
        let mut monPartyId = monPartyId;
        let mut tournamentTrainerId = tournamentTrainerId;
        let mut tournamentMonId = tournamentMonId;
        let mut otId = otId;
        let mut i: i32 = 0i32;
        let mut friendship: u8 = 255u8;
        let mut fixedIv: u8 = GetDomeTrainerMonIvs(tournamentTrainerId);
        let mut level: u8 = SetFacilityPtrsGetLevel();
        CreateMonWithEVSpreadNatureOTID(
            ((&raw mut gEnemyParty).cast::<u8>())
                .wrapping_offset(((monPartyId) as i32) as isize * 100),
            (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset(
                (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1816))
                .cast::<u8>())
                .wrapping_offset(((tournamentTrainerId) as i32) as isize * 6))
                .cast::<u16>())
                .wrapping_offset(((tournamentMonId) as i32) as isize))
                .read()) as i32) as isize
                    * 16,
            ))
            .cast::<u16>())
            .read(),
            level,
            (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset(
                (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1816))
                .cast::<u8>())
                .wrapping_offset(((tournamentTrainerId) as i32) as isize * 6))
                .cast::<u16>())
                .wrapping_offset(((tournamentMonId) as i32) as isize))
                .read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(12))
            .read(),
            fixedIv,
            (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset(
                (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1816))
                .cast::<u8>())
                .wrapping_offset(((tournamentTrainerId) as i32) as isize * 6))
                .cast::<u16>())
                .wrapping_offset(((tournamentMonId) as i32) as isize))
                .read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(11))
            .read(),
            otId,
        );
        friendship = 255u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    SetMonMoveSlot(
                        ((&raw mut gEnemyParty).cast::<u8>())
                            .wrapping_offset(((monPartyId) as i32) as isize * 100),
                        (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(
                                (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(1816))
                                .cast::<u8>())
                                .wrapping_offset(((tournamentTrainerId) as i32) as isize * 6))
                                .cast::<u16>())
                                .wrapping_offset(((tournamentMonId) as i32) as isize))
                                .read()) as i32) as isize
                                    * 16,
                            ))
                        .wrapping_add(2))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                        ((i) as u8),
                    );
                    if (((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                        .wrapping_offset(
                            (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1816))
                            .cast::<u8>())
                            .wrapping_offset(((tournamentTrainerId) as i32) as isize * 6))
                            .cast::<u16>())
                            .wrapping_offset(((tournamentMonId) as i32) as isize))
                            .read()) as i32) as isize
                                * 16,
                        ))
                    .wrapping_add(2))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 218i32
                    {
                        friendship = 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        SetMonData(
            ((&raw mut gEnemyParty).cast::<u8>())
                .wrapping_offset(((monPartyId) as i32) as isize * 100),
            32i32,
            &raw mut friendship,
        );
        SetMonData(
            ((&raw mut gEnemyParty).cast::<u8>())
                .wrapping_offset(((monPartyId) as i32) as isize * 100),
            12i32,
            ((((&raw mut gBattleFrontierHeldItems).cast::<u16>()).cast::<u16>()).wrapping_offset(
                (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset(
                    (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1816))
                    .cast::<u8>())
                    .wrapping_offset(((tournamentTrainerId) as i32) as isize * 6))
                    .cast::<u16>())
                    .wrapping_offset(((tournamentMonId) as i32) as isize))
                    .read()) as i32) as isize
                        * 16,
                ))
                .wrapping_add(10))
                .read()) as i32) as isize,
            ))
            .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateDomeOpponentMons(tournamentTrainerId: u16) {
    unsafe {
        let mut tournamentTrainerId = tournamentTrainerId;
        let mut monsCount: u8 = 0u8;
        let mut otId: u32 = 0u32;
        let mut i: i32 = 0i32;
        let mut selectedMonBits: i32 = 0i32;
        ZeroEnemyPartyMons();
        selectedMonBits = GetDomeTrainerSelectedMons(tournamentTrainerId);
        otId = ((((Random()) as i32) | (((Random()) as i32) << 16)) as u32);
        if crate::c::rem_i32(((Random()) as i32), 10i32) > 5i32 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 3i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (selectedMonBits & 1i32) != 0 {
                            CreateDomeOpponentMon(
                                monsCount,
                                tournamentTrainerId,
                                ((i) as u8),
                                otId,
                            );
                            monsCount = (monsCount).wrapping_add(1);
                        }
                        selectedMonBits = (selectedMonBits >> 1);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 2i32;
                'l3: loop {
                    if !(i >= 0i32) {
                        break 'l3;
                    }
                    'l4: {
                        if (selectedMonBits & 4i32) != 0 {
                            CreateDomeOpponentMon(
                                monsCount,
                                tournamentTrainerId,
                                ((i) as u8),
                                otId,
                            );
                            monsCount = (monsCount).wrapping_add(1);
                        }
                        selectedMonBits = (selectedMonBits << 1);
                    }
                    i = (i).wrapping_sub(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDomeTrainerSelectedMons(tournamentTrainerId: u16) -> i32 {
    unsafe {
        let mut tournamentTrainerId = tournamentTrainerId;
        let mut selectedMonBits: i32 = 0i32;
        if (((Random()) as i32) & 1i32) != 0 {
            selectedMonBits = SelectOpponentMons_Good(tournamentTrainerId, 0u8);
            if selectedMonBits == 0i32 {
                selectedMonBits = SelectOpponentMons_Bad(tournamentTrainerId, 1u8);
            }
        } else {
            selectedMonBits = SelectOpponentMons_Bad(tournamentTrainerId, 0u8);
            if selectedMonBits == 0i32 {
                selectedMonBits = SelectOpponentMons_Good(tournamentTrainerId, 1u8);
            }
        }
        return selectedMonBits;
    }
}
pub(crate) unsafe extern "C" fn SelectOpponentMons_Good(
    tournamentTrainerId: u16,
    allowRandom: u8,
) -> i32 {
    unsafe {
        let mut tournamentTrainerId = tournamentTrainerId;
        let mut allowRandom = allowRandom;
        let mut i: i32 = 0i32;
        let mut moveIndex: i32 = 0i32;
        let mut playerMonId: i32 = 0i32;
        let mut partyMovePoints = crate::ffi::Align4([0u8; 12]);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut partyMovePoints).cast::<i32>()).wrapping_offset((i) as isize))
                        .write(0i32);
                    {
                        moveIndex = 0i32;
                        'l3: loop {
                            if !(moveIndex < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                {
                                    playerMonId = 0i32;
                                    'l5: loop {
                                        if !(playerMonId < 3i32) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            if ((crate::c::bf_read(
                                                (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1752))
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((tournamentTrainerId) as i32) as isize * 4,
                                                ))
                                                .wrapping_add(0),
                                                0,
                                                10,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                                == 1022i32
                                            {
                                                let __p1 = ((&raw mut partyMovePoints)
                                                    .cast::<i32>())
                                                .wrapping_offset((i) as isize);
                                                (__p1).write(
                                                    ((__p1).read()).wrapping_add(
                                                        GetTypeEffectivenessPoints(
                                                            ((GetFrontierBrainMonMove(
                                                                ((i) as u8),
                                                                ((moveIndex) as u8),
                                                            ))
                                                                as i32),
                                                            ((GetMonData3(
                                                                ((&raw mut gPlayerParty)
                                                                    .cast::<u8>())
                                                                .wrapping_offset(
                                                                    (playerMonId) as isize * 100,
                                                                ),
                                                                11i32,
                                                                core::ptr::null_mut(),
                                                            ))
                                                                as i32),
                                                            0i32,
                                                        ),
                                                    ),
                                                );
                                            } else {
                                                let __p2 = ((&raw mut partyMovePoints)
                                                    .cast::<i32>())
                                                .wrapping_offset((i) as isize);
                                                (__p2).write(((__p2).read()).wrapping_add(GetTypeEffectivenessPoints((((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset(((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612)).wrapping_add(1816)).cast::<u8>()).wrapping_offset((((tournamentTrainerId) as i32)) as isize * 6)).cast::<u16>()).wrapping_offset((i) as isize)).read()) as i32)) as isize * 16)).wrapping_add(2)).cast::<u16>()).wrapping_offset((moveIndex) as isize)).read()) as i32), ((GetMonData3((((&raw mut gPlayerParty)).cast::<u8>()).wrapping_offset((playerMonId) as isize * 100), 11i32, core::ptr::null_mut())) as i32), 0i32)));
                                            }
                                        }
                                        playerMonId = (playerMonId).wrapping_add(1);
                                    }
                                }
                            }
                            moveIndex = (moveIndex).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return SelectOpponentMonsFromParty((&raw mut partyMovePoints).cast::<i32>(), allowRandom);
    }
}
pub(crate) unsafe extern "C" fn SelectOpponentMons_Bad(
    tournamentTrainerId: u16,
    allowRandom: u8,
) -> i32 {
    unsafe {
        let mut tournamentTrainerId = tournamentTrainerId;
        let mut allowRandom = allowRandom;
        let mut i: i32 = 0i32;
        let mut moveIndex: i32 = 0i32;
        let mut playerMonId: i32 = 0i32;
        let mut partyMovePoints = crate::ffi::Align4([0u8; 12]);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut partyMovePoints).cast::<i32>()).wrapping_offset((i) as isize))
                        .write(0i32);
                    {
                        moveIndex = 0i32;
                        'l3: loop {
                            if !(moveIndex < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                {
                                    playerMonId = 0i32;
                                    'l5: loop {
                                        if !(playerMonId < 3i32) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            if ((crate::c::bf_read(
                                                (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1752))
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((tournamentTrainerId) as i32) as isize * 4,
                                                ))
                                                .wrapping_add(0),
                                                0,
                                                10,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                                == 1022i32
                                            {
                                                let __p1 = ((&raw mut partyMovePoints)
                                                    .cast::<i32>())
                                                .wrapping_offset((i) as isize);
                                                (__p1).write(
                                                    ((__p1).read()).wrapping_add(
                                                        GetTypeEffectivenessPoints(
                                                            ((GetFrontierBrainMonMove(
                                                                ((i) as u8),
                                                                ((moveIndex) as u8),
                                                            ))
                                                                as i32),
                                                            ((GetMonData3(
                                                                ((&raw mut gPlayerParty)
                                                                    .cast::<u8>())
                                                                .wrapping_offset(
                                                                    (playerMonId) as isize * 100,
                                                                ),
                                                                11i32,
                                                                core::ptr::null_mut(),
                                                            ))
                                                                as i32),
                                                            1i32,
                                                        ),
                                                    ),
                                                );
                                            } else {
                                                let __p2 = ((&raw mut partyMovePoints)
                                                    .cast::<i32>())
                                                .wrapping_offset((i) as isize);
                                                (__p2).write(((__p2).read()).wrapping_add(GetTypeEffectivenessPoints((((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset(((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612)).wrapping_add(1816)).cast::<u8>()).wrapping_offset((((tournamentTrainerId) as i32)) as isize * 6)).cast::<u16>()).wrapping_offset((i) as isize)).read()) as i32)) as isize * 16)).wrapping_add(2)).cast::<u16>()).wrapping_offset((moveIndex) as isize)).read()) as i32), ((GetMonData3((((&raw mut gPlayerParty)).cast::<u8>()).wrapping_offset((playerMonId) as isize * 100), 11i32, core::ptr::null_mut())) as i32), 1i32)));
                                            }
                                        }
                                        playerMonId = (playerMonId).wrapping_add(1);
                                    }
                                }
                            }
                            moveIndex = (moveIndex).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return SelectOpponentMonsFromParty((&raw mut partyMovePoints).cast::<i32>(), allowRandom);
    }
}
pub(crate) unsafe extern "C" fn SelectOpponentMonsFromParty(
    partyMovePoints: *mut i32,
    allowRandom: u8,
) -> i32 {
    unsafe {
        let mut partyMovePoints = partyMovePoints;
        let mut allowRandom = allowRandom;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut selectedMonBits: i32 = 0i32;
        let mut partyPositions = crate::ffi::Align4([0u8; 12]);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut partyPositions).cast::<i32>()).wrapping_offset((i) as isize))
                        .write(i);
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((partyMovePoints).read() == ((partyMovePoints).wrapping_offset(1)).read())
            && ((partyMovePoints).read() == ((partyMovePoints).wrapping_offset(2)).read())
        {
            if (allowRandom) != 0 {
                i = 0i32;
                'l3: loop {
                    if !(i != 2i32) {
                        break 'l3;
                    }
                    let mut rand: u32 = ((((Random()) as i32) & 3i32) as u32);
                    if (rand != 3u32)
                        && (!((((selectedMonBits) as u32)
                            & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset(((rand) as i32) as isize))
                            .read())
                            != 0))
                    {
                        selectedMonBits = ((((selectedMonBits) as u32)
                            | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset(((rand) as i32) as isize))
                            .read()) as i32);
                        i = (i).wrapping_add(1);
                    }
                }
            }
        } else {
            {
                i = 0i32;
                'l4: loop {
                    if !(i < 2i32) {
                        break 'l4;
                    }
                    'l5: {
                        {
                            j = (i).wrapping_add(1i32);
                            'l6: loop {
                                if !(j < 3i32) {
                                    break 'l6;
                                }
                                'l7: {
                                    let mut temp: i32 = 0i32;
                                    if ((partyMovePoints).wrapping_offset((i) as isize)).read()
                                        < ((partyMovePoints).wrapping_offset((j) as isize)).read()
                                    {
                                        {
                                            temp = ((partyMovePoints)
                                                .wrapping_offset((i) as isize))
                                            .read();
                                            ((partyMovePoints).wrapping_offset((i) as isize))
                                                .write(
                                                    ((partyMovePoints)
                                                        .wrapping_offset((j) as isize))
                                                    .read(),
                                                );
                                            ((partyMovePoints).wrapping_offset((j) as isize))
                                                .write(temp);
                                        }
                                        {
                                            temp = (((&raw mut partyPositions).cast::<i32>())
                                                .wrapping_offset((i) as isize))
                                            .read();
                                            (((&raw mut partyPositions).cast::<i32>())
                                                .wrapping_offset((i) as isize))
                                            .write(
                                                (((&raw mut partyPositions).cast::<i32>())
                                                    .wrapping_offset((j) as isize))
                                                .read(),
                                            );
                                            (((&raw mut partyPositions).cast::<i32>())
                                                .wrapping_offset((j) as isize))
                                            .write(temp);
                                        }
                                    }
                                    if (((partyMovePoints).wrapping_offset((i) as isize)).read()
                                        == ((partyMovePoints).wrapping_offset((j) as isize)).read())
                                        && ((((Random()) as i32) & 1i32) != 0)
                                    {
                                        {
                                            temp = ((partyMovePoints)
                                                .wrapping_offset((i) as isize))
                                            .read();
                                            ((partyMovePoints).wrapping_offset((i) as isize))
                                                .write(
                                                    ((partyMovePoints)
                                                        .wrapping_offset((j) as isize))
                                                    .read(),
                                                );
                                            ((partyMovePoints).wrapping_offset((j) as isize))
                                                .write(temp);
                                        }
                                        {
                                            temp = (((&raw mut partyPositions).cast::<i32>())
                                                .wrapping_offset((i) as isize))
                                            .read();
                                            (((&raw mut partyPositions).cast::<i32>())
                                                .wrapping_offset((i) as isize))
                                            .write(
                                                (((&raw mut partyPositions).cast::<i32>())
                                                    .wrapping_offset((j) as isize))
                                                .read(),
                                            );
                                            (((&raw mut partyPositions).cast::<i32>())
                                                .wrapping_offset((j) as isize))
                                            .write(temp);
                                        }
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
                'l8: loop {
                    if !(i < 2i32) {
                        break 'l8;
                    }
                    'l9: {
                        selectedMonBits = ((((selectedMonBits) as u32)
                            | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset(
                                    ((((&raw mut partyPositions).cast::<i32>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as isize,
                                ))
                            .read()) as i32);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return selectedMonBits;
    }
}
pub(crate) unsafe extern "C" fn GetTypeEffectivenessPoints(
    r#move: i32,
    targetSpecies: i32,
    mode: i32,
) -> i32 {
    unsafe {
        let mut r#move = r#move;
        let mut targetSpecies = targetSpecies;
        let mut mode = mode;
        let mut defType1: i32 = 0i32;
        let mut defType2: i32 = 0i32;
        let mut defAbility: i32 = 0i32;
        let mut moveType: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut typePower: i32 = 20i32;
        if ((r#move == 0i32) || (r#move == 65535i32))
            || (((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset((r#move) as isize * 12))
            .wrapping_add(1))
            .read()) as i32)
                == 0i32)
        {
            return 0i32;
        }
        defType1 = (((((((&raw mut gSpeciesInfo).cast::<u8>())
            .wrapping_offset((targetSpecies) as isize * 28))
        .wrapping_add(6))
        .cast::<u8>())
        .read()) as i32);
        defType2 = ((((((((&raw mut gSpeciesInfo).cast::<u8>())
            .wrapping_offset((targetSpecies) as isize * 28))
        .wrapping_add(6))
        .cast::<u8>())
        .wrapping_offset(1))
        .read()) as i32);
        defAbility = (((((((&raw mut gSpeciesInfo).cast::<u8>())
            .wrapping_offset((targetSpecies) as isize * 28))
        .wrapping_add(22))
        .cast::<u8>())
        .read()) as i32);
        moveType = ((((((&raw mut gBattleMoves).cast::<u8>())
            .wrapping_offset((r#move) as isize * 12))
        .wrapping_add(2))
        .read()) as i32);
        if (defAbility == 26i32) && (moveType == 4i32) {
            if mode == 1i32 {
                typePower = 8i32;
            }
        } else {
            'l1: loop {
                if !((((((&raw mut gTypeEffectiveness).cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(0i32)) as isize))
                .read()) as i32)
                    != 255i32)
                {
                    break 'l1;
                }
                if (((((&raw mut gTypeEffectiveness).cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(0i32)) as isize))
                .read()) as i32)
                    == 254i32
                {
                    i = (i).wrapping_add(3i32);
                    continue 'l1;
                }
                if (((((&raw mut gTypeEffectiveness).cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(0i32)) as isize))
                .read()) as i32)
                    == moveType
                {
                    if (((((&raw mut gTypeEffectiveness).cast::<u8>())
                        .wrapping_offset(((i).wrapping_add(1i32)) as isize))
                    .read()) as i32)
                        == defType1
                    {
                        if ((defAbility == 25i32)
                            && ((((((&raw mut gTypeEffectiveness).cast::<u8>())
                                .wrapping_offset(((i).wrapping_add(2i32)) as isize))
                            .read()) as i32)
                                == 40i32))
                            || (defAbility != 25i32)
                        {
                            typePower = crate::c::div_i32(
                                (typePower).wrapping_mul(
                                    (((((&raw mut gTypeEffectiveness).cast::<u8>())
                                        .wrapping_offset(((i).wrapping_add(2i32)) as isize))
                                    .read()) as i32),
                                ),
                                10i32,
                            );
                        }
                    }
                    if ((((((&raw mut gTypeEffectiveness).cast::<u8>())
                        .wrapping_offset(((i).wrapping_add(1i32)) as isize))
                    .read()) as i32)
                        == defType2)
                        && (defType1 != defType2)
                    {
                        if ((defAbility == 25i32)
                            && ((((((&raw mut gTypeEffectiveness).cast::<u8>())
                                .wrapping_offset(((i).wrapping_add(2i32)) as isize))
                            .read()) as i32)
                                == 40i32))
                            || (defAbility != 25i32)
                        {
                            typePower = crate::c::div_i32(
                                (typePower).wrapping_mul(
                                    (((((&raw mut gTypeEffectiveness).cast::<u8>())
                                        .wrapping_offset(((i).wrapping_add(2i32)) as isize))
                                    .read()) as i32),
                                ),
                                10i32,
                            );
                        }
                    }
                }
                i = (i).wrapping_add(3i32);
            }
        }
        'l2: {
            let __sw1 = mode;
            if __sw1 == 0i32 {
                'l3: {
                    let __sw2 = typePower;
                    let __matched = __sw2 == 0i32
                        || __sw2 == 5i32
                        || __sw2 == 10i32
                        || __sw2 == 20i32
                        || __sw2 == 40i32
                        || __sw2 == 80i32;
                    if __sw2 == 0i32 || __sw2 == 5i32 || __sw2 == 10i32 || !__matched {
                        typePower = 0i32;
                        break 'l3;
                    }
                    if __sw2 == 20i32 {
                        typePower = 2i32;
                        break 'l3;
                    }
                    if __sw2 == 40i32 {
                        typePower = 4i32;
                        break 'l3;
                    }
                    if __sw2 == 80i32 {
                        typePower = 8i32;
                        break 'l3;
                    }
                }
                break 'l2;
            }
            if __sw1 == 1i32 {
                'l4: {
                    let __sw3 = typePower;
                    let __matched = __sw3 == 0i32
                        || __sw3 == 5i32
                        || __sw3 == 10i32
                        || __sw3 == 20i32
                        || __sw3 == 40i32
                        || __sw3 == 80i32;
                    if __sw3 == 0i32 {
                        typePower = 8i32;
                        break 'l4;
                    }
                    if __sw3 == 5i32 {
                        typePower = 4i32;
                        break 'l4;
                    }
                    if __sw3 == 10i32 {
                        typePower = 2i32;
                        break 'l4;
                    }
                    if __sw3 == 20i32 || !__matched {
                        typePower = 0i32;
                        break 'l4;
                    }
                    if __sw3 == 40i32 {
                        typePower = (-2i32);
                        break 'l4;
                    }
                    if __sw3 == 80i32 {
                        typePower = (-4i32);
                        break 'l4;
                    }
                }
                break 'l2;
            }
            if __sw1 == 2i32 {
                'l5: {
                    let __sw4 = typePower;
                    let __matched = __sw4 == 0i32
                        || __sw4 == 5i32
                        || __sw4 == 10i32
                        || __sw4 == 20i32
                        || __sw4 == 40i32
                        || __sw4 == 80i32;
                    if __sw4 == 0i32 {
                        typePower = (-16i32);
                        break 'l5;
                    }
                    if __sw4 == 5i32 {
                        typePower = (-8i32);
                        break 'l5;
                    }
                    if __sw4 == 10i32 || !__matched {
                        typePower = 0i32;
                        break 'l5;
                    }
                    if __sw4 == 20i32 {
                        typePower = 4i32;
                        break 'l5;
                    }
                    if __sw4 == 40i32 {
                        typePower = 12i32;
                        break 'l5;
                    }
                    if __sw4 == 80i32 {
                        typePower = 20i32;
                        break 'l5;
                    }
                }
                break 'l2;
            }
        }
        return typePower;
    }
}
pub(crate) unsafe extern "C" fn GetDomeTrainerMonIvs(trainerId: u16) -> u8 {
    unsafe {
        let mut trainerId = trainerId;
        let mut fixedIv: u8 = 0u8;
        if ((trainerId) as i32) <= 99i32 {
            fixedIv = 3u8;
        } else {
            if ((trainerId) as i32) <= 119i32 {
                fixedIv = 6u8;
            } else {
                if ((trainerId) as i32) <= 139i32 {
                    fixedIv = 9u8;
                } else {
                    if ((trainerId) as i32) <= 159i32 {
                        fixedIv = 12u8;
                    } else {
                        if ((trainerId) as i32) <= 179i32 {
                            fixedIv = 15u8;
                        } else {
                            if ((trainerId) as i32) <= 199i32 {
                                fixedIv = 18u8;
                            } else {
                                if ((trainerId) as i32) <= 219i32 {
                                    fixedIv = 21u8;
                                } else {
                                    fixedIv = 31u8;
                                }
                            }
                        }
                    }
                }
            }
        }
        return fixedIv;
    }
}
pub(crate) unsafe extern "C" fn TournamentIdOfOpponent(roundId: i32, trainerId: i32) -> i32 {
    unsafe {
        let mut roundId = roundId;
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut opponentMax: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(0),
                        0,
                        10,
                        false,
                    ) as u16) as i32)
                        == trainerId
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if roundId != 0i32 {
            if roundId == 3i32 {
                opponentMax = ((((((((&raw const sIdToOpponentId).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset((i) as isize * 4))
                .cast::<u8>())
                .wrapping_offset((roundId) as isize))
                .read()) as i32)
                    .wrapping_add(8i32);
            } else {
                opponentMax = ((((((((&raw const sIdToOpponentId).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset((i) as isize * 4))
                .cast::<u8>())
                .wrapping_offset((roundId) as isize))
                .read()) as i32)
                    .wrapping_add(4i32);
            }
            {
                j = ((((((((&raw const sIdToOpponentId).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((i) as isize * 4))
                .cast::<u8>())
                .wrapping_offset((roundId) as isize))
                .read()) as i32);
                'l3: loop {
                    if !(j < opponentMax) {
                        break 'l3;
                    }
                    'l4: {
                        if (((((((&raw const sTourneyTreeTrainerOpponentIds)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((j) as isize))
                        .read()) as i32)
                            != i)
                            && (!((crate::c::bf_read(
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1752))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw const sTourneyTreeTrainerOpponentIds)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as i32) as isize
                                        * 4,
                                ))
                                .wrapping_add(1),
                                2,
                                1,
                                false,
                            ) as u16)
                                != 0))
                        {
                            break 'l3;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if j != opponentMax {
                return ((((((&raw const sTourneyTreeTrainerOpponentIds)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset((j) as isize))
                .read()) as i32);
            } else {
                return 255i32;
            }
        } else {
            if !((crate::c::bf_read(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1752))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((((&raw const sIdToOpponentId).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                    .cast::<u8>())
                    .wrapping_offset((roundId) as isize))
                    .read()) as i32) as isize
                        * 4,
                ))
                .wrapping_add(1),
                2,
                1,
                false,
            ) as u16)
                != 0)
            {
                return ((((((((&raw const sIdToOpponentId).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset((i) as isize * 4))
                .cast::<u8>())
                .wrapping_offset((roundId) as isize))
                .read()) as i32);
            } else {
                return 255i32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn SetDomeOpponentId() {
    unsafe {
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(TrainerIdOfPlayerOpponent());
    }
}
pub(crate) unsafe extern "C" fn TrainerIdOfPlayerOpponent() -> u16 {
    unsafe {
        return (crate::c::bf_read(
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1752))
            .cast::<u8>())
            .wrapping_offset(
                (TournamentIdOfOpponent(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1638)
                        .cast::<u16>())
                    .read()) as i32),
                    1023i32,
                )) as isize
                    * 4,
            ))
            .wrapping_add(0),
            0,
            10,
            false,
        ) as u16);
    }
}
pub(crate) unsafe extern "C" fn SetDomeOpponentGraphicsId() {
    unsafe {
        SetBattleFacilityTrainerGfxId(
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn SaveDomeChallenge() {
    unsafe {
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1628))
        .write(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8));
        VarSet(16384u16, 0u16);
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            2,
            1,
            (1u8) as i32,
        );
        SaveGameFrontier();
    }
}
pub(crate) unsafe extern "C" fn IncrementDomeStreaks() {
    unsafe {
        let mut lvlMode: u8 = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        let mut battleMode: u8 = ((VarGet(16590u16)) as u8);
        if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1728))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read()) as i32)
            < 999i32
        {
            let __p1 = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1728))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1744))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read()) as i32)
            < 999i32
        {
            let __p2 = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1744))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1728))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read()) as i32)
            > (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1736))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read()) as i32)
        {
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1736))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .write(
                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1728))
                .cast::<u8>())
                .wrapping_offset(((battleMode) as i32) as isize * 4))
                .cast::<u16>())
                .wrapping_offset(((lvlMode) as i32) as isize))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ShowDomeOpponentInfo() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_ShowTourneyInfoCard), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((TrainerIdToTournamentId(TrainerIdOfPlayerOpponent())) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(0i16);
        SetMainCallback2(Some(CB2_TourneyTree));
    }
}
pub(crate) unsafe extern "C" fn Task_ShowTourneyInfoCard(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut tournamentId: i32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32);
        let mut mode: i32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32);
        let mut id: i32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32);
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                SetHBlankCallback(None);
                SetVBlankCallback(None);
                EnableInterrupts(1u16);
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
                            'l4: loop {
                                'l5: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((100663296i32) as usize as *mut u8),
                                        ((83886080i32
                                            | (crate::c::div_i32(
                                                98304i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sInfoCardBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(16u32, 4u32)) as u8),
                );
                InitWindows(
                    ((&raw const sInfoCardWindowTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                DeactivateAllTextPrinters();
                ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG3_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(0u16);
                if mode == 2i32 {
                    ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                    ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
                } else {
                    ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                    ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(160u16);
                }
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                SetGpuReg(76u8, 0u16);
                SetGpuReg(64u8, 0u16);
                SetGpuReg(68u8, 0u16);
                SetGpuReg(66u8, 0u16);
                SetGpuReg(70u8, 0u16);
                SetGpuReg(72u8, 0u16);
                SetGpuReg(74u8, 63u16);
                ResetPaletteFade();
                ResetSpriteData();
                FreeAllSpritePalettes();
                ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(4u8);
                let __p3 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                DecompressAndLoadBgGfxUsingHeap(
                    2u8,
                    (((&raw mut gDomeTourneyInfoCard_Gfx).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    8192u32,
                    0u16,
                    0u8,
                );
                DecompressAndLoadBgGfxUsingHeap(
                    2u8,
                    (((&raw mut gDomeTourneyInfoCard_Tilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    8192u32,
                    0u16,
                    1u8,
                );
                DecompressAndLoadBgGfxUsingHeap(
                    3u8,
                    (((&raw mut gDomeTourneyInfoCardBg_Tilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    2048u32,
                    0u16,
                    1u8,
                );
                LoadCompressedSpriteSheet(
                    ((&raw const sTourneyTreeButtonsSpriteSheet)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                LoadCompressedPalette(
                    ((&raw mut gDomeTourneyTree_Pal).cast::<u32>()).cast::<u32>(),
                    0u16,
                    512u16,
                );
                LoadCompressedPalette(
                    ((&raw mut gDomeTourneyTreeButtons_Pal).cast::<u32>()).cast::<u32>(),
                    256u16,
                    512u16,
                );
                LoadCompressedPalette(
                    ((&raw mut gBattleWindowTextPalette).cast::<u32>()).cast::<u32>(),
                    240u16,
                    32u16,
                );
                if mode == 2i32 {
                    LoadCompressedPalette(
                        ((&raw mut gDomeTourneyMatchCardBg_Pal).cast::<u32>()).cast::<u32>(),
                        80u16,
                        32u16,
                    );
                }
                'l6: loop {
                    'l7: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
                            'l8: loop {
                                'l9: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                            .cast::<u8>(),
                                        ((83886080i32
                                            | (crate::c::div_i32(
                                                1024i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l8;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l6;
                    }
                }
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                let __p4 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetVBlankCallback(Some(VblankCb_TourneyInfoCard));
                ((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(20u32));
                {
                    i = 0i32;
                    'l10: loop {
                        if !(i < 16i32) {
                            break 'l10;
                        }
                        'l11: {
                            (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(255u8);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                LoadMonIconPalettes();
                i = ((CreateTask(Some(Task_HandleInfoCardInput), 0u8)) as i32);
                (((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                    .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                    .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                    .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(((mode) as i16));
                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                    .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(((id) as i16));
                if mode == 2i32 {
                    DisplayMatchInfoOnCard(0u8, ((tournamentId) as u8));
                    ((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .write(1u8);
                } else {
                    DisplayTrainerInfoOnCard(0u8, ((tournamentId) as u8));
                }
                SetGpuReg(0u8, 8000u16);
                if mode != 0i32 {
                    id = ((CreateSprite(
                        (&raw const sVerticalScrollArrowSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        4i16,
                        0u8,
                    )) as i32);
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset((id) as isize * 68),
                        0u8,
                    );
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset((id) as isize * 68))
                        .wrapping_add(46))
                    .cast::<i16>())
                    .write(((i) as i16));
                    id = ((CreateSprite(
                        (&raw const sVerticalScrollArrowSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        156i16,
                        0u8,
                    )) as i32);
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset((id) as isize * 68),
                        1u8,
                    );
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset((id) as isize * 68))
                        .wrapping_add(46))
                    .cast::<i16>())
                    .write(((i) as i16));
                    id = ((CreateSprite(
                        (&raw const sHorizontalScrollArrowSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        6i16,
                        80i16,
                        0u8,
                    )) as i32);
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset((id) as isize * 68),
                        0u8,
                    );
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset((id) as isize * 68))
                        .wrapping_add(46))
                    .cast::<i16>())
                    .write(((i) as i16));
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset((id) as isize * 68))
                        .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    if mode == 1i32 {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset((id) as isize * 68))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                    }
                    id = ((CreateSprite(
                        (&raw const sHorizontalScrollArrowSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        234i16,
                        80i16,
                        0u8,
                    )) as i32);
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset((id) as isize * 68),
                        1u8,
                    );
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset((id) as isize * 68))
                        .wrapping_add(46))
                    .cast::<i16>())
                    .write(((i) as i16));
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset((id) as isize * 68))
                        .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(1i16);
                }
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerIconCardScrollUp(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) >= (-32i32) {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
            if (({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 40i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        } else {
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) >= 192i32 {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize,
                    ))
                .write(255u8);
                FreeAndDestroyTrainerPicSprite(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as u16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerIconCardScrollDown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(4i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) <= 192i32 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
            if (({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 40i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        } else {
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) <= (-32i32) {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize,
                    ))
                .write(255u8);
                FreeAndDestroyTrainerPicSprite(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as u16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerIconCardScrollLeft(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= (-32i32) {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
            if (({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 64i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        } else {
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= 272i32 {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize,
                    ))
                .write(255u8);
                FreeAndDestroyTrainerPicSprite(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as u16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerIconCardScrollRight(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(4i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) <= 272i32 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
            if (({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 64i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        } else {
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) <= (-32i32) {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize,
                    ))
                .write(255u8);
                FreeAndDestroyTrainerPicSprite(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as u16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIconDomeInfo(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0) {
            UpdateMonIconFrame(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIconCardScrollUp(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0) {
            UpdateMonIconFrame(sprite);
        }
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) >= (-16i32) {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
            if (({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 40i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_MonIconDomeInfo));
            }
        } else {
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) >= 176i32 {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize,
                    ))
                .write(255u8);
                FreeAndDestroyMonIconSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIconCardScrollDown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0) {
            UpdateMonIconFrame(sprite);
        }
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(4i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) <= 176i32 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
            if (({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 40i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_MonIconDomeInfo));
            }
        } else {
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) <= (-16i32) {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize,
                    ))
                .write(255u8);
                FreeAndDestroyMonIconSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIconCardScrollLeft(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0) {
            UpdateMonIconFrame(sprite);
        }
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= (-16i32) {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
            if (({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 64i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_MonIconDomeInfo));
            }
        } else {
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= 256i32 {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize,
                    ))
                .write(255u8);
                FreeAndDestroyMonIconSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIconCardScrollRight(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0) {
            UpdateMonIconFrame(sprite);
        }
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(4i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) <= 256i32 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
            if (({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 64i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_MonIconDomeInfo));
            }
        } else {
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) <= (-16i32) {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize,
                    ))
                .write(255u8);
                FreeAndDestroyMonIconSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HorizontalScrollArrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut taskId1: i32 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
        let mut arrId: i32 = ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((taskId1) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32);
        let mut tournmanetTrainerId: i32 =
            ((((((&raw const sTourneyTreeTrainerIds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((arrId) as isize))
            .read()) as i32);
        let mut roundId: i32 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1638)
        .cast::<u16>())
        .read()) as i32);
        if ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((taskId1) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            == 1i32
        {
            if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0 {
                if ((crate::c::bf_read(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1752))
                    .cast::<u8>())
                    .wrapping_offset((tournmanetTrainerId) as isize * 4))
                    .wrapping_add(1),
                    2,
                    1,
                    false,
                ) as u16)
                    != 0)
                    && (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                        < ((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset((tournmanetTrainerId) as isize * 4))
                            .wrapping_add(1),
                            3,
                            2,
                            false,
                        ) as u16) as i32))
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                } else {
                    if (!((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((tournmanetTrainerId) as isize * 4))
                        .wrapping_add(1),
                        2,
                        1,
                        false,
                    ) as u16)
                        != 0))
                        && (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16))
                        .read()) as i32)
                            .wrapping_sub(1i32)
                            < roundId)
                    {
                        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    } else {
                        if (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset((taskId1) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .read()) as i32)
                            == 2i32
                        {
                            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                        }
                    }
                }
            } else {
                if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16))
                .read()) as i32)
                    != 0i32
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                } else {
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset((taskId1) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32)
                        == 2i32
                    {
                        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    }
                }
            }
        } else {
            if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0 {
                if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16))
                .read()) as i32)
                    > 1i32
                {
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset((taskId1) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32)
                        == 2i32
                    {
                        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    }
                } else {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                }
            } else {
                if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16))
                .read()) as i32)
                    != 0i32
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                } else {
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset((taskId1) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32)
                        == 2i32
                    {
                        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_VerticalScrollArrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut taskId1: i32 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
        if ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((taskId1) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            == 1i32
        {
            if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
                .read()) as i32)
                != 0i32
            {
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset((taskId1) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    == 2i32
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                }
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
        } else {
            if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
                .read()) as i32)
                != 1i32
            {
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset((taskId1) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    == 2i32
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                }
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleInfoCardInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut windowId: i32 = 0i32;
        let mut mode: i32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32);
        let mut taskId2: i32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as i32);
        let mut trainerTourneyId: i32 = 0i32;
        let mut matchNo: i32 = 0i32;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
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
                i = ((Task_GetInfoCardInput(taskId)) as i32);
                'l2: {
                    let __sw2 = i;
                    if __sw2 == 9i32 {
                        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(8i16);
                        break 'l2;
                    }
                    if (1i32..=4i32).contains(&__sw2) || (5i32..=8i32).contains(&__sw2) {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(((i) as i16));
                        if (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read())
                            != 0
                        {
                            windowId = 9i32;
                        } else {
                            windowId = 0i32;
                        }
                        {
                            i = windowId;
                            'l3: loop {
                                if !(i < (windowId).wrapping_add(9i32)) {
                                    break 'l3;
                                }
                                'l4: {
                                    CopyWindowToVram(((i) as u8), 2u8);
                                    FillWindowPixelBuffer(((i) as u8), 0u8);
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(3i16);
                        break 'l2;
                    }
                    if __sw2 == 0i32 {
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                i = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as i32);
                'l5: {
                    let __sw3 = i;
                    if __sw3 == 1i32 || __sw3 == 5i32 {
                        if (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read())
                            != 0
                        {
                            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(160u16);
                        } else {
                            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(160u16);
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                        }
                        if i == 1i32 {
                            if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(16))
                            .read()) as i32)
                                == 0i32
                            {
                                ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                                ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(320u16);
                                trainerTourneyId = ((((((&raw const sTourneyTreeTrainerIds)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset((taskId2) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .read()) as i32) as isize,
                                ))
                                .read())
                                    as i32);
                                DisplayTrainerInfoOnCard(
                                    ((((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        | 16i32) as u8),
                                    ((trainerTourneyId) as u8),
                                );
                            } else {
                                ((&raw mut gBattle_BG2_X).cast::<u16>()).write(256u16);
                                ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
                                trainerTourneyId = ((((((&raw const sTourneyTreeTrainerIds)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset((taskId2) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .read()) as i32) as isize,
                                ))
                                .read())
                                    as i32);
                                DisplayTrainerInfoOnCard(
                                    ((((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        | 16i32) as u8),
                                    ((trainerTourneyId) as u8),
                                );
                                ((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(16))
                                .write(0u8);
                            }
                        } else {
                            if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(16))
                            .read()) as i32)
                                == 0i32
                            {
                                matchNo = ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset((taskId2) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    .wrapping_sub(16i32);
                                BufferDomeWinString(
                                    ((matchNo) as u8),
                                    ((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(17))
                                    .cast::<u8>(),
                                );
                                ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                                ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(320u16);
                                trainerTourneyId =
                                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(17))
                                    .cast::<u8>())
                                    .read()) as i32);
                                DisplayTrainerInfoOnCard(
                                    ((((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        | 16i32) as u8),
                                    ((trainerTourneyId) as u8),
                                );
                            } else {
                                if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(16))
                                .read()) as i32)
                                    == 2i32
                                {
                                    matchNo = ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset((taskId2) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_sub(16i32);
                                    BufferDomeWinString(
                                        ((matchNo) as u8),
                                        ((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(17))
                                        .cast::<u8>(),
                                    );
                                    ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                                    ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(320u16);
                                    trainerTourneyId = ((((((((&raw mut sInfoCard)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(17))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read())
                                        as i32);
                                    DisplayTrainerInfoOnCard(
                                        ((((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(2))
                                        .read()) as i32)
                                            | 16i32)
                                            as u8),
                                        ((trainerTourneyId) as u8),
                                    );
                                } else {
                                    ((&raw mut gBattle_BG2_X).cast::<u16>()).write(256u16);
                                    ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(160u16);
                                    matchNo = ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset((taskId2) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_sub(16i32);
                                    DisplayMatchInfoOnCard(
                                        ((((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(2))
                                        .read()) as i32)
                                            | 16i32)
                                            as u8),
                                        ((matchNo) as u8),
                                    );
                                }
                            }
                        }
                        {
                            i = 0i32;
                            'l6: loop {
                                if !(i < crate::c::div_i32(16i32, 2i32)) {
                                    break 'l6;
                                }
                                'l7: {
                                    if i < 2i32 {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_TrainerIconCardScrollUp));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    ^ 1i32)
                                                    as i16),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i16),
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_MonIconCardScrollUp));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    ^ 1i32)
                                                    as i16),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        {
                            i = crate::c::div_i32(16i32, 2i32);
                            'l8: loop {
                                if !(i < 16i32) {
                                    break 'l8;
                                }
                                'l9: {
                                    if i < 10i32 {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_TrainerIconCardScrollUp));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read(),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i16),
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_MonIconCardScrollUp));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read(),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(4i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(0i16);
                        break 'l5;
                    }
                    if __sw3 == 2i32 || __sw3 == 6i32 {
                        if (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read())
                            != 0
                        {
                            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(65376u16);
                        } else {
                            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(65376u16);
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                        }
                        if i == 2i32 {
                            if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(16))
                            .read()) as i32)
                                == 0i32
                            {
                                ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                                ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(160u16);
                                trainerTourneyId = ((((((&raw const sTourneyTreeTrainerIds)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset((taskId2) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .read()) as i32) as isize,
                                ))
                                .read())
                                    as i32);
                                DisplayTrainerInfoOnCard(
                                    ((((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        | 4i32) as u8),
                                    ((trainerTourneyId) as u8),
                                );
                            } else {
                                ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                                ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
                                trainerTourneyId = ((((((&raw const sTourneyTreeTrainerIds)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset((taskId2) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .read()) as i32) as isize,
                                ))
                                .read())
                                    as i32);
                                DisplayTrainerInfoOnCard(
                                    ((((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        | 4i32) as u8),
                                    ((trainerTourneyId) as u8),
                                );
                                ((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(16))
                                .write(0u8);
                            }
                        } else {
                            if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(16))
                            .read()) as i32)
                                == 0i32
                            {
                                matchNo = ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset((taskId2) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    .wrapping_sub(16i32);
                                BufferDomeWinString(
                                    ((matchNo) as u8),
                                    ((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(17))
                                    .cast::<u8>(),
                                );
                                ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                                ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(160u16);
                                trainerTourneyId =
                                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(17))
                                    .cast::<u8>())
                                    .read()) as i32);
                                DisplayTrainerInfoOnCard(
                                    ((((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        | 4i32) as u8),
                                    ((trainerTourneyId) as u8),
                                );
                            } else {
                                if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(16))
                                .read()) as i32)
                                    == 2i32
                                {
                                    matchNo = ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset((taskId2) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_sub(16i32);
                                    BufferDomeWinString(
                                        ((matchNo) as u8),
                                        ((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(17))
                                        .cast::<u8>(),
                                    );
                                    ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                                    ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(160u16);
                                    trainerTourneyId = ((((((((&raw mut sInfoCard)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(17))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read())
                                        as i32);
                                    DisplayTrainerInfoOnCard(
                                        ((((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(2))
                                        .read()) as i32)
                                            | 4i32) as u8),
                                        ((trainerTourneyId) as u8),
                                    );
                                } else {
                                    ((&raw mut gBattle_BG2_X).cast::<u16>()).write(256u16);
                                    ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
                                    matchNo = ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset((taskId2) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_sub(16i32);
                                    DisplayMatchInfoOnCard(
                                        ((((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(2))
                                        .read()) as i32)
                                            | 4i32) as u8),
                                        ((matchNo) as u8),
                                    );
                                }
                            }
                        }
                        {
                            i = 0i32;
                            'l10: loop {
                                if !(i < crate::c::div_i32(16i32, 2i32)) {
                                    break 'l10;
                                }
                                'l11: {
                                    if i < 2i32 {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_TrainerIconCardScrollDown));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    ^ 1i32)
                                                    as i16),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i16),
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_MonIconCardScrollDown));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    ^ 1i32)
                                                    as i16),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        {
                            i = crate::c::div_i32(16i32, 2i32);
                            'l12: loop {
                                if !(i < 16i32) {
                                    break 'l12;
                                }
                                'l13: {
                                    if i < 10i32 {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_TrainerIconCardScrollDown));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read(),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i16),
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_MonIconCardScrollDown));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read(),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(5i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(0i16);
                        break 'l5;
                    }
                    if __sw3 == 3i32 {
                        if (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read())
                            != 0
                        {
                            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(256u16);
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                        } else {
                            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(256u16);
                            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                        }
                        if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16))
                        .read()) as i32)
                            == 0i32
                        {
                            ((&raw mut gBattle_BG2_X).cast::<u16>()).write(256u16);
                            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(160u16);
                            trainerTourneyId =
                                ((((((&raw const sTourneyTreeTrainerIds).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset((taskId2) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .read()) as i32) as isize,
                                ))
                                .read()) as i32);
                            DisplayTrainerInfoOnCard(
                                ((((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    | 8i32) as u8),
                                ((trainerTourneyId) as u8),
                            );
                        } else {
                            ((&raw mut gBattle_BG2_X).cast::<u16>()).write(256u16);
                            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
                            matchNo = ((((((((&raw const sIdToMatchNumber)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset((taskId2) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as i32) as isize
                                    * 4,
                            ))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(16))
                                .read()) as i32)
                                    .wrapping_sub(1i32)) as isize,
                            ))
                            .read()) as i32);
                            DisplayMatchInfoOnCard(
                                ((((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    | 8i32) as u8),
                                ((matchNo) as u8),
                            );
                        }
                        {
                            i = 0i32;
                            'l14: loop {
                                if !(i < crate::c::div_i32(16i32, 2i32)) {
                                    break 'l14;
                                }
                                'l15: {
                                    if i < 2i32 {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_TrainerIconCardScrollLeft));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    ^ 1i32)
                                                    as i16),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i16),
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_MonIconCardScrollLeft));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    ^ 1i32)
                                                    as i16),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        {
                            i = crate::c::div_i32(16i32, 2i32);
                            'l16: loop {
                                if !(i < 16i32) {
                                    break 'l16;
                                }
                                'l17: {
                                    if i < 10i32 {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_TrainerIconCardScrollLeft));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read(),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i16),
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_MonIconCardScrollLeft));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read(),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(6i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(0i16);
                        break 'l5;
                    }
                    if __sw3 == 7i32 {
                        if (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read())
                            != 0
                        {
                            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(256u16);
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                        } else {
                            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(256u16);
                            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                        }
                        if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16))
                        .read()) as i32)
                            == 0i32
                        {
                            ((&raw mut gBattle_BG2_X).cast::<u16>()).write(256u16);
                            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(160u16);
                            trainerTourneyId =
                                (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(17))
                                .cast::<u8>())
                                .read()) as i32);
                            DisplayTrainerInfoOnCard(
                                ((((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    | 8i32) as u8),
                                ((trainerTourneyId) as u8),
                            );
                        } else {
                            ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(160u16);
                            matchNo = ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset((taskId2) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_sub(16i32);
                            DisplayMatchInfoOnCard(
                                ((((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    | 8i32) as u8),
                                ((matchNo) as u8),
                            );
                        }
                        {
                            i = 0i32;
                            'l18: loop {
                                if !(i < crate::c::div_i32(16i32, 2i32)) {
                                    break 'l18;
                                }
                                'l19: {
                                    if i < 2i32 {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_TrainerIconCardScrollLeft));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    ^ 1i32)
                                                    as i16),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i16),
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_MonIconCardScrollLeft));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    ^ 1i32)
                                                    as i16),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        {
                            i = crate::c::div_i32(16i32, 2i32);
                            'l20: loop {
                                if !(i < 16i32) {
                                    break 'l20;
                                }
                                'l21: {
                                    if i < 10i32 {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_TrainerIconCardScrollLeft));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read(),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i16),
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_MonIconCardScrollLeft));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read(),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(6i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(0i16);
                        break 'l5;
                    }
                    if __sw3 == 4i32 {
                        if (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read())
                            != 0
                        {
                            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(65280u16);
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                        } else {
                            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(65280u16);
                            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                        }
                        if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16))
                        .read()) as i32)
                            == 1i32
                        {
                            ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(160u16);
                        } else {
                            ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
                        }
                        matchNo = ((((((((&raw const sIdToMatchNumber).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset((taskId2) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 4,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(16))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize,
                        ))
                        .read()) as i32);
                        DisplayMatchInfoOnCard(
                            ((((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read()) as i32)
                                | 2i32) as u8),
                            ((matchNo) as u8),
                        );
                        {
                            i = 0i32;
                            'l22: loop {
                                if !(i < crate::c::div_i32(16i32, 2i32)) {
                                    break 'l22;
                                }
                                'l23: {
                                    if i < 2i32 {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_TrainerIconCardScrollRight));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    ^ 1i32)
                                                    as i16),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i16),
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_MonIconCardScrollRight));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    ^ 1i32)
                                                    as i16),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        {
                            i = crate::c::div_i32(16i32, 2i32);
                            'l24: loop {
                                if !(i < 16i32) {
                                    break 'l24;
                                }
                                'l25: {
                                    if i < 10i32 {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_TrainerIconCardScrollRight));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read(),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i16),
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_MonIconCardScrollRight));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read(),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(7i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(0i16);
                        break 'l5;
                    }
                    if __sw3 == 8i32 {
                        if (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read())
                            != 0
                        {
                            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(65280u16);
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                        } else {
                            ((&raw mut gBattle_BG0_X).cast::<u16>()).write(65280u16);
                            ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                        }
                        if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16))
                        .read()) as i32)
                            == 2i32
                        {
                            ((&raw mut gBattle_BG2_X).cast::<u16>()).write(256u16);
                            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(160u16);
                            trainerTourneyId =
                                ((((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(17))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32);
                            DisplayTrainerInfoOnCard(
                                ((((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    | 2i32) as u8),
                                ((trainerTourneyId) as u8),
                            );
                        } else {
                            ((&raw mut gBattle_BG2_X).cast::<u16>()).write(0u16);
                            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(160u16);
                            matchNo = ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset((taskId2) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_sub(16i32);
                            DisplayMatchInfoOnCard(
                                ((((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    | 2i32) as u8),
                                ((matchNo) as u8),
                            );
                        }
                        {
                            i = 0i32;
                            'l26: loop {
                                if !(i < crate::c::div_i32(16i32, 2i32)) {
                                    break 'l26;
                                }
                                'l27: {
                                    if i < 2i32 {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_TrainerIconCardScrollRight));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    ^ 1i32)
                                                    as i16),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i16),
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_MonIconCardScrollRight));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    ^ 1i32)
                                                    as i16),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        {
                            i = crate::c::div_i32(16i32, 2i32);
                            'l28: loop {
                                if !(i < 16i32) {
                                    break 'l28;
                                }
                                'l29: {
                                    if i < 10i32 {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_TrainerIconCardScrollRight));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read(),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i16),
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut sInfoCard)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            != 255i32
                                        {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(28)
                                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                            .write(Some(SpriteCB_MonIconCardScrollRight));
                                            (((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .write(
                                                ((((((&raw mut gTasks).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((taskId) as i32) as isize * 40,
                                                    ))
                                                .wrapping_add(8))
                                                .cast::<i16>())
                                                .wrapping_offset(2))
                                                .read(),
                                            );
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(1))
                                            .write(0i16);
                                            ((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut sInfoCard)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(46))
                                            .cast::<i16>())
                                            .wrapping_offset(2))
                                            .write(((i) as i16));
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(7i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(0i16);
                        break 'l5;
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (({
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    != 41i32
                {
                    let __p6 = (&raw mut gBattle_BG0_Y).cast::<u16>();
                    (__p6).write((((((__p6).read()) as i32).wrapping_sub(4i32)) as u16));
                    let __p7 = (&raw mut gBattle_BG1_Y).cast::<u16>();
                    (__p7).write((((((__p7).read()) as i32).wrapping_sub(4i32)) as u16));
                    let __p8 = (&raw mut gBattle_BG2_Y).cast::<u16>();
                    (__p8).write((((((__p8).read()) as i32).wrapping_sub(4i32)) as u16));
                } else {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (({
                    let __p9 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    != 41i32
                {
                    let __p11 = (&raw mut gBattle_BG0_Y).cast::<u16>();
                    (__p11).write((((((__p11).read()) as i32).wrapping_add(4i32)) as u16));
                    let __p12 = (&raw mut gBattle_BG1_Y).cast::<u16>();
                    (__p12).write((((((__p12).read()) as i32).wrapping_add(4i32)) as u16));
                    let __p13 = (&raw mut gBattle_BG2_Y).cast::<u16>();
                    (__p13).write((((((__p13).read()) as i32).wrapping_add(4i32)) as u16));
                } else {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (({
                    let __p14 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5);
                    let __t15 = ((__p14).read()).wrapping_add(1);
                    (__p14).write(__t15);
                    __t15
                }) as i32)
                    != 65i32
                {
                    let __p16 = (&raw mut gBattle_BG0_X).cast::<u16>();
                    (__p16).write((((((__p16).read()) as i32).wrapping_sub(4i32)) as u16));
                    let __p17 = (&raw mut gBattle_BG1_X).cast::<u16>();
                    (__p17).write((((((__p17).read()) as i32).wrapping_sub(4i32)) as u16));
                    let __p18 = (&raw mut gBattle_BG2_X).cast::<u16>();
                    (__p18).write((((((__p18).read()) as i32).wrapping_sub(4i32)) as u16));
                } else {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (({
                    let __p19 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5);
                    let __t20 = ((__p19).read()).wrapping_add(1);
                    (__p19).write(__t20);
                    __t20
                }) as i32)
                    != 65i32
                {
                    let __p21 = (&raw mut gBattle_BG0_X).cast::<u16>();
                    (__p21).write((((((__p21).read()) as i32).wrapping_add(4i32)) as u16));
                    let __p22 = (&raw mut gBattle_BG1_X).cast::<u16>();
                    (__p22).write((((((__p22).read()) as i32).wrapping_add(4i32)) as u16));
                    let __p23 = (&raw mut gBattle_BG2_X).cast::<u16>();
                    (__p23).write((((((__p23).read()) as i32).wrapping_add(4i32)) as u16));
                } else {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    {
                        i = 0i32;
                        'l30: loop {
                            if !(i < crate::c::div_i32(16i32, 2i32)) {
                                break 'l30;
                            }
                            'l31: {
                                if i < 2i32 {
                                    if (((((((&raw mut sInfoCard)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        != 255i32
                                    {
                                        FreeAndDestroyTrainerPicSprite(
                                            (((((((&raw mut sInfoCard)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as u16),
                                        );
                                    }
                                } else {
                                    if (((((((&raw mut sInfoCard)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        != 255i32
                                    {
                                        FreeAndDestroyMonIconSprite(
                                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ),
                                        );
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    {
                        i = crate::c::div_i32(16i32, 2i32);
                        'l32: loop {
                            if !(i < 16i32) {
                                break 'l32;
                            }
                            'l33: {
                                if i < 10i32 {
                                    if (((((((&raw mut sInfoCard)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        != 255i32
                                    {
                                        FreeAndDestroyTrainerPicSprite(
                                            (((((((&raw mut sInfoCard)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as u16),
                                        );
                                    }
                                } else {
                                    if (((((((&raw mut sInfoCard)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        != 255i32
                                    {
                                        FreeAndDestroyMonIconSprite(
                                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((((&raw mut sInfoCard)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ),
                                        );
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    FreeMonIconPalettes();
                    {
                        Free(((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                    FreeAllWindowBuffers();
                    if mode == 0i32 {
                        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                    } else {
                        i = ((CreateTask(Some(Task_ShowTourneyTree), 0u8)) as i32);
                        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                            .wrapping_add(8))
                        .cast::<i16>())
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                            .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                            .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(3i16);
                        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                            .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .write(
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read(),
                        );
                        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                            .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset((taskId2) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .read(),
                        );
                    }
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_GetInfoCardInput(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut input: u8 = 0u8;
        let mut taskId2: i32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as i32);
        let mut position: i32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset((taskId2) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32);
        let mut tourneyId: u8 = ((((&raw const sTourneyTreeTrainerIds).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset((position) as isize))
        .read();
        let mut roundId: u16 = (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1638)
        .cast::<u16>())
        .read();
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            input = 9u8;
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            == 0i32
        {
            return input;
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            == 1i32
        {
            if (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 64i32)
                != 0)
                && (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16))
                .read()) as i32)
                    == 0i32)
            {
                if position == 0i32 {
                    position = 15i32;
                } else {
                    position = (position).wrapping_sub(1);
                }
                input = 1u8;
            } else {
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 128i32)
                    != 0)
                    && (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32)
                        == 0i32)
                {
                    if position == 15i32 {
                        position = 0i32;
                    } else {
                        position = (position).wrapping_add(1);
                    }
                    input = 2u8;
                } else {
                    if (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 32i32)
                        != 0)
                        && (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16))
                        .read()) as i32)
                            != 0i32)
                    {
                        let __p1 = (((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16);
                        (__p1).write(((__p1).read()).wrapping_sub(1));
                        input = 3u8;
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 16i32)
                            != 0
                        {
                            if ((crate::c::bf_read(
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1752))
                                .cast::<u8>())
                                .wrapping_offset(((tourneyId) as i32) as isize * 4))
                                .wrapping_add(1),
                                2,
                                1,
                                false,
                            ) as u16)
                                != 0)
                                && (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(16))
                                .read()) as i32)
                                    .wrapping_sub(1i32)
                                    < ((crate::c::bf_read(
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1752))
                                        .cast::<u8>())
                                        .wrapping_offset(((tourneyId) as i32) as isize * 4))
                                        .wrapping_add(1),
                                        3,
                                        2,
                                        false,
                                    ) as u16) as i32))
                            {
                                let __p2 = (((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(16);
                                (__p2).write(((__p2).read()).wrapping_add(1));
                                input = 4u8;
                            }
                            if (!((crate::c::bf_read(
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1752))
                                .cast::<u8>())
                                .wrapping_offset(((tourneyId) as i32) as isize * 4))
                                .wrapping_add(1),
                                2,
                                1,
                                false,
                            ) as u16)
                                != 0))
                                && (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(16))
                                .read()) as i32)
                                    .wrapping_sub(1i32)
                                    < ((roundId) as i32))
                            {
                                let __p3 = (((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(16);
                                (__p3).write(((__p3).read()).wrapping_add(1));
                                input = 4u8;
                            }
                        }
                    }
                }
            }
            if ((input) as i32) == 9i32 {
                if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16))
                .read()) as i32)
                    != 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset((taskId2) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(
                        ((((((((&raw const sTrainerAndRoundToLastMatchCardNum)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((crate::c::div_i32(position, 2i32)) as isize * 4))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(16))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize,
                        ))
                        .read()) as i16),
                    );
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset((taskId2) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(((position) as i16));
                }
            }
        } else {
            if (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 64i32)
                != 0)
                && (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16))
                .read()) as i32)
                    == 1i32)
            {
                if position == 16i32 {
                    position = ((((((&raw const sLastMatchCardNum).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((roundId) as i32) as isize))
                    .read()) as i32);
                } else {
                    position = (position).wrapping_sub(1);
                }
                input = 5u8;
            } else {
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 128i32)
                    != 0)
                    && (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32)
                        == 1i32)
                {
                    if position
                        == ((((((&raw const sLastMatchCardNum).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((roundId) as i32) as isize))
                        .read()) as i32)
                    {
                        position = 16i32;
                    } else {
                        position = (position).wrapping_add(1);
                    }
                    input = 6u8;
                } else {
                    if (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 32i32)
                        != 0)
                        && (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16))
                        .read()) as i32)
                            != 0i32)
                    {
                        input = 7u8;
                        let __p4 = (((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16);
                        (__p4).write(((__p4).read()).wrapping_sub(1));
                    } else {
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 16i32)
                            != 0)
                            && ((((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(16))
                            .read()) as i32)
                                == 0i32)
                                || (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(16))
                                .read()) as i32)
                                    == 1i32))
                        {
                            input = 8u8;
                            let __p5 = (((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16);
                            (__p5).write(((__p5).read()).wrapping_add(1));
                        }
                    }
                }
            }
            if ((input) as i32) == 9i32 {
                if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16))
                .read()) as i32)
                    == 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset((taskId2) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(
                        ((((((&raw const sTournamentIdToPairedTrainerIds)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(17))
                            .cast::<u8>())
                            .read()) as i32) as isize,
                        ))
                        .read()) as i16),
                    );
                } else {
                    if ((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .read()) as i32)
                        == 2i32
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset((taskId2) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(
                            ((((((&raw const sTournamentIdToPairedTrainerIds)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(17))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i16),
                        );
                    } else {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset((taskId2) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(((position) as i16));
                    }
                }
            }
        }
        if (((input) as i32) != 0i32) && (((input) as i32) != 9i32) {
            PlaySE(5u16);
            ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((taskId2) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((position) as i16));
            let __p6 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p6).write((((((__p6).read()) as i32) ^ 1i32) as i16));
        }
        return input;
    }
}
pub(crate) unsafe extern "C" fn DisplayTrainerInfoOnCard(flags: u8, trainerTourneyId: u8) {
    unsafe {
        let mut flags = flags;
        let mut trainerTourneyId = trainerTourneyId;
        let mut textPrinter = crate::ffi::Align4([0u8; 16]);
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut trainerId: i32 = 0i32;
        let mut nature: u8 = 0u8;
        let mut arrId: i32 = 0i32;
        let mut windowId: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut palSlot: u8 = 0u8;
        let mut allocatedArray: *mut i16 = (AllocZeroed(
            (2u32).wrapping_mul(((if 18i32 >= 16i32 { 18i32 } else { 16i32 }) as u32)),
        ))
        .cast::<i16>();
        trainerId = ((crate::c::bf_read(
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1752))
            .cast::<u8>())
            .wrapping_offset(((trainerTourneyId) as i32) as isize * 4))
            .wrapping_add(0),
            0,
            10,
            false,
        ) as u16) as i32);
        if (((flags) as i32) & 1i32) != 0 {
            arrId = 8i32;
            windowId = 9i32;
            palSlot = 2u8;
        }
        if (((flags) as i32) & 2i32) != 0 {
            x = 256i32;
        }
        if (((flags) as i32) & 4i32) != 0 {
            y = 160i32;
        }
        if (((flags) as i32) & 8i32) != 0 {
            x = (-256i32);
        }
        if (((flags) as i32) & 16i32) != 0 {
            y = (-160i32);
        }
        if trainerId == 1023i32 {
            (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset((arrId) as isize))
            .write(
                ((CreateTrainerPicSprite(
                    PlayerGenderToFrontTrainerPicId(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                            .read(),
                    ),
                    1u8,
                    (((x).wrapping_add(48i32)) as i16),
                    (((y).wrapping_add(64i32)) as i16),
                    ((((palSlot) as i32).wrapping_add(12i32)) as u8),
                    65535u16,
                )) as u8),
            );
        } else {
            if trainerId == 1022i32 {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset((arrId) as isize))
                .write(
                    ((CreateTrainerPicSprite(
                        ((GetDomeBrainTrainerPicId()) as u16),
                        1u8,
                        (((x).wrapping_add(48i32)) as i16),
                        (((y).wrapping_add(64i32)) as i16),
                        ((((palSlot) as i32).wrapping_add(12i32)) as u8),
                        65535u16,
                    )) as u8),
                );
            } else {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset((arrId) as isize))
                .write(
                    ((CreateTrainerPicSprite(
                        ((GetFrontierTrainerFrontSpriteId(((trainerId) as u16))) as u16),
                        1u8,
                        (((x).wrapping_add(48i32)) as i16),
                        (((y).wrapping_add(64i32)) as i16),
                        ((((palSlot) as i32).wrapping_add(12i32)) as u8),
                        65535u16,
                    )) as u8),
                );
            }
        }
        if (((flags) as i32) & 30i32) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset((arrId) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if trainerId == 1023i32 {
                        (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset((((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize))
                        .write(CreateMonIcon(
                            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1816))
                            .cast::<u8>())
                            .wrapping_offset(((trainerTourneyId) as i32) as isize * 6))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read(),
                            Some(SpriteCB_MonIconDomeInfo),
                            ((x | ((((((&raw const sInfoTrainerMonX).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)) as i16),
                            (((y).wrapping_add(
                                ((((((&raw const sInfoTrainerMonY).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32),
                            )) as i16),
                            0u8,
                            0u32,
                            1u32,
                        ));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            (0u16) as i32,
                        );
                    } else {
                        if trainerId == 1022i32 {
                            (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                            ))
                            .write(CreateMonIcon(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1816))
                                .cast::<u8>())
                                .wrapping_offset(((trainerTourneyId) as i32) as isize * 6))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read(),
                                Some(SpriteCB_MonIconDomeInfo),
                                ((x | ((((((&raw const sInfoTrainerMonX).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)) as i16),
                                (((y).wrapping_add(
                                    ((((((&raw const sInfoTrainerMonY).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32),
                                )) as i16),
                                0u8,
                                0u32,
                                1u32,
                            ));
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                2,
                                2,
                                (0u16) as i32,
                            );
                        } else {
                            (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                            ))
                            .write(CreateMonIcon(
                                (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                    .wrapping_offset(
                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1816))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((trainerTourneyId) as i32) as isize * 6,
                                        ))
                                        .cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                .cast::<u16>())
                                .read(),
                                Some(SpriteCB_MonIconDomeInfo),
                                ((x | ((((((&raw const sInfoTrainerMonX).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)) as i16),
                                (((y).wrapping_add(
                                    ((((((&raw const sInfoTrainerMonY).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32),
                                )) as i16),
                                0u8,
                                0u32,
                                1u32,
                            ));
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                2,
                                2,
                                (0u16) as i32,
                            );
                        }
                    }
                    if (((flags) as i32) & 30i32) != 0 {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                ))
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
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(5)).write(2u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(6)).write(0u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).write(0u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8))
            .write((((&raw mut textPrinter).cast::<u8>()).wrapping_add(6)).read());
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9))
            .write((((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).read());
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(10)).write(2u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(11)).write(0u8);
        crate::c::bf_write(
            ((&raw mut textPrinter).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut textPrinter).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (14u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut textPrinter).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut textPrinter).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (13u8) as i32,
        );
        i = 0i32;
        if trainerId == 1023i32 {
            j = (((((&raw mut gFacilityClassToTrainerClass).cast::<u8>()).wrapping_offset(60))
                .read()) as i32);
        } else {
            if trainerId == 1022i32 {
                j = ((GetDomeBrainTrainerClass()) as i32);
            } else {
                j = ((GetFrontierOpponentClass(((trainerId) as u16))) as i32);
            }
        }
        {
            'l3: loop {
                if !((((((((&raw mut gTrainerClassNames).cast::<u8>())
                    .wrapping_offset((j) as isize * 13))
                .cast::<u8>())
                .wrapping_offset((i) as isize))
                .read()) as i32)
                    != 255i32)
                {
                    break 'l3;
                }
                'l4: {
                    (((&raw mut gStringVar1).cast::<u8>()).wrapping_offset((i) as isize)).write(
                        (((((&raw mut gTrainerClassNames).cast::<u8>())
                            .wrapping_offset((j) as isize * 13))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut gStringVar1).cast::<u8>()).wrapping_offset((i) as isize)).write(0u8);
        (((&raw mut gStringVar1).cast::<u8>()).wrapping_offset(((i).wrapping_add(1i32)) as isize))
            .write(255u8);
        if trainerId == 1023i32 {
            StringAppend(
                (&raw mut gStringVar1).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
        } else {
            if trainerId == 1022i32 {
                CopyDomeBrainTrainerName((&raw mut gStringVar2).cast::<u8>());
                StringAppend(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (&raw mut gStringVar2).cast::<u8>(),
                );
            } else {
                CopyDomeTrainerName((&raw mut gStringVar2).cast::<u8>(), ((trainerId) as u16));
                StringAppend(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (&raw mut gStringVar2).cast::<u8>(),
                );
            }
        }
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8)).write(
            ((GetStringCenterAlignXOffsetWithLetterSpacing(
                (((((&raw mut textPrinter).cast::<u8>()).wrapping_add(5)).read()) as i32),
                (&raw mut gStringVar1).cast::<u8>(),
                208i32,
                (((((&raw mut textPrinter).cast::<u8>()).wrapping_add(10)).read()) as i32),
            )) as u8),
        );
        (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>())
            .write((&raw mut gStringVar1).cast::<u8>());
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(4)).write(((windowId) as u8));
        PutWindowTilemap(((windowId) as u8));
        CopyWindowToVram(((windowId) as u8), 3u8);
        AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(10)).write(0u8);
        {
            i = 0i32;
            'l5: loop {
                if !(i < 3i32) {
                    break 'l5;
                }
                'l6: {
                    (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9)).write(
                        ((((&raw const sSpeciesNameTextYCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    if trainerId == 1023i32 {
                        (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).write(
                            (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                                (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(1816))
                                .cast::<u8>())
                                .wrapping_offset(((trainerTourneyId) as i32) as isize * 6))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 11,
                            ))
                            .cast::<u8>(),
                        );
                    } else {
                        if trainerId == 1022i32 {
                            (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).write(
                                (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                                    (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(1816))
                                    .cast::<u8>())
                                    .wrapping_offset(((trainerTourneyId) as i32) as isize * 6))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize
                                        * 11,
                                ))
                                .cast::<u8>(),
                            );
                        } else {
                            (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).write(
                                (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                                    (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(
                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1816))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((trainerTourneyId) as i32) as isize * 6,
                                        ))
                                        .cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .cast::<u16>())
                                    .read()) as i32) as isize
                                        * 11,
                                ))
                                .cast::<u8>(),
                            );
                        }
                    }
                    (((&raw mut textPrinter).cast::<u8>()).wrapping_add(4))
                        .write(((((1i32).wrapping_add(i)).wrapping_add(windowId)) as u8));
                    if i == 1i32 {
                        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8)).write(7u8);
                    } else {
                        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8)).write(0u8);
                    }
                    PutWindowTilemap(((((1i32).wrapping_add(i)).wrapping_add(windowId)) as u8));
                    CopyWindowToVram(
                        ((((1i32).wrapping_add(i)).wrapping_add(windowId)) as u8),
                        3u8,
                    );
                    AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
                }
                i = (i).wrapping_add(1);
            }
        }
        PutWindowTilemap((((windowId).wrapping_add(4i32)) as u8));
        CopyWindowToVram((((windowId).wrapping_add(4i32)) as u8), 3u8);
        if trainerId == 1022i32 {
            (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).write(
                ((((&raw const sBattleDomePotentialTexts)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(16))
                .read(),
            );
        } else {
            (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).write(
                ((((&raw const sBattleDomePotentialTexts)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(((trainerTourneyId) as i32) as isize))
                .read(),
            );
        }
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(5)).write(1u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(4))
            .write((((windowId).wrapping_add(4i32)) as u8));
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8)).write(0u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).write(4u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9)).write(4u8);
        AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
        {
            i = 0i32;
            'l7: loop {
                if !(i < 3i32) {
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
                                {
                                    k = 0i32;
                                    'l11: loop {
                                        if !(k < 16i32) {
                                            break 'l11;
                                        }
                                        'l12: {
                                            if trainerId == 1022i32 {
                                                let __p1 =
                                                    (allocatedArray).wrapping_offset((k) as isize);
                                                (__p1).write((((((((__p1).read()) as i32))).wrapping_add((((((((((&raw const sBattleStyleMovePoints).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset((((GetFrontierBrainMonMove(((i) as u8), ((j) as u8))) as i32)) as isize * 16)).cast::<u8>()).wrapping_offset((k) as isize)).read()) as i32)))) as i16));
                                            } else {
                                                if trainerId == 1023i32 {
                                                    let __p2 = (allocatedArray)
                                                        .wrapping_offset((k) as isize);
                                                    (__p2).write((((((((__p2).read()) as i32))).wrapping_add((((((((((&raw const sBattleStyleMovePoints).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset((((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612)).wrapping_add(2224)).cast::<u8>()).wrapping_offset((i) as isize * 16))).cast::<u16>()).wrapping_offset((j) as isize)).read()) as i32)) as isize * 16)).cast::<u8>()).wrapping_offset((k) as isize)).read()) as i32)))) as i16));
                                                } else {
                                                    let __p3 = (allocatedArray)
                                                        .wrapping_offset((k) as isize);
                                                    (__p3).write((((((((__p3).read()) as i32))).wrapping_add((((((((((&raw const sBattleStyleMovePoints).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(((((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset(((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612)).wrapping_add(1816)).cast::<u8>()).wrapping_offset((((trainerTourneyId) as i32)) as isize * 6)).cast::<u16>()).wrapping_offset((i) as isize)).read()) as i32)) as isize * 16)).wrapping_add(2)).cast::<u16>()).wrapping_offset((j) as isize)).read()) as i32)) as isize * 16)).cast::<u8>()).wrapping_offset((k) as isize)).read()) as i32)))) as i16));
                                                }
                                            }
                                        }
                                        k = (k).wrapping_add(1);
                                    }
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
            'l13: loop {
                if !(((i) as u32) < crate::c::div_u32(496u32, 16u32)) {
                    break 'l13;
                }
                'l14: {
                    let mut thresholdStatCount: i32 = 0i32;
                    {
                        k = 0i32;
                        j = 0i32;
                        'l15: loop {
                            if !(j < 16i32) {
                                break 'l15;
                            }
                            'l16: {
                                if ((((((((&raw const sBattleStyleThresholds)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 16))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .read()) as i32)
                                    != 0i32
                                {
                                    thresholdStatCount = (thresholdStatCount).wrapping_add(1);
                                    if (((((allocatedArray).wrapping_offset((j) as isize)).read())
                                        as i32)
                                        != 0i32)
                                        && (((((allocatedArray).wrapping_offset((j) as isize))
                                            .read())
                                            as i32)
                                            >= ((((((((&raw const sBattleStyleThresholds)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 16))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32))
                                    {
                                        k = (k).wrapping_add(1);
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if thresholdStatCount == k {
                        break 'l13;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).write(
            ((((&raw const sBattleDomeOpponentStyleTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset((i) as isize))
            .read(),
        );
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).write(20u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9)).write(20u8);
        AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
        {
            i = 0i32;
            'l17: loop {
                if !(i < (if 18i32 >= 16i32 { 18i32 } else { 16i32 })) {
                    break 'l17;
                }
                'l18: {
                    ((allocatedArray).wrapping_offset((i) as isize)).write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
        if (trainerId == 1022i32) || (trainerId == 1023i32) {
            {
                i = 0i32;
                'l19: loop {
                    if !(i < 3i32) {
                        break 'l19;
                    }
                    'l20: {
                        {
                            j = 0i32;
                            'l21: loop {
                                if !(j < 6i32) {
                                    break 'l21;
                                }
                                'l22: {
                                    if trainerId == 1022i32 {
                                        ((allocatedArray).wrapping_offset((j) as isize)).write(
                                            ((GetFrontierBrainMonEvs(((i) as u8), ((j) as u8)))
                                                as i16),
                                        );
                                    } else {
                                        ((allocatedArray).wrapping_offset((j) as isize)).write(
                                            ((((((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(2224))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 16))
                                            .wrapping_add(8))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i16),
                                        );
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        let __p4 = (allocatedArray).wrapping_offset(6);
                        (__p4).write(
                            (((((__p4).read()) as i32)
                                .wrapping_add((((allocatedArray).read()) as i32)))
                                as i16),
                        );
                        {
                            j = 0i32;
                            'l23: loop {
                                if !(j < 5i32) {
                                    break 'l23;
                                }
                                'l24: {
                                    if trainerId == 1022i32 {
                                        nature = GetFrontierBrainMonNature(((i) as u8));
                                    } else {
                                        nature = ((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(2224))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 16))
                                        .wrapping_add(14))
                                        .read();
                                    }
                                    if (((((((&raw mut gNatureStatTable).cast::<u8>())
                                        .wrapping_offset(((nature) as i32) as isize * 5))
                                    .cast::<i8>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                        > 0i32
                                    {
                                        let __p5 = (allocatedArray).wrapping_offset(
                                            (((j).wrapping_add(6i32)).wrapping_add(1i32)) as isize,
                                        );
                                        (__p5).write(
                                            (((((__p5).read()) as i32).wrapping_add(
                                                crate::c::div_i32(
                                                    ((((allocatedArray).wrapping_offset(
                                                        ((j).wrapping_add(1i32)) as isize,
                                                    ))
                                                    .read())
                                                        as i32)
                                                        .wrapping_mul(110i32),
                                                    100i32,
                                                ),
                                            )) as i16),
                                        );
                                    } else {
                                        if (((((((&raw mut gNatureStatTable).cast::<u8>())
                                            .wrapping_offset(((nature) as i32) as isize * 5))
                                        .cast::<i8>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            < 0i32
                                        {
                                            let __p6 = (allocatedArray).wrapping_offset(
                                                (((j).wrapping_add(6i32)).wrapping_add(1i32))
                                                    as isize,
                                            );
                                            (__p6).write(
                                                (((((__p6).read()) as i32).wrapping_add(
                                                    crate::c::div_i32(
                                                        ((((allocatedArray).wrapping_offset(
                                                            ((j).wrapping_add(1i32)) as isize,
                                                        ))
                                                        .read())
                                                            as i32)
                                                            .wrapping_mul(90i32),
                                                        100i32,
                                                    ),
                                                ))
                                                    as i16),
                                            );
                                            let __p7 = (allocatedArray).wrapping_offset(
                                                ((((j).wrapping_add(6i32)).wrapping_add(5i32))
                                                    .wrapping_add(2i32))
                                                    as isize,
                                            );
                                            (__p7).write(((__p7).read()).wrapping_add(1));
                                        } else {
                                            let __p8 = (allocatedArray).wrapping_offset(
                                                (((j).wrapping_add(6i32)).wrapping_add(1i32))
                                                    as isize,
                                            );
                                            (__p8).write(
                                                (((((__p8).read()) as i32).wrapping_add(
                                                    ((((allocatedArray).wrapping_offset(
                                                        ((j).wrapping_add(1i32)) as isize,
                                                    ))
                                                    .read())
                                                        as i32),
                                                ))
                                                    as i16),
                                            );
                                        }
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
                j = 0i32;
                i = 0i32;
                'l25: loop {
                    if !(i < 6i32) {
                        break 'l25;
                    }
                    'l26: {
                        j = (j).wrapping_add(
                            ((((allocatedArray).wrapping_offset(((6i32).wrapping_add(i)) as isize))
                                .read()) as i32),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l27: loop {
                    if !(i < 6i32) {
                        break 'l27;
                    }
                    'l28: {
                        ((allocatedArray).wrapping_offset((i) as isize)).write(
                            ((crate::c::div_i32(
                                ((((allocatedArray)
                                    .wrapping_offset(((6i32).wrapping_add(i)) as isize))
                                .read()) as i32)
                                    .wrapping_mul(100i32),
                                j,
                            )) as i16),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0i32;
                'l29: loop {
                    if !(i < 3i32) {
                        break 'l29;
                    }
                    'l30: {
                        let mut evBits: i32 = (((((((&raw mut gFacilityTrainerMons)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(
                            (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1816))
                            .cast::<u8>())
                            .wrapping_offset(((trainerTourneyId) as i32) as isize * 6))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 16,
                        ))
                        .wrapping_add(11))
                        .read()) as i32);
                        {
                            k = 0i32;
                            j = 0i32;
                            'l31: loop {
                                if !(j < 6i32) {
                                    break 'l31;
                                }
                                'l32: {
                                    ((allocatedArray).wrapping_offset((j) as isize)).write(0i16);
                                    if (evBits & 1i32) != 0 {
                                        k = (k).wrapping_add(1);
                                    }
                                    evBits = (evBits >> 1);
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        k = crate::c::div_i32(510i32, k);
                        evBits = (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(
                                (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(1816))
                                .cast::<u8>())
                                .wrapping_offset(((trainerTourneyId) as i32) as isize * 6))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 16,
                            ))
                        .wrapping_add(11))
                        .read()) as i32);
                        {
                            j = 0i32;
                            'l33: loop {
                                if !(j < 6i32) {
                                    break 'l33;
                                }
                                'l34: {
                                    if (evBits & 1i32) != 0 {
                                        ((allocatedArray).wrapping_offset((j) as isize))
                                            .write(((k) as i16));
                                    }
                                    evBits = (evBits >> 1);
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        let __p9 = (allocatedArray).wrapping_offset(6);
                        (__p9).write(
                            (((((__p9).read()) as i32)
                                .wrapping_add((((allocatedArray).read()) as i32)))
                                as i16),
                        );
                        {
                            j = 0i32;
                            'l35: loop {
                                if !(j < 5i32) {
                                    break 'l35;
                                }
                                'l36: {
                                    nature = (((((&raw mut gFacilityTrainerMons)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(
                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1816))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((trainerTourneyId) as i32) as isize * 6,
                                        ))
                                        .cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(12))
                                    .read();
                                    if (((((((&raw mut gNatureStatTable).cast::<u8>())
                                        .wrapping_offset(((nature) as i32) as isize * 5))
                                    .cast::<i8>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                        > 0i32
                                    {
                                        let __p10 = (allocatedArray).wrapping_offset(
                                            (((j).wrapping_add(6i32)).wrapping_add(1i32)) as isize,
                                        );
                                        (__p10).write(
                                            (((((__p10).read()) as i32).wrapping_add(
                                                crate::c::div_i32(
                                                    ((((allocatedArray).wrapping_offset(
                                                        ((j).wrapping_add(1i32)) as isize,
                                                    ))
                                                    .read())
                                                        as i32)
                                                        .wrapping_mul(110i32),
                                                    100i32,
                                                ),
                                            )) as i16),
                                        );
                                    } else {
                                        if (((((((&raw mut gNatureStatTable).cast::<u8>())
                                            .wrapping_offset(((nature) as i32) as isize * 5))
                                        .cast::<i8>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            < 0i32
                                        {
                                            let __p11 = (allocatedArray).wrapping_offset(
                                                (((j).wrapping_add(6i32)).wrapping_add(1i32))
                                                    as isize,
                                            );
                                            (__p11).write(
                                                (((((__p11).read()) as i32).wrapping_add(
                                                    crate::c::div_i32(
                                                        ((((allocatedArray).wrapping_offset(
                                                            ((j).wrapping_add(1i32)) as isize,
                                                        ))
                                                        .read())
                                                            as i32)
                                                            .wrapping_mul(90i32),
                                                        100i32,
                                                    ),
                                                ))
                                                    as i16),
                                            );
                                            let __p12 = (allocatedArray).wrapping_offset(
                                                ((((j).wrapping_add(6i32)).wrapping_add(5i32))
                                                    .wrapping_add(2i32))
                                                    as isize,
                                            );
                                            (__p12).write(((__p12).read()).wrapping_add(1));
                                        } else {
                                            let __p13 = (allocatedArray).wrapping_offset(
                                                (((j).wrapping_add(6i32)).wrapping_add(1i32))
                                                    as isize,
                                            );
                                            (__p13).write(
                                                (((((__p13).read()) as i32).wrapping_add(
                                                    ((((allocatedArray).wrapping_offset(
                                                        ((j).wrapping_add(1i32)) as isize,
                                                    ))
                                                    .read())
                                                        as i32),
                                                ))
                                                    as i16),
                                            );
                                        }
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
                j = 0i32;
                i = 0i32;
                'l37: loop {
                    if !(i < 6i32) {
                        break 'l37;
                    }
                    'l38: {
                        j = (j).wrapping_add(
                            ((((allocatedArray).wrapping_offset(((i).wrapping_add(6i32)) as isize))
                                .read()) as i32),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l39: loop {
                    if !(i < 6i32) {
                        break 'l39;
                    }
                    'l40: {
                        ((allocatedArray).wrapping_offset((i) as isize)).write(
                            ((crate::c::div_i32(
                                ((((allocatedArray)
                                    .wrapping_offset(((6i32).wrapping_add(i)) as isize))
                                .read()) as i32)
                                    .wrapping_mul(100i32),
                                j,
                            )) as i16),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        {
            i = 0i32;
            j = 0i32;
            k = 0i32;
            'l41: loop {
                if !(k < 6i32) {
                    break 'l41;
                }
                'l42: {
                    if ((((allocatedArray).wrapping_offset((k) as isize)).read()) as i32) > 29i32 {
                        if i == 2i32 {
                            if ((((allocatedArray).wrapping_offset(6)).read()) as i32)
                                < ((((allocatedArray).wrapping_offset((k) as isize)).read()) as i32)
                            {
                                if ((((allocatedArray).wrapping_offset(7)).read()) as i32)
                                    < ((((allocatedArray).wrapping_offset((k) as isize)).read())
                                        as i32)
                                {
                                    if ((((allocatedArray).wrapping_offset(6)).read()) as i32)
                                        < ((((allocatedArray).wrapping_offset(7)).read()) as i32)
                                    {
                                        ((allocatedArray).wrapping_offset(6))
                                            .write(((allocatedArray).wrapping_offset(7)).read());
                                        ((allocatedArray).wrapping_offset(7)).write(((k) as i16));
                                    } else {
                                        ((allocatedArray).wrapping_offset(7)).write(((k) as i16));
                                    }
                                } else {
                                    ((allocatedArray).wrapping_offset(6))
                                        .write(((allocatedArray).wrapping_offset(7)).read());
                                    ((allocatedArray).wrapping_offset(7)).write(((k) as i16));
                                }
                            } else {
                                if ((((allocatedArray).wrapping_offset(7)).read()) as i32)
                                    < ((((allocatedArray).wrapping_offset((k) as isize)).read())
                                        as i32)
                                {
                                    ((allocatedArray).wrapping_offset(7)).write(((k) as i16));
                                }
                            }
                        } else {
                            ((allocatedArray).wrapping_offset(((i).wrapping_add(6i32)) as isize))
                                .write(((k) as i16));
                            i = (i).wrapping_add(1);
                        }
                    }
                    if ((((allocatedArray).wrapping_offset((k) as isize)).read()) as i32) == 0i32 {
                        if j == 2i32 {
                            if (((((allocatedArray)
                                .wrapping_offset(((k).wrapping_add(12i32)) as isize))
                            .read()) as i32)
                                >= 2i32)
                                || (((((((allocatedArray)
                                    .wrapping_offset(((k).wrapping_add(12i32)) as isize))
                                .read()) as i32)
                                    == 1i32)
                                    && (((((allocatedArray).wrapping_offset(
                                        ((12i32).wrapping_add(
                                            ((((allocatedArray).wrapping_offset(8)).read()) as i32),
                                        )) as isize,
                                    ))
                                    .read()) as i32)
                                        == 0i32))
                                    && (((((allocatedArray).wrapping_offset(
                                        ((12i32).wrapping_add(
                                            ((((allocatedArray).wrapping_offset(9)).read()) as i32),
                                        )) as isize,
                                    ))
                                    .read()) as i32)
                                        == 0i32))
                            {
                                ((allocatedArray).wrapping_offset(8))
                                    .write(((allocatedArray).wrapping_offset(9)).read());
                                ((allocatedArray).wrapping_offset(9)).write(((k) as i16));
                            } else {
                                if (((((allocatedArray)
                                    .wrapping_offset(((k).wrapping_add(12i32)) as isize))
                                .read()) as i32)
                                    == 1i32)
                                    && (((((allocatedArray).wrapping_offset(
                                        ((12i32).wrapping_add(
                                            ((((allocatedArray).wrapping_offset(8)).read()) as i32),
                                        )) as isize,
                                    ))
                                    .read()) as i32)
                                        == 0i32)
                                {
                                    ((allocatedArray).wrapping_offset(8))
                                        .write(((allocatedArray).wrapping_offset(9)).read());
                                    ((allocatedArray).wrapping_offset(9)).write(((k) as i16));
                                } else {
                                    if (((((allocatedArray)
                                        .wrapping_offset(((k).wrapping_add(12i32)) as isize))
                                    .read()) as i32)
                                        == 1i32)
                                        && (((((allocatedArray).wrapping_offset(
                                            ((12i32).wrapping_add(
                                                ((((allocatedArray).wrapping_offset(9)).read())
                                                    as i32),
                                            )) as isize,
                                        ))
                                        .read())
                                            as i32)
                                            == 0i32)
                                    {
                                        ((allocatedArray).wrapping_offset(9)).write(((k) as i16));
                                    }
                                }
                            }
                        } else {
                            ((allocatedArray).wrapping_offset(((j).wrapping_add(8i32)) as isize))
                                .write(((k) as i16));
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                k = (k).wrapping_add(1);
            }
        }
        if i == 2i32 {
            i = (((((((&raw const sStatTextOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((((allocatedArray).wrapping_offset(6)).read()) as i32) as isize))
            .read()) as i32)
                .wrapping_add(
                    ((((allocatedArray).wrapping_offset(7)).read()) as i32).wrapping_sub(
                        ((((allocatedArray).wrapping_offset(6)).read()) as i32).wrapping_add(1i32),
                    ),
                ))
            .wrapping_add(0i32);
        } else {
            if i == 1i32 {
                i = ((((allocatedArray).wrapping_offset(6)).read()) as i32).wrapping_add(15i32);
            } else {
                if j == 2i32 {
                    i = (((((((&raw const sStatTextOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((allocatedArray).wrapping_offset(8)).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                        .wrapping_add(
                            ((((allocatedArray).wrapping_offset(9)).read()) as i32).wrapping_sub(
                                ((((allocatedArray).wrapping_offset(8)).read()) as i32)
                                    .wrapping_add(1i32),
                            ),
                        ))
                    .wrapping_add(21i32);
                } else {
                    if j == 1i32 {
                        i = ((((allocatedArray).wrapping_offset(8)).read()) as i32)
                            .wrapping_add(36i32);
                    } else {
                        i = 42i32;
                    }
                }
            }
        }
        (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).write(
            ((((&raw const sBattleDomeOpponentStatsTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset((i) as isize))
            .read(),
        );
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).write(36u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9)).write(36u8);
        AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
        Free((allocatedArray).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn BufferDomeWinString(matchNum: u8, tournamentIds: *mut u8) -> i32 {
    unsafe {
        let mut matchNum = matchNum;
        let mut tournamentIds = tournamentIds;
        let mut i: i32 = 0i32;
        let mut tournamentId: u8 = 0u8;
        let mut winStringId: i32 = 0i32;
        let mut count: i32 = 0i32;
        {
            i = (((((((&raw const sCompetitorRangeByMatch).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((matchNum) as i32) as isize * 3))
            .cast::<u8>())
            .read()) as i32);
            'l1: loop {
                if !(i
                    < (((((((&raw const sCompetitorRangeByMatch).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((matchNum) as i32) as isize * 3))
                    .cast::<u8>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((((((&raw const sCompetitorRangeByMatch)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((matchNum) as i32) as isize * 3))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32),
                        ))
                {
                    break 'l1;
                }
                'l2: {
                    tournamentId =
                        ((((&raw const sTourneyTreeTrainerIds2).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read();
                    if !((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset(((tournamentId) as i32) as isize * 4))
                        .wrapping_add(1),
                        2,
                        1,
                        false,
                    ) as u16)
                        != 0)
                    {
                        ((tournamentIds).wrapping_offset((count) as isize)).write(tournamentId);
                        if ((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset(((tournamentId) as i32) as isize * 4))
                            .wrapping_add(0),
                            0,
                            10,
                            false,
                        ) as u16) as i32)
                            == 1023i32
                        {
                            StringCopy(
                                (&raw mut gStringVar1).cast::<u8>(),
                                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                            );
                        } else {
                            if ((crate::c::bf_read(
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1752))
                                .cast::<u8>())
                                .wrapping_offset(((tournamentId) as i32) as isize * 4))
                                .wrapping_add(0),
                                0,
                                10,
                                false,
                            ) as u16) as i32)
                                == 1022i32
                            {
                                CopyDomeBrainTrainerName((&raw mut gStringVar1).cast::<u8>());
                            } else {
                                CopyDomeTrainerName(
                                    (&raw mut gStringVar1).cast::<u8>(),
                                    (crate::c::bf_read(
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1752))
                                        .cast::<u8>())
                                        .wrapping_offset(((tournamentId) as i32) as isize * 4))
                                        .wrapping_add(0),
                                        0,
                                        10,
                                        false,
                                    ) as u16),
                                );
                            }
                        }
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if count == 2i32 {
            return 0i32;
        }
        {
            i = (((((((&raw const sCompetitorRangeByMatch).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((matchNum) as i32) as isize * 3))
            .cast::<u8>())
            .read()) as i32);
            'l3: loop {
                if !(i
                    < (((((((&raw const sCompetitorRangeByMatch).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((matchNum) as i32) as isize * 3))
                    .cast::<u8>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((((((&raw const sCompetitorRangeByMatch)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((matchNum) as i32) as isize * 3))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32),
                        ))
                {
                    break 'l3;
                }
                'l4: {
                    tournamentId =
                        ((((&raw const sTourneyTreeTrainerIds2).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read();
                    if ((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset(((tournamentId) as i32) as isize * 4))
                        .wrapping_add(1),
                        2,
                        1,
                        false,
                    ) as u16)
                        != 0)
                        && (((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset(((tournamentId) as i32) as isize * 4))
                            .wrapping_add(1),
                            3,
                            2,
                            false,
                        ) as u16) as i32)
                            >= ((((((((&raw const sCompetitorRangeByMatch)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((matchNum) as i32) as isize * 3))
                            .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32))
                    {
                        ((tournamentIds).wrapping_offset((count) as isize)).write(tournamentId);
                        count = (count).wrapping_add(1);
                        if ((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset(((tournamentId) as i32) as isize * 4))
                            .wrapping_add(1),
                            3,
                            2,
                            false,
                        ) as u16) as i32)
                            == ((((((((&raw const sCompetitorRangeByMatch)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((matchNum) as i32) as isize * 3))
                            .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32)
                        {
                            StringCopy(
                                (&raw mut gStringVar2).cast::<u8>(),
                                (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(2164))
                                    .cast::<u16>())
                                    .wrapping_offset(((tournamentId) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 13,
                                ))
                                .cast::<u8>(),
                            );
                            winStringId = ((crate::c::bf_read(
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1752))
                                .cast::<u8>())
                                .wrapping_offset(((tournamentId) as i32) as isize * 4))
                                .wrapping_add(1),
                                5,
                                3,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(2i32);
                            if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2164))
                            .cast::<u16>())
                            .wrapping_offset(((tournamentId) as i32) as isize))
                            .read()) as i32)
                                == 0i32)
                                && (((crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1752))
                                    .cast::<u8>())
                                    .wrapping_offset(((tournamentId) as i32) as isize * 4))
                                    .wrapping_add(1),
                                    5,
                                    3,
                                    false,
                                ) as u16) as i32)
                                    == 0i32)
                            {
                                winStringId = 4i32;
                            }
                        } else {
                            if ((crate::c::bf_read(
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1752))
                                .cast::<u8>())
                                .wrapping_offset(((tournamentId) as i32) as isize * 4))
                                .wrapping_add(0),
                                0,
                                10,
                                false,
                            ) as u16) as i32)
                                == 1023i32
                            {
                                StringCopy(
                                    (&raw mut gStringVar1).cast::<u8>(),
                                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .cast::<u8>(),
                                );
                            } else {
                                if ((crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1752))
                                    .cast::<u8>())
                                    .wrapping_offset(((tournamentId) as i32) as isize * 4))
                                    .wrapping_add(0),
                                    0,
                                    10,
                                    false,
                                ) as u16) as i32)
                                    == 1022i32
                                {
                                    CopyDomeBrainTrainerName((&raw mut gStringVar1).cast::<u8>());
                                } else {
                                    CopyDomeTrainerName(
                                        (&raw mut gStringVar1).cast::<u8>(),
                                        (crate::c::bf_read(
                                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1752))
                                            .cast::<u8>())
                                            .wrapping_offset(((tournamentId) as i32) as isize * 4))
                                            .wrapping_add(0),
                                            0,
                                            10,
                                            false,
                                        ) as u16),
                                    );
                                }
                            }
                        }
                    }
                    if count == 2i32 {
                        break 'l3;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((matchNum) as i32) == 14i32 {
            return (winStringId).wrapping_add(2i32);
        } else {
            return (winStringId).wrapping_add(1i32);
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayMatchInfoOnCard(flags: u8, matchNo: u8) {
    unsafe {
        let mut flags = flags;
        let mut matchNo = matchNo;
        let mut textPrinter = crate::ffi::Align4([0u8; 16]);
        let mut tournamentIds = crate::ffi::Align4([0u8; 8]);
        let mut trainerIds = crate::ffi::Align4([0u8; 8]);
        let mut lost = crate::ffi::Align4([0u8; 8]);
        let mut i: i32 = 0i32;
        let mut winStringId: i32 = 0i32;
        let mut arrId: i32 = 0i32;
        let mut windowId: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut palSlot: u8 = 0u8;
        if (((flags) as i32) & 1i32) != 0 {
            arrId = 8i32;
            windowId = 9i32;
            palSlot = 2u8;
        }
        if (((flags) as i32) & 2i32) != 0 {
            x = 256i32;
        }
        if (((flags) as i32) & 4i32) != 0 {
            y = 160i32;
        }
        if (((flags) as i32) & 8i32) != 0 {
            x = (-256i32);
        }
        if (((flags) as i32) & 16i32) != 0 {
            y = (-160i32);
        }
        winStringId = BufferDomeWinString(
            matchNo,
            ((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(17))
                .cast::<u8>(),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 2i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut tournamentIds).cast::<i32>()).wrapping_offset((i) as isize)).write(
                        ((((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(17))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32),
                    );
                    (((&raw mut trainerIds).cast::<i32>()).wrapping_offset((i) as isize)).write(
                        ((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut tournamentIds).cast::<i32>())
                                    .wrapping_offset((i) as isize))
                                .read()) as isize
                                    * 4,
                            ))
                            .wrapping_add(0),
                            0,
                            10,
                            false,
                        ) as u16) as i32),
                    );
                    if (((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut tournamentIds).cast::<i32>())
                                .wrapping_offset((i) as isize))
                            .read()) as isize
                                * 4,
                        ))
                        .wrapping_add(1),
                        3,
                        2,
                        false,
                    ) as u16) as i32)
                        <= ((((((((&raw const sCompetitorRangeByMatch).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((matchNo) as i32) as isize * 3))
                        .cast::<u8>())
                        .wrapping_offset(2))
                        .read()) as i32))
                        && ((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut tournamentIds).cast::<i32>())
                                    .wrapping_offset((i) as isize))
                                .read()) as isize
                                    * 4,
                            ))
                            .wrapping_add(1),
                            2,
                            1,
                            false,
                        ) as u16)
                            != 0)
                    {
                        (((&raw mut lost).cast::<u32>()).wrapping_offset((i) as isize)).write(1u32);
                    } else {
                        (((&raw mut lost).cast::<u32>()).wrapping_offset((i) as isize)).write(0u32);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((&raw mut trainerIds).cast::<i32>()).read() == 1023i32 {
            (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset((arrId) as isize))
            .write(
                ((CreateTrainerPicSprite(
                    PlayerGenderToFrontTrainerPicId(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                            .read(),
                    ),
                    1u8,
                    (((x).wrapping_add(48i32)) as i16),
                    (((y).wrapping_add(88i32)) as i16),
                    ((((palSlot) as i32).wrapping_add(12i32)) as u8),
                    65535u16,
                )) as u8),
            );
        } else {
            if ((&raw mut trainerIds).cast::<i32>()).read() == 1022i32 {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset((arrId) as isize))
                .write(
                    ((CreateTrainerPicSprite(
                        ((GetDomeBrainTrainerPicId()) as u16),
                        1u8,
                        (((x).wrapping_add(48i32)) as i16),
                        (((y).wrapping_add(88i32)) as i16),
                        ((((palSlot) as i32).wrapping_add(12i32)) as u8),
                        65535u16,
                    )) as u8),
                );
            } else {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset((arrId) as isize))
                .write(
                    ((CreateTrainerPicSprite(
                        ((GetFrontierTrainerFrontSpriteId(
                            ((((&raw mut trainerIds).cast::<i32>()).read()) as u16),
                        )) as u16),
                        1u8,
                        (((x).wrapping_add(48i32)) as i16),
                        (((y).wrapping_add(88i32)) as i16),
                        ((((palSlot) as i32).wrapping_add(12i32)) as u8),
                        65535u16,
                    )) as u8),
                );
            }
        }
        if (((flags) as i32) & 30i32) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset((arrId) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        if (((&raw mut lost).cast::<u32>()).read()) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset((arrId) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                4,
                4,
                (3u16) as i32,
            );
        }
        if (((&raw mut trainerIds).cast::<i32>()).wrapping_offset(1)).read() == 1023i32 {
            (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(((1i32).wrapping_add(arrId)) as isize))
            .write(
                ((CreateTrainerPicSprite(
                    PlayerGenderToFrontTrainerPicId(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                            .read(),
                    ),
                    1u8,
                    (((x).wrapping_add(192i32)) as i16),
                    (((y).wrapping_add(88i32)) as i16),
                    ((((palSlot) as i32).wrapping_add(13i32)) as u8),
                    65535u16,
                )) as u8),
            );
        } else {
            if (((&raw mut trainerIds).cast::<i32>()).wrapping_offset(1)).read() == 1022i32 {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(((1i32).wrapping_add(arrId)) as isize))
                .write(
                    ((CreateTrainerPicSprite(
                        ((GetDomeBrainTrainerPicId()) as u16),
                        1u8,
                        (((x).wrapping_add(192i32)) as i16),
                        (((y).wrapping_add(88i32)) as i16),
                        ((((palSlot) as i32).wrapping_add(13i32)) as u8),
                        65535u16,
                    )) as u8),
                );
            } else {
                (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(((1i32).wrapping_add(arrId)) as isize))
                .write(
                    ((CreateTrainerPicSprite(
                        ((GetFrontierTrainerFrontSpriteId(
                            (((((&raw mut trainerIds).cast::<i32>()).wrapping_offset(1)).read())
                                as u16),
                        )) as u16),
                        1u8,
                        (((x).wrapping_add(192i32)) as i16),
                        (((y).wrapping_add(88i32)) as i16),
                        ((((palSlot) as i32).wrapping_add(13i32)) as u8),
                        65535u16,
                    )) as u8),
                );
            }
        }
        if (((flags) as i32) & 30i32) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(((1i32).wrapping_add(arrId)) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        if ((((&raw mut lost).cast::<u32>()).wrapping_offset(1)).read()) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(((1i32).wrapping_add(arrId)) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                4,
                4,
                (3u16) as i32,
            );
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 3i32) {
                    break 'l3;
                }
                'l4: {
                    if ((&raw mut trainerIds).cast::<i32>()).read() == 1023i32 {
                        (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset((((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize))
                        .write(CreateMonIcon(
                            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1816))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((&raw mut tournamentIds).cast::<i32>()).read()) as isize * 6,
                            ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read(),
                            Some(SpriteCB_MonIconDomeInfo),
                            ((x | ((((((&raw const sLeftTrainerMonX).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)) as i16),
                            (((y).wrapping_add(
                                ((((((&raw const sLeftTrainerMonY).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32),
                            )) as i16),
                            0u8,
                            0u32,
                            1u32,
                        ));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            (0u16) as i32,
                        );
                    } else {
                        if ((&raw mut trainerIds).cast::<i32>()).read() == 1022i32 {
                            (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                            ))
                            .write(CreateMonIcon(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1816))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((&raw mut tournamentIds).cast::<i32>()).read()) as isize * 6,
                                ))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read(),
                                Some(SpriteCB_MonIconDomeInfo),
                                ((x | ((((((&raw const sLeftTrainerMonX).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)) as i16),
                                (((y).wrapping_add(
                                    ((((((&raw const sLeftTrainerMonY).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32),
                                )) as i16),
                                0u8,
                                0u32,
                                1u32,
                            ));
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                2,
                                2,
                                (0u16) as i32,
                            );
                        } else {
                            (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                            ))
                            .write(CreateMonIcon(
                                (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                    .wrapping_offset(
                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1816))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((&raw mut tournamentIds).cast::<i32>()).read())
                                                as isize
                                                * 6,
                                        ))
                                        .cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                .cast::<u16>())
                                .read(),
                                Some(SpriteCB_MonIconDomeInfo),
                                ((x | ((((((&raw const sLeftTrainerMonX).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)) as i16),
                                (((y).wrapping_add(
                                    ((((((&raw const sLeftTrainerMonY).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32),
                                )) as i16),
                                0u8,
                                0u32,
                                1u32,
                            ));
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                2,
                                2,
                                (0u16) as i32,
                            );
                        }
                    }
                    if (((flags) as i32) & 30i32) != 0 {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                    }
                    if (((&raw mut lost).cast::<u32>()).read()) != 0 {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            4,
                            4,
                            (3u16) as i32,
                        );
                        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((2i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .write(1i16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 3i32) {
                    break 'l5;
                }
                'l6: {
                    if (((&raw mut trainerIds).cast::<i32>()).wrapping_offset(1)).read() == 1023i32
                    {
                        (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset((((5i32).wrapping_add(i)).wrapping_add(arrId)) as isize))
                        .write(CreateMonIcon(
                            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1816))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut tournamentIds).cast::<i32>()).wrapping_offset(1))
                                    .read()) as isize
                                    * 6,
                            ))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read(),
                            Some(SpriteCB_MonIconDomeInfo),
                            ((x | ((((((&raw const sRightTrainerMonX).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)) as i16),
                            (((y).wrapping_add(
                                ((((((&raw const sRightTrainerMonY).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32),
                            )) as i16),
                            0u8,
                            0u32,
                            1u32,
                        ));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((5i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            (0u16) as i32,
                        );
                    } else {
                        if (((&raw mut trainerIds).cast::<i32>()).wrapping_offset(1)).read()
                            == 1022i32
                        {
                            (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((5i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                            ))
                            .write(CreateMonIcon(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1816))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut tournamentIds).cast::<i32>()).wrapping_offset(1))
                                        .read()) as isize
                                        * 6,
                                ))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read(),
                                Some(SpriteCB_MonIconDomeInfo),
                                ((x | ((((((&raw const sRightTrainerMonX)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)) as i16),
                                (((y).wrapping_add(
                                    ((((((&raw const sRightTrainerMonY).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32),
                                )) as i16),
                                0u8,
                                0u32,
                                1u32,
                            ));
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((5i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                2,
                                2,
                                (0u16) as i32,
                            );
                        } else {
                            (((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((5i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                            ))
                            .write(CreateMonIcon(
                                (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                    .wrapping_offset(
                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1816))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut tournamentIds).cast::<i32>())
                                                .wrapping_offset(1))
                                            .read())
                                                as isize
                                                * 6,
                                        ))
                                        .cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                .cast::<u16>())
                                .read(),
                                Some(SpriteCB_MonIconDomeInfo),
                                ((x | ((((((&raw const sRightTrainerMonX)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)) as i16),
                                (((y).wrapping_add(
                                    ((((((&raw const sRightTrainerMonY).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32),
                                )) as i16),
                                0u8,
                                0u32,
                                1u32,
                            ));
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((5i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                2,
                                2,
                                (0u16) as i32,
                            );
                        }
                    }
                    if (((flags) as i32) & 30i32) != 0 {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((5i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                    }
                    if ((((&raw mut lost).cast::<u32>()).wrapping_offset(1)).read()) != 0 {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((5i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            4,
                            4,
                            (3u16) as i32,
                        );
                        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((5i32).wrapping_add(i)).wrapping_add(arrId)) as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .write(1i16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(6)).write(0u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).write(2u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8))
            .write((((&raw mut textPrinter).cast::<u8>()).wrapping_add(6)).read());
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9))
            .write((((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).read());
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(11)).write(0u8);
        crate::c::bf_write(
            ((&raw mut textPrinter).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut textPrinter).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (14u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut textPrinter).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut textPrinter).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (13u8) as i32,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            ((((&raw const sBattleDomeWinTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset((winStringId) as isize))
            .read(),
        );
        (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>())
            .write((&raw mut gStringVar4).cast::<u8>());
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(4))
            .write((((windowId).wrapping_add(8i32)) as u8));
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(5)).write(1u8);
        PutWindowTilemap((((windowId).wrapping_add(8i32)) as u8));
        CopyWindowToVram((((windowId).wrapping_add(8i32)) as u8), 3u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8)).write(0u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9)).write({
            let __v1 = 0u8;
            (((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).write(__v1);
            __v1
        });
        AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
        if ((&raw mut trainerIds).cast::<i32>()).read() == 1023i32 {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
        } else {
            if ((&raw mut trainerIds).cast::<i32>()).read() == 1022i32 {
                CopyDomeBrainTrainerName((&raw mut gStringVar1).cast::<u8>());
            } else {
                CopyDomeTrainerName(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((&raw mut trainerIds).cast::<i32>()).read()) as u16),
                );
            }
        }
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(5)).write(2u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(10)).write(2u8);
        (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>())
            .write((&raw mut gStringVar1).cast::<u8>());
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(4))
            .write((((windowId).wrapping_add(6i32)) as u8));
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8)).write(
            ((GetStringCenterAlignXOffsetWithLetterSpacing(
                (((((&raw mut textPrinter).cast::<u8>()).wrapping_add(5)).read()) as i32),
                (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).read(),
                64i32,
                (((((&raw mut textPrinter).cast::<u8>()).wrapping_add(10)).read()) as i32),
            )) as u8),
        );
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9)).write({
            let __v2 = 2u8;
            (((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).write(__v2);
            __v2
        });
        PutWindowTilemap((((windowId).wrapping_add(6i32)) as u8));
        CopyWindowToVram((((windowId).wrapping_add(6i32)) as u8), 3u8);
        AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
        if (((&raw mut trainerIds).cast::<i32>()).wrapping_offset(1)).read() == 1023i32 {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
        } else {
            if (((&raw mut trainerIds).cast::<i32>()).wrapping_offset(1)).read() == 1022i32 {
                CopyDomeBrainTrainerName((&raw mut gStringVar1).cast::<u8>());
            } else {
                CopyDomeTrainerName(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((((&raw mut trainerIds).cast::<i32>()).wrapping_offset(1)).read()) as u16),
                );
            }
        }
        (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>())
            .write((&raw mut gStringVar1).cast::<u8>());
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(4))
            .write((((windowId).wrapping_add(7i32)) as u8));
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8)).write(
            ((GetStringCenterAlignXOffsetWithLetterSpacing(
                (((((&raw mut textPrinter).cast::<u8>()).wrapping_add(5)).read()) as i32),
                (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).read(),
                64i32,
                (((((&raw mut textPrinter).cast::<u8>()).wrapping_add(10)).read()) as i32),
            )) as u8),
        );
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9)).write({
            let __v3 = 2u8;
            (((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).write(__v3);
            __v3
        });
        PutWindowTilemap((((windowId).wrapping_add(7i32)) as u8));
        CopyWindowToVram((((windowId).wrapping_add(7i32)) as u8), 3u8);
        AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).write(
            ((((&raw const sBattleDomeMatchNumberTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((matchNo) as i32) as isize))
            .read(),
        );
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(4))
            .write((((windowId).wrapping_add(5i32)) as u8));
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8)).write(
            ((GetStringCenterAlignXOffsetWithLetterSpacing(
                (((((&raw mut textPrinter).cast::<u8>()).wrapping_add(5)).read()) as i32),
                (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).read(),
                160i32,
                (((((&raw mut textPrinter).cast::<u8>()).wrapping_add(10)).read()) as i32),
            )) as u8),
        );
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9)).write({
            let __v4 = 2u8;
            (((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).write(__v4);
            __v4
        });
        PutWindowTilemap((((windowId).wrapping_add(5i32)) as u8));
        CopyWindowToVram((((windowId).wrapping_add(5i32)) as u8), 3u8);
        AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
    }
}
pub(crate) unsafe extern "C" fn ShowDomeTourneyTree() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_ShowTourneyTree), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(2i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        SetMainCallback2(Some(CB2_TourneyTree));
    }
}
pub(crate) unsafe extern "C" fn ShowPreviousDomeTourneyTree() {
    unsafe {
        let mut taskId: u8 = 0u8;
        SetFacilityTrainerAndMonPtrs();
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1726))
            .read()) as i32)
                .wrapping_sub(1i32)) as u8) as i32,
        );
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .write(3u16);
        taskId = CreateTask(Some(Task_ShowTourneyTree), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(2i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(1i16);
        SetMainCallback2(Some(CB2_TourneyTree));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleTourneyTreeInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut newTaskId: u8 = 0u8;
        let mut spriteId: i32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32);
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset((spriteId) as isize * 68),
                        1u8,
                    );
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
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
                'l2: {
                    let __sw2 = ((UpdateTourneyTreeCursor(taskId)) as i32);
                    let __matched =
                        __sw2 == 0i32 || __sw2 == 1i32 || __sw2 == 2i32 || __sw2 == 3i32;
                    if __sw2 == 0i32 || !__matched {
                        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(7i16);
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(3i16);
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(5i16);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    FreeAllWindowBuffers();
                    ScanlineEffect_Stop();
                    {
                        Free(((&raw mut sTilemapBuffer).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sTilemapBuffer).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                    newTaskId = CreateTask(Some(Task_ShowTourneyInfoCard), 0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((newTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((newTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(
                        ((((((&raw const sTourneyTreeTrainerIds).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((spriteId) as isize))
                        .read()) as i16),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((newTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(1i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((newTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(((taskId) as i16));
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(4i16);
                    ((((&raw mut sInfoCard).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    FreeAllWindowBuffers();
                    ScanlineEffect_Stop();
                    {
                        Free(((&raw mut sTilemapBuffer).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sTilemapBuffer).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                    newTaskId = CreateTask(Some(Task_ShowTourneyInfoCard), 0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((newTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((newTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write((((spriteId).wrapping_sub(16i32)) as i16));
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((newTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(2i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((newTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(((taskId) as i16));
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(6i16);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    FreeAllWindowBuffers();
                    ScanlineEffect_Stop();
                    {
                        Free(((&raw mut sTilemapBuffer).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sTilemapBuffer).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                    DestroyTask(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(7))
                        .read()) as u8),
                    );
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateTourneyTreeCursor(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut selection: u8 = 1u8;
        let mut direction: i32 = 4i32;
        let mut tourneyTreeCursorSpriteId: i32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32);
        let mut roundId: i32 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1638)
        .cast::<u16>())
        .read()) as i32);
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            == 2i32)
            || ((((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0)
                && (tourneyTreeCursorSpriteId == 31i32))
        {
            PlaySE(5u16);
            selection = 0u8;
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                if tourneyTreeCursorSpriteId < 16i32 {
                    PlaySE(5u16);
                    selection = 2u8;
                } else {
                    PlaySE(5u16);
                    selection = 3u8;
                }
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    == 64i32)
                    && ((((((((((&raw const sTourneyTreeCursorMovementMap)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((tourneyTreeCursorSpriteId) as isize * 20))
                    .cast::<u8>())
                    .wrapping_offset((roundId) as isize * 4))
                    .cast::<u8>())
                    .read()) as i32)
                        != 255i32)
                {
                    direction = 0i32;
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        == 128i32)
                        && (((((((((((&raw const sTourneyTreeCursorMovementMap)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((tourneyTreeCursorSpriteId) as isize * 20))
                        .cast::<u8>())
                        .wrapping_offset((roundId) as isize * 4))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            != 255i32)
                    {
                        direction = 1i32;
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            == 32i32)
                            && (((((((((((&raw const sTourneyTreeCursorMovementMap)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset((tourneyTreeCursorSpriteId) as isize * 20))
                            .cast::<u8>())
                            .wrapping_offset((roundId) as isize * 4))
                            .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32)
                                != 255i32)
                        {
                            direction = 2i32;
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                == 16i32)
                                && (((((((((((&raw const sTourneyTreeCursorMovementMap)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((tourneyTreeCursorSpriteId) as isize * 20))
                                .cast::<u8>())
                                .wrapping_offset((roundId) as isize * 4))
                                .cast::<u8>())
                                .wrapping_offset(3))
                                .read()) as i32)
                                    != 255i32)
                            {
                                direction = 3i32;
                            }
                        }
                    }
                }
            }
        }
        if direction != 4i32 {
            PlaySE(5u16);
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset((tourneyTreeCursorSpriteId) as isize * 68),
                0u8,
            );
            tourneyTreeCursorSpriteId = ((((((((((&raw const sTourneyTreeCursorMovementMap)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((tourneyTreeCursorSpriteId) as isize * 20))
            .cast::<u8>())
            .wrapping_offset((roundId) as isize * 4))
            .cast::<u8>())
            .wrapping_offset((direction) as isize))
            .read()) as i32);
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset((tourneyTreeCursorSpriteId) as isize * 68),
                1u8,
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((tourneyTreeCursorSpriteId) as i16));
        }
        return selection;
    }
}
pub(crate) unsafe extern "C" fn ShowNonInteractiveDomeTourneyTree() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_ShowTourneyTree), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(1i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(2i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        SetMainCallback2(Some(CB2_TourneyTree));
    }
}
pub(crate) unsafe extern "C" fn ResolveDomeRoundWinners() {
    unsafe {
        let mut i: i32 = 0i32;
        if ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) == 1i32 {
            crate::c::bf_write(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1752))
                .cast::<u8>())
                .wrapping_offset(
                    (TrainerIdToTournamentId(
                        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                    )) as isize
                        * 4,
                ))
                .wrapping_add(1),
                2,
                1,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1752))
                .cast::<u8>())
                .wrapping_offset(
                    (TrainerIdToTournamentId(
                        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                    )) as isize
                        * 4,
                ))
                .wrapping_add(1),
                3,
                2,
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32,
            );
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2164))
            .cast::<u16>())
            .wrapping_offset(
                (TrainerIdToTournamentId(
                    ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                )) as isize,
            ))
            .write(
                (((&raw mut gBattleResults).cast::<u8>())
                    .wrapping_add(34)
                    .cast::<u16>())
                .read(),
            );
            if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1638)
                .cast::<u16>())
            .read()) as i32)
                < 3i32
            {
                DecideRoundWinners(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1638)
                        .cast::<u16>())
                    .read()) as u8),
                );
            }
        } else {
            crate::c::bf_write(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1752))
                .cast::<u8>())
                .wrapping_offset((TrainerIdToTournamentId(1023u16)) as isize * 4))
                .wrapping_add(1),
                2,
                1,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1752))
                .cast::<u8>())
                .wrapping_offset((TrainerIdToTournamentId(1023u16)) as isize * 4))
                .wrapping_add(1),
                3,
                2,
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32,
            );
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2164))
            .cast::<u16>())
            .wrapping_offset((TrainerIdToTournamentId(1023u16)) as isize))
            .write(
                (((&raw mut gBattleResults).cast::<u8>())
                    .wrapping_add(36)
                    .cast::<u16>())
                .read(),
            );
            if (((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32) == 9i32)
                || (((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) == 9i32)
            {
                crate::c::bf_write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1752))
                    .cast::<u8>())
                    .wrapping_offset((TrainerIdToTournamentId(1023u16)) as isize * 4))
                    .wrapping_add(1),
                    5,
                    3,
                    (1u16) as i32,
                );
            }
            {
                i = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32);
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        DecideRoundWinners(((i) as u8));
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetWinningMove(
    winnerTournamentId: i32,
    loserTournamentId: i32,
    roundId: u8,
) -> u16 {
    unsafe {
        let mut winnerTournamentId = winnerTournamentId;
        let mut loserTournamentId = loserTournamentId;
        let mut roundId = roundId;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut moveScores = crate::ffi::Align4([0u8; 48]);
        let mut moves = crate::ffi::Align4([0u8; 24]);
        let mut bestScore: u16 = 0u16;
        let mut bestId: u16 = 0u16;
        let mut movePower: i32 = 0i32;
        SetFacilityPtrsGetLevel();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
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
                                (((&raw mut moveScores).cast::<i32>()).wrapping_offset(
                                    (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                ))
                                .write(0i32);
                                if ((crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1752))
                                    .cast::<u8>())
                                    .wrapping_offset((winnerTournamentId) as isize * 4))
                                    .wrapping_add(0),
                                    0,
                                    10,
                                    false,
                                ) as u16) as i32)
                                    == 1022i32
                                {
                                    (((&raw mut moves).cast::<u16>()).wrapping_offset(
                                        (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                    ))
                                    .write(GetFrontierBrainMonMove(((i) as u8), ((j) as u8)));
                                } else {
                                    (((&raw mut moves).cast::<u16>()).wrapping_offset(
                                        (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                    ))
                                    .write(
                                        (((((((&raw mut gFacilityTrainerMons)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(
                                            (((((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1816))
                                            .cast::<u8>())
                                            .wrapping_offset((winnerTournamentId) as isize * 6))
                                            .cast::<u16>())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 16,
                                        ))
                                        .wrapping_add(2))
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .read(),
                                    );
                                }
                                movePower = ((((((&raw mut gBattleMoves).cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut moves).cast::<u16>()).wrapping_offset(
                                            (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                        ))
                                        .read()) as i32)
                                            as isize
                                            * 12,
                                    ))
                                .wrapping_add(1))
                                .read()) as i32);
                                if movePower == 0i32 {
                                    movePower = 40i32;
                                } else {
                                    if movePower == 1i32 {
                                        movePower = 60i32;
                                    } else {
                                        if ((((((&raw mut moves).cast::<u16>()).wrapping_offset(
                                            (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                        ))
                                        .read())
                                            as i32)
                                            == 120i32)
                                            || ((((((&raw mut moves).cast::<u16>())
                                                .wrapping_offset(
                                                    (((i).wrapping_mul(4i32)).wrapping_add(j))
                                                        as isize,
                                                ))
                                            .read())
                                                as i32)
                                                == 153i32)
                                        {
                                            movePower = crate::c::div_i32(movePower, 2i32);
                                        }
                                    }
                                }
                                {
                                    k = 0i32;
                                    'l5: loop {
                                        if !(k < 3i32) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            let mut var: u32 = 0u32;
                                            let mut targetSpecies: u16 = 0u16;
                                            let mut targetAbility: u16 = 0u16;
                                            'l7: loop {
                                                'l8: {
                                                    var = ((((Random()) as i32)
                                                        | (((Random()) as i32) << 16))
                                                        as u32);
                                                }
                                                if !((((((((&raw mut gFacilityTrainerMons)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_offset(
                                                    (((((((((((&raw mut gSaveBlock2Ptr)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(1612))
                                                    .wrapping_add(1816))
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        (loserTournamentId) as isize * 6,
                                                    ))
                                                    .cast::<u16>())
                                                    .wrapping_offset((k) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 16,
                                                ))
                                                .wrapping_add(12))
                                                .read())
                                                    as i32)
                                                    != ((GetNatureFromPersonality(var)) as i32))
                                                {
                                                    break 'l7;
                                                }
                                            }
                                            targetSpecies = (((((&raw mut gFacilityTrainerMons)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset(
                                                (((((((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1816))
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    (loserTournamentId) as isize * 6,
                                                ))
                                                .cast::<u16>())
                                                .wrapping_offset((k) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 16,
                                            ))
                                            .cast::<u16>())
                                            .read();
                                            if (var & 1u32) != 0 {
                                                targetAbility = ((((((((&raw mut gSpeciesInfo)
                                                    .cast::<u8>())
                                                .wrapping_offset(
                                                    ((targetSpecies) as i32) as isize * 28,
                                                ))
                                                .wrapping_add(22))
                                                .cast::<u8>())
                                                .wrapping_offset(1))
                                                .read())
                                                    as u16);
                                            } else {
                                                targetAbility = (((((((&raw mut gSpeciesInfo)
                                                    .cast::<u8>())
                                                .wrapping_offset(
                                                    ((targetSpecies) as i32) as isize * 28,
                                                ))
                                                .wrapping_add(22))
                                                .cast::<u8>())
                                                .read())
                                                    as u16);
                                            }
                                            var = ((AI_TypeCalc(
                                                (((&raw mut moves).cast::<u16>()).wrapping_offset(
                                                    (((i).wrapping_mul(4i32)).wrapping_add(j))
                                                        as isize,
                                                ))
                                                .read(),
                                                targetSpecies,
                                                ((targetAbility) as u8),
                                            ))
                                                as u32);
                                            if ((var & 4u32) != 0) && ((var & 2u32) != 0) {
                                                let __p1 = ((&raw mut moveScores).cast::<i32>())
                                                    .wrapping_offset(
                                                        (((i).wrapping_mul(4i32)).wrapping_add(j))
                                                            as isize,
                                                    );
                                                (__p1)
                                                    .write(((__p1).read()).wrapping_add(movePower));
                                            } else {
                                                if (var & 41u32) != 0 {
                                                    let __p2 = ((&raw mut moveScores)
                                                        .cast::<i32>())
                                                    .wrapping_offset(
                                                        (((i).wrapping_mul(4i32)).wrapping_add(j))
                                                            as isize,
                                                    );
                                                    (__p2)
                                                        .write(((__p2).read()).wrapping_add(0i32));
                                                } else {
                                                    if (var & 2u32) != 0 {
                                                        let __p3 = ((&raw mut moveScores)
                                                            .cast::<i32>())
                                                        .wrapping_offset(
                                                            (((i).wrapping_mul(4i32))
                                                                .wrapping_add(j))
                                                                as isize,
                                                        );
                                                        (__p3).write(((__p3).read()).wrapping_add(
                                                            (movePower).wrapping_mul(2i32),
                                                        ));
                                                    } else {
                                                        if (var & 4u32) != 0 {
                                                            let __p4 = ((&raw mut moveScores)
                                                                .cast::<i32>())
                                                            .wrapping_offset(
                                                                (((i).wrapping_mul(4i32))
                                                                    .wrapping_add(j))
                                                                    as isize,
                                                            );
                                                            (__p4).write(
                                                                ((__p4).read()).wrapping_add(
                                                                    crate::c::div_i32(
                                                                        movePower, 2i32,
                                                                    ),
                                                                ),
                                                            );
                                                        } else {
                                                            let __p5 = ((&raw mut moveScores)
                                                                .cast::<i32>())
                                                            .wrapping_offset(
                                                                (((i).wrapping_mul(4i32))
                                                                    .wrapping_add(j))
                                                                    as isize,
                                                            );
                                                            (__p5).write(
                                                                ((__p5).read())
                                                                    .wrapping_add(movePower),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        k = (k).wrapping_add(1);
                                    }
                                }
                                if ((bestScore) as i32)
                                    < (((&raw mut moveScores).cast::<i32>()).wrapping_offset(
                                        (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                    ))
                                    .read()
                                {
                                    bestId = ((((i).wrapping_mul(4i32)).wrapping_add(j)) as u16);
                                    bestScore =
                                        (((((&raw mut moveScores).cast::<i32>()).wrapping_offset(
                                            (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                        ))
                                        .read()) as u16);
                                } else {
                                    if ((bestScore) as i32)
                                        == (((&raw mut moveScores).cast::<i32>()).wrapping_offset(
                                            (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                        ))
                                        .read()
                                    {
                                        if (((((&raw mut moves).cast::<u16>())
                                            .wrapping_offset(((bestId) as i32) as isize))
                                        .read()) as i32)
                                            < (((((&raw mut moves).cast::<u16>()).wrapping_offset(
                                                (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                            ))
                                            .read())
                                                as i32)
                                        {
                                            bestId =
                                                ((((i).wrapping_mul(4i32)).wrapping_add(j)) as u16);
                                        }
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        j = ((bestId) as i32);
        'l9: loop {
            'l10: {
                {
                    i = 0i32;
                    'l11: loop {
                        if !(i < ((roundId) as i32).wrapping_sub(1i32)) {
                            break 'l11;
                        }
                        'l12: {
                            if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2164))
                            .cast::<u16>())
                            .wrapping_offset(
                                ((GetOpposingNPCTournamentIdByRound(
                                    ((winnerTournamentId) as u8),
                                    ((i) as u8),
                                )) as i32) as isize,
                            ))
                            .read()) as i32)
                                == (((((&raw mut moves).cast::<u16>())
                                    .wrapping_offset((j) as isize))
                                .read()) as i32)
                            {
                                break 'l11;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i != ((roundId) as i32).wrapping_sub(1i32) {
                    (((&raw mut moveScores).cast::<i32>()).wrapping_offset((j) as isize))
                        .write(0i32);
                    bestScore = 0u16;
                    j = 0i32;
                    {
                        k = 0i32;
                        'l13: loop {
                            if !(k < 12i32) {
                                break 'l13;
                            }
                            'l14: {
                                j = (j).wrapping_add(
                                    (((&raw mut moveScores).cast::<i32>())
                                        .wrapping_offset((k) as isize))
                                    .read(),
                                );
                            }
                            k = (k).wrapping_add(1);
                        }
                    }
                    if j == 0i32 {
                        break 'l9;
                    }
                    j = 0i32;
                    {
                        k = 0i32;
                        'l15: loop {
                            if !(k < 12i32) {
                                break 'l15;
                            }
                            'l16: {
                                if ((bestScore) as i32)
                                    < (((&raw mut moveScores).cast::<i32>())
                                        .wrapping_offset((k) as isize))
                                    .read()
                                {
                                    j = k;
                                    bestScore = (((((&raw mut moveScores).cast::<i32>())
                                        .wrapping_offset((k) as isize))
                                    .read())
                                        as u16);
                                } else {
                                    if (((bestScore) as i32)
                                        == (((&raw mut moveScores).cast::<i32>())
                                            .wrapping_offset((k) as isize))
                                        .read())
                                        && ((((((&raw mut moves).cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                        .read())
                                            as i32)
                                            < (((((&raw mut moves).cast::<u16>())
                                                .wrapping_offset((k) as isize))
                                            .read())
                                                as i32))
                                    {
                                        j = k;
                                        bestScore = (((((&raw mut moveScores).cast::<i32>())
                                            .wrapping_offset((k) as isize))
                                        .read())
                                            as u16);
                                    }
                                }
                            }
                            k = (k).wrapping_add(1);
                        }
                    }
                }
            }
            if !(i != ((roundId) as i32).wrapping_sub(1i32)) {
                break 'l9;
            }
        }
        if (((&raw mut moveScores).cast::<i32>()).wrapping_offset((j) as isize)).read() == 0i32 {
            j = ((bestId) as i32);
        }
        return (((&raw mut moves).cast::<u16>()).wrapping_offset((j) as isize)).read();
    }
}
pub(crate) unsafe extern "C" fn Task_ShowTourneyTree(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut textPrinter = crate::ffi::Align4([0u8; 16]);
        let mut notInteractive: i32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32);
        let mut r4: i32 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32);
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                SetHBlankCallback(None);
                SetVBlankCallback(None);
                EnableInterrupts(3u16);
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
                            'l4: loop {
                                'l5: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((100663296i32) as usize as *mut u8),
                                        ((83886080i32
                                            | (crate::c::div_i32(
                                                98304i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sTourneyTreeBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(16u32, 4u32)) as u8),
                );
                InitWindows(
                    ((&raw const sTourneyTreeWindowTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                DeactivateAllTextPrinters();
                ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                ChangeBgX(2u8, 0i32, 0u8);
                ChangeBgY(2u8, 0i32, 0u8);
                ChangeBgX(3u8, 0i32, 0u8);
                ChangeBgY(3u8, 2816i32, 0u8);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                SetGpuReg(76u8, 0u16);
                SetGpuReg(64u8, 22624u16);
                SetGpuReg(68u8, 159u16);
                SetGpuReg(66u8, 37016u16);
                SetGpuReg(70u8, 159u16);
                SetGpuReg(72u8, 0u16);
                SetGpuReg(74u8, 63u16);
                ResetPaletteFade();
                ResetSpriteData();
                FreeAllSpritePalettes();
                let __p3 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut sTilemapBuffer).cast::<u8>().cast::<*mut u8>())
                    .write(AllocZeroed(2048u32));
                LZDecompressWram(
                    ((&raw mut gDomeTourneyTree_Tilemap).cast::<u32>()).cast::<u32>(),
                    ((&raw mut sTilemapBuffer).cast::<u8>().cast::<*mut u8>()).read(),
                );
                SetBgTilemapBuffer(
                    1u8,
                    ((&raw mut sTilemapBuffer).cast::<u8>().cast::<*mut u8>()).read(),
                );
                CopyBgTilemapBufferToVram(1u8);
                DecompressAndLoadBgGfxUsingHeap(
                    1u8,
                    (((&raw mut gDomeTourneyTree_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    8192u32,
                    0u16,
                    0u8,
                );
                DecompressAndLoadBgGfxUsingHeap(
                    2u8,
                    (((&raw mut gDomeTourneyLine_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    8192u32,
                    0u16,
                    0u8,
                );
                DecompressAndLoadBgGfxUsingHeap(
                    2u8,
                    (((&raw mut gDomeTourneyLineDown_Tilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    8192u32,
                    0u16,
                    1u8,
                );
                DecompressAndLoadBgGfxUsingHeap(
                    3u8,
                    (((&raw mut gDomeTourneyLineUp_Tilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    8192u32,
                    0u16,
                    1u8,
                );
                LoadCompressedPalette(
                    ((&raw mut gDomeTourneyTree_Pal).cast::<u32>()).cast::<u32>(),
                    0u16,
                    512u16,
                );
                LoadCompressedPalette(
                    ((&raw mut gDomeTourneyTreeButtons_Pal).cast::<u32>()).cast::<u32>(),
                    256u16,
                    512u16,
                );
                LoadCompressedPalette(
                    ((&raw mut gBattleWindowTextPalette).cast::<u32>()).cast::<u32>(),
                    240u16,
                    32u16,
                );
                'l6: loop {
                    'l7: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
                            'l8: loop {
                                'l9: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                            .cast::<u8>(),
                                        ((83886080i32
                                            | (crate::c::div_i32(
                                                1024i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l8;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l6;
                    }
                }
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                let __p4 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                LoadCompressedSpriteSheet(
                    ((&raw const sTourneyTreeButtonsSpriteSheet)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                if notInteractive == 0i32 {
                    {
                        i = 0i32;
                        'l10: loop {
                            if !(((i) as u32) < crate::c::div_u32(62u32, 2u32)) {
                                break 'l10;
                            }
                            'l11: {
                                CreateSprite(
                                    (&raw const sTourneyTreePokeballSpriteTemplate)
                                        .cast::<u8>()
                                        .cast_mut(),
                                    (((((((&raw const sTourneyTreePokeballCoords)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 2))
                                    .cast::<u8>())
                                    .read()) as i16),
                                    ((((((((&raw const sTourneyTreePokeballCoords)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 2))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i16),
                                    0u8,
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read())
                        != 0
                    {
                        CreateSprite(
                            (&raw const sExitButtonSpriteTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                            218i16,
                            12i16,
                            0u8,
                        );
                    } else {
                        CreateSprite(
                            (&raw const sCancelButtonSpriteTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                            218i16,
                            12i16,
                            0u8,
                        );
                    }
                }
                SetGpuReg(0u8, 32576u16);
                let __p5 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                (((&raw mut textPrinter).cast::<u8>()).wrapping_add(5)).write(2u8);
                (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>())
                    .write((&raw mut gText_BattleTourney).cast::<u8>());
                (((&raw mut textPrinter).cast::<u8>()).wrapping_add(4)).write(2u8);
                (((&raw mut textPrinter).cast::<u8>()).wrapping_add(6)).write(0u8);
                (((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).write(0u8);
                (((&raw mut textPrinter).cast::<u8>()).wrapping_add(10)).write(2u8);
                (((&raw mut textPrinter).cast::<u8>()).wrapping_add(11)).write(0u8);
                (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8)).write(
                    ((GetStringCenterAlignXOffsetWithLetterSpacing(
                        (((((&raw mut textPrinter).cast::<u8>()).wrapping_add(5)).read()) as i32),
                        (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).read(),
                        112i32,
                        (((((&raw mut textPrinter).cast::<u8>()).wrapping_add(10)).read()) as i32),
                    )) as u8),
                );
                (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9)).write(1u8);
                crate::c::bf_write(
                    ((&raw mut textPrinter).cast::<u8>()).wrapping_add(12),
                    0,
                    4,
                    (0u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut textPrinter).cast::<u8>()).wrapping_add(12),
                    4,
                    4,
                    (14u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut textPrinter).cast::<u8>()).wrapping_add(13),
                    0,
                    4,
                    (0u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut textPrinter).cast::<u8>()).wrapping_add(13),
                    4,
                    4,
                    (13u8) as i32,
                );
                AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
                {
                    i = 0i32;
                    'l12: loop {
                        if !(i < 16i32) {
                            break 'l12;
                        }
                        'l13: {
                            let mut roundId: i32 = 0i32;
                            let mut var2: i32 = 0i32;
                            CopyDomeTrainerName(
                                (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                (crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1752))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 4))
                                    .wrapping_add(0),
                                    0,
                                    10,
                                    false,
                                ) as u16),
                            );
                            if notInteractive == 1i32 {
                                if (crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1752))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 4))
                                    .wrapping_add(1),
                                    2,
                                    1,
                                    false,
                                ) as u16)
                                    != 0
                                {
                                    if ((crate::c::bf_read(
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1752))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 4))
                                        .wrapping_add(1),
                                        3,
                                        2,
                                        false,
                                    ) as u16) as i32)
                                        != 0i32
                                    {
                                        var2 = ((crate::c::bf_read(
                                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1752))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 4))
                                            .wrapping_add(1),
                                            3,
                                            2,
                                            false,
                                        ) as u16)
                                            as i32)
                                            .wrapping_sub(1i32);
                                        DrawTourneyAdvancementLine(((i) as u8), ((var2) as u8));
                                    }
                                } else {
                                    if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1638)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        != 1i32
                                    {
                                        DrawTourneyAdvancementLine(
                                            ((i) as u8),
                                            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1638)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                .wrapping_sub(2i32))
                                                as u8),
                                        );
                                    }
                                }
                            } else {
                                if notInteractive == 0i32 {
                                    if (crate::c::bf_read(
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1752))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 4))
                                        .wrapping_add(1),
                                        2,
                                        1,
                                        false,
                                    ) as u16)
                                        != 0
                                    {
                                        if ((crate::c::bf_read(
                                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1752))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 4))
                                            .wrapping_add(1),
                                            3,
                                            2,
                                            false,
                                        ) as u16)
                                            as i32)
                                            != 0i32
                                        {
                                            var2 = ((crate::c::bf_read(
                                                (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1752))
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize * 4))
                                                .wrapping_add(1),
                                                3,
                                                2,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                                .wrapping_sub(1i32);
                                            DrawTourneyAdvancementLine(((i) as u8), ((var2) as u8));
                                        }
                                    } else {
                                        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1638)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            != 0i32
                                        {
                                            if (((((((&raw mut gTasks).cast::<u8>())
                                                .wrapping_offset(
                                                    ((taskId) as i32) as isize * 40,
                                                ))
                                            .wrapping_add(8))
                                            .cast::<i16>())
                                            .wrapping_offset(4))
                                            .read())
                                                != 0
                                            {
                                                var2 = (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1638)
                                                .cast::<u16>())
                                                .read())
                                                    as i32);
                                            } else {
                                                var2 = (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1638)
                                                .cast::<u16>())
                                                .read())
                                                    as i32)
                                                    .wrapping_sub(1i32);
                                            }
                                            DrawTourneyAdvancementLine(((i) as u8), ((var2) as u8));
                                        }
                                    }
                                }
                            }
                            if (((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read())
                                != 0
                            {
                                roundId = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(1638)
                                .cast::<u16>())
                                .read()) as i32);
                            } else {
                                roundId = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(1638)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_sub(1i32);
                            }
                            if (((notInteractive == 1i32)
                                && (((crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1752))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 4))
                                    .wrapping_add(1),
                                    3,
                                    2,
                                    false,
                                ) as u16) as i32)
                                    < (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1638)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        .wrapping_sub(1i32)))
                                || ((notInteractive == 0i32)
                                    && (((crate::c::bf_read(
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1752))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 4))
                                        .wrapping_add(1),
                                        3,
                                        2,
                                        false,
                                    ) as u16) as i32)
                                        <= roundId)))
                                && ((crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1752))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 4))
                                    .wrapping_add(1),
                                    2,
                                    1,
                                    false,
                                ) as u16)
                                    != 0)
                            {
                                if ((crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1752))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 4))
                                    .wrapping_add(0),
                                    0,
                                    10,
                                    false,
                                ) as u16) as i32)
                                    == 1023i32
                                {
                                    crate::c::bf_write(
                                        ((&raw mut textPrinter).cast::<u8>()).wrapping_add(12),
                                        4,
                                        4,
                                        (3u8) as i32,
                                    );
                                    crate::c::bf_write(
                                        ((&raw mut textPrinter).cast::<u8>()).wrapping_add(13),
                                        4,
                                        4,
                                        (4u8) as i32,
                                    );
                                } else {
                                    crate::c::bf_write(
                                        ((&raw mut textPrinter).cast::<u8>()).wrapping_add(12),
                                        4,
                                        4,
                                        (11u8) as i32,
                                    );
                                    crate::c::bf_write(
                                        ((&raw mut textPrinter).cast::<u8>()).wrapping_add(13),
                                        4,
                                        4,
                                        (13u8) as i32,
                                    );
                                }
                            } else {
                                if ((crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1752))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 4))
                                    .wrapping_add(0),
                                    0,
                                    10,
                                    false,
                                ) as u16) as i32)
                                    == 1023i32
                                {
                                    crate::c::bf_write(
                                        ((&raw mut textPrinter).cast::<u8>()).wrapping_add(12),
                                        4,
                                        4,
                                        (3u8) as i32,
                                    );
                                    crate::c::bf_write(
                                        ((&raw mut textPrinter).cast::<u8>()).wrapping_add(13),
                                        4,
                                        4,
                                        (4u8) as i32,
                                    );
                                } else {
                                    crate::c::bf_write(
                                        ((&raw mut textPrinter).cast::<u8>()).wrapping_add(12),
                                        4,
                                        4,
                                        (14u8) as i32,
                                    );
                                    crate::c::bf_write(
                                        ((&raw mut textPrinter).cast::<u8>()).wrapping_add(13),
                                        4,
                                        4,
                                        (13u8) as i32,
                                    );
                                }
                            }
                            if (((((((&raw const sTrainerNamePositions).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 2))
                            .cast::<u8>())
                            .read()) as i32)
                                == 0i32
                            {
                                (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8)).write(
                                    ((GetStringWidthDifference(
                                        (((((&raw mut textPrinter).cast::<u8>()).wrapping_add(5))
                                            .read())
                                            as i32),
                                        (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                        61i32,
                                        (((((&raw mut textPrinter).cast::<u8>()).wrapping_add(10))
                                            .read())
                                            as i32),
                                    )) as u8),
                                );
                            } else {
                                (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8)).write(3u8);
                            }
                            (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>())
                                .write((&raw mut gDisplayedStringBattle).cast::<u8>());
                            (((&raw mut textPrinter).cast::<u8>()).wrapping_add(4)).write(
                                (((((&raw const sTrainerNamePositions).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 2))
                                .cast::<u8>())
                                .read(),
                            );
                            (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9)).write(
                                ((((((&raw const sTrainerNamePositions)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 2))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read(),
                            );
                            AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p6 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                PutWindowTilemap(0u8);
                PutWindowTilemap(1u8);
                PutWindowTilemap(2u8);
                CopyWindowToVram(0u8, 3u8);
                CopyWindowToVram(1u8, 3u8);
                CopyWindowToVram(2u8, 3u8);
                SetHBlankCallback(Some(HblankCb_TourneyTree));
                SetVBlankCallback(Some(VblankCb_TourneyTree));
                if r4 == 2i32 {
                    if notInteractive == 0i32 {
                        i = ((CreateTask(Some(Task_HandleTourneyTreeInput), 0u8)) as i32);
                        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                            .wrapping_add(8))
                        .cast::<i16>())
                        .write(((notInteractive) as i16));
                        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                            .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(((notInteractive) as i16));
                        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                            .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .write(
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read(),
                        );
                    } else {
                        i = ((CreateTask(Some(Task_HandleStaticTourneyTreeInput), 0u8)) as i32);
                        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                            .wrapping_add(8))
                        .cast::<i16>())
                        .write(0i16);
                    }
                } else {
                    i = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32);
                    (((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                        .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                }
                ScanlineEffect_Clear();
                i = 0i32;
                'l14: loop {
                    if !(i < 91i32) {
                        break 'l14;
                    }
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(7946u16);
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(7946u16);
                    i = (i).wrapping_add(1);
                }
                'l15: loop {
                    if !(i < 160i32) {
                        break 'l15;
                    }
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(7945u16);
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(7945u16);
                    i = (i).wrapping_add(1);
                }
                ScanlineEffect_SetParams(
                    (&raw const sTourneyTreeScanlineEffectParams)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<crate::c::Rec4<12>>()
                        .read_unaligned(),
                );
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawTourneyAdvancementLine(tournamentId: u8, roundId: u8) {
    unsafe {
        let mut tournamentId = tournamentId;
        let mut roundId = roundId;
        let mut i: i32 = 0i32;
        let mut lineSection: *mut u8 = ((((((&raw const sTourneyTreeLineSections)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(((tournamentId) as i32) as isize * 16))
        .cast::<*mut u8>())
        .wrapping_offset(((roundId) as i32) as isize))
        .read();
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((((&raw const sTourneyTreeLineSectionArrayCounts)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((tournamentId) as i32) as isize * 4))
                    .cast::<u8>())
                    .wrapping_offset(((roundId) as i32) as isize))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    CopyToBgTilemapBufferRect_ChangePalette(
                        1u8,
                        (((lineSection).wrapping_offset((i) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                        .cast::<u8>(),
                        ((lineSection).wrapping_offset((i) as isize * 4)).read(),
                        (((lineSection).wrapping_offset((i) as isize * 4)).wrapping_add(1)).read(),
                        1u8,
                        1u8,
                        17u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleStaticTourneyTreeInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut textPrinter = crate::ffi::Align4([0u8; 16]);
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
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
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(64i16);
                    (((&raw mut textPrinter).cast::<u8>()).wrapping_add(5)).write(2u8);
                    (((&raw mut textPrinter).cast::<u8>()).wrapping_add(6)).write(0u8);
                    (((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).write(0u8);
                    (((&raw mut textPrinter).cast::<u8>()).wrapping_add(10)).write(2u8);
                    (((&raw mut textPrinter).cast::<u8>()).wrapping_add(11)).write(0u8);
                    crate::c::bf_write(
                        ((&raw mut textPrinter).cast::<u8>()).wrapping_add(12),
                        0,
                        4,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        ((&raw mut textPrinter).cast::<u8>()).wrapping_add(12),
                        4,
                        4,
                        (11u8) as i32,
                    );
                    crate::c::bf_write(
                        ((&raw mut textPrinter).cast::<u8>()).wrapping_add(13),
                        0,
                        4,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        ((&raw mut textPrinter).cast::<u8>()).wrapping_add(13),
                        4,
                        4,
                        (13u8) as i32,
                    );
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 16i32) {
                                break 'l2;
                            }
                            'l3: {
                                CopyDomeTrainerName(
                                    (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                    (crate::c::bf_read(
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1752))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 4))
                                        .wrapping_add(0),
                                        0,
                                        10,
                                        false,
                                    ) as u16),
                                );
                                if (((crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1752))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 4))
                                    .wrapping_add(1),
                                    3,
                                    2,
                                    false,
                                ) as u16) as i32)
                                    == (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1638)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        .wrapping_sub(1i32))
                                    && ((crate::c::bf_read(
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1752))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 4))
                                        .wrapping_add(1),
                                        2,
                                        1,
                                        false,
                                    ) as u16)
                                        != 0)
                                {
                                    if (((((((&raw const sTrainerNamePositions)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 2))
                                    .cast::<u8>())
                                    .read()) as i32)
                                        == 0i32
                                    {
                                        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8))
                                            .write(
                                                ((GetStringWidthDifference(
                                                    (((((&raw mut textPrinter).cast::<u8>())
                                                        .wrapping_add(5))
                                                    .read())
                                                        as i32),
                                                    (&raw mut gDisplayedStringBattle).cast::<u8>(),
                                                    61i32,
                                                    (((((&raw mut textPrinter).cast::<u8>())
                                                        .wrapping_add(10))
                                                    .read())
                                                        as i32),
                                                ))
                                                    as u8),
                                            );
                                    } else {
                                        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8))
                                            .write(3u8);
                                    }
                                    (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>())
                                        .write((&raw mut gDisplayedStringBattle).cast::<u8>());
                                    (((&raw mut textPrinter).cast::<u8>()).wrapping_add(4)).write(
                                        (((((&raw const sTrainerNamePositions)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 2))
                                        .cast::<u8>())
                                        .read(),
                                    );
                                    (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9)).write(
                                        ((((((&raw const sTrainerNamePositions)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 2))
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read(),
                                    );
                                    AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
                                }
                                if !((crate::c::bf_read(
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1752))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 4))
                                    .wrapping_add(1),
                                    2,
                                    1,
                                    false,
                                ) as u16)
                                    != 0)
                                {
                                    let mut roundId: i32 =
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1638)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_sub(1i32);
                                    DrawTourneyAdvancementLine(((i) as u8), ((roundId) as u8));
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    let __t3 = ((__p2).read()).wrapping_sub(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    == 0i32
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
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(4i16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_TourneyTree() {
    unsafe {
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
        RunTasks();
    }
}
pub(crate) unsafe extern "C" fn VblankCb_TourneyInfoCard() {
    unsafe {
        ChangeBgX(3u8, 128i32, 1u8);
        ChangeBgY(3u8, 128i32, 2u8);
        SetGpuReg(16u8, ((&raw mut gBattle_BG0_X).cast::<u16>()).read());
        SetGpuReg(18u8, ((&raw mut gBattle_BG0_Y).cast::<u16>()).read());
        SetGpuReg(20u8, ((&raw mut gBattle_BG1_X).cast::<u16>()).read());
        SetGpuReg(22u8, ((&raw mut gBattle_BG1_Y).cast::<u16>()).read());
        SetGpuReg(24u8, ((&raw mut gBattle_BG2_X).cast::<u16>()).read());
        SetGpuReg(26u8, ((&raw mut gBattle_BG2_Y).cast::<u16>()).read());
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn HblankCb_TourneyTree() {
    unsafe {
        let mut vCount: u16 = ((67108870i32) as usize as *mut u16).read_volatile();
        if ((vCount) as i32) < 42i32 {
            crate::c::volatile_write(((67108936i32) as usize as *mut u16), 16191u16);
            {
                crate::c::volatile_write(((67108928i32) as usize as *mut u32), 0u32);
            }
        } else {
            if ((vCount) as i32) < 50i32 {
                crate::c::volatile_write(((67108936i32) as usize as *mut u16), 15163u16);
                {
                    crate::c::volatile_write(((67108928i32) as usize as *mut u32), 2560316760u32);
                }
            } else {
                if ((vCount) as i32) < 58i32 {
                    crate::c::volatile_write(((67108936i32) as usize as *mut u16), 16191u16);
                    {
                        crate::c::volatile_write(((67108928i32) as usize as *mut u32), 0u32);
                    }
                } else {
                    if ((vCount) as i32) < 75i32 {
                        crate::c::volatile_write(((67108936i32) as usize as *mut u16), 15163u16);
                        {
                            crate::c::volatile_write(
                                ((67108928i32) as usize as *mut u32),
                                2425903200u32,
                            );
                        }
                    } else {
                        if ((vCount) as i32) < 82i32 {
                            crate::c::volatile_write(
                                ((67108936i32) as usize as *mut u16),
                                15163u16,
                            );
                            {
                                crate::c::volatile_write(
                                    ((67108928i32) as usize as *mut u32),
                                    2560316760u32,
                                );
                            }
                        } else {
                            if ((vCount) as i32) < 95i32 {
                                crate::c::volatile_write(
                                    ((67108936i32) as usize as *mut u16),
                                    16191u16,
                                );
                                {
                                    crate::c::volatile_write(
                                        ((67108928i32) as usize as *mut u32),
                                        0u32,
                                    );
                                }
                            } else {
                                if ((vCount) as i32) < 103i32 {
                                    crate::c::volatile_write(
                                        ((67108936i32) as usize as *mut u16),
                                        14135u16,
                                    );
                                    {
                                        crate::c::volatile_write(
                                            ((67108928i32) as usize as *mut u32),
                                            2560316760u32,
                                        );
                                    }
                                } else {
                                    if ((vCount) as i32) < 119i32 {
                                        crate::c::volatile_write(
                                            ((67108936i32) as usize as *mut u16),
                                            14135u16,
                                        );
                                        {
                                            crate::c::volatile_write(
                                                ((67108928i32) as usize as *mut u32),
                                                2425903200u32,
                                            );
                                        }
                                    } else {
                                        if ((vCount) as i32) < 127i32 {
                                            crate::c::volatile_write(
                                                ((67108936i32) as usize as *mut u16),
                                                16191u16,
                                            );
                                            {
                                                crate::c::volatile_write(
                                                    ((67108928i32) as usize as *mut u32),
                                                    0u32,
                                                );
                                            }
                                        } else {
                                            if ((vCount) as i32) < 135i32 {
                                                crate::c::volatile_write(
                                                    ((67108936i32) as usize as *mut u16),
                                                    14135u16,
                                                );
                                                {
                                                    crate::c::volatile_write(
                                                        ((67108928i32) as usize as *mut u32),
                                                        2560316760u32,
                                                    );
                                                }
                                            } else {
                                                crate::c::volatile_write(
                                                    ((67108936i32) as usize as *mut u16),
                                                    16191u16,
                                                );
                                                {
                                                    crate::c::volatile_write(
                                                        ((67108928i32) as usize as *mut u32),
                                                        0u32,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn VblankCb_TourneyTree() {
    unsafe {
        SetGpuReg(16u8, ((&raw mut gBattle_BG0_X).cast::<u16>()).read());
        SetGpuReg(18u8, ((&raw mut gBattle_BG0_Y).cast::<u16>()).read());
        SetGpuReg(20u8, ((&raw mut gBattle_BG1_X).cast::<u16>()).read());
        SetGpuReg(22u8, ((&raw mut gBattle_BG1_Y).cast::<u16>()).read());
        ChangeBgY(2u8, 128i32, 2u8);
        ChangeBgY(3u8, 128i32, 1u8);
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        ScanlineEffect_InitHBlankDmaTransfer();
    }
}
pub(crate) unsafe extern "C" fn SetFacilityTrainerAndMonPtrs() {
    unsafe {
        ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
            .write((&raw mut gBattleFrontierMons).cast::<u8>());
        ((&raw mut gFacilityTrainers).cast::<*mut u8>())
            .write((&raw mut gBattleFrontierTrainers).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn ResetSketchedMoves() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut moveSlot: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 2i32) {
                    break 'l1;
                }
                'l2: {
                    let mut playerMonId: i32 =
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1630))
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((&raw mut gSelectedOrderFromParty).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_sub(1i32);
                    let mut count: i32 = 0i32;
                    {
                        moveSlot = 0i32;
                        'l3: loop {
                            if !(moveSlot < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                count = 0i32;
                                'l5: loop {
                                    if !(count < 4i32) {
                                        break 'l5;
                                    }
                                    if GetMonData3(
                                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                            .wrapping_add(568))
                                        .cast::<u8>())
                                        .wrapping_offset((playerMonId) as isize * 100),
                                        (13i32).wrapping_add(count),
                                        core::ptr::null_mut(),
                                    ) == GetMonData3(
                                        ((&raw mut gPlayerParty).cast::<u8>())
                                            .wrapping_offset((i) as isize * 100),
                                        (13i32).wrapping_add(moveSlot),
                                        core::ptr::null_mut(),
                                    ) {
                                        break 'l5;
                                    }
                                    count = (count).wrapping_add(1);
                                }
                                if count == 4i32 {
                                    SetMonMoveSlot(
                                        ((&raw mut gPlayerParty).cast::<u8>())
                                            .wrapping_offset((i) as isize * 100),
                                        166u16,
                                        ((moveSlot) as u8),
                                    );
                                }
                            }
                            moveSlot = (moveSlot).wrapping_add(1);
                        }
                    }
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(568))
                        .cast::<u8>())
                    .wrapping_offset((playerMonId) as isize * 100)
                    .cast::<crate::c::Rec4<100>>()
                    .write_unaligned(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset((i) as isize * 100)
                            .cast::<crate::c::Rec4<100>>()
                            .read_unaligned(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RestoreDomePlayerPartyHeldItems() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 2i32) {
                    break 'l1;
                }
                'l2: {
                    let mut playerMonId: i32 =
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1630))
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((&raw mut gSelectedOrderFromParty).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_sub(1i32);
                    let mut item: u16 = ((GetMonData3(
                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(568))
                        .cast::<u8>())
                        .wrapping_offset((playerMonId) as isize * 100),
                        12i32,
                        core::ptr::null_mut(),
                    )) as u16);
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        12i32,
                        (&raw mut item).cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReduceDomePlayerPartyToSelectedMons() {
    unsafe {
        ReducePlayerPartyToSelectedMons();
    }
}
pub(crate) unsafe extern "C" fn GetPlayerSeededBeforeOpponent() {
    unsafe {
        if TrainerIdToTournamentId(((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
            > TrainerIdToTournamentId(1023u16)
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
        }
    }
}
pub(crate) unsafe extern "C" fn BufferLastDomeWinnerName() {
    unsafe {
        let mut i: i32 = 0i32;
        SetFacilityTrainerAndMonPtrs();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if !((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(1),
                        2,
                        1,
                        false,
                    ) as u16)
                        != 0)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyDomeTrainerName(
            (&raw mut gStringVar1).cast::<u8>(),
            (crate::c::bf_read(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1752))
                .cast::<u8>())
                .wrapping_offset((i) as isize * 4))
                .wrapping_add(0),
                0,
                10,
                false,
            ) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn InitRandomTourneyTreeResults() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut monLevel: i32 = 0i32;
        let mut species = crate::ffi::Align4([0u8; 12]);
        let mut monTypesBits: i32 = 0i32;
        let mut trainerId: i32 = 0i32;
        let mut monId: i32 = 0i32;
        let mut zero1: i32 = 0i32;
        let mut zero2: i32 = 0i32;
        let mut lvlMode: u8 = 0u8;
        let mut statSums: *mut u16 = core::ptr::null_mut();
        let mut statValues: *mut i32 = core::ptr::null_mut();
        let mut ivs: u8 = 0u8;
        ((&raw mut species).cast::<i32>()).write(0i32);
        (((&raw mut species).cast::<i32>()).wrapping_offset(1)).write(0i32);
        (((&raw mut species).cast::<i32>()).wrapping_offset(2)).write(0i32);
        if ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1726))
        .read()) as i32)
            != (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1727))
            .read()) as i32)
                .wrapping_neg())
            && ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1628))
            .read()) as i32)
                != 1i32)
        {
            return;
        }
        statSums = (AllocZeroed(32u32)).cast::<u16>();
        statValues = (AllocZeroed(24u32)).cast::<i32>();
        lvlMode = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            (0u8) as i32,
        );
        zero1 = 0i32;
        zero2 = 0i32;
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1726))
        .write((((zero1).wrapping_add(1i32)) as u8));
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1727))
        .write((((zero2).wrapping_add(1i32)) as u8));
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    'l3: loop {
                        'l4: {
                            if i < 5i32 {
                                trainerId = crate::c::rem_i32(((Random()) as i32), 10i32);
                            } else {
                                if i < 15i32 {
                                    trainerId = (crate::c::rem_i32(((Random()) as i32), 20i32))
                                        .wrapping_add(10i32);
                                } else {
                                    trainerId = (crate::c::rem_i32(((Random()) as i32), 10i32))
                                        .wrapping_add(30i32);
                                }
                            }
                            {
                                j = 0i32;
                                'l5: loop {
                                    if !(j < i) {
                                        break 'l5;
                                    }
                                    'l6: {
                                        if ((crate::c::bf_read(
                                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1752))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize * 4))
                                            .wrapping_add(0),
                                            0,
                                            10,
                                            false,
                                        ) as u16)
                                            as i32)
                                            == trainerId
                                        {
                                            break 'l5;
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        if !(j != i) {
                            break 'l3;
                        }
                    }
                    crate::c::bf_write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(0),
                        0,
                        10,
                        ((trainerId) as u16) as i32,
                    );
                    {
                        j = 0i32;
                        'l7: loop {
                            if !(j < 3i32) {
                                break 'l7;
                            }
                            'l8: {
                                'l9: loop {
                                    'l10: {
                                        monId = ((GetRandomFrontierMonFromSet(((trainerId) as u16)))
                                            as i32);
                                        {
                                            k = 0i32;
                                            'l11: loop {
                                                if !(k < j) {
                                                    break 'l11;
                                                }
                                                'l12: {
                                                    let mut alreadySelectedMonId: i32 =
                                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(1612))
                                                        .wrapping_add(1816))
                                                        .cast::<u8>())
                                                        .wrapping_offset((i) as isize * 6))
                                                        .cast::<u16>())
                                                        .wrapping_offset((k) as isize))
                                                        .read())
                                                            as i32);
                                                    if (((alreadySelectedMonId == monId) || (((&raw mut species).cast::<i32>()).read() == ((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset((monId) as isize * 16)).cast::<u16>()).read()) as i32)))) || ((((&raw mut species).cast::<i32>()).wrapping_offset(1)).read() == ((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset((monId) as isize * 16)).cast::<u16>()).read()) as i32)))) || (((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset((alreadySelectedMonId) as isize * 16)).wrapping_add(10)).read()) as i32)) == ((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset((monId) as isize * 16)).wrapping_add(10)).read()) as i32))) {
break 'l11;
}
                                                }
                                                k = (k).wrapping_add(1);
                                            }
                                        }
                                    }
                                    if !(k != j) {
                                        break 'l9;
                                    }
                                }
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1816))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 6))
                                .cast::<u16>())
                                .wrapping_offset((j) as isize))
                                .write(((monId) as u16));
                                (((&raw mut species).cast::<i32>()).wrapping_offset((j) as isize))
                                    .write(
                                        (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                            .read())
                                        .wrapping_offset((monId) as isize * 16))
                                        .cast::<u16>())
                                        .read()) as i32),
                                    );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    crate::c::bf_write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(1),
                        2,
                        1,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(1),
                        3,
                        2,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(1),
                        5,
                        3,
                        (0u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        monLevel = 50i32;
        {
            i = 0i32;
            'l13: loop {
                if !(i < 16i32) {
                    break 'l13;
                }
                'l14: {
                    monTypesBits = 0i32;
                    ((statSums).wrapping_offset((i) as isize)).write(0u16);
                    ivs = GetDomeTrainerMonIvs(
                        (crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(0),
                            0,
                            10,
                            false,
                        ) as u16),
                    );
                    {
                        j = 0i32;
                        'l15: loop {
                            if !(j < 3i32) {
                                break 'l15;
                            }
                            'l16: {
                                CalcDomeMonStats(
                                    (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(
                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1816))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .cast::<u16>())
                                    .read(),
                                    monLevel,
                                    ((ivs) as i32),
                                    (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(
                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1816))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(11))
                                    .read(),
                                    (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(
                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1816))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .wrapping_add(12))
                                    .read(),
                                    statValues,
                                );
                                let __p1 = (statSums).wrapping_offset((i) as isize);
                                (__p1).write(
                                    (((((__p1).read()) as i32)
                                        .wrapping_add(((statValues).wrapping_offset(1)).read()))
                                        as u16),
                                );
                                let __p2 = (statSums).wrapping_offset((i) as isize);
                                (__p2).write(
                                    (((((__p2).read()) as i32)
                                        .wrapping_add(((statValues).wrapping_offset(2)).read()))
                                        as u16),
                                );
                                let __p3 = (statSums).wrapping_offset((i) as isize);
                                (__p3).write(
                                    (((((__p3).read()) as i32)
                                        .wrapping_add(((statValues).wrapping_offset(4)).read()))
                                        as u16),
                                );
                                let __p4 = (statSums).wrapping_offset((i) as isize);
                                (__p4).write(
                                    (((((__p4).read()) as i32)
                                        .wrapping_add(((statValues).wrapping_offset(5)).read()))
                                        as u16),
                                );
                                let __p5 = (statSums).wrapping_offset((i) as isize);
                                (__p5).write(
                                    (((((__p5).read()) as i32)
                                        .wrapping_add(((statValues).wrapping_offset(3)).read()))
                                        as u16),
                                );
                                let __p6 = (statSums).wrapping_offset((i) as isize);
                                (__p6).write(
                                    (((((__p6).read()) as i32).wrapping_add((statValues).read()))
                                        as u16),
                                );
                                monTypesBits = ((((monTypesBits) as u32)
                                    | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            (((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut gFacilityTrainerMons)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_offset(
                                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(1612))
                                                        .wrapping_add(1816))
                                                        .cast::<u8>())
                                                        .wrapping_offset((i) as isize * 6))
                                                        .cast::<u16>())
                                                        .wrapping_offset((j) as isize))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 16,
                                                    ))
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 28,
                                                ))
                                            .wrapping_add(6))
                                            .cast::<u8>())
                                            .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read())
                                    as i32);
                                monTypesBits = ((((monTypesBits) as u32)
                                    | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((&raw mut gFacilityTrainerMons)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_offset(
                                                        (((((((((((&raw mut gSaveBlock2Ptr)
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(1612))
                                                        .wrapping_add(1816))
                                                        .cast::<u8>())
                                                        .wrapping_offset((i) as isize * 6))
                                                        .cast::<u16>())
                                                        .wrapping_offset((j) as isize))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 16,
                                                    ))
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 28,
                                                ))
                                            .wrapping_add(6))
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read())
                                    as i32);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    {
                        trainerId = 0i32;
                        j = 0i32;
                        'l17: loop {
                            if !(j < 32i32) {
                                break 'l17;
                            }
                            'l18: {
                                if (monTypesBits & 1i32) != 0 {
                                    trainerId = (trainerId).wrapping_add(1);
                                }
                                monTypesBits = (monTypesBits >> 1);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    let __p7 = (statSums).wrapping_offset((i) as isize);
                    (__p7).write(
                        (((((__p7).read()) as i32).wrapping_add(crate::c::div_i32(
                            (trainerId).wrapping_mul(monLevel),
                            20i32,
                        ))) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l19: loop {
                if !(i < 15i32) {
                    break 'l19;
                }
                'l20: {
                    {
                        j = (i).wrapping_add(1i32);
                        'l21: loop {
                            if !(j < 16i32) {
                                break 'l21;
                            }
                            'l22: {
                                if ((((statSums).wrapping_offset((i) as isize)).read()) as i32)
                                    < ((((statSums).wrapping_offset((j) as isize)).read()) as i32)
                                {
                                    SwapDomeTrainers(i, j, statSums);
                                } else {
                                    if ((((statSums).wrapping_offset((i) as isize)).read()) as i32)
                                        == ((((statSums).wrapping_offset((j) as isize)).read())
                                            as i32)
                                    {
                                        if ((crate::c::bf_read(
                                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1752))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 4))
                                            .wrapping_add(0),
                                            0,
                                            10,
                                            false,
                                        ) as u16)
                                            as i32)
                                            > ((crate::c::bf_read(
                                                (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1752))
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize * 4))
                                                .wrapping_add(0),
                                                0,
                                                10,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                        {
                                            SwapDomeTrainers(i, j, statSums);
                                        }
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        Free((statSums).cast::<u8>());
        Free((statValues).cast::<u8>());
        {
            i = 0i32;
            'l23: loop {
                if !(i < 4i32) {
                    break 'l23;
                }
                'l24: {
                    DecideRoundWinners(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            (lvlMode) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn TrainerIdToTournamentId(trainerId: u16) -> i32 {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(0),
                        0,
                        10,
                        false,
                    ) as u16) as i32)
                        == ((trainerId) as i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return i;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrainerIdToDomeTournamentId(trainerId: u16) -> i32 {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(0),
                        0,
                        10,
                        false,
                    ) as u16) as i32)
                        == ((trainerId) as i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return i;
    }
}
pub(crate) unsafe extern "C" fn GetOpposingNPCTournamentIdByRound(
    tournamentId: u8,
    round: u8,
) -> u8 {
    unsafe {
        let mut tournamentId = tournamentId;
        let mut round = round;
        let mut tournamentIds = crate::ffi::Align4([0u8; 2]);
        BufferDomeWinString(
            ((((((((((&raw const sTrainerAndRoundToLastMatchCardNum)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                (crate::c::div_i32(
                    ((((((&raw const sTournamentIdToPairedTrainerIds)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((tournamentId) as i32) as isize))
                    .read()) as i32),
                    2i32,
                )) as isize
                    * 4,
            ))
            .cast::<u8>())
            .wrapping_offset(((round) as i32) as isize))
            .read()) as i32)
                .wrapping_sub(16i32)) as u8),
            (&raw mut tournamentIds).cast::<u8>(),
        );
        if ((tournamentId) as i32) == ((((&raw mut tournamentIds).cast::<u8>()).read()) as i32) {
            return (((&raw mut tournamentIds).cast::<u8>()).wrapping_offset(1)).read();
        } else {
            return ((&raw mut tournamentIds).cast::<u8>()).read();
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn DecideRoundWinners(roundId: u8) {
    unsafe {
        let mut roundId = roundId;
        let mut i: i32 = 0i32;
        let mut moveSlot: i32 = 0i32;
        let mut monId1: i32 = 0i32;
        let mut monId2: i32 = 0i32;
        let mut tournamentId1: i32 = 0i32;
        let mut tournamentId2: i32 = 0i32;
        let mut species: i32 = 0i32;
        let mut points1: i32 = 0i32;
        let mut points2: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(1),
                        2,
                        1,
                        false,
                    ) as u16)
                        != 0)
                        || (((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(0),
                            0,
                            10,
                            false,
                        ) as u16) as i32)
                            == 1023i32)
                    {
                        break 'l2;
                    }
                    tournamentId1 = i;
                    tournamentId2 = TournamentIdOfOpponent(
                        ((roundId) as i32),
                        ((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset((tournamentId1) as isize * 4))
                            .wrapping_add(0),
                            0,
                            10,
                            false,
                        ) as u16) as i32),
                    );
                    if (((crate::c::bf_read(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1752))
                        .cast::<u8>())
                        .wrapping_offset((tournamentId1) as isize * 4))
                        .wrapping_add(0),
                        0,
                        10,
                        false,
                    ) as u16) as i32)
                        == 1022i32)
                        && (tournamentId2 != 255i32)
                    {
                        crate::c::bf_write(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset((tournamentId2) as isize * 4))
                            .wrapping_add(1),
                            2,
                            1,
                            (1u16) as i32,
                        );
                        crate::c::bf_write(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset((tournamentId2) as isize * 4))
                            .wrapping_add(1),
                            3,
                            2,
                            ((roundId) as u16) as i32,
                        );
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(2164))
                        .cast::<u16>())
                        .wrapping_offset((tournamentId2) as isize))
                        .write(GetWinningMove(tournamentId1, tournamentId2, roundId));
                    } else {
                        if (((crate::c::bf_read(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1752))
                            .cast::<u8>())
                            .wrapping_offset((tournamentId2) as isize * 4))
                            .wrapping_add(0),
                            0,
                            10,
                            false,
                        ) as u16) as i32)
                            == 1022i32)
                            && (tournamentId1 != 255i32)
                        {
                            crate::c::bf_write(
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1752))
                                .cast::<u8>())
                                .wrapping_offset((tournamentId1) as isize * 4))
                                .wrapping_add(1),
                                2,
                                1,
                                (1u16) as i32,
                            );
                            crate::c::bf_write(
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1752))
                                .cast::<u8>())
                                .wrapping_offset((tournamentId1) as isize * 4))
                                .wrapping_add(1),
                                3,
                                2,
                                ((roundId) as u16) as i32,
                            );
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2164))
                            .cast::<u16>())
                            .wrapping_offset((tournamentId1) as isize))
                            .write(GetWinningMove(tournamentId2, tournamentId1, roundId));
                        } else {
                            if tournamentId2 != 255i32 {
                                {
                                    monId1 = 0i32;
                                    'l3: loop {
                                        if !(monId1 < 3i32) {
                                            break 'l3;
                                        }
                                        'l4: {
                                            {
                                                moveSlot = 0i32;
                                                'l5: loop {
                                                    if !(moveSlot < 4i32) {
                                                        break 'l5;
                                                    }
                                                    'l6: {
                                                        {
                                                            monId2 = 0i32;
                                                            'l7: loop {
                                                                if !(monId2 < 3i32) {
                                                                    break 'l7;
                                                                }
                                                                'l8: {
                                                                    points1 = (points1).wrapping_add(GetTypeEffectivenessPoints((((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset(((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612)).wrapping_add(1816)).cast::<u8>()).wrapping_offset((tournamentId1) as isize * 6)).cast::<u16>()).wrapping_offset((monId1) as isize)).read()) as i32)) as isize * 16)).wrapping_add(2)).cast::<u16>()).wrapping_offset((moveSlot) as isize)).read()) as i32), (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset(((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612)).wrapping_add(1816)).cast::<u8>()).wrapping_offset((tournamentId2) as isize * 6)).cast::<u16>()).wrapping_offset((monId2) as isize)).read()) as i32)) as isize * 16)).cast::<u16>()).read()) as i32), 2i32));
                                                                }
                                                                monId2 = (monId2).wrapping_add(1);
                                                            }
                                                        }
                                                    }
                                                    moveSlot = (moveSlot).wrapping_add(1);
                                                }
                                            }
                                            species = (((((((&raw mut gFacilityTrainerMons)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset(
                                                (((((((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1816))
                                                .cast::<u8>())
                                                .wrapping_offset((tournamentId1) as isize * 6))
                                                .cast::<u16>())
                                                .wrapping_offset((monId1) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 16,
                                            ))
                                            .cast::<u16>())
                                            .read())
                                                as i32);
                                            points1 = (points1).wrapping_add(crate::c::div_i32(
                                                (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                    .wrapping_offset((species) as isize * 28))
                                                .read())
                                                    as i32)
                                                    .wrapping_add(
                                                        ((((((&raw mut gSpeciesInfo)
                                                            .cast::<u8>())
                                                        .wrapping_offset((species) as isize * 28))
                                                        .wrapping_add(1))
                                                        .read())
                                                            as i32),
                                                    ))
                                                .wrapping_add(
                                                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                        .wrapping_offset((species) as isize * 28))
                                                    .wrapping_add(2))
                                                    .read())
                                                        as i32),
                                                ))
                                                .wrapping_add(
                                                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                        .wrapping_offset((species) as isize * 28))
                                                    .wrapping_add(3))
                                                    .read())
                                                        as i32),
                                                ))
                                                .wrapping_add(
                                                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                        .wrapping_offset((species) as isize * 28))
                                                    .wrapping_add(4))
                                                    .read())
                                                        as i32),
                                                ))
                                                .wrapping_add(
                                                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                        .wrapping_offset((species) as isize * 28))
                                                    .wrapping_add(5))
                                                    .read())
                                                        as i32),
                                                ),
                                                10i32,
                                            ));
                                        }
                                        monId1 = (monId1).wrapping_add(1);
                                    }
                                }
                                points1 = (points1).wrapping_add((((Random()) as i32) & 31i32));
                                points1 = (points1).wrapping_add(tournamentId1);
                                {
                                    monId1 = 0i32;
                                    'l9: loop {
                                        if !(monId1 < 3i32) {
                                            break 'l9;
                                        }
                                        'l10: {
                                            {
                                                moveSlot = 0i32;
                                                'l11: loop {
                                                    if !(moveSlot < 4i32) {
                                                        break 'l11;
                                                    }
                                                    'l12: {
                                                        {
                                                            monId2 = 0i32;
                                                            'l13: loop {
                                                                if !(monId2 < 3i32) {
                                                                    break 'l13;
                                                                }
                                                                'l14: {
                                                                    points2 = (points2).wrapping_add(GetTypeEffectivenessPoints((((((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset(((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612)).wrapping_add(1816)).cast::<u8>()).wrapping_offset((tournamentId2) as isize * 6)).cast::<u16>()).wrapping_offset((monId1) as isize)).read()) as i32)) as isize * 16)).wrapping_add(2)).cast::<u16>()).wrapping_offset((moveSlot) as isize)).read()) as i32), (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read()).wrapping_offset(((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612)).wrapping_add(1816)).cast::<u8>()).wrapping_offset((tournamentId1) as isize * 6)).cast::<u16>()).wrapping_offset((monId2) as isize)).read()) as i32)) as isize * 16)).cast::<u16>()).read()) as i32), 2i32));
                                                                }
                                                                monId2 = (monId2).wrapping_add(1);
                                                            }
                                                        }
                                                    }
                                                    moveSlot = (moveSlot).wrapping_add(1);
                                                }
                                            }
                                            species = (((((((&raw mut gFacilityTrainerMons)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset(
                                                (((((((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1816))
                                                .cast::<u8>())
                                                .wrapping_offset((tournamentId2) as isize * 6))
                                                .cast::<u16>())
                                                .wrapping_offset((monId1) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 16,
                                            ))
                                            .cast::<u16>())
                                            .read())
                                                as i32);
                                            points2 = (points2).wrapping_add(crate::c::div_i32(
                                                (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                    .wrapping_offset((species) as isize * 28))
                                                .read())
                                                    as i32)
                                                    .wrapping_add(
                                                        ((((((&raw mut gSpeciesInfo)
                                                            .cast::<u8>())
                                                        .wrapping_offset((species) as isize * 28))
                                                        .wrapping_add(1))
                                                        .read())
                                                            as i32),
                                                    ))
                                                .wrapping_add(
                                                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                        .wrapping_offset((species) as isize * 28))
                                                    .wrapping_add(2))
                                                    .read())
                                                        as i32),
                                                ))
                                                .wrapping_add(
                                                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                        .wrapping_offset((species) as isize * 28))
                                                    .wrapping_add(3))
                                                    .read())
                                                        as i32),
                                                ))
                                                .wrapping_add(
                                                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                        .wrapping_offset((species) as isize * 28))
                                                    .wrapping_add(4))
                                                    .read())
                                                        as i32),
                                                ))
                                                .wrapping_add(
                                                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                                                        .wrapping_offset((species) as isize * 28))
                                                    .wrapping_add(5))
                                                    .read())
                                                        as i32),
                                                ),
                                                10i32,
                                            ));
                                        }
                                        monId1 = (monId1).wrapping_add(1);
                                    }
                                }
                                points2 = (points2).wrapping_add((((Random()) as i32) & 31i32));
                                points2 = (points2).wrapping_add(tournamentId2);
                                if points1 > points2 {
                                    crate::c::bf_write(
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1752))
                                        .cast::<u8>())
                                        .wrapping_offset((tournamentId2) as isize * 4))
                                        .wrapping_add(1),
                                        2,
                                        1,
                                        (1u16) as i32,
                                    );
                                    crate::c::bf_write(
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1752))
                                        .cast::<u8>())
                                        .wrapping_offset((tournamentId2) as isize * 4))
                                        .wrapping_add(1),
                                        3,
                                        2,
                                        ((roundId) as u16) as i32,
                                    );
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(2164))
                                    .cast::<u16>())
                                    .wrapping_offset((tournamentId2) as isize))
                                    .write(GetWinningMove(tournamentId1, tournamentId2, roundId));
                                } else {
                                    if points1 < points2 {
                                        crate::c::bf_write(
                                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1752))
                                            .cast::<u8>())
                                            .wrapping_offset((tournamentId1) as isize * 4))
                                            .wrapping_add(1),
                                            2,
                                            1,
                                            (1u16) as i32,
                                        );
                                        crate::c::bf_write(
                                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1752))
                                            .cast::<u8>())
                                            .wrapping_offset((tournamentId1) as isize * 4))
                                            .wrapping_add(1),
                                            3,
                                            2,
                                            ((roundId) as u16) as i32,
                                        );
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(2164))
                                        .cast::<u16>())
                                        .wrapping_offset((tournamentId1) as isize))
                                        .write(GetWinningMove(
                                            tournamentId2,
                                            tournamentId1,
                                            roundId,
                                        ));
                                    } else {
                                        if tournamentId1 > tournamentId2 {
                                            crate::c::bf_write(
                                                (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1752))
                                                .cast::<u8>())
                                                .wrapping_offset((tournamentId2) as isize * 4))
                                                .wrapping_add(1),
                                                2,
                                                1,
                                                (1u16) as i32,
                                            );
                                            crate::c::bf_write(
                                                (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1752))
                                                .cast::<u8>())
                                                .wrapping_offset((tournamentId2) as isize * 4))
                                                .wrapping_add(1),
                                                3,
                                                2,
                                                ((roundId) as u16) as i32,
                                            );
                                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(2164))
                                            .cast::<u16>())
                                            .wrapping_offset((tournamentId2) as isize))
                                            .write(GetWinningMove(
                                                tournamentId1,
                                                tournamentId2,
                                                roundId,
                                            ));
                                        } else {
                                            crate::c::bf_write(
                                                (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1752))
                                                .cast::<u8>())
                                                .wrapping_offset((tournamentId1) as isize * 4))
                                                .wrapping_add(1),
                                                2,
                                                1,
                                                (1u16) as i32,
                                            );
                                            crate::c::bf_write(
                                                (((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(1612))
                                                .wrapping_add(1752))
                                                .cast::<u8>())
                                                .wrapping_offset((tournamentId1) as isize * 4))
                                                .wrapping_add(1),
                                                3,
                                                2,
                                                ((roundId) as u16) as i32,
                                            );
                                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(2164))
                                            .cast::<u16>())
                                            .wrapping_offset((tournamentId1) as isize))
                                            .write(GetWinningMove(
                                                tournamentId2,
                                                tournamentId1,
                                                roundId,
                                            ));
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
    }
}
pub(crate) unsafe extern "C" fn CopyDomeTrainerName(str: *mut u8, trainerId: u16) {
    unsafe {
        let mut str = str;
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        SetFacilityPtrsGetLevel();
        if ((trainerId) as i32) == 1022i32 {
            CopyDomeBrainTrainerName(str);
        } else {
            if ((trainerId) as i32) == 1023i32 {
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i < 7i32) {
                            break 'l1;
                        }
                        'l2: {
                            ((str).wrapping_offset((i) as isize)).write(
                                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                if ((trainerId) as i32) < 300i32 {
                    {
                        i = 0i32;
                        'l3: loop {
                            if !(i < 7i32) {
                                break 'l3;
                            }
                            'l4: {
                                ((str).wrapping_offset((i) as isize)).write(
                                    (((((((&raw mut gFacilityTrainers).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(((trainerId) as i32) as isize * 52))
                                    .wrapping_add(4))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
            }
            ((str).wrapping_offset((i) as isize)).write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn GetDomeBrainTrainerPicId() -> u8 {
    unsafe {
        return ((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(32240)).wrapping_add(3))
            .read();
    }
}
pub(crate) unsafe extern "C" fn GetDomeBrainTrainerClass() -> u8 {
    unsafe {
        return ((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(32240)).wrapping_add(1))
            .read();
    }
}
pub(crate) unsafe extern "C" fn CopyDomeBrainTrainerName(str: *mut u8) {
    unsafe {
        let mut str = str;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 7i32) {
                    break 'l1;
                }
                'l2: {
                    ((str).wrapping_offset((i) as isize)).write(
                        ((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(32240))
                            .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((str).wrapping_offset((i) as isize)).write(255u8);
    }
}
