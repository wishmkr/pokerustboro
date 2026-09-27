//! Translated from `src/contest_util.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sResultsTextWindow_Pal sResultsTextWindow_Gfx sMiscBlank_Pal sOamData_ResultsTextWindow sSpriteTemplate_ResultsTextWindow sSpriteSheets_ResultsTextWindow sSpritePalette_ResultsTextWindow sOamData_Confetti sSpriteTemplate_Confetti sSpriteSheet_Confetti sSpritePalette_Confetti sBgTemplates sWindowTemplates sOamData_WirelessIndicatorWindow sSpriteTemplate_WirelessIndicatorWindow sSpriteSheet_WirelessIndicatorWindow sContestLinkTextColors sContestantLocalIds.0
#[allow(unused_imports)]
use crate::data::contest_util::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sContestResults: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
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
    static mut gContestFinalStandings: u8;
    static mut gContestLinkLeaderIndex: u8;
    static mut gContestMonPartyIndex: u8;
    static mut gContestMonRound1Points: u8;
    static mut gContestMonRound2Points: u8;
    static mut gContestMonTotalPoints: u8;
    static mut gContestMons: u8;
    static mut gContestPlayerMonIndex: u8;
    static mut gContestResultsTitle_Beauty_Tilemap: u8;
    static mut gContestResultsTitle_Cool_Tilemap: u8;
    static mut gContestResultsTitle_Cute_Tilemap: u8;
    static mut gContestResultsTitle_Hyper_Tilemap: u8;
    static mut gContestResultsTitle_Link_Tilemap: u8;
    static mut gContestResultsTitle_Master_Tilemap: u8;
    static mut gContestResultsTitle_Normal_Tilemap: u8;
    static mut gContestResultsTitle_Smart_Tilemap: u8;
    static mut gContestResultsTitle_Super_Tilemap: u8;
    static mut gContestResultsTitle_Tilemap: u8;
    static mut gContestResultsTitle_Tough_Tilemap: u8;
    static mut gContestResults_Bg_Tilemap: u8;
    static mut gContestResults_Gfx: u8;
    static mut gContestResults_Interface_Tilemap: u8;
    static mut gContestResults_Pal: u8;
    static mut gContestResults_WinnerBanner_Tilemap: u8;
    static mut gContestRngValue: u8;
    static mut gCurContestWinnerIsForArtist: u8;
    static mut gCurContestWinnerSaveIdx: u8;
    static mut gDisplayedStringBattle: u8;
    static mut gLinkContestFlags: u8;
    static mut gLinkPlayers: u8;
    static mut gMain: u8;
    static mut gMonFrontPicTable: u8;
    static mut gMonIconPaletteIndices: u8;
    static mut gMonIconPalettes: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gMultiuseSpriteTemplate: u8;
    static mut gNumLinkContestPlayers: u8;
    static mut gObjectEventPal_Brendan: u8;
    static mut gObjectEventPal_May: u8;
    static mut gObjectEventPal_RubySapphireBrendan: u8;
    static mut gObjectEventPal_RubySapphireMay: u8;
    static mut gObjectEvents: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_ContestCategory: u8;
    static mut gSpecialVar_ContestRank: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gTasks: u8;
    static mut gText_AnnouncingResults: u8;
    static mut gText_ColorDarkGray: u8;
    static mut gText_CommunicationStandby: u8;
    static mut gText_ContestantsMonWon: u8;
    static mut gText_PreliminaryResults: u8;
    static mut gText_Round2Results: u8;
    static mut gText_Slash: u8;
    static mut gWirelessStatusIndicatorSpriteId: u8;
    fn AddTextPrinter(a0: *mut u8, a1: u8, a2: Option<unsafe extern "C" fn(*mut u8, u16)>) -> u16;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AllocateMonSpritesGfx();
    fn AnimateSprites();
    fn BeginHardwarePaletteFade(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BravoTrainerPokemonProfile_BeforeInterview2(a0: u8);
    fn BuildOamBuffer();
    fn CB2_ContestPainting();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn CB2_StartContest();
    fn CalculateRound1Points(a0: u8);
    fn ClearContinueGameWarpStatus2();
    fn ClearToTransparentAndRemoveWindow(a0: u8);
    fn ConvertInternationalContestantName(a0: *mut u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateContestMonFromParty(a0: u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWindowFromRect(a0: u8, a1: u8, a2: u8, a3: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DestroyWirelessStatusIndicatorSprite();
    fn DoMonFrontSpriteAnimation(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeMonSpritesGfx();
    fn FreeOamMatrix(a0: u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn GetContestEntryEligibility(a0: *mut u8) -> u8;
    fn GetContestWinnerSaveIdx(a0: u8, a1: u8) -> u8;
    fn GetIconSpecies(a0: u16, a1: u32) -> u16;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonIconPtr(a0: u16, a1: u32, a2: u32) -> *mut u8;
    fn GetMonSpritePalStructFromOtIdPersonality(a0: u16, a1: u32, a2: u32) -> *mut u8;
    fn GetMultiplayerId() -> u8;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetRibbonCount(a0: *mut u8) -> u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetSpritePaletteTagByPaletteNum(a0: u8) -> u16;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn HandleLoadSpecialPokePic_2(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn IncrementGameStat(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn InterviewAfter();
    fn InterviewBefore();
    fn IsLinkTaskFinished() -> u8;
    fn LZDecompressVram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn LockPlayerFieldControls();
    fn PlayBGM(a0: u16);
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn RequestDma3Copy(a0: *mut u8, a1: *mut u8, a2: u16, a3: u8) -> i16;
    fn RequestDma3Fill(a0: i32, a1: *mut u8, a2: u16, a3: u8) -> i16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn SaveContestWinner(a0: u8) -> u8;
    fn SaveLinkContestResults();
    fn ScanlineEffect_Clear();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScriptContext_Enable();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCloseLinkCallback();
    fn SetContestants(a0: u8, a1: u8);
    fn SetContinueGameWarpStatusToDynamicWarp();
    fn SetDynamicWarp(a0: i32, a1: i8, a2: i8, a3: i8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SortContestants(a0: u8);
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
    fn TaskDummy(a0: u8);
    fn Task_LinkContest_CommunicateCategoryRS(a0: u8);
    fn Task_LinkContest_CommunicateLeaderIdsRS(a0: u8);
    fn Task_LinkContest_CommunicateMonIdxs(a0: u8);
    fn Task_LinkContest_CommunicateMonsRS(a0: u8);
    fn Task_LinkContest_CommunicateRngRS(a0: u8);
    fn Task_LinkContest_CommunicateRound1Points(a0: u8);
    fn Task_LinkContest_CommunicateTurnOrder(a0: u8);
    fn Task_LinkContest_Init(a0: u8);
    fn Task_LinkContest_StartCommunicationEm(a0: u8);
    fn TransferPlttBuffer();
    fn TryGainNewFanFromCounter(a0: u8) -> u8;
    fn TryPutSpotTheCutiesOnAir(a0: *mut u8, a1: u8);
    fn TrySavingData(a0: u8) -> u8;
    fn UnlockPlayerFieldControls();
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
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
}

pub(crate) unsafe extern "C" fn InitContestResultsDisplay() {
    unsafe {
        let mut i: i32 = 0i32;
        SetGpuReg(0u8, 64u16);
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(16u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    SetBgTilemapBuffer(
                        ((i) as u8),
                        ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
        SetGpuReg(76u8, 0u16);
        SetGpuReg(72u8, 16191u16);
        SetGpuReg(74u8, 16174u16);
        SetGpuReg(64u8, 0u16);
        SetGpuReg(68u8, 0u16);
        SetGpuReg(66u8, 0u16);
        SetGpuReg(70u8, 0u16);
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
        SetGpuReg(16u8, 0u16);
        SetGpuReg(18u8, 0u16);
        SetGpuReg(20u8, 0u16);
        SetGpuReg(22u8, 0u16);
        SetGpuReg(24u8, 0u16);
        SetGpuReg(26u8, 0u16);
        SetGpuReg(28u8, 0u16);
        SetGpuReg(30u8, 0u16);
        SetGpuRegBits(0u8, 65280u16);
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
pub(crate) unsafe extern "C" fn LoadContestResultsBgGfx() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut numStars: i8 = 0i8;
        let mut round2Points: i8 = 0i8;
        let mut tile1: u16 = 0u16;
        let mut tile2: u16 = 0u16;
        LZDecompressVram(
            ((&raw mut gContestResults_Gfx).cast::<u32>()).cast::<u32>(),
            ((100663296i32) as usize as *mut u8),
        );
        CopyToBgTilemapBuffer(
            3u8,
            (((&raw mut gContestResults_Bg_Tilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u16,
            0u16,
        );
        CopyToBgTilemapBuffer(
            2u8,
            (((&raw mut gContestResults_Interface_Tilemap).cast::<u32>()).cast::<u32>())
                .cast::<u8>(),
            0u16,
            0u16,
        );
        CopyToBgTilemapBuffer(
            0u8,
            (((&raw mut gContestResults_WinnerBanner_Tilemap).cast::<u32>()).cast::<u32>())
                .cast::<u8>(),
            0u16,
            0u16,
        );
        LoadContestResultsTitleBarTilemaps();
        LoadCompressedPalette(
            ((&raw mut gContestResults_Pal).cast::<u32>()).cast::<u32>(),
            0u16,
            512u16,
        );
        LoadPalette(
            (((&raw const sResultsTextWindow_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    numStars = ((GetNumPreliminaryPoints(((i) as u8), 1u8)) as i8);
                    round2Points = GetNumRound2Points(((i) as u8), 1u8);
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 10i32) {
                                break 'l3;
                            }
                            'l4: {
                                tile1 = 24754u16;
                                if j < ((numStars) as i32) {
                                    tile1 = ((((tile1) as i32).wrapping_add(2i32)) as u16);
                                }
                                if j < (if ((round2Points) as i32) < 0i32 {
                                    ((round2Points) as i32).wrapping_neg()
                                } else {
                                    ((round2Points) as i32)
                                }) {
                                    tile2 = 24740u16;
                                    if ((round2Points) as i32) < 0i32 {
                                        tile2 = ((((tile2) as i32).wrapping_add(2i32)) as u16);
                                    }
                                } else {
                                    tile2 = 24738u16;
                                }
                                FillBgTilemapBufferRect_Palette0(
                                    1u8,
                                    tile1,
                                    (((j).wrapping_add(19i32)) as u8),
                                    ((((i).wrapping_mul(3i32)).wrapping_add(5i32)) as u8),
                                    1u8,
                                    1u8,
                                );
                                FillBgTilemapBufferRect_Palette0(
                                    1u8,
                                    tile2,
                                    (((j).wrapping_add(19i32)) as u8),
                                    ((((i).wrapping_mul(3i32)).wrapping_add(6i32)) as u8),
                                    1u8,
                                    1u8,
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(0u8);
        CopyBgTilemapBufferToVram(1u8);
        CopyBgTilemapBufferToVram(2u8);
        CopyBgTilemapBufferToVram(3u8);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        ShowBg(3u8);
    }
}
pub(crate) unsafe extern "C" fn LoadContestMonName(monIndex: u8) {
    unsafe {
        let mut monIndex = monIndex;
        let mut mon: *mut u8 = ((&raw mut gContestMons).cast::<u8>())
            .wrapping_offset(((monIndex) as i32) as isize * 64);
        let mut str: *mut u8 = (&raw mut gDisplayedStringBattle).cast::<u8>();
        if ((monIndex) as i32) == ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32)
        {
            str = StringCopy(
                (&raw mut gDisplayedStringBattle).cast::<u8>(),
                (&raw mut gText_ColorDarkGray).cast::<u8>(),
            );
        }
        StringCopy(str, ((mon).wrapping_add(2)).cast::<u8>());
        AddContestTextPrinter(
            ((monIndex) as i32),
            (&raw mut gDisplayedStringBattle).cast::<u8>(),
            0i32,
        );
        StringCopy(str, (&raw mut gText_Slash).cast::<u8>());
        StringAppend(str, ((mon).wrapping_add(13)).cast::<u8>());
        AddContestTextPrinter(
            ((monIndex) as i32),
            (&raw mut gDisplayedStringBattle).cast::<u8>(),
            50i32,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadAllContestMonNames() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    LoadContestMonName(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn CB2_StartShowContestResults() {
    unsafe {
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            7,
            1,
            (1u16) as i32,
        );
        SetVBlankCallback(None);
        AllocContestResults();
        InitContestResultsDisplay();
        ScanlineEffect_Clear();
        ResetPaletteFade();
        ResetSpriteData();
        ResetTasks();
        FreeAllSpritePalettes();
        LoadContestResultsBgGfx();
        LoadAllContestMonIconPalettes();
        LoadAllContestMonIcons(0u8, 1u8);
        LoadAllContestMonNames();
        crate::c::memset(
            ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read(),
            0i32,
            24u32,
        );
        crate::c::memset(
            ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read(),
            0i32,
            80u32,
        );
        CreateResultsTextWindowSprites();
        TryCreateWirelessSprites();
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            7,
            1,
            (0u16) as i32,
        );
        ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(2))
        .write(CreateTask(Some(Task_ShowContestResults), 5u8));
        SetMainCallback2(Some(CB2_ShowContestResults));
        ((&raw mut gBattle_WIN1H).cast::<u16>()).write(240u16);
        ((&raw mut gBattle_WIN1V).cast::<u16>()).write(32928u16);
        CreateTask(Some(Task_SlideContestResultsBg), 20u8);
        CalculateContestantsResultData();
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 2i32) != 0 {
            crate::c::bf_write(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                7,
                1,
                (1u16) as i32,
            );
        } else {
            PlayBGM(446u16);
        }
        SetVBlankCallback(Some(VBlankCB_ShowContestResults));
    }
}
pub(crate) unsafe extern "C" fn CB2_ShowContestResults() {
    unsafe {
        AnimateSprites();
        BuildOamBuffer();
        RunTasks();
        UpdatePaletteFade();
        CopyBgTilemapBufferToVram(1u8);
        CopyBgTilemapBufferToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_ShowContestResults() {
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
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        ScanlineEffect_InitHBlankDmaTransfer();
    }
}
pub(crate) unsafe extern "C" fn Task_ShowContestResults(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var: u16 = 0u16;
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 1i32) != 0 {
            'l1: {
                let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32);
                if __sw1 == 0i32 {
                    SaveLinkContestResults();
                    if (((((&raw mut gContestFinalStandings).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        == 0i32
                    {
                        IncrementGameStat(35u8);
                        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(8u16);
                        InterviewBefore();
                        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) != 1i32 {
                            InterviewAfter();
                        }
                    }
                    TryGainNewFanFromCounter(2u8);
                    SaveContestWinner(
                        ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as u8),
                    );
                    SaveContestWinner(254u8);
                    ((&raw mut gCurContestWinnerIsForArtist).cast::<u8>()).write(1u8);
                    ((&raw mut gCurContestWinnerSaveIdx).cast::<u8>())
                        .write(GetContestWinnerSaveIdx(254u8, 0u8));
                    var = VarGet(16518u16);
                    VarSet(16518u16, 0u16);
                    SetContinueGameWarpStatusToDynamicWarp();
                    TrySavingData(1u8);
                    ClearContinueGameWarpStatus2();
                    VarSet(16518u16, var);
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    if !((((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 2i32)
                        != 0)
                    {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(100i16);
                    }
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    if (IsLinkTaskFinished()) != 0 {
                        SetLinkStandbyCallback();
                        let __p4 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    }
                    return;
                }
                if __sw1 == 3i32 {
                    if ((IsLinkTaskFinished()) as i32) == 1i32 {
                        PlayBGM(446u16);
                        crate::c::bf_write(
                            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                            7,
                            1,
                            (0u16) as i32,
                        );
                        let __p5 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                        break 'l1;
                    }
                    return;
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
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 1i32) != 0 {
                ShowLinkResultsTextBox((&raw mut gText_CommunicationStandby).cast::<u8>());
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_WaitForLinkPartnersBeforeResults));
            } else {
                IncrementGameStat(36u8);
                if (((((&raw mut gContestFinalStandings).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32)
                    == 0i32
                {
                    IncrementGameStat(37u8);
                }
                SaveContestWinner(
                    ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as u8),
                );
                SaveContestWinner(254u8);
                ((&raw mut gCurContestWinnerIsForArtist).cast::<u8>()).write(1u8);
                ((&raw mut gCurContestWinnerSaveIdx).cast::<u8>())
                    .write(GetContestWinnerSaveIdx(254u8, 0u8));
                TryGainNewFanFromCounter(2u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_AnnouncePreliminaryResults));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForLinkPartnersBeforeResults(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            CreateTask(Some(Task_CommunicateMonIdxsForResults), 0u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(TaskDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CommunicateMonIdxsForResults(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateMonIdxs),
            Some(Task_WaitForLinkPartnerMonIdxs),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForLinkPartnerMonIdxs(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (IsLinkTaskFinished()) != 0 {
            DestroyTask(taskId);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2))
                .read()) as i32) as isize
                    * 40,
            ))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_AnnouncePreliminaryResults));
            HideLinkResultsTextBox();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_AnnouncePreliminaryResults(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut x: i16 = 0i16;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            CreateTask(Some(Task_FlashStarsAndHearts), 20u8);
            x = ((DrawResultsTextWindow(
                (&raw mut gText_AnnouncingResults).cast::<u8>(),
                (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .read(),
            )) as i16);
            StartTextBoxSlideIn(x, 144u16, 120u16, 1088u16);
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 1i32
            {
                if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4))
                .read()) as i32)
                    == 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
            } else {
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    == 2i32
                {
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
                        == 21i32
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(0i16);
                        let __p5 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    }
                } else {
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32)
                        == 3i32
                    {
                        x = ((DrawResultsTextWindow(
                            (&raw mut gText_PreliminaryResults).cast::<u8>(),
                            (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<*mut u8>())
                            .read())
                            .read(),
                        )) as i16);
                        StartTextBoxSlideIn(x, 144u16, 65535u16, 1088u16);
                        let __p6 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    } else {
                        if (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .read()) as i32)
                            == 4i32
                        {
                            if ((((((((&raw mut sContestResults)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4))
                            .read()) as i32)
                                == 2i32
                            {
                                (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .write(0i16);
                                ((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                                .write(Some(Task_ShowPreliminaryResults));
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowPreliminaryResults(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(10))
                .read())
                    != 0)
                {
                    UpdateContestResultBars(
                        0u8,
                        (({
                            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2);
                            let __t3 = (__p2).read();
                            (__p2).write(((__p2).read()).wrapping_add(1));
                            __t3
                        }) as u8),
                    );
                    if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(20))
                    .read()) as i32)
                        == 0i32
                    {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(2i16);
                    } else {
                        let __p4 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(20))
                .read()) as i32)
                    == 0i32
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                StartTextBoxSlideOut(1088u16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
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
                .write(Some(Task_AnnounceRound2Results));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_AnnounceRound2Results(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut x: i16 = 0i16;
        if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .read()) as i32)
            == 0i32
        {
            if (({
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 21i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(0i16);
                x = ((DrawResultsTextWindow(
                    (&raw mut gText_Round2Results).cast::<u8>(),
                    (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .read(),
                )) as i16);
                StartTextBoxSlideIn(x, 144u16, 65535u16, 1088u16);
            }
        } else {
            if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32)
                == 2i32
            {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ShowRound2Results));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowRound2Results(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(10))
                .read())
                    != 0)
                {
                    UpdateContestResultBars(
                        1u8,
                        (({
                            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2);
                            let __t3 = (__p2).read();
                            (__p2).write(((__p2).read()).wrapping_add(1));
                            __t3
                        }) as u8),
                    );
                    if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(20))
                    .read()) as i32)
                        == 0i32
                    {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(2i16);
                    } else {
                        let __p4 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(20))
                .read()) as i32)
                    == 0i32
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                StartTextBoxSlideOut(1088u16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_AnnounceWinner));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_AnnounceWinner(taskId: u8) {
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
                if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4))
                .read()) as i32)
                    == 0i32
                {
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
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
                    == 31i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    let __p5 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            let mut newTaskId: u8 =
                                CreateTask(Some(Task_DrawFinalStandingNumber), 10u8);
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((newTaskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(
                                (((((&raw mut gContestFinalStandings).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i16),
                            );
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((newTaskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .write(((i) as i16));
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
            if __sw1 == 3i32 {
                if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(5))
                .read()) as i32)
                    == 4i32
                {
                    if (({
                        let __p7 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1);
                        let __t8 = ((__p7).read()).wrapping_add(1);
                        (__p7).write(__t8);
                        __t8
                    }) as i32)
                        == 31i32
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(0i16);
                        CreateTask(Some(Task_StartHighlightWinnersBox), 10u8);
                        let __p9 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p9).write(((__p9).read()).wrapping_add(1));
                        {
                            {
                                i = 0i32;
                                'l4: loop {
                                    if !((i < 4i32)
                                        && ((((((&raw mut gContestFinalStandings).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read())
                                            as i32)
                                            != 0i32))
                                    {
                                        break 'l4;
                                    }
                                    'l5: {}
                                    i = (i).wrapping_add(1);
                                }
                            }
                        }
                        BounceMonIconInBox(((i) as u8), 14u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (({
                    let __p10 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    let __t11 = ((__p10).read()).wrapping_add(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    == 21i32
                {
                    let mut winnerTextBuffer = crate::ffi::Align4([0u8; 100]);
                    let mut x: i16 = 0i16;
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    {
                        {
                            i = 0i32;
                            'l6: loop {
                                if !((i < 4i32)
                                    && ((((((&raw mut gContestFinalStandings).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        != 0i32))
                                {
                                    break 'l6;
                                }
                                'l7: {}
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        ((((&raw mut gContestMons).cast::<u8>())
                            .wrapping_offset((i) as isize * 64))
                        .wrapping_add(13))
                        .cast::<u8>(),
                    );
                    ConvertInternationalContestantName((&raw mut gStringVar1).cast::<u8>());
                    StringCopy(
                        (&raw mut gStringVar2).cast::<u8>(),
                        ((((&raw mut gContestMons).cast::<u8>())
                            .wrapping_offset((i) as isize * 64))
                        .wrapping_add(2))
                        .cast::<u8>(),
                    );
                    StringExpandPlaceholders(
                        (&raw mut winnerTextBuffer).cast::<u8>(),
                        (&raw mut gText_ContestantsMonWon).cast::<u8>(),
                    );
                    x = ((DrawResultsTextWindow(
                        (&raw mut winnerTextBuffer).cast::<u8>(),
                        (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .read(),
                    )) as i16);
                    StartTextBoxSlideIn(x, 144u16, 65535u16, 1088u16);
                    let __p12 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ShowWinnerMonBanner));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowWinnerMonBanner(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut spriteId: u8 = 0u8;
        let mut species: u16 = 0u16;
        let mut otId: u32 = 0u32;
        let mut personality: u32 = 0u32;
        let mut pokePal: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gBattle_WIN0H).cast::<u16>()).write(240u16);
                ((&raw mut gBattle_WIN0V).cast::<u16>()).write(
                    (((crate::c::div_i32(160i32, 2i32) << 8) | crate::c::div_i32(160i32, 2i32))
                        as u16),
                );
                {
                    {
                        i = 0i32;
                        'l2: loop {
                            if !((i < 4i32)
                                && ((((((&raw mut gContestFinalStandings).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    != 0i32))
                            {
                                break 'l2;
                            }
                            'l3: {}
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                species = ((((&raw mut gContestMons).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .cast::<u16>())
                .read();
                personality = ((((&raw mut gContestMons).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .wrapping_add(56)
                .cast::<u32>())
                .read();
                otId = ((((&raw mut gContestMons).cast::<u8>())
                    .wrapping_offset((i) as isize * 64))
                .wrapping_add(60)
                .cast::<u32>())
                .read();
                if i == ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32) {
                    HandleLoadSpecialPokePic_2(
                        ((&raw mut gMonFrontPicTable).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 8),
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(1))
                        .read(),
                        ((species) as i32),
                        personality,
                    );
                } else {
                    HandleLoadSpecialPokePic_DontHandleDeoxys(
                        ((&raw mut gMonFrontPicTable).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 8),
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(1))
                        .read(),
                        ((species) as i32),
                        personality,
                    );
                }
                pokePal = GetMonSpritePalStructFromOtIdPersonality(species, otId, personality);
                LoadCompressedSpritePalette(pokePal);
                SetMultiuseSpriteTemplateToPokemon(species, 1u8);
                (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .write(((pokePal).wrapping_add(4).cast::<u16>()).read());
                spriteId = CreateSprite(
                    (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
                    272i16,
                    ((crate::c::div_i32(160i32, 2i32)) as i16),
                    10u8,
                );
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(((species) as i16));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_WinnerMonSlideIn));
                ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8))
                .write(spriteId);
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheet_Confetti).cast::<u8>().cast_mut(),
                );
                LoadCompressedSpritePalette(
                    (&raw const sSpritePalette_Confetti).cast::<u8>().cast_mut(),
                );
                CreateTask(Some(Task_CreateConfetti), 10u8);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == 1i32
                {
                    let mut counter: u8 = 0u8;
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(0i16);
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p5).write((((((__p5).read()) as i32).wrapping_add(2i32)) as i16));
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        > 32i32
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(32i16);
                    }
                    counter = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as u8);
                    ((&raw mut gBattle_WIN0V).cast::<u16>()).write(
                        ((((crate::c::div_i32(160i32, 2i32)).wrapping_sub(((counter) as i32)) << 8)
                            | (crate::c::div_i32(160i32, 2i32)).wrapping_add(((counter) as i32)))
                            as u16),
                    );
                    if ((counter) as i32) == 32i32 {
                        let __p6 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6))
                .read()) as i32)
                    == 1i32
                {
                    let __p7 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __p8 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    == 121i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_WinnerMonSlideOut));
                    let __p10 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6))
                .read()) as i32)
                    == 2i32
                {
                    let mut top: u8 =
                        ((((((&raw mut gBattle_WIN0V).cast::<u16>()).read()) as i32) >> 8) as u8);
                    top = ((((top) as i32).wrapping_add(2i32)) as u8);
                    if ((top) as i32) > crate::c::div_i32(160i32, 2i32) {
                        top = ((crate::c::div_i32(160i32, 2i32)) as u8);
                    }
                    ((&raw mut gBattle_WIN0V).cast::<u16>()).write(
                        (((((top) as i32) << 8) | (160i32).wrapping_sub(((top) as i32))) as u16),
                    );
                    if ((top) as i32) == crate::c::div_i32(160i32, 2i32) {
                        let __p11 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p11).write(((__p11).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6))
                .read()) as i32)
                    == 2i32
                {
                    ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(9))
                    .write(1u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_SetSeenWinnerMon));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SetSeenWinnerMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut nationalDexNum: i32 = 0i32;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if !((((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 1i32) != 0) {
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i < 4i32) {
                            break 'l1;
                        }
                        'l2: {
                            nationalDexNum = ((SpeciesToNationalPokedexNum(
                                ((((&raw mut gContestMons).cast::<u8>())
                                    .wrapping_offset((i) as isize * 64))
                                .cast::<u16>())
                                .read(),
                            )) as i32);
                            GetSetPokedexFlag(((nationalDexNum) as u16), 2u8);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_TryDisconnectLinkPartners));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TryDisconnectLinkPartners(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 1i32) != 0 {
            if !((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read())
                != 0)
            {
                ShowLinkResultsTextBox((&raw mut gText_CommunicationStandby).cast::<u8>());
                SetCloseLinkCallback();
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_WaitForLinkPartnersDisconnect));
            }
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_TrySetContestInterviewData));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForLinkPartnersDisconnect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
            if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 2i32) != 0 {
                DestroyWirelessStatusIndicatorSprite();
            }
            HideLinkResultsTextBox();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_TrySetContestInterviewData));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TrySetContestInterviewData(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 1i32) != 0) {
            BravoTrainerPokemonProfile_BeforeInterview2(
                (((&raw mut gContestFinalStandings).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32) as isize,
                ))
                .read(),
            );
        }
        BeginHardwarePaletteFade(255u8, 0u8, 0u8, 16u8, 0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_EndShowContestResults));
    }
}
pub(crate) unsafe extern "C" fn Task_EndShowContestResults(taskId: u8) {
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
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == 0i32
            {
                DestroyTask(
                    ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(3))
                    .read(),
                );
                BlendPalettes(65535u32, 16u8, 0u16);
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 1i32
                {
                    BlendPalettes(4294901760u32, 16u8, 0u16);
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                } else {
                    SetGpuReg(80u8, 0u16);
                    SetGpuReg(84u8, 0u16);
                    DestroyTask(taskId);
                    FreeAllWindowBuffers();
                    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                    FreeContestResults();
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SlideContestResultsBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = (&raw mut gBattle_BG3_X).cast::<u16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as u16));
        let __p2 = (&raw mut gBattle_BG3_Y).cast::<u16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(1i32)) as u16));
        if ((((&raw mut gBattle_BG3_X).cast::<u16>()).read()) as i32) > 255i32 {
            let __p3 = (&raw mut gBattle_BG3_X).cast::<u16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_sub(255i32)) as u16));
        }
        if ((((&raw mut gBattle_BG3_Y).cast::<u16>()).read()) as i32) > 255i32 {
            let __p4 = (&raw mut gBattle_BG3_Y).cast::<u16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_sub(255i32)) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_FlashStarsAndHearts(taskId: u8) {
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
            == 2i32
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            if !((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read())
                != 0)
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
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == 16i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(1i16);
            } else {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(0i16);
                }
            }
            BlendPalette(
                107u16,
                1u16,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8),
                11998u16,
            );
            BlendPalette(
                104u16,
                1u16,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8),
                32767u16,
            );
            BlendPalette(
                110u16,
                1u16,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8),
                30654u16,
            );
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 0i32
        {
            ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(10))
            .write(0u8);
        } else {
            ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(10))
            .write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadContestMonIcon(
    species: u16,
    monIndex: u8,
    srcOffset: u8,
    useDmaNow: u8,
    personality: u32,
) {
    unsafe {
        let mut species = species;
        let mut monIndex = monIndex;
        let mut srcOffset = srcOffset;
        let mut useDmaNow = useDmaNow;
        let mut personality = personality;
        let mut iconPtr: *mut u8 = core::ptr::null_mut();
        let mut var0: u16 = 0u16;
        let mut var1: u16 = 0u16;
        let mut frameNum: u16 = 0u16;
        if ((monIndex) as i32) == ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32)
        {
            frameNum = 1u16;
        } else {
            frameNum = 0u16;
        }
        iconPtr = GetMonIconPtr(species, personality, ((frameNum) as u32));
        iconPtr = (iconPtr).wrapping_offset(
            ((((srcOffset) as i32).wrapping_mul(512i32)).wrapping_add(128i32)) as isize,
        );
        if (useDmaNow) != 0 {
            RequestDma3Copy(
                iconPtr,
                ((100679680i32) as usize as *mut u8)
                    .wrapping_offset((((monIndex) as i32).wrapping_mul(512i32)) as isize * 1),
                384u16,
                1u8,
            );
            var0 = ((((monIndex) as i32).wrapping_add(10i32) << 12) as u16);
            var1 = (((((monIndex) as i32).wrapping_mul(16i32)).wrapping_add(512i32)) as u16);
            WriteSequenceToBgTilemapBuffer(
                1u8,
                ((((var1) as i32) | ((var0) as i32)) as u16),
                3u8,
                (((((monIndex) as i32).wrapping_mul(3i32)).wrapping_add(4i32)) as u8),
                4u8,
                3u8,
                17u8,
                1i16,
            );
        } else {
            RequestDma3Copy(
                iconPtr,
                ((100679680i32) as usize as *mut u8)
                    .wrapping_offset((((monIndex) as i32).wrapping_mul(512i32)) as isize * 1),
                384u16,
                1u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn LoadAllContestMonIcons(srcOffset: u8, useDmaNow: u8) {
    unsafe {
        let mut srcOffset = srcOffset;
        let mut useDmaNow = useDmaNow;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    LoadContestMonIcon(
                        ((((&raw mut gContestMons).cast::<u8>())
                            .wrapping_offset((i) as isize * 64))
                        .cast::<u16>())
                        .read(),
                        ((i) as u8),
                        srcOffset,
                        useDmaNow,
                        ((((&raw mut gContestMons).cast::<u8>())
                            .wrapping_offset((i) as isize * 64))
                        .wrapping_add(56)
                        .cast::<u32>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadAllContestMonIconPalettes() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut species: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    species = ((((((&raw mut gContestMons).cast::<u8>())
                        .wrapping_offset((i) as isize * 64))
                    .cast::<u16>())
                    .read()) as i32);
                    LoadPalette(
                        ((((&raw mut gMonIconPalettes).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gMonIconPaletteIndices).cast::<u8>()).wrapping_offset(
                                ((GetIconSpecies(((species) as u16), 0u32)) as i32) as isize,
                            ))
                            .read()) as i32) as isize
                                * 32,
                        ))
                        .cast::<u16>())
                        .cast::<u8>(),
                        (((0i32).wrapping_add(((10i32).wrapping_add(i)).wrapping_mul(16i32)))
                            as u16),
                        32u16,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryCreateWirelessSprites() {
    unsafe {
        let mut sheet: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 2i32) != 0 {
            LoadWirelessStatusIndicatorSpriteGfx();
            CreateWirelessStatusIndicatorSprite(8u8, 8u8);
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut gWirelessStatusIndicatorSpriteId).cast::<u8>()).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(67))
            .write(1u8);
            sheet = LoadSpriteSheet(
                (&raw const sSpriteSheet_WirelessIndicatorWindow)
                    .cast::<u8>()
                    .cast_mut(),
            );
            RequestDma3Fill(
                (-1i32),
                ((100728832i32) as usize as *mut u8)
                    .wrapping_offset((((sheet) as i32).wrapping_mul(32i32)) as isize * 1),
                128u16,
                1u8,
            );
            spriteId = CreateSprite(
                (&raw const sSpriteTemplate_WirelessIndicatorWindow)
                    .cast::<u8>()
                    .cast_mut(),
                8i16,
                8i16,
                0u8,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                2,
                2,
                (2u32) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DrawResultsTextWindow(text: *mut u8, spriteId: u8) -> i32 {
    unsafe {
        let mut text = text;
        let mut spriteId = spriteId;
        let mut windowId: u16 = 0u16;
        let mut tileWidth: i32 = 0i32;
        let mut strWidth: i32 = 0i32;
        let mut spriteTilePtrs = crate::ffi::Align4([0u8; 16]);
        let mut dst: *mut u8 = core::ptr::null_mut();
        let mut windowTemplate = crate::ffi::Align4([0u8; 8]);
        crate::c::memset((&raw mut windowTemplate).cast::<u8>(), 0i32, 8u32);
        (((&raw mut windowTemplate).cast::<u8>()).wrapping_add(3))
            .write(((crate::c::div_i32(240i32, 8i32)) as u8));
        (((&raw mut windowTemplate).cast::<u8>()).wrapping_add(4)).write(2u8);
        windowId = AddWindow((&raw mut windowTemplate).cast::<u8>());
        FillWindowPixelBuffer(((windowId) as u8), 17u8);
        strWidth = GetStringWidth(1u8, text, 0i16);
        tileWidth = crate::c::div_i32((strWidth).wrapping_add(9i32), 8i32);
        if tileWidth > crate::c::div_i32(240i32, 8i32) {
            tileWidth = crate::c::div_i32(240i32, 8i32);
        }
        AddTextPrinterParameterized3(
            ((windowId) as u8),
            1u8,
            ((crate::c::div_i32(
                ((tileWidth).wrapping_mul(8i32)).wrapping_sub(strWidth),
                2i32,
            )) as u8),
            1u8,
            ((&raw const sContestLinkTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            text,
        );
        {
            let mut i: i32 = 0i32;
            let mut sprite: *mut u8 = core::ptr::null_mut();
            let mut src: *mut u8 = core::ptr::null_mut();
            let mut windowTilesPtr: *mut u8 = core::ptr::null_mut();
            windowTilesPtr = ((GetWindowAttribute(((windowId) as u8), 7u8)) as usize as *mut u8);
            src = ((&raw const sResultsTextWindow_Gfx).cast::<u8>().cast_mut()).cast::<u8>();
            sprite = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            ((&raw mut spriteTilePtrs).cast::<*mut u8>()).write(
                (((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                    .wrapping_mul(32i32))
                .wrapping_add(100728832i32)) as usize as *mut u8),
            );
            {
                i = 1i32;
                'l1: loop {
                    if !(i < ((crate::c::div_u32(16u32, 4u32)) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        (((&raw mut spriteTilePtrs).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .write(
                            (((((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((sprite).wrapping_add(46)).cast::<i16>())
                                        .wrapping_offset(((i).wrapping_sub(1i32)) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(4),
                                0,
                                10,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(32i32))
                            .wrapping_add(100728832i32)) as usize
                                as *mut u8),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l3: loop {
                    if !(i < ((crate::c::div_u32(16u32, 4u32)) as i32)) {
                        break 'l3;
                    }
                    'l4: {
                        'l5: loop {
                            'l6: {
                                {
                                    let mut tmp: u32 = 0u32;
                                    (&raw mut tmp).write_volatile(0u32);
                                    'l7: loop {
                                        'l8: {
                                            CpuSet(
                                                (&raw mut tmp).cast::<u8>(),
                                                (((&raw mut spriteTilePtrs).cast::<*mut u8>())
                                                    .wrapping_offset((i) as isize))
                                                .read(),
                                                ((83886080i32
                                                    | (crate::c::div_i32(
                                                        1024i32,
                                                        crate::c::div_i32(32i32, 8i32),
                                                    ) & 2097151i32))
                                                    as u32),
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
                    }
                    i = (i).wrapping_add(1);
                }
            }
            dst = ((&raw mut spriteTilePtrs).cast::<*mut u8>()).read();
            'l9: loop {
                'l10: {
                    'l11: loop {
                        'l12: {
                            CpuSet(
                                src,
                                dst,
                                ((67108864i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l9;
                }
            }
            'l13: loop {
                'l14: {
                    'l15: loop {
                        'l16: {
                            CpuSet(
                                (src).wrapping_offset(128),
                                (dst).wrapping_offset(256),
                                ((67108864i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l15;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l13;
                }
            }
            'l17: loop {
                'l18: {
                    'l19: loop {
                        'l20: {
                            CpuSet(
                                (src).wrapping_offset(128),
                                (dst).wrapping_offset(512),
                                ((67108864i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l19;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l17;
                }
            }
            'l21: loop {
                'l22: {
                    'l23: loop {
                        'l24: {
                            CpuSet(
                                (src).wrapping_offset(64),
                                (dst).wrapping_offset(768),
                                ((67108864i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l23;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l21;
                }
            }
            {
                i = 0i32;
                'l25: loop {
                    if !(i < tileWidth) {
                        break 'l25;
                    }
                    'l26: {
                        dst = ((((&raw mut spriteTilePtrs).cast::<*mut u8>()).wrapping_offset(
                            (crate::c::div_i32((i).wrapping_add(1i32), 8i32)) as isize,
                        ))
                        .read())
                        .wrapping_offset(
                            ((crate::c::rem_i32((i).wrapping_add(1i32), 8i32)).wrapping_mul(32i32))
                                as isize,
                        );
                        'l27: loop {
                            'l28: {
                                'l29: loop {
                                    'l30: {
                                        CpuSet(
                                            (src).wrapping_offset(192),
                                            dst,
                                            ((67108864i32
                                                | (crate::c::div_i32(
                                                    32i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
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
                        'l31: loop {
                            'l32: {
                                'l33: loop {
                                    'l34: {
                                        CpuSet(
                                            windowTilesPtr,
                                            (dst).wrapping_offset(256),
                                            ((67108864i32
                                                | (crate::c::div_i32(
                                                    32i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
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
                        'l35: loop {
                            'l36: {
                                'l37: loop {
                                    'l38: {
                                        CpuSet(
                                            (windowTilesPtr).wrapping_offset(960),
                                            (dst).wrapping_offset(512),
                                            ((67108864i32
                                                | (crate::c::div_i32(
                                                    32i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l37;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l35;
                            }
                        }
                        'l39: loop {
                            'l40: {
                                'l41: loop {
                                    'l42: {
                                        CpuSet(
                                            (src).wrapping_offset(224),
                                            (dst).wrapping_offset(768),
                                            ((67108864i32
                                                | (crate::c::div_i32(
                                                    32i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l41;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l39;
                            }
                        }
                        windowTilesPtr = (windowTilesPtr).wrapping_offset(32);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            dst = ((((&raw mut spriteTilePtrs).cast::<*mut u8>())
                .wrapping_offset((crate::c::div_i32((i).wrapping_add(1i32), 8i32)) as isize))
            .read())
            .wrapping_offset(
                ((crate::c::rem_i32((i).wrapping_add(1i32), 8i32)).wrapping_mul(32i32)) as isize,
            );
            'l43: loop {
                'l44: {
                    'l45: loop {
                        'l46: {
                            CpuSet(
                                (src).wrapping_offset(32),
                                dst,
                                ((67108864i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l45;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l43;
                }
            }
            'l47: loop {
                'l48: {
                    'l49: loop {
                        'l50: {
                            CpuSet(
                                (src).wrapping_offset(160),
                                (dst).wrapping_offset(256),
                                ((67108864i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l49;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l47;
                }
            }
            'l51: loop {
                'l52: {
                    'l53: loop {
                        'l54: {
                            CpuSet(
                                (src).wrapping_offset(160),
                                (dst).wrapping_offset(512),
                                ((67108864i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l53;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l51;
                }
            }
            'l55: loop {
                'l56: {
                    'l57: loop {
                        'l58: {
                            CpuSet(
                                (src).wrapping_offset(96),
                                (dst).wrapping_offset(768),
                                ((67108864i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l57;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l55;
                }
            }
        }
        RemoveWindow(((windowId) as u8));
        return crate::c::div_i32(
            (240i32).wrapping_sub(((tileWidth).wrapping_add(2i32)).wrapping_mul(8i32)),
            2i32,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateResultsTextWindowSprites() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut template = crate::ffi::Align4([0u8; 24]);
        let mut spriteIds = crate::ffi::Align4([0u8; 8]);
        (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_ResultsTextWindow)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(64u32, 8u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    LoadSpriteSheet(
                        (((&raw const sSpriteSheets_ResultsTextWindow)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        LoadSpritePalette(
            (&raw const sSpritePalette_ResultsTextWindow)
                .cast::<u8>()
                .cast_mut(),
        );
        {
            i = 0i32;
            'l3: loop {
                if !(i < ((crate::c::div_u32(64u32, 8u32)) as i32)) {
                    break 'l3;
                }
                'l4: {
                    (((&raw mut spriteIds).cast::<u8>()).wrapping_offset((i) as isize)).write(
                        CreateSprite((&raw mut template).cast::<u8>(), 272i16, 144i16, 10u8),
                    );
                    let __p1 = ((&raw mut template).cast::<u8>()).cast::<u16>();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut spriteIds).cast::<u8>()).read()) as i32) as isize * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .write((((((&raw mut spriteIds).cast::<u8>()).wrapping_offset(1)).read()) as i16));
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut spriteIds).cast::<u8>()).read()) as i32) as isize * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write((((((&raw mut spriteIds).cast::<u8>()).wrapping_offset(2)).read()) as i16));
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((&raw mut spriteIds).cast::<u8>()).read()) as i32) as isize * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write((((((&raw mut spriteIds).cast::<u8>()).wrapping_offset(3)).read()) as i16));
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut spriteIds).cast::<u8>()).wrapping_offset(4)).read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .write((((((&raw mut spriteIds).cast::<u8>()).wrapping_offset(5)).read()) as i16));
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut spriteIds).cast::<u8>()).wrapping_offset(4)).read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write((((((&raw mut spriteIds).cast::<u8>()).wrapping_offset(6)).read()) as i16));
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut spriteIds).cast::<u8>()).wrapping_offset(4)).read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write((((((&raw mut spriteIds).cast::<u8>()).wrapping_offset(7)).read()) as i16));
        (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read())
        .write(((&raw mut spriteIds).cast::<u8>()).read());
        ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .write(0u8);
        ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1))
        .write((((&raw mut spriteIds).cast::<u8>()).wrapping_offset(4)).read());
        HideLinkResultsTextBox();
    }
}
pub(crate) unsafe extern "C" fn StartTextBoxSlideIn(
    x: i16,
    y: u16,
    slideOutTimer: u16,
    slideIncrement: u16,
) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut slideOutTimer = slideOutTimer;
        let mut slideIncrement = slideIncrement;
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .read()) as i32) as isize
                * 68,
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(272i16);
        ((sprite).wrapping_add(34).cast::<i16>()).write(((y) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((((x) as i32).wrapping_add(32i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((slideOutTimer) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write(((slideIncrement) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_TextBoxSlideIn));
        ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn StartTextBoxSlideOut(slideIncrement: u16) {
    unsafe {
        let mut slideIncrement = slideIncrement;
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .read()) as i32) as isize
                * 68,
        );
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write(((slideIncrement) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_TextBoxSlideOut));
        ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .write(3u8);
    }
}
pub(crate) unsafe extern "C" fn EndTextBoxSlideOut(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(272i16);
        ((sprite).wrapping_add(34).cast::<i16>()).write(144i16);
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TextBoxSlideIn(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i32 = 0i32;
        let mut delta: i16 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
            .read()) as i32)
            .wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16);
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub((((delta) as i32) >> 8))) as i16));
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
        {
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read());
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    let mut sprite2: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32) as isize
                            * 68,
                    );
                    ((sprite2).wrapping_add(32).cast::<i16>()).write(
                        (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                            .wrapping_add(
                                ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32),
                            ))
                        .wrapping_add(((i).wrapping_add(1i32)).wrapping_mul(64i32)))
                            as i16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_EndTextBoxSlideIn));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_EndTextBoxSlideIn(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .write(2u8);
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u16)
            as i32)
            != 65535i32
        {
            if (({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == (-1i32)
            {
                StartTextBoxSlideOut(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as u16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TextBoxSlideOut(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i32 = 0i32;
        let mut delta: i16 = 0i16;
        delta = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
            as i32)
            .wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16);
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub((((delta) as i32) >> 8))) as i16));
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    let mut sprite2: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32) as isize
                            * 68,
                    );
                    ((sprite2).wrapping_add(32).cast::<i16>()).write(
                        (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                            .wrapping_add(
                                ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32),
                            ))
                        .wrapping_add(((i).wrapping_add(1i32)).wrapping_mul(64i32)))
                            as i16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
            < (-224i32)
        {
            EndTextBoxSlideOut(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn ShowLinkResultsTextBox(text: *mut u8) {
    unsafe {
        let mut text = text;
        let mut i: i32 = 0i32;
        let mut x: u16 = 0u16;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        x = ((DrawResultsTextWindow(
            text,
            ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read(),
        )) as u16);
        sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        );
        ((sprite).wrapping_add(32).cast::<i16>())
            .write(((((x) as i32).wrapping_add(32i32)) as i16));
        ((sprite).wrapping_add(34).cast::<i16>()).write(80i16);
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .write(
                        (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                            .wrapping_add(
                                ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32),
                            ))
                        .wrapping_add(((i).wrapping_add(1i32)).wrapping_mul(64i32)))
                            as i16),
                    );
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(((sprite).wrapping_add(34).cast::<i16>()).read());
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(240u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(
            (((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_sub(16i32)
                << 8)
                | ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(16i32))
                as u16),
        );
        SetGpuReg(72u8, 16190u16);
    }
}
pub(crate) unsafe extern "C" fn HideLinkResultsTextBox() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        );
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>())
                                .wrapping_offset((i) as isize))
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
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
        SetGpuReg(64u8, ((&raw mut gBattle_WIN0H).cast::<u16>()).read());
        SetGpuReg(68u8, ((&raw mut gBattle_WIN0V).cast::<u16>()).read());
        SetGpuReg(72u8, 16191u16);
    }
}
pub(crate) unsafe extern "C" fn LoadContestResultsTitleBarTilemaps() {
    unsafe {
        let mut palette: u8 = 0u8;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        x = 5i32;
        y = 1i32;
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 1i32) != 0 {
            CopyToBgTilemapBufferRect(
                2u8,
                (((&raw mut gContestResultsTitle_Link_Tilemap).cast::<u16>()).cast::<u16>())
                    .cast::<u8>(),
                5u8,
                1u8,
                5u8,
                2u8,
            );
            x = 10i32;
        } else {
            if ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as i32) == 0i32 {
                CopyToBgTilemapBufferRect(
                    2u8,
                    (((&raw mut gContestResultsTitle_Normal_Tilemap).cast::<u16>()).cast::<u16>())
                        .cast::<u8>(),
                    5u8,
                    1u8,
                    10u8,
                    2u8,
                );
                x = 15i32;
            } else {
                if ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as i32) == 1i32 {
                    CopyToBgTilemapBufferRect(
                        2u8,
                        (((&raw mut gContestResultsTitle_Super_Tilemap).cast::<u16>())
                            .cast::<u16>())
                        .cast::<u8>(),
                        5u8,
                        1u8,
                        10u8,
                        2u8,
                    );
                    x = 15i32;
                } else {
                    if ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as i32) == 2i32
                    {
                        CopyToBgTilemapBufferRect(
                            2u8,
                            (((&raw mut gContestResultsTitle_Hyper_Tilemap).cast::<u16>())
                                .cast::<u16>())
                            .cast::<u8>(),
                            5u8,
                            1u8,
                            10u8,
                            2u8,
                        );
                        x = 15i32;
                    } else {
                        CopyToBgTilemapBufferRect(
                            2u8,
                            (((&raw mut gContestResultsTitle_Master_Tilemap).cast::<u16>())
                                .cast::<u16>())
                            .cast::<u8>(),
                            5u8,
                            1u8,
                            10u8,
                            2u8,
                        );
                        x = 15i32;
                    }
                }
            }
        }
        if ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as i32) == 0i32 {
            palette = 0u8;
            CopyToBgTilemapBufferRect(
                2u8,
                (((&raw mut gContestResultsTitle_Cool_Tilemap).cast::<u16>()).cast::<u16>())
                    .cast::<u8>(),
                ((x) as u8),
                ((y) as u8),
                5u8,
                2u8,
            );
        } else {
            if ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as i32) == 1i32 {
                palette = 1u8;
                CopyToBgTilemapBufferRect(
                    2u8,
                    (((&raw mut gContestResultsTitle_Beauty_Tilemap).cast::<u16>()).cast::<u16>())
                        .cast::<u8>(),
                    ((x) as u8),
                    ((y) as u8),
                    5u8,
                    2u8,
                );
            } else {
                if ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as i32) == 2i32
                {
                    palette = 2u8;
                    CopyToBgTilemapBufferRect(
                        2u8,
                        (((&raw mut gContestResultsTitle_Cute_Tilemap).cast::<u16>())
                            .cast::<u16>())
                        .cast::<u8>(),
                        ((x) as u8),
                        ((y) as u8),
                        5u8,
                        2u8,
                    );
                } else {
                    if ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as i32)
                        == 3i32
                    {
                        palette = 3u8;
                        CopyToBgTilemapBufferRect(
                            2u8,
                            (((&raw mut gContestResultsTitle_Smart_Tilemap).cast::<u16>())
                                .cast::<u16>())
                            .cast::<u8>(),
                            ((x) as u8),
                            ((y) as u8),
                            5u8,
                            2u8,
                        );
                    } else {
                        palette = 4u8;
                        CopyToBgTilemapBufferRect(
                            2u8,
                            (((&raw mut gContestResultsTitle_Tough_Tilemap).cast::<u16>())
                                .cast::<u16>())
                            .cast::<u8>(),
                            ((x) as u8),
                            ((y) as u8),
                            5u8,
                            2u8,
                        );
                    }
                }
            }
        }
        x = (x).wrapping_add(5i32);
        CopyToBgTilemapBufferRect(
            2u8,
            (((&raw mut gContestResultsTitle_Tilemap).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            ((x) as u8),
            ((y) as u8),
            6u8,
            2u8,
        );
        CopyToBgTilemapBufferRect_ChangePalette(
            2u8,
            ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<*mut u8>())
            .wrapping_offset(2))
            .read(),
            0u8,
            0u8,
            32u8,
            4u8,
            palette,
        );
    }
}
pub(crate) unsafe extern "C" fn GetNumPreliminaryPoints(monIndex: u8, capPoints: u8) -> u8 {
    unsafe {
        let mut monIndex = monIndex;
        let mut capPoints = capPoints;
        let mut condition: u32 = ((((((((&raw mut gContestMonRound1Points).cast::<i16>())
            .cast::<i16>())
        .wrapping_offset(((monIndex) as i32) as isize))
        .read()) as i32)
            << 16) as u32);
        let mut numStars: u32 = crate::c::div_u32(condition, 63u32);
        if (numStars & 65535u32) != 0 {
            numStars = (numStars).wrapping_add(65536u32);
        }
        numStars = (numStars >> 16);
        if (numStars == 0u32) && ((condition) != 0) {
            numStars = 1u32;
        }
        if ((capPoints) != 0) && (numStars > 10u32) {
            numStars = 10u32;
        }
        return ((numStars) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetNumRound2Points(monIndex: u8, capPoints: u8) -> i8 {
    unsafe {
        let mut monIndex = monIndex;
        let mut capPoints = capPoints;
        let mut r4: u32 = 0u32;
        let mut numHearts: u32 = 0u32;
        let mut results: i16 = 0i16;
        let mut points: i8 = 0i8;
        results = ((((&raw mut gContestMonRound2Points).cast::<i16>()).cast::<i16>())
            .wrapping_offset(((monIndex) as i32) as isize))
        .read();
        if ((results) as i32) < 0i32 {
            r4 = ((((results) as i32).wrapping_neg() << 16) as u32);
        } else {
            r4 = ((((results) as i32) << 16) as u32);
        }
        numHearts = crate::c::div_u32(r4, 80u32);
        if (numHearts & 65535u32) != 0 {
            numHearts = (numHearts).wrapping_add(65536u32);
        }
        numHearts = (numHearts >> 16);
        if (numHearts == 0u32) && (r4 != 0u32) {
            numHearts = 1u32;
        }
        if ((capPoints) != 0) && (numHearts > 10u32) {
            numHearts = 10u32;
        }
        if ((((((&raw mut gContestMonRound2Points).cast::<i16>()).cast::<i16>())
            .wrapping_offset(((monIndex) as i32) as isize))
        .read()) as i32)
            < 0i32
        {
            points = (((numHearts).wrapping_neg()) as i8);
        } else {
            points = ((numHearts) as i8);
        }
        return points;
    }
}
pub(crate) unsafe extern "C" fn Task_DrawFinalStandingNumber(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut firstTileNum: u16 = 0u16;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .read()) as i32)
            == 0i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(
                ((((3i32).wrapping_sub(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32),
                ))
                .wrapping_mul(40i32)) as i16),
            );
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32)
                == 1i32
            {
                if (({
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t3 = ((__p2).read()).wrapping_sub(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    == (-1i32)
                {
                    firstTileNum = ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_mul(2i32))
                    .wrapping_add(20547i32)) as u16);
                    WriteSequenceToBgTilemapBuffer(
                        2u8,
                        firstTileNum,
                        1u8,
                        (((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_mul(3i32))
                        .wrapping_add(5i32)) as u8),
                        2u8,
                        1u8,
                        17u8,
                        1i16,
                    );
                    WriteSequenceToBgTilemapBuffer(
                        2u8,
                        ((((firstTileNum) as i32).wrapping_add(16i32)) as u16),
                        1u8,
                        (((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_mul(3i32))
                        .wrapping_add(6i32)) as u8),
                        2u8,
                        1u8,
                        17u8,
                        1i16,
                    );
                    let __p4 = (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_add(5);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    DestroyTask(taskId);
                    PlaySE(24u16);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartHighlightWinnersBox(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        {
            {
                i = 0i32;
                'l1: loop {
                    if !((i < 4i32)
                        && ((((((&raw mut gContestFinalStandings).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            != 0i32))
                    {
                        break 'l1;
                    }
                    'l2: {}
                    i = (i).wrapping_add(1);
                }
            }
        }
        CopyToBgTilemapBufferRect_ChangePalette(
            2u8,
            (((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<*mut u8>())
            .wrapping_offset(2))
            .read())
            .wrapping_offset((((i).wrapping_mul(192i32)).wrapping_add(256i32)) as isize),
            0u8,
            ((((i).wrapping_mul(3i32)).wrapping_add(4i32)) as u8),
            32u8,
            3u8,
            9u8,
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(((i) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(12))
        .write(1i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HighlightWinnersBox));
        ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(3))
        .write(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_HighlightWinnersBox(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 1i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(0i16);
            BlendPalette(
                145u16,
                1u16,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12))
                .read()) as u8),
                28557u16,
            );
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(13))
            .read()) as i32)
                == 0i32
            {
                if (({
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == 16i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(13))
                    .write(1i16);
                }
            } else {
                if (({
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    let __t6 = ((__p5).read()).wrapping_sub(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    == 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(13))
                    .write(0i16);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WinnerMonSlideIn(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 10i32 {
            if (({
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 10i32
            {
                PlayCry_Normal(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as u16),
                    0i8,
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            }
        } else {
            let mut delta: i16 = ((((((((sprite).wrapping_add(46)).cast::<i16>())
                .wrapping_offset(1))
            .read()) as i32)
                .wrapping_add(1536i32)) as i16);
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_sub((((delta) as i32) >> 8))) as i16));
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p4).write((((((__p4).read()) as i32).wrapping_add(1536i32)) as i16));
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p5).write((((((__p5).read()) as i32) & 255i32) as i16));
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                < crate::c::div_i32(240i32, 2i32)
            {
                ((sprite).wrapping_add(32).cast::<i16>())
                    .write(((crate::c::div_i32(240i32, 2i32)) as i16));
            }
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                == crate::c::div_i32(240i32, 2i32)
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6))
                .write(1u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WinnerMonSlideOut(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut delta: i16 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .read()) as i32)
            .wrapping_add(1536i32)) as i16);
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub((((delta) as i32) >> 8))) as i16));
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(1536i32)) as i16));
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < (-32i32) {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(6))
            .write(2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CreateConfetti(taskId: u8) {
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
            == 5i32
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            if ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(7))
            .read()) as i32)
                < 40i32
            {
                let mut spriteId: u8 = CreateSprite(
                    (&raw const sSpriteTemplate_Confetti)
                        .cast::<u8>()
                        .cast_mut(),
                    (((crate::c::rem_i32(((Random()) as i32), 240i32)).wrapping_sub(20i32)) as i16),
                    44i16,
                    5u8,
                );
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(((crate::c::rem_i32(((Random()) as i32), 512i32)) as i16));
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(
                    (((crate::c::rem_i32(((Random()) as i32), 24i32)).wrapping_add(16i32)) as i16),
                );
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(
                    (((crate::c::rem_i32(((Random()) as i32), 256i32)).wrapping_add(48i32)) as i16),
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(4),
                        0,
                        10,
                        false,
                    ) as u16) as i32)
                        .wrapping_add(crate::c::rem_i32(((Random()) as i32), 17i32)))
                        as u16) as i32,
                );
                let __p3 = (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(7);
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
        }
        if (((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(9))
        .read())
            != 0
        {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Confetti(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut delta: i16 = 0i16;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                >> 8) as i16),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
        ));
        delta = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
            as i32)
            .wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16);
        let __p2 = (sprite).wrapping_add(32).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add((((delta) as i32) >> 8))) as i16));
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p4).write((((((__p4).read()) as i32) & 255i32) as i16));
        let __p5 = (sprite).wrapping_add(34).cast::<i16>();
        (__p5).write(((__p5).read()).wrapping_add(1));
        if (((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(9))
        .read())
            != 0
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 248i32)
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) > 116i32)
        {
            DestroySprite(sprite);
            let __p6 = (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(7);
            (__p6).write(((__p6).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn BounceMonIconInBox(monIndex: u8, numFrames: u8) {
    unsafe {
        let mut monIndex = monIndex;
        let mut numFrames = numFrames;
        let mut taskId: u8 = CreateTask(Some(Task_BounceMonIconInBox), 8u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((monIndex) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((numFrames) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((((&raw mut gContestMons).cast::<u8>())
                .wrapping_offset(((monIndex) as i32) as isize * 64))
            .cast::<u16>())
            .read()) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_BounceMonIconInBox(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut monIndex: u8 = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
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
            == ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            LoadContestMonIcon(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u16),
                monIndex,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .read()) as u8),
                0u8,
                ((((&raw mut gContestMons).cast::<u8>())
                    .wrapping_offset(((monIndex) as i32) as isize * 64))
                .wrapping_add(56)
                .cast::<u32>())
                .read(),
            );
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11);
            (__p3).write((((((__p3).read()) as i32) ^ 1i32) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn CalculateContestantsResultData() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut relativePoints: i32 = 0i32;
        let mut barLength: u32 = 0u32;
        let mut highestPoints: i16 = 0i16;
        let mut round2Points: i8 = 0i8;
        highestPoints = (((&raw mut gContestMonTotalPoints).cast::<i16>()).cast::<i16>()).read();
        {
            i = 1i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((highestPoints) as i32)
                        < ((((((&raw mut gContestMonTotalPoints).cast::<i16>()).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        highestPoints = ((((&raw mut gContestMonTotalPoints).cast::<i16>())
                            .cast::<i16>())
                        .wrapping_offset((i) as isize))
                        .read();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((highestPoints) as i32) < 0i32 {
            highestPoints =
                (((&raw mut gContestMonTotalPoints).cast::<i16>()).cast::<i16>()).read();
            {
                i = 1i32;
                'l3: loop {
                    if !(i < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        if ((highestPoints) as i32)
                            > ((((((&raw mut gContestMonTotalPoints).cast::<i16>()).cast::<i16>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            highestPoints = ((((&raw mut gContestMonTotalPoints).cast::<i16>())
                                .cast::<i16>())
                            .wrapping_offset((i) as isize))
                            .read();
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 4i32) {
                    break 'l5;
                }
                'l6: {
                    relativePoints = crate::c::div_i32(
                        ((((((&raw mut gContestMonRound1Points).cast::<i16>()).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            .wrapping_mul(1000i32),
                        (if ((highestPoints) as i32) < 0i32 {
                            ((highestPoints) as i32).wrapping_neg()
                        } else {
                            ((highestPoints) as i32)
                        }),
                    );
                    if crate::c::rem_i32(relativePoints, 10i32) > 4i32 {
                        relativePoints = (relativePoints).wrapping_add(10i32);
                    }
                    ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 20))
                    .cast::<i32>())
                    .write(crate::c::div_i32(relativePoints, 10i32));
                    relativePoints = crate::c::div_i32(
                        (if ((((((&raw mut gContestMonRound2Points).cast::<i16>()).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            < 0i32
                        {
                            ((((((&raw mut gContestMonRound2Points).cast::<i16>()).cast::<i16>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32)
                                .wrapping_neg()
                        } else {
                            ((((((&raw mut gContestMonRound2Points).cast::<i16>()).cast::<i16>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32)
                        })
                        .wrapping_mul(1000i32),
                        (if ((highestPoints) as i32) < 0i32 {
                            ((highestPoints) as i32).wrapping_neg()
                        } else {
                            ((highestPoints) as i32)
                        }),
                    );
                    if crate::c::rem_i32(relativePoints, 10i32) > 4i32 {
                        relativePoints = (relativePoints).wrapping_add(10i32);
                    }
                    ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 20))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .write(crate::c::div_i32(relativePoints, 10i32));
                    if ((((((&raw mut gContestMonRound2Points).cast::<i16>()).cast::<i16>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        < 0i32
                    {
                        ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                        .wrapping_add(16))
                        .write(1u8);
                    }
                    barLength = ((crate::c::div_i32(
                        (((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                        .cast::<i32>())
                        .read())
                        .wrapping_mul(22528i32),
                        100i32,
                    )) as u32);
                    if (barLength & 255u32) > 127u32 {
                        barLength = (barLength).wrapping_add(256u32);
                    }
                    ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 20))
                    .wrapping_add(8)
                    .cast::<u32>())
                    .write((barLength >> 8));
                    barLength = ((crate::c::div_i32(
                        (((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                        .wrapping_add(4)
                        .cast::<i32>())
                        .read())
                        .wrapping_mul(22528i32),
                        100i32,
                    )) as u32);
                    if (barLength & 255u32) > 127u32 {
                        barLength = (barLength).wrapping_add(256u32);
                    }
                    ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 20))
                    .wrapping_add(12)
                    .cast::<u32>())
                    .write((barLength >> 8));
                    ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 20))
                    .wrapping_add(17))
                    .write(GetNumPreliminaryPoints(((i) as u8), 1u8));
                    round2Points = GetNumRound2Points(((i) as u8), 1u8);
                    ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 20))
                    .wrapping_add(18))
                    .write(
                        ((if ((round2Points) as i32) < 0i32 {
                            ((round2Points) as i32).wrapping_neg()
                        } else {
                            ((round2Points) as i32)
                        }) as u8),
                    );
                    if ((((&raw mut gContestFinalStandings).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read())
                        != 0
                    {
                        let mut barLengthPreliminary: i16 =
                            ((((((((((&raw mut sContestResults)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                            .wrapping_add(8)
                            .cast::<u32>())
                            .read()) as i16);
                        let mut barLengthRound2: i16 = ((((((((((&raw mut sContestResults)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                        .wrapping_add(12)
                        .cast::<u32>())
                        .read()) as i16);
                        if (((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                        .wrapping_add(16))
                        .read())
                            != 0
                        {
                            barLengthRound2 =
                                ((((barLengthRound2) as i32).wrapping_mul((-1i32))) as i16);
                        }
                        if ((barLengthPreliminary) as i32).wrapping_add(((barLengthRound2) as i32))
                            == 88i32
                        {
                            if ((barLengthRound2) as i32) > 0i32 {
                                let __p1 = (((((((&raw mut sContestResults)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 20))
                                .wrapping_add(12)
                                .cast::<u32>();
                                (__p1).write(((__p1).read()).wrapping_sub(1));
                            } else {
                                if ((barLengthPreliminary) as i32) > 0i32 {
                                    let __p2 = (((((((&raw mut sContestResults)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 20))
                                    .wrapping_add(8)
                                    .cast::<u32>();
                                    (__p2).write(((__p2).read()).wrapping_sub(1));
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
pub(crate) unsafe extern "C" fn UpdateContestResultBars(isRound2: u8, numUpdates: u8) {
    unsafe {
        let mut isRound2 = isRound2;
        let mut numUpdates = numUpdates;
        let mut i: i32 = 0i32;
        let mut taskId: i32 = 0i32;
        let mut target: u32 = 0u32;
        let mut numIncreasing: u8 = 0u8;
        let mut numDecreasing: u8 = 0u8;
        if !((isRound2) != 0) {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        let mut numStars: u8 =
                            ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                            .wrapping_add(17))
                            .read();
                        if ((numUpdates) as i32) < ((numStars) as i32) {
                            FillBgTilemapBufferRect_Palette0(
                                1u8,
                                24755u16,
                                (((((19i32).wrapping_add(((numStars) as i32)))
                                    .wrapping_sub(((numUpdates) as i32)))
                                .wrapping_sub(1i32)) as u8),
                                ((((i).wrapping_mul(3i32)).wrapping_add(5i32)) as u8),
                                1u8,
                                1u8,
                            );
                            taskId = ((CreateTask(Some(Task_UpdateContestResultBar), 10u8)) as i32);
                            target = (crate::c::div_u32(
                                (((((((((&raw mut sContestResults)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 20))
                                .wrapping_add(8)
                                .cast::<u32>())
                                .read()
                                    << 16),
                                ((((((((((&raw mut sContestResults)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 20))
                                .wrapping_add(17))
                                .read()) as u32),
                            ))
                            .wrapping_mul(((((numUpdates) as i32).wrapping_add(1i32)) as u32));
                            if (target & 65535u32) > 32767u32 {
                                target = (target).wrapping_add(65536u32);
                            }
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset((taskId) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(((i) as i16));
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset((taskId) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .write(((target >> 16) as i16));
                            let __p1 =
                                (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(20);
                            (__p1).write(((__p1).read()).wrapping_add(1));
                            numIncreasing = (numIncreasing).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        let mut numHearts: i8 = ((((((((((&raw mut sContestResults)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                        .wrapping_add(18))
                        .read()) as i8);
                        let mut tile: u32 = ((if (((((((((&raw mut sContestResults)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                        .wrapping_add(16))
                        .read())
                            != 0
                        {
                            24741i32
                        } else {
                            24739i32
                        }) as u32);
                        if ((numUpdates) as i32) < ((numHearts) as i32) {
                            FillBgTilemapBufferRect_Palette0(
                                1u8,
                                ((tile) as u16),
                                (((((19i32).wrapping_add(((numHearts) as i32)))
                                    .wrapping_sub(((numUpdates) as i32)))
                                .wrapping_sub(1i32)) as u8),
                                ((((i).wrapping_mul(3i32)).wrapping_add(6i32)) as u8),
                                1u8,
                                1u8,
                            );
                            taskId = ((CreateTask(Some(Task_UpdateContestResultBar), 10u8)) as i32);
                            target = (crate::c::div_u32(
                                (((((((((&raw mut sContestResults)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 20))
                                .wrapping_add(12)
                                .cast::<u32>())
                                .read()
                                    << 16),
                                ((((((((((&raw mut sContestResults)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 20))
                                .wrapping_add(18))
                                .read()) as u32),
                            ))
                            .wrapping_mul(((((numUpdates) as i32).wrapping_add(1i32)) as u32));
                            if (target & 65535u32) > 32767u32 {
                                target = (target).wrapping_add(65536u32);
                            }
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset((taskId) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(((i) as i16));
                            if (((((((((&raw mut sContestResults)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                            .wrapping_add(16))
                            .read())
                                != 0
                            {
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset((taskId) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .write(1i16);
                                numDecreasing = (numDecreasing).wrapping_add(1);
                            } else {
                                numIncreasing = (numIncreasing).wrapping_add(1);
                            }
                            if (((((((((&raw mut sContestResults)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                            .wrapping_add(16))
                            .read())
                                != 0
                            {
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset((taskId) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(
                                    ((((target >> 16).wrapping_neg()).wrapping_add(
                                        ((((((((&raw mut sContestResults)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 20))
                                        .wrapping_add(8)
                                        .cast::<u32>())
                                        .read(),
                                    )) as i16),
                                );
                            } else {
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset((taskId) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(
                                    (((target >> 16).wrapping_add(
                                        ((((((((&raw mut sContestResults)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 20))
                                        .wrapping_add(8)
                                        .cast::<u32>())
                                        .read(),
                                    )) as i16),
                                );
                            }
                            let __p2 =
                                (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(20);
                            (__p2).write(((__p2).read()).wrapping_add(1));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if (numDecreasing) != 0 {
            PlaySE(22u16);
        }
        if (numIncreasing) != 0 {
            PlaySE(21u16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateContestResultBar(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut minMaxReached: u32 = 0u32;
        let mut targetReached: u32 = 0u32;
        let mut monId: u8 = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        let mut target: i16 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read();
        let mut decreasing: i16 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read();
        if (decreasing) != 0 {
            if ((((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12))
            .cast::<i16>())
            .wrapping_offset(((monId) as i32) as isize))
            .read()) as i32)
                <= 0i32
            {
                minMaxReached = 1u32;
            }
        } else {
            if ((((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12))
            .cast::<i16>())
            .wrapping_offset(((monId) as i32) as isize))
            .read()) as i32)
                >= 88i32
            {
                minMaxReached = 1u32;
            }
        }
        if ((((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_add(12))
        .cast::<i16>())
        .wrapping_offset(((monId) as i32) as isize))
        .read()) as i32)
            == ((target) as i32)
        {
            targetReached = 1u32;
        }
        if !((targetReached) != 0) {
            if (minMaxReached) != 0 {
                ((((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(12))
                .cast::<i16>())
                .wrapping_offset(((monId) as i32) as isize))
                .write(target);
            } else {
                if (decreasing) != 0 {
                    let __p1 =
                        (((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12))
                        .cast::<i16>())
                        .wrapping_offset(((monId) as i32) as isize);
                    (__p1).write(((__p1).read()).wrapping_sub(1));
                } else {
                    let __p2 =
                        (((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12))
                        .cast::<i16>())
                        .wrapping_offset(((monId) as i32) as isize);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
            }
        }
        if (!((minMaxReached) != 0)) && (!((targetReached) != 0)) {
            let mut tileOffset: u8 = 0u8;
            let mut tileNum: u16 = 0u16;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 11i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((((((&raw mut sContestResults)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12))
                        .cast::<i16>())
                        .wrapping_offset(((monId) as i32) as isize))
                        .read()) as i32)
                            >= ((i).wrapping_add(1i32)).wrapping_mul(8i32)
                        {
                            tileOffset = 8u8;
                        } else {
                            if ((((((((((&raw mut sContestResults)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_add(12))
                            .cast::<i16>())
                            .wrapping_offset(((monId) as i32) as isize))
                            .read()) as i32)
                                >= (i).wrapping_mul(8i32)
                            {
                                tileOffset = ((crate::c::rem_i32(
                                    ((((((((((&raw mut sContestResults)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(12))
                                    .cast::<i16>())
                                    .wrapping_offset(((monId) as i32) as isize))
                                    .read()) as i32),
                                    8i32,
                                )) as u8);
                            } else {
                                tileOffset = 0u8;
                            }
                        }
                        if ((tileOffset) as i32) < 4i32 {
                            tileNum = (((20556i32).wrapping_add(((tileOffset) as i32))) as u16);
                        } else {
                            tileNum = (((20567i32).wrapping_add(((tileOffset) as i32))) as u16);
                        }
                        FillBgTilemapBufferRect_Palette0(
                            2u8,
                            tileNum,
                            (((i).wrapping_add(7i32)) as u8),
                            (((((monId) as i32).wrapping_mul(3i32)).wrapping_add(6i32)) as u8),
                            1u8,
                            1u8,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if (targetReached) != 0 {
            let __p3 = (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_add(20);
            (__p3).write(((__p3).read()).wrapping_sub(1));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AllocContestResults() {
    unsafe {
        ((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(32u32));
        ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
            .write(AllocZeroed(24u32));
        ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write(AllocZeroed(80u32));
        ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .write(AllocZeroed(2048u32));
        (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
            .cast::<*mut u8>())
        .write(AllocZeroed(2048u32));
        ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
            .cast::<*mut u8>())
        .wrapping_offset(1))
        .write(AllocZeroed(2048u32));
        ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
            .cast::<*mut u8>())
        .wrapping_offset(2))
        .write(AllocZeroed(2048u32));
        ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
            .cast::<*mut u8>())
        .wrapping_offset(3))
        .write(AllocZeroed(2048u32));
        ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .write(AllocZeroed(4096u32));
        AllocateMonSpritesGfx();
    }
}
pub(crate) unsafe extern "C" fn FreeContestResults() {
    unsafe {
        {
            Free(
                ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .cast::<*mut u8>())
                .read(),
            );
            (((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .cast::<*mut u8>())
                .wrapping_offset(1))
                .read(),
            );
            ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<*mut u8>())
            .wrapping_offset(1))
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .cast::<*mut u8>())
                .wrapping_offset(2))
                .read(),
            );
            ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<*mut u8>())
            .wrapping_offset(2))
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .cast::<*mut u8>())
                .wrapping_offset(3))
                .read(),
            );
            ((((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<*mut u8>())
            .wrapping_offset(3))
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sContestResults).cast::<u8>().cast::<*mut u8>())
                .write(core::ptr::null_mut());
        }
        FreeMonSpritesGfx();
    }
}
pub(crate) unsafe extern "C" fn AddContestTextPrinter(windowId: i32, str: *mut u8, x: i32) {
    unsafe {
        let mut windowId = windowId;
        let mut str = str;
        let mut x = x;
        let mut textPrinter = crate::ffi::Align4([0u8; 16]);
        (((&raw mut textPrinter).cast::<u8>()).cast::<*mut u8>()).write(str);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(4)).write(((windowId) as u8));
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(5)).write(7u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(6)).write(((x) as u8));
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(7)).write(2u8);
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(8)).write(((x) as u8));
        (((&raw mut textPrinter).cast::<u8>()).wrapping_add(9)).write(2u8);
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
            (1u8) as i32,
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
            (8u8) as i32,
        );
        AddTextPrinter((&raw mut textPrinter).cast::<u8>(), 0u8, None);
        PutWindowTilemap(((windowId) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryEnterContestMon() {
    unsafe {
        let mut eligibility: u8 =
            GetContestEntryEligibility(((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize * 100,
            ));
        if (eligibility) != 0 {
            SetContestants(
                ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as u8),
                ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as u8),
            );
            CalculateRound1Points(
                ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as u8),
            );
        }
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((eligibility) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasMonWonThisContestBefore() -> u16 {
    unsafe {
        let mut hasRankRibbon: u16 = 0u16;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize * 100,
        );
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                if GetMonData2(mon, 50i32)
                    > ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as u32)
                {
                    hasRankRibbon = 1u16;
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if GetMonData2(mon, 51i32)
                    > ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as u32)
                {
                    hasRankRibbon = 1u16;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if GetMonData2(mon, 52i32)
                    > ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as u32)
                {
                    hasRankRibbon = 1u16;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if GetMonData2(mon, 53i32)
                    > ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as u32)
                {
                    hasRankRibbon = 1u16;
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if GetMonData2(mon, 54i32)
                    > ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as u32)
                {
                    hasRankRibbon = 1u16;
                }
                break 'l1;
            }
        }
        return hasRankRibbon;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveMonContestRibbon() {
    unsafe {
        let mut ribbonData: u8 = 0u8;
        if (((((&raw mut gContestFinalStandings).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32) as isize,
        ))
        .read()) as i32)
            != 0i32
        {
            return;
        }
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                ribbonData = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ),
                    50i32,
                )) as u8);
                if (((ribbonData) as i32)
                    <= ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as i32))
                    && (((ribbonData) as i32) <= 3i32)
                {
                    ribbonData = (ribbonData).wrapping_add(1);
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        50i32,
                        &raw mut ribbonData,
                    );
                    if ((GetRibbonCount(((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ))) as i32)
                        > 4i32
                    {
                        TryPutSpotTheCutiesOnAir(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 100,
                            ),
                            50u8,
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ribbonData = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ),
                    51i32,
                )) as u8);
                if (((ribbonData) as i32)
                    <= ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as i32))
                    && (((ribbonData) as i32) <= 3i32)
                {
                    ribbonData = (ribbonData).wrapping_add(1);
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        51i32,
                        &raw mut ribbonData,
                    );
                    if ((GetRibbonCount(((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ))) as i32)
                        > 4i32
                    {
                        TryPutSpotTheCutiesOnAir(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 100,
                            ),
                            51u8,
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ribbonData = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ),
                    52i32,
                )) as u8);
                if (((ribbonData) as i32)
                    <= ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as i32))
                    && (((ribbonData) as i32) <= 3i32)
                {
                    ribbonData = (ribbonData).wrapping_add(1);
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        52i32,
                        &raw mut ribbonData,
                    );
                    if ((GetRibbonCount(((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ))) as i32)
                        > 4i32
                    {
                        TryPutSpotTheCutiesOnAir(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 100,
                            ),
                            52u8,
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ribbonData = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ),
                    53i32,
                )) as u8);
                if (((ribbonData) as i32)
                    <= ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as i32))
                    && (((ribbonData) as i32) <= 3i32)
                {
                    ribbonData = (ribbonData).wrapping_add(1);
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        53i32,
                        &raw mut ribbonData,
                    );
                    if ((GetRibbonCount(((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ))) as i32)
                        > 4i32
                    {
                        TryPutSpotTheCutiesOnAir(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 100,
                            ),
                            53u8,
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                ribbonData = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ),
                    54i32,
                )) as u8);
                if (((ribbonData) as i32)
                    <= ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as i32))
                    && (((ribbonData) as i32) <= 3i32)
                {
                    ribbonData = (ribbonData).wrapping_add(1);
                    SetMonData(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        54i32,
                        &raw mut ribbonData,
                    );
                    if ((GetRibbonCount(((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ))) as i32)
                        > 4i32
                    {
                        TryPutSpotTheCutiesOnAir(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 100,
                            ),
                            54u8,
                        );
                    }
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestantTrainerName() {
    unsafe {
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) as isize * 64,
            ))
            .wrapping_add(13))
            .cast::<u8>(),
        );
        ConvertInternationalContestantName((&raw mut gStringVar1).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestantMonNickname() {
    unsafe {
        StringCopy(
            (&raw mut gStringVar3).cast::<u8>(),
            ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) as isize * 64,
            ))
            .wrapping_add(2))
            .cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestMonConditionRanking() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut rank: u8 = 0u8;
        {
            i = 0u8;
            rank = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gContestMonRound1Points).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(
                            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)
                                as isize,
                        ))
                    .read()) as i32)
                        < ((((((&raw mut gContestMonRound1Points).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    {
                        rank = (rank).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(((rank) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestMonCondition() {
    unsafe {
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(
            ((((((&raw mut gContestMonRound1Points).cast::<i16>()).cast::<i16>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) as isize,
            ))
            .read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestWinnerId() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            {
                i = 0u8;
                'l1: loop {
                    if !((((i) as i32) < 4i32)
                        && ((((((&raw mut gContestFinalStandings).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != 0i32))
                    {
                        break 'l1;
                    }
                    'l2: {}
                    i = (i).wrapping_add(1);
                }
            }
        }
        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(((i) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestWinnerTrainerName() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            {
                i = 0u8;
                'l1: loop {
                    if !((((i) as i32) < 4i32)
                        && ((((((&raw mut gContestFinalStandings).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != 0i32))
                    {
                        break 'l1;
                    }
                    'l2: {}
                    i = (i).wrapping_add(1);
                }
            }
        }
        StringCopy(
            (&raw mut gStringVar3).cast::<u8>(),
            ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 64))
                .wrapping_add(13))
            .cast::<u8>(),
        );
        ConvertInternationalContestantName((&raw mut gStringVar3).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestWinnerMonName() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            {
                i = 0u8;
                'l1: loop {
                    if !((((i) as i32) < 4i32)
                        && ((((((&raw mut gContestFinalStandings).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != 0i32))
                    {
                        break 'l1;
                    }
                    'l2: {}
                    i = (i).wrapping_add(1);
                }
            }
        }
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(((i) as i32) as isize * 64))
                .wrapping_add(2))
            .cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_SetStartContestCallback() {
    unsafe {
        SetMainCallback2(Some(CB2_StartContest));
    }
}
pub(crate) unsafe extern "C" fn Task_StartContest(taskId: u8) {
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
            SetMainCallback2(Some(CB2_SetStartContestCallback));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartContest() {
    unsafe {
        LockPlayerFieldControls();
        CreateTask(Some(Task_StartContest), 10u8);
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestantMonSpecies() {
    unsafe {
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(
            ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) as isize * 64,
            ))
            .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_StartShowContestResults(taskId: u8) {
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
            SetMainCallback2(Some(CB2_StartShowContestResults));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowContestResults() {
    unsafe {
        LockPlayerFieldControls();
        CreateTask(Some(Task_StartShowContestResults), 10u8);
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestPlayerId() {
    unsafe {
        ((&raw mut gSpecialVar_0x8004).cast::<u16>())
            .write(((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestLinkTransfer(category: u8) {
    unsafe {
        let mut category = category;
        let mut newTaskId: u8 = 0u8;
        LockPlayerFieldControls();
        newTaskId = CreateTask(Some(Task_LinkContest_Init), 0u8);
        SetTaskFuncWithFollowupFunc(
            newTaskId,
            Some(Task_LinkContest_Init),
            Some(Task_StartCommunication),
        );
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((newTaskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(((category) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_StartCommunication(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 4i32) != 0 {
            CreateContestMonFromParty(((&raw mut gContestMonPartyIndex).cast::<u8>()).read());
            SetTaskFuncWithFollowupFunc(
                taskId,
                Some(Task_LinkContest_CommunicateMonsRS),
                Some(Task_StartCommunicateRngRS),
            );
        } else {
            CreateContestMonFromParty(((&raw mut gContestMonPartyIndex).cast::<u8>()).read());
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LinkContest_StartCommunicationEm));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartCommunicateRngRS(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateRngRS),
            Some(Task_StartCommunicateLeaderIdsRS),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_StartCommunicateLeaderIdsRS(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateLeaderIdsRS),
            Some(Task_StartCommunicateCategoryRS),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_StartCommunicateCategoryRS(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateCategoryRS),
            Some(Task_LinkContest_SetUpContestRS),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_SetUpContestRS(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut categories = crate::ffi::Align4([0u8; 4]);
        let mut leaderIds = crate::ffi::Align4([0u8; 4]);
        crate::c::memset((&raw mut categories).cast::<u8>(), 0i32, 4u32);
        crate::c::memset((&raw mut leaderIds).cast::<u8>(), 0i32, 4u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut categories).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                            .read()) as u8),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !((((i) as i32)
                    < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32))
                    && (((((&raw mut categories).cast::<u8>()).read()) as i32)
                        == (((((&raw mut categories).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)))
                {
                    break 'l3;
                }
                'l4: {}
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as i32) == ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32) {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(0u16);
        } else {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(1u16);
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32)
                    < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32))
                {
                    break 'l5;
                }
                'l6: {
                    (((&raw mut leaderIds).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                            .read()) as u8),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gContestLinkLeaderIndex).cast::<u8>()).write(LinkContest_GetLeaderIndex(
            (&raw mut leaderIds).cast::<u8>(),
        ));
        CalculateRound1Points(
            ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as u8),
        );
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateRound1Points),
            Some(Task_LinkContest_CalculateTurnOrderRS),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CalculateTurnOrderRS(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SortContestants(0u8);
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateTurnOrder),
            Some(Task_LinkContest_FinalizeConnection),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContest_GetLeaderIndex(ids: *mut u8) -> u8 {
    unsafe {
        let mut ids = ids;
        let mut i: i32 = 0i32;
        let mut leaderIdx: u8 = 0u8;
        {
            i = 1i32;
            'l1: loop {
                if !(i < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((ids).wrapping_offset(((leaderIdx) as i32) as isize)).read()) as i32)
                        < ((((ids).wrapping_offset((i) as isize)).read()) as i32)
                    {
                        leaderIdx = ((i) as u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return leaderIdx;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_FinalizeConnection(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32 {
            if (IsLinkTaskFinished()) != 0 {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_LinkContest_Disconnect));
            }
        } else {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        StringGet_Nickname(
                            ((((&raw mut gContestMons).cast::<u8>())
                                .wrapping_offset((i) as isize * 64))
                            .wrapping_add(2))
                            .cast::<u8>(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            DestroyTask(taskId);
            SetDynamicWarp(
                0i32,
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read(),
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read(),
                (-1i8),
            );
            UnlockPlayerFieldControls();
            ScriptContext_Enable();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_Disconnect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetCloseLinkCallback();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_LinkContest_WaitDisconnect));
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_WaitDisconnect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
            DestroyTask(taskId);
            UnlockPlayerFieldControls();
            ScriptContext_Enable();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContestTrainerGfxIds() {
    unsafe {
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(16))
        .write((((((&raw mut gContestMons).cast::<u8>()).wrapping_add(21)).read()) as u16));
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(17))
        .write(
            ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(64)).wrapping_add(21))
                .read()) as u16),
        );
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5020))
            .cast::<u16>())
        .wrapping_offset(18))
        .write(
            ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(128)).wrapping_add(21))
                .read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNpcContestantLocalId() {
    unsafe {
        let mut localId: u16 = 0u16;
        let mut contestant: u8 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8);
        'l1: {
            let __sw1 = ((contestant) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                localId = 3u16;
                break 'l1;
            }
            if __sw1 == 1i32 {
                localId = 4u16;
                break 'l1;
            }
            if __sw1 == 2i32 {
                localId = 5u16;
                break 'l1;
            }
            if !__matched {
                localId = 100u16;
                break 'l1;
            }
        }
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(localId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestTrainerAndMonNames() {
    unsafe {
        BufferContestantTrainerName();
        BufferContestantMonNickname();
        BufferContestantMonSpecies();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoesContestCategoryHaveMuseumPainting() {
    unsafe {
        let mut contestWinner: i32 = 0i32;
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                contestWinner = 8i32;
                break 'l1;
            }
            if __sw1 == 1i32 {
                contestWinner = 9i32;
                break 'l1;
            }
            if __sw1 == 2i32 {
                contestWinner = 10i32;
                break 'l1;
            }
            if __sw1 == 3i32 {
                contestWinner = 11i32;
                break 'l1;
            }
            if __sw1 == 4i32 || !__matched {
                contestWinner = 12i32;
                break 'l1;
            }
        }
        if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11920))
            .cast::<u8>())
        .wrapping_offset((contestWinner) as isize * 32))
        .wrapping_add(8)
        .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(0u16);
        } else {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(1u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveMuseumContestPainting() {
    unsafe {
        SaveContestWinner(255u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldReadyContestArtist() {
    unsafe {
        if (((((((&raw mut gContestFinalStandings).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32) as isize,
        ))
        .read()) as i32)
            == 0i32)
            && (((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as i32) == 3i32))
            && (((((((&raw mut gContestMonTotalPoints).cast::<i16>()).cast::<i16>())
                .wrapping_offset(
                    ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32) as isize,
                ))
            .read()) as i32)
                >= 800i32)
        {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPlayerMuseumPaintings() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut count: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11920))
                    .cast::<u8>())
                    .wrapping_offset(((8i32).wrapping_add(i)) as isize * 32))
                    .wrapping_add(8)
                    .cast::<u16>())
                    .read())
                        != 0
                    {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return count;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestantNamesAtRank() {
    unsafe {
        let mut conditions = crate::ffi::Align4([0u8; 8]);
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut condition: i16 = 0i16;
        let mut numAtCondition: i8 = 0i8;
        let mut contestantOffset: u8 = 0u8;
        let mut tieRank: u8 = 0u8;
        let mut rank: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut conditions).cast::<i16>()).wrapping_offset((i) as isize)).write(
                        ((((&raw mut gContestMonRound1Points).cast::<i16>()).cast::<i16>())
                            .wrapping_offset((i) as isize))
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
                                if (((((&raw mut conditions).cast::<i16>())
                                    .wrapping_offset(((j).wrapping_sub(1i32)) as isize))
                                .read()) as i32)
                                    < (((((&raw mut conditions).cast::<i16>())
                                        .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                {
                                    let mut temp: i32 = 0i32;
                                    {
                                        temp = (((((&raw mut conditions).cast::<i16>())
                                            .wrapping_offset((j) as isize))
                                        .read())
                                            as i32);
                                        (((&raw mut conditions).cast::<i16>())
                                            .wrapping_offset((j) as isize))
                                        .write(
                                            (((&raw mut conditions).cast::<i16>())
                                                .wrapping_offset(
                                                    ((j).wrapping_sub(1i32)) as isize,
                                                ))
                                            .read(),
                                        );
                                        (((&raw mut conditions).cast::<i16>())
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
        condition = (((&raw mut conditions).cast::<i16>()).wrapping_offset(
            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) as isize,
        ))
        .read();
        numAtCondition = 0i8;
        tieRank = 0u8;
        {
            i = 0i32;
            'l7: loop {
                if !(i < 4i32) {
                    break 'l7;
                }
                'l8: {
                    if (((((&raw mut conditions).cast::<i16>()).wrapping_offset((i) as isize))
                        .read()) as i32)
                        == ((condition) as i32)
                    {
                        numAtCondition = (numAtCondition).wrapping_add(1);
                        if i == ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) {
                            tieRank = ((numAtCondition) as u8);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l9: loop {
                if !(i < 4i32) {
                    break 'l9;
                }
                'l10: {
                    if (((((&raw mut conditions).cast::<i16>()).wrapping_offset((i) as isize))
                        .read()) as i32)
                        == ((condition) as i32)
                    {
                        break 'l9;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        rank = ((i) as u8);
        contestantOffset = tieRank;
        {
            i = 0i32;
            'l11: loop {
                if !(i < 4i32) {
                    break 'l11;
                }
                'l12: {
                    if ((condition) as i32)
                        == ((((((&raw mut gContestMonRound1Points).cast::<i16>()).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        if ((contestantOffset) as i32) == 1i32 {
                            break 'l11;
                        }
                        contestantOffset = (contestantOffset).wrapping_sub(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset((i) as isize * 64))
                .wrapping_add(2))
            .cast::<u8>(),
        );
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset((i) as isize * 64))
                .wrapping_add(13))
            .cast::<u8>(),
        );
        ConvertInternationalContestantName((&raw mut gStringVar2).cast::<u8>());
        if ((numAtCondition) as i32) == 1i32 {
            ((&raw mut gSpecialVar_0x8006).cast::<u16>()).write(((rank) as u16));
        } else {
            if ((tieRank) as i32) == ((numAtCondition) as i32) {
                ((&raw mut gSpecialVar_0x8006).cast::<u16>()).write(((rank) as u16));
            } else {
                ((&raw mut gSpecialVar_0x8006).cast::<u16>())
                    .write(((((rank) as i32).wrapping_add(4i32)) as u16));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ExitContestPainting() {
    unsafe {
        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowContestPainting() {
    unsafe {
        SetMainCallback2(Some(CB2_ContestPainting));
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(ExitContestPainting));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLinkContestPlayerGfx() {
    unsafe {
        let mut i: i32 = 0i32;
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 1i32) != 0 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        let mut version: i32 = (((((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset((i) as isize * 28))
                        .cast::<u16>())
                        .read()) as u8) as i32);
                        if (version == 2i32) || (version == 1i32) {
                            if ((((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(19))
                            .read()) as i32)
                                == 0i32
                            {
                                ((((&raw mut gContestMons).cast::<u8>())
                                    .wrapping_offset((i) as isize * 64))
                                .wrapping_add(21))
                                .write(235u8);
                            } else {
                                ((((&raw mut gContestMons).cast::<u8>())
                                    .wrapping_offset((i) as isize * 64))
                                .wrapping_add(21))
                                .write(236u8);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            VarSet(
                16400u16,
                (((((&raw mut gContestMons).cast::<u8>()).wrapping_add(21)).read()) as u16),
            );
            VarSet(
                16401u16,
                ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(64)).wrapping_add(21))
                    .read()) as u16),
            );
            VarSet(
                16402u16,
                ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(128)).wrapping_add(21))
                    .read()) as u16),
            );
            VarSet(
                16403u16,
                ((((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(192)).wrapping_add(21))
                    .read()) as u16),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadLinkContestPlayerPalettes() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut objectEventId: u8 = 0u8;
        let mut version: i32 = 0i32;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(12u8);
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 1i32) != 0 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        objectEventId = GetObjectEventIdByLocalIdAndMap(
                            ((((&raw const sContestantLocalIds_0).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .wrapping_add(1)
                            .cast::<i8>())
                            .read()) as u8),
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .cast::<i8>())
                            .read()) as u8),
                        );
                        sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gObjectEvents).cast::<u8>())
                                .wrapping_offset(((objectEventId) as i32) as isize * 36))
                            .wrapping_add(4))
                            .read()) as i32) as isize
                                * 68,
                        );
                        crate::c::bf_write(
                            (sprite).wrapping_add(5),
                            4,
                            4,
                            (((6i32).wrapping_add(i)) as u16) as i32,
                        );
                        version = (((((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset((i) as isize * 28))
                        .cast::<u16>())
                        .read()) as u8) as i32);
                        if (version == 2i32) || (version == 1i32) {
                            if ((((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(19))
                            .read()) as i32)
                                == 0i32
                            {
                                LoadPalette(
                                    (((&raw mut gObjectEventPal_RubySapphireBrendan)
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .cast::<u8>(),
                                    (((256i32)
                                        .wrapping_add(((6i32).wrapping_add(i)).wrapping_mul(16i32)))
                                        as u16),
                                    32u16,
                                );
                            } else {
                                LoadPalette(
                                    (((&raw mut gObjectEventPal_RubySapphireMay).cast::<u16>())
                                        .cast::<u16>())
                                    .cast::<u8>(),
                                    (((256i32)
                                        .wrapping_add(((6i32).wrapping_add(i)).wrapping_mul(16i32)))
                                        as u16),
                                    32u16,
                                );
                            }
                        } else {
                            if ((((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(19))
                            .read()) as i32)
                                == 0i32
                            {
                                LoadPalette(
                                    (((&raw mut gObjectEventPal_Brendan).cast::<u16>())
                                        .cast::<u16>())
                                    .cast::<u8>(),
                                    (((256i32)
                                        .wrapping_add(((6i32).wrapping_add(i)).wrapping_mul(16i32)))
                                        as u16),
                                    32u16,
                                );
                            } else {
                                LoadPalette(
                                    (((&raw mut gObjectEventPal_May).cast::<u16>()).cast::<u16>())
                                        .cast::<u8>(),
                                    (((256i32)
                                        .wrapping_add(((6i32).wrapping_add(i)).wrapping_mul(16i32)))
                                        as u16),
                                    32u16,
                                );
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveMonArtistRibbon() -> u8 {
    unsafe {
        let mut hasArtistRibbon: u8 = 0u8;
        hasArtistRibbon = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize * 100,
            ),
            70i32,
        )) as u8);
        if (((!((hasArtistRibbon) != 0))
            && ((((((&raw mut gContestFinalStandings).cast::<u8>()).wrapping_offset(
                ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                == 0i32))
            && (((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as i32) == 3i32))
            && (((((((&raw mut gContestMonTotalPoints).cast::<i16>()).cast::<i16>())
                .wrapping_offset(
                    ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32) as isize,
                ))
            .read()) as i32)
                >= 800i32)
        {
            hasArtistRibbon = 1u8;
            SetMonData(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                        * 100,
                ),
                70i32,
                &raw mut hasArtistRibbon,
            );
            if ((GetRibbonCount(((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize * 100,
            ))) as i32)
                > 4i32
            {
                TryPutSpotTheCutiesOnAir(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ),
                    70u8,
                );
            }
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsContestDebugActive() -> u8 {
    unsafe {
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowContestEntryMonPic() {
    unsafe {
        let mut palette: *mut u8 = core::ptr::null_mut();
        let mut personality: u32 = 0u32;
        let mut otId: u32 = 0u32;
        let mut species: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let mut left: u8 = 0u8;
        let mut top: u8 = 0u8;
        if ((FindTaskIdByFunc(Some(Task_ShowContestEntryMonPic))) as i32) == 255i32 {
            AllocateMonSpritesGfx();
            left = 10u8;
            top = 3u8;
            species = ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) as isize * 64,
            ))
            .cast::<u16>())
            .read();
            personality = ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) as isize * 64,
            ))
            .wrapping_add(56)
            .cast::<u32>())
            .read();
            otId = ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) as isize * 64,
            ))
            .wrapping_add(60)
            .cast::<u32>())
            .read();
            taskId = CreateTask(Some(Task_ShowContestEntryMonPic), 80u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((species) as i16));
            if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)
                == ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32)
            {
                HandleLoadSpecialPokePic_2(
                    ((&raw mut gMonFrontPicTable).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 8),
                    ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<*mut u8>())
                    .wrapping_offset(1))
                    .read(),
                    ((species) as i32),
                    personality,
                );
            } else {
                HandleLoadSpecialPokePic_DontHandleDeoxys(
                    ((&raw mut gMonFrontPicTable).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 8),
                    ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<*mut u8>())
                    .wrapping_offset(1))
                    .read(),
                    ((species) as i32),
                    personality,
                );
            }
            palette = GetMonSpritePalStructFromOtIdPersonality(species, otId, personality);
            LoadCompressedSpritePalette(palette);
            SetMultiuseSpriteTemplateToPokemon(species, 1u8);
            (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .write(((palette).wrapping_add(4).cast::<u16>()).read());
            spriteId = CreateSprite(
                (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
                ((((((left) as i32).wrapping_add(1i32)).wrapping_mul(8i32)).wrapping_add(32i32))
                    as i16),
                (((((top) as i32).wrapping_mul(8i32)).wrapping_add(40i32)) as i16),
                0u8,
            );
            if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 1i32) != 0 {
                if !((((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 4i32) != 0) {
                    DoMonFrontSpriteAnimation(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        species,
                        0u8,
                        0u8,
                    );
                }
            } else {
                DoMonFrontSpriteAnimation(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    species,
                    0u8,
                    0u8,
                );
            }
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((spriteId) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(((left) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(((top) as i16));
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                (0u16) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideContestEntryMonPic() {
    unsafe {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_ShowContestEntryMonPic));
        if ((taskId) as i32) != 255i32 {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            FreeMonSpritesGfx();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowContestEntryMonPic(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut sprite: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                    .write(((CreateWindowFromRect(10u8, 3u8, 8u8, 8u8)) as i16));
                SetStandardWindowBorderStyle(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as u8),
                    1u8,
                );
                let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                break 'l1;
            }
            if __sw1 == 3i32 {
                sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                        as isize
                        * 68,
                );
                FreeSpritePaletteByTag(GetSpritePaletteTagByPaletteNum(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as u8),
                ));
                if (crate::c::bf_read((sprite).wrapping_add(1), 0, 2, false) as u32) != 0 {
                    FreeOamMatrix(
                        ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
                    );
                }
                DestroySprite(sprite);
                let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                ClearToTransparentAndRemoveWindow(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as u8),
                );
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestMultiplayerId() {
    unsafe {
        if (((((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 1i32) != 0)
            && (((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32) == 4i32))
            && (!((((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 2i32) != 0))
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((GetMultiplayerId()) as u16));
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(4u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateContestRand() {
    unsafe {
        let mut random: u16 = 0u16;
        let mut result: *mut u16 = core::ptr::null_mut();
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 1i32) != 0 {
            ((&raw mut gContestRngValue).cast::<u32>()).write(
                ((1103515245u32).wrapping_mul(((&raw mut gContestRngValue).cast::<u32>()).read()))
                    .wrapping_add(24691u32),
            );
            random = ((((&raw mut gContestRngValue).cast::<u32>()).read() >> 16) as u16);
            result = (&raw mut gSpecialVar_Result).cast::<u16>();
        } else {
            result = (&raw mut gSpecialVar_Result).cast::<u16>();
            random = Random();
        }
        (result).write(((crate::c::rem_i32(((random) as i32), (((result).read()) as i32))) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestRand() -> u16 {
    unsafe {
        ((&raw mut gContestRngValue).cast::<u32>()).write(
            ((1103515245u32).wrapping_mul(((&raw mut gContestRngValue).cast::<u32>()).read()))
                .wrapping_add(24691u32),
        );
        return ((((&raw mut gContestRngValue).cast::<u32>()).read() >> 16) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContestWaitForConnection() -> u8 {
    unsafe {
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 2i32) != 0 {
            CreateTask(Some(Task_LinkContestWaitForConnection), 5u8);
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
pub(crate) unsafe extern "C" fn Task_LinkContestWaitForConnection(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                if (IsLinkTaskFinished()) != 0 {
                    SetLinkStandbyCallback();
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                if ((IsLinkTaskFinished()) as i32) == 1i32 {
                    ScriptContext_Enable();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContestTryShowWirelessIndicator() {
    unsafe {
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 2i32) != 0 {
            if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(8u8, 8u8);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContestTryHideWirelessIndicator() {
    unsafe {
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 2i32) != 0 {
            if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                DestroyWirelessStatusIndicatorSprite();
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsContestWithRSPlayer() -> u8 {
    unsafe {
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 4i32) != 0 {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearLinkContestFlags() {
    unsafe {
        ((&raw mut gLinkContestFlags).cast::<u8>()).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWirelessContest() -> u8 {
    unsafe {
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 2i32) != 0 {
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
