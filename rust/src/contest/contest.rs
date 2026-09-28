//! Translated from `src/contest.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sSliderHeartYPositions sNextTurnSpriteYPositions sSpriteSheet_SliderHeart sOam_SliderHeart sAffineAnim_SliderHeart_Normal sAffineAnim_SliderHeart_SpinDisappear sAffineAnim_SliderHeart_SpinAppear sAffineAnims_SliderHeart sSpriteTemplate_SliderHeart sSpriteSheet_NextTurn sSpritePalette_NextTurn sOam_NextTurn sSpriteTemplates_NextTurn sSubsprites_NextTurn sSubspriteTable_NextTurn sSpriteSheet_Faces sOam_Faces sSpriteTemplate_Faces sSpriteSheet_ApplauseMeter sSpritePalette_ApplauseMeter sOam_ApplauseMeter sSpriteTemplate_ApplauseMeter sOam_Judge sSpriteTemplate_Judge sSpriteSheet_Judge sSpriteSheet_JudgeSymbols sSpritePalette_JudgeSymbols sSpriteTemplate_JudgeSpeechBubble sText_Pal gContestEffectDescriptionPointers sUnusedComboMoveNameTexts gContestMoveTypeTextPointers sUnusedAppealResultTexts sRoundResultTexts sAppealResultTexts sContestConditions sInvalidContestMoveNames sContestBgTemplates sContestWindowTemplates gDefaultContestWinners gContestOpponents gPostgameContestOpponentFilter sSpriteSheets_ContestantsTurnBlinkEffect sSpritePalettes_ContestantsTurnBlinkEffect sOam_ContestantsTurnBlinkEffect sAffineAnim_ContestantsTurnBlinkEffect_0 sAffineAnim_ContestantsTurnBlinkEffect_1 sAffineAnims_ContestantsTurnBlinkEffect sSpriteTemplates_ContestantsTurnBlinkEffect sContestExcitementTable

const APPEALSTATE_CHECK_REPEATED_MOVE: i16 = 17;
const APPEALSTATE_CHECK_SKIP_TURN: i16 = 2;
const APPEALSTATE_CHECK_TURN_ORDER_MOD: i16 = 48;
const APPEALSTATE_DO_CROWD_EXCITED: i16 = 54;
const APPEALSTATE_DO_CROWD_UNEXCITED: i16 = 53;
const APPEALSTATE_FREE_MON_SPRITE: i16 = 11;
const APPEALSTATE_MOVE_ANIM: i16 = 7;
const APPEALSTATE_MOVE_ANIM_MULTITURN: i16 = 9;
const APPEALSTATE_PRINT_COMBO_MSG: i16 = 14;
const APPEALSTATE_PRINT_CROWD_WATCHES_MSG: i16 = 57;
const APPEALSTATE_PRINT_MON_MOVE_IGNORED_MSG: i16 = 58;
const APPEALSTATE_PRINT_SKIP_TURN_MSG: i16 = 31;
const APPEALSTATE_PRINT_TOO_NERVOUS_MSG: i16 = 33;
const APPEALSTATE_PRINT_USED_MOVE_MSG: i16 = 5;
const APPEALSTATE_SLIDE_APPLAUSE_OUT: i16 = 55;
const APPEALSTATE_SLIDE_MON_IN: i16 = 3;
const APPEALSTATE_SLIDE_MON_OUT: i16 = 10;
const APPEALSTATE_START_NEXT_TURN: i16 = 22;
const APPEALSTATE_START_TURN: i16 = 0;
const APPEALSTATE_START_TURN_END_DELAY: i16 = 20;
const APPEALSTATE_TRY_JUDGE_STAR: i16 = 35;
const APPEALSTATE_TRY_PRINT_MOVE_RESULT: i16 = 23;
const APPEALSTATE_TRY_PRINT_SKIP_NEXT_TURN_MSG: i16 = 51;
const APPEALSTATE_TRY_SHOW_NEXT_TURN_GFX: i16 = 47;
const APPEALSTATE_TRY_UPDATE_HEARTS_FROM_COMBO: i16 = 15;
const APPEALSTATE_TURN_END_DELAY: i16 = 21;
const APPEALSTATE_UPDATE_CROWD: i16 = 41;
const APPEALSTATE_UPDATE_HEARTS_FROM_REPEAT: i16 = 19;
const APPEALSTATE_UPDATE_MOVE_USERS_HEARTS: i16 = 12;
const APPEALSTATE_UPDATE_MOVE_USERS_STARS: i16 = 37;
const APPEALSTATE_UPDATE_MOVE_USERS_STATUS: i16 = 50;
const APPEALSTATE_UPDATE_OPPONENT: i16 = 26;
const APPEALSTATE_UPDATE_OPPONENTS: i16 = 25;
const APPEALSTATE_UPDATE_OPPONENT_HEARTS: i16 = 28;
const APPEALSTATE_UPDATE_OPPONENT_STARS: i16 = 39;
const APPEALSTATE_UPDATE_OPPONENT_STATUS: i16 = 30;
const APPEALSTATE_WAIT_EXCITEMENT_HEARTS: i16 = 43;
const APPEALSTATE_WAIT_HEARTS_FROM_COMBO: i16 = 16;
const APPEALSTATE_WAIT_HEARTS_FROM_REPEAT: i16 = 18;
const APPEALSTATE_WAIT_JUDGE_COMBO: i16 = 45;
const APPEALSTATE_WAIT_JUDGE_REPEATED_MOVE: i16 = 46;
const APPEALSTATE_WAIT_JUDGE_STAR: i16 = 36;
const APPEALSTATE_WAIT_JUDGE_TURN_ORDER: i16 = 49;
const APPEALSTATE_WAIT_LINK: i16 = 1;
const APPEALSTATE_WAIT_MON_MOVE_IGNORED_MSG: i16 = 59;
const APPEALSTATE_WAIT_MOVE_ANIM: i16 = 8;
const APPEALSTATE_WAIT_MOVE_RESULT_MSG: i16 = 24;
const APPEALSTATE_WAIT_MOVE_USERS_HEARTS: i16 = 13;
const APPEALSTATE_WAIT_MOVE_USERS_STARS: i16 = 38;
const APPEALSTATE_WAIT_OPPONENT_HEARTS: i16 = 29;
const APPEALSTATE_WAIT_OPPONENT_RESPONSE_MSG: i16 = 27;
const APPEALSTATE_WAIT_OPPONENT_STARS: i16 = 40;
const APPEALSTATE_WAIT_SKIP_NEXT_TURN_MSG: i16 = 52;
const APPEALSTATE_WAIT_SKIP_TURN_MSG: i16 = 32;
const APPEALSTATE_WAIT_SLIDE_APPLAUSE: i16 = 56;
const APPEALSTATE_WAIT_SLIDE_MON: i16 = 4;
const APPEALSTATE_WAIT_TOO_NERVOUS_MSG: i16 = 34;
const APPEALSTATE_WAIT_USED_MOVE_MSG: i16 = 6;
const APPEAL_MOVES_END: u16 = 65535;
const CONTESTANT_TEXT_COLOR_START: u8 = 10;
const CONTEST_DEBUG_MODE_OFF: u8 = 0;
const CONTEST_DEBUG_MODE_PRINT_LOSER_FLAGS: u8 = 3;
const CONTEST_DEBUG_MODE_PRINT_POINT_TOTAL: u8 = 1;
const CONTEST_DEBUG_MODE_PRINT_WINNER_FLAGS: u8 = 2;
const JUDGE_SYMBOL_NUMBER_FOUR: u8 = 6;
const JUDGE_SYMBOL_NUMBER_ONE: u8 = 5;
const JUDGE_SYMBOL_NUMBER_ONE_UNUSED: u8 = 4;
const JUDGE_SYMBOL_ONE_EXCLAMATION: u8 = 2;
const JUDGE_SYMBOL_QUESTION_MARK: u8 = 7;
const JUDGE_SYMBOL_STAR: u8 = 8;
const JUDGE_SYMBOL_SWIRL: u8 = 0;
const JUDGE_SYMBOL_SWIRL_UNUSED: u8 = 1;
const JUDGE_SYMBOL_TWO_EXCLAMATIONS: u8 = 3;
const SLIDER_HEART_ANIM_APPEAR: u8 = 2;
const SLIDER_HEART_ANIM_DISAPPEAR: u8 = 1;
const STAT_SYMBOL_CIRCLE: u8 = 0;
const STAT_SYMBOL_SQUARE: u8 = 4;
const STAT_SYMBOL_SWIRL: u8 = 3;
const STAT_SYMBOL_WAVE: u8 = 1;
const STAT_SYMBOL_X: u8 = 2;
const TAG_APPLAUSE_METER: u16 = 44002;
const TILE_EMPTY_APPEAL_HEART: u16 = 20533;
const TILE_EMPTY_JAM_HEART: u16 = 20534;
const TILE_FILLED_APPEAL_HEART: u16 = 20498;
const TILE_FILLED_JAM_HEART: u16 = 20500;
const WIN_GENERAL_TEXT: u8 = 4;
const WIN_MOVE0: u8 = 5;
const WIN_MOVE_DESCRIPTION: u8 = 10;
const WIN_SLASH: u32 = 9;

static gContestEffectDescriptionPointers: Table<CArray<*mut u8, 48>> =
    Table((&raw const crate::data::contest::gContestEffectDescriptionPointers).cast());
static gContestOpponents: Table<CArray<ContestPokemon, 96>> =
    Table((&raw const crate::data::contest::gContestOpponents).cast());
static gDefaultContestWinners: Table<CArray<ContestWinner, 8>> =
    Table((&raw const crate::data::contest::gDefaultContestWinners).cast());
static gPostgameContestOpponentFilter: Table<CArray<u8, 96>> =
    Table((&raw const crate::data::contest::gPostgameContestOpponentFilter).cast());
static sAppealResultTexts: Table<CArray<*mut u8, 62>> =
    Table((&raw const crate::data::contest::sAppealResultTexts).cast());
static sContestBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::contest::sContestBgTemplates).cast());
static sContestConditions: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::contest::sContestConditions).cast());
static sContestExcitementTable: Table<CArray<CArray<i8, 5>, 5>> =
    Table((&raw const crate::data::contest::sContestExcitementTable).cast());
static sContestWindowTemplates: Table<CArray<WindowTemplate, 12>> =
    Table((&raw const crate::data::contest::sContestWindowTemplates).cast());
static sInvalidContestMoveNames: Table<CArray<*mut u8, 6>> =
    Table((&raw const crate::data::contest::sInvalidContestMoveNames).cast());
static sNextTurnSpriteYPositions: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::contest::sNextTurnSpriteYPositions).cast());
static sRoundResultTexts: Table<CArray<*mut u8, 32>> =
    Table((&raw const crate::data::contest::sRoundResultTexts).cast());
static sSliderHeartYPositions: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::contest::sSliderHeartYPositions).cast());
static sSpritePalette_ApplauseMeter: Table<SpritePalette> =
    Table((&raw const crate::data::contest::sSpritePalette_ApplauseMeter).cast());
static sSpritePalette_JudgeSymbols: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::contest::sSpritePalette_JudgeSymbols).cast());
static sSpritePalette_NextTurn: Table<SpritePalette> =
    Table((&raw const crate::data::contest::sSpritePalette_NextTurn).cast());
static sSpritePalettes_ContestantsTurnBlinkEffect: Table<CArray<SpritePalette, 4>> =
    Table((&raw const crate::data::contest::sSpritePalettes_ContestantsTurnBlinkEffect).cast());
static sSpriteSheet_ApplauseMeter: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::contest::sSpriteSheet_ApplauseMeter).cast());
static sSpriteSheet_Judge: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::contest::sSpriteSheet_Judge).cast());
static sSpriteSheet_JudgeSymbols: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::contest::sSpriteSheet_JudgeSymbols).cast());
static sSpriteSheet_NextTurn: Table<CArray<CompressedSpriteSheet, 4>> =
    Table((&raw const crate::data::contest::sSpriteSheet_NextTurn).cast());
static sSpriteSheet_SliderHeart: Table<SpriteSheet> =
    Table((&raw const crate::data::contest::sSpriteSheet_SliderHeart).cast());
static sSpriteSheets_ContestantsTurnBlinkEffect: Table<CArray<CompressedSpriteSheet, 4>> =
    Table((&raw const crate::data::contest::sSpriteSheets_ContestantsTurnBlinkEffect).cast());
static sSpriteTemplate_ApplauseMeter: Table<SpriteTemplate> =
    Table((&raw const crate::data::contest::sSpriteTemplate_ApplauseMeter).cast());
static sSpriteTemplate_Judge: Table<SpriteTemplate> =
    Table((&raw const crate::data::contest::sSpriteTemplate_Judge).cast());
static sSpriteTemplate_JudgeSpeechBubble: Table<SpriteTemplate> =
    Table((&raw const crate::data::contest::sSpriteTemplate_JudgeSpeechBubble).cast());
static sSpriteTemplate_SliderHeart: Table<SpriteTemplate> =
    Table((&raw const crate::data::contest::sSpriteTemplate_SliderHeart).cast());
static sSpriteTemplates_ContestantsTurnBlinkEffect: Table<CArray<SpriteTemplate, 4>> =
    Table((&raw const crate::data::contest::sSpriteTemplates_ContestantsTurnBlinkEffect).cast());
static sSpriteTemplates_NextTurn: Table<CArray<SpriteTemplate, 4>> =
    Table((&raw const crate::data::contest::sSpriteTemplates_NextTurn).cast());
static sSubspriteTable_NextTurn: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::contest::sSubspriteTable_NextTurn).cast());
static sText_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::contest::sText_Pal).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestMons: CArray<ContestPokemon, 4> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestMonRound1Points: Aligned<CArray<i16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestMonTotalPoints: Aligned<CArray<i16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestMonAppealPointTotals: Aligned<CArray<i16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestMonRound2Points: Aligned<CArray<i16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestFinalStandings: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestMonPartyIndex: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestPlayerMonIndex: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestantTurnOrder: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkContestFlags: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestLinkLeaderIndex: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSpecialVar_ContestCategory: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSpecialVar_ContestRank: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gNumLinkContestPlayers: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gHighestRibbonRank: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gContestResources: *mut ContestResources = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sContestBgCopyFlags: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurContestWinner: ContestWinner = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurContestWinnerIsForArtist: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurContestWinnerSaveIdx: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gContestRngValue: u32 = 0;

unsafe extern "C" {
    static gAffineAnims_BattleSpriteContest: CArray<*mut AffineAnimCmd, 0>;
    static gAffineAnims_BattleSpriteOpponentSide: CArray<*mut AffineAnimCmd, 0>;
    static mut gAnimFriendship: u8;
    static mut gAnimMoveTurn: u8;
    static mut gAnimScriptActive: u8;
    static mut gAnimScriptCallback: Option<unsafe extern "C" fn()>;
    static mut gBattleAnimBgTileBuffer: *mut u8;
    static mut gBattleAnimBgTilemapBuffer: *mut u8;
    static mut gBattleMonForms: CArray<u8, 4>;
    static gBattleMoves: CArray<BattleMove, 0>;
    static mut gBattleTypeFlags: u32;
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
    static mut gBattlerAttacker: u8;
    static mut gBattlerPositions: CArray<u8, 4>;
    static mut gBattlerSpriteIds: CArray<u8, 4>;
    static mut gBattlerTarget: u8;
    static gContest2Pal: CArray<u32, 0>;
    static gContestApplauseMeterGfx: CArray<u8, 0>;
    static gContestAudienceGfx: CArray<u32, 0>;
    static gContestAudienceTilemap: CArray<u32, 0>;
    static gContestCurtainTilemap: CArray<u32, 0>;
    static gContestEffectFuncs: CArray<Option<unsafe extern "C" fn()>, 0>;
    static gContestEffects: CArray<ContestEffect, 0>;
    static gContestInterfaceAudiencePalette: CArray<u32, 0>;
    static gContestInterfaceGfx: CArray<u32, 0>;
    static gContestInterfaceTilemap: CArray<u32, 0>;
    static gContestMoves: CArray<ContestMove, 0>;
    static gContestNextTurnNumbersGfx: CArray<u8, 0>;
    static gContestNextTurnRandomGfx: CArray<u8, 0>;
    static mut gDisplayedStringBattle: CArray<u8, 300>;
    static mut gEnableContestDebugging: u8;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gHeap: CArray<u8, 114688>;
    static mut gMPlayInfo_SE1: MusicPlayerInfo;
    static mut gMain: Main;
    static gMonBackPicTable: CArray<CompressedSpriteSheet, 0>;
    static mut gMonSpritesGfxPtr: *mut MonSpritesGfx;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static mut gMultiuseSpriteTemplate: SpriteTemplate;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gReservedSpritePaletteCount: u8;
    static mut gRngValue: u32;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static mut gTextFlags: TextFlags;
    static gText_AllOutOfAppealTime: CArray<u8, 0>;
    static gText_AppealComboWentOverExcellently: CArray<u8, 0>;
    static gText_AppealComboWentOverVeryWell: CArray<u8, 0>;
    static gText_AppealComboWentOverWell: CArray<u8, 0>;
    static gText_AppealNumButItCantParticipate: CArray<u8, 0>;
    static gText_AppealNumWhichMoveWillBePlayed: CArray<u8, 0>;
    static gText_BDot: CArray<u8, 0>;
    static gText_CDot: CArray<u8, 0>;
    static gText_ColorBlue: CArray<u8, 0>;
    static gText_ColorLightShadowDarkGray: CArray<u8, 0>;
    static gText_ColorTransparent: CArray<u8, 0>;
    static gText_Contest_Anxiety: CArray<u8, 0>;
    static gText_Contest_Fear: CArray<u8, 0>;
    static gText_Contest_Hesitancy: CArray<u8, 0>;
    static gText_Contest_Laziness: CArray<u8, 0>;
    static gText_Contest_Shyness: CArray<u8, 0>;
    static gText_CrowdContinuesToWatchMon: CArray<u8, 0>;
    static gText_JudgeLookedAtMonExpectantly: CArray<u8, 0>;
    static gText_LinkStandby4: CArray<u8, 0>;
    static gText_MonAppealedWithMove: CArray<u8, 0>;
    static gText_MonCantAppealNextTurn: CArray<u8, 0>;
    static gText_MonWasTooNervousToMove: CArray<u8, 0>;
    static gText_MonWasWatchingOthers: CArray<u8, 0>;
    static gText_MonsMoveIsIgnored: CArray<u8, 0>;
    static gText_MonsXDidntGoOverWell: CArray<u8, 0>;
    static gText_MonsXGotTheCrowdGoing: CArray<u8, 0>;
    static gText_MonsXWentOverGreat: CArray<u8, 0>;
    static gText_OneDash: CArray<u8, 0>;
    static gText_RepeatedAppeal: CArray<u8, 0>;
    static gText_Slash: CArray<u8, 0>;
    fn AddTextPrinter(
        a0: *mut TextPrinterTemplate,
        a1: u8,
        a2: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AllocOamMatrix() -> u8;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AllocateMonSpritesGfx();
    fn AnimateSprite(a0: *mut Sprite);
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
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut Sprite)>) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut Sprite);
    fn DestroySpriteAndFreeResources(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DoMoveAnim(a0: u16);
    fn FillPalette(a0: u16, a1: u16, a2: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeMonSpritesGfx();
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteFinal_Y(a0: u8, a1: u16, a2: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetContestRand() -> u16;
    fn GetGpuReg(a0: u8) -> u16;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetMultiplayerId() -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
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
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitSpriteAffineAnim(a0: *mut Sprite);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsLinkTaskFinished() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LZDecompressVram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RequestDma3Copy(a0: *mut c_void, a1: *mut c_void, a2: u16, a3: u8) -> i16;
    fn RequestDma3Fill(a0: i32, a1: *mut c_void, a2: u16, a3: u8) -> i16;
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
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetSubspriteTables(a0: *mut Sprite, a1: *mut SubspriteTable);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
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
    fn m4aMPlayImmInit(a0: *mut MusicPlayerInfo);
    fn m4aMPlayPitchControl(a0: *mut MusicPlayerInfo, a1: u16, a2: i16);
}

pub(crate) unsafe extern "C" fn TaskDummy1(taskId: u8) {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetLinkContestBoolean() {
    gLinkContestFlags = 0;
}
pub(crate) unsafe extern "C" fn SetupContestGpuRegs() {
    SetGpuReg(REG_OFFSET_DISPCNT, DISPCNT_OBJ_1D_MAP);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, 16191);
    SetGpuRegBits(REG_OFFSET_DISPCNT, 32512);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadContestBgAfterMoveAnim() {
    let mut i: i32 = 0;
    LZDecompressVram(
        gContestInterfaceGfx.as_ptr().cast_mut(),
        VRAM as usize as *mut c_void,
    );
    LZDecompressVram(
        gContestAudienceGfx.as_ptr().cast_mut(),
        0x6002000 as usize as *mut c_void,
    );
    CopyToBgTilemapBuffer(
        3,
        gContestAudienceTilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyBgTilemapBufferToVram(3);
    LoadCompressedPalette(
        gContestInterfaceAudiencePalette.as_ptr().cast_mut(),
        BG_PLTT_OFFSET,
        BG_PLTT_SIZE,
    );
    LoadContestPalettes();
    i = 0;
    while i < CONTESTANT_COUNT {
        let mut contestantWindowId: u32 = 5 + i as u32;
        LoadPalette(
            (*(gHeap.as_mut_ptr().at(106500) as *mut ContestTempSave)).cachedWindowPalettes
                [contestantWindowId]
                .as_mut_ptr() as *mut c_void,
            0x000 + (5 + gContestantTurnOrder[i] as u16) * 16,
            32,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn InitContestInfoBgs() {
    let mut i: i32 = 0;
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sContestBgTemplates.as_ptr().cast_mut(), 4);
    SetBgAttribute(3, BG_ATTR_WRAPAROUND, 1);
    i = 0;
    while i < CONTESTANT_COUNT {
        SetBgTilemapBuffer(
            i as u8,
            (*gContestResources).contestBgTilemaps[i] as *mut c_void,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn InitContestWindows() {
    InitWindows(sContestWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        gTextFlags.set_canABSpeedUpPrint(FALSE);
    } else {
        gTextFlags.set_canABSpeedUpPrint(TRUE);
    }
}
pub(crate) unsafe extern "C" fn LoadContestPalettes() {
    let mut i: i32 = 0;
    LoadPalette(sText_Pal.as_ptr().cast_mut() as *mut c_void, 240, 32);
    SetBackdropFromColor(0);
    i = 10;
    while i < 14 {
        LoadPalette(
            &raw mut gPlttBufferUnfaded[241] as *mut c_void,
            240 + i as u16,
            2,
        );
        i += 1;
    }
    FillPalette(32319, 243, 2);
}
pub(crate) unsafe extern "C" fn InitContestResources() {
    let mut i: i32 = 0;
    *(*gContestResources).contest = {
        let mut lit1: Contest = zeroed();
        lit1.playerMoveChoice = 0;
        lit1
    };
    i = 0;
    while i < CONTESTANT_COUNT {
        (*(*gContestResources).contest).unk[i] = 0xFF;
        i += 1;
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        *(*gContestResources).status.at(i) = {
            let mut lit2: ContestantStatus = zeroed();
            lit2.baseAppeal = 0;
            lit2
        };
        i += 1;
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        (*(*gContestResources).status.at(i)).set_ranking(0);
        (*(*gContestResources).status.at(i)).effectStringId = CONTEST_STRING_NONE;
        (*(*gContestResources).status.at(i)).effectStringId2 = CONTEST_STRING_NONE;
        i += 1;
    }
    *(*gContestResources).appealResults = {
        let mut lit3: ContestAppealMoveResults = zeroed();
        lit3
    };
    *(*gContestResources).aiData = {
        let mut lit4: ContestAIInfo = zeroed();
        lit4.aiState = 0;
        lit4
    };
    *(*gContestResources).excitement = {
        let mut lit5: ContestExcitement = zeroed();
        lit5.moveExcitement = 0;
        lit5
    };
    memset((*gContestResources).gfxState as *mut u8, 0, 16);
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK == 0 {
        SortContestants(FALSE);
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        (*(*gContestResources).status.at(i)).nextTurnOrder = CONTESTANT_NONE;
        (*(*gContestResources).contest).prevTurnOrder[i] = gContestantTurnOrder[i];
        i += 1;
    }
    ApplyNextTurnOrder();
    memset((*gContestResources).tv as *mut u8, 0, 64);
}
pub(crate) unsafe extern "C" fn AllocContestResources() {
    gContestResources = AllocZeroed(64) as *mut ContestResources;
    (*gContestResources).contest = AllocZeroed(92) as *mut Contest;
    (*gContestResources).status = AllocZeroed(112) as *mut ContestantStatus;
    (*gContestResources).appealResults = AllocZeroed(20) as *mut ContestAppealMoveResults;
    (*gContestResources).aiData = AllocZeroed(68) as *mut ContestAIInfo;
    (*gContestResources).excitement = AllocZeroed(16) as *mut ContestExcitement;
    (*gContestResources).gfxState = AllocZeroed(16) as *mut ContestGraphicsState;
    (*gContestResources).moveAnim = AllocZeroed(20) as *mut ContestMoveAnimData;
    (*gContestResources).tv = AllocZeroed(64) as *mut ContestTV;
    (*gContestResources).unused = AllocZeroed(12) as *mut ContestUnused;
    (*gContestResources).contestBgTilemaps[0] = AllocZeroed(0x1000) as *mut u8;
    (*gContestResources).contestBgTilemaps[1] = AllocZeroed(0x1000) as *mut u8;
    (*gContestResources).contestBgTilemaps[2] = AllocZeroed(0x1000) as *mut u8;
    (*gContestResources).contestBgTilemaps[3] = AllocZeroed(0x1000) as *mut u8;
    (*gContestResources).boxBlinkTiles1 = AllocZeroed(0x800);
    (*gContestResources).boxBlinkTiles2 = AllocZeroed(0x800);
    (*gContestResources).animBgTileBuffer = AllocZeroed(0x2000);
    gBattleAnimBgTileBuffer = (*gContestResources).animBgTileBuffer as *mut u8;
    gBattleAnimBgTilemapBuffer = (*gContestResources).contestBgTilemaps[1];
}
pub(crate) unsafe extern "C" fn FreeContestResources() {
    Free((*gContestResources).contest as *mut c_void);
    (*gContestResources).contest = null_mut();
    Free((*gContestResources).status as *mut c_void);
    (*gContestResources).status = null_mut();
    Free((*gContestResources).appealResults as *mut c_void);
    (*gContestResources).appealResults = null_mut();
    Free((*gContestResources).aiData as *mut c_void);
    (*gContestResources).aiData = null_mut();
    Free((*gContestResources).excitement as *mut c_void);
    (*gContestResources).excitement = null_mut();
    Free((*gContestResources).gfxState as *mut c_void);
    (*gContestResources).gfxState = null_mut();
    Free((*gContestResources).moveAnim as *mut c_void);
    (*gContestResources).moveAnim = null_mut();
    Free((*gContestResources).tv as *mut c_void);
    (*gContestResources).tv = null_mut();
    Free((*gContestResources).unused as *mut c_void);
    (*gContestResources).unused = null_mut();
    Free((*gContestResources).contestBgTilemaps[0] as *mut c_void);
    (*gContestResources).contestBgTilemaps[0] = null_mut();
    Free((*gContestResources).contestBgTilemaps[1] as *mut c_void);
    (*gContestResources).contestBgTilemaps[1] = null_mut();
    Free((*gContestResources).contestBgTilemaps[2] as *mut c_void);
    (*gContestResources).contestBgTilemaps[2] = null_mut();
    Free((*gContestResources).contestBgTilemaps[3] as *mut c_void);
    (*gContestResources).contestBgTilemaps[3] = null_mut();
    Free((*gContestResources).boxBlinkTiles1);
    (*gContestResources).boxBlinkTiles1 = null_mut();
    Free((*gContestResources).boxBlinkTiles2);
    (*gContestResources).boxBlinkTiles2 = null_mut();
    Free((*gContestResources).animBgTileBuffer);
    (*gContestResources).animBgTileBuffer = null_mut();
    Free(gContestResources as *mut c_void);
    gContestResources = null_mut();
    gBattleAnimBgTileBuffer = null_mut();
    gBattleAnimBgTilemapBuffer = null_mut();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_StartContest() {
    match gMain.state {
        0 => {
            sContestBgCopyFlags = 0;
            AllocContestResources();
            AllocateMonSpritesGfx();
            Free((*gMonSpritesGfxPtr).firstDecompressed);
            (*gMonSpritesGfxPtr).firstDecompressed = null_mut();
            (*gMonSpritesGfxPtr).firstDecompressed = AllocZeroed(0x4000);
            SetVBlankCallback(None);
            InitContestInfoBgs();
            InitContestWindows();
            SetupContestGpuRegs();
            ScanlineEffect_Clear();
            ResetPaletteFade();
            gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
            ResetSpriteData();
            ResetTasks();
            FreeAllSpritePalettes();
            gReservedSpritePaletteCount = 4;
            gHeap[0x1a000] = CONTEST_DEBUG_MODE_OFF;
            ClearBattleMonForms();
            InitContestResources();
            gMain.state += 1;
        }
        1 => {
            gMain.state += 1;
        }
        2 => {
            if SetupContestGraphics(&raw mut (*(*gContestResources).contest).contestSetupState) != 0
            {
                (*(*gContestResources).contest).contestSetupState = 0;
                gMain.state += 1;
            }
        }
        3 => {
            SetBgForCurtainDrop();
            gBattle_BG1_X = 0;
            gBattle_BG1_Y = 0;
            BeginFastPaletteFade(2);
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            SetVBlankCallback(Some(VBlankCB_Contest));
            (*(*gContestResources).contest).mainTaskId =
                CreateTask(Some(Task_StartContestWaitFade), 10);
            SetMainCallback2(Some(CB2_ContestMain));
            if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(8, 8);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_StartContestWaitFade(taskId: u8) {
    if gPaletteFade.active() == 0 {
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].func = Some(Task_TryStartLinkContest);
    }
}
pub(crate) unsafe extern "C" fn Task_TryStartLinkContest(taskId: u8) {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS != 0 {
            'l1: {
                let sw1: i16 = gTasks[taskId].data[0];
                let mut fall = false;
                if sw1 == 0 {
                    fall = true;
                    ContestPrintLinkStandby();
                    gTasks[taskId].data[0] += 1;
                }
                if fall || sw1 == 1 {
                    fall = true;
                    if IsLinkTaskFinished() != 0 {
                        SetLinkStandbyCallback();
                        gTasks[taskId].data[0] += 1;
                    }
                    return;
                }
                if sw1 == 2 {
                    fall = true;
                    if IsLinkTaskFinished() != TRUE {
                        return;
                    }
                    gTasks[taskId].data[0] += 1;
                    break 'l1;
                }
            }
        }
        if gPaletteFade.active() == 0 {
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_WIRELESS == 0 {
                ContestPrintLinkStandby();
            }
            CreateTask(Some(Task_CommunicateMonIdxs), 0);
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].func = Some(TaskDummy1);
        }
    } else {
        gTasks[taskId].func = Some(Task_WaitToRaiseCurtainAtStart);
    }
}
pub(crate) unsafe extern "C" fn Task_CommunicateMonIdxs(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateMonIdxs),
        Some(Task_EndCommunicateMonIdxs),
    );
}
pub(crate) unsafe extern "C" fn Task_EndCommunicateMonIdxs(taskId: u8) {
    gTasks[taskId].data[0] = 1;
    gTasks[taskId].func = Some(Task_ReadyStartLinkContest);
}
pub(crate) unsafe extern "C" fn Task_ReadyStartLinkContest(taskId: u8) {
    gTasks[taskId].data[0] -= 1;
    if gTasks[taskId].data[0] <= 0 {
        GetMultiplayerId();
        DestroyTask(taskId);
        gTasks[(*(*gContestResources).contest).mainTaskId].func =
            Some(Task_WaitToRaiseCurtainAtStart);
        gRngValue = gContestRngValue;
    }
}
pub(crate) unsafe extern "C" fn SetupContestGraphics(stateVar: *mut u8) -> u8 {
    let mut tempPalette1: CArray<u16, 16> = zeroed();
    let mut tempPalette2: CArray<u16, 16> = zeroed();
    match *stateVar {
        0 => {
            gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
            RequestDma3Fill(0, VRAM as usize as *mut c_void, 0x8000, 1);
            RequestDma3Fill(
                0,
                (VRAM as usize as *mut c_void as *mut u8).at(32768) as *mut c_void,
                0x8000,
                1,
            );
            RequestDma3Fill(
                0,
                (VRAM as usize as *mut c_void as *mut u8).at(0x10000) as *mut c_void,
                0x8000,
                1,
            );
        }
        1 => {
            LZDecompressVram(
                gContestInterfaceGfx.as_ptr().cast_mut(),
                VRAM as usize as *mut c_void,
            );
        }
        2 => {
            LZDecompressVram(
                gContestAudienceGfx.as_ptr().cast_mut(),
                0x6002000 as usize as *mut c_void,
            );
            {
                let mut _src: *mut c_void = 0x6002000 as usize as *mut c_void;
                let mut _dest: *mut c_void = gHeap.as_mut_ptr().at(0x18000) as *mut c_void;
                let mut _size: u32 = 0x2000;
                loop {
                    if _size <= 0x1000 {
                        {
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x84000000 | _size / 4);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x84000400);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                    _src = (_src as *mut u8).at(4096) as *mut c_void;
                    _dest = (_dest as *mut u8).at(4096) as *mut c_void;
                    _size -= 0x1000;
                }
            }
        }
        3 => {
            CopyToBgTilemapBuffer(
                3,
                gContestAudienceTilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyBgTilemapBufferToVram(3);
        }
        4 => {
            CopyToBgTilemapBuffer(
                2,
                gContestInterfaceTilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyBgTilemapBufferToVram(2);
            {
                let mut _src: *mut c_void =
                    (*gContestResources).contestBgTilemaps[2] as *mut c_void;
                let mut _dest: *mut c_void = (*(gHeap.as_mut_ptr().at(106500)
                    as *mut ContestTempSave))
                    .savedJunk
                    .as_mut_ptr() as *mut c_void;
                let mut _size: u32 = 2048;
                {
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, _src as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x84000000 | _size / 4);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
        5 => {
            LoadCompressedPalette(
                gContestInterfaceAudiencePalette.as_ptr().cast_mut(),
                BG_PLTT_OFFSET,
                BG_PLTT_SIZE,
            );
            CpuSet(
                &raw mut gPlttBufferUnfaded[128] as *mut c_void,
                tempPalette1.as_mut_ptr() as *mut c_void,
                0x4000008,
            );
            CpuSet(
                &raw mut gPlttBufferUnfaded[0x000 + (5 + gContestPlayerMonIndex as i32) * 16]
                    as *mut c_void,
                tempPalette2.as_mut_ptr() as *mut c_void,
                0x4000008,
            );
            CpuSet(
                tempPalette2.as_mut_ptr() as *mut c_void,
                &raw mut gPlttBufferUnfaded[128] as *mut c_void,
                0x4000008,
            );
            CpuSet(
                tempPalette1.as_mut_ptr() as *mut c_void,
                &raw mut gPlttBufferUnfaded[0x000 + (5 + gContestPlayerMonIndex as i32) * 16]
                    as *mut c_void,
                0x4000008,
            );
            {
                let mut _src: *mut c_void = gPlttBufferUnfaded.as_mut_ptr() as *mut c_void;
                let mut _dest: *mut c_void = (*(gHeap.as_mut_ptr().at(106500)
                    as *mut ContestTempSave))
                    .cachedWindowPalettes
                    .as_mut_ptr() as *mut c_void;
                let mut _size: u32 = 512;
                {
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, _src as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x84000000 | _size / 4);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
            LoadContestPalettes();
        }
        6 => {
            DrawContestantWindows();
            FillContestantWindowBgs();
            SwapMoveDescAndContestTilemaps();
            (*(*gContestResources).contest).judgeSpeechBubbleSpriteId =
                CreateJudgeSpeechBubbleSprite();
            CreateSliderHeartSprites();
            CreateNextTurnSprites();
            CreateApplauseMeterSprite();
            CreateJudgeAttentionEyeTask();
            CreateUnusedBlendTask();
            gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
            gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
            gBattlerPositions[2] = B_POSITION_OPPONENT_RIGHT;
            gBattlerPositions[3] = B_POSITION_PLAYER_RIGHT;
            gBattleTypeFlags = 0;
            gBattlerAttacker = B_BATTLER_2;
            gBattlerTarget = B_BATTLER_3;
            gBattlerSpriteIds[gBattlerAttacker] = CreateJudgeSprite();
            CreateInvisibleBattleTargetSprite();
            CopyBgTilemapBufferToVram(3);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(1);
            ShowBg(3);
            ShowBg(2);
            ShowBg(0);
            ShowBg(1);
        }
        _ => {
            *stateVar = 0;
            return TRUE;
        }
    }
    *stateVar += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_WaitToRaiseCurtainAtStart(taskId: u8) {
    gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
    if gPaletteFade.active() == 0 {
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].data[1] = 0;
        gTasks[taskId].func = Some(Task_RaiseCurtainAtStart);
    }
}
pub(crate) unsafe extern "C" fn Task_RaiseCurtainAtStart(taskId: u8) {
    'l1: {
        match gTasks[taskId].data[0] {
            0 => {
                if ({
                    let t1 = gTasks[taskId].data[1];
                    gTasks[taskId].data[1] += 1;
                    t1
                }) <= 60
                {
                    break 'l1;
                }
                gTasks[taskId].data[1] = 0;
                PlaySE12WithPanning(SE_CONTEST_CURTAIN_RISE, 0);
                gTasks[taskId].data[0] += 1;
            }
            1 => {
                *(&raw mut gBattle_BG1_Y as *mut i16) += 7;
                if gBattle_BG1_Y as i16 <= DISPLAY_HEIGHT as i16 {
                    break 'l1;
                }
                gTasks[taskId].data[0] += 1;
            }
            2 => {
                UpdateContestantBoxOrder();
                gTasks[taskId].data[0] += 1;
            }
            3 => {
                let mut bg0Cnt: u16 = GetGpuReg(REG_OFFSET_BG0CNT);
                let mut bg2Cnt: u16 = GetGpuReg(REG_OFFSET_BG2CNT);
                (*(&raw mut bg0Cnt as *mut BgCnt)).set_priority(0);
                (*(&raw mut bg2Cnt as *mut BgCnt)).set_priority(0);
                SetGpuReg(REG_OFFSET_BG0CNT, bg0Cnt);
                SetGpuReg(REG_OFFSET_BG2CNT, bg2Cnt);
                SlideApplauseMeterIn();
                gTasks[taskId].data[0] += 1;
                break 'l1;
            }
            _ => {
                if (*(*gContestResources).contest).applauseMeterIsMoving() != 0 {
                    break 'l1;
                }
                gTasks[taskId].data[0] = 0;
                gTasks[taskId].data[1] = 0;
                gTasks[taskId].func = Some(Task_DisplayAppealNumberText);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ContestMain() {
    let mut i: i32 = 0;
    AnimateSprites();
    RunTasks();
    BuildOamBuffer();
    UpdatePaletteFade();
    i = 0;
    while i < 4 {
        if shr_i32(sContestBgCopyFlags as i32, i as u32) & 1 != 0 {
            CopyBgTilemapBufferToVram(i as u8);
        }
        i += 1;
    }
    sContestBgCopyFlags = 0;
}
pub(crate) unsafe extern "C" fn VBlankCB_Contest() {
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
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe extern "C" fn Task_DisplayAppealNumberText(taskId: u8) {
    if gTasks[taskId].data[0] == 0 {
        gBattle_BG0_Y = 0;
        gBattle_BG2_Y = 0;
        ContestDebugDoPrint();
        {
            let mut _src: *mut c_void = gPlttBufferUnfaded.as_mut_ptr() as *mut c_void;
            let mut _dest: *mut c_void = (*(gHeap.as_mut_ptr().at(106500) as *mut ContestTempSave))
                .cachedPlttBufferUnfaded
                .as_mut_ptr() as *mut c_void;
            let mut _size: u32 = PLTT_SIZE;
            {
                {
                    {
                        let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                        volatile_write(dmaRegs, _src as usize as u32);
                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                        volatile_write(dmaRegs.at(2), 0x84000000 | _size / 4);
                        let _ = (dmaRegs.at(2)).read_volatile();
                    }
                }
            }
        }
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*(*gContestResources).contest).appealNumber as i32 + 1,
            STR_CONV_MODE_LEFT_ALIGN,
            1,
        );
        if Contest_IsMonsTurnDisabled(gContestPlayerMonIndex) == 0 {
            StringCopy(
                gDisplayedStringBattle.as_mut_ptr(),
                gText_AppealNumWhichMoveWillBePlayed.as_ptr().cast_mut(),
            );
        } else {
            StringCopy(
                gDisplayedStringBattle.as_mut_ptr(),
                gText_AppealNumButItCantParticipate.as_ptr().cast_mut(),
            );
        }
        ContestClearGeneralTextWindow();
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gDisplayedStringBattle.as_mut_ptr(),
        );
        Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
        gTasks[taskId].data[0] += 1;
    } else {
        if Contest_RunTextPrinters() == 0 {
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].func = Some(Task_TryShowMoveSelectScreen);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TryShowMoveSelectScreen(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys == B_BUTTON as u16 {
        PlaySE(SE_SELECT);
        if Contest_IsMonsTurnDisabled(gContestPlayerMonIndex) == 0 {
            SetBottomSliderHeartsInvisibility(TRUE);
            gTasks[taskId].func = Some(Task_ShowMoveSelectScreen);
        } else {
            gTasks[taskId].func = Some(Task_SelectedMove);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowMoveSelectScreen(taskId: u8) {
    let mut i: u8 = 0;
    let mut moveName: CArray<u8, 32> = zeroed();
    gBattle_BG0_Y = DISPLAY_HEIGHT;
    gBattle_BG2_Y = DISPLAY_HEIGHT;
    i = 0;
    while i < MAX_MON_MOVES as u8 {
        let mut r#move: u16 = gContestMons[gContestPlayerMonIndex].moves[i];
        let mut moveNameBuffer: *mut u8 = moveName.as_mut_ptr();
        if (*(*gContestResources).status.at(gContestPlayerMonIndex)).prevMove != MOVE_NONE
            && IsContestantAllowedToCombo(gContestPlayerMonIndex) != 0
            && AreMovesContestCombo(
                (*(*gContestResources).status.at(gContestPlayerMonIndex)).prevMove,
                r#move,
            ) != 0
            && (*(*gContestResources).status.at(gContestPlayerMonIndex)).hasJudgesAttention() != 0
        {
            moveNameBuffer = StringCopy(
                moveName.as_mut_ptr(),
                gText_ColorLightShadowDarkGray.as_ptr().cast_mut(),
            );
        } else if r#move != MOVE_NONE
            && (*(*gContestResources).status.at(gContestPlayerMonIndex)).prevMove == r#move
            && gContestMoves[r#move].effect != CONTEST_EFFECT_REPETITION_NOT_BORING
        {
            moveNameBuffer = StringCopy(moveName.as_mut_ptr(), gText_ColorBlue.as_ptr().cast_mut());
        }
        moveNameBuffer = StringCopy(moveNameBuffer, gMoveNames[r#move].as_ptr().cast_mut());
        FillWindowPixelBuffer(i + WIN_MOVE0, 0);
        Contest_PrintTextToBg0WindowAt(
            i as u32 + WIN_MOVE0 as u32,
            moveName.as_mut_ptr(),
            5,
            1,
            FONT_NARROW as i32,
        );
        i += 1;
    }
    DrawMoveSelectArrow((*(*gContestResources).contest).playerMoveChoice as i8);
    PrintContestMoveDescription(
        gContestMons[gContestPlayerMonIndex].moves
            [(*(*gContestResources).contest).playerMoveChoice],
    );
    gTasks[taskId].func = Some(Task_HandleMoveSelectInput);
}
pub(crate) unsafe extern "C" fn Task_HandleMoveSelectInput(taskId: u8) {
    let mut numMoves: u8 = 0;
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        if gContestMons[gContestPlayerMonIndex].moves[i] != MOVE_NONE {
            numMoves += 1;
        }
        i += 1;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        gTasks[taskId].func = Some(Task_SelectedMove);
    } else {
        match gMain.newAndRepeatedKeys {
            2 => {
                PlaySE(SE_SELECT);
                SetBottomSliderHeartsInvisibility(FALSE);
                ConvertIntToDecimalStringN(
                    gStringVar1.as_mut_ptr(),
                    (*(*gContestResources).contest).appealNumber as i32 + 1,
                    STR_CONV_MODE_LEFT_ALIGN,
                    1,
                );
                if Contest_IsMonsTurnDisabled(gContestPlayerMonIndex) == 0 {
                    StringCopy(
                        gDisplayedStringBattle.as_mut_ptr(),
                        gText_AppealNumWhichMoveWillBePlayed.as_ptr().cast_mut(),
                    );
                } else {
                    StringCopy(
                        gDisplayedStringBattle.as_mut_ptr(),
                        gText_AppealNumButItCantParticipate.as_ptr().cast_mut(),
                    );
                }
                ContestClearGeneralTextWindow();
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gDisplayedStringBattle.as_mut_ptr(),
                );
                Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), FALSE as u32);
                gBattle_BG0_Y = 0;
                gBattle_BG2_Y = 0;
                gTasks[taskId].func = Some(Task_TryShowMoveSelectScreen);
            }
            32 | 16 => {}
            64 => {
                EraseMoveSelectArrow((*(*gContestResources).contest).playerMoveChoice as i8);
                if (*(*gContestResources).contest).playerMoveChoice == 0 {
                    (*(*gContestResources).contest).playerMoveChoice = numMoves - 1;
                } else {
                    (*(*gContestResources).contest).playerMoveChoice -= 1;
                }
                DrawMoveSelectArrow((*(*gContestResources).contest).playerMoveChoice as i8);
                PrintContestMoveDescription(
                    gContestMons[gContestPlayerMonIndex].moves
                        [(*(*gContestResources).contest).playerMoveChoice],
                );
                if numMoves > 1 {
                    PlaySE(SE_SELECT);
                }
            }
            128 => {
                EraseMoveSelectArrow((*(*gContestResources).contest).playerMoveChoice as i8);
                if (*(*gContestResources).contest).playerMoveChoice as i32 == numMoves as i32 - 1 {
                    (*(*gContestResources).contest).playerMoveChoice = 0;
                } else {
                    (*(*gContestResources).contest).playerMoveChoice += 1;
                }
                DrawMoveSelectArrow((*(*gContestResources).contest).playerMoveChoice as i8);
                PrintContestMoveDescription(
                    gContestMons[gContestPlayerMonIndex].moves
                        [(*(*gContestResources).contest).playerMoveChoice],
                );
                if numMoves > 1 {
                    PlaySE(SE_SELECT);
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn DrawMoveSelectArrow(moveIndex: i8) {
    ContestBG_FillBoxWithIncrementingTile(2, 55, 0, 31 + moveIndex as u8 * 2, 2, 2, 17, 1);
}
pub(crate) unsafe extern "C" fn EraseMoveSelectArrow(moveIndex: i8) {
    ContestBG_FillBoxWithIncrementingTile(2, 11, 0, 31 + moveIndex as u8 * 2, 2, 1, 17, 1);
    ContestBG_FillBoxWithIncrementingTile(2, 11, 0, 32 + moveIndex as u8 * 2, 2, 1, 17, 1);
}
pub(crate) unsafe extern "C" fn Task_SelectedMove(taskId: u8) {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        let mut r#move: u16 = GetChosenMove(gContestPlayerMonIndex);
        let mut taskId2: u8 = 0;
        (*(*gContestResources).status.at(gContestPlayerMonIndex)).currMove = r#move;
        taskId2 = CreateTask(Some(Task_LinkContest_CommunicateMoveSelections), 0);
        SetTaskFuncWithFollowupFunc(
            taskId2,
            Some(Task_LinkContest_CommunicateMoveSelections),
            Some(Task_EndCommunicateMoveSelections),
        );
        gTasks[taskId].func = Some(TaskDummy1);
        ContestPrintLinkStandby();
        SetBottomSliderHeartsInvisibility(FALSE);
    } else {
        GetAllChosenMoves();
        gTasks[taskId].func = Some(Task_HideMoveSelectScreen);
    }
}
pub(crate) unsafe extern "C" fn Task_EndCommunicateMoveSelections(taskId: u8) {
    DestroyTask(taskId);
    gTasks[(*(*gContestResources).contest).mainTaskId].func = Some(Task_HideMoveSelectScreen);
}
pub(crate) unsafe extern "C" fn Task_HideMoveSelectScreen(taskId: u8) {
    let mut i: i32 = 0;
    ContestClearGeneralTextWindow();
    gBattle_BG0_Y = 0;
    gBattle_BG2_Y = 0;
    SetBottomSliderHeartsInvisibility(FALSE);
    i = 0;
    while i < MAX_MON_MOVES {
        FillWindowPixelBuffer(WIN_MOVE0 + i as u8, 0);
        PutWindowTilemap(WIN_MOVE0 + i as u8);
        CopyWindowToVram(WIN_MOVE0 + i as u8, COPYWIN_GFX);
        i += 1;
    }
    Contest_SetBgCopyFlags(0);
    {
        let mut _src: *mut c_void = gPlttBufferFaded.as_mut_ptr() as *mut c_void;
        let mut _dest: *mut c_void = (*(gHeap.as_mut_ptr().at(106500) as *mut ContestTempSave))
            .cachedPlttBufferFaded
            .as_mut_ptr() as *mut c_void;
        let mut _size: u32 = PLTT_SIZE;
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, _src as usize as u32);
                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x84000000 | _size / 4);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    LoadPalette(
        (*(gHeap.as_mut_ptr().at(106500) as *mut ContestTempSave))
            .cachedPlttBufferUnfaded
            .as_mut_ptr() as *mut c_void,
        0,
        PLTT_SIZE as u16,
    );
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = 0;
    gTasks[taskId].func = Some(Task_HideApplauseMeterForAppealStart);
}
pub(crate) unsafe extern "C" fn Task_HideApplauseMeterForAppealStart(taskId: u8) {
    if ({
        gTasks[taskId].data[0] += 1;
        gTasks[taskId].data[0]
    }) > 2
    {
        gTasks[taskId].data[0] = 0;
        if ({
            gTasks[taskId].data[1] += 1;
            gTasks[taskId].data[1]
        }) == 2
        {
            SlideApplauseMeterOut();
            AnimateSliderHearts(SLIDER_HEART_ANIM_DISAPPEAR);
            gTasks[taskId].func = Some(Task_WaitHideApplauseMeterForAppealStart);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitHideApplauseMeterForAppealStart(taskId: u8) {
    if (*(*gContestResources).contest).applauseMeterIsMoving() == 0
        && (*(*gContestResources).contest).sliderHeartsAnimating() == 0
    {
        gTasks[taskId].func = Some(Task_AppealSetup);
    }
}
pub(crate) unsafe extern "C" fn Task_AppealSetup(taskId: u8) {
    if ({
        gTasks[taskId].data[0] += 1;
        gTasks[taskId].data[0]
    }) > 19
    {
        (*(*gContestResources).contest).turnNumber = 0;
        (*(*gContestResources).contest).unusedRng = gRngValue;
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 && IsPlayerLinkLeader() != 0 {
            let mut i: i32 = 0;
            i = 0;
            while (i + gNumLinkContestPlayers as i32) < CONTESTANT_COUNT {
                (*(*gContestResources)
                    .status
                    .at(gNumLinkContestPlayers as i32 + i))
                .currMove = GetChosenMove(gNumLinkContestPlayers + i as u8);
                i += 1;
            }
        }
        gTasks[taskId].data[0] = APPEALSTATE_START_TURN;
        gTasks[taskId].func = Some(Task_DoAppeals);
    }
}
pub(crate) unsafe extern "C" fn Task_DoAppeals(taskId: u8) {
    let mut spriteId: u8 = 0;
    let mut i: i32 = 0;
    let mut contestant: u8 = (*(*gContestResources).contest).currentContestant;
    let mut r3: i8 = 0;
    match gTasks[taskId].data[0] {
        APPEALSTATE_START_TURN => {
            ContestDebugDoPrint();
            i = 0;
            while (*(*gContestResources).contest).turnNumber
                != (*(*gContestResources).appealResults).turnOrder[i]
            {
                i += 1;
            }
            (*(*gContestResources).contest).currentContestant = i as u8;
            contestant = (*(*gContestResources).contest).currentContestant;
            if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
                let mut taskId2: u8 = 0;
                (*(*gContestResources).contest).set_waitForLink(TRUE as u16);
                if IsPlayerLinkLeader() != 0 {
                    CalculateAppealMoveImpact((*(*gContestResources).contest).currentContestant);
                }
                taskId2 = CreateTask(Some(Task_LinkContest_CommunicateAppealsState), 0);
                SetTaskFuncWithFollowupFunc(
                    taskId2,
                    Some(Task_LinkContest_CommunicateAppealsState),
                    Some(Task_EndWaitForLink),
                );
                ContestPrintLinkStandby();
                gTasks[taskId].data[0] = APPEALSTATE_WAIT_LINK;
            } else {
                CalculateAppealMoveImpact((*(*gContestResources).contest).currentContestant);
                gTasks[taskId].data[0] = APPEALSTATE_CHECK_SKIP_TURN;
            }
            return;
        }
        APPEALSTATE_WAIT_LINK => {
            if (*(*gContestResources).contest).waitForLink() == 0 {
                gTasks[taskId].data[0] = APPEALSTATE_CHECK_SKIP_TURN;
            }
            return;
        }
        APPEALSTATE_CHECK_SKIP_TURN => {
            SetContestLiveUpdateFlags(contestant);
            ContestDebugPrintBitStrings();
            if (*(*gContestResources).status.at(contestant)).numTurnsSkipped() != 0
                || (*(*gContestResources).status.at(contestant)).noMoreTurns() != 0
            {
                gTasks[taskId].data[0] = APPEALSTATE_PRINT_SKIP_TURN_MSG;
            } else {
                ContestClearGeneralTextWindow();
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[0] = APPEALSTATE_SLIDE_MON_IN;
            }
            return;
        }
        APPEALSTATE_SLIDE_MON_IN => {
            i = 0;
            while i < CONTESTANT_COUNT {
                gBattleMonForms[i] = 0;
                i += 1;
            }
            memset((*gContestResources).moveAnim as *mut u8, 0, 20);
            SetMoveAnimAttackerData((*(*gContestResources).contest).currentContestant);
            spriteId = CreateContestantSprite(
                gContestMons[(*(*gContestResources).contest).currentContestant].species,
                gContestMons[(*(*gContestResources).contest).currentContestant].otId,
                gContestMons[(*(*gContestResources).contest).currentContestant].personality,
                (*(*gContestResources).contest).currentContestant as u32,
            );
            gSprites[spriteId].x2 = 120;
            gSprites[spriteId].callback = Some(SpriteCB_MonSlideIn);
            gTasks[taskId].data[2] = spriteId as i16;
            gBattlerSpriteIds[gBattlerAttacker] = spriteId;
            BlinkContestantBox(
                CreateContestantBoxBlinkSprites((*(*gContestResources).contest).currentContestant),
                FALSE,
            );
            gTasks[taskId].data[0] = APPEALSTATE_WAIT_SLIDE_MON;
            return;
        }
        APPEALSTATE_WAIT_SLIDE_MON => {
            spriteId = gTasks[taskId].data[2] as u8;
            if gSprites[spriteId].callback
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            {
                if (*(*gContestResources).gfxState.at(contestant)).boxBlinking() == 0 {
                    gTasks[taskId].data[0] = APPEALSTATE_PRINT_USED_MOVE_MSG;
                }
            }
            return;
        }
        APPEALSTATE_PRINT_USED_MOVE_MSG => {
            if (*(*gContestResources).status.at(contestant)).nervous() != 0 {
                gTasks[taskId].data[0] = APPEALSTATE_PRINT_TOO_NERVOUS_MSG;
            } else {
                ContestClearGeneralTextWindow();
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    gContestMons[contestant].nickname.as_mut_ptr(),
                );
                if (*(*gContestResources).status.at(contestant)).currMove < MOVES_COUNT {
                    StringCopy(
                        gStringVar2.as_mut_ptr(),
                        gMoveNames[(*(*gContestResources).status.at(contestant)).currMove]
                            .as_ptr()
                            .cast_mut(),
                    );
                } else {
                    StringCopy(
                        gStringVar2.as_mut_ptr(),
                        sInvalidContestMoveNames
                            [(*(*gContestResources).status.at(contestant)).moveCategory],
                    );
                }
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_MonAppealedWithMove.as_ptr().cast_mut(),
                );
                Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
                gTasks[taskId].data[0] = APPEALSTATE_WAIT_USED_MOVE_MSG;
            }
            return;
        }
        APPEALSTATE_WAIT_USED_MOVE_MSG => {
            if Contest_RunTextPrinters() == 0 {
                (*(*gContestResources).contest).moveAnimTurnCount = 0;
                gTasks[taskId].data[0] = APPEALSTATE_MOVE_ANIM;
            }
            return;
        }
        APPEALSTATE_MOVE_ANIM => {
            {
                let mut r#move: u16 = SanitizeMove(
                    (*(*gContestResources)
                        .status
                        .at((*(*gContestResources).contest).currentContestant))
                    .currMove,
                );
                SetMoveSpecificAnimData((*(*gContestResources).contest).currentContestant);
                SetMoveAnimAttackerData((*(*gContestResources).contest).currentContestant);
                SetMoveTargetPosition(r#move);
                DoMoveAnim(r#move);
                gTasks[taskId].data[0] = APPEALSTATE_WAIT_MOVE_ANIM;
            }
            return;
        }
        APPEALSTATE_WAIT_MOVE_ANIM => {
            gAnimScriptCallback.unwrap_unchecked()();
            if gAnimScriptActive == 0 {
                ClearMoveAnimData(contestant);
                if (*(*gContestResources).contest).moveAnimTurnCount != 0 {
                    gTasks[taskId].data[10] = 0;
                    gTasks[taskId].data[0] = APPEALSTATE_MOVE_ANIM_MULTITURN;
                } else {
                    if (*(*gContestResources).status.at(contestant)).hasJudgesAttention() == 0 {
                        StopFlashJudgeAttentionEye(contestant);
                    }
                    DrawUnnervedSymbols();
                    gTasks[taskId].data[0] = APPEALSTATE_TRY_PRINT_MOVE_RESULT;
                }
            }
            return;
        }
        APPEALSTATE_MOVE_ANIM_MULTITURN => {
            if ({
                let t1 = gTasks[taskId].data[10];
                gTasks[taskId].data[10] += 1;
                t1
            }) > 30
            {
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[0] = APPEALSTATE_MOVE_ANIM;
            }
            return;
        }
        APPEALSTATE_TRY_PRINT_MOVE_RESULT => {
            gTasks[taskId].data[1] = 0;
            if (*(*gContestResources).status.at(contestant)).effectStringId != CONTEST_STRING_NONE {
                PrintAppealMoveResultText(
                    contestant,
                    (*(*gContestResources).status.at(contestant)).effectStringId,
                );
                (*(*gContestResources).status.at(contestant)).effectStringId = CONTEST_STRING_NONE;
                gTasks[taskId].data[0] = APPEALSTATE_WAIT_MOVE_RESULT_MSG;
            } else {
                if (*(*gContestResources).status.at(contestant)).effectStringId2
                    != CONTEST_STRING_NONE
                {
                    i = 0;
                    while i < CONTESTANT_COUNT {
                        if i != contestant as i32
                            && (*(*gContestResources).status.at(i)).effectStringId
                                != CONTEST_STRING_NONE
                        {
                            break;
                        }
                        i += 1;
                    }
                    if i == CONTESTANT_COUNT {
                        PrintAppealMoveResultText(
                            contestant,
                            (*(*gContestResources).status.at(contestant)).effectStringId2,
                        );
                        (*(*gContestResources).status.at(contestant)).effectStringId2 =
                            CONTEST_STRING_NONE;
                        gTasks[taskId].data[0] = APPEALSTATE_WAIT_MOVE_RESULT_MSG;
                    } else {
                        gTasks[taskId].data[0] = APPEALSTATE_CHECK_TURN_ORDER_MOD;
                    }
                } else {
                    gTasks[taskId].data[0] = APPEALSTATE_CHECK_TURN_ORDER_MOD;
                }
            }
            return;
        }
        APPEALSTATE_WAIT_MOVE_RESULT_MSG => {
            if Contest_RunTextPrinters() == 0 {
                gTasks[taskId].data[0] = APPEALSTATE_TRY_PRINT_MOVE_RESULT;
            }
            return;
        }
        APPEALSTATE_CHECK_TURN_ORDER_MOD => {
            if (*(*gContestResources).status.at(contestant)).turnOrderModAction() == 1 {
                DoJudgeSpeechBubble(JUDGE_SYMBOL_NUMBER_ONE);
            } else if (*(*gContestResources).status.at(contestant)).turnOrderModAction() == 2 {
                DoJudgeSpeechBubble(JUDGE_SYMBOL_NUMBER_FOUR);
            } else if (*(*gContestResources).status.at(contestant)).turnOrderModAction() == 3 {
                DoJudgeSpeechBubble(JUDGE_SYMBOL_QUESTION_MARK);
            } else {
                gTasks[taskId].data[0] = APPEALSTATE_TRY_SHOW_NEXT_TURN_GFX;
                return;
            }
            gTasks[taskId].data[0] = APPEALSTATE_WAIT_JUDGE_TURN_ORDER;
            return;
        }
        APPEALSTATE_WAIT_JUDGE_TURN_ORDER => {
            if (*(*gContestResources).contest).waitForJudgeSpeechBubble() == 0 {
                gTasks[taskId].data[0] = APPEALSTATE_TRY_SHOW_NEXT_TURN_GFX;
            }
            return;
        }
        APPEALSTATE_TRY_SHOW_NEXT_TURN_GFX => {
            ShowHideNextTurnGfx(TRUE);
            gTasks[taskId].data[0] = APPEALSTATE_UPDATE_MOVE_USERS_HEARTS;
            return;
        }
        APPEALSTATE_UPDATE_MOVE_USERS_HEARTS => {
            UpdateAppealHearts(
                0,
                (*(*gContestResources).status.at(contestant)).appeal,
                contestant,
            );
            gTasks[taskId].data[0] = APPEALSTATE_WAIT_MOVE_USERS_HEARTS;
            return;
        }
        APPEALSTATE_WAIT_MOVE_USERS_HEARTS => {
            if (*(*gContestResources)
                .gfxState
                .at((*(*gContestResources).contest).currentContestant))
            .updatingAppealHearts()
                == 0
            {
                gTasks[taskId].data[0] = APPEALSTATE_TRY_JUDGE_STAR;
            }
            return;
        }
        APPEALSTATE_TRY_JUDGE_STAR => {
            if (*(*gContestResources).status.at(contestant)).conditionMod() == CONDITION_GAIN {
                DoJudgeSpeechBubble(JUDGE_SYMBOL_STAR);
            }
            gTasks[taskId].data[0] = APPEALSTATE_WAIT_JUDGE_STAR;
            return;
        }
        APPEALSTATE_WAIT_JUDGE_STAR => {
            if (*(*gContestResources).contest).waitForJudgeSpeechBubble() == 0 {
                gTasks[taskId].data[0] = APPEALSTATE_UPDATE_MOVE_USERS_STARS;
            }
            return;
        }
        APPEALSTATE_UPDATE_MOVE_USERS_STARS => {
            if UpdateConditionStars(contestant, TRUE) != 0 {
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[0] = APPEALSTATE_WAIT_MOVE_USERS_STARS;
            } else {
                gTasks[taskId].data[0] = APPEALSTATE_UPDATE_MOVE_USERS_STATUS;
            }
            return;
        }
        APPEALSTATE_WAIT_MOVE_USERS_STARS => {
            if ({
                gTasks[taskId].data[10] += 1;
                gTasks[taskId].data[10]
            }) > 20
            {
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[0] = APPEALSTATE_UPDATE_MOVE_USERS_STATUS;
            }
            return;
        }
        APPEALSTATE_UPDATE_MOVE_USERS_STATUS => {
            if DrawStatusSymbol(contestant) != 0 {
                PlaySE(SE_CONTEST_ICON_CHANGE);
            }
            gTasks[taskId].data[0] = APPEALSTATE_UPDATE_OPPONENTS;
            return;
        }
        APPEALSTATE_UPDATE_OPPONENTS => {
            gTasks[taskId].data[1] = 0;
            gTasks[taskId].data[0] = APPEALSTATE_UPDATE_OPPONENT;
            return;
        }
        APPEALSTATE_UPDATE_OPPONENT => {
            {
                let mut j: i32 = 0;
                r3 = FALSE as i8;
                i = gTasks[taskId].data[1] as i32;
                while i < CONTESTANT_COUNT {
                    r3 = FALSE as i8;
                    j = 0;
                    while j < CONTESTANT_COUNT {
                        if j != contestant as i32
                            && gContestantTurnOrder[j] as i32 == i
                            && (*(*gContestResources).status.at(j)).effectStringId
                                != CONTEST_STRING_NONE
                        {
                            r3 = TRUE as i8;
                            break;
                        }
                        j += 1;
                    }
                    if r3 != 0 {
                        break;
                    }
                    i += 1;
                }
                if r3 != 0 {
                    gTasks[taskId].data[1] = gContestantTurnOrder[j] as i16;
                    PrintAppealMoveResultText(
                        j as u8,
                        (*(*gContestResources).status.at(j)).effectStringId,
                    );
                    (*(*gContestResources).status.at(j)).effectStringId = CONTEST_STRING_NONE;
                    gTasks[taskId].data[0] = APPEALSTATE_WAIT_OPPONENT_RESPONSE_MSG;
                } else {
                    gTasks[taskId].data[1] = 0;
                    gTasks[taskId].data[10] = 0;
                    gTasks[taskId].data[0] = APPEALSTATE_TRY_PRINT_SKIP_NEXT_TURN_MSG;
                    DrawStatusSymbols();
                }
            }
            return;
        }
        APPEALSTATE_WAIT_OPPONENT_RESPONSE_MSG => {
            if Contest_RunTextPrinters() == 0 {
                gTasks[taskId].data[0] = APPEALSTATE_UPDATE_OPPONENT_HEARTS;
            }
            return;
        }
        APPEALSTATE_UPDATE_OPPONENT_HEARTS => {
            i = 0;
            while gTasks[taskId].data[1] != gContestantTurnOrder[i] as i16 {
                i += 1;
            }
            UpdateAppealHearts(
                (*(*gContestResources).status.at(i)).appeal
                    + (*(*gContestResources).status.at(i)).jam as i16,
                -((*(*gContestResources).status.at(i)).jam as i16),
                i as u8,
            );
            gTasks[taskId].data[0] = APPEALSTATE_WAIT_OPPONENT_HEARTS;
            return;
        }
        APPEALSTATE_WAIT_OPPONENT_HEARTS => {
            i = 0;
            while gTasks[taskId].data[1] != gContestantTurnOrder[i] as i16 {
                i += 1;
            }
            if (*(*gContestResources).gfxState.at(i)).updatingAppealHearts() == 0 {
                gTasks[taskId].data[0] = APPEALSTATE_UPDATE_OPPONENT_STARS;
            }
            return;
        }
        APPEALSTATE_UPDATE_OPPONENT_STARS => {
            i = 0;
            while gTasks[taskId].data[1] != gContestantTurnOrder[i] as i16 {
                i += 1;
            }
            if UpdateConditionStars(i as u8, TRUE) != 0 {
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[0] = APPEALSTATE_WAIT_OPPONENT_STARS;
            } else {
                gTasks[taskId].data[0] = APPEALSTATE_UPDATE_OPPONENT_STATUS;
            }
            return;
        }
        APPEALSTATE_WAIT_OPPONENT_STARS => {
            if ({
                gTasks[taskId].data[10] += 1;
                gTasks[taskId].data[10]
            }) > 20
            {
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[0] = APPEALSTATE_UPDATE_OPPONENT_STATUS;
            }
            return;
        }
        APPEALSTATE_UPDATE_OPPONENT_STATUS => {
            i = 0;
            while i < CONTESTANT_COUNT {
                if gContestantTurnOrder[i] as i16 == gTasks[taskId].data[1] {
                    break;
                }
                i += 1;
            }
            if DrawStatusSymbol(i as u8) != 0 {
                PlaySE(SE_CONTEST_ICON_CHANGE);
            } else {
                PlaySE(SE_CONTEST_ICON_CLEAR);
            }
            if (*(*gContestResources).status.at(i)).judgesAttentionWasRemoved() != 0 {
                StopFlashJudgeAttentionEye(i as u8);
                (*(*gContestResources).status.at(i)).set_judgesAttentionWasRemoved(FALSE);
            }
            gTasks[taskId].data[1] += 1;
            gTasks[taskId].data[0] = APPEALSTATE_UPDATE_OPPONENT;
            return;
        }
        APPEALSTATE_TRY_PRINT_SKIP_NEXT_TURN_MSG => {
            if ({
                let t4 = gTasks[taskId].data[10];
                gTasks[taskId].data[10] += 1;
                t4
            }) > 9
            {
                gTasks[taskId].data[10] = 0;
                if (*(*gContestResources).status.at(contestant)).numTurnsSkipped() != 0
                    || (*(*gContestResources).status.at(contestant)).turnSkipped() != 0
                {
                    ContestClearGeneralTextWindow();
                    StringCopy(
                        gStringVar1.as_mut_ptr(),
                        gContestMons[contestant].nickname.as_mut_ptr(),
                    );
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        gText_MonCantAppealNextTurn.as_ptr().cast_mut(),
                    );
                    Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
                }
                gTasks[taskId].data[0] = APPEALSTATE_WAIT_SKIP_NEXT_TURN_MSG;
            }
            return;
        }
        APPEALSTATE_WAIT_SKIP_NEXT_TURN_MSG => {
            if Contest_RunTextPrinters() == 0 {
                if (*(*gContestResources).status.at(contestant)).usedComboMove() == 0 {
                    gTasks[taskId].data[0] = APPEALSTATE_CHECK_REPEATED_MOVE;
                } else {
                    gTasks[taskId].data[0] = APPEALSTATE_PRINT_COMBO_MSG;
                }
            }
            return;
        }
        APPEALSTATE_PRINT_COMBO_MSG => {
            let mut completedCombo: i8 =
                (*(*gContestResources).status.at(contestant)).completedCombo as i8;
            if (*(*gContestResources).status.at(contestant)).completedCombo != 0 {
                ContestClearGeneralTextWindow();
                if completedCombo == 1 {
                    Contest_StartTextPrinter(
                        gText_AppealComboWentOverWell.as_ptr().cast_mut(),
                        TRUE as u32,
                    );
                } else if completedCombo == 2 {
                    Contest_StartTextPrinter(
                        gText_AppealComboWentOverVeryWell.as_ptr().cast_mut(),
                        TRUE as u32,
                    );
                } else {
                    Contest_StartTextPrinter(
                        gText_AppealComboWentOverExcellently.as_ptr().cast_mut(),
                        TRUE as u32,
                    );
                }
                DoJudgeSpeechBubble(JUDGE_SYMBOL_TWO_EXCLAMATIONS);
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[0] = APPEALSTATE_WAIT_JUDGE_COMBO;
            } else {
                ContestClearGeneralTextWindow();
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    gContestMons[contestant].nickname.as_mut_ptr(),
                );
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_JudgeLookedAtMonExpectantly.as_ptr().cast_mut(),
                );
                Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
                DoJudgeSpeechBubble(JUDGE_SYMBOL_ONE_EXCLAMATION);
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[0] = APPEALSTATE_WAIT_JUDGE_COMBO;
            }
            return;
        }
        APPEALSTATE_WAIT_JUDGE_COMBO => {
            if (*(*gContestResources).contest).waitForJudgeSpeechBubble() == 0 {
                StartStopFlashJudgeAttentionEye((*(*gContestResources).contest).currentContestant);
                gTasks[taskId].data[0] = APPEALSTATE_TRY_UPDATE_HEARTS_FROM_COMBO;
            }
            return;
        }
        APPEALSTATE_TRY_UPDATE_HEARTS_FROM_COMBO => {
            if Contest_RunTextPrinters() == 0 {
                if ({
                    gTasks[taskId].data[10] += 1;
                    gTasks[taskId].data[10]
                }) > 50
                {
                    if (*(*gContestResources).status.at(contestant)).hasJudgesAttention() == 0 {
                        UpdateAppealHearts(
                            (*(*gContestResources).status.at(contestant)).appeal,
                            (*(*gContestResources).status.at(contestant)).comboAppealBonus as i16,
                            contestant,
                        );
                        (*(*gContestResources).status.at(contestant)).appeal +=
                            (*(*gContestResources).status.at(contestant)).comboAppealBonus as i16;
                    }
                    gTasks[taskId].data[0] = APPEALSTATE_WAIT_HEARTS_FROM_COMBO;
                }
            }
            return;
        }
        APPEALSTATE_WAIT_HEARTS_FROM_COMBO => {
            if (*(*gContestResources).gfxState.at(contestant)).updatingAppealHearts() == 0 {
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[0] = APPEALSTATE_CHECK_REPEATED_MOVE;
            }
            return;
        }
        APPEALSTATE_CHECK_REPEATED_MOVE => {
            if (*(*gContestResources).status.at(contestant)).repeatedMove() != 0 {
                ContestClearGeneralTextWindow();
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    gContestMons[contestant].nickname.as_mut_ptr(),
                );
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_RepeatedAppeal.as_ptr().cast_mut(),
                );
                Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
                gTasks[taskId].data[10] = 0;
                DoJudgeSpeechBubble(JUDGE_SYMBOL_SWIRL);
                gTasks[taskId].data[0] = APPEALSTATE_WAIT_JUDGE_REPEATED_MOVE;
            } else {
                gTasks[taskId].data[0] = APPEALSTATE_UPDATE_CROWD;
            }
            return;
        }
        APPEALSTATE_WAIT_JUDGE_REPEATED_MOVE => {
            if (*(*gContestResources).contest).waitForJudgeSpeechBubble() == 0 {
                gTasks[taskId].data[0] = APPEALSTATE_UPDATE_HEARTS_FROM_REPEAT;
            }
            return;
        }
        APPEALSTATE_UPDATE_HEARTS_FROM_REPEAT => {
            if Contest_RunTextPrinters() == 0 {
                UpdateAppealHearts(
                    (*(*gContestResources).status.at(contestant)).appeal,
                    -((*(*gContestResources).status.at(contestant)).repeatJam as i16),
                    contestant,
                );
                (*(*gContestResources).status.at(contestant)).appeal -=
                    (*(*gContestResources).status.at(contestant)).repeatJam as i16;
                gTasks[taskId].data[0] = APPEALSTATE_WAIT_HEARTS_FROM_REPEAT;
            }
            return;
        }
        APPEALSTATE_WAIT_HEARTS_FROM_REPEAT => {
            ContestDebugDoPrint();
            if (*(*gContestResources).gfxState.at(contestant)).updatingAppealHearts() == 0 {
                gTasks[taskId].data[10] = 0;
                ContestClearGeneralTextWindow();
                gTasks[taskId].data[0] = APPEALSTATE_UPDATE_CROWD;
            }
            return;
        }
        APPEALSTATE_UPDATE_CROWD => {
            if (*(*gContestResources).excitement).frozen() != 0
                && contestant != (*(*gContestResources).excitement).freezer()
            {
                gTasks[taskId].data[0] = APPEALSTATE_PRINT_CROWD_WATCHES_MSG;
            } else {
                r3 = (*(*gContestResources).excitement).moveExcitement;
                if (*(*gContestResources).status.at(contestant)).overrideCategoryExcitementMod()
                    != 0
                {
                    r3 = 1;
                    StringCopy(
                        gStringVar3.as_mut_ptr(),
                        gMoveNames[(*(*gContestResources).status.at(contestant)).currMove]
                            .as_ptr()
                            .cast_mut(),
                    );
                } else {
                    StringCopy(
                        gStringVar3.as_mut_ptr(),
                        sContestConditions[gContestMoves
                            [(*(*gContestResources).status.at(contestant)).currMove]
                            .contestCategory()],
                    );
                }
                if r3 > 0 && (*(*gContestResources).status.at(contestant)).repeatedMove() != 0 {
                    r3 = 0;
                }
                ContestClearGeneralTextWindow();
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    gContestMons[contestant].nickname.as_mut_ptr(),
                );
                (*(*gContestResources).contest).applauseLevel += r3;
                if (*(*gContestResources).contest).applauseLevel < 0 {
                    (*(*gContestResources).contest).applauseLevel = 0;
                }
                if r3 == 0 {
                    gTasks[taskId].data[0] = APPEALSTATE_SLIDE_APPLAUSE_OUT;
                } else {
                    if r3 < 0 {
                        StringExpandPlaceholders(
                            gStringVar4.as_mut_ptr(),
                            gText_MonsXDidntGoOverWell.as_ptr().cast_mut(),
                        );
                    } else if r3 > 0 && (*(*gContestResources).contest).applauseLevel <= 4 {
                        StringExpandPlaceholders(
                            gStringVar4.as_mut_ptr(),
                            gText_MonsXWentOverGreat.as_ptr().cast_mut(),
                        );
                    } else {
                        StringExpandPlaceholders(
                            gStringVar4.as_mut_ptr(),
                            gText_MonsXGotTheCrowdGoing.as_ptr().cast_mut(),
                        );
                    }
                    Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
                    gTasks[taskId].data[10] = 0;
                    gTasks[taskId].data[11] = 0;
                    if r3 < 0 {
                        gTasks[taskId].data[0] = APPEALSTATE_DO_CROWD_UNEXCITED;
                    } else {
                        gTasks[taskId].data[0] = APPEALSTATE_DO_CROWD_EXCITED;
                    }
                }
            }
            return;
        }
        APPEALSTATE_DO_CROWD_UNEXCITED => {
            match gTasks[taskId].data[10] {
                0 => {
                    BlendAudienceBackground(-1, 1);
                    PlayFanfare(MUS_TOO_BAD);
                    gTasks[taskId].data[10] += 1;
                }
                1 => {
                    if (*(*gContestResources).contest).waitForAudienceBlend() == 0
                        && Contest_RunTextPrinters() == 0
                    {
                        ShowAndUpdateApplauseMeter(-1);
                        gTasks[taskId].data[10] += 1;
                    }
                }
                2 => {
                    if (*(*gContestResources).contest).isShowingApplauseMeter() == 0 {
                        if ({
                            let t6 = gTasks[taskId].data[11];
                            gTasks[taskId].data[11] += 1;
                            t6
                        }) > 29
                        {
                            gTasks[taskId].data[11] = 0;
                            BlendAudienceBackground(-1, -1);
                            gTasks[taskId].data[10] += 1;
                        }
                    }
                }
                3 => {
                    if gPaletteFade.active() == 0 {
                        gTasks[taskId].data[10] = 0;
                        gTasks[taskId].data[11] = 0;
                        gTasks[taskId].data[0] = APPEALSTATE_WAIT_EXCITEMENT_HEARTS;
                    }
                }
                _ => {}
            }
            return;
        }
        APPEALSTATE_DO_CROWD_EXCITED => {
            match gTasks[taskId].data[10] {
                0 => {
                    if Contest_RunTextPrinters() == 0 {
                        BlendAudienceBackground(1, 1);
                        gTasks[taskId].data[10] += 1;
                    }
                }
                1 => {
                    if (*(*gContestResources).contest).waitForAudienceBlend() == 0 {
                        AnimateAudience();
                        PlaySE(SE_M_ENCORE2);
                        ShowAndUpdateApplauseMeter(1);
                        gTasks[taskId].data[10] += 1;
                    }
                }
                2 => {
                    if (*(*gContestResources).contest).isShowingApplauseMeter() == 0 {
                        if ({
                            let t7 = gTasks[taskId].data[11];
                            gTasks[taskId].data[11] += 1;
                            t7
                        }) > 29
                        {
                            gTasks[taskId].data[11] = 0;
                            UpdateAppealHearts(
                                (*(*gContestResources).status.at(contestant)).appeal,
                                (*(*gContestResources).excitement).excitementAppealBonus as i16,
                                contestant,
                            );
                            (*(*gContestResources).status.at(contestant)).appeal +=
                                (*(*gContestResources).excitement).excitementAppealBonus as i16;
                            gTasks[taskId].data[10] += 1;
                        }
                    }
                }
                3 => {
                    if (*(*gContestResources).gfxState.at(contestant)).updatingAppealHearts() == 0 {
                        if (*(*gContestResources).contest).animatingAudience() == 0 {
                            BlendAudienceBackground(1, -1);
                            gTasks[taskId].data[10] += 1;
                        }
                    }
                }
                4 => {
                    if gPaletteFade.active() == 0 {
                        gTasks[taskId].data[10] = 0;
                        gTasks[taskId].data[11] = 0;
                        gTasks[taskId].data[0] = APPEALSTATE_WAIT_EXCITEMENT_HEARTS;
                    }
                }
                _ => {}
            }
            return;
        }
        APPEALSTATE_WAIT_EXCITEMENT_HEARTS => {
            if (*(*gContestResources).gfxState.at(contestant)).updatingAppealHearts() == 0 {
                ContestClearGeneralTextWindow();
                gTasks[taskId].data[0] = APPEALSTATE_SLIDE_APPLAUSE_OUT;
            }
            return;
        }
        APPEALSTATE_PRINT_CROWD_WATCHES_MSG => {
            ContestClearGeneralTextWindow();
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gContestMons[(*(*gContestResources).excitement).freezer()]
                    .nickname
                    .as_mut_ptr(),
            );
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gContestMons[contestant].nickname.as_mut_ptr(),
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gMoveNames[(*(*gContestResources).status.at(contestant)).currMove]
                    .as_ptr()
                    .cast_mut(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_CrowdContinuesToWatchMon.as_ptr().cast_mut(),
            );
            Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
            gTasks[taskId].data[0] = APPEALSTATE_PRINT_MON_MOVE_IGNORED_MSG;
            return;
        }
        APPEALSTATE_PRINT_MON_MOVE_IGNORED_MSG => {
            if Contest_RunTextPrinters() == 0 {
                ContestClearGeneralTextWindow();
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_MonsMoveIsIgnored.as_ptr().cast_mut(),
                );
                Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
                gTasks[taskId].data[0] = APPEALSTATE_WAIT_MON_MOVE_IGNORED_MSG;
            }
            return;
        }
        APPEALSTATE_WAIT_MON_MOVE_IGNORED_MSG => {
            if Contest_RunTextPrinters() == 0 {
                ContestClearGeneralTextWindow();
                gTasks[taskId].data[0] = APPEALSTATE_SLIDE_APPLAUSE_OUT;
            }
            return;
        }
        APPEALSTATE_PRINT_TOO_NERVOUS_MSG => {
            if (*(*gContestResources).status.at(contestant)).hasJudgesAttention() != 0 {
                (*(*gContestResources).status.at(contestant)).set_hasJudgesAttention(FALSE);
            }
            StartStopFlashJudgeAttentionEye(contestant);
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gContestMons[contestant].nickname.as_mut_ptr(),
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gMoveNames[(*(*gContestResources).status.at(contestant)).currMove]
                    .as_ptr()
                    .cast_mut(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_MonWasTooNervousToMove.as_ptr().cast_mut(),
            );
            Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
            gTasks[taskId].data[0] = APPEALSTATE_WAIT_TOO_NERVOUS_MSG;
            return;
        }
        APPEALSTATE_WAIT_TOO_NERVOUS_MSG => {
            if Contest_RunTextPrinters() == 0 {
                gTasks[taskId].data[0] = APPEALSTATE_SLIDE_APPLAUSE_OUT;
            }
            return;
        }
        APPEALSTATE_SLIDE_APPLAUSE_OUT => {
            SlideApplauseMeterOut();
            gTasks[taskId].data[0] = APPEALSTATE_WAIT_SLIDE_APPLAUSE;
            return;
        }
        APPEALSTATE_WAIT_SLIDE_APPLAUSE => {
            if (*(*gContestResources).contest).applauseMeterIsMoving() == 0 {
                if (*(*gContestResources).contest).applauseLevel > 4 {
                    (*(*gContestResources).contest).applauseLevel = 0;
                    UpdateApplauseMeter();
                }
                gTasks[taskId].data[0] = APPEALSTATE_SLIDE_MON_OUT;
            }
            return;
        }
        APPEALSTATE_SLIDE_MON_OUT => {
            spriteId = gTasks[taskId].data[2] as u8;
            gSprites[spriteId].callback = Some(SpriteCB_MonSlideOut);
            gTasks[taskId].data[0] = APPEALSTATE_FREE_MON_SPRITE;
            return;
        }
        APPEALSTATE_FREE_MON_SPRITE => {
            spriteId = gTasks[taskId].data[2] as u8;
            if gSprites[spriteId].invisible() != 0 {
                FreeSpriteOamMatrix(&raw mut gSprites[spriteId]);
                DestroySprite(&raw mut gSprites[spriteId]);
                gTasks[taskId].data[0] = APPEALSTATE_START_TURN_END_DELAY;
            }
            return;
        }
        APPEALSTATE_START_TURN_END_DELAY => {
            gTasks[taskId].data[10] = 0;
            gTasks[taskId].data[0] = APPEALSTATE_TURN_END_DELAY;
            return;
        }
        APPEALSTATE_PRINT_SKIP_TURN_MSG => {
            ContestClearGeneralTextWindow();
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gContestMons[contestant].nickname.as_mut_ptr(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_MonWasWatchingOthers.as_ptr().cast_mut(),
            );
            Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
            gTasks[taskId].data[0] = APPEALSTATE_WAIT_SKIP_TURN_MSG;
            return;
        }
        APPEALSTATE_WAIT_SKIP_TURN_MSG => {
            if Contest_RunTextPrinters() == 0 {
                gTasks[taskId].data[0] = APPEALSTATE_TURN_END_DELAY;
            }
            return;
        }
        APPEALSTATE_TURN_END_DELAY => {
            if ({
                gTasks[taskId].data[10] += 1;
                gTasks[taskId].data[10]
            }) > 29
            {
                gTasks[taskId].data[10] = 0;
                gTasks[taskId].data[0] = APPEALSTATE_START_NEXT_TURN;
            }
            return;
        }
        APPEALSTATE_START_NEXT_TURN => {
            if ({
                (*(*gContestResources).contest).turnNumber += 1;
                (*(*gContestResources).contest).turnNumber
            }) == CONTESTANT_COUNT as u8
            {
                gTasks[taskId].data[0] = 0;
                gTasks[taskId].data[1] = 0;
                gTasks[taskId].data[2] = 0;
                gTasks[taskId].func = Some(Task_FinishRoundOfAppeals);
            } else {
                gTasks[taskId].data[0] = APPEALSTATE_START_TURN;
            }
            return;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_EndWaitForLink(taskId: u8) {
    (*(*gContestResources).contest).set_waitForLink(FALSE as u16);
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn SpriteCB_MonSlideIn(sprite: *mut Sprite) {
    if (*sprite).x2 != 0 {
        (*sprite).x2 -= 2;
    } else {
        if ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) == 31
        {
            (*sprite).data[0] = 0;
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonSlideOut(sprite: *mut Sprite) {
    (*sprite).x2 -= 6;
    if ((*sprite).x as i32 + (*sprite).x2 as i32) < -32 {
        (*sprite).callback = Some(SpriteCallbackDummy);
        (*sprite).set_invisible(TRUE as u16);
    }
}
pub(crate) unsafe extern "C" fn Task_FinishRoundOfAppeals(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
                let mut taskId2: u8 = 0;
                (*(*gContestResources).contest).set_waitForLink(TRUE as u16);
                if IsPlayerLinkLeader() != 0 {
                    RankContestants();
                    SetAttentionLevels();
                }
                taskId2 = CreateTask(Some(Task_LinkContest_CommunicateAppealsState), 0);
                SetTaskFuncWithFollowupFunc(
                    taskId2,
                    Some(Task_LinkContest_CommunicateAppealsState),
                    Some(Task_EndWaitForLink),
                );
                ContestPrintLinkStandby();
                gTasks[taskId].data[0] = 1;
            } else {
                RankContestants();
                SetAttentionLevels();
                gTasks[taskId].data[0] = 2;
            }
        }
        1 => {
            if (*(*gContestResources).contest).waitForLink() == 0 {
                gTasks[taskId].data[0] = 2;
            }
        }
        2 => {
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].func = Some(Task_ReadyUpdateHeartSliders);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ReadyUpdateHeartSliders(taskId: u8) {
    ShowHideNextTurnGfx(FALSE);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = 0;
    gTasks[taskId].func = Some(Task_UpdateHeartSliders);
}
pub(crate) unsafe extern "C" fn Task_UpdateHeartSliders(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            if ({
                gTasks[taskId].data[1] += 1;
                gTasks[taskId].data[1]
            }) > 20
            {
                AnimateSliderHearts(SLIDER_HEART_ANIM_APPEAR);
                gTasks[taskId].data[1] = 0;
                gTasks[taskId].data[0] += 1;
            }
        }
        1 => {
            if (*(*gContestResources).contest).sliderHeartsAnimating() == 0 {
                if ({
                    gTasks[taskId].data[1] += 1;
                    gTasks[taskId].data[1]
                }) > 20
                {
                    gTasks[taskId].data[1] = 0;
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        2 => {
            UpdateHeartSliders();
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].data[1] = 0;
            gTasks[taskId].func = Some(Task_WaitForHeartSliders);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForHeartSliders(taskId: u8) {
    if SlidersDoneUpdating() != 0 {
        gTasks[taskId].func = Some(Task_RestorePlttBufferUnfaded);
    }
}
pub(crate) unsafe extern "C" fn Task_RestorePlttBufferUnfaded(taskId: u8) {
    {
        let mut _src: *mut c_void = (*(gHeap.as_mut_ptr().at(106500) as *mut ContestTempSave))
            .cachedPlttBufferUnfaded
            .as_mut_ptr() as *mut c_void;
        let mut _dest: *mut c_void = gPlttBufferUnfaded.as_mut_ptr() as *mut c_void;
        let mut _size: u32 = PLTT_SIZE;
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, _src as usize as u32);
                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x84000000 | _size / 4);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = 2;
    gTasks[taskId].func = Some(Task_WaitPrintRoundResult);
}
pub(crate) unsafe extern "C" fn Task_WaitPrintRoundResult(taskId: u8) {
    if ({
        gTasks[taskId].data[0] += 1;
        gTasks[taskId].data[0]
    }) > 2
    {
        gTasks[taskId].data[0] = 0;
        if ({
            gTasks[taskId].data[1] -= 1;
            gTasks[taskId].data[1]
        }) == 0
        {
            gTasks[taskId].func = Some(Task_PrintRoundResultText);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintRoundResultText(taskId: u8) {
    if gTasks[taskId].data[0] == 0 {
        let mut attention: u8 =
            (*(*gContestResources).status.at(gContestPlayerMonIndex)).attentionLevel;
        ContestClearGeneralTextWindow();
        StringCopy(
            gStringVar1.as_mut_ptr(),
            gContestMons[gContestPlayerMonIndex].nickname.as_mut_ptr(),
        );
        StringExpandPlaceholders(gStringVar4.as_mut_ptr(), sRoundResultTexts[attention]);
        Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
        gTasks[taskId].data[0] += 1;
    } else {
        if Contest_RunTextPrinters() == 0 {
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].func = Some(Task_ReUpdateHeartSliders);
            ContestDebugDoPrint();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReUpdateHeartSliders(taskId: u8) {
    if ({
        let t1 = gTasks[taskId].data[0];
        gTasks[taskId].data[0] += 1;
        t1
    }) > 29
    {
        gTasks[taskId].data[0] = 0;
        UpdateHeartSliders();
        gTasks[taskId].func = Some(Task_WaitForHeartSlidersAgain);
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForHeartSlidersAgain(taskId: u8) {
    if SlidersDoneUpdating() != 0 {
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].func = Some(Task_DropCurtainAtRoundEnd);
    }
}
pub(crate) unsafe extern "C" fn Task_DropCurtainAtRoundEnd(taskId: u8) {
    SetBgForCurtainDrop();
    gTasks[taskId].func = Some(Task_StartDropCurtainAtRoundEnd);
}
pub(crate) unsafe extern "C" fn Task_UpdateContestantBoxOrder(taskId: u8) {
    UpdateContestantBoxOrder();
    gTasks[taskId].func = Some(Task_TryStartNextRoundOfAppeals);
}
pub(crate) unsafe extern "C" fn Task_TryStartNextRoundOfAppeals(taskId: u8) {
    let mut sp0: u16 = 0;
    volatile_write(&raw mut sp0, GetGpuReg(REG_OFFSET_BG0CNT));
    let mut sp2: u16 = 0;
    volatile_write(&raw mut sp2, GetGpuReg(REG_OFFSET_BG2CNT));
    (*(&raw mut sp0 as *mut BgCnt)).set_priority(0);
    (*(&raw mut sp2 as *mut BgCnt)).set_priority(0);
    SetGpuReg(REG_OFFSET_BG0CNT, (&raw mut sp0).read_volatile());
    SetGpuReg(REG_OFFSET_BG2CNT, (&raw mut sp2).read_volatile());
    (*(*gContestResources).contest).appealNumber += 1;
    if (*(*gContestResources).contest).appealNumber == CONTEST_NUM_APPEALS as u8 {
        gTasks[taskId].func = Some(Task_EndAppeals);
    } else {
        SlideApplauseMeterIn();
        gTasks[taskId].func = Some(Task_StartNewRoundOfAppeals);
    }
}
pub(crate) unsafe extern "C" fn Task_StartNewRoundOfAppeals(taskId: u8) {
    if (*(*gContestResources).contest).applauseMeterIsMoving() == 0 {
        gTasks[taskId].func = Some(Task_DisplayAppealNumberText);
    }
}
pub(crate) unsafe extern "C" fn Task_EndAppeals(taskId: u8) {
    let mut i: i32 = 0;
    gBattle_BG0_Y = 0;
    gBattle_BG2_Y = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        gContestMonAppealPointTotals[i] = (*(*gContestResources).status.at(i)).pointTotal;
        i += 1;
    }
    CalculateFinalScores();
    ContestClearGeneralTextWindow();
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK == 0 {
        BravoTrainerPokemonProfile_BeforeInterview1(
            (*(*gContestResources).status.at(gContestPlayerMonIndex)).prevMove,
        );
    } else {
        CalculateContestLiveUpdateData();
        SetConestLiveUpdateTVData();
        ContestDebugPrintBitStrings();
    }
    gContestRngValue = gRngValue;
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_AllOutOfAppealTime.as_ptr().cast_mut(),
    );
    Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
    gTasks[taskId].data[2] = 0;
    gTasks[taskId].func = Some(Task_WaitForOutOfTimeMsg);
}
pub(crate) unsafe extern "C" fn Task_WaitForOutOfTimeMsg(taskId: u8) {
    if Contest_RunTextPrinters() == 0 {
        SetBgForCurtainDrop();
        gBattle_BG1_X = 0;
        gBattle_BG1_Y = DISPLAY_HEIGHT;
        PlaySE12WithPanning(SE_CONTEST_CURTAIN_FALL, 0);
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].func = Some(Task_DropCurtainAtAppealsEnd);
    }
}
pub(crate) unsafe extern "C" fn Task_DropCurtainAtAppealsEnd(taskId: u8) {
    gBattle_BG1_Y -= 7;
    if (gBattle_BG1_Y as i16) < 0 {
        gBattle_BG1_Y = 0;
    }
    if gBattle_BG1_Y == 0 {
        gTasks[taskId].func = Some(Task_TryCommunicateFinalStandings);
        gTasks[taskId].data[0] = 0;
    }
}
pub(crate) unsafe extern "C" fn Task_TryCommunicateFinalStandings(taskId: u8) {
    if ({
        let t1 = gTasks[taskId].data[0];
        gTasks[taskId].data[0] += 1;
        t1
    }) >= 50
    {
        gTasks[taskId].data[0] = 0;
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
            gTasks[taskId].func = Some(Task_CommunicateFinalStandings);
        } else {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            gTasks[taskId].func = Some(Task_ContestReturnToField);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CommunicateFinalStandings(taskId: u8) {
    let mut taskId2: u8 = CreateTask(Some(Task_LinkContest_CommunicateFinalStandings), 0);
    SetTaskFuncWithFollowupFunc(
        taskId2,
        Some(Task_LinkContest_CommunicateFinalStandings),
        Some(Task_EndCommunicateFinalStandings),
    );
    gTasks[taskId].func = Some(TaskDummy1);
    ContestPrintLinkStandby();
    SetBottomSliderHeartsInvisibility(FALSE);
}
pub(crate) unsafe extern "C" fn Task_EndCommunicateFinalStandings(taskId: u8) {
    DestroyTask(taskId);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    gTasks[(*(*gContestResources).contest).mainTaskId].func = Some(Task_ContestReturnToField);
}
pub(crate) unsafe extern "C" fn Task_ContestReturnToField(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DestroyTask(taskId);
        gFieldCallback = Some(FieldCB_ContestReturnToField);
        FreeAllWindowBuffers();
        FreeContestResources();
        FreeMonSpritesGfx();
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
pub(crate) unsafe extern "C" fn FieldCB_ContestReturnToField() {
    UnlockPlayerFieldControls();
    ScriptContext_Enable();
}
pub(crate) unsafe extern "C" fn TryPutPlayerLast() {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK == 0 {
        gContestPlayerMonIndex = 3;
    }
}
pub(crate) unsafe extern "C" fn IsPlayerLinkLeader() -> u8 {
    if gContestPlayerMonIndex == gContestLinkLeaderIndex {
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateContestMonFromParty(partyIndex: u8) {
    let mut name: CArray<u8, 20> = zeroed();
    let mut heldItem: u16 = 0;
    let mut cool: i16 = 0;
    let mut beauty: i16 = 0;
    let mut cute: i16 = 0;
    let mut smart: i16 = 0;
    let mut tough: i16 = 0;
    StringCopy(name.as_mut_ptr(), (*gSaveBlock2Ptr).playerName.as_mut_ptr());
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        StripPlayerNameForLinkContest(name.as_mut_ptr());
    }
    memcpy(
        gContestMons[gContestPlayerMonIndex]
            .trainerName
            .as_mut_ptr(),
        name.as_mut_ptr(),
        8,
    );
    if (*gSaveBlock2Ptr).playerGender == MALE {
        gContestMons[gContestPlayerMonIndex].trainerGfxId = OBJ_EVENT_GFX_LINK_BRENDAN;
    } else {
        gContestMons[gContestPlayerMonIndex].trainerGfxId = OBJ_EVENT_GFX_LINK_MAY;
    }
    gContestMons[gContestPlayerMonIndex].aiFlags = 0;
    gContestMons[gContestPlayerMonIndex].highestRank = 0;
    gContestMons[gContestPlayerMonIndex].species =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_SPECIES) as u16;
    GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_NICKNAME,
        name.as_mut_ptr(),
    );
    StringGet_Nickname(name.as_mut_ptr());
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        StripMonNameForLinkContest(
            name.as_mut_ptr(),
            GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_LANGUAGE) as i32,
        );
    }
    memcpy(
        gContestMons[gContestPlayerMonIndex].nickname.as_mut_ptr(),
        name.as_mut_ptr(),
        11,
    );
    StringCopy(
        gContestMons[gContestPlayerMonIndex].nickname.as_mut_ptr(),
        name.as_mut_ptr(),
    );
    gContestMons[gContestPlayerMonIndex].cool =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_COOL) as u8;
    gContestMons[gContestPlayerMonIndex].beauty =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_BEAUTY) as u8;
    gContestMons[gContestPlayerMonIndex].cute =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_CUTE) as u8;
    gContestMons[gContestPlayerMonIndex].smart =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_SMART) as u8;
    gContestMons[gContestPlayerMonIndex].tough =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_TOUGH) as u8;
    gContestMons[gContestPlayerMonIndex].sheen =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_SHEEN) as u8;
    gContestMons[gContestPlayerMonIndex].moves[0] =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_MOVE1) as u16;
    gContestMons[gContestPlayerMonIndex].moves[1] =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_MOVE2) as u16;
    gContestMons[gContestPlayerMonIndex].moves[2] =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_MOVE3) as u16;
    gContestMons[gContestPlayerMonIndex].moves[3] =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_MOVE4) as u16;
    gContestMons[gContestPlayerMonIndex].personality =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_PERSONALITY);
    gContestMons[gContestPlayerMonIndex].otId =
        GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_OT_ID);
    heldItem = GetMonData2(&raw mut gPlayerParty[partyIndex], MON_DATA_HELD_ITEM) as u16;
    cool = gContestMons[gContestPlayerMonIndex].cool as i16;
    beauty = gContestMons[gContestPlayerMonIndex].beauty as i16;
    cute = gContestMons[gContestPlayerMonIndex].cute as i16;
    smart = gContestMons[gContestPlayerMonIndex].smart as i16;
    tough = gContestMons[gContestPlayerMonIndex].tough as i16;
    if heldItem == ITEM_RED_SCARF {
        cool += 20;
    } else if heldItem == ITEM_BLUE_SCARF {
        beauty += 20;
    } else if heldItem == ITEM_PINK_SCARF {
        cute += 20;
    } else if heldItem == ITEM_GREEN_SCARF {
        smart += 20;
    } else if heldItem == ITEM_YELLOW_SCARF {
        tough += 20;
    }
    if cool > 255 {
        cool = 255;
    }
    if beauty > 255 {
        beauty = 255;
    }
    if cute > 255 {
        cute = 255;
    }
    if smart > 255 {
        smart = 255;
    }
    if tough > 255 {
        tough = 255;
    }
    gContestMons[gContestPlayerMonIndex].cool = cool as u8;
    gContestMons[gContestPlayerMonIndex].beauty = beauty as u8;
    gContestMons[gContestPlayerMonIndex].cute = cute as u8;
    gContestMons[gContestPlayerMonIndex].smart = smart as u8;
    gContestMons[gContestPlayerMonIndex].tough = tough as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContestants(contestType: u8, rank: u8) {
    let mut i: i32 = 0;
    let mut opponentsCount: u8 = 0;
    let mut opponents: CArray<u8, 100> = zeroed();
    let mut allowPostgameContestants: u8 = FALSE;
    let mut filter: *mut u8 = null_mut();
    TryPutPlayerLast();
    if FlagGet(FLAG_SYS_GAME_CLEAR) != 0
        && gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK == 0
    {
        allowPostgameContestants = TRUE;
    }
    filter = gPostgameContestOpponentFilter.as_ptr().cast_mut();
    i = 0;
    while i < 96 {
        'l1: {
            if rank == gContestOpponents[i].whichRank() {
                if allowPostgameContestants == TRUE {
                    if *filter.at(i) == CONTEST_FILTER_NO_POSTGAME {
                        break 'l1;
                    }
                } else {
                    if *filter.at(i) == CONTEST_FILTER_ONLY_POSTGAME {
                        break 'l1;
                    }
                }
                if contestType == CONTEST_CATEGORY_COOL && gContestOpponents[i].aiPool_Cool() != 0 {
                    opponents[{
                        let t1 = opponentsCount;
                        opponentsCount += 1;
                        t1
                    }] = i as u8;
                } else if contestType == CONTEST_CATEGORY_BEAUTY
                    && gContestOpponents[i].aiPool_Beauty() != 0
                {
                    opponents[{
                        let t2 = opponentsCount;
                        opponentsCount += 1;
                        t2
                    }] = i as u8;
                } else if contestType == CONTEST_CATEGORY_CUTE
                    && gContestOpponents[i].aiPool_Cute() != 0
                {
                    opponents[{
                        let t3 = opponentsCount;
                        opponentsCount += 1;
                        t3
                    }] = i as u8;
                } else if contestType == CONTEST_CATEGORY_SMART
                    && gContestOpponents[i].aiPool_Smart() != 0
                {
                    opponents[{
                        let t4 = opponentsCount;
                        opponentsCount += 1;
                        t4
                    }] = i as u8;
                } else if contestType == CONTEST_CATEGORY_TOUGH
                    && gContestOpponents[i].aiPool_Tough() != 0
                {
                    opponents[{
                        let t5 = opponentsCount;
                        opponentsCount += 1;
                        t5
                    }] = i as u8;
                }
            }
        }
        i += 1;
    }
    opponents[opponentsCount] = CONTESTANT_NONE;
    i = 0;
    while i < 3 {
        let mut rnd: u16 = rem_i32(Random() as i32, opponentsCount as i32) as u16;
        let mut j: i32 = 0;
        gContestMons[i] = gContestOpponents[opponents[rnd]];
        j = rnd as i32;
        while opponents[j] != CONTESTANT_NONE {
            opponents[j] = opponents[j + 1];
            j += 1;
        }
        opponentsCount -= 1;
        i += 1;
    }
    CreateContestMonFromParty(gContestMonPartyIndex);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLinkAIContestants(contestType: u8, rank: u8, isPostgame: u32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut opponentsCount: u8 = 0;
    let mut opponents: CArray<u8, 100> = zeroed();
    if gNumLinkContestPlayers == CONTESTANT_COUNT as u8 {
        return;
    }
    i = 0;
    while i < 96 {
        'l1: {
            if rank != gContestOpponents[i].whichRank() {
                break 'l1;
            }
            if isPostgame == TRUE as u32 {
                if gPostgameContestOpponentFilter[i] == CONTEST_FILTER_NO_POSTGAME {
                    break 'l1;
                }
            } else {
                if gPostgameContestOpponentFilter[i] == CONTEST_FILTER_ONLY_POSTGAME {
                    break 'l1;
                }
            }
            if contestType == CONTEST_CATEGORY_COOL && gContestOpponents[i].aiPool_Cool() != 0
                || contestType == CONTEST_CATEGORY_BEAUTY
                    && gContestOpponents[i].aiPool_Beauty() != 0
                || contestType == CONTEST_CATEGORY_CUTE && gContestOpponents[i].aiPool_Cute() != 0
                || contestType == CONTEST_CATEGORY_SMART && gContestOpponents[i].aiPool_Smart() != 0
                || contestType == CONTEST_CATEGORY_TOUGH && gContestOpponents[i].aiPool_Tough() != 0
            {
                opponents[{
                    let t1 = opponentsCount;
                    opponentsCount += 1;
                    t1
                }] = i as u8;
            }
        }
        i += 1;
    }
    opponents[opponentsCount] = CONTESTANT_NONE;
    i = 0;
    while i < CONTESTANT_COUNT - gNumLinkContestPlayers as i32 {
        let mut rnd: u16 = rem_i32(GetContestRand() as i32, opponentsCount as i32) as u16;
        gContestMons[gNumLinkContestPlayers as i32 + i] = gContestOpponents[opponents[rnd]];
        StripPlayerNameForLinkContest(
            gContestMons[gNumLinkContestPlayers as i32 + i]
                .trainerName
                .as_mut_ptr(),
        );
        StripMonNameForLinkContest(
            gContestMons[gNumLinkContestPlayers as i32 + i]
                .nickname
                .as_mut_ptr(),
            GAME_LANGUAGE as i32,
        );
        j = rnd as i32;
        while opponents[j] != CONTESTANT_NONE {
            opponents[j] = opponents[j + 1];
            j += 1;
        }
        opponentsCount -= 1;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestEntryEligibility(pkmn: *mut Pokemon) -> u8 {
    let mut ribbon: u8 = 0;
    let mut eligibility: u8 = 0;
    if GetMonData2(pkmn, MON_DATA_IS_EGG) != 0 {
        return CANT_ENTER_CONTEST_EGG;
    }
    if GetMonData2(pkmn, MON_DATA_HP) == 0 {
        return CANT_ENTER_CONTEST_FAINTED;
    }
    match gSpecialVar_ContestCategory {
        0 => {
            ribbon = GetMonData2(pkmn, MON_DATA_COOL_RIBBON) as u8;
        }
        1 => {
            ribbon = GetMonData2(pkmn, MON_DATA_BEAUTY_RIBBON) as u8;
        }
        2 => {
            ribbon = GetMonData2(pkmn, MON_DATA_CUTE_RIBBON) as u8;
        }
        3 => {
            ribbon = GetMonData2(pkmn, MON_DATA_SMART_RIBBON) as u8;
        }
        4 => {
            ribbon = GetMonData2(pkmn, MON_DATA_TOUGH_RIBBON) as u8;
        }
        _ => {
            return CANT_ENTER_CONTEST;
        }
    }
    if ribbon as u16 > gSpecialVar_ContestRank {
        eligibility = CAN_ENTER_CONTEST_HIGH_RANK;
    } else if ribbon as u16 >= gSpecialVar_ContestRank {
        eligibility = CAN_ENTER_CONTEST_EQUAL_RANK;
    } else {
        eligibility = CANT_ENTER_CONTEST;
    }
    return eligibility;
}
pub(crate) unsafe extern "C" fn DrawContestantWindowText() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        FillWindowPixelBuffer(gContestantTurnOrder[i], 0);
        PrintContestantTrainerName(i as u8);
        PrintContestantMonName(i as u8);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Contest_CopyStringWithColor(string: *mut u8, color: u8) -> *mut u8 {
    let mut ptr: *mut u8 = StringCopy(
        gDisplayedStringBattle.as_mut_ptr(),
        gText_ColorTransparent.as_ptr().cast_mut(),
    );
    *ptr.at(-1) = color;
    ptr = StringCopy(ptr, string);
    return ptr;
}
pub(crate) unsafe extern "C" fn PrintContestantTrainerName(contestant: u8) {
    PrintContestantTrainerNameWithColor(contestant, contestant + CONTESTANT_TEXT_COLOR_START);
}
pub(crate) unsafe extern "C" fn PrintContestantTrainerNameWithColor(contestant: u8, color: u8) {
    let mut buffer: CArray<u8, 32> = zeroed();
    let mut offset: i32 = 0;
    StringCopy(buffer.as_mut_ptr(), gText_Slash.as_ptr().cast_mut());
    StringAppend(
        buffer.as_mut_ptr(),
        gContestMons[contestant].trainerName.as_mut_ptr(),
    );
    Contest_CopyStringWithColor(buffer.as_mut_ptr(), color);
    offset = GetStringRightAlignXOffset(
        FONT_NARROW as i32,
        gDisplayedStringBattle.as_mut_ptr(),
        0x60,
    );
    if offset > 55 {
        offset = 55;
    }
    Contest_PrintTextToBg0WindowAt(
        gContestantTurnOrder[contestant] as u32,
        gDisplayedStringBattle.as_mut_ptr(),
        offset,
        1,
        FONT_NARROW as i32,
    );
}
pub(crate) unsafe extern "C" fn PrintContestantMonName(contestant: u8) {
    PrintContestantMonNameWithColor(contestant, contestant + CONTESTANT_TEXT_COLOR_START);
}
pub(crate) unsafe extern "C" fn PrintContestantMonNameWithColor(contestant: u8, color: u8) {
    Contest_CopyStringWithColor(gContestMons[contestant].nickname.as_mut_ptr(), color);
    Contest_PrintTextToBg0WindowAt(
        gContestantTurnOrder[contestant] as u32,
        gDisplayedStringBattle.as_mut_ptr(),
        5,
        1,
        FONT_NARROW as i32,
    );
}
pub(crate) unsafe extern "C" fn CalculateContestantRound1Points(
    who: u8,
    contestCategory: u8,
) -> u16 {
    let mut statMain: u8 = 0;
    let mut statSub1: u8 = 0;
    let mut statSub2: u8 = 0;
    match contestCategory {
        CONTEST_CATEGORY_COOL => {
            statMain = gContestMons[who].cool;
            statSub1 = gContestMons[who].tough;
            statSub2 = gContestMons[who].beauty;
        }
        CONTEST_CATEGORY_BEAUTY => {
            statMain = gContestMons[who].beauty;
            statSub1 = gContestMons[who].cool;
            statSub2 = gContestMons[who].cute;
        }
        CONTEST_CATEGORY_CUTE => {
            statMain = gContestMons[who].cute;
            statSub1 = gContestMons[who].beauty;
            statSub2 = gContestMons[who].smart;
        }
        CONTEST_CATEGORY_SMART => {
            statMain = gContestMons[who].smart;
            statSub1 = gContestMons[who].cute;
            statSub2 = gContestMons[who].tough;
        }
        _ => {
            statMain = gContestMons[who].tough;
            statSub1 = gContestMons[who].smart;
            statSub2 = gContestMons[who].cool;
        }
    }
    return statMain as u16
        + ((statSub1 as i32 + statSub2 as i32 + gContestMons[who].sheen as i32) / 2) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculateRound1Points(contestCategory: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        gContestMonRound1Points[i] =
            CalculateContestantRound1Points(i as u8, contestCategory) as i16;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateJudgeSprite() -> u8 {
    let mut spriteId: u8 = 0;
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_Judge).cast_mut());
    LoadCompressedPalette(gContest2Pal.as_ptr().cast_mut(), 272, 32);
    spriteId = CreateSprite((&raw const *sSpriteTemplate_Judge).cast_mut(), 112, 36, 30);
    gSprites[spriteId].oam.set_paletteNum(1);
    gSprites[spriteId].callback = Some(SpriteCallbackDummy);
    return spriteId;
}
pub(crate) unsafe extern "C" fn CreateJudgeSpeechBubbleSprite() -> u8 {
    let mut spriteId: u8 = 0;
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_JudgeSymbols).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePalette_JudgeSymbols).cast_mut());
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_JudgeSpeechBubble).cast_mut(),
        96,
        10,
        29,
    );
    gSprites[spriteId].set_invisible(TRUE as u16);
    gSprites[spriteId].data[0] = gSprites[spriteId].oam.tileNum() as i16;
    return spriteId;
}
pub(crate) unsafe extern "C" fn CreateContestantSprite(
    mut species: u16,
    otId: u32,
    personality: u32,
    index: u32,
) -> u8 {
    let mut spriteId: u8 = 0;
    species = SanitizeSpecies(species);
    if index == gContestPlayerMonIndex as u32 {
        HandleLoadSpecialPokePic_2(
            (&raw const gMonBackPicTable[species]).cast_mut(),
            (*gMonSpritesGfxPtr).sprites.ptr[0],
            species as i32,
            personality,
        );
    } else {
        HandleLoadSpecialPokePic_DontHandleDeoxys(
            (&raw const gMonBackPicTable[species]).cast_mut(),
            (*gMonSpritesGfxPtr).sprites.ptr[0],
            species as i32,
            personality,
        );
    }
    LoadCompressedPalette(
        GetMonSpritePalFromSpeciesAndPersonality(species, otId, personality),
        288,
        32,
    );
    SetMultiuseSpriteTemplateToPokemon(species, B_POSITION_PLAYER_LEFT);
    spriteId = CreateSprite(
        &raw mut gMultiuseSpriteTemplate,
        0x70,
        GetBattlerSpriteFinal_Y(2, species, FALSE) as i16,
        30,
    );
    gSprites[spriteId].oam.set_paletteNum(2);
    gSprites[spriteId].oam.set_priority(2);
    gSprites[spriteId].subpriority = GetBattlerSpriteSubpriority(2);
    gSprites[spriteId].callback = Some(SpriteCallbackDummy);
    gSprites[spriteId].data[0] = gSprites[spriteId].oam.paletteNum() as i16;
    gSprites[spriteId].data[2] = species as i16;
    if IsSpeciesNotUnown(species) != 0 {
        gSprites[spriteId].affineAnims = gAffineAnims_BattleSpriteContest.as_ptr().cast_mut();
    } else {
        gSprites[spriteId].affineAnims = gAffineAnims_BattleSpriteOpponentSide.as_ptr().cast_mut();
    }
    StartSpriteAffineAnim(&raw mut gSprites[spriteId], BATTLER_AFFINE_NORMAL);
    return spriteId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSpeciesNotUnown(species: u16) -> u8 {
    if species == SPECIES_UNOWN {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SwapMoveDescAndContestTilemaps() {
    CpuSet(
        (*gContestResources).contestBgTilemaps[0] as *mut c_void,
        (*gContestResources).contestBgTilemaps[0].at(1280) as *mut c_void,
        320,
    );
    CpuSet(
        (*gContestResources).contestBgTilemaps[2] as *mut c_void,
        (*gContestResources).contestBgTilemaps[2].at(1280) as *mut c_void,
        320,
    );
}
pub(crate) unsafe extern "C" fn GetMoveEffectSymbolTileOffset(r#move: u16, contestant: u8) -> u16 {
    let mut offset: u16 = 0;
    match gContestEffects[gContestMoves[r#move].effect].effectType {
        CONTEST_EFFECT_TYPE_APPEAL
        | CONTEST_EFFECT_TYPE_AVOID_STARTLE
        | CONTEST_EFFECT_TYPE_UNKNOWN => {
            offset = 0x9082;
        }
        CONTEST_EFFECT_TYPE_STARTLE_MON | CONTEST_EFFECT_TYPE_STARTLE_MONS => {
            offset = 0x9088;
        }
        _ => {
            offset = 0x9086;
        }
    }
    offset += 0x9000 + ((contestant as u16) << 12);
    return offset;
}
pub(crate) unsafe extern "C" fn PrintContestMoveDescription(r#move: u16) {
    let mut category: u8 = 0;
    let mut categoryTile: u16 = 0;
    let mut numHearts: u8 = 0;
    category = gContestMoves[r#move].contestCategory();
    if category == CONTEST_CATEGORY_COOL {
        categoryTile = 0x4040;
    } else if category == CONTEST_CATEGORY_BEAUTY {
        categoryTile = 0x4045;
    } else if category == CONTEST_CATEGORY_CUTE {
        categoryTile = 0x404A;
    } else if category == CONTEST_CATEGORY_SMART {
        categoryTile = 0x406A;
    } else {
        categoryTile = 0x408A;
    }
    ContestBG_FillBoxWithIncrementingTile(0, categoryTile, 0x0b, 0x1f, 0x05, 0x01, 0x11, 0x01);
    ContestBG_FillBoxWithIncrementingTile(
        0,
        categoryTile + 0x10,
        0x0b,
        0x20,
        0x05,
        0x01,
        0x11,
        0x01,
    );
    if gContestEffects[gContestMoves[r#move].effect].appeal == 0xFF {
        numHearts = 0;
    } else {
        numHearts = (gContestEffects[gContestMoves[r#move].effect].appeal as i32 / 10) as u8;
    }
    if numHearts > MAX_CONTEST_MOVE_HEARTS {
        numHearts = MAX_CONTEST_MOVE_HEARTS;
    }
    ContestBG_FillBoxWithTile(
        0,
        TILE_EMPTY_APPEAL_HEART,
        0x15,
        0x1f,
        MAX_CONTEST_MOVE_HEARTS,
        0x01,
        0x11,
    );
    ContestBG_FillBoxWithTile(
        0,
        TILE_FILLED_APPEAL_HEART,
        0x15,
        0x1f,
        numHearts,
        0x01,
        0x11,
    );
    if gContestEffects[gContestMoves[r#move].effect].jam == 0xFF {
        numHearts = 0;
    } else {
        numHearts = (gContestEffects[gContestMoves[r#move].effect].jam as i32 / 10) as u8;
    }
    if numHearts > MAX_CONTEST_MOVE_HEARTS {
        numHearts = MAX_CONTEST_MOVE_HEARTS;
    }
    ContestBG_FillBoxWithTile(
        0,
        TILE_EMPTY_JAM_HEART,
        0x15,
        0x20,
        MAX_CONTEST_MOVE_HEARTS,
        0x01,
        0x11,
    );
    ContestBG_FillBoxWithTile(0, TILE_FILLED_JAM_HEART, 0x15, 0x20, numHearts, 0x01, 0x11);
    FillWindowPixelBuffer(WIN_MOVE_DESCRIPTION, 0);
    Contest_PrintTextToBg0WindowStd(
        WIN_MOVE_DESCRIPTION as u32,
        gContestEffectDescriptionPointers[gContestMoves[r#move].effect],
    );
    Contest_PrintTextToBg0WindowStd(WIN_SLASH, gText_Slash.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn DrawMoveEffectSymbol(r#move: u16, contestant: u8) {
    let mut contestantOffset: u8 = gContestantTurnOrder[contestant] * 5 + 2;
    if Contest_IsMonsTurnDisabled(contestant) == 0 && r#move != MOVE_NONE {
        let mut tile: u16 = GetMoveEffectSymbolTileOffset(r#move, contestant);
        ContestBG_FillBoxWithIncrementingTile(0, tile, 20, contestantOffset, 2, 1, 17, 1);
        ContestBG_FillBoxWithIncrementingTile(0, tile + 16, 20, contestantOffset + 1, 2, 1, 17, 1);
    } else {
        ContestBG_FillBoxWithTile(0, 0, 20, contestantOffset, 2, 2, 17);
    }
}
pub(crate) unsafe extern "C" fn DrawMoveEffectSymbols() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        DrawMoveEffectSymbol((*(*gContestResources).status.at(i)).currMove, i as u8);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetStarTileOffset() -> u16 {
    return 0x2034;
}
pub(crate) unsafe extern "C" fn UpdateConditionStars(contestantIdx: u8, resetMod: u8) -> u8 {
    let mut contestantOffset: u8 = 0;
    let mut numStars: i32 = 0;
    if (*(*gContestResources).status.at(contestantIdx)).conditionMod() == CONDITION_NO_CHANGE {
        return FALSE;
    }
    contestantOffset = gContestantTurnOrder[contestantIdx] * 5 + 2;
    numStars = ((*(*gContestResources).status.at(contestantIdx)).condition / 10) as i32;
    if (*(*gContestResources).status.at(contestantIdx)).conditionMod() == CONDITION_GAIN {
        ContestBG_FillBoxWithTile(
            0,
            GetStarTileOffset(),
            19,
            contestantOffset,
            1,
            numStars as u8,
            17,
        );
        if resetMod != 0 {
            PlaySE(SE_EXP_MAX);
            (*(*gContestResources).status.at(contestantIdx)).set_conditionMod(CONDITION_NO_CHANGE);
        }
    } else {
        ContestBG_FillBoxWithTile(
            0,
            0,
            19,
            contestantOffset + numStars as u8,
            1,
            3 - numStars as u8,
            17,
        );
        if resetMod != 0 {
            PlaySE(SE_CONTEST_CONDITION_LOSE);
            (*(*gContestResources).status.at(contestantIdx)).set_conditionMod(CONDITION_NO_CHANGE);
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn DrawConditionStars() {
    let mut i: i32 = 0;
    let mut numStars: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        let mut contestantOffset: u8 = gContestantTurnOrder[i] * 5 + 2;
        let mut starOffset: u16 = GetStarTileOffset();
        numStars = ((*(*gContestResources).status.at(i)).condition / 10) as i32;
        ContestBG_FillBoxWithTile(0, starOffset, 19, contestantOffset, 1, numStars as u8, 17);
        ContestBG_FillBoxWithTile(
            0,
            0,
            19,
            contestantOffset + numStars as u8,
            1,
            3 - numStars as u8,
            17,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetStatusSymbolTileOffset(status: u8) -> u16 {
    let mut offset: u16 = 0;
    match status {
        STAT_SYMBOL_CIRCLE => {
            offset = 0x80;
        }
        STAT_SYMBOL_WAVE => {
            offset = 0x84;
        }
        STAT_SYMBOL_X => {
            offset = 0x86;
        }
        STAT_SYMBOL_SWIRL => {
            offset = 0x88;
        }
        STAT_SYMBOL_SQUARE => {
            offset = 0x82;
        }
        _ => {}
    }
    offset += 0x9000;
    return offset;
}
pub(crate) unsafe extern "C" fn DrawStatusSymbol(contestant: u8) -> u8 {
    let mut statused: u8 = TRUE;
    let mut symbolOffset: u16 = 0;
    let mut contestantOffset: u8 = gContestantTurnOrder[contestant] * 5 + 2;
    if (*(*gContestResources).status.at(contestant)).resistant() != 0
        || (*(*gContestResources).status.at(contestant)).immune() != 0
        || (*(*gContestResources).status.at(contestant)).jamSafetyCount != 0
        || (*(*gContestResources).status.at(contestant)).jamReduction != 0
    {
        symbolOffset = GetStatusSymbolTileOffset(STAT_SYMBOL_CIRCLE);
    } else if (*(*gContestResources).status.at(contestant)).nervous() != 0 {
        symbolOffset = GetStatusSymbolTileOffset(STAT_SYMBOL_WAVE);
    } else if (*(*gContestResources).status.at(contestant)).numTurnsSkipped() != 0
        || (*(*gContestResources).status.at(contestant)).noMoreTurns() != 0
    {
        symbolOffset = GetStatusSymbolTileOffset(STAT_SYMBOL_X);
    } else {
        statused = FALSE;
    }
    if statused != 0 {
        ContestBG_FillBoxWithIncrementingTile(0, symbolOffset, 20, contestantOffset, 2, 1, 17, 1);
        ContestBG_FillBoxWithIncrementingTile(
            0,
            symbolOffset + 16,
            20,
            contestantOffset + 1,
            2,
            1,
            17,
            1,
        );
    } else {
        ContestBG_FillBoxWithTile(0, 0, 20, contestantOffset, 2, 2, 17);
    }
    return statused;
}
pub(crate) unsafe extern "C" fn DrawStatusSymbols() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        DrawStatusSymbol(i as u8);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ContestClearGeneralTextWindow() {
    FillWindowPixelBuffer(WIN_GENERAL_TEXT, 0);
    CopyWindowToVram(WIN_GENERAL_TEXT, COPYWIN_GFX);
    Contest_SetBgCopyFlags(0);
}
pub(crate) unsafe extern "C" fn GetChosenMove(contestant: u8) -> u16 {
    if Contest_IsMonsTurnDisabled(contestant) != 0 {
        return MOVE_NONE;
    }
    if contestant == gContestPlayerMonIndex {
        return gContestMons[contestant].moves[(*(*gContestResources).contest).playerMoveChoice];
    } else {
        let mut moveChoice: u8 = 0;
        ContestAI_ResetAI(contestant);
        moveChoice = ContestAI_GetActionToUse();
        return gContestMons[contestant].moves[moveChoice];
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetAllChosenMoves() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        (*(*gContestResources).status.at(i)).currMove = GetChosenMove(i as u8);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn RankContestants() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut arr: CArray<i16, 4> = zeroed();
    i = 0;
    while i < CONTESTANT_COUNT {
        (*(*gContestResources).status.at(i)).pointTotal +=
            (*(*gContestResources).status.at(i)).appeal;
        arr[i] = (*(*gContestResources).status.at(i)).pointTotal;
        i += 1;
    }
    i = 0;
    while i < 3 {
        j = 3;
        while j > i {
            if arr[j - 1] < arr[j] {
                let mut temp: u16 = 0;
                temp = arr[j] as u16;
                arr[j] = arr[j - 1];
                arr[j - 1] = temp as i16;
            }
            j -= 1;
        }
        i += 1;
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        j = 0;
        while j < CONTESTANT_COUNT {
            if (*(*gContestResources).status.at(i)).pointTotal == arr[j] {
                (*(*gContestResources).status.at(i)).set_ranking(j as u8);
                break;
            }
            j += 1;
        }
        i += 1;
    }
    SortContestants(TRUE);
    ApplyNextTurnOrder();
}
pub(crate) unsafe extern "C" fn SetAttentionLevels() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        let mut attentionLevel: u8 = 0;
        if (*(*gContestResources).status.at(i)).currMove == MOVE_NONE {
            attentionLevel = 5;
        } else if (*(*gContestResources).status.at(i)).appeal <= 0 {
            attentionLevel = 0;
        } else if (*(*gContestResources).status.at(i)).appeal < 30 {
            attentionLevel = 1;
        } else if (*(*gContestResources).status.at(i)).appeal < 60 {
            attentionLevel = 2;
        } else if (*(*gContestResources).status.at(i)).appeal < 80 {
            attentionLevel = 3;
        } else {
            attentionLevel = 4;
        }
        (*(*gContestResources).status.at(i)).attentionLevel = attentionLevel;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ContestantCanUseTurn(contestant: u8) -> u8 {
    if (*(*gContestResources).status.at(contestant)).numTurnsSkipped() != 0
        || (*(*gContestResources).status.at(contestant)).noMoreTurns() != 0
    {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetContestantStatusesForNextRound() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        (*(*gContestResources).status.at(i)).appeal = 0;
        (*(*gContestResources).status.at(i)).baseAppeal = 0;
        (*(*gContestResources).status.at(i)).jamSafetyCount = 0;
        if (*(*gContestResources).status.at(i)).numTurnsSkipped() > 0 {
            (*(*gContestResources).status.at(i))
                .set_numTurnsSkipped((*(*gContestResources).status.at(i)).numTurnsSkipped() - 1);
        }
        (*(*gContestResources).status.at(i)).jam = 0;
        (*(*gContestResources).status.at(i)).set_resistant(0);
        (*(*gContestResources).status.at(i)).jamReduction = 0;
        (*(*gContestResources).status.at(i)).set_immune(0);
        (*(*gContestResources).status.at(i)).set_moreEasilyStartled(0);
        (*(*gContestResources).status.at(i)).set_usedRepeatableMove(0);
        (*(*gContestResources).status.at(i)).set_nervous(FALSE);
        (*(*gContestResources).status.at(i)).effectStringId = CONTEST_STRING_NONE;
        (*(*gContestResources).status.at(i)).effectStringId2 = CONTEST_STRING_NONE;
        (*(*gContestResources).status.at(i)).set_conditionMod(CONDITION_NO_CHANGE);
        (*(*gContestResources).status.at(i))
            .set_repeatedPrevMove((*(*gContestResources).status.at(i)).repeatedMove());
        (*(*gContestResources).status.at(i)).set_repeatedMove(FALSE);
        (*(*gContestResources).status.at(i)).set_turnOrderModAction(0);
        (*(*gContestResources).status.at(i)).set_appealTripleCondition(0);
        if (*(*gContestResources).status.at(i)).turnSkipped() != 0 {
            (*(*gContestResources).status.at(i)).set_numTurnsSkipped(1);
            (*(*gContestResources).status.at(i)).set_turnSkipped(0);
        }
        if (*(*gContestResources).status.at(i)).exploded() != 0 {
            (*(*gContestResources).status.at(i)).set_noMoreTurns(TRUE);
            (*(*gContestResources).status.at(i)).set_exploded(FALSE);
        }
        (*(*gContestResources).status.at(i)).set_overrideCategoryExcitementMod(0);
        i += 1;
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        (*(*gContestResources).status.at(i)).prevMove =
            (*(*gContestResources).status.at(i)).currMove;
        (*(*gContestResources).contest).moveHistory[(*(*gContestResources).contest).appealNumber]
            [i] = (*(*gContestResources).status.at(i)).currMove;
        (*(*gContestResources).contest).excitementHistory
            [(*(*gContestResources).contest).appealNumber][i] =
            Contest_GetMoveExcitement((*(*gContestResources).status.at(i)).currMove) as u8;
        (*(*gContestResources).status.at(i)).currMove = MOVE_NONE;
        i += 1;
    }
    (*(*gContestResources).excitement).set_frozen(FALSE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Contest_IsMonsTurnDisabled(contestant: u8) -> u8 {
    if (*(*gContestResources).status.at(contestant)).numTurnsSkipped() != 0
        || (*(*gContestResources).status.at(contestant)).noMoreTurns() != 0
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CalculateTotalPointsForContestant(contestant: u8) {
    gContestMonRound2Points[contestant] = GetContestantRound2Points(contestant);
    gContestMonTotalPoints[contestant] =
        gContestMonRound1Points[contestant] + gContestMonRound2Points[contestant];
}
pub(crate) unsafe extern "C" fn CalculateFinalScores() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        CalculateTotalPointsForContestant(i as u8);
        i += 1;
    }
    DetermineFinalStandings();
}
pub(crate) unsafe extern "C" fn GetContestantRound2Points(contestant: u8) -> i16 {
    return gContestMonAppealPointTotals[contestant] * 2;
}
pub(crate) unsafe extern "C" fn DetermineFinalStandings() {
    let mut randomOrdering: CArray<u16, 4> = CArray([0, 0, 0, 0]);
    let mut standings: CArray<ContestFinalStandings, 4> = zeroed();
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        let mut j: i32 = 0;
        randomOrdering[i] = Random();
        j = 0;
        while j < i {
            if randomOrdering[i] == randomOrdering[j] {
                i -= 1;
                break;
            }
            j += 1;
        }
        i += 1;
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        standings[i].totalPoints = gContestMonTotalPoints[i] as i32;
        standings[i].round1Points = gContestMonRound1Points[i] as i32;
        standings[i].random = randomOrdering[i] as i32;
        standings[i].contestant = i;
        i += 1;
    }
    i = 0;
    while i < 3 {
        let mut j: i32 = 0;
        j = 3;
        while j > i {
            if DidContestantPlaceHigher(j - 1, j, standings.as_mut_ptr()) != 0 {
                let mut temp: ContestFinalStandings = zeroed();
                temp.totalPoints = standings[j - 1].totalPoints;
                temp.round1Points = standings[j - 1].round1Points;
                temp.random = standings[j - 1].random;
                temp.contestant = standings[j - 1].contestant;
                standings[j - 1].totalPoints = standings[j].totalPoints;
                standings[j - 1].round1Points = standings[j].round1Points;
                standings[j - 1].random = standings[j].random;
                standings[j - 1].contestant = standings[j].contestant;
                standings[j].totalPoints = temp.totalPoints;
                standings[j].round1Points = temp.round1Points;
                standings[j].random = temp.random;
                standings[j].contestant = temp.contestant;
            }
            j -= 1;
        }
        i += 1;
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        gContestFinalStandings[standings[i].contestant] = i as u8;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveLinkContestResults() {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
        (*gSaveBlock2Ptr).contestLinkResults[gSpecialVar_ContestCategory]
            [gContestFinalStandings[gContestPlayerMonIndex]] =
            (if (*gSaveBlock2Ptr).contestLinkResults[gSpecialVar_ContestCategory]
                [gContestFinalStandings[gContestPlayerMonIndex]] as i32
                + 1
                > 9999
            {
                9999
            } else {
                (*gSaveBlock2Ptr).contestLinkResults[gSpecialVar_ContestCategory]
                    [gContestFinalStandings[gContestPlayerMonIndex]] as i32
                    + 1
            }) as u16;
    }
}
pub(crate) unsafe extern "C" fn DidContestantPlaceHigher(
    a: i32,
    b: i32,
    standings: *mut ContestFinalStandings,
) -> u8 {
    let mut retVal: u8 = 0;
    if (*standings.at(a)).totalPoints < (*standings.at(b)).totalPoints {
        retVal = TRUE;
    } else if (*standings.at(a)).totalPoints > (*standings.at(b)).totalPoints {
        retVal = FALSE;
    } else if (*standings.at(a)).round1Points < (*standings.at(b)).round1Points {
        retVal = TRUE;
    } else if (*standings.at(a)).round1Points > (*standings.at(b)).round1Points {
        retVal = FALSE;
    } else if (*standings.at(a)).random < (*standings.at(b)).random {
        retVal = TRUE;
    } else {
        retVal = FALSE;
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn ContestPrintLinkStandby() {
    gBattle_BG0_Y = 0;
    gBattle_BG2_Y = 0;
    ContestClearGeneralTextWindow();
    Contest_StartTextPrinter(gText_LinkStandby4.as_ptr().cast_mut(), FALSE as u32);
}
pub(crate) unsafe extern "C" fn FillContestantWindowBgs() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        ContestBG_FillBoxWithTile(0, 0, 0x16, 2 + i as u8 * 5, 8, 2, 0x11);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetAppealHeartTileOffset(contestant: u8) -> u16 {
    let mut offset: u16 = 0;
    if contestant == 0 {
        offset = 0x5011;
    } else if contestant == 1 {
        offset = 0x6011;
    } else if contestant == 2 {
        offset = 0x7011;
    } else {
        offset = 0x8011;
    }
    return offset + 1;
}
pub(crate) unsafe extern "C" fn GetNumHeartsFromAppealPoints(appeal: i16) -> i8 {
    let mut hearts: i8 = (appeal / 10) as i8;
    if hearts > 16 {
        hearts = 16;
    } else if hearts < -16 {
        hearts = -16;
    }
    return hearts;
}
pub(crate) unsafe extern "C" fn UpdateAppealHearts(
    startAppeal: i16,
    appealDelta: i16,
    contestant: u8,
) -> u8 {
    let mut taskId: u8 = 0;
    let mut startHearts: i8 = 0;
    let mut heartsDelta: i8 = 0;
    (*(*gContestResources).gfxState.at(contestant)).set_updatingAppealHearts(TRUE);
    taskId = CreateTask(Some(Task_UpdateAppealHearts), 20);
    startHearts = GetNumHeartsFromAppealPoints(startAppeal);
    heartsDelta = GetNumHeartsFromAppealPoints(startAppeal + appealDelta) - startHearts;
    GetAppealHeartTileOffset(contestant);
    gTasks[taskId].data[0] = (if startHearts < 0 {
        -(startHearts as i32)
    } else {
        startHearts as i32
    }) as i16;
    gTasks[taskId].data[1] = heartsDelta as i16;
    if startHearts > 0 || startHearts == 0 && heartsDelta > 0 {
        gTasks[taskId].data[2] = 1;
    } else {
        gTasks[taskId].data[2] = -1;
    }
    gTasks[taskId].data[3] = contestant as i16;
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_UpdateAppealHearts(taskId: u8) {
    let mut contestant: u8 = gTasks[taskId].data[3] as u8;
    let mut startHearts: i16 = gTasks[taskId].data[0];
    let mut heartsDelta: i16 = gTasks[taskId].data[1];
    if ({
        gTasks[taskId].data[10] += 1;
        gTasks[taskId].data[10]
    }) > 14
    {
        let mut heartOffset: u16 = 0;
        let mut newNumHearts: u8 = 0;
        let mut pitchMod: u8 = 0;
        let mut onSecondLine: u8 = 0;
        gTasks[taskId].data[10] = 0;
        if gTasks[taskId].data[1] == 0 {
            DestroyTask(taskId);
            (*(*gContestResources).gfxState.at(contestant)).set_updatingAppealHearts(FALSE);
            return;
        } else if startHearts == 0 {
            if heartsDelta < 0 {
                heartOffset = GetAppealHeartTileOffset(contestant) + 2;
                gTasks[taskId].data[1] += 1;
            } else {
                heartOffset = GetAppealHeartTileOffset(contestant);
                gTasks[taskId].data[1] -= 1;
            }
            newNumHearts = ({
                let t2 = gTasks[taskId].data[0];
                gTasks[taskId].data[0] += 1;
                t2
            }) as u8;
        } else {
            if gTasks[taskId].data[2] < 0 {
                if heartsDelta < 0 {
                    newNumHearts = ({
                        let t3 = gTasks[taskId].data[0];
                        gTasks[taskId].data[0] += 1;
                        t3
                    }) as u8;
                    gTasks[taskId].data[1] += 1;
                    heartOffset = GetAppealHeartTileOffset(contestant) + 2;
                } else {
                    newNumHearts = ({
                        gTasks[taskId].data[0] -= 1;
                        gTasks[taskId].data[0]
                    }) as u8;
                    heartOffset = 0;
                    gTasks[taskId].data[1] -= 1;
                }
            } else {
                if heartsDelta < 0 {
                    newNumHearts = ({
                        gTasks[taskId].data[0] -= 1;
                        gTasks[taskId].data[0]
                    }) as u8;
                    heartOffset = 0;
                    gTasks[taskId].data[1] += 1;
                } else {
                    newNumHearts = ({
                        let t6 = gTasks[taskId].data[0];
                        gTasks[taskId].data[0] += 1;
                        t6
                    }) as u8;
                    gTasks[taskId].data[1] -= 1;
                    heartOffset = GetAppealHeartTileOffset(contestant);
                }
            }
        }
        pitchMod = newNumHearts;
        onSecondLine = FALSE;
        if newNumHearts > 7 {
            onSecondLine = TRUE;
            newNumHearts -= 8;
        }
        ContestBG_FillBoxWithTile(
            0,
            heartOffset,
            newNumHearts + 22,
            gContestantTurnOrder[contestant] * 5 + 2 + onSecondLine,
            1,
            1,
            17,
        );
        if heartsDelta > 0 {
            PlaySE(SE_CONTEST_HEART);
            m4aMPlayImmInit(&raw mut gMPlayInfo_SE1);
            m4aMPlayPitchControl(&raw mut gMPlayInfo_SE1, TRACKS_ALL, pitchMod as i16 * 256);
        } else {
            PlaySE(SE_BOO);
        }
        if onSecondLine == 0 && newNumHearts == 0 && heartOffset == 0 {
            gTasks[taskId].data[2] = -gTasks[taskId].data[2];
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSliderHeartSprites() {
    let mut i: i32 = 0;
    LoadSpriteSheet((&raw const *sSpriteSheet_SliderHeart).cast_mut());
    i = 0;
    while i < CONTESTANT_COUNT {
        let mut y: u8 = sSliderHeartYPositions[gContestantTurnOrder[i]];
        (*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId = CreateSprite(
            (&raw const *sSpriteTemplate_SliderHeart).cast_mut(),
            180,
            y as i16,
            1,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateHeartSlider(contestant: u8) {
    let mut spriteId: u8 = 0;
    let mut slideTarget: i16 = 0;
    (*(*gContestResources).gfxState.at(contestant)).set_sliderUpdating(TRUE);
    spriteId = (*(*gContestResources).gfxState.at(contestant)).sliderHeartSpriteId;
    slideTarget = (*(*gContestResources).status.at(contestant)).pointTotal / 10 * 2;
    if slideTarget > 56 {
        slideTarget = 56;
    } else if slideTarget < 0 {
        slideTarget = 0;
    }
    gSprites[spriteId].set_invisible(FALSE as u16);
    gSprites[spriteId].data[0] = contestant as i16;
    gSprites[spriteId].data[1] = slideTarget;
    if gSprites[spriteId].data[1] > gSprites[spriteId].x2 {
        gSprites[spriteId].data[2] = 1;
    } else {
        gSprites[spriteId].data[2] = -1;
    }
    gSprites[spriteId].callback = Some(SpriteCB_UpdateHeartSlider);
}
pub(crate) unsafe extern "C" fn UpdateHeartSliders() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        UpdateHeartSlider(i as u8);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SlidersDoneUpdating() -> u8 {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        if (*(*gContestResources).gfxState.at(i)).sliderUpdating() != 0 {
            break;
        }
        i += 1;
    }
    if i == CONTESTANT_COUNT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UpdateHeartSlider(sprite: *mut Sprite) {
    if (*sprite).x2 == (*sprite).data[1] {
        (*(*gContestResources).gfxState.at((*sprite).data[0])).set_sliderUpdating(FALSE);
        (*sprite).callback = Some(SpriteCallbackDummy);
    } else {
        (*sprite).x2 += (*sprite).data[2];
    }
}
pub(crate) unsafe extern "C" fn UpdateSliderHeartSpriteYPositions() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        gSprites[(*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId].y =
            sSliderHeartYPositions[gContestantTurnOrder[i]] as i16;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetBottomSliderHeartsInvisibility(invisible: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        if gContestantTurnOrder[i] > 1 {
            if invisible == 0 {
                gSprites[(*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId].x = 180;
            } else {
                gSprites[(*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId].x = 256;
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateNextTurnSprites() {
    let mut i: i32 = 0;
    LoadSpritePalette((&raw const *sSpritePalette_NextTurn).cast_mut());
    i = 0;
    while i < CONTESTANT_COUNT {
        LoadCompressedSpriteSheet((&raw const sSpriteSheet_NextTurn[i]).cast_mut());
        (*(*gContestResources).gfxState.at(i)).nextTurnSpriteId = CreateSprite(
            (&raw const sSpriteTemplates_NextTurn[i]).cast_mut(),
            204,
            sNextTurnSpriteYPositions[gContestantTurnOrder[i]] as i16,
            0,
        );
        SetSubspriteTables(
            &raw mut gSprites[(*(*gContestResources).gfxState.at(i)).nextTurnSpriteId],
            sSubspriteTable_NextTurn.as_ptr().cast_mut(),
        );
        gSprites[(*(*gContestResources).gfxState.at(i)).nextTurnSpriteId]
            .set_invisible(TRUE as u16);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateApplauseMeterSprite() {
    let mut spriteId: u8 = 0;
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_ApplauseMeter).cast_mut());
    LoadSpritePalette((&raw const *sSpritePalette_ApplauseMeter).cast_mut());
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_ApplauseMeter).cast_mut(),
        30,
        44,
        1,
    );
    gSprites[spriteId].set_invisible(TRUE as u16);
    (*(*gContestResources).contest).applauseMeterSpriteId = spriteId;
}
pub(crate) unsafe extern "C" fn CreateJudgeAttentionEyeTask() {
    let mut i: u8 = 0;
    let mut taskId: u8 = CreateTask(Some(Task_FlashJudgeAttentionEye), 30);
    (*(*gContestResources).contest).judgeAttentionTaskId = taskId;
    i = 0;
    while i < CONTESTANT_COUNT as u8 {
        gTasks[taskId].data[i as i32 * 4] = 0xFF;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn StartFlashJudgeAttentionEye(contestant: u8) {
    gTasks[(*(*gContestResources).contest).judgeAttentionTaskId].data[contestant as i32 * 4 + 0] =
        0;
    gTasks[(*(*gContestResources).contest).judgeAttentionTaskId].data[contestant as i32 * 4 + 1] =
        0;
}
pub(crate) unsafe extern "C" fn StopFlashJudgeAttentionEye(contestant: u8) {
    let mut taskId: u8 = CreateTask(Some(Task_StopFlashJudgeAttentionEye), 31);
    gTasks[taskId].data[0] = contestant as i16;
}
pub(crate) unsafe extern "C" fn Task_StopFlashJudgeAttentionEye(taskId: u8) {
    let mut contestant: u8 = gTasks[taskId].data[0] as u8;
    if gTasks[(*(*gContestResources).contest).judgeAttentionTaskId].data[contestant as i32 * 4 + 0]
        == 0
        || gTasks[(*(*gContestResources).contest).judgeAttentionTaskId].data
            [contestant as i32 * 4 + 0]
            == 0xFF
    {
        gTasks[(*(*gContestResources).contest).judgeAttentionTaskId].data
            [contestant as i32 * 4 + 0] = 0xFF;
        gTasks[(*(*gContestResources).contest).judgeAttentionTaskId].data
            [contestant as i32 * 4 + 1] = 0;
        BlendPalette(
            0x000 + (5 + (*(*gContestResources).contest).prevTurnOrder[contestant] as u16) * 16 + 6,
            2,
            0,
            19455,
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_FlashJudgeAttentionEye(taskId: u8) {
    let mut i: u8 = 0;
    i = 0;
    while i < CONTESTANT_COUNT as u8 {
        let mut offset: u8 = i * 4;
        if gTasks[taskId].data[offset as i32 + 0] != 0xFF {
            if gTasks[taskId].data[offset as i32 + 1] == 0 {
                gTasks[taskId].data[offset as i32 + 0] += 1;
            } else {
                gTasks[taskId].data[offset as i32 + 0] -= 1;
            }
            if gTasks[taskId].data[offset as i32 + 0] == 16
                || gTasks[taskId].data[offset as i32 + 0] == 0
            {
                gTasks[taskId].data[offset as i32 + 1] ^= 1;
            }
            BlendPalette(
                0x000 + (5 + (*(*gContestResources).contest).prevTurnOrder[i] as u16) * 16 + 6,
                2,
                gTasks[taskId].data[offset as i32 + 0] as u8,
                19455,
            );
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateUnusedBlendTask() {
    let mut i: i32 = 0;
    (*(*gContestResources).contest).blendTaskId = CreateTask(Some(Task_UnusedBlend), 30);
    i = 0;
    while i < CONTESTANT_COUNT {
        InitUnusedBlendTaskData(i as u8);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn InitUnusedBlendTaskData(contestant: u8) {
    gTasks[(*(*gContestResources).contest).blendTaskId].data[contestant as i32 * 4] = 0xFF;
    gTasks[(*(*gContestResources).contest).blendTaskId].data[contestant as i32 * 4 + 1] = 0;
}
pub(crate) unsafe extern "C" fn UpdateBlendTaskContestantsData() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        UpdateBlendTaskContestantData(i as u8);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateBlendTaskContestantData(contestant: u8) {
    let mut palOffset1: u32 = 0;
    let mut palOffset2: u32 = 0;
    InitUnusedBlendTaskData(contestant);
    palOffset1 = contestant as u32 + 5;
    {
        let mut _src: *mut c_void =
            &raw mut gPlttBufferUnfaded[palOffset1 * 16 + 10] as *mut c_void;
        let mut _dest: *mut c_void = &raw mut gPlttBufferFaded[palOffset1 * 16 + 10] as *mut c_void;
        let mut _size: u32 = 2;
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, _src as usize as u32);
                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    palOffset2 = (contestant as u32 + 5) * 16 + 12 + contestant as u32;
    {
        let mut _src: *mut c_void = &raw mut gPlttBufferUnfaded[palOffset2] as *mut c_void;
        let mut _dest: *mut c_void = &raw mut gPlttBufferFaded[palOffset2] as *mut c_void;
        let mut _size: u32 = 2;
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, _src as usize as u32);
                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UnusedBlend(taskId: u8) {
    let mut i: u8 = 0;
    i = 0;
    while i < CONTESTANT_COUNT as u8 {
        let mut idx: u8 = i * 4;
        if gTasks[taskId].data[idx] != 0xFF {
            if ({
                gTasks[taskId].data[idx as i32 + 2] += 1;
                gTasks[taskId].data[idx as i32 + 2]
            }) > 2
            {
                gTasks[taskId].data[idx as i32 + 2] = 0;
                if gTasks[taskId].data[idx as i32 + 1] == 0 {
                    gTasks[taskId].data[idx] += 1;
                } else {
                    gTasks[taskId].data[idx] -= 1;
                }
                if gTasks[taskId].data[idx] == 16 || gTasks[taskId].data[idx] == 0 {
                    gTasks[taskId].data[idx as i32 + 1] ^= 1;
                }
                BlendPalette(
                    0x000 + (5 + i as u16) * 16 + 10,
                    1,
                    gTasks[taskId].data[idx as i32 + 0] as u8,
                    19455,
                );
                BlendPalette(
                    0x000 + (5 + i as u16) * 16 + 12 + i as u16,
                    1,
                    gTasks[taskId].data[idx as i32 + 0] as u8,
                    19455,
                );
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn StartStopFlashJudgeAttentionEye(contestant: u8) {
    if (*(*gContestResources).status.at(contestant)).hasJudgesAttention() != 0 {
        StartFlashJudgeAttentionEye(contestant);
    } else {
        StopFlashJudgeAttentionEye(contestant);
    }
}
pub(crate) unsafe extern "C" fn CreateContestantBoxBlinkSprites(contestant: u8) -> u8 {
    let mut spriteId1: u8 = 0;
    let mut spriteId2: u8 = 0;
    let mut x: u8 = gContestantTurnOrder[contestant] * 40 + 32;
    LoadCompressedSpriteSheet(
        (&raw const sSpriteSheets_ContestantsTurnBlinkEffect[contestant]).cast_mut(),
    );
    LoadSpritePalette(
        (&raw const sSpritePalettes_ContestantsTurnBlinkEffect[contestant]).cast_mut(),
    );
    spriteId1 = CreateSprite(
        (&raw const sSpriteTemplates_ContestantsTurnBlinkEffect[contestant]).cast_mut(),
        184,
        x as i16,
        29,
    );
    spriteId2 = CreateSprite(
        (&raw const sSpriteTemplates_ContestantsTurnBlinkEffect[contestant]).cast_mut(),
        248,
        x as i16,
        29,
    );
    gSprites[spriteId2]
        .oam
        .set_tileNum(gSprites[spriteId2].oam.tileNum() + 64);
    CopySpriteTiles(
        0,
        3,
        VRAM as usize as *mut c_void as *mut u8,
        (0x600e000 + gContestantTurnOrder[contestant] as i32 * 5 * 64 + 0x26) as usize as *mut u16,
        (*gContestResources).boxBlinkTiles1 as *mut u8,
    );
    CopySpriteTiles(
        0,
        3,
        VRAM as usize as *mut c_void as *mut u8,
        (0x600e000 + gContestantTurnOrder[contestant] as i32 * 5 * 64 + 0x36) as usize as *mut u16,
        (*gContestResources).boxBlinkTiles2 as *mut u8,
    );
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                ((*gContestResources).boxBlinkTiles1 as *mut u8).at(1280) as *mut c_void,
                0x50000c0,
            );
        }
    }
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                ((*gContestResources).boxBlinkTiles2 as *mut u8).at(1280) as *mut c_void,
                0x50000c0,
            );
        }
    }
    RequestDma3Copy(
        (*gContestResources).boxBlinkTiles1,
        (OBJ_VRAM0 + gSprites[spriteId1].oam.tileNum() as i32 * 32) as usize as *mut u8
            as *mut c_void,
        0x800,
        1,
    );
    RequestDma3Copy(
        (*gContestResources).boxBlinkTiles2,
        (OBJ_VRAM0 + gSprites[spriteId2].oam.tileNum() as i32 * 32) as usize as *mut u8
            as *mut c_void,
        0x800,
        1,
    );
    gSprites[spriteId1].data[0] = spriteId2 as i16;
    gSprites[spriteId2].data[0] = spriteId1 as i16;
    gSprites[spriteId1].data[1] = contestant as i16;
    gSprites[spriteId2].data[1] = contestant as i16;
    return spriteId1;
}
pub(crate) unsafe extern "C" fn DestroyContestantBoxBlinkSprites(spriteId: u8) {
    let mut spriteId2: u8 = gSprites[spriteId].data[0] as u8;
    FreeSpriteOamMatrix(&raw mut gSprites[spriteId2]);
    DestroySprite(&raw mut gSprites[spriteId2]);
    DestroySpriteAndFreeResources(&raw mut gSprites[spriteId]);
}
pub(crate) unsafe extern "C" fn SetBlendForContestantBoxBlink() {
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, 2311);
}
pub(crate) unsafe extern "C" fn ResetBlendForContestantBoxBlink() {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
}
pub(crate) unsafe extern "C" fn BlinkContestantBox(spriteId: u8, b: u8) {
    let mut spriteId2: u8 = 0;
    SetBlendForContestantBoxBlink();
    (*(*gContestResources).gfxState.at(gSprites[spriteId].data[1])).set_boxBlinking(1);
    spriteId2 = gSprites[spriteId].data[0] as u8;
    StartSpriteAffineAnim(&raw mut gSprites[spriteId], 1);
    StartSpriteAffineAnim(&raw mut gSprites[spriteId2], 1);
    gSprites[spriteId].callback = Some(SpriteCB_BlinkContestantBox);
    gSprites[spriteId2].callback = Some(SpriteCallbackDummy);
    if b == FALSE {
        PlaySE(SE_CONTEST_MONS_TURN);
    } else {
        PlaySE(SE_PC_LOGIN);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BlinkContestantBox(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        let mut spriteId2: u8 = (*sprite).data[0] as u8;
        if gSprites[spriteId2].affineAnimEnded() != 0 {
            (*sprite).set_invisible(TRUE as u16);
            gSprites[spriteId2].set_invisible(TRUE as u16);
            (*sprite).callback = Some(SpriteCB_EndBlinkContestantBox);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_EndBlinkContestantBox(sprite: *mut Sprite) {
    (*(*gContestResources).gfxState.at((*sprite).data[1])).set_boxBlinking(FALSE);
    DestroyContestantBoxBlinkSprites((*sprite).data[0] as u8);
    ResetBlendForContestantBoxBlink();
}
pub(crate) unsafe extern "C" fn ContestDebugTogglePointTotal() {
    if gHeap[0x1a000] == CONTEST_DEBUG_MODE_PRINT_POINT_TOTAL {
        gHeap[0x1a000] = CONTEST_DEBUG_MODE_OFF;
    } else {
        gHeap[0x1a000] = CONTEST_DEBUG_MODE_PRINT_POINT_TOTAL;
    }
    if gHeap[0x1a000] == CONTEST_DEBUG_MODE_OFF {
        DrawContestantWindowText();
        SwapMoveDescAndContestTilemaps();
    } else {
        ContestDebugDoPrint();
    }
}
pub(crate) unsafe extern "C" fn ContestDebugDoPrint() {
    let mut i: u8 = 0;
    let mut value: i16 = 0;
    let mut txtPtr: *mut u8 = null_mut();
    let mut text: CArray<u8, 8> = zeroed();
    if gEnableContestDebugging == 0 {
        return;
    }
    match gHeap[0x1a000] {
        CONTEST_DEBUG_MODE_OFF => {}
        CONTEST_DEBUG_MODE_PRINT_WINNER_FLAGS | CONTEST_DEBUG_MODE_PRINT_LOSER_FLAGS => {
            ContestDebugPrintBitStrings();
        }
        _ => {
            i = 0;
            while i < CONTESTANT_COUNT as u8 {
                FillWindowPixelBuffer(i, 0);
                i += 1;
            }
            i = 0;
            while i < CONTESTANT_COUNT as u8 {
                value = (*(*gContestResources).status.at(i)).pointTotal;
                txtPtr = text.as_mut_ptr();
                if (*(*gContestResources).status.at(i)).pointTotal < 0 {
                    value *= -1;
                    txtPtr = StringCopy(txtPtr, gText_OneDash.as_ptr().cast_mut());
                }
                ConvertIntToDecimalStringN(txtPtr, value as i32, STR_CONV_MODE_LEFT_ALIGN, 4);
                Contest_PrintTextToBg0WindowAt(
                    gContestantTurnOrder[i] as u32,
                    text.as_mut_ptr(),
                    55,
                    1,
                    FONT_NARROW as i32,
                );
                i += 1;
            }
            i = 0;
            while i < CONTESTANT_COUNT as u8 {
                value = (*(*gContestResources).status.at(i)).appeal;
                txtPtr = text.as_mut_ptr();
                if (*(*gContestResources).status.at(i)).appeal < 0 {
                    value *= -1;
                    txtPtr = StringCopy(txtPtr, gText_OneDash.as_ptr().cast_mut());
                }
                ConvertIntToDecimalStringN(txtPtr, value as i32, STR_CONV_MODE_LEFT_ALIGN, 4);
                Contest_PrintTextToBg0WindowAt(
                    gContestantTurnOrder[i] as u32,
                    text.as_mut_ptr(),
                    5,
                    1,
                    FONT_NARROW as i32,
                );
                i += 1;
            }
            SwapMoveDescAndContestTilemaps();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SortContestants(useRanking: u8) {
    let mut scratch: CArray<u8, 4> = zeroed();
    let mut randomOrdering: CArray<u16, 4> = CArray([0, 0, 0, 0]);
    let mut i: i32 = 0;
    let mut v3: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        let mut j: i32 = 0;
        randomOrdering[i] = Random();
        j = 0;
        while j < i {
            if randomOrdering[i] == randomOrdering[j] {
                i -= 1;
                break;
            }
            j += 1;
        }
        i += 1;
    }
    if useRanking == 0 {
        i = 0;
        while i < CONTESTANT_COUNT {
            gContestantTurnOrder[i] = i as u8;
            v3 = 0;
            while v3 < i {
                if gContestMonRound1Points[gContestantTurnOrder[v3]] < gContestMonRound1Points[i]
                    || gContestMonRound1Points[gContestantTurnOrder[v3]]
                        == gContestMonRound1Points[i]
                        && randomOrdering[gContestantTurnOrder[v3]] < randomOrdering[i]
                {
                    let mut j: i32 = 0;
                    j = i;
                    while j > v3 {
                        gContestantTurnOrder[j] = gContestantTurnOrder[j - 1];
                        j -= 1;
                    }
                    gContestantTurnOrder[v3] = i as u8;
                    break;
                }
                v3 += 1;
            }
            if v3 == i {
                gContestantTurnOrder[i] = i as u8;
            }
            i += 1;
        }
        memcpy(scratch.as_mut_ptr(), gContestantTurnOrder.as_mut_ptr(), 4);
        i = 0;
        while i < CONTESTANT_COUNT {
            gContestantTurnOrder[scratch[i]] = i as u8;
            i += 1;
        }
    } else {
        memset(scratch.as_mut_ptr(), CONTESTANT_NONE as i32, 4);
        i = 0;
        while i < CONTESTANT_COUNT {
            let mut j: u8 = (*(*gContestResources).status.at(i)).ranking();
            loop {
                let mut ptr: *mut u8 = &raw mut scratch[j];
                if *ptr == CONTESTANT_NONE {
                    *ptr = i as u8;
                    gContestantTurnOrder[i] = j;
                    break;
                }
                j += 1;
            }
            i += 1;
        }
        i = 0;
        while i < 3 {
            v3 = 3;
            while v3 > i {
                if (*(*gContestResources).status.at(v3 - 1)).ranking()
                    == (*(*gContestResources).status.at(v3)).ranking()
                    && gContestantTurnOrder[v3 - 1] < gContestantTurnOrder[v3]
                    && randomOrdering[v3 - 1] < randomOrdering[v3]
                {
                    let mut temp: u8 = gContestantTurnOrder[v3];
                    gContestantTurnOrder[v3] = gContestantTurnOrder[v3 - 1];
                    gContestantTurnOrder[v3 - 1] = temp;
                }
                v3 -= 1;
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn DrawContestantWindows() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        let mut windowId: i32 = i + 5;
        LoadPalette(
            (*(gHeap.as_mut_ptr().at(106500) as *mut ContestTempSave)).cachedWindowPalettes
                [windowId]
                .as_mut_ptr() as *mut c_void,
            0x000 + (5 + gContestantTurnOrder[i] as u16) * 16,
            32,
        );
        i += 1;
    }
    DrawContestantWindowText();
}
pub(crate) unsafe extern "C" fn CalculateAppealMoveImpact(contestant: u8) {
    let mut r#move: u16 = 0;
    let mut effect: u8 = 0;
    let mut rnd: u8 = 0;
    let mut i: i32 = 0;
    (*(*gContestResources).status.at(contestant)).appeal = 0;
    (*(*gContestResources).status.at(contestant)).baseAppeal = 0;
    if ContestantCanUseTurn(contestant) == 0 {
        return;
    }
    r#move = (*(*gContestResources).status.at(contestant)).currMove;
    effect = gContestMoves[r#move].effect;
    (*(*gContestResources).status.at(contestant)).moveCategory =
        gContestMoves[(*(*gContestResources).status.at(contestant)).currMove].contestCategory();
    if (*(*gContestResources).status.at(contestant)).currMove
        == (*(*gContestResources).status.at(contestant)).prevMove
        && (*(*gContestResources).status.at(contestant)).currMove != MOVE_NONE
    {
        (*(*gContestResources).status.at(contestant)).set_repeatedMove(TRUE);
        (*(*gContestResources).status.at(contestant)).set_moveRepeatCount(
            (*(*gContestResources).status.at(contestant)).moveRepeatCount() + 1,
        );
    } else {
        (*(*gContestResources).status.at(contestant)).set_moveRepeatCount(0);
    }
    (*(*gContestResources).status.at(contestant)).baseAppeal =
        gContestEffects[effect].appeal as i16;
    (*(*gContestResources).status.at(contestant)).appeal =
        (*(*gContestResources).status.at(contestant)).baseAppeal;
    (*(*gContestResources).appealResults).jam = gContestEffects[effect].jam as i16;
    (*(*gContestResources).appealResults).jam2 = (*(*gContestResources).appealResults).jam;
    (*(*gContestResources).appealResults).contestant = contestant;
    i = 0;
    while i < CONTESTANT_COUNT {
        (*(*gContestResources).status.at(i)).jam = 0;
        (*(*gContestResources).appealResults).unnervedPokes[i] = 0;
        i += 1;
    }
    if (*(*gContestResources).status.at(contestant)).hasJudgesAttention() != 0
        && AreMovesContestCombo(
            (*(*gContestResources).status.at(contestant)).prevMove,
            (*(*gContestResources).status.at(contestant)).currMove,
        ) == 0
    {
        (*(*gContestResources).status.at(contestant)).set_hasJudgesAttention(FALSE);
    }
    gContestEffectFuncs[effect].unwrap_unchecked()();
    if (*(*gContestResources).status.at(contestant)).conditionMod() == CONDITION_GAIN {
        (*(*gContestResources).status.at(contestant)).appeal +=
            (*(*gContestResources).status.at(contestant)).condition as i16 - 10;
    } else if (*(*gContestResources).status.at(contestant)).appealTripleCondition() != 0 {
        (*(*gContestResources).status.at(contestant)).appeal +=
            (*(*gContestResources).status.at(contestant)).condition as i16 * 3;
    } else {
        (*(*gContestResources).status.at(contestant)).appeal +=
            (*(*gContestResources).status.at(contestant)).condition as i16;
    }
    (*(*gContestResources).status.at(contestant)).completedCombo = FALSE;
    (*(*gContestResources).status.at(contestant)).set_usedComboMove(FALSE);
    if IsContestantAllowedToCombo(contestant) != 0 {
        let mut completedCombo: u8 = AreMovesContestCombo(
            (*(*gContestResources).status.at(contestant)).prevMove,
            (*(*gContestResources).status.at(contestant)).currMove,
        );
        if completedCombo != 0
            && (*(*gContestResources).status.at(contestant)).hasJudgesAttention() != 0
        {
            (*(*gContestResources).status.at(contestant)).completedCombo = completedCombo;
            (*(*gContestResources).status.at(contestant)).set_usedComboMove(TRUE);
            (*(*gContestResources).status.at(contestant)).set_hasJudgesAttention(FALSE);
            (*(*gContestResources).status.at(contestant)).comboAppealBonus =
                (*(*gContestResources).status.at(contestant)).baseAppeal as u8
                    * (*(*gContestResources).status.at(contestant)).completedCombo;
            (*(*gContestResources).status.at(contestant)).set_completedComboFlag(TRUE);
        } else {
            if gContestMoves[(*(*gContestResources).status.at(contestant)).currMove].comboStarterId
                != 0
            {
                (*(*gContestResources).status.at(contestant)).set_hasJudgesAttention(TRUE);
                (*(*gContestResources).status.at(contestant)).set_usedComboMove(TRUE);
            } else {
                (*(*gContestResources).status.at(contestant)).set_hasJudgesAttention(FALSE);
            }
        }
    }
    if (*(*gContestResources).status.at(contestant)).repeatedMove() != 0 {
        (*(*gContestResources).status.at(contestant)).repeatJam =
            ((*(*gContestResources).status.at(contestant)).moveRepeatCount() + 1) * 10;
    }
    if (*(*gContestResources).status.at(contestant)).nervous() != 0 {
        (*(*gContestResources).status.at(contestant)).set_hasJudgesAttention(FALSE);
        (*(*gContestResources).status.at(contestant)).appeal = 0;
        (*(*gContestResources).status.at(contestant)).baseAppeal = 0;
    }
    (*(*gContestResources).excitement).moveExcitement =
        Contest_GetMoveExcitement((*(*gContestResources).status.at(contestant)).currMove);
    if (*(*gContestResources).status.at(contestant)).overrideCategoryExcitementMod() != 0 {
        (*(*gContestResources).excitement).moveExcitement = 1;
    }
    if (*(*gContestResources).excitement).moveExcitement > 0 {
        if (*(*gContestResources).contest).applauseLevel as i32
            + (*(*gContestResources).excitement).moveExcitement as i32
            > 4
        {
            (*(*gContestResources).excitement).excitementAppealBonus = 60;
        } else {
            (*(*gContestResources).excitement).excitementAppealBonus = 10;
        }
    } else {
        (*(*gContestResources).excitement).excitementAppealBonus = 0;
    }
    rnd = (Random() as i32 % 3) as u8;
    i = 0;
    while i < CONTESTANT_COUNT {
        if i != contestant as i32 {
            if rnd == 0 {
                break;
            }
            rnd -= 1;
        }
        i += 1;
    }
    (*(*gContestResources).status.at(contestant)).contestantAnimTarget = i as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContestantEffectStringID(contestant: u8, effectStringId: u8) {
    (*(*gContestResources).status.at(contestant)).effectStringId = effectStringId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContestantEffectStringID2(contestant: u8, effectStringId: u8) {
    (*(*gContestResources).status.at(contestant)).effectStringId2 = effectStringId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetStartledString(contestant: u8, jam: u8) {
    if jam >= 60 {
        SetContestantEffectStringID(contestant, CONTEST_STRING_TRIPPED_OVER);
    } else if jam >= 40 {
        SetContestantEffectStringID(contestant, CONTEST_STRING_LEAPT_UP);
    } else if jam >= 30 {
        SetContestantEffectStringID(contestant, CONTEST_STRING_UTTER_CRY);
    } else if jam >= 20 {
        SetContestantEffectStringID(contestant, CONTEST_STRING_TURNED_BACK);
    } else if jam >= 10 {
        SetContestantEffectStringID(contestant, CONTEST_STRING_LOOKED_DOWN);
    }
}
pub(crate) unsafe extern "C" fn PrintAppealMoveResultText(contestant: u8, stringId: u8) {
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gContestMons[contestant].nickname.as_mut_ptr(),
    );
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gMoveNames[(*(*gContestResources).status.at(contestant)).currMove]
            .as_ptr()
            .cast_mut(),
    );
    if gContestMoves[(*(*gContestResources)
        .status
        .at((*(*gContestResources).appealResults).contestant))
    .currMove]
        .contestCategory()
        == CONTEST_CATEGORY_COOL
    {
        StringCopy(
            gStringVar3.as_mut_ptr(),
            gText_Contest_Shyness.as_ptr().cast_mut(),
        );
    } else if gContestMoves[(*(*gContestResources)
        .status
        .at((*(*gContestResources).appealResults).contestant))
    .currMove]
        .contestCategory()
        == CONTEST_CATEGORY_BEAUTY
    {
        StringCopy(
            gStringVar3.as_mut_ptr(),
            gText_Contest_Anxiety.as_ptr().cast_mut(),
        );
    } else if gContestMoves[(*(*gContestResources)
        .status
        .at((*(*gContestResources).appealResults).contestant))
    .currMove]
        .contestCategory()
        == CONTEST_CATEGORY_CUTE
    {
        StringCopy(
            gStringVar3.as_mut_ptr(),
            gText_Contest_Laziness.as_ptr().cast_mut(),
        );
    } else if gContestMoves[(*(*gContestResources)
        .status
        .at((*(*gContestResources).appealResults).contestant))
    .currMove]
        .contestCategory()
        == CONTEST_CATEGORY_SMART
    {
        StringCopy(
            gStringVar3.as_mut_ptr(),
            gText_Contest_Hesitancy.as_ptr().cast_mut(),
        );
    } else {
        StringCopy(
            gStringVar3.as_mut_ptr(),
            gText_Contest_Fear.as_ptr().cast_mut(),
        );
    }
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), sAppealResultTexts[stringId]);
    ContestClearGeneralTextWindow();
    Contest_StartTextPrinter(gStringVar4.as_mut_ptr(), TRUE as u32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MakeContestantNervous(p: u8) {
    (*(*gContestResources).status.at(p)).set_nervous(TRUE);
    (*(*gContestResources).status.at(p)).currMove = MOVE_NONE;
}
pub(crate) unsafe extern "C" fn ApplyNextTurnOrder() {
    let mut nextContestant: u8 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut newTurnOrder: CArray<u8, 4> = zeroed();
    let mut isContestantOrdered: CArray<u8, 4> = zeroed();
    i = 0;
    while i < CONTESTANT_COUNT {
        newTurnOrder[i] = gContestantTurnOrder[i];
        isContestantOrdered[i] = FALSE;
        i += 1;
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        j = 0;
        while j < CONTESTANT_COUNT {
            if (*(*gContestResources).status.at(j)).nextTurnOrder as i32 == i {
                newTurnOrder[j] = i as u8;
                isContestantOrdered[j] = TRUE;
                break;
            }
            j += 1;
        }
        if j == CONTESTANT_COUNT {
            j = 0;
            while j < CONTESTANT_COUNT {
                if isContestantOrdered[j] == 0
                    && (*(*gContestResources).status.at(j)).nextTurnOrder == CONTESTANT_NONE
                {
                    nextContestant = j as u8;
                    j += 1;
                    break;
                }
                j += 1;
            }
            while j < CONTESTANT_COUNT {
                if isContestantOrdered[j] == 0
                    && (*(*gContestResources).status.at(j)).nextTurnOrder == CONTESTANT_NONE
                    && gContestantTurnOrder[nextContestant] > gContestantTurnOrder[j]
                {
                    nextContestant = j as u8;
                }
                j += 1;
            }
            newTurnOrder[nextContestant] = i as u8;
            isContestantOrdered[nextContestant] = TRUE;
        }
        i += 1;
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        (*(*gContestResources).appealResults).turnOrder[i] = newTurnOrder[i];
        (*(*gContestResources).status.at(i)).nextTurnOrder = CONTESTANT_NONE;
        (*(*gContestResources).status.at(i)).set_turnOrderMod(0);
        gContestantTurnOrder[i] = newTurnOrder[i];
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_JudgeSpeechBubble(sprite: *mut Sprite) {
    if ({
        let t1 = (*sprite).data[1];
        (*sprite).data[1] += 1;
        t1
    }) > 84
    {
        (*sprite).data[1] = 0;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(SpriteCallbackDummy);
        (*(*gContestResources).contest).set_waitForJudgeSpeechBubble(FALSE as u16);
    }
}
pub(crate) unsafe extern "C" fn DoJudgeSpeechBubble(symbolId: u8) {
    let mut spriteId: u8 = (*(*gContestResources).contest).judgeSpeechBubbleSpriteId;
    match symbolId {
        JUDGE_SYMBOL_SWIRL | JUDGE_SYMBOL_SWIRL_UNUSED => {
            gSprites[spriteId]
                .oam
                .set_tileNum(gSprites[spriteId].data[0] as u16);
            PlaySE(SE_FAILURE);
        }
        JUDGE_SYMBOL_ONE_EXCLAMATION => {
            gSprites[spriteId]
                .oam
                .set_tileNum(gSprites[spriteId].data[0] as u16 + 4);
            PlaySE(SE_SUCCESS);
        }
        JUDGE_SYMBOL_TWO_EXCLAMATIONS => {
            gSprites[spriteId]
                .oam
                .set_tileNum(gSprites[spriteId].data[0] as u16 + 8);
            PlaySE(SE_SUCCESS);
        }
        JUDGE_SYMBOL_NUMBER_ONE_UNUSED => {
            gSprites[spriteId]
                .oam
                .set_tileNum(gSprites[spriteId].data[0] as u16 + 12);
            PlaySE(SE_WARP_IN);
        }
        JUDGE_SYMBOL_NUMBER_ONE => {
            gSprites[spriteId]
                .oam
                .set_tileNum(gSprites[spriteId].data[0] as u16 + 12);
            PlaySE(SE_WARP_IN);
        }
        JUDGE_SYMBOL_NUMBER_FOUR => {
            gSprites[spriteId]
                .oam
                .set_tileNum(gSprites[spriteId].data[0] as u16 + 16);
            PlaySE(SE_WARP_IN);
        }
        JUDGE_SYMBOL_STAR => {
            gSprites[spriteId]
                .oam
                .set_tileNum(gSprites[spriteId].data[0] as u16 + 24);
            PlaySE(SE_M_HEAL_BELL);
        }
        _ => {
            gSprites[spriteId]
                .oam
                .set_tileNum(gSprites[spriteId].data[0] as u16 + 20);
            PlaySE(SE_WARP_IN);
        }
    }
    gSprites[spriteId].data[1] = 0;
    gSprites[spriteId].set_invisible(FALSE as u16);
    gSprites[spriteId].callback = Some(SpriteCB_JudgeSpeechBubble);
    (*(*gContestResources).contest).set_waitForJudgeSpeechBubble(TRUE as u16);
}
pub(crate) unsafe extern "C" fn UpdateApplauseMeter() {
    let mut i: i32 = 0;
    i = 0;
    while i < APPLAUSE_METER_SIZE {
        let mut src: *mut u8 = null_mut();
        if i < (*(*gContestResources).contest).applauseLevel as i32 {
            src = (&raw const gContestApplauseMeterGfx[64]).cast_mut();
        } else {
            src = gContestApplauseMeterGfx.as_ptr().cast_mut();
        }
        CpuSet(
            src as *mut c_void,
            (OBJ_VRAM0
                + (gSprites[(*(*gContestResources).contest).applauseMeterSpriteId]
                    .oam
                    .tileNum() as i32
                    + 17
                    + i)
                    * 32) as usize as *mut c_void,
            0x4000008,
        );
        CpuSet(
            src.at(32) as *mut c_void,
            (OBJ_VRAM0
                + (gSprites[(*(*gContestResources).contest).applauseMeterSpriteId]
                    .oam
                    .tileNum() as i32
                    + 25
                    + i)
                    * 32) as usize as *mut c_void,
            0x4000008,
        );
        if (*(*gContestResources).contest).applauseLevel > 4 {
            StartApplauseOverflowAnimation();
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Contest_GetMoveExcitement(r#move: u16) -> i8 {
    return sContestExcitementTable[gSpecialVar_ContestCategory]
        [gContestMoves[r#move].contestCategory()];
}
pub(crate) unsafe extern "C" fn StartApplauseOverflowAnimation() -> u8 {
    let mut taskId: u8 = CreateTask(Some(Task_ApplauseOverflowAnimation), 10);
    gTasks[taskId].data[1] = 1;
    gTasks[taskId].data[2] = IndexOfSpritePaletteTag(TAG_APPLAUSE_METER) as i16;
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_ApplauseOverflowAnimation(taskId: u8) {
    if ({
        gTasks[taskId].data[0] += 1;
        gTasks[taskId].data[0]
    }) == 1
    {
        gTasks[taskId].data[0] = 0;
        if gTasks[taskId].data[3] == 0 {
            gTasks[taskId].data[4] += 1;
        } else {
            gTasks[taskId].data[4] -= 1;
        }
        BlendPalette(
            0x100 + gTasks[taskId].data[2] as u16 * 16 + 8,
            1,
            gTasks[taskId].data[4] as u8,
            32767,
        );
        if gTasks[taskId].data[4] == 0 || gTasks[taskId].data[4] == 16 {
            gTasks[taskId].data[3] ^= 1;
            if (*(*gContestResources).contest).applauseLevel < 5 {
                BlendPalette(0x100 + gTasks[taskId].data[2] as u16 * 16 + 8, 1, 0, 31);
                DestroyTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SlideApplauseMeterIn() {
    CreateTask(Some(Task_SlideApplauseMeterIn), 10);
    gSprites[(*(*gContestResources).contest).applauseMeterSpriteId].x2 = -70;
    gSprites[(*(*gContestResources).contest).applauseMeterSpriteId].set_invisible(FALSE as u16);
    (*(*gContestResources).contest).set_applauseMeterIsMoving(TRUE as u16);
}
pub(crate) unsafe extern "C" fn Task_SlideApplauseMeterIn(taskId: u8) {
    let mut sprite: *mut Sprite =
        &raw mut gSprites[(*(*gContestResources).contest).applauseMeterSpriteId];
    gTasks[taskId].data[10] += 1664;
    (*sprite).x2 += gTasks[taskId].data[10] >> 8;
    gTasks[taskId].data[10] = gTasks[taskId].data[10] & 0xFF;
    if (*sprite).x2 > 0 {
        (*sprite).x2 = 0;
    }
    if (*sprite).x2 == 0 {
        (*(*gContestResources).contest).set_applauseMeterIsMoving(FALSE as u16);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SlideApplauseMeterOut() {
    if gSprites[(*(*gContestResources).contest).applauseMeterSpriteId].invisible() == TRUE as u16 {
        (*(*gContestResources).contest).set_applauseMeterIsMoving(FALSE as u16);
    } else {
        CreateTask(Some(Task_SlideApplauseMeterOut), 10);
        gSprites[(*(*gContestResources).contest).applauseMeterSpriteId].x2 = 0;
        (*(*gContestResources).contest).set_applauseMeterIsMoving(TRUE as u16);
    }
}
pub(crate) unsafe extern "C" fn Task_SlideApplauseMeterOut(taskId: u8) {
    let mut sprite: *mut Sprite =
        &raw mut gSprites[(*(*gContestResources).contest).applauseMeterSpriteId];
    gTasks[taskId].data[10] += 1664;
    (*sprite).x2 -= gTasks[taskId].data[10] >> 8;
    gTasks[taskId].data[10] = gTasks[taskId].data[10] & 0xFF;
    if (*sprite).x2 < -70 {
        (*sprite).x2 = -70;
    }
    if (*sprite).x2 == -70 {
        (*sprite).set_invisible(TRUE as u16);
        (*(*gContestResources).contest).set_applauseMeterIsMoving(FALSE as u16);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn ShowAndUpdateApplauseMeter(unused: i8) {
    let mut taskId: u8 = CreateTask(Some(Task_ShowAndUpdateApplauseMeter), 5);
    gTasks[taskId].data[0] = unused as i16;
    (*(*gContestResources).contest).set_isShowingApplauseMeter(TRUE as u16);
}
pub(crate) unsafe extern "C" fn Task_ShowAndUpdateApplauseMeter(taskId: u8) {
    match gTasks[taskId].data[10] {
        0 => {
            SlideApplauseMeterIn();
            gTasks[taskId].data[10] += 1;
        }
        1 => {
            if (*(*gContestResources).contest).applauseMeterIsMoving() == 0 {
                gTasks[taskId].data[10] += 1;
            }
        }
        2 => {
            if ({
                let t1 = gTasks[taskId].data[11];
                gTasks[taskId].data[11] += 1;
                t1
            }) > 20
            {
                gTasks[taskId].data[11] = 0;
                UpdateApplauseMeter();
                (*(*gContestResources).contest).set_isShowingApplauseMeter(FALSE as u16);
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn HideApplauseMeterNoAnim() {
    gSprites[(*(*gContestResources).contest).applauseMeterSpriteId].x2 = 0;
    gSprites[(*(*gContestResources).contest).applauseMeterSpriteId].set_invisible(FALSE as u16);
}
pub(crate) unsafe extern "C" fn ShowApplauseMeterNoAnim() {
    gSprites[(*(*gContestResources).contest).applauseMeterSpriteId].set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn AnimateAudience() {
    CreateTask(Some(Task_AnimateAudience), 15);
    (*(*gContestResources).contest).set_animatingAudience(TRUE as u16);
}
pub(crate) unsafe extern "C" fn Task_AnimateAudience(taskId: u8) {
    if ({
        let t1 = gTasks[taskId].data[10];
        gTasks[taskId].data[10] += 1;
        t1
    }) > 6
    {
        gTasks[taskId].data[10] = 0;
        if gTasks[taskId].data[11] == 0 {
            RequestDma3Copy(
                gHeap.as_mut_ptr().at(0x19000) as *mut c_void,
                0x6002000 as usize as *mut c_void,
                0x1000,
                1,
            );
        } else {
            RequestDma3Copy(
                gHeap.as_mut_ptr().at(0x18000) as *mut c_void,
                0x6002000 as usize as *mut c_void,
                0x1000,
                1,
            );
            gTasks[taskId].data[12] += 1;
        }
        gTasks[taskId].data[11] ^= 1;
        if gTasks[taskId].data[12] == 9 {
            (*(*gContestResources).contest).set_animatingAudience(FALSE as u16);
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn BlendAudienceBackground(excitementDir: i8, blendDir: i8) {
    let mut taskId: u8 = CreateTask(Some(Task_BlendAudienceBackground), 10);
    let mut blendColor: u16 = 0;
    let mut blendCoeff: u8 = 0;
    let mut targetBlendCoeff: u8 = 0;
    if excitementDir > 0 {
        blendColor = 9086;
        if blendDir > 0 {
            blendCoeff = 0;
            targetBlendCoeff = (*(*gContestResources).contest).applauseLevel as u8 * 3;
        } else {
            blendCoeff = (*(*gContestResources).contest).applauseLevel as u8 * 3;
            targetBlendCoeff = 0;
        }
    } else {
        blendColor = 0;
        if blendDir > 0 {
            blendCoeff = 0;
            targetBlendCoeff = 12;
        } else {
            blendCoeff = 12;
            targetBlendCoeff = 0;
        }
    }
    gTasks[taskId].data[0] = blendColor as i16;
    gTasks[taskId].data[1] = blendCoeff as i16;
    gTasks[taskId].data[2] = blendDir as i16;
    gTasks[taskId].data[3] = targetBlendCoeff as i16;
    (*(*gContestResources).contest).set_waitForAudienceBlend(FALSE as u16);
}
pub(crate) unsafe extern "C" fn Task_BlendAudienceBackground(taskId: u8) {
    if ({
        let t1 = gTasks[taskId].data[10];
        gTasks[taskId].data[10] += 1;
        t1
    }) >= 0
    {
        gTasks[taskId].data[10] = 0;
        if gTasks[taskId].data[2] > 0 {
            gTasks[taskId].data[1] += 1;
        } else {
            gTasks[taskId].data[1] -= 1;
        }
        BlendPalette(
            17,
            1,
            gTasks[taskId].data[1] as u8,
            gTasks[taskId].data[0] as u16,
        );
        BlendPalette(
            26,
            1,
            gTasks[taskId].data[1] as u8,
            gTasks[taskId].data[0] as u16,
        );
        if gTasks[taskId].data[1] == gTasks[taskId].data[3] {
            DestroyTask(taskId);
            (*(*gContestResources).contest).set_waitForAudienceBlend(FALSE as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn ShowHideNextTurnGfx(show: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        if (*(*gContestResources).status.at(i)).turnOrderMod() != 0 && show != 0 {
            CpuSet(
                GetTurnOrderNumberGfx(i as u8) as *mut c_void,
                (OBJ_VRAM0
                    + (gSprites[(*(*gContestResources).gfxState.at(i)).nextTurnSpriteId]
                        .oam
                        .tileNum() as i32
                        + 6)
                        * 32) as usize as *mut c_void,
                0x4000008,
            );
            gSprites[(*(*gContestResources).gfxState.at(i)).nextTurnSpriteId].y =
                sNextTurnSpriteYPositions[gContestantTurnOrder[i]] as i16;
            gSprites[(*(*gContestResources).gfxState.at(i)).nextTurnSpriteId]
                .set_invisible(FALSE as u16);
        } else {
            gSprites[(*(*gContestResources).gfxState.at(i)).nextTurnSpriteId]
                .set_invisible(TRUE as u16);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetTurnOrderNumberGfx(contestant: u8) -> *mut u8 {
    if (*(*gContestResources).status.at(contestant)).turnOrderMod() != 1 {
        return gContestNextTurnRandomGfx.as_ptr().cast_mut();
    } else {
        return gContestNextTurnNumbersGfx
            .as_ptr()
            .cast_mut()
            .at((*(*gContestResources).status.at(contestant)).nextTurnOrder as i32 * 32);
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
pub(crate) unsafe extern "C" fn DrawUnnervedSymbols() {
    let mut i: i32 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        if (*(*gContestResources).appealResults).unnervedPokes[i] != 0
            && Contest_IsMonsTurnDisabled(i as u8) == 0
        {
            let mut contestantOffset: u32 = gContestantTurnOrder[i] as u32 * 5 + 2;
            let mut symbolOffset: u16 = GetStatusSymbolTileOffset(STAT_SYMBOL_SWIRL);
            ContestBG_FillBoxWithIncrementingTile(
                0,
                symbolOffset,
                20,
                contestantOffset as u8,
                2,
                1,
                17,
                1,
            );
            symbolOffset += 16;
            ContestBG_FillBoxWithIncrementingTile(
                0,
                symbolOffset,
                20,
                contestantOffset as u8 + 1,
                2,
                1,
                17,
                1,
            );
            PlaySE(SE_CONTEST_ICON_CHANGE);
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsContestantAllowedToCombo(contestant: u8) -> u8 {
    if (*(*gContestResources).status.at(contestant)).repeatedMove() != 0
        || (*(*gContestResources).status.at(contestant)).nervous() != 0
    {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetBgForCurtainDrop() {
    let mut i: i32 = 0;
    let mut bg0Cnt: u16 = 0;
    let mut bg1Cnt: u16 = 0;
    let mut bg2Cnt: u16 = 0;
    bg1Cnt = GetGpuReg(REG_OFFSET_BG1CNT);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_priority(0);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_screenSize(2);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_areaOverflowMode(0);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_charBaseBlock(0);
    SetGpuReg(REG_OFFSET_BG1CNT, bg1Cnt);
    bg0Cnt = GetGpuReg(REG_OFFSET_BG0CNT);
    bg2Cnt = GetGpuReg(REG_OFFSET_BG2CNT);
    (*(&raw mut bg0Cnt as *mut BgCnt)).set_priority(1);
    (*(&raw mut bg2Cnt as *mut BgCnt)).set_priority(1);
    SetGpuReg(REG_OFFSET_BG0CNT, bg0Cnt);
    SetGpuReg(REG_OFFSET_BG2CNT, bg2Cnt);
    gBattle_BG1_X = DISPLAY_WIDTH;
    gBattle_BG1_Y = DISPLAY_HEIGHT;
    SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*gContestResources).contestBgTilemaps[1] as *mut c_void,
                0x5000400,
            );
        }
    }
    CopyToBgTilemapBuffer(
        1,
        gContestCurtainTilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    Contest_SetBgCopyFlags(1);
    i = 0;
    while i < CONTESTANT_COUNT {
        gSprites[(*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId]
            .oam
            .set_priority(1);
        gSprites[(*(*gContestResources).gfxState.at(i)).nextTurnSpriteId]
            .oam
            .set_priority(1);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateContestantBoxOrder() {
    let mut i: i32 = 0;
    let mut bg1Cnt: u16 = 0;
    RequestDma3Fill(0, 0x6008000 as usize as *mut c_void, 0x2000, 1);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*gContestResources).contestBgTilemaps[1] as *mut c_void,
                0x5000400,
            );
        }
    }
    Contest_SetBgCopyFlags(1);
    bg1Cnt = GetGpuReg(REG_OFFSET_BG1CNT);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_priority(1);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_screenSize(0);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_areaOverflowMode(0);
    (*(&raw mut bg1Cnt as *mut BgCnt)).set_charBaseBlock(2);
    SetGpuReg(REG_OFFSET_BG1CNT, bg1Cnt);
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        gSprites[(*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId]
            .oam
            .set_priority(0);
        gSprites[(*(*gContestResources).gfxState.at(i)).nextTurnSpriteId]
            .oam
            .set_priority(0);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Task_StartDropCurtainAtRoundEnd(taskId: u8) {
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = DISPLAY_HEIGHT;
    PlaySE12WithPanning(SE_CONTEST_CURTAIN_FALL, 0);
    gTasks[taskId].func = Some(Task_UpdateCurtainDropAtRoundEnd);
}
pub(crate) unsafe extern "C" fn Task_UpdateCurtainDropAtRoundEnd(taskId: u8) {
    if (({
        gBattle_BG1_Y -= 7;
        gBattle_BG1_Y
    }) as i16)
        < 0
    {
        gBattle_BG1_Y = 0;
    }
    if gBattle_BG1_Y == 0 {
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].data[1] = 0;
        gTasks[taskId].data[2] = 0;
        gTasks[taskId].func = Some(Task_ResetForNextRound);
    }
}
pub(crate) unsafe extern "C" fn Task_ResetForNextRound(taskId: u8) {
    let mut i: i32 = 0;
    match gTasks[taskId].data[0] {
        0 => {
            i = 0;
            while i < CONTESTANT_COUNT {
                (*(*gContestResources).contest).prevTurnOrder[i] = gContestantTurnOrder[i];
                i += 1;
            }
            FillContestantWindowBgs();
            UpdateBlendTaskContestantsData();
            DrawConditionStars();
            DrawContestantWindows();
            ShowHideNextTurnGfx(TRUE);
            UpdateSliderHeartSpriteYPositions();
            gTasks[taskId].data[0] = 1;
        }
        1 => {
            if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
                let mut taskId2: u8 = 0;
                (*(*gContestResources).contest).set_waitForLink(TRUE as u16);
                if IsPlayerLinkLeader() != 0 {
                    SetContestantStatusesForNextRound();
                }
                taskId2 = CreateTask(Some(Task_LinkContest_CommunicateAppealsState), 0);
                SetTaskFuncWithFollowupFunc(
                    taskId2,
                    Some(Task_LinkContest_CommunicateAppealsState),
                    Some(Task_EndWaitForLink),
                );
                ContestPrintLinkStandby();
                gTasks[taskId].data[0] = 2;
            } else {
                SetContestantStatusesForNextRound();
                gTasks[taskId].data[0] = 3;
            }
        }
        2 => {
            if (*(*gContestResources).contest).waitForLink() == 0 {
                gTasks[taskId].data[0] = 3;
            }
        }
        3 => {
            DrawStatusSymbols();
            SwapMoveDescAndContestTilemaps();
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].func = Some(Task_WaitRaiseCurtainAtRoundEnd);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateRaiseCurtainAtRoundEnd(taskId: u8) {
    if ({
        gBattle_BG1_Y += 7;
        gBattle_BG1_Y
    }) as i16
        > DISPLAY_HEIGHT as i16
    {
        gTasks[taskId].func = Some(Task_UpdateContestantBoxOrder);
    }
}
pub(crate) unsafe extern "C" fn Task_WaitRaiseCurtainAtRoundEnd(taskId: u8) {
    if gTasks[taskId].data[2] < 10 {
        gTasks[taskId].data[2] += 1;
    } else {
        if gTasks[taskId].data[1] == 0 {
            if gTasks[taskId].data[0] == 16 {
                gTasks[taskId].data[1] += 1;
            } else {
                gTasks[taskId].data[0] += 1;
            }
        } else {
            if gTasks[taskId].data[0] == 0 {
                gTasks[taskId].data[1] = 0;
                gTasks[taskId].data[2] = 0;
                gTasks[taskId].func = Some(Task_StartRaiseCurtainAtRoundEnd);
            } else {
                gTasks[taskId].data[0] -= 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartRaiseCurtainAtRoundEnd(taskId: u8) {
    if gTasks[taskId].data[2] < 10 {
        gTasks[taskId].data[2] += 1;
    } else {
        gTasks[taskId].data[2] = 0;
        PlaySE12WithPanning(SE_CONTEST_CURTAIN_RISE, 0);
        gTasks[taskId].func = Some(Task_UpdateRaiseCurtainAtRoundEnd);
    }
}
pub(crate) unsafe extern "C" fn AnimateSliderHearts(animId: u8) {
    let mut i: i32 = 0;
    let mut taskId: u8 = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        gSprites[(*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId]
            .oam
            .set_matrixNum(AllocOamMatrix() as u32);
        gSprites[(*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId]
            .oam
            .set_affineMode(ST_OAM_AFFINE_NORMAL);
        StartSpriteAffineAnim(
            &raw mut gSprites[(*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId],
            animId,
        );
        if animId == SLIDER_HEART_ANIM_APPEAR {
            AnimateSprite(
                &raw mut gSprites[(*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId],
            );
            gSprites[(*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId]
                .set_invisible(FALSE as u16);
        }
        i += 1;
    }
    taskId = CreateTask(Some(Task_WaitForSliderHeartAnim), 5);
    gTasks[taskId].data[0] = animId as i16;
    (*(*gContestResources).contest).set_sliderHeartsAnimating(TRUE as u16);
}
pub(crate) unsafe extern "C" fn Task_WaitForSliderHeartAnim(taskId: u8) {
    let mut i: i32 = 0;
    if gSprites[(*(*gContestResources).gfxState).sliderHeartSpriteId].affineAnimEnded() != 0 {
        if gTasks[taskId].data[0] as u8 == SLIDER_HEART_ANIM_DISAPPEAR {
            i = 0;
            while i < CONTESTANT_COUNT {
                gSprites[(*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId]
                    .set_invisible(TRUE as u16);
                i += 1;
            }
        }
        i = 0;
        while i < CONTESTANT_COUNT {
            FreeSpriteOamMatrix(
                &raw mut gSprites[(*(*gContestResources).gfxState.at(i)).sliderHeartSpriteId],
            );
            i += 1;
        }
        (*(*gContestResources).contest).set_sliderHeartsAnimating(FALSE as u16);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SanitizeMove(mut r#move: u16) -> u16 {
    if r#move >= MOVES_COUNT {
        r#move = MOVE_POUND as u16;
    }
    return r#move;
}
pub(crate) unsafe extern "C" fn SanitizeSpecies(mut species: u16) -> u16 {
    if species >= NUM_SPECIES {
        species = SPECIES_NONE;
    }
    return species;
}
pub(crate) unsafe extern "C" fn SetMoveSpecificAnimData(contestant: u8) {
    let mut i: i32 = 0;
    let mut r#move: u16 = SanitizeMove((*(*gContestResources).status.at(contestant)).currMove);
    let mut species: u16 = SanitizeSpecies(gContestMons[contestant].species);
    let mut targetContestant: u8 = 0;
    memset(
        &raw mut (*(*gContestResources).moveAnim).species as *mut u8,
        0,
        20,
    );
    ClearBattleAnimationVars();
    i = 0;
    while i < CONTESTANT_COUNT {
        gBattleMonForms[i] = 0;
        i += 1;
    }
    match r#move {
        MOVE_CURSE => {
            if gSpeciesInfo[species].types[0] == TYPE_GHOST
                || gSpeciesInfo[species].types[1] == TYPE_GHOST
            {
                gAnimMoveTurn = 0;
            } else {
                gAnimMoveTurn = 1;
            }
        }
        MOVE_TRANSFORM | MOVE_ROLE_PLAY => {
            targetContestant = (*(*gContestResources).status.at(contestant)).contestantAnimTarget;
            (*(*gContestResources).moveAnim).targetSpecies =
                SanitizeSpecies(gContestMons[targetContestant].species);
            (*(*gContestResources).moveAnim).targetPersonality =
                gContestMons[targetContestant].personality;
            (*(*gContestResources).moveAnim).set_hasTargetAnim(TRUE);
        }
        MOVE_RETURN => {
            gAnimFriendship = MAX_FRIENDSHIP;
        }
        MOVE_FRUSTRATION => {
            gAnimFriendship = 0;
        }
        MOVE_SOLAR_BEAM | MOVE_RAZOR_WIND | MOVE_SKULL_BASH | MOVE_SKY_ATTACK => {
            if (*(*gContestResources).contest).moveAnimTurnCount == 0 {
                (*(*gContestResources).contest).moveAnimTurnCount = 2;
                gAnimMoveTurn = 0;
            } else {
                gAnimMoveTurn = 1;
            }
        }
        _ => {}
    }
    SetBattleTargetSpritePosition();
}
pub(crate) unsafe extern "C" fn ClearMoveAnimData(contestant: u8) {
    memset((*gContestResources).moveAnim as *mut u8, 0, 20);
    if (*(*gContestResources).contest).moveAnimTurnCount != 0 {
        (*(*gContestResources).contest).moveAnimTurnCount -= 1;
    }
}
pub(crate) unsafe extern "C" fn SetMoveAnimAttackerData(contestant: u8) {
    (*(*gContestResources).moveAnim).contestant = contestant;
    (*(*gContestResources).moveAnim).species = SanitizeSpecies(gContestMons[contestant].species);
    (*(*gContestResources).moveAnim).personality = gContestMons[contestant].personality;
    (*(*gContestResources).moveAnim).otId = gContestMons[contestant].otId;
}
pub(crate) unsafe extern "C" fn CreateInvisibleBattleTargetSprite() {
    gBattlerSpriteIds[3] = CreateInvisibleSpriteWithCallback(Some(SpriteCallbackDummy));
    InitSpriteAffineAnim(&raw mut gSprites[gBattlerSpriteIds[gBattlerTarget]]);
    SetBattleTargetSpritePosition();
}
pub(crate) unsafe extern "C" fn SetBattleTargetSpritePosition() {
    let mut sprite: *mut Sprite = &raw mut gSprites[gBattlerSpriteIds[3]];
    (*sprite).x2 = 0;
    (*sprite).y2 = 0;
    (*sprite).x = GetBattlerSpriteCoord(B_POSITION_OPPONENT_RIGHT, BATTLER_COORD_X) as i16;
    (*sprite).y = GetBattlerSpriteCoord(B_POSITION_OPPONENT_RIGHT, BATTLER_COORD_Y) as i16;
    (*sprite).set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn SetMoveTargetPosition(r#move: u16) {
    match gBattleMoves[r#move].target {
        MOVE_TARGET_USER_OR_SELECTED | MOVE_TARGET_USER => {
            gBattlerTarget = B_POSITION_PLAYER_RIGHT;
        }
        _ => {
            gBattlerTarget = B_POSITION_OPPONENT_RIGHT;
        }
    }
}
pub(crate) unsafe extern "C" fn Contest_PrintTextToBg0WindowStd(windowId: u32, b: *mut u8) {
    let mut printerTemplate: TextPrinterTemplate = zeroed();
    printerTemplate.currentChar = b;
    printerTemplate.windowId = windowId as u8;
    printerTemplate.fontId = FONT_NORMAL;
    printerTemplate.x = 0;
    printerTemplate.y = 1;
    printerTemplate.currentX = 0;
    printerTemplate.currentY = 1;
    printerTemplate.letterSpacing = 0;
    printerTemplate.lineSpacing = 0;
    printerTemplate.set_unk(0);
    printerTemplate.set_fgColor(15);
    printerTemplate.set_bgColor(0);
    printerTemplate.set_shadowColor(8);
    AddTextPrinter(&raw mut printerTemplate, 0, None);
    PutWindowTilemap(windowId as u8);
    Contest_SetBgCopyFlags(0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Contest_PrintTextToBg0WindowAt(
    windowId: u32,
    currChar: *mut u8,
    x: i32,
    y: i32,
    fontId: i32,
) {
    let mut printerTemplate: TextPrinterTemplate = zeroed();
    printerTemplate.currentChar = currChar;
    printerTemplate.windowId = windowId as u8;
    printerTemplate.fontId = fontId as u8;
    printerTemplate.x = x as u8;
    printerTemplate.y = y as u8;
    printerTemplate.currentX = x as u8;
    printerTemplate.currentY = y as u8;
    printerTemplate.letterSpacing = 0;
    printerTemplate.lineSpacing = 0;
    printerTemplate.set_unk(0);
    printerTemplate.set_fgColor(15);
    printerTemplate.set_bgColor(0);
    printerTemplate.set_shadowColor(8);
    AddTextPrinter(&raw mut printerTemplate, 0, None);
    PutWindowTilemap(windowId as u8);
    Contest_SetBgCopyFlags(0);
}
pub(crate) unsafe extern "C" fn Contest_StartTextPrinter(currChar: *mut u8, b: u32) {
    let mut printerTemplate: TextPrinterTemplate = zeroed();
    let mut speed: u8 = 0;
    printerTemplate.currentChar = currChar;
    printerTemplate.windowId = WIN_GENERAL_TEXT;
    printerTemplate.fontId = FONT_NORMAL;
    printerTemplate.x = 0;
    printerTemplate.y = 1;
    printerTemplate.currentX = 0;
    printerTemplate.currentY = 1;
    printerTemplate.letterSpacing = 0;
    printerTemplate.lineSpacing = 0;
    printerTemplate.set_unk(0);
    printerTemplate.set_fgColor(1);
    printerTemplate.set_bgColor(0);
    printerTemplate.set_shadowColor(8);
    if b == 0 {
        AddTextPrinter(&raw mut printerTemplate, 0, None);
    } else {
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
            speed = 4;
        } else {
            speed = GetPlayerTextSpeedDelay();
        }
        AddTextPrinter(&raw mut printerTemplate, speed, None);
    }
    PutWindowTilemap(WIN_GENERAL_TEXT);
    Contest_SetBgCopyFlags(0);
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
    Contest_SetBgCopyFlags(bg as u32);
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
    ContestBG_FillBoxWithIncrementingTile(bg, firstTileNum, x, y, width, height, paletteSlot, 0);
}
pub(crate) unsafe extern "C" fn Contest_RunTextPrinters() -> u32 {
    RunTextPrinters();
    return IsTextPrinterActive(WIN_GENERAL_TEXT) as u32;
}
pub(crate) unsafe extern "C" fn Contest_SetBgCopyFlags(flagIndex: u32) {
    sContestBgCopyFlags |= shl_i32(1, flagIndex) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetContestLinkResults() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < CONTEST_CATEGORIES_COUNT {
        j = 0;
        while j < CONTESTANT_COUNT {
            (*gSaveBlock2Ptr).contestLinkResults[i][j] = 0;
            j += 1;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveContestWinner(rank: u8) -> u8 {
    let mut i: i32 = 0;
    let mut captionId: u8 = (Random() as i32 % 3) as u8;
    i = 0;
    while i < 3 {
        if gContestFinalStandings[i] == 0 {
            break;
        }
        i += 1;
    }
    if rank == CONTEST_SAVE_FOR_MUSEUM as u8 && i != gContestPlayerMonIndex as i32 {
        return FALSE;
    }
    match gSpecialVar_ContestCategory {
        0 => {
            captionId += CONTEST_CATEGORY_COOL;
        }
        1 => {
            captionId += NUM_PAINTING_CAPTIONS;
        }
        2 => {
            captionId += 6;
        }
        3 => {
            captionId += 9;
        }
        4 => {
            captionId += 12;
        }
        _ => {}
    }
    if rank != CONTEST_SAVE_FOR_ARTIST as u8 {
        let mut id: u8 = GetContestWinnerSaveIdx(rank, TRUE);
        (*gSaveBlock1Ptr).contestWinners[id].personality = gContestMons[i].personality;
        (*gSaveBlock1Ptr).contestWinners[id].species = gContestMons[i].species;
        (*gSaveBlock1Ptr).contestWinners[id].trainerId = gContestMons[i].otId;
        StringCopy(
            (*gSaveBlock1Ptr).contestWinners[id].monName.as_mut_ptr(),
            gContestMons[i].nickname.as_mut_ptr(),
        );
        StringCopy(
            (*gSaveBlock1Ptr).contestWinners[id]
                .trainerName
                .as_mut_ptr(),
            gContestMons[i].trainerName.as_mut_ptr(),
        );
        if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_IS_LINK != 0 {
            (*gSaveBlock1Ptr).contestWinners[id].contestRank = CONTEST_RANK_LINK;
        } else {
            (*gSaveBlock1Ptr).contestWinners[id].contestRank = gSpecialVar_ContestRank as u8;
        }
        if rank != CONTEST_SAVE_FOR_MUSEUM as u8 {
            (*gSaveBlock1Ptr).contestWinners[id].contestCategory =
                gSpecialVar_ContestCategory as u8;
        } else {
            (*gSaveBlock1Ptr).contestWinners[id].contestCategory = captionId;
        }
    } else {
        gCurContestWinner.personality = gContestMons[i].personality;
        gCurContestWinner.trainerId = gContestMons[i].otId;
        gCurContestWinner.species = gContestMons[i].species;
        StringCopy(
            gCurContestWinner.monName.as_mut_ptr(),
            gContestMons[i].nickname.as_mut_ptr(),
        );
        StringCopy(
            gCurContestWinner.trainerName.as_mut_ptr(),
            gContestMons[i].trainerName.as_mut_ptr(),
        );
        gCurContestWinner.contestCategory = captionId;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestWinnerSaveIdx(rank: u8, shift: u8) -> u8 {
    let mut i: i32 = 0;
    match rank {
        CONTEST_RANK_NORMAL | CONTEST_RANK_SUPER | CONTEST_RANK_HYPER | CONTEST_RANK_MASTER => {
            if shift != 0 {
                i = 5;
                while i > 0 {
                    memcpy(
                        &raw mut (*gSaveBlock1Ptr).contestWinners[i] as *mut u8,
                        &raw mut (*gSaveBlock1Ptr).contestWinners[i - 1] as *mut u8,
                        32,
                    );
                    i -= 1;
                }
            }
            return 0;
        }
        _ => match gSpecialVar_ContestCategory {
            0 => {
                return 8;
            }
            1 => {
                return 9;
            }
            2 => {
                return 10;
            }
            3 => {
                return 11;
            }
            _ => {
                return 12;
            }
        },
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearContestWinnerPicsInContestHall() {
    let mut i: i32 = 0;
    i = 0;
    while i < MUSEUM_CONTEST_WINNERS_START as i32 {
        (*gSaveBlock1Ptr).contestWinners[i] = gDefaultContestWinners[i];
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetContestLiveUpdateFlags(contestant: u8) {
    let mut i: i32 = 0;
    if (*(*gContestResources).excitement).frozen() == 0
        && (*(*gContestResources).excitement).moveExcitement > 0
        && (*(*gContestResources).status.at(contestant)).repeatedMove() == 0
    {
        (*(*gContestResources).tv.at(contestant)).winnerFlags |= CONTESTLIVE_FLAG_EXCITING_APPEAL;
        (*(*gContestResources).tv.at(contestant)).set_madeExcitingAppeal(TRUE);
    }
    if (*(*gContestResources).status.at(contestant)).nervous() != 0 {
        (*(*gContestResources).tv.at(contestant)).winnerFlags |= CONTESTLIVE_FLAG_GOT_NERVOUS;
    }
    if (*(*gContestResources).excitement).frozen() == 0
        && (*(*gContestResources).excitement).moveExcitement != 0
        && (*(*gContestResources).excitement).excitementAppealBonus == 60
    {
        (*(*gContestResources).tv.at(contestant)).winnerFlags |= CONTESTLIVE_FLAG_MAXED_EXCITEMENT;
    }
    if (*(*gContestResources).status.at(contestant)).usedComboMove() != 0
        && (*(*gContestResources).status.at(contestant)).completedCombo != 0
    {
        (*(*gContestResources).tv.at(contestant)).winnerFlags |= CONTESTLIVE_FLAG_USED_COMBO;
    }
    i = 0;
    while i < CONTESTANT_COUNT {
        if i != contestant as i32 && (*(*gContestResources).status.at(i)).jam != 0 {
            (*(*gContestResources).tv.at(contestant)).winnerFlags |=
                CONTESTLIVE_FLAG_STARTLED_OTHER;
            (*(*gContestResources).tv.at(i)).winnerFlags |= CONTESTLIVE_FLAG_GOT_STARTLED;
        }
        i += 1;
    }
    if (*(*gContestResources).status.at(contestant)).numTurnsSkipped() != 0
        || (*(*gContestResources).status.at(contestant)).noMoreTurns() != 0
    {
        (*(*gContestResources).tv.at(contestant)).winnerFlags |= CONTESTLIVE_FLAG_SKIPPED_TURN;
    } else if (*(*gContestResources).status.at(contestant)).nervous() == 0 {
        (*(*gContestResources).tv.at(contestant)).winnerFlags |= CONTESTLIVE_FLAG_MADE_APPEAL;
        (*(*gContestResources).tv.at(contestant)).set_madeAppeal(TRUE);
        (*(*gContestResources).tv.at(contestant)).appeals
            [(*(*gContestResources).contest).appealNumber] =
            (*(*gContestResources).status.at(contestant)).currMove;
    }
    if (*(*gContestResources).status.at(contestant)).repeatedMove() != 0 {
        (*(*gContestResources).tv.at(contestant)).loserFlags |= CONTESTLIVE_FLAG_REPEATED_MOVE;
    }
    if (*(*gContestResources).contest).applauseLevel == 4
        && (*(*gContestResources).excitement).frozen() == 0
        && (*(*gContestResources).excitement).moveExcitement < 0
    {
        (*(*gContestResources).tv.at(contestant)).loserFlags |= CONTESTLIVE_FLAG_MISSED_EXCITEMENT;
    }
}
pub(crate) unsafe extern "C" fn CalculateContestLiveUpdateData() {
    let mut loser: u8 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut notLastInRound1: u32 = 0;
    let mut notLastInRound2: u32 = 0;
    let mut appealMoves: CArray<u16, 6> = zeroed();
    let mut numMoveUses: CArray<u8, 6> = zeroed();
    let mut moveCandidates: CArray<u16, 5> = zeroed();
    let mut winner: u8 = 0;
    let mut mostUses: u8 = 0;
    let mut numMoveCandidates: u8 = 0;
    loser = 0;
    winner = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        if gContestFinalStandings[i] == 0 {
            winner = i as u8;
        } else if gContestFinalStandings[i] == 3 {
            loser = i as u8;
        }
        i += 1;
    }
    (*(*gContestResources).tv.at(loser)).loserFlags |= CONTESTLIVE_FLAG_LOST;
    i = 0;
    while i < CONTESTANT_COUNT {
        if i != winner as i32
            && gContestMonTotalPoints[winner] as i32 - gContestMonTotalPoints[i] as i32 <= 50
        {
            (*(*gContestResources).tv.at(i)).loserFlags |= CONTESTLIVE_FLAG_LOST_SMALL_MARGIN;
        }
        if (*(*gContestResources).tv.at(i)).madeExcitingAppeal() == 0 {
            (*(*gContestResources).tv.at(i)).loserFlags |= CONTESTLIVE_FLAG_NO_EXCITEMENT;
        }
        j = 0;
        while j < CONTESTANT_COUNT {
            if gContestMonRound1Points[i] < gContestMonRound1Points[j] {
                break;
            }
            j += 1;
        }
        if j == CONTESTANT_COUNT && gContestFinalStandings[i] != 0 {
            (*(*gContestResources).tv.at(i)).loserFlags |= CONTESTLIVE_FLAG_BLEW_LEAD;
        }
        notLastInRound1 = FALSE as u32;
        notLastInRound2 = FALSE as u32;
        j = 0;
        while j < CONTESTANT_COUNT {
            if gContestMonRound1Points[i] > gContestMonRound1Points[j] {
                notLastInRound1 = TRUE as u32;
            }
            if gContestMonRound2Points[i] > gContestMonRound2Points[j] {
                notLastInRound2 = TRUE as u32;
            }
            j += 1;
        }
        if notLastInRound1 == 0 && notLastInRound2 == 0 {
            (*(*gContestResources).tv.at(i)).loserFlags |= CONTESTLIVE_FLAG_LAST_BOTH_ROUNDS;
        }
        if (*(*gContestResources).tv.at(i)).madeAppeal() == 0 {
            (*(*gContestResources).tv.at(i)).loserFlags |= CONTESTLIVE_FLAG_NO_APPEALS;
        }
        i += 1;
    }
    i = 0;
    while i < CONTEST_NUM_APPEALS {
        appealMoves[i] = MOVE_NONE;
        numMoveUses[i] = 0;
        i += 1;
    }
    appealMoves[5] = APPEAL_MOVES_END;
    numMoveUses[5] = 0;
    i = 0;
    while i < CONTEST_NUM_APPEALS {
        if (*(*gContestResources).tv.at(winner)).appeals[i] != MOVE_NONE {
            j = 0;
            while j < CONTEST_NUM_APPEALS {
                if (*(*gContestResources).tv.at(winner)).appeals[i] != appealMoves[j] {
                    if appealMoves[j] == MOVE_NONE {
                        appealMoves[j] = (*(*gContestResources).tv.at(winner)).appeals[i];
                        numMoveUses[j] += 1;
                    }
                } else {
                    numMoveUses[j] += 1;
                }
                j += 1;
            }
        }
        i += 1;
    }
    moveCandidates[0] = appealMoves[0];
    mostUses = numMoveUses[0];
    numMoveCandidates = 0;
    i = 1;
    while appealMoves[i] != APPEAL_MOVES_END {
        if mostUses < numMoveUses[i] {
            moveCandidates[0] = appealMoves[i];
            mostUses = numMoveUses[i];
            numMoveCandidates = 1;
        } else if mostUses == numMoveUses[i] {
            moveCandidates[numMoveCandidates] = appealMoves[i];
            numMoveCandidates += 1;
        }
        i += 1;
    }
    (*(*gContestResources).tv.at(winner)).r#move =
        moveCandidates[rem_i32(Random() as i32, numMoveCandidates as i32)] as i16;
}
pub(crate) unsafe extern "C" fn SetConestLiveUpdateTVData() {
    let mut i: i32 = 0;
    let mut flags: u32 = 0;
    let mut winner: u8 = 0;
    let mut round1Placing: u8 = 0;
    let mut round2Placing: u8 = 0;
    let mut count: u8 = 0;
    let mut randAction: u8 = 0;
    let mut numLoserCandidates: u8 = 0;
    let mut flagId: u8 = 0;
    let mut winnerFlag: u16 = 0;
    let mut loserFlag: u8 = 0;
    let mut loser: u8 = 0;
    let mut loserCandidates: CArray<u8, 3> = zeroed();
    if gContestFinalStandings[gContestPlayerMonIndex] != 0 {
        return;
    }
    winner = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        if gContestFinalStandings[i] == 0 {
            winner = i as u8;
        }
        i += 1;
    }
    round1Placing = 0;
    round2Placing = 0;
    i = 0;
    while i < CONTESTANT_COUNT {
        if gContestMonRound1Points[winner] < gContestMonRound1Points[i] {
            round1Placing += 1;
        }
        if gContestMonRound2Points[winner] < gContestMonRound2Points[i] {
            round2Placing += 1;
        }
        i += 1;
    }
    flags = (*(*gContestResources).tv.at(winner)).winnerFlags as u32;
    count = 0;
    i = 0;
    while i < 8 {
        if flags & 1 != 0 {
            count += 1;
        }
        flags >>= 1;
        i += 1;
    }
    randAction = rem_i32(Random() as i32, count as i32) as u8;
    flags = (*(*gContestResources).tv.at(winner)).winnerFlags as u32;
    count = 0;
    flagId = 0;
    i = 0;
    'l5: while i < 8 {
        'l4: {
            if flags & 1 == 0 {
                break 'l4;
            }
            if randAction == count {
                break 'l5;
            }
            count += 1;
        }
        flags >>= 1;
        flagId += 1;
        i += 1;
    }
    winnerFlag = shl_i32(1, flagId as u32) as u16;
    if winner == 0 {
        loserCandidates[0] = 1;
        loserFlag = (*(*gContestResources).tv.at(1)).loserFlags;
        i = 2;
    } else {
        loserCandidates[0] = 0;
        loserFlag = (*(*gContestResources).tv).loserFlags;
        i = 1;
    }
    numLoserCandidates = 1;
    while i < CONTESTANT_COUNT {
        if i != winner as i32 {
            if loserFlag < (*(*gContestResources).tv.at(i)).loserFlags {
                loserCandidates[0] = i as u8;
                loserFlag = (*(*gContestResources).tv.at(i)).loserFlags;
                numLoserCandidates = 1;
            } else if loserFlag == (*(*gContestResources).tv.at(i)).loserFlags {
                loserCandidates[numLoserCandidates] = i as u8;
                numLoserCandidates += 1;
            }
        }
        i += 1;
    }
    loser = loserCandidates[rem_i32(Random() as i32, numLoserCandidates as i32)];
    flagId = CONTESTLIVE_FLAG_NO_APPEALS;
    i = 0;
    while i < 8 {
        loserFlag = (*(*gContestResources).tv.at(loser)).loserFlags & flagId;
        if loserFlag != 0 {
            break;
        }
        flagId >>= 1;
        i += 1;
    }
    ContestLiveUpdates_Init(round1Placing);
    ContestLiveUpdates_SetRound2Placing(round2Placing);
    ContestLiveUpdates_SetWinnerAppealFlag(winnerFlag as u8);
    ContestLiveUpdates_SetWinnerMoveUsed((*(*gContestResources).tv.at(winner)).r#move as u16);
    ContestLiveUpdates_SetLoserData(loserFlag, loser);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestDebugToggleBitfields(loserFlags: u8) {
    if gHeap[0x1a000] == CONTEST_DEBUG_MODE_OFF {
        if loserFlags == 0 {
            gHeap[0x1a000] = CONTEST_DEBUG_MODE_PRINT_WINNER_FLAGS;
        } else {
            gHeap[0x1a000] = CONTEST_DEBUG_MODE_PRINT_LOSER_FLAGS;
        }
    } else {
        gHeap[0x1a000] = CONTEST_DEBUG_MODE_OFF;
    }
    if gHeap[0x1a000] == CONTEST_DEBUG_MODE_OFF {
        DrawContestantWindowText();
        SwapMoveDescAndContestTilemaps();
    } else {
        ContestDebugPrintBitStrings();
    }
}
pub(crate) unsafe extern "C" fn ContestDebugPrintBitStrings() {
    let mut i: u8 = 0;
    let mut j: i8 = 0;
    let mut text1: CArray<u8, 20> = zeroed();
    let mut text2: CArray<u8, 20> = zeroed();
    let mut txtPtr: *mut u8 = null_mut();
    let mut bits: u32 = 0;
    if gEnableContestDebugging == 0 {
        return;
    }
    if gHeap[0x1a000] != CONTEST_DEBUG_MODE_PRINT_WINNER_FLAGS
        && gHeap[0x1a000] != CONTEST_DEBUG_MODE_PRINT_LOSER_FLAGS
    {
        return;
    }
    i = 0;
    while i < CONTESTANT_COUNT as u8 {
        FillWindowPixelBuffer(i, 0);
        i += 1;
    }
    if gHeap[0x1a000] == CONTEST_DEBUG_MODE_PRINT_WINNER_FLAGS {
        i = 0;
        while i < CONTESTANT_COUNT as u8 {
            txtPtr = StringCopy(text1.as_mut_ptr(), gText_CDot.as_ptr().cast_mut());
            Contest_PrintTextToBg0WindowAt(
                gContestantTurnOrder[i] as u32,
                text1.as_mut_ptr(),
                5,
                1,
                FONT_NARROW as i32,
            );
            bits = (*(*gContestResources).tv.at(i)).winnerFlags as u32;
            j = 7;
            while j > -1 {
                txtPtr = ConvertIntToDecimalStringN(
                    txtPtr,
                    bits as i32 & 1,
                    STR_CONV_MODE_LEFT_ALIGN,
                    1,
                );
                bits >>= 1;
                j -= 1;
            }
            j = 0;
            while j < 5 {
                text2[j] = text1[j];
                j += 1;
            }
            text2[j] = EOS;
            Contest_PrintTextToBg0WindowAt(
                gContestantTurnOrder[i] as u32,
                text2.as_mut_ptr(),
                5,
                1,
                7,
            );
            Contest_PrintTextToBg0WindowAt(
                gContestantTurnOrder[i] as u32,
                text1.as_mut_ptr().at(j),
                55,
                1,
                FONT_NARROW as i32,
            );
            i += 1;
        }
    } else {
        i = 0;
        while i < CONTESTANT_COUNT as u8 {
            StringCopy(text1.as_mut_ptr(), gText_BDot.as_ptr().cast_mut());
            bits = (*(*gContestResources).tv.at(i)).loserFlags as u32;
            txtPtr = &raw mut text1[2];
            j = 7;
            while j > -1 {
                txtPtr = ConvertIntToDecimalStringN(
                    txtPtr,
                    bits as i32 & 1,
                    STR_CONV_MODE_LEFT_ALIGN,
                    1,
                );
                bits >>= 1;
                j -= 1;
            }
            j = 0;
            while j < 5 {
                text2[j] = text1[j];
                j += 1;
            }
            text2[j] = EOS;
            Contest_PrintTextToBg0WindowAt(
                gContestantTurnOrder[i] as u32,
                text2.as_mut_ptr(),
                5,
                1,
                FONT_NARROW as i32,
            );
            Contest_PrintTextToBg0WindowAt(
                gContestantTurnOrder[i] as u32,
                text1.as_mut_ptr().at(j),
                55,
                1,
                FONT_NARROW as i32,
            );
            i += 1;
        }
    }
    SwapMoveDescAndContestTilemaps();
}
pub(crate) unsafe extern "C" fn GetMonNicknameLanguage(mut nickname: *mut u8) -> u8 {
    let mut ret: u8 = GAME_LANGUAGE;
    if *nickname == EXT_CTRL_CODE_BEGIN && *nickname.at(1) == EXT_CTRL_CODE_JPN {
        return GAME_LANGUAGE;
    }
    if StringLength(nickname) <= 5 {
        while *nickname != EOS {
            if *nickname >= CHAR_A && *nickname <= CHAR_z
                || *nickname >= CHAR_0 && *nickname <= CHAR_9
                || *nickname == CHAR_SPACE
                || *nickname == CHAR_PERIOD
                || *nickname == CHAR_COMMA
                || *nickname == CHAR_EXCL_MARK
                || *nickname == CHAR_QUESTION_MARK
                || *nickname == CHAR_MALE
                || *nickname == CHAR_FEMALE
                || *nickname == CHAR_SLASH
                || *nickname == CHAR_HYPHEN
                || *nickname == CHAR_ELLIPSIS
                || *nickname == CHAR_DBL_QUOTE_LEFT
                || *nickname == CHAR_DBL_QUOTE_RIGHT
                || *nickname == CHAR_SGL_QUOTE_LEFT
                || *nickname == CHAR_DBL_QUOTE_LEFT
            {
                nickname = nickname.at(1);
            } else {
                ret = LANGUAGE_JAPANESE;
                break;
            }
        }
    }
    return ret;
}
pub(crate) unsafe extern "C" fn StripPlayerNameForLinkContest(mut playerName: *mut u8) {
    let mut chr: u8 = *playerName.at(5);
    *playerName.at(5) = EOS;
    *playerName.at(7) = chr;
}
pub(crate) unsafe extern "C" fn StripMonNameForLinkContest(mut monName: *mut u8, language: i32) {
    let mut chr: u8 = 0;
    StripExtCtrlCodes(monName);
    if language == LANGUAGE_JAPANESE as i32 {
        *monName.at(5) = EOS;
        *monName.at(10) = EXT_CTRL_CODE_BEGIN;
    } else {
        chr = *monName.at(5);
        *monName.at(5) = EOS;
        *monName.at(10) = chr;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StripPlayerAndMonNamesForLinkContest(
    mon: *mut ContestPokemon,
    language: i32,
) {
    let mut name: *mut u8 = (*mon).nickname.as_mut_ptr();
    if language == LANGUAGE_JAPANESE as i32 {
        ConvertInternationalString(name, GetMonNicknameLanguage(name));
    } else if *name.at(10) == EXT_CTRL_CODE_BEGIN {
        ConvertInternationalString(name, LANGUAGE_JAPANESE);
    } else {
        *name.at(5) = *name.at(10);
        *name.at(10) = EOS;
    }
    name = (*mon).trainerName.as_mut_ptr();
    if language == LANGUAGE_JAPANESE as i32 {
        *name.at(7) = EOS;
        *name.at(6) = *name.at(4);
        *name.at(5) = *name.at(3);
        *name.at(4) = *name.at(2);
        *name.at(3) = *name.at(1);
        *name.at(2) = (*mon).trainerName[0];
        *name.at(1) = EXT_CTRL_CODE_JPN;
        *name = EXT_CTRL_CODE_BEGIN;
    } else {
        *name.at(5) = *name.at(7);
        *name.at(7) = EOS;
    }
}
pub(crate) unsafe extern "C" fn SetBackdropFromColor(color: u16) {
    FillPalette(color, 0, 2);
}
