//! Translated from `src/pokemon_jump.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sPokeJumpMons sPokeJumpLeaderFuncs sPokeJumpMemberFuncs sVineBaseSpeeds sVineSpeedDelays sSoundEffects sJumpOffsets sScoreBonuses sPrizeItems sPrizeQuantityData sPokeJumpPal1 sPokeJumpPal2 sVine1_Gfx sVine2_Gfx sVine3_Gfx sVine4_Gfx sStar_Gfx sCompressedSpriteSheets sSpritePalettes sOamData_JumpMon sSpriteTemplate_Vine1 sSpriteTemplate_Vine2 sSpriteTemplate_Vine3 sSpriteTemplate_Vine4 sSpriteTemplate_JumpMon sVineYCoords sVineXCoords sSpriteTemplates_Vine sOamData_JumpMon sOamData_Vine16x32 sOamData_Vine32x32 sOamData_Vine32x16 sAnims_Vine_Highest sAnims_Vine_Higher sAnims_Vine_High sAnims_Vine_Low sAnims_Vine_Lower sAnims_Vine_Lowest sAnims_VineTall_Highest sAnims_VineTall_Higher sAnims_VineTall_High sAnims_VineTall_Low sAnims_VineTall_Lower sAnims_VineTall_Lowest sAnims_Vine sAnims_VineTall sSpriteTemplate_Vine1 sSpriteTemplate_Vine2 sSpriteTemplate_Vine3 sSpriteTemplate_Vine4 sOamData_Star sAnim_Star_Still sAnim_Star_Spinning sAnims_Star sSpriteTemplate_Star sInterface_Pal sBg_Pal sBg_Gfx sBg_Tilemap sVenusaur_Pal sVenusaur_Gfx sVenusaur_Tilemap sBonuses_Pal sBonuses_Gfx sBonuses_Tilemap sBgTemplates sWindowTemplates sPokeJumpGfxFuncs sVenusaurStates sSpriteSheet_Digits sSpritePalette_Digits sPlayerNameWindowCoords_2Players sPlayerNameWindowCoords_3Players sPlayerNameWindowCoords_4Players sPlayerNameWindowCoords_5Players sPlayerNameWindowCoords sMonXCoords_2Players sMonXCoords_3Players sMonXCoords_4Players sMonXCoords_5Players sMonXCoords sWindowTemplate_Records sRecordsTexts

/// `struct PokemonJump`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonJump {
    pub exitCallback: Option<unsafe extern "C" fn()>,
    pub taskId: u8,
    pub numPlayers: u8,
    pub multiplayerId: u8,
    pub startDelayTimer: u8,
    pub mainState: u16,
    pub helperState: u16,
    pub excellentsInRow: u16,
    pub excellentsInRowRecord: u16,
    pub gameOver: u32,
    pub vineState: u32,
    pub prevVineState: u32,
    pub vineSpeed: i32,
    pub vineSpeedAccel: u32,
    pub rngSeed: u32,
    pub nextVineSpeed: u32,
    pub linkTimer: i32,
    pub linkTimerLimit: u32,
    pub vineStateTimer: u16,
    pub ignoreJumpInput: u16,
    pub unused1: u16,
    pub unused2: u16,
    pub timer: u16,
    pub prizeItemId: u16,
    pub prizeItemQuantity: u16,
    pub playAgainComm: u16,
    pub unused3: u8,
    pub playAgainState: u8,
    pub allowVineUpdates: u8,
    pub isLeader: u8,
    pub funcActive: u8,
    pub allPlayersReady: u8,
    pub vineTimer: u16,
    pub nextFuncId: u8,
    pub showBonus: u8,
    pub vineSpeedDelay: u16,
    pub vineBaseSpeedIdx: u8,
    pub vineSpeedStage: u8,
    pub numPlayersAtPeak: i32,
    pub initScoreUpdate: u32,
    pub updateScore: u32,
    pub unused4: u32,
    pub giveBonus: u32,
    pub skipJumpUpdate: u32,
    pub atMaxSpeedStage: u32,
    pub comm: PokemonJump_CommData,
    pub atJumpPeak: CArray<u8, 5>,
    pub atJumpPeak2: CArray<u8, 5>,
    pub atJumpPeak3: CArray<u8, 5>,
    pub memberFuncIds: CArray<u8, 5>,
    pub playAgainStates: CArray<u16, 5>,
    pub jumpTimeStarts: CArray<u16, 5>,
    pub jumpGfx: PokemonJumpGfx,
    pub monInfo: CArray<PokemonJump_MonInfo, 5>,
    pub players: CArray<PokemonJump_Player, 5>,
    pub player: *mut PokemonJump_Player,
}

unsafe impl Sync for PokemonJump {}

/// `struct PokemonJumpGfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonJumpGfx {
    pub funcFinished: u32,
    pub mainState: u16,
    pub taskId: u8,
    pub unused1: CArray<u8, 3>,
    pub resetVineState: u8,
    pub resetVineTimer: u8,
    pub vineState: u8,
    pub msgWindowState: u8,
    pub vinePalNumDownswing: u8,
    pub vinePalNumUpswing: u8,
    pub unused2: u16,
    pub msgWindowId: u16,
    pub fanfare: u16,
    pub bonusTimer: u32,
    pub nameWindowIds: CArray<u16, 5>,
    pub itemName: CArray<u8, 64>,
    pub itemQuantityStr: CArray<u8, 64>,
    pub prizeMsg: CArray<u8, 256>,
    pub tilemapBuffer: CArray<u16, 16384>,
    pub monSprites: CArray<*mut Sprite, 5>,
    pub starSprites: CArray<*mut Sprite, 5>,
    pub vineSprites: CArray<*mut Sprite, 8>,
    pub unused3: CArray<u8, 12>,
    pub monSpriteSubpriorities: CArray<u8, 5>,
}

unsafe impl Sync for PokemonJumpGfx {}

/// `struct PokemonJump_MonInfo`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonJump_MonInfo {
    pub species: u16,
    pub otId: u32,
    pub personality: u32,
}

unsafe impl Sync for PokemonJump_MonInfo {}

/// `struct PokemonJump_Player`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonJump_Player {
    pub jumpOffset: i32,
    pub jumpOffsetIdx: i32,
    pub unused: u32,
    pub monJumpType: u16,
    pub jumpTimeStart: u16,
    pub monState: u16,
    pub prevMonState: u16,
    pub jumpState: i32,
    pub funcFinished: u32,
    pub name: CArray<u8, 11>,
}

unsafe impl Sync for PokemonJump_Player {}

/// `struct PokemonJump_CommData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonJump_CommData {
    pub funcId: u8,
    pub receivedBonusFlags: u8,
    pub data: u16,
    pub jumpsInRow: u16,
    pub jumpScore: u32,
}

unsafe impl Sync for PokemonJump_CommData {}

/// `struct MonInfoPacket`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MonInfoPacket {
    pub id: u8,
    pub species: u16,
    pub personality: u32,
    pub otId: u32,
}

unsafe impl Sync for MonInfoPacket {}

/// `struct UnusedPacket`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnusedPacket {
    pub id: u8,
    pub data: u32,
    pub filler: u32,
}

unsafe impl Sync for UnusedPacket {}

/// `struct LeaderStatePacket`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LeaderStatePacket {
    pub id: u8,
    pub funcId: u8,
    pub monState: u8,
    bits_3: u8,
    pub jumpTimeStart: u16,
    pub vineTimer: u16,
    bits_8: u32,
}

impl LeaderStatePacket {
    #[inline(always)]
    pub fn receivedBonusFlags(&self) -> u8 {
        ((self.bits_3 as u32 >> 0) & 0x1f) as u8
    }
    #[inline(always)]
    pub fn set_receivedBonusFlags(&mut self, v: u8) {
        self.bits_3 = (self.bits_3 & !(0x1f << 0)) | ((v as u8 & 0x1f) << 0);
    }
    #[inline(always)]
    pub fn jumpState(&self) -> u8 {
        ((self.bits_3 as u32 >> 5) & 0x7) as u8
    }
    #[inline(always)]
    pub fn set_jumpState(&mut self, v: u8) {
        self.bits_3 = (self.bits_3 & !(0x7 << 5)) | ((v as u8 & 0x7) << 5);
    }
    #[inline(always)]
    pub fn jumpsInRow(&self) -> u32 {
        ((self.bits_8 as u32 >> 0) & 0x7fff) as u32
    }
    #[inline(always)]
    pub fn set_jumpsInRow(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x7fff << 0)) | ((v as u32 & 0x7fff) << 0);
    }
    #[inline(always)]
    pub fn jumpScore(&self) -> u32 {
        ((self.bits_8 as u32 >> 15) & 0x1ffff) as u32
    }
    #[inline(always)]
    pub fn set_jumpScore(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x1ffff << 15)) | ((v as u32 & 0x1ffff) << 15);
    }
}

unsafe impl Sync for LeaderStatePacket {}

/// `struct MemberStatePacket`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct MemberStatePacket {
    pub id: u8,
    pub monState: u8,
    pub jumpState: u8,
    pub funcFinished: u8,
    pub jumpTimeStart: u16,
    pub funcId: u8,
    pub playAgainState: u16,
}

unsafe impl Sync for MemberStatePacket {}

/// `__typeof__(sPokeJumpGfxFuncs[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sPokeJumpGfxFuncs_0_t {
    pub id: i32,
    pub func: Option<unsafe extern "C" fn()>,
}

unsafe impl Sync for sPokeJumpGfxFuncs_0_t {}

/// `struct PokemonJumpMons`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PokemonJumpMons {
    pub species: u16,
    pub jumpType: u16,
}

unsafe impl Sync for PokemonJumpMons {}

/// `__typeof__(sPrizeQuantityData[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sPrizeQuantityData_0_t {
    pub score: u32,
    pub quantity: u32,
}

unsafe impl Sync for sPrizeQuantityData_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PokemonJump>() == 33712);
    assert!(offset_of!(PokemonJump, exitCallback) == 0);
    assert!(offset_of!(PokemonJump, taskId) == 4);
    assert!(offset_of!(PokemonJump, numPlayers) == 5);
    assert!(offset_of!(PokemonJump, multiplayerId) == 6);
    assert!(offset_of!(PokemonJump, startDelayTimer) == 7);
    assert!(offset_of!(PokemonJump, mainState) == 8);
    assert!(offset_of!(PokemonJump, helperState) == 10);
    assert!(offset_of!(PokemonJump, excellentsInRow) == 12);
    assert!(offset_of!(PokemonJump, excellentsInRowRecord) == 14);
    assert!(offset_of!(PokemonJump, gameOver) == 16);
    assert!(offset_of!(PokemonJump, vineState) == 20);
    assert!(offset_of!(PokemonJump, prevVineState) == 24);
    assert!(offset_of!(PokemonJump, vineSpeed) == 28);
    assert!(offset_of!(PokemonJump, vineSpeedAccel) == 32);
    assert!(offset_of!(PokemonJump, rngSeed) == 36);
    assert!(offset_of!(PokemonJump, nextVineSpeed) == 40);
    assert!(offset_of!(PokemonJump, linkTimer) == 44);
    assert!(offset_of!(PokemonJump, linkTimerLimit) == 48);
    assert!(offset_of!(PokemonJump, vineStateTimer) == 52);
    assert!(offset_of!(PokemonJump, ignoreJumpInput) == 54);
    assert!(offset_of!(PokemonJump, unused1) == 56);
    assert!(offset_of!(PokemonJump, unused2) == 58);
    assert!(offset_of!(PokemonJump, timer) == 60);
    assert!(offset_of!(PokemonJump, prizeItemId) == 62);
    assert!(offset_of!(PokemonJump, prizeItemQuantity) == 64);
    assert!(offset_of!(PokemonJump, playAgainComm) == 66);
    assert!(offset_of!(PokemonJump, unused3) == 68);
    assert!(offset_of!(PokemonJump, playAgainState) == 69);
    assert!(offset_of!(PokemonJump, allowVineUpdates) == 70);
    assert!(offset_of!(PokemonJump, isLeader) == 71);
    assert!(offset_of!(PokemonJump, funcActive) == 72);
    assert!(offset_of!(PokemonJump, allPlayersReady) == 73);
    assert!(offset_of!(PokemonJump, vineTimer) == 74);
    assert!(offset_of!(PokemonJump, nextFuncId) == 76);
    assert!(offset_of!(PokemonJump, showBonus) == 77);
    assert!(offset_of!(PokemonJump, vineSpeedDelay) == 78);
    assert!(offset_of!(PokemonJump, vineBaseSpeedIdx) == 80);
    assert!(offset_of!(PokemonJump, vineSpeedStage) == 81);
    assert!(offset_of!(PokemonJump, numPlayersAtPeak) == 84);
    assert!(offset_of!(PokemonJump, initScoreUpdate) == 88);
    assert!(offset_of!(PokemonJump, updateScore) == 92);
    assert!(offset_of!(PokemonJump, unused4) == 96);
    assert!(offset_of!(PokemonJump, giveBonus) == 100);
    assert!(offset_of!(PokemonJump, skipJumpUpdate) == 104);
    assert!(offset_of!(PokemonJump, atMaxSpeedStage) == 108);
    assert!(offset_of!(PokemonJump, comm) == 112);
    assert!(offset_of!(PokemonJump, atJumpPeak) == 124);
    assert!(offset_of!(PokemonJump, atJumpPeak2) == 129);
    assert!(offset_of!(PokemonJump, atJumpPeak3) == 134);
    assert!(offset_of!(PokemonJump, memberFuncIds) == 139);
    assert!(offset_of!(PokemonJump, playAgainStates) == 144);
    assert!(offset_of!(PokemonJump, jumpTimeStarts) == 154);
    assert!(offset_of!(PokemonJump, jumpGfx) == 164);
    assert!(offset_of!(PokemonJump, monInfo) == 33448);
    assert!(offset_of!(PokemonJump, players) == 33508);
    assert!(offset_of!(PokemonJump, player) == 33708);
    assert!(size_of::<PokemonJumpGfx>() == 33284);
    assert!(offset_of!(PokemonJumpGfx, funcFinished) == 0);
    assert!(offset_of!(PokemonJumpGfx, mainState) == 4);
    assert!(offset_of!(PokemonJumpGfx, taskId) == 6);
    assert!(offset_of!(PokemonJumpGfx, unused1) == 7);
    assert!(offset_of!(PokemonJumpGfx, resetVineState) == 10);
    assert!(offset_of!(PokemonJumpGfx, resetVineTimer) == 11);
    assert!(offset_of!(PokemonJumpGfx, vineState) == 12);
    assert!(offset_of!(PokemonJumpGfx, msgWindowState) == 13);
    assert!(offset_of!(PokemonJumpGfx, vinePalNumDownswing) == 14);
    assert!(offset_of!(PokemonJumpGfx, vinePalNumUpswing) == 15);
    assert!(offset_of!(PokemonJumpGfx, unused2) == 16);
    assert!(offset_of!(PokemonJumpGfx, msgWindowId) == 18);
    assert!(offset_of!(PokemonJumpGfx, fanfare) == 20);
    assert!(offset_of!(PokemonJumpGfx, bonusTimer) == 24);
    assert!(offset_of!(PokemonJumpGfx, nameWindowIds) == 28);
    assert!(offset_of!(PokemonJumpGfx, itemName) == 38);
    assert!(offset_of!(PokemonJumpGfx, itemQuantityStr) == 102);
    assert!(offset_of!(PokemonJumpGfx, prizeMsg) == 166);
    assert!(offset_of!(PokemonJumpGfx, tilemapBuffer) == 422);
    assert!(offset_of!(PokemonJumpGfx, monSprites) == 33192);
    assert!(offset_of!(PokemonJumpGfx, starSprites) == 33212);
    assert!(offset_of!(PokemonJumpGfx, vineSprites) == 33232);
    assert!(offset_of!(PokemonJumpGfx, unused3) == 33264);
    assert!(offset_of!(PokemonJumpGfx, monSpriteSubpriorities) == 33276);
    assert!(size_of::<PokemonJump_MonInfo>() == 12);
    assert!(offset_of!(PokemonJump_MonInfo, species) == 0);
    assert!(offset_of!(PokemonJump_MonInfo, otId) == 4);
    assert!(offset_of!(PokemonJump_MonInfo, personality) == 8);
    assert!(size_of::<PokemonJump_Player>() == 40);
    assert!(offset_of!(PokemonJump_Player, jumpOffset) == 0);
    assert!(offset_of!(PokemonJump_Player, jumpOffsetIdx) == 4);
    assert!(offset_of!(PokemonJump_Player, unused) == 8);
    assert!(offset_of!(PokemonJump_Player, monJumpType) == 12);
    assert!(offset_of!(PokemonJump_Player, jumpTimeStart) == 14);
    assert!(offset_of!(PokemonJump_Player, monState) == 16);
    assert!(offset_of!(PokemonJump_Player, prevMonState) == 18);
    assert!(offset_of!(PokemonJump_Player, jumpState) == 20);
    assert!(offset_of!(PokemonJump_Player, funcFinished) == 24);
    assert!(offset_of!(PokemonJump_Player, name) == 28);
    assert!(size_of::<PokemonJump_CommData>() == 12);
    assert!(offset_of!(PokemonJump_CommData, funcId) == 0);
    assert!(offset_of!(PokemonJump_CommData, receivedBonusFlags) == 1);
    assert!(offset_of!(PokemonJump_CommData, data) == 2);
    assert!(offset_of!(PokemonJump_CommData, jumpsInRow) == 4);
    assert!(offset_of!(PokemonJump_CommData, jumpScore) == 8);
    assert!(size_of::<MonInfoPacket>() == 12);
    assert!(offset_of!(MonInfoPacket, id) == 0);
    assert!(offset_of!(MonInfoPacket, species) == 2);
    assert!(offset_of!(MonInfoPacket, personality) == 4);
    assert!(offset_of!(MonInfoPacket, otId) == 8);
    assert!(size_of::<UnusedPacket>() == 12);
    assert!(offset_of!(UnusedPacket, id) == 0);
    assert!(offset_of!(UnusedPacket, data) == 4);
    assert!(offset_of!(UnusedPacket, filler) == 8);
    assert!(size_of::<LeaderStatePacket>() == 12);
    assert!(offset_of!(LeaderStatePacket, id) == 0);
    assert!(offset_of!(LeaderStatePacket, funcId) == 1);
    assert!(offset_of!(LeaderStatePacket, monState) == 2);
    assert!(offset_of!(LeaderStatePacket, bits_3) == 3);
    assert!(offset_of!(LeaderStatePacket, jumpTimeStart) == 4);
    assert!(offset_of!(LeaderStatePacket, vineTimer) == 6);
    assert!(offset_of!(LeaderStatePacket, bits_8) == 8);
    assert!(size_of::<MemberStatePacket>() == 12);
    assert!(offset_of!(MemberStatePacket, id) == 0);
    assert!(offset_of!(MemberStatePacket, monState) == 1);
    assert!(offset_of!(MemberStatePacket, jumpState) == 2);
    assert!(offset_of!(MemberStatePacket, funcFinished) == 3);
    assert!(offset_of!(MemberStatePacket, jumpTimeStart) == 4);
    assert!(offset_of!(MemberStatePacket, funcId) == 6);
    assert!(offset_of!(MemberStatePacket, playAgainState) == 8);
    assert!(size_of::<sPokeJumpGfxFuncs_0_t>() == 8);
    assert!(offset_of!(sPokeJumpGfxFuncs_0_t, id) == 0);
    assert!(offset_of!(sPokeJumpGfxFuncs_0_t, func) == 4);
    assert!(size_of::<PokemonJumpMons>() == 4);
    assert!(offset_of!(PokemonJumpMons, species) == 0);
    assert!(offset_of!(PokemonJumpMons, jumpType) == 2);
    assert!(size_of::<sPrizeQuantityData_0_t>() == 8);
    assert!(offset_of!(sPrizeQuantityData_0_t, score) == 0);
    assert!(offset_of!(sPrizeQuantityData_0_t, quantity) == 4);
};

const BG_BONUSES: u8 = 1;
const BG_INTERFACE: u8 = 0;
const BG_SCENERY: u8 = 3;
const BG_VENUSAUR: u8 = 2;
const DATAIDX_GAME_STRUCT: u8 = 14;
const FUNC_ASK_PLAY_AGAIN: u8 = 4;
const FUNC_EXIT: u8 = 6;
const FUNC_GAME_INTRO: u8 = 0;
const FUNC_GAME_OVER: u8 = 3;
const FUNC_GAME_ROUND: u8 = 2;
const FUNC_GIVE_PRIZE: u8 = 7;
const FUNC_NONE: u8 = 9;
const FUNC_RESET_GAME: u8 = 5;
const FUNC_SAVE: u8 = 8;
const FUNC_WAIT_ROUND: u8 = 1;
const F_SE_FAIL: i32 = 2;
const F_SE_JUMP: i32 = 1;
const GFXFUNC_COUNTDOWN: i32 = 9;
const GFXFUNC_ERASE_MSG: i32 = 6;
const GFXFUNC_ERASE_NAMES: i32 = 3;
const GFXFUNC_MSG_COMM_STANDBY: i32 = 8;
const GFXFUNC_MSG_PLAYER_DROPPED: i32 = 7;
const GFXFUNC_MSG_PLAY_AGAIN: i32 = 4;
const GFXFUNC_MSG_SAVING: i32 = 5;
const GFXFUNC_SHOW_NAMES: i32 = 1;
const GFXFUNC_SHOW_NAMES_HIGHLIGHT: i32 = 2;
const GFXTAG_COUNTDOWN: u16 = 9;
const JUMPSTATE_FAILURE: i32 = 2;
const JUMPSTATE_NONE: i32 = 0;
const JUMPSTATE_SUCCESS: i32 = 1;
const JUMP_PEAK: i32 = -30;
const LINK_INTERVAL_LONG: i32 = 5;
const LINK_INTERVAL_MEDIUM: i32 = 4;
const LINK_INTERVAL_NONE: i32 = 0;
const LINK_INTERVAL_SHORT: i32 = 3;
const LINK_TIMER_STOPPED: u32 = 4369;
const MAX_JUMPS: u16 = 9999;
const MAX_JUMP_SCORE: u32 = 0x18696;
const MONSTATE_HIT: u16 = 2;
const MONSTATE_JUMP: u16 = 1;
const MONSTATE_NORMAL: u16 = 0;
const NUM_VINESTATES: i32 = 10;
const NUM_WINDOWS: u32 = 2;
const PACKET_LEADER_STATE: u8 = 3;
const PACKET_MEMBER_STATE: u8 = 4;
const PACKET_MON_INFO: u8 = 1;
const PACKET_UNUSED: u8 = 2;
const PALTAG_1: u16 = 5;
const PALTAG_2: u16 = 6;
const PALTAG_COUNTDOWN: u16 = 7;
const PLAY_AGAIN_NO: u8 = 1;
const PLAY_AGAIN_YES: u8 = 2;
const VINE_HIGHEST: u32 = 0;
const VINE_LOWEST: i32 = 5;
const VINE_SPRITES_PER_SIDE: i32 = 4;
const VINE_UPSWING_HIGH: u32 = 8;
const VINE_UPSWING_LOW: u32 = 7;
const VINE_UPSWING_LOWER: u32 = 6;
const WIN_POINTS: u8 = 0;
const WIN_TIMES: u8 = 1;

static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::pokemon_jump::sBgTemplates).cast());
static sBg_Gfx: Table<CArray<u32, 139>> =
    Table((&raw const crate::data::pokemon_jump::sBg_Gfx).cast());
static sBg_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_jump::sBg_Pal).cast());
static sBg_Tilemap: Table<CArray<u32, 108>> =
    Table((&raw const crate::data::pokemon_jump::sBg_Tilemap).cast());
static sBonuses_Gfx: Table<CArray<u32, 684>> =
    Table((&raw const crate::data::pokemon_jump::sBonuses_Gfx).cast());
static sBonuses_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_jump::sBonuses_Pal).cast());
static sBonuses_Tilemap: Table<CArray<u32, 331>> =
    Table((&raw const crate::data::pokemon_jump::sBonuses_Tilemap).cast());
static sCompressedSpriteSheets: Table<CArray<CompressedSpriteSheet, 5>> =
    Table((&raw const crate::data::pokemon_jump::sCompressedSpriteSheets).cast());
static sInterface_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_jump::sInterface_Pal).cast());
static sJumpOffsets: Table<CArray<CArray<i8, 48>, 3>> =
    Table((&raw const crate::data::pokemon_jump::sJumpOffsets).cast());
static sMonXCoords: Table<CArray<*mut i16, 4>> =
    Table((&raw const crate::data::pokemon_jump::sMonXCoords).cast());
static sPlayerNameWindowCoords: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::pokemon_jump::sPlayerNameWindowCoords).cast());
static sPokeJumpGfxFuncs: Table<CArray<sPokeJumpGfxFuncs_0_t, 10>> =
    Table((&raw const crate::data::pokemon_jump::sPokeJumpGfxFuncs).cast());
static sPokeJumpLeaderFuncs: Table<CArray<Option<unsafe extern "C" fn() -> u32>, 9>> =
    Table((&raw const crate::data::pokemon_jump::sPokeJumpLeaderFuncs).cast());
static sPokeJumpMemberFuncs: Table<CArray<Option<unsafe extern "C" fn() -> u32>, 9>> =
    Table((&raw const crate::data::pokemon_jump::sPokeJumpMemberFuncs).cast());
static sPokeJumpMons: Table<CArray<PokemonJumpMons, 100>> =
    Table((&raw const crate::data::pokemon_jump::sPokeJumpMons).cast());
static sPrizeItems: Table<CArray<u16, 8>> =
    Table((&raw const crate::data::pokemon_jump::sPrizeItems).cast());
static sPrizeQuantityData: Table<CArray<sPrizeQuantityData_0_t, 5>> =
    Table((&raw const crate::data::pokemon_jump::sPrizeQuantityData).cast());
static sRecordsTexts: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::pokemon_jump::sRecordsTexts).cast());
static sScoreBonuses: Table<CArray<i32, 6>> =
    Table((&raw const crate::data::pokemon_jump::sScoreBonuses).cast());
static sSoundEffects: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::pokemon_jump::sSoundEffects).cast());
static sSpritePalette_Digits: Table<SpritePalette> =
    Table((&raw const crate::data::pokemon_jump::sSpritePalette_Digits).cast());
static sSpritePalettes: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::pokemon_jump::sSpritePalettes).cast());
static sSpriteSheet_Digits: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::pokemon_jump::sSpriteSheet_Digits).cast());
static sSpriteTemplate_JumpMon: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_jump::sSpriteTemplate_JumpMon).cast());
static sSpriteTemplate_Star: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_jump::sSpriteTemplate_Star).cast());
static sSpriteTemplates_Vine: Table<CArray<*mut SpriteTemplate, 4>> =
    Table((&raw const crate::data::pokemon_jump::sSpriteTemplates_Vine).cast());
static sVenusaurStates: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::pokemon_jump::sVenusaurStates).cast());
static sVenusaur_Gfx: Table<CArray<u32, 596>> =
    Table((&raw const crate::data::pokemon_jump::sVenusaur_Gfx).cast());
static sVenusaur_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_jump::sVenusaur_Pal).cast());
static sVenusaur_Tilemap: Table<CArray<u32, 238>> =
    Table((&raw const crate::data::pokemon_jump::sVenusaur_Tilemap).cast());
static sVineBaseSpeeds: Table<CArray<u16, 8>> =
    Table((&raw const crate::data::pokemon_jump::sVineBaseSpeeds).cast());
static sVineSpeedDelays: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::pokemon_jump::sVineSpeedDelays).cast());
static sVineXCoords: Table<CArray<i16, 8>> =
    Table((&raw const crate::data::pokemon_jump::sVineXCoords).cast());
static sVineYCoords: Table<CArray<CArray<i16, 10>, 4>> =
    Table((&raw const crate::data::pokemon_jump::sVineYCoords).cast());
static sWindowTemplate_Records: Table<WindowTemplate> =
    Table((&raw const crate::data::pokemon_jump::sWindowTemplate_Records).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 3>> =
    Table((&raw const crate::data::pokemon_jump::sWindowTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokemonJump: *mut PokemonJump = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokemonJumpGfx: *mut PokemonJumpGfx = null_mut();

unsafe extern "C" {
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gMain: Main;
    static gMonFrontPicCoords: CArray<MonCoords, 0>;
    static gMonStillFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecvCmds: CArray<CArray<u16, 8>, 5>;
    static mut gRfu: RfuManager;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static gSineTable: CArray<i16, 0>;
    static mut gSpecialVar_Result: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gTasks: CArray<Task, 0>;
    static gText_AwesomeWonF701F700: CArray<u8, 0>;
    static gText_CantHoldMore: CArray<u8, 0>;
    static gText_CommunicationStandby4: CArray<u8, 0>;
    static gText_FilledStorageSpace2: CArray<u8, 0>;
    static gText_PkmnJumpRecords: CArray<u8, 0>;
    static gText_SavingDontTurnOffPower: CArray<u8, 0>;
    static gText_SomeoneDroppedOut2: CArray<u8, 0>;
    static gText_SpacePoints2: CArray<u8, 0>;
    static gText_SpaceTimes3: CArray<u8, 0>;
    static gText_WantToPlayAgain2: CArray<u8, 0>;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
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
    fn Alloc(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckBagHasSpace(a0: u16, a1: u16) -> u8;
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CopyItemNameHandlePlural(a0: u16, a1: *mut u8, a2: u32);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroyTask(a0: u8);
    fn DigitObjUtil_CreatePrinter(a0: u32, a1: i32, a2: *mut DigitObjUtilTemplate) -> u32;
    fn DigitObjUtil_Free();
    fn DigitObjUtil_Init(a0: u32) -> u32;
    fn DigitObjUtil_PrintNumOn(a0: u32, a1: i32);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn EraseYesNoWindow();
    fn FadeOutAndPlayNewMapMusic(a0: u16, a1: u8);
    fn FadeOutMapMusic(a0: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetMultiplayerId() -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetWordTaskArg(a0: u8, a1: u8) -> u32;
    fn HandleLoadSpecialPokePic(a0: *mut CompressedSpriteSheet, a1: *mut c_void, a2: i32, a3: u32);
    fn HideBg(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsFanfareTaskInactive() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn IsMinigameCountdownRunning() -> u32;
    fn IsNotWaitingForBGMStop() -> u8;
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LoadUserWindowBorderGfxOnBg(a0: u8, a1: u16, a2: u8);
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn ResetBgPositions();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn Rfu_SendPacket(a0: *mut c_void);
    fn RunTasks();
    fn ScriptContext_Enable();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetCloseLinkCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWordTaskArg(a0: u8, a1: u8, a2: u32);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartMinigameCountdown(a0: u16, a1: u16, a2: i16, a3: i16, a4: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_LinkFullSave(a0: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn rbox_fill_rectangle(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartPokemonJump(
    partyId: u16,
    exitCallback: Option<unsafe extern "C" fn()>,
) {
    let mut taskId: u8 = 0;
    if gReceivedRemoteLinkPlayers != 0 {
        sPokemonJump = Alloc(33712) as *mut PokemonJump;
        if !sPokemonJump.is_null() {
            ResetTasks();
            taskId = CreateTask(Some(Task_StartPokemonJump), 1);
            (*sPokemonJump).mainState = 0;
            (*sPokemonJump).exitCallback = exitCallback;
            (*sPokemonJump).taskId = taskId;
            (*sPokemonJump).multiplayerId = GetMultiplayerId();
            InitJumpMonInfo(
                &raw mut (*sPokemonJump).monInfo[(*sPokemonJump).multiplayerId],
                &raw mut gPlayerParty[partyId],
            );
            InitGame(sPokemonJump);
            SetWordTaskArg(taskId, 2, sPokemonJump as usize as u32);
            SetMainCallback2(Some(CB2_PokemonJump));
            return;
        }
    }
    SetMainCallback2(exitCallback);
}
pub(crate) unsafe extern "C" fn FreePokemonJump() {
    FreeWindowsAndDigitObj();
    Free(sPokemonJump as *mut c_void);
}
pub(crate) unsafe extern "C" fn InitGame(jump: *mut PokemonJump) {
    (*jump).numPlayers = GetLinkPlayerCount();
    (*jump).comm.funcId = FUNC_RESET_GAME;
    (*jump).comm.data = 0;
    InitPlayerAndJumpTypes();
    ResetForNewGame(jump);
    if (*jump).numPlayers == MAX_RFU_PLAYERS as u8 {
        IncrementGamesWithMaxPlayers();
    }
}
pub(crate) unsafe extern "C" fn ResetForNewGame(jump: *mut PokemonJump) {
    let mut i: i32 = 0;
    (*jump).vineState = VINE_UPSWING_LOWER;
    (*jump).prevVineState = VINE_UPSWING_LOWER;
    (*jump).vineTimer = 0;
    (*jump).vineSpeed = 0;
    (*jump).updateScore = FALSE as u32;
    (*jump).isLeader = (GetMultiplayerId() == 0) as u8;
    (*jump).mainState = 0;
    (*jump).helperState = 0;
    (*jump).excellentsInRow = 0;
    (*jump).excellentsInRowRecord = 0;
    (*jump).initScoreUpdate = FALSE as u32;
    (*jump).unused2 = 0;
    (*jump).unused3 = 0;
    (*jump).numPlayersAtPeak = 0;
    (*jump).allowVineUpdates = FALSE;
    (*jump).allPlayersReady = FALSE;
    (*jump).funcActive = TRUE;
    (*jump).comm.jumpScore = 0;
    (*jump).comm.receivedBonusFlags = 0;
    (*jump).comm.jumpsInRow = 0;
    (*jump).unused4 = TRUE as u32;
    (*jump).showBonus = FALSE;
    (*jump).skipJumpUpdate = FALSE as u32;
    (*jump).giveBonus = FALSE as u32;
    (*jump).linkTimer = 0;
    (*jump).linkTimerLimit = 0;
    ResetPlayersForNewGame();
    ResetPlayersJumpStates();
    i = 0;
    while i < MAX_RFU_PLAYERS {
        (*jump).atJumpPeak[i] = FALSE;
        (*jump).jumpTimeStarts[i] = 0;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn InitPlayerAndJumpTypes() {
    let mut i: i32 = 0;
    let mut index: i32 = 0;
    i = 0;
    while i < MAX_RFU_PLAYERS {
        index = GetPokemonJumpSpeciesIdx((*sPokemonJump).monInfo[i].species) as i32;
        (*sPokemonJump).players[i].monJumpType = sPokeJumpMons[index].jumpType;
        i += 1;
    }
    (*sPokemonJump).player = &raw mut (*sPokemonJump).players[(*sPokemonJump).multiplayerId];
}
pub(crate) unsafe extern "C" fn ResetPlayersForNewGame() {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_RFU_PLAYERS {
        (*sPokemonJump).players[i].jumpTimeStart = 0;
        (*sPokemonJump).players[i].monState = MONSTATE_NORMAL;
        (*sPokemonJump).players[i].prevMonState = MONSTATE_NORMAL;
        (*sPokemonJump).players[i].jumpOffset = 0;
        (*sPokemonJump).players[i].jumpOffsetIdx = 2147483647;
        (*sPokemonJump).players[i].jumpState = JUMPSTATE_NONE;
        (*sPokemonJump).memberFuncIds[i] = FUNC_NONE;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetPokemonJumpSpeciesIdx(species: u16) -> i16 {
    let mut i: u32 = 0;
    i = 0;
    while i < 100 {
        if sPokeJumpMons[i].species == species {
            return i as i16;
        }
        i += 1;
    }
    return -1;
}
pub(crate) unsafe extern "C" fn InitJumpMonInfo(
    monInfo: *mut PokemonJump_MonInfo,
    mon: *mut Pokemon,
) {
    (*monInfo).species = GetMonData2(mon, MON_DATA_SPECIES) as u16;
    (*monInfo).otId = GetMonData2(mon, MON_DATA_OT_ID);
    (*monInfo).personality = GetMonData2(mon, MON_DATA_PERSONALITY);
}
pub(crate) unsafe extern "C" fn VBlankCB_PokemonJump() {
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
}
pub(crate) unsafe extern "C" fn CB2_PokemonJump() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn SetPokeJumpTask(func: Option<unsafe extern "C" fn(u8)>) {
    (*sPokemonJump).taskId = CreateTask(func, 1);
    (*sPokemonJump).mainState = 0;
}
pub(crate) unsafe extern "C" fn Task_StartPokemonJump(taskId: u8) {
    match (*sPokemonJump).mainState {
        0 => {
            SetVBlankCallback(None);
            ResetSpriteData();
            FreeAllSpritePalettes();
            SetTaskWithPokeJumpStruct(Some(Task_CommunicateMonInfo), 5);
            FadeOutMapMusic(4);
            (*sPokemonJump).mainState += 1;
        }
        1 => {
            if FuncIsActiveTask(Some(Task_CommunicateMonInfo)) == 0 {
                StartPokeJumpGfx(&raw mut (*sPokemonJump).jumpGfx);
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(0, 0);
                (*sPokemonJump).mainState += 1;
            }
        }
        2 => {
            if IsPokeJumpGfxFuncFinished() == 0 && IsNotWaitingForBGMStop() == TRUE {
                FadeOutAndPlayNewMapMusic(MUS_RG_POKE_JUMP, 8);
                (*sPokemonJump).mainState += 1;
            }
        }
        3 => {
            if IsLinkTaskFinished() != 0 {
                BlendPalettes(PALETTES_ALL, 16, 0);
                BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
                SetVBlankCallback(Some(VBlankCB_PokemonJump));
                (*sPokemonJump).mainState += 1;
            }
        }
        4 => {
            UpdatePaletteFade();
            if gPaletteFade.active() == 0 {
                (*sPokemonJump).startDelayTimer = 0;
                (*sPokemonJump).mainState += 1;
            }
        }
        5 => {
            (*sPokemonJump).startDelayTimer += 1;
            if (*sPokemonJump).startDelayTimer >= 20 {
                if (*sPokemonJump).isLeader != 0 {
                    SetPokeJumpTask(Some(Task_PokemonJump_Leader));
                } else {
                    SetPokeJumpTask(Some(Task_PokemonJump_Member));
                }
                InitVineState();
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetLinkTimeInterval(intervalId: i32) {
    if intervalId == LINK_INTERVAL_NONE {
        (*sPokemonJump).linkTimerLimit = LINK_TIMER_STOPPED;
        (*sPokemonJump).linkTimer = 1;
    } else {
        (*sPokemonJump).linkTimerLimit = shl_i32(1, intervalId as u32 - 1) as u32 - 1;
        (*sPokemonJump).linkTimer = 0;
    }
}
pub(crate) unsafe extern "C" fn SetFunc_Leader(funcId: u8) {
    let mut i: i32 = 0;
    (*sPokemonJump).comm.funcId = funcId;
    (*sPokemonJump).mainState = 0;
    (*sPokemonJump).helperState = 0;
    (*sPokemonJump).funcActive = TRUE;
    (*sPokemonJump).allPlayersReady = FALSE;
    i = 1;
    while i < (*sPokemonJump).numPlayers as i32 {
        (*sPokemonJump).players[i].funcFinished = FALSE as u32;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn RecvLinkData_Leader() {
    let mut i: i32 = 0;
    let mut numReady: i32 = 0;
    let mut monState: u16 = 0;
    let mut funcId: u8 = 0;
    let mut playAgainState: u16 = 0;
    i = 1;
    numReady = 0;
    while i < (*sPokemonJump).numPlayers as i32 {
        monState = (*sPokemonJump).players[i].monState;
        if RecvPacket_MemberStateToLeader(
            &raw mut (*sPokemonJump).players[i],
            i,
            &raw mut funcId,
            &raw mut playAgainState,
        ) != 0
        {
            (*sPokemonJump).playAgainStates[i] = playAgainState;
            (*sPokemonJump).memberFuncIds[i] = funcId;
            (*sPokemonJump).players[i].prevMonState = monState;
        }
        if (*sPokemonJump).players[i].funcFinished != 0
            && (*sPokemonJump).memberFuncIds[i] == (*sPokemonJump).comm.funcId
        {
            numReady += 1;
        }
        i += 1;
    }
    if numReady == (*sPokemonJump).numPlayers as i32 - 1 {
        (*sPokemonJump).allPlayersReady = TRUE;
    }
}
pub(crate) unsafe extern "C" fn Task_PokemonJump_Leader(taskId: u8) {
    RecvLinkData_Leader();
    TryUpdateScore();
    if (*sPokemonJump).funcActive == 0 && (*sPokemonJump).allPlayersReady != 0 {
        SetFunc_Leader((*sPokemonJump).nextFuncId);
        SetLinkTimeInterval(LINK_INTERVAL_SHORT);
    }
    if (*sPokemonJump).funcActive == TRUE {
        if sPokeJumpLeaderFuncs[(*sPokemonJump).comm.funcId].unwrap_unchecked()() == 0 {
            (*sPokemonJump).funcActive = FALSE;
            (*sPokemonJump).players[(*sPokemonJump).multiplayerId].funcFinished = TRUE as u32;
        }
    }
    UpdateGame();
    SendLinkData_Leader();
}
pub(crate) unsafe extern "C" fn SendLinkData_Leader() {
    if (*sPokemonJump).linkTimer == 0 {
        SendPacket_LeaderState(
            (*sPokemonJump).players.as_mut_ptr(),
            &raw mut (*sPokemonJump).comm,
        );
    }
    if (*sPokemonJump).linkTimerLimit != LINK_TIMER_STOPPED {
        (*sPokemonJump).linkTimer += 1;
        (*sPokemonJump).linkTimer &= (*sPokemonJump).linkTimerLimit as i32;
    }
}
pub(crate) unsafe extern "C" fn SetFunc_Member(funcId: u8) {
    (*sPokemonJump).comm.funcId = funcId;
    (*sPokemonJump).mainState = 0;
    (*sPokemonJump).helperState = 0;
    (*sPokemonJump).funcActive = TRUE;
    (*sPokemonJump).players[(*sPokemonJump).multiplayerId].funcFinished = FALSE as u32;
}
pub(crate) unsafe extern "C" fn RecvLinkData_Member() {
    let mut i: i32 = 0;
    let mut monState: u16 = 0;
    let mut leaderData: PokemonJump_CommData = zeroed();
    monState = (*sPokemonJump).players[0].monState;
    if RecvPacket_LeaderState((*sPokemonJump).players.as_mut_ptr(), &raw mut leaderData) != 0 {
        if (*sPokemonJump).players[(*sPokemonJump).multiplayerId].funcFinished == TRUE as u32
            && leaderData.funcId != (*sPokemonJump).comm.funcId
        {
            SetFunc_Member(leaderData.funcId);
        }
        if (*sPokemonJump).comm.jumpScore != leaderData.jumpScore {
            (*sPokemonJump).comm.jumpScore = leaderData.jumpScore;
            (*sPokemonJump).updateScore = TRUE as u32;
            (*sPokemonJump).comm.receivedBonusFlags = leaderData.receivedBonusFlags;
            if (*sPokemonJump).comm.receivedBonusFlags != 0 {
                (*sPokemonJump).showBonus = TRUE;
            } else {
                (*sPokemonJump).showBonus = FALSE;
            }
        }
        (*sPokemonJump).comm.data = leaderData.data;
        (*sPokemonJump).comm.jumpsInRow = leaderData.jumpsInRow;
        (*sPokemonJump).players[0].prevMonState = monState;
    }
    i = 1;
    while i < (*sPokemonJump).numPlayers as i32 {
        if i != (*sPokemonJump).multiplayerId as i32 {
            monState = (*sPokemonJump).players[i].monState;
            if RecvPacket_MemberStateToMember(&raw mut (*sPokemonJump).players[i], i) != 0 {
                (*sPokemonJump).players[i].prevMonState = monState;
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Task_PokemonJump_Member(taskId: u8) {
    RecvLinkData_Member();
    if (*sPokemonJump).funcActive != 0 {
        if sPokeJumpMemberFuncs[(*sPokemonJump).comm.funcId].unwrap_unchecked()() == 0 {
            (*sPokemonJump).funcActive = FALSE;
            (*sPokemonJump).players[(*sPokemonJump).multiplayerId].funcFinished = TRUE as u32;
            SetLinkTimeInterval(LINK_INTERVAL_SHORT);
        }
    }
    UpdateGame();
    SendLinkData_Member();
}
pub(crate) unsafe extern "C" fn SendLinkData_Member() {
    if (*sPokemonJump).linkTimer == 0 {
        SendPacket_MemberState(
            &raw mut (*sPokemonJump).players[(*sPokemonJump).multiplayerId],
            (*sPokemonJump).comm.funcId,
            (*sPokemonJump).playAgainComm,
        );
    }
    if (*sPokemonJump).linkTimerLimit != LINK_TIMER_STOPPED {
        (*sPokemonJump).linkTimer += 1;
        (*sPokemonJump).linkTimer &= (*sPokemonJump).linkTimerLimit as i32;
    }
}
pub(crate) unsafe extern "C" fn GameIntro_Leader() -> u32 {
    'l1: {
        let sw1: u16 = (*sPokemonJump).mainState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            SetLinkTimeInterval(LINK_INTERVAL_SHORT);
            (*sPokemonJump).mainState += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if DoGameIntro() == 0 {
                (*sPokemonJump).comm.data = (*sPokemonJump).vineTimer;
                (*sPokemonJump).nextFuncId = FUNC_WAIT_ROUND;
                return FALSE as u32;
            }
            break 'l1;
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn GameIntro_Member() -> u32 {
    'l1: {
        let sw1: u16 = (*sPokemonJump).mainState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            SetLinkTimeInterval(LINK_INTERVAL_NONE);
            (*sPokemonJump).rngSeed = (*sPokemonJump).comm.data as u32;
            (*sPokemonJump).mainState += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            return DoGameIntro();
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn WaitRound_Leader() -> u32 {
    match (*sPokemonJump).mainState {
        0 => {
            ResetPlayersJumpStates();
            SetLinkTimeInterval(LINK_INTERVAL_LONG);
            (*sPokemonJump).mainState += 1;
        }
        1 => {
            if (*sPokemonJump).allPlayersReady != 0 {
                (*sPokemonJump).nextFuncId = FUNC_GAME_ROUND;
                return FALSE as u32;
            }
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn WaitRound_Member() -> u32 {
    'l1: {
        let sw1: u16 = (*sPokemonJump).mainState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            ResetPlayersJumpStates();
            SetLinkTimeInterval(LINK_INTERVAL_NONE);
            (*sPokemonJump).vineTimer = (*sPokemonJump).comm.data;
            (*sPokemonJump).mainState += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if AreLinkQueuesEmpty() != 0 {
                return FALSE as u32;
            }
            break 'l1;
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn GameRound_Leader() -> u32 {
    if HandleSwingRound() == 0 {
        (*sPokemonJump).comm.data = (*sPokemonJump).vineTimer;
        (*sPokemonJump).nextFuncId = FUNC_WAIT_ROUND;
    } else if UpdateVineHitStates() != 0 {
        return TRUE as u32;
    } else {
        ResetVineAfterHit();
        (*sPokemonJump).nextFuncId = FUNC_GAME_OVER;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn GameRound_Member() -> u32 {
    if HandleSwingRound() == 0 {
    } else if UpdateVineHitStates() != 0 {
        return TRUE as u32;
    } else {
        ResetVineAfterHit();
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn GameOver_Leader() -> u32 {
    match (*sPokemonJump).mainState {
        0 => {
            UpdateVineHitStates();
            if AllPlayersJumpedOrHit() != 0 {
                (*sPokemonJump).mainState += 1;
            }
        }
        1 => {
            if DoVineHitEffect() == 0 {
                if HasEnoughScoreForPrize() != 0 {
                    (*sPokemonJump).comm.data = GetPrizeData();
                    (*sPokemonJump).nextFuncId = FUNC_GIVE_PRIZE;
                } else if (*sPokemonJump).comm.jumpsInRow >= 200 {
                    (*sPokemonJump).comm.data = (*sPokemonJump).excellentsInRowRecord;
                    (*sPokemonJump).nextFuncId = FUNC_SAVE;
                } else {
                    (*sPokemonJump).comm.data = (*sPokemonJump).excellentsInRowRecord;
                    (*sPokemonJump).nextFuncId = FUNC_ASK_PLAY_AGAIN;
                }
                (*sPokemonJump).mainState += 1;
                return FALSE as u32;
            }
        }
        2 => {
            return FALSE as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn GameOver_Member() -> u32 {
    match (*sPokemonJump).mainState {
        0 => {
            if UpdateVineHitStates() == 0 {
                ResetVineAfterHit();
            }
            if AllPlayersJumpedOrHit() != 0 {
                (*sPokemonJump).mainState += 1;
            }
        }
        1 => {
            if DoVineHitEffect() == 0 {
                (*sPokemonJump).mainState += 1;
                return FALSE as u32;
            }
        }
        2 => {
            return FALSE as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn AskPlayAgain_Leader() -> u32 {
    'l1: {
        let sw1: u16 = (*sPokemonJump).mainState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            SetLinkTimeInterval(LINK_INTERVAL_MEDIUM);
            (*sPokemonJump).mainState += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if DoPlayAgainPrompt() == 0 {
                TryUpdateRecords(
                    (*sPokemonJump).comm.jumpScore,
                    (*sPokemonJump).comm.jumpsInRow,
                    (*sPokemonJump).comm.data,
                );
                (*sPokemonJump).mainState += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if (*sPokemonJump).allPlayersReady != 0 {
                if ShouldPlayAgain() != 0 {
                    (*sPokemonJump).nextFuncId = FUNC_RESET_GAME;
                } else {
                    (*sPokemonJump).nextFuncId = FUNC_EXIT;
                }
                (*sPokemonJump).mainState += 1;
                return FALSE as u32;
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            return FALSE as u32;
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn AskPlayAgain_Member() -> u32 {
    'l1: {
        let sw1: u16 = (*sPokemonJump).mainState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            SetLinkTimeInterval(LINK_INTERVAL_NONE);
            (*sPokemonJump).mainState += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if DoPlayAgainPrompt() == 0 {
                TryUpdateRecords(
                    (*sPokemonJump).comm.jumpScore,
                    (*sPokemonJump).comm.jumpsInRow,
                    (*sPokemonJump).comm.data,
                );
                (*sPokemonJump).playAgainComm = (*sPokemonJump).playAgainState as u16;
                return FALSE as u32;
            }
            break 'l1;
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn ResetGame_Leader() -> u32 {
    match (*sPokemonJump).mainState {
        0 => {
            if CloseMessageAndResetScore() == 0 {
                (*sPokemonJump).mainState += 1;
            }
        }
        1 => {
            if (*sPokemonJump).allPlayersReady != 0 {
                ResetForNewGame(sPokemonJump);
                (*sPokemonJump).rngSeed = Random() as u32;
                (*sPokemonJump).comm.data = (*sPokemonJump).rngSeed as u16;
                (*sPokemonJump).nextFuncId = FUNC_GAME_INTRO;
                return FALSE as u32;
            }
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn ResetGame_Member() -> u32 {
    match (*sPokemonJump).mainState {
        0 => {
            if CloseMessageAndResetScore() == 0 {
                ResetForNewGame(sPokemonJump);
                (*sPokemonJump).mainState += 1;
                return FALSE as u32;
            }
        }
        1 => {
            return FALSE as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn ExitGame() -> u32 {
    match (*sPokemonJump).mainState {
        0 => {
            (*sPokemonJump).mainState = 1;
        }
        1 => {
            SetLinkTimeInterval(LINK_INTERVAL_NONE);
            (*sPokemonJump).mainState += 1;
        }
        2 => {
            if ClosePokeJumpLink() == 0 {
                SetMainCallback2((*sPokemonJump).exitCallback);
                FreePokemonJump();
            }
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn GivePrize_Leader() -> u32 {
    match (*sPokemonJump).mainState {
        0 => {
            SetLinkTimeInterval(LINK_INTERVAL_MEDIUM);
            (*sPokemonJump).mainState += 1;
        }
        1 => {
            if TryGivePrize() == 0 {
                (*sPokemonJump).comm.data = (*sPokemonJump).excellentsInRowRecord;
                (*sPokemonJump).nextFuncId = FUNC_SAVE;
                return FALSE as u32;
            }
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn GivePrize_Member() -> u32 {
    SetLinkTimeInterval(LINK_INTERVAL_NONE);
    if TryGivePrize() == 0 {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SavePokeJump() -> u32 {
    match (*sPokemonJump).mainState {
        0 => {
            TryUpdateRecords(
                (*sPokemonJump).comm.jumpScore,
                (*sPokemonJump).comm.jumpsInRow,
                (*sPokemonJump).comm.data,
            );
            SetUpPokeJumpGfxFuncById(GFXFUNC_MSG_SAVING);
            (*sPokemonJump).mainState += 1;
        }
        1 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                SetLinkTimeInterval(LINK_INTERVAL_NONE);
                (*sPokemonJump).mainState += 1;
            }
        }
        2 => {
            if AreLinkQueuesEmpty() != 0 {
                CreateTask(Some(Task_LinkFullSave), 6);
                (*sPokemonJump).mainState += 1;
            }
        }
        3 => {
            if FuncIsActiveTask(Some(Task_LinkFullSave)) == 0 {
                ClearMessageWindow();
                (*sPokemonJump).mainState += 1;
            }
        }
        4 => {
            if RemoveMessageWindow() == 0 {
                (*sPokemonJump).nextFuncId = FUNC_ASK_PLAY_AGAIN;
                return FALSE as u32;
            }
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn DoGameIntro() -> u32 {
    match (*sPokemonJump).helperState {
        0 => {
            SetUpPokeJumpGfxFuncById(GFXFUNC_SHOW_NAMES_HIGHLIGHT);
            ResetMonSpriteSubpriorities();
            (*sPokemonJump).helperState += 1;
        }
        1 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                StartMonIntroBounce((*sPokemonJump).multiplayerId as i32);
                (*sPokemonJump).timer = 0;
                (*sPokemonJump).helperState += 1;
            }
        }
        2 => {
            if ({
                (*sPokemonJump).timer += 1;
                (*sPokemonJump).timer
            }) > 120
            {
                SetUpPokeJumpGfxFuncById(GFXFUNC_ERASE_NAMES);
                (*sPokemonJump).helperState += 1;
            }
        }
        3 => {
            if IsPokeJumpGfxFuncFinished() != TRUE as u32 && IsMonIntroBounceActive() != TRUE as i32
            {
                (*sPokemonJump).helperState += 1;
            }
        }
        4 => {
            SetUpPokeJumpGfxFuncById(GFXFUNC_COUNTDOWN);
            (*sPokemonJump).helperState += 1;
        }
        5 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                DisallowVineUpdates();
                SetUpResetVineGfx();
                (*sPokemonJump).helperState += 1;
            }
        }
        6 => {
            if ResetVineGfx() == 0 {
                AllowVineUpdates();
                ResetVineState();
                (*sPokemonJump).helperState += 1;
                return FALSE as u32;
            }
        }
        7 => {
            return FALSE as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn HandleSwingRound() -> u32 {
    UpdateVineState();
    if (*sPokemonJump).ignoreJumpInput != 0 {
        (*sPokemonJump).ignoreJumpInput = FALSE as u16;
        return FALSE as u32;
    }
    'l1: {
        let sw1: u16 = (*sPokemonJump).helperState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if IsPlayersMonState(MONSTATE_NORMAL) != 0 {
                (*sPokemonJump).helperState += 1;
            } else {
                break 'l1;
            }
        }
        if fall || sw1 == 1 {
            fall = true;
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                SetMonStateJump();
                SetLinkTimeInterval(LINK_INTERVAL_SHORT);
                (*sPokemonJump).helperState += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if IsPlayersMonState(MONSTATE_JUMP) == TRUE as u32 {
                (*sPokemonJump).helperState += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if IsPlayersMonState(MONSTATE_NORMAL) == TRUE as u32 {
                (*sPokemonJump).helperState = 0;
            }
            break 'l1;
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn DoVineHitEffect() -> u32 {
    let mut i: i32 = 0;
    match (*sPokemonJump).helperState {
        0 => {
            i = 0;
            while i < (*sPokemonJump).numPlayers as i32 {
                if IsMonHitShakeActive(i) == TRUE as i32 {
                    return TRUE as u32;
                }
                i += 1;
            }
            (*sPokemonJump).helperState += 1;
        }
        1 => {
            i = 0;
            while i < (*sPokemonJump).numPlayers as i32 {
                if (*sPokemonJump).players[i].monState == MONSTATE_HIT {
                    StartMonHitFlash(i as u8);
                }
                i += 1;
            }
            SetUpPokeJumpGfxFuncById(GFXFUNC_SHOW_NAMES);
            (*sPokemonJump).timer = 0;
            (*sPokemonJump).helperState += 1;
        }
        2 => {
            if ({
                (*sPokemonJump).timer += 1;
                (*sPokemonJump).timer
            }) > 100
            {
                SetUpPokeJumpGfxFuncById(GFXFUNC_ERASE_NAMES);
                (*sPokemonJump).timer = 0;
                (*sPokemonJump).helperState += 1;
            }
        }
        3 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                StopMonHitFlash();
                (*sPokemonJump).comm.receivedBonusFlags = 0;
                ResetPlayersMonState();
                (*sPokemonJump).helperState += 1;
                return FALSE as u32;
            }
        }
        4 => {
            return FALSE as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn TryGivePrize() -> u32 {
    'l1: {
        match (*sPokemonJump).helperState {
            0 => {
                UnpackPrizeData(
                    (*sPokemonJump).comm.data,
                    &raw mut (*sPokemonJump).prizeItemId,
                    &raw mut (*sPokemonJump).prizeItemQuantity,
                );
                PrintPrizeMessage(
                    (*sPokemonJump).prizeItemId,
                    (*sPokemonJump).prizeItemQuantity,
                );
                (*sPokemonJump).helperState += 1;
            }
            1 | 4 => {
                if DoPrizeMessageAndFanfare() == 0 {
                    (*sPokemonJump).timer = 0;
                    (*sPokemonJump).helperState += 1;
                }
            }
            2 | 5 => {
                (*sPokemonJump).timer += 1;
                if gMain.newKeys as i32 & 3 != 0 || (*sPokemonJump).timer > 180 {
                    ClearMessageWindow();
                    (*sPokemonJump).helperState += 1;
                }
            }
            3 => {
                if RemoveMessageWindow() == 0 {
                    (*sPokemonJump).prizeItemQuantity = GetQuantityLimitedByBag(
                        (*sPokemonJump).prizeItemId,
                        (*sPokemonJump).prizeItemQuantity,
                    );
                    if (*sPokemonJump).prizeItemQuantity != 0
                        && AddBagItem(
                            (*sPokemonJump).prizeItemId,
                            (*sPokemonJump).prizeItemQuantity,
                        ) != 0
                    {
                        if CheckBagHasSpace((*sPokemonJump).prizeItemId, 1) == 0 {
                            PrintPrizeFilledBagMessage((*sPokemonJump).prizeItemId);
                            (*sPokemonJump).helperState = 4;
                        } else {
                            (*sPokemonJump).helperState = 6;
                            break 'l1;
                        }
                    } else {
                        PrintNoRoomForPrizeMessage((*sPokemonJump).prizeItemId);
                        (*sPokemonJump).helperState = 4;
                    }
                }
            }
            6 => {
                if RemoveMessageWindow() == 0 {
                    return FALSE as u32;
                }
            }
            _ => {}
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn DoPlayAgainPrompt() -> u32 {
    let mut input: i8 = 0;
    match (*sPokemonJump).helperState {
        0 => {
            SetUpPokeJumpGfxFuncById(GFXFUNC_MSG_PLAY_AGAIN);
            (*sPokemonJump).helperState += 1;
        }
        1 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                (*sPokemonJump).helperState += 1;
            }
        }
        2 => {
            input = HandlePlayAgainInput();
            match input {
                MENU_B_PRESSED | 1 => {
                    (*sPokemonJump).playAgainState = PLAY_AGAIN_NO;
                    SetUpPokeJumpGfxFuncById(GFXFUNC_ERASE_MSG);
                    (*sPokemonJump).helperState += 1;
                }
                0 => {
                    (*sPokemonJump).playAgainState = PLAY_AGAIN_YES;
                    SetUpPokeJumpGfxFuncById(GFXFUNC_ERASE_MSG);
                    (*sPokemonJump).helperState += 1;
                }
                _ => {}
            }
        }
        3 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                (*sPokemonJump).helperState += 1;
            }
        }
        4 => {
            SetUpPokeJumpGfxFuncById(GFXFUNC_MSG_COMM_STANDBY);
            (*sPokemonJump).helperState += 1;
        }
        5 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                (*sPokemonJump).helperState += 1;
                return FALSE as u32;
            }
        }
        6 => {
            return FALSE as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn ClosePokeJumpLink() -> u32 {
    match (*sPokemonJump).helperState {
        0 => {
            ClearMessageWindow();
            (*sPokemonJump).helperState += 1;
        }
        1 => {
            if RemoveMessageWindow() == 0 {
                SetUpPokeJumpGfxFuncById(GFXFUNC_MSG_PLAYER_DROPPED);
                (*sPokemonJump).helperState += 1;
            }
        }
        2 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                (*sPokemonJump).timer = 0;
                (*sPokemonJump).helperState += 1;
            }
        }
        3 => {
            if ({
                (*sPokemonJump).timer += 1;
                (*sPokemonJump).timer
            }) > 120
            {
                BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
                (*sPokemonJump).helperState += 1;
            }
        }
        4 => {
            if gPaletteFade.active() == 0 {
                SetCloseLinkCallback();
                (*sPokemonJump).helperState += 1;
            }
        }
        5 => {
            if gReceivedRemoteLinkPlayers == 0 {
                return FALSE as u32;
            }
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn CloseMessageAndResetScore() -> u32 {
    match (*sPokemonJump).helperState {
        0 => {
            ClearMessageWindow();
            PrintScore(0);
            (*sPokemonJump).helperState += 1;
        }
        1 => {
            if RemoveMessageWindow() == 0 {
                (*sPokemonJump).helperState += 1;
                return FALSE as u32;
            }
        }
        2 => {
            return FALSE as u32;
        }
        _ => {}
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Task_CommunicateMonInfo(taskId: u8) {
    let mut i: i32 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut jump: *mut PokemonJump =
        GetWordTaskArg(taskId, DATAIDX_GAME_STRUCT) as usize as *mut PokemonJump;
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            i = 0;
            while i < MAX_RFU_PLAYERS {
                *data.at(i + 2) = FALSE as i16;
                i += 1;
            }
            *data += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            SendPacket_MonInfo(&raw mut (*jump).monInfo[(*jump).multiplayerId]);
            i = 0;
            while i < MAX_RFU_PLAYERS {
                if *data.at(i + 2) == 0 && RecvPacket_MonInfo(i, &raw mut (*jump).monInfo[i]) != 0 {
                    StringCopy(
                        (*jump).players[i].name.as_mut_ptr(),
                        gLinkPlayers[i].name.as_mut_ptr(),
                    );
                    *data.at(i + 2) = TRUE as i16;
                    *data.at(1) += 1;
                    if *data.at(1) == (*jump).numPlayers as i16 {
                        InitPlayerAndJumpTypes();
                        DestroyTask(taskId);
                        break;
                    }
                }
                i += 1;
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn SetTaskWithPokeJumpStruct(
    func: Option<unsafe extern "C" fn(u8)>,
    taskPriority: u8,
) {
    let mut taskId: u8 = CreateTask(func, taskPriority);
    SetWordTaskArg(taskId, DATAIDX_GAME_STRUCT, sPokemonJump as usize as u32);
}
pub(crate) unsafe extern "C" fn InitVineState() {
    (*sPokemonJump).vineTimer = 0;
    (*sPokemonJump).vineState = VINE_UPSWING_LOWER;
    (*sPokemonJump).vineStateTimer = 0;
    (*sPokemonJump).vineSpeed = 0;
    (*sPokemonJump).ignoreJumpInput = FALSE as u16;
    (*sPokemonJump).gameOver = FALSE as u32;
}
pub(crate) unsafe extern "C" fn ResetVineState() {
    (*sPokemonJump).vineTimer = 0;
    (*sPokemonJump).vineStateTimer = 1791;
    (*sPokemonJump).vineState = VINE_UPSWING_LOW;
    (*sPokemonJump).ignoreJumpInput = FALSE as u16;
    (*sPokemonJump).gameOver = FALSE as u32;
    (*sPokemonJump).vineSpeedStage = 0;
    (*sPokemonJump).vineBaseSpeedIdx = 0;
    (*sPokemonJump).vineSpeedAccel = 0;
    (*sPokemonJump).vineSpeedDelay = 0;
    (*sPokemonJump).atMaxSpeedStage = FALSE as u32;
    UpdateVineSpeed();
}
pub(crate) unsafe extern "C" fn UpdateVineState() {
    if (*sPokemonJump).allowVineUpdates != 0 {
        (*sPokemonJump).vineTimer += 1;
        (*sPokemonJump).vineStateTimer += GetVineSpeed() as u16;
        if (*sPokemonJump).vineStateTimer >= 2559 {
            (*sPokemonJump).vineStateTimer -= 2559;
        }
        (*sPokemonJump).prevVineState = (*sPokemonJump).vineState;
        (*sPokemonJump).vineState = ((*sPokemonJump).vineStateTimer >> 8) as u32;
        if (*sPokemonJump).vineState > VINE_UPSWING_LOWER
            && (*sPokemonJump).prevVineState < VINE_UPSWING_LOW
        {
            (*sPokemonJump).ignoreJumpInput += 1;
            UpdateVineSpeed();
        }
    }
}
pub(crate) unsafe extern "C" fn GetVineSpeed() -> i32 {
    let mut speed: i32 = 0;
    if (*sPokemonJump).gameOver != 0 {
        return 0;
    }
    speed = (*sPokemonJump).vineSpeed;
    if (*sPokemonJump).vineStateTimer <= 1535 {
        (*sPokemonJump).vineSpeedAccel += 80;
        speed += ((*sPokemonJump).vineSpeedAccel / 256) as i32;
    }
    return speed;
}
pub(crate) unsafe extern "C" fn UpdateVineSpeed() {
    let mut baseSpeed: i32 = 0;
    (*sPokemonJump).vineSpeedAccel = 0;
    if (*sPokemonJump).vineSpeedDelay != 0 {
        (*sPokemonJump).vineSpeedDelay -= 1;
        if (*sPokemonJump).atMaxSpeedStage != 0 {
            if PokeJumpRandom() % 4 != 0 {
                (*sPokemonJump).vineSpeed = (*sPokemonJump).nextVineSpeed as i32;
            } else {
                if (*sPokemonJump).nextVineSpeed > 54 {
                    (*sPokemonJump).vineSpeed = 30;
                } else {
                    (*sPokemonJump).vineSpeed = 82;
                }
            }
        }
    } else {
        if (*sPokemonJump).vineBaseSpeedIdx as u32 & 8 == 0 {
            (*sPokemonJump).nextVineSpeed = sVineBaseSpeeds[(*sPokemonJump).vineBaseSpeedIdx]
                as u32
                + (*sPokemonJump).vineSpeedStage as u32 * 7;
            (*sPokemonJump).vineSpeedDelay = sVineSpeedDelays[PokeJumpRandom() % 4] + 2;
            (*sPokemonJump).vineBaseSpeedIdx += 1;
        } else {
            if (*sPokemonJump).vineBaseSpeedIdx == 8 {
                if (*sPokemonJump).vineSpeedStage < 3 {
                    (*sPokemonJump).vineSpeedStage += 1;
                } else {
                    (*sPokemonJump).atMaxSpeedStage = TRUE as u32;
                }
            }
            baseSpeed = sVineBaseSpeeds[15 - (*sPokemonJump).vineBaseSpeedIdx as i32] as i32;
            (*sPokemonJump).nextVineSpeed =
                baseSpeed as u32 + (*sPokemonJump).vineSpeedStage as u32 * 7;
            if ({
                (*sPokemonJump).vineBaseSpeedIdx += 1;
                (*sPokemonJump).vineBaseSpeedIdx
            }) > 15
            {
                if PokeJumpRandom() % 4 == 0 {
                    (*sPokemonJump).nextVineSpeed -= 5;
                }
                (*sPokemonJump).vineBaseSpeedIdx = 0;
            }
        }
        (*sPokemonJump).vineSpeed = (*sPokemonJump).nextVineSpeed as i32;
    }
}
pub(crate) unsafe extern "C" fn PokeJumpRandom() -> i32 {
    (*sPokemonJump).rngSeed = 0x41c64e6d * (*sPokemonJump).rngSeed + 24691;
    return ((*sPokemonJump).rngSeed >> 16) as i32;
}
pub(crate) unsafe extern "C" fn ResetVineAfterHit() {
    (*sPokemonJump).gameOver = TRUE as u32;
    (*sPokemonJump).vineState = VINE_UPSWING_LOWER;
    (*sPokemonJump).vineStateTimer = 1535;
    AllowVineUpdates();
}
pub(crate) unsafe extern "C" fn IsGameOver() -> i32 {
    return (*sPokemonJump).gameOver as i32;
}
pub(crate) unsafe extern "C" fn ResetPlayersJumpStates() {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_RFU_PLAYERS {
        (*sPokemonJump).players[i].jumpState = JUMPSTATE_NONE;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ResetPlayersMonState() {
    (*(*sPokemonJump).player).monState = MONSTATE_NORMAL;
    (*(*sPokemonJump).player).prevMonState = MONSTATE_NORMAL;
}
pub(crate) unsafe extern "C" fn IsPlayersMonState(monState: u16) -> u32 {
    if (*sPokemonJump).players[(*sPokemonJump).multiplayerId].monState == monState {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetMonStateJump() {
    (*(*sPokemonJump).player).jumpTimeStart = (*sPokemonJump).vineTimer;
    (*(*sPokemonJump).player).prevMonState = (*(*sPokemonJump).player).monState;
    (*(*sPokemonJump).player).monState = MONSTATE_JUMP;
}
pub(crate) unsafe extern "C" fn SetMonStateHit() {
    (*(*sPokemonJump).player).prevMonState = (*(*sPokemonJump).player).monState;
    (*(*sPokemonJump).player).monState = MONSTATE_HIT;
    (*(*sPokemonJump).player).jumpTimeStart = (*sPokemonJump).vineTimer;
    (*(*sPokemonJump).player).jumpState = JUMPSTATE_FAILURE;
}
pub(crate) unsafe extern "C" fn SetMonStateNormal() {
    (*(*sPokemonJump).player).prevMonState = (*(*sPokemonJump).player).monState;
    (*(*sPokemonJump).player).monState = MONSTATE_NORMAL;
}
pub(crate) unsafe extern "C" fn UpdateGame() {
    if (*sPokemonJump).updateScore != 0 {
        PrintScore((*sPokemonJump).comm.jumpScore as i32);
        (*sPokemonJump).updateScore = FALSE as u32;
        if (*sPokemonJump).showBonus != 0 {
            let mut numPlayers: i32 = DoSameJumpTimeBonus((*sPokemonJump).comm.receivedBonusFlags);
            PlaySE(sSoundEffects[numPlayers - 2]);
            (*sPokemonJump).showBonus = FALSE;
        }
    }
    PrintJumpsInRow((*sPokemonJump).comm.jumpsInRow);
    HandleMonState();
    TryUpdateVineSwing();
}
pub(crate) unsafe extern "C" fn TryUpdateVineSwing() {
    if (*sPokemonJump).allowVineUpdates != 0 {
        UpdateVineSwing((*sPokemonJump).vineState as i32);
    }
}
pub(crate) unsafe extern "C" fn DisallowVineUpdates() {
    (*sPokemonJump).allowVineUpdates = FALSE;
}
pub(crate) unsafe extern "C" fn AllowVineUpdates() {
    (*sPokemonJump).allowVineUpdates = TRUE;
}
pub(crate) unsafe extern "C" fn HandleMonState() {
    let mut i: i32 = 0;
    let mut soundFlags: i32 = 0;
    let mut numPlayers: i32 = (*sPokemonJump).numPlayers as i32;
    i = 0;
    while i < numPlayers {
        match (*sPokemonJump).players[i].monState {
            MONSTATE_NORMAL => {
                SetMonSpriteY(i as u32, 0);
            }
            MONSTATE_JUMP => {
                if (*sPokemonJump).players[i].prevMonState != MONSTATE_JUMP
                    || (*sPokemonJump).players[i].jumpTimeStart != (*sPokemonJump).jumpTimeStarts[i]
                {
                    if i == (*sPokemonJump).multiplayerId as i32 {
                        (*sPokemonJump).players[i].prevMonState = MONSTATE_JUMP;
                    }
                    soundFlags |= F_SE_JUMP;
                    (*sPokemonJump).players[i].jumpOffsetIdx = 2147483647;
                    (*sPokemonJump).jumpTimeStarts[i] = (*sPokemonJump).players[i].jumpTimeStart;
                }
                UpdateJump(i);
            }
            MONSTATE_HIT => {
                if (*sPokemonJump).players[i].prevMonState != MONSTATE_HIT {
                    if i == (*sPokemonJump).multiplayerId as i32 {
                        (*sPokemonJump).players[i].prevMonState = MONSTATE_HIT;
                    }
                    soundFlags |= F_SE_FAIL;
                    StartMonHitShake(i as u8);
                }
            }
            _ => {}
        }
        i += 1;
    }
    if soundFlags & F_SE_FAIL != 0 {
        PlaySE(SE_RG_POKE_JUMP_FAILURE);
    } else if soundFlags & F_SE_JUMP != 0 {
        PlaySE(SE_LEDGE);
    }
}
pub(crate) unsafe extern "C" fn UpdateJump(multiplayerId: i32) {
    let mut jumpOffsetIdx: i32 = 0;
    let mut jumpOffset: i32 = 0;
    let mut player: *mut PokemonJump_Player = null_mut();
    if (*sPokemonJump).skipJumpUpdate != 0 {
        return;
    }
    player = &raw mut (*sPokemonJump).players[multiplayerId];
    if (*player).jumpOffsetIdx != 2147483647 {
        (*player).jumpOffsetIdx += 1;
        jumpOffsetIdx = (*player).jumpOffsetIdx;
    } else {
        jumpOffsetIdx = (*sPokemonJump).vineTimer as i32 - (*player).jumpTimeStart as i32;
        if jumpOffsetIdx >= 65000 {
            jumpOffsetIdx -= 65000;
            jumpOffsetIdx += (*sPokemonJump).vineTimer as i32;
        }
        (*player).jumpOffsetIdx = jumpOffsetIdx;
    }
    if jumpOffsetIdx < 4 {
        return;
    }
    jumpOffsetIdx -= 4;
    if jumpOffsetIdx < 48 {
        jumpOffset = sJumpOffsets[(*player).monJumpType][jumpOffsetIdx] as i32;
    } else {
        jumpOffset = 0;
    }
    SetMonSpriteY(multiplayerId as u32, jumpOffset as i16);
    if jumpOffset == 0 && multiplayerId == (*sPokemonJump).multiplayerId as i32 {
        SetMonStateNormal();
    }
    (*player).jumpOffset = jumpOffset;
}
pub(crate) unsafe extern "C" fn TryUpdateScore() {
    if (*sPokemonJump).vineState == VINE_UPSWING_HIGH
        && (*sPokemonJump).prevVineState == VINE_UPSWING_LOW
    {
        if (*sPokemonJump).initScoreUpdate == 0 {
            ClearUnreadField();
            (*sPokemonJump).numPlayersAtPeak = 0;
            (*sPokemonJump).initScoreUpdate = TRUE as u32;
            (*sPokemonJump).comm.receivedBonusFlags = 0;
        } else {
            if (*sPokemonJump).numPlayersAtPeak == MAX_RFU_PLAYERS {
                (*sPokemonJump).excellentsInRow += 1;
                TryUpdateExcellentsRecord((*sPokemonJump).excellentsInRow);
            } else {
                (*sPokemonJump).excellentsInRow = 0;
            }
            if (*sPokemonJump).numPlayersAtPeak > 1 {
                (*sPokemonJump).giveBonus = TRUE as u32;
                memcpy(
                    (*sPokemonJump).atJumpPeak3.as_mut_ptr(),
                    (*sPokemonJump).atJumpPeak2.as_mut_ptr(),
                    MAX_RFU_PLAYERS as u32,
                );
            }
            ClearUnreadField();
            (*sPokemonJump).numPlayersAtPeak = 0;
            (*sPokemonJump).initScoreUpdate = TRUE as u32;
            (*sPokemonJump).comm.receivedBonusFlags = 0;
            if (*sPokemonJump).comm.jumpsInRow < MAX_JUMPS {
                (*sPokemonJump).comm.jumpsInRow += 1;
            }
            AddJumpScore(10);
            SetLinkTimeInterval(LINK_INTERVAL_SHORT);
        }
    }
    if (*sPokemonJump).giveBonus != 0
        && (DidAllPlayersClearVine() == TRUE as u32 || (*sPokemonJump).vineState == VINE_HIGHEST)
    {
        let mut numPlayers: i32 = GetNumPlayersForBonus((*sPokemonJump).atJumpPeak3.as_mut_ptr());
        AddJumpScore(GetScoreBonus(numPlayers));
        SetLinkTimeInterval(LINK_INTERVAL_SHORT);
        (*sPokemonJump).giveBonus = FALSE as u32;
    }
    if (*sPokemonJump).initScoreUpdate != 0 {
        let mut numAtPeak: i32 = GetPlayersAtJumpPeak();
        if numAtPeak > (*sPokemonJump).numPlayersAtPeak {
            (*sPokemonJump).numPlayersAtPeak = numAtPeak;
            memcpy(
                (*sPokemonJump).atJumpPeak2.as_mut_ptr(),
                (*sPokemonJump).atJumpPeak.as_mut_ptr(),
                MAX_RFU_PLAYERS as u32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateVineHitStates() -> u32 {
    let mut i: i32 = 0;
    if (*sPokemonJump).vineState == VINE_UPSWING_LOWER && (*(*sPokemonJump).player).jumpOffset == 0
    {
        if (*(*sPokemonJump).player).prevMonState == MONSTATE_JUMP && IsGameOver() == TRUE as i32 {
            (*(*sPokemonJump).player).jumpState = JUMPSTATE_SUCCESS;
        } else {
            SetMonStateHit();
            SetLinkTimeInterval(LINK_INTERVAL_SHORT);
        }
    }
    if (*sPokemonJump).vineState == VINE_UPSWING_LOW
        && (*sPokemonJump).prevVineState == VINE_UPSWING_LOWER
        && (*(*sPokemonJump).player).monState != MONSTATE_HIT
    {
        (*(*sPokemonJump).player).jumpState = JUMPSTATE_SUCCESS;
        SetLinkTimeInterval(LINK_INTERVAL_SHORT);
    }
    i = 0;
    while i < (*sPokemonJump).numPlayers as i32 {
        if (*sPokemonJump).players[i].monState == MONSTATE_HIT {
            return FALSE as u32;
        }
        i += 1;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn AllPlayersJumpedOrHit() -> u32 {
    let mut i: i32 = 0;
    let mut numPlayers: i32 = (*sPokemonJump).numPlayers as i32;
    let mut numJumpedOrHit: i32 = 0;
    i = 0;
    while i < numPlayers {
        if (*sPokemonJump).players[i].jumpState != JUMPSTATE_NONE {
            numJumpedOrHit += 1;
        }
        i += 1;
    }
    return (numJumpedOrHit == numPlayers) as u32;
}
pub(crate) unsafe extern "C" fn DidAllPlayersClearVine() -> u32 {
    let mut i: i32 = 0;
    i = 0;
    while i < (*sPokemonJump).numPlayers as i32 {
        if (*sPokemonJump).players[i].jumpState != JUMPSTATE_SUCCESS {
            return FALSE as u32;
        }
        i += 1;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn ShouldPlayAgain() -> u32 {
    let mut i: i32 = 0;
    if (*sPokemonJump).playAgainState == PLAY_AGAIN_NO {
        return FALSE as u32;
    }
    i = 1;
    while i < (*sPokemonJump).numPlayers as i32 {
        if (*sPokemonJump).playAgainStates[i] == PLAY_AGAIN_NO as u16 {
            return FALSE as u32;
        }
        i += 1;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn AddJumpScore(score: i32) {
    (*sPokemonJump).comm.jumpScore += score as u32;
    (*sPokemonJump).updateScore = TRUE as u32;
    if (*sPokemonJump).comm.jumpScore >= MAX_JUMP_SCORE {
        (*sPokemonJump).comm.jumpScore = MAX_JUMP_SCORE;
    }
}
pub(crate) unsafe extern "C" fn GetPlayersAtJumpPeak() -> i32 {
    let mut i: i32 = 0;
    let mut numAtPeak: i32 = 0;
    let mut numPlayers: i32 = (*sPokemonJump).numPlayers as i32;
    i = 0;
    while i < numPlayers {
        if (*sPokemonJump).players[i].jumpOffset == JUMP_PEAK {
            (*sPokemonJump).atJumpPeak[i] = TRUE;
            numAtPeak += 1;
        } else {
            (*sPokemonJump).atJumpPeak[i] = FALSE;
        }
        i += 1;
    }
    return numAtPeak;
}
pub(crate) unsafe extern "C" fn AreLinkQueuesEmpty() -> u32 {
    return ((&raw mut gRfu.recvQueue.count).read_volatile() == 0
        && (&raw mut gRfu.sendQueue.count).read_volatile() == 0) as u32;
}
pub(crate) unsafe extern "C" fn GetNumPlayersForBonus(atJumpPeak: *mut u8) -> i32 {
    let mut i: i32 = 0;
    let mut flags: i32 = 0;
    let mut count: i32 = 0;
    while i < MAX_RFU_PLAYERS {
        if *atJumpPeak.at(i) != 0 {
            flags |= shl_i32(1, i as u32);
            count += 1;
        }
        i += 1;
    }
    (*sPokemonJump).comm.receivedBonusFlags = flags as u8;
    if flags != 0 {
        (*sPokemonJump).showBonus = TRUE;
    }
    return count;
}
pub(crate) unsafe extern "C" fn ClearUnreadField() {
    (*sPokemonJump).unused3 = 0;
}
pub(crate) unsafe extern "C" fn GetScoreBonus(numPlayers: i32) -> i32 {
    return sScoreBonuses[numPlayers];
}
pub(crate) unsafe extern "C" fn TryUpdateExcellentsRecord(excellentsInRow: u16) {
    if excellentsInRow > (*sPokemonJump).excellentsInRowRecord {
        (*sPokemonJump).excellentsInRowRecord = excellentsInRow;
    }
}
pub(crate) unsafe extern "C" fn HasEnoughScoreForPrize() -> u32 {
    if (*sPokemonJump).comm.jumpScore >= sPrizeQuantityData[0].score {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetPrizeData() -> u16 {
    let mut itemId: u16 = GetPrizeItemId();
    let mut quantity: u16 = GetPrizeQuantity();
    return quantity << 12 | itemId & 0xFFF;
}
pub(crate) unsafe extern "C" fn UnpackPrizeData(data: u16, itemId: *mut u16, quantity: *mut u16) {
    *quantity = data >> 12;
    *itemId = data & 0xFFF;
}
pub(crate) unsafe extern "C" fn GetPrizeItemId() -> u16 {
    let mut index: u16 = Random() % 8;
    return sPrizeItems[index];
}
pub(crate) unsafe extern "C" fn GetPrizeQuantity() -> u16 {
    let mut quantity: u32 = 0;
    let mut i: u32 = 0;
    quantity = 0;
    i = 0;
    while i < 5 {
        if (*sPokemonJump).comm.jumpScore >= sPrizeQuantityData[i].score {
            quantity = sPrizeQuantityData[i].quantity;
        } else {
            break;
        }
        i += 1;
    }
    return quantity as u16;
}
pub(crate) unsafe extern "C" fn GetQuantityLimitedByBag(item: u16, mut quantity: u16) -> u16 {
    while quantity != 0 && CheckBagHasSpace(item, quantity) == 0 {
        quantity -= 1;
    }
    return quantity;
}
pub(crate) unsafe extern "C" fn GetNumPokeJumpPlayers() -> u16 {
    return GetLinkPlayerCount() as u16;
}
pub(crate) unsafe extern "C" fn GetPokeJumpMultiplayerId() -> u16 {
    return (*sPokemonJump).multiplayerId as u16;
}
pub(crate) unsafe extern "C" fn GetMonInfoByMultiplayerId(
    multiplayerId: u8,
) -> *mut PokemonJump_MonInfo {
    return &raw mut (*sPokemonJump).monInfo[multiplayerId];
}
pub(crate) unsafe extern "C" fn GetPokeJumpPlayerName(multiplayerId: u8) -> *mut u8 {
    return (*sPokemonJump).players[multiplayerId].name.as_mut_ptr();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSpeciesAllowedInPokemonJump(species: u16) -> u32 {
    return (GetPokemonJumpSpeciesIdx(species) > -1) as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPokemonJumpSpeciesInParty() {
    let mut i: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SANITY_HAS_SPECIES) != 0 {
            let mut species: u16 =
                GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) as u16;
            if IsSpeciesAllowedInPokemonJump(species) != 0 {
                gSpecialVar_Result = TRUE as u16;
                return;
            }
        }
        i += 1;
    }
    gSpecialVar_Result = FALSE as u16;
}
pub(crate) unsafe extern "C" fn LoadSpriteSheetsAndPalettes(jumpGfx: *mut PokemonJumpGfx) {
    let mut i: i32 = 0;
    i = 0;
    while i < 5 {
        LoadCompressedSpriteSheet((&raw const sCompressedSpriteSheets[i]).cast_mut());
        i += 1;
    }
    i = 0;
    while i < 2 {
        LoadSpritePalette((&raw const sSpritePalettes[i]).cast_mut());
        i += 1;
    }
    (*jumpGfx).vinePalNumDownswing = IndexOfSpritePaletteTag(PALTAG_1);
    (*jumpGfx).vinePalNumUpswing = IndexOfSpritePaletteTag(PALTAG_2);
}
pub(crate) unsafe extern "C" fn ResetPokeJumpSpriteData(sprite: *mut Sprite) {
    let mut i: i32 = 0;
    i = 0;
    while i < 8 {
        (*sprite).data[i] = 0;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateJumpMonSprite(
    jumpGfx: *mut PokemonJumpGfx,
    monInfo: *mut PokemonJump_MonInfo,
    x: i16,
    y: i16,
    multiplayerId: u8,
) {
    let mut spriteTemplate: SpriteTemplate = zeroed();
    let mut spriteSheet: SpriteSheet = zeroed();
    let mut spritePalette: CompressedSpritePalette = zeroed();
    let mut buffer: *mut u8 = null_mut();
    let mut unusedBuffer: *mut u8 = null_mut();
    let mut subpriority: u8 = 0;
    let mut spriteId: u8 = 0;
    spriteTemplate = *sSpriteTemplate_JumpMon;
    buffer = Alloc(8192) as *mut u8;
    unusedBuffer = Alloc(MON_PIC_SIZE as u32) as *mut u8;
    if multiplayerId as u16 == GetPokeJumpMultiplayerId() {
        subpriority = 3;
    } else {
        subpriority = multiplayerId + 4;
    }
    if !buffer.is_null() && !unusedBuffer.is_null() {
        HandleLoadSpecialPokePic(
            (&raw const gMonStillFrontPicTable[(*monInfo).species]).cast_mut(),
            buffer as *mut c_void,
            (*monInfo).species as i32,
            (*monInfo).personality,
        );
        spriteSheet.data = buffer as *mut c_void;
        spriteSheet.tag = multiplayerId as u16;
        spriteSheet.size = MON_PIC_SIZE;
        LoadSpriteSheet(&raw mut spriteSheet);
        spritePalette.data = GetMonSpritePalFromSpeciesAndPersonality(
            (*monInfo).species,
            (*monInfo).otId,
            (*monInfo).personality,
        );
        spritePalette.tag = multiplayerId as u16;
        LoadCompressedSpritePalette(&raw mut spritePalette);
        Free(buffer as *mut c_void);
        Free(unusedBuffer as *mut c_void);
        spriteTemplate.tileTag += multiplayerId as u16;
        spriteTemplate.paletteTag += multiplayerId as u16;
        spriteId = CreateSprite(&raw mut spriteTemplate, x, y, subpriority);
        if spriteId != MAX_SPRITES {
            (*jumpGfx).monSprites[multiplayerId] = &raw mut gSprites[spriteId];
            (*jumpGfx).monSpriteSubpriorities[multiplayerId] = subpriority;
            return;
        }
    }
    (*jumpGfx).monSprites[multiplayerId] = null_mut();
}
pub(crate) unsafe extern "C" fn DoStarAnim(jumpGfx: *mut PokemonJumpGfx, multiplayerId: i32) {
    ResetPokeJumpSpriteData((*jumpGfx).starSprites[multiplayerId]);
    (*(*jumpGfx).starSprites[multiplayerId]).data[7] =
        (((*jumpGfx).monSprites[multiplayerId] as usize)
            .wrapping_sub(gSprites.as_mut_ptr() as usize) as i32
            / 68) as i16;
    (*(*jumpGfx).starSprites[multiplayerId]).set_invisible(FALSE as u16);
    (*(*jumpGfx).starSprites[multiplayerId]).y = 96;
    (*(*jumpGfx).starSprites[multiplayerId]).callback = Some(SpriteCB_Star);
    StartSpriteAnim((*jumpGfx).starSprites[multiplayerId], 1);
}
pub(crate) unsafe extern "C" fn SpriteCB_Star(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if (*sprite).animEnded() != 0 {
                (*sprite).set_invisible(TRUE as u16);
                (*sprite).callback = Some(SpriteCallbackDummy);
            }
        }
        1 => {
            (*sprite).y -= 1;
            (*sprite).data[1] += 1;
            if (*sprite).y <= 72 {
                (*sprite).y = 72;
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) >= 48
            {
                (*sprite).set_invisible(TRUE as u16);
                (*sprite).callback = Some(SpriteCallbackDummy);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Gfx_StartMonHitShake(
    jumpGfx: *mut PokemonJumpGfx,
    multiplayerId: i32,
) {
    (*(*jumpGfx).monSprites[multiplayerId]).callback = Some(SpriteCB_MonHitShake);
    (*(*jumpGfx).monSprites[multiplayerId]).y2 = 0;
    ResetPokeJumpSpriteData((*jumpGfx).monSprites[multiplayerId]);
}
pub(crate) unsafe extern "C" fn Gfx_IsMonHitShakeActive(
    jumpGfx: *mut PokemonJumpGfx,
    multiplayerId: i32,
) -> u32 {
    return ((*(*jumpGfx).monSprites[multiplayerId]).callback
        == Some(SpriteCB_MonHitShake as unsafe extern "C" fn(*mut Sprite))) as u32;
}
pub(crate) unsafe extern "C" fn SpriteCB_MonHitShake(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 1
    {
        if ({
            (*sprite).data[2] += 1;
            (*sprite).data[2]
        }) as i32
            & 1
            != 0
        {
            (*sprite).y2 = 2;
        } else {
            (*sprite).y2 = -2;
        }
        (*sprite).data[1] = 0;
    }
    if (*sprite).data[2] > 12 {
        (*sprite).y2 = 0;
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe extern "C" fn Gfx_StartMonHitFlash(
    jumpGfx: *mut PokemonJumpGfx,
    multiplayerId: i32,
) {
    ResetPokeJumpSpriteData((*jumpGfx).monSprites[multiplayerId]);
    (*(*jumpGfx).monSprites[multiplayerId]).callback = Some(SpriteCB_MonHitFlash);
}
pub(crate) unsafe extern "C" fn Gfx_StopMonHitFlash(jumpGfx: *mut PokemonJumpGfx) {
    let mut i: i32 = 0;
    let mut numPlayers: u16 = GetNumPokeJumpPlayers();
    i = 0;
    while i < numPlayers as i32 {
        if (*(*jumpGfx).monSprites[i]).callback
            == Some(SpriteCB_MonHitFlash as unsafe extern "C" fn(*mut Sprite))
        {
            (*(*jumpGfx).monSprites[i]).set_invisible(FALSE as u16);
            (*(*jumpGfx).monSprites[i]).callback = Some(SpriteCallbackDummy);
            (*(*jumpGfx).monSprites[i]).subpriority = 10;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonHitFlash(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 3
    {
        (*sprite).data[0] = 0;
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
    }
}
pub(crate) unsafe extern "C" fn Gfx_ResetMonSpriteSubpriorities(jumpGfx: *mut PokemonJumpGfx) {
    let mut i: i32 = 0;
    let mut numPlayers: u16 = GetNumPokeJumpPlayers();
    i = 0;
    while i < numPlayers as i32 {
        (*(*jumpGfx).monSprites[i]).subpriority = (*jumpGfx).monSpriteSubpriorities[i];
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Gfx_StartMonIntroBounce(
    jumpGfx: *mut PokemonJumpGfx,
    multiplayerId: i32,
) {
    ResetPokeJumpSpriteData((*jumpGfx).monSprites[multiplayerId]);
    (*(*jumpGfx).monSprites[multiplayerId]).callback = Some(SpriteCB_MonIntroBounce);
}
pub(crate) unsafe extern "C" fn Gfx_IsMonIntroBounceActive(jumpGfx: *mut PokemonJumpGfx) -> u32 {
    let mut i: i32 = 0;
    let mut numPlayers: u16 = GetNumPokeJumpPlayers();
    i = 0;
    while i < numPlayers as i32 {
        if (*(*jumpGfx).monSprites[i]).callback
            == Some(SpriteCB_MonIntroBounce as unsafe extern "C" fn(*mut Sprite))
        {
            return TRUE as u32;
        }
        i += 1;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIntroBounce(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            PlaySE(SE_BIKE_HOP);
            (*sprite).data[1] = 0;
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            (*sprite).data[1] += 4;
            if (*sprite).data[1] > 127 {
                (*sprite).data[1] = 0;
            }
            (*sprite).y2 = -(gSineTable[(*sprite).data[1]] >> 3);
            if (*sprite).data[1] == 0 {
                if ({
                    (*sprite).data[2] += 1;
                    (*sprite).data[2]
                }) < 2
                {
                    (*sprite).data[0] = 0;
                } else {
                    (*sprite).callback = Some(SpriteCallbackDummy);
                }
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateStarSprite(
    jumpGfx: *mut PokemonJumpGfx,
    x: i16,
    y: i16,
    multiplayerId: u8,
) {
    let mut spriteId: u8 = CreateSprite((&raw const *sSpriteTemplate_Star).cast_mut(), x, y, 1);
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].set_invisible(TRUE as u16);
        (*jumpGfx).starSprites[multiplayerId] = &raw mut gSprites[spriteId];
    }
}
pub(crate) unsafe extern "C" fn CreateVineSprites(jumpGfx: *mut PokemonJumpGfx) {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut spriteId: u8 = 0;
    count = 0;
    i = 0;
    while i < VINE_SPRITES_PER_SIDE {
        spriteId = CreateSprite(
            sSpriteTemplates_Vine[i],
            sVineXCoords[count],
            sVineYCoords[i][0],
            2,
        );
        (*jumpGfx).vineSprites[count] = &raw mut gSprites[spriteId];
        count += 1;
        i += 1;
    }
    i = 3;
    while i >= 0 {
        spriteId = CreateSprite(
            sSpriteTemplates_Vine[i],
            sVineXCoords[count],
            sVineYCoords[i][0],
            2,
        );
        (*jumpGfx).vineSprites[count] = &raw mut gSprites[spriteId];
        (*(*jumpGfx).vineSprites[count]).set_hFlip(TRUE as u16);
        count += 1;
        i -= 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateVineAnim(jumpGfx: *mut PokemonJumpGfx, mut vineState: i32) {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut palNum: i32 = 0;
    let mut priority: i32 = 0;
    if vineState > VINE_LOWEST {
        vineState = NUM_VINESTATES - vineState;
        priority = 3;
        palNum = (*jumpGfx).vinePalNumUpswing as i32;
    } else {
        priority = 2;
        palNum = (*jumpGfx).vinePalNumDownswing as i32;
    }
    count = 0;
    i = 0;
    while i < VINE_SPRITES_PER_SIDE {
        (*(*jumpGfx).vineSprites[count]).y = sVineYCoords[i][vineState];
        (*(*jumpGfx).vineSprites[count])
            .oam
            .set_priority(priority as u16);
        (*(*jumpGfx).vineSprites[count])
            .oam
            .set_paletteNum(palNum as u16);
        StartSpriteAnim((*jumpGfx).vineSprites[count], vineState as u8);
        count += 1;
        i += 1;
    }
    i = 3;
    while i >= 0 {
        (*(*jumpGfx).vineSprites[count]).y = sVineYCoords[i][vineState];
        (*(*jumpGfx).vineSprites[count])
            .oam
            .set_priority(priority as u16);
        (*(*jumpGfx).vineSprites[count])
            .oam
            .set_paletteNum(palNum as u16);
        StartSpriteAnim((*jumpGfx).vineSprites[count], vineState as u8);
        count += 1;
        i -= 1;
    }
}
pub(crate) unsafe extern "C" fn StartPokeJumpCountdown(jumpGfx: *mut PokemonJumpGfx) {
    StartMinigameCountdown(GFXTAG_COUNTDOWN, PALTAG_COUNTDOWN, 120, 80, 0);
    Gfx_ResetMonSpriteSubpriorities(jumpGfx);
}
pub(crate) unsafe extern "C" fn IsPokeJumpCountdownRunning() -> u32 {
    return IsMinigameCountdownRunning();
}
pub(crate) unsafe extern "C" fn StartPokeJumpGfx(jumpGfx: *mut PokemonJumpGfx) {
    let mut taskId: u8 = 0;
    sPokemonJumpGfx = jumpGfx;
    InitPokeJumpGfx(sPokemonJumpGfx);
    taskId = CreateTask(Some(Task_RunPokeJumpGfxFunc), 3);
    (*sPokemonJumpGfx).taskId = taskId;
    SetWordTaskArg(
        (*sPokemonJumpGfx).taskId,
        2,
        sPokemonJumpGfx as usize as u32,
    );
    SetUpPokeJumpGfxFunc(Some(LoadPokeJumpGfx));
}
pub(crate) unsafe extern "C" fn FreeWindowsAndDigitObj() {
    FreeAllWindowBuffers();
    DigitObjUtil_Free();
}
pub(crate) unsafe extern "C" fn InitPokeJumpGfx(jumpGfx: *mut PokemonJumpGfx) {
    (*jumpGfx).mainState = 0;
    (*jumpGfx).funcFinished = FALSE as u32;
    (*jumpGfx).msgWindowId = WINDOW_NONE as u16;
}
pub(crate) unsafe extern "C" fn SetUpPokeJumpGfxFuncById(id: i32) {
    let mut i: i32 = 0;
    i = 0;
    while i < 10 {
        if sPokeJumpGfxFuncs[i].id == id {
            SetUpPokeJumpGfxFunc(sPokeJumpGfxFuncs[i].func);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn IsPokeJumpGfxFuncFinished() -> u32 {
    return ((*sPokemonJumpGfx).funcFinished != TRUE as u32) as u32;
}
pub(crate) unsafe extern "C" fn SetUpPokeJumpGfxFunc(func: Option<unsafe extern "C" fn()>) {
    SetWordTaskArg(
        (*sPokemonJumpGfx).taskId,
        0,
        core::mem::transmute::<_, usize>(func) as u32,
    );
    (*sPokemonJumpGfx).mainState = 0;
    (*sPokemonJumpGfx).funcFinished = FALSE as u32;
}
pub(crate) unsafe extern "C" fn Task_RunPokeJumpGfxFunc(taskId: u8) {
    if (*sPokemonJumpGfx).funcFinished == 0 {
        let mut func: Option<unsafe extern "C" fn()> =
            core::mem::transmute::<_, Option<unsafe extern "C" fn()>>(GetWordTaskArg(taskId, 0)
                as usize
                as *mut c_void);
        func.unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn LoadPokeJumpGfx() {
    match (*sPokemonJumpGfx).mainState {
        0 => {
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
            InitWindows(sWindowTemplates.as_ptr().cast_mut());
            ResetTempTileDataBuffers();
            LoadSpriteSheetsAndPalettes(sPokemonJumpGfx);
            InitDigitPrinters();
            LoadPalette(sBg_Pal.as_ptr().cast_mut() as *mut c_void, 0, 32);
            DecompressAndCopyTileDataToVram(
                BG_SCENERY,
                sBg_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            DecompressAndCopyTileDataToVram(
                BG_SCENERY,
                sBg_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                1,
            );
            LoadPalette(sVenusaur_Pal.as_ptr().cast_mut() as *mut c_void, 48, 32);
            DecompressAndCopyTileDataToVram(
                BG_VENUSAUR,
                sVenusaur_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            DecompressAndCopyTileDataToVram(
                BG_VENUSAUR,
                sVenusaur_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                1,
            );
            LoadPalette(sBonuses_Pal.as_ptr().cast_mut() as *mut c_void, 16, 32);
            DecompressAndCopyTileDataToVram(
                BG_BONUSES,
                sBonuses_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            DecompressAndCopyTileDataToVram(
                BG_BONUSES,
                sBonuses_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                1,
            );
            LoadPalette(sInterface_Pal.as_ptr().cast_mut() as *mut c_void, 32, 32);
            SetBgTilemapBuffer(
                BG_INTERFACE,
                (*sPokemonJumpGfx).tilemapBuffer.as_mut_ptr() as *mut c_void,
            );
            FillBgTilemapBufferRect_Palette0(BG_INTERFACE, 0, 0, 0, 0x20, 0x20);
            PrintScoreSuffixes();
            PrintScore(0);
            LoadUserWindowBorderGfxOnBg(0, 1, 224);
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            CopyBgTilemapBufferToVram(BG_VENUSAUR);
            CopyBgTilemapBufferToVram(BG_BONUSES);
            ResetBgPositions();
            (*sPokemonJumpGfx).mainState += 1;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() == 0 {
                CreateJumpMonSprites();
                CreateVineSprites(sPokemonJumpGfx);
                UpdateVineAnim(sPokemonJumpGfx, VINE_UPSWING_LOWER as i32);
                ShowBg(BG_SCENERY);
                ShowBg(BG_INTERFACE);
                ShowBg(BG_VENUSAUR);
                HideBg(BG_BONUSES);
                (*sPokemonJumpGfx).mainState += 1;
            }
        }
        2 => {
            (*sPokemonJumpGfx).funcFinished = TRUE as u32;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn PrintPlayerNamesNoHighlight() {
    match (*sPokemonJumpGfx).mainState {
        0 => {
            AddPlayerNameWindows();
            (*sPokemonJumpGfx).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PrintPokeJumpPlayerNames(FALSE as u32);
                (*sPokemonJumpGfx).mainState += 1;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                DrawPlayerNameWindows();
                (*sPokemonJumpGfx).mainState += 1;
            }
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                (*sPokemonJumpGfx).funcFinished = TRUE as u32;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn PrintPlayerNamesWithHighlight() {
    match (*sPokemonJumpGfx).mainState {
        0 => {
            AddPlayerNameWindows();
            (*sPokemonJumpGfx).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PrintPokeJumpPlayerNames(TRUE as u32);
                (*sPokemonJumpGfx).mainState += 1;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                DrawPlayerNameWindows();
                (*sPokemonJumpGfx).mainState += 1;
            }
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                (*sPokemonJumpGfx).funcFinished = TRUE as u32;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ErasePlayerNames() {
    let mut i: i32 = 0;
    let mut numPlayers: i32 = 0;
    numPlayers = GetNumPokeJumpPlayers() as i32;
    match (*sPokemonJumpGfx).mainState {
        0 => {
            i = 0;
            while i < numPlayers {
                ClearWindowTilemap((*sPokemonJumpGfx).nameWindowIds[i] as u8);
                i += 1;
            }
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            (*sPokemonJumpGfx).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                i = 0;
                while i < numPlayers {
                    RemoveWindow((*sPokemonJumpGfx).nameWindowIds[i] as u8);
                    i += 1;
                }
                (*sPokemonJumpGfx).funcFinished = TRUE as u32;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Msg_WantToPlayAgain() {
    match (*sPokemonJumpGfx).mainState {
        0 => {
            (*sPokemonJumpGfx).msgWindowId = AddMessageWindow(1, 8, 20, 2) as u16;
            AddTextPrinterParameterized(
                (*sPokemonJumpGfx).msgWindowId as u8,
                FONT_NORMAL,
                gText_WantToPlayAgain2.as_ptr().cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram((*sPokemonJumpGfx).msgWindowId as u8, COPYWIN_GFX);
            (*sPokemonJumpGfx).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*sPokemonJumpGfx).msgWindowId as u8);
                DrawTextBorderOuter((*sPokemonJumpGfx).msgWindowId as u8, 1, 14);
                CreatePokeJumpYesNoMenu(23, 7, 0);
                CopyBgTilemapBufferToVram(BG_INTERFACE);
                (*sPokemonJumpGfx).mainState += 1;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                (*sPokemonJumpGfx).funcFinished = TRUE as u32;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Msg_SavingDontTurnOff() {
    match (*sPokemonJumpGfx).mainState {
        0 => {
            (*sPokemonJumpGfx).msgWindowId = AddMessageWindow(2, 7, 26, 4) as u16;
            AddTextPrinterParameterized(
                (*sPokemonJumpGfx).msgWindowId as u8,
                FONT_NORMAL,
                gText_SavingDontTurnOffPower.as_ptr().cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram((*sPokemonJumpGfx).msgWindowId as u8, COPYWIN_GFX);
            (*sPokemonJumpGfx).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*sPokemonJumpGfx).msgWindowId as u8);
                DrawTextBorderOuter((*sPokemonJumpGfx).msgWindowId as u8, 1, 14);
                CopyBgTilemapBufferToVram(BG_INTERFACE);
                (*sPokemonJumpGfx).mainState += 1;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                (*sPokemonJumpGfx).funcFinished = TRUE as u32;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn EraseMessage() {
    match (*sPokemonJumpGfx).mainState {
        0 => {
            ClearMessageWindow();
            EraseYesNoWindow();
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            (*sPokemonJumpGfx).mainState += 1;
        }
        1 => {
            if RemoveMessageWindow() == 0 && IsDma3ManagerBusyWithBgCopy() == 0 {
                (*sPokemonJumpGfx).funcFinished = TRUE as u32;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Msg_SomeoneDroppedOut() {
    match (*sPokemonJumpGfx).mainState {
        0 => {
            (*sPokemonJumpGfx).msgWindowId = AddMessageWindow(2, 8, 22, 4) as u16;
            AddTextPrinterParameterized(
                (*sPokemonJumpGfx).msgWindowId as u8,
                FONT_NORMAL,
                gText_SomeoneDroppedOut2.as_ptr().cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram((*sPokemonJumpGfx).msgWindowId as u8, COPYWIN_GFX);
            (*sPokemonJumpGfx).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*sPokemonJumpGfx).msgWindowId as u8);
                DrawTextBorderOuter((*sPokemonJumpGfx).msgWindowId as u8, 1, 14);
                CopyBgTilemapBufferToVram(BG_INTERFACE);
                (*sPokemonJumpGfx).mainState += 1;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                (*sPokemonJumpGfx).funcFinished = TRUE as u32;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Msg_CommunicationStandby() {
    match (*sPokemonJumpGfx).mainState {
        0 => {
            (*sPokemonJumpGfx).msgWindowId = AddMessageWindow(7, 10, 16, 2) as u16;
            AddTextPrinterParameterized(
                (*sPokemonJumpGfx).msgWindowId as u8,
                FONT_NORMAL,
                gText_CommunicationStandby4.as_ptr().cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram((*sPokemonJumpGfx).msgWindowId as u8, COPYWIN_GFX);
            (*sPokemonJumpGfx).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*sPokemonJumpGfx).msgWindowId as u8);
                DrawTextBorderOuter((*sPokemonJumpGfx).msgWindowId as u8, 1, 14);
                CopyBgTilemapBufferToVram(BG_INTERFACE);
                (*sPokemonJumpGfx).mainState += 1;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                (*sPokemonJumpGfx).funcFinished = TRUE as u32;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn DoPokeJumpCountdown() {
    match (*sPokemonJumpGfx).mainState {
        0 => {
            StartPokeJumpCountdown(sPokemonJumpGfx);
            (*sPokemonJumpGfx).mainState += 1;
        }
        1 => {
            if IsPokeJumpCountdownRunning() == 0 {
                (*sPokemonJumpGfx).funcFinished = TRUE as u32;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetUpResetVineGfx() {
    (*sPokemonJumpGfx).resetVineState = 0;
    (*sPokemonJumpGfx).resetVineTimer = 0;
    (*sPokemonJumpGfx).vineState = VINE_UPSWING_LOWER as u8;
    UpdateVineSwing((*sPokemonJumpGfx).vineState as i32);
}
pub(crate) unsafe extern "C" fn ResetVineGfx() -> u32 {
    'l1: {
        let sw1: u8 = (*sPokemonJumpGfx).resetVineState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sPokemonJumpGfx).resetVineTimer += 1;
            if (*sPokemonJumpGfx).resetVineTimer > 10 {
                (*sPokemonJumpGfx).resetVineTimer = 0;
                (*sPokemonJumpGfx).vineState += 1;
                if (*sPokemonJumpGfx).vineState >= NUM_VINESTATES as u8 {
                    (*sPokemonJumpGfx).vineState = VINE_HIGHEST as u8;
                    (*sPokemonJumpGfx).resetVineState += 1;
                }
            }
            UpdateVineSwing((*sPokemonJumpGfx).vineState as i32);
            if (*sPokemonJumpGfx).vineState != VINE_UPSWING_LOW as u8 {
                break 'l1;
            }
        }
        if fall || sw1 == 1 {
            fall = true;
            return FALSE as u32;
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn PrintPrizeMessage(itemId: u16, quantity: u16) {
    CopyItemNameHandlePlural(
        itemId,
        (*sPokemonJumpGfx).itemName.as_mut_ptr(),
        quantity as u32,
    );
    ConvertIntToDecimalStringN(
        (*sPokemonJumpGfx).itemQuantityStr.as_mut_ptr(),
        quantity as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        1,
    );
    DynamicPlaceholderTextUtil_Reset();
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, (*sPokemonJumpGfx).itemName.as_mut_ptr());
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(
        1,
        (*sPokemonJumpGfx).itemQuantityStr.as_mut_ptr(),
    );
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        (*sPokemonJumpGfx).prizeMsg.as_mut_ptr(),
        gText_AwesomeWonF701F700.as_ptr().cast_mut(),
    );
    (*sPokemonJumpGfx).msgWindowId = AddMessageWindow(4, 8, 22, 4) as u16;
    AddTextPrinterParameterized(
        (*sPokemonJumpGfx).msgWindowId as u8,
        FONT_NORMAL,
        (*sPokemonJumpGfx).prizeMsg.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram((*sPokemonJumpGfx).msgWindowId as u8, COPYWIN_GFX);
    (*sPokemonJumpGfx).fanfare = MUS_LEVEL_UP;
    (*sPokemonJumpGfx).msgWindowState = 0;
}
pub(crate) unsafe extern "C" fn PrintPrizeFilledBagMessage(itemId: u16) {
    CopyItemName(itemId, (*sPokemonJumpGfx).itemName.as_mut_ptr());
    DynamicPlaceholderTextUtil_Reset();
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, (*sPokemonJumpGfx).itemName.as_mut_ptr());
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        (*sPokemonJumpGfx).prizeMsg.as_mut_ptr(),
        gText_FilledStorageSpace2.as_ptr().cast_mut(),
    );
    (*sPokemonJumpGfx).msgWindowId = AddMessageWindow(4, 8, 22, 4) as u16;
    AddTextPrinterParameterized(
        (*sPokemonJumpGfx).msgWindowId as u8,
        FONT_NORMAL,
        (*sPokemonJumpGfx).prizeMsg.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram((*sPokemonJumpGfx).msgWindowId as u8, COPYWIN_GFX);
    (*sPokemonJumpGfx).fanfare = MUS_DUMMY;
    (*sPokemonJumpGfx).msgWindowState = 0;
}
pub(crate) unsafe extern "C" fn PrintNoRoomForPrizeMessage(itemId: u16) {
    CopyItemName(itemId, (*sPokemonJumpGfx).itemName.as_mut_ptr());
    DynamicPlaceholderTextUtil_Reset();
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, (*sPokemonJumpGfx).itemName.as_mut_ptr());
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        (*sPokemonJumpGfx).prizeMsg.as_mut_ptr(),
        gText_CantHoldMore.as_ptr().cast_mut(),
    );
    (*sPokemonJumpGfx).msgWindowId = AddMessageWindow(4, 9, 22, 2) as u16;
    AddTextPrinterParameterized(
        (*sPokemonJumpGfx).msgWindowId as u8,
        FONT_NORMAL,
        (*sPokemonJumpGfx).prizeMsg.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram((*sPokemonJumpGfx).msgWindowId as u8, COPYWIN_GFX);
    (*sPokemonJumpGfx).fanfare = MUS_DUMMY;
    (*sPokemonJumpGfx).msgWindowState = 0;
}
pub(crate) unsafe extern "C" fn DoPrizeMessageAndFanfare() -> u32 {
    'l1: {
        let sw1: u8 = (*sPokemonJumpGfx).msgWindowState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*sPokemonJumpGfx).msgWindowId as u8);
                DrawTextBorderOuter((*sPokemonJumpGfx).msgWindowId as u8, 1, 14);
                CopyBgTilemapBufferToVram(BG_INTERFACE);
                (*sPokemonJumpGfx).msgWindowState += 1;
            }
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                break 'l1;
            }
            if (*sPokemonJumpGfx).fanfare == MUS_DUMMY {
                (*sPokemonJumpGfx).msgWindowState += 2;
                return FALSE as u32;
            }
            PlayFanfare((*sPokemonJumpGfx).fanfare);
            (*sPokemonJumpGfx).msgWindowState += 1;
        }
        if fall || sw1 == 2 {
            fall = true;
            if IsFanfareTaskInactive() == 0 {
                break 'l1;
            }
            (*sPokemonJumpGfx).msgWindowState += 1;
        }
        if fall || sw1 == 3 {
            fall = true;
            return FALSE as u32;
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn ClearMessageWindow() {
    if (*sPokemonJumpGfx).msgWindowId != WINDOW_NONE as u16 {
        rbox_fill_rectangle((*sPokemonJumpGfx).msgWindowId as u8);
        CopyWindowToVram((*sPokemonJumpGfx).msgWindowId as u8, COPYWIN_MAP);
        (*sPokemonJumpGfx).msgWindowState = 0;
    }
}
pub(crate) unsafe extern "C" fn RemoveMessageWindow() -> u32 {
    if (*sPokemonJumpGfx).msgWindowId == WINDOW_NONE as u16 {
        return FALSE as u32;
    }
    'l1: {
        let sw1: u8 = (*sPokemonJumpGfx).msgWindowState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                RemoveWindow((*sPokemonJumpGfx).msgWindowId as u8);
                (*sPokemonJumpGfx).msgWindowId = WINDOW_NONE as u16;
                (*sPokemonJumpGfx).msgWindowState += 1;
            } else {
                break 'l1;
            }
        }
        if fall || sw1 == 1 {
            fall = true;
            return FALSE as u32;
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn HandlePlayAgainInput() -> i8 {
    return Menu_ProcessInputNoWrapClearOnChoose();
}
pub(crate) unsafe extern "C" fn AddMessageWindow(
    left: u32,
    top: u32,
    width: u32,
    height: u32,
) -> u32 {
    let mut windowId: u32 = 0;
    let mut window: WindowTemplate = zeroed();
    window.bg = BG_INTERFACE;
    window.tilemapLeft = left as u8;
    window.tilemapTop = top as u8;
    window.width = width as u8;
    window.height = height as u8;
    window.paletteNum = 15;
    window.baseBlock = 0x43;
    windowId = AddWindow(&raw mut window) as u32;
    FillWindowPixelBuffer(windowId as u8, 0x11);
    return windowId;
}
pub(crate) unsafe extern "C" fn CreatePokeJumpYesNoMenu(left: u16, top: u16, cursorPos: u8) {
    let mut window: WindowTemplate = zeroed();
    window.bg = BG_INTERFACE;
    window.tilemapLeft = left as u8;
    window.tilemapTop = top as u8;
    window.width = 6;
    window.height = 4;
    window.paletteNum = 2;
    window.baseBlock = 0x2B;
    CreateYesNoMenu(&raw mut window, 1, 0xD, cursorPos);
}
pub(crate) unsafe extern "C" fn PrintScoreSuffixes() {
    let mut color: CArray<u8, 3> = CArray([0, 2, 3]);
    PutWindowTilemap(WIN_POINTS);
    PutWindowTilemap(WIN_TIMES);
    FillWindowPixelBuffer(WIN_POINTS, 0);
    FillWindowPixelBuffer(WIN_TIMES, 0);
    AddTextPrinterParameterized3(
        WIN_POINTS,
        FONT_SMALL,
        0,
        1,
        color.as_mut_ptr(),
        0,
        gText_SpacePoints2.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized3(
        WIN_TIMES,
        FONT_SMALL,
        0,
        1,
        color.as_mut_ptr(),
        0,
        gText_SpaceTimes3.as_ptr().cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn CreateJumpMonSprites() {
    let mut i: i32 = 0;
    let mut y: i32 = 0;
    let mut playersCount: i32 = GetNumPokeJumpPlayers() as i32;
    let mut xCoords: *mut i16 = sMonXCoords[playersCount - 2];
    i = 0;
    while i < playersCount {
        let mut monInfo: *mut PokemonJump_MonInfo = GetMonInfoByMultiplayerId(i as u8);
        y = gMonFrontPicCoords[(*monInfo).species].y_offset as i32;
        CreateJumpMonSprite(sPokemonJumpGfx, monInfo, *xCoords, y as i16 + 112, i as u8);
        CreateStarSprite(sPokemonJumpGfx, *xCoords, 112, i as u8);
        xCoords = xCoords.at(1);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetMonSpriteY(id: u32, y: i16) {
    (*(*sPokemonJumpGfx).monSprites[id]).y2 = y;
}
pub(crate) unsafe extern "C" fn UpdateVineSwing(vineState: i32) {
    UpdateVineAnim(sPokemonJumpGfx, vineState);
    ChangeBgY(
        BG_VENUSAUR,
        (sVenusaurStates[vineState] as i32 * 5) << 13,
        BG_COORD_SET,
    );
}
pub(crate) unsafe extern "C" fn DoSameJumpTimeBonus(mut flags: u8) -> i32 {
    let mut i: i32 = 0;
    let mut numPlayers: i32 = 0;
    i = 0;
    numPlayers = 0;
    while i < MAX_RFU_PLAYERS {
        if flags as i32 & 1 != 0 {
            DoStarAnim(sPokemonJumpGfx, i);
            numPlayers += 1;
        }
        flags >>= 1;
        i += 1;
    }
    ShowBonus(numPlayers as u8 - 2);
    return numPlayers;
}
pub(crate) unsafe extern "C" fn InitDigitPrinters() {
    let mut template: DigitObjUtilTemplate = zeroed();
    template.set_shape(0);
    template.set_size(0);
    template.set_strConvMode(0);
    template.set_priority(1);
    template.oamCount = 5;
    template.xDelta = 8;
    template.x = 108;
    template.y = 6;
    template.spriteSheet =
        (&raw const *sSpriteSheet_Digits).cast_mut() as *mut c_void as *mut SpriteSheet;
    template.spritePal = (&raw const *sSpritePalette_Digits).cast_mut();
    DigitObjUtil_Init(NUM_WINDOWS);
    DigitObjUtil_CreatePrinter(WIN_POINTS as u32, 0, &raw mut template);
    template.oamCount = 4;
    template.x = 30;
    template.y = 6;
    DigitObjUtil_CreatePrinter(WIN_TIMES as u32, 0, &raw mut template);
}
pub(crate) unsafe extern "C" fn PrintScore(num: i32) {
    DigitObjUtil_PrintNumOn(WIN_POINTS as u32, num);
}
pub(crate) unsafe extern "C" fn PrintJumpsInRow(num: u16) {
    DigitObjUtil_PrintNumOn(WIN_TIMES as u32, num as i32);
}
pub(crate) unsafe extern "C" fn StartMonHitShake(multiplayerId: u8) {
    Gfx_StartMonHitShake(sPokemonJumpGfx, multiplayerId as i32);
}
pub(crate) unsafe extern "C" fn StartMonHitFlash(multiplayerId: u8) {
    Gfx_StartMonHitFlash(sPokemonJumpGfx, multiplayerId as i32);
}
pub(crate) unsafe extern "C" fn IsMonHitShakeActive(multiplayerId: i32) -> i32 {
    return Gfx_IsMonHitShakeActive(sPokemonJumpGfx, multiplayerId) as i32;
}
pub(crate) unsafe extern "C" fn StopMonHitFlash() {
    Gfx_StopMonHitFlash(sPokemonJumpGfx);
}
pub(crate) unsafe extern "C" fn ResetMonSpriteSubpriorities() {
    Gfx_ResetMonSpriteSubpriorities(sPokemonJumpGfx);
}
pub(crate) unsafe extern "C" fn StartMonIntroBounce(multiplayerId: i32) {
    Gfx_StartMonIntroBounce(sPokemonJumpGfx, multiplayerId);
}
pub(crate) unsafe extern "C" fn IsMonIntroBounceActive() -> i32 {
    return Gfx_IsMonIntroBounceActive(sPokemonJumpGfx) as i32;
}
pub(crate) unsafe extern "C" fn AddPlayerNameWindows() {
    let mut window: WindowTemplate = zeroed();
    let mut i: i32 = 0;
    let mut playersCount: i32 = GetNumPokeJumpPlayers() as i32;
    let mut winCoords: *mut u16 = sPlayerNameWindowCoords[playersCount - 2];
    window.bg = BG_INTERFACE;
    window.width = 8;
    window.height = 2;
    window.paletteNum = 2;
    window.baseBlock = 0x2B;
    i = 0;
    while i < playersCount {
        window.tilemapLeft = *winCoords as u8;
        window.tilemapTop = *winCoords.at(1) as u8;
        (*sPokemonJumpGfx).nameWindowIds[i] = AddWindow(&raw mut window);
        ClearWindowTilemap((*sPokemonJumpGfx).nameWindowIds[i] as u8);
        window.baseBlock += 0x10;
        winCoords = winCoords.at(2);
        i += 1;
    }
    CopyBgTilemapBufferToVram(BG_INTERFACE);
}
pub(crate) unsafe extern "C" fn PrintPokeJumpPlayerName(
    multiplayerId: i32,
    bgColor: u8,
    fgColor: u8,
    shadow: u8,
) {
    let mut x: u32 = 0;
    let mut colors: CArray<u8, 3> = zeroed();
    colors[0] = bgColor;
    colors[1] = fgColor;
    colors[2] = shadow;
    FillWindowPixelBuffer((*sPokemonJumpGfx).nameWindowIds[multiplayerId] as u8, 0);
    x = 64 - GetStringWidth(FONT_NORMAL, GetPokeJumpPlayerName(multiplayerId as u8), -1) as u32;
    x = x / 2;
    AddTextPrinterParameterized3(
        (*sPokemonJumpGfx).nameWindowIds[multiplayerId] as u8,
        FONT_NORMAL,
        x as u8,
        1,
        colors.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        GetPokeJumpPlayerName(multiplayerId as u8),
    );
    CopyWindowToVram(
        (*sPokemonJumpGfx).nameWindowIds[multiplayerId] as u8,
        COPYWIN_GFX,
    );
}
pub(crate) unsafe extern "C" fn PrintPokeJumpPlayerNames(highlightSelf: u32) {
    let mut i: i32 = 0;
    let mut multiplayerId: i32 = 0;
    let mut playersCount: i32 = GetNumPokeJumpPlayers() as i32;
    if highlightSelf == 0 {
        i = 0;
        while i < playersCount {
            PrintPokeJumpPlayerName(
                i,
                TEXT_COLOR_TRANSPARENT,
                TEXT_COLOR_DARK_GRAY,
                TEXT_COLOR_LIGHT_GRAY,
            );
            i += 1;
        }
    } else {
        multiplayerId = GetPokeJumpMultiplayerId() as i32;
        i = 0;
        while i < playersCount {
            if multiplayerId != i {
                PrintPokeJumpPlayerName(
                    i,
                    TEXT_COLOR_TRANSPARENT,
                    TEXT_COLOR_DARK_GRAY,
                    TEXT_COLOR_LIGHT_GRAY,
                );
            } else {
                PrintPokeJumpPlayerName(
                    i,
                    TEXT_COLOR_TRANSPARENT,
                    TEXT_COLOR_RED,
                    TEXT_COLOR_LIGHT_RED,
                );
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn DrawPlayerNameWindows() {
    let mut i: i32 = 0;
    let mut playersCount: i32 = GetNumPokeJumpPlayers() as i32;
    i = 0;
    while i < playersCount {
        PutWindowTilemap((*sPokemonJumpGfx).nameWindowIds[i] as u8);
        i += 1;
    }
    CopyBgTilemapBufferToVram(BG_INTERFACE);
}
pub(crate) unsafe extern "C" fn ShowBonus(bonusId: u8) {
    (*sPokemonJumpGfx).bonusTimer = 0;
    ChangeBgX(BG_BONUSES, bonusId as i32 / 2 * 256 * 256, BG_COORD_SET);
    ChangeBgY(
        BG_BONUSES,
        (bonusId as i32 % 2 * 256 - 40) * 256,
        BG_COORD_SET,
    );
    ShowBg(BG_BONUSES);
    CreateTask(Some(Task_UpdateBonus), 4);
}
pub(crate) unsafe extern "C" fn UpdateBonus() -> u32 {
    if (*sPokemonJumpGfx).bonusTimer >= 32 {
        return FALSE as u32;
    } else {
        ChangeBgY(BG_BONUSES, 128, BG_COORD_ADD);
        if ({
            (*sPokemonJumpGfx).bonusTimer += 1;
            (*sPokemonJumpGfx).bonusTimer
        }) >= 32
        {
            HideBg(BG_BONUSES);
        }
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateBonus(taskId: u8) {
    if UpdateBonus() == 0 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SendPacket_MonInfo(monInfo: *mut PokemonJump_MonInfo) {
    let mut packet: MonInfoPacket = zeroed();
    packet.id = PACKET_MON_INFO;
    packet.species = (*monInfo).species;
    packet.otId = (*monInfo).otId;
    packet.personality = (*monInfo).personality;
    Rfu_SendPacket(&raw mut packet as *mut c_void);
}
pub(crate) unsafe extern "C" fn RecvPacket_MonInfo(
    multiplayerId: i32,
    monInfo: *mut PokemonJump_MonInfo,
) -> u32 {
    let mut packet: MonInfoPacket = zeroed();
    if gRecvCmds[multiplayerId][0] as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        return FALSE as u32;
    }
    memcpy(
        &raw mut packet as *mut u8,
        &raw mut gRecvCmds[multiplayerId][1] as *mut u8,
        12,
    );
    if packet.id == PACKET_MON_INFO {
        (*monInfo).species = packet.species;
        (*monInfo).otId = packet.otId;
        (*monInfo).personality = packet.personality;
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn SendPacket_Unused(data: u32) {
    let mut packet: UnusedPacket = zeroed();
    packet.id = PACKET_UNUSED;
    packet.data = data;
    Rfu_SendPacket(&raw mut packet as *mut c_void);
}
pub(crate) unsafe extern "C" fn SendPacket_LeaderState(
    player: *mut PokemonJump_Player,
    comm: *mut PokemonJump_CommData,
) {
    let mut packet: LeaderStatePacket = zeroed();
    packet.id = PACKET_LEADER_STATE;
    packet.set_jumpScore((*comm).jumpScore);
    packet.set_receivedBonusFlags((*comm).receivedBonusFlags);
    packet.funcId = (*comm).funcId;
    packet.vineTimer = (*comm).data;
    packet.set_jumpsInRow((*comm).jumpsInRow as u32);
    packet.monState = (*player).monState as u8;
    packet.set_jumpState((*player).jumpState as u8);
    packet.jumpTimeStart = (*player).jumpTimeStart;
    Rfu_SendPacket(&raw mut packet as *mut c_void);
}
pub(crate) unsafe extern "C" fn RecvPacket_LeaderState(
    player: *mut PokemonJump_Player,
    comm: *mut PokemonJump_CommData,
) -> u32 {
    let mut packet: LeaderStatePacket = zeroed();
    if gRecvCmds[0][0] as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        return FALSE as u32;
    }
    memcpy(
        &raw mut packet as *mut u8,
        &raw mut gRecvCmds[0][1] as *mut u8,
        12,
    );
    if packet.id != PACKET_LEADER_STATE {
        return FALSE as u32;
    }
    (*comm).jumpScore = packet.jumpScore();
    (*comm).receivedBonusFlags = packet.receivedBonusFlags();
    (*comm).funcId = packet.funcId;
    (*comm).data = packet.vineTimer;
    (*comm).jumpsInRow = packet.jumpsInRow() as u16;
    (*player).monState = packet.monState as u16;
    (*player).jumpState = packet.jumpState() as i32;
    (*player).jumpTimeStart = packet.jumpTimeStart;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn SendPacket_MemberState(
    player: *mut PokemonJump_Player,
    funcId: u8,
    playAgainState: u16,
) {
    let mut packet: MemberStatePacket = zeroed();
    packet.id = PACKET_MEMBER_STATE;
    packet.monState = (*player).monState as u8;
    packet.jumpState = (*player).jumpState as u8;
    packet.funcFinished = (*player).funcFinished as u8;
    packet.jumpTimeStart = (*player).jumpTimeStart;
    packet.funcId = funcId;
    packet.playAgainState = playAgainState;
    Rfu_SendPacket(&raw mut packet as *mut c_void);
}
pub(crate) unsafe extern "C" fn RecvPacket_MemberStateToLeader(
    player: *mut PokemonJump_Player,
    multiplayerId: i32,
    funcId: *mut u8,
    playAgainState: *mut u16,
) -> u32 {
    let mut packet: MemberStatePacket = zeroed();
    if gRecvCmds[multiplayerId][0] as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        return FALSE as u32;
    }
    memcpy(
        &raw mut packet as *mut u8,
        &raw mut gRecvCmds[multiplayerId][1] as *mut u8,
        12,
    );
    if packet.id != PACKET_MEMBER_STATE {
        return FALSE as u32;
    }
    (*player).monState = packet.monState as u16;
    (*player).jumpState = packet.jumpState as i32;
    (*player).funcFinished = packet.funcFinished as u32;
    (*player).jumpTimeStart = packet.jumpTimeStart;
    *funcId = packet.funcId;
    *playAgainState = packet.playAgainState;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn RecvPacket_MemberStateToMember(
    player: *mut PokemonJump_Player,
    multiplayerId: i32,
) -> u32 {
    let mut packet: MemberStatePacket = zeroed();
    if gRecvCmds[multiplayerId][0] as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        return FALSE as u32;
    }
    memcpy(
        &raw mut packet as *mut u8,
        &raw mut gRecvCmds[multiplayerId][1] as *mut u8,
        12,
    );
    if packet.id != PACKET_MEMBER_STATE {
        return FALSE as u32;
    }
    (*player).monState = packet.monState as u16;
    (*player).jumpState = packet.jumpState as i32;
    (*player).funcFinished = packet.funcFinished as u32;
    (*player).jumpTimeStart = packet.jumpTimeStart;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn GetPokeJumpRecords() -> *mut PokemonJumpRecords {
    return &raw mut (*gSaveBlock2Ptr).pokeJump;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPokemonJumpRecords() {
    let mut records: *mut PokemonJumpRecords = GetPokeJumpRecords();
    (*records).jumpsInRow = 0;
    (*records).bestJumpScore = 0;
    (*records).excellentsInRow = 0;
    (*records).gamesWithMaxPlayers = 0;
    (*records).unused2 = 0;
    (*records).unused1 = 0;
}
pub(crate) unsafe extern "C" fn TryUpdateRecords(
    jumpScore: u32,
    jumpsInRow: u16,
    excellentsInRow: u16,
) -> u32 {
    let mut records: *mut PokemonJumpRecords = GetPokeJumpRecords();
    let mut newRecord: u32 = FALSE as u32;
    if (*records).bestJumpScore < jumpScore && jumpScore <= MAX_JUMP_SCORE {
        (*records).bestJumpScore = jumpScore;
        newRecord = TRUE as u32;
    }
    if (*records).jumpsInRow < jumpsInRow && jumpsInRow <= MAX_JUMPS {
        (*records).jumpsInRow = jumpsInRow;
        newRecord = TRUE as u32;
    }
    if (*records).excellentsInRow < excellentsInRow && excellentsInRow <= MAX_JUMPS {
        (*records).excellentsInRow = excellentsInRow;
        newRecord = TRUE as u32;
    }
    return newRecord;
}
pub(crate) unsafe extern "C" fn IncrementGamesWithMaxPlayers() {
    let mut records: *mut PokemonJumpRecords = GetPokeJumpRecords();
    if (*records).gamesWithMaxPlayers < 9999 {
        (*records).gamesWithMaxPlayers += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPokemonJumpRecords() {
    let mut taskId: u8 = CreateTask(Some(Task_ShowPokemonJumpRecords), 0);
    Task_ShowPokemonJumpRecords(taskId);
}
pub(crate) unsafe extern "C" fn Task_ShowPokemonJumpRecords(taskId: u8) {
    let mut window: WindowTemplate = zeroed();
    let mut i: i32 = 0;
    let mut width: i32 = 0;
    let mut widthCurr: i32 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            window = *sWindowTemplate_Records;
            width = GetStringWidth(FONT_NORMAL, gText_PkmnJumpRecords.as_ptr().cast_mut(), 0);
            i = 0;
            while i < 3 {
                widthCurr = GetStringWidth(FONT_NORMAL, sRecordsTexts[i], 0) + 38;
                if widthCurr > width {
                    width = widthCurr;
                }
                i += 1;
            }
            width = (width + 7) / 8;
            if width & 1 != 0 {
                width += 1;
            }
            window.tilemapLeft = ((30 - width) / 2) as u8;
            window.width = width as u8;
            *data.at(1) = AddWindow(&raw mut window) as i16;
            PrintRecordsText(*data.at(1) as u16, width);
            CopyWindowToVram(*data.at(1) as u8, COPYWIN_FULL);
            *data += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                *data += 1;
            }
        }
        2 => {
            if gMain.newKeys as i32 & 3 != 0 {
                rbox_fill_rectangle(*data.at(1) as u8);
                CopyWindowToVram(*data.at(1) as u8, COPYWIN_MAP);
                *data += 1;
            }
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                RemoveWindow(*data.at(1) as u8);
                DestroyTask(taskId);
                ScriptContext_Enable();
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn PrintRecordsText(windowId: u16, width: i32) {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut recordNums: CArray<i32, 3> = zeroed();
    let mut records: *mut PokemonJumpRecords = GetPokeJumpRecords();
    recordNums[0] = (*records).jumpsInRow as i32;
    recordNums[1] = (*records).bestJumpScore as i32;
    recordNums[2] = (*records).excellentsInRow as i32;
    LoadUserWindowBorderGfx_(windowId as u8, 0x21D, 208);
    DrawTextBorderOuter(windowId as u8, 0x21D, 13);
    FillWindowPixelBuffer(windowId as u8, 17);
    AddTextPrinterParameterized(
        windowId as u8,
        FONT_NORMAL,
        gText_PkmnJumpRecords.as_ptr().cast_mut(),
        GetStringCenterAlignXOffset(
            FONT_NORMAL as i32,
            gText_PkmnJumpRecords.as_ptr().cast_mut(),
            width * 8,
        ) as u8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    i = 0;
    while i < 3 {
        AddTextPrinterParameterized(
            windowId as u8,
            FONT_NORMAL,
            sRecordsTexts[i],
            0,
            25 + i as u8 * 16,
            TEXT_SKIP_DRAW,
            None,
        );
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            recordNums[i],
            STR_CONV_MODE_LEFT_ALIGN,
            5,
        );
        TruncateToFirstWordOnly(gStringVar1.as_mut_ptr());
        x = width * 8 - GetStringWidth(FONT_NORMAL, gStringVar1.as_mut_ptr(), 0);
        AddTextPrinterParameterized(
            windowId as u8,
            FONT_NORMAL,
            gStringVar1.as_mut_ptr(),
            x as u8,
            25 + i as u8 * 16,
            TEXT_SKIP_DRAW,
            None,
        );
        i += 1;
    }
    PutWindowTilemap(windowId as u8);
}
pub(crate) unsafe extern "C" fn TruncateToFirstWordOnly(mut str: *mut u8) {
    while *str != EOS {
        if *str == CHAR_SPACE {
            *str = EOS;
            break;
        }
        str = str.at(1);
    }
}
