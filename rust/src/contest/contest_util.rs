//! Translated from `src/contest_util.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sResultsTextWindow_Pal sResultsTextWindow_Gfx sMiscBlank_Pal sOamData_ResultsTextWindow sSpriteTemplate_ResultsTextWindow sSpriteSheets_ResultsTextWindow sSpritePalette_ResultsTextWindow sOamData_Confetti sSpriteTemplate_Confetti sSpriteSheet_Confetti sSpritePalette_Confetti sBgTemplates sWindowTemplates sOamData_WirelessIndicatorWindow sSpriteTemplate_WirelessIndicatorWindow sSpriteSheet_WirelessIndicatorWindow sContestLinkTextColors sContestantLocalIds.0

/// `struct ContestResults`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ContestResults {
    pub data: *mut ContestResultsInternal,
    pub monResults: *mut CArray<ContestMonResults, 4>,
    pub unusedBg: *mut u8,
    pub tilemapBuffers: CArray<*mut u8, 4>,
    pub unused: *mut u8,
}

unsafe impl Sync for ContestResults {}

/// `struct ContestResultsInternal`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct ContestResultsInternal {
    pub slidingTextBoxSpriteId: u8,
    pub linkTextBoxSpriteId: u8,
    pub showResultsTaskId: u8,
    pub highlightWinnerTaskId: u8,
    pub slidingTextBoxState: u8,
    pub numStandingsPrinted: u8,
    pub winnerMonSlidingState: u8,
    pub confettiCount: u8,
    pub winnerMonSpriteId: u8,
    pub destroyConfetti: u8,
    pub pointsFlashing: u8,
    pub barLength: CArray<i16, 4>,
    pub numBarsUpdating: u8,
}

unsafe impl Sync for ContestResultsInternal {}

/// `struct ContestMonResults`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ContestMonResults {
    pub relativePreliminaryPoints: i32,
    pub relativeRound2Points: i32,
    pub barLengthPreliminary: u32,
    pub barLengthRound2: u32,
    pub lostPoints: u8,
    pub numStars: u8,
    pub numHearts: u8,
}

unsafe impl Sync for ContestMonResults {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<ContestResults>() == 32);
    assert!(offset_of!(ContestResults, data) == 0);
    assert!(offset_of!(ContestResults, monResults) == 4);
    assert!(offset_of!(ContestResults, unusedBg) == 8);
    assert!(offset_of!(ContestResults, tilemapBuffers) == 12);
    assert!(offset_of!(ContestResults, unused) == 28);
    assert!(size_of::<ContestResultsInternal>() == 24);
    assert!(offset_of!(ContestResultsInternal, slidingTextBoxSpriteId) == 0);
    assert!(offset_of!(ContestResultsInternal, linkTextBoxSpriteId) == 1);
    assert!(offset_of!(ContestResultsInternal, showResultsTaskId) == 2);
    assert!(offset_of!(ContestResultsInternal, highlightWinnerTaskId) == 3);
    assert!(offset_of!(ContestResultsInternal, slidingTextBoxState) == 4);
    assert!(offset_of!(ContestResultsInternal, numStandingsPrinted) == 5);
    assert!(offset_of!(ContestResultsInternal, winnerMonSlidingState) == 6);
    assert!(offset_of!(ContestResultsInternal, confettiCount) == 7);
    assert!(offset_of!(ContestResultsInternal, winnerMonSpriteId) == 8);
    assert!(offset_of!(ContestResultsInternal, destroyConfetti) == 9);
    assert!(offset_of!(ContestResultsInternal, pointsFlashing) == 10);
    assert!(offset_of!(ContestResultsInternal, barLength) == 12);
    assert!(offset_of!(ContestResultsInternal, numBarsUpdating) == 20);
    assert!(size_of::<ContestMonResults>() == 20);
    assert!(offset_of!(ContestMonResults, relativePreliminaryPoints) == 0);
    assert!(offset_of!(ContestMonResults, relativeRound2Points) == 4);
    assert!(offset_of!(ContestMonResults, barLengthPreliminary) == 8);
    assert!(offset_of!(ContestMonResults, barLengthRound2) == 12);
    assert!(offset_of!(ContestMonResults, lostPoints) == 16);
    assert!(offset_of!(ContestMonResults, numStars) == 17);
    assert!(offset_of!(ContestMonResults, numHearts) == 18);
};

const BAR_SEGMENT_LENGTH: i32 = 8;
const MAX_BAR_LENGTH: i32 = 88;
const NUM_BAR_SEGMENTS: i32 = 11;
const SLIDING_MON_ENTERED: u8 = 1;
const SLIDING_MON_EXITED: u8 = 2;
const SLIDING_TEXT_ARRIVED: u8 = 2;
const SLIDING_TEXT_ENTERING: u8 = 1;
const SLIDING_TEXT_EXITING: u8 = 3;
const SLIDING_TEXT_OFFSCREEN: u8 = 0;
const TEXT_BOX_X: i16 = 272;
const TEXT_BOX_Y: u16 = 144;

static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::contest_util::sBgTemplates).cast());
static sContestLinkTextColors: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::contest_util::sContestLinkTextColors).cast());
static sContestantLocalIds_0: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::contest_util::sContestantLocalIds_0).cast());
static sResultsTextWindow_Gfx: Table<CArray<u8, 256>> =
    Table((&raw const crate::data::contest_util::sResultsTextWindow_Gfx).cast());
static sResultsTextWindow_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::contest_util::sResultsTextWindow_Pal).cast());
static sSpritePalette_Confetti: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::contest_util::sSpritePalette_Confetti).cast());
static sSpritePalette_ResultsTextWindow: Table<SpritePalette> =
    Table((&raw const crate::data::contest_util::sSpritePalette_ResultsTextWindow).cast());
static sSpriteSheet_Confetti: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::contest_util::sSpriteSheet_Confetti).cast());
static sSpriteSheet_WirelessIndicatorWindow: Table<SpriteSheet> =
    Table((&raw const crate::data::contest_util::sSpriteSheet_WirelessIndicatorWindow).cast());
static sSpriteSheets_ResultsTextWindow: Table<CArray<SpriteSheet, 8>> =
    Table((&raw const crate::data::contest_util::sSpriteSheets_ResultsTextWindow).cast());
static sSpriteTemplate_Confetti: Table<SpriteTemplate> =
    Table((&raw const crate::data::contest_util::sSpriteTemplate_Confetti).cast());
static sSpriteTemplate_ResultsTextWindow: Table<SpriteTemplate> =
    Table((&raw const crate::data::contest_util::sSpriteTemplate_ResultsTextWindow).cast());
static sSpriteTemplate_WirelessIndicatorWindow: Table<SpriteTemplate> =
    Table((&raw const crate::data::contest_util::sSpriteTemplate_WirelessIndicatorWindow).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 5>> =
    Table((&raw const crate::data::contest_util::sWindowTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sContestResults: *mut ContestResults = null_mut();

unsafe extern "C" {
    static mut gBattle_BG0_X: u16;
    static mut gBattle_BG0_Y: u16;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattle_BG2_X: u16;
    static mut gBattle_BG2_Y: u16;
    static mut gBattle_BG3_X: u16;
    static mut gBattle_BG3_Y: u16;
    static mut gBattle_WIN0H: u16;
    static mut gBattle_WIN0V: u16;
    static mut gBattle_WIN1H: u16;
    static mut gBattle_WIN1V: u16;
    static mut gContestFinalStandings: CArray<u8, 4>;
    static mut gContestLinkLeaderIndex: u8;
    static mut gContestMonPartyIndex: u8;
    static mut gContestMonRound1Points: CArray<i16, 4>;
    static mut gContestMonRound2Points: CArray<i16, 4>;
    static mut gContestMonTotalPoints: CArray<i16, 4>;
    static mut gContestMons: CArray<ContestPokemon, 4>;
    static mut gContestPlayerMonIndex: u8;
    static gContestResultsTitle_Beauty_Tilemap: CArray<u16, 0>;
    static gContestResultsTitle_Cool_Tilemap: CArray<u16, 0>;
    static gContestResultsTitle_Cute_Tilemap: CArray<u16, 0>;
    static gContestResultsTitle_Hyper_Tilemap: CArray<u16, 0>;
    static gContestResultsTitle_Link_Tilemap: CArray<u16, 0>;
    static gContestResultsTitle_Master_Tilemap: CArray<u16, 0>;
    static gContestResultsTitle_Normal_Tilemap: CArray<u16, 0>;
    static gContestResultsTitle_Smart_Tilemap: CArray<u16, 0>;
    static gContestResultsTitle_Super_Tilemap: CArray<u16, 0>;
    static gContestResultsTitle_Tilemap: CArray<u16, 0>;
    static gContestResultsTitle_Tough_Tilemap: CArray<u16, 0>;
    static gContestResults_Bg_Tilemap: CArray<u32, 0>;
    static gContestResults_Gfx: CArray<u32, 0>;
    static gContestResults_Interface_Tilemap: CArray<u32, 0>;
    static gContestResults_Pal: CArray<u32, 0>;
    static gContestResults_WinnerBanner_Tilemap: CArray<u32, 0>;
    static mut gContestRngValue: u32;
    static mut gCurContestWinnerIsForArtist: u8;
    static mut gCurContestWinnerSaveIdx: u8;
    static mut gDisplayedStringBattle: CArray<u8, 300>;
    static mut gLinkContestFlags: u8;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gMain: Main;
    static gMonFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    static gMonIconPaletteIndices: CArray<u8, 0>;
    static gMonIconPalettes: CArray<CArray<u16, 16>, 0>;
    static mut gMonSpritesGfxPtr: *mut MonSpritesGfx;
    static mut gMultiuseSpriteTemplate: SpriteTemplate;
    static mut gNumLinkContestPlayers: u8;
    static gObjectEventPal_Brendan: CArray<u16, 0>;
    static gObjectEventPal_May: CArray<u16, 0>;
    static gObjectEventPal_RubySapphireBrendan: CArray<u16, 0>;
    static gObjectEventPal_RubySapphireMay: CArray<u16, 0>;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_ContestCategory: u16;
    static mut gSpecialVar_ContestRank: u16;
    static mut gSpecialVar_Result: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gTasks: CArray<Task, 0>;
    static gText_AnnouncingResults: CArray<u8, 0>;
    static gText_ColorDarkGray: CArray<u8, 0>;
    static gText_CommunicationStandby: CArray<u8, 0>;
    static gText_ContestantsMonWon: CArray<u8, 0>;
    static gText_PreliminaryResults: CArray<u8, 0>;
    static gText_Round2Results: CArray<u8, 0>;
    static gText_Slash: CArray<u8, 0>;
    static mut gWirelessStatusIndicatorSpriteId: u8;
    fn AddTextPrinter(
        a0: *mut TextPrinterTemplate,
        a1: u8,
        a2: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocZeroed(a0: u32) -> *mut c_void;
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
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateContestMonFromParty(a0: u8);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWindowFromRect(a0: u8, a1: u8, a2: u8, a3: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DestroyWirelessStatusIndicatorSprite();
    fn DoMonFrontSpriteAnimation(a0: *mut Sprite, a1: u16, a2: u8, a3: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeMonSpritesGfx();
    fn FreeOamMatrix(a0: u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn GetContestEntryEligibility(a0: *mut Pokemon) -> u8;
    fn GetContestWinnerSaveIdx(a0: u8, a1: u8) -> u8;
    fn GetIconSpecies(a0: u16, a1: u32) -> u16;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonIconPtr(a0: u16, a1: u32, a2: u32) -> *mut u8;
    fn GetMonSpritePalStructFromOtIdPersonality(
        a0: u16,
        a1: u32,
        a2: u32,
    ) -> *mut CompressedSpritePalette;
    fn GetMultiplayerId() -> u8;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetRibbonCount(a0: *mut Pokemon) -> u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetSpritePaletteTagByPaletteNum(a0: u8) -> u16;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn HandleLoadSpecialPokePic_2(
        a0: *mut CompressedSpriteSheet,
        a1: *mut c_void,
        a2: i32,
        a3: u32,
    );
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(
        a0: *mut CompressedSpriteSheet,
        a1: *mut c_void,
        a2: i32,
        a3: u32,
    );
    fn IncrementGameStat(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn InterviewAfter();
    fn InterviewBefore();
    fn IsLinkTaskFinished() -> u8;
    fn LZDecompressVram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn LockPlayerFieldControls();
    fn PlayBGM(a0: u16);
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn RequestDma3Copy(a0: *mut c_void, a1: *mut c_void, a2: u16, a3: u8) -> i16;
    fn RequestDma3Fill(a0: i32, a1: *mut c_void, a2: u16, a3: u8) -> i16;
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
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetCloseLinkCallback();
    fn SetContestants(a0: u8, a1: u8);
    fn SetContinueGameWarpStatusToDynamicWarp();
    fn SetDynamicWarp(a0: i32, a1: i8, a2: i8, a3: i8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
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
    fn SpriteCallbackDummy(a0: *mut Sprite);
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
    fn TryPutSpotTheCutiesOnAir(a0: *mut Pokemon, a1: u8);
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
    let mut i: i32 = 0;
    SetGpuReg(REG_OFFSET_DISPCNT, DISPCNT_OBJ_1D_MAP);
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
    i = 0;
    while i < 4 {
        SetBgTilemapBuffer(i as u8, (*sContestResults).tilemapBuffers[i] as *mut c_void);
        i += 1;
    }
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    SetGpuReg(REG_OFFSET_MOSAIC, 0);
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, 16174);
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    SetGpuReg(REG_OFFSET_WIN1H, 0);
    SetGpuReg(REG_OFFSET_WIN1V, 0);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    SetGpuReg(REG_OFFSET_BG3HOFS, 0);
    SetGpuReg(REG_OFFSET_BG3VOFS, 0);
    SetGpuRegBits(REG_OFFSET_DISPCNT, 65280);
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    gBattle_BG2_Y = 0;
    gBattle_BG3_X = 0;
    gBattle_BG3_Y = 0;
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    gBattle_WIN1H = 0;
    gBattle_WIN1V = 0;
}
pub(crate) unsafe extern "C" fn LoadContestResultsBgGfx() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut numStars: i8 = 0;
    let mut round2Points: i8 = 0;
    let mut tile1: u16 = 0;
    let mut tile2: u16 = 0;
    LZDecompressVram(
        gContestResults_Gfx.as_ptr().cast_mut(),
        0x6000000 as usize as *mut c_void,
    );
    CopyToBgTilemapBuffer(
        3,
        gContestResults_Bg_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyToBgTilemapBuffer(
        2,
        gContestResults_Interface_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyToBgTilemapBuffer(
        0,
        gContestResults_WinnerBanner_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    LoadContestResultsTitleBarTilemaps();
    LoadCompressedPalette(
        gContestResults_Pal.as_ptr().cast_mut(),
        BG_PLTT_OFFSET,
        BG_PLTT_SIZE,
    );
    LoadPalette(
        sResultsTextWindow_Pal.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
    i = 0;
    while i < CONTESTANT_COUNT {
        numStars = GetNumPreliminaryPoints(i as u8, TRUE) as i8;
        round2Points = GetNumRound2Points(i as u8, TRUE);
        j = 0;
        while j < 10 {
            tile1 = 0x60B2;
            if j < numStars as i32 {
                tile1 += 2;
            }
            if j < (if round2Points < 0 {
                -(round2Points as i32)
            } else {
                round2Points as i32
            }) {
                tile2 = 0x60A4;
                if round2Points < 0 {
                    tile2 += 2;
                }
            } else {
                tile2 = 0x60A2;
            }
            FillBgTilemapBufferRect_Palette0(1, tile1, j as u8 + 19, i as u8 * 3 + 5, 1, 1);
            FillBgTilemapBufferRect_Palette0(1, tile2, j as u8 + 19, i as u8 * 3 + 6, 1, 1);
            j += 1;
        }
        i += 1;
    }
    CopyBgTilemapBufferToVram(0);
    CopyBgTilemapBufferToVram(1);
    CopyBgTilemapBufferToVram(2);
    CopyBgTilemapBufferToVram(3);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
}
pub(crate) unsafe extern "C" fn LoadContestMonName(monIndex: u8) {
    let mut mon: *mut ContestPokemon = &raw mut gContestMons[monIndex];
    let mut str: *mut u8 = gDisplayedStringBattle.as_mut_ptr();
    if monIndex == gContestPlayerMonIndex {
        str = StringCopy(
            gDisplayedStringBattle.as_mut_ptr(),
            gText_ColorDarkGray.as_ptr().cast_mut(),
        );
    }
    StringCopy(str, (*mon).nickname.as_mut_ptr());
    AddContestTextPrinter(monIndex as i32, gDisplayedStringBattle.as_mut_ptr(), 0);
    StringCopy(str, gText_Slash.as_ptr().cast_mut());
    StringAppend(str, (*mon).trainerName.as_mut_ptr());
    AddContestTextPrinter(monIndex as i32, gDisplayedStringBattle.as_mut_ptr(), 50);
}
pub(crate) unsafe extern "C" fn LoadAllContestMonNames() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        LoadContestMonName(i as u8);
        i += 1;
    }
    CopyBgTilemapBufferToVram(1);
}
pub(crate) unsafe extern "C" fn CB2_StartShowContestResults() {
    gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
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
    LoadAllContestMonIcons(0, TRUE);
    LoadAllContestMonNames();
    memset((*sContestResults).data as *mut u8, 0, 24);
    memset((*sContestResults).monResults as *mut u8, 0, 80);
    CreateResultsTextWindowSprites();
    TryCreateWirelessSprites();
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
    (*(*sContestResults).data).showResultsTaskId = CreateTask(Some(Task_ShowContestResults), 5);
    SetMainCallback2(Some(CB2_ShowContestResults));
    gBattle_WIN1H = DISPLAY_WIDTH;
    gBattle_WIN1V = 32928;
    CreateTask(Some(Task_SlideContestResultsBg), 20);
    CalculateContestantsResultData();
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
        gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
    } else {
        PlayBGM(MUS_CONTEST_RESULTS);
    }
    SetVBlankCallback(Some(VBlankCB_ShowContestResults));
}
pub(crate) unsafe extern "C" fn CB2_ShowContestResults() {
    AnimateSprites();
    BuildOamBuffer();
    RunTasks();
    UpdatePaletteFade();
    CopyBgTilemapBufferToVram(1);
    CopyBgTilemapBufferToVram(2);
}
pub(crate) unsafe extern "C" fn VBlankCB_ShowContestResults() {
    SetGpuReg(REG_OFFSET_BG0HOFS, gBattle_BG0_X);
    SetGpuReg(REG_OFFSET_BG0VOFS, gBattle_BG0_Y);
    SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
    SetGpuReg(REG_OFFSET_BG2HOFS, gBattle_BG2_X);
    SetGpuReg(REG_OFFSET_BG2VOFS, gBattle_BG2_Y);
    SetGpuReg(REG_OFFSET_BG3HOFS, gBattle_BG3_X);
    SetGpuReg(REG_OFFSET_BG3VOFS, gBattle_BG3_Y);
    SetGpuReg(REG_OFFSET_WIN0H, gBattle_WIN0H);
    SetGpuReg(REG_OFFSET_WIN0V, gBattle_WIN0V);
    SetGpuReg(REG_OFFSET_WIN1H, gBattle_WIN1H);
    SetGpuReg(REG_OFFSET_WIN1V, gBattle_WIN1V);
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe extern "C" fn Task_ShowContestResults(taskId: u8) {
    let mut var: u16 = 0;
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        'l1: {
            match gTasks[taskId].data[0] {
                0 => {
                    SaveLinkContestResults();
                    if gContestFinalStandings[gContestPlayerMonIndex] == 0 {
                        IncrementGameStat(GAME_STAT_WON_LINK_CONTEST);
                        gSpecialVar_0x8005 = TVSHOW_CONTEST_LIVE_UPDATES as u16;
                        InterviewBefore();
                        if gSpecialVar_Result != TRUE as u16 {
                            InterviewAfter();
                        }
                    }
                    TryGainNewFanFromCounter(FANCOUNTER_FINISHED_CONTEST);
                    SaveContestWinner(gSpecialVar_ContestRank as u8);
                    SaveContestWinner(CONTEST_SAVE_FOR_ARTIST as u8);
                    gCurContestWinnerIsForArtist = TRUE;
                    gCurContestWinnerSaveIdx =
                        GetContestWinnerSaveIdx(CONTEST_SAVE_FOR_ARTIST as u8, FALSE);
                    var = VarGet(VAR_CONTEST_HALL_STATE);
                    VarSet(VAR_CONTEST_HALL_STATE, 0);
                    SetContinueGameWarpStatusToDynamicWarp();
                    TrySavingData(SAVE_LINK);
                    ClearContinueGameWarpStatus2();
                    VarSet(VAR_CONTEST_HALL_STATE, var);
                    gTasks[taskId].data[0] += 1;
                }
                1 => {
                    gTasks[taskId].data[0] += 1;
                    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS == 0 {
                        gTasks[taskId].data[0] = 100;
                    }
                }
                2 => {
                    if IsLinkTaskFinished() != 0 {
                        SetLinkStandbyCallback();
                        gTasks[taskId].data[0] += 1;
                    }
                    return;
                }
                3 => {
                    if IsLinkTaskFinished() == TRUE {
                        PlayBGM(MUS_CONTEST_RESULTS);
                        gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
                        gTasks[taskId].data[0] += 1;
                        break 'l1;
                    }
                    return;
                }
                _ => {}
            }
        }
    }
    if gPaletteFade.active() == 0 {
        gTasks[taskId].data[0] = 0;
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
            ShowLinkResultsTextBox(gText_CommunicationStandby.as_ptr().cast_mut());
            gTasks[taskId].func = Some(Task_WaitForLinkPartnersBeforeResults);
        } else {
            IncrementGameStat(GAME_STAT_ENTERED_CONTEST);
            if gContestFinalStandings[gContestPlayerMonIndex] == 0 {
                IncrementGameStat(GAME_STAT_WON_CONTEST);
            }
            SaveContestWinner(gSpecialVar_ContestRank as u8);
            SaveContestWinner(CONTEST_SAVE_FOR_ARTIST as u8);
            gCurContestWinnerIsForArtist = TRUE;
            gCurContestWinnerSaveIdx =
                GetContestWinnerSaveIdx(CONTEST_SAVE_FOR_ARTIST as u8, FALSE);
            TryGainNewFanFromCounter(FANCOUNTER_FINISHED_CONTEST);
            gTasks[taskId].func = Some(Task_AnnouncePreliminaryResults);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForLinkPartnersBeforeResults(taskId: u8) {
    if gReceivedRemoteLinkPlayers != 0 {
        CreateTask(Some(Task_CommunicateMonIdxsForResults), 0);
        gTasks[taskId].func = Some(TaskDummy);
    }
}
pub(crate) unsafe extern "C" fn Task_CommunicateMonIdxsForResults(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateMonIdxs),
        Some(Task_WaitForLinkPartnerMonIdxs),
    );
}
pub(crate) unsafe extern "C" fn Task_WaitForLinkPartnerMonIdxs(taskId: u8) {
    if IsLinkTaskFinished() != 0 {
        DestroyTask(taskId);
        gTasks[(*(*sContestResults).data).showResultsTaskId].func =
            Some(Task_AnnouncePreliminaryResults);
        HideLinkResultsTextBox();
    }
}
pub(crate) unsafe extern "C" fn Task_AnnouncePreliminaryResults(taskId: u8) {
    let mut x: i16 = 0;
    if gTasks[taskId].data[0] == 0 {
        CreateTask(Some(Task_FlashStarsAndHearts), 20);
        x = DrawResultsTextWindow(
            gText_AnnouncingResults.as_ptr().cast_mut(),
            (*(*sContestResults).data).slidingTextBoxSpriteId,
        ) as i16;
        StartTextBoxSlideIn(x, TEXT_BOX_Y, 120, 1088);
        gTasks[taskId].data[0] += 1;
    } else if gTasks[taskId].data[0] == 1 {
        if (*(*sContestResults).data).slidingTextBoxState == SLIDING_TEXT_OFFSCREEN {
            gTasks[taskId].data[1] = 0;
            gTasks[taskId].data[0] += 1;
        }
    } else if gTasks[taskId].data[0] == 2 {
        if ({
            gTasks[taskId].data[1] += 1;
            gTasks[taskId].data[1]
        }) == 21
        {
            gTasks[taskId].data[1] = 0;
            gTasks[taskId].data[0] += 1;
        }
    } else if gTasks[taskId].data[0] == 3 {
        x = DrawResultsTextWindow(
            gText_PreliminaryResults.as_ptr().cast_mut(),
            (*(*sContestResults).data).slidingTextBoxSpriteId,
        ) as i16;
        StartTextBoxSlideIn(x, TEXT_BOX_Y, 65535, 1088);
        gTasks[taskId].data[0] += 1;
    } else if gTasks[taskId].data[0] == 4 {
        if (*(*sContestResults).data).slidingTextBoxState == SLIDING_TEXT_ARRIVED {
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].func = Some(Task_ShowPreliminaryResults);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowPreliminaryResults(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            if (*(*sContestResults).data).pointsFlashing == 0 {
                UpdateContestResultBars(
                    FALSE,
                    ({
                        let t1 = gTasks[taskId].data[2];
                        gTasks[taskId].data[2] += 1;
                        t1
                    }) as u8,
                );
                if (*(*sContestResults).data).numBarsUpdating == 0 {
                    gTasks[taskId].data[0] = 2;
                } else {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        1 => {
            if (*(*sContestResults).data).numBarsUpdating == 0 {
                gTasks[taskId].data[0] = 0;
            }
        }
        2 => {
            StartTextBoxSlideOut(1088);
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].data[2] = 0;
            gTasks[taskId].func = Some(Task_AnnounceRound2Results);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_AnnounceRound2Results(taskId: u8) {
    let mut x: i16 = 0;
    if (*(*sContestResults).data).slidingTextBoxState == SLIDING_TEXT_OFFSCREEN {
        if ({
            gTasks[taskId].data[1] += 1;
            gTasks[taskId].data[1]
        }) == 21
        {
            gTasks[taskId].data[1] = 0;
            x = DrawResultsTextWindow(
                gText_Round2Results.as_ptr().cast_mut(),
                (*(*sContestResults).data).slidingTextBoxSpriteId,
            ) as i16;
            StartTextBoxSlideIn(x, TEXT_BOX_Y, 65535, 1088);
        }
    } else if (*(*sContestResults).data).slidingTextBoxState == SLIDING_TEXT_ARRIVED {
        gTasks[taskId].func = Some(Task_ShowRound2Results);
    }
}
pub(crate) unsafe extern "C" fn Task_ShowRound2Results(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            if (*(*sContestResults).data).pointsFlashing == 0 {
                UpdateContestResultBars(
                    TRUE,
                    ({
                        let t1 = gTasks[taskId].data[2];
                        gTasks[taskId].data[2] += 1;
                        t1
                    }) as u8,
                );
                if (*(*sContestResults).data).numBarsUpdating == 0 {
                    gTasks[taskId].data[0] = 2;
                } else {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        1 => {
            if (*(*sContestResults).data).numBarsUpdating == 0 {
                gTasks[taskId].data[0] = 0;
            }
        }
        2 => {
            StartTextBoxSlideOut(1088);
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].func = Some(Task_AnnounceWinner);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_AnnounceWinner(taskId: u8) {
    let mut i: i32 = 0;
    match gTasks[taskId].data[0] {
        0 => {
            if (*(*sContestResults).data).slidingTextBoxState == SLIDING_TEXT_OFFSCREEN {
                gTasks[taskId].data[0] += 1;
            }
        }
        1 => {
            if ({
                gTasks[taskId].data[1] += 1;
                gTasks[taskId].data[1]
            }) == 31
            {
                gTasks[taskId].data[1] = 0;
                gTasks[taskId].data[0] += 1;
            }
        }
        2 => {
            i = 0;
            while i < CONTESTANT_COUNT {
                let mut newTaskId: u8 = CreateTask(Some(Task_DrawFinalStandingNumber), 10);
                gTasks[newTaskId].data[0] = gContestFinalStandings[i] as i16;
                gTasks[newTaskId].data[1] = i as i16;
                i += 1;
            }
            gTasks[taskId].data[0] += 1;
        }
        3 => {
            if (*(*sContestResults).data).numStandingsPrinted == CONTESTANT_COUNT as u8 {
                if ({
                    gTasks[taskId].data[1] += 1;
                    gTasks[taskId].data[1]
                }) == 31
                {
                    gTasks[taskId].data[1] = 0;
                    CreateTask(Some(Task_StartHighlightWinnersBox), 10);
                    gTasks[taskId].data[0] += 1;
                    i = 0;
                    while i < 4 && gContestFinalStandings[i] != 0 {
                        i += 1;
                    }
                    BounceMonIconInBox(i as u8, 14);
                }
            }
        }
        4 => {
            if ({
                gTasks[taskId].data[1] += 1;
                gTasks[taskId].data[1]
            }) == 21
            {
                let mut winnerTextBuffer: CArray<u8, 100> = zeroed();
                let mut x: i16 = 0;
                gTasks[taskId].data[1] = 0;
                i = 0;
                while i < 4 && gContestFinalStandings[i] != 0 {
                    i += 1;
                }
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    gContestMons[i].trainerName.as_mut_ptr(),
                );
                ConvertInternationalContestantName(gStringVar1.as_mut_ptr());
                StringCopy(
                    gStringVar2.as_mut_ptr(),
                    gContestMons[i].nickname.as_mut_ptr(),
                );
                StringExpandPlaceholders(
                    winnerTextBuffer.as_mut_ptr(),
                    gText_ContestantsMonWon.as_ptr().cast_mut(),
                );
                x = DrawResultsTextWindow(
                    winnerTextBuffer.as_mut_ptr(),
                    (*(*sContestResults).data).slidingTextBoxSpriteId,
                ) as i16;
                StartTextBoxSlideIn(x, TEXT_BOX_Y, 65535, 1088);
                gTasks[taskId].data[0] += 1;
            }
        }
        5 => {
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].func = Some(Task_ShowWinnerMonBanner);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ShowWinnerMonBanner(taskId: u8) {
    let mut i: i32 = 0;
    let mut spriteId: u8 = 0;
    let mut species: u16 = 0;
    let mut otId: u32 = 0;
    let mut personality: u32 = 0;
    let mut pokePal: *mut CompressedSpritePalette = null_mut();
    match gTasks[taskId].data[0] {
        0 => {
            gBattle_WIN0H = DISPLAY_WIDTH;
            gBattle_WIN0V = 20560;
            i = 0;
            while i < 4 && gContestFinalStandings[i] != 0 {
                i += 1;
            }
            species = gContestMons[i].species;
            personality = gContestMons[i].personality;
            otId = gContestMons[i].otId;
            if i == gContestPlayerMonIndex as i32 {
                HandleLoadSpecialPokePic_2(
                    (&raw const gMonFrontPicTable[species]).cast_mut(),
                    (*gMonSpritesGfxPtr).sprites.ptr[1],
                    species as i32,
                    personality,
                );
            } else {
                HandleLoadSpecialPokePic_DontHandleDeoxys(
                    (&raw const gMonFrontPicTable[species]).cast_mut(),
                    (*gMonSpritesGfxPtr).sprites.ptr[1],
                    species as i32,
                    personality,
                );
            }
            pokePal = GetMonSpritePalStructFromOtIdPersonality(species, otId, personality);
            LoadCompressedSpritePalette(pokePal);
            SetMultiuseSpriteTemplateToPokemon(species, B_POSITION_OPPONENT_LEFT);
            gMultiuseSpriteTemplate.paletteTag = (*pokePal).tag;
            spriteId = CreateSprite(&raw mut gMultiuseSpriteTemplate, 272, 80, 10);
            gSprites[spriteId].data[1] = species as i16;
            gSprites[spriteId].oam.set_priority(0);
            gSprites[spriteId].callback = Some(SpriteCB_WinnerMonSlideIn);
            (*(*sContestResults).data).winnerMonSpriteId = spriteId;
            LoadCompressedSpriteSheet((&raw const *sSpriteSheet_Confetti).cast_mut());
            LoadCompressedSpritePalette((&raw const *sSpritePalette_Confetti).cast_mut());
            CreateTask(Some(Task_CreateConfetti), 10);
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            if ({
                gTasks[taskId].data[3] += 1;
                gTasks[taskId].data[3]
            }) == 1
            {
                let mut counter: u8 = 0;
                gTasks[taskId].data[3] = 0;
                gTasks[taskId].data[2] += 2;
                if gTasks[taskId].data[2] > 32 {
                    gTasks[taskId].data[2] = 32;
                }
                counter = gTasks[taskId].data[2] as u8;
                gBattle_WIN0V = (80 - counter as u16) << 8 | 80 + counter as u16;
                if counter == 32 {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        2 => {
            if (*(*sContestResults).data).winnerMonSlidingState == SLIDING_MON_ENTERED {
                gTasks[taskId].data[0] += 1;
            }
        }
        3 => {
            if ({
                gTasks[taskId].data[1] += 1;
                gTasks[taskId].data[1]
            }) == 121
            {
                gTasks[taskId].data[1] = 0;
                gSprites[(*(*sContestResults).data).winnerMonSpriteId].callback =
                    Some(SpriteCB_WinnerMonSlideOut);
                gTasks[taskId].data[0] += 1;
            }
        }
        4 => {
            if (*(*sContestResults).data).winnerMonSlidingState == SLIDING_MON_EXITED {
                let mut top: u8 = (gBattle_WIN0V >> 8) as u8;
                top += 2;
                if top > 80 {
                    top = 80;
                }
                gBattle_WIN0V = (top as u16) << 8 | DISPLAY_HEIGHT - top as u16;
                if top == 80 {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        5 => {
            if (*(*sContestResults).data).winnerMonSlidingState == SLIDING_MON_EXITED {
                (*(*sContestResults).data).destroyConfetti = TRUE;
                gTasks[taskId].data[0] = 0;
                gTasks[taskId].func = Some(Task_SetSeenWinnerMon);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_SetSeenWinnerMon(taskId: u8) {
    let mut i: i32 = 0;
    let mut nationalDexNum: i32 = 0;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK == 0 {
            i = 0;
            while i < CONTESTANT_COUNT {
                nationalDexNum = SpeciesToNationalPokedexNum(gContestMons[i].species) as i32;
                GetSetPokedexFlag(nationalDexNum as u16, FLAG_SET_SEEN);
                i += 1;
            }
        }
        gTasks[taskId].data[10] = 0;
        gTasks[taskId].func = Some(Task_TryDisconnectLinkPartners);
    }
}
pub(crate) unsafe extern "C" fn Task_TryDisconnectLinkPartners(taskId: u8) {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        if gTasks[taskId].data[10] == 0 {
            ShowLinkResultsTextBox(gText_CommunicationStandby.as_ptr().cast_mut());
            SetCloseLinkCallback();
            gTasks[taskId].func = Some(Task_WaitForLinkPartnersDisconnect);
        }
    } else {
        gTasks[taskId].func = Some(Task_TrySetContestInterviewData);
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForLinkPartnersDisconnect(taskId: u8) {
    if gReceivedRemoteLinkPlayers == 0 {
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
            DestroyWirelessStatusIndicatorSprite();
        }
        HideLinkResultsTextBox();
        gTasks[taskId].func = Some(Task_TrySetContestInterviewData);
    }
}
pub(crate) unsafe extern "C" fn Task_TrySetContestInterviewData(taskId: u8) {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK == 0 {
        BravoTrainerPokemonProfile_BeforeInterview2(gContestFinalStandings[gContestPlayerMonIndex]);
    }
    BeginHardwarePaletteFade(0xFF, 0, 0, 16, 0);
    gTasks[taskId].func = Some(Task_EndShowContestResults);
}
pub(crate) unsafe extern "C" fn Task_EndShowContestResults(taskId: u8) {
    if gPaletteFade.active() == 0 {
        if gTasks[taskId].data[1] == 0 {
            DestroyTask((*(*sContestResults).data).highlightWinnerTaskId);
            BlendPalettes(PALETTES_BG, 16, 0);
            gTasks[taskId].data[1] += 1;
        } else if gTasks[taskId].data[1] == 1 {
            BlendPalettes(PALETTES_OBJECTS, 16, 0);
            gTasks[taskId].data[1] += 1;
        } else {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            DestroyTask(taskId);
            FreeAllWindowBuffers();
            SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
            FreeContestResults();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SlideContestResultsBg(taskId: u8) {
    gBattle_BG3_X += 2;
    gBattle_BG3_Y += 1;
    if gBattle_BG3_X > 255 {
        gBattle_BG3_X -= 255;
    }
    if gBattle_BG3_Y > 255 {
        gBattle_BG3_Y -= 255;
    }
}
pub(crate) unsafe extern "C" fn Task_FlashStarsAndHearts(taskId: u8) {
    if ({
        gTasks[taskId].data[0] += 1;
        gTasks[taskId].data[0]
    }) == 2
    {
        gTasks[taskId].data[0] = 0;
        if gTasks[taskId].data[2] == 0 {
            gTasks[taskId].data[1] += 1;
        } else {
            gTasks[taskId].data[1] -= 1;
        }
        if gTasks[taskId].data[1] == 16 {
            gTasks[taskId].data[2] = TRUE as i16;
        } else if gTasks[taskId].data[1] == 0 {
            gTasks[taskId].data[2] = FALSE as i16;
        }
        BlendPalette(107, 1, gTasks[taskId].data[1] as u8, 11998);
        BlendPalette(104, 1, gTasks[taskId].data[1] as u8, 32767);
        BlendPalette(110, 1, gTasks[taskId].data[1] as u8, 30654);
    }
    if gTasks[taskId].data[1] == 0 {
        (*(*sContestResults).data).pointsFlashing = FALSE;
    } else {
        (*(*sContestResults).data).pointsFlashing = TRUE;
    }
}
pub(crate) unsafe extern "C" fn LoadContestMonIcon(
    species: u16,
    monIndex: u8,
    srcOffset: u8,
    useDmaNow: u8,
    personality: u32,
) {
    let mut iconPtr: *mut u8 = null_mut();
    let mut var0: u16 = 0;
    let mut var1: u16 = 0;
    let mut frameNum: u16 = 0;
    if monIndex == gContestPlayerMonIndex {
        frameNum = 1;
    } else {
        frameNum = 0;
    }
    iconPtr = GetMonIconPtr(species, personality, frameNum as u32);
    iconPtr = iconPtr.at(srcOffset as i32 * 0x200 + 0x80);
    if useDmaNow != 0 {
        RequestDma3Copy(
            iconPtr as *mut c_void,
            (0x6004000 as usize as *mut c_void as *mut u8).at(monIndex as i32 * 0x200)
                as *mut c_void,
            0x180,
            1,
        );
        var0 = monIndex as u16 + 10 << 12;
        var1 = monIndex as u16 * 0x10 + 0x200;
        WriteSequenceToBgTilemapBuffer(1, var1 | var0, 3, monIndex * 3 + 4, 4, 3, 17, 1);
    } else {
        RequestDma3Copy(
            iconPtr as *mut c_void,
            (0x6004000 as usize as *mut c_void as *mut u8).at(monIndex as i32 * 0x200)
                as *mut c_void,
            0x180,
            1,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadAllContestMonIcons(srcOffset: u8, useDmaNow: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        LoadContestMonIcon(
            gContestMons[i].species,
            i as u8,
            srcOffset,
            useDmaNow,
            gContestMons[i].personality,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn LoadAllContestMonIconPalettes() {
    let mut i: i32 = 0;
    let mut species: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        species = gContestMons[i].species as i32;
        LoadPalette(
            gMonIconPalettes[gMonIconPaletteIndices[GetIconSpecies(species as u16, 0)]]
                .as_ptr()
                .cast_mut() as *mut c_void,
            0x000 + (10 + i as u16) * 16,
            32,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn TryCreateWirelessSprites() {
    let mut sheet: u16 = 0;
    let mut spriteId: u8 = 0;
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
        LoadWirelessStatusIndicatorSpriteGfx();
        CreateWirelessStatusIndicatorSprite(8, 8);
        gSprites[gWirelessStatusIndicatorSpriteId].subpriority = 1;
        sheet = LoadSpriteSheet((&raw const *sSpriteSheet_WirelessIndicatorWindow).cast_mut());
        RequestDma3Fill(
            -1,
            (0x6010000 as usize as *mut c_void as *mut u8).at(sheet as i32 * 0x20) as *mut c_void,
            0x80,
            1,
        );
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_WirelessIndicatorWindow).cast_mut(),
            8,
            8,
            0,
        );
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_WINDOW);
    }
}
pub(crate) unsafe extern "C" fn DrawResultsTextWindow(text: *mut u8, spriteId: u8) -> i32 {
    let mut windowId: u16 = 0;
    let mut tileWidth: i32 = 0;
    let mut strWidth: i32 = 0;
    let mut spriteTilePtrs: CArray<*mut u8, 4> = zeroed();
    let mut dst: *mut u8 = null_mut();
    let mut windowTemplate: WindowTemplate = zeroed();
    memset(&raw mut windowTemplate as *mut u8, 0, 8);
    windowTemplate.width = DISPLAY_TILE_WIDTH;
    windowTemplate.height = 2;
    windowId = AddWindow(&raw mut windowTemplate);
    FillWindowPixelBuffer(windowId as u8, 17);
    strWidth = GetStringWidth(FONT_NORMAL, text, 0);
    tileWidth = (strWidth + 9) / 8;
    if tileWidth > DISPLAY_TILE_WIDTH as i32 {
        tileWidth = DISPLAY_TILE_WIDTH as i32;
    }
    AddTextPrinterParameterized3(
        windowId as u8,
        FONT_NORMAL,
        ((tileWidth * 8 - strWidth) / 2) as u8,
        1,
        sContestLinkTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        text,
    );
    {
        let mut i: i32 = 0;
        let mut sprite: *mut Sprite = null_mut();
        let mut src: *mut u8 = null_mut();
        let mut windowTilesPtr: *mut u8 = null_mut();
        windowTilesPtr = GetWindowAttribute(windowId as u8, WINDOW_TILE_DATA) as usize as *mut u8;
        src = sResultsTextWindow_Gfx.as_ptr().cast_mut();
        sprite = &raw mut gSprites[spriteId];
        spriteTilePtrs[0] = ((*sprite).oam.tileNum() as i32 * 32 + OBJ_VRAM0) as usize as *mut u8;
        i = 1;
        while i < 4 {
            spriteTilePtrs[i] = (gSprites[(*sprite).data[i - 1]].oam.tileNum() as i32 * 32
                + OBJ_VRAM0) as usize as *mut c_void as *mut u8;
            i += 1;
        }
        i = 0;
        while i < 4 {
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        spriteTilePtrs[i] as *mut c_void,
                        0x5000100,
                    );
                }
            }
            i += 1;
        }
        dst = spriteTilePtrs[0];
        CpuSet(src as *mut c_void, dst as *mut c_void, 0x4000008);
        CpuSet(
            src.at(128) as *mut c_void,
            dst.at(256) as *mut c_void,
            0x4000008,
        );
        CpuSet(
            src.at(128) as *mut c_void,
            dst.at(512) as *mut c_void,
            0x4000008,
        );
        CpuSet(
            src.at(64) as *mut c_void,
            dst.at(768) as *mut c_void,
            0x4000008,
        );
        i = 0;
        while i < tileWidth {
            dst = spriteTilePtrs[(i + 1) / 8].at((i + 1) % 8 * 32);
            CpuSet(src.at(192) as *mut c_void, dst as *mut c_void, 0x4000008);
            CpuSet(
                windowTilesPtr as *mut c_void,
                dst.at(256) as *mut c_void,
                0x4000008,
            );
            CpuSet(
                windowTilesPtr.at(960) as *mut c_void,
                dst.at(512) as *mut c_void,
                0x4000008,
            );
            CpuSet(
                src.at(224) as *mut c_void,
                dst.at(768) as *mut c_void,
                0x4000008,
            );
            windowTilesPtr = windowTilesPtr.at(32);
            i += 1;
        }
        dst = spriteTilePtrs[(i + 1) / 8].at((i + 1) % 8 * 32);
        CpuSet(src.at(32) as *mut c_void, dst as *mut c_void, 0x4000008);
        CpuSet(
            src.at(160) as *mut c_void,
            dst.at(256) as *mut c_void,
            0x4000008,
        );
        CpuSet(
            src.at(160) as *mut c_void,
            dst.at(512) as *mut c_void,
            0x4000008,
        );
        CpuSet(
            src.at(96) as *mut c_void,
            dst.at(768) as *mut c_void,
            0x4000008,
        );
    }
    RemoveWindow(windowId as u8);
    return (DISPLAY_WIDTH as i32 - (tileWidth + 2) * 8) / 2;
}
pub(crate) unsafe extern "C" fn CreateResultsTextWindowSprites() {
    let mut i: i32 = 0;
    let mut template: SpriteTemplate = zeroed();
    let mut spriteIds: CArray<u8, 8> = zeroed();
    template = *sSpriteTemplate_ResultsTextWindow;
    i = 0;
    while i < 8 {
        LoadSpriteSheet((&raw const sSpriteSheets_ResultsTextWindow[i]).cast_mut());
        i += 1;
    }
    LoadSpritePalette((&raw const *sSpritePalette_ResultsTextWindow).cast_mut());
    i = 0;
    while i < 8 {
        spriteIds[i] = CreateSprite(&raw mut template, TEXT_BOX_X, TEXT_BOX_Y as i16, 10);
        template.tileTag += 1;
        i += 1;
    }
    gSprites[spriteIds[0]].data[0] = spriteIds[1] as i16;
    gSprites[spriteIds[0]].data[1] = spriteIds[2] as i16;
    gSprites[spriteIds[0]].data[2] = spriteIds[3] as i16;
    gSprites[spriteIds[4]].data[0] = spriteIds[5] as i16;
    gSprites[spriteIds[4]].data[1] = spriteIds[6] as i16;
    gSprites[spriteIds[4]].data[2] = spriteIds[7] as i16;
    (*(*sContestResults).data).slidingTextBoxSpriteId = spriteIds[0];
    (*(*sContestResults).data).slidingTextBoxState = SLIDING_TEXT_OFFSCREEN;
    (*(*sContestResults).data).linkTextBoxSpriteId = spriteIds[4];
    HideLinkResultsTextBox();
}
pub(crate) unsafe extern "C" fn StartTextBoxSlideIn(
    x: i16,
    y: u16,
    slideOutTimer: u16,
    slideIncrement: u16,
) {
    let mut sprite: *mut Sprite =
        &raw mut gSprites[(*(*sContestResults).data).slidingTextBoxSpriteId];
    (*sprite).x = TEXT_BOX_X;
    (*sprite).y = y as i16;
    (*sprite).x2 = 0;
    (*sprite).y2 = 0;
    (*sprite).data[4] = x + 32;
    (*sprite).data[5] = slideOutTimer as i16;
    (*sprite).data[6] = slideIncrement as i16;
    (*sprite).data[7] = 0;
    (*sprite).callback = Some(SpriteCB_TextBoxSlideIn);
    (*(*sContestResults).data).slidingTextBoxState = SLIDING_TEXT_ENTERING;
}
pub(crate) unsafe extern "C" fn StartTextBoxSlideOut(slideIncrement: u16) {
    let mut sprite: *mut Sprite =
        &raw mut gSprites[(*(*sContestResults).data).slidingTextBoxSpriteId];
    (*sprite).x += (*sprite).x2;
    (*sprite).y += (*sprite).y2;
    (*sprite).y2 = 0;
    (*sprite).x2 = 0;
    (*sprite).data[6] = slideIncrement as i16;
    (*sprite).data[7] = 0;
    (*sprite).callback = Some(SpriteCB_TextBoxSlideOut);
    (*(*sContestResults).data).slidingTextBoxState = SLIDING_TEXT_EXITING;
}
pub(crate) unsafe extern "C" fn EndTextBoxSlideOut(sprite: *mut Sprite) {
    (*sprite).x = TEXT_BOX_X;
    (*sprite).y = TEXT_BOX_Y as i16;
    (*sprite).y2 = 0;
    (*sprite).x2 = 0;
    (*sprite).callback = Some(SpriteCallbackDummy);
    (*(*sContestResults).data).slidingTextBoxState = SLIDING_TEXT_OFFSCREEN;
}
pub(crate) unsafe extern "C" fn SpriteCB_TextBoxSlideIn(sprite: *mut Sprite) {
    let mut i: i32 = 0;
    let mut delta: i16 = (*sprite).data[7] + (*sprite).data[6];
    (*sprite).x -= delta >> 8;
    (*sprite).data[7] += (*sprite).data[6];
    (*sprite).data[7] &= 0xFF;
    if (*sprite).x < (*sprite).data[4] {
        (*sprite).x = (*sprite).data[4];
    }
    i = 0;
    while i < 3 {
        let mut sprite2: *mut Sprite = &raw mut gSprites[(*sprite).data[i]];
        (*sprite2).x = (*sprite).x + (*sprite).x2 + (i as i16 + 1) * 64;
        i += 1;
    }
    if (*sprite).x == (*sprite).data[4] {
        (*sprite).callback = Some(SpriteCB_EndTextBoxSlideIn);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_EndTextBoxSlideIn(sprite: *mut Sprite) {
    (*(*sContestResults).data).slidingTextBoxState = SLIDING_TEXT_ARRIVED;
    if (*sprite).data[5] as u16 != 0xFFFF {
        if ({
            (*sprite).data[5] -= 1;
            (*sprite).data[5]
        }) == -1
        {
            StartTextBoxSlideOut((*sprite).data[6] as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TextBoxSlideOut(sprite: *mut Sprite) {
    let mut i: i32 = 0;
    let mut delta: i16 = 0;
    delta = (*sprite).data[7] + (*sprite).data[6];
    (*sprite).x -= delta >> 8;
    (*sprite).data[7] += (*sprite).data[6];
    (*sprite).data[7] &= 0xFF;
    i = 0;
    while i < 3 {
        let mut sprite2: *mut Sprite = &raw mut gSprites[(*sprite).data[i]];
        (*sprite2).x = (*sprite).x + (*sprite).x2 + (i as i16 + 1) * 64;
        i += 1;
    }
    if ((*sprite).x as i32 + (*sprite).x2 as i32) < -224 {
        EndTextBoxSlideOut(sprite);
    }
}
pub(crate) unsafe extern "C" fn ShowLinkResultsTextBox(text: *mut u8) {
    let mut i: i32 = 0;
    let mut x: u16 = 0;
    let mut sprite: *mut Sprite = null_mut();
    x = DrawResultsTextWindow(text, (*(*sContestResults).data).linkTextBoxSpriteId) as u16;
    sprite = &raw mut gSprites[(*(*sContestResults).data).linkTextBoxSpriteId];
    (*sprite).x = x as i16 + 32;
    (*sprite).y = 80;
    (*sprite).set_invisible(FALSE as u16);
    i = 0;
    while i < 3 {
        gSprites[(*sprite).data[i]].x = (*sprite).x + (*sprite).x2 + (i as i16 + 1) * 64;
        gSprites[(*sprite).data[i]].y = (*sprite).y;
        gSprites[(*sprite).data[i]].set_invisible(FALSE as u16);
        i += 1;
    }
    gBattle_WIN0H = DISPLAY_WIDTH;
    gBattle_WIN0V = (*sprite).y as u16 - 16 << 8 | (*sprite).y as u16 + 16;
    SetGpuReg(REG_OFFSET_WININ, 16190);
}
pub(crate) unsafe extern "C" fn HideLinkResultsTextBox() {
    let mut i: i32 = 0;
    let mut sprite: *mut Sprite = null_mut();
    sprite = &raw mut gSprites[(*(*sContestResults).data).linkTextBoxSpriteId];
    (*sprite).set_invisible(TRUE as u16);
    i = 0;
    while i < 3 {
        gSprites[(*sprite).data[i]].set_invisible(TRUE as u16);
        i += 1;
    }
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    SetGpuReg(REG_OFFSET_WIN0H, gBattle_WIN0H);
    SetGpuReg(REG_OFFSET_WIN0V, gBattle_WIN0V);
    SetGpuReg(REG_OFFSET_WININ, 16191);
}
pub(crate) unsafe extern "C" fn LoadContestResultsTitleBarTilemaps() {
    let mut palette: u8 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    x = 5;
    y = 1;
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        CopyToBgTilemapBufferRect(
            2,
            gContestResultsTitle_Link_Tilemap.as_ptr().cast_mut() as *mut c_void,
            5,
            1,
            5,
            2,
        );
        x = 10;
    } else if gSpecialVar_ContestRank == CONTEST_RANK_NORMAL as u16 {
        CopyToBgTilemapBufferRect(
            2,
            gContestResultsTitle_Normal_Tilemap.as_ptr().cast_mut() as *mut c_void,
            5,
            1,
            10,
            2,
        );
        x = 15;
    } else if gSpecialVar_ContestRank == CONTEST_RANK_SUPER as u16 {
        CopyToBgTilemapBufferRect(
            2,
            gContestResultsTitle_Super_Tilemap.as_ptr().cast_mut() as *mut c_void,
            5,
            1,
            10,
            2,
        );
        x = 15;
    } else if gSpecialVar_ContestRank == CONTEST_RANK_HYPER as u16 {
        CopyToBgTilemapBufferRect(
            2,
            gContestResultsTitle_Hyper_Tilemap.as_ptr().cast_mut() as *mut c_void,
            5,
            1,
            10,
            2,
        );
        x = 15;
    } else {
        CopyToBgTilemapBufferRect(
            2,
            gContestResultsTitle_Master_Tilemap.as_ptr().cast_mut() as *mut c_void,
            5,
            1,
            10,
            2,
        );
        x = 15;
    }
    if gSpecialVar_ContestCategory == CONTEST_CATEGORY_COOL as u16 {
        palette = 0;
        CopyToBgTilemapBufferRect(
            2,
            gContestResultsTitle_Cool_Tilemap.as_ptr().cast_mut() as *mut c_void,
            x as u8,
            y as u8,
            5,
            2,
        );
    } else if gSpecialVar_ContestCategory == CONTEST_CATEGORY_BEAUTY as u16 {
        palette = 1;
        CopyToBgTilemapBufferRect(
            2,
            gContestResultsTitle_Beauty_Tilemap.as_ptr().cast_mut() as *mut c_void,
            x as u8,
            y as u8,
            5,
            2,
        );
    } else if gSpecialVar_ContestCategory == CONTEST_CATEGORY_CUTE as u16 {
        palette = 2;
        CopyToBgTilemapBufferRect(
            2,
            gContestResultsTitle_Cute_Tilemap.as_ptr().cast_mut() as *mut c_void,
            x as u8,
            y as u8,
            5,
            2,
        );
    } else if gSpecialVar_ContestCategory == CONTEST_CATEGORY_SMART as u16 {
        palette = 3;
        CopyToBgTilemapBufferRect(
            2,
            gContestResultsTitle_Smart_Tilemap.as_ptr().cast_mut() as *mut c_void,
            x as u8,
            y as u8,
            5,
            2,
        );
    } else {
        palette = 4;
        CopyToBgTilemapBufferRect(
            2,
            gContestResultsTitle_Tough_Tilemap.as_ptr().cast_mut() as *mut c_void,
            x as u8,
            y as u8,
            5,
            2,
        );
    }
    x += 5;
    CopyToBgTilemapBufferRect(
        2,
        gContestResultsTitle_Tilemap.as_ptr().cast_mut() as *mut c_void,
        x as u8,
        y as u8,
        6,
        2,
    );
    CopyToBgTilemapBufferRect_ChangePalette(
        2,
        (*sContestResults).tilemapBuffers[2] as *mut c_void,
        0,
        0,
        32,
        4,
        palette,
    );
}
pub(crate) unsafe extern "C" fn GetNumPreliminaryPoints(monIndex: u8, capPoints: u8) -> u8 {
    let mut condition: u32 = (gContestMonRound1Points[monIndex] as u32) << 16;
    let mut numStars: u32 = condition / 63;
    if numStars & 0xFFFF != 0 {
        numStars += 0x10000;
    }
    numStars >>= 16;
    if numStars == 0 && condition != 0 {
        numStars = 1;
    }
    if capPoints != 0 && numStars > 10 {
        numStars = 10;
    }
    return numStars as u8;
}
pub(crate) unsafe extern "C" fn GetNumRound2Points(monIndex: u8, capPoints: u8) -> i8 {
    let mut r4: u32 = 0;
    let mut numHearts: u32 = 0;
    let mut results: i16 = 0;
    let mut points: i8 = 0;
    results = gContestMonRound2Points[monIndex];
    if results < 0 {
        r4 = (results as u32).wrapping_neg() << 16;
    } else {
        r4 = (results as u32) << 16;
    }
    numHearts = r4 / 80;
    if numHearts & 0xFFFF != 0 {
        numHearts += 0x10000;
    }
    numHearts >>= 16;
    if numHearts == 0 && r4 != 0 {
        numHearts = 1;
    }
    if capPoints != 0 && numHearts > 10 {
        numHearts = 10;
    }
    if gContestMonRound2Points[monIndex] < 0 {
        points = -(numHearts as i8);
    } else {
        points = numHearts as i8;
    }
    return points;
}
pub(crate) unsafe extern "C" fn Task_DrawFinalStandingNumber(taskId: u8) {
    let mut firstTileNum: u16 = 0;
    if gTasks[taskId].data[10] == 0 {
        gTasks[taskId].data[11] = (3 - gTasks[taskId].data[0]) * 40;
        gTasks[taskId].data[10] += 1;
    } else if gTasks[taskId].data[10] == 1 {
        if ({
            gTasks[taskId].data[11] -= 1;
            gTasks[taskId].data[11]
        }) == -1
        {
            firstTileNum = gTasks[taskId].data[0] as u16 * 2 + 0x5043;
            WriteSequenceToBgTilemapBuffer(
                2,
                firstTileNum,
                1,
                gTasks[taskId].data[1] as u8 * 3 + 5,
                2,
                1,
                17,
                1,
            );
            WriteSequenceToBgTilemapBuffer(
                2,
                firstTileNum + 0x10,
                1,
                gTasks[taskId].data[1] as u8 * 3 + 6,
                2,
                1,
                17,
                1,
            );
            (*(*sContestResults).data).numStandingsPrinted += 1;
            DestroyTask(taskId);
            PlaySE(SE_CONTEST_PLACE);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartHighlightWinnersBox(taskId: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < 4 && gContestFinalStandings[i] != 0 {
        i += 1;
    }
    CopyToBgTilemapBufferRect_ChangePalette(
        2,
        (*sContestResults).tilemapBuffers[2].at(i * 0xC0 + 0x100) as *mut c_void,
        0,
        i as u8 * 3 + 4,
        32,
        3,
        9,
    );
    gTasks[taskId].data[10] = i as i16;
    gTasks[taskId].data[12] = 1;
    gTasks[taskId].func = Some(Task_HighlightWinnersBox);
    (*(*sContestResults).data).highlightWinnerTaskId = taskId;
}
pub(crate) unsafe extern "C" fn Task_HighlightWinnersBox(taskId: u8) {
    if ({
        gTasks[taskId].data[11] += 1;
        gTasks[taskId].data[11]
    }) == 1
    {
        gTasks[taskId].data[11] = 0;
        BlendPalette(145, 1, gTasks[taskId].data[12] as u8, 28557);
        if gTasks[taskId].data[13] == 0 {
            if ({
                gTasks[taskId].data[12] += 1;
                gTasks[taskId].data[12]
            }) == 16
            {
                gTasks[taskId].data[13] = 1;
            }
        } else {
            if ({
                gTasks[taskId].data[12] -= 1;
                gTasks[taskId].data[12]
            }) == 0
            {
                gTasks[taskId].data[13] = 0;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WinnerMonSlideIn(sprite: *mut Sprite) {
    if (*sprite).data[0] < 10 {
        if ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) == 10
        {
            PlayCry_Normal((*sprite).data[1] as u16, 0);
            (*sprite).data[1] = 0;
        }
    } else {
        let mut delta: i16 = (*sprite).data[1] + 0x600;
        (*sprite).x -= delta >> 8;
        (*sprite).data[1] += 0x600;
        (*sprite).data[1] &= 0xFF;
        if (*sprite).x < 120 {
            (*sprite).x = 120;
        }
        if (*sprite).x == 120 {
            (*sprite).callback = Some(SpriteCallbackDummy);
            (*sprite).data[1] = 0;
            (*(*sContestResults).data).winnerMonSlidingState = SLIDING_MON_ENTERED;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WinnerMonSlideOut(sprite: *mut Sprite) {
    let mut delta: i16 = (*sprite).data[1] + 0x600;
    (*sprite).x -= delta >> 8;
    (*sprite).data[1] += 0x600;
    (*sprite).data[1] &= 0xFF;
    if (*sprite).x < -32 {
        (*sprite).callback = Some(SpriteCallbackDummy);
        (*sprite).set_invisible(TRUE as u16);
        (*(*sContestResults).data).winnerMonSlidingState = SLIDING_MON_EXITED;
    }
}
pub(crate) unsafe extern "C" fn Task_CreateConfetti(taskId: u8) {
    if ({
        gTasks[taskId].data[0] += 1;
        gTasks[taskId].data[0]
    }) == 5
    {
        gTasks[taskId].data[0] = 0;
        if (*(*sContestResults).data).confettiCount < 40 {
            let mut spriteId: u8 = CreateSprite(
                (&raw const *sSpriteTemplate_Confetti).cast_mut(),
                (Random() as i32 % 240) as i16 - 20,
                44,
                5,
            );
            gSprites[spriteId].data[0] = (Random() as i32 % 512) as i16;
            gSprites[spriteId].data[1] = (Random() as i32 % 24) as i16 + 16;
            gSprites[spriteId].data[2] = (Random() as i32 % 256) as i16 + 48;
            gSprites[spriteId]
                .oam
                .set_tileNum(gSprites[spriteId].oam.tileNum() + (Random() as i32 % 17) as u16);
            (*(*sContestResults).data).confettiCount += 1;
        }
    }
    if (*(*sContestResults).data).destroyConfetti != 0 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Confetti(sprite: *mut Sprite) {
    let mut delta: i16 = 0;
    (*sprite).data[3] += (*sprite).data[0];
    (*sprite).x2 = Sin((*sprite).data[3] >> 8, (*sprite).data[1]);
    delta = (*sprite).data[4] + (*sprite).data[2];
    (*sprite).x += delta >> 8;
    (*sprite).data[4] += (*sprite).data[2];
    (*sprite).data[4] &= 0xff;
    (*sprite).y += 1;
    if (*(*sContestResults).data).destroyConfetti != 0 {
        (*sprite).set_invisible(TRUE as u16);
    }
    if (*sprite).x > 248 || (*sprite).y > 116 {
        DestroySprite(sprite);
        (*(*sContestResults).data).confettiCount -= 1;
    }
}
pub(crate) unsafe extern "C" fn BounceMonIconInBox(monIndex: u8, numFrames: u8) {
    let mut taskId: u8 = CreateTask(Some(Task_BounceMonIconInBox), 8);
    gTasks[taskId].data[0] = monIndex as i16;
    gTasks[taskId].data[1] = numFrames as i16;
    gTasks[taskId].data[2] = gContestMons[monIndex].species as i16;
}
pub(crate) unsafe extern "C" fn Task_BounceMonIconInBox(taskId: u8) {
    let mut monIndex: u8 = gTasks[taskId].data[0] as u8;
    if ({
        let t1 = gTasks[taskId].data[10];
        gTasks[taskId].data[10] += 1;
        t1
    }) == gTasks[taskId].data[1]
    {
        gTasks[taskId].data[10] = 0;
        LoadContestMonIcon(
            gTasks[taskId].data[2] as u16,
            monIndex,
            gTasks[taskId].data[11] as u8,
            FALSE,
            gContestMons[monIndex].personality,
        );
        gTasks[taskId].data[11] ^= 1;
    }
}
pub(crate) unsafe extern "C" fn CalculateContestantsResultData() {
    let mut i: i32 = 0;
    let mut relativePoints: i32 = 0;
    let mut barLength: u32 = 0;
    let mut highestPoints: i16 = 0;
    let mut round2Points: i8 = 0;
    highestPoints = gContestMonTotalPoints[0];
    i = 1;
    while i < CONTESTANT_COUNT {
        if highestPoints < gContestMonTotalPoints[i] {
            highestPoints = gContestMonTotalPoints[i];
        }
        i += 1;
    }
    if highestPoints < 0 {
        highestPoints = gContestMonTotalPoints[0];
        i = 1;
        while i < CONTESTANT_COUNT {
            if highestPoints > gContestMonTotalPoints[i] {
                highestPoints = gContestMonTotalPoints[i];
            }
            i += 1;
        }
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        relativePoints = div_i32(
            gContestMonRound1Points[i] as i32 * 1000,
            if highestPoints < 0 {
                -(highestPoints as i32)
            } else {
                highestPoints as i32
            },
        );
        if relativePoints % 10 > 4 {
            relativePoints += 10;
        }
        (*(*sContestResults).monResults)[i].relativePreliminaryPoints = relativePoints / 10;
        relativePoints = div_i32(
            (if gContestMonRound2Points[i] < 0 {
                -(gContestMonRound2Points[i] as i32)
            } else {
                gContestMonRound2Points[i] as i32
            }) * 1000,
            if highestPoints < 0 {
                -(highestPoints as i32)
            } else {
                highestPoints as i32
            },
        );
        if relativePoints % 10 > 4 {
            relativePoints += 10;
        }
        (*(*sContestResults).monResults)[i].relativeRound2Points = relativePoints / 10;
        if gContestMonRound2Points[i] < 0 {
            (*(*sContestResults).monResults)[i].lostPoints = TRUE;
        }
        barLength =
            ((*(*sContestResults).monResults)[i].relativePreliminaryPoints * 0x5800 / 100) as u32;
        if barLength & 0xFF > 0x7F {
            barLength += 0x100;
        }
        (*(*sContestResults).monResults)[i].barLengthPreliminary = barLength >> 8;
        barLength =
            ((*(*sContestResults).monResults)[i].relativeRound2Points * 0x5800 / 100) as u32;
        if barLength & 0xFF > 0x7F {
            barLength += 0x100;
        }
        (*(*sContestResults).monResults)[i].barLengthRound2 = barLength >> 8;
        (*(*sContestResults).monResults)[i].numStars = GetNumPreliminaryPoints(i as u8, TRUE);
        round2Points = GetNumRound2Points(i as u8, TRUE);
        (*(*sContestResults).monResults)[i].numHearts = (if round2Points < 0 {
            -(round2Points as i32)
        } else {
            round2Points as i32
        }) as u8;
        if gContestFinalStandings[i] != 0 {
            let mut barLengthPreliminary: i16 =
                (*(*sContestResults).monResults)[i].barLengthPreliminary as i16;
            let mut barLengthRound2: i16 =
                (*(*sContestResults).monResults)[i].barLengthRound2 as i16;
            if (*(*sContestResults).monResults)[i].lostPoints != 0 {
                barLengthRound2 *= -1;
            }
            if barLengthPreliminary as i32 + barLengthRound2 as i32 == MAX_BAR_LENGTH {
                if barLengthRound2 > 0 {
                    (*(*sContestResults).monResults)[i].barLengthRound2 -= 1;
                } else if barLengthPreliminary > 0 {
                    (*(*sContestResults).monResults)[i].barLengthPreliminary -= 1;
                }
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateContestResultBars(isRound2: u8, numUpdates: u8) {
    let mut i: i32 = 0;
    let mut taskId: i32 = 0;
    let mut target: u32 = 0;
    let mut numIncreasing: u8 = 0;
    let mut numDecreasing: u8 = 0;
    if isRound2 == 0 {
        i = 0;
        while i < CONTESTANT_COUNT {
            let mut numStars: u8 = (*(*sContestResults).monResults)[i].numStars;
            if numUpdates < numStars {
                FillBgTilemapBufferRect_Palette0(
                    1,
                    0x60B3,
                    19 + numStars - numUpdates - 1,
                    i as u8 * 3 + 5,
                    1,
                    1,
                );
                taskId = CreateTask(Some(Task_UpdateContestResultBar), 10) as i32;
                target = div_u32(
                    (*(*sContestResults).monResults)[i].barLengthPreliminary << 16,
                    (*(*sContestResults).monResults)[i].numStars as u32,
                ) * (numUpdates as u32 + 1);
                if target & 0xFFFF > 0x7FFF {
                    target += 0x10000;
                }
                gTasks[taskId].data[0] = i as i16;
                gTasks[taskId].data[1] = (target >> 16) as i16;
                (*(*sContestResults).data).numBarsUpdating += 1;
                numIncreasing += 1;
            }
            i += 1;
        }
    } else {
        i = 0;
        while i < CONTESTANT_COUNT {
            let mut numHearts: i8 = (*(*sContestResults).monResults)[i].numHearts as i8;
            let mut tile: u32 = (if (*(*sContestResults).monResults)[i].lostPoints != 0 {
                0x60A5
            } else {
                0x60A3
            }) as u32;
            if (numUpdates as i32) < numHearts as i32 {
                FillBgTilemapBufferRect_Palette0(
                    1,
                    tile as u16,
                    19 + numHearts as u8 - numUpdates - 1,
                    i as u8 * 3 + 6,
                    1,
                    1,
                );
                taskId = CreateTask(Some(Task_UpdateContestResultBar), 10) as i32;
                target = div_u32(
                    (*(*sContestResults).monResults)[i].barLengthRound2 << 16,
                    (*(*sContestResults).monResults)[i].numHearts as u32,
                ) * (numUpdates as u32 + 1);
                if target & 0xFFFF > 0x7FFF {
                    target += 0x10000;
                }
                gTasks[taskId].data[0] = i as i16;
                if (*(*sContestResults).monResults)[i].lostPoints != 0 {
                    gTasks[taskId].data[2] = TRUE as i16;
                    numDecreasing += 1;
                } else {
                    numIncreasing += 1;
                }
                if (*(*sContestResults).monResults)[i].lostPoints != 0 {
                    gTasks[taskId].data[1] = -((target >> 16) as i16)
                        + (*(*sContestResults).monResults)[i].barLengthPreliminary as i16;
                } else {
                    gTasks[taskId].data[1] = (target >> 16) as i16
                        + (*(*sContestResults).monResults)[i].barLengthPreliminary as i16;
                }
                (*(*sContestResults).data).numBarsUpdating += 1;
            }
            i += 1;
        }
    }
    if numDecreasing != 0 {
        PlaySE(SE_BOO);
    }
    if numIncreasing != 0 {
        PlaySE(SE_PIN);
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateContestResultBar(taskId: u8) {
    let mut i: i32 = 0;
    let mut minMaxReached: u32 = FALSE as u32;
    let mut targetReached: u32 = FALSE as u32;
    let mut monId: u8 = gTasks[taskId].data[0] as u8;
    let mut target: i16 = gTasks[taskId].data[1];
    let mut decreasing: i16 = gTasks[taskId].data[2];
    if decreasing != 0 {
        if (*(*sContestResults).data).barLength[monId] <= 0 {
            minMaxReached = TRUE as u32;
        }
    } else {
        if (*(*sContestResults).data).barLength[monId] >= MAX_BAR_LENGTH as i16 {
            minMaxReached = TRUE as u32;
        }
    }
    if (*(*sContestResults).data).barLength[monId] == target {
        targetReached = TRUE as u32;
    }
    if targetReached == 0 {
        if minMaxReached != 0 {
            (*(*sContestResults).data).barLength[monId] = target;
        } else if decreasing != 0 {
            (*(*sContestResults).data).barLength[monId] -= 1;
        } else {
            (*(*sContestResults).data).barLength[monId] += 1;
        }
    }
    if minMaxReached == 0 && targetReached == 0 {
        let mut tileOffset: u8 = 0;
        let mut tileNum: u16 = 0;
        i = 0;
        while i < NUM_BAR_SEGMENTS {
            if (*(*sContestResults).data).barLength[monId] as i32 >= (i + 1) * BAR_SEGMENT_LENGTH {
                tileOffset = 8;
            } else if (*(*sContestResults).data).barLength[monId] as i32 >= i * BAR_SEGMENT_LENGTH {
                tileOffset = ((*(*sContestResults).data).barLength[monId] % 8) as u8;
            } else {
                tileOffset = 0;
            }
            if tileOffset < 4 {
                tileNum = 0x504C + tileOffset as u16;
            } else {
                tileNum = 0x5057 + tileOffset as u16;
            }
            FillBgTilemapBufferRect_Palette0(2, tileNum, i as u8 + 7, monId * 3 + 6, 1, 1);
            i += 1;
        }
    }
    if targetReached != 0 {
        (*(*sContestResults).data).numBarsUpdating -= 1;
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AllocContestResults() {
    sContestResults = AllocZeroed(32) as *mut ContestResults;
    (*sContestResults).data = AllocZeroed(24) as *mut ContestResultsInternal;
    (*sContestResults).monResults = AllocZeroed(80) as *mut CArray<ContestMonResults, 4>;
    (*sContestResults).unusedBg = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
    (*sContestResults).tilemapBuffers[0] = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
    (*sContestResults).tilemapBuffers[1] = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
    (*sContestResults).tilemapBuffers[2] = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
    (*sContestResults).tilemapBuffers[3] = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
    (*sContestResults).unused = AllocZeroed(0x1000) as *mut u8;
    AllocateMonSpritesGfx();
}
pub(crate) unsafe extern "C" fn FreeContestResults() {
    Free((*sContestResults).data as *mut c_void);
    (*sContestResults).data = null_mut();
    Free((*sContestResults).monResults as *mut c_void);
    (*sContestResults).monResults = null_mut();
    Free((*sContestResults).unusedBg as *mut c_void);
    (*sContestResults).unusedBg = null_mut();
    Free((*sContestResults).tilemapBuffers[0] as *mut c_void);
    (*sContestResults).tilemapBuffers[0] = null_mut();
    Free((*sContestResults).tilemapBuffers[1] as *mut c_void);
    (*sContestResults).tilemapBuffers[1] = null_mut();
    Free((*sContestResults).tilemapBuffers[2] as *mut c_void);
    (*sContestResults).tilemapBuffers[2] = null_mut();
    Free((*sContestResults).tilemapBuffers[3] as *mut c_void);
    (*sContestResults).tilemapBuffers[3] = null_mut();
    Free((*sContestResults).unused as *mut c_void);
    (*sContestResults).unused = null_mut();
    Free(sContestResults as *mut c_void);
    sContestResults = null_mut();
    FreeMonSpritesGfx();
}
pub(crate) unsafe extern "C" fn AddContestTextPrinter(windowId: i32, str: *mut u8, x: i32) {
    let mut textPrinter: TextPrinterTemplate = zeroed();
    textPrinter.currentChar = str;
    textPrinter.windowId = windowId as u8;
    textPrinter.fontId = FONT_NARROW;
    textPrinter.x = x as u8;
    textPrinter.y = 2;
    textPrinter.currentX = x as u8;
    textPrinter.currentY = 2;
    textPrinter.letterSpacing = 0;
    textPrinter.lineSpacing = 0;
    textPrinter.set_unk(0);
    textPrinter.set_fgColor(1);
    textPrinter.set_bgColor(0);
    textPrinter.set_shadowColor(8);
    AddTextPrinter(&raw mut textPrinter, 0, None);
    PutWindowTilemap(windowId as u8);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryEnterContestMon() {
    let mut eligibility: u8 =
        GetContestEntryEligibility(&raw mut gPlayerParty[gContestMonPartyIndex]);
    if eligibility != 0 {
        SetContestants(
            gSpecialVar_ContestCategory as u8,
            gSpecialVar_ContestRank as u8,
        );
        CalculateRound1Points(gSpecialVar_ContestCategory as u8);
    }
    gSpecialVar_Result = eligibility as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasMonWonThisContestBefore() -> u16 {
    let mut hasRankRibbon: u16 = FALSE as u16;
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gContestMonPartyIndex];
    match gSpecialVar_ContestCategory {
        0 => {
            if GetMonData2(mon, MON_DATA_COOL_RIBBON) > gSpecialVar_ContestRank as u32 {
                hasRankRibbon = TRUE as u16;
            }
        }
        1 => {
            if GetMonData2(mon, MON_DATA_BEAUTY_RIBBON) > gSpecialVar_ContestRank as u32 {
                hasRankRibbon = TRUE as u16;
            }
        }
        2 => {
            if GetMonData2(mon, MON_DATA_CUTE_RIBBON) > gSpecialVar_ContestRank as u32 {
                hasRankRibbon = TRUE as u16;
            }
        }
        3 => {
            if GetMonData2(mon, MON_DATA_SMART_RIBBON) > gSpecialVar_ContestRank as u32 {
                hasRankRibbon = TRUE as u16;
            }
        }
        4 => {
            if GetMonData2(mon, MON_DATA_TOUGH_RIBBON) > gSpecialVar_ContestRank as u32 {
                hasRankRibbon = TRUE as u16;
            }
        }
        _ => {}
    }
    return hasRankRibbon;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveMonContestRibbon() {
    let mut ribbonData: u8 = 0;
    if gContestFinalStandings[gContestPlayerMonIndex] != 0 {
        return;
    }
    match gSpecialVar_ContestCategory {
        0 => {
            ribbonData = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_COOL_RIBBON,
            ) as u8;
            if ribbonData as u16 <= gSpecialVar_ContestRank && ribbonData <= CONTEST_RANK_MASTER {
                ribbonData += 1;
                SetMonData(
                    &raw mut gPlayerParty[gContestMonPartyIndex],
                    MON_DATA_COOL_RIBBON,
                    &raw mut ribbonData as *mut c_void,
                );
                if GetRibbonCount(&raw mut gPlayerParty[gContestMonPartyIndex]) > NUM_CUTIES_RIBBONS
                {
                    TryPutSpotTheCutiesOnAir(
                        &raw mut gPlayerParty[gContestMonPartyIndex],
                        MON_DATA_COOL_RIBBON as u8,
                    );
                }
            }
        }
        1 => {
            ribbonData = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_BEAUTY_RIBBON,
            ) as u8;
            if ribbonData as u16 <= gSpecialVar_ContestRank && ribbonData <= CONTEST_RANK_MASTER {
                ribbonData += 1;
                SetMonData(
                    &raw mut gPlayerParty[gContestMonPartyIndex],
                    MON_DATA_BEAUTY_RIBBON,
                    &raw mut ribbonData as *mut c_void,
                );
                if GetRibbonCount(&raw mut gPlayerParty[gContestMonPartyIndex]) > NUM_CUTIES_RIBBONS
                {
                    TryPutSpotTheCutiesOnAir(
                        &raw mut gPlayerParty[gContestMonPartyIndex],
                        MON_DATA_BEAUTY_RIBBON as u8,
                    );
                }
            }
        }
        2 => {
            ribbonData = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_CUTE_RIBBON,
            ) as u8;
            if ribbonData as u16 <= gSpecialVar_ContestRank && ribbonData <= CONTEST_RANK_MASTER {
                ribbonData += 1;
                SetMonData(
                    &raw mut gPlayerParty[gContestMonPartyIndex],
                    MON_DATA_CUTE_RIBBON,
                    &raw mut ribbonData as *mut c_void,
                );
                if GetRibbonCount(&raw mut gPlayerParty[gContestMonPartyIndex]) > NUM_CUTIES_RIBBONS
                {
                    TryPutSpotTheCutiesOnAir(
                        &raw mut gPlayerParty[gContestMonPartyIndex],
                        MON_DATA_CUTE_RIBBON as u8,
                    );
                }
            }
        }
        3 => {
            ribbonData = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_SMART_RIBBON,
            ) as u8;
            if ribbonData as u16 <= gSpecialVar_ContestRank && ribbonData <= CONTEST_RANK_MASTER {
                ribbonData += 1;
                SetMonData(
                    &raw mut gPlayerParty[gContestMonPartyIndex],
                    MON_DATA_SMART_RIBBON,
                    &raw mut ribbonData as *mut c_void,
                );
                if GetRibbonCount(&raw mut gPlayerParty[gContestMonPartyIndex]) > NUM_CUTIES_RIBBONS
                {
                    TryPutSpotTheCutiesOnAir(
                        &raw mut gPlayerParty[gContestMonPartyIndex],
                        MON_DATA_SMART_RIBBON as u8,
                    );
                }
            }
        }
        4 => {
            ribbonData = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_TOUGH_RIBBON,
            ) as u8;
            if ribbonData as u16 <= gSpecialVar_ContestRank && ribbonData <= CONTEST_RANK_MASTER {
                ribbonData += 1;
                SetMonData(
                    &raw mut gPlayerParty[gContestMonPartyIndex],
                    MON_DATA_TOUGH_RIBBON,
                    &raw mut ribbonData as *mut c_void,
                );
                if GetRibbonCount(&raw mut gPlayerParty[gContestMonPartyIndex]) > NUM_CUTIES_RIBBONS
                {
                    TryPutSpotTheCutiesOnAir(
                        &raw mut gPlayerParty[gContestMonPartyIndex],
                        MON_DATA_TOUGH_RIBBON as u8,
                    );
                }
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestantTrainerName() {
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gContestMons[gSpecialVar_0x8006].trainerName.as_mut_ptr(),
    );
    ConvertInternationalContestantName(gStringVar1.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestantMonNickname() {
    StringCopy(
        gStringVar3.as_mut_ptr(),
        gContestMons[gSpecialVar_0x8006].nickname.as_mut_ptr(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestMonConditionRanking() {
    let mut i: u8 = 0;
    let mut rank: u8 = 0;
    i = 0;
    rank = 0;
    while i < CONTESTANT_COUNT as u8 {
        if gContestMonRound1Points[gSpecialVar_0x8006] < gContestMonRound1Points[i] {
            rank += 1;
        }
        i += 1;
    }
    gSpecialVar_0x8004 = rank as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestMonCondition() {
    gSpecialVar_0x8004 = gContestMonRound1Points[gSpecialVar_0x8006] as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestWinnerId() {
    let mut i: u8 = 0;
    i = 0;
    while i < 4 && gContestFinalStandings[i] != 0 {
        i += 1;
    }
    gSpecialVar_0x8005 = i as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestWinnerTrainerName() {
    let mut i: u8 = 0;
    i = 0;
    while i < 4 && gContestFinalStandings[i] != 0 {
        i += 1;
    }
    StringCopy(
        gStringVar3.as_mut_ptr(),
        gContestMons[i].trainerName.as_mut_ptr(),
    );
    ConvertInternationalContestantName(gStringVar3.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestWinnerMonName() {
    let mut i: u8 = 0;
    i = 0;
    while i < 4 && gContestFinalStandings[i] != 0 {
        i += 1;
    }
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gContestMons[i].nickname.as_mut_ptr(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_SetStartContestCallback() {
    SetMainCallback2(Some(CB2_StartContest));
}
pub(crate) unsafe extern "C" fn Task_StartContest(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DestroyTask(taskId);
        SetMainCallback2(Some(CB2_SetStartContestCallback));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartContest() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_StartContest), 10);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestantMonSpecies() {
    gSpecialVar_0x8004 = gContestMons[gSpecialVar_0x8006].species;
}
pub(crate) unsafe extern "C" fn Task_StartShowContestResults(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DestroyTask(taskId);
        SetMainCallback2(Some(CB2_StartShowContestResults));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowContestResults() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_StartShowContestResults), 10);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestPlayerId() {
    gSpecialVar_0x8004 = gContestPlayerMonIndex as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestLinkTransfer(category: u8) {
    let mut newTaskId: u8 = 0;
    LockPlayerFieldControls();
    newTaskId = CreateTask(Some(Task_LinkContest_Init), 0);
    SetTaskFuncWithFollowupFunc(
        newTaskId,
        Some(Task_LinkContest_Init),
        Some(Task_StartCommunication),
    );
    gTasks[newTaskId].data[9] = category as i16;
}
pub(crate) unsafe extern "C" fn Task_StartCommunication(taskId: u8) {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_HAS_RS_PLAYER != 0 {
        CreateContestMonFromParty(gContestMonPartyIndex);
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateMonsRS),
            Some(Task_StartCommunicateRngRS),
        );
    } else {
        CreateContestMonFromParty(gContestMonPartyIndex);
        gTasks[taskId].func = Some(Task_LinkContest_StartCommunicationEm);
    }
}
pub(crate) unsafe extern "C" fn Task_StartCommunicateRngRS(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateRngRS),
        Some(Task_StartCommunicateLeaderIdsRS),
    );
}
pub(crate) unsafe extern "C" fn Task_StartCommunicateLeaderIdsRS(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateLeaderIdsRS),
        Some(Task_StartCommunicateCategoryRS),
    );
}
pub(crate) unsafe extern "C" fn Task_StartCommunicateCategoryRS(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateCategoryRS),
        Some(Task_LinkContest_SetUpContestRS),
    );
}
pub(crate) unsafe extern "C" fn Task_LinkContest_SetUpContestRS(taskId: u8) {
    let mut i: u8 = 0;
    let mut categories: CArray<u8, 4> = zeroed();
    let mut leaderIds: CArray<u8, 4> = zeroed();
    memset(categories.as_mut_ptr(), 0, 4);
    memset(leaderIds.as_mut_ptr(), 0, 4);
    i = 0;
    while i < gNumLinkContestPlayers {
        categories[i] = gTasks[taskId].data[i as i32 + 1] as u8;
        i += 1;
    }
    i = 0;
    while i < gNumLinkContestPlayers && categories[0] == categories[i] {
        i += 1;
    }
    if i == gNumLinkContestPlayers {
        gSpecialVar_0x8004 = FALSE as u16;
    } else {
        gSpecialVar_0x8004 = TRUE as u16;
    }
    i = 0;
    while i < gNumLinkContestPlayers {
        leaderIds[i] = gTasks[taskId].data[i as i32 + 5] as u8;
        i += 1;
    }
    gContestLinkLeaderIndex = LinkContest_GetLeaderIndex(leaderIds.as_mut_ptr());
    CalculateRound1Points(gSpecialVar_ContestCategory as u8);
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateRound1Points),
        Some(Task_LinkContest_CalculateTurnOrderRS),
    );
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CalculateTurnOrderRS(taskId: u8) {
    SortContestants(FALSE);
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateTurnOrder),
        Some(Task_LinkContest_FinalizeConnection),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContest_GetLeaderIndex(ids: *mut u8) -> u8 {
    let mut i: i32 = 0;
    let mut leaderIdx: u8 = 0;
    i = 1;
    while i < gNumLinkContestPlayers as i32 {
        if *ids.at(leaderIdx) < *ids.at(i) {
            leaderIdx = i as u8;
        }
        i += 1;
    }
    return leaderIdx;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_FinalizeConnection(taskId: u8) {
    let mut i: i32 = 0;
    if gSpecialVar_0x8004 == TRUE as u16 {
        if IsLinkTaskFinished() != 0 {
            gTasks[taskId].func = Some(Task_LinkContest_Disconnect);
        }
    } else {
        i = 0;
        while i < CONTESTANT_COUNT {
            StringGet_Nickname(gContestMons[i].nickname.as_mut_ptr());
            i += 1;
        }
        DestroyTask(taskId);
        SetDynamicWarp(
            0,
            (*gSaveBlock1Ptr).location.mapGroup,
            (*gSaveBlock1Ptr).location.mapNum,
            WARP_ID_NONE,
        );
        UnlockPlayerFieldControls();
        ScriptContext_Enable();
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_Disconnect(taskId: u8) {
    SetCloseLinkCallback();
    gTasks[taskId].func = Some(Task_LinkContest_WaitDisconnect);
}
pub(crate) unsafe extern "C" fn Task_LinkContest_WaitDisconnect(taskId: u8) {
    if gReceivedRemoteLinkPlayers == 0 {
        DestroyTask(taskId);
        UnlockPlayerFieldControls();
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContestTrainerGfxIds() {
    (*gSaveBlock1Ptr).vars[16] = gContestMons[0].trainerGfxId as u16;
    (*gSaveBlock1Ptr).vars[17] = gContestMons[1].trainerGfxId as u16;
    (*gSaveBlock1Ptr).vars[18] = gContestMons[2].trainerGfxId as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNpcContestantLocalId() {
    let mut localId: u16 = 0;
    let mut contestant: u8 = gSpecialVar_0x8005 as u8;
    match contestant {
        0 => {
            localId = LOCALID_CONTESTANT_1;
        }
        1 => {
            localId = LOCALID_CONTESTANT_2;
        }
        2 => {
            localId = LOCALID_CONTESTANT_3;
        }
        _ => {
            localId = 100;
        }
    }
    gSpecialVar_0x8004 = localId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestTrainerAndMonNames() {
    BufferContestantTrainerName();
    BufferContestantMonNickname();
    BufferContestantMonSpecies();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoesContestCategoryHaveMuseumPainting() {
    let mut contestWinner: i32 = 0;
    match gSpecialVar_ContestCategory {
        0 => {
            contestWinner = 8;
        }
        1 => {
            contestWinner = 9;
        }
        2 => {
            contestWinner = 10;
        }
        3 => {
            contestWinner = 11;
        }
        _ => {
            contestWinner = 12;
        }
    }
    if (*gSaveBlock1Ptr).contestWinners[contestWinner].species == SPECIES_NONE {
        gSpecialVar_0x8004 = FALSE as u16;
    } else {
        gSpecialVar_0x8004 = TRUE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveMuseumContestPainting() {
    SaveContestWinner(CONTEST_SAVE_FOR_MUSEUM as u8);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldReadyContestArtist() {
    if gContestFinalStandings[gContestPlayerMonIndex] == 0
        && gSpecialVar_ContestRank == CONTEST_RANK_MASTER as u16
        && gContestMonTotalPoints[gContestPlayerMonIndex] >= 800
    {
        gSpecialVar_0x8004 = TRUE as u16;
    } else {
        gSpecialVar_0x8004 = FALSE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPlayerMuseumPaintings() -> u8 {
    let mut i: i32 = 0;
    let mut count: u8 = 0;
    i = 0;
    while i < 5 {
        if (*gSaveBlock1Ptr).contestWinners[MUSEUM_CONTEST_WINNERS_START as i32 + i].species != 0 {
            count += 1;
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestantNamesAtRank() {
    let mut conditions: CArray<i16, 4> = zeroed();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut condition: i16 = 0;
    let mut numAtCondition: i8 = 0;
    let mut contestantOffset: u8 = 0;
    let mut tieRank: u8 = 0;
    let mut rank: u8 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        conditions[i] = gContestMonRound1Points[i];
        i += 1;
    }
    i = 0;
    while i < 3 {
        j = 3;
        while j > i {
            if conditions[j - 1] < conditions[j] {
                let mut temp: i32 = 0;
                temp = conditions[j] as i32;
                conditions[j] = conditions[j - 1];
                conditions[j - 1] = temp as i16;
            }
            j -= 1;
        }
        i += 1;
    }
    condition = conditions[gSpecialVar_0x8006];
    numAtCondition = 0;
    tieRank = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        if conditions[i] == condition {
            numAtCondition += 1;
            if i == gSpecialVar_0x8006 as i32 {
                tieRank = numAtCondition as u8;
            }
        }
        i += 1;
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        if conditions[i] == condition {
            break;
        }
        i += 1;
    }
    rank = i as u8;
    contestantOffset = tieRank;
    i = 0;
    while i < CONTESTANT_COUNT {
        if condition == gContestMonRound1Points[i] {
            if contestantOffset == 1 {
                break;
            }
            contestantOffset -= 1;
        }
        i += 1;
    }
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gContestMons[i].nickname.as_mut_ptr(),
    );
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gContestMons[i].trainerName.as_mut_ptr(),
    );
    ConvertInternationalContestantName(gStringVar2.as_mut_ptr());
    if numAtCondition == 1 {
        gSpecialVar_0x8006 = rank as u16;
    } else if tieRank as i32 == numAtCondition as i32 {
        gSpecialVar_0x8006 = rank as u16;
    } else {
        gSpecialVar_0x8006 = rank as u16 + CONTESTANT_COUNT as u16;
    }
}
pub(crate) unsafe extern "C" fn ExitContestPainting() {
    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowContestPainting() {
    SetMainCallback2(Some(CB2_ContestPainting));
    gMain.savedCallback = Some(ExitContestPainting);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLinkContestPlayerGfx() {
    let mut i: i32 = 0;
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        i = 0;
        while i < gNumLinkContestPlayers as i32 {
            let mut version: i32 = gLinkPlayers[i].version as u8 as i32;
            if version == VERSION_RUBY || version == VERSION_SAPPHIRE {
                if gLinkPlayers[i].gender == MALE {
                    gContestMons[i].trainerGfxId = OBJ_EVENT_GFX_LINK_RS_BRENDAN;
                } else {
                    gContestMons[i].trainerGfxId = OBJ_EVENT_GFX_LINK_RS_MAY;
                }
            }
            i += 1;
        }
        VarSet(VAR_OBJ_GFX_ID_0, gContestMons[0].trainerGfxId as u16);
        VarSet(VAR_OBJ_GFX_ID_1, gContestMons[1].trainerGfxId as u16);
        VarSet(VAR_OBJ_GFX_ID_2, gContestMons[2].trainerGfxId as u16);
        VarSet(VAR_OBJ_GFX_ID_3, gContestMons[3].trainerGfxId as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadLinkContestPlayerPalettes() {
    let mut i: i32 = 0;
    let mut objectEventId: u8 = 0;
    let mut version: i32 = 0;
    let mut sprite: *mut Sprite = null_mut();
    gReservedSpritePaletteCount = 12;
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        i = 0;
        while i < gNumLinkContestPlayers as i32 {
            objectEventId = GetObjectEventIdByLocalIdAndMap(
                sContestantLocalIds_0[i],
                (*gSaveBlock1Ptr).location.mapNum as u8,
                (*gSaveBlock1Ptr).location.mapGroup as u8,
            );
            sprite = &raw mut gSprites[gObjectEvents[objectEventId].spriteId];
            (*sprite).oam.set_paletteNum(6 + i as u16);
            version = gLinkPlayers[i].version as u8 as i32;
            if version == VERSION_RUBY || version == VERSION_SAPPHIRE {
                if gLinkPlayers[i].gender == MALE {
                    LoadPalette(
                        gObjectEventPal_RubySapphireBrendan.as_ptr().cast_mut() as *mut c_void,
                        0x100 + (6 + i as u16) * 16,
                        32,
                    );
                } else {
                    LoadPalette(
                        gObjectEventPal_RubySapphireMay.as_ptr().cast_mut() as *mut c_void,
                        0x100 + (6 + i as u16) * 16,
                        32,
                    );
                }
            } else {
                if gLinkPlayers[i].gender == MALE {
                    LoadPalette(
                        gObjectEventPal_Brendan.as_ptr().cast_mut() as *mut c_void,
                        0x100 + (6 + i as u16) * 16,
                        32,
                    );
                } else {
                    LoadPalette(
                        gObjectEventPal_May.as_ptr().cast_mut() as *mut c_void,
                        0x100 + (6 + i as u16) * 16,
                        32,
                    );
                }
            }
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveMonArtistRibbon() -> u8 {
    let mut hasArtistRibbon: u8 = 0;
    hasArtistRibbon = GetMonData2(
        &raw mut gPlayerParty[gContestMonPartyIndex],
        MON_DATA_ARTIST_RIBBON,
    ) as u8;
    if hasArtistRibbon == 0
        && gContestFinalStandings[gContestPlayerMonIndex] == 0
        && gSpecialVar_ContestRank == CONTEST_RANK_MASTER as u16
        && gContestMonTotalPoints[gContestPlayerMonIndex] >= 800
    {
        hasArtistRibbon = 1;
        SetMonData(
            &raw mut gPlayerParty[gContestMonPartyIndex],
            MON_DATA_ARTIST_RIBBON,
            &raw mut hasArtistRibbon as *mut c_void,
        );
        if GetRibbonCount(&raw mut gPlayerParty[gContestMonPartyIndex]) > NUM_CUTIES_RIBBONS {
            TryPutSpotTheCutiesOnAir(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_ARTIST_RIBBON as u8,
            );
        }
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsContestDebugActive() -> u8 {
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowContestEntryMonPic() {
    let mut palette: *mut CompressedSpritePalette = null_mut();
    let mut personality: u32 = 0;
    let mut otId: u32 = 0;
    let mut species: u16 = 0;
    let mut spriteId: u8 = 0;
    let mut taskId: u8 = 0;
    let mut left: u8 = 0;
    let mut top: u8 = 0;
    if FindTaskIdByFunc(Some(Task_ShowContestEntryMonPic)) == TASK_NONE {
        AllocateMonSpritesGfx();
        left = 10;
        top = 3;
        species = gContestMons[gSpecialVar_0x8006].species;
        personality = gContestMons[gSpecialVar_0x8006].personality;
        otId = gContestMons[gSpecialVar_0x8006].otId;
        taskId = CreateTask(Some(Task_ShowContestEntryMonPic), 0x50);
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].data[1] = species as i16;
        if gSpecialVar_0x8006 == gContestPlayerMonIndex as u16 {
            HandleLoadSpecialPokePic_2(
                (&raw const gMonFrontPicTable[species]).cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[1],
                species as i32,
                personality,
            );
        } else {
            HandleLoadSpecialPokePic_DontHandleDeoxys(
                (&raw const gMonFrontPicTable[species]).cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr[1],
                species as i32,
                personality,
            );
        }
        palette = GetMonSpritePalStructFromOtIdPersonality(species, otId, personality);
        LoadCompressedSpritePalette(palette);
        SetMultiuseSpriteTemplateToPokemon(species, B_POSITION_OPPONENT_LEFT);
        gMultiuseSpriteTemplate.paletteTag = (*palette).tag;
        spriteId = CreateSprite(
            &raw mut gMultiuseSpriteTemplate,
            (left as i16 + 1) * 8 + 32,
            top as i16 * 8 + 40,
            0,
        );
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
            if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_HAS_RS_PLAYER == 0 {
                DoMonFrontSpriteAnimation(&raw mut gSprites[spriteId], species, 0, 0);
            }
        } else {
            DoMonFrontSpriteAnimation(&raw mut gSprites[spriteId], species, 0, 0);
        }
        gTasks[taskId].data[2] = spriteId as i16;
        gTasks[taskId].data[3] = left as i16;
        gTasks[taskId].data[4] = top as i16;
        gSprites[spriteId].callback = Some(SpriteCallbackDummy);
        gSprites[spriteId].oam.set_priority(0);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideContestEntryMonPic() {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_ShowContestEntryMonPic));
    if taskId != TASK_NONE {
        gTasks[taskId].data[0] += 1;
        FreeMonSpritesGfx();
    }
}
pub(crate) unsafe extern "C" fn Task_ShowContestEntryMonPic(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    let mut sprite: *mut Sprite = null_mut();
    match (*task).data[0] {
        0 => {
            (*task).data[0] += 1;
        }
        1 => {
            (*task).data[5] = CreateWindowFromRect(10, 3, 8, 8) as i16;
            SetStandardWindowBorderStyle((*task).data[5] as u8, TRUE);
            (*task).data[0] += 1;
        }
        2 => {}
        3 => {
            sprite = &raw mut gSprites[(*task).data[2]];
            FreeSpritePaletteByTag(GetSpritePaletteTagByPaletteNum(
                (*sprite).oam.paletteNum() as u8
            ));
            if (*sprite).oam.affineMode() != 0 {
                FreeOamMatrix((*sprite).oam.matrixNum() as u8);
            }
            DestroySprite(sprite);
            (*task).data[0] += 1;
        }
        4 => {
            ClearToTransparentAndRemoveWindow(gTasks[taskId].data[5] as u8);
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestMultiplayerId() {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0
        && gNumLinkContestPlayers == CONTESTANT_COUNT as u8
        && gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS == 0
    {
        gSpecialVar_Result = GetMultiplayerId() as u16;
    } else {
        gSpecialVar_Result = MAX_LINK_PLAYERS as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateContestRand() {
    let mut random: u16 = 0;
    let mut result: *mut u16 = null_mut();
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        gContestRngValue = 0x41c64e6d * gContestRngValue + 24691;
        random = (gContestRngValue >> 16) as u16;
        result = &raw mut gSpecialVar_Result;
    } else {
        result = &raw mut gSpecialVar_Result;
        random = Random();
    }
    *result = rem_i32(random as i32, *result as i32) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestRand() -> u16 {
    gContestRngValue = 0x41c64e6d * gContestRngValue + 24691;
    return (gContestRngValue >> 16) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContestWaitForConnection() -> u8 {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
        CreateTask(Some(Task_LinkContestWaitForConnection), 5);
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContestWaitForConnection(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            if IsLinkTaskFinished() != 0 {
                SetLinkStandbyCallback();
                gTasks[taskId].data[0] += 1;
            }
        }
        1 => {
            gTasks[taskId].data[0] += 1;
        }
        _ => {
            if IsLinkTaskFinished() == 1 {
                ScriptContext_Enable();
                DestroyTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContestTryShowWirelessIndicator() {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
        if gReceivedRemoteLinkPlayers != 0 {
            LoadWirelessStatusIndicatorSpriteGfx();
            CreateWirelessStatusIndicatorSprite(8, 8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContestTryHideWirelessIndicator() {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
        if gReceivedRemoteLinkPlayers != 0 {
            DestroyWirelessStatusIndicatorSprite();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsContestWithRSPlayer() -> u8 {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_HAS_RS_PLAYER != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearLinkContestFlags() {
    gLinkContestFlags = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWirelessContest() -> u8 {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
