//! Translated from `src/battle_dome.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sBattleStyleMovePoints sBattleStyleThresholds sUnusedArray sTourneyTreeCursorMovementMap sTourneyTreeBgTemplates sInfoCardBgTemplates sTourneyTreeWindowTemplates sInfoCardWindowTemplates sTourneyTreeScanlineEffectParams sTourneyTreeButtonsSpriteSheet sTourneyTreeButtonsSpritePal sOamData_TourneyTreePokeball sOamData_TourneyTreeCloseButton sOamData_VerticalScrollArrow sOamData_HorizontalScrollArrow sSpriteAnim_TourneyTreePokeballNormal sSpriteAnim_TourneyTreePokeballSelected sSpriteAnimTable_TourneyTreePokeball sTourneyTreePokeballSpriteTemplate sSpriteAnim_TourneyTreeCancelButtonNormal sSpriteAnim_TourneyTreeCancelButtonSelected sSpriteAnimTable_TourneyTreeCancelButton sCancelButtonSpriteTemplate sSpriteAnim_TourneyTreeExitButtonNormal sSpriteAnim_TourneyTreeExitButtonSelected sSpriteAnimTable_TourneyTreeExitButton sExitButtonSpriteTemplate sSpriteAnim_UpArrow sSpriteAnim_DownArrow sSpriteAnim_LeftArrow sSpriteAnim_RightArrow sSpriteAnimTable_VerticalScrollArrow sSpriteAnimTable_HorizontalScrollArrow sHorizontalScrollArrowSpriteTemplate sVerticalScrollArrowSpriteTemplate sTourneyTreeTrainerIds sBattleDomeFunctions sWinStreakFlags sWinStreakMasks sIdToOpponentId sTourneyTreeTrainerOpponentIds sIdToMatchNumber sLastMatchCardNum sTrainerAndRoundToLastMatchCardNum sTournamentIdToPairedTrainerIds sBattleDomePotentialTexts sBattleDomeOpponentStyleTexts sBattleDomeOpponentStatsTexts sInfoTrainerMonX sInfoTrainerMonY sSpeciesNameTextYCoords sStatTextOffsets sBattleDomeMatchNumberTexts sBattleDomeWinTexts sLeftTrainerMonX sLeftTrainerMonY sRightTrainerMonX sRightTrainerMonY sTourneyTreeTrainerIds2 sCompetitorRangeByMatch sTrainerNamePositions sTourneyTreePokeballCoords sLineSectionTrainer1Round1 sLineSectionTrainer1Round2 sLineSectionTrainer1Semifinal sLineSectionTrainer1Final sLineSectionTrainer9Round1 sLineSectionTrainer9Round2 sLineSectionTrainer9Semifinal sLineSectionTrainer9Final sLineSectionTrainer13Round1 sLineSectionTrainer13Round2 sLineSectionTrainer13Semifinal sLineSectionTrainer13Final sLineSectionTrainer5Round1 sLineSectionTrainer5Round2 sLineSectionTrainer5Semifinal sLineSectionTrainer5Final sLineSectionTrainer8Round1 sLineSectionTrainer8Round2 sLineSectionTrainer8Semifinal sLineSectionTrainer8Final sLineSectionTrainer16Round1 sLineSectionTrainer16Round2 sLineSectionTrainer16Semifinal sLineSectionTrainer16Final sLineSectionTrainer12Round1 sLineSectionTrainer12Round2 sLineSectionTrainer12Semifinal sLineSectionTrainer12Final sLineSectionTrainer4Round1 sLineSectionTrainer4Round2 sLineSectionTrainer4Semifinal sLineSectionTrainer4Final sLineSectionTrainer3Round1 sLineSectionTrainer3Round2 sLineSectionTrainer3Semifinal sLineSectionTrainer3Final sLineSectionTrainer11Round1 sLineSectionTrainer11Round2 sLineSectionTrainer11Semifinal sLineSectionTrainer11Final sLineSectionTrainer15Round1 sLineSectionTrainer15Round2 sLineSectionTrainer15Semifinal sLineSectionTrainer15Final sLineSectionTrainer7Round1 sLineSectionTrainer7Round2 sLineSectionTrainer7Semifinal sLineSectionTrainer7Final sLineSectionTrainer6Round1 sLineSectionTrainer6Round2 sLineSectionTrainer6Semifinal sLineSectionTrainer6Final sLineSectionTrainer14Round1 sLineSectionTrainer14Round2 sLineSectionTrainer14Semifinal sLineSectionTrainer14Final sLineSectionTrainer10Round1 sLineSectionTrainer10Round2 sLineSectionTrainer10Semifinal sLineSectionTrainer10Final sLineSectionTrainer2Round1 sLineSectionTrainer2Round2 sLineSectionTrainer2Semifinal sLineSectionTrainer2Final sTourneyTreeLineSections sTourneyTreeLineSectionArrayCounts

/// `struct TourneyTreeInfoCard`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct TourneyTreeInfoCard {
    pub spriteIds: CArray<u8, 16>,
    pub pos: u8,
    pub tournamentIds: CArray<u8, 2>,
}

unsafe impl Sync for TourneyTreeInfoCard {}

/// `struct TourneyTreeLineSection`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct TourneyTreeLineSection {
    pub x: u8,
    pub y: u8,
    pub tile: u16,
}

unsafe impl Sync for TourneyTreeLineSection {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<TourneyTreeInfoCard>() == 20);
    assert!(offset_of!(TourneyTreeInfoCard, spriteIds) == 0);
    assert!(offset_of!(TourneyTreeInfoCard, pos) == 16);
    assert!(offset_of!(TourneyTreeInfoCard, tournamentIds) == 17);
    assert!(size_of::<TourneyTreeLineSection>() == 4);
    assert!(offset_of!(TourneyTreeLineSection, x) == 0);
    assert!(offset_of!(TourneyTreeLineSection, y) == 1);
    assert!(offset_of!(TourneyTreeLineSection, tile) == 2);
};

const EFFECTIVENESS_MODE_AI_VS_AI: i32 = 2;
const EFFECTIVENESS_MODE_BAD: i32 = 1;
const EFFECTIVENESS_MODE_GOOD: i32 = 0;
const MOVE_DIR_DOWN: i32 = 1;
const MOVE_DIR_LEFT: i32 = 2;
const MOVE_DIR_NONE: i32 = 4;
const MOVE_DIR_RIGHT: i32 = 3;
const MOVE_DIR_UP: i32 = 0;
const NUM_INFOCARD_SPRITES: i32 = 16;
const NUM_INFOCARD_TRAINERS: i32 = 2;
const NUM_INFO_CARD_WINDOWS: i32 = 9;
const STATE_CLOSE_CARD: i16 = 8;
const STATE_CLOSE_TOURNEY_TREE: i16 = 4;
const STATE_DELAY: i16 = 2;
const STATE_FADE_IN: i16 = 0;
const STATE_GET_INPUT: i16 = 2;
const STATE_MOVE_DOWN: i16 = 5;
const STATE_MOVE_LEFT: i16 = 6;
const STATE_MOVE_RIGHT: i16 = 7;
const STATE_MOVE_UP: i16 = 4;
const STATE_REACT_INPUT: i16 = 3;
const STATE_SHOW_INFOCARD_MATCH: i16 = 5;
const STATE_SHOW_INFOCARD_TRAINER: i16 = 3;
const STATE_SHOW_RESULTS: i16 = 1;
const STATE_WAIT_FADE: i16 = 1;
const STATE_WAIT_FOR_INPUT: i16 = 3;
const TOURNEYWIN_NAMES_LEFT: u8 = 0;
const TOURNEYWIN_NAMES_RIGHT: u8 = 1;
const TOURNEYWIN_TITLE: u8 = 2;
const TYPE_x0: i32 = 0;
const TYPE_x0_25: i32 = 5;
const TYPE_x0_50: i32 = 10;
const TYPE_x1: i32 = 20;
const TYPE_x2: i32 = 40;
const TYPE_x4: i32 = 80;
const WIN_MATCH_NUMBER: u8 = 5;
const WIN_MATCH_TRAINER_NAME_LEFT: u8 = 6;
const WIN_MATCH_TRAINER_NAME_RIGHT: u8 = 7;
const WIN_MATCH_WIN_TEXT: u8 = 8;
const WIN_TRAINER_FLAVOR_TEXT: u8 = 4;
const WIN_TRAINER_MON1_NAME: u8 = 1;
const WIN_TRAINER_NAME: i32 = 0;
const WONDER_GUARD_EFFECTIVENESS: u8 = 40;

static sBattleDomeFunctions: Table<CArray<Option<unsafe extern "C" fn()>, 23>> =
    Table((&raw const crate::data::battle_dome::sBattleDomeFunctions).cast());
static sBattleDomeMatchNumberTexts: Table<CArray<*mut u8, 15>> =
    Table((&raw const crate::data::battle_dome::sBattleDomeMatchNumberTexts).cast());
static sBattleDomeOpponentStatsTexts: Table<CArray<*mut u8, 43>> =
    Table((&raw const crate::data::battle_dome::sBattleDomeOpponentStatsTexts).cast());
static sBattleDomeOpponentStyleTexts: Table<CArray<*mut u8, 32>> =
    Table((&raw const crate::data::battle_dome::sBattleDomeOpponentStyleTexts).cast());
static sBattleDomePotentialTexts: Table<CArray<*mut u8, 17>> =
    Table((&raw const crate::data::battle_dome::sBattleDomePotentialTexts).cast());
static sBattleDomeWinTexts: Table<CArray<*mut u8, 7>> =
    Table((&raw const crate::data::battle_dome::sBattleDomeWinTexts).cast());
static sBattleStyleMovePoints: Table<CArray<CArray<u8, 16>, 355>> =
    Table((&raw const crate::data::battle_dome::sBattleStyleMovePoints).cast());
static sBattleStyleThresholds: Table<CArray<CArray<u8, 16>, 31>> =
    Table((&raw const crate::data::battle_dome::sBattleStyleThresholds).cast());
static sCancelButtonSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_dome::sCancelButtonSpriteTemplate).cast());
static sCompetitorRangeByMatch: Table<CArray<CArray<u8, 3>, 15>> =
    Table((&raw const crate::data::battle_dome::sCompetitorRangeByMatch).cast());
static sExitButtonSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_dome::sExitButtonSpriteTemplate).cast());
static sHorizontalScrollArrowSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_dome::sHorizontalScrollArrowSpriteTemplate).cast());
static sIdToMatchNumber: Table<CArray<CArray<u8, 4>, 16>> =
    Table((&raw const crate::data::battle_dome::sIdToMatchNumber).cast());
static sIdToOpponentId: Table<CArray<CArray<u8, 4>, 16>> =
    Table((&raw const crate::data::battle_dome::sIdToOpponentId).cast());
static sInfoCardBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::battle_dome::sInfoCardBgTemplates).cast());
static sInfoCardWindowTemplates: Table<CArray<WindowTemplate, 19>> =
    Table((&raw const crate::data::battle_dome::sInfoCardWindowTemplates).cast());
static sInfoTrainerMonX: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_dome::sInfoTrainerMonX).cast());
static sInfoTrainerMonY: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_dome::sInfoTrainerMonY).cast());
static sLastMatchCardNum: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_dome::sLastMatchCardNum).cast());
static sLeftTrainerMonX: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_dome::sLeftTrainerMonX).cast());
static sLeftTrainerMonY: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_dome::sLeftTrainerMonY).cast());
static sRightTrainerMonX: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_dome::sRightTrainerMonX).cast());
static sRightTrainerMonY: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_dome::sRightTrainerMonY).cast());
static sSpeciesNameTextYCoords: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_dome::sSpeciesNameTextYCoords).cast());
static sStatTextOffsets: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::battle_dome::sStatTextOffsets).cast());
static sTournamentIdToPairedTrainerIds: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::battle_dome::sTournamentIdToPairedTrainerIds).cast());
static sTourneyTreeBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::battle_dome::sTourneyTreeBgTemplates).cast());
static sTourneyTreeButtonsSpriteSheet: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::battle_dome::sTourneyTreeButtonsSpriteSheet).cast());
static sTourneyTreeCursorMovementMap: Table<CArray<CArray<CArray<u8, 4>, 5>, 32>> =
    Table((&raw const crate::data::battle_dome::sTourneyTreeCursorMovementMap).cast());
static sTourneyTreeLineSectionArrayCounts: Table<CArray<CArray<u8, 4>, 16>> =
    Table((&raw const crate::data::battle_dome::sTourneyTreeLineSectionArrayCounts).cast());
static sTourneyTreeLineSections: Table<CArray<CArray<*mut TourneyTreeLineSection, 4>, 16>> =
    Table((&raw const crate::data::battle_dome::sTourneyTreeLineSections).cast());
static sTourneyTreePokeballCoords: Table<CArray<CArray<u8, 2>, 31>> =
    Table((&raw const crate::data::battle_dome::sTourneyTreePokeballCoords).cast());
static sTourneyTreePokeballSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_dome::sTourneyTreePokeballSpriteTemplate).cast());
static sTourneyTreeScanlineEffectParams: Table<ScanlineEffectParams> =
    Table((&raw const crate::data::battle_dome::sTourneyTreeScanlineEffectParams).cast());
static sTourneyTreeTrainerIds: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::battle_dome::sTourneyTreeTrainerIds).cast());
static sTourneyTreeTrainerIds2: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::battle_dome::sTourneyTreeTrainerIds2).cast());
static sTourneyTreeTrainerOpponentIds: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::battle_dome::sTourneyTreeTrainerOpponentIds).cast());
static sTourneyTreeWindowTemplates: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::battle_dome::sTourneyTreeWindowTemplates).cast());
static sTrainerAndRoundToLastMatchCardNum: Table<CArray<CArray<u8, 4>, 8>> =
    Table((&raw const crate::data::battle_dome::sTrainerAndRoundToLastMatchCardNum).cast());
static sTrainerNamePositions: Table<CArray<CArray<u8, 2>, 16>> =
    Table((&raw const crate::data::battle_dome::sTrainerNamePositions).cast());
static sVerticalScrollArrowSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_dome::sVerticalScrollArrowSpriteTemplate).cast());
static sWinStreakFlags: Table<CArray<CArray<u32, 2>, 2>> =
    Table((&raw const crate::data::battle_dome::sWinStreakFlags).cast());
static sWinStreakMasks: Table<CArray<CArray<u32, 2>, 2>> =
    Table((&raw const crate::data::battle_dome::sWinStreakMasks).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerPartyLostHP: u32 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlayerPartyMaxHP: u32 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInfoCard: *mut TourneyTreeInfoCard = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTilemapBuffer: *mut u8 = null_mut();

unsafe extern "C" {
    static gBattleFrontierHeldItems: CArray<u16, 0>;
    static gBattleFrontierMons: CArray<FacilityMon, 0>;
    static gBattleFrontierTrainers: CArray<BattleFrontierTrainer, 0>;
    static gBattleMoves: CArray<BattleMove, 0>;
    static mut gBattleOutcome: u8;
    static mut gBattleResults: BattleResults;
    static gBattleWindowTextPalette: CArray<u32, 0>;
    static mut gBattle_BG0_X: u16;
    static mut gBattle_BG0_Y: u16;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattle_BG2_X: u16;
    static mut gBattle_BG2_Y: u16;
    static mut gBattle_BG3_X: u16;
    static mut gBattle_BG3_Y: u16;
    static gBitTable: CArray<u32, 0>;
    static mut gDisplayedStringBattle: CArray<u8, 300>;
    static gDomeTourneyInfoCardBg_Tilemap: CArray<u32, 0>;
    static gDomeTourneyInfoCard_Gfx: CArray<u32, 0>;
    static gDomeTourneyInfoCard_Tilemap: CArray<u32, 0>;
    static gDomeTourneyLineDown_Tilemap: CArray<u32, 0>;
    static gDomeTourneyLineUp_Tilemap: CArray<u32, 0>;
    static gDomeTourneyLine_Gfx: CArray<u32, 0>;
    static gDomeTourneyMatchCardBg_Pal: CArray<u32, 0>;
    static gDomeTourneyTreeButtons_Pal: CArray<u32, 0>;
    static gDomeTourneyTree_Gfx: CArray<u32, 0>;
    static gDomeTourneyTree_Pal: CArray<u32, 0>;
    static gDomeTourneyTree_Tilemap: CArray<u32, 0>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static gFacilityClassToTrainerClass: CArray<u8, 0>;
    static mut gFacilityTrainerMons: *mut FacilityMon;
    static mut gFacilityTrainers: *mut BattleFrontierTrainer;
    static mut gMain: Main;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static gNatureStatTable: CArray<CArray<i8, 5>, 0>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gReservedSpritePaletteCount: u8;
    static gRoundsStringTable: CArray<*mut u8, 0>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gScanlineEffectRegBuffers: CArray<CArray<u16, 960>, 2>;
    static mut gSelectedOrderFromParty: CArray<u8, 4>;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_Result: u16;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_BattleTourney: CArray<u8, 0>;
    static mut gTrainerBattleOpponent_A: u16;
    static gTrainerClassNames: CArray<CArray<u8, 13>, 0>;
    static gTrainers: CArray<Trainer, 0>;
    static gTypeEffectiveness: CArray<u8, 336>;
    fn AI_TypeCalc(a0: u16, a1: u16, a2: u8) -> u8;
    fn AddTextPrinter(
        a0: *mut TextPrinterTemplate,
        a1: u8,
        a2: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AllocZeroed(a0: u32) -> *mut c_void;
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
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateMonIcon(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut Sprite)>,
        a2: i16,
        a3: i16,
        a4: u8,
        a5: u32,
        a6: u32,
    ) -> u8;
    fn CreateMonWithEVSpreadNatureOTID(
        a0: *mut Pokemon,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u32,
    );
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTrainerPicSprite(a0: u16, a1: u8, a2: i16, a3: i16, a4: u8, a5: u16) -> u16;
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut c_void, a2: u32, a3: u16, a4: u8);
    fn DestroyTask(a0: u8);
    fn EnableInterrupts(a0: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeAndDestroyMonIconSprite(a0: *mut Sprite);
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
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetNature(a0: *mut Pokemon) -> u8;
    fn GetNatureFromPersonality(a0: u32) -> u8;
    fn GetRandomFrontierMonFromSet(a0: u16) -> u16;
    fn GetRandomScaledFrontierTrainerId(a0: u8, a1: u8) -> u16;
    fn GetStringCenterAlignXOffsetWithLetterSpacing(a0: i32, a1: *mut u8, a2: i32, a3: i32) -> i32;
    fn GetStringWidthDifference(a0: i32, a1: *mut u8, a2: i32, a3: i32) -> i32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
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
    fn ScanlineEffect_SetParams(a0: ScanlineEffectParams);
    fn ScanlineEffect_Stop();
    fn SetBattleFacilityTrainerGfxId(a0: u16, a1: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetDynamicWarp(a0: i32, a1: i8, a2: i8, a3: i8);
    fn SetFacilityPtrsGetLevel() -> u8;
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMonMoveSlot(a0: *mut Pokemon, a1: u16, a2: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdateMonIconFrame(a0: *mut Sprite) -> u8;
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn ZeroEnemyPartyMons();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallBattleDomeFunction() {
    sBattleDomeFunctions[gSpecialVar_0x8004].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn InitDomeChallenge() {
    let mut lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let mut battleMode: u32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u32;
    (*gSaveBlock2Ptr).frontier.challengeStatus = 0;
    (*gSaveBlock2Ptr).frontier.curChallengeBattleNum = 0;
    (*gSaveBlock2Ptr).frontier.set_challengePaused(FALSE);
    (*gSaveBlock2Ptr).frontier.set_disableRecordBattle(FALSE);
    if (*gSaveBlock2Ptr).frontier.winStreakActiveFlags & sWinStreakFlags[battleMode][lvlMode] == 0 {
        (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode] = 0;
    }
    SetDynamicWarp(
        0,
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
        WARP_ID_NONE,
    );
    gTrainerBattleOpponent_A = 0;
}
pub(crate) unsafe extern "C" fn GetDomeData() {
    let mut lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let mut battleMode: u32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u32;
    match gSpecialVar_0x8005 {
        DOME_DATA_WIN_STREAK => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode];
        }
        DOME_DATA_WIN_STREAK_ACTIVE => {
            gSpecialVar_Result = ((*gSaveBlock2Ptr).frontier.winStreakActiveFlags
                & sWinStreakFlags[battleMode][lvlMode]
                != 0) as u16;
        }
        DOME_DATA_ATTEMPTED_SINGLES_50 => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.domeAttemptedSingles50() as u16;
        }
        DOME_DATA_ATTEMPTED_SINGLES_OPEN => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.domeAttemptedSinglesOpen() as u16;
        }
        DOME_DATA_HAS_WON_SINGLES_50 => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.domeHasWonSingles50() as u16;
        }
        DOME_DATA_HAS_WON_SINGLES_OPEN => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.domeHasWonSinglesOpen() as u16;
        }
        DOME_DATA_ATTEMPTED_CHALLENGE => {
            if VarGet(VAR_FRONTIER_BATTLE_MODE) == FRONTIER_MODE_DOUBLES {
                if lvlMode != FRONTIER_LVL_50 as u32 {
                    gSpecialVar_Result =
                        (*gSaveBlock2Ptr).frontier.domeAttemptedDoublesOpen() as u16;
                } else {
                    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.domeAttemptedDoubles50() as u16;
                }
            } else {
                if lvlMode != FRONTIER_LVL_50 as u32 {
                    gSpecialVar_Result =
                        (*gSaveBlock2Ptr).frontier.domeAttemptedSinglesOpen() as u16;
                } else {
                    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.domeAttemptedSingles50() as u16;
                }
            }
        }
        DOME_DATA_HAS_WON_CHALLENGE => {
            if VarGet(VAR_FRONTIER_BATTLE_MODE) == FRONTIER_MODE_DOUBLES {
                if lvlMode != FRONTIER_LVL_50 as u32 {
                    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.domeHasWonDoublesOpen() as u16;
                } else {
                    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.domeHasWonDoubles50() as u16;
                }
            } else {
                if lvlMode != FRONTIER_LVL_50 as u32 {
                    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.domeHasWonSinglesOpen() as u16;
                } else {
                    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.domeHasWonSingles50() as u16;
                }
            }
        }
        DOME_DATA_SELECTED_MONS => {
            ClearSelectedPartyOrder();
            gSelectedOrderFromParty[0] = (*gSaveBlock2Ptr).frontier.selectedPartyMons[3] as u8;
            gSelectedOrderFromParty[1] =
                ((*gSaveBlock2Ptr).frontier.selectedPartyMons[3] >> 8) as u8;
        }
        DOME_DATA_PREV_TOURNEY_TYPE => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.domeLvlMode as u16 * 2 - 3
                + (*gSaveBlock2Ptr).frontier.domeBattleMode as u16;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetDomeData() {
    let mut lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let mut battleMode: u32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u32;
    match gSpecialVar_0x8005 {
        DOME_DATA_WIN_STREAK => {
            (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode] = gSpecialVar_0x8006;
        }
        DOME_DATA_WIN_STREAK_ACTIVE => {
            if gSpecialVar_0x8006 != 0 {
                (*gSaveBlock2Ptr).frontier.winStreakActiveFlags |=
                    sWinStreakFlags[battleMode][lvlMode];
            } else {
                (*gSaveBlock2Ptr).frontier.winStreakActiveFlags &=
                    sWinStreakMasks[battleMode][lvlMode];
            }
        }
        DOME_DATA_ATTEMPTED_SINGLES_50 => {
            (*gSaveBlock2Ptr)
                .frontier
                .set_domeAttemptedSingles50(gSpecialVar_0x8006 as u8);
        }
        DOME_DATA_ATTEMPTED_SINGLES_OPEN => {
            (*gSaveBlock2Ptr)
                .frontier
                .set_domeAttemptedSinglesOpen(gSpecialVar_0x8006 as u8);
        }
        DOME_DATA_HAS_WON_SINGLES_50 => {
            (*gSaveBlock2Ptr)
                .frontier
                .set_domeHasWonSingles50(gSpecialVar_0x8006 as u8);
        }
        DOME_DATA_HAS_WON_SINGLES_OPEN => {
            (*gSaveBlock2Ptr)
                .frontier
                .set_domeHasWonSinglesOpen(gSpecialVar_0x8006 as u8);
        }
        DOME_DATA_ATTEMPTED_CHALLENGE => {
            if VarGet(VAR_FRONTIER_BATTLE_MODE) == FRONTIER_MODE_DOUBLES {
                if lvlMode != FRONTIER_LVL_50 as u32 {
                    (*gSaveBlock2Ptr)
                        .frontier
                        .set_domeAttemptedDoublesOpen(gSpecialVar_0x8006 as u8);
                } else {
                    (*gSaveBlock2Ptr)
                        .frontier
                        .set_domeAttemptedDoubles50(gSpecialVar_0x8006 as u8);
                }
            } else {
                if lvlMode != FRONTIER_LVL_50 as u32 {
                    (*gSaveBlock2Ptr)
                        .frontier
                        .set_domeAttemptedSinglesOpen(gSpecialVar_0x8006 as u8);
                } else {
                    (*gSaveBlock2Ptr)
                        .frontier
                        .set_domeAttemptedSingles50(gSpecialVar_0x8006 as u8);
                }
            }
        }
        DOME_DATA_HAS_WON_CHALLENGE => {
            if VarGet(VAR_FRONTIER_BATTLE_MODE) == FRONTIER_MODE_DOUBLES {
                if lvlMode != FRONTIER_LVL_50 as u32 {
                    (*gSaveBlock2Ptr)
                        .frontier
                        .set_domeHasWonDoublesOpen(gSpecialVar_0x8006 as u8);
                } else {
                    (*gSaveBlock2Ptr)
                        .frontier
                        .set_domeHasWonDoubles50(gSpecialVar_0x8006 as u8);
                }
            } else {
                if lvlMode != FRONTIER_LVL_50 as u32 {
                    (*gSaveBlock2Ptr)
                        .frontier
                        .set_domeHasWonSinglesOpen(gSpecialVar_0x8006 as u8);
                } else {
                    (*gSaveBlock2Ptr)
                        .frontier
                        .set_domeHasWonSingles50(gSpecialVar_0x8006 as u8);
                }
            }
        }
        DOME_DATA_SELECTED_MONS => {
            (*gSaveBlock2Ptr).frontier.selectedPartyMons[3] =
                gSelectedOrderFromParty[0] as u16 | (gSelectedOrderFromParty[1] as u16) << 8;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn InitDomeTrainers() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut monLevel: i32 = 0;
    let mut species: CArray<i32, 3> = zeroed();
    let mut monTypesBits: i32 = 0;
    let mut monTypesCount: i32 = 0;
    let mut trainerId: i32 = 0;
    let mut monId: i32 = 0;
    let mut rankingScores: *mut u16 = null_mut();
    let mut statValues: *mut i32 = null_mut();
    let mut ivs: u8 = 0;
    species[0] = 0;
    species[1] = 0;
    species[2] = 0;
    rankingScores = AllocZeroed(32) as *mut u16;
    statValues = AllocZeroed(24) as *mut i32;
    (*gSaveBlock2Ptr).frontier.domeLvlMode = (*gSaveBlock2Ptr).frontier.lvlMode() + 1;
    (*gSaveBlock2Ptr).frontier.domeBattleMode = VarGet(VAR_FRONTIER_BATTLE_MODE) as u8 + 1;
    (*gSaveBlock2Ptr).frontier.domeTrainers[0].set_trainerId(TRAINER_PLAYER);
    (*gSaveBlock2Ptr).frontier.domeTrainers[0].set_isEliminated(0);
    (*gSaveBlock2Ptr).frontier.domeTrainers[0].set_eliminatedAt(0);
    (*gSaveBlock2Ptr).frontier.domeTrainers[0].set_forfeited(0);
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        (*gSaveBlock2Ptr).frontier.domeMonIds[0][i] = GetMonData3(
            &raw mut gPlayerParty[(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1],
            MON_DATA_SPECIES,
            null_mut(),
        ) as u16;
        j = 0;
        while j < MAX_MON_MOVES {
            (*gSaveBlock2Ptr).frontier.domePlayerPartyData[i].moves[j] = GetMonData3(
                &raw mut gPlayerParty[(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1],
                MON_DATA_MOVE1 + j,
                null_mut(),
            ) as u16;
            j += 1;
        }
        j = 0;
        while j < NUM_STATS {
            (*gSaveBlock2Ptr).frontier.domePlayerPartyData[i].evs[j] = GetMonData3(
                &raw mut gPlayerParty[(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1],
                MON_DATA_HP_EV + j,
                null_mut(),
            ) as u8;
            j += 1;
        }
        (*gSaveBlock2Ptr).frontier.domePlayerPartyData[i].nature = GetNature(
            &raw mut gPlayerParty[(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1],
        );
        i += 1;
    }
    i = 1;
    while i < DOME_TOURNAMENT_TRAINERS_COUNT {
        if i > 5 {
            loop {
                trainerId =
                    GetRandomScaledFrontierTrainerId(GetCurrentFacilityWinStreak() as u8, 0) as i32;
                j = 1;
                while j < i {
                    if (*gSaveBlock2Ptr).frontier.domeTrainers[j].trainerId() as i32 == trainerId {
                        break;
                    }
                    j += 1;
                }
                if j == i {
                    break;
                }
            }
            (*gSaveBlock2Ptr).frontier.domeTrainers[i].set_trainerId(trainerId as u16);
        } else {
            loop {
                trainerId =
                    GetRandomScaledFrontierTrainerId(GetCurrentFacilityWinStreak() as u8 + 1, 0)
                        as i32;
                j = 1;
                while j < i {
                    if (*gSaveBlock2Ptr).frontier.domeTrainers[j].trainerId() as i32 == trainerId {
                        break;
                    }
                    j += 1;
                }
                if j == i {
                    break;
                }
            }
            (*gSaveBlock2Ptr).frontier.domeTrainers[i].set_trainerId(trainerId as u16);
        }
        j = 0;
        while j < FRONTIER_PARTY_SIZE {
            loop {
                monId = GetRandomFrontierMonFromSet(trainerId as u16) as i32;
                k = 0;
                while k < j {
                    let mut alreadySelectedMonId: i32 =
                        (*gSaveBlock2Ptr).frontier.domeMonIds[i][k] as i32;
                    if alreadySelectedMonId == monId
                        || species[0] == (*gFacilityTrainerMons.at(monId)).species as i32
                        || species[1] == (*gFacilityTrainerMons.at(monId)).species as i32
                        || (*gFacilityTrainerMons.at(alreadySelectedMonId)).itemTableId
                            == (*gFacilityTrainerMons.at(monId)).itemTableId
                    {
                        break;
                    }
                    k += 1;
                }
                if k == j {
                    break;
                }
            }
            (*gSaveBlock2Ptr).frontier.domeMonIds[i][j] = monId as u16;
            species[j] = (*gFacilityTrainerMons.at(monId)).species as i32;
            j += 1;
        }
        (*gSaveBlock2Ptr).frontier.domeTrainers[i].set_isEliminated(FALSE as u16);
        (*gSaveBlock2Ptr).frontier.domeTrainers[i].set_eliminatedAt(0);
        (*gSaveBlock2Ptr).frontier.domeTrainers[i].set_forfeited(FALSE as u16);
        i += 1;
    }
    monTypesBits = 0;
    *rankingScores = 0;
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        trainerId = (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1;
        *rankingScores +=
            GetMonData3(&raw mut gPlayerParty[trainerId], MON_DATA_ATK, null_mut()) as u16;
        *rankingScores +=
            GetMonData3(&raw mut gPlayerParty[trainerId], MON_DATA_DEF, null_mut()) as u16;
        *rankingScores +=
            GetMonData3(&raw mut gPlayerParty[trainerId], MON_DATA_SPATK, null_mut()) as u16;
        *rankingScores +=
            GetMonData3(&raw mut gPlayerParty[trainerId], MON_DATA_SPDEF, null_mut()) as u16;
        *rankingScores +=
            GetMonData3(&raw mut gPlayerParty[trainerId], MON_DATA_SPEED, null_mut()) as u16;
        *rankingScores += GetMonData3(
            &raw mut gPlayerParty[trainerId],
            MON_DATA_MAX_HP,
            null_mut(),
        ) as u16;
        monTypesBits |= gBitTable[gSpeciesInfo[GetMonData3(
            &raw mut gPlayerParty[trainerId],
            MON_DATA_SPECIES,
            null_mut(),
        )]
        .types[0]] as i32;
        monTypesBits |= gBitTable[gSpeciesInfo[GetMonData3(
            &raw mut gPlayerParty[trainerId],
            MON_DATA_SPECIES,
            null_mut(),
        )]
        .types[1]] as i32;
        i += 1;
    }
    monTypesCount = 0;
    j = 0;
    while j < 32 {
        if monTypesBits & 1 != 0 {
            monTypesCount += 1;
        }
        monTypesBits >>= 1;
        j += 1;
    }
    monLevel = SetFacilityPtrsGetLevel() as i32;
    *rankingScores += (monTypesCount * monLevel / 20) as u16;
    i = 1;
    while i < DOME_TOURNAMENT_TRAINERS_COUNT {
        monTypesBits = 0;
        *rankingScores.at(i) = 0;
        ivs = GetDomeTrainerMonIvs((*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId());
        j = 0;
        while j < FRONTIER_PARTY_SIZE {
            CalcDomeMonStats(
                (*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.domeMonIds[i][j])).species,
                monLevel,
                ivs as i32,
                (*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.domeMonIds[i][j])).evSpread,
                (*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.domeMonIds[i][j])).nature,
                statValues,
            );
            *rankingScores.at(i) += *statValues.at(1) as u16;
            *rankingScores.at(i) += *statValues.at(2) as u16;
            *rankingScores.at(i) += *statValues.at(4) as u16;
            *rankingScores.at(i) += *statValues.at(5) as u16;
            *rankingScores.at(i) += *statValues.at(3) as u16;
            *rankingScores.at(i) += *statValues as u16;
            monTypesBits |= gBitTable[gSpeciesInfo
                [(*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.domeMonIds[i][j])).species]
                .types[0]] as i32;
            monTypesBits |= gBitTable[gSpeciesInfo
                [(*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.domeMonIds[i][j])).species]
                .types[1]] as i32;
            j += 1;
        }
        monTypesCount = 0;
        j = 0;
        while j < 32 {
            if monTypesBits & 1 != 0 {
                monTypesCount += 1;
            }
            monTypesBits >>= 1;
            j += 1;
        }
        *rankingScores.at(i) += (monTypesCount * monLevel / 20) as u16;
        i += 1;
    }
    i = 0;
    while i < 15 {
        j = i + 1;
        while j < DOME_TOURNAMENT_TRAINERS_COUNT {
            if *rankingScores.at(i) < *rankingScores.at(j) {
                SwapDomeTrainers(i, j, rankingScores);
            } else {
                if *rankingScores.at(i) == *rankingScores.at(j) {
                    if (*gSaveBlock2Ptr).frontier.domeTrainers[j].trainerId() == TRAINER_PLAYER {
                        SwapDomeTrainers(i, j, rankingScores);
                    } else if (*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId()
                        > (*gSaveBlock2Ptr).frontier.domeTrainers[j].trainerId()
                    {
                        SwapDomeTrainers(i, j, rankingScores);
                    }
                }
            }
            j += 1;
        }
        i += 1;
    }
    if GetFrontierBrainStatus() != FRONTIER_BRAIN_NOT_READY {
        i = 0;
        while i < DOME_TOURNAMENT_TRAINERS_COUNT {
            if (*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId() == TRAINER_PLAYER {
                break;
            }
            i += 1;
        }
        if sTrainerNamePositions[i][0] != TOURNEYWIN_NAMES_LEFT {
            j = 0;
            (*gSaveBlock2Ptr).frontier.domeTrainers[j].set_trainerId(TRAINER_FRONTIER_BRAIN);
        } else {
            j = 1;
            (*gSaveBlock2Ptr).frontier.domeTrainers[j].set_trainerId(TRAINER_FRONTIER_BRAIN);
        }
        i = 0;
        while i < FRONTIER_PARTY_SIZE {
            (*gSaveBlock2Ptr).frontier.domeMonIds[j][i] = GetFrontierBrainMonSpecies(i as u8);
            i += 1;
        }
    }
    Free(rankingScores as *mut c_void);
    Free(statValues as *mut c_void);
}
pub(crate) unsafe extern "C" fn CalcDomeMonStats(
    species: u16,
    level: i32,
    ivs: i32,
    evBits: u8,
    nature: u8,
    mut stats: *mut i32,
) {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut bits: u8 = 0;
    let mut resultingEvs: u16 = 0;
    let mut evs: CArray<i32, 6> = zeroed();
    count = 0;
    bits = evBits;
    i = 0;
    while i < NUM_STATS {
        if bits as i32 & 1 != 0 {
            count += 1;
        }
        bits >>= 1;
        i += 1;
    }
    resultingEvs = div_i32(MAX_TOTAL_EVS, count) as u16;
    i = 0;
    while i < NUM_STATS {
        evs[i] = 0;
        if evBits as i32 & bits as i32 != 0 {
            evs[i] = resultingEvs as i32;
        }
        bits <<= 1;
        i += 1;
    }
    if species == SPECIES_SHEDINJA {
        *stats = 1;
    } else {
        let mut n: i32 = 2 * gSpeciesInfo[species].baseHP as i32;
        *stats = (n + ivs + evs[0] / 4) * level / 100 + level + 10;
    }
    {
        let mut baseStat: u8 = gSpeciesInfo[species].baseAttack;
        *stats.at(1) = (2 * baseStat as i32 + ivs + evs[1] / 4) * level / 100 + 5;
        *stats.at(1) = ModifyStatByNature(nature, *stats.at(1) as u16, STAT_ATK) as u8 as i32;
    }
    {
        let mut baseStat: u8 = gSpeciesInfo[species].baseDefense;
        *stats.at(2) = (STAT_DEF * baseStat as i32 + ivs + evs[2] / 4) * level / 100 + 5;
        *stats.at(2) = ModifyStatByNature(nature, *stats.at(2) as u16, STAT_DEF as u8) as u8 as i32;
    }
    {
        let mut baseStat: u8 = gSpeciesInfo[species].baseSpeed;
        *stats.at(3) = (2 * baseStat as i32 + ivs + evs[3] / 4) * level / 100 + 5;
        *stats.at(3) = ModifyStatByNature(nature, *stats.at(3) as u16, STAT_SPEED) as u8 as i32;
    }
    {
        let mut baseStat: u8 = gSpeciesInfo[species].baseSpAttack;
        *stats.at(4) = (2 * baseStat as i32 + ivs + evs[4] / 4) * level / 100 + 5;
        *stats.at(4) = ModifyStatByNature(nature, *stats.at(4) as u16, STAT_SPATK) as u8 as i32;
    }
    {
        let mut baseStat: u8 = gSpeciesInfo[species].baseSpDefense;
        *stats.at(5) = (2 * baseStat as i32 + ivs + evs[5] / 4) * level / 100 + STAT_SPDEF;
        *stats.at(5) =
            ModifyStatByNature(nature, *stats.at(5) as u16, STAT_SPDEF as u8) as u8 as i32;
    }
}
pub(crate) unsafe extern "C" fn SwapDomeTrainers(id1: i32, id2: i32, mut statsArray: *mut u16) {
    let mut i: i32 = 0;
    let mut temp: u16 = 0;
    temp = *statsArray.at(id1);
    *statsArray.at(id1) = *statsArray.at(id2);
    *statsArray.at(id2) = temp;
    temp = (*gSaveBlock2Ptr).frontier.domeTrainers[id1].trainerId();
    (*gSaveBlock2Ptr).frontier.domeTrainers[id1]
        .set_trainerId((*gSaveBlock2Ptr).frontier.domeTrainers[id2].trainerId());
    (*gSaveBlock2Ptr).frontier.domeTrainers[id2].set_trainerId(temp);
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        temp = (*gSaveBlock2Ptr).frontier.domeMonIds[id1][i];
        (*gSaveBlock2Ptr).frontier.domeMonIds[id1][i] =
            (*gSaveBlock2Ptr).frontier.domeMonIds[id2][i];
        (*gSaveBlock2Ptr).frontier.domeMonIds[id2][i] = temp;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn BufferDomeRoundText() {
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gRoundsStringTable[(*gSaveBlock2Ptr).frontier.curChallengeBattleNum],
    );
}
pub(crate) unsafe extern "C" fn BufferDomeOpponentName() {
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gRoundsStringTable[(*gSaveBlock2Ptr).frontier.curChallengeBattleNum],
    );
    CopyDomeTrainerName(gStringVar2.as_mut_ptr(), gTrainerBattleOpponent_A);
}
pub(crate) unsafe extern "C" fn InitDomeOpponentParty() {
    gPlayerPartyLostHP = 0;
    sPlayerPartyMaxHP = GetMonData3(&raw mut gPlayerParty[0], MON_DATA_MAX_HP, null_mut());
    sPlayerPartyMaxHP += GetMonData3(&raw mut gPlayerParty[1], MON_DATA_MAX_HP, null_mut());
    CalculatePlayerPartyCount();
    CreateDomeOpponentMons(TrainerIdToTournamentId(gTrainerBattleOpponent_A) as u16);
}
pub(crate) unsafe extern "C" fn CreateDomeOpponentMon(
    monPartyId: u8,
    tournamentTrainerId: u16,
    tournamentMonId: u8,
    otId: u32,
) {
    let mut i: i32 = 0;
    let mut friendship: u8 = MAX_FRIENDSHIP;
    let mut fixedIv: u8 = GetDomeTrainerMonIvs(tournamentTrainerId);
    let mut level: u8 = SetFacilityPtrsGetLevel();
    CreateMonWithEVSpreadNatureOTID(
        &raw mut gEnemyParty[monPartyId],
        (*gFacilityTrainerMons
            .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentTrainerId][tournamentMonId]))
        .species,
        level,
        (*gFacilityTrainerMons
            .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentTrainerId][tournamentMonId]))
        .nature,
        fixedIv,
        (*gFacilityTrainerMons
            .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentTrainerId][tournamentMonId]))
        .evSpread,
        otId,
    );
    friendship = MAX_FRIENDSHIP;
    i = 0;
    while i < MAX_MON_MOVES {
        SetMonMoveSlot(
            &raw mut gEnemyParty[monPartyId],
            (*gFacilityTrainerMons
                .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentTrainerId][tournamentMonId]))
            .moves[i],
            i as u8,
        );
        if (*gFacilityTrainerMons
            .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentTrainerId][tournamentMonId]))
        .moves[i]
            == MOVE_FRUSTRATION
        {
            friendship = 0;
        }
        i += 1;
    }
    SetMonData(
        &raw mut gEnemyParty[monPartyId],
        MON_DATA_FRIENDSHIP,
        &raw mut friendship as *mut c_void,
    );
    SetMonData(
        &raw mut gEnemyParty[monPartyId],
        MON_DATA_HELD_ITEM,
        (&raw const gBattleFrontierHeldItems[(*gFacilityTrainerMons
            .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentTrainerId][tournamentMonId]))
        .itemTableId])
            .cast_mut() as *mut c_void,
    );
}
pub(crate) unsafe extern "C" fn CreateDomeOpponentMons(tournamentTrainerId: u16) {
    let mut monsCount: u8 = 0;
    let mut otId: u32 = 0;
    let mut i: i32 = 0;
    let mut selectedMonBits: i32 = 0;
    ZeroEnemyPartyMons();
    selectedMonBits = GetDomeTrainerSelectedMons(tournamentTrainerId);
    otId = Random() as u32 | (Random() as u32) << 16;
    if Random() as i32 % 10 > 5 {
        i = 0;
        while i < FRONTIER_PARTY_SIZE {
            if selectedMonBits & 1 != 0 {
                CreateDomeOpponentMon(monsCount, tournamentTrainerId, i as u8, otId);
                monsCount += 1;
            }
            selectedMonBits >>= 1;
            i += 1;
        }
    } else {
        i = 2;
        while i >= 0 {
            if selectedMonBits & 4 != 0 {
                CreateDomeOpponentMon(monsCount, tournamentTrainerId, i as u8, otId);
                monsCount += 1;
            }
            selectedMonBits <<= 1;
            i -= 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDomeTrainerSelectedMons(tournamentTrainerId: u16) -> i32 {
    let mut selectedMonBits: i32 = 0;
    if Random() as i32 & 1 != 0 {
        selectedMonBits = SelectOpponentMons_Good(tournamentTrainerId, FALSE);
        if selectedMonBits == 0 {
            selectedMonBits = SelectOpponentMons_Bad(tournamentTrainerId, TRUE);
        }
    } else {
        selectedMonBits = SelectOpponentMons_Bad(tournamentTrainerId, FALSE);
        if selectedMonBits == 0 {
            selectedMonBits = SelectOpponentMons_Good(tournamentTrainerId, TRUE);
        }
    }
    return selectedMonBits;
}
pub(crate) unsafe extern "C" fn SelectOpponentMons_Good(
    tournamentTrainerId: u16,
    allowRandom: u8,
) -> i32 {
    let mut i: i32 = 0;
    let mut moveIndex: i32 = 0;
    let mut playerMonId: i32 = 0;
    let mut partyMovePoints: CArray<i32, 3> = zeroed();
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        partyMovePoints[i] = 0;
        moveIndex = 0;
        while moveIndex < MAX_MON_MOVES {
            playerMonId = 0;
            while playerMonId < FRONTIER_PARTY_SIZE {
                if (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentTrainerId].trainerId()
                    == TRAINER_FRONTIER_BRAIN
                {
                    partyMovePoints[i] += GetTypeEffectivenessPoints(
                        GetFrontierBrainMonMove(i as u8, moveIndex as u8) as i32,
                        GetMonData3(
                            &raw mut gPlayerParty[playerMonId],
                            MON_DATA_SPECIES,
                            null_mut(),
                        ) as i32,
                        EFFECTIVENESS_MODE_GOOD,
                    );
                } else {
                    partyMovePoints[i] += GetTypeEffectivenessPoints(
                        (*gFacilityTrainerMons
                            .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentTrainerId][i]))
                        .moves[moveIndex] as i32,
                        GetMonData3(
                            &raw mut gPlayerParty[playerMonId],
                            MON_DATA_SPECIES,
                            null_mut(),
                        ) as i32,
                        EFFECTIVENESS_MODE_GOOD,
                    );
                }
                playerMonId += 1;
            }
            moveIndex += 1;
        }
        i += 1;
    }
    return SelectOpponentMonsFromParty(partyMovePoints.as_mut_ptr(), allowRandom);
}
pub(crate) unsafe extern "C" fn SelectOpponentMons_Bad(
    tournamentTrainerId: u16,
    allowRandom: u8,
) -> i32 {
    let mut i: i32 = 0;
    let mut moveIndex: i32 = 0;
    let mut playerMonId: i32 = 0;
    let mut partyMovePoints: CArray<i32, 3> = zeroed();
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        partyMovePoints[i] = 0;
        moveIndex = 0;
        while moveIndex < MAX_MON_MOVES {
            playerMonId = 0;
            while playerMonId < FRONTIER_PARTY_SIZE {
                if (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentTrainerId].trainerId()
                    == TRAINER_FRONTIER_BRAIN
                {
                    partyMovePoints[i] += GetTypeEffectivenessPoints(
                        GetFrontierBrainMonMove(i as u8, moveIndex as u8) as i32,
                        GetMonData3(
                            &raw mut gPlayerParty[playerMonId],
                            MON_DATA_SPECIES,
                            null_mut(),
                        ) as i32,
                        EFFECTIVENESS_MODE_BAD,
                    );
                } else {
                    partyMovePoints[i] += GetTypeEffectivenessPoints(
                        (*gFacilityTrainerMons
                            .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentTrainerId][i]))
                        .moves[moveIndex] as i32,
                        GetMonData3(
                            &raw mut gPlayerParty[playerMonId],
                            MON_DATA_SPECIES,
                            null_mut(),
                        ) as i32,
                        EFFECTIVENESS_MODE_BAD,
                    );
                }
                playerMonId += 1;
            }
            moveIndex += 1;
        }
        i += 1;
    }
    return SelectOpponentMonsFromParty(partyMovePoints.as_mut_ptr(), allowRandom);
}
pub(crate) unsafe extern "C" fn SelectOpponentMonsFromParty(
    mut partyMovePoints: *mut i32,
    allowRandom: u8,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut selectedMonBits: i32 = 0;
    let mut partyPositions: CArray<i32, 3> = zeroed();
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        partyPositions[i] = i;
        i += 1;
    }
    if *partyMovePoints == *partyMovePoints.at(1) && *partyMovePoints == *partyMovePoints.at(2) {
        if allowRandom != 0 {
            i = 0;
            while i != DOME_BATTLE_PARTY_SIZE {
                let mut rand: u32 = Random() as u32 & FRONTIER_PARTY_SIZE as u32;
                if rand != FRONTIER_PARTY_SIZE as u32
                    && selectedMonBits as u32 & gBitTable[rand] == 0
                {
                    selectedMonBits |= gBitTable[rand] as i32;
                    i += 1;
                }
            }
        }
    } else {
        i = 0;
        while i < DOME_BATTLE_PARTY_SIZE {
            j = i + 1;
            while j < FRONTIER_PARTY_SIZE {
                let mut temp: i32 = 0;
                if *partyMovePoints.at(i) < *partyMovePoints.at(j) {
                    temp = *partyMovePoints.at(i);
                    *partyMovePoints.at(i) = *partyMovePoints.at(j);
                    *partyMovePoints.at(j) = temp;
                    temp = partyPositions[i];
                    partyPositions[i] = partyPositions[j];
                    partyPositions[j] = temp;
                }
                if *partyMovePoints.at(i) == *partyMovePoints.at(j) && Random() as i32 & 1 != 0 {
                    temp = *partyMovePoints.at(i);
                    *partyMovePoints.at(i) = *partyMovePoints.at(j);
                    *partyMovePoints.at(j) = temp;
                    temp = partyPositions[i];
                    partyPositions[i] = partyPositions[j];
                    partyPositions[j] = temp;
                }
                j += 1;
            }
            i += 1;
        }
        i = 0;
        while i < DOME_BATTLE_PARTY_SIZE {
            selectedMonBits |= gBitTable[partyPositions[i]] as i32;
            i += 1;
        }
    }
    return selectedMonBits;
}
pub(crate) unsafe extern "C" fn GetTypeEffectivenessPoints(
    r#move: i32,
    targetSpecies: i32,
    mode: i32,
) -> i32 {
    let mut defType1: i32 = 0;
    let mut defType2: i32 = 0;
    let mut defAbility: i32 = 0;
    let mut moveType: i32 = 0;
    let mut i: i32 = 0;
    let mut typePower: i32 = TYPE_x1;
    if r#move == 0 || r#move == MOVE_UNAVAILABLE as i32 || gBattleMoves[r#move].power == 0 {
        return 0;
    }
    defType1 = gSpeciesInfo[targetSpecies].types[0] as i32;
    defType2 = gSpeciesInfo[targetSpecies].types[1] as i32;
    defAbility = gSpeciesInfo[targetSpecies].abilities[0] as i32;
    moveType = gBattleMoves[r#move].r#type as i32;
    if defAbility == ABILITY_LEVITATE as i32 && moveType == TYPE_GROUND as i32 {
        if mode == EFFECTIVENESS_MODE_BAD {
            typePower = 8;
        }
    } else {
        while gTypeEffectiveness[i + 0] != TYPE_ENDTABLE {
            if gTypeEffectiveness[i + 0] == TYPE_FORESIGHT {
                i += 3;
                continue;
            }
            if gTypeEffectiveness[i + 0] as i32 == moveType {
                if gTypeEffectiveness[i + 1] as i32 == defType1 {
                    if defAbility == ABILITY_WONDER_GUARD as i32
                        && gTypeEffectiveness[i + 2] == WONDER_GUARD_EFFECTIVENESS
                        || defAbility != ABILITY_WONDER_GUARD as i32
                    {
                        typePower = typePower * gTypeEffectiveness[i + 2] as i32 / 10;
                    }
                }
                if gTypeEffectiveness[i + 1] as i32 == defType2 && defType1 != defType2 {
                    if defAbility == ABILITY_WONDER_GUARD as i32
                        && gTypeEffectiveness[i + 2] == WONDER_GUARD_EFFECTIVENESS
                        || defAbility != ABILITY_WONDER_GUARD as i32
                    {
                        typePower = typePower * gTypeEffectiveness[i + 2] as i32 / 10;
                    }
                }
            }
            i += 3;
        }
    }
    match mode {
        EFFECTIVENESS_MODE_GOOD => match typePower {
            TYPE_x1 => {
                typePower = 2;
            }
            TYPE_x2 => {
                typePower = 4;
            }
            TYPE_x4 => {
                typePower = 8;
            }
            _ => {
                typePower = 0;
            }
        },
        EFFECTIVENESS_MODE_BAD => match typePower {
            TYPE_x0 => {
                typePower = 8;
            }
            TYPE_x0_25 => {
                typePower = 4;
            }
            TYPE_x0_50 => {
                typePower = 2;
            }
            TYPE_x2 => {
                typePower = -2;
            }
            TYPE_x4 => {
                typePower = -4;
            }
            _ => {
                typePower = 0;
            }
        },
        EFFECTIVENESS_MODE_AI_VS_AI => match typePower {
            TYPE_x0 => {
                typePower = -16;
            }
            TYPE_x0_25 => {
                typePower = -8;
            }
            TYPE_x1 => {
                typePower = 4;
            }
            TYPE_x2 => {
                typePower = 12;
            }
            TYPE_x4 => {
                typePower = 20;
            }
            _ => {
                typePower = 0;
            }
        },
        _ => {}
    }
    return typePower;
}
pub(crate) unsafe extern "C" fn GetDomeTrainerMonIvs(trainerId: u16) -> u8 {
    let mut fixedIv: u8 = 0;
    if trainerId <= 99 {
        fixedIv = 3;
    } else if trainerId <= 119 {
        fixedIv = 6;
    } else if trainerId <= 139 {
        fixedIv = 9;
    } else if trainerId <= 159 {
        fixedIv = 12;
    } else if trainerId <= 179 {
        fixedIv = 15;
    } else if trainerId <= 199 {
        fixedIv = 18;
    } else if trainerId <= 219 {
        fixedIv = 21;
    } else {
        fixedIv = MAX_PER_STAT_IVS;
    }
    return fixedIv;
}
pub(crate) unsafe extern "C" fn TournamentIdOfOpponent(roundId: i32, trainerId: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut opponentMax: i32 = 0;
    i = 0;
    while i < DOME_TOURNAMENT_TRAINERS_COUNT {
        if (*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId() as i32 == trainerId {
            break;
        }
        i += 1;
    }
    if roundId != DOME_ROUND1 as i32 {
        if roundId == DOME_FINAL as i32 {
            opponentMax = sIdToOpponentId[i][roundId] as i32 + 8;
        } else {
            opponentMax = sIdToOpponentId[i][roundId] as i32 + 4;
        }
        j = sIdToOpponentId[i][roundId] as i32;
        while j < opponentMax {
            if sTourneyTreeTrainerOpponentIds[j] as i32 != i
                && (*gSaveBlock2Ptr).frontier.domeTrainers[sTourneyTreeTrainerOpponentIds[j]]
                    .isEliminated()
                    == 0
            {
                break;
            }
            j += 1;
        }
        if j != opponentMax {
            return sTourneyTreeTrainerOpponentIds[j] as i32;
        } else {
            return 0xFF;
        }
    } else {
        if (*gSaveBlock2Ptr).frontier.domeTrainers[sIdToOpponentId[i][roundId]].isEliminated() == 0
        {
            return sIdToOpponentId[i][roundId] as i32;
        } else {
            return 0xFF;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetDomeOpponentId() {
    gTrainerBattleOpponent_A = TrainerIdOfPlayerOpponent();
}
pub(crate) unsafe extern "C" fn TrainerIdOfPlayerOpponent() -> u16 {
    return (*gSaveBlock2Ptr).frontier.domeTrainers[TournamentIdOfOpponent(
        (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32,
        TRAINER_PLAYER as i32,
    )]
    .trainerId();
}
pub(crate) unsafe extern "C" fn SetDomeOpponentGraphicsId() {
    SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_A, 0);
}
pub(crate) unsafe extern "C" fn SaveDomeChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveGameFrontier();
}
pub(crate) unsafe extern "C" fn IncrementDomeStreaks() {
    let mut lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    let mut battleMode: u8 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u8;
    if (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode] < 999 {
        (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode] += 1;
    }
    if (*gSaveBlock2Ptr).frontier.domeTotalChampionships[battleMode][lvlMode] < 999 {
        (*gSaveBlock2Ptr).frontier.domeTotalChampionships[battleMode][lvlMode] += 1;
    }
    if (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode]
        > (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[battleMode][lvlMode]
    {
        (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[battleMode][lvlMode] =
            (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode];
    }
}
pub(crate) unsafe extern "C" fn ShowDomeOpponentInfo() {
    let mut taskId: u8 = CreateTask(Some(Task_ShowTourneyInfoCard), 0);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = TrainerIdToTournamentId(TrainerIdOfPlayerOpponent()) as i16;
    gTasks[taskId].data[2] = INFOCARD_NEXT_OPPONENT;
    gTasks[taskId].data[3] = 0;
    SetMainCallback2(Some(CB2_TourneyTree));
}
pub(crate) unsafe extern "C" fn Task_ShowTourneyInfoCard(taskId: u8) {
    let mut i: i32 = 0;
    let mut tournamentId: i32 = gTasks[taskId].data[1] as i32;
    let mut mode: i32 = gTasks[taskId].data[2] as i32;
    let mut id: i32 = gTasks[taskId].data[3] as i32;
    match gTasks[taskId].data[0] {
        0 => {
            SetHBlankCallback(None);
            SetVBlankCallback(None);
            EnableInterrupts(INTR_FLAG_VBLANK);
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        VRAM as usize as *mut c_void,
                        0x5006000,
                    );
                }
            }
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sInfoCardBgTemplates.as_ptr().cast_mut(), 4);
            InitWindows(sInfoCardWindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            gBattle_BG0_X = 0;
            gBattle_BG0_Y = 0;
            gBattle_BG1_X = 0;
            gBattle_BG1_Y = 0;
            gBattle_BG3_X = 0;
            gBattle_BG3_Y = 0;
            if mode == INFOCARD_MATCH {
                gBattle_BG2_X = 0;
                gBattle_BG2_Y = 0;
            } else {
                gBattle_BG2_X = 0;
                gBattle_BG2_Y = DISPLAY_HEIGHT;
            }
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(REG_OFFSET_MOSAIC, 0);
            SetGpuReg(REG_OFFSET_WIN0H, 0);
            SetGpuReg(REG_OFFSET_WIN0V, 0);
            SetGpuReg(REG_OFFSET_WIN1H, 0);
            SetGpuReg(REG_OFFSET_WIN1V, 0);
            SetGpuReg(REG_OFFSET_WININ, 0);
            SetGpuReg(REG_OFFSET_WINOUT, 63);
            ResetPaletteFade();
            ResetSpriteData();
            FreeAllSpritePalettes();
            gReservedSpritePaletteCount = 4;
            gTasks[taskId].data[0] += 1;
        }
        2 => {
            DecompressAndLoadBgGfxUsingHeap(
                2,
                gDomeTourneyInfoCard_Gfx.as_ptr().cast_mut() as *mut c_void,
                0x2000,
                0,
                0,
            );
            DecompressAndLoadBgGfxUsingHeap(
                2,
                gDomeTourneyInfoCard_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0x2000,
                0,
                1,
            );
            DecompressAndLoadBgGfxUsingHeap(
                3,
                gDomeTourneyInfoCardBg_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0x800,
                0,
                1,
            );
            LoadCompressedSpriteSheet(sTourneyTreeButtonsSpriteSheet.as_ptr().cast_mut());
            LoadCompressedPalette(
                gDomeTourneyTree_Pal.as_ptr().cast_mut(),
                BG_PLTT_OFFSET,
                BG_PLTT_SIZE,
            );
            LoadCompressedPalette(
                gDomeTourneyTreeButtons_Pal.as_ptr().cast_mut(),
                OBJ_PLTT_OFFSET,
                OBJ_PLTT_SIZE,
            );
            LoadCompressedPalette(gBattleWindowTextPalette.as_ptr().cast_mut(), 240, 32);
            if mode == INFOCARD_MATCH {
                LoadCompressedPalette(gDomeTourneyMatchCardBg_Pal.as_ptr().cast_mut(), 80, 32);
            }
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        gPlttBufferFaded.as_mut_ptr() as *mut c_void,
                        0x5000100,
                    );
                }
            }
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            gTasks[taskId].data[0] += 1;
        }
        3 => {
            SetVBlankCallback(Some(VblankCb_TourneyInfoCard));
            sInfoCard = AllocZeroed(20) as *mut TourneyTreeInfoCard;
            i = 0;
            while i < NUM_INFOCARD_SPRITES {
                (*sInfoCard).spriteIds[i] = SPRITE_NONE;
                i += 1;
            }
            LoadMonIconPalettes();
            i = CreateTask(Some(Task_HandleInfoCardInput), 0) as i32;
            gTasks[i].data[0] = 0;
            gTasks[i].data[2] = 0;
            gTasks[i].data[3] = mode as i16;
            gTasks[i].data[4] = id as i16;
            if mode == INFOCARD_MATCH {
                DisplayMatchInfoOnCard(0, tournamentId as u8);
                (*sInfoCard).pos = 1;
            } else {
                DisplayTrainerInfoOnCard(0, tournamentId as u8);
            }
            SetGpuReg(REG_OFFSET_DISPCNT, 8000);
            if mode != INFOCARD_NEXT_OPPONENT as i32 {
                id = CreateSprite(
                    (&raw const *sVerticalScrollArrowSpriteTemplate).cast_mut(),
                    120,
                    4,
                    0,
                ) as i32;
                StartSpriteAnim(&raw mut gSprites[id], 0);
                gSprites[id].data[0] = i as i16;
                id = CreateSprite(
                    (&raw const *sVerticalScrollArrowSpriteTemplate).cast_mut(),
                    120,
                    156,
                    0,
                ) as i32;
                StartSpriteAnim(&raw mut gSprites[id], 1);
                gSprites[id].data[0] = i as i16;
                id = CreateSprite(
                    (&raw const *sHorizontalScrollArrowSpriteTemplate).cast_mut(),
                    6,
                    80,
                    0,
                ) as i32;
                StartSpriteAnim(&raw mut gSprites[id], 0);
                gSprites[id].data[0] = i as i16;
                gSprites[id].data[1] = 0;
                if mode == INFOCARD_TRAINER as i32 {
                    gSprites[id].set_invisible(TRUE as u16);
                }
                id = CreateSprite(
                    (&raw const *sHorizontalScrollArrowSpriteTemplate).cast_mut(),
                    234,
                    80,
                    0,
                ) as i32;
                StartSpriteAnim(&raw mut gSprites[id], 1);
                gSprites[id].data[0] = i as i16;
                gSprites[id].data[1] = 1;
            }
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerIconCardScrollUp(sprite: *mut Sprite) {
    (*sprite).y += 4;
    if (*sprite).data[0] != 0 {
        if (*sprite).y >= -32 {
            (*sprite).set_invisible(FALSE as u16);
        }
        if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) == 40
        {
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
    } else {
        if (*sprite).y >= 192 {
            (*sInfoCard).spriteIds[(*sprite).data[2]] = SPRITE_NONE;
            FreeAndDestroyTrainerPicSprite((*sprite).data[3] as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerIconCardScrollDown(sprite: *mut Sprite) {
    (*sprite).y -= 4;
    if (*sprite).data[0] != 0 {
        if (*sprite).y <= 192 {
            (*sprite).set_invisible(FALSE as u16);
        }
        if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) == 40
        {
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
    } else {
        if (*sprite).y <= -32 {
            (*sInfoCard).spriteIds[(*sprite).data[2]] = SPRITE_NONE;
            FreeAndDestroyTrainerPicSprite((*sprite).data[3] as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerIconCardScrollLeft(sprite: *mut Sprite) {
    (*sprite).x += 4;
    if (*sprite).data[0] != 0 {
        if (*sprite).x >= -32 {
            (*sprite).set_invisible(FALSE as u16);
        }
        if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) == 64
        {
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
    } else {
        if (*sprite).x >= 272 {
            (*sInfoCard).spriteIds[(*sprite).data[2]] = SPRITE_NONE;
            FreeAndDestroyTrainerPicSprite((*sprite).data[3] as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerIconCardScrollRight(sprite: *mut Sprite) {
    (*sprite).x -= 4;
    if (*sprite).data[0] != 0 {
        if (*sprite).x <= 272 {
            (*sprite).set_invisible(FALSE as u16);
        }
        if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) == 64
        {
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
    } else {
        if (*sprite).x <= -32 {
            (*sInfoCard).spriteIds[(*sprite).data[2]] = SPRITE_NONE;
            FreeAndDestroyTrainerPicSprite((*sprite).data[3] as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIconDomeInfo(sprite: *mut Sprite) {
    if (*sprite).data[3] == 0 {
        UpdateMonIconFrame(sprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIconCardScrollUp(sprite: *mut Sprite) {
    if (*sprite).data[3] == 0 {
        UpdateMonIconFrame(sprite);
    }
    (*sprite).y += 4;
    if (*sprite).data[0] != 0 {
        if (*sprite).y >= -16 {
            (*sprite).set_invisible(FALSE as u16);
        }
        if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) == 40
        {
            (*sprite).callback = Some(SpriteCB_MonIconDomeInfo);
        }
    } else {
        if (*sprite).y >= 176 {
            (*sInfoCard).spriteIds[(*sprite).data[2]] = SPRITE_NONE;
            FreeAndDestroyMonIconSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIconCardScrollDown(sprite: *mut Sprite) {
    if (*sprite).data[3] == 0 {
        UpdateMonIconFrame(sprite);
    }
    (*sprite).y -= 4;
    if (*sprite).data[0] != 0 {
        if (*sprite).y <= 176 {
            (*sprite).set_invisible(FALSE as u16);
        }
        if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) == 40
        {
            (*sprite).callback = Some(SpriteCB_MonIconDomeInfo);
        }
    } else {
        if (*sprite).y <= -16 {
            (*sInfoCard).spriteIds[(*sprite).data[2]] = SPRITE_NONE;
            FreeAndDestroyMonIconSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIconCardScrollLeft(sprite: *mut Sprite) {
    if (*sprite).data[3] == 0 {
        UpdateMonIconFrame(sprite);
    }
    (*sprite).x += 4;
    if (*sprite).data[0] != 0 {
        if (*sprite).x >= -16 {
            (*sprite).set_invisible(FALSE as u16);
        }
        if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) == 64
        {
            (*sprite).callback = Some(SpriteCB_MonIconDomeInfo);
        }
    } else {
        if (*sprite).x >= 256 {
            (*sInfoCard).spriteIds[(*sprite).data[2]] = SPRITE_NONE;
            FreeAndDestroyMonIconSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIconCardScrollRight(sprite: *mut Sprite) {
    if (*sprite).data[3] == 0 {
        UpdateMonIconFrame(sprite);
    }
    (*sprite).x -= 4;
    if (*sprite).data[0] != 0 {
        if (*sprite).x <= 256 {
            (*sprite).set_invisible(FALSE as u16);
        }
        if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) == 64
        {
            (*sprite).callback = Some(SpriteCB_MonIconDomeInfo);
        }
    } else {
        if (*sprite).x <= -16 {
            (*sInfoCard).spriteIds[(*sprite).data[2]] = SPRITE_NONE;
            FreeAndDestroyMonIconSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HorizontalScrollArrow(sprite: *mut Sprite) {
    let mut taskId1: i32 = (*sprite).data[0] as i32;
    let mut arrId: i32 = gTasks[gTasks[taskId1].data[4]].data[1] as i32;
    let mut tournmanetTrainerId: i32 = sTourneyTreeTrainerIds[arrId] as i32;
    let mut roundId: i32 = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32;
    if gTasks[taskId1].data[3] == 1 {
        if (*sprite).data[1] != 0 {
            if (*gSaveBlock2Ptr).frontier.domeTrainers[tournmanetTrainerId].isEliminated() != 0
                && (*sInfoCard).pos as i32 - 1
                    < (*gSaveBlock2Ptr).frontier.domeTrainers[tournmanetTrainerId].eliminatedAt()
                        as i32
            {
                (*sprite).set_invisible(FALSE as u16);
            } else if (*gSaveBlock2Ptr).frontier.domeTrainers[tournmanetTrainerId].isEliminated()
                == 0
                && (*sInfoCard).pos as i32 - 1 < roundId
            {
                (*sprite).set_invisible(FALSE as u16);
            } else {
                if gTasks[taskId1].data[0] == 2 {
                    (*sprite).set_invisible(TRUE as u16);
                }
            }
        } else {
            if (*sInfoCard).pos != 0 {
                (*sprite).set_invisible(FALSE as u16);
            } else {
                if gTasks[taskId1].data[0] == 2 {
                    (*sprite).set_invisible(TRUE as u16);
                }
            }
        }
    } else {
        if (*sprite).data[1] != 0 {
            if (*sInfoCard).pos > 1 {
                if gTasks[taskId1].data[0] == 2 {
                    (*sprite).set_invisible(TRUE as u16);
                }
            } else {
                (*sprite).set_invisible(FALSE as u16);
            }
        } else {
            if (*sInfoCard).pos != 0 {
                (*sprite).set_invisible(FALSE as u16);
            } else {
                if gTasks[taskId1].data[0] == 2 {
                    (*sprite).set_invisible(TRUE as u16);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_VerticalScrollArrow(sprite: *mut Sprite) {
    let mut taskId1: i32 = (*sprite).data[0] as i32;
    if gTasks[taskId1].data[3] == 1 {
        if (*sInfoCard).pos != 0 {
            if gTasks[taskId1].data[0] == 2 {
                (*sprite).set_invisible(TRUE as u16);
            }
        } else {
            (*sprite).set_invisible(FALSE as u16);
        }
    } else {
        if (*sInfoCard).pos != 1 {
            if gTasks[taskId1].data[0] == 2 {
                (*sprite).set_invisible(TRUE as u16);
            }
        } else {
            (*sprite).set_invisible(FALSE as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleInfoCardInput(taskId: u8) {
    let mut i: i32 = 0;
    let mut windowId: i32 = 0;
    let mut mode: i32 = gTasks[taskId].data[3] as i32;
    let mut taskId2: i32 = gTasks[taskId].data[4] as i32;
    let mut trainerTourneyId: i32 = 0;
    let mut matchNo: i32 = 0;
    match gTasks[taskId].data[0] {
        STATE_FADE_IN => {
            if gPaletteFade.active() == 0 {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
                gTasks[taskId].data[0] = STATE_WAIT_FADE;
            }
        }
        STATE_WAIT_FADE => {
            if gPaletteFade.active() == 0 {
                gTasks[taskId].data[0] = STATE_GET_INPUT;
            }
        }
        STATE_GET_INPUT => {
            i = Task_GetInfoCardInput(taskId) as i32;
            match i {
                9 => {
                    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                    gTasks[taskId].data[0] = STATE_CLOSE_CARD;
                }
                1..=4 | 5..=8 => {
                    gTasks[taskId].data[5] = i as i16;
                    if gTasks[taskId].data[2] != 0 {
                        windowId = NUM_INFO_CARD_WINDOWS;
                    } else {
                        windowId = 0;
                    }
                    i = windowId;
                    while i < windowId + NUM_INFO_CARD_WINDOWS {
                        CopyWindowToVram(i as u8, COPYWIN_GFX);
                        FillWindowPixelBuffer(i as u8, 0);
                        i += 1;
                    }
                    gTasks[taskId].data[0] = STATE_REACT_INPUT;
                }
                0 => {}
                _ => {}
            }
        }
        STATE_REACT_INPUT => {
            i = gTasks[taskId].data[5] as i32;
            match i {
                TRAINERCARD_INPUT_UP | MATCHCARD_INPUT_UP => {
                    if gTasks[taskId].data[2] != 0 {
                        gBattle_BG0_X = 0;
                        gBattle_BG0_Y = 0;
                        gBattle_BG1_X = 0;
                        gBattle_BG1_Y = DISPLAY_HEIGHT;
                    } else {
                        gBattle_BG0_X = 0;
                        gBattle_BG0_Y = DISPLAY_HEIGHT;
                        gBattle_BG1_X = 0;
                        gBattle_BG1_Y = 0;
                    }
                    if i == TRAINERCARD_INPUT_UP {
                        if (*sInfoCard).pos == 0 {
                            gBattle_BG2_X = 0;
                            gBattle_BG2_Y = 320;
                            trainerTourneyId =
                                sTourneyTreeTrainerIds[gTasks[taskId2].data[1]] as i32;
                            DisplayTrainerInfoOnCard(
                                gTasks[taskId].data[2] as u8 | MOVE_CARD_UP,
                                trainerTourneyId as u8,
                            );
                        } else {
                            gBattle_BG2_X = 256;
                            gBattle_BG2_Y = 0;
                            trainerTourneyId =
                                sTourneyTreeTrainerIds[gTasks[taskId2].data[1]] as i32;
                            DisplayTrainerInfoOnCard(
                                gTasks[taskId].data[2] as u8 | MOVE_CARD_UP,
                                trainerTourneyId as u8,
                            );
                            (*sInfoCard).pos = 0;
                        }
                    } else {
                        if (*sInfoCard).pos == 0 {
                            matchNo = gTasks[taskId2].data[1] as i32 - 16;
                            BufferDomeWinString(
                                matchNo as u8,
                                (*sInfoCard).tournamentIds.as_mut_ptr(),
                            );
                            gBattle_BG2_X = 0;
                            gBattle_BG2_Y = 320;
                            trainerTourneyId = (*sInfoCard).tournamentIds[0] as i32;
                            DisplayTrainerInfoOnCard(
                                gTasks[taskId].data[2] as u8 | MOVE_CARD_UP,
                                trainerTourneyId as u8,
                            );
                        } else if (*sInfoCard).pos == 2 {
                            matchNo = gTasks[taskId2].data[1] as i32 - 16;
                            BufferDomeWinString(
                                matchNo as u8,
                                (*sInfoCard).tournamentIds.as_mut_ptr(),
                            );
                            gBattle_BG2_X = 0;
                            gBattle_BG2_Y = 320;
                            trainerTourneyId = (*sInfoCard).tournamentIds[1] as i32;
                            DisplayTrainerInfoOnCard(
                                gTasks[taskId].data[2] as u8 | MOVE_CARD_UP,
                                trainerTourneyId as u8,
                            );
                        } else {
                            gBattle_BG2_X = 256;
                            gBattle_BG2_Y = DISPLAY_HEIGHT;
                            matchNo = gTasks[taskId2].data[1] as i32 - 16;
                            DisplayMatchInfoOnCard(
                                gTasks[taskId].data[2] as u8 | MOVE_CARD_UP,
                                matchNo as u8,
                            );
                        }
                    }
                    i = 0;
                    while i < 8 {
                        if i < 2 {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_TrainerIconCardScrollUp);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2] ^ 1;
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                                gSprites[(*sInfoCard).spriteIds[i]].data[3] =
                                    (*sInfoCard).spriteIds[i] as i16;
                            }
                        } else {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_MonIconCardScrollUp);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2] ^ 1;
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                            }
                        }
                        i += 1;
                    }
                    i = 8;
                    while i < NUM_INFOCARD_SPRITES {
                        if i < 10 {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_TrainerIconCardScrollUp);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2];
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                                gSprites[(*sInfoCard).spriteIds[i]].data[3] =
                                    (*sInfoCard).spriteIds[i] as i16;
                            }
                        } else {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_MonIconCardScrollUp);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2];
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                            }
                        }
                        i += 1;
                    }
                    gTasks[taskId].data[0] = STATE_MOVE_UP;
                    gTasks[taskId].data[5] = 0;
                }
                TRAINERCARD_INPUT_DOWN | MATCHCARD_INPUT_DOWN => {
                    if gTasks[taskId].data[2] != 0 {
                        gBattle_BG0_X = 0;
                        gBattle_BG0_Y = 0;
                        gBattle_BG1_X = 0;
                        gBattle_BG1_Y = 65376;
                    } else {
                        gBattle_BG0_X = 0;
                        gBattle_BG0_Y = 65376;
                        gBattle_BG1_X = 0;
                        gBattle_BG1_Y = 0;
                    }
                    if i == TRAINERCARD_INPUT_DOWN {
                        if (*sInfoCard).pos == 0 {
                            gBattle_BG2_X = 0;
                            gBattle_BG2_Y = DISPLAY_HEIGHT;
                            trainerTourneyId =
                                sTourneyTreeTrainerIds[gTasks[taskId2].data[1]] as i32;
                            DisplayTrainerInfoOnCard(
                                gTasks[taskId].data[2] as u8 | MOVE_CARD_DOWN,
                                trainerTourneyId as u8,
                            );
                        } else {
                            gBattle_BG2_X = 0;
                            gBattle_BG2_Y = 0;
                            trainerTourneyId =
                                sTourneyTreeTrainerIds[gTasks[taskId2].data[1]] as i32;
                            DisplayTrainerInfoOnCard(
                                gTasks[taskId].data[2] as u8 | MOVE_CARD_DOWN,
                                trainerTourneyId as u8,
                            );
                            (*sInfoCard).pos = 0;
                        }
                    } else {
                        if (*sInfoCard).pos == 0 {
                            matchNo = gTasks[taskId2].data[1] as i32 - 16;
                            BufferDomeWinString(
                                matchNo as u8,
                                (*sInfoCard).tournamentIds.as_mut_ptr(),
                            );
                            gBattle_BG2_X = 0;
                            gBattle_BG2_Y = DISPLAY_HEIGHT;
                            trainerTourneyId = (*sInfoCard).tournamentIds[0] as i32;
                            DisplayTrainerInfoOnCard(
                                gTasks[taskId].data[2] as u8 | MOVE_CARD_DOWN,
                                trainerTourneyId as u8,
                            );
                        } else if (*sInfoCard).pos == 2 {
                            matchNo = gTasks[taskId2].data[1] as i32 - 16;
                            BufferDomeWinString(
                                matchNo as u8,
                                (*sInfoCard).tournamentIds.as_mut_ptr(),
                            );
                            gBattle_BG2_X = 0;
                            gBattle_BG2_Y = DISPLAY_HEIGHT;
                            trainerTourneyId = (*sInfoCard).tournamentIds[1] as i32;
                            DisplayTrainerInfoOnCard(
                                gTasks[taskId].data[2] as u8 | MOVE_CARD_DOWN,
                                trainerTourneyId as u8,
                            );
                        } else {
                            gBattle_BG2_X = 256;
                            gBattle_BG2_Y = 0;
                            matchNo = gTasks[taskId2].data[1] as i32 - 16;
                            DisplayMatchInfoOnCard(
                                gTasks[taskId].data[2] as u8 | MOVE_CARD_DOWN,
                                matchNo as u8,
                            );
                        }
                    }
                    i = 0;
                    while i < 8 {
                        if i < 2 {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_TrainerIconCardScrollDown);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2] ^ 1;
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                                gSprites[(*sInfoCard).spriteIds[i]].data[3] =
                                    (*sInfoCard).spriteIds[i] as i16;
                            }
                        } else {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_MonIconCardScrollDown);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2] ^ 1;
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                            }
                        }
                        i += 1;
                    }
                    i = 8;
                    while i < NUM_INFOCARD_SPRITES {
                        if i < 10 {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_TrainerIconCardScrollDown);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2];
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                                gSprites[(*sInfoCard).spriteIds[i]].data[3] =
                                    (*sInfoCard).spriteIds[i] as i16;
                            }
                        } else {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_MonIconCardScrollDown);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2];
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                            }
                        }
                        i += 1;
                    }
                    gTasks[taskId].data[0] = STATE_MOVE_DOWN;
                    gTasks[taskId].data[5] = 0;
                }
                TRAINERCARD_INPUT_LEFT => {
                    if gTasks[taskId].data[2] != 0 {
                        gBattle_BG0_X = 0;
                        gBattle_BG0_Y = 0;
                        gBattle_BG1_X = 256;
                        gBattle_BG1_Y = 0;
                    } else {
                        gBattle_BG0_X = 256;
                        gBattle_BG0_Y = 0;
                        gBattle_BG1_X = 0;
                        gBattle_BG1_Y = 0;
                    }
                    if (*sInfoCard).pos == 0 {
                        gBattle_BG2_X = 256;
                        gBattle_BG2_Y = DISPLAY_HEIGHT;
                        trainerTourneyId = sTourneyTreeTrainerIds[gTasks[taskId2].data[1]] as i32;
                        DisplayTrainerInfoOnCard(
                            gTasks[taskId].data[2] as u8 | MOVE_CARD_LEFT,
                            trainerTourneyId as u8,
                        );
                    } else {
                        gBattle_BG2_X = 256;
                        gBattle_BG2_Y = 0;
                        matchNo = sIdToMatchNumber[gTasks[taskId2].data[1]]
                            [(*sInfoCard).pos as i32 - 1] as i32;
                        DisplayMatchInfoOnCard(
                            gTasks[taskId].data[2] as u8 | MOVE_CARD_LEFT,
                            matchNo as u8,
                        );
                    }
                    i = 0;
                    while i < 8 {
                        if i < 2 {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_TrainerIconCardScrollLeft);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2] ^ 1;
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                                gSprites[(*sInfoCard).spriteIds[i]].data[3] =
                                    (*sInfoCard).spriteIds[i] as i16;
                            }
                        } else {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_MonIconCardScrollLeft);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2] ^ 1;
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                            }
                        }
                        i += 1;
                    }
                    i = 8;
                    while i < NUM_INFOCARD_SPRITES {
                        if i < 10 {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_TrainerIconCardScrollLeft);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2];
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                                gSprites[(*sInfoCard).spriteIds[i]].data[3] =
                                    (*sInfoCard).spriteIds[i] as i16;
                            }
                        } else {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_MonIconCardScrollLeft);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2];
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                            }
                        }
                        i += 1;
                    }
                    gTasks[taskId].data[0] = STATE_MOVE_LEFT;
                    gTasks[taskId].data[5] = 0;
                }
                MATCHCARD_INPUT_LEFT => {
                    if gTasks[taskId].data[2] != 0 {
                        gBattle_BG0_X = 0;
                        gBattle_BG0_Y = 0;
                        gBattle_BG1_X = 256;
                        gBattle_BG1_Y = 0;
                    } else {
                        gBattle_BG0_X = 256;
                        gBattle_BG0_Y = 0;
                        gBattle_BG1_X = 0;
                        gBattle_BG1_Y = 0;
                    }
                    if (*sInfoCard).pos == 0 {
                        gBattle_BG2_X = 256;
                        gBattle_BG2_Y = DISPLAY_HEIGHT;
                        trainerTourneyId = (*sInfoCard).tournamentIds[0] as i32;
                        DisplayTrainerInfoOnCard(
                            gTasks[taskId].data[2] as u8 | MOVE_CARD_LEFT,
                            trainerTourneyId as u8,
                        );
                    } else {
                        gBattle_BG2_X = 0;
                        gBattle_BG2_Y = DISPLAY_HEIGHT;
                        matchNo = gTasks[taskId2].data[1] as i32 - 16;
                        DisplayMatchInfoOnCard(
                            gTasks[taskId].data[2] as u8 | MOVE_CARD_LEFT,
                            matchNo as u8,
                        );
                    }
                    i = 0;
                    while i < 8 {
                        if i < 2 {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_TrainerIconCardScrollLeft);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2] ^ 1;
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                                gSprites[(*sInfoCard).spriteIds[i]].data[3] =
                                    (*sInfoCard).spriteIds[i] as i16;
                            }
                        } else {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_MonIconCardScrollLeft);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2] ^ 1;
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                            }
                        }
                        i += 1;
                    }
                    i = 8;
                    while i < NUM_INFOCARD_SPRITES {
                        if i < 10 {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_TrainerIconCardScrollLeft);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2];
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                                gSprites[(*sInfoCard).spriteIds[i]].data[3] =
                                    (*sInfoCard).spriteIds[i] as i16;
                            }
                        } else {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_MonIconCardScrollLeft);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2];
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                            }
                        }
                        i += 1;
                    }
                    gTasks[taskId].data[0] = STATE_MOVE_LEFT;
                    gTasks[taskId].data[5] = 0;
                }
                4 => {
                    if gTasks[taskId].data[2] != 0 {
                        gBattle_BG0_X = 0;
                        gBattle_BG0_Y = 0;
                        gBattle_BG1_X = 65280;
                        gBattle_BG1_Y = 0;
                    } else {
                        gBattle_BG0_X = 65280;
                        gBattle_BG0_Y = 0;
                        gBattle_BG1_X = 0;
                        gBattle_BG1_Y = 0;
                    }
                    if (*sInfoCard).pos == 1 {
                        gBattle_BG2_X = 0;
                        gBattle_BG2_Y = DISPLAY_HEIGHT;
                    } else {
                        gBattle_BG2_X = 0;
                        gBattle_BG2_Y = 0;
                    }
                    matchNo = sIdToMatchNumber[gTasks[taskId2].data[1]][(*sInfoCard).pos as i32 - 1]
                        as i32;
                    DisplayMatchInfoOnCard(
                        gTasks[taskId].data[2] as u8 | MOVE_CARD_RIGHT,
                        matchNo as u8,
                    );
                    i = 0;
                    while i < 8 {
                        if i < 2 {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_TrainerIconCardScrollRight);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2] ^ 1;
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                                gSprites[(*sInfoCard).spriteIds[i]].data[3] =
                                    (*sInfoCard).spriteIds[i] as i16;
                            }
                        } else {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_MonIconCardScrollRight);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2] ^ 1;
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                            }
                        }
                        i += 1;
                    }
                    i = 8;
                    while i < NUM_INFOCARD_SPRITES {
                        if i < 10 {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_TrainerIconCardScrollRight);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2];
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                                gSprites[(*sInfoCard).spriteIds[i]].data[3] =
                                    (*sInfoCard).spriteIds[i] as i16;
                            }
                        } else {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_MonIconCardScrollRight);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2];
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                            }
                        }
                        i += 1;
                    }
                    gTasks[taskId].data[0] = STATE_MOVE_RIGHT;
                    gTasks[taskId].data[5] = 0;
                }
                MATCHCARD_INPUT_RIGHT => {
                    if gTasks[taskId].data[2] != 0 {
                        gBattle_BG0_X = 0;
                        gBattle_BG0_Y = 0;
                        gBattle_BG1_X = 65280;
                        gBattle_BG1_Y = 0;
                    } else {
                        gBattle_BG0_X = 65280;
                        gBattle_BG0_Y = 0;
                        gBattle_BG1_X = 0;
                        gBattle_BG1_Y = 0;
                    }
                    if (*sInfoCard).pos == 2 {
                        gBattle_BG2_X = 256;
                        gBattle_BG2_Y = DISPLAY_HEIGHT;
                        trainerTourneyId = (*sInfoCard).tournamentIds[1] as i32;
                        DisplayTrainerInfoOnCard(
                            gTasks[taskId].data[2] as u8 | MOVE_CARD_RIGHT,
                            trainerTourneyId as u8,
                        );
                    } else {
                        gBattle_BG2_X = 0;
                        gBattle_BG2_Y = DISPLAY_HEIGHT;
                        matchNo = gTasks[taskId2].data[1] as i32 - 16;
                        DisplayMatchInfoOnCard(
                            gTasks[taskId].data[2] as u8 | MOVE_CARD_RIGHT,
                            matchNo as u8,
                        );
                    }
                    i = 0;
                    while i < 8 {
                        if i < 2 {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_TrainerIconCardScrollRight);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2] ^ 1;
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                                gSprites[(*sInfoCard).spriteIds[i]].data[3] =
                                    (*sInfoCard).spriteIds[i] as i16;
                            }
                        } else {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_MonIconCardScrollRight);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2] ^ 1;
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                            }
                        }
                        i += 1;
                    }
                    i = 8;
                    while i < NUM_INFOCARD_SPRITES {
                        if i < 10 {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_TrainerIconCardScrollRight);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2];
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                                gSprites[(*sInfoCard).spriteIds[i]].data[3] =
                                    (*sInfoCard).spriteIds[i] as i16;
                            }
                        } else {
                            if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                                gSprites[(*sInfoCard).spriteIds[i]].callback =
                                    Some(SpriteCB_MonIconCardScrollRight);
                                gSprites[(*sInfoCard).spriteIds[i]].data[0] =
                                    gTasks[taskId].data[2];
                                gSprites[(*sInfoCard).spriteIds[i]].data[1] = 0;
                                gSprites[(*sInfoCard).spriteIds[i]].data[2] = i as i16;
                            }
                        }
                        i += 1;
                    }
                    gTasks[taskId].data[0] = STATE_MOVE_RIGHT;
                    gTasks[taskId].data[5] = 0;
                }
                _ => {}
            }
        }
        STATE_MOVE_UP => {
            if ({
                gTasks[taskId].data[5] += 1;
                gTasks[taskId].data[5]
            }) != 41
            {
                gBattle_BG0_Y -= 4;
                gBattle_BG1_Y -= 4;
                gBattle_BG2_Y -= 4;
            } else {
                gTasks[taskId].data[0] = STATE_GET_INPUT;
            }
        }
        STATE_MOVE_DOWN => {
            if ({
                gTasks[taskId].data[5] += 1;
                gTasks[taskId].data[5]
            }) != 41
            {
                gBattle_BG0_Y += 4;
                gBattle_BG1_Y += 4;
                gBattle_BG2_Y += 4;
            } else {
                gTasks[taskId].data[0] = STATE_GET_INPUT;
            }
        }
        STATE_MOVE_LEFT => {
            if ({
                gTasks[taskId].data[5] += 1;
                gTasks[taskId].data[5]
            }) != 65
            {
                gBattle_BG0_X -= 4;
                gBattle_BG1_X -= 4;
                gBattle_BG2_X -= 4;
            } else {
                gTasks[taskId].data[0] = STATE_GET_INPUT;
            }
        }
        STATE_MOVE_RIGHT => {
            if ({
                gTasks[taskId].data[5] += 1;
                gTasks[taskId].data[5]
            }) != 65
            {
                gBattle_BG0_X += 4;
                gBattle_BG1_X += 4;
                gBattle_BG2_X += 4;
            } else {
                gTasks[taskId].data[0] = STATE_GET_INPUT;
            }
        }
        STATE_CLOSE_CARD => {
            if gPaletteFade.active() == 0 {
                i = 0;
                while i < 8 {
                    if i < 2 {
                        if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                            FreeAndDestroyTrainerPicSprite((*sInfoCard).spriteIds[i] as u16);
                        }
                    } else {
                        if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                            FreeAndDestroyMonIconSprite(
                                &raw mut gSprites[(*sInfoCard).spriteIds[i]],
                            );
                        }
                    }
                    i += 1;
                }
                i = 8;
                while i < NUM_INFOCARD_SPRITES {
                    if i < 10 {
                        if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                            FreeAndDestroyTrainerPicSprite((*sInfoCard).spriteIds[i] as u16);
                        }
                    } else {
                        if (*sInfoCard).spriteIds[i] != SPRITE_NONE {
                            FreeAndDestroyMonIconSprite(
                                &raw mut gSprites[(*sInfoCard).spriteIds[i]],
                            );
                        }
                    }
                    i += 1;
                }
                FreeMonIconPalettes();
                Free(sInfoCard as *mut c_void);
                sInfoCard = null_mut();
                FreeAllWindowBuffers();
                if mode == INFOCARD_NEXT_OPPONENT as i32 {
                    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                } else {
                    i = CreateTask(Some(Task_ShowTourneyTree), 0) as i32;
                    gTasks[i].data[0] = 0;
                    gTasks[i].data[1] = FALSE as i16;
                    gTasks[i].data[2] = 3;
                    gTasks[i].data[3] = gTasks[taskId].data[4];
                    gTasks[i].data[4] = gTasks[taskId2].data[6];
                }
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_GetInfoCardInput(taskId: u8) -> u8 {
    let mut input: u8 = INFOCARD_INPUT_NONE;
    let mut taskId2: i32 = gTasks[taskId].data[4] as i32;
    let mut position: i32 = gTasks[taskId2].data[1] as i32;
    let mut tourneyId: u8 = sTourneyTreeTrainerIds[position];
    let mut roundId: u16 = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum;
    if gMain.newKeys as i32 & 3 != 0 {
        input = INFOCARD_INPUT_AB;
    }
    if gTasks[taskId].data[3] == INFOCARD_NEXT_OPPONENT {
        return input;
    }
    if gTasks[taskId].data[3] == INFOCARD_TRAINER {
        if gMain.newKeys as i32 & DPAD_UP != 0 && (*sInfoCard).pos == 0 {
            if position == 0 {
                position = 15;
            } else {
                position -= 1;
            }
            input = TRAINERCARD_INPUT_UP as u8;
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 && (*sInfoCard).pos == 0 {
            if position == 15 {
                position = 0;
            } else {
                position += 1;
            }
            input = TRAINERCARD_INPUT_DOWN as u8;
        } else if gMain.newKeys as i32 & DPAD_LEFT != 0 && (*sInfoCard).pos != 0 {
            (*sInfoCard).pos -= 1;
            input = TRAINERCARD_INPUT_LEFT as u8;
        } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
            if (*gSaveBlock2Ptr).frontier.domeTrainers[tourneyId].isEliminated() != 0
                && (*sInfoCard).pos as i32 - 1
                    < (*gSaveBlock2Ptr).frontier.domeTrainers[tourneyId].eliminatedAt() as i32
            {
                (*sInfoCard).pos += 1;
                input = TRAINERCARD_INPUT_RIGHT;
            }
            if (*gSaveBlock2Ptr).frontier.domeTrainers[tourneyId].isEliminated() == 0
                && (*sInfoCard).pos as i32 - 1 < roundId as i32
            {
                (*sInfoCard).pos += 1;
                input = TRAINERCARD_INPUT_RIGHT;
            }
        }
        if input == INFOCARD_INPUT_AB {
            if (*sInfoCard).pos != 0 {
                gTasks[taskId2].data[1] = sTrainerAndRoundToLastMatchCardNum[position / 2]
                    [(*sInfoCard).pos as i32 - 1] as i16;
            } else {
                gTasks[taskId2].data[1] = position as i16;
            }
        }
    } else {
        if gMain.newKeys as i32 & DPAD_UP != 0 && (*sInfoCard).pos == 1 {
            if position == DOME_TOURNAMENT_TRAINERS_COUNT {
                position = sLastMatchCardNum[roundId] as i32;
            } else {
                position -= 1;
            }
            input = MATCHCARD_INPUT_UP as u8;
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 && (*sInfoCard).pos == 1 {
            if position == sLastMatchCardNum[roundId] as i32 {
                position = DOME_TOURNAMENT_TRAINERS_COUNT;
            } else {
                position += 1;
            }
            input = MATCHCARD_INPUT_DOWN as u8;
        } else if gMain.newKeys as i32 & DPAD_LEFT != 0 && (*sInfoCard).pos != 0 {
            input = MATCHCARD_INPUT_LEFT as u8;
            (*sInfoCard).pos -= 1;
        } else if gMain.newKeys as i32 & DPAD_RIGHT != 0
            && ((*sInfoCard).pos == 0 || (*sInfoCard).pos == 1)
        {
            input = MATCHCARD_INPUT_RIGHT as u8;
            (*sInfoCard).pos += 1;
        }
        if input == INFOCARD_INPUT_AB {
            if (*sInfoCard).pos == 0 {
                gTasks[taskId2].data[1] =
                    sTournamentIdToPairedTrainerIds[(*sInfoCard).tournamentIds[0]] as i16;
            } else if (*sInfoCard).pos == 2 {
                gTasks[taskId2].data[1] =
                    sTournamentIdToPairedTrainerIds[(*sInfoCard).tournamentIds[1]] as i16;
            } else {
                gTasks[taskId2].data[1] = position as i16;
            }
        }
    }
    if input != INFOCARD_INPUT_NONE && input != INFOCARD_INPUT_AB {
        PlaySE(SE_SELECT);
        gTasks[taskId2].data[1] = position as i16;
        gTasks[taskId].data[2] ^= 1;
    }
    return input;
}
pub(crate) unsafe extern "C" fn DisplayTrainerInfoOnCard(flags: u8, trainerTourneyId: u8) {
    let mut textPrinter: TextPrinterTemplate = zeroed();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut trainerId: i32 = 0;
    let mut nature: u8 = 0;
    let mut arrId: i32 = 0;
    let mut windowId: i32 = WIN_TRAINER_NAME;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut palSlot: u8 = 0;
    let mut allocatedArray: *mut i16 =
        AllocZeroed(2 * (if 18 >= 16 { 18 } else { 16 }) as u32) as *mut i16;
    trainerId = (*gSaveBlock2Ptr).frontier.domeTrainers[trainerTourneyId].trainerId() as i32;
    if flags as i32 & CARD_ALTERNATE_SLOT != 0 {
        arrId = 8;
        windowId = 9;
        palSlot = 2;
    }
    if flags as i32 & MOVE_CARD_RIGHT as i32 != 0 {
        x = 256;
    }
    if flags as i32 & MOVE_CARD_DOWN as i32 != 0 {
        y = DISPLAY_HEIGHT as i32;
    }
    if flags as i32 & MOVE_CARD_LEFT as i32 != 0 {
        x = -256;
    }
    if flags as i32 & MOVE_CARD_UP as i32 != 0 {
        y = -160;
    }
    if trainerId == TRAINER_PLAYER as i32 {
        (*sInfoCard).spriteIds[arrId] = CreateTrainerPicSprite(
            PlayerGenderToFrontTrainerPicId((*gSaveBlock2Ptr).playerGender),
            TRUE,
            x as i16 + 48,
            y as i16 + 64,
            palSlot + 12,
            TAG_NONE,
        ) as u8;
    } else if trainerId == TRAINER_FRONTIER_BRAIN as i32 {
        (*sInfoCard).spriteIds[arrId] = CreateTrainerPicSprite(
            GetDomeBrainTrainerPicId() as u16,
            TRUE,
            x as i16 + 48,
            y as i16 + 64,
            palSlot + 12,
            TAG_NONE,
        ) as u8;
    } else {
        (*sInfoCard).spriteIds[arrId] = CreateTrainerPicSprite(
            GetFrontierTrainerFrontSpriteId(trainerId as u16) as u16,
            TRUE,
            x as i16 + 48,
            y as i16 + 64,
            palSlot + 12,
            TAG_NONE,
        ) as u8;
    }
    if flags as i32 & MOVE_CARD != 0 {
        gSprites[(*sInfoCard).spriteIds[arrId]].set_invisible(TRUE as u16);
    }
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        if trainerId == TRAINER_PLAYER as i32 {
            (*sInfoCard).spriteIds[2 + i + arrId] = CreateMonIcon(
                (*gSaveBlock2Ptr).frontier.domeMonIds[trainerTourneyId][i],
                Some(SpriteCB_MonIconDomeInfo),
                x as i16 | sInfoTrainerMonX[i] as i16,
                y as i16 + sInfoTrainerMonY[i] as i16,
                0,
                0,
                TRUE as u32,
            );
            gSprites[(*sInfoCard).spriteIds[2 + i + arrId]]
                .oam
                .set_priority(0);
        } else if trainerId == TRAINER_FRONTIER_BRAIN as i32 {
            (*sInfoCard).spriteIds[2 + i + arrId] = CreateMonIcon(
                (*gSaveBlock2Ptr).frontier.domeMonIds[trainerTourneyId][i],
                Some(SpriteCB_MonIconDomeInfo),
                x as i16 | sInfoTrainerMonX[i] as i16,
                y as i16 + sInfoTrainerMonY[i] as i16,
                0,
                0,
                TRUE as u32,
            );
            gSprites[(*sInfoCard).spriteIds[2 + i + arrId]]
                .oam
                .set_priority(0);
        } else {
            (*sInfoCard).spriteIds[2 + i + arrId] = CreateMonIcon(
                (*gFacilityTrainerMons
                    .at((*gSaveBlock2Ptr).frontier.domeMonIds[trainerTourneyId][i]))
                .species,
                Some(SpriteCB_MonIconDomeInfo),
                x as i16 | sInfoTrainerMonX[i] as i16,
                y as i16 + sInfoTrainerMonY[i] as i16,
                0,
                0,
                TRUE as u32,
            );
            gSprites[(*sInfoCard).spriteIds[2 + i + arrId]]
                .oam
                .set_priority(0);
        }
        if flags as i32 & MOVE_CARD != 0 {
            gSprites[(*sInfoCard).spriteIds[2 + i + arrId]].set_invisible(TRUE as u16);
        }
        i += 1;
    }
    textPrinter.fontId = FONT_SHORT;
    textPrinter.x = 0;
    textPrinter.y = 0;
    textPrinter.currentX = textPrinter.x;
    textPrinter.currentY = textPrinter.y;
    textPrinter.letterSpacing = 2;
    textPrinter.lineSpacing = 0;
    textPrinter.set_unk(0);
    textPrinter.set_fgColor(TEXT_DYNAMIC_COLOR_5);
    textPrinter.set_bgColor(TEXT_COLOR_TRANSPARENT);
    textPrinter.set_shadowColor(TEXT_DYNAMIC_COLOR_4);
    i = 0;
    if trainerId == TRAINER_PLAYER as i32 {
        j = gFacilityClassToTrainerClass[60] as i32;
    } else if trainerId == TRAINER_FRONTIER_BRAIN as i32 {
        j = GetDomeBrainTrainerClass() as i32;
    } else {
        j = GetFrontierOpponentClass(trainerId as u16) as i32;
    }
    while gTrainerClassNames[j][i] != EOS {
        gStringVar1[i] = gTrainerClassNames[j][i];
        i += 1;
    }
    gStringVar1[i] = CHAR_SPACE;
    gStringVar1[i + 1] = EOS;
    if trainerId == TRAINER_PLAYER as i32 {
        StringAppend(
            gStringVar1.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
    } else if trainerId == TRAINER_FRONTIER_BRAIN as i32 {
        CopyDomeBrainTrainerName(gStringVar2.as_mut_ptr());
        StringAppend(gStringVar1.as_mut_ptr(), gStringVar2.as_mut_ptr());
    } else {
        CopyDomeTrainerName(gStringVar2.as_mut_ptr(), trainerId as u16);
        StringAppend(gStringVar1.as_mut_ptr(), gStringVar2.as_mut_ptr());
    }
    textPrinter.currentX = GetStringCenterAlignXOffsetWithLetterSpacing(
        textPrinter.fontId as i32,
        gStringVar1.as_mut_ptr(),
        0xD0,
        textPrinter.letterSpacing as i32,
    ) as u8;
    textPrinter.currentChar = gStringVar1.as_mut_ptr();
    textPrinter.windowId = windowId as u8;
    PutWindowTilemap(windowId as u8);
    CopyWindowToVram(windowId as u8, COPYWIN_FULL);
    AddTextPrinter(&raw mut textPrinter, 0, None);
    textPrinter.letterSpacing = 0;
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        textPrinter.currentY = sSpeciesNameTextYCoords[i];
        if trainerId == TRAINER_PLAYER as i32 {
            textPrinter.currentChar = gSpeciesNames
                [(*gSaveBlock2Ptr).frontier.domeMonIds[trainerTourneyId][i]]
                .as_ptr()
                .cast_mut();
        } else if trainerId == TRAINER_FRONTIER_BRAIN as i32 {
            textPrinter.currentChar = gSpeciesNames
                [(*gSaveBlock2Ptr).frontier.domeMonIds[trainerTourneyId][i]]
                .as_ptr()
                .cast_mut();
        } else {
            textPrinter.currentChar = gSpeciesNames[(*gFacilityTrainerMons
                .at((*gSaveBlock2Ptr).frontier.domeMonIds[trainerTourneyId][i]))
            .species]
                .as_ptr()
                .cast_mut();
        }
        textPrinter.windowId = WIN_TRAINER_MON1_NAME + i as u8 + windowId as u8;
        if i == 1 {
            textPrinter.currentX = 7;
        } else {
            textPrinter.currentX = 0;
        }
        PutWindowTilemap(WIN_TRAINER_MON1_NAME + i as u8 + windowId as u8);
        CopyWindowToVram(
            WIN_TRAINER_MON1_NAME + i as u8 + windowId as u8,
            COPYWIN_FULL,
        );
        AddTextPrinter(&raw mut textPrinter, 0, None);
        i += 1;
    }
    PutWindowTilemap(windowId as u8 + WIN_TRAINER_FLAVOR_TEXT);
    CopyWindowToVram(windowId as u8 + WIN_TRAINER_FLAVOR_TEXT, COPYWIN_FULL);
    if trainerId == TRAINER_FRONTIER_BRAIN as i32 {
        textPrinter.currentChar = sBattleDomePotentialTexts[16];
    } else {
        textPrinter.currentChar = sBattleDomePotentialTexts[trainerTourneyId];
    }
    textPrinter.fontId = FONT_NORMAL;
    textPrinter.windowId = windowId as u8 + WIN_TRAINER_FLAVOR_TEXT;
    textPrinter.currentX = 0;
    textPrinter.y = 4;
    textPrinter.currentY = 4;
    AddTextPrinter(&raw mut textPrinter, 0, None);
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        j = 0;
        while j < MAX_MON_MOVES {
            k = 0;
            while k < NUM_MOVE_POINT_TYPES {
                if trainerId == TRAINER_FRONTIER_BRAIN as i32 {
                    *allocatedArray.at(k) +=
                        sBattleStyleMovePoints[GetFrontierBrainMonMove(i as u8, j as u8)][k] as i16;
                } else if trainerId == TRAINER_PLAYER as i32 {
                    *allocatedArray.at(k) += sBattleStyleMovePoints
                        [(*gSaveBlock2Ptr).frontier.domePlayerPartyData[i].moves[j]][k]
                        as i16;
                } else {
                    *allocatedArray.at(k) += sBattleStyleMovePoints[(*gFacilityTrainerMons
                        .at((*gSaveBlock2Ptr).frontier.domeMonIds[trainerTourneyId][i]))
                    .moves[j]][k] as i16;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    i = 0;
    while i < 31 {
        let mut thresholdStatCount: i32 = 0;
        k = 0;
        j = 0;
        while j < NUM_MOVE_POINT_TYPES {
            if sBattleStyleThresholds[i][j] != 0 {
                thresholdStatCount += 1;
                if *allocatedArray.at(j) != 0
                    && *allocatedArray.at(j) >= sBattleStyleThresholds[i][j] as i16
                {
                    k += 1;
                }
            }
            j += 1;
        }
        if thresholdStatCount == k {
            break;
        }
        i += 1;
    }
    textPrinter.currentChar = sBattleDomeOpponentStyleTexts[i];
    textPrinter.y = 20;
    textPrinter.currentY = 20;
    AddTextPrinter(&raw mut textPrinter, 0, None);
    i = 0;
    while i < (if 18 >= 16 { 18 } else { 16 }) {
        *allocatedArray.at(i) = 0;
        i += 1;
    }
    if trainerId == TRAINER_FRONTIER_BRAIN as i32 || trainerId == TRAINER_PLAYER as i32 {
        i = 0;
        while i < FRONTIER_PARTY_SIZE {
            j = 0;
            while j < NUM_STATS {
                if trainerId == TRAINER_FRONTIER_BRAIN as i32 {
                    *allocatedArray.at(j) = GetFrontierBrainMonEvs(i as u8, j as u8) as i16;
                } else {
                    *allocatedArray.at(j) =
                        (*gSaveBlock2Ptr).frontier.domePlayerPartyData[i].evs[j] as i16;
                }
                j += 1;
            }
            *allocatedArray.at(6) += *allocatedArray;
            j = 0;
            while j < NUM_NATURE_STATS {
                if trainerId == TRAINER_FRONTIER_BRAIN as i32 {
                    nature = GetFrontierBrainMonNature(i as u8);
                } else {
                    nature = (*gSaveBlock2Ptr).frontier.domePlayerPartyData[i].nature;
                }
                if gNatureStatTable[nature][j] > 0 {
                    *allocatedArray.at(j + NUM_STATS + 1) +=
                        (*allocatedArray.at(j + 1) as i32 * 110 / 100) as i16;
                } else if gNatureStatTable[nature][j] < 0 {
                    *allocatedArray.at(j + NUM_STATS + 1) +=
                        (*allocatedArray.at(j + 1) as i32 * 90 / 100) as i16;
                    *allocatedArray.at(j + NUM_STATS + NUM_NATURE_STATS + 2) += 1;
                } else {
                    *allocatedArray.at(j + NUM_STATS + 1) += *allocatedArray.at(j + 1);
                }
                j += 1;
            }
            i += 1;
        }
        j = 0;
        i = 0;
        while i < NUM_STATS {
            j += *allocatedArray.at(NUM_STATS + i) as i32;
            i += 1;
        }
        i = 0;
        while i < NUM_STATS {
            *allocatedArray.at(i) =
                div_i32(*allocatedArray.at(NUM_STATS + i) as i32 * 100, j) as i16;
            i += 1;
        }
    } else {
        i = 0;
        while i < FRONTIER_PARTY_SIZE {
            let mut evBits: i32 = (*gFacilityTrainerMons
                .at((*gSaveBlock2Ptr).frontier.domeMonIds[trainerTourneyId][i]))
            .evSpread as i32;
            k = 0;
            j = 0;
            while j < NUM_STATS {
                *allocatedArray.at(j) = 0;
                if evBits & 1 != 0 {
                    k += 1;
                }
                evBits >>= 1;
                j += 1;
            }
            k = div_i32(MAX_TOTAL_EVS, k);
            evBits = (*gFacilityTrainerMons
                .at((*gSaveBlock2Ptr).frontier.domeMonIds[trainerTourneyId][i]))
            .evSpread as i32;
            j = 0;
            while j < NUM_STATS {
                if evBits & 1 != 0 {
                    *allocatedArray.at(j) = k as i16;
                }
                evBits >>= 1;
                j += 1;
            }
            *allocatedArray.at(6) += *allocatedArray;
            j = 0;
            while j < NUM_NATURE_STATS {
                nature = (*gFacilityTrainerMons
                    .at((*gSaveBlock2Ptr).frontier.domeMonIds[trainerTourneyId][i]))
                .nature;
                if gNatureStatTable[nature][j] > 0 {
                    *allocatedArray.at(j + NUM_STATS + 1) +=
                        (*allocatedArray.at(j + 1) as i32 * 110 / 100) as i16;
                } else if gNatureStatTable[nature][j] < 0 {
                    *allocatedArray.at(j + NUM_STATS + 1) +=
                        (*allocatedArray.at(j + 1) as i32 * 90 / 100) as i16;
                    *allocatedArray.at(j + NUM_STATS + NUM_NATURE_STATS + 2) += 1;
                } else {
                    *allocatedArray.at(j + NUM_STATS + 1) += *allocatedArray.at(j + 1);
                }
                j += 1;
            }
            i += 1;
        }
        j = 0;
        i = 0;
        while i < NUM_STATS {
            j += *allocatedArray.at(i + NUM_STATS) as i32;
            i += 1;
        }
        i = 0;
        while i < NUM_STATS {
            *allocatedArray.at(i) =
                div_i32(*allocatedArray.at(NUM_STATS + i) as i32 * 100, j) as i16;
            i += 1;
        }
    }
    i = 0;
    j = 0;
    k = 0;
    while k < NUM_STATS {
        if *allocatedArray.at(k) > 29 {
            if i == 2 {
                if *allocatedArray.at(6) < *allocatedArray.at(k) {
                    if *allocatedArray.at(7) < *allocatedArray.at(k) {
                        if *allocatedArray.at(6) < *allocatedArray.at(7) {
                            *allocatedArray.at(6) = *allocatedArray.at(7);
                            *allocatedArray.at(7) = k as i16;
                        } else {
                            *allocatedArray.at(7) = k as i16;
                        }
                    } else {
                        *allocatedArray.at(6) = *allocatedArray.at(7);
                        *allocatedArray.at(7) = k as i16;
                    }
                } else {
                    if *allocatedArray.at(7) < *allocatedArray.at(k) {
                        *allocatedArray.at(7) = k as i16;
                    }
                }
            } else {
                *allocatedArray.at(i + 6) = k as i16;
                i += 1;
            }
        }
        if *allocatedArray.at(k) == 0 {
            if j == 2 {
                if *allocatedArray.at(k + 12) >= 2
                    || *allocatedArray.at(k + 12) == 1
                        && *allocatedArray.at(12 + *allocatedArray.at(8) as i32) == 0
                        && *allocatedArray.at(12 + *allocatedArray.at(9) as i32) == 0
                {
                    *allocatedArray.at(8) = *allocatedArray.at(9);
                    *allocatedArray.at(9) = k as i16;
                } else if *allocatedArray.at(k + 12) == 1
                    && *allocatedArray.at(12 + *allocatedArray.at(8) as i32) == 0
                {
                    *allocatedArray.at(8) = *allocatedArray.at(9);
                    *allocatedArray.at(9) = k as i16;
                } else if *allocatedArray.at(k + 12) == 1
                    && *allocatedArray.at(12 + *allocatedArray.at(9) as i32) == 0
                {
                    *allocatedArray.at(9) = k as i16;
                }
            } else {
                *allocatedArray.at(j + 8) = k as i16;
                j += 1;
            }
        }
        k += 1;
    }
    if i == 2 {
        i = sStatTextOffsets[*allocatedArray.at(6)] as i32
            + (*allocatedArray.at(7) as i32 - (*allocatedArray.at(6) as i32 + 1))
            + DOME_TEXT_TWO_GOOD_STATS;
    } else if i == 1 {
        i = *allocatedArray.at(6) as i32 + DOME_TEXT_ONE_GOOD_STAT;
    } else if j == 2 {
        i = sStatTextOffsets[*allocatedArray.at(8)] as i32
            + (*allocatedArray.at(9) as i32 - (*allocatedArray.at(8) as i32 + 1))
            + DOME_TEXT_TWO_BAD_STATS;
    } else if j == 1 {
        i = *allocatedArray.at(8) as i32 + DOME_TEXT_ONE_BAD_STAT;
    } else {
        i = DOME_TEXT_WELL_BALANCED;
    }
    textPrinter.currentChar = sBattleDomeOpponentStatsTexts[i];
    textPrinter.y = 36;
    textPrinter.currentY = 36;
    AddTextPrinter(&raw mut textPrinter, 0, None);
    Free(allocatedArray as *mut c_void);
}
pub(crate) unsafe extern "C" fn BufferDomeWinString(
    matchNum: u8,
    mut tournamentIds: *mut u8,
) -> i32 {
    let mut i: i32 = 0;
    let mut tournamentId: u8 = 0;
    let mut winStringId: i32 = 0;
    let mut count: i32 = 0;
    i = sCompetitorRangeByMatch[matchNum][0] as i32;
    while i < sCompetitorRangeByMatch[matchNum][0] as i32
        + sCompetitorRangeByMatch[matchNum][1] as i32
    {
        tournamentId = sTourneyTreeTrainerIds2[i];
        if (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId].isEliminated() == 0 {
            *tournamentIds.at(count) = tournamentId;
            if (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId].trainerId() == TRAINER_PLAYER {
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                );
            } else if (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId].trainerId()
                == TRAINER_FRONTIER_BRAIN
            {
                CopyDomeBrainTrainerName(gStringVar1.as_mut_ptr());
            } else {
                CopyDomeTrainerName(
                    gStringVar1.as_mut_ptr(),
                    (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId].trainerId(),
                );
            }
            count += 1;
        }
        i += 1;
    }
    if count == 2 {
        return DOME_TEXT_NO_WINNER_YET;
    }
    i = sCompetitorRangeByMatch[matchNum][0] as i32;
    while i < sCompetitorRangeByMatch[matchNum][0] as i32
        + sCompetitorRangeByMatch[matchNum][1] as i32
    {
        tournamentId = sTourneyTreeTrainerIds2[i];
        if (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId].isEliminated() != 0
            && (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId].eliminatedAt()
                >= sCompetitorRangeByMatch[matchNum][2] as u16
        {
            *tournamentIds.at(count) = tournamentId;
            count += 1;
            if (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId].eliminatedAt()
                == sCompetitorRangeByMatch[matchNum][2] as u16
            {
                StringCopy(
                    gStringVar2.as_mut_ptr(),
                    gMoveNames[(*gSaveBlock2Ptr).frontier.domeWinningMoves[tournamentId]]
                        .as_ptr()
                        .cast_mut(),
                );
                winStringId =
                    (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId].forfeited() as i32 * 2;
                if (*gSaveBlock2Ptr).frontier.domeWinningMoves[tournamentId] == 0
                    && (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId].forfeited() == 0
                {
                    winStringId = 4;
                }
            } else {
                if (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId].trainerId()
                    == TRAINER_PLAYER
                {
                    StringCopy(
                        gStringVar1.as_mut_ptr(),
                        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                    );
                } else if (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId].trainerId()
                    == TRAINER_FRONTIER_BRAIN
                {
                    CopyDomeBrainTrainerName(gStringVar1.as_mut_ptr());
                } else {
                    CopyDomeTrainerName(
                        gStringVar1.as_mut_ptr(),
                        (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId].trainerId(),
                    );
                }
            }
        }
        if count == 2 {
            break;
        }
        i += 1;
    }
    if matchNum == 14 {
        return winStringId + 2;
    } else {
        return winStringId + 1;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn DisplayMatchInfoOnCard(flags: u8, matchNo: u8) {
    let mut textPrinter: TextPrinterTemplate = zeroed();
    let mut tournamentIds: CArray<i32, 2> = zeroed();
    let mut trainerIds: CArray<i32, 2> = zeroed();
    let mut lost: CArray<u32, 2> = zeroed();
    let mut i: i32 = 0;
    let mut winStringId: i32 = 0;
    let mut arrId: i32 = 0;
    let mut windowId: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut palSlot: u8 = 0;
    if flags as i32 & CARD_ALTERNATE_SLOT != 0 {
        arrId = 8;
        windowId = NUM_INFO_CARD_WINDOWS;
        palSlot = 2;
    }
    if flags as i32 & MOVE_CARD_RIGHT as i32 != 0 {
        x = 256;
    }
    if flags as i32 & MOVE_CARD_DOWN as i32 != 0 {
        y = DISPLAY_HEIGHT as i32;
    }
    if flags as i32 & MOVE_CARD_LEFT as i32 != 0 {
        x = -256;
    }
    if flags as i32 & MOVE_CARD_UP as i32 != 0 {
        y = -160;
    }
    winStringId = BufferDomeWinString(matchNo, (*sInfoCard).tournamentIds.as_mut_ptr());
    i = 0;
    while i < NUM_INFOCARD_TRAINERS {
        tournamentIds[i] = (*sInfoCard).tournamentIds[i] as i32;
        trainerIds[i] =
            (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentIds[i]].trainerId() as i32;
        if (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentIds[i]].eliminatedAt()
            <= sCompetitorRangeByMatch[matchNo][2] as u16
            && (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentIds[i]].isEliminated() != 0
        {
            lost[i] = TRUE as u32;
        } else {
            lost[i] = FALSE as u32;
        }
        i += 1;
    }
    if trainerIds[0] == TRAINER_PLAYER as i32 {
        (*sInfoCard).spriteIds[arrId] = CreateTrainerPicSprite(
            PlayerGenderToFrontTrainerPicId((*gSaveBlock2Ptr).playerGender),
            TRUE,
            x as i16 + 48,
            y as i16 + 88,
            palSlot + 12,
            TAG_NONE,
        ) as u8;
    } else if trainerIds[0] == TRAINER_FRONTIER_BRAIN as i32 {
        (*sInfoCard).spriteIds[arrId] = CreateTrainerPicSprite(
            GetDomeBrainTrainerPicId() as u16,
            TRUE,
            x as i16 + 48,
            y as i16 + 88,
            palSlot + 12,
            TAG_NONE,
        ) as u8;
    } else {
        (*sInfoCard).spriteIds[arrId] = CreateTrainerPicSprite(
            GetFrontierTrainerFrontSpriteId(trainerIds[0] as u16) as u16,
            TRUE,
            x as i16 + 48,
            y as i16 + 88,
            palSlot + 12,
            TAG_NONE,
        ) as u8;
    }
    if flags as i32 & MOVE_CARD != 0 {
        gSprites[(*sInfoCard).spriteIds[arrId]].set_invisible(TRUE as u16);
    }
    if lost[0] != 0 {
        gSprites[(*sInfoCard).spriteIds[arrId]]
            .oam
            .set_paletteNum(3);
    }
    if trainerIds[1] == TRAINER_PLAYER as i32 {
        (*sInfoCard).spriteIds[1 + arrId] = CreateTrainerPicSprite(
            PlayerGenderToFrontTrainerPicId((*gSaveBlock2Ptr).playerGender),
            1,
            x as i16 + 192,
            y as i16 + 88,
            palSlot + 13,
            TAG_NONE,
        ) as u8;
    } else if trainerIds[1] == TRAINER_FRONTIER_BRAIN as i32 {
        (*sInfoCard).spriteIds[1 + arrId] = CreateTrainerPicSprite(
            GetDomeBrainTrainerPicId() as u16,
            1,
            x as i16 + 192,
            y as i16 + 88,
            palSlot + 13,
            TAG_NONE,
        ) as u8;
    } else {
        (*sInfoCard).spriteIds[1 + arrId] = CreateTrainerPicSprite(
            GetFrontierTrainerFrontSpriteId(trainerIds[1] as u16) as u16,
            1,
            x as i16 + 192,
            y as i16 + 88,
            palSlot + 13,
            TAG_NONE,
        ) as u8;
    }
    if flags as i32 & MOVE_CARD != 0 {
        gSprites[(*sInfoCard).spriteIds[1 + arrId]].set_invisible(1);
    }
    if lost[1] != 0 {
        gSprites[(*sInfoCard).spriteIds[1 + arrId]]
            .oam
            .set_paletteNum(3);
    }
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        if trainerIds[0] == TRAINER_PLAYER as i32 {
            (*sInfoCard).spriteIds[2 + i + arrId] = CreateMonIcon(
                (*gSaveBlock2Ptr).frontier.domeMonIds[tournamentIds[0]][i],
                Some(SpriteCB_MonIconDomeInfo),
                x as i16 | sLeftTrainerMonX[i] as i16,
                y as i16 + sLeftTrainerMonY[i] as i16,
                0,
                0,
                TRUE as u32,
            );
            gSprites[(*sInfoCard).spriteIds[2 + i + arrId]]
                .oam
                .set_priority(0);
        } else if trainerIds[0] == TRAINER_FRONTIER_BRAIN as i32 {
            (*sInfoCard).spriteIds[2 + i + arrId] = CreateMonIcon(
                (*gSaveBlock2Ptr).frontier.domeMonIds[tournamentIds[0]][i],
                Some(SpriteCB_MonIconDomeInfo),
                x as i16 | sLeftTrainerMonX[i] as i16,
                y as i16 + sLeftTrainerMonY[i] as i16,
                0,
                0,
                TRUE as u32,
            );
            gSprites[(*sInfoCard).spriteIds[2 + i + arrId]]
                .oam
                .set_priority(0);
        } else {
            (*sInfoCard).spriteIds[2 + i + arrId] = CreateMonIcon(
                (*gFacilityTrainerMons
                    .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentIds[0]][i]))
                .species,
                Some(SpriteCB_MonIconDomeInfo),
                x as i16 | sLeftTrainerMonX[i] as i16,
                y as i16 + sLeftTrainerMonY[i] as i16,
                0,
                0,
                TRUE as u32,
            );
            gSprites[(*sInfoCard).spriteIds[2 + i + arrId]]
                .oam
                .set_priority(0);
        }
        if flags as i32 & MOVE_CARD != 0 {
            gSprites[(*sInfoCard).spriteIds[2 + i + arrId]].set_invisible(TRUE as u16);
        }
        if lost[0] != 0 {
            gSprites[(*sInfoCard).spriteIds[2 + i + arrId]]
                .oam
                .set_paletteNum(3);
            gSprites[(*sInfoCard).spriteIds[2 + i + arrId]].data[3] = TRUE as i16;
        }
        i += 1;
    }
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        if trainerIds[1] == TRAINER_PLAYER as i32 {
            (*sInfoCard).spriteIds[5 + i + arrId] = CreateMonIcon(
                (*gSaveBlock2Ptr).frontier.domeMonIds[tournamentIds[1]][i],
                Some(SpriteCB_MonIconDomeInfo),
                x as i16 | sRightTrainerMonX[i] as i16,
                y as i16 + sRightTrainerMonY[i] as i16,
                0,
                0,
                TRUE as u32,
            );
            gSprites[(*sInfoCard).spriteIds[5 + i + arrId]]
                .oam
                .set_priority(0);
        } else if trainerIds[1] == TRAINER_FRONTIER_BRAIN as i32 {
            (*sInfoCard).spriteIds[5 + i + arrId] = CreateMonIcon(
                (*gSaveBlock2Ptr).frontier.domeMonIds[tournamentIds[1]][i],
                Some(SpriteCB_MonIconDomeInfo),
                x as i16 | sRightTrainerMonX[i] as i16,
                y as i16 + sRightTrainerMonY[i] as i16,
                0,
                0,
                TRUE as u32,
            );
            gSprites[(*sInfoCard).spriteIds[5 + i + arrId]]
                .oam
                .set_priority(0);
        } else {
            (*sInfoCard).spriteIds[5 + i + arrId] = CreateMonIcon(
                (*gFacilityTrainerMons
                    .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentIds[1]][i]))
                .species,
                Some(SpriteCB_MonIconDomeInfo),
                x as i16 | sRightTrainerMonX[i] as i16,
                y as i16 + sRightTrainerMonY[i] as i16,
                0,
                0,
                TRUE as u32,
            );
            gSprites[(*sInfoCard).spriteIds[5 + i + arrId]]
                .oam
                .set_priority(0);
        }
        if flags as i32 & MOVE_CARD != 0 {
            gSprites[(*sInfoCard).spriteIds[5 + i + arrId]].set_invisible(TRUE as u16);
        }
        if lost[1] != 0 {
            gSprites[(*sInfoCard).spriteIds[5 + i + arrId]]
                .oam
                .set_paletteNum(3);
            gSprites[(*sInfoCard).spriteIds[5 + i + arrId]].data[3] = TRUE as i16;
        }
        i += 1;
    }
    textPrinter.x = 0;
    textPrinter.y = 2;
    textPrinter.currentX = textPrinter.x;
    textPrinter.currentY = textPrinter.y;
    textPrinter.letterSpacing = 0;
    textPrinter.lineSpacing = 0;
    textPrinter.set_unk(0);
    textPrinter.set_fgColor(TEXT_DYNAMIC_COLOR_5);
    textPrinter.set_bgColor(TEXT_COLOR_TRANSPARENT);
    textPrinter.set_shadowColor(TEXT_DYNAMIC_COLOR_4);
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), sBattleDomeWinTexts[winStringId]);
    textPrinter.currentChar = gStringVar4.as_mut_ptr();
    textPrinter.windowId = windowId as u8 + WIN_MATCH_WIN_TEXT;
    textPrinter.fontId = FONT_NORMAL;
    PutWindowTilemap(windowId as u8 + WIN_MATCH_WIN_TEXT);
    CopyWindowToVram(windowId as u8 + WIN_MATCH_WIN_TEXT, COPYWIN_FULL);
    textPrinter.currentX = 0;
    textPrinter.currentY = {
        textPrinter.y = 0;
        textPrinter.y
    };
    AddTextPrinter(&raw mut textPrinter, 0, None);
    if trainerIds[0] == TRAINER_PLAYER as i32 {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
    } else if trainerIds[0] == TRAINER_FRONTIER_BRAIN as i32 {
        CopyDomeBrainTrainerName(gStringVar1.as_mut_ptr());
    } else {
        CopyDomeTrainerName(gStringVar1.as_mut_ptr(), trainerIds[0] as u16);
    }
    textPrinter.fontId = FONT_SHORT;
    textPrinter.letterSpacing = 2;
    textPrinter.currentChar = gStringVar1.as_mut_ptr();
    textPrinter.windowId = windowId as u8 + WIN_MATCH_TRAINER_NAME_LEFT;
    textPrinter.currentX = GetStringCenterAlignXOffsetWithLetterSpacing(
        textPrinter.fontId as i32,
        textPrinter.currentChar,
        0x40,
        textPrinter.letterSpacing as i32,
    ) as u8;
    textPrinter.currentY = {
        textPrinter.y = 2;
        textPrinter.y
    };
    PutWindowTilemap(windowId as u8 + WIN_MATCH_TRAINER_NAME_LEFT);
    CopyWindowToVram(windowId as u8 + WIN_MATCH_TRAINER_NAME_LEFT, COPYWIN_FULL);
    AddTextPrinter(&raw mut textPrinter, 0, None);
    if trainerIds[1] == TRAINER_PLAYER as i32 {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
    } else if trainerIds[1] == TRAINER_FRONTIER_BRAIN as i32 {
        CopyDomeBrainTrainerName(gStringVar1.as_mut_ptr());
    } else {
        CopyDomeTrainerName(gStringVar1.as_mut_ptr(), trainerIds[1] as u16);
    }
    textPrinter.currentChar = gStringVar1.as_mut_ptr();
    textPrinter.windowId = windowId as u8 + WIN_MATCH_TRAINER_NAME_RIGHT;
    textPrinter.currentX = GetStringCenterAlignXOffsetWithLetterSpacing(
        textPrinter.fontId as i32,
        textPrinter.currentChar,
        0x40,
        textPrinter.letterSpacing as i32,
    ) as u8;
    textPrinter.currentY = {
        textPrinter.y = 2;
        textPrinter.y
    };
    PutWindowTilemap(windowId as u8 + WIN_MATCH_TRAINER_NAME_RIGHT);
    CopyWindowToVram(windowId as u8 + WIN_MATCH_TRAINER_NAME_RIGHT, COPYWIN_FULL);
    AddTextPrinter(&raw mut textPrinter, 0, None);
    textPrinter.letterSpacing = 0;
    textPrinter.currentChar = sBattleDomeMatchNumberTexts[matchNo];
    textPrinter.windowId = windowId as u8 + WIN_MATCH_NUMBER;
    textPrinter.currentX = GetStringCenterAlignXOffsetWithLetterSpacing(
        textPrinter.fontId as i32,
        textPrinter.currentChar,
        0xA0,
        textPrinter.letterSpacing as i32,
    ) as u8;
    textPrinter.currentY = {
        textPrinter.y = 2;
        textPrinter.y
    };
    PutWindowTilemap(windowId as u8 + WIN_MATCH_NUMBER);
    CopyWindowToVram(windowId as u8 + WIN_MATCH_NUMBER, COPYWIN_FULL);
    AddTextPrinter(&raw mut textPrinter, 0, None);
}
pub(crate) unsafe extern "C" fn ShowDomeTourneyTree() {
    let mut taskId: u8 = CreateTask(Some(Task_ShowTourneyTree), 0);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = FALSE as i16;
    gTasks[taskId].data[2] = 2;
    gTasks[taskId].data[4] = FALSE as i16;
    SetMainCallback2(Some(CB2_TourneyTree));
}
pub(crate) unsafe extern "C" fn ShowPreviousDomeTourneyTree() {
    let mut taskId: u8 = 0;
    SetFacilityTrainerAndMonPtrs();
    (*gSaveBlock2Ptr)
        .frontier
        .set_lvlMode((*gSaveBlock2Ptr).frontier.domeLvlMode - 1);
    (*gSaveBlock2Ptr).frontier.curChallengeBattleNum = DOME_FINAL;
    taskId = CreateTask(Some(Task_ShowTourneyTree), 0);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = FALSE as i16;
    gTasks[taskId].data[2] = 2;
    gTasks[taskId].data[4] = TRUE as i16;
    SetMainCallback2(Some(CB2_TourneyTree));
}
pub(crate) unsafe extern "C" fn Task_HandleTourneyTreeInput(taskId: u8) {
    let mut newTaskId: u8 = 0;
    let mut spriteId: i32 = gTasks[taskId].data[1] as i32;
    match gTasks[taskId].data[0] {
        STATE_FADE_IN => {
            if gPaletteFade.active() == 0 {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
                gTasks[taskId].data[0] = STATE_WAIT_FADE;
                StartSpriteAnim(&raw mut gSprites[spriteId], 1);
            }
        }
        STATE_WAIT_FADE => {
            if gPaletteFade.active() == 0 {
                gTasks[taskId].data[0] = STATE_GET_INPUT;
            }
        }
        STATE_GET_INPUT => match UpdateTourneyTreeCursor(taskId) {
            TOURNEY_TREE_NO_SELECTION => {}
            TOURNEY_TREE_SELECTED_TRAINER => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                gTasks[taskId].data[0] = STATE_SHOW_INFOCARD_TRAINER;
            }
            TOURNEY_TREE_SELECTED_MATCH => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                gTasks[taskId].data[0] = STATE_SHOW_INFOCARD_MATCH;
            }
            _ => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                gTasks[taskId].data[0] = 7;
            }
        },
        STATE_SHOW_INFOCARD_TRAINER => {
            if gPaletteFade.active() == 0 {
                FreeAllWindowBuffers();
                ScanlineEffect_Stop();
                Free(sTilemapBuffer as *mut c_void);
                sTilemapBuffer = null_mut();
                newTaskId = CreateTask(Some(Task_ShowTourneyInfoCard), 0);
                gTasks[newTaskId].data[0] = 0;
                gTasks[newTaskId].data[1] = sTourneyTreeTrainerIds[spriteId] as i16;
                gTasks[newTaskId].data[2] = INFOCARD_TRAINER;
                gTasks[newTaskId].data[3] = taskId as i16;
                gTasks[taskId].data[0] = 4;
                (*sInfoCard).pos = 0;
            }
        }
        4 => {}
        STATE_SHOW_INFOCARD_MATCH => {
            if gPaletteFade.active() == 0 {
                FreeAllWindowBuffers();
                ScanlineEffect_Stop();
                Free(sTilemapBuffer as *mut c_void);
                sTilemapBuffer = null_mut();
                newTaskId = CreateTask(Some(Task_ShowTourneyInfoCard), 0);
                gTasks[newTaskId].data[0] = 0;
                gTasks[newTaskId].data[1] = spriteId as i16 - DOME_TOURNAMENT_TRAINERS_COUNT as i16;
                gTasks[newTaskId].data[2] = INFOCARD_MATCH as i16;
                gTasks[newTaskId].data[3] = taskId as i16;
                gTasks[taskId].data[0] = 6;
            }
        }
        6 => {}
        7 => {
            if gPaletteFade.active() == 0 {
                FreeAllWindowBuffers();
                ScanlineEffect_Stop();
                Free(sTilemapBuffer as *mut c_void);
                sTilemapBuffer = null_mut();
                SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                DestroyTask(gTasks[taskId].data[7] as u8);
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn UpdateTourneyTreeCursor(taskId: u8) -> u8 {
    let mut selection: u8 = TOURNEY_TREE_NO_SELECTION;
    let mut direction: i32 = MOVE_DIR_NONE;
    let mut tourneyTreeCursorSpriteId: i32 = gTasks[taskId].data[1] as i32;
    let mut roundId: i32 = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32;
    if gMain.newKeys == B_BUTTON as u16
        || gMain.newKeys as i32 & A_BUTTON != 0
            && tourneyTreeCursorSpriteId == TOURNEY_TREE_CLOSE_BUTTON
    {
        PlaySE(SE_SELECT);
        selection = TOURNEY_TREE_SELECTED_CLOSE;
    } else if gMain.newKeys as i32 & A_BUTTON != 0 {
        if tourneyTreeCursorSpriteId < DOME_TOURNAMENT_TRAINERS_COUNT {
            PlaySE(SE_SELECT);
            selection = TOURNEY_TREE_SELECTED_TRAINER;
        } else {
            PlaySE(SE_SELECT);
            selection = TOURNEY_TREE_SELECTED_MATCH;
        }
    } else {
        if gMain.newKeys == DPAD_UP as u16
            && sTourneyTreeCursorMovementMap[tourneyTreeCursorSpriteId][roundId][0] != 0xFF
        {
            direction = MOVE_DIR_UP;
        } else if gMain.newKeys == DPAD_DOWN as u16
            && sTourneyTreeCursorMovementMap[tourneyTreeCursorSpriteId][roundId][1] != 0xFF
        {
            direction = MOVE_DIR_DOWN;
        } else if gMain.newKeys == DPAD_LEFT as u16
            && sTourneyTreeCursorMovementMap[tourneyTreeCursorSpriteId][roundId][2] != 0xFF
        {
            direction = MOVE_DIR_LEFT;
        } else if gMain.newKeys == DPAD_RIGHT as u16
            && sTourneyTreeCursorMovementMap[tourneyTreeCursorSpriteId][roundId][3] != 0xFF
        {
            direction = MOVE_DIR_RIGHT;
        }
    }
    if direction != MOVE_DIR_NONE {
        PlaySE(SE_SELECT);
        StartSpriteAnim(&raw mut gSprites[tourneyTreeCursorSpriteId], 0);
        tourneyTreeCursorSpriteId =
            sTourneyTreeCursorMovementMap[tourneyTreeCursorSpriteId][roundId][direction] as i32;
        StartSpriteAnim(&raw mut gSprites[tourneyTreeCursorSpriteId], 1);
        gTasks[taskId].data[1] = tourneyTreeCursorSpriteId as i16;
    }
    return selection;
}
pub(crate) unsafe extern "C" fn ShowNonInteractiveDomeTourneyTree() {
    let mut taskId: u8 = CreateTask(Some(Task_ShowTourneyTree), 0);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = TRUE as i16;
    gTasks[taskId].data[2] = 2;
    gTasks[taskId].data[4] = FALSE as i16;
    SetMainCallback2(Some(CB2_TourneyTree));
}
pub(crate) unsafe extern "C" fn ResolveDomeRoundWinners() {
    let mut i: i32 = 0;
    if gSpecialVar_0x8005 == DOME_PLAYER_WON_MATCH {
        (*gSaveBlock2Ptr).frontier.domeTrainers[TrainerIdToTournamentId(gTrainerBattleOpponent_A)]
            .set_isEliminated(TRUE as u16);
        (*gSaveBlock2Ptr).frontier.domeTrainers[TrainerIdToTournamentId(gTrainerBattleOpponent_A)]
            .set_eliminatedAt((*gSaveBlock2Ptr).frontier.curChallengeBattleNum);
        (*gSaveBlock2Ptr).frontier.domeWinningMoves
            [TrainerIdToTournamentId(gTrainerBattleOpponent_A)] = gBattleResults.lastUsedMovePlayer;
        if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum < DOME_FINAL {
            DecideRoundWinners((*gSaveBlock2Ptr).frontier.curChallengeBattleNum as u8);
        }
    } else {
        (*gSaveBlock2Ptr).frontier.domeTrainers[TrainerIdToTournamentId(TRAINER_PLAYER)]
            .set_isEliminated(TRUE as u16);
        (*gSaveBlock2Ptr).frontier.domeTrainers[TrainerIdToTournamentId(TRAINER_PLAYER)]
            .set_eliminatedAt((*gSaveBlock2Ptr).frontier.curChallengeBattleNum);
        (*gSaveBlock2Ptr).frontier.domeWinningMoves[TrainerIdToTournamentId(TRAINER_PLAYER)] =
            gBattleResults.lastUsedMoveOpponent;
        if gBattleOutcome == 9 || gSpecialVar_0x8005 == 9 {
            (*gSaveBlock2Ptr).frontier.domeTrainers[TrainerIdToTournamentId(TRAINER_PLAYER)]
                .set_forfeited(TRUE as u16);
        }
        i = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32;
        while i < DOME_ROUNDS_COUNT {
            DecideRoundWinners(i as u8);
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn GetWinningMove(
    winnerTournamentId: i32,
    loserTournamentId: i32,
    roundId: u8,
) -> u16 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut moveScores: CArray<i32, 12> = zeroed();
    let mut moves: CArray<u16, 12> = zeroed();
    let mut bestScore: u16 = 0;
    let mut bestId: u16 = 0;
    let mut movePower: i32 = 0;
    SetFacilityPtrsGetLevel();
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        j = 0;
        while j < MAX_MON_MOVES {
            moveScores[i * MAX_MON_MOVES + j] = 0;
            if (*gSaveBlock2Ptr).frontier.domeTrainers[winnerTournamentId].trainerId()
                == TRAINER_FRONTIER_BRAIN
            {
                moves[i * MAX_MON_MOVES + j] = GetFrontierBrainMonMove(i as u8, j as u8);
            } else {
                moves[i * MAX_MON_MOVES + j] = (*gFacilityTrainerMons
                    .at((*gSaveBlock2Ptr).frontier.domeMonIds[winnerTournamentId][i]))
                .moves[j];
            }
            movePower = gBattleMoves[moves[i * MAX_MON_MOVES + j]].power as i32;
            if movePower == 0 {
                movePower = 40;
            } else if movePower == 1 {
                movePower = 60;
            } else if moves[i * MAX_MON_MOVES + j] == MOVE_SELF_DESTRUCT
                || moves[i * MAX_MON_MOVES + j] == MOVE_EXPLOSION
            {
                movePower = movePower / 2;
            }
            k = 0;
            while k < FRONTIER_PARTY_SIZE {
                let mut var: u32 = 0;
                let mut targetSpecies: u16 = SPECIES_NONE;
                let mut targetAbility: u16 = ABILITY_NONE as u16;
                loop {
                    var = Random() as u32 | (Random() as u32) << 16;
                    if (*gFacilityTrainerMons
                        .at((*gSaveBlock2Ptr).frontier.domeMonIds[loserTournamentId][k]))
                    .nature
                        == GetNatureFromPersonality(var)
                    {
                        break;
                    }
                }
                targetSpecies = (*gFacilityTrainerMons
                    .at((*gSaveBlock2Ptr).frontier.domeMonIds[loserTournamentId][k]))
                .species;
                if var & 1 != 0 {
                    targetAbility = gSpeciesInfo[targetSpecies].abilities[1] as u16;
                } else {
                    targetAbility = gSpeciesInfo[targetSpecies].abilities[0] as u16;
                }
                var = AI_TypeCalc(
                    moves[i * MAX_MON_MOVES + j],
                    targetSpecies,
                    targetAbility as u8,
                ) as u32;
                if var & MOVE_RESULT_NOT_VERY_EFFECTIVE as u32 != 0
                    && var & MOVE_RESULT_SUPER_EFFECTIVE as u32 != 0
                {
                    moveScores[i * MAX_MON_MOVES + j] += movePower;
                } else if var & MOVE_RESULT_NO_EFFECT as u32 != 0 {
                    moveScores[i * MAX_MON_MOVES + j] += 0;
                } else if var & MOVE_RESULT_SUPER_EFFECTIVE as u32 != 0 {
                    moveScores[i * MAX_MON_MOVES + j] += movePower * 2;
                } else if var & MOVE_RESULT_NOT_VERY_EFFECTIVE as u32 != 0 {
                    moveScores[i * MAX_MON_MOVES + j] += movePower / 2;
                } else {
                    moveScores[i * MAX_MON_MOVES + j] += movePower;
                }
                k += 1;
            }
            if (bestScore as i32) < moveScores[i * MAX_MON_MOVES + j] {
                bestId = i as u16 * MAX_MON_MOVES as u16 + j as u16;
                bestScore = moveScores[i * MAX_MON_MOVES + j] as u16;
            } else if bestScore as i32 == moveScores[i * MAX_MON_MOVES + j] {
                if moves[bestId] < moves[i * MAX_MON_MOVES + j] {
                    bestId = i as u16 * MAX_MON_MOVES as u16 + j as u16;
                }
            }
            j += 1;
        }
        i += 1;
    }
    j = bestId as i32;
    loop {
        i = 0;
        while i < roundId as i32 - 1 {
            if (*gSaveBlock2Ptr).frontier.domeWinningMoves
                [GetOpposingNPCTournamentIdByRound(winnerTournamentId as u8, i as u8)]
                == moves[j]
            {
                break;
            }
            i += 1;
        }
        if i != roundId as i32 - 1 {
            moveScores[j] = 0;
            bestScore = 0;
            j = 0;
            k = 0;
            while k < 12 {
                j += moveScores[k];
                k += 1;
            }
            if j == 0 {
                break;
            }
            j = 0;
            k = 0;
            while k < 12 {
                if (bestScore as i32) < moveScores[k] {
                    j = k;
                    bestScore = moveScores[k] as u16;
                } else if bestScore as i32 == moveScores[k] && moves[j] < moves[k] {
                    j = k;
                    bestScore = moveScores[k] as u16;
                }
                k += 1;
            }
        }
        if i == roundId as i32 - 1 {
            break;
        }
    }
    if moveScores[j] == 0 {
        j = bestId as i32;
    }
    return moves[j];
}
pub(crate) unsafe extern "C" fn Task_ShowTourneyTree(taskId: u8) {
    let mut i: i32 = 0;
    let mut textPrinter: TextPrinterTemplate = zeroed();
    let mut notInteractive: i32 = gTasks[taskId].data[1] as i32;
    let mut r4: i32 = gTasks[taskId].data[2] as i32;
    match gTasks[taskId].data[0] {
        0 => {
            SetHBlankCallback(None);
            SetVBlankCallback(None);
            EnableInterrupts(3);
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        VRAM as usize as *mut c_void,
                        0x5006000,
                    );
                }
            }
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sTourneyTreeBgTemplates.as_ptr().cast_mut(), 4);
            InitWindows(sTourneyTreeWindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            gBattle_BG0_X = 0;
            gBattle_BG0_Y = 0;
            gBattle_BG1_X = 0;
            gBattle_BG1_Y = 0;
            ChangeBgX(2, 0, BG_COORD_SET);
            ChangeBgY(2, 0, BG_COORD_SET);
            ChangeBgX(3, 0, BG_COORD_SET);
            ChangeBgY(3, 0xB00, BG_COORD_SET);
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(REG_OFFSET_MOSAIC, 0);
            SetGpuReg(REG_OFFSET_WIN0H, 22624);
            SetGpuReg(REG_OFFSET_WIN0V, 159);
            SetGpuReg(REG_OFFSET_WIN1H, 37016);
            SetGpuReg(REG_OFFSET_WIN1V, 159);
            SetGpuReg(REG_OFFSET_WININ, 0);
            SetGpuReg(REG_OFFSET_WINOUT, 63);
            ResetPaletteFade();
            ResetSpriteData();
            FreeAllSpritePalettes();
            gTasks[taskId].data[0] += 1;
        }
        2 => {
            sTilemapBuffer = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
            LZDecompressWram(
                gDomeTourneyTree_Tilemap.as_ptr().cast_mut(),
                sTilemapBuffer as *mut c_void,
            );
            SetBgTilemapBuffer(1, sTilemapBuffer as *mut c_void);
            CopyBgTilemapBufferToVram(1);
            DecompressAndLoadBgGfxUsingHeap(
                1,
                gDomeTourneyTree_Gfx.as_ptr().cast_mut() as *mut c_void,
                0x2000,
                0,
                0,
            );
            DecompressAndLoadBgGfxUsingHeap(
                2,
                gDomeTourneyLine_Gfx.as_ptr().cast_mut() as *mut c_void,
                0x2000,
                0,
                0,
            );
            DecompressAndLoadBgGfxUsingHeap(
                2,
                gDomeTourneyLineDown_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0x2000,
                0,
                1,
            );
            DecompressAndLoadBgGfxUsingHeap(
                3,
                gDomeTourneyLineUp_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0x2000,
                0,
                1,
            );
            LoadCompressedPalette(
                gDomeTourneyTree_Pal.as_ptr().cast_mut(),
                BG_PLTT_OFFSET,
                BG_PLTT_SIZE,
            );
            LoadCompressedPalette(
                gDomeTourneyTreeButtons_Pal.as_ptr().cast_mut(),
                OBJ_PLTT_OFFSET,
                OBJ_PLTT_SIZE,
            );
            LoadCompressedPalette(gBattleWindowTextPalette.as_ptr().cast_mut(), 240, 32);
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        gPlttBufferFaded.as_mut_ptr() as *mut c_void,
                        0x5000100,
                    );
                }
            }
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            gTasks[taskId].data[0] += 1;
        }
        3 => {
            LoadCompressedSpriteSheet(sTourneyTreeButtonsSpriteSheet.as_ptr().cast_mut());
            if notInteractive == FALSE as i32 {
                i = 0;
                while i < 31 {
                    CreateSprite(
                        (&raw const *sTourneyTreePokeballSpriteTemplate).cast_mut(),
                        sTourneyTreePokeballCoords[i][0] as i16,
                        sTourneyTreePokeballCoords[i][1] as i16,
                        0,
                    );
                    i += 1;
                }
                if gTasks[taskId].data[4] != 0 {
                    CreateSprite(
                        (&raw const *sExitButtonSpriteTemplate).cast_mut(),
                        218,
                        12,
                        0,
                    );
                } else {
                    CreateSprite(
                        (&raw const *sCancelButtonSpriteTemplate).cast_mut(),
                        218,
                        12,
                        0,
                    );
                }
            }
            SetGpuReg(REG_OFFSET_DISPCNT, 32576);
            gTasks[taskId].data[0] += 1;
        }
        4 => {
            textPrinter.fontId = FONT_SHORT;
            textPrinter.currentChar = gText_BattleTourney.as_ptr().cast_mut();
            textPrinter.windowId = TOURNEYWIN_TITLE;
            textPrinter.x = 0;
            textPrinter.y = 0;
            textPrinter.letterSpacing = 2;
            textPrinter.lineSpacing = 0;
            textPrinter.currentX = GetStringCenterAlignXOffsetWithLetterSpacing(
                textPrinter.fontId as i32,
                textPrinter.currentChar,
                0x70,
                textPrinter.letterSpacing as i32,
            ) as u8;
            textPrinter.currentY = 1;
            textPrinter.set_unk(0);
            textPrinter.set_fgColor(TEXT_DYNAMIC_COLOR_5);
            textPrinter.set_bgColor(TEXT_COLOR_TRANSPARENT);
            textPrinter.set_shadowColor(TEXT_DYNAMIC_COLOR_4);
            AddTextPrinter(&raw mut textPrinter, 0, None);
            i = 0;
            while i < DOME_TOURNAMENT_TRAINERS_COUNT {
                let mut roundId: i32 = 0;
                let mut var2: i32 = 0;
                CopyDomeTrainerName(
                    gDisplayedStringBattle.as_mut_ptr(),
                    (*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId(),
                );
                if notInteractive == TRUE as i32 {
                    if (*gSaveBlock2Ptr).frontier.domeTrainers[i].isEliminated() != 0 {
                        if (*gSaveBlock2Ptr).frontier.domeTrainers[i].eliminatedAt() != DOME_ROUND1
                        {
                            var2 = (*gSaveBlock2Ptr).frontier.domeTrainers[i].eliminatedAt() as i32
                                - 1;
                            DrawTourneyAdvancementLine(i as u8, var2 as u8);
                        }
                    } else if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum != DOME_ROUND2 {
                        DrawTourneyAdvancementLine(
                            i as u8,
                            (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as u8 - 2,
                        );
                    }
                } else if notInteractive == FALSE as i32 {
                    if (*gSaveBlock2Ptr).frontier.domeTrainers[i].isEliminated() != 0 {
                        if (*gSaveBlock2Ptr).frontier.domeTrainers[i].eliminatedAt() != DOME_ROUND1
                        {
                            var2 = (*gSaveBlock2Ptr).frontier.domeTrainers[i].eliminatedAt() as i32
                                - 1;
                            DrawTourneyAdvancementLine(i as u8, var2 as u8);
                        }
                    } else if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum != DOME_ROUND1 {
                        if gTasks[taskId].data[4] != 0 {
                            var2 = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32;
                        } else {
                            var2 = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 - 1;
                        }
                        DrawTourneyAdvancementLine(i as u8, var2 as u8);
                    }
                }
                if gTasks[taskId].data[4] != 0 {
                    roundId = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32;
                } else {
                    roundId = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 - 1;
                }
                if (notInteractive == 1
                    && ((*gSaveBlock2Ptr).frontier.domeTrainers[i].eliminatedAt() as i32)
                        < (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 - 1
                    || notInteractive == FALSE as i32
                        && (*gSaveBlock2Ptr).frontier.domeTrainers[i].eliminatedAt() as i32
                            <= roundId)
                    && (*gSaveBlock2Ptr).frontier.domeTrainers[i].isEliminated() != 0
                {
                    if (*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId() == TRAINER_PLAYER {
                        textPrinter.set_fgColor(TEXT_COLOR_LIGHT_GRAY);
                        textPrinter.set_shadowColor(TEXT_COLOR_RED);
                    } else {
                        textPrinter.set_fgColor(TEXT_DYNAMIC_COLOR_2);
                        textPrinter.set_shadowColor(TEXT_DYNAMIC_COLOR_4);
                    }
                } else {
                    if (*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId() == TRAINER_PLAYER {
                        textPrinter.set_fgColor(TEXT_COLOR_LIGHT_GRAY);
                        textPrinter.set_shadowColor(TEXT_COLOR_RED);
                    } else {
                        textPrinter.set_fgColor(TEXT_DYNAMIC_COLOR_5);
                        textPrinter.set_shadowColor(TEXT_DYNAMIC_COLOR_4);
                    }
                }
                if sTrainerNamePositions[i][0] == TOURNEYWIN_NAMES_LEFT {
                    textPrinter.currentX = GetStringWidthDifference(
                        textPrinter.fontId as i32,
                        gDisplayedStringBattle.as_mut_ptr(),
                        0x3D,
                        textPrinter.letterSpacing as i32,
                    ) as u8;
                } else {
                    textPrinter.currentX = 3;
                }
                textPrinter.currentChar = gDisplayedStringBattle.as_mut_ptr();
                textPrinter.windowId = sTrainerNamePositions[i][0];
                textPrinter.currentY = sTrainerNamePositions[i][1];
                AddTextPrinter(&raw mut textPrinter, 0, None);
                i += 1;
            }
            gTasks[taskId].data[0] += 1;
        }
        5 => {
            PutWindowTilemap(TOURNEYWIN_NAMES_LEFT);
            PutWindowTilemap(TOURNEYWIN_NAMES_RIGHT);
            PutWindowTilemap(TOURNEYWIN_TITLE);
            CopyWindowToVram(TOURNEYWIN_NAMES_LEFT, COPYWIN_FULL);
            CopyWindowToVram(TOURNEYWIN_NAMES_RIGHT, COPYWIN_FULL);
            CopyWindowToVram(TOURNEYWIN_TITLE, COPYWIN_FULL);
            SetHBlankCallback(Some(HblankCb_TourneyTree));
            SetVBlankCallback(Some(VblankCb_TourneyTree));
            if r4 == 2 {
                if notInteractive == FALSE as i32 {
                    i = CreateTask(Some(Task_HandleTourneyTreeInput), 0) as i32;
                    gTasks[i].data[0] = notInteractive as i16;
                    gTasks[i].data[1] = notInteractive as i16;
                    gTasks[i].data[6] = gTasks[taskId].data[4];
                } else {
                    i = CreateTask(Some(Task_HandleStaticTourneyTreeInput), 0) as i32;
                    gTasks[i].data[0] = 0;
                }
            } else {
                i = gTasks[taskId].data[3] as i32;
                gTasks[i].data[0] = 0;
            }
            ScanlineEffect_Clear();
            i = 0;
            while i < 91 {
                gScanlineEffectRegBuffers[0][i] = 7946;
                gScanlineEffectRegBuffers[1][i] = 7946;
                i += 1;
            }
            while i < 160 {
                gScanlineEffectRegBuffers[0][i] = 7945;
                gScanlineEffectRegBuffers[1][i] = 7945;
                i += 1;
            }
            ScanlineEffect_SetParams(*sTourneyTreeScanlineEffectParams);
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn DrawTourneyAdvancementLine(tournamentId: u8, roundId: u8) {
    let mut i: i32 = 0;
    let mut lineSection: *mut TourneyTreeLineSection =
        sTourneyTreeLineSections[tournamentId][roundId];
    i = 0;
    while i < sTourneyTreeLineSectionArrayCounts[tournamentId][roundId] as i32 {
        CopyToBgTilemapBufferRect_ChangePalette(
            1,
            &raw mut (*lineSection.at(i)).tile as *mut c_void,
            (*lineSection.at(i)).x,
            (*lineSection.at(i)).y,
            1,
            1,
            17,
        );
        i += 1;
    }
    CopyBgTilemapBufferToVram(1);
}
pub(crate) unsafe extern "C" fn Task_HandleStaticTourneyTreeInput(taskId: u8) {
    let mut i: i32 = 0;
    let mut textPrinter: TextPrinterTemplate = zeroed();
    match gTasks[taskId].data[0] {
        STATE_FADE_IN => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
            gTasks[taskId].data[0] = STATE_SHOW_RESULTS;
        }
        STATE_SHOW_RESULTS => {
            if gPaletteFade.active() == 0 {
                gTasks[taskId].data[0] = STATE_DELAY;
                gTasks[taskId].data[3] = 64;
                textPrinter.fontId = FONT_SHORT;
                textPrinter.x = 0;
                textPrinter.y = 0;
                textPrinter.letterSpacing = 2;
                textPrinter.lineSpacing = 0;
                textPrinter.set_unk(0);
                textPrinter.set_fgColor(TEXT_DYNAMIC_COLOR_2);
                textPrinter.set_bgColor(TEXT_COLOR_TRANSPARENT);
                textPrinter.set_shadowColor(TEXT_DYNAMIC_COLOR_4);
                i = 0;
                while i < DOME_TOURNAMENT_TRAINERS_COUNT {
                    CopyDomeTrainerName(
                        gDisplayedStringBattle.as_mut_ptr(),
                        (*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId(),
                    );
                    if (*gSaveBlock2Ptr).frontier.domeTrainers[i].eliminatedAt() as i32
                        == (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 - 1
                        && (*gSaveBlock2Ptr).frontier.domeTrainers[i].isEliminated() != 0
                    {
                        if sTrainerNamePositions[i][0] == TOURNEYWIN_NAMES_LEFT {
                            textPrinter.currentX = GetStringWidthDifference(
                                textPrinter.fontId as i32,
                                gDisplayedStringBattle.as_mut_ptr(),
                                0x3D,
                                textPrinter.letterSpacing as i32,
                            ) as u8;
                        } else {
                            textPrinter.currentX = 3;
                        }
                        textPrinter.currentChar = gDisplayedStringBattle.as_mut_ptr();
                        textPrinter.windowId = sTrainerNamePositions[i][0];
                        textPrinter.currentY = sTrainerNamePositions[i][1];
                        AddTextPrinter(&raw mut textPrinter, 0, None);
                    }
                    if (*gSaveBlock2Ptr).frontier.domeTrainers[i].isEliminated() == 0 {
                        let mut roundId: i32 =
                            (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 - 1;
                        DrawTourneyAdvancementLine(i as u8, roundId as u8);
                    }
                    i += 1;
                }
            }
        }
        STATE_DELAY => {
            if ({
                gTasks[taskId].data[3] -= 1;
                gTasks[taskId].data[3]
            }) == 0
            {
                gTasks[taskId].data[0] = STATE_WAIT_FOR_INPUT;
            }
        }
        STATE_WAIT_FOR_INPUT => {
            if gMain.newKeys as i32 & 3 != 0 {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                gTasks[taskId].data[0] = STATE_CLOSE_TOURNEY_TREE;
            }
        }
        STATE_CLOSE_TOURNEY_TREE => {
            if gPaletteFade.active() == 0 {
                SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CB2_TourneyTree() {
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
    RunTasks();
}
pub(crate) unsafe extern "C" fn VblankCb_TourneyInfoCard() {
    ChangeBgX(3, 0x80, BG_COORD_ADD);
    ChangeBgY(3, 0x80, BG_COORD_SUB);
    SetGpuReg(REG_OFFSET_BG0HOFS, gBattle_BG0_X);
    SetGpuReg(REG_OFFSET_BG0VOFS, gBattle_BG0_Y);
    SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
    SetGpuReg(REG_OFFSET_BG2HOFS, gBattle_BG2_X);
    SetGpuReg(REG_OFFSET_BG2VOFS, gBattle_BG2_Y);
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn HblankCb_TourneyTree() {
    let mut vCount: u16 = (67108870 as usize as *mut u16).read_volatile();
    if vCount < 42 {
        volatile_write(67108936 as usize as *mut u16, 16191);
        volatile_write(67108928 as usize as *mut u32, 0);
    } else if vCount < 50 {
        volatile_write(67108936 as usize as *mut u16, 15163);
        volatile_write(67108928 as usize as *mut u32, 0x989b5558);
    } else if vCount < 58 {
        volatile_write(67108936 as usize as *mut u16, 16191);
        volatile_write(67108928 as usize as *mut u32, 0);
    } else if vCount < 75 {
        volatile_write(67108936 as usize as *mut u16, 15163);
        volatile_write(67108928 as usize as *mut u32, 0x90985860);
    } else if vCount < 82 {
        volatile_write(67108936 as usize as *mut u16, 15163);
        volatile_write(67108928 as usize as *mut u32, 0x989b5558);
    } else if vCount < 95 {
        volatile_write(67108936 as usize as *mut u16, 16191);
        volatile_write(67108928 as usize as *mut u32, 0);
    } else if vCount < 103 {
        volatile_write(67108936 as usize as *mut u16, 14135);
        volatile_write(67108928 as usize as *mut u32, 0x989b5558);
    } else if vCount < 119 {
        volatile_write(67108936 as usize as *mut u16, 14135);
        volatile_write(67108928 as usize as *mut u32, 0x90985860);
    } else if vCount < 127 {
        volatile_write(67108936 as usize as *mut u16, 16191);
        volatile_write(67108928 as usize as *mut u32, 0);
    } else if vCount < 135 {
        volatile_write(67108936 as usize as *mut u16, 14135);
        volatile_write(67108928 as usize as *mut u32, 0x989b5558);
    } else {
        volatile_write(67108936 as usize as *mut u16, 16191);
        volatile_write(67108928 as usize as *mut u32, 0);
    }
}
pub(crate) unsafe extern "C" fn VblankCb_TourneyTree() {
    SetGpuReg(REG_OFFSET_BG0HOFS, gBattle_BG0_X);
    SetGpuReg(REG_OFFSET_BG0VOFS, gBattle_BG0_Y);
    SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
    ChangeBgY(2, 0x80, BG_COORD_SUB);
    ChangeBgY(3, 0x80, BG_COORD_ADD);
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe extern "C" fn SetFacilityTrainerAndMonPtrs() {
    gFacilityTrainerMons = gBattleFrontierMons.as_ptr().cast_mut();
    gFacilityTrainers = gBattleFrontierTrainers.as_ptr().cast_mut();
}
pub(crate) unsafe extern "C" fn ResetSketchedMoves() {
    let mut i: i32 = 0;
    let mut moveSlot: i32 = 0;
    i = 0;
    while i < DOME_BATTLE_PARTY_SIZE {
        let mut playerMonId: i32 = (*gSaveBlock2Ptr).frontier.selectedPartyMons
            [gSelectedOrderFromParty[i] as i32 - 1] as i32
            - 1;
        let mut count: i32 = 0;
        moveSlot = 0;
        while moveSlot < MAX_MON_MOVES {
            count = 0;
            while count < MAX_MON_MOVES {
                if GetMonData3(
                    &raw mut (*gSaveBlock1Ptr).playerParty[playerMonId],
                    MON_DATA_MOVE1 + count,
                    null_mut(),
                ) == GetMonData3(
                    &raw mut gPlayerParty[i],
                    MON_DATA_MOVE1 + moveSlot,
                    null_mut(),
                ) {
                    break;
                }
                count += 1;
            }
            if count == MAX_MON_MOVES {
                SetMonMoveSlot(&raw mut gPlayerParty[i], MOVE_SKETCH, moveSlot as u8);
            }
            moveSlot += 1;
        }
        (*gSaveBlock1Ptr).playerParty[playerMonId] = gPlayerParty[i];
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn RestoreDomePlayerPartyHeldItems() {
    let mut i: i32 = 0;
    i = 0;
    while i < DOME_BATTLE_PARTY_SIZE {
        let mut playerMonId: i32 = (*gSaveBlock2Ptr).frontier.selectedPartyMons
            [gSelectedOrderFromParty[i] as i32 - 1] as i32
            - 1;
        let mut item: u16 = GetMonData3(
            &raw mut (*gSaveBlock1Ptr).playerParty[playerMonId],
            MON_DATA_HELD_ITEM,
            null_mut(),
        ) as u16;
        SetMonData(
            &raw mut gPlayerParty[i],
            MON_DATA_HELD_ITEM,
            &raw mut item as *mut c_void,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ReduceDomePlayerPartyToSelectedMons() {
    ReducePlayerPartyToSelectedMons();
}
pub(crate) unsafe extern "C" fn GetPlayerSeededBeforeOpponent() {
    if TrainerIdToTournamentId(gTrainerBattleOpponent_A) > TrainerIdToTournamentId(TRAINER_PLAYER) {
        gSpecialVar_Result = 1;
    } else {
        gSpecialVar_Result = 2;
    }
}
pub(crate) unsafe extern "C" fn BufferLastDomeWinnerName() {
    let mut i: i32 = 0;
    SetFacilityTrainerAndMonPtrs();
    i = 0;
    while i < DOME_TOURNAMENT_TRAINERS_COUNT {
        if (*gSaveBlock2Ptr).frontier.domeTrainers[i].isEliminated() == 0 {
            break;
        }
        i += 1;
    }
    CopyDomeTrainerName(
        gStringVar1.as_mut_ptr(),
        (*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId(),
    );
}
pub(crate) unsafe extern "C" fn InitRandomTourneyTreeResults() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut monLevel: i32 = 0;
    let mut species: CArray<i32, 3> = zeroed();
    let mut monTypesBits: i32 = 0;
    let mut trainerId: i32 = 0;
    let mut monId: i32 = 0;
    let mut zero1: i32 = 0;
    let mut zero2: i32 = 0;
    let mut lvlMode: u8 = 0;
    let mut statSums: *mut u16 = null_mut();
    let mut statValues: *mut i32 = null_mut();
    let mut ivs: u8 = 0;
    species[0] = 0;
    species[1] = 0;
    species[2] = 0;
    if (*gSaveBlock2Ptr).frontier.domeLvlMode as i32
        != -((*gSaveBlock2Ptr).frontier.domeBattleMode as i32)
        && (*gSaveBlock2Ptr).frontier.challengeStatus != CHALLENGE_STATUS_SAVING
    {
        return;
    }
    statSums = AllocZeroed(32) as *mut u16;
    statValues = AllocZeroed(24) as *mut i32;
    lvlMode = (*gSaveBlock2Ptr).frontier.lvlMode();
    (*gSaveBlock2Ptr).frontier.set_lvlMode(FRONTIER_LVL_50);
    zero1 = 0;
    zero2 = 0;
    (*gSaveBlock2Ptr).frontier.domeLvlMode = zero1 as u8 + 1;
    (*gSaveBlock2Ptr).frontier.domeBattleMode = zero2 as u8 + 1;
    i = 0;
    while i < DOME_TOURNAMENT_TRAINERS_COUNT {
        loop {
            if i < 5 {
                trainerId = Random() as i32 % 10;
            } else if i < 15 {
                trainerId = Random() as i32 % 20 + 10;
            } else {
                trainerId = Random() as i32 % 10 + 30;
            }
            j = 0;
            while j < i {
                if (*gSaveBlock2Ptr).frontier.domeTrainers[j].trainerId() as i32 == trainerId {
                    break;
                }
                j += 1;
            }
            if j == i {
                break;
            }
        }
        (*gSaveBlock2Ptr).frontier.domeTrainers[i].set_trainerId(trainerId as u16);
        j = 0;
        while j < FRONTIER_PARTY_SIZE {
            loop {
                monId = GetRandomFrontierMonFromSet(trainerId as u16) as i32;
                k = 0;
                while k < j {
                    let mut alreadySelectedMonId: i32 =
                        (*gSaveBlock2Ptr).frontier.domeMonIds[i][k] as i32;
                    if alreadySelectedMonId == monId
                        || species[0] == (*gFacilityTrainerMons.at(monId)).species as i32
                        || species[1] == (*gFacilityTrainerMons.at(monId)).species as i32
                        || (*gFacilityTrainerMons.at(alreadySelectedMonId)).itemTableId
                            == (*gFacilityTrainerMons.at(monId)).itemTableId
                    {
                        break;
                    }
                    k += 1;
                }
                if k == j {
                    break;
                }
            }
            (*gSaveBlock2Ptr).frontier.domeMonIds[i][j] = monId as u16;
            species[j] = (*gFacilityTrainerMons.at(monId)).species as i32;
            j += 1;
        }
        (*gSaveBlock2Ptr).frontier.domeTrainers[i].set_isEliminated(FALSE as u16);
        (*gSaveBlock2Ptr).frontier.domeTrainers[i].set_eliminatedAt(0);
        (*gSaveBlock2Ptr).frontier.domeTrainers[i].set_forfeited(FALSE as u16);
        i += 1;
    }
    monLevel = FRONTIER_MAX_LEVEL_50 as i32;
    i = 0;
    while i < DOME_TOURNAMENT_TRAINERS_COUNT {
        monTypesBits = 0;
        *statSums.at(i) = 0;
        ivs = GetDomeTrainerMonIvs((*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId());
        j = 0;
        while j < FRONTIER_PARTY_SIZE {
            CalcDomeMonStats(
                (*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.domeMonIds[i][j])).species,
                monLevel,
                ivs as i32,
                (*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.domeMonIds[i][j])).evSpread,
                (*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.domeMonIds[i][j])).nature,
                statValues,
            );
            *statSums.at(i) += *statValues.at(1) as u16;
            *statSums.at(i) += *statValues.at(2) as u16;
            *statSums.at(i) += *statValues.at(4) as u16;
            *statSums.at(i) += *statValues.at(5) as u16;
            *statSums.at(i) += *statValues.at(3) as u16;
            *statSums.at(i) += *statValues as u16;
            monTypesBits |= gBitTable[gSpeciesInfo
                [(*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.domeMonIds[i][j])).species]
                .types[0]] as i32;
            monTypesBits |= gBitTable[gSpeciesInfo
                [(*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.domeMonIds[i][j])).species]
                .types[1]] as i32;
            j += 1;
        }
        trainerId = 0;
        j = 0;
        while j < 32 {
            if monTypesBits & 1 != 0 {
                trainerId += 1;
            }
            monTypesBits >>= 1;
            j += 1;
        }
        *statSums.at(i) += (trainerId * monLevel / 20) as u16;
        i += 1;
    }
    i = 0;
    while i < 15 {
        j = i + 1;
        while j < DOME_TOURNAMENT_TRAINERS_COUNT {
            if *statSums.at(i) < *statSums.at(j) {
                SwapDomeTrainers(i, j, statSums);
            } else if *statSums.at(i) == *statSums.at(j) {
                if (*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId()
                    > (*gSaveBlock2Ptr).frontier.domeTrainers[j].trainerId()
                {
                    SwapDomeTrainers(i, j, statSums);
                }
            }
            j += 1;
        }
        i += 1;
    }
    Free(statSums as *mut c_void);
    Free(statValues as *mut c_void);
    i = 0;
    while i < DOME_ROUNDS_COUNT {
        DecideRoundWinners(i as u8);
        i += 1;
    }
    (*gSaveBlock2Ptr).frontier.set_lvlMode(lvlMode);
}
pub(crate) unsafe extern "C" fn TrainerIdToTournamentId(trainerId: u16) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while i < DOME_TOURNAMENT_TRAINERS_COUNT {
        if (*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId() == trainerId {
            break;
        }
        i += 1;
    }
    return i;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrainerIdToDomeTournamentId(trainerId: u16) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while i < DOME_TOURNAMENT_TRAINERS_COUNT {
        if (*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId() == trainerId {
            break;
        }
        i += 1;
    }
    return i;
}
pub(crate) unsafe extern "C" fn GetOpposingNPCTournamentIdByRound(
    tournamentId: u8,
    round: u8,
) -> u8 {
    let mut tournamentIds: CArray<u8, 2> = zeroed();
    BufferDomeWinString(
        sTrainerAndRoundToLastMatchCardNum
            [sTournamentIdToPairedTrainerIds[tournamentId] as i32 / 2][round]
            - 16,
        tournamentIds.as_mut_ptr(),
    );
    if tournamentId == tournamentIds[0] {
        return tournamentIds[1];
    } else {
        return tournamentIds[0];
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn DecideRoundWinners(roundId: u8) {
    let mut i: i32 = 0;
    let mut moveSlot: i32 = 0;
    let mut monId1: i32 = 0;
    let mut monId2: i32 = 0;
    let mut tournamentId1: i32 = 0;
    let mut tournamentId2: i32 = 0;
    let mut species: i32 = 0;
    let mut points1: i32 = 0;
    let mut points2: i32 = 0;
    i = 0;
    while i < DOME_TOURNAMENT_TRAINERS_COUNT {
        'l1: {
            if (*gSaveBlock2Ptr).frontier.domeTrainers[i].isEliminated() != 0
                || (*gSaveBlock2Ptr).frontier.domeTrainers[i].trainerId() == TRAINER_PLAYER
            {
                break 'l1;
            }
            tournamentId1 = i;
            tournamentId2 = TournamentIdOfOpponent(
                roundId as i32,
                (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId1].trainerId() as i32,
            );
            if (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId1].trainerId()
                == TRAINER_FRONTIER_BRAIN
                && tournamentId2 != 0xFF
            {
                (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId2]
                    .set_isEliminated(TRUE as u16);
                (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId2]
                    .set_eliminatedAt(roundId as u16);
                (*gSaveBlock2Ptr).frontier.domeWinningMoves[tournamentId2] =
                    GetWinningMove(tournamentId1, tournamentId2, roundId);
            } else if (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId2].trainerId()
                == TRAINER_FRONTIER_BRAIN
                && tournamentId1 != 0xFF
            {
                (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId1]
                    .set_isEliminated(TRUE as u16);
                (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId1]
                    .set_eliminatedAt(roundId as u16);
                (*gSaveBlock2Ptr).frontier.domeWinningMoves[tournamentId1] =
                    GetWinningMove(tournamentId2, tournamentId1, roundId);
            } else if tournamentId2 != 0xFF {
                monId1 = 0;
                while monId1 < FRONTIER_PARTY_SIZE {
                    moveSlot = 0;
                    while moveSlot < MAX_MON_MOVES {
                        monId2 = 0;
                        while monId2 < FRONTIER_PARTY_SIZE {
                            points1 += GetTypeEffectivenessPoints(
                                (*gFacilityTrainerMons
                                    .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentId1]
                                        [monId1]))
                                .moves[moveSlot] as i32,
                                (*gFacilityTrainerMons
                                    .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentId2]
                                        [monId2]))
                                .species as i32,
                                EFFECTIVENESS_MODE_AI_VS_AI,
                            );
                            monId2 += 1;
                        }
                        moveSlot += 1;
                    }
                    species = (*gFacilityTrainerMons
                        .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentId1][monId1]))
                    .species as i32;
                    points1 += (gSpeciesInfo[species].baseHP as i32
                        + gSpeciesInfo[species].baseAttack as i32
                        + gSpeciesInfo[species].baseDefense as i32
                        + gSpeciesInfo[species].baseSpeed as i32
                        + gSpeciesInfo[species].baseSpAttack as i32
                        + gSpeciesInfo[species].baseSpDefense as i32)
                        / 10;
                    monId1 += 1;
                }
                points1 += Random() as i32 & 0x1F;
                points1 += tournamentId1;
                monId1 = 0;
                while monId1 < FRONTIER_PARTY_SIZE {
                    moveSlot = 0;
                    while moveSlot < MAX_MON_MOVES {
                        monId2 = 0;
                        while monId2 < FRONTIER_PARTY_SIZE {
                            points2 += GetTypeEffectivenessPoints(
                                (*gFacilityTrainerMons
                                    .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentId2]
                                        [monId1]))
                                .moves[moveSlot] as i32,
                                (*gFacilityTrainerMons
                                    .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentId1]
                                        [monId2]))
                                .species as i32,
                                EFFECTIVENESS_MODE_AI_VS_AI,
                            );
                            monId2 += 1;
                        }
                        moveSlot += 1;
                    }
                    species = (*gFacilityTrainerMons
                        .at((*gSaveBlock2Ptr).frontier.domeMonIds[tournamentId2][monId1]))
                    .species as i32;
                    points2 += (gSpeciesInfo[species].baseHP as i32
                        + gSpeciesInfo[species].baseAttack as i32
                        + gSpeciesInfo[species].baseDefense as i32
                        + gSpeciesInfo[species].baseSpeed as i32
                        + gSpeciesInfo[species].baseSpAttack as i32
                        + gSpeciesInfo[species].baseSpDefense as i32)
                        / 10;
                    monId1 += 1;
                }
                points2 += Random() as i32 & 0x1F;
                points2 += tournamentId2;
                if points1 > points2 {
                    (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId2]
                        .set_isEliminated(TRUE as u16);
                    (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId2]
                        .set_eliminatedAt(roundId as u16);
                    (*gSaveBlock2Ptr).frontier.domeWinningMoves[tournamentId2] =
                        GetWinningMove(tournamentId1, tournamentId2, roundId);
                } else if points1 < points2 {
                    (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId1]
                        .set_isEliminated(TRUE as u16);
                    (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId1]
                        .set_eliminatedAt(roundId as u16);
                    (*gSaveBlock2Ptr).frontier.domeWinningMoves[tournamentId1] =
                        GetWinningMove(tournamentId2, tournamentId1, roundId);
                } else if tournamentId1 > tournamentId2 {
                    (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId2]
                        .set_isEliminated(TRUE as u16);
                    (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId2]
                        .set_eliminatedAt(roundId as u16);
                    (*gSaveBlock2Ptr).frontier.domeWinningMoves[tournamentId2] =
                        GetWinningMove(tournamentId1, tournamentId2, roundId);
                } else {
                    (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId1]
                        .set_isEliminated(TRUE as u16);
                    (*gSaveBlock2Ptr).frontier.domeTrainers[tournamentId1]
                        .set_eliminatedAt(roundId as u16);
                    (*gSaveBlock2Ptr).frontier.domeWinningMoves[tournamentId1] =
                        GetWinningMove(tournamentId2, tournamentId1, roundId);
                }
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CopyDomeTrainerName(mut str: *mut u8, trainerId: u16) {
    let mut i: i32 = 0;
    SetFacilityPtrsGetLevel();
    if trainerId == TRAINER_FRONTIER_BRAIN {
        CopyDomeBrainTrainerName(str);
    } else {
        if trainerId == TRAINER_PLAYER {
            i = 0;
            while i < PLAYER_NAME_LENGTH {
                *str.at(i) = (*gSaveBlock2Ptr).playerName[i];
                i += 1;
            }
        } else if trainerId < FRONTIER_TRAINERS_COUNT {
            i = 0;
            while i < PLAYER_NAME_LENGTH {
                *str.at(i) = (*gFacilityTrainers.at(trainerId)).trainerName[i];
                i += 1;
            }
        }
        *str.at(i) = EOS;
    }
}
pub(crate) unsafe extern "C" fn GetDomeBrainTrainerPicId() -> u8 {
    return gTrainers[806].trainerPic;
}
pub(crate) unsafe extern "C" fn GetDomeBrainTrainerClass() -> u8 {
    return gTrainers[806].trainerClass;
}
pub(crate) unsafe extern "C" fn CopyDomeBrainTrainerName(mut str: *mut u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < PLAYER_NAME_LENGTH {
        *str.at(i) = gTrainers[806].trainerName[i];
        i += 1;
    }
    *str.at(i) = EOS;
}
