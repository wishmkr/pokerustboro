//! Translated from `src/berry_blender.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sBlenderCenter_Pal sBlenderCenter_Tilemap sBlenderOuter_Pal sUnused_Pal sEmpty_Pal sUnusedText_YesNo sUnusedText_2 sUnusedText_Space sUnusedText_Terminating sUnusedText_LinkPartnerNotFound sText_BerryBlenderStart sText_NewParagraph sText_WasMade sText_Mister sText_Laddie sText_Lassie sText_Master sText_Dude sText_Miss sBlenderOpponentsNames sText_PressAToStart sText_PleaseWaitAWhile sText_CommunicationStandby sText_WouldLikeToBlendAnotherBerry sText_RunOutOfBerriesForBlending sText_YourPokeblockCaseIsFull sText_HasNoBerriesToPut sText_ApostropheSPokeblockCaseIsFull sText_BlendingResults sText_BerryUsed sText_SpaceBerry sText_Time sText_Min sText_Sec sText_MaximumSpeed sText_RPM sText_Dot sText_NewLine sText_Space sText_Ranking sText_TheLevelIs sText_TheFeelIs sText_Dot2 sBgTemplates sWindowTemplates sYesNoWindowTemplate_ContinuePlaying sPlayerArrowQuadrant sPlayerArrowPos sPlayerIdMap sArrowStartPos sArrowStartPosIds sArrowHitRangeStart sLocalOpponentTasks sOam_PlayerArrow sAnim_PlayerArrow_TopLeft sAnim_PlayerArrow_TopRight sAnim_PlayerArrow_BottomLeft sAnim_PlayerArrow_BottomRight sAnim_PlayerArrow_TopLeft_Flash sAnim_PlayerArrow_TopRight_Flash sAnim_PlayerArrow_BottomLeft_Flash sAnim_PlayerArrow_BottomRight_Flash sAnim_PlayerArrow_TopLeft_Off sAnim_PlayerArrow_TopRight_Off sAnim_PlayerArrow_BottomLeft_Off sAnim_PlayerArrow_BottomRight_Off sAnims_PlayerArrow sSpriteSheet_PlayerArrow sSpritePal_BlenderMisc sSpritePal_PlayerArrow sSpriteTemplate_PlayerArrow sOam_ScoreSymbols sAnim_ScoreSymbols_Good sAnim_ScoreSymbols_Miss sAnim_ScoreSymbols_BestFlash sAnim_ScoreSymbols_BestStatic sAnims_ScoreSymbols sSpriteSheet_ScoreSymbols sSpriteTemplate_ScoreSymbols sOam_Particles sAnim_SparkleCrossToX sAnim_SparkleXToCross sAnim_SparkleFull sAnim_GreenArrow sAnim_GreenDot sAnims_Particles sSpriteSheet_Particles sSpriteTemplate_Particles sOam_CountdownNumbers sAnim_CountdownNumbers_3 sAnim_CountdownNumbers_2 sAnim_CountdownNumbers_1 sAnims_CountdownNumbers sSpriteSheet_CountdownNumbers sSpriteTemplate_CountdownNumbers sOam_Start sAnim_Start sAnims_Start sSpriteSheet_Start sSpriteTemplate_Start sBerrySpriteData sOpponentBerrySets sBerryMasterBerries sNumPlayersToSpeedDivisor sBlackPokeblockFlavorFlags sJPText_GoodTvReady sJPText_BadTvReady sJPText_Flavors sUnused sBlenderRecordWindowTemplate

/// `struct BerryBlender`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BerryBlender {
    pub mainState: u8,
    pub loadGfxState: u8,
    pub unused0: CArray<u8, 66>,
    pub unk0: u16,
    pub scoreIconIds: CArray<u8, 3>,
    pub arrowPos: u16,
    pub speed: i16,
    pub maxRPM: u16,
    pub playerArrowSpriteIds: CArray<u8, 4>,
    pub playerArrowSpriteIds2: CArray<u8, 4>,
    pub unused1: CArray<u8, 11>,
    pub gameEndState: u8,
    pub playerContinueResponses: CArray<u16, 4>,
    pub canceledPlayerCmd: u16,
    pub canceledPlayerId: u16,
    pub playAgainState: u16,
    pub slowdownTimer: u8,
    pub chosenItemId: CArray<u16, 4>,
    pub numPlayers: u8,
    pub unused2: CArray<u8, 16>,
    pub arrowIdToPlayerId: CArray<u16, 4>,
    pub playerIdToArrowId: CArray<u16, 4>,
    pub yesNoAnswer: u8,
    pub stringVar: CArray<u8, 100>,
    pub gameFrameTime: u32,
    pub framesToWait: i32,
    pub unk1: u32,
    pub unused3: CArray<u8, 4>,
    pub playerToThrowBerry: u8,
    pub progressBarValue: u16,
    pub maxProgressBarValue: u16,
    pub centerScale: u16,
    pub bg_X: u16,
    pub bg_Y: u16,
    pub opponentTaskIds: CArray<u8, 3>,
    pub perfectOpponents: u8,
    pub scores: CArray<CArray<u16, 3>, 4>,
    pub playerPlaces: CArray<u8, 4>,
    pub bgAffineSrc: BgAffineSrcData,
    pub savedMusic: u16,
    pub blendedBerries: CArray<BlenderBerry, 4>,
    pub smallBlock: TimeAndRPM,
    pub linkPlayAgainState: u32,
    pub ownRanking: u8,
    pub tvBlender: TvBlenderStruct,
    pub tilemapBuffers: CArray<CArray<u8, 2048>, 2>,
    pub textState: i16,
    pub tilesBuffer: *mut core::ffi::c_void,
    pub gameBlock: BlenderGameBlock,
}

unsafe impl Sync for BerryBlender {}

/// `struct BlenderBerry`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct BlenderBerry {
    pub itemId: u16,
    pub name: CArray<u8, 7>,
    pub flavors: CArray<u8, 6>,
}

unsafe impl Sync for BlenderBerry {}

/// `struct BlenderGameBlock`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BlenderGameBlock {
    pub timeRPM: TimeAndRPM,
    pub scores: CArray<CArray<u16, 3>, 4>,
}

unsafe impl Sync for BlenderGameBlock {}

/// `struct TimeAndRPM`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TimeAndRPM {
    pub time: u32,
    pub maxRPM: u16,
}

unsafe impl Sync for TimeAndRPM {}

/// `struct TvBlenderStruct`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct TvBlenderStruct {
    pub name: CArray<u8, 11>,
    pub pokeblockFlavor: u8,
    pub pokeblockColor: u8,
    pub pokeblockSheen: u8,
}

unsafe impl Sync for TvBlenderStruct {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<BerryBlender>() == 4576);
    assert!(offset_of!(BerryBlender, mainState) == 0);
    assert!(offset_of!(BerryBlender, loadGfxState) == 1);
    assert!(offset_of!(BerryBlender, unused0) == 2);
    assert!(offset_of!(BerryBlender, unk0) == 68);
    assert!(offset_of!(BerryBlender, scoreIconIds) == 70);
    assert!(offset_of!(BerryBlender, arrowPos) == 74);
    assert!(offset_of!(BerryBlender, speed) == 76);
    assert!(offset_of!(BerryBlender, maxRPM) == 78);
    assert!(offset_of!(BerryBlender, playerArrowSpriteIds) == 80);
    assert!(offset_of!(BerryBlender, playerArrowSpriteIds2) == 84);
    assert!(offset_of!(BerryBlender, unused1) == 88);
    assert!(offset_of!(BerryBlender, gameEndState) == 99);
    assert!(offset_of!(BerryBlender, playerContinueResponses) == 100);
    assert!(offset_of!(BerryBlender, canceledPlayerCmd) == 108);
    assert!(offset_of!(BerryBlender, canceledPlayerId) == 110);
    assert!(offset_of!(BerryBlender, playAgainState) == 112);
    assert!(offset_of!(BerryBlender, slowdownTimer) == 114);
    assert!(offset_of!(BerryBlender, chosenItemId) == 116);
    assert!(offset_of!(BerryBlender, numPlayers) == 124);
    assert!(offset_of!(BerryBlender, unused2) == 125);
    assert!(offset_of!(BerryBlender, arrowIdToPlayerId) == 142);
    assert!(offset_of!(BerryBlender, playerIdToArrowId) == 150);
    assert!(offset_of!(BerryBlender, yesNoAnswer) == 158);
    assert!(offset_of!(BerryBlender, stringVar) == 159);
    assert!(offset_of!(BerryBlender, gameFrameTime) == 260);
    assert!(offset_of!(BerryBlender, framesToWait) == 264);
    assert!(offset_of!(BerryBlender, unk1) == 268);
    assert!(offset_of!(BerryBlender, unused3) == 272);
    assert!(offset_of!(BerryBlender, playerToThrowBerry) == 276);
    assert!(offset_of!(BerryBlender, progressBarValue) == 278);
    assert!(offset_of!(BerryBlender, maxProgressBarValue) == 280);
    assert!(offset_of!(BerryBlender, centerScale) == 282);
    assert!(offset_of!(BerryBlender, bg_X) == 284);
    assert!(offset_of!(BerryBlender, bg_Y) == 286);
    assert!(offset_of!(BerryBlender, opponentTaskIds) == 288);
    assert!(offset_of!(BerryBlender, perfectOpponents) == 291);
    assert!(offset_of!(BerryBlender, scores) == 292);
    assert!(offset_of!(BerryBlender, playerPlaces) == 316);
    assert!(offset_of!(BerryBlender, bgAffineSrc) == 320);
    assert!(offset_of!(BerryBlender, savedMusic) == 340);
    assert!(offset_of!(BerryBlender, blendedBerries) == 344);
    assert!(offset_of!(BerryBlender, smallBlock) == 408);
    assert!(offset_of!(BerryBlender, linkPlayAgainState) == 416);
    assert!(offset_of!(BerryBlender, ownRanking) == 420);
    assert!(offset_of!(BerryBlender, tvBlender) == 424);
    assert!(offset_of!(BerryBlender, tilemapBuffers) == 440);
    assert!(offset_of!(BerryBlender, textState) == 4536);
    assert!(offset_of!(BerryBlender, tilesBuffer) == 4540);
    assert!(offset_of!(BerryBlender, gameBlock) == 4544);
    assert!(size_of::<BlenderBerry>() == 16);
    assert!(offset_of!(BlenderBerry, itemId) == 0);
    assert!(offset_of!(BlenderBerry, name) == 2);
    assert!(offset_of!(BlenderBerry, flavors) == 9);
    assert!(size_of::<BlenderGameBlock>() == 32);
    assert!(offset_of!(BlenderGameBlock, timeRPM) == 0);
    assert!(offset_of!(BlenderGameBlock, scores) == 8);
    assert!(size_of::<TimeAndRPM>() == 8);
    assert!(offset_of!(TimeAndRPM, time) == 0);
    assert!(offset_of!(TimeAndRPM, maxRPM) == 4);
    assert!(size_of::<TvBlenderStruct>() == 16);
    assert!(offset_of!(TvBlenderStruct, name) == 0);
    assert!(offset_of!(TvBlenderStruct, pokeblockFlavor) == 11);
    assert!(offset_of!(TvBlenderStruct, pokeblockColor) == 12);
    assert!(offset_of!(TvBlenderStruct, pokeblockSheen) == 13);
};

const ARROW_FALL_ROTATION: u16 = 22528;
const BLENDER_DUDE: i32 = 4;
const BLENDER_LADDIE: i32 = 1;
const BLENDER_LASSIE: i32 = 2;
const BLENDER_MASTER: i32 = 3;
const BLENDER_MAX_PLAYERS: i32 = 4;
const BLENDER_MISS: i32 = 5;
const BLENDER_MISTER: i32 = 0;
const CANT_PLAY_NO_BERRIES: u16 = 2;
const CANT_PLAY_NO_PKBLCK_SPACE: u16 = 3;
const MAX_ARROW_POS: i32 = 0x10000;
const MAX_PROGRESS_BAR: u16 = 1000;
const MIN_ARROW_SPEED: i16 = 128;
const NO_PLAYER: u16 = 255;
const NUM_SCORE_TYPES: i32 = 3;
const PLAY_AGAIN_NO: u16 = 1;
const PLAY_AGAIN_YES: u16 = 0;
const PROGRESS_BAR_EMPTY_BOTTOM: u16 = 33009;
const PROGRESS_BAR_EMPTY_TOP: u16 = 32993;
const PROGRESS_BAR_FILLED_BOTTOM: u16 = 33017;
const PROGRESS_BAR_FILLED_TOP: u16 = 33001;
const PROXIMITY_BEST: u8 = 2;
const PROXIMITY_GOOD: u8 = 1;
const PROXIMITY_MISS: u8 = 0;
const RPM_DIGIT: u16 = 32882;
const SCOREANIM_BEST_FLASH: u8 = 2;
const SCOREANIM_BEST_STATIC: u8 = 3;
const SCOREANIM_GOOD: u8 = 0;
const SCOREANIM_MISS: u8 = 1;
const SCORE_BEST: i32 = 0;
const SCORE_GOOD: i32 = 1;
const SCORE_MISS: i32 = 2;
const WIN_MSG: u8 = 4;
const WIN_RESULTS: u8 = 5;

static sArrowHitRangeStart: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::berry_blender::sArrowHitRangeStart).cast());
static sArrowStartPos: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::berry_blender::sArrowStartPos).cast());
static sArrowStartPosIds: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::berry_blender::sArrowStartPosIds).cast());
static sBerryMasterBerries: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::berry_blender::sBerryMasterBerries).cast());
static sBerrySpriteData: Table<CArray<CArray<i16, 5>, 4>> =
    Table((&raw const crate::data::berry_blender::sBerrySpriteData).cast());
static sBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::berry_blender::sBgTemplates).cast());
static sBlackPokeblockFlavorFlags: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::berry_blender::sBlackPokeblockFlavorFlags).cast());
static sBlenderCenter_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::berry_blender::sBlenderCenter_Pal).cast());
static sBlenderCenter_Tilemap: Table<CArray<u8, 1024>> =
    Table((&raw const crate::data::berry_blender::sBlenderCenter_Tilemap).cast());
static sBlenderOpponentsNames: Table<CArray<*mut u8, 6>> =
    Table((&raw const crate::data::berry_blender::sBlenderOpponentsNames).cast());
static sBlenderOuter_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::berry_blender::sBlenderOuter_Pal).cast());
static sBlenderRecordWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::berry_blender::sBlenderRecordWindowTemplate).cast());
static sLocalOpponentTasks: Table<CArray<Option<unsafe extern "C" fn(u8)>, 3>> =
    Table((&raw const crate::data::berry_blender::sLocalOpponentTasks).cast());
static sNumPlayersToSpeedDivisor: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::berry_blender::sNumPlayersToSpeedDivisor).cast());
static sOpponentBerrySets: Table<CArray<CArray<u8, 3>, 10>> =
    Table((&raw const crate::data::berry_blender::sOpponentBerrySets).cast());
static sPlayerArrowPos: Table<CArray<CArray<u8, 2>, 4>> =
    Table((&raw const crate::data::berry_blender::sPlayerArrowPos).cast());
static sPlayerArrowQuadrant: Table<CArray<CArray<i8, 2>, 4>> =
    Table((&raw const crate::data::berry_blender::sPlayerArrowQuadrant).cast());
static sPlayerIdMap: Table<CArray<CArray<u8, 4>, 3>> =
    Table((&raw const crate::data::berry_blender::sPlayerIdMap).cast());
static sSpritePal_BlenderMisc: Table<SpritePalette> =
    Table((&raw const crate::data::berry_blender::sSpritePal_BlenderMisc).cast());
static sSpritePal_PlayerArrow: Table<SpritePalette> =
    Table((&raw const crate::data::berry_blender::sSpritePal_PlayerArrow).cast());
static sSpriteSheet_CountdownNumbers: Table<SpriteSheet> =
    Table((&raw const crate::data::berry_blender::sSpriteSheet_CountdownNumbers).cast());
static sSpriteSheet_Particles: Table<SpriteSheet> =
    Table((&raw const crate::data::berry_blender::sSpriteSheet_Particles).cast());
static sSpriteSheet_PlayerArrow: Table<SpriteSheet> =
    Table((&raw const crate::data::berry_blender::sSpriteSheet_PlayerArrow).cast());
static sSpriteSheet_ScoreSymbols: Table<SpriteSheet> =
    Table((&raw const crate::data::berry_blender::sSpriteSheet_ScoreSymbols).cast());
static sSpriteSheet_Start: Table<SpriteSheet> =
    Table((&raw const crate::data::berry_blender::sSpriteSheet_Start).cast());
static sSpriteTemplate_CountdownNumbers: Table<SpriteTemplate> =
    Table((&raw const crate::data::berry_blender::sSpriteTemplate_CountdownNumbers).cast());
static sSpriteTemplate_Particles: Table<SpriteTemplate> =
    Table((&raw const crate::data::berry_blender::sSpriteTemplate_Particles).cast());
static sSpriteTemplate_PlayerArrow: Table<SpriteTemplate> =
    Table((&raw const crate::data::berry_blender::sSpriteTemplate_PlayerArrow).cast());
static sSpriteTemplate_ScoreSymbols: Table<SpriteTemplate> =
    Table((&raw const crate::data::berry_blender::sSpriteTemplate_ScoreSymbols).cast());
static sSpriteTemplate_Start: Table<SpriteTemplate> =
    Table((&raw const crate::data::berry_blender::sSpriteTemplate_Start).cast());
static sText_ApostropheSPokeblockCaseIsFull: Table<CArray<u8, 24>> =
    Table((&raw const crate::data::berry_blender::sText_ApostropheSPokeblockCaseIsFull).cast());
static sText_BerryBlenderStart: Table<CArray<u8, 97>> =
    Table((&raw const crate::data::berry_blender::sText_BerryBlenderStart).cast());
static sText_BlendingResults: Table<CArray<u8, 20>> =
    Table((&raw const crate::data::berry_blender::sText_BlendingResults).cast());
static sText_CommunicationStandby: Table<CArray<u8, 23>> =
    Table((&raw const crate::data::berry_blender::sText_CommunicationStandby).cast());
static sText_Dot: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::berry_blender::sText_Dot).cast());
static sText_Dot2: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::berry_blender::sText_Dot2).cast());
static sText_HasNoBerriesToPut: Table<CArray<u8, 45>> =
    Table((&raw const crate::data::berry_blender::sText_HasNoBerriesToPut).cast());
static sText_MaximumSpeed: Table<CArray<u8, 14>> =
    Table((&raw const crate::data::berry_blender::sText_MaximumSpeed).cast());
static sText_Min: Table<CArray<u8, 7>> =
    Table((&raw const crate::data::berry_blender::sText_Min).cast());
static sText_NewLine: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::berry_blender::sText_NewLine).cast());
static sText_NewParagraph: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::berry_blender::sText_NewParagraph).cast());
static sText_RPM: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::berry_blender::sText_RPM).cast());
static sText_Ranking: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::berry_blender::sText_Ranking).cast());
static sText_RunOutOfBerriesForBlending: Table<CArray<u8, 62>> =
    Table((&raw const crate::data::berry_blender::sText_RunOutOfBerriesForBlending).cast());
static sText_Sec: Table<CArray<u8, 6>> =
    Table((&raw const crate::data::berry_blender::sText_Sec).cast());
static sText_SpaceBerry: Table<CArray<u8, 7>> =
    Table((&raw const crate::data::berry_blender::sText_SpaceBerry).cast());
static sText_TheFeelIs: Table<CArray<u8, 19>> =
    Table((&raw const crate::data::berry_blender::sText_TheFeelIs).cast());
static sText_TheLevelIs: Table<CArray<u8, 14>> =
    Table((&raw const crate::data::berry_blender::sText_TheLevelIs).cast());
static sText_Time: Table<CArray<u8, 6>> =
    Table((&raw const crate::data::berry_blender::sText_Time).cast());
static sText_WasMade: Table<CArray<u8, 11>> =
    Table((&raw const crate::data::berry_blender::sText_WasMade).cast());
static sText_WouldLikeToBlendAnotherBerry: Table<CArray<u8, 39>> =
    Table((&raw const crate::data::berry_blender::sText_WouldLikeToBlendAnotherBerry).cast());
static sText_YourPokeblockCaseIsFull: Table<CArray<u8, 26>> =
    Table((&raw const crate::data::berry_blender::sText_YourPokeblockCaseIsFull).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 7>> =
    Table((&raw const crate::data::berry_blender::sWindowTemplates).cast());
static sYesNoWindowTemplate_ContinuePlaying: Table<WindowTemplate> =
    Table((&raw const crate::data::berry_blender::sYesNoWindowTemplate_ContinuePlaying).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBerryBlender: *mut BerryBlender = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDebug_PokeblockFactorFlavors: CArray<i32, 5> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDebug_PokeblockFactorFlavorsAfterRPM: CArray<i32, 5> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDebug_PokeblockFactorRPM: u32 = 0;
pub(crate) static mut sPokeblockFlavors: Aligned<CArray<i16, 6>> = Aligned(unsafe { zeroed() });
pub(crate) static mut sPokeblockPresentFlavors: Aligned<CArray<i16, 6>> =
    Aligned(unsafe { zeroed() });
pub(crate) static mut sDebug_MaxRPMStage: i16 = 0;
pub(crate) static mut sDebug_GameTimeStage: i16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gInGameOpponentsNo: u8 = 0;

unsafe extern "C" {
    static gBerryBlenderCenter_Gfx: CArray<u32, 0>;
    static gBerryBlenderOuter_Gfx: CArray<u32, 0>;
    static gBerryBlenderOuter_Tilemap: CArray<u32, 0>;
    static mut gBlockRecvBuffer: CArray<CArray<u16, 128>, 5>;
    static mut gBlockSendBuffer: CArray<u8, 256>;
    static mut gEnableContestDebugging: u8;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gLinkType: u16;
    static mut gMPlayInfo_BGM: MusicPlayerInfo;
    static mut gMPlayInfo_SE2: MusicPlayerInfo;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static gPokeblockNames: CArray<*mut u8, 0>;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecordsWindowId: u8;
    static mut gRecvCmds: CArray<CArray<u16, 8>, 5>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSendCmd: CArray<u16, 8>;
    static gSineTable: CArray<i16, 0>;
    static mut gSoftResetDisabled: u8;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_ItemId: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_234Players: CArray<u8, 0>;
    static gText_BlenderMaxSpeedRecord: CArray<u8, 0>;
    static gText_SavingDontTurnOff2: CArray<u8, 0>;
    static gText_Space: CArray<u8, 0>;
    static mut gWirelessCommType: u8;
    fn AddPokeblock(a0: *mut Pokeblock) -> u32;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized4(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: i8,
        a8: *mut u8,
    );
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginFastPaletteFade(a0: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChooseBerryForMachine(a0: Option<unsafe extern "C" fn()>);
    fn ClearDialogWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearLinkCallback();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSpinningBerrySprite(a0: u8, a1: u8, a2: u8, a3: u8) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DrawDialogFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetBerryInfo(a0: u8) -> *mut Berry;
    fn GetBlockReceivedStatus() -> u8;
    fn GetCurrentMapMusic() -> u16;
    fn GetDecompressedDataSize(a0: *mut u32) -> u32;
    fn GetFirstFreePokeblockSlot() -> i8;
    fn GetHighestPokeblocksFlavorLevel(a0: *mut Pokeblock) -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerCountAsBitFlags() -> u8;
    fn GetMultiplayerId() -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetPokeblocksFeel(a0: *mut Pokeblock) -> u8;
    fn GetPokeblocksFlavor(a0: *mut Pokeblock) -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn IncrementDailyBerryBlender();
    fn IncrementGameStat(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsBagPocketNonEmpty(a0: u8) -> u8;
    fn IsFanfareTaskInactive() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn Menu_LoadStdPalAt(a0: u16);
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PlayBGM(a0: u16);
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn Put3CheersForPokeblocksOnTheAir(a0: *mut u8, a1: u8, a2: u8, a3: u8, a4: u8) -> u8;
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveBagItem(a0: u16, a1: u16) -> u8;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetBlockReceivedFlags();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn RunTextPrinters();
    fn SendBlock(a0: u8, a1: *mut c_void, a2: u16) -> u8;
    fn SendBlockRequest(a0: u8) -> u8;
    fn SetBerryBlenderLinkCallback();
    fn SetBgAffine(a0: u8, a1: i32, a2: i32, a3: i16, a4: i16, a5: i16, a6: i16, a7: u16);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetCloseLinkCallback();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetLinkDebugValues(a0: u32, a1: u32);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWirelessCommType0();
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn WriteSaveBlock1Sector() -> u8;
    fn WriteSaveBlock2() -> u8;
    fn m4aMPlayPitchControl(a0: *mut MusicPlayerInfo, a1: u16, a2: i16);
    fn m4aMPlayStop(a0: *mut MusicPlayerInfo);
    fn m4aMPlayTempoControl(a0: *mut MusicPlayerInfo, a1: u16);
}

pub(crate) unsafe extern "C" fn UpdateHitPitch() {
    m4aMPlayPitchControl(
        &raw mut gMPlayInfo_SE2,
        TRACKS_ALL,
        2 * ((*sBerryBlender).speed - MIN_ARROW_SPEED),
    );
}
pub(crate) unsafe extern "C" fn VBlankCB_BerryBlender() {
    SetBgPos();
    SetBgAffine(
        2,
        (*sBerryBlender).bgAffineSrc.texX,
        (*sBerryBlender).bgAffineSrc.texY,
        (*sBerryBlender).bgAffineSrc.scrX,
        (*sBerryBlender).bgAffineSrc.scrY,
        (*sBerryBlender).bgAffineSrc.sx,
        (*sBerryBlender).bgAffineSrc.sy,
        (*sBerryBlender).bgAffineSrc.alpha,
    );
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn LoadBerryBlenderGfx() -> u8 {
    match (*sBerryBlender).loadGfxState {
        0 => {
            (*sBerryBlender).tilesBuffer = AllocZeroed(
                GetDecompressedDataSize(gBerryBlenderCenter_Gfx.as_ptr().cast_mut()) + 100,
            );
            LZDecompressWram(
                gBerryBlenderCenter_Gfx.as_ptr().cast_mut(),
                (*sBerryBlender).tilesBuffer,
            );
            (*sBerryBlender).loadGfxState += 1;
        }
        1 => {
            CopyToBgTilemapBuffer(
                2,
                sBlenderCenter_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0x400,
                0,
            );
            CopyBgTilemapBufferToVram(2);
            LoadPalette(
                sBlenderCenter_Pal.as_ptr().cast_mut() as *mut c_void,
                0,
                256,
            );
            (*sBerryBlender).loadGfxState += 1;
        }
        2 => {
            LoadBgTiles(
                2,
                (*sBerryBlender).tilesBuffer,
                GetDecompressedDataSize(gBerryBlenderCenter_Gfx.as_ptr().cast_mut()) as u16,
                0,
            );
            (*sBerryBlender).loadGfxState += 1;
        }
        3 => {
            LZDecompressWram(
                gBerryBlenderOuter_Gfx.as_ptr().cast_mut(),
                (*sBerryBlender).tilesBuffer,
            );
            (*sBerryBlender).loadGfxState += 1;
        }
        4 => {
            LoadBgTiles(
                1,
                (*sBerryBlender).tilesBuffer,
                GetDecompressedDataSize(gBerryBlenderOuter_Gfx.as_ptr().cast_mut()) as u16,
                0,
            );
            (*sBerryBlender).loadGfxState += 1;
        }
        5 => {
            LZDecompressWram(
                gBerryBlenderOuter_Tilemap.as_ptr().cast_mut(),
                (*sBerryBlender).tilesBuffer,
            );
            (*sBerryBlender).loadGfxState += 1;
        }
        6 => {
            CopyToBgTilemapBuffer(
                1,
                (*sBerryBlender).tilesBuffer,
                GetDecompressedDataSize(gBerryBlenderOuter_Tilemap.as_ptr().cast_mut()) as u16,
                0,
            );
            CopyBgTilemapBufferToVram(1);
            (*sBerryBlender).loadGfxState += 1;
        }
        7 => {
            LoadPalette(
                sBlenderOuter_Pal.as_ptr().cast_mut() as *mut c_void,
                128,
                32,
            );
            (*sBerryBlender).loadGfxState += 1;
        }
        8 => {
            LoadSpriteSheet((&raw const *sSpriteSheet_PlayerArrow).cast_mut());
            LoadSpriteSheet((&raw const *sSpriteSheet_Particles).cast_mut());
            LoadSpriteSheet((&raw const *sSpriteSheet_ScoreSymbols).cast_mut());
            (*sBerryBlender).loadGfxState += 1;
        }
        9 => {
            LoadSpriteSheet((&raw const *sSpriteSheet_CountdownNumbers).cast_mut());
            LoadSpriteSheet((&raw const *sSpriteSheet_Start).cast_mut());
            LoadSpritePalette((&raw const *sSpritePal_PlayerArrow).cast_mut());
            LoadSpritePalette((&raw const *sSpritePal_BlenderMisc).cast_mut());
            Free((*sBerryBlender).tilesBuffer);
            (*sBerryBlender).loadGfxState = 0;
            return TRUE;
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DrawBlenderBg() {
    FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
    CopyBgTilemapBufferToVram(0);
    ShowBg(0);
    ShowBg(1);
    SetGpuRegBits(REG_OFFSET_DISPCNT, 4160);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
}
pub(crate) unsafe extern "C" fn InitBerryBlenderWindows() {
    if InitWindows(sWindowTemplates.as_ptr().cast_mut()) != 0 {
        let mut i: i32 = 0;
        DeactivateAllTextPrinters();
        i = 0;
        while i < WIN_RESULTS as i32 {
            FillWindowPixelBuffer(i as u8, 0);
            i += 1;
        }
        FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
        Menu_LoadStdPalAt(224);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBerryBlending() {
    if sBerryBlender.is_null() {
        sBerryBlender = AllocZeroed(4576) as *mut BerryBlender;
    }
    (*sBerryBlender).gameEndState = 0;
    (*sBerryBlender).mainState = 0;
    (*sBerryBlender).gameEndState = 0;
    InitLocalPlayers(gSpecialVar_0x8004 as u8);
    SetMainCallback2(Some(CB2_LoadBerryBlender));
}
pub(crate) unsafe extern "C" fn CB2_LoadBerryBlender() {
    let mut i: i32 = 0;
    match (*sBerryBlender).mainState {
        0 => {
            SetGpuReg(0x0, 0);
            ResetSpriteData();
            FreeAllSpritePalettes();
            SetVBlankCallback(None);
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(1, sBgTemplates.as_ptr().cast_mut(), 3);
            SetBgTilemapBuffer(
                1,
                (*sBerryBlender).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
            );
            SetBgTilemapBuffer(
                2,
                (*sBerryBlender).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
            );
            LoadUserWindowBorderGfx(0, 1, 208);
            LoadMessageBoxGfx(0, 0x14, 240);
            InitBerryBlenderWindows();
            (*sBerryBlender).mainState += 1;
            (*sBerryBlender).maxProgressBarValue = 0;
            (*sBerryBlender).progressBarValue = 0;
            (*sBerryBlender).centerScale = 80;
            (*sBerryBlender).bg_X = 0;
            (*sBerryBlender).bg_Y = 0;
            (*sBerryBlender).loadGfxState = 0;
            UpdateBlenderCenter();
        }
        1 => {
            if LoadBerryBlenderGfx() != 0 {
                i = 0;
                while i < BLENDER_MAX_PLAYERS {
                    (*sBerryBlender).playerArrowSpriteIds[i] = CreateSprite(
                        (&raw const *sSpriteTemplate_PlayerArrow).cast_mut(),
                        sPlayerArrowPos[i][0] as i16,
                        sPlayerArrowPos[i][1] as i16,
                        1,
                    );
                    StartSpriteAnim(
                        &raw mut gSprites[(*sBerryBlender).playerArrowSpriteIds[i]],
                        i as u8 + 8,
                    );
                    i += 1;
                }
                if gReceivedRemoteLinkPlayers != 0 && gWirelessCommType != 0 {
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0, 0);
                }
                SetVBlankCallback(Some(VBlankCB_BerryBlender));
                (*sBerryBlender).mainState += 1;
            }
        }
        2 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
            UpdateBlenderCenter();
            (*sBerryBlender).mainState += 1;
        }
        3 => {
            DrawBlenderBg();
            if gPaletteFade.active() == 0 {
                (*sBerryBlender).mainState += 1;
            }
        }
        4 => {
            if PrintMessage(
                &raw mut (*sBerryBlender).textState,
                sText_BerryBlenderStart.as_ptr().cast_mut(),
                GetPlayerTextSpeedDelay() as i32,
            ) != 0
            {
                (*sBerryBlender).mainState += 1;
            }
        }
        5 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
            (*sBerryBlender).mainState += 1;
        }
        6 => {
            if gPaletteFade.active() == 0 {
                FreeAllWindowBuffers();
                UnsetBgTilemapBuffer(2);
                UnsetBgTilemapBuffer(1);
                SetVBlankCallback(None);
                ChooseBerryForMachine(Some(StartBlender));
                (*sBerryBlender).mainState = 0;
            }
        }
        _ => {}
    }
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn SpriteCB_Berry(sprite: *mut Sprite) {
    (*sprite).data[1] += (*sprite).data[6];
    (*sprite).data[2] -= (*sprite).data[4];
    (*sprite).data[2] += (*sprite).data[7];
    (*sprite).data[0] += (*sprite).data[7];
    (*sprite).data[4] -= 1;
    if (*sprite).data[0] < (*sprite).data[2] {
        (*sprite).data[3] = {
            (*sprite).data[4] = (*sprite).data[3] - 1;
            (*sprite).data[4]
        };
        if ({
            (*sprite).data[5] += 1;
            (*sprite).data[5]
        }) > 3
        {
            DestroySprite(sprite);
        } else {
            PlaySE(SE_BALL_TRAY_EXIT);
        }
    }
    (*sprite).x = (*sprite).data[1];
    (*sprite).y = (*sprite).data[2];
}
pub(crate) unsafe extern "C" fn SetBerrySpriteData(
    sprite: *mut Sprite,
    x: i16,
    y: i16,
    bounceSpeed: i16,
    xSpeed: i16,
    ySpeed: i16,
) {
    (*sprite).data[0] = y;
    (*sprite).data[1] = x;
    (*sprite).data[2] = y;
    (*sprite).data[3] = bounceSpeed;
    (*sprite).data[4] = 10;
    (*sprite).data[5] = 0;
    (*sprite).data[6] = xSpeed;
    (*sprite).data[7] = ySpeed;
    (*sprite).callback = Some(SpriteCB_Berry);
}
pub(crate) unsafe extern "C" fn CreateBerrySprite(itemId: u16, playerId: u8) {
    let mut spriteId: u8 = CreateSpinningBerrySprite(
        itemId as u8 - ITEM_CHERI_BERRY as u8 + 1 - 1,
        0,
        80,
        playerId & 1,
    );
    SetBerrySpriteData(
        &raw mut gSprites[spriteId],
        sBerrySpriteData[playerId][0],
        sBerrySpriteData[playerId][1],
        sBerrySpriteData[playerId][2],
        sBerrySpriteData[playerId][3],
        sBerrySpriteData[playerId][4],
    );
}
pub(crate) unsafe extern "C" fn ConvertItemToBlenderBerry(berry: *mut BlenderBerry, itemId: u16) {
    let mut berryInfo: *mut Berry = GetBerryInfo(itemId as u8 - ITEM_CHERI_BERRY as u8 + 1);
    (*berry).itemId = itemId;
    StringCopy((*berry).name.as_mut_ptr(), (*berryInfo).name.as_mut_ptr());
    (*berry).flavors[0] = (*berryInfo).spicy;
    (*berry).flavors[1] = (*berryInfo).dry;
    (*berry).flavors[2] = (*berryInfo).sweet;
    (*berry).flavors[3] = (*berryInfo).bitter;
    (*berry).flavors[4] = (*berryInfo).sour;
    (*berry).flavors[5] = (*berryInfo).smoothness;
}
pub(crate) unsafe extern "C" fn InitLocalPlayers(opponentsNum: u8) {
    match opponentsNum {
        0 => {
            gInGameOpponentsNo = 0;
        }
        1 => {
            gInGameOpponentsNo = 1;
            (*sBerryBlender).numPlayers = 2;
            StringCopy(
                gLinkPlayers[0].name.as_mut_ptr(),
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            );
            if FlagGet(FLAG_HIDE_LILYCOVE_CONTEST_HALL_BLEND_MASTER) == 0 {
                StringCopy(gLinkPlayers[1].name.as_mut_ptr(), sBlenderOpponentsNames[3]);
            } else {
                StringCopy(gLinkPlayers[1].name.as_mut_ptr(), sBlenderOpponentsNames[0]);
            }
            gLinkPlayers[0].language = GAME_LANGUAGE as u16;
            gLinkPlayers[1].language = GAME_LANGUAGE as u16;
        }
        2 => {
            gInGameOpponentsNo = 2;
            (*sBerryBlender).numPlayers = 3;
            StringCopy(
                gLinkPlayers[0].name.as_mut_ptr(),
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            );
            StringCopy(gLinkPlayers[1].name.as_mut_ptr(), sBlenderOpponentsNames[4]);
            StringCopy(gLinkPlayers[2].name.as_mut_ptr(), sBlenderOpponentsNames[2]);
            gLinkPlayers[0].language = GAME_LANGUAGE as u16;
            gLinkPlayers[1].language = GAME_LANGUAGE as u16;
            gLinkPlayers[2].language = 2;
        }
        3 => {
            gInGameOpponentsNo = 3;
            (*sBerryBlender).numPlayers = 4;
            StringCopy(
                gLinkPlayers[0].name.as_mut_ptr(),
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            );
            StringCopy(gLinkPlayers[1].name.as_mut_ptr(), sBlenderOpponentsNames[5]);
            StringCopy(gLinkPlayers[2].name.as_mut_ptr(), sBlenderOpponentsNames[1]);
            StringCopy(gLinkPlayers[3].name.as_mut_ptr(), sBlenderOpponentsNames[2]);
            gLinkPlayers[0].language = GAME_LANGUAGE as u16;
            gLinkPlayers[1].language = GAME_LANGUAGE as u16;
            gLinkPlayers[2].language = 2;
            gLinkPlayers[3].language = GAME_LANGUAGE as u16;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn StartBlender() {
    let mut i: i32 = 0;
    SetGpuReg(0x0, 0);
    if sBerryBlender.is_null() {
        sBerryBlender = AllocZeroed(4576) as *mut BerryBlender;
    }
    (*sBerryBlender).mainState = 0;
    (*sBerryBlender).unk1 = 0;
    i = 0;
    while i < BLENDER_MAX_PLAYERS {
        (*sBerryBlender).chosenItemId[i] = ITEM_NONE;
        i += 1;
    }
    InitLocalPlayers(gSpecialVar_0x8004 as u8);
    if gSpecialVar_0x8004 == 0 {
        SetMainCallback2(Some(CB2_StartBlenderLink));
    } else {
        SetMainCallback2(Some(CB2_StartBlenderLocal));
    }
}
pub(crate) unsafe extern "C" fn CB2_StartBlenderLink() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    match (*sBerryBlender).mainState {
        0 => {
            InitBlenderBgs();
            gLinkType = LINKTYPE_BERRY_BLENDER;
            (*sBerryBlender).slowdownTimer = 0;
            i = 0;
            while i < BLENDER_MAX_PLAYERS {
                (*sBerryBlender).playerContinueResponses[i] = 0;
                j = 0;
                while j < NUM_SCORE_TYPES {
                    (*sBerryBlender).scores[i][j] = 0;
                    j += 1;
                }
                i += 1;
            }
            (*sBerryBlender).playAgainState = 0;
            (*sBerryBlender).maxRPM = 0;
            (*sBerryBlender).loadGfxState = 0;
            (*sBerryBlender).mainState += 1;
        }
        1 => {
            if LoadBerryBlenderGfx() != 0 {
                (*sBerryBlender).mainState += 1;
                UpdateBlenderCenter();
            }
        }
        2 => {
            i = 0;
            while i < BLENDER_MAX_PLAYERS {
                (*sBerryBlender).playerArrowSpriteIds2[i] = CreateSprite(
                    (&raw const *sSpriteTemplate_PlayerArrow).cast_mut(),
                    sPlayerArrowPos[i][0] as i16,
                    sPlayerArrowPos[i][1] as i16,
                    1,
                );
                StartSpriteAnim(
                    &raw mut gSprites[(*sBerryBlender).playerArrowSpriteIds2[i]],
                    i as u8 + 8,
                );
                i += 1;
            }
            if gReceivedRemoteLinkPlayers != 0 && gWirelessCommType != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(0, 0);
            }
            (*sBerryBlender).mainState += 1;
        }
        3 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
            (*sBerryBlender).mainState += 1;
        }
        4 => {
            DrawBlenderBg();
            if gPaletteFade.active() == 0 {
                (*sBerryBlender).mainState += 1;
            }
        }
        5 => {
            PrintMessage(
                &raw mut (*sBerryBlender).textState,
                sText_CommunicationStandby.as_ptr().cast_mut(),
                0,
            );
            (*sBerryBlender).mainState = 8;
            (*sBerryBlender).framesToWait = 0;
        }
        8 => {
            (*sBerryBlender).mainState += 1;
            (*sBerryBlender).playerToThrowBerry = 0;
            ConvertItemToBlenderBerry(
                &raw mut (*sBerryBlender).blendedBerries[0],
                gSpecialVar_ItemId,
            );
            memcpy(
                gBlockSendBuffer.as_mut_ptr(),
                &raw mut (*sBerryBlender).blendedBerries[0] as *mut u8,
                16,
            );
            SetLinkStandbyCallback();
            (*sBerryBlender).framesToWait = 0;
        }
        9 => {
            if IsLinkTaskFinished() != 0 {
                ResetBlockReceivedFlags();
                if GetMultiplayerId() == 0 {
                    SendBlockRequest(BLOCK_REQ_SIZE_40);
                }
                (*sBerryBlender).mainState += 1;
            }
        }
        10 => {
            if ({
                (*sBerryBlender).framesToWait += 1;
                (*sBerryBlender).framesToWait
            }) > 20
            {
                ClearDialogWindowAndFrameToTransparent(WIN_MSG, TRUE);
                if GetBlockReceivedStatus() == GetLinkPlayerCountAsBitFlags() {
                    i = 0;
                    while i < GetLinkPlayerCount() as i32 {
                        memcpy(
                            &raw mut (*sBerryBlender).blendedBerries[i] as *mut u8,
                            &raw mut gBlockRecvBuffer[i][0] as *mut u8,
                            16,
                        );
                        (*sBerryBlender).chosenItemId[i] =
                            (*sBerryBlender).blendedBerries[i].itemId;
                        i += 1;
                    }
                    ResetBlockReceivedFlags();
                    (*sBerryBlender).mainState += 1;
                }
            }
        }
        11 => {
            (*sBerryBlender).numPlayers = GetLinkPlayerCount();
            i = 0;
            while i < BLENDER_MAX_PLAYERS {
                if (*sBerryBlender).playerToThrowBerry
                    == sPlayerIdMap[(*sBerryBlender).numPlayers as i32 - 2][i]
                {
                    CreateBerrySprite(
                        (*sBerryBlender).chosenItemId[(*sBerryBlender).playerToThrowBerry],
                        i as u8,
                    );
                    break;
                }
                i += 1;
            }
            (*sBerryBlender).framesToWait = 0;
            (*sBerryBlender).mainState += 1;
            (*sBerryBlender).playerToThrowBerry += 1;
        }
        12 => {
            if ({
                (*sBerryBlender).framesToWait += 1;
                (*sBerryBlender).framesToWait
            }) > 60
            {
                if (*sBerryBlender).playerToThrowBerry >= (*sBerryBlender).numPlayers {
                    (*sBerryBlender).mainState += 1;
                    (*sBerryBlender).arrowPos = sArrowStartPos
                        [sArrowStartPosIds[(*sBerryBlender).numPlayers as i32 - 2]]
                        - ARROW_FALL_ROTATION;
                } else {
                    (*sBerryBlender).mainState -= 1;
                }
                (*sBerryBlender).framesToWait = 0;
            }
        }
        13 => {
            if IsLinkTaskFinished() != 0 {
                (*sBerryBlender).mainState += 1;
                DrawBlenderCenter(&raw mut (*sBerryBlender).bgAffineSrc);
                PlaySE(SE_FALL);
                ShowBg(2);
            }
        }
        14 => {
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG2_ON);
            (*sBerryBlender).arrowPos += 0x200;
            (*sBerryBlender).centerScale += 4;
            if (*sBerryBlender).centerScale > 255 {
                SetGpuRegBits(REG_OFFSET_BG2CNT, 2);
                (*sBerryBlender).mainState += 1;
                (*sBerryBlender).centerScale = 256;
                (*sBerryBlender).arrowPos =
                    sArrowStartPos[sArrowStartPosIds[(*sBerryBlender).numPlayers as i32 - 2]];
                (*sBerryBlender).framesToWait = 0;
                PlaySE(SE_TRUCK_DOOR);
                SetPlayerIdMaps();
                PrintPlayerNames();
            }
            DrawBlenderCenter(&raw mut (*sBerryBlender).bgAffineSrc);
        }
        15 => {
            if UpdateBlenderLandScreenShake() != 0 {
                (*sBerryBlender).framesToWait = 0;
                (*sBerryBlender).mainState += 1;
            }
            DrawBlenderCenter(&raw mut (*sBerryBlender).bgAffineSrc);
        }
        16 => {
            CreateSprite(
                (&raw const *sSpriteTemplate_CountdownNumbers).cast_mut(),
                120,
                -16,
                3,
            );
            (*sBerryBlender).mainState += 1;
        }
        17 => {}
        18 => {
            (*sBerryBlender).mainState += 1;
        }
        19 => {
            SetLinkStandbyCallback();
            (*sBerryBlender).mainState += 1;
        }
        20 => {
            if IsLinkTaskFinished() != 0 {
                SetBerryBlenderLinkCallback();
                (*sBerryBlender).mainState += 1;
            }
        }
        21 => {
            (*sBerryBlender).speed = MIN_ARROW_SPEED;
            (*sBerryBlender).gameFrameTime = 0;
            SetMainCallback2(Some(CB2_PlayBlender));
            if GetCurrentMapMusic() != MUS_CYCLING {
                (*sBerryBlender).savedMusic = GetCurrentMapMusic();
            }
            PlayBGM(MUS_CYCLING);
        }
        _ => {}
    }
    Blender_DummiedOutFunc((*sBerryBlender).bg_X as i16, (*sBerryBlender).bg_Y as i16);
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn InitBlenderBgs() {
    SetGpuReg(0x0, 0);
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetTasks();
    SetVBlankCallback(Some(VBlankCB_BerryBlender));
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(1, sBgTemplates.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(
        1,
        (*sBerryBlender).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*sBerryBlender).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    LoadUserWindowBorderGfx(0, 1, 208);
    LoadMessageBoxGfx(0, 0x14, 240);
    InitBerryBlenderWindows();
    (*sBerryBlender).unk0 = 0;
    (*sBerryBlender).speed = 0;
    (*sBerryBlender).arrowPos = 0;
    (*sBerryBlender).maxRPM = 0;
    (*sBerryBlender).bg_X = 0;
    (*sBerryBlender).bg_Y = 0;
}
pub(crate) unsafe extern "C" fn GetArrowProximity(arrowPos: u16, playerId: u8) -> u8 {
    let mut pos: u32 = (arrowPos as i32 / 256) as u32 + 24;
    let mut arrowId: u8 = (*sBerryBlender).playerIdToArrowId[playerId] as u8;
    let mut hitRangeStart: u32 = sArrowHitRangeStart[arrowId] as u32;
    if pos >= hitRangeStart && pos < hitRangeStart + 48 {
        if pos >= hitRangeStart + 20 && pos < hitRangeStart + 28 {
            return PROXIMITY_BEST;
        } else {
            return PROXIMITY_GOOD;
        }
    }
    return PROXIMITY_MISS;
}
pub(crate) unsafe extern "C" fn SetOpponentsBerryData(
    playerBerryItemId: u16,
    playersNum: u8,
    playerBerry: *mut BlenderBerry,
) {
    let mut opponentSetId: u16 = 0;
    let mut opponentBerryId: u16 = 0;
    let mut berryMasterDiff: u16 = 0;
    let mut i: u16 = 0;
    if playerBerryItemId == ITEM_ENIGMA_BERRY {
        i = 0;
        while i < FLAVOR_COUNT as u16 {
            if (*playerBerry).flavors[opponentSetId] > (*playerBerry).flavors[i] {
                opponentSetId = i;
            }
            i += 1;
        }
        opponentSetId += 5;
    } else {
        opponentSetId = playerBerryItemId - ITEM_CHERI_BERRY + 1 - 1;
        if opponentSetId >= 5 {
            opponentSetId = (opponentSetId as i32 % 5) as u16 + 5;
        }
    }
    i = 0;
    while (i as i32) < playersNum as i32 - 1 {
        opponentBerryId = sOpponentBerrySets[opponentSetId][i] as u16;
        berryMasterDiff = playerBerryItemId - ITEM_CHERI_BERRY + 1 - 31;
        if FlagGet(FLAG_HIDE_LILYCOVE_CONTEST_HALL_BLEND_MASTER) == 0 && gSpecialVar_0x8004 == 1 {
            opponentSetId = opponentSetId % 5;
            opponentBerryId = sBerryMasterBerries[opponentSetId] as u16;
            if berryMasterDiff < 5 {
                opponentBerryId -= 5;
            }
        }
        SetPlayerBerryData(i as u8 + 1, opponentBerryId + ITEM_CHERI_BERRY);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetPlayerIdMaps() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < BLENDER_MAX_PLAYERS {
        (*sBerryBlender).playerIdToArrowId[i] = NO_PLAYER;
        (*sBerryBlender).arrowIdToPlayerId[i] =
            sPlayerIdMap[(*sBerryBlender).numPlayers as i32 - 2][i] as u16;
        i += 1;
    }
    j = 0;
    while j < BLENDER_MAX_PLAYERS {
        i = 0;
        while i < BLENDER_MAX_PLAYERS {
            if (*sBerryBlender).arrowIdToPlayerId[i] as i32 == j {
                (*sBerryBlender).playerIdToArrowId[j] = i as u16;
            }
            i += 1;
        }
        j += 1;
    }
}
pub(crate) unsafe extern "C" fn PrintPlayerNames() {
    let mut i: i32 = 0;
    let mut xPos: i32 = 0;
    let mut playerId: u32 = 0;
    let mut text: CArray<u8, 20> = zeroed();
    if gReceivedRemoteLinkPlayers != 0 {
        playerId = GetMultiplayerId() as u32;
    }
    i = 0;
    while i < BLENDER_MAX_PLAYERS {
        if (*sBerryBlender).arrowIdToPlayerId[i] != NO_PLAYER {
            (*sBerryBlender).playerArrowSpriteIds[(*sBerryBlender).arrowIdToPlayerId[i]] =
                (*sBerryBlender).playerArrowSpriteIds2[i];
            StartSpriteAnim(
                &raw mut gSprites
                    [(*sBerryBlender).playerArrowSpriteIds[(*sBerryBlender).arrowIdToPlayerId[i]]],
                i as u8,
            );
            text[0] = EOS;
            StringCopy(
                text.as_mut_ptr(),
                gLinkPlayers[(*sBerryBlender).arrowIdToPlayerId[i]]
                    .name
                    .as_mut_ptr(),
            );
            xPos = GetStringCenterAlignXOffset(FONT_NORMAL as i32, text.as_mut_ptr(), 0x38);
            if playerId == (*sBerryBlender).arrowIdToPlayerId[i] as u32 {
                Blender_AddTextPrinter(i as u8, text.as_mut_ptr(), xPos as u8, 1, 0, 2);
            } else {
                Blender_AddTextPrinter(i as u8, text.as_mut_ptr(), xPos as u8, 1, 0, 1);
            }
            PutWindowTilemap(i as u8);
            CopyWindowToVram(i as u8, COPYWIN_FULL);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CB2_StartBlenderLocal() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    match (*sBerryBlender).mainState {
        0 => {
            SetWirelessCommType0();
            InitBlenderBgs();
            SetPlayerBerryData(0, gSpecialVar_ItemId);
            ConvertItemToBlenderBerry(
                &raw mut (*sBerryBlender).blendedBerries[0],
                gSpecialVar_ItemId,
            );
            SetOpponentsBerryData(
                gSpecialVar_ItemId,
                (*sBerryBlender).numPlayers,
                &raw mut (*sBerryBlender).blendedBerries[0],
            );
            i = 0;
            while i < BLENDER_MAX_PLAYERS {
                (*sBerryBlender).playerContinueResponses[i] = 0;
                j = 0;
                while j < NUM_SCORE_TYPES {
                    (*sBerryBlender).scores[i][j] = 0;
                    j += 1;
                }
                i += 1;
            }
            (*sBerryBlender).playAgainState = 0;
            (*sBerryBlender).loadGfxState = 0;
            gLinkType = LINKTYPE_BERRY_BLENDER;
            (*sBerryBlender).mainState += 1;
        }
        1 => {
            if LoadBerryBlenderGfx() != 0 {
                (*sBerryBlender).mainState += 1;
                UpdateBlenderCenter();
            }
        }
        2 => {
            i = 0;
            while i < BLENDER_MAX_PLAYERS {
                (*sBerryBlender).playerArrowSpriteIds2[i] = CreateSprite(
                    (&raw const *sSpriteTemplate_PlayerArrow).cast_mut(),
                    sPlayerArrowPos[i][0] as i16,
                    sPlayerArrowPos[i][1] as i16,
                    1,
                );
                StartSpriteAnim(
                    &raw mut gSprites[(*sBerryBlender).playerArrowSpriteIds2[i]],
                    i as u8 + 8,
                );
                i += 1;
            }
            (*sBerryBlender).mainState += 1;
        }
        3 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
            (*sBerryBlender).mainState += 1;
            (*sBerryBlender).framesToWait = 0;
        }
        4 => {
            if ({
                (*sBerryBlender).framesToWait += 1;
                (*sBerryBlender).framesToWait
            }) == 2
            {
                DrawBlenderBg();
            }
            if gPaletteFade.active() == 0 {
                (*sBerryBlender).mainState = 8;
            }
        }
        8 => {
            (*sBerryBlender).mainState = 11;
            (*sBerryBlender).playerToThrowBerry = 0;
        }
        11 => {
            i = 0;
            while i < BLENDER_MAX_PLAYERS {
                let mut playerId: u32 =
                    sPlayerIdMap[(*sBerryBlender).numPlayers as i32 - 2][i] as u32;
                if (*sBerryBlender).playerToThrowBerry as u32 == playerId {
                    CreateBerrySprite(
                        (*sBerryBlender).chosenItemId[(*sBerryBlender).playerToThrowBerry],
                        i as u8,
                    );
                    break;
                }
                i += 1;
            }
            (*sBerryBlender).framesToWait = 0;
            (*sBerryBlender).mainState += 1;
            (*sBerryBlender).playerToThrowBerry += 1;
        }
        12 => {
            if ({
                (*sBerryBlender).framesToWait += 1;
                (*sBerryBlender).framesToWait
            }) > 60
            {
                if (*sBerryBlender).playerToThrowBerry >= (*sBerryBlender).numPlayers {
                    (*sBerryBlender).arrowPos = sArrowStartPos
                        [sArrowStartPosIds[(*sBerryBlender).numPlayers as i32 - 2]]
                        - ARROW_FALL_ROTATION;
                    (*sBerryBlender).mainState += 1;
                } else {
                    (*sBerryBlender).mainState -= 1;
                }
                (*sBerryBlender).framesToWait = 0;
            }
        }
        13 => {
            (*sBerryBlender).mainState += 1;
            SetPlayerIdMaps();
            PlaySE(SE_FALL);
            DrawBlenderCenter(&raw mut (*sBerryBlender).bgAffineSrc);
            ShowBg(2);
        }
        14 => {
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG2_ON);
            (*sBerryBlender).arrowPos += 0x200;
            (*sBerryBlender).centerScale += 4;
            if (*sBerryBlender).centerScale > 255 {
                (*sBerryBlender).mainState += 1;
                (*sBerryBlender).centerScale = 256;
                (*sBerryBlender).arrowPos =
                    sArrowStartPos[sArrowStartPosIds[(*sBerryBlender).numPlayers as i32 - 2]];
                SetGpuRegBits(REG_OFFSET_BG2CNT, 2);
                (*sBerryBlender).framesToWait = 0;
                PlaySE(SE_TRUCK_DOOR);
                PrintPlayerNames();
            }
            DrawBlenderCenter(&raw mut (*sBerryBlender).bgAffineSrc);
        }
        15 => {
            if UpdateBlenderLandScreenShake() != 0 {
                (*sBerryBlender).mainState += 1;
            }
            DrawBlenderCenter(&raw mut (*sBerryBlender).bgAffineSrc);
        }
        16 => {
            CreateSprite(
                (&raw const *sSpriteTemplate_CountdownNumbers).cast_mut(),
                120,
                -16,
                3,
            );
            (*sBerryBlender).mainState += 1;
        }
        17 => {}
        18 => {
            (*sBerryBlender).mainState += 1;
        }
        19 => {
            (*sBerryBlender).mainState += 1;
        }
        20 => {
            (*sBerryBlender).mainState += 1;
        }
        21 => {
            ResetLinkCmds();
            (*sBerryBlender).speed = MIN_ARROW_SPEED;
            (*sBerryBlender).gameFrameTime = 0;
            (*sBerryBlender).perfectOpponents = FALSE;
            (*sBerryBlender).slowdownTimer = 0;
            SetMainCallback2(Some(CB2_PlayBlender));
            if gSpecialVar_0x8004 == 1 {
                if FlagGet(FLAG_HIDE_LILYCOVE_CONTEST_HALL_BLEND_MASTER) == 0 {
                    (*sBerryBlender).opponentTaskIds[0] =
                        CreateTask(Some(Task_HandleBerryMaster), 10);
                } else {
                    (*sBerryBlender).opponentTaskIds[0] = CreateTask(sLocalOpponentTasks[0], 10);
                }
            }
            if gSpecialVar_0x8004 > 1 {
                i = 0;
                while i < gSpecialVar_0x8004 as i32 {
                    (*sBerryBlender).opponentTaskIds[i] =
                        CreateTask(sLocalOpponentTasks[i], 10 + i as u8);
                    i += 1;
                }
            }
            if GetCurrentMapMusic() != MUS_CYCLING {
                (*sBerryBlender).savedMusic = GetCurrentMapMusic();
            }
            PlayBGM(MUS_CYCLING);
            PlaySE(SE_BERRY_BLENDER);
            UpdateHitPitch();
        }
        _ => {}
    }
    Blender_DummiedOutFunc((*sBerryBlender).bg_X as i16, (*sBerryBlender).bg_Y as i16);
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn ResetLinkCmds() {
    let mut i: i32 = 0;
    i = 0;
    while i < BLENDER_MAX_PLAYERS {
        gSendCmd[0] = 0;
        gSendCmd[2] = 0;
        gRecvCmds[i][0] = 0;
        gRecvCmds[i][2] = 0;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Task_OpponentMiss(taskId: u8) {
    if ({
        gTasks[taskId].data[0] += 1;
        gTasks[taskId].data[0]
    }) > gTasks[taskId].data[1]
    {
        gRecvCmds[gTasks[taskId].data[2]][2] = LINKCMD_BLENDER_SCORE_MISS;
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn CreateOpponentMissTask(playerId: u8, delay: u8) {
    let mut taskId: u8 = CreateTask(Some(Task_OpponentMiss), 80);
    gTasks[taskId].data[1] = delay as i16;
    gTasks[taskId].data[2] = playerId as i16;
}
pub(crate) unsafe extern "C" fn Task_HandleOpponent1(taskId: u8) {
    if GetArrowProximity((*sBerryBlender).arrowPos, 1) == PROXIMITY_BEST {
        if gTasks[taskId].data[0] == 0 {
            if (*sBerryBlender).perfectOpponents == 0 {
                let mut rand: u8 = (Random() as i32 / 655) as u8;
                if (*sBerryBlender).speed < 500 {
                    if rand > 75 {
                        gRecvCmds[1][2] = LINKCMD_BLENDER_SCORE_BEST;
                    } else {
                        gRecvCmds[1][2] = LINKCMD_BLENDER_SCORE_GOOD;
                    }
                    gRecvCmds[1][2] = LINKCMD_BLENDER_SCORE_GOOD;
                } else if (*sBerryBlender).speed < 1500 {
                    if rand > 80 {
                        gRecvCmds[1][2] = LINKCMD_BLENDER_SCORE_BEST;
                    } else {
                        let mut value: u8 = rand - 21;
                        if value < 60 {
                            gRecvCmds[1][2] = LINKCMD_BLENDER_SCORE_GOOD;
                        } else if rand < 10 {
                            CreateOpponentMissTask(1, 5);
                        }
                    }
                } else if rand <= 90 {
                    let mut value: u8 = rand - 71;
                    if value < 20 {
                        gRecvCmds[1][2] = LINKCMD_BLENDER_SCORE_GOOD;
                    } else if rand < 30 {
                        CreateOpponentMissTask(1, 5);
                    }
                } else {
                    gRecvCmds[1][2] = LINKCMD_BLENDER_SCORE_BEST;
                }
            } else {
                gRecvCmds[1][2] = LINKCMD_BLENDER_SCORE_BEST;
            }
            gTasks[taskId].data[0] = TRUE as i16;
        }
    } else {
        gTasks[taskId].data[0] = FALSE as i16;
    }
}
pub(crate) unsafe extern "C" fn Task_HandleOpponent2(taskId: u8) {
    let mut var1: u32 = (*sBerryBlender).arrowPos as u32 + 0x1800 & 0xFFFF;
    let mut arrowId: u8 = (*sBerryBlender).playerIdToArrowId[2] as u8;
    if var1 >> 8 > sArrowHitRangeStart[arrowId] as u32 + 20
        && var1 >> 8 < sArrowHitRangeStart[arrowId] as u32 + 40
    {
        if gTasks[taskId].data[0] == 0 {
            if (*sBerryBlender).perfectOpponents == 0 {
                let mut rand: u8 = (Random() as i32 / 655) as u8;
                if (*sBerryBlender).speed < 500 {
                    if rand > 66 {
                        gRecvCmds[2][2] = LINKCMD_BLENDER_SCORE_BEST;
                    } else {
                        gRecvCmds[2][2] = LINKCMD_BLENDER_SCORE_GOOD;
                    }
                } else {
                    if rand > 65 {
                        gRecvCmds[2][2] = LINKCMD_BLENDER_SCORE_BEST;
                    }
                    if rand > 40 && rand <= 65 {
                        gRecvCmds[2][2] = LINKCMD_BLENDER_SCORE_GOOD;
                    }
                    if rand < 10 {
                        CreateOpponentMissTask(2, 5);
                    }
                }
                gTasks[taskId].data[0] = TRUE as i16;
            } else {
                gRecvCmds[2][2] = LINKCMD_BLENDER_SCORE_BEST;
                gTasks[taskId].data[0] = TRUE as i16;
            }
        }
    } else {
        gTasks[taskId].data[0] = FALSE as i16;
    }
}
pub(crate) unsafe extern "C" fn Task_HandleOpponent3(taskId: u8) {
    let mut var1: u32 = (*sBerryBlender).arrowPos as u32 + 0x1800 & 0xFFFF;
    let mut arrowId: u8 = (*sBerryBlender).playerIdToArrowId[3] as u8;
    if var1 >> 8 > sArrowHitRangeStart[arrowId] as u32 + 20
        && var1 >> 8 < sArrowHitRangeStart[arrowId] as u32 + 40
    {
        if gTasks[taskId].data[0] == 0 {
            if (*sBerryBlender).perfectOpponents == 0 {
                let mut rand: u8 = (Random() as i32 / 655) as u8;
                if (*sBerryBlender).speed < 500 {
                    if rand > 88 {
                        gRecvCmds[3][2] = LINKCMD_BLENDER_SCORE_BEST;
                    } else {
                        gRecvCmds[3][2] = LINKCMD_BLENDER_SCORE_GOOD;
                    }
                } else {
                    if rand > 60 {
                        gRecvCmds[3][2] = LINKCMD_BLENDER_SCORE_BEST;
                    } else if rand > 55 && rand <= 60 {
                        gRecvCmds[3][2] = LINKCMD_BLENDER_SCORE_GOOD;
                    }
                    if rand < 5 {
                        CreateOpponentMissTask(3, 5);
                    }
                }
                gTasks[taskId].data[0] = TRUE as i16;
            } else {
                gRecvCmds[3][2] = LINKCMD_BLENDER_SCORE_BEST;
                gTasks[taskId].data[0] = TRUE as i16;
            }
        }
    } else {
        gTasks[taskId].data[0] = FALSE as i16;
    }
}
pub(crate) unsafe extern "C" fn Task_HandleBerryMaster(taskId: u8) {
    if GetArrowProximity((*sBerryBlender).arrowPos, 1) == PROXIMITY_BEST {
        if gTasks[taskId].data[0] == 0 {
            gRecvCmds[1][2] = LINKCMD_BLENDER_SCORE_BEST;
            gTasks[taskId].data[0] = TRUE as i16;
        }
    } else {
        gTasks[taskId].data[0] = FALSE as i16;
    }
}
pub(crate) unsafe extern "C" fn CreateScoreSymbolSprite(cmd: u16, arrowId: u8) {
    let mut spriteId: u8 = 0;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_ScoreSymbols).cast_mut(),
        sPlayerArrowPos[arrowId][0] as i16 - 10 * sPlayerArrowQuadrant[arrowId][0] as i16,
        sPlayerArrowPos[arrowId][1] as i16 - 10 * sPlayerArrowQuadrant[arrowId][1] as i16,
        1,
    );
    if cmd == LINKCMD_BLENDER_SCORE_BEST {
        StartSpriteAnim(&raw mut gSprites[spriteId], SCOREANIM_BEST_FLASH);
        gSprites[spriteId].callback = Some(SpriteCB_ScoreSymbolBest);
        PlaySE(SE_ICE_STAIRS);
    } else if cmd == LINKCMD_BLENDER_SCORE_GOOD {
        StartSpriteAnim(&raw mut gSprites[spriteId], SCOREANIM_GOOD);
        PlaySE(SE_SUCCESS);
    } else if cmd == LINKCMD_BLENDER_SCORE_MISS {
        StartSpriteAnim(&raw mut gSprites[spriteId], SCOREANIM_MISS);
        PlaySE(SE_FAILURE);
    }
    CreateParticleSprites();
}
pub(crate) unsafe extern "C" fn UpdateSpeedFromHit(cmd: u16) {
    UpdateHitPitch();
    match cmd {
        LINKCMD_BLENDER_SCORE_BEST => {
            if (*sBerryBlender).speed < 1500 {
                (*sBerryBlender).speed += div_i32(
                    384,
                    sNumPlayersToSpeedDivisor[(*sBerryBlender).numPlayers] as i32,
                ) as i16;
            } else {
                (*sBerryBlender).speed += div_i32(
                    128,
                    sNumPlayersToSpeedDivisor[(*sBerryBlender).numPlayers] as i32,
                ) as i16;
                ShakeBgCoordForHit(
                    &raw mut (*sBerryBlender).bg_X as *mut i16,
                    ((*sBerryBlender).speed / 100) as u16 - 10,
                );
                ShakeBgCoordForHit(
                    &raw mut (*sBerryBlender).bg_Y as *mut i16,
                    ((*sBerryBlender).speed / 100) as u16 - 10,
                );
            }
        }
        LINKCMD_BLENDER_SCORE_GOOD => {
            if (*sBerryBlender).speed < 1500 {
                (*sBerryBlender).speed += div_i32(
                    256,
                    sNumPlayersToSpeedDivisor[(*sBerryBlender).numPlayers] as i32,
                ) as i16;
            }
        }
        LINKCMD_BLENDER_SCORE_MISS => {
            (*sBerryBlender).speed -= div_i32(
                256,
                sNumPlayersToSpeedDivisor[(*sBerryBlender).numPlayers] as i32,
            ) as i16;
            if (*sBerryBlender).speed < MIN_ARROW_SPEED {
                (*sBerryBlender).speed = MIN_ARROW_SPEED;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CheckRecvCmdMatches(
    recvCmd: u16,
    linkCmd: u16,
    rfuCmd: u16,
) -> u32 {
    if gReceivedRemoteLinkPlayers != 0 && gWirelessCommType != 0 {
        if recvCmd as i32 & RFUCMD_MASK == rfuCmd as i32 {
            return TRUE as u32;
        }
    } else {
        if recvCmd == linkCmd {
            return TRUE as u32;
        }
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn UpdateOpponentScores() {
    let mut i: i32 = 0;
    if gSpecialVar_0x8004 != 0 {
        if gSendCmd[2] != 0 {
            gRecvCmds[0][2] = gSendCmd[2];
            gRecvCmds[0][0] = LINKCMD_BLENDER_SEND_KEYS;
            gSendCmd[2] = 0;
        }
        i = 1;
        while i < BLENDER_MAX_PLAYERS {
            if gRecvCmds[i][2] != 0 {
                gRecvCmds[i][0] = LINKCMD_BLENDER_SEND_KEYS;
            }
            i += 1;
        }
    }
    i = 0;
    while i < (*sBerryBlender).numPlayers as i32 {
        if CheckRecvCmdMatches(
            gRecvCmds[i][0],
            LINKCMD_BLENDER_SEND_KEYS,
            RFUCMD_BLENDER_SEND_KEYS,
        ) != 0
        {
            let mut arrowId: u32 = (*sBerryBlender).playerIdToArrowId[i] as u32;
            if gRecvCmds[i][2] == LINKCMD_BLENDER_SCORE_BEST {
                UpdateSpeedFromHit(LINKCMD_BLENDER_SCORE_BEST);
                (*sBerryBlender).progressBarValue += ((*sBerryBlender).speed / 55) as u16;
                if (*sBerryBlender).progressBarValue >= MAX_PROGRESS_BAR {
                    (*sBerryBlender).progressBarValue = MAX_PROGRESS_BAR;
                }
                CreateScoreSymbolSprite(LINKCMD_BLENDER_SCORE_BEST, arrowId as u8);
                (*sBerryBlender).scores[i][0] += 1;
            } else if gRecvCmds[i][2] == LINKCMD_BLENDER_SCORE_GOOD {
                UpdateSpeedFromHit(LINKCMD_BLENDER_SCORE_GOOD);
                (*sBerryBlender).progressBarValue += ((*sBerryBlender).speed / 70) as u16;
                CreateScoreSymbolSprite(LINKCMD_BLENDER_SCORE_GOOD, arrowId as u8);
                (*sBerryBlender).scores[i][1] += 1;
            } else if gRecvCmds[i][2] == LINKCMD_BLENDER_SCORE_MISS {
                CreateScoreSymbolSprite(LINKCMD_BLENDER_SCORE_MISS, arrowId as u8);
                UpdateSpeedFromHit(LINKCMD_BLENDER_SCORE_MISS);
                if (*sBerryBlender).scores[i][2] < 999 {
                    (*sBerryBlender).scores[i][2] += 1;
                }
            }
            if gRecvCmds[i][2] == LINKCMD_BLENDER_SCORE_MISS
                || gRecvCmds[2][i] == LINKCMD_BLENDER_SCORE_BEST
                || gRecvCmds[2][i] == LINKCMD_BLENDER_SCORE_GOOD
            {
                if (*sBerryBlender).speed > 1500 {
                    m4aMPlayTempoControl(
                        &raw mut gMPlayInfo_BGM,
                        (((*sBerryBlender).speed as i32 - 750) / 20) as u16 + 256,
                    );
                } else {
                    m4aMPlayTempoControl(&raw mut gMPlayInfo_BGM, 256);
                }
            }
        }
        i += 1;
    }
    if gSpecialVar_0x8004 != 0 {
        i = 0;
        while i < (*sBerryBlender).numPlayers as i32 {
            gRecvCmds[i][0] = 0;
            gRecvCmds[i][2] = 0;
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn HandlePlayerInput() {
    let mut arrowId: u8 = 0;
    let mut pressedA: u8 = FALSE;
    let mut playerId: u8 = 0;
    if gReceivedRemoteLinkPlayers != 0 {
        playerId = GetMultiplayerId();
    }
    arrowId = (*sBerryBlender).playerIdToArrowId[playerId] as u8;
    if (*sBerryBlender).gameEndState == 0 {
        if (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_L_EQUALS_A
            && gMain.newKeys as i32 & A_BUTTON != 0
        {
            if gMain.heldKeysRaw as i32 & 513 != 513 {
                pressedA = TRUE;
            }
        } else if gMain.newKeys as i32 & A_BUTTON != 0 {
            pressedA = TRUE;
        }
        if pressedA != 0 {
            let mut proximity: u8 = 0;
            StartSpriteAnim(
                &raw mut gSprites[(*sBerryBlender).playerArrowSpriteIds
                    [(*sBerryBlender).arrowIdToPlayerId[arrowId]]],
                arrowId + 4,
            );
            proximity = GetArrowProximity((*sBerryBlender).arrowPos, playerId);
            if proximity == PROXIMITY_BEST {
                gSendCmd[2] = LINKCMD_BLENDER_SCORE_BEST;
            } else if proximity == PROXIMITY_GOOD {
                gSendCmd[2] = LINKCMD_BLENDER_SCORE_GOOD;
            } else {
                gSendCmd[2] = LINKCMD_BLENDER_SCORE_MISS;
            }
        }
    }
    if ({
        (*sBerryBlender).slowdownTimer += 1;
        (*sBerryBlender).slowdownTimer
    }) > 5
    {
        if (*sBerryBlender).speed > MIN_ARROW_SPEED {
            (*sBerryBlender).speed -= 1;
        }
        (*sBerryBlender).slowdownTimer = 0;
    }
    if gEnableContestDebugging != 0 && gMain.newKeys as i32 & L_BUTTON != 0 {
        (*sBerryBlender).perfectOpponents ^= 1;
    }
}
pub(crate) unsafe extern "C" fn CB2_PlayBlender() {
    UpdateBlenderCenter();
    if (*sBerryBlender).gameFrameTime < 0x57e04 {
        (*sBerryBlender).gameFrameTime += 1;
    }
    HandlePlayerInput();
    SetLinkDebugValues(
        (*sBerryBlender).speed as u16 as u32,
        (*sBerryBlender).progressBarValue as u32,
    );
    UpdateOpponentScores();
    TryUpdateProgressBar((*sBerryBlender).progressBarValue, MAX_PROGRESS_BAR);
    UpdateRPM((*sBerryBlender).speed as u16);
    RestoreBgCoords();
    ProcessLinkPlayerCmds();
    if (*sBerryBlender).gameEndState == 0
        && (*sBerryBlender).maxProgressBarValue >= MAX_PROGRESS_BAR
    {
        (*sBerryBlender).progressBarValue = MAX_PROGRESS_BAR;
        (*sBerryBlender).gameEndState = 1;
        SetMainCallback2(Some(CB2_EndBlenderGame));
    }
    Blender_DummiedOutFunc((*sBerryBlender).bg_X as i16, (*sBerryBlender).bg_Y as i16);
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn Blender_DummiedOutFunc(bgX: i16, bgY: i16) {}
pub(crate) unsafe extern "C" fn AreBlenderBerriesSame(
    berries: *mut BlenderBerry,
    a: u8,
    b: u8,
) -> u8 {
    if (*berries.at(a)).itemId != (*berries.at(b)).itemId
        || StringCompare(
            (*berries.at(a)).name.as_mut_ptr(),
            (*berries.at(b)).name.as_mut_ptr(),
        ) == 0
            && ((*berries.at(a)).flavors[0] == (*berries.at(b)).flavors[0]
                && (*berries.at(a)).flavors[1] == (*berries.at(b)).flavors[1]
                && (*berries.at(a)).flavors[2] == (*berries.at(b)).flavors[2]
                && (*berries.at(a)).flavors[3] == (*berries.at(b)).flavors[3]
                && (*berries.at(a)).flavors[4] == (*berries.at(b)).flavors[4]
                && (*berries.at(a)).flavors[5] == (*berries.at(b)).flavors[5])
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
pub(crate) unsafe extern "C" fn CalculatePokeblockColor(
    berries: *mut BlenderBerry,
    _flavors: *mut i16,
    numPlayers: u8,
    negativeFlavors: u8,
) -> u32 {
    let mut flavors: CArray<i16, 6> = zeroed();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut numFlavors: u8 = 0;
    i = 0;
    while i < 6 {
        flavors[i] = *_flavors.at(i);
        i += 1;
    }
    j = 0;
    i = 0;
    while i < FLAVOR_COUNT {
        if flavors[i] == 0 {
            j += 1;
        }
        i += 1;
    }
    if j == FLAVOR_COUNT || negativeFlavors > 3 {
        return PBLOCK_CLR_BLACK;
    }
    i = 0;
    while i < numPlayers as i32 {
        j = 0;
        while j < numPlayers as i32 {
            if (*berries.at(i)).itemId == (*berries.at(j)).itemId
                && i != j
                && ((*berries.at(i)).itemId != ITEM_ENIGMA_BERRY
                    || AreBlenderBerriesSame(berries, i as u8, j as u8) != 0)
            {
                return PBLOCK_CLR_BLACK;
            }
            j += 1;
        }
        i += 1;
    }
    numFlavors = 0;
    numFlavors = 0;
    i = 0;
    while i < FLAVOR_COUNT {
        if flavors[i] > 0 {
            numFlavors += 1;
        }
        i += 1;
    }
    if numFlavors > 3 {
        return PBLOCK_CLR_WHITE;
    }
    if numFlavors == 3 {
        return PBLOCK_CLR_GRAY;
    }
    i = 0;
    while i < FLAVOR_COUNT {
        if flavors[i] > 50 {
            return PBLOCK_CLR_GOLD;
        }
        i += 1;
    }
    if numFlavors == 1 && flavors[0] > 0 {
        return PBLOCK_CLR_RED;
    }
    if numFlavors == 1 && flavors[1] > 0 {
        return PBLOCK_CLR_BLUE;
    }
    if numFlavors == 1 && flavors[2] > 0 {
        return PBLOCK_CLR_PINK;
    }
    if numFlavors == 1 && flavors[3] > 0 {
        return PBLOCK_CLR_GREEN;
    }
    if numFlavors == 1 && flavors[4] > 0 {
        return PBLOCK_CLR_YELLOW;
    }
    if numFlavors == 2 {
        let mut idx: i32 = 0;
        i = 0;
        while i < FLAVOR_COUNT {
            if flavors[i] > 0 {
                sPokeblockPresentFlavors[{
                    let t1 = idx;
                    idx += 1;
                    t1
                }] = i as i16;
            }
            i += 1;
        }
        if flavors[sPokeblockPresentFlavors[0]] >= flavors[sPokeblockPresentFlavors[1]] {
            if sPokeblockPresentFlavors[0] == 0 {
                return (sPokeblockPresentFlavors[1] as u32) << 16 | PBLOCK_CLR_PURPLE;
            }
            if sPokeblockPresentFlavors[0] == FLAVOR_DRY as i16 {
                return (sPokeblockPresentFlavors[1] as u32) << 16 | PBLOCK_CLR_INDIGO;
            }
            if sPokeblockPresentFlavors[0] == FLAVOR_SWEET as i16 {
                return (sPokeblockPresentFlavors[1] as u32) << 16 | PBLOCK_CLR_BROWN;
            }
            if sPokeblockPresentFlavors[0] == FLAVOR_BITTER as i16 {
                return (sPokeblockPresentFlavors[1] as u32) << 16 | PBLOCK_CLR_LITE_BLUE;
            }
            if sPokeblockPresentFlavors[0] == FLAVOR_SOUR as i16 {
                return (sPokeblockPresentFlavors[1] as u32) << 16 | PBLOCK_CLR_OLIVE;
            }
        } else {
            if sPokeblockPresentFlavors[1] == FLAVOR_SPICY as i16 {
                return (sPokeblockPresentFlavors[0] as u32) << 16 | PBLOCK_CLR_PURPLE;
            }
            if sPokeblockPresentFlavors[1] == 1 {
                return (sPokeblockPresentFlavors[0] as u32) << 16 | PBLOCK_CLR_INDIGO;
            }
            if sPokeblockPresentFlavors[1] == FLAVOR_SWEET as i16 {
                return (sPokeblockPresentFlavors[0] as u32) << 16 | PBLOCK_CLR_BROWN;
            }
            if sPokeblockPresentFlavors[1] == FLAVOR_BITTER as i16 {
                return (sPokeblockPresentFlavors[0] as u32) << 16 | PBLOCK_CLR_LITE_BLUE;
            }
            if sPokeblockPresentFlavors[1] == FLAVOR_SOUR as i16 {
                return (sPokeblockPresentFlavors[0] as u32) << 16 | PBLOCK_CLR_OLIVE;
            }
        }
    }
    return PBLOCK_CLR_NONE as u32;
}
pub(crate) unsafe extern "C" fn Debug_SetMaxRPMStage(value: i16) {
    sDebug_MaxRPMStage = value;
}
pub(crate) unsafe extern "C" fn Debug_GetMaxRPMStage() -> i16 {
    return sDebug_MaxRPMStage;
}
pub(crate) unsafe extern "C" fn Debug_SetGameTimeStage(value: i16) {
    sDebug_GameTimeStage = value;
}
pub(crate) unsafe extern "C" fn Debug_GetGameTimeStage() -> i16 {
    return sDebug_GameTimeStage;
}
pub(crate) unsafe extern "C" fn CalculatePokeblock(
    berries: *mut BlenderBerry,
    pokeblock: *mut Pokeblock,
    numPlayers: u8,
    mut flavors: *mut u8,
    maxRPM: u16,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut multiuseVar: i32 = 0;
    let mut numNegatives: u8 = 0;
    i = 0;
    while i < 6 {
        sPokeblockFlavors[i] = 0;
        i += 1;
    }
    i = 0;
    while i < numPlayers as i32 {
        j = 0;
        while j < 6 {
            sPokeblockFlavors[j] += (*berries.at(i)).flavors[j] as i16;
            j += 1;
        }
        i += 1;
    }
    multiuseVar = sPokeblockFlavors[0] as i32;
    sPokeblockFlavors[0] -= sPokeblockFlavors[1];
    sPokeblockFlavors[1] -= sPokeblockFlavors[2];
    sPokeblockFlavors[2] -= sPokeblockFlavors[3];
    sPokeblockFlavors[3] -= sPokeblockFlavors[4];
    sPokeblockFlavors[4] -= multiuseVar as i16;
    multiuseVar = 0;
    i = 0;
    while i < FLAVOR_COUNT {
        if sPokeblockFlavors[i] < 0 {
            sPokeblockFlavors[i] = 0;
            multiuseVar += 1;
        }
        i += 1;
    }
    numNegatives = multiuseVar as u8;
    i = 0;
    while i < FLAVOR_COUNT {
        if sPokeblockFlavors[i] > 0 {
            if (sPokeblockFlavors[i] as i32) < multiuseVar {
                sPokeblockFlavors[i] = 0;
            } else {
                sPokeblockFlavors[i] -= multiuseVar as i16;
            }
        }
        i += 1;
    }
    i = 0;
    while i < FLAVOR_COUNT {
        sDebug_PokeblockFactorFlavors[i] = sPokeblockFlavors[i] as i32;
        i += 1;
    }
    sDebug_PokeblockFactorRPM = ({
        multiuseVar = maxRPM as i32 / 333 + 100;
        multiuseVar
    }) as u32;
    i = 0;
    while i < FLAVOR_COUNT {
        let mut remainder: i32 = 0;
        let mut flavor: i32 = sPokeblockFlavors[i] as i32;
        flavor = flavor * multiuseVar / 10;
        remainder = flavor % 10;
        flavor = flavor / 10;
        if remainder > 4 {
            flavor += 1;
        }
        sPokeblockFlavors[i] = flavor as i16;
        i += 1;
    }
    i = 0;
    while i < FLAVOR_COUNT {
        sDebug_PokeblockFactorFlavorsAfterRPM[i] = sPokeblockFlavors[i] as i32;
        i += 1;
    }
    (*pokeblock).color = CalculatePokeblockColor(
        berries,
        &raw mut sPokeblockFlavors[0],
        numPlayers,
        numNegatives,
    ) as u8;
    sPokeblockFlavors[5] =
        div_i32(sPokeblockFlavors[5] as i32, numPlayers as i32) as i16 - numPlayers as i16;
    if sPokeblockFlavors[5] < 0 {
        sPokeblockFlavors[5] = 0;
    }
    if (*pokeblock).color == PBLOCK_CLR_BLACK as u8 {
        multiuseVar = (Random() % 10) as i32;
        i = 0;
        while i < FLAVOR_COUNT {
            if shr_i32(sBlackPokeblockFlavorFlags[multiuseVar] as i32, i as u32) & 1 != 0 {
                sPokeblockFlavors[i] = 2;
            } else {
                sPokeblockFlavors[i] = 0;
            }
            i += 1;
        }
    }
    i = 0;
    while i < 6 {
        if sPokeblockFlavors[i] > 255 {
            sPokeblockFlavors[i] = 255;
        }
        i += 1;
    }
    (*pokeblock).spicy = sPokeblockFlavors[0] as u8;
    (*pokeblock).dry = sPokeblockFlavors[1] as u8;
    (*pokeblock).sweet = sPokeblockFlavors[2] as u8;
    (*pokeblock).bitter = sPokeblockFlavors[3] as u8;
    (*pokeblock).sour = sPokeblockFlavors[4] as u8;
    (*pokeblock).feel = sPokeblockFlavors[5] as u8;
    i = 0;
    while i < 6 {
        *flavors.at(i) = sPokeblockFlavors[i] as u8;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Debug_CalculatePokeblock(
    berries: *mut BlenderBerry,
    pokeblock: *mut Pokeblock,
    numPlayers: u8,
    flavors: *mut u8,
    maxRPM: u16,
) {
    CalculatePokeblock(berries, pokeblock, numPlayers, flavors, maxRPM);
}
pub(crate) unsafe extern "C" fn Debug_SetStageVars() {
    let mut frames: u32 = (*sBerryBlender).gameFrameTime as u16 as u32;
    let mut maxRPM: u16 = (*sBerryBlender).maxRPM;
    let mut stage: i16 = 0;
    if frames < 900 {
        stage = 5;
    } else if frames as u16 as i32 - 900 < 600 {
        stage = 4;
    } else if frames as u16 as i32 - 1500 < 600 {
        stage = 3;
    } else if frames as u16 as i32 - 2100 < 900 {
        stage = 2;
    } else if frames as u16 as i32 - 3300 < 300 {
        stage = 1;
    }
    Debug_SetGameTimeStage(stage);
    stage = 0;
    if maxRPM <= 64 {
        if maxRPM >= 50 && maxRPM < 100 {
            stage = -1;
        } else if maxRPM >= 100 && maxRPM < 150 {
            stage = -2;
        } else if maxRPM >= 150 && maxRPM < 200 {
            stage = -3;
        } else if maxRPM >= 200 && maxRPM < 250 {
            stage = -4;
        } else if maxRPM >= 250 && maxRPM < 300 {
            stage = -5;
        } else if maxRPM >= 350 && maxRPM < 400 {
            stage = -6;
        } else if maxRPM >= 400 && maxRPM < 450 {
            stage = -7;
        } else if maxRPM >= 500 && maxRPM < 550 {
            stage = -8;
        } else if maxRPM >= 550 && maxRPM < 600 {
            stage = -9;
        } else if maxRPM >= 600 {
            stage = -10;
        }
    }
    Debug_SetMaxRPMStage(stage);
}
pub(crate) unsafe extern "C" fn SendContinuePromptResponse(cmd: *mut u16) {
    if gReceivedRemoteLinkPlayers != 0 && gWirelessCommType != 0 {
        *cmd = RFUCMD_SEND_PACKET as u16;
    } else {
        *cmd = LINKCMD_SEND_PACKET;
    }
}
pub(crate) unsafe extern "C" fn CB2_EndBlenderGame() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    if (*sBerryBlender).gameEndState < 3 {
        UpdateBlenderCenter();
    }
    GetMultiplayerId();
    match (*sBerryBlender).gameEndState {
        1 => {
            m4aMPlayTempoControl(&raw mut gMPlayInfo_BGM, 256);
            i = 0;
            while (i as u16) < gSpecialVar_0x8004 {
                DestroyTask((*sBerryBlender).opponentTaskIds[i]);
                i += 1;
            }
            (*sBerryBlender).gameEndState += 1;
        }
        2 => {
            (*sBerryBlender).speed -= 32;
            if (*sBerryBlender).speed <= 0 {
                ClearLinkCallback();
                (*sBerryBlender).speed = 0;
                if gReceivedRemoteLinkPlayers != 0 {
                    (*sBerryBlender).gameEndState += 1;
                } else {
                    (*sBerryBlender).gameEndState = 5;
                }
                (*sBerryBlender).mainState = 0;
                m4aMPlayStop(&raw mut gMPlayInfo_SE2);
            }
            UpdateHitPitch();
        }
        3 => {
            if GetMultiplayerId() != 0 {
                (*sBerryBlender).gameEndState += 1;
            } else if IsLinkTaskFinished() != 0 {
                if gReceivedRemoteLinkPlayers != 0 && gWirelessCommType != 0 {
                    (*sBerryBlender).gameBlock.timeRPM.time = (*sBerryBlender).gameFrameTime;
                    (*sBerryBlender).gameBlock.timeRPM.maxRPM = (*sBerryBlender).maxRPM;
                    i = 0;
                    while i < BLENDER_MAX_PLAYERS as u8 {
                        j = 0;
                        while j < NUM_SCORE_TYPES as u8 {
                            (*sBerryBlender).gameBlock.scores[i][j] = (*sBerryBlender).scores[i][j];
                            j += 1;
                        }
                        i += 1;
                    }
                    if SendBlock(0, &raw mut (*sBerryBlender).gameBlock as *mut c_void, 32) != 0 {
                        (*sBerryBlender).gameEndState += 1;
                    }
                } else {
                    (*sBerryBlender).smallBlock.time = (*sBerryBlender).gameFrameTime;
                    (*sBerryBlender).smallBlock.maxRPM = (*sBerryBlender).maxRPM;
                    if SendBlock(0, &raw mut (*sBerryBlender).smallBlock as *mut c_void, 40) != 0 {
                        (*sBerryBlender).gameEndState += 1;
                    }
                }
            }
        }
        4 => {
            if GetBlockReceivedStatus() != 0 {
                ResetBlockReceivedFlags();
                (*sBerryBlender).gameEndState += 1;
                if gReceivedRemoteLinkPlayers != 0 && gWirelessCommType != 0 {
                    let mut receivedBlock: *mut BlenderGameBlock =
                        &raw mut gBlockRecvBuffer as *mut BlenderGameBlock;
                    (*sBerryBlender).maxRPM = (*receivedBlock).timeRPM.maxRPM;
                    (*sBerryBlender).gameFrameTime = (*receivedBlock).timeRPM.time;
                    i = 0;
                    while i < BLENDER_MAX_PLAYERS as u8 {
                        j = 0;
                        while j < NUM_SCORE_TYPES as u8 {
                            (*sBerryBlender).scores[i][j] = (*receivedBlock).scores[i][j];
                            j += 1;
                        }
                        i += 1;
                    }
                } else {
                    let mut receivedBlock: *mut TimeAndRPM =
                        &raw mut gBlockRecvBuffer as *mut TimeAndRPM;
                    (*sBerryBlender).maxRPM = (*receivedBlock).maxRPM;
                    (*sBerryBlender).gameFrameTime = (*receivedBlock).time;
                }
            }
        }
        5 => {
            if PrintBlendingRanking() != 0 {
                (*sBerryBlender).gameEndState += 1;
            }
        }
        6 => {
            if PrintBlendingResults() != 0 {
                if gInGameOpponentsNo == 0 {
                    IncrementGameStat(GAME_STAT_POKEBLOCKS_WITH_FRIENDS);
                } else {
                    IncrementGameStat(GAME_STAT_POKEBLOCKS);
                }
                (*sBerryBlender).gameEndState += 1;
            }
        }
        7 => {
            if PrintMessage(
                &raw mut (*sBerryBlender).textState,
                sText_WouldLikeToBlendAnotherBerry.as_ptr().cast_mut(),
                GetPlayerTextSpeedDelay() as i32,
            ) != 0
            {
                (*sBerryBlender).gameEndState += 1;
            }
        }
        9 => {
            (*sBerryBlender).yesNoAnswer = 0;
            CreateYesNoMenu(
                (&raw const *sYesNoWindowTemplate_ContinuePlaying).cast_mut(),
                1,
                0xD,
                0,
            );
            (*sBerryBlender).gameEndState += 1;
        }
        10 => match Menu_ProcessInputNoWrapClearOnChoose() {
            1 | MENU_B_PRESSED => {
                (*sBerryBlender).yesNoAnswer = 1;
                (*sBerryBlender).gameEndState += 1;
                i = 0;
                while i < BLENDER_MAX_PLAYERS as u8 {
                    if (*sBerryBlender).arrowIdToPlayerId[i] != NO_PLAYER {
                        PutWindowTilemap(i);
                        CopyWindowToVram(i, COPYWIN_FULL);
                    }
                    i += 1;
                }
            }
            0 => {
                (*sBerryBlender).yesNoAnswer = 0;
                (*sBerryBlender).gameEndState += 1;
                i = 0;
                while i < BLENDER_MAX_PLAYERS as u8 {
                    if (*sBerryBlender).arrowIdToPlayerId[i] != NO_PLAYER {
                        PutWindowTilemap(i);
                        CopyWindowToVram(i, COPYWIN_FULL);
                    }
                    i += 1;
                }
            }
            _ => {}
        },
        11 => {
            SendContinuePromptResponse(&raw mut gSendCmd[0]);
            if (*sBerryBlender).yesNoAnswer == 0 {
                if IsBagPocketNonEmpty(POCKET_BERRIES) == FALSE {
                    (*sBerryBlender).playAgainState = CANT_PLAY_NO_BERRIES;
                    gSendCmd[1] = LINKCMD_BLENDER_NO_BERRIES;
                } else if GetFirstFreePokeblockSlot() == -1 {
                    (*sBerryBlender).playAgainState = CANT_PLAY_NO_PKBLCK_SPACE;
                    gSendCmd[1] = LINKCMD_BLENDER_NO_PBLOCK_SPACE;
                } else {
                    (*sBerryBlender).playAgainState = PLAY_AGAIN_YES;
                    gSendCmd[1] = LINKCMD_BLENDER_PLAY_AGAIN;
                }
                (*sBerryBlender).gameEndState += 1;
            } else {
                (*sBerryBlender).playAgainState = PLAY_AGAIN_NO;
                gSendCmd[1] = LINKCMD_CONT_BLOCK;
                (*sBerryBlender).gameEndState += 1;
            }
        }
        12 => {
            if gInGameOpponentsNo != 0 {
                SetMainCallback2(Some(CB2_CheckPlayAgainLocal));
                (*sBerryBlender).gameEndState = 0;
                (*sBerryBlender).mainState = 0;
            } else {
                (*sBerryBlender).gameEndState += 1;
            }
        }
        8 => {
            (*sBerryBlender).gameEndState += 1;
        }
        13 => {
            if PrintMessage(
                &raw mut (*sBerryBlender).textState,
                sText_CommunicationStandby.as_ptr().cast_mut(),
                GetPlayerTextSpeedDelay() as i32,
            ) != 0
            {
                SetMainCallback2(Some(CB2_CheckPlayAgainLink));
                (*sBerryBlender).gameEndState = 0;
                (*sBerryBlender).mainState = 0;
            }
        }
        _ => {}
    }
    RestoreBgCoords();
    UpdateRPM((*sBerryBlender).speed as u16);
    ProcessLinkPlayerCmds();
    Blender_DummiedOutFunc((*sBerryBlender).bg_X as i16, (*sBerryBlender).bg_Y as i16);
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn LinkPlayAgainHandleSaving() -> u8 {
    match (*sBerryBlender).linkPlayAgainState {
        0 => {
            SetLinkStandbyCallback();
            (*sBerryBlender).linkPlayAgainState = 1;
            (*sBerryBlender).framesToWait = 0;
        }
        1 => {
            if IsLinkTaskFinished() != 0 {
                (*sBerryBlender).linkPlayAgainState += 1;
                gSoftResetDisabled = TRUE;
            }
        }
        2 => {
            WriteSaveBlock2();
            (*sBerryBlender).linkPlayAgainState += 1;
            (*sBerryBlender).framesToWait = 0;
        }
        3 => {
            if ({
                (*sBerryBlender).framesToWait += 1;
                (*sBerryBlender).framesToWait
            }) == 10
            {
                SetLinkStandbyCallback();
                (*sBerryBlender).linkPlayAgainState += 1;
            }
        }
        4 => {
            if IsLinkTaskFinished() != 0 {
                if WriteSaveBlock1Sector() != 0 {
                    (*sBerryBlender).linkPlayAgainState = 5;
                } else {
                    (*sBerryBlender).framesToWait = 0;
                    (*sBerryBlender).linkPlayAgainState = 3;
                }
            }
        }
        5 => {
            (*sBerryBlender).linkPlayAgainState += 1;
            (*sBerryBlender).framesToWait = 0;
        }
        6 => {
            if ({
                (*sBerryBlender).framesToWait += 1;
                (*sBerryBlender).framesToWait
            }) > 5
            {
                gSoftResetDisabled = FALSE;
                return TRUE;
            }
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn CB2_CheckPlayAgainLink() {
    match (*sBerryBlender).gameEndState {
        0 => {
            if (*sBerryBlender).playerContinueResponses[0] == LINKCMD_SEND_LINK_TYPE {
                (*sBerryBlender).gameEndState = 5;
            } else if (*sBerryBlender).playerContinueResponses[0] == LINKCMD_BLENDER_STOP {
                if (*sBerryBlender).canceledPlayerCmd == LINKCMD_BLENDER_NO_BERRIES {
                    (*sBerryBlender).gameEndState = 2;
                } else if (*sBerryBlender).canceledPlayerCmd == LINKCMD_BLENDER_NO_PBLOCK_SPACE {
                    (*sBerryBlender).gameEndState = 1;
                } else {
                    (*sBerryBlender).gameEndState = 5;
                }
            }
        }
        1 => {
            (*sBerryBlender).gameEndState = 3;
            StringCopy(
                gStringVar4.as_mut_ptr(),
                gLinkPlayers[(*sBerryBlender).canceledPlayerId]
                    .name
                    .as_mut_ptr(),
            );
            StringAppend(
                gStringVar4.as_mut_ptr(),
                sText_ApostropheSPokeblockCaseIsFull.as_ptr().cast_mut(),
            );
        }
        2 => {
            (*sBerryBlender).gameEndState += 1;
            StringCopy(
                gStringVar4.as_mut_ptr(),
                gLinkPlayers[(*sBerryBlender).canceledPlayerId]
                    .name
                    .as_mut_ptr(),
            );
            StringAppend(
                gStringVar4.as_mut_ptr(),
                sText_HasNoBerriesToPut.as_ptr().cast_mut(),
            );
        }
        3 => {
            if PrintMessage(
                &raw mut (*sBerryBlender).textState,
                gStringVar4.as_mut_ptr(),
                GetPlayerTextSpeedDelay() as i32,
            ) != 0
            {
                (*sBerryBlender).framesToWait = 0;
                (*sBerryBlender).gameEndState += 1;
            }
        }
        4 => {
            if ({
                (*sBerryBlender).framesToWait += 1;
                (*sBerryBlender).framesToWait
            }) > 60
            {
                (*sBerryBlender).gameEndState = 5;
            }
        }
        5 => {
            PrintMessage(
                &raw mut (*sBerryBlender).textState,
                gText_SavingDontTurnOff2.as_ptr().cast_mut(),
                0,
            );
            SetLinkStandbyCallback();
            (*sBerryBlender).gameEndState += 1;
        }
        6 => {
            if IsLinkTaskFinished() != 0 {
                (*sBerryBlender).framesToWait = 0;
                (*sBerryBlender).gameEndState += 1;
                (*sBerryBlender).linkPlayAgainState = 0;
            }
        }
        7 => {
            if LinkPlayAgainHandleSaving() != 0 {
                PlaySE(SE_SAVE);
                (*sBerryBlender).gameEndState += 1;
            }
        }
        8 => {
            (*sBerryBlender).gameEndState += 1;
            SetLinkStandbyCallback();
        }
        9 => {
            if IsLinkTaskFinished() != 0 {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
                (*sBerryBlender).gameEndState += 1;
            }
        }
        10 => {
            if gPaletteFade.active() == 0 {
                if (*sBerryBlender).playerContinueResponses[0] == LINKCMD_SEND_LINK_TYPE {
                    FreeAllWindowBuffers();
                    UnsetBgTilemapBuffer(2);
                    UnsetBgTilemapBuffer(1);
                    Free(sBerryBlender as *mut c_void);
                    sBerryBlender = null_mut();
                    SetMainCallback2(Some(DoBerryBlending));
                } else {
                    (*sBerryBlender).framesToWait = 0;
                    (*sBerryBlender).gameEndState += 1;
                }
            }
        }
        11 => {
            if ({
                (*sBerryBlender).framesToWait += 1;
                (*sBerryBlender).framesToWait
            }) > 30
            {
                SetCloseLinkCallback();
                (*sBerryBlender).gameEndState += 1;
            }
        }
        12 => {
            if gReceivedRemoteLinkPlayers == 0 {
                Free(sBerryBlender as *mut c_void);
                sBerryBlender = null_mut();
                SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
            }
        }
        _ => {}
    }
    ProcessLinkPlayerCmds();
    Blender_DummiedOutFunc((*sBerryBlender).bg_X as i16, (*sBerryBlender).bg_Y as i16);
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn CB2_CheckPlayAgainLocal() {
    match (*sBerryBlender).gameEndState {
        0 => {
            if (*sBerryBlender).playAgainState == PLAY_AGAIN_YES
                || (*sBerryBlender).playAgainState == PLAY_AGAIN_NO
            {
                (*sBerryBlender).gameEndState = 9;
            }
            if (*sBerryBlender).playAgainState == CANT_PLAY_NO_BERRIES {
                (*sBerryBlender).gameEndState = 2;
            }
            if (*sBerryBlender).playAgainState == CANT_PLAY_NO_PKBLCK_SPACE {
                (*sBerryBlender).gameEndState = 1;
            }
        }
        1 => {
            (*sBerryBlender).gameEndState = 3;
            (*sBerryBlender).textState = 0;
            StringCopy(
                gStringVar4.as_mut_ptr(),
                sText_YourPokeblockCaseIsFull.as_ptr().cast_mut(),
            );
        }
        2 => {
            (*sBerryBlender).gameEndState += 1;
            (*sBerryBlender).textState = 0;
            StringCopy(
                gStringVar4.as_mut_ptr(),
                sText_RunOutOfBerriesForBlending.as_ptr().cast_mut(),
            );
        }
        3 => {
            if PrintMessage(
                &raw mut (*sBerryBlender).textState,
                gStringVar4.as_mut_ptr(),
                GetPlayerTextSpeedDelay() as i32,
            ) != 0
            {
                (*sBerryBlender).gameEndState = 9;
            }
        }
        9 => {
            BeginFastPaletteFade(3);
            (*sBerryBlender).gameEndState += 1;
        }
        10 => {
            if gPaletteFade.active() == 0 {
                if (*sBerryBlender).playAgainState == PLAY_AGAIN_YES {
                    SetMainCallback2(Some(DoBerryBlending));
                } else {
                    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                }
                FreeAllWindowBuffers();
                UnsetBgTilemapBuffer(2);
                UnsetBgTilemapBuffer(1);
                Free(sBerryBlender as *mut c_void);
                sBerryBlender = null_mut();
            }
        }
        _ => {}
    }
    ProcessLinkPlayerCmds();
    Blender_DummiedOutFunc((*sBerryBlender).bg_X as i16, (*sBerryBlender).bg_Y as i16);
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn ProcessLinkPlayerCmds() {
    if gReceivedRemoteLinkPlayers != 0 {
        if CheckRecvCmdMatches(
            gRecvCmds[0][0],
            LINKCMD_SEND_PACKET,
            RFUCMD_SEND_PACKET as u16,
        ) != 0
        {
            if gRecvCmds[0][1] == LINKCMD_BLENDER_STOP {
                match gRecvCmds[0][2] {
                    LINKCMD_CONT_BLOCK => {
                        (*sBerryBlender).canceledPlayerCmd = LINKCMD_CONT_BLOCK;
                        (*sBerryBlender).canceledPlayerId = gRecvCmds[0][3];
                    }
                    LINKCMD_BLENDER_NO_BERRIES => {
                        (*sBerryBlender).canceledPlayerCmd = LINKCMD_BLENDER_NO_BERRIES;
                        (*sBerryBlender).canceledPlayerId = gRecvCmds[0][3];
                    }
                    LINKCMD_BLENDER_NO_PBLOCK_SPACE => {
                        (*sBerryBlender).canceledPlayerCmd = LINKCMD_BLENDER_NO_PBLOCK_SPACE;
                        (*sBerryBlender).canceledPlayerId = gRecvCmds[0][3];
                    }
                    _ => {}
                }
                (*sBerryBlender).playerContinueResponses[0] = LINKCMD_BLENDER_STOP;
            } else if gRecvCmds[0][1] == LINKCMD_SEND_LINK_TYPE {
                (*sBerryBlender).playerContinueResponses[0] = LINKCMD_SEND_LINK_TYPE;
            }
        }
        if GetMultiplayerId() == 0
            && (*sBerryBlender).playerContinueResponses[0] != LINKCMD_BLENDER_STOP
            && (*sBerryBlender).playerContinueResponses[0] != LINKCMD_SEND_LINK_TYPE
        {
            let mut i: u8 = 0;
            i = 0;
            while i < GetLinkPlayerCount() {
                if CheckRecvCmdMatches(
                    gRecvCmds[i][0],
                    LINKCMD_SEND_PACKET,
                    RFUCMD_SEND_PACKET as u16,
                ) != 0
                {
                    match gRecvCmds[i][1] {
                        LINKCMD_CONT_BLOCK => {
                            (*sBerryBlender).playerContinueResponses[i] = LINKCMD_CONT_BLOCK;
                        }
                        LINKCMD_BLENDER_PLAY_AGAIN => {
                            (*sBerryBlender).playerContinueResponses[i] =
                                LINKCMD_BLENDER_PLAY_AGAIN;
                        }
                        LINKCMD_BLENDER_NO_BERRIES => {
                            (*sBerryBlender).playerContinueResponses[i] =
                                LINKCMD_BLENDER_NO_BERRIES;
                        }
                        LINKCMD_BLENDER_NO_PBLOCK_SPACE => {
                            (*sBerryBlender).playerContinueResponses[i] =
                                LINKCMD_BLENDER_NO_PBLOCK_SPACE;
                        }
                        _ => {}
                    }
                }
                i += 1;
            }
            i = 0;
            while i < GetLinkPlayerCount() {
                if (*sBerryBlender).playerContinueResponses[i] == 0 {
                    break;
                }
                i += 1;
            }
            if i == GetLinkPlayerCount() {
                i = 0;
                while i < GetLinkPlayerCount() {
                    if (*sBerryBlender).playerContinueResponses[i] != LINKCMD_BLENDER_PLAY_AGAIN {
                        break;
                    }
                    i += 1;
                }
                SendContinuePromptResponse(&raw mut gSendCmd[0]);
                if i == GetLinkPlayerCount() {
                    gSendCmd[1] = LINKCMD_SEND_LINK_TYPE;
                } else {
                    gSendCmd[1] = LINKCMD_BLENDER_STOP;
                    gSendCmd[2] = (*sBerryBlender).playerContinueResponses[i];
                    gSendCmd[3] = i as u16;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawBlenderCenter(dest: *mut BgAffineSrcData) {
    let mut affineSrc: BgAffineSrcData = zeroed();
    affineSrc.texX = 30720;
    affineSrc.texY = 20480;
    affineSrc.scrX = 120 - (*sBerryBlender).bg_X as i16;
    affineSrc.scrY = 80 - (*sBerryBlender).bg_Y as i16;
    affineSrc.sx = (*sBerryBlender).centerScale as i16;
    affineSrc.sy = (*sBerryBlender).centerScale as i16;
    affineSrc.alpha = (*sBerryBlender).arrowPos;
    *dest = affineSrc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBlenderArrowPosition() -> u16 {
    return (*sBerryBlender).arrowPos;
}
pub(crate) unsafe extern "C" fn UpdateBlenderCenter() {
    let mut playerId: u8 = 0;
    if gReceivedRemoteLinkPlayers != 0 {
        playerId = GetMultiplayerId();
    }
    if gWirelessCommType != 0 && gReceivedRemoteLinkPlayers != 0 {
        if playerId == 0 {
            (*sBerryBlender).arrowPos += (*sBerryBlender).speed as u16;
            gSendCmd[5] = (*sBerryBlender).progressBarValue;
            gSendCmd[6] = (*sBerryBlender).arrowPos;
            DrawBlenderCenter(&raw mut (*sBerryBlender).bgAffineSrc);
        } else {
            if gRecvCmds[0][0] as i32 & RFUCMD_MASK == RFUCMD_BLENDER_SEND_KEYS as i32 {
                (*sBerryBlender).progressBarValue = gRecvCmds[0][5];
                (*sBerryBlender).arrowPos = gRecvCmds[0][6];
                DrawBlenderCenter(&raw mut (*sBerryBlender).bgAffineSrc);
            }
        }
    } else {
        (*sBerryBlender).arrowPos += (*sBerryBlender).speed as u16;
        DrawBlenderCenter(&raw mut (*sBerryBlender).bgAffineSrc);
    }
}
pub(crate) unsafe extern "C" fn SetBgPos() {
    SetGpuReg(REG_OFFSET_BG1HOFS, (*sBerryBlender).bg_X);
    SetGpuReg(REG_OFFSET_BG1VOFS, (*sBerryBlender).bg_Y);
    SetGpuReg(REG_OFFSET_BG0HOFS, (*sBerryBlender).bg_X);
    SetGpuReg(REG_OFFSET_BG0VOFS, (*sBerryBlender).bg_Y);
}
pub(crate) unsafe extern "C" fn SpriteCB_Particle(sprite: *mut Sprite) {
    (*sprite).data[2] += (*sprite).data[0];
    (*sprite).data[3] += (*sprite).data[1];
    (*sprite).x2 = (*sprite).data[2] / 8;
    (*sprite).y2 = (*sprite).data[3] / 8;
    if (*sprite).animEnded() != 0 {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreateParticleSprites() {
    let mut limit: i32 = Random() as i32 % 2 + 1;
    let mut i: i32 = 0;
    i = 0;
    while i < limit {
        let mut rand: u16 = 0;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut spriteId: u8 = 0;
        rand = (*sBerryBlender).arrowPos + (Random() as i32 % 20) as u16;
        x = (gSineTable[(rand as i32 & 0xFF) + 64] / 4) as i32;
        y = (gSineTable[rand as i32 & 0xFF] / 4) as i32;
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Particles).cast_mut(),
            x as i16 + 120,
            y as i16 + 80,
            1,
        );
        gSprites[spriteId].data[0] = 16 - (Random() as i32 % 32) as i16;
        gSprites[spriteId].data[1] = 16 - (Random() as i32 % 32) as i16;
        gSprites[spriteId].callback = Some(SpriteCB_Particle);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ScoreSymbol(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    (*sprite).y2 = -((*sprite).data[0] / 3);
    if (*sprite).animEnded() != 0 {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ScoreSymbolBest(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    (*sprite).y2 = -((*sprite).data[0] * 2);
    if (*sprite).y2 < -12 {
        (*sprite).y2 = -12;
    }
    if (*sprite).animEnded() != 0 {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn SetPlayerBerryData(playerId: u8, itemId: u16) {
    (*sBerryBlender).chosenItemId[playerId] = itemId;
    ConvertItemToBlenderBerry(&raw mut (*sBerryBlender).blendedBerries[playerId], itemId);
}
pub(crate) unsafe extern "C" fn SpriteCB_CountdownNumber(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).data[1] += 8;
            if (*sprite).data[1] > 88 {
                (*sprite).data[1] = 88;
                (*sprite).data[0] += 1;
                PlaySE(SE_BALL_BOUNCE_1);
            }
        }
        1 => {
            if ({
                (*sprite).data[2] += 1;
                (*sprite).data[2]
            }) > 20
            {
                (*sprite).data[0] += 1;
                (*sprite).data[2] = 0;
            }
        }
        2 => {
            (*sprite).data[1] += 4;
            if (*sprite).data[1] > 176 {
                if ({
                    (*sprite).data[3] += 1;
                    (*sprite).data[3]
                }) == 3
                {
                    DestroySprite(sprite);
                    CreateSprite((&raw const *sSpriteTemplate_Start).cast_mut(), 120, -20, 2);
                } else {
                    (*sprite).data[0] = 0;
                    (*sprite).data[1] = -16;
                    StartSpriteAnim(sprite, (*sprite).data[3] as u8);
                }
            }
        }
        _ => {}
    }
    (*sprite).y2 = (*sprite).data[1];
}
pub(crate) unsafe extern "C" fn SpriteCB_Start(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).data[1] += 8;
            if (*sprite).data[1] > 92 {
                (*sprite).data[1] = 92;
                (*sprite).data[0] += 1;
                PlaySE(SE_PIN);
            }
        }
        1 => {
            (*sprite).data[2] += 1;
            if (*sprite).data[2] > 20 {
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            (*sprite).data[1] += 4;
            if (*sprite).data[1] > 176 {
                (*sBerryBlender).mainState += 1;
                DestroySprite(sprite);
            }
        }
        _ => {}
    }
    (*sprite).y2 = (*sprite).data[1];
}
pub(crate) unsafe extern "C" fn TryUpdateProgressBar(current: u16, limit: u16) {
    if (*sBerryBlender).maxProgressBarValue < current {
        (*sBerryBlender).maxProgressBarValue += 2;
        UpdateProgressBar((*sBerryBlender).maxProgressBarValue, limit);
    }
}
pub(crate) unsafe extern "C" fn UpdateProgressBar(value: u16, limit: u16) {
    let mut amountFilled: i32 = 0;
    let mut maxFilledSegment: i32 = 0;
    let mut subSegmentsFilled: i32 = 0;
    let mut i: i32 = 0;
    let mut vram: *mut u16 = null_mut();
    vram = 0x6006000 as usize as *mut u16;
    amountFilled = div_i32(value as i32 * 64, limit as i32);
    maxFilledSegment = amountFilled / 8;
    i = 0;
    while i < maxFilledSegment {
        *vram.at(11 + i) = PROGRESS_BAR_FILLED_TOP;
        *vram.at(43 + i) = PROGRESS_BAR_FILLED_BOTTOM;
        i += 1;
    }
    subSegmentsFilled = amountFilled % 8;
    if subSegmentsFilled != 0 {
        *vram.at(11 + i) = subSegmentsFilled as u16 + PROGRESS_BAR_EMPTY_TOP;
        *vram.at(43 + i) = subSegmentsFilled as u16 + PROGRESS_BAR_EMPTY_BOTTOM;
        i += 1;
    }
    while i < 8 {
        *vram.at(11 + i) = PROGRESS_BAR_EMPTY_TOP;
        *vram.at(43 + i) = PROGRESS_BAR_EMPTY_BOTTOM;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ArrowSpeedToRPM(speed: u16) -> u32 {
    return (360000 * speed as i32 / 0x10000) as u32;
}
pub(crate) unsafe extern "C" fn UpdateRPM(speed: u16) {
    let mut i: u8 = 0;
    let mut digits: CArray<u8, 5> = zeroed();
    let mut currentRPM: u32 = ArrowSpeedToRPM(speed);
    if ((*sBerryBlender).maxRPM as u32) < currentRPM {
        (*sBerryBlender).maxRPM = currentRPM as u16;
    }
    i = 0;
    while i < 5 {
        digits[i] = (currentRPM % 10) as u8;
        currentRPM = currentRPM / 10;
        i += 1;
    }
    *(100688984 as usize as *mut u16) = digits[4] as u16 + RPM_DIGIT;
    *(100688986 as usize as *mut u16) = digits[3] as u16 + RPM_DIGIT;
    *(100688988 as usize as *mut u16) = digits[2] as u16 + RPM_DIGIT;
    *(100688992 as usize as *mut u16) = digits[1] as u16 + RPM_DIGIT;
    *(100688994 as usize as *mut u16) = digits[0] as u16 + RPM_DIGIT;
}
pub(crate) unsafe extern "C" fn ShakeBgCoordForHit(coord: *mut i16, speed: u16) {
    if *coord == 0 {
        *coord = rem_i32(Random() as i32, speed as i32) as i16 - (speed as i32 / 2) as i16;
    }
}
pub(crate) unsafe extern "C" fn RestoreBgCoord(coord: *mut i16) {
    if *coord < 0 {
        *coord += 1;
    }
    if *coord > 0 {
        *coord -= 1;
    }
}
pub(crate) unsafe extern "C" fn RestoreBgCoords() {
    RestoreBgCoord(&raw mut (*sBerryBlender).bg_X as *mut i16);
    RestoreBgCoord(&raw mut (*sBerryBlender).bg_Y as *mut i16);
}
pub(crate) unsafe extern "C" fn BlenderLandShakeBgCoord(coord: *mut i16, timer: u16) {
    let mut strength: i32 = 0;
    if timer < 10 {
        strength = 16;
    } else {
        strength = 8;
    }
    if *coord == 0 {
        *coord = rem_i32(Random() as i32, strength) as i16 - (strength / 2) as i16;
    } else {
        if *coord < 0 {
            *coord += 1;
        }
        if *coord > 0 {
            *coord -= 1;
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateBlenderLandScreenShake() -> u8 {
    if (*sBerryBlender).framesToWait == 0 {
        (*sBerryBlender).bg_X = 0;
        (*sBerryBlender).bg_Y = 0;
    }
    (*sBerryBlender).framesToWait += 1;
    BlenderLandShakeBgCoord(
        &raw mut (*sBerryBlender).bg_X as *mut i16,
        (*sBerryBlender).framesToWait as u16,
    );
    BlenderLandShakeBgCoord(
        &raw mut (*sBerryBlender).bg_Y as *mut i16,
        (*sBerryBlender).framesToWait as u16,
    );
    if (*sBerryBlender).framesToWait == 20 {
        (*sBerryBlender).bg_X = 0;
        (*sBerryBlender).bg_Y = 0;
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerArrow(sprite: *mut Sprite) {
    (*sprite).x2 = -((*sBerryBlender).bg_X as i16);
    (*sprite).y2 = -((*sBerryBlender).bg_Y as i16);
}
pub(crate) unsafe extern "C" fn TryUpdateBerryBlenderRecord() {
    if (*gSaveBlock1Ptr).berryBlenderRecords[(*sBerryBlender).numPlayers as i32 - 2]
        < (*sBerryBlender).maxRPM
    {
        (*gSaveBlock1Ptr).berryBlenderRecords[(*sBerryBlender).numPlayers as i32 - 2] =
            (*sBerryBlender).maxRPM;
    }
}
pub(crate) unsafe extern "C" fn PrintBlendingResults() -> u8 {
    let mut i: u16 = 0;
    let mut xPos: i32 = 0;
    let mut yPos: i32 = 0;
    let mut pokeblock: Pokeblock = zeroed();
    let mut flavors: CArray<u8, 6> = zeroed();
    let mut text: CArray<u8, 40> = zeroed();
    let mut berryIds: CArray<u16, 4> = zeroed();
    match (*sBerryBlender).mainState {
        0 => {
            (*sBerryBlender).mainState += 1;
            (*sBerryBlender).framesToWait = 17;
        }
        1 => {
            (*sBerryBlender).framesToWait -= 10;
            if (*sBerryBlender).framesToWait < 0 {
                (*sBerryBlender).framesToWait = 0;
                (*sBerryBlender).mainState += 1;
            }
        }
        2 => {
            if ({
                (*sBerryBlender).framesToWait += 1;
                (*sBerryBlender).framesToWait
            }) > 20
            {
                i = 0;
                while i < NUM_SCORE_TYPES as u16 {
                    DestroySprite(&raw mut gSprites[(*sBerryBlender).scoreIconIds[i]]);
                    i += 1;
                }
                (*sBerryBlender).framesToWait = 0;
                (*sBerryBlender).mainState += 1;
            }
        }
        3 => {
            let mut minutes: u16 = 0;
            let mut seconds: u16 = 0;
            let mut txtPtr: *mut u8 = null_mut();
            xPos = GetStringCenterAlignXOffset(
                FONT_NORMAL as i32,
                sText_BlendingResults.as_ptr().cast_mut(),
                0xA8,
            );
            Blender_AddTextPrinter(
                WIN_RESULTS,
                sText_BlendingResults.as_ptr().cast_mut(),
                xPos as u8,
                1,
                TEXT_SKIP_DRAW as i32,
                0,
            );
            if (*sBerryBlender).numPlayers == BLENDER_MAX_PLAYERS as u8 {
                yPos = 17;
            } else {
                yPos = 21;
            }
            i = 0;
            while i < (*sBerryBlender).numPlayers as u16 {
                let mut place: u8 = (*sBerryBlender).playerPlaces[i];
                ConvertIntToDecimalStringN(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    i as i32 + 1,
                    STR_CONV_MODE_LEFT_ALIGN,
                    1,
                );
                StringAppend(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    sText_Dot.as_ptr().cast_mut(),
                );
                StringAppend(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    gText_Space.as_ptr().cast_mut(),
                );
                StringAppend(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    gLinkPlayers[place].name.as_mut_ptr(),
                );
                Blender_AddTextPrinter(
                    WIN_RESULTS,
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    8,
                    yPos as u8,
                    TEXT_SKIP_DRAW as i32,
                    3,
                );
                StringCopy(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    (*sBerryBlender).blendedBerries[place].name.as_mut_ptr(),
                );
                ConvertInternationalString(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    gLinkPlayers[place].language as u8,
                );
                StringAppend(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    sText_SpaceBerry.as_ptr().cast_mut(),
                );
                Blender_AddTextPrinter(
                    WIN_RESULTS,
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    0x54,
                    yPos as u8,
                    TEXT_SKIP_DRAW as i32,
                    3,
                );
                yPos += 16;
                i += 1;
            }
            Blender_AddTextPrinter(
                WIN_RESULTS,
                sText_MaximumSpeed.as_ptr().cast_mut(),
                0,
                0x51,
                TEXT_SKIP_DRAW as i32,
                3,
            );
            ConvertIntToDecimalStringN(
                (*sBerryBlender).stringVar.as_mut_ptr(),
                (*sBerryBlender).maxRPM as i32 / 100,
                STR_CONV_MODE_RIGHT_ALIGN,
                3,
            );
            StringAppend(
                (*sBerryBlender).stringVar.as_mut_ptr(),
                sText_Dot.as_ptr().cast_mut(),
            );
            ConvertIntToDecimalStringN(
                text.as_mut_ptr(),
                (*sBerryBlender).maxRPM as i32 % 100,
                STR_CONV_MODE_LEADING_ZEROS,
                2,
            );
            StringAppend((*sBerryBlender).stringVar.as_mut_ptr(), text.as_mut_ptr());
            StringAppend(
                (*sBerryBlender).stringVar.as_mut_ptr(),
                sText_RPM.as_ptr().cast_mut(),
            );
            xPos = GetStringRightAlignXOffset(
                FONT_NORMAL as i32,
                (*sBerryBlender).stringVar.as_mut_ptr(),
                0xA8,
            );
            Blender_AddTextPrinter(
                WIN_RESULTS,
                (*sBerryBlender).stringVar.as_mut_ptr(),
                xPos as u8,
                0x51,
                TEXT_SKIP_DRAW as i32,
                3,
            );
            Blender_AddTextPrinter(
                WIN_RESULTS,
                sText_Time.as_ptr().cast_mut(),
                0,
                0x61,
                TEXT_SKIP_DRAW as i32,
                3,
            );
            seconds = ((*sBerryBlender).gameFrameTime / 60 % 60) as u16;
            minutes = ((*sBerryBlender).gameFrameTime / 3600) as u16;
            ConvertIntToDecimalStringN(
                (*sBerryBlender).stringVar.as_mut_ptr(),
                minutes as i32,
                STR_CONV_MODE_LEADING_ZEROS,
                2,
            );
            txtPtr = StringAppend(
                (*sBerryBlender).stringVar.as_mut_ptr(),
                sText_Min.as_ptr().cast_mut(),
            );
            ConvertIntToDecimalStringN(txtPtr, seconds as i32, STR_CONV_MODE_LEADING_ZEROS, 2);
            StringAppend(
                (*sBerryBlender).stringVar.as_mut_ptr(),
                sText_Sec.as_ptr().cast_mut(),
            );
            xPos = GetStringRightAlignXOffset(
                FONT_NORMAL as i32,
                (*sBerryBlender).stringVar.as_mut_ptr(),
                0xA8,
            );
            Blender_AddTextPrinter(
                WIN_RESULTS,
                (*sBerryBlender).stringVar.as_mut_ptr(),
                xPos as u8,
                0x61,
                TEXT_SKIP_DRAW as i32,
                3,
            );
            (*sBerryBlender).framesToWait = 0;
            (*sBerryBlender).mainState += 1;
            CopyWindowToVram(WIN_RESULTS, COPYWIN_GFX);
        }
        4 => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                (*sBerryBlender).mainState += 1;
            }
        }
        5 => {
            ClearStdWindowAndFrameToTransparent(WIN_RESULTS, TRUE);
            i = 0;
            while i < BLENDER_MAX_PLAYERS as u16 {
                if (*sBerryBlender).chosenItemId[i] != 0 {
                    berryIds[i] = (*sBerryBlender).chosenItemId[i] - ITEM_CHERI_BERRY;
                }
                if (*sBerryBlender).arrowIdToPlayerId[i] != NO_PLAYER {
                    PutWindowTilemap(i as u8);
                    CopyWindowToVram(i as u8, COPYWIN_FULL);
                }
                i += 1;
            }
            Debug_SetStageVars();
            CalculatePokeblock(
                (*sBerryBlender).blendedBerries.as_mut_ptr(),
                &raw mut pokeblock,
                (*sBerryBlender).numPlayers,
                flavors.as_mut_ptr(),
                (*sBerryBlender).maxRPM,
            );
            PrintMadePokeblockString(&raw mut pokeblock, (*sBerryBlender).stringVar.as_mut_ptr());
            TryAddContestLinkTvShow(&raw mut pokeblock, &raw mut (*sBerryBlender).tvBlender);
            CreateTask(Some(Task_PlayPokeblockFanfare), 6);
            IncrementDailyBerryBlender();
            RemoveBagItem(gSpecialVar_ItemId, 1);
            AddPokeblock(&raw mut pokeblock);
            (*sBerryBlender).textState = 0;
            (*sBerryBlender).mainState += 1;
        }
        6 => {
            if PrintMessage(
                &raw mut (*sBerryBlender).textState,
                (*sBerryBlender).stringVar.as_mut_ptr(),
                GetPlayerTextSpeedDelay() as i32,
            ) != 0
            {
                TryUpdateBerryBlenderRecord();
                return TRUE;
            }
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn PrintMadePokeblockString(
    pokeblock: *mut Pokeblock,
    mut dst: *mut u8,
) {
    let mut text: CArray<u8, 12> = zeroed();
    let mut flavorLvl: u8 = 0;
    let mut feel: u8 = 0;
    *dst = EOS;
    StringCopy(dst, gPokeblockNames[(*pokeblock).color]);
    StringAppend(dst, sText_WasMade.as_ptr().cast_mut());
    StringAppend(dst, sText_NewLine.as_ptr().cast_mut());
    flavorLvl = GetHighestPokeblocksFlavorLevel(pokeblock);
    feel = GetPokeblocksFeel(pokeblock);
    StringAppend(dst, sText_TheLevelIs.as_ptr().cast_mut());
    ConvertIntToDecimalStringN(
        text.as_mut_ptr(),
        flavorLvl as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    StringAppend(dst, text.as_mut_ptr());
    StringAppend(dst, sText_TheFeelIs.as_ptr().cast_mut());
    ConvertIntToDecimalStringN(text.as_mut_ptr(), feel as i32, STR_CONV_MODE_LEFT_ALIGN, 3);
    StringAppend(dst, text.as_mut_ptr());
    StringAppend(dst, sText_Dot2.as_ptr().cast_mut());
    StringAppend(dst, sText_NewParagraph.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn SortBasedOnPoints(
    mut places: *mut u8,
    playersNum: u8,
    scores: *mut u32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < playersNum as i32 {
        j = 0;
        while j < playersNum as i32 {
            if *scores.at(*places.at(i)) > *scores.at(*places.at(j)) {
                let mut temp: u8 = 0;
                temp = *places.at(i);
                *places.at(i) = *places.at(j);
                *places.at(j) = temp;
            }
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SortScores() {
    let mut playerId: u8 = 0;
    let mut i: u8 = 0;
    let mut places: CArray<u8, 4> = zeroed();
    let mut points: CArray<u32, 4> = zeroed();
    i = 0;
    while i < (*sBerryBlender).numPlayers {
        places[i] = i;
        i += 1;
    }
    i = 0;
    while i < (*sBerryBlender).numPlayers {
        points[i] = 0xf4240 * (*sBerryBlender).scores[i][0] as u32;
        points[i] += 1000 * (*sBerryBlender).scores[i][1] as u32;
        points[i] += 1000 - (*sBerryBlender).scores[i][2] as u32;
        i += 1;
    }
    SortBasedOnPoints(
        places.as_mut_ptr(),
        (*sBerryBlender).numPlayers,
        points.as_mut_ptr(),
    );
    i = 0;
    while i < (*sBerryBlender).numPlayers {
        (*sBerryBlender).playerPlaces[i] = places[i];
        i += 1;
    }
    if gReceivedRemoteLinkPlayers == 0 {
        playerId = 0;
    } else {
        playerId = GetMultiplayerId();
    }
    i = 0;
    while i < (*sBerryBlender).numPlayers {
        if (*sBerryBlender).playerPlaces[i] == playerId {
            (*sBerryBlender).ownRanking = i;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn PrintBlendingRanking() -> u8 {
    let mut i: u16 = 0;
    let mut xPos: i32 = 0;
    let mut yPos: i32 = 0;
    match (*sBerryBlender).mainState {
        0 => {
            (*sBerryBlender).mainState += 1;
            (*sBerryBlender).framesToWait = 255;
        }
        1 => {
            (*sBerryBlender).framesToWait -= 10;
            if (*sBerryBlender).framesToWait < 0 {
                (*sBerryBlender).framesToWait = 0;
                (*sBerryBlender).mainState += 1;
            }
        }
        2 => {
            if ({
                (*sBerryBlender).framesToWait += 1;
                (*sBerryBlender).framesToWait
            }) > 20
            {
                (*sBerryBlender).framesToWait = 0;
                (*sBerryBlender).mainState += 1;
            }
        }
        3 => {
            DrawStdFrameWithCustomTileAndPalette(WIN_RESULTS, FALSE, 1, 0xD);
            xPos = GetStringCenterAlignXOffset(
                FONT_NORMAL as i32,
                sText_Ranking.as_ptr().cast_mut(),
                168,
            );
            Blender_AddTextPrinter(
                WIN_RESULTS,
                sText_Ranking.as_ptr().cast_mut(),
                xPos as u8,
                1,
                TEXT_SKIP_DRAW as i32,
                0,
            );
            (*sBerryBlender).scoreIconIds[0] = CreateSprite(
                (&raw const *sSpriteTemplate_ScoreSymbols).cast_mut(),
                128,
                52,
                0,
            );
            StartSpriteAnim(
                &raw mut gSprites[(*sBerryBlender).scoreIconIds[0]],
                SCOREANIM_BEST_STATIC,
            );
            gSprites[(*sBerryBlender).scoreIconIds[0]].callback = Some(SpriteCallbackDummy);
            (*sBerryBlender).scoreIconIds[1] = CreateSprite(
                (&raw const *sSpriteTemplate_ScoreSymbols).cast_mut(),
                160,
                52,
                0,
            );
            gSprites[(*sBerryBlender).scoreIconIds[1]].callback = Some(SpriteCallbackDummy);
            (*sBerryBlender).scoreIconIds[2] = CreateSprite(
                (&raw const *sSpriteTemplate_ScoreSymbols).cast_mut(),
                192,
                52,
                0,
            );
            StartSpriteAnim(
                &raw mut gSprites[(*sBerryBlender).scoreIconIds[2]],
                SCOREANIM_MISS,
            );
            gSprites[(*sBerryBlender).scoreIconIds[2]].callback = Some(SpriteCallbackDummy);
            SortScores();
            yPos = 41;
            i = 0;
            while i < (*sBerryBlender).numPlayers as u16 {
                let mut place: u8 = (*sBerryBlender).playerPlaces[i];
                ConvertIntToDecimalStringN(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    i as i32 + 1,
                    STR_CONV_MODE_LEFT_ALIGN,
                    1,
                );
                StringAppend(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    sText_Dot.as_ptr().cast_mut(),
                );
                StringAppend(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    gText_Space.as_ptr().cast_mut(),
                );
                StringAppend(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    gLinkPlayers[place].name.as_mut_ptr(),
                );
                Blender_AddTextPrinter(
                    WIN_RESULTS,
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    0,
                    yPos as u8,
                    TEXT_SKIP_DRAW as i32,
                    3,
                );
                ConvertIntToDecimalStringN(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    (*sBerryBlender).scores[place][0] as i32,
                    STR_CONV_MODE_RIGHT_ALIGN,
                    3,
                );
                Blender_AddTextPrinter(
                    WIN_RESULTS,
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    78,
                    yPos as u8,
                    TEXT_SKIP_DRAW as i32,
                    3,
                );
                ConvertIntToDecimalStringN(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    (*sBerryBlender).scores[place][1] as i32,
                    STR_CONV_MODE_RIGHT_ALIGN,
                    3,
                );
                Blender_AddTextPrinter(
                    WIN_RESULTS,
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    110,
                    yPos as u8,
                    TEXT_SKIP_DRAW as i32,
                    3,
                );
                ConvertIntToDecimalStringN(
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    (*sBerryBlender).scores[place][2] as i32,
                    STR_CONV_MODE_RIGHT_ALIGN,
                    3,
                );
                Blender_AddTextPrinter(
                    WIN_RESULTS,
                    (*sBerryBlender).stringVar.as_mut_ptr(),
                    142,
                    yPos as u8,
                    TEXT_SKIP_DRAW as i32,
                    3,
                );
                yPos += 16;
                i += 1;
            }
            PutWindowTilemap(WIN_RESULTS);
            CopyWindowToVram(WIN_RESULTS, COPYWIN_FULL);
            (*sBerryBlender).framesToWait = 0;
            (*sBerryBlender).mainState += 1;
        }
        4 => {
            if ({
                (*sBerryBlender).framesToWait += 1;
                (*sBerryBlender).framesToWait
            }) > 20
            {
                (*sBerryBlender).mainState += 1;
            }
        }
        5 => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                (*sBerryBlender).mainState += 1;
            }
        }
        6 => {
            (*sBerryBlender).mainState = 0;
            return TRUE;
        }
        _ => {}
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowBerryBlenderRecordWindow() {
    let mut i: i32 = 0;
    let mut xPos: i32 = 0;
    let mut yPos: i32 = 0;
    let mut winTemplate: WindowTemplate = zeroed();
    let mut text: CArray<u8, 32> = zeroed();
    winTemplate = *sBlenderRecordWindowTemplate;
    gRecordsWindowId = AddWindow(&raw mut winTemplate) as u8;
    DrawStdWindowFrame(gRecordsWindowId, FALSE);
    FillWindowPixelBuffer(gRecordsWindowId, 17);
    xPos = GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        gText_BlenderMaxSpeedRecord.as_ptr().cast_mut(),
        144,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_BlenderMaxSpeedRecord.as_ptr().cast_mut(),
        xPos as u8,
        1,
        0,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_234Players.as_ptr().cast_mut(),
        4,
        41,
        0,
        None,
    );
    i = 0;
    yPos = 41;
    while i < NUM_SCORE_TYPES {
        let mut txtPtr: *mut u8 = null_mut();
        let mut record: u32 = 0;
        record = (*gSaveBlock1Ptr).berryBlenderRecords[i] as u32;
        txtPtr = ConvertIntToDecimalStringN(
            text.as_mut_ptr(),
            (record / 100) as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            3,
        );
        txtPtr = StringAppend(txtPtr, sText_Dot.as_ptr().cast_mut());
        txtPtr = ConvertIntToDecimalStringN(
            txtPtr,
            (record % 100) as i32,
            STR_CONV_MODE_LEADING_ZEROS,
            2,
        );
        txtPtr = StringAppend(txtPtr, sText_RPM.as_ptr().cast_mut());
        xPos = GetStringRightAlignXOffset(FONT_NORMAL as i32, text.as_mut_ptr(), 140);
        AddTextPrinterParameterized(
            gRecordsWindowId,
            FONT_NORMAL,
            text.as_mut_ptr(),
            xPos as u8,
            yPos as u8 + i as u8 * 16,
            0,
            None,
        );
        i += 1;
    }
    PutWindowTilemap(gRecordsWindowId);
    CopyWindowToVram(gRecordsWindowId, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn Task_PlayPokeblockFanfare(taskId: u8) {
    if gTasks[taskId].data[0] == 0 {
        PlayFanfare(MUS_LEVEL_UP);
        gTasks[taskId].data[0] += 1;
    }
    if IsFanfareTaskInactive() != 0 {
        PlayBGM((*sBerryBlender).savedMusic);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn TryAddContestLinkTvShow(
    pokeblock: *mut Pokeblock,
    tvBlender: *mut TvBlenderStruct,
) -> u32 {
    let mut flavorLevel: u8 = GetHighestPokeblocksFlavorLevel(pokeblock);
    let mut sheen: u16 =
        div_i32(flavorLevel as i32 * 10, GetPokeblocksFeel(pokeblock) as i32) as u16;
    (*tvBlender).pokeblockSheen = sheen as u8;
    (*tvBlender).pokeblockColor = (*pokeblock).color;
    (*tvBlender).name[0] = EOS;
    if gReceivedRemoteLinkPlayers != 0 {
        if (*sBerryBlender).ownRanking == 0 && sheen > 20 {
            StringCopy(
                (*tvBlender).name.as_mut_ptr(),
                gLinkPlayers[(*sBerryBlender).playerPlaces[(*sBerryBlender).numPlayers as i32 - 1]]
                    .name
                    .as_mut_ptr(),
            );
            (*tvBlender).pokeblockFlavor = GetPokeblocksFlavor(pokeblock);
            if Put3CheersForPokeblocksOnTheAir(
                (*tvBlender).name.as_mut_ptr(),
                (*tvBlender).pokeblockFlavor,
                (*tvBlender).pokeblockColor,
                (*tvBlender).pokeblockSheen,
                gLinkPlayers[(*sBerryBlender).playerPlaces[(*sBerryBlender).numPlayers as i32 - 1]]
                    .language as u8,
            ) != 0
            {
                return TRUE as u32;
            }
            return FALSE as u32;
        } else if (*sBerryBlender).ownRanking as i32 == (*sBerryBlender).numPlayers as i32 - 1
            && sheen <= 20
        {
            StringCopy(
                (*tvBlender).name.as_mut_ptr(),
                gLinkPlayers[(*sBerryBlender).playerPlaces[0]]
                    .name
                    .as_mut_ptr(),
            );
            (*tvBlender).pokeblockFlavor = GetPokeblocksFlavor(pokeblock);
            if Put3CheersForPokeblocksOnTheAir(
                (*tvBlender).name.as_mut_ptr(),
                (*tvBlender).pokeblockFlavor,
                (*tvBlender).pokeblockColor,
                (*tvBlender).pokeblockSheen,
                gLinkPlayers[(*sBerryBlender).playerPlaces[0]].language as u8,
            ) != 0
            {
                return TRUE as u32;
            }
            return FALSE as u32;
        }
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn Blender_AddTextPrinter(
    windowId: u8,
    string: *mut u8,
    x: u8,
    y: u8,
    speed: i32,
    caseId: i32,
) {
    let mut txtColor: CArray<u8, 3> = zeroed();
    let mut letterSpacing: u32 = 0;
    match caseId {
        1 => {
            txtColor[0] = 0x0;
            txtColor[1] = TEXT_COLOR_DARK_GRAY;
            txtColor[2] = TEXT_COLOR_LIGHT_GRAY;
        }
        2 => {
            txtColor[0] = 0x0;
            txtColor[1] = TEXT_COLOR_RED;
            txtColor[2] = TEXT_COLOR_LIGHT_RED;
        }
        _ => {
            txtColor[0] = TEXT_COLOR_WHITE;
            txtColor[1] = TEXT_COLOR_DARK_GRAY;
            txtColor[2] = TEXT_COLOR_LIGHT_GRAY;
        }
    }
    if caseId != 3 {
        FillWindowPixelBuffer(windowId, txtColor[0] | txtColor[0] << 4);
    }
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        x,
        y,
        letterSpacing as u8,
        1,
        txtColor.as_mut_ptr(),
        speed as i8,
        string,
    );
}
pub(crate) unsafe extern "C" fn PrintMessage(
    textState: *mut i16,
    string: *mut u8,
    textSpeed: i32,
) -> u32 {
    match *textState {
        0 => {
            DrawDialogFrameWithCustomTileAndPalette(WIN_MSG, FALSE, 0x14, 0xF);
            Blender_AddTextPrinter(WIN_MSG, string, 0, 1, textSpeed, 0);
            PutWindowTilemap(WIN_MSG);
            CopyWindowToVram(WIN_MSG, COPYWIN_FULL);
            *textState += 1;
        }
        1 => {
            if IsTextPrinterActive(WIN_MSG) == 0 {
                *textState = 0;
                return TRUE as u32;
            }
        }
        _ => {}
    }
    return FALSE as u32;
}
