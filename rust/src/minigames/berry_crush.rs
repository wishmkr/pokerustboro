//! Translated from `src/berry_crush.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sBitTable sSyncPressBonus sIntroOutroVibrationData sVibrationData sMessages sBgTemplates sTextColorTable sWindowTemplate_Rankings sWindowTemplates_PlayerNames sWindowTemplates_Results sResultsWindowHeights sPressingSpeedConversionTable sCrusherBase_Pal sEffects_Pal sTimerDigits_Pal sCrusherBase_Gfx sImpact_Gfx sSparkle_Gfx sTimerDigits_Gfx sCrusherTop_Tilemap sContainerCap_Tilemap sBg_Tilemap sPlayerIdToPosId sPlayerCoords sImpactCoords sSparkleCoords sPlayerBerrySpriteTags sSpriteSheets sSpritePals sAnim_CrusherBase sAnim_Impact_Small sAnim_Impact_Big sAnim_Sparkle_Small sAnim_Sparkle_Big sAnim_Timer sAnim_PlayerBerry sAffineAnim_PlayerBerry_0 sAffineAnim_PlayerBerry_1 sAnims_CrusherBase sAnims_Impact sAnims_Sparkle sAnims_Timer sAnims_PlayerBerry sAffineAnims_PlayerBerry sSpriteTemplate_CrusherBase sSpriteTemplate_Impact sSpriteTemplate_Sparkle sSpriteTemplate_Timer sSpriteTemplate_PlayerBerry sDigitObjTemplates sResultsTexts sBerryCrushCommands sSparkleThresholds sBigSparkleThresholds sReceivedPlayerBitmasks

/// `struct BerryCrushGame`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BerryCrushGame {
    pub exitCallback: Option<unsafe extern "C" fn()>,
    pub cmdCallback: Option<unsafe extern "C" fn(*mut BerryCrushGame, *mut u8) -> u32>,
    pub localId: u8,
    pub playerCount: u8,
    pub taskId: u8,
    pub textSpeed: u8,
    pub cmdState: u8,
    pub unused: u8,
    pub nextCmd: u8,
    pub afterPalFadeCmd: u8,
    pub cmdTimer: u16,
    pub gameState: u16,
    pub playAgainState: u16,
    pub pressingSpeed: u16,
    pub targetAPresses: i16,
    pub totalAPresses: i16,
    pub powder: i32,
    pub targetDepth: i32,
    pub newDepth: u8,
    bits_37: u8,
    pub leaderTimer: u16,
    pub timer: u16,
    pub depth: i16,
    pub vibration: i16,
    pub bigSparkleCounter: i16,
    pub numBigSparkles: i16,
    pub numBigSparkleChecks: i16,
    pub sparkleCounter: i16,
    pub commandArgs: CArray<u8, 12>,
    pub sendCmd: CArray<u16, 6>,
    pub recvCmd: CArray<u16, 7>,
    pub localState: BerryCrushGame_LocalState,
    pub results: BerryCrushGame_Results,
    pub players: CArray<BerryCrushGame_Player, 5>,
    pub gfx: BerryCrushGame_Gfx,
}

impl BerryCrushGame {
    #[inline(always)]
    pub fn noRoomForPowder(&self) -> u8 {
        ((self.bits_37 as u32 >> 0) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_noRoomForPowder(&mut self, v: u8) {
        self.bits_37 = (self.bits_37 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn newRecord(&self) -> u8 {
        ((self.bits_37 as u32 >> 1) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_newRecord(&mut self, v: u8) {
        self.bits_37 = (self.bits_37 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn playedSound(&self) -> u8 {
        ((self.bits_37 as u32 >> 2) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_playedSound(&mut self, v: u8) {
        self.bits_37 = (self.bits_37 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn endGame(&self) -> u8 {
        ((self.bits_37 as u32 >> 3) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_endGame(&mut self, v: u8) {
        self.bits_37 = (self.bits_37 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn bigSparkle(&self) -> u8 {
        ((self.bits_37 as u32 >> 4) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_bigSparkle(&mut self, v: u8) {
        self.bits_37 = (self.bits_37 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn sparkleAmount(&self) -> u8 {
        ((self.bits_37 as u32 >> 5) & 0x7) as u8
    }
    #[inline(always)]
    pub fn set_sparkleAmount(&mut self, v: u8) {
        self.bits_37 = (self.bits_37 & !(0x7 << 5)) | ((v as u8 & 0x7) << 5);
    }
}

unsafe impl Sync for BerryCrushGame {}

/// `struct BerryCrushGame_Gfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BerryCrushGame_Gfx {
    pub counter: u8,
    pub vibrationIdx: u8,
    pub numVibrations: u8,
    pub vibrating: u8,
    pub minutes: i16,
    pub secondsInt: i16,
    pub secondsFrac: i16,
    pub playerCoords: CArray<*mut BerryCrushPlayerCoords, 5>,
    pub coreSprite: *mut Sprite,
    pub impactSprites: CArray<*mut Sprite, 5>,
    pub berrySprites: CArray<*mut Sprite, 5>,
    pub sparkleSprites: CArray<*mut Sprite, 11>,
    pub timerSprites: CArray<*mut Sprite, 2>,
    pub resultsState: u8,
    pub unused: u8,
    pub resultsWindowId: u8,
    pub nameWindowIds: CArray<u8, 5>,
    pub bgBuffers: CArray<CArray<u16, 2048>, 4>,
}

unsafe impl Sync for BerryCrushGame_Gfx {}

/// `struct BerryCrushGame_LinkState`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct BerryCrushGame_LinkState {
    pub rfuCmd: u16,
    pub sendFlag: u16,
    bits_4: u8,
    pub vibration: i8,
    pub depth: u16,
    pub timer: u16,
    pub inputFlags: u16,
    pub sparkleAmount: u16,
}

impl BerryCrushGame_LinkState {
    #[inline(always)]
    pub fn endGame(&self) -> u8 {
        ((self.bits_4 as u32 >> 0) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_endGame(&mut self, v: u8) {
        self.bits_4 = (self.bits_4 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn bigSparkle(&self) -> u8 {
        ((self.bits_4 as u32 >> 1) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_bigSparkle(&mut self, v: u8) {
        self.bits_4 = (self.bits_4 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn pushedAButton(&self) -> u8 {
        ((self.bits_4 as u32 >> 2) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_pushedAButton(&mut self, v: u8) {
        self.bits_4 = (self.bits_4 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn playerPressedAFlags(&self) -> u8 {
        ((self.bits_4 as u32 >> 3) & 0x1f) as u8
    }
    #[inline(always)]
    pub fn set_playerPressedAFlags(&mut self, v: u8) {
        self.bits_4 = (self.bits_4 & !(0x1f << 3)) | ((v as u8 & 0x1f) << 3);
    }
}

unsafe impl Sync for BerryCrushGame_LinkState {}

/// `struct BerryCrushGame_Results`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BerryCrushGame_Results {
    pub powder: u32,
    pub time: u16,
    pub targetPressesPerSec: u16,
    pub silkiness: u16,
    pub totalAPresses: u16,
    pub stats: CArray<CArray<u16, 5>, 2>,
    pub playerIdsRanked: CArray<CArray<u8, 8>, 2>,
}

unsafe impl Sync for BerryCrushGame_Results {}

/// `struct BerryCrushPlayerCoords`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct BerryCrushPlayerCoords {
    pub playerId: u8,
    pub windowGfxX: u8,
    pub windowGfxY: u8,
    pub impactXOffset: i16,
    pub impactYOffset: i16,
    pub berryXOffset: i16,
    pub berryXDest: i16,
}

unsafe impl Sync for BerryCrushPlayerCoords {}

/// `struct BerryCrushGame_LocalState`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct BerryCrushGame_LocalState {
    pub sendFlag: u16,
    bits_2: u8,
    pub vibration: i8,
    pub depth: u16,
    pub timer: u16,
    pub inputFlags: u16,
    pub sparkleAmount: u16,
}

impl BerryCrushGame_LocalState {
    #[inline(always)]
    pub fn endGame(&self) -> u8 {
        ((self.bits_2 as u32 >> 0) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_endGame(&mut self, v: u8) {
        self.bits_2 = (self.bits_2 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn bigSparkle(&self) -> u8 {
        ((self.bits_2 as u32 >> 1) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_bigSparkle(&mut self, v: u8) {
        self.bits_2 = (self.bits_2 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn pushedAButton(&self) -> u8 {
        ((self.bits_2 as u32 >> 2) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_pushedAButton(&mut self, v: u8) {
        self.bits_2 = (self.bits_2 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn playerPressedAFlags(&self) -> u8 {
        ((self.bits_2 as u32 >> 3) & 0x1f) as u8
    }
    #[inline(always)]
    pub fn set_playerPressedAFlags(&mut self, v: u8) {
        self.bits_2 = (self.bits_2 & !(0x1f << 3)) | ((v as u8 & 0x1f) << 3);
    }
}

unsafe impl Sync for BerryCrushGame_LocalState {}

/// `struct BerryCrushGame_Player`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct BerryCrushGame_Player {
    pub name: CArray<u8, 12>,
    pub berryId: u16,
    pub inputTime: u16,
    pub neatInputStreak: u16,
    pub timeSincePrevInput: u16,
    pub maxNeatInputStreak: u16,
    pub numAPresses: u16,
    pub numSyncedAPresses: u16,
    pub timePressingA: u16,
    pub inputFlags: u8,
    pub inputState: u8,
}

unsafe impl Sync for BerryCrushGame_Player {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<BerryCrushGame>() == 16832);
    assert!(offset_of!(BerryCrushGame, exitCallback) == 0);
    assert!(offset_of!(BerryCrushGame, cmdCallback) == 4);
    assert!(offset_of!(BerryCrushGame, localId) == 8);
    assert!(offset_of!(BerryCrushGame, playerCount) == 9);
    assert!(offset_of!(BerryCrushGame, taskId) == 10);
    assert!(offset_of!(BerryCrushGame, textSpeed) == 11);
    assert!(offset_of!(BerryCrushGame, cmdState) == 12);
    assert!(offset_of!(BerryCrushGame, unused) == 13);
    assert!(offset_of!(BerryCrushGame, nextCmd) == 14);
    assert!(offset_of!(BerryCrushGame, afterPalFadeCmd) == 15);
    assert!(offset_of!(BerryCrushGame, cmdTimer) == 16);
    assert!(offset_of!(BerryCrushGame, gameState) == 18);
    assert!(offset_of!(BerryCrushGame, playAgainState) == 20);
    assert!(offset_of!(BerryCrushGame, pressingSpeed) == 22);
    assert!(offset_of!(BerryCrushGame, targetAPresses) == 24);
    assert!(offset_of!(BerryCrushGame, totalAPresses) == 26);
    assert!(offset_of!(BerryCrushGame, powder) == 28);
    assert!(offset_of!(BerryCrushGame, targetDepth) == 32);
    assert!(offset_of!(BerryCrushGame, newDepth) == 36);
    assert!(offset_of!(BerryCrushGame, bits_37) == 37);
    assert!(offset_of!(BerryCrushGame, leaderTimer) == 38);
    assert!(offset_of!(BerryCrushGame, timer) == 40);
    assert!(offset_of!(BerryCrushGame, depth) == 42);
    assert!(offset_of!(BerryCrushGame, vibration) == 44);
    assert!(offset_of!(BerryCrushGame, bigSparkleCounter) == 46);
    assert!(offset_of!(BerryCrushGame, numBigSparkles) == 48);
    assert!(offset_of!(BerryCrushGame, numBigSparkleChecks) == 50);
    assert!(offset_of!(BerryCrushGame, sparkleCounter) == 52);
    assert!(offset_of!(BerryCrushGame, commandArgs) == 54);
    assert!(offset_of!(BerryCrushGame, sendCmd) == 66);
    assert!(offset_of!(BerryCrushGame, recvCmd) == 78);
    assert!(offset_of!(BerryCrushGame, localState) == 92);
    assert!(offset_of!(BerryCrushGame, results) == 104);
    assert!(offset_of!(BerryCrushGame, players) == 152);
    assert!(offset_of!(BerryCrushGame, gfx) == 312);
    assert!(size_of::<BerryCrushGame_Gfx>() == 16520);
    assert!(offset_of!(BerryCrushGame_Gfx, counter) == 0);
    assert!(offset_of!(BerryCrushGame_Gfx, vibrationIdx) == 1);
    assert!(offset_of!(BerryCrushGame_Gfx, numVibrations) == 2);
    assert!(offset_of!(BerryCrushGame_Gfx, vibrating) == 3);
    assert!(offset_of!(BerryCrushGame_Gfx, minutes) == 4);
    assert!(offset_of!(BerryCrushGame_Gfx, secondsInt) == 6);
    assert!(offset_of!(BerryCrushGame_Gfx, secondsFrac) == 8);
    assert!(offset_of!(BerryCrushGame_Gfx, playerCoords) == 12);
    assert!(offset_of!(BerryCrushGame_Gfx, coreSprite) == 32);
    assert!(offset_of!(BerryCrushGame_Gfx, impactSprites) == 36);
    assert!(offset_of!(BerryCrushGame_Gfx, berrySprites) == 56);
    assert!(offset_of!(BerryCrushGame_Gfx, sparkleSprites) == 76);
    assert!(offset_of!(BerryCrushGame_Gfx, timerSprites) == 120);
    assert!(offset_of!(BerryCrushGame_Gfx, resultsState) == 128);
    assert!(offset_of!(BerryCrushGame_Gfx, unused) == 129);
    assert!(offset_of!(BerryCrushGame_Gfx, resultsWindowId) == 130);
    assert!(offset_of!(BerryCrushGame_Gfx, nameWindowIds) == 131);
    assert!(offset_of!(BerryCrushGame_Gfx, bgBuffers) == 136);
    assert!(size_of::<BerryCrushGame_LinkState>() == 16);
    assert!(offset_of!(BerryCrushGame_LinkState, rfuCmd) == 0);
    assert!(offset_of!(BerryCrushGame_LinkState, sendFlag) == 2);
    assert!(offset_of!(BerryCrushGame_LinkState, bits_4) == 4);
    assert!(offset_of!(BerryCrushGame_LinkState, vibration) == 5);
    assert!(offset_of!(BerryCrushGame_LinkState, depth) == 6);
    assert!(offset_of!(BerryCrushGame_LinkState, timer) == 8);
    assert!(offset_of!(BerryCrushGame_LinkState, inputFlags) == 10);
    assert!(offset_of!(BerryCrushGame_LinkState, sparkleAmount) == 12);
    assert!(size_of::<BerryCrushGame_Results>() == 48);
    assert!(offset_of!(BerryCrushGame_Results, powder) == 0);
    assert!(offset_of!(BerryCrushGame_Results, time) == 4);
    assert!(offset_of!(BerryCrushGame_Results, targetPressesPerSec) == 6);
    assert!(offset_of!(BerryCrushGame_Results, silkiness) == 8);
    assert!(offset_of!(BerryCrushGame_Results, totalAPresses) == 10);
    assert!(offset_of!(BerryCrushGame_Results, stats) == 12);
    assert!(offset_of!(BerryCrushGame_Results, playerIdsRanked) == 32);
    assert!(size_of::<BerryCrushPlayerCoords>() == 12);
    assert!(offset_of!(BerryCrushPlayerCoords, playerId) == 0);
    assert!(offset_of!(BerryCrushPlayerCoords, windowGfxX) == 1);
    assert!(offset_of!(BerryCrushPlayerCoords, windowGfxY) == 2);
    assert!(offset_of!(BerryCrushPlayerCoords, impactXOffset) == 4);
    assert!(offset_of!(BerryCrushPlayerCoords, impactYOffset) == 6);
    assert!(offset_of!(BerryCrushPlayerCoords, berryXOffset) == 8);
    assert!(offset_of!(BerryCrushPlayerCoords, berryXDest) == 10);
    assert!(size_of::<BerryCrushGame_LocalState>() == 12);
    assert!(offset_of!(BerryCrushGame_LocalState, sendFlag) == 0);
    assert!(offset_of!(BerryCrushGame_LocalState, bits_2) == 2);
    assert!(offset_of!(BerryCrushGame_LocalState, vibration) == 3);
    assert!(offset_of!(BerryCrushGame_LocalState, depth) == 4);
    assert!(offset_of!(BerryCrushGame_LocalState, timer) == 6);
    assert!(offset_of!(BerryCrushGame_LocalState, inputFlags) == 8);
    assert!(offset_of!(BerryCrushGame_LocalState, sparkleAmount) == 10);
    assert!(size_of::<BerryCrushGame_Player>() == 32);
    assert!(offset_of!(BerryCrushGame_Player, name) == 0);
    assert!(offset_of!(BerryCrushGame_Player, berryId) == 12);
    assert!(offset_of!(BerryCrushGame_Player, inputTime) == 14);
    assert!(offset_of!(BerryCrushGame_Player, neatInputStreak) == 16);
    assert!(offset_of!(BerryCrushGame_Player, timeSincePrevInput) == 18);
    assert!(offset_of!(BerryCrushGame_Player, maxNeatInputStreak) == 20);
    assert!(offset_of!(BerryCrushGame_Player, numAPresses) == 22);
    assert!(offset_of!(BerryCrushGame_Player, numSyncedAPresses) == 24);
    assert!(offset_of!(BerryCrushGame_Player, timePressingA) == 26);
    assert!(offset_of!(BerryCrushGame_Player, inputFlags) == 28);
    assert!(offset_of!(BerryCrushGame_Player, inputState) == 29);
};

const CMD_ASK_PICK_BERRY: u16 = 7;
const CMD_ASK_PLAY_AGAIN: u16 = 20;
const CMD_CALC_RESULTS: u16 = 17;
const CMD_CLOSE_LINK: u16 = 24;
const CMD_COMM_PLAY_AGAIN: u8 = 21;
const CMD_COUNTDOWN: u16 = 12;
const CMD_DROP_BERRIES: u16 = 10;
const CMD_DROP_LID: u16 = 11;
const CMD_FADE: u8 = 1;
const CMD_FINISH_GAME: u16 = 15;
const CMD_HIDE_GAME: u16 = 5;
const CMD_NONE: u16 = 0;
const CMD_PICK_BERRY: u8 = 8;
const CMD_PLAY_AGAIN_NO: u16 = 23;
const CMD_PLAY_AGAIN_YES: u16 = 22;
const CMD_PLAY_GAME_LEADER: u16 = 13;
const CMD_PLAY_GAME_MEMBER: u16 = 14;
const CMD_PRINT_MSG: u16 = 3;
const CMD_QUIT: u8 = 25;
const CMD_READY_BEGIN: u8 = 6;
const CMD_SAVE: u8 = 19;
const CMD_SHOW_GAME: u16 = 4;
const CMD_SHOW_RESULTS: u16 = 18;
const CMD_TIMES_UP: u16 = 16;
const CMD_WAIT_BERRIES: u8 = 9;
const CMD_WAIT_FADE: u8 = 2;
const COLORID_BLACK: i32 = 1;
const COLORID_BLUE: u8 = 3;
const COLORID_GRAY: i32 = 0;
const COLORID_GREEN: u8 = 4;
const COLORID_LIGHT_GRAY: i32 = 2;
const COLORID_RED: i32 = 5;
const CRUSHER_START_Y: i16 = -104;
const F_INPUT_HIT_A: u16 = 1;
const F_INPUT_HIT_B: u8 = 2;
const F_INPUT_HIT_SYNC: i32 = 4;
const F_MOVE_HORIZ: i32 = 32768;
const F_MSG_CLEAR: u8 = 1;
const F_MSG_EXPAND: i32 = 2;
const GFXTAG_IMPACT: u16 = 2;
const GFXTAG_SPARKLE: u16 = 3;
const INPUT_FLAGS_PER_PLAYER: u32 = 3;
const INPUT_FLAG_MASK: u16 = 7;
const INPUT_STATE_HIT: u8 = 1;
const INPUT_STATE_HIT_SYNC: u8 = 2;
const INPUT_STATE_NONE: u8 = 0;
const MASK_TARGET_Y: i16 = 32767;
const MAX_TIME: u16 = 36000;
const MSG_COMM_STANDBY: u8 = 8;
const MSG_DROPPED: i32 = 6;
const MSG_NO_BERRIES: i32 = 5;
const MSG_PICK_BERRY: u8 = 0;
const MSG_PLAY_AGAIN: u8 = 4;
const MSG_POWDER: u8 = 2;
const MSG_TIMES_UP: u8 = 7;
const MSG_WAIT_PICK: u8 = 1;
const NUM_RANDOM_RESULTS_PAGES: i32 = 3;
const NUM_RESULTS_PAGES: i32 = 3;
const PALTAG_EFFECT: u16 = 2;
const PLAY_AGAIN_NO: u16 = 1;
const PLAY_AGAIN_NO_BERRIES: u16 = 3;
const PLAY_AGAIN_YES: u16 = 0;
const RESULTS_PAGE_COOPERATIVE: u8 = 1;
const RESULTS_PAGE_CRUSHING: u8 = 2;
const RESULTS_PAGE_NEATNESS: u8 = 0;
const RESULTS_PAGE_POWER: u8 = 2;
const RESULTS_PAGE_PRESSES: u8 = 0;
const RESULTS_PAGE_RANDOM: u8 = 1;
const RUN_CMD: u8 = 0;
const SCHEDULE_CMD: u8 = 1;
const SEND_GAME_STATE: u16 = 2;
const STATE_COUNTDOWN: u16 = 6;
const STATE_DROP_BERRIES: u16 = 4;
const STATE_DROP_LID: u16 = 5;
const STATE_FINISHED: u16 = 8;
const STATE_INIT: u16 = 1;
const STATE_PICK_BERRY: u16 = 3;
const STATE_PLAYING: u16 = 7;
const STATE_PLAY_AGAIN: u16 = 15;
const STATE_RESET: u16 = 2;
const STATE_RESULTS_CRUSHING: u16 = 13;
const STATE_RESULTS_PRESSES: u16 = 11;
const STATE_RESULTS_RANDOM: u16 = 12;
const STATE_TIMES_UP: u16 = 9;
const TAG_COUNTDOWN: u16 = 4096;
const TAG_CRUSHER_BASE: u16 = 1;
const TAG_TIMER_DIGITS: u16 = 4;

static sBerryCrushCommands: Table<
    CArray<Option<unsafe extern "C" fn(*mut BerryCrushGame, *mut u8) -> u32>, 26>,
> = Table((&raw const crate::data::berry_crush::sBerryCrushCommands).cast());
static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::berry_crush::sBgTemplates).cast());
static sBg_Tilemap: Table<CArray<u8, 552>> =
    Table((&raw const crate::data::berry_crush::sBg_Tilemap).cast());
static sBigSparkleThresholds: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::berry_crush::sBigSparkleThresholds).cast());
static sBitTable: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::berry_crush::sBitTable).cast());
static sContainerCap_Tilemap: Table<CArray<u8, 408>> =
    Table((&raw const crate::data::berry_crush::sContainerCap_Tilemap).cast());
static sCrusherTop_Tilemap: Table<CArray<u8, 304>> =
    Table((&raw const crate::data::berry_crush::sCrusherTop_Tilemap).cast());
static sDigitObjTemplates: Table<CArray<DigitObjUtilTemplate, 3>> =
    Table((&raw const crate::data::berry_crush::sDigitObjTemplates).cast());
static sImpactCoords: Table<CArray<CArray<i8, 2>, 3>> =
    Table((&raw const crate::data::berry_crush::sImpactCoords).cast());
static sIntroOutroVibrationData: Table<CArray<CArray<i8, 7>, 5>> =
    Table((&raw const crate::data::berry_crush::sIntroOutroVibrationData).cast());
static sMessages: Table<CArray<*mut u8, 9>> =
    Table((&raw const crate::data::berry_crush::sMessages).cast());
static sPlayerBerrySpriteTags: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::berry_crush::sPlayerBerrySpriteTags).cast());
static sPlayerCoords: Table<CArray<BerryCrushPlayerCoords, 5>> =
    Table((&raw const crate::data::berry_crush::sPlayerCoords).cast());
static sPlayerIdToPosId: Table<CArray<CArray<u8, 5>, 4>> =
    Table((&raw const crate::data::berry_crush::sPlayerIdToPosId).cast());
static sPressingSpeedConversionTable: Table<CArray<u32, 8>> =
    Table((&raw const crate::data::berry_crush::sPressingSpeedConversionTable).cast());
static sReceivedPlayerBitmasks: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::berry_crush::sReceivedPlayerBitmasks).cast());
static sResultsTexts: Table<CArray<*mut u8, 6>> =
    Table((&raw const crate::data::berry_crush::sResultsTexts).cast());
static sResultsWindowHeights: Table<CArray<CArray<u8, 4>, 2>> =
    Table((&raw const crate::data::berry_crush::sResultsWindowHeights).cast());
static sSparkleCoords: Table<CArray<CArray<i8, 2>, 11>> =
    Table((&raw const crate::data::berry_crush::sSparkleCoords).cast());
static sSparkleThresholds: Table<CArray<CArray<u8, 4>, 4>> =
    Table((&raw const crate::data::berry_crush::sSparkleThresholds).cast());
static sSpritePals: Table<CArray<SpritePalette, 4>> =
    Table((&raw const crate::data::berry_crush::sSpritePals).cast());
static sSpriteSheets: Table<CArray<CompressedSpriteSheet, 5>> =
    Table((&raw const crate::data::berry_crush::sSpriteSheets).cast());
static sSpriteTemplate_CrusherBase: Table<SpriteTemplate> =
    Table((&raw const crate::data::berry_crush::sSpriteTemplate_CrusherBase).cast());
static sSpriteTemplate_Impact: Table<SpriteTemplate> =
    Table((&raw const crate::data::berry_crush::sSpriteTemplate_Impact).cast());
static sSpriteTemplate_PlayerBerry: Table<SpriteTemplate> =
    Table((&raw const crate::data::berry_crush::sSpriteTemplate_PlayerBerry).cast());
static sSpriteTemplate_Sparkle: Table<SpriteTemplate> =
    Table((&raw const crate::data::berry_crush::sSpriteTemplate_Sparkle).cast());
static sSpriteTemplate_Timer: Table<SpriteTemplate> =
    Table((&raw const crate::data::berry_crush::sSpriteTemplate_Timer).cast());
static sSyncPressBonus: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::berry_crush::sSyncPressBonus).cast());
static sTextColorTable: Table<CArray<CArray<u8, 3>, 6>> =
    Table((&raw const crate::data::berry_crush::sTextColorTable).cast());
static sVibrationData: Table<CArray<CArray<u8, 4>, 5>> =
    Table((&raw const crate::data::berry_crush::sVibrationData).cast());
static sWindowTemplate_Rankings: Table<WindowTemplate> =
    Table((&raw const crate::data::berry_crush::sWindowTemplate_Rankings).cast());
static sWindowTemplates_PlayerNames: Table<CArray<WindowTemplate, 6>> =
    Table((&raw const crate::data::berry_crush::sWindowTemplates_PlayerNames).cast());
static sWindowTemplates_Results: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::berry_crush::sWindowTemplates_Results).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGame: *mut BerryCrushGame = null_mut();

unsafe extern "C" {
    static gBerries: CArray<Berry, 0>;
    static gBerryCrush_BerryData: CArray<BerryCrushBerryData, 0>;
    static gBerryCrush_Crusher_Gfx: CArray<u32, 0>;
    static gBerryCrush_Crusher_Pal: CArray<u16, 0>;
    static gBerryCrush_TextWindows_Tilemap: CArray<u32, 0>;
    static mut gBlockRecvBuffer: CArray<CArray<u16, 128>, 5>;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecvCmds: CArray<CArray<u16, 8>, 5>;
    static mut gReservedSpritePaletteCount: u8;
    static mut gRfu: RfuManager;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_ItemId: u16;
    static mut gSpriteCoordOffsetX: i16;
    static mut gSpriteCoordOffsetY: i16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static mut gTextFlags: TextFlags;
    static gText_1DotBlueF700: CArray<u8, 0>;
    static gText_1DotF700: CArray<u8, 0>;
    static gText_BerryCrush2: CArray<u8, 0>;
    static gText_CrushingResults: CArray<u8, 0>;
    static gText_PressesRankings: CArray<u8, 0>;
    static gText_PressingSpeed: CArray<u8, 0>;
    static gText_PressingSpeedRankings: CArray<u8, 0>;
    static gText_SavingDontTurnOffPower: CArray<u8, 0>;
    static gText_Silkiness: CArray<u8, 0>;
    static gText_SpaceMin: CArray<u8, 0>;
    static gText_SpaceSec: CArray<u8, 0>;
    static gText_StrVar1: CArray<u8, 0>;
    static gText_TimeColon: CArray<u8, 0>;
    static gText_TimesPerSec: CArray<u8, 0>;
    static gText_Var1Percent: CArray<u8, 0>;
    static gText_Var1Players: CArray<u8, 0>;
    static gText_XDotY2: CArray<u8, 0>;
    static gText_XDotY3: CArray<u8, 0>;
    static mut gWirelessCommType: u8;
    fn AddCustomItemIconSprite(a0: *mut SpriteTemplate, a1: u16, a2: u16, a3: u16) -> u8;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
        a5: u8,
        a6: u8,
        a7: u8,
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
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB1_Overworld();
    fn CB2_ReturnToField();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChooseBerryForMachine(a0: Option<unsafe extern "C" fn()>);
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearRecvCommands();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DestroyWirelessStatusIndicatorSprite();
    fn DigitObjUtil_CreatePrinter(a0: u32, a1: i32, a2: *mut DigitObjUtilTemplate) -> u32;
    fn DigitObjUtil_DeletePrinter(a0: u32);
    fn DigitObjUtil_Free();
    fn DigitObjUtil_HideOrShow(a0: u32, a1: u32);
    fn DigitObjUtil_Init(a0: u32) -> u32;
    fn DigitObjUtil_PrintNumOn(a0: u32, a1: i32);
    fn DisplayYesNoMenuDefaultYes();
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBerryPowder() -> u32;
    fn GetBlockReceivedStatus() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetMultiplayerId() -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn GiveBerryPowder(a0: u32) -> u8;
    fn HasAtLeastOneBerry() -> u8;
    fn HideBg(a0: u8);
    fn IncrementGameStat(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitStandardTextBoxWindows();
    fn InitTextBoxGfxAndPrinters();
    fn IsLinkTaskFinished() -> u8;
    fn IsMinigameCountdownRunning() -> u32;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn LockPlayerFieldControls();
    fn MathUtil_Div16Shift(a0: u8, a1: i16, a2: i16) -> i16;
    fn MathUtil_Div32(a0: i32, a1: i32) -> i32;
    fn MathUtil_Mul16(a0: i16, a1: i16) -> i16;
    fn MathUtil_Mul16Shift(a0: u8, a1: i16, a2: i16) -> i16;
    fn MathUtil_Mul32(a0: i32, a1: i32) -> i32;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveBagItem(a0: u16, a1: u16) -> u8;
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetBlockReceivedFlags();
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTempTileDataBuffers();
    fn Rfu_SendPacket(a0: *mut c_void);
    fn Rfu_SetLinkStandbyCallback();
    fn RunTasks();
    fn RunTextPrinters();
    fn ScanlineEffect_Stop();
    fn ScriptContext_Enable();
    fn SendBlock(a0: u8, a1: *mut c_void, a2: u16) -> u8;
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetCloseLinkCallback();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback1(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartMinigameCountdown(a0: u16, a1: u16, a2: i16, a3: i16, a4: u8);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_LinkFullSave(a0: u8);
    fn TransferPlttBuffer();
    fn UnlockPlayerFieldControls();
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn GetBerryCrushGame() -> *mut BerryCrushGame {
    return sGame;
}
pub(crate) unsafe extern "C" fn QuitBerryCrush(
    mut exitCallback: Option<unsafe extern "C" fn()>,
) -> u32 {
    if sGame.is_null() {
        return 2;
    }
    if exitCallback.is_none() {
        exitCallback = (*sGame).exitCallback;
    }
    DestroyTask((*sGame).taskId);
    Free(sGame as *mut c_void);
    sGame = null_mut();
    SetMainCallback2(exitCallback);
    if exitCallback == Some(CB2_ReturnToField as unsafe extern "C" fn()) {
        gTextFlags.set_autoScroll(TRUE);
        PlayNewMapMusic(MUS_POKE_CENTER);
        SetMainCallback1(Some(CB1_Overworld));
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartBerryCrush(exitCallback: Option<unsafe extern "C" fn()>) {
    let mut playerCount: u8 = 0;
    let mut multiplayerId: u8 = 0;
    if gReceivedRemoteLinkPlayers == 0 || gWirelessCommType == 0 {
        SetMainCallback2(exitCallback);
        gRfu.errorParam0 = 0;
        gRfu.errorParam1 = 0;
        volatile_write(&raw mut gRfu.errorState, RFU_ERROR_STATE_OCCURRED);
        return;
    }
    playerCount = GetLinkPlayerCount();
    multiplayerId = GetMultiplayerId();
    if playerCount < 2 || multiplayerId >= playerCount {
        SetMainCallback2(exitCallback);
        gRfu.errorParam0 = 0;
        gRfu.errorParam1 = 0;
        volatile_write(&raw mut gRfu.errorState, RFU_ERROR_STATE_OCCURRED);
        return;
    }
    sGame = AllocZeroed(16832) as *mut BerryCrushGame;
    if sGame.is_null() {
        SetMainCallback2(exitCallback);
        gRfu.errorParam0 = 0;
        gRfu.errorParam1 = 0;
        volatile_write(&raw mut gRfu.errorState, RFU_ERROR_STATE_OCCURRED);
        return;
    }
    (*sGame).exitCallback = exitCallback;
    (*sGame).localId = multiplayerId;
    (*sGame).playerCount = playerCount;
    SetNamesAndTextSpeed(sGame);
    (*sGame).gameState = STATE_INIT;
    (*sGame).nextCmd = CMD_FADE;
    (*sGame).afterPalFadeCmd = CMD_READY_BEGIN;
    SetPaletteFadeArgs(
        (*sGame).commandArgs.as_mut_ptr(),
        TRUE,
        PALETTES_ALL,
        0,
        16,
        0,
        0,
    );
    RunOrScheduleCommand(CMD_SHOW_GAME, 1, (*sGame).commandArgs.as_mut_ptr());
    SetMainCallback2(Some(MainCB));
    (*sGame).taskId = CreateTask(Some(MainTask), 8);
    gTextFlags.set_autoScroll(FALSE);
}
pub(crate) unsafe extern "C" fn GetBerryFromBag() {
    if gSpecialVar_ItemId < ITEM_CHERI_BERRY || gSpecialVar_ItemId > 176 {
        gSpecialVar_ItemId = ITEM_CHERI_BERRY;
    } else {
        RemoveBagItem(gSpecialVar_ItemId, 1);
    }
    (*sGame).players[(*sGame).localId].berryId = gSpecialVar_ItemId - ITEM_CHERI_BERRY;
    (*sGame).nextCmd = CMD_FADE;
    (*sGame).afterPalFadeCmd = CMD_WAIT_BERRIES;
    SetPaletteFadeArgs(
        (*sGame).commandArgs.as_mut_ptr(),
        0,
        PALETTES_ALL,
        0,
        16,
        0,
        0,
    );
    RunOrScheduleCommand(CMD_SHOW_GAME, 1, (*sGame).commandArgs.as_mut_ptr());
    (*sGame).taskId = CreateTask(Some(MainTask), 8);
    SetMainCallback2(Some(MainCB));
}
pub(crate) unsafe extern "C" fn ChooseBerry() {
    DestroyTask((*sGame).taskId);
    ChooseBerryForMachine(Some(GetBerryFromBag));
}
pub(crate) unsafe extern "C" fn BerryCrush_SetVBlankCB() {
    SetVBlankCallback(Some(VBlankCB));
}
pub(crate) unsafe extern "C" fn BerryCrush_InitVBlankCB() {
    SetVBlankCallback(None);
}
pub(crate) unsafe extern "C" fn SaveResults() {
    let mut time: u32 = 0;
    let mut presses: u32 = 0;
    time = (*sGame).results.time as u32;
    time = (time as i32 as u32) << 8;
    time = MathUtil_Div32(time as i32, 15360) as u32;
    presses = (*sGame).results.totalAPresses as u32;
    presses = (presses as i32 as u32) << 8;
    presses = MathUtil_Div32(presses as i32, time as i32) as u32 & 0xFFFF;
    (*sGame).pressingSpeed = presses as u16;
    match (*sGame).playerCount {
        2 => {
            if (*sGame).pressingSpeed > (*gSaveBlock2Ptr).berryCrush.pressingSpeeds[0] {
                (*sGame).set_newRecord(TRUE);
                (*gSaveBlock2Ptr).berryCrush.pressingSpeeds[0] = (*sGame).pressingSpeed;
            }
        }
        3 => {
            if (*sGame).pressingSpeed > (*gSaveBlock2Ptr).berryCrush.pressingSpeeds[1] {
                (*sGame).set_newRecord(TRUE);
                (*gSaveBlock2Ptr).berryCrush.pressingSpeeds[1] = (*sGame).pressingSpeed;
            }
        }
        4 => {
            if (*sGame).pressingSpeed > (*gSaveBlock2Ptr).berryCrush.pressingSpeeds[2] {
                (*sGame).set_newRecord(TRUE);
                (*gSaveBlock2Ptr).berryCrush.pressingSpeeds[2] = (*sGame).pressingSpeed;
            }
        }
        5 => {
            if (*sGame).pressingSpeed > (*gSaveBlock2Ptr).berryCrush.pressingSpeeds[3] {
                (*sGame).set_newRecord(TRUE);
                (*gSaveBlock2Ptr).berryCrush.pressingSpeeds[3] = (*sGame).pressingSpeed;
            }
        }
        _ => {}
    }
    (*sGame).powder = (*sGame).results.powder as i32;
    if GiveBerryPowder((*sGame).powder as u32) != 0 {
        return;
    }
    (*sGame).set_noRoomForPowder(TRUE);
}
pub(crate) unsafe extern "C" fn VBlankCB() {
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
}
pub(crate) unsafe extern "C" fn MainCB() {
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
}
pub(crate) unsafe extern "C" fn MainTask(taskId: u8) {
    if (*sGame).cmdCallback.is_some() {
        (*sGame).cmdCallback.unwrap_unchecked()(sGame, (*sGame).commandArgs.as_mut_ptr());
    }
    UpdateGame(sGame);
}
pub(crate) unsafe extern "C" fn SetNamesAndTextSpeed(game: *mut BerryCrushGame) {
    let mut i: u8 = 0;
    i = 0;
    while i < (*game).playerCount {
        StringCopy(
            (*game).players[i].name.as_mut_ptr(),
            gLinkPlayers[i].name.as_mut_ptr(),
        );
        i += 1;
    }
    while i < MAX_RFU_PLAYERS as u8 {
        memset(
            (*game).players[i].name.as_mut_ptr(),
            1,
            PLAYER_NAME_LENGTH as u32,
        );
        (*game).players[i].name[7] = EOS;
        i += 1;
    }
    match (*gSaveBlock2Ptr).optionsTextSpeed() {
        OPTIONS_TEXT_SPEED_SLOW => {
            (*game).textSpeed = 8;
        }
        OPTIONS_TEXT_SPEED_MID => {
            (*game).textSpeed = 4;
        }
        OPTIONS_TEXT_SPEED_FAST => {
            (*game).textSpeed = 1;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ShowGameDisplay() -> i32 {
    let mut game: *mut BerryCrushGame = GetBerryCrushGame();
    if game.is_null() {
        return -1;
    }
    match (*game).cmdState {
        0 => {
            SetVBlankCallback(None);
            SetHBlankCallback(None);
            SetGpuReg(0x0, 0);
            ScanlineEffect_Stop();
            ResetTempTileDataBuffers();
        }
        1 => {
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        OAM as i32 as usize as *mut c_void,
                        0x1000200,
                    );
                }
            }
            gReservedSpritePaletteCount = 0;
            DigitObjUtil_Init(3);
        }
        2 => {
            ResetPaletteFade();
            ResetSpriteData();
            FreeAllSpritePalettes();
        }
        3 => {
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
            SetBgTilemapBuffer(1, (*game).gfx.bgBuffers[0].as_mut_ptr() as *mut c_void);
            SetBgTilemapBuffer(2, (*game).gfx.bgBuffers[2].as_mut_ptr() as *mut c_void);
            SetBgTilemapBuffer(3, (*game).gfx.bgBuffers[3].as_mut_ptr() as *mut c_void);
            ChangeBgX(0, 0, BG_COORD_SET);
            ChangeBgY(0, 0, BG_COORD_SET);
            ChangeBgX(2, 0, BG_COORD_SET);
            ChangeBgY(2, 0, BG_COORD_SET);
            ChangeBgX(3, 0, BG_COORD_SET);
            ChangeBgY(3, 0, BG_COORD_SET);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        }
        4 => {
            FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 32, 32);
            FillBgTilemapBufferRect_Palette0(1, 0, 0, 0, 32, 64);
            FillBgTilemapBufferRect_Palette0(2, 0, 0, 0, 32, 32);
            FillBgTilemapBufferRect_Palette0(3, 0, 0, 0, 32, 32);
        }
        5 => {
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(3);
            DecompressAndCopyTileDataToVram(
                1,
                gBerryCrush_Crusher_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
        }
        6 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return 0;
            }
            InitStandardTextBoxWindows();
            InitTextBoxGfxAndPrinters();
            CreatePlayerNameWindows(game);
            DrawPlayerNameWindows(game);
            gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
        }
        7 => {
            LoadPalette(
                gBerryCrush_Crusher_Pal.as_ptr().cast_mut() as *mut c_void,
                0,
                384,
            );
            CopyToBgTilemapBuffer(
                1,
                sCrusherTop_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                2,
                sContainerCap_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyToBgTilemapBuffer(3, sBg_Tilemap.as_ptr().cast_mut() as *mut c_void, 0, 0);
            CopyPlayerNameWindowGfxToBg(game);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(3);
        }
        8 => {
            LoadWirelessStatusIndicatorSpriteGfx();
            CreateWirelessStatusIndicatorSprite(0, 0);
            CreateGameSprites(game);
            SetGpuReg(
                REG_OFFSET_BG1VOFS,
                (gSpriteCoordOffsetY as u16).wrapping_neg(),
            );
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
        }
        9 => {
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            BlendPalettes(PALETTES_ALL, 16, 0);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            SetGpuRegBits(REG_OFFSET_DISPCNT, 4160);
            BerryCrush_SetVBlankCB();
            (*game).cmdState = 0;
            return 1;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn HideGameDisplay() -> i32 {
    let mut game: *mut BerryCrushGame = GetBerryCrushGame();
    if game.is_null() {
        return -1;
    }
    'l1: {
        let sw1: u8 = (*game).cmdState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            Rfu_SetLinkStandbyCallback();
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
        }
        if fall || sw1 == 2 {
            fall = true;
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            UpdatePaletteFade();
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if UpdatePaletteFade() != 0 {
                return 0;
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 32, 32);
            FillBgTilemapBufferRect_Palette0(1, 0, 0, 0, 32, 32);
            FillBgTilemapBufferRect_Palette0(2, 0, 0, 0, 32, 32);
            FillBgTilemapBufferRect_Palette0(3, 0, 0, 0, 32, 32);
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
            CopyBgTilemapBufferToVram(3);
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            FreeAllWindowBuffers();
            HideBg(0);
            UnsetBgTilemapBuffer(0);
            HideBg(1);
            UnsetBgTilemapBuffer(1);
            HideBg(2);
            UnsetBgTilemapBuffer(2);
            HideBg(3);
            UnsetBgTilemapBuffer(3);
            ClearGpuRegBits(REG_OFFSET_DISPCNT, 4160);
            break 'l1;
        }
        if sw1 == 6 {
            fall = true;
            DestroyWirelessStatusIndicatorSprite();
            DestroyGameSprites(game);
            DigitObjUtil_Free();
            break 'l1;
        }
        if sw1 == 7 {
            fall = true;
            (*game).cmdState = 0;
            return 1;
        }
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn UpdateGame(game: *mut BerryCrushGame) -> i32 {
    gSpriteCoordOffsetY = (*game).depth + (*game).vibration;
    SetGpuReg(
        REG_OFFSET_BG1VOFS,
        (gSpriteCoordOffsetY as u16).wrapping_neg(),
    );
    if (*game).gameState == STATE_PLAYING {
        PrintTimer(&raw mut (*game).gfx, (*game).timer);
    }
    return 0;
}
pub(crate) unsafe extern "C" fn ResetCrusherPos(game: *mut BerryCrushGame) {
    (*game).depth = CRUSHER_START_Y;
    (*game).vibration = 0;
    gSpriteCoordOffsetX = 0;
    gSpriteCoordOffsetY = CRUSHER_START_Y;
}
pub(crate) unsafe extern "C" fn CreateBerrySprites(
    game: *mut BerryCrushGame,
    gfx: *mut BerryCrushGame_Gfx,
) {
    let mut i: u8 = 0;
    let mut spriteId: u8 = 0;
    let mut distance: i16 = 0;
    let mut var1: i16 = 0;
    let mut data: *mut i16 = null_mut();
    let mut speed: i16 = 0;
    let mut var2: u32 = 0;
    i = 0;
    while i < (*game).playerCount {
        spriteId = AddCustomItemIconSprite(
            (&raw const *sSpriteTemplate_PlayerBerry).cast_mut(),
            sPlayerBerrySpriteTags[i],
            sPlayerBerrySpriteTags[i],
            (*game).players[i].berryId + ITEM_CHERI_BERRY,
        );
        (*gfx).berrySprites[i] = &raw mut gSprites[spriteId];
        (*(*gfx).berrySprites[i]).oam.set_priority(3);
        (*(*gfx).berrySprites[i]).set_affineAnimPaused(TRUE);
        (*(*gfx).berrySprites[i]).x = (*(*gfx).playerCoords[i]).berryXOffset + 120;
        (*(*gfx).berrySprites[i]).y = -16;
        data = (*(*gfx).berrySprites[i]).data.as_mut_ptr();
        speed = 512;
        *data.at(1) = speed;
        *data.at(2) = 32;
        *data.at(7) = 112;
        distance = (*(*gfx).playerCoords[i]).berryXDest - (*(*gfx).playerCoords[i]).berryXOffset;
        *data.at(6) = distance / 4;
        distance *= 128;
        var2 = speed as u32 + 32;
        var2 = var2 / 2;
        var1 = MathUtil_Div16Shift(7, (63.5f32 as f32 * 256 as f32) as i16, var2 as i16);
        *data = (*(*gfx).berrySprites[i]).x as u16 as i16 * 128;
        *data.at(3) = MathUtil_Div16Shift(7, distance, var1);
        var1 = MathUtil_Mul16Shift(7, var1, 85);
        *data.at(4) = 0;
        *data.at(5) = MathUtil_Div16Shift(7, (63.5f32 as f32 * 256 as f32) as i16, var1);
        *data.at(7) |= F_MOVE_HORIZ as i16;
        if (*(*gfx).playerCoords[i]).berryXOffset < 0 {
            StartSpriteAffineAnim((*gfx).berrySprites[i], 1);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DropBerryIntoCrusher(sprite: *mut Sprite) {
    let mut data: *mut i16 = (*sprite).data.as_mut_ptr();
    *data.at(1) += *data.at(2);
    (*sprite).y2 += *data.at(1) >> 8;
    if *data.at(7) as i32 & F_MOVE_HORIZ != 0 {
        (*sprite).data[0] += *data.at(3);
        *data.at(4) += *data.at(5);
        (*sprite).x2 = Sin(*data.at(4) >> 7, *data.at(6));
        if *data.at(7) as i32 & F_MOVE_HORIZ != 0 && *data.at(4) >> 7 > 126 {
            (*sprite).x2 = 0;
            *data.at(7) &= MASK_TARGET_Y;
        }
    }
    (*sprite).x = *data >> 7;
    if (*sprite).y as i32 + (*sprite).y2 as i32 >= *data.at(7) as i32 & MASK_TARGET_Y as i32 {
        (*sprite).callback = Some(SpriteCallbackDummy);
        FreeSpriteOamMatrix(sprite);
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn BerryCrushFreeBerrySpriteGfx(
    game: *mut BerryCrushGame,
    gfx: *mut BerryCrushGame_Gfx,
) {
    let mut i: u8 = 0;
    i = 0;
    while i < (*game).playerCount {
        FreeSpritePaletteByTag(sPlayerBerrySpriteTags[i]);
        FreeSpriteTilesByTag(sPlayerBerrySpriteTags[i]);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateInputEffects(
    game: *mut BerryCrushGame,
    gfx: *mut BerryCrushGame_Gfx,
) {
    let mut numPlayersPressed: u8 = 0;
    let mut linkState: *mut BerryCrushGame_LinkState = null_mut();
    let mut i: u8 = 0;
    let mut temp1: u16 = 0;
    let mut xModifier: u16 = 0;
    numPlayersPressed = 0;
    linkState = (*game).recvCmd.as_mut_ptr() as *mut BerryCrushGame_LinkState;
    i = 0;
    while i < (*game).playerCount {
        temp1 = shr_i32(
            (*linkState).inputFlags as i32,
            i as u32 * INPUT_FLAGS_PER_PLAYER,
        ) as u16;
        temp1 &= INPUT_FLAG_MASK;
        if temp1 != 0 {
            numPlayersPressed += 1;
            if temp1 as i32 & F_INPUT_HIT_SYNC != 0 {
                StartSpriteAnim((*gfx).impactSprites[i], 1);
            } else {
                StartSpriteAnim((*gfx).impactSprites[i], 0);
            }
            (*(*gfx).impactSprites[i]).set_invisible(FALSE as u16);
            (*(*gfx).impactSprites[i]).set_animPaused(FALSE);
            (*(*gfx).impactSprites[i]).x2 = sImpactCoords[(temp1 % 4) as i32 - 1][0] as i16;
            (*(*gfx).impactSprites[i]).y2 = sImpactCoords[(temp1 % 4) as i32 - 1][1] as i16;
        }
        i += 1;
    }
    if numPlayersPressed == 0 {
        (*game).set_playedSound(FALSE);
    } else {
        temp1 = ((*game).timer as i32 % 3) as u8 as u16;
        xModifier = temp1;
        i = 0;
        while (i as i32) < (*linkState).sparkleAmount as i32 * 2 + 3 {
            if (*(*gfx).sparkleSprites[i]).invisible() != 0 {
                (*(*gfx).sparkleSprites[i]).callback = Some(SpriteCB_Sparkle_Init);
                (*(*gfx).sparkleSprites[i]).x = sSparkleCoords[i][0] as i16 + 120;
                (*(*gfx).sparkleSprites[i]).y =
                    sSparkleCoords[i][1] as i16 + 136 - temp1 as i16 * 4;
                (*(*gfx).sparkleSprites[i]).x2 = sSparkleCoords[i][0] as i16
                    + div_i32(sSparkleCoords[i][0] as i32, xModifier as i32 * 4) as i16;
                (*(*gfx).sparkleSprites[i]).y2 = sSparkleCoords[i][1] as i16;
                if (*linkState).bigSparkle() != 0 {
                    StartSpriteAnim((*gfx).sparkleSprites[i], 1);
                } else {
                    StartSpriteAnim((*gfx).sparkleSprites[i], 0);
                }
                temp1 += 1;
                if temp1 > 3 {
                    temp1 = 0;
                }
            }
            i += 1;
        }
        if (*game).playedSound() != 0 {
            (*game).set_playedSound(FALSE);
        } else {
            if numPlayersPressed == 1 {
                PlaySE(SE_MUD_BALL);
            } else {
                PlaySE(SE_BREAKABLE_DOOR);
            }
            (*game).set_playedSound(TRUE);
        }
    }
}
pub(crate) unsafe extern "C" fn AreEffectsFinished(
    game: *mut BerryCrushGame,
    gfx: *mut BerryCrushGame_Gfx,
) -> u32 {
    let mut i: u8 = 0;
    i = 0;
    while i < (*game).playerCount {
        if (*(*gfx).impactSprites[i]).invisible() == 0 {
            return FALSE as u32;
        }
        i += 1;
    }
    i = 0;
    while i < 11 {
        if (*(*gfx).sparkleSprites[i]).invisible() == 0 {
            return FALSE as u32;
        }
        i += 1;
    }
    if (*game).vibration != 0 {
        (*game).vibration = 0;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn FramesToMinSec(gfx: *mut BerryCrushGame_Gfx, frames: u16) {
    let mut i: u8 = 0;
    let mut fractionalFrames: u32 = 0;
    let mut r3: i16 = 0;
    (*gfx).minutes = (frames as i32 / 3600) as i16;
    (*gfx).secondsInt = (frames as i32 % 3600 / 60) as i16;
    r3 = MathUtil_Mul16((frames as i32 % 60) as i16 * 256, 4);
    i = 0;
    while i < 8 {
        if shr_i32(r3 as i32, 7 - i as u32) & 1 != 0 {
            fractionalFrames += sPressingSpeedConversionTable[i];
        }
        i += 1;
    }
    (*gfx).secondsFrac = (fractionalFrames / 0xf4240) as i16;
}
pub(crate) unsafe extern "C" fn PrintTextCentered(
    windowId: u8,
    mut left: u8,
    colorId: u8,
    string: *mut u8,
) {
    left = left * 4 - (GetStringWidth(FONT_SHORT, string, -1) / 2) as u8;
    AddTextPrinterParameterized3(
        windowId,
        FONT_SHORT,
        left,
        0,
        sTextColorTable[colorId].as_ptr().cast_mut(),
        0,
        string,
    );
}
pub(crate) unsafe extern "C" fn PrintResultsText(
    game: *mut BerryCrushGame,
    page: u8,
    sp14: u8,
    mut baseY: u8,
) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut playerId: u8 = 0;
    let mut ranking: u8 = 0;
    let mut x: i32 = 0;
    let mut stat: u8 = 0;
    let mut results: *mut BerryCrushGame_Results = &raw mut (*game).results;
    let mut xOffset: u32 = 0;
    let mut y: i32 = 0;
    baseY -= 16;
    if page == RESULTS_PAGE_CRUSHING {
        baseY -= 42;
    }
    y = baseY as i32 - 14 * (*game).playerCount as i32;
    if y > 0 {
        y = y / 2 + 16;
    } else {
        y = 16;
    }
    i = 0;
    while i < (*game).playerCount {
        DynamicPlaceholderTextUtil_Reset();
        match page {
            RESULTS_PAGE_PRESSES => {
                playerId = (*results).playerIdsRanked[page][i];
                if i != 0 && (*results).stats[page][i] != (*results).stats[page][i as i32 - 1] {
                    ranking = i;
                }
                ConvertIntToDecimalStringN(
                    gStringVar4.as_mut_ptr(),
                    (*results).stats[page][i] as i32,
                    STR_CONV_MODE_RIGHT_ALIGN,
                    4,
                );
                StringAppend(gStringVar4.as_mut_ptr(), sResultsTexts[page]);
            }
            RESULTS_PAGE_RANDOM => {
                playerId = (*results).playerIdsRanked[page][i];
                if i != 0 && (*results).stats[page][i] != (*results).stats[page][i as i32 - 1] {
                    ranking = i;
                }
                ConvertIntToDecimalStringN(
                    gStringVar1.as_mut_ptr(),
                    ((*results).stats[page][i] >> 4) as i32,
                    STR_CONV_MODE_RIGHT_ALIGN,
                    3,
                );
                xOffset = 0;
                stat = (*results).stats[page][i] as u8 & 15;
                j = 0;
                while j < 4 {
                    if shr_i32(stat as i32, 3 - j as u32) & 1 != 0 {
                        xOffset += sPressingSpeedConversionTable[j];
                    }
                    j += 1;
                }
                stat = (xOffset / 0xf4240) as u8;
                ConvertIntToDecimalStringN(
                    gStringVar2.as_mut_ptr(),
                    stat as i32,
                    STR_CONV_MODE_LEADING_ZEROS,
                    2,
                );
                StringExpandPlaceholders(gStringVar4.as_mut_ptr(), sResultsTexts[page]);
            }
            RESULTS_PAGE_CRUSHING => {
                playerId = i;
                ranking = i;
                j = (*game).players[i].berryId as u8;
                if j >= 44 {
                    j = 0;
                }
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    gBerries[j].name.as_ptr().cast_mut(),
                );
                StringExpandPlaceholders(gStringVar4.as_mut_ptr(), sResultsTexts[page]);
            }
            _ => {}
        }
        x = GetStringRightAlignXOffset(
            FONT_SHORT as i32,
            gStringVar4.as_mut_ptr(),
            sp14 as i32 - 4,
        );
        AddTextPrinterParameterized3(
            (*game).gfx.resultsWindowId,
            FONT_SHORT,
            x as u8,
            y as u8,
            sTextColorTable[0].as_ptr().cast_mut(),
            0,
            gStringVar4.as_mut_ptr(),
        );
        if playerId == (*game).localId {
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gText_1DotBlueF700.as_ptr().cast_mut(),
            );
        } else {
            StringCopy(gStringVar3.as_mut_ptr(), gText_1DotF700.as_ptr().cast_mut());
        }
        gStringVar3[0] = ranking + CHAR_1;
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            0,
            (*game).players[playerId].name.as_mut_ptr(),
        );
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gStringVar3.as_mut_ptr(),
        );
        AddTextPrinterParameterized3(
            (*game).gfx.resultsWindowId,
            FONT_SHORT,
            4,
            y as u8,
            sTextColorTable[0].as_ptr().cast_mut(),
            0,
            gStringVar4.as_mut_ptr(),
        );
        y += 14;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn PrintCrushingResults(game: *mut BerryCrushGame) {
    let mut i: u8 = 0;
    let mut x: u8 = 0;
    let mut pressingSpeedFrac: u32 = 0;
    let mut results: *mut BerryCrushGame_Results = &raw mut (*game).results;
    let mut y: u8 = GetWindowAttribute((*game).gfx.resultsWindowId, WINDOW_HEIGHT) as u8 * 8 - 42;
    FramesToMinSec(&raw mut (*game).gfx, (*results).time);
    AddTextPrinterParameterized3(
        (*game).gfx.resultsWindowId,
        FONT_SHORT,
        x,
        y,
        sTextColorTable[0].as_ptr().cast_mut(),
        0,
        gText_TimeColon.as_ptr().cast_mut(),
    );
    x = 176 - GetStringWidth(FONT_SHORT, gText_SpaceSec.as_ptr().cast_mut(), -1) as u8;
    AddTextPrinterParameterized3(
        (*game).gfx.resultsWindowId,
        FONT_SHORT,
        x,
        y,
        sTextColorTable[0].as_ptr().cast_mut(),
        0,
        gText_SpaceSec.as_ptr().cast_mut(),
    );
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        (*game).gfx.secondsInt as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        (*game).gfx.secondsFrac as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), gText_XDotY2.as_ptr().cast_mut());
    x -= GetStringWidth(FONT_SHORT, gStringVar4.as_mut_ptr(), -1) as u8;
    AddTextPrinterParameterized3(
        (*game).gfx.resultsWindowId,
        FONT_SHORT,
        x,
        y,
        sTextColorTable[0].as_ptr().cast_mut(),
        0,
        gStringVar4.as_mut_ptr(),
    );
    x -= GetStringWidth(FONT_SHORT, gText_SpaceMin.as_ptr().cast_mut(), -1) as u8;
    AddTextPrinterParameterized3(
        (*game).gfx.resultsWindowId,
        FONT_SHORT,
        x,
        y,
        sTextColorTable[0].as_ptr().cast_mut(),
        0,
        gText_SpaceMin.as_ptr().cast_mut(),
    );
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        (*game).gfx.minutes as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        1,
    );
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), gText_StrVar1.as_ptr().cast_mut());
    x -= GetStringWidth(FONT_SHORT, gStringVar4.as_mut_ptr(), -1) as u8;
    AddTextPrinterParameterized3(
        (*game).gfx.resultsWindowId,
        FONT_SHORT,
        x,
        y,
        sTextColorTable[0].as_ptr().cast_mut(),
        0,
        gStringVar4.as_mut_ptr(),
    );
    y += 14;
    AddTextPrinterParameterized3(
        (*game).gfx.resultsWindowId,
        FONT_SHORT,
        0,
        y,
        sTextColorTable[0].as_ptr().cast_mut(),
        0,
        gText_PressingSpeed.as_ptr().cast_mut(),
    );
    x = 176 - GetStringWidth(FONT_SHORT, gText_TimesPerSec.as_ptr().cast_mut(), -1) as u8;
    AddTextPrinterParameterized3(
        (*game).gfx.resultsWindowId,
        FONT_SHORT,
        x,
        y,
        sTextColorTable[0].as_ptr().cast_mut(),
        0,
        gText_TimesPerSec.as_ptr().cast_mut(),
    );
    i = 0;
    while i < 8 {
        if shr_i32((*game).pressingSpeed as u8 as i32, 7 - i as u32) & 1 != 0 {
            pressingSpeedFrac += *sPressingSpeedConversionTable.as_ptr().cast_mut().at(i);
        }
        i += 1;
    }
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        ((*game).pressingSpeed >> 8) as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        (pressingSpeedFrac / 0xf4240) as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), gText_XDotY3.as_ptr().cast_mut());
    x -= GetStringWidth(FONT_SHORT, gStringVar4.as_mut_ptr(), -1) as u8;
    if (*game).newRecord() != 0 {
        AddTextPrinterParameterized3(
            (*game).gfx.resultsWindowId,
            FONT_SHORT,
            x,
            y,
            sTextColorTable[5].as_ptr().cast_mut(),
            0,
            gStringVar4.as_mut_ptr(),
        );
    } else {
        AddTextPrinterParameterized3(
            (*game).gfx.resultsWindowId,
            FONT_SHORT,
            x,
            y,
            sTextColorTable[0].as_ptr().cast_mut(),
            0,
            gStringVar4.as_mut_ptr(),
        );
    }
    y += 14;
    AddTextPrinterParameterized3(
        (*game).gfx.resultsWindowId,
        FONT_SHORT,
        0,
        y,
        sTextColorTable[0].as_ptr().cast_mut(),
        0,
        gText_Silkiness.as_ptr().cast_mut(),
    );
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        (*results).silkiness as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_Var1Percent.as_ptr().cast_mut(),
    );
    x = 176 - GetStringWidth(FONT_SHORT, gStringVar4.as_mut_ptr(), -1) as u8;
    AddTextPrinterParameterized3(
        (*game).gfx.resultsWindowId,
        FONT_SHORT,
        x,
        y,
        sTextColorTable[0].as_ptr().cast_mut(),
        0,
        gStringVar4.as_mut_ptr(),
    );
}
pub(crate) unsafe extern "C" fn OpenResultsWindow(
    game: *mut BerryCrushGame,
    gfx: *mut BerryCrushGame_Gfx,
) -> u32 {
    let mut playerCountIdx: u8 = 0;
    let mut template: WindowTemplate = zeroed();
    match (*gfx).resultsState {
        0 => {
            playerCountIdx = (*game).playerCount - 2;
            HideTimer(gfx);
            memcpy(
                &raw mut template as *mut u8,
                (&raw const sWindowTemplates_Results
                    [(*game).gameState as i32 - STATE_RESULTS_PRESSES as i32])
                    .cast_mut() as *mut u8,
                8,
            );
            if (*game).gameState == STATE_RESULTS_CRUSHING {
                template.height = sResultsWindowHeights[1][playerCountIdx];
            } else {
                template.height = sResultsWindowHeights[0][playerCountIdx];
            }
            (*gfx).resultsWindowId = AddWindow(&raw mut template) as u8;
        }
        1 => {
            PutWindowTilemap((*gfx).resultsWindowId);
            FillWindowPixelBuffer((*gfx).resultsWindowId, 0);
        }
        2 => {
            LoadUserWindowBorderGfx_((*gfx).resultsWindowId, 541, 208);
            DrawStdFrameWithCustomTileAndPalette((*gfx).resultsWindowId, FALSE, 541, 13);
        }
        3 => {
            playerCountIdx = (*game).playerCount - 2;
            match (*game).gameState {
                STATE_RESULTS_PRESSES => {
                    PrintTextCentered(
                        (*gfx).resultsWindowId,
                        20,
                        COLORID_BLUE,
                        gText_PressesRankings.as_ptr().cast_mut(),
                    );
                    PrintResultsText(
                        game,
                        RESULTS_PAGE_PRESSES,
                        0xA0,
                        8 * sResultsWindowHeights[0][playerCountIdx],
                    );
                    (*gfx).resultsState = 5;
                    return FALSE as u32;
                }
                STATE_RESULTS_RANDOM => {
                    PrintTextCentered(
                        (*gfx).resultsWindowId,
                        20,
                        COLORID_GREEN,
                        sResultsTexts
                            [(*game).results.playerIdsRanked[0][7] as i32 + NUM_RESULTS_PAGES],
                    );
                    PrintResultsText(
                        game,
                        RESULTS_PAGE_RANDOM,
                        0xA0,
                        8 * sResultsWindowHeights[0][playerCountIdx],
                    );
                    (*gfx).resultsState = 5;
                    return FALSE as u32;
                }
                STATE_RESULTS_CRUSHING => {
                    PrintTextCentered(
                        (*gfx).resultsWindowId,
                        22,
                        COLORID_BLUE,
                        gText_CrushingResults.as_ptr().cast_mut(),
                    );
                    PrintResultsText(
                        game,
                        RESULTS_PAGE_CRUSHING,
                        0xB0,
                        8 * sResultsWindowHeights[1][playerCountIdx],
                    );
                }
                _ => {}
            }
        }
        4 => {
            PrintCrushingResults(game);
        }
        5 => {
            CopyWindowToVram((*gfx).resultsWindowId, COPYWIN_FULL);
            (*gfx).resultsState = 0;
            return TRUE as u32;
        }
        _ => {}
    }
    (*gfx).resultsState += 1;
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn CloseResultsWindow(game: *mut BerryCrushGame) {
    ClearStdWindowAndFrameToTransparent((*game).gfx.resultsWindowId, TRUE);
    RemoveWindow((*game).gfx.resultsWindowId);
    DrawPlayerNameWindows(game);
}
pub(crate) unsafe extern "C" fn Task_ShowRankings(taskId: u8) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut xPos: u8 = 0;
    let mut yPos: u8 = 0;
    let mut score: u32 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            *data.at(1) = AddWindow((&raw const *sWindowTemplate_Rankings).cast_mut()) as i16;
            PutWindowTilemap(*data.at(1) as u8);
            FillWindowPixelBuffer(*data.at(1) as u8, 0);
            LoadUserWindowBorderGfx_(*data.at(1) as u8, 541, 208);
            DrawStdFrameWithCustomTileAndPalette(*data.at(1) as u8, FALSE, 541, 13);
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            xPos = 96
                - (GetStringWidth(FONT_NORMAL, gText_BerryCrush2.as_ptr().cast_mut(), -1) / 2)
                    as u8;
            AddTextPrinterParameterized3(
                *data.at(1) as u8,
                FONT_NORMAL,
                xPos,
                1,
                sTextColorTable[3].as_ptr().cast_mut(),
                0,
                gText_BerryCrush2.as_ptr().cast_mut(),
            );
            xPos = 96
                - (GetStringWidth(
                    FONT_NORMAL,
                    gText_PressingSpeedRankings.as_ptr().cast_mut(),
                    -1,
                ) / 2) as u8;
            AddTextPrinterParameterized3(
                *data.at(1) as u8,
                FONT_NORMAL,
                xPos,
                17,
                sTextColorTable[3].as_ptr().cast_mut(),
                0,
                gText_PressingSpeedRankings.as_ptr().cast_mut(),
            );
            yPos = 41;
            i = 0;
            while i < 4 {
                ConvertIntToDecimalStringN(
                    gStringVar1.as_mut_ptr(),
                    i as i32 + 2,
                    STR_CONV_MODE_LEFT_ALIGN,
                    1,
                );
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_Var1Players.as_ptr().cast_mut(),
                );
                AddTextPrinterParameterized3(
                    *data.at(1) as u8,
                    FONT_NORMAL,
                    0,
                    yPos,
                    sTextColorTable[0].as_ptr().cast_mut(),
                    0,
                    gStringVar4.as_mut_ptr(),
                );
                xPos = 192
                    - GetStringWidth(FONT_NORMAL, gText_TimesPerSec.as_ptr().cast_mut(), -1) as u8;
                AddTextPrinterParameterized3(
                    *data.at(1) as u8,
                    FONT_NORMAL,
                    xPos,
                    yPos,
                    sTextColorTable[0].as_ptr().cast_mut(),
                    0,
                    gText_TimesPerSec.as_ptr().cast_mut(),
                );
                j = 0;
                while j < 8 {
                    if shr_i32(*data.at(2 + i as i32) as i32 & 0xFF, 7 - j as u32) & 1 != 0 {
                        score += sPressingSpeedConversionTable[j];
                    }
                    j += 1;
                }
                ConvertIntToDecimalStringN(
                    gStringVar1.as_mut_ptr(),
                    (*data.at(2 + i as i32) as u16 >> 8) as i32,
                    STR_CONV_MODE_RIGHT_ALIGN,
                    3,
                );
                ConvertIntToDecimalStringN(
                    gStringVar2.as_mut_ptr(),
                    (score / 0xf4240) as i32,
                    STR_CONV_MODE_LEADING_ZEROS,
                    2,
                );
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_XDotY3.as_ptr().cast_mut(),
                );
                xPos -= GetStringWidth(FONT_NORMAL, gStringVar4.as_mut_ptr(), -1) as u8;
                AddTextPrinterParameterized3(
                    *data.at(1) as u8,
                    FONT_NORMAL,
                    xPos,
                    yPos,
                    sTextColorTable[0].as_ptr().cast_mut(),
                    0,
                    gStringVar4.as_mut_ptr(),
                );
                yPos += 16;
                score = 0;
                i += 1;
            }
            CopyWindowToVram(*data.at(1) as u8, COPYWIN_FULL);
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if gMain.newKeys as i32 & 3 != 0 {
                break 'l1;
            } else {
                return;
            }
        }
        if fall || sw1 == 3 {
            fall = true;
            ClearStdWindowAndFrameToTransparent(*data.at(1) as u8, TRUE);
            ClearWindowTilemap(*data.at(1) as u8);
            RemoveWindow(*data.at(1) as u8);
            DestroyTask(taskId);
            ScriptContext_Enable();
            UnlockPlayerFieldControls();
            *data = 0;
            return;
        }
    }
    *data += 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowBerryCrushRankings() {
    let mut taskId: u8 = 0;
    LockPlayerFieldControls();
    taskId = CreateTask(Some(Task_ShowRankings), 0);
    gTasks[taskId].data[2] = (*gSaveBlock2Ptr).berryCrush.pressingSpeeds[0] as i16;
    gTasks[taskId].data[3] = (*gSaveBlock2Ptr).berryCrush.pressingSpeeds[1] as i16;
    gTasks[taskId].data[4] = (*gSaveBlock2Ptr).berryCrush.pressingSpeeds[2] as i16;
    gTasks[taskId].data[5] = (*gSaveBlock2Ptr).berryCrush.pressingSpeeds[3] as i16;
}
pub(crate) unsafe extern "C" fn PrintTimer(gfx: *mut BerryCrushGame_Gfx, timer: u16) {
    FramesToMinSec(gfx, timer);
    DigitObjUtil_PrintNumOn(0, (*gfx).minutes as i32);
    DigitObjUtil_PrintNumOn(1, (*gfx).secondsInt as i32);
    DigitObjUtil_PrintNumOn(2, (*gfx).secondsFrac as i32);
}
pub(crate) unsafe extern "C" fn HideTimer(gfx: *mut BerryCrushGame_Gfx) {
    (*(*gfx).timerSprites[0]).set_invisible(TRUE as u16);
    (*(*gfx).timerSprites[1]).set_invisible(1);
    DigitObjUtil_HideOrShow(2, TRUE as u32);
    DigitObjUtil_HideOrShow(1, 1);
    DigitObjUtil_HideOrShow(0, TRUE as u32);
}
pub(crate) unsafe extern "C" fn CreatePlayerNameWindows(game: *mut BerryCrushGame) {
    let mut i: u8 = 0;
    i = 0;
    while i < (*game).playerCount {
        (*game).gfx.playerCoords[i] = (&raw const sPlayerCoords
            [sPlayerIdToPosId[(*game).playerCount as i32 - 2][i]])
            .cast_mut();
        (*game).gfx.nameWindowIds[i] = AddWindow(
            (&raw const sWindowTemplates_PlayerNames[(*(*game).gfx.playerCoords[i]).playerId])
                .cast_mut(),
        ) as u8;
        PutWindowTilemap((*game).gfx.nameWindowIds[i]);
        FillWindowPixelBuffer((*game).gfx.nameWindowIds[i], 0);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DrawPlayerNameWindows(game: *mut BerryCrushGame) {
    let mut i: u8 = 0;
    i = 0;
    while i < (*game).playerCount {
        PutWindowTilemap((*game).gfx.nameWindowIds[i]);
        if i == (*game).localId {
            AddTextPrinterParameterized4(
                (*game).gfx.nameWindowIds[i],
                FONT_SHORT,
                36 - (GetStringWidth(FONT_SHORT, (*game).players[i].name.as_mut_ptr(), 0) / 2)
                    as u8,
                1,
                0,
                0,
                sTextColorTable[1].as_ptr().cast_mut(),
                0,
                (*game).players[i].name.as_mut_ptr(),
            );
        } else {
            AddTextPrinterParameterized4(
                (*game).gfx.nameWindowIds[i],
                FONT_SHORT,
                36 - (GetStringWidth(FONT_SHORT, (*game).players[i].name.as_mut_ptr(), 0) / 2)
                    as u8,
                1,
                0,
                0,
                sTextColorTable[2].as_ptr().cast_mut(),
                0,
                (*game).players[i].name.as_mut_ptr(),
            );
        }
        CopyWindowToVram((*game).gfx.nameWindowIds[i], COPYWIN_FULL);
        i += 1;
    }
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn CopyPlayerNameWindowGfxToBg(game: *mut BerryCrushGame) {
    let mut i: u8 = 0;
    let mut windowGfx: *mut u8 = null_mut();
    LZ77UnCompWram(
        gBerryCrush_TextWindows_Tilemap.as_ptr().cast_mut(),
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
    );
    windowGfx = gDecompressionBuffer.as_mut_ptr();
    while i < (*game).playerCount {
        CopyToBgTilemapBufferRect(
            3,
            windowGfx.at((*(*game).gfx.playerCoords[i]).playerId as i32 * 40) as *mut c_void,
            (*(*game).gfx.playerCoords[i]).windowGfxX,
            (*(*game).gfx.playerCoords[i]).windowGfxY,
            10,
            2,
        );
        i += 1;
    }
    CopyBgTilemapBufferToVram(3);
}
pub(crate) unsafe extern "C" fn CreateGameSprites(game: *mut BerryCrushGame) {
    let mut i: u8 = 0;
    let mut spriteId: u8 = 0;
    (*game).depth = CRUSHER_START_Y;
    (*game).vibration = 0;
    gSpriteCoordOffsetX = 0;
    gSpriteCoordOffsetY = CRUSHER_START_Y;
    i = 0;
    while i < 4 {
        LoadCompressedSpriteSheet((&raw const sSpriteSheets[i]).cast_mut());
        i += 1;
    }
    LoadSpritePalettes(sSpritePals.as_ptr().cast_mut());
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_CrusherBase).cast_mut(),
        120,
        88,
        5,
    );
    (*game).gfx.coreSprite = &raw mut gSprites[spriteId];
    (*(*game).gfx.coreSprite).oam.set_priority(3);
    (*(*game).gfx.coreSprite).set_coordOffsetEnabled(TRUE as u16);
    (*(*game).gfx.coreSprite).set_animPaused(TRUE);
    i = 0;
    while i < (*game).playerCount {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Impact).cast_mut(),
            (*(*game).gfx.playerCoords[i]).impactXOffset + 120,
            (*(*game).gfx.playerCoords[i]).impactYOffset + 32,
            0,
        );
        (*game).gfx.impactSprites[i] = &raw mut gSprites[spriteId];
        (*(*game).gfx.impactSprites[i]).oam.set_priority(1);
        (*(*game).gfx.impactSprites[i]).set_invisible(TRUE as u16);
        (*(*game).gfx.impactSprites[i]).set_coordOffsetEnabled(TRUE as u16);
        (*(*game).gfx.impactSprites[i]).set_animPaused(TRUE);
        i += 1;
    }
    i = 0;
    while i < 11 {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Sparkle).cast_mut(),
            sSparkleCoords[i][0] as i16 + 120,
            sSparkleCoords[i][1] as i16 + 136,
            6,
        );
        (*game).gfx.sparkleSprites[i] = &raw mut gSprites[spriteId];
        (*(*game).gfx.sparkleSprites[i]).oam.set_priority(3);
        (*(*game).gfx.sparkleSprites[i]).set_invisible(TRUE as u16);
        (*(*game).gfx.sparkleSprites[i]).set_animPaused(TRUE);
        (*(*game).gfx.sparkleSprites[i]).data[0] = i as i16;
        i += 1;
    }
    i = 0;
    while i < 2 {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Timer).cast_mut(),
            24 * i as i16 + 176,
            8,
            0,
        );
        (*game).gfx.timerSprites[i] = &raw mut gSprites[spriteId];
        (*(*game).gfx.timerSprites[i]).oam.set_priority(0);
        (*(*game).gfx.timerSprites[i]).set_invisible(FALSE as u16);
        (*(*game).gfx.timerSprites[i]).set_animPaused(FALSE);
        i += 1;
    }
    DigitObjUtil_CreatePrinter(0, 0, (&raw const sDigitObjTemplates[0]).cast_mut());
    DigitObjUtil_CreatePrinter(1, 0, (&raw const sDigitObjTemplates[1]).cast_mut());
    DigitObjUtil_CreatePrinter(2, 0, (&raw const sDigitObjTemplates[2]).cast_mut());
    if (*game).gameState == STATE_INIT {
        HideTimer(&raw mut (*game).gfx);
    }
}
pub(crate) unsafe extern "C" fn DestroyGameSprites(game: *mut BerryCrushGame) {
    let mut i: u8 = 0;
    FreeSpriteTilesByTag(TAG_TIMER_DIGITS);
    FreeSpriteTilesByTag(GFXTAG_SPARKLE);
    FreeSpriteTilesByTag(GFXTAG_IMPACT);
    FreeSpriteTilesByTag(TAG_CRUSHER_BASE);
    FreeSpritePaletteByTag(TAG_TIMER_DIGITS);
    FreeSpritePaletteByTag(PALTAG_EFFECT);
    FreeSpritePaletteByTag(TAG_CRUSHER_BASE);
    i = 0;
    while i < 2 {
        DestroySprite((*game).gfx.timerSprites[i]);
        i += 1;
    }
    DigitObjUtil_DeletePrinter(2);
    DigitObjUtil_DeletePrinter(1);
    DigitObjUtil_DeletePrinter(0);
    i = 0;
    while i < 11 {
        DestroySprite((*game).gfx.sparkleSprites[i]);
        i += 1;
    }
    i = 0;
    while i < (*game).playerCount {
        DestroySprite((*game).gfx.impactSprites[i]);
        i += 1;
    }
    if (*(*game).gfx.coreSprite).inUse() != 0 {
        DestroySprite((*game).gfx.coreSprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Impact(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).set_animPaused(TRUE);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_End(sprite: *mut Sprite) {
    let mut i: u8 = 0;
    i = 0;
    while i < 8 {
        (*sprite).data[i] = 0;
        i += 1;
    }
    (*sprite).x2 = 0;
    (*sprite).y2 = 0;
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).set_animPaused(TRUE);
    (*sprite).callback = Some(SpriteCallbackDummy);
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle(sprite: *mut Sprite) {
    let mut data: *mut i16 = (*sprite).data.as_mut_ptr();
    *data.at(1) += *data.at(2);
    (*sprite).y2 += *data.at(1) >> 8;
    if *data.at(7) as i32 & F_MOVE_HORIZ != 0 {
        (*sprite).data[0] += *data.at(3);
        *data.at(4) += *data.at(5);
        (*sprite).x2 = Sin(*data.at(4) >> 7, *data.at(6));
        if *data.at(7) as i32 & F_MOVE_HORIZ != 0 && *data.at(4) >> 7 > 126 {
            (*sprite).x2 = 0;
            *data.at(7) &= MASK_TARGET_Y;
        }
    }
    (*sprite).x = *data >> 7;
    if (*sprite).y as i32 + (*sprite).y2 as i32 > *data.at(7) as i32 & MASK_TARGET_Y as i32 {
        (*sprite).callback = Some(SpriteCB_Sparkle_End);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_Init(sprite: *mut Sprite) {
    let mut data: *mut i16 = (*sprite).data.as_mut_ptr();
    let mut xMult: i16 = 0;
    let mut xDiv: i16 = 0;
    let mut var: i32 = 0;
    let mut zero: u32 = 0;
    var = 640;
    *data.at(1) = var as i16;
    *data.at(2) = 32;
    *data.at(7) = 168;
    xMult = (*sprite).x2 * 128;
    xDiv = MathUtil_Div16Shift(7, 168 - (*sprite).y << 7, (var + 32 >> 1) as i16);
    (*sprite).data[0] = (*sprite).x << 7;
    *data.at(3) = MathUtil_Div16Shift(7, xMult, xDiv);
    var = MathUtil_Mul16Shift(7, xDiv, 85) as i32;
    *data.at(4) = zero as i16;
    *data.at(5) = MathUtil_Div16Shift(7, (63.5f32 as f32 * 256 as f32) as i16, var as i16);
    *data.at(6) = (*sprite).x2 / 4;
    *data.at(7) |= F_MOVE_HORIZ as i16;
    (*sprite).y2 = zero as i16;
    (*sprite).x2 = zero as i16;
    (*sprite).callback = Some(SpriteCB_Sparkle);
    (*sprite).set_animPaused(FALSE);
    (*sprite).set_invisible(FALSE as u16);
}
pub(crate) unsafe extern "C" fn RunOrScheduleCommand(mut cmdId: u16, mode: u8, args: *mut u8) {
    let mut game: *mut BerryCrushGame = GetBerryCrushGame();
    if cmdId >= 26 {
        cmdId = CMD_NONE;
    }
    match mode {
        RUN_CMD => {
            if cmdId != CMD_NONE {
                sBerryCrushCommands[cmdId].unwrap_unchecked()(game, args);
            }
            if (*game).nextCmd >= 26 {
                (*game).nextCmd = CMD_NONE as u8;
            }
            (*game).cmdCallback = sBerryCrushCommands[(*game).nextCmd];
        }
        SCHEDULE_CMD => {
            (*game).cmdCallback = sBerryCrushCommands[cmdId];
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Cmd_BeginNormalPaletteFade(
    game: *mut BerryCrushGame,
    mut args: *mut u8,
) -> u32 {
    let mut color: u16 = 0;
    let mut selectedPals: CArray<u32, 2> = zeroed();
    selectedPals[0] = *args as u32;
    selectedPals[1] = *args.at(1) as u32;
    selectedPals[1] <<= 8;
    selectedPals[0] |= selectedPals[1];
    selectedPals[1] = *args.at(2) as u32;
    selectedPals[1] <<= 16;
    selectedPals[0] |= selectedPals[1];
    selectedPals[1] = *args.at(3) as u32;
    selectedPals[1] <<= 24;
    selectedPals[0] |= selectedPals[1];
    *args = *args.at(9);
    color = *args.at(8) as u16;
    color <<= 8;
    color |= *args.at(7) as u16;
    gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
    BeginNormalPaletteFade(
        selectedPals[0],
        *args.at(4) as i8,
        *args.at(5),
        *args.at(6),
        color,
    );
    UpdatePaletteFade();
    (*game).nextCmd = CMD_WAIT_FADE;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_WaitPaletteFade(
    game: *mut BerryCrushGame,
    args: *mut u8,
) -> u32 {
    match (*game).cmdState {
        0 => {
            if UpdatePaletteFade() != 0 {
                return 0;
            }
            if *args != 0 {
                (*game).cmdState += 1;
            } else {
                (*game).cmdState = 3;
            }
            return 0;
        }
        1 => {
            Rfu_SetLinkStandbyCallback();
            (*game).cmdState += 1;
            return 0;
        }
        2 => {
            if IsLinkTaskFinished() != 0 {
                (*game).cmdState += 1;
                return 0;
            }
            return 0;
        }
        3 => {
            RunOrScheduleCommand((*game).afterPalFadeCmd as u16, SCHEDULE_CMD, null_mut());
            (*game).cmdState = 0;
            return 0;
        }
        _ => {
            (*game).cmdState += 1;
            return 0;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Cmd_PrintMessage(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    let mut keys: u16 = *args.at(3) as u16;
    keys <<= 8;
    keys |= *args.at(2) as u16;
    'l1: {
        match (*game).cmdState {
            0 => {
                DrawDialogueFrame(0, 0);
                if *args.at(1) as i32 & F_MSG_EXPAND != 0 {
                    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), sMessages[*args]);
                    AddTextPrinterParameterized2(
                        0,
                        FONT_NORMAL,
                        gStringVar4.as_mut_ptr(),
                        (*game).textSpeed,
                        None,
                        TEXT_COLOR_DARK_GRAY,
                        TEXT_COLOR_WHITE,
                        TEXT_COLOR_LIGHT_GRAY,
                    );
                } else {
                    AddTextPrinterParameterized2(
                        0,
                        FONT_NORMAL,
                        sMessages[*args],
                        (*game).textSpeed,
                        None,
                        TEXT_COLOR_DARK_GRAY,
                        TEXT_COLOR_WHITE,
                        TEXT_COLOR_LIGHT_GRAY,
                    );
                }
                CopyWindowToVram(0, COPYWIN_FULL);
            }
            1 => {
                if IsTextPrinterActive(0) == 0 {
                    if keys == 0 {
                        (*game).cmdState += 1;
                    }
                    break 'l1;
                }
                return 0;
            }
            2 => {
                if gMain.newKeys as i32 & keys as i32 == 0 {
                    return 0;
                }
            }
            3 => {
                if *args.at(1) as i32 & 1 != 0 {
                    ClearDialogWindowAndFrame(0, TRUE);
                }
                RunOrScheduleCommand((*game).nextCmd as u16, SCHEDULE_CMD, null_mut());
                (*game).cmdState = *args.at(4);
                return 0;
            }
            _ => {}
        }
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_ShowGameDisplay(
    game: *mut BerryCrushGame,
    args: *mut u8,
) -> u32 {
    if ShowGameDisplay() != 0 {
        RunOrScheduleCommand(
            (*game).nextCmd as u16,
            RUN_CMD,
            (*game).commandArgs.as_mut_ptr(),
        );
    }
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_HideGameDisplay(
    game: *mut BerryCrushGame,
    args: *mut u8,
) -> u32 {
    if HideGameDisplay() != 0 {
        RunOrScheduleCommand(
            (*game).nextCmd as u16,
            RUN_CMD,
            (*game).commandArgs.as_mut_ptr(),
        );
    }
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_SignalReadyToBegin(
    game: *mut BerryCrushGame,
    args: *mut u8,
) -> u32 {
    match (*game).cmdState {
        0 => {
            Rfu_SetLinkStandbyCallback();
        }
        1 => {
            if IsLinkTaskFinished() != 0 {
                PlayNewMapMusic(MUS_RG_GAME_CORNER);
                RunOrScheduleCommand(CMD_ASK_PICK_BERRY, SCHEDULE_CMD, null_mut());
                (*game).gameState = STATE_PICK_BERRY;
                (*game).cmdState = 0;
            }
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_AskPickBerry(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    match (*game).cmdState {
        0 => {
            ResetGame(game);
            SetPrintMessageArgs(args, MSG_PICK_BERRY, 1, 0, 1);
            (*game).nextCmd = CMD_ASK_PICK_BERRY as u8;
            RunOrScheduleCommand(CMD_PRINT_MSG, SCHEDULE_CMD, null_mut());
        }
        1 => {
            (*game).nextCmd = CMD_PICK_BERRY;
            RunOrScheduleCommand(CMD_HIDE_GAME, SCHEDULE_CMD, null_mut());
            (*game).cmdState = 2;
        }
        _ => {
            (*game).cmdState += 1;
        }
    }
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_GoToBerryPouch(
    game: *mut BerryCrushGame,
    args: *mut u8,
) -> u32 {
    (*game).cmdCallback = None;
    SetMainCallback2(Some(ChooseBerry));
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_WaitForOthersToPickBerries(
    game: *mut BerryCrushGame,
    args: *mut u8,
) -> u32 {
    let mut i: u8 = 0;
    match (*game).cmdState {
        0 => {
            SetPrintMessageArgs(args, MSG_WAIT_PICK, 0, 0, 1);
            (*game).nextCmd = CMD_WAIT_BERRIES;
            RunOrScheduleCommand(CMD_PRINT_MSG, SCHEDULE_CMD, null_mut());
            return 0;
        }
        1 => {
            Rfu_SetLinkStandbyCallback();
        }
        2 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            memset((*game).sendCmd.as_mut_ptr() as *mut u8, 0, 12);
            (*game).sendCmd[0] = (*game).players[(*game).localId].berryId;
            SendBlock(0, (*game).sendCmd.as_mut_ptr() as *mut c_void, 2);
        }
        3 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            (*game).cmdTimer = 0;
        }
        4 => {
            if GetBlockReceivedStatus() != sReceivedPlayerBitmasks[(*game).playerCount as i32 - 2] {
                return 0;
            }
            i = 0;
            while i < (*game).playerCount {
                (*game).players[i].berryId = gBlockRecvBuffer[i][0];
                if (*game).players[i].berryId > 176 {
                    (*game).players[i].berryId = 0;
                }
                (*game).targetAPresses +=
                    gBerryCrush_BerryData[(*game).players[i].berryId].difficulty as i16;
                (*game).powder += gBerryCrush_BerryData[(*game).players[i].berryId].powder as i32;
                i += 1;
            }
            (*game).cmdTimer = 0;
            ResetBlockReceivedFlags();
            (*game).targetDepth = MathUtil_Div32(((*game).targetAPresses as i32) << 8, 8192);
        }
        5 => {
            ClearDialogWindowAndFrame(0, TRUE);
            RunOrScheduleCommand(CMD_DROP_BERRIES, SCHEDULE_CMD, null_mut());
            (*game).gameState = STATE_DROP_BERRIES;
            (*game).cmdState = 0;
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_DropBerriesIntoCrusher(
    game: *mut BerryCrushGame,
    args: *mut u8,
) -> u32 {
    match (*game).cmdState {
        0 => {
            CreateBerrySprites(game, &raw mut (*game).gfx);
            Rfu_SetLinkStandbyCallback();
        }
        1 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            (*game).gfx.counter = 0;
            (*game).gfx.vibrationIdx = 0;
            (*game).gfx.numVibrations = 0;
            (*game).gfx.vibrating = FALSE;
        }
        2 => {
            (*(*game).gfx.berrySprites[(*game).gfx.counter]).callback =
                Some(SpriteCB_DropBerryIntoCrusher);
            (*(*game).gfx.berrySprites[(*game).gfx.counter]).set_affineAnimPaused(FALSE);
            PlaySE(SE_BALL_THROW);
        }
        3 => {
            if (*(*game).gfx.berrySprites[(*game).gfx.counter]).callback
                == Some(SpriteCB_DropBerryIntoCrusher as unsafe extern "C" fn(*mut Sprite))
            {
                return 0;
            }
            (*game).gfx.berrySprites[(*game).gfx.counter] = null_mut();
            (*game).gfx.counter += 1;
            Rfu_SetLinkStandbyCallback();
        }
        4 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            if (*game).gfx.counter < (*game).playerCount {
                (*game).cmdState = 2;
                return 0;
            }
            (*game).gfx.counter = 0;
        }
        5 => {
            BerryCrushFreeBerrySpriteGfx(game, &raw mut (*game).gfx);
            Rfu_SetLinkStandbyCallback();
        }
        6 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            PlaySE(SE_FALL);
            RunOrScheduleCommand(CMD_DROP_LID, SCHEDULE_CMD, null_mut());
            (*game).gameState = STATE_DROP_LID;
            (*game).cmdState = 0;
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_DropLid(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    'l1: {
        match (*game).cmdState {
            0 => {
                (*game).depth += 4;
                if (*game).depth < 0 {
                    return 0;
                }
                (*game).depth = 0;
                (*game).gfx.vibrationIdx = 4;
                (*game).gfx.counter = 0;
                (*game).gfx.numVibrations =
                    sIntroOutroVibrationData[(*game).gfx.vibrationIdx][0] as u8;
                PlaySE(SE_M_STRENGTH);
            }
            1 => {
                (*game).vibration =
                    sIntroOutroVibrationData[(*game).gfx.vibrationIdx][(*game).gfx.counter] as i16;
                SetGpuReg(
                    REG_OFFSET_BG0VOFS,
                    ((*game).vibration as u16).wrapping_neg(),
                );
                SetGpuReg(
                    REG_OFFSET_BG2VOFS,
                    ((*game).vibration as u16).wrapping_neg(),
                );
                SetGpuReg(
                    REG_OFFSET_BG3VOFS,
                    ((*game).vibration as u16).wrapping_neg(),
                );
                (*game).gfx.counter += 1;
                if (*game).gfx.counter < (*game).gfx.numVibrations {
                    return 0;
                }
                if (*game).gfx.vibrationIdx == 0 {
                    break 'l1;
                }
                (*game).gfx.vibrationIdx -= 1;
                (*game).gfx.numVibrations =
                    sIntroOutroVibrationData[(*game).gfx.vibrationIdx][0] as u8;
                (*game).gfx.counter = 0;
                return 0;
            }
            2 => {
                (*game).vibration = 0;
                SetGpuReg(REG_OFFSET_BG0VOFS, 0);
                SetGpuReg(REG_OFFSET_BG2VOFS, 0);
                SetGpuReg(REG_OFFSET_BG3VOFS, 0);
                Rfu_SetLinkStandbyCallback();
            }
            3 => {
                if IsLinkTaskFinished() == 0 {
                    return 0;
                }
                RunOrScheduleCommand(CMD_COUNTDOWN, SCHEDULE_CMD, null_mut());
                (*game).gameState = STATE_COUNTDOWN;
                (*game).cmdState = 0;
                return 0;
            }
            _ => {}
        }
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_Countdown(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    'l1: {
        let sw1: u8 = (*game).cmdState;
        let mut fall = false;
        if sw1 == 1 {
            fall = true;
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            StartMinigameCountdown(TAG_COUNTDOWN, TAG_COUNTDOWN, 120, 80, 0);
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if IsMinigameCountdownRunning() != 0 {
                return 0;
            }
        }
        if fall || sw1 == 0 {
            fall = true;
            Rfu_SetLinkStandbyCallback();
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            (*game).gfx.counter = 0;
            (*game).gfx.vibrationIdx = 0;
            (*game).gfx.numVibrations = 0;
            (*game).gfx.vibrating = FALSE;
            (*game).cmdTimer = 0;
            if (*game).localId == 0 {
                RunOrScheduleCommand(CMD_PLAY_GAME_LEADER, SCHEDULE_CMD, null_mut());
            } else {
                RunOrScheduleCommand(CMD_PLAY_GAME_MEMBER, SCHEDULE_CMD, null_mut());
            }
            (*game).gameState = STATE_PLAYING;
            (*game).cmdState = 0;
            return 0;
        }
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn HandlePartnerInput(game: *mut BerryCrushGame) {
    let mut numPlayersPressed: u8 = 0;
    let mut i: u8 = 0;
    let mut timeDiff: u16 = 0;
    let mut temp: i32 = 0;
    let mut linkState: *mut BerryCrushGame_LinkState = null_mut();
    i = 0;
    while i < (*game).playerCount {
        'l1: {
            linkState = gRecvCmds[i].as_mut_ptr() as *mut BerryCrushGame_LinkState;
            if (*linkState).rfuCmd as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
                break 'l1;
            }
            if (*linkState).sendFlag != SEND_GAME_STATE {
                break 'l1;
            }
            if (*linkState).pushedAButton() != 0 {
                (*game).localState.set_playerPressedAFlags(
                    (*game).localState.playerPressedAFlags() | sBitTable[i],
                );
                (*game).players[i].inputState = INPUT_STATE_HIT;
                (*game).players[i].numAPresses += 1;
                numPlayersPressed += 1;
                timeDiff = (*game).timer - (*game).players[i].inputTime;
                if timeDiff as i32 >= (*game).players[i].timeSincePrevInput as i32 - 1
                    && timeDiff as i32 <= (*game).players[i].timeSincePrevInput as i32 + 1
                {
                    (*game).players[i].neatInputStreak += 1;
                    (*game).players[i].timeSincePrevInput = timeDiff;
                    if (*game).players[i].neatInputStreak > (*game).players[i].maxNeatInputStreak {
                        (*game).players[i].maxNeatInputStreak = (*game).players[i].neatInputStreak;
                    }
                } else {
                    (*game).players[i].neatInputStreak = 0;
                    (*game).players[i].timeSincePrevInput = timeDiff;
                }
                (*game).players[i].inputTime = (*game).timer;
                (*game).players[i].inputFlags += 1;
                if (*game).players[i].inputFlags > F_INPUT_HIT_B {
                    (*game).players[i].inputFlags = 0;
                }
            } else {
                (*game).players[i].inputState = INPUT_STATE_NONE;
            }
        }
        i += 1;
    }
    if numPlayersPressed > 1 {
        i = 0;
        while i < (*game).playerCount {
            'l3: {
                if (*game).players[i].inputState == INPUT_STATE_NONE {
                    break 'l3;
                }
                (*game).players[i].inputState |= INPUT_STATE_HIT_SYNC;
                (*game).players[i].numSyncedAPresses += 1;
            }
            i += 1;
        }
    }
    if numPlayersPressed == 0 {
        return;
    }
    (*game).bigSparkleCounter += numPlayersPressed as i16;
    numPlayersPressed += sSyncPressBonus[numPlayersPressed as i32 - 1];
    (*game).sparkleCounter += numPlayersPressed as i16;
    (*game).totalAPresses += numPlayersPressed as i16;
    if (*game).targetAPresses as i32 - (*game).totalAPresses as i32 > 0 {
        temp = (*game).totalAPresses as i32;
        temp = temp << 8;
        temp = MathUtil_Div32(temp, (*game).targetDepth);
        temp = temp >> 8;
        (*game).newDepth = temp as u8;
        return;
    }
    (*game).newDepth = 32;
    (*game).localState.set_endGame(TRUE);
}
pub(crate) unsafe extern "C" fn UpdateLeaderGameState(game: *mut BerryCrushGame) {
    let mut numPlayersPressed: u8 = 0;
    let mut flags: u16 = 0;
    let mut temp: u16 = 0;
    let mut i: u8 = 0;
    i = 0;
    while i < (*game).playerCount {
        if (*game).players[i].inputState != INPUT_STATE_NONE {
            numPlayersPressed += 1;
            flags = (*game).players[i].inputFlags as u16 + F_INPUT_HIT_A;
            if (*game).players[i].inputState as i32 & INPUT_STATE_HIT_SYNC as i32 != 0 {
                flags |= F_INPUT_HIT_SYNC as u16;
            }
            flags = shl_i32(flags as i32, INPUT_FLAGS_PER_PLAYER * i as u32) as u16;
            (*game).localState.inputFlags |= flags;
        }
        i += 1;
    }
    temp = (*game).newDepth as u16;
    (*game).localState.depth = temp;
    if numPlayersPressed == 0 {
        if (*game).gfx.vibrating != 0 {
            (*game).gfx.counter += 1;
        }
    } else if (*game).gfx.vibrating != 0 {
        if numPlayersPressed != (*game).gfx.vibrationIdx {
            (*game).gfx.vibrationIdx = numPlayersPressed - 1;
            (*game).gfx.numVibrations = sVibrationData[numPlayersPressed as i32 - 1][0];
        } else {
            (*game).gfx.counter += 1;
        }
    } else {
        (*game).gfx.counter = 0;
        (*game).gfx.vibrationIdx = numPlayersPressed - 1;
        (*game).gfx.numVibrations = sVibrationData[numPlayersPressed as i32 - 1][0];
        (*game).gfx.vibrating = TRUE;
    }
    if (*game).gfx.vibrating != 0 {
        if (*game).gfx.counter >= (*game).gfx.numVibrations {
            (*game).gfx.counter = 0;
            (*game).gfx.vibrationIdx = 0;
            (*game).gfx.numVibrations = 0;
            (*game).gfx.vibrating = FALSE;
            temp = 0;
        } else {
            temp = sVibrationData[(*game).gfx.vibrationIdx][(*game).gfx.counter as i32 + 1] as u16;
        }
        (*game).localState.vibration = temp as u8 as i8;
    } else {
        (*game).localState.vibration = 0;
    }
    (*game).localState.timer = (*game).leaderTimer;
}
pub(crate) unsafe extern "C" fn HandlePlayerInput(game: *mut BerryCrushGame) {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        (*game).localState.set_pushedAButton(TRUE);
    }
    if gMain.heldKeys as i32 & A_BUTTON != 0 {
        if (*game).players[(*game).localId].timePressingA < (*game).timer {
            (*game).players[(*game).localId].timePressingA += 1;
        }
    }
    if (*game).localId != 0 && (*game).localState.pushedAButton() == 0 {
        return;
    }
    (*game).localState.sendFlag = SEND_GAME_STATE;
    if (*game).timer as i32 % 30 == 0 {
        if (*game).bigSparkleCounter > sBigSparkleThresholds[(*game).playerCount as i32 - 2] as i16
        {
            (*game).numBigSparkles += 1;
            (*game).set_bigSparkle(TRUE);
        } else {
            (*game).set_bigSparkle(FALSE);
        }
        (*game).bigSparkleCounter = 0;
        (*game).numBigSparkleChecks += 1;
    }
    if (*game).timer as i32 % 15 == 0 {
        if (*game).sparkleCounter < sSparkleThresholds[(*game).playerCount as i32 - 2][0] as i16 {
            (*game).set_sparkleAmount(0);
        } else if (*game).sparkleCounter
            < sSparkleThresholds[(*game).playerCount as i32 - 2][1] as i16
        {
            (*game).set_sparkleAmount(1);
        } else if (*game).sparkleCounter
            < sSparkleThresholds[(*game).playerCount as i32 - 2][2] as i16
        {
            (*game).sparkleCounter = 2;
        } else if (*game).sparkleCounter
            < sSparkleThresholds[(*game).playerCount as i32 - 2][3] as i16
        {
            (*game).sparkleCounter = 3;
        } else {
            (*game).set_sparkleAmount(4);
        }
        (*game).sparkleCounter = 0;
    } else {
        (*game).cmdTimer += 1;
        if (*game).cmdTimer > 60 {
            if (*game).cmdTimer > 70 {
                ClearRecvCommands();
                (*game).cmdTimer = 0;
            } else if (*game).localState.playerPressedAFlags() == 0 {
                ClearRecvCommands();
                (*game).cmdTimer = 0;
            }
        }
    }
    if (*game).timer >= MAX_TIME {
        (*game).localState.set_endGame(TRUE);
    }
    (*game).localState.set_bigSparkle((*game).bigSparkle());
    (*game).localState.sparkleAmount = (*game).sparkleAmount() as u16;
    memcpy(
        (*game).sendCmd.as_mut_ptr() as *mut u8,
        &raw mut (*game).localState as *mut u8,
        12,
    );
    Rfu_SendPacket((*game).sendCmd.as_mut_ptr() as *mut c_void);
}
pub(crate) unsafe extern "C" fn RecvLinkData(game: *mut BerryCrushGame) {
    let mut i: u8 = 0;
    let mut linkState: *mut BerryCrushGame_LinkState = null_mut();
    i = 0;
    while i < (*game).playerCount {
        (*game).players[i].inputState = INPUT_STATE_NONE;
        i += 1;
    }
    if gRecvCmds[0][0] as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        (*game).set_playedSound(FALSE);
        return;
    }
    if gRecvCmds[0][1] != 2 {
        (*game).set_playedSound(FALSE);
        return;
    }
    memcpy(
        (*game).recvCmd.as_mut_ptr() as *mut u8,
        gRecvCmds[0].as_mut_ptr() as *mut u8,
        14,
    );
    linkState = &raw mut (*game).recvCmd as *mut BerryCrushGame_LinkState;
    (*game).depth = (*linkState).depth as i16;
    (*game).vibration = (*linkState).vibration as i16;
    (*game).timer = (*linkState).timer;
    UpdateInputEffects(game, &raw mut (*game).gfx);
    if (*linkState).endGame() != 0 {
        (*game).set_endGame(TRUE);
    }
}
pub(crate) unsafe extern "C" fn Cmd_PlayGame_Leader(
    game: *mut BerryCrushGame,
    args: *mut u8,
) -> u32 {
    memset(&raw mut (*game).localState as *mut u8, 0, 12);
    memset(&raw mut (*game).recvCmd as *mut u8, 0, 14);
    RecvLinkData(game);
    SetGpuReg(
        REG_OFFSET_BG0VOFS,
        ((*game).vibration as u16).wrapping_neg(),
    );
    SetGpuReg(
        REG_OFFSET_BG2VOFS,
        ((*game).vibration as u16).wrapping_neg(),
    );
    SetGpuReg(
        REG_OFFSET_BG3VOFS,
        ((*game).vibration as u16).wrapping_neg(),
    );
    if (*game).endGame() != 0 {
        if (*game).timer >= MAX_TIME {
            (*game).timer = MAX_TIME;
            RunOrScheduleCommand(CMD_TIMES_UP, SCHEDULE_CMD, null_mut());
        } else {
            RunOrScheduleCommand(CMD_FINISH_GAME, SCHEDULE_CMD, null_mut());
        }
        (*game).cmdTimer = 0;
        (*game).cmdState = 0;
        return 0;
    } else {
        (*game).leaderTimer += 1;
        HandlePartnerInput(game);
        UpdateLeaderGameState(game);
        HandlePlayerInput(game);
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Cmd_PlayGame_Member(
    game: *mut BerryCrushGame,
    args: *mut u8,
) -> u32 {
    memset(&raw mut (*game).localState as *mut u8, 0, 12);
    memset(&raw mut (*game).recvCmd as *mut u8, 0, 14);
    RecvLinkData(game);
    SetGpuReg(
        REG_OFFSET_BG0VOFS,
        ((*game).vibration as u16).wrapping_neg(),
    );
    SetGpuReg(
        REG_OFFSET_BG2VOFS,
        ((*game).vibration as u16).wrapping_neg(),
    );
    SetGpuReg(
        REG_OFFSET_BG3VOFS,
        ((*game).vibration as u16).wrapping_neg(),
    );
    if (*game).endGame() != 0 {
        if (*game).timer >= MAX_TIME {
            (*game).timer = MAX_TIME;
            RunOrScheduleCommand(CMD_TIMES_UP, SCHEDULE_CMD, null_mut());
        } else {
            RunOrScheduleCommand(CMD_FINISH_GAME, SCHEDULE_CMD, null_mut());
        }
        (*game).cmdTimer = 0;
        (*game).cmdState = 0;
        return 0;
    } else {
        HandlePlayerInput(game);
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Cmd_FinishGame(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    match (*game).cmdState {
        0 => {
            (*game).gameState = STATE_FINISHED;
            PlaySE(SE_M_STRENGTH);
            BlendPalettes(PALETTES_ALL, 8, 1023);
            (*game).gfx.counter = 2;
        }
        1 => {
            if ({
                (*game).gfx.counter -= 1;
                (*game).gfx.counter
            }) != 255
            {
                return 0;
            }
            BlendPalettes(PALETTES_ALL, 0, 1023);
            (*game).gfx.vibrationIdx = 4;
            (*game).gfx.counter = 0;
            (*game).gfx.numVibrations = sIntroOutroVibrationData[(*game).gfx.vibrationIdx][0] as u8;
        }
        2 => {
            (*game).vibration =
                sIntroOutroVibrationData[(*game).gfx.vibrationIdx][(*game).gfx.counter] as i16;
            SetGpuReg(
                REG_OFFSET_BG0VOFS,
                ((*game).vibration as u16).wrapping_neg(),
            );
            SetGpuReg(
                REG_OFFSET_BG2VOFS,
                ((*game).vibration as u16).wrapping_neg(),
            );
            SetGpuReg(
                REG_OFFSET_BG3VOFS,
                ((*game).vibration as u16).wrapping_neg(),
            );
            if ({
                (*game).gfx.counter += 1;
                (*game).gfx.counter
            }) < (*game).gfx.numVibrations
            {
                return 0;
            }
            if (*game).gfx.vibrationIdx != 0 {
                (*game).gfx.vibrationIdx -= 1;
                (*game).gfx.numVibrations =
                    sIntroOutroVibrationData[(*game).gfx.vibrationIdx][0] as u8;
                (*game).gfx.counter = 0;
                return 0;
            }
        }
        3 => {
            (*game).vibration = 0;
            SetGpuReg(REG_OFFSET_BG0VOFS, 0);
            SetGpuReg(REG_OFFSET_BG2VOFS, 0);
            SetGpuReg(REG_OFFSET_BG3VOFS, 0);
        }
        4 => {
            if AreEffectsFinished(game, &raw mut (*game).gfx) == 0 {
                return 0;
            }
            Rfu_SetLinkStandbyCallback();
            (*game).cmdTimer = 0;
        }
        5 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            RunOrScheduleCommand(CMD_CALC_RESULTS, SCHEDULE_CMD, null_mut());
            (*game).cmdTimer = 0;
            (*game).cmdState = 0;
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_HandleTimeUp(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    match (*game).cmdState {
        0 => {
            (*game).gameState = STATE_TIMES_UP;
            PlaySE(SE_FAILURE);
            BlendPalettes(PALETTES_ALL, 8, 31);
            (*game).gfx.counter = 4;
        }
        1 => {
            if ({
                (*game).gfx.counter -= 1;
                (*game).gfx.counter
            }) != 255
            {
                return 0;
            }
            BlendPalettes(PALETTES_ALL, 0, 31);
            (*game).gfx.counter = 0;
        }
        2 => {
            if AreEffectsFinished(game, &raw mut (*game).gfx) == 0 {
                return 0;
            }
            Rfu_SetLinkStandbyCallback();
            (*game).cmdTimer = 0;
            SetGpuReg(REG_OFFSET_BG0VOFS, 0);
            SetGpuReg(REG_OFFSET_BG2VOFS, 0);
            SetGpuReg(REG_OFFSET_BG3VOFS, 0);
        }
        3 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                (*game).powder,
                STR_CONV_MODE_LEFT_ALIGN,
                6,
            );
            SetPrintMessageArgs(args, MSG_TIMES_UP, F_MSG_CLEAR, 0, 0);
            (*game).nextCmd = CMD_SAVE;
            RunOrScheduleCommand(CMD_PRINT_MSG, SCHEDULE_CMD, null_mut());
            (*game).cmdTimer = 0;
            (*game).cmdState = 0;
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_TabulateResults(
    game: *mut BerryCrushGame,
    args: *mut u8,
) -> u32 {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut tempPlayerId: u8 = 0;
    let mut temp1: i32 = 0;
    let mut temp2: i32 = 0;
    let mut tempStat: u16 = 0;
    match (*game).cmdState {
        0 => {
            memset((*game).sendCmd.as_mut_ptr() as *mut u8, 0, 4);
            if (*game).players[(*game).localId].timePressingA > (*game).timer {
                (*game).players[(*game).localId].timePressingA = (*game).timer;
            }
            (*game).sendCmd[0] = (*game).players[(*game).localId].timePressingA;
            SendBlock(0, (*game).sendCmd.as_mut_ptr() as *mut c_void, 2);
        }
        1 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            (*game).cmdTimer = 0;
        }
        2 => {
            if GetBlockReceivedStatus() != sReceivedPlayerBitmasks[(*game).playerCount as i32 - 2] {
                return 0;
            }
            i = 0;
            while i < (*game).playerCount {
                (*game).players[i].timePressingA = gBlockRecvBuffer[i][0];
                i += 1;
            }
            (*game).cmdTimer = 0;
            (*game).sendCmd[0] = 0;
            ResetBlockReceivedFlags();
            if (*game).localId == 0 {
                (*game).cmdState = 3;
            } else {
                (*game).cmdState = 6;
            }
            return 0;
        }
        3 => {
            memset(&raw mut (*game).results as *mut u8, 0, 48);
            (*game).results.time = (*game).timer;
            (*game).results.targetPressesPerSec =
                div_i32((*game).targetAPresses as i32, (*game).timer as i32 / 60) as u16;
            temp1 = MathUtil_Mul32(((*game).numBigSparkles as i32) << 8, 12800);
            temp1 = MathUtil_Div32(temp1, ((*game).numBigSparkleChecks as i32) << 8) + 12800;
            temp1 = temp1 >> 8;
            (*game).results.silkiness = temp1 as u16 & 0x7F;
            temp1 = temp1 << 8;
            temp1 = MathUtil_Div32(temp1, 25600);
            temp2 = ((*game).powder * (*game).playerCount as i32) << 8;
            temp2 = MathUtil_Mul32(temp2, temp1);
            (*game).results.powder = (temp2 >> 8) as u32;
            (*game).results.playerIdsRanked[0][7] = (Random() as i32 % 3) as u8;
            i = 0;
            while i < (*game).playerCount {
                (*game).results.playerIdsRanked[0][i] = i;
                (*game).results.playerIdsRanked[1][i] = i;
                (*game).results.stats[0][i] = (*game).players[i].numAPresses;
                (*game).results.totalAPresses += (*game).results.stats[0][i];
                match (*game).results.playerIdsRanked[0][7] {
                    RESULTS_PAGE_NEATNESS => {
                        if (*game).players[i].numAPresses != 0 {
                            temp1 = (*game).players[i].maxNeatInputStreak as i32;
                            temp1 = temp1 << 8;
                            temp1 = MathUtil_Mul32(temp1, 25600);
                            temp2 = (*game).players[i].numAPresses as i32;
                            temp2 = temp2 << 8;
                            temp2 = MathUtil_Div32(temp1, temp2);
                        } else {
                            temp2 = 0;
                        }
                    }
                    RESULTS_PAGE_COOPERATIVE => {
                        if (*game).players[i].numAPresses != 0 {
                            temp1 = (*game).players[i].numSyncedAPresses as i32;
                            temp1 = temp1 << 8;
                            temp1 = MathUtil_Mul32(temp1, 25600);
                            temp2 = (*game).players[i].numAPresses as i32;
                            temp2 = temp2 << 8;
                            temp2 = MathUtil_Div32(temp1, temp2);
                        } else {
                            temp2 = 0;
                        }
                    }
                    RESULTS_PAGE_POWER => {
                        if (*game).players[i].numAPresses == 0 {
                            temp2 = 0;
                        } else if (*game).players[i].timePressingA >= (*game).timer {
                            temp2 = 25600;
                        } else {
                            temp1 = (*game).players[i].timePressingA as i32;
                            temp1 = temp1 << 8;
                            temp1 = MathUtil_Mul32(temp1, 25600);
                            temp2 = (*game).timer as i32;
                            temp2 = temp2 << 8;
                            temp2 = MathUtil_Div32(temp1, temp2);
                        }
                    }
                    _ => {}
                }
                temp2 >>= 4;
                (*game).results.stats[1][i] = temp2 as u16;
                i += 1;
            }
        }
        4 => {
            i = 0;
            while (i as i32) < (*game).playerCount as i32 - 1 {
                j = (*game).playerCount - 1;
                while j > i {
                    if (*game).results.stats[0][j as i32 - 1] < (*game).results.stats[0][j] {
                        tempStat = (*game).results.stats[0][j];
                        (*game).results.stats[0][j] = (*game).results.stats[0][j as i32 - 1];
                        (*game).results.stats[0][j as i32 - 1] = tempStat;
                        tempPlayerId = (*game).results.playerIdsRanked[0][j];
                        (*game).results.playerIdsRanked[0][j] =
                            (*game).results.playerIdsRanked[0][j as i32 - 1];
                        (*game).results.playerIdsRanked[0][j as i32 - 1] = tempPlayerId;
                    }
                    if (*game).results.stats[1][j as i32 - 1] < (*game).results.stats[1][j] {
                        tempStat = (*game).results.stats[1][j];
                        (*game).results.stats[1][j] = (*game).results.stats[1][j as i32 - 1];
                        (*game).results.stats[1][j as i32 - 1] = tempStat;
                        tempPlayerId = (*game).results.playerIdsRanked[1][j];
                        (*game).results.playerIdsRanked[1][j] =
                            (*game).results.playerIdsRanked[1][j as i32 - 1];
                        (*game).results.playerIdsRanked[1][j as i32 - 1] = tempPlayerId;
                    }
                    j -= 1;
                }
                i += 1;
            }
            SendBlock(0, &raw mut (*game).results as *mut c_void, 48);
        }
        5 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            (*game).cmdTimer = 0;
        }
        6 => {
            if GetBlockReceivedStatus() != 1 {
                return 0;
            }
            memset(&raw mut (*game).results as *mut u8, 0, 48);
            memcpy(
                &raw mut (*game).results as *mut u8,
                gBlockRecvBuffer.as_mut_ptr() as *mut u8,
                48,
            );
            ResetBlockReceivedFlags();
            (*game).cmdTimer = 0;
        }
        7 => {
            SaveResults();
            RunOrScheduleCommand(CMD_SHOW_RESULTS, SCHEDULE_CMD, null_mut());
            (*game).gameState = STATE_RESULTS_PRESSES;
            (*game).cmdState = 0;
            (*game).newDepth = 0;
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_ShowResults(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    match (*game).cmdState {
        0 => {
            if OpenResultsWindow(game, &raw mut (*game).gfx) == 0 {
                return 0;
            }
        }
        1 => {
            CopyBgTilemapBufferToVram(0);
            (*game).gfx.counter = 30;
        }
        2 => {
            if (*game).gfx.counter != 0 {
                (*game).gfx.counter -= 1;
                return 0;
            }
            if gMain.newKeys as i32 & A_BUTTON == 0 {
                return 0;
            }
            PlaySE(SE_SELECT);
            CloseResultsWindow(game);
        }
        3 => {
            if (*game).gameState < STATE_RESULTS_CRUSHING {
                (*game).gameState += 1;
                (*game).cmdState = 0;
                return 0;
            }
        }
        4 => {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                (*game).powder,
                STR_CONV_MODE_LEFT_ALIGN,
                6,
            );
            ConvertIntToDecimalStringN(
                gStringVar2.as_mut_ptr(),
                GetBerryPowder() as i32,
                STR_CONV_MODE_LEFT_ALIGN,
                6,
            );
            SetPrintMessageArgs(args, MSG_POWDER, 3, 0, 0);
            (*game).nextCmd = CMD_SAVE;
            RunOrScheduleCommand(CMD_PRINT_MSG, SCHEDULE_CMD, null_mut());
            (*game).cmdState = 0;
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_SaveGame(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    match (*game).cmdState {
        0 => {
            if (*game).timer >= MAX_TIME {
                HideTimer(&raw mut (*game).gfx);
            }
            SetPrintMessageArgs(args, MSG_COMM_STANDBY, 0, 0, 1);
            (*game).nextCmd = CMD_SAVE;
            RunOrScheduleCommand(CMD_PRINT_MSG, SCHEDULE_CMD, null_mut());
            (*game).cmdState = 0;
            return 0;
        }
        1 => {
            Rfu_SetLinkStandbyCallback();
        }
        2 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            DrawDialogueFrame(0, 0);
            AddTextPrinterParameterized2(
                0,
                FONT_NORMAL,
                gText_SavingDontTurnOffPower.as_ptr().cast_mut(),
                0,
                None,
                TEXT_COLOR_DARK_GRAY,
                TEXT_COLOR_WHITE,
                TEXT_COLOR_LIGHT_GRAY,
            );
            CopyWindowToVram(0, COPYWIN_FULL);
            CreateTask(Some(Task_LinkFullSave), 0);
        }
        3 => {
            if FuncIsActiveTask(Some(Task_LinkFullSave)) != 0 {
                return 0;
            }
        }
        4 => {
            RunOrScheduleCommand(CMD_ASK_PLAY_AGAIN, SCHEDULE_CMD, null_mut());
            (*game).gameState = STATE_PLAY_AGAIN;
            (*game).cmdState = 0;
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_AskPlayAgain(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    let mut input: i8 = 0;
    match (*game).cmdState {
        0 => {
            SetPrintMessageArgs(args, MSG_PLAY_AGAIN, 0, 0, 1);
            (*game).nextCmd = CMD_ASK_PLAY_AGAIN as u8;
            RunOrScheduleCommand(CMD_PRINT_MSG, SCHEDULE_CMD, null_mut());
            (*game).cmdState = 0;
            return 0;
        }
        1 => {
            DisplayYesNoMenuDefaultYes();
        }
        2 => {
            input = Menu_ProcessInputNoWrapClearOnChoose();
            if input != -2 {
                memset((*game).sendCmd.as_mut_ptr() as *mut u8, 0, 12);
                if input == 0 {
                    if HasAtLeastOneBerry() != 0 {
                        (*game).playAgainState = PLAY_AGAIN_YES;
                    } else {
                        (*game).playAgainState = PLAY_AGAIN_NO_BERRIES;
                    }
                } else {
                    (*game).playAgainState = PLAY_AGAIN_NO;
                }
                ClearDialogWindowAndFrame(0, TRUE);
                SetPrintMessageArgs(args, MSG_COMM_STANDBY, 0, 0, 0);
                (*game).nextCmd = CMD_COMM_PLAY_AGAIN;
                RunOrScheduleCommand(CMD_PRINT_MSG, SCHEDULE_CMD, null_mut());
                (*game).cmdState = 0;
            }
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_CommunicatePlayAgainResponses(
    game: *mut BerryCrushGame,
    args: *mut u8,
) -> u32 {
    let mut i: u8 = 0;
    match (*game).cmdState {
        0 => {
            Rfu_SetLinkStandbyCallback();
        }
        1 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            (*game).sendCmd[0] = (*game).playAgainState;
            (*game).recvCmd[0] = 0;
            SendBlock(0, (*game).sendCmd.as_mut_ptr() as *mut c_void, 2);
        }
        2 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            (*game).cmdTimer = 0;
        }
        3 => {
            if GetBlockReceivedStatus() != sReceivedPlayerBitmasks[(*game).playerCount as i32 - 2] {
                return 0;
            }
            i = 0;
            while i < (*game).playerCount {
                (*game).recvCmd[0] += gBlockRecvBuffer[i][0];
                i += 1;
            }
            if (*game).recvCmd[0] != 0 {
                RunOrScheduleCommand(CMD_PLAY_AGAIN_NO, SCHEDULE_CMD, null_mut());
            } else {
                RunOrScheduleCommand(CMD_PLAY_AGAIN_YES, SCHEDULE_CMD, null_mut());
            }
            ResetBlockReceivedFlags();
            (*game).sendCmd[0] = 0;
            (*game).recvCmd[0] = 0;
            (*game).cmdTimer = 0;
            (*game).cmdState = 0;
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_PlayAgain(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    match (*game).cmdState {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 1, 0, 16, 0);
            UpdatePaletteFade();
        }
        1 => {
            if UpdatePaletteFade() != 0 {
                return 0;
            }
        }
        2 => {
            ClearDialogWindowAndFrame(0, TRUE);
            ResetCrusherPos(game);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            UpdatePaletteFade();
        }
        3 => {
            if UpdatePaletteFade() != 0 {
                return 0;
            }
            RunOrScheduleCommand(CMD_ASK_PICK_BERRY, SCHEDULE_CMD, null_mut());
            (*game).gameState = STATE_PICK_BERRY;
            (*game).cmdState = 0;
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_StopGame(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    match (*game).cmdState {
        0 => {
            DrawDialogueFrame(0, 0);
            if (*game).playAgainState == PLAY_AGAIN_NO_BERRIES {
                AddTextPrinterParameterized2(
                    0,
                    FONT_NORMAL,
                    sMessages[5],
                    (*game).textSpeed,
                    None,
                    TEXT_COLOR_DARK_GRAY,
                    TEXT_COLOR_WHITE,
                    TEXT_COLOR_LIGHT_GRAY,
                );
            } else {
                AddTextPrinterParameterized2(
                    0,
                    FONT_NORMAL,
                    sMessages[6],
                    (*game).textSpeed,
                    None,
                    TEXT_COLOR_DARK_GRAY,
                    TEXT_COLOR_WHITE,
                    TEXT_COLOR_LIGHT_GRAY,
                );
            }
            CopyWindowToVram(0, COPYWIN_FULL);
        }
        1 => {
            if IsTextPrinterActive(0) != 0 {
                return 0;
            }
            (*game).gfx.counter = 120;
        }
        2 => {
            if (*game).gfx.counter != 0 {
                (*game).gfx.counter -= 1;
            } else {
                RunOrScheduleCommand(CMD_CLOSE_LINK, SCHEDULE_CMD, null_mut());
                (*game).cmdState = 0;
            }
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_CloseLink(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    match (*game).cmdState {
        0 => {
            Rfu_SetLinkStandbyCallback();
        }
        1 => {
            if IsLinkTaskFinished() == 0 {
                return 0;
            }
            SetCloseLinkCallback();
        }
        2 => {
            if gReceivedRemoteLinkPlayers != 0 {
                return 0;
            }
            (*game).nextCmd = CMD_QUIT;
            RunOrScheduleCommand(CMD_HIDE_GAME, SCHEDULE_CMD, null_mut());
            (*game).cmdState = 2;
            return 0;
        }
        _ => {}
    }
    (*game).cmdState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn Cmd_Quit(game: *mut BerryCrushGame, args: *mut u8) -> u32 {
    QuitBerryCrush(None);
    return 0;
}
pub(crate) unsafe extern "C" fn ResetGame(game: *mut BerryCrushGame) {
    let mut i: u8 = 0;
    IncrementGameStat(GAME_STAT_PLAYED_BERRY_CRUSH);
    (*game).unused = 0;
    (*game).cmdTimer = 0;
    (*game).gameState = STATE_RESET;
    (*game).playAgainState = 0;
    (*game).powder = 0;
    (*game).targetAPresses = 0;
    (*game).totalAPresses = 0;
    (*game).targetDepth = 0;
    (*game).newDepth = 0;
    (*game).set_noRoomForPowder(FALSE);
    (*game).set_newRecord(FALSE);
    (*game).set_playedSound(FALSE);
    (*game).set_endGame(FALSE);
    (*game).set_bigSparkle(FALSE);
    (*game).set_sparkleAmount(0);
    (*game).leaderTimer = 0;
    (*game).timer = 0;
    (*game).bigSparkleCounter = 0;
    (*game).numBigSparkleChecks = -1;
    (*game).numBigSparkles = 0;
    (*game).sparkleCounter = 0;
    i = 0;
    while i < MAX_RFU_PLAYERS as u8 {
        (*game).players[i].berryId = 65535;
        (*game).players[i].inputTime = 0;
        (*game).players[i].neatInputStreak = 0;
        (*game).players[i].timeSincePrevInput = 1;
        (*game).players[i].maxNeatInputStreak = 0;
        (*game).players[i].numAPresses = 0;
        (*game).players[i].numSyncedAPresses = 0;
        (*game).players[i].timePressingA = 0;
        (*game).players[i].inputFlags = 0;
        (*game).players[i].inputState = INPUT_STATE_NONE;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetPaletteFadeArgs(
    mut args: *mut u8,
    communicateAfter: u8,
    mut selectedPals: u32,
    delay: i8,
    startY: u8,
    targetY: u8,
    mut palette: u16,
) {
    *args = *(&raw mut selectedPals as *mut u8);
    *args.at(1) = *(&raw mut selectedPals as *mut u8).at(1);
    *args.at(2) = *(&raw mut selectedPals as *mut u8).at(2);
    *args.at(3) = *(&raw mut selectedPals as *mut u8).at(3);
    *args.at(4) = delay as u8;
    *args.at(5) = startY;
    *args.at(6) = targetY;
    *args.at(7) = *(&raw mut palette as *mut u8);
    *args.at(8) = *(&raw mut palette as *mut u8).at(1);
    *args.at(9) = communicateAfter;
}
pub(crate) unsafe extern "C" fn SetPrintMessageArgs(
    mut args: *mut u8,
    msgId: u8,
    flags: u8,
    mut waitKeys: u16,
    followupState: u8,
) {
    *args = msgId;
    *args.at(1) = flags;
    *args.at(2) = *(&raw mut waitKeys as *mut u8);
    *args.at(3) = *(&raw mut waitKeys as *mut u8).at(1);
    *args.at(4) = followupState;
}
