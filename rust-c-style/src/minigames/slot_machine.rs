//! Translated from `src/slot_machine.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sDigitalDisplayScenes sUnkPalette sSpecialDrawOdds sBiasSymbols sBiasesSpecial sBiasesRegular sDigitalDisplay_SpriteCoords sDigitalDisplay_SpriteCallbacks sSpriteTemplates_DigitalDisplay sSubspriteTables_DigitalDisplay sSpriteTemplate_PikaPowerBolt sSpriteTemplate_ReelTimeSmoke sSpriteTemplate_ReelTimeDuck sSpriteTemplate_ReelTimeExplosion sSpriteTemplate_ReelTimePikachuAura sReelTimeExplodeProbability sPokeballShiningPalTable sReelTimeSpeed_Probabilities sQuarterSpeed_ProbabilityBoost sSlotMatchFlags sSlotPayouts sReelBackground_Tilemap sReelTimeGfx sSlotMachineSpriteSheets sSlotMachineSpritePalettes sDigitalDisplay_Pal sInitialReelPositions sBiasProbabilities_Special sBiasProbabilities_Regular sReelTimeProbabilities_NormalGame sReelTimeProbabilities_LuckyGame sSymbolToMatch sReelTimeSymbols sReelSymbols sLitMatchLinePalTable sDarkMatchLinePalTable sMatchLinePalOffsets sBetToMatchLineIds sMatchLinesPerBet sFlashingLightsPalTable sSlotMachineMenu_Pal sReelTimeWindow_Tilemap sEmptyTilemap sDigitalDisplaySceneExitCallbacks sSpriteTemplate_ReelTimeBolt sSpriteTemplate_ReelTimeNumberGap sSpriteTemplate_ReelTimeShadow sSpriteTemplate_ReelTimeNumbers sSpriteTemplate_BrokenReelTimeMachine sSpriteTemplate_ReelTimeMachineAntennae sSpriteTemplate_ReelTimeMachine sSpriteTemplate_ReelBackground sSpriteTemplate_CoinNumber sSpriteTemplate_ReelSymbol sSpriteTemplate_ReelTimePikachu sSubspriteTable_ReelTimeNumberGap sSubspriteTable_ReelTimeShadow sSubspriteTable_BrokenReelTimeMachine sSubspriteTable_ReelTimeMachineAntennae sSubspriteTable_ReelTimeMachine sSubspriteTable_ReelBackground sBgTemplates sWindowTemplates sWindowTemplate_InfoBox sColors_ReeltimeHelp sSlotTasks sPayoutTasks sReelTasks sDecideStop_Bias sDecideStop_NoBias sReelStopShocks sDecideStop_Bias_Reel1_Bets sDecideStop_Bias_Reel2_Bets sDecideStop_Bias_Reel3_Bets sDecideStop_NoBias_Reel2_Bets sDecideStop_NoBias_Reel3_Bets sReelStopButtonTasks sReelButtonOffsets sPikaPowerBoltTasks sPikaPowerTileTable sReelTimeTasks sReelTimePikachuAnimIds sReelTimeBoltDelays sPikachuAuraFlashDelays sInfoBoxTasks sDigitalDisplayTasks sReelSymbols sReelTimeSymbols sInitialReelPositions sSpecialDrawOdds sBiasProbabilities_Special sBiasProbabilities_Regular sReelTimeProbabilities_NormalGame sReelTimeProbabilities_LuckyGame sReelTimeExplodeProbability sReelTimeSpeed_Probabilities sQuarterSpeed_ProbabilityBoost sBiasSymbols sBiasesSpecial sBiasesRegular sSymbolToMatch sSlotMatchFlags sSlotPayouts sDigitalDisplay_SpriteCoords sDigitalDisplay_SpriteCallbacks sDigitalDisplay_InsertBet sDigitalDisplay_StopReel sDigitalDisplay_Win sDigitalDisplay_Lose sDigitalDisplay_ReelTime sDigitalDisplay_BonusBig sDigitalDisplay_BonusRegular sDigitalDisplayScenes sDigitalDisplaySceneExitCallbacks sOam_8x8 sOam_8x16 sOam_16x16 sOam_16x32 sOam_32x32 sOam_32x64 sOam_64x32 sOam_64x64 sImageTable_ReelTimeNumbers sImageTable_ReelTimeShadow sImageTable_ReelTimeNumberGap sImageTable_ReelTimeBolt sImageTable_ReelTimePikachuAura sImageTable_ReelTimeExplosion sImageTable_ReelTimeDuck sImageTable_ReelTimeSmoke sImageTable_PikaPowerBolt sAnim_SingleFrame sAnim_ReelTimeDuck sAnim_ReelTimePikachu_Still sAnim_ReelTimePikachu_ChargingSlow sAnim_ReelTimePikachu_ChargingMedium sAnim_ReelTimePikachu_ChargingFast sAnim_ReelTimePikachu_Cheering sAnim_ReelTimePikachu_FellOver sAnim_ReelTimeNumber_0 sAnim_ReelTimeNumber_1 sAnim_ReelTimeNumber_2 sAnim_ReelTimeNumber_3 sAnim_ReelTimeNumber_4 sAnim_ReelTimeNumber_5 sAnim_ReelTimeBolt sAnim_ReelTimeExplosion sAnim_DigitalDisplay_AButton_Flashing sAnim_DigitalDisplay_AButton_Static sAnim_DigitalDisplay_DPad_Flashing sAnim_DigitalDisplay_Pokeball_Rocking sAnim_DigitalDisplay_Pokeball_Static sAnim_DigitalDisplay_Number_1 sAnim_DigitalDisplay_Number_2 sAnim_DigitalDisplay_Number_3 sAnim_DigitalDisplay_Number_4 sAnim_DigitalDisplay_Number_5 sAnims_SingleFrame sAnims_ReelTimeDuck sAnims_ReelTimePikachu sAnims_ReelTimeNumbers sAnims_ReelTimeBolt sAnims_ReelTimeExplosion sAnims_DigitalDisplay_AButton sAnims_DigitalDisplay_DPad sAnims_DigitalDisplay_Pokeball sAnims_DigitalDisplay_Number sAffineAnim_ReelTimeSmoke sAffineAnims_ReelTimeSmoke sAffineAnim_PikaPowerBolt sAffineAnims_PikaPowerBolt sSpriteTemplate_ReelSymbol sSpriteTemplate_CoinNumber sSpriteTemplate_ReelBackground sSpriteTemplate_ReelTimePikachu sSpriteTemplate_ReelTimeMachineAntennae sSpriteTemplate_ReelTimeMachine sSpriteTemplate_BrokenReelTimeMachine sSpriteTemplate_ReelTimeNumbers sSpriteTemplate_ReelTimeShadow sSpriteTemplate_ReelTimeNumberGap sSpriteTemplate_ReelTimeBolt sSpriteTemplate_ReelTimePikachuAura sSpriteTemplate_ReelTimeExplosion sSpriteTemplate_ReelTimeDuck sSpriteTemplate_ReelTimeSmoke sSpriteTemplate_DigitalDisplay_Reel sSpriteTemplate_DigitalDisplay_Time sSpriteTemplate_DigitalDisplay_Insert sSpriteTemplate_DigitalDisplay_Stop sSpriteTemplate_DigitalDisplay_Win sSpriteTemplate_DigitalDisplay_Lose sSpriteTemplate_DigitalDisplay_Bonus sSpriteTemplate_DigitalDisplay_Big sSpriteTemplate_DigitalDisplay_Reg sSpriteTemplate_DigitalDisplay_AButton sSpriteTemplate_DigitalDisplay_Smoke sSpriteTemplate_DigitalDisplay_Number sSpriteTemplate_DigitalDisplay_Pokeball sSpriteTemplate_DigitalDisplay_DPad sSpriteTemplate_PikaPowerBolt sSubsprites_ReelBackground sSubspriteTable_ReelBackground sSubsprites_ReelTimeMachineAntennae sSubspriteTable_ReelTimeMachineAntennae sSubsprites_ReelTimeMachine sSubspriteTable_ReelTimeMachine sSubsprites_BrokenReelTimeMachine sSubspriteTable_BrokenReelTimeMachine sSubsprites_ReelTimeShadow sSubspriteTable_ReelTimeShadow sSubsprites_ReelTimeNumberGap sSubspriteTable_ReelTimeNumberGap sSubsprites_DigitalDisplay_Reel sSubspriteTable_DigitalDisplay_Reel sSubsprites_DigitalDisplay_Time sSubspriteTable_DigitalDisplay_Time sSubsprites_DigitalDisplay_Insert sSubspriteTable_DigitalDisplay_Insert sSubsprites_DigitalDisplay_Unused1 sSubspriteTable_DigitalDisplay_Unused1 sSubsprites_DigitalDisplay_Win sSubspriteTable_DigitalDisplay_Win sSubsprites_DigitalDisplay_SmokeBig sSubsprites_DigitalDisplay_SmokeSmall sSubspriteTable_DigitalDisplay_Smoke sSubsprites_DigitalDisplay_Pokeball sSubspriteTable_DigitalDisplay_Pokeball sSubsprites_DigitalDisplay_DPad sSubspriteTable_DigitalDisplay_DPad sSubsprites_DigitalDisplay_StopS sSubspriteTable_DigitalDisplay_StopS sSubsprites_DigitalDisplay_StopT sSubspriteTable_DigitalDisplay_StopT sSubsprites_DigitalDisplay_StopO sSubspriteTable_DigitalDisplay_StopO sSubsprites_DigitalDisplay_StopP sSubspriteTable_DigitalDisplay_StopP sSubsprites_DigitalDisplay_BonusB sSubspriteTable_DigitalDisplay_BonusB sSubsprites_DigitalDisplay_BonusO sSubspriteTable_DigitalDisplay_BonusO sSubsprites_DigitalDisplay_BonusN sSubspriteTable_DigitalDisplay_BonusN sSubsprites_DigitalDisplay_BonusU sSubspriteTable_DigitalDisplay_BonusU sSubsprites_DigitalDisplay_BonusS sSubspriteTable_DigitalDisplay_BonusS sSubsprites_DigitalDisplay_BigB sSubspriteTable_DigitalDisplay_BigB sSubsprites_DigitalDisplay_BigI sSubspriteTable_DigitalDisplay_BigI sSubsprites_DigitalDisplay_BigG sSubspriteTable_DigitalDisplay_BigG sSubsprites_DigitalDisplay_RegR sSubspriteTable_DigitalDisplay_RegR sSubsprites_DigitalDisplay_RegE sSubspriteTable_DigitalDisplay_RegE sSubsprites_DigitalDisplay_RegG sSubspriteTable_DigitalDisplay_RegG sSpriteTemplates_DigitalDisplay sSubspriteTables_DigitalDisplay sSlotMachineSpriteSheets sReelBackground_Tilemap sUnusedColors sMiddleRowLit_Pal sTopRowLit_Pal sBottomRowt_Pal sNWSEDiagLit_Pal sNESWDiagLit_Pal sLitMatchLinePalTable sDarkMatchLinePalTable sMatchLinePalOffsets sBetToMatchLineIds sMatchLinesPerBet sFlashingLightsInside_Pal sFlashingLightsMiddle_Pal sFlashingLightsOutside_Pal sFlashingLightsPalTable sSlotMachineMenu_Pal sPokeballShining0_Pal sPokeballShining1_Pal sPokeballShining2_Pal sPokeballShiningPalTable sDigitalDisplay_Pal sUnkPalette sSlotMachineSpritePalettes sReelTimeGfx sReelTimeWindow_Tilemap sEmptyTilemap

/// `struct SlotMachine`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SlotMachine {
    pub state: u8,
    pub machineId: u8,
    pub pikaPowerBolts: u8,
    pub luckyGame: u8,
    pub machineBias: u8,
    pub reelTimeDraw: u8,
    pub didNotFailBias: u8,
    pub biasSymbol: u8,
    pub matches: u16,
    pub reelTimeSpinsLeft: u8,
    pub reelTimeSpinsUsed: u8,
    pub coins: i16,
    pub payout: i16,
    pub netCoinLoss: i16,
    pub bet: i16,
    pub reeltimePixelOffset: i16,
    pub reeltimePosition: i16,
    pub currentReel: i16,
    pub reelSpeed: i16,
    pub reelPixelOffsets: CArray<i16, 3>,
    pub reelShockOffsets: CArray<u16, 3>,
    pub reelPositions: CArray<i16, 3>,
    pub reelExtraTurns: CArray<i16, 3>,
    pub winnerRows: CArray<i16, 3>,
    pub slotReelTasks: CArray<u8, 3>,
    pub digDisplayTaskId: u8,
    pub pikaPowerBoltTaskId: u8,
    pub reelTimePikachuSpriteId: u8,
    pub reelTimeNumberGapSpriteId: u8,
    pub reelTimeExplosionSpriteId: u8,
    pub reelTimeBrokenMachineSpriteId: u8,
    pub reelTimeSmokeSpriteId: u8,
    pub flashMatchLineSpriteIds: CArray<u8, 5>,
    pub reelTimeMachineSpriteIds: CArray<u8, 2>,
    pub reelTimeNumberSpriteIds: CArray<u8, 3>,
    pub reelTimeShadowSpriteIds: CArray<u8, 2>,
    pub reelTimeBoltSpriteIds: CArray<u8, 2>,
    pub reelTimePikachuAuraSpriteIds: CArray<u8, 2>,
    pub reelTimeDuckSpriteIds: CArray<u8, 4>,
    pub win0h: u16,
    pub win0v: u16,
    pub winIn: u16,
    pub winOut: u16,
    pub backupMapMusic: u16,
    pub prevMainCb: Option<unsafe extern "C" fn()>,
}

unsafe impl Sync for SlotMachine {}

/// `struct DigitalDisplaySprite`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct DigitalDisplaySprite {
    pub spriteTemplateId: u8,
    pub dispInfoId: u8,
    pub spriteId: i16,
}

unsafe impl Sync for DigitalDisplaySprite {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<SlotMachine>() == 104);
    assert!(offset_of!(SlotMachine, state) == 0);
    assert!(offset_of!(SlotMachine, machineId) == 1);
    assert!(offset_of!(SlotMachine, pikaPowerBolts) == 2);
    assert!(offset_of!(SlotMachine, luckyGame) == 3);
    assert!(offset_of!(SlotMachine, machineBias) == 4);
    assert!(offset_of!(SlotMachine, reelTimeDraw) == 5);
    assert!(offset_of!(SlotMachine, didNotFailBias) == 6);
    assert!(offset_of!(SlotMachine, biasSymbol) == 7);
    assert!(offset_of!(SlotMachine, matches) == 8);
    assert!(offset_of!(SlotMachine, reelTimeSpinsLeft) == 10);
    assert!(offset_of!(SlotMachine, reelTimeSpinsUsed) == 11);
    assert!(offset_of!(SlotMachine, coins) == 12);
    assert!(offset_of!(SlotMachine, payout) == 14);
    assert!(offset_of!(SlotMachine, netCoinLoss) == 16);
    assert!(offset_of!(SlotMachine, bet) == 18);
    assert!(offset_of!(SlotMachine, reeltimePixelOffset) == 20);
    assert!(offset_of!(SlotMachine, reeltimePosition) == 22);
    assert!(offset_of!(SlotMachine, currentReel) == 24);
    assert!(offset_of!(SlotMachine, reelSpeed) == 26);
    assert!(offset_of!(SlotMachine, reelPixelOffsets) == 28);
    assert!(offset_of!(SlotMachine, reelShockOffsets) == 34);
    assert!(offset_of!(SlotMachine, reelPositions) == 40);
    assert!(offset_of!(SlotMachine, reelExtraTurns) == 46);
    assert!(offset_of!(SlotMachine, winnerRows) == 52);
    assert!(offset_of!(SlotMachine, slotReelTasks) == 58);
    assert!(offset_of!(SlotMachine, digDisplayTaskId) == 61);
    assert!(offset_of!(SlotMachine, pikaPowerBoltTaskId) == 62);
    assert!(offset_of!(SlotMachine, reelTimePikachuSpriteId) == 63);
    assert!(offset_of!(SlotMachine, reelTimeNumberGapSpriteId) == 64);
    assert!(offset_of!(SlotMachine, reelTimeExplosionSpriteId) == 65);
    assert!(offset_of!(SlotMachine, reelTimeBrokenMachineSpriteId) == 66);
    assert!(offset_of!(SlotMachine, reelTimeSmokeSpriteId) == 67);
    assert!(offset_of!(SlotMachine, flashMatchLineSpriteIds) == 68);
    assert!(offset_of!(SlotMachine, reelTimeMachineSpriteIds) == 73);
    assert!(offset_of!(SlotMachine, reelTimeNumberSpriteIds) == 75);
    assert!(offset_of!(SlotMachine, reelTimeShadowSpriteIds) == 78);
    assert!(offset_of!(SlotMachine, reelTimeBoltSpriteIds) == 80);
    assert!(offset_of!(SlotMachine, reelTimePikachuAuraSpriteIds) == 82);
    assert!(offset_of!(SlotMachine, reelTimeDuckSpriteIds) == 84);
    assert!(offset_of!(SlotMachine, win0h) == 88);
    assert!(offset_of!(SlotMachine, win0v) == 90);
    assert!(offset_of!(SlotMachine, winIn) == 92);
    assert!(offset_of!(SlotMachine, winOut) == 94);
    assert!(offset_of!(SlotMachine, backupMapMusic) == 96);
    assert!(offset_of!(SlotMachine, prevMainCb) == 100);
    assert!(size_of::<DigitalDisplaySprite>() == 4);
    assert!(offset_of!(DigitalDisplaySprite, spriteTemplateId) == 0);
    assert!(offset_of!(DigitalDisplaySprite, dispInfoId) == 1);
    assert!(offset_of!(DigitalDisplaySprite, spriteId) == 2);
};

const BIAS_CHERRY: i32 = 2;
const BIAS_MIXED_7: i32 = 64;
const BIAS_REELTIME: i32 = 32;
const BIAS_STRAIGHT_7: i32 = 128;
const DIG_DISPLAY_BONUS_BIG: u8 = 6;
const DIG_DISPLAY_BONUS_REG: u8 = 5;
const DIG_DISPLAY_INSERT_BET: u8 = 0;
const DIG_DISPLAY_LOSE: u8 = 3;
const DIG_DISPLAY_REEL_TIME: u8 = 4;
const DIG_DISPLAY_STOP_REEL: u8 = 1;
const DIG_DISPLAY_WIN: u8 = 2;
const DIG_SPRITE_A_BUTTON: u8 = 5;
const DIG_SPRITE_BIG_B: i32 = 19;
const DIG_SPRITE_BIG_G: i32 = 21;
const DIG_SPRITE_BIG_I: i32 = 20;
const DIG_SPRITE_BONUS_B: i32 = 14;
const DIG_SPRITE_BONUS_N: i32 = 16;
const DIG_SPRITE_BONUS_O: i32 = 15;
const DIG_SPRITE_BONUS_S: i32 = 18;
const DIG_SPRITE_BONUS_U: i32 = 17;
const DIG_SPRITE_D_PAD: i32 = 9;
const DIG_SPRITE_EMPTY: i32 = 25;
const DIG_SPRITE_INSERT: i32 = 2;
const DIG_SPRITE_LOSE: i32 = 4;
const DIG_SPRITE_NUMBER: i32 = 7;
const DIG_SPRITE_POKE_BALL: i32 = 8;
const DIG_SPRITE_REEL: i32 = 0;
const DIG_SPRITE_REG_E: i32 = 23;
const DIG_SPRITE_REG_G: i32 = 24;
const DIG_SPRITE_REG_R: i32 = 22;
const DIG_SPRITE_SMOKE: i32 = 6;
const DIG_SPRITE_STOP_O: i32 = 12;
const DIG_SPRITE_STOP_P: i32 = 13;
const DIG_SPRITE_STOP_S: i32 = 10;
const DIG_SPRITE_STOP_T: i32 = 11;
const DIG_SPRITE_TIME: i32 = 1;
const DIG_SPRITE_WIN: i32 = 3;
const GFXTAG_BIG: i32 = 20;
const GFXTAG_BONUS: i32 = 19;
const GFXTAG_NUM_0: u16 = 7;
const GFXTAG_REEL_BG: u16 = 17;
const GFXTAG_REG: i32 = 21;
const GFXTAG_STOP: i32 = 18;
const LEFT_REEL: u8 = 0;
const MATCH_BLUE_7: i32 = 8;
const MATCH_BOTTOM_ROW: u8 = 2;
const MATCH_CHERRY: u8 = 0;
const MATCH_MIDDLE_ROW: u8 = 0;
const MATCH_MIXED_7: u8 = 6;
const MATCH_NESW_DIAG: u8 = 4;
const MATCH_NONE: u8 = 9;
const MATCH_NWSE_DIAG: u8 = 3;
const MATCH_POWER: i32 = 5;
const MATCH_RED_7: i32 = 7;
const MATCH_REPLAY: i32 = 2;
const MATCH_TOPBOT_CHERRY: u8 = 1;
const MATCH_TOP_ROW: u8 = 1;
const MAX_BET: i16 = 3;
const MAX_EXTRA_TURNS: i16 = 4;
const MIDDLE_REEL: u8 = 1;
const NUM_REELS: u8 = 3;
const PALTAG_DIG_DISPLAY: u16 = 6;
const PALTAG_PIKA_AURA: u16 = 7;
const PAYOUT_TASK_FREE: i16 = 2;
const PIKABOLT_TASK_ADD_BOLT: i16 = 1;
const PIKABOLT_TASK_CLEAR_ALL: i16 = 3;
const PIKABOLT_TASK_IDLE: i16 = 0;
const REELTIME_REEL_HEIGHT: i32 = 120;
const REELTIME_SYMBOLS: i16 = 6;
const REELTIME_SYMBOL_HEIGHT: i32 = 20;
const REEL_HALF_SPEED: u16 = 4;
const REEL_HEIGHT: i16 = 504;
const REEL_NORMAL_SPEED: i16 = 8;
const REEL_QUARTER_SPEED: u16 = 2;
const REEL_SYMBOL_HEIGHT: i16 = 24;
const REEL_TASK_DECIDE_STOP: i16 = 2;
const REEL_TASK_SPIN: i16 = 1;
const RIGHT_REEL: u8 = 2;
const RT_TASK_EXPLODE: i16 = 14;
const SLOTTASK_ASK_INSERT_BET: u8 = 4;
const SLOTTASK_ASK_QUIT: u8 = 21;
const SLOTTASK_BET_INPUT: u8 = 5;
const SLOTTASK_CHECK_MATCHES: u8 = 14;
const SLOTTASK_END: u8 = 27;
const SLOTTASK_END_PAYOUT: u8 = 16;
const SLOTTASK_HANDLE_QUIT_INPUT: u8 = 22;
const SLOTTASK_MATCHED_POWER: u8 = 17;
const SLOTTASK_MSG_MAX_COINS: u8 = 23;
const SLOTTASK_MSG_NEED_3_COINS: u8 = 6;
const SLOTTASK_MSG_NO_MORE_COINS: u8 = 25;
const SLOTTASK_NO_MATCHES: u8 = 20;
const SLOTTASK_READY_NEW_RT_SPIN: u8 = 3;
const SLOTTASK_READY_NEW_SPIN: u8 = 2;
const SLOTTASK_RESET_BET_TILES: u8 = 19;
const SLOTTASK_RESET_BIAS_FAILURE: u8 = 11;
const SLOTTASK_START_RT_SPIN: u8 = 10;
const SLOTTASK_START_SPIN: u8 = 9;
const SLOTTASK_UNFADE: u8 = 0;
const SLOTTASK_WAIT_ALL_REELS_STOP: u8 = 13;
const SLOTTASK_WAIT_INFO_BOX: u8 = 8;
const SLOTTASK_WAIT_MSG_MAX_COINS: u8 = 24;
const SLOTTASK_WAIT_MSG_NEED_3_COINS: u8 = 7;
const SLOTTASK_WAIT_MSG_NO_MORE_COINS: u8 = 26;
const SLOTTASK_WAIT_PAYOUT: u8 = 15;
const SLOTTASK_WAIT_REEL_STOP: u8 = 12;
const SLOTTASK_WAIT_RT_ANIM: u8 = 18;
const SYMBOLS_PER_REEL: i16 = 21;
const SYMBOL_7_BLUE: u8 = 1;
const SYMBOL_7_RED: u8 = 0;
const SYMBOL_CHERRY: u8 = 4;
const WIN_INFO: u8 = 1;
const WIN_MSG: u8 = 0;

static sBetToMatchLineIds: Table<CArray<CArray<u8, 2>, 3>> =
    Table((&raw const crate::data::slot_machine::sBetToMatchLineIds).cast());
static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::slot_machine::sBgTemplates).cast());
static sBiasProbabilities_Regular: Table<CArray<CArray<u8, 6>, 5>> =
    Table((&raw const crate::data::slot_machine::sBiasProbabilities_Regular).cast());
static sBiasProbabilities_Special: Table<CArray<CArray<u8, 6>, 3>> =
    Table((&raw const crate::data::slot_machine::sBiasProbabilities_Special).cast());
static sBiasSymbols: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::slot_machine::sBiasSymbols).cast());
static sBiasesRegular: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::slot_machine::sBiasesRegular).cast());
static sBiasesSpecial: Table<CArray<u16, 3>> =
    Table((&raw const crate::data::slot_machine::sBiasesSpecial).cast());
static sColors_ReeltimeHelp: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::slot_machine::sColors_ReeltimeHelp).cast());
static sDarkMatchLinePalTable: Table<CArray<*mut u16, 5>> =
    Table((&raw const crate::data::slot_machine::sDarkMatchLinePalTable).cast());
static sDecideStop_Bias: Table<CArray<Option<unsafe extern "C" fn() -> u8>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_Bias).cast());
static sDecideStop_Bias_Reel1_Bets: Table<CArray<Option<unsafe extern "C" fn(u8, u8) -> u8>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_Bias_Reel1_Bets).cast());
static sDecideStop_Bias_Reel2_Bets: Table<CArray<Option<unsafe extern "C" fn() -> u8>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_Bias_Reel2_Bets).cast());
static sDecideStop_Bias_Reel3_Bets: Table<CArray<Option<unsafe extern "C" fn(u8) -> u8>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_Bias_Reel3_Bets).cast());
static sDecideStop_NoBias: Table<CArray<Option<unsafe extern "C" fn()>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_NoBias).cast());
static sDecideStop_NoBias_Reel2_Bets: Table<CArray<Option<unsafe extern "C" fn()>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_NoBias_Reel2_Bets).cast());
static sDecideStop_NoBias_Reel3_Bets: Table<CArray<Option<unsafe extern "C" fn()>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_NoBias_Reel3_Bets).cast());
static sDigitalDisplaySceneExitCallbacks: Table<CArray<Option<unsafe extern "C" fn()>, 7>> =
    Table((&raw const crate::data::slot_machine::sDigitalDisplaySceneExitCallbacks).cast());
static sDigitalDisplayScenes: Table<CArray<*mut DigitalDisplaySprite, 7>> =
    Table((&raw const crate::data::slot_machine::sDigitalDisplayScenes).cast());
static sDigitalDisplayTasks: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 1>> =
    Table((&raw const crate::data::slot_machine::sDigitalDisplayTasks).cast());
static sDigitalDisplay_Pal: Table<*mut u16> =
    Table((&raw const crate::data::slot_machine::sDigitalDisplay_Pal).cast());
static sDigitalDisplay_SpriteCallbacks: Table<
    CArray<Option<unsafe extern "C" fn(*mut Sprite)>, 35>,
> = Table((&raw const crate::data::slot_machine::sDigitalDisplay_SpriteCallbacks).cast());
static sDigitalDisplay_SpriteCoords: Table<CArray<CArray<i16, 2>, 35>> =
    Table((&raw const crate::data::slot_machine::sDigitalDisplay_SpriteCoords).cast());
static sEmptyTilemap: Table<CArray<u16, 1>> =
    Table((&raw const crate::data::slot_machine::sEmptyTilemap).cast());
static sFlashingLightsPalTable: Table<CArray<*mut u16, 3>> =
    Table((&raw const crate::data::slot_machine::sFlashingLightsPalTable).cast());
static sInfoBoxTasks: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 15>> =
    Table((&raw const crate::data::slot_machine::sInfoBoxTasks).cast());
static sInitialReelPositions: Table<CArray<CArray<i16, 2>, 3>> =
    Table((&raw const crate::data::slot_machine::sInitialReelPositions).cast());
static sLitMatchLinePalTable: Table<CArray<*mut u16, 5>> =
    Table((&raw const crate::data::slot_machine::sLitMatchLinePalTable).cast());
static sMatchLinePalOffsets: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::slot_machine::sMatchLinePalOffsets).cast());
static sMatchLinesPerBet: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::slot_machine::sMatchLinesPerBet).cast());
static sPayoutTasks: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 3>> =
    Table((&raw const crate::data::slot_machine::sPayoutTasks).cast());
static sPikaPowerBoltTasks: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 4>> =
    Table((&raw const crate::data::slot_machine::sPikaPowerBoltTasks).cast());
static sPikaPowerTileTable: Table<CArray<CArray<u16, 2>, 3>> =
    Table((&raw const crate::data::slot_machine::sPikaPowerTileTable).cast());
static sPikachuAuraFlashDelays: Table<CArray<i16, 4>> =
    Table((&raw const crate::data::slot_machine::sPikachuAuraFlashDelays).cast());
static sPokeballShiningPalTable: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::slot_machine::sPokeballShiningPalTable).cast());
static sQuarterSpeed_ProbabilityBoost: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::slot_machine::sQuarterSpeed_ProbabilityBoost).cast());
static sReelBackground_Tilemap: Table<*mut u8> =
    Table((&raw const crate::data::slot_machine::sReelBackground_Tilemap).cast());
static sReelButtonOffsets: Table<CArray<i16, 3>> =
    Table((&raw const crate::data::slot_machine::sReelButtonOffsets).cast());
static sReelStopButtonTasks: Table<CArray<Option<unsafe extern "C" fn(*mut Task, u8)>, 3>> =
    Table((&raw const crate::data::slot_machine::sReelStopButtonTasks).cast());
static sReelStopShocks: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::slot_machine::sReelStopShocks).cast());
static sReelSymbols: Table<CArray<CArray<u8, 21>, 3>> =
    Table((&raw const crate::data::slot_machine::sReelSymbols).cast());
static sReelTasks: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 5>> =
    Table((&raw const crate::data::slot_machine::sReelTasks).cast());
static sReelTimeBoltDelays: Table<CArray<i16, 4>> =
    Table((&raw const crate::data::slot_machine::sReelTimeBoltDelays).cast());
static sReelTimeExplodeProbability: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::slot_machine::sReelTimeExplodeProbability).cast());
static sReelTimeGfx: Table<CArray<u32, 1109>> =
    Table((&raw const crate::data::slot_machine::sReelTimeGfx).cast());
static sReelTimePikachuAnimIds: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::slot_machine::sReelTimePikachuAnimIds).cast());
static sReelTimeProbabilities_LuckyGame: Table<CArray<CArray<u8, 17>, 6>> =
    Table((&raw const crate::data::slot_machine::sReelTimeProbabilities_LuckyGame).cast());
static sReelTimeProbabilities_NormalGame: Table<CArray<CArray<u8, 17>, 6>> =
    Table((&raw const crate::data::slot_machine::sReelTimeProbabilities_NormalGame).cast());
static sReelTimeSpeed_Probabilities: Table<CArray<CArray<u16, 2>, 5>> =
    Table((&raw const crate::data::slot_machine::sReelTimeSpeed_Probabilities).cast());
static sReelTimeSymbols: Table<CArray<u8, 6>> =
    Table((&raw const crate::data::slot_machine::sReelTimeSymbols).cast());
static sReelTimeTasks: Table<CArray<Option<unsafe extern "C" fn(*mut Task)>, 19>> =
    Table((&raw const crate::data::slot_machine::sReelTimeTasks).cast());
static sReelTimeWindow_Tilemap: Table<CArray<u16, 220>> =
    Table((&raw const crate::data::slot_machine::sReelTimeWindow_Tilemap).cast());
static sSlotMachineMenu_Pal: Table<*mut u16> =
    Table((&raw const crate::data::slot_machine::sSlotMachineMenu_Pal).cast());
static sSlotMachineSpritePalettes: Table<CArray<SpritePalette, 9>> =
    Table((&raw const crate::data::slot_machine::sSlotMachineSpritePalettes).cast());
static sSlotMachineSpriteSheets: Table<CArray<SpriteSheet, 22>> =
    Table((&raw const crate::data::slot_machine::sSlotMachineSpriteSheets).cast());
static sSlotMatchFlags: Table<CArray<u16, 9>> =
    Table((&raw const crate::data::slot_machine::sSlotMatchFlags).cast());
static sSlotPayouts: Table<CArray<u16, 9>> =
    Table((&raw const crate::data::slot_machine::sSlotPayouts).cast());
static sSlotTasks: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 29>> =
    Table((&raw const crate::data::slot_machine::sSlotTasks).cast());
static sSpecialDrawOdds: Table<CArray<CArray<u8, 3>, 6>> =
    Table((&raw const crate::data::slot_machine::sSpecialDrawOdds).cast());
static sSpriteTemplate_BrokenReelTimeMachine: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_BrokenReelTimeMachine).cast());
static sSpriteTemplate_CoinNumber: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_CoinNumber).cast());
static sSpriteTemplate_PikaPowerBolt: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_PikaPowerBolt).cast());
static sSpriteTemplate_ReelBackground: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelBackground).cast());
static sSpriteTemplate_ReelSymbol: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelSymbol).cast());
static sSpriteTemplate_ReelTimeBolt: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelTimeBolt).cast());
static sSpriteTemplate_ReelTimeDuck: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelTimeDuck).cast());
static sSpriteTemplate_ReelTimeExplosion: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelTimeExplosion).cast());
static sSpriteTemplate_ReelTimeMachine: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelTimeMachine).cast());
static sSpriteTemplate_ReelTimeMachineAntennae: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelTimeMachineAntennae).cast());
static sSpriteTemplate_ReelTimeNumberGap: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelTimeNumberGap).cast());
static sSpriteTemplate_ReelTimeNumbers: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelTimeNumbers).cast());
static sSpriteTemplate_ReelTimePikachu: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelTimePikachu).cast());
static sSpriteTemplate_ReelTimePikachuAura: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelTimePikachuAura).cast());
static sSpriteTemplate_ReelTimeShadow: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelTimeShadow).cast());
static sSpriteTemplate_ReelTimeSmoke: Table<SpriteTemplate> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplate_ReelTimeSmoke).cast());
static sSpriteTemplates_DigitalDisplay: Table<CArray<*mut SpriteTemplate, 26>> =
    Table((&raw const crate::data::slot_machine::sSpriteTemplates_DigitalDisplay).cast());
static sSubspriteTable_BrokenReelTimeMachine: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::slot_machine::sSubspriteTable_BrokenReelTimeMachine).cast());
static sSubspriteTable_ReelBackground: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::slot_machine::sSubspriteTable_ReelBackground).cast());
static sSubspriteTable_ReelTimeMachine: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::slot_machine::sSubspriteTable_ReelTimeMachine).cast());
static sSubspriteTable_ReelTimeMachineAntennae: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::slot_machine::sSubspriteTable_ReelTimeMachineAntennae).cast());
static sSubspriteTable_ReelTimeNumberGap: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::slot_machine::sSubspriteTable_ReelTimeNumberGap).cast());
static sSubspriteTable_ReelTimeShadow: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::slot_machine::sSubspriteTable_ReelTimeShadow).cast());
static sSubspriteTables_DigitalDisplay: Table<CArray<*mut SubspriteTable, 26>> =
    Table((&raw const crate::data::slot_machine::sSubspriteTables_DigitalDisplay).cast());
static sSymbolToMatch: Table<CArray<u8, 7>> =
    Table((&raw const crate::data::slot_machine::sSymbolToMatch).cast());
static sUnkPalette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::slot_machine::sUnkPalette).cast());
static sWindowTemplate_InfoBox: Table<WindowTemplate> =
    Table((&raw const crate::data::slot_machine::sWindowTemplate_InfoBox).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::slot_machine::sWindowTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMenuGfx: *mut u16 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSelectedPikaPowerTile: *mut u16 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReelOverlay_Tilemap: *mut u16 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDigitalDisplayGfxPtr: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReelTimeGfxPtr: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReelButtonPress_Tilemap: *mut u16 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReelBackground_Gfx: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_ReelTimePikachu: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_ReelTimeMachineAntennae: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_ReelTimeMachine: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_BrokenReelTimeMachine: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Reel: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Time: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Insert: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Stop: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Win: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Lose: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Bonus: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Big: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Reg: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_AButton: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Smoke: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Number: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_Pokeball: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sImageTable_DigitalDisplay_DPad: *mut SpriteFrameImage = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReelBackgroundSpriteSheet: *mut SpriteSheet = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSlotMachineSpritesheetsPtr: *mut SpriteSheet = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSlotMachine: *mut SlotMachine = null_mut();
pub(crate) static mut sImageTables_DigitalDisplay: CArray<*mut SpriteFrameImage, 26> =
    unsafe { zeroed() };

unsafe extern "C" {
    static mut gMain: Main;
    static mut gOamLimit: u8;
    static mut gPaletteFade: PaletteFadeControl;
    static gSlotMachineDigitalDisplay_Gfx: CArray<u32, 0>;
    static gSlotMachineInfoBox_Tilemap: CArray<u16, 0>;
    static gSlotMachineMenu_Gfx: CArray<u32, 0>;
    static gSlotMachineMenu_Pal: CArray<u16, 0>;
    static gSlotMachineMenu_Tilemap: CArray<u16, 0>;
    static mut gSpriteCoordOffsetX: i16;
    static mut gSpriteCoordOffsetY: i16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gText_QuitTheGame: CArray<u8, 0>;
    static gText_ReelTimeHelp: CArray<u8, 0>;
    static gText_YouDontHaveThreeCoins: CArray<u8, 0>;
    static gText_YouveGot9999Coins: CArray<u8, 0>;
    static gText_YouveRunOutOfCoins: CArray<u8, 0>;
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
    fn AlertTVThatPlayerPlayedSlotMachine(a0: u16);
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateInvisibleSprite(a0: Option<unsafe extern "C" fn(*mut Sprite)>) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenuParameterized(a0: u8, a1: u8, a2: u16, a3: u16, a4: u8, a5: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn EnableInterrupts(a0: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeOamMatrix(a0: u8);
    fn GetCoins() -> u16;
    fn GetCurrentMapMusic() -> u16;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn HideBg(a0: u8);
    fn IncrementDailySlotsUses();
    fn IncrementGameStat(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitSpriteAffineAnim(a0: *mut Sprite);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsFanfareTaskInactive() -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn LoadBgTilemap(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LoadSpriteSheets(a0: *mut SpriteSheet);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LoadWordFromTwoHalfwords(a0: *mut u16, a1: *mut u32);
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MultiplyInvertedPaletteRGBComponents(a0: u16, a1: u8, a2: u8, a3: u8);
    fn MultiplyPaletteRGBComponents(a0: u16, a1: u8, a2: u8, a3: u8);
    fn PlayFanfare(a0: u16);
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn SetCoins(a0: u16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetSpriteSheetFrameTileNum(a0: *mut Sprite);
    fn SetSubspriteTables(a0: *mut Sprite, a1: *mut SubspriteTable);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8);
    fn StopMapMusic();
    fn StoreWordInTwoHalfwords(a0: *mut u16, a1: u32);
    fn TransferPlttBuffer();
    fn TryPutFindThatGamerOnAir(a0: u16);
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn Task_FadeToSlotMachine(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                SetMainCallback2(Some(CB2_SlotMachineSetup));
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlaySlotMachine(
    machineId: u8,
    exitCallback: Option<unsafe extern "C" fn()>,
) {
    let mut taskId: u8 = 0;
    sSlotMachine = AllocZeroed(104) as *mut SlotMachine;
    PlaySlotMachine_Internal(machineId, exitCallback);
    taskId = CreateTask(Some(Task_FadeToSlotMachine), 0);
    gTasks[taskId].data[0] = 0;
}
pub(crate) unsafe extern "C" fn CB2_SlotMachineSetup() {
    match gMain.state {
        0 => {
            SlotMachineSetup_InitBgsWindows();
            InitSlotMachine();
            gMain.state += 1;
        }
        1 => {
            SlotMachineSetup_InitVRAM();
            gMain.state += 1;
        }
        2 => {
            SlotMachineSetup_InitOAM();
            SlotMachineSetup_InitGpuRegs();
            gMain.state += 1;
        }
        3 => {
            SlotMachineSetup_InitPalsSpritesTasks();
            gMain.state += 1;
        }
        4 => {
            SlotMachineSetup_InitTilemaps();
            gMain.state += 1;
        }
        5 => {
            SlotMachineSetup_LoadGfxAndTilemaps();
            gMain.state += 1;
        }
        6 => {
            SlotMachineSetup_InitVBlank();
            gMain.state += 1;
        }
        7 => {
            BeginNormalPaletteFade(0xffffffff, 0, 0x10, 0, 0);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            gMain.state += 1;
        }
        8 => {
            AllocDigitalDisplayGfx();
            gMain.state += 1;
        }
        9 => {
            SetDigitalDisplayImagePtrs();
            gMain.state += 1;
        }
        10 => {
            CreateSlotMachineSprites();
            CreateGameplayTasks();
            gMain.state += 1;
        }
        11 => {
            SetMainCallback2(Some(CB2_SlotMachine));
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CB2_SlotMachine() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn SlotMachine_VBlankCB() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    SetGpuReg(REG_OFFSET_WIN0H, (*sSlotMachine).win0h);
    SetGpuReg(REG_OFFSET_WIN0V, (*sSlotMachine).win0v);
    SetGpuReg(REG_OFFSET_WININ, (*sSlotMachine).winIn);
    SetGpuReg(REG_OFFSET_WINOUT, (*sSlotMachine).winOut);
}
pub(crate) unsafe extern "C" fn PlaySlotMachine_Internal(
    machineId: u8,
    exitCallback: Option<unsafe extern "C" fn()>,
) {
    let mut task: *mut Task = &raw mut gTasks[CreateTask(Some(SlotMachineDummyTask), 0xFF)];
    (*task).data[0] = machineId as i16;
    StoreWordInTwoHalfwords(
        &raw mut (*task).data[1] as *mut u16,
        core::mem::transmute::<_, usize>(exitCallback) as i32 as u32,
    );
}
pub(crate) unsafe extern "C" fn SlotMachine_InitFromTask() {
    let mut task: *mut Task = &raw mut gTasks[FindTaskIdByFunc(Some(SlotMachineDummyTask))];
    (*sSlotMachine).machineId = (*task).data[0] as u8;
    LoadWordFromTwoHalfwords(
        &raw mut (*task).data[1] as *mut u16,
        &raw mut (*sSlotMachine).prevMainCb as *mut u32,
    );
}
pub(crate) unsafe extern "C" fn SlotMachineDummyTask(taskId: u8) {}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitBgsWindows() {
    SetVBlankCallback(None);
    SetHBlankCallback(None);
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
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitVBlank() {
    SetVBlankCallback(Some(SlotMachine_VBlankCB));
    EnableInterrupts(INTR_FLAG_VBLANK);
    SetGpuReg(REG_OFFSET_DISPCNT, 12352);
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitVRAM() {
    {
        let mut _dest: *mut c_void = BG_VRAM as usize as *mut u16 as *mut c_void;
        let mut _size: u32 = BG_VRAM_SIZE;
        loop {
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000800);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
            _dest = (_dest as *mut u8).at(4096) as *mut c_void;
            _size -= 0x1000;
            if _size <= 0x1000 {
                {
                    {
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
                break;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitOAM() {
    {
        {
            let mut _dest: *mut u16 = OAM as i32 as usize as *mut u16;
            let mut _size: u32 = OAM_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitGpuRegs() {
    SetGpuReg(REG_OFFSET_BG0CNT, 0);
    SetGpuReg(REG_OFFSET_BG1CNT, 0);
    SetGpuReg(REG_OFFSET_BG2CNT, 0);
    SetGpuReg(REG_OFFSET_BG3CNT, 0);
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    SetGpuReg(REG_OFFSET_BG3HOFS, 0);
    SetGpuReg(REG_OFFSET_BG3VOFS, 0);
    SetGpuReg(REG_OFFSET_WININ, 63);
    SetGpuReg(REG_OFFSET_WINOUT, 63);
    SetGpuReg(REG_OFFSET_BLDCNT, 4168);
    SetGpuReg(REG_OFFSET_BLDALPHA, 2057);
}
pub(crate) unsafe extern "C" fn InitSlotMachine() {
    let mut i: u8 = 0;
    SlotMachine_InitFromTask();
    (*sSlotMachine).state = SLOTTASK_UNFADE;
    (*sSlotMachine).pikaPowerBolts = 0;
    (*sSlotMachine).luckyGame = Random() as u8 & 1;
    (*sSlotMachine).machineBias = 0;
    (*sSlotMachine).matches = 0;
    (*sSlotMachine).reelTimeSpinsLeft = 0;
    (*sSlotMachine).reelTimeSpinsUsed = 0;
    (*sSlotMachine).coins = GetCoins() as i16;
    (*sSlotMachine).payout = 0;
    (*sSlotMachine).netCoinLoss = 0;
    (*sSlotMachine).bet = 0;
    (*sSlotMachine).currentReel = LEFT_REEL as i16;
    (*sSlotMachine).reelSpeed = REEL_NORMAL_SPEED;
    (*sSlotMachine).win0h = DISPLAY_WIDTH;
    (*sSlotMachine).win0v = DISPLAY_HEIGHT;
    (*sSlotMachine).winIn = 63;
    (*sSlotMachine).winOut = 63;
    (*sSlotMachine).backupMapMusic = GetCurrentMapMusic();
    i = 0;
    while i < NUM_REELS {
        (*sSlotMachine).reelShockOffsets[i] = 0;
        (*sSlotMachine).reelPositions[i] = sInitialReelPositions[i][(*sSlotMachine).luckyGame] % 21;
        (*sSlotMachine).reelPixelOffsets[i] =
            REEL_HEIGHT - (*sSlotMachine).reelPositions[i] * REEL_SYMBOL_HEIGHT;
        (*sSlotMachine).reelPixelOffsets[i] = (*sSlotMachine).reelPixelOffsets[i] % 504;
        i += 1;
    }
    AlertTVThatPlayerPlayedSlotMachine(GetCoins());
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitPalsSpritesTasks() {
    ResetPaletteFade();
    ResetSpriteData();
    gOamLimit = 0x80;
    FreeAllSpritePalettes();
    ResetTasks();
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_InitTilemaps() {
    sSelectedPikaPowerTile = Alloc(8) as *mut u16;
    sReelOverlay_Tilemap = AllocZeroed(14) as *mut u16;
    sReelButtonPress_Tilemap = AllocZeroed(8) as *mut u16;
    *sReelOverlay_Tilemap = 0x2051;
    *sReelOverlay_Tilemap.at(1) = 0x2851;
    *sReelOverlay_Tilemap.at(2) = 0x2061;
    *sReelOverlay_Tilemap.at(3) = 0x2861;
    *sReelOverlay_Tilemap.at(4) = 0x20BE;
    *sReelOverlay_Tilemap.at(5) = 0x28BE;
    *sReelOverlay_Tilemap.at(6) = 0x20BF;
}
pub(crate) unsafe extern "C" fn SlotMachineSetup_LoadGfxAndTilemaps() {
    LoadMenuGfx();
    LoadMenuAndReelOverlayTilemaps();
    LoadSlotMachineGfx();
    LoadMessageBoxGfx(0, 0x200, 240);
    LoadUserWindowBorderGfx(0, 0x214, 224);
    PutWindowTilemap(WIN_MSG);
}
pub(crate) unsafe extern "C" fn CreateSlotMachineSprites() {
    CreateReelSymbolSprites();
    CreateCreditPayoutNumberSprites();
    CreateInvisibleFlashMatchLineSprites();
    CreateReelBackgroundSprite();
}
pub(crate) unsafe extern "C" fn CreateGameplayTasks() {
    CreatePikaPowerBoltTask();
    CreateReelTasks();
    CreateDigitalDisplayTask();
    CreateSlotMachineTasks();
}
pub(crate) unsafe extern "C" fn CreateSlotMachineTasks() {
    Task_SlotMachine(CreateTask(Some(Task_SlotMachine), 0));
}
pub(crate) unsafe extern "C" fn Task_SlotMachine(taskId: u8) {
    while sSlotTasks[(*sSlotMachine).state].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn SlotTask_UnfadeScreen(task: *mut Task) -> u8 {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    LoadPikaPowerMeter((*sSlotMachine).pikaPowerBolts);
    (*sSlotMachine).state += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_WaitUnfade(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        (*sSlotMachine).state += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_ReadyNewSpin(task: *mut Task) -> u8 {
    (*sSlotMachine).payout = 0;
    (*sSlotMachine).bet = 0;
    (*sSlotMachine).currentReel = LEFT_REEL as i16;
    (*sSlotMachine).machineBias &= 192;
    (*sSlotMachine).state = SLOTTASK_ASK_INSERT_BET;
    if (*sSlotMachine).coins <= 0 {
        (*sSlotMachine).state = SLOTTASK_MSG_NO_MORE_COINS;
    } else if (*sSlotMachine).reelTimeSpinsLeft != 0 {
        (*sSlotMachine).state = SLOTTASK_READY_NEW_RT_SPIN;
        CreateDigitalDisplayScene(DIG_DISPLAY_REEL_TIME);
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn SlotTask_ReadyNewReelTimeSpin(task: *mut Task) -> u8 {
    if IsDigitalDisplayAnimFinished() != 0 {
        (*sSlotMachine).state = SLOTTASK_ASK_INSERT_BET;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_AskInsertBet(task: *mut Task) -> u8 {
    CreateDigitalDisplayScene(DIG_DISPLAY_INSERT_BET);
    (*sSlotMachine).state = SLOTTASK_BET_INPUT;
    if (*sSlotMachine).coins >= MAX_COINS {
        (*sSlotMachine).state = SLOTTASK_MSG_MAX_COINS;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn SlotTask_HandleBetInput(task: *mut Task) -> u8 {
    let mut i: i16 = 0;
    if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
        OpenInfoBox(DIG_DISPLAY_INSERT_BET);
        (*sSlotMachine).state = SLOTTASK_WAIT_INFO_BOX;
    } else if gMain.newKeys as i32 & R_BUTTON != 0 {
        if (*sSlotMachine).coins as i32 - (MAX_BET as i32 - (*sSlotMachine).bet as i32) >= 0 {
            i = (*sSlotMachine).bet;
            while i < MAX_BET {
                LightenBetTiles(i as u8);
                i += 1;
            }
            (*sSlotMachine).coins -= MAX_BET - (*sSlotMachine).bet;
            (*sSlotMachine).bet = MAX_BET;
            (*sSlotMachine).state = SLOTTASK_START_SPIN;
            PlaySE(SE_SHOP);
        } else {
            (*sSlotMachine).state = SLOTTASK_MSG_NEED_3_COINS;
        }
    } else {
        if gMain.newKeys as i32 & DPAD_DOWN != 0 && (*sSlotMachine).coins != 0 {
            PlaySE(SE_SHOP);
            LightenBetTiles((*sSlotMachine).bet as u8);
            (*sSlotMachine).coins -= 1;
            (*sSlotMachine).bet += 1;
        }
        if (*sSlotMachine).bet >= MAX_BET
            || (*sSlotMachine).bet != 0 && gMain.newKeys as i32 & A_BUTTON != 0
        {
            (*sSlotMachine).state = SLOTTASK_START_SPIN;
        }
        if gMain.newKeys as i32 & B_BUTTON != 0 {
            (*sSlotMachine).state = SLOTTASK_ASK_QUIT;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_PrintMsg_Need3Coins(task: *mut Task) -> u8 {
    DrawDialogueFrame(WIN_MSG, FALSE);
    AddTextPrinterParameterized(
        WIN_MSG,
        FONT_NORMAL,
        gText_YouDontHaveThreeCoins.as_ptr().cast_mut(),
        0,
        1,
        0,
        None,
    );
    CopyWindowToVram(WIN_MSG, COPYWIN_FULL);
    (*sSlotMachine).state = SLOTTASK_WAIT_MSG_NEED_3_COINS;
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_WaitMsg_Need3Coins(task: *mut Task) -> u8 {
    if gMain.newKeys as i32 & 3 != 0 {
        ClearDialogWindowAndFrame(WIN_MSG, TRUE);
        (*sSlotMachine).state = SLOTTASK_BET_INPUT;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_WaitInfoBox(task: *mut Task) -> u8 {
    if IsInfoBoxClosed() != 0 {
        (*sSlotMachine).state = SLOTTASK_BET_INPUT;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_StartSpin(task: *mut Task) -> u8 {
    DrawMachineBias();
    DestroyDigitalDisplayScene();
    SpinSlotReel(LEFT_REEL);
    SpinSlotReel(MIDDLE_REEL);
    SpinSlotReel(RIGHT_REEL);
    IncrementDailySlotsUses();
    (*task).data[0] = 0;
    if (*sSlotMachine).machineBias as i32 & BIAS_REELTIME != 0 {
        BeginReelTime();
        (*sSlotMachine).state = SLOTTASK_START_RT_SPIN;
    } else {
        CreateDigitalDisplayScene(DIG_DISPLAY_STOP_REEL);
        (*sSlotMachine).state = SLOTTASK_RESET_BIAS_FAILURE;
    }
    (*sSlotMachine).reelSpeed = REEL_NORMAL_SPEED;
    if (*sSlotMachine).reelTimeSpinsLeft != 0 {
        (*sSlotMachine).reelSpeed = ReelTimeSpeed() as i16;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_StartReelTimeSpin(task: *mut Task) -> u8 {
    if IsReelTimeTaskDone() != 0 {
        CreateDigitalDisplayScene(DIG_DISPLAY_STOP_REEL);
        (*sSlotMachine).machineBias &= 223;
        (*sSlotMachine).state = SLOTTASK_RESET_BIAS_FAILURE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_ResetBiasFailure(task: *mut Task) -> u8 {
    if ({
        (*task).data[0] += 1;
        (*task).data[0]
    }) >= 30
    {
        ResetBiasFailure();
        (*sSlotMachine).state = SLOTTASK_WAIT_REEL_STOP;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_WaitReelStop(task: *mut Task) -> u8 {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_CONTEST_PLACE);
        StopSlotReel((*sSlotMachine).currentReel as u8);
        PressStopReelButton((*sSlotMachine).currentReel as u8);
        (*sSlotMachine).state = SLOTTASK_WAIT_ALL_REELS_STOP;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_WaitAllReelsStop(task: *mut Task) -> u8 {
    if IsSlotReelMoving((*sSlotMachine).currentReel as u8) == 0 {
        (*sSlotMachine).currentReel += 1;
        (*sSlotMachine).state = SLOTTASK_WAIT_REEL_STOP;
        if (*sSlotMachine).currentReel >= NUM_REELS as i16 {
            (*sSlotMachine).state = SLOTTASK_CHECK_MATCHES;
        }
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_CheckMatches(task: *mut Task) -> u8 {
    (*sSlotMachine).machineBias &= 192;
    CheckMatch();
    if (*sSlotMachine).reelTimeSpinsLeft != 0 {
        (*sSlotMachine).reelTimeSpinsLeft -= 1;
        (*sSlotMachine).reelTimeSpinsUsed += 1;
    }
    if (*sSlotMachine).matches != 0 {
        (*sSlotMachine).state = SLOTTASK_WAIT_PAYOUT;
        AwardPayout();
        FlashSlotMachineLights();
        if ({
            (*sSlotMachine).netCoinLoss -= (*sSlotMachine).payout;
            (*sSlotMachine).netCoinLoss
        }) < 0
        {
            (*sSlotMachine).netCoinLoss = 0;
        }
        if (*sSlotMachine).matches as i32 & 384 != 0 {
            PlayFanfare(MUS_SLOTS_JACKPOT);
            CreateDigitalDisplayScene(DIG_DISPLAY_BONUS_BIG);
        } else if (*sSlotMachine).matches as i32 & 64 != 0 {
            PlayFanfare(MUS_SLOTS_JACKPOT);
            CreateDigitalDisplayScene(DIG_DISPLAY_BONUS_REG);
        } else {
            PlayFanfare(MUS_SLOTS_WIN);
            CreateDigitalDisplayScene(DIG_DISPLAY_WIN);
        }
        if (*sSlotMachine).matches as i32 & 448 != 0 {
            (*sSlotMachine).machineBias &= 63;
            if (*sSlotMachine).matches as i32 & 384 != 0 {
                (*sSlotMachine).reelTimeSpinsLeft = 0;
                (*sSlotMachine).reelTimeSpinsUsed = 0;
                (*sSlotMachine).luckyGame = FALSE;
                if (*sSlotMachine).matches as i32 & 256 != 0 {
                    (*sSlotMachine).luckyGame = TRUE;
                }
            }
        }
        if (*sSlotMachine).matches as i32 & 32 != 0 && (*sSlotMachine).pikaPowerBolts < 16 {
            (*sSlotMachine).pikaPowerBolts += 1;
            AddPikaPowerBolt((*sSlotMachine).pikaPowerBolts);
        }
    } else {
        CreateDigitalDisplayScene(DIG_DISPLAY_LOSE);
        (*sSlotMachine).state = SLOTTASK_NO_MATCHES;
        if ({
            (*sSlotMachine).netCoinLoss += (*sSlotMachine).bet;
            (*sSlotMachine).netCoinLoss
        }) > MAX_COINS
        {
            (*sSlotMachine).netCoinLoss = MAX_COINS;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_WaitPayout(task: *mut Task) -> u8 {
    if IsFinalTask_Task_Payout() != 0 {
        (*sSlotMachine).state = SLOTTASK_END_PAYOUT;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_EndPayout(task: *mut Task) -> u8 {
    if TryStopSlotMachineLights() != 0 {
        (*sSlotMachine).state = SLOTTASK_RESET_BET_TILES;
        if (*sSlotMachine).matches as i32 & 384 != 0 {
            IncrementGameStat(GAME_STAT_SLOT_JACKPOTS);
        }
        if (*sSlotMachine).matches as i32 & 4 != 0 {
            (*sSlotMachine).currentReel = LEFT_REEL as i16;
            (*sSlotMachine).state = SLOTTASK_START_SPIN;
        }
        if (*sSlotMachine).matches as i32 & 32 != 0 {
            (*sSlotMachine).state = SLOTTASK_MATCHED_POWER;
        }
        if (*sSlotMachine).reelTimeSpinsLeft != 0 && (*sSlotMachine).matches as i32 & 4 != 0 {
            CreateDigitalDisplayScene(DIG_DISPLAY_REEL_TIME);
            (*sSlotMachine).state = SLOTTASK_WAIT_RT_ANIM;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_MatchedPower(task: *mut Task) -> u8 {
    if IsPikaPowerBoltAnimating() == 0 {
        (*sSlotMachine).state = SLOTTASK_RESET_BET_TILES;
        if (*sSlotMachine).matches as i32 & 4 != 0 {
            (*sSlotMachine).state = SLOTTASK_START_SPIN;
            if (*sSlotMachine).reelTimeSpinsLeft != 0 {
                CreateDigitalDisplayScene(DIG_DISPLAY_REEL_TIME);
                (*sSlotMachine).state = SLOTTASK_WAIT_RT_ANIM;
            }
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_WaitReelTimeAnim(task: *mut Task) -> u8 {
    if IsDigitalDisplayAnimFinished() != 0 {
        (*sSlotMachine).state = SLOTTASK_RESET_BET_TILES;
        if (*sSlotMachine).matches as i32 & 4 != 0 {
            (*sSlotMachine).state = SLOTTASK_START_SPIN;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_ResetBetTiles(task: *mut Task) -> u8 {
    DarkenBetTiles(0);
    DarkenBetTiles(1);
    DarkenBetTiles(2);
    (*sSlotMachine).state = SLOTTASK_READY_NEW_SPIN;
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_NoMatches(task: *mut Task) -> u8 {
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) > 64
    {
        (*task).data[1] = 0;
        (*sSlotMachine).state = SLOTTASK_RESET_BET_TILES;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_AskQuit(task: *mut Task) -> u8 {
    DrawDialogueFrame(WIN_MSG, FALSE);
    AddTextPrinterParameterized(
        WIN_MSG,
        FONT_NORMAL,
        gText_QuitTheGame.as_ptr().cast_mut(),
        0,
        1,
        0,
        None,
    );
    CopyWindowToVram(WIN_MSG, COPYWIN_FULL);
    CreateYesNoMenuParameterized(0x15, 7, 0x214, 0x180, 0xE, 0xF);
    (*sSlotMachine).state = SLOTTASK_HANDLE_QUIT_INPUT;
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_HandleQuitInput(task: *mut Task) -> u8 {
    let mut input: i8 = Menu_ProcessInputNoWrapClearOnChoose();
    if input == 0 {
        ClearDialogWindowAndFrame(WIN_MSG, TRUE);
        DarkenBetTiles(0);
        DarkenBetTiles(1);
        DarkenBetTiles(2);
        (*sSlotMachine).coins += (*sSlotMachine).bet;
        (*sSlotMachine).state = SLOTTASK_END;
    } else if input == 1 || input == MENU_B_PRESSED {
        ClearDialogWindowAndFrame(WIN_MSG, TRUE);
        (*sSlotMachine).state = SLOTTASK_BET_INPUT;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_PrintMsg_MaxCoins(task: *mut Task) -> u8 {
    DrawDialogueFrame(WIN_MSG, FALSE);
    AddTextPrinterParameterized(
        WIN_MSG,
        FONT_NORMAL,
        gText_YouveGot9999Coins.as_ptr().cast_mut(),
        0,
        1,
        0,
        None,
    );
    CopyWindowToVram(WIN_MSG, COPYWIN_FULL);
    (*sSlotMachine).state = SLOTTASK_WAIT_MSG_MAX_COINS;
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_WaitMsg_MaxCoins(task: *mut Task) -> u8 {
    if gMain.newKeys as i32 & 3 != 0 {
        ClearDialogWindowAndFrame(WIN_MSG, TRUE);
        (*sSlotMachine).state = SLOTTASK_BET_INPUT;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_PrintMsg_NoMoreCoins(task: *mut Task) -> u8 {
    DrawDialogueFrame(WIN_MSG, FALSE);
    AddTextPrinterParameterized(
        WIN_MSG,
        FONT_NORMAL,
        gText_YouveRunOutOfCoins.as_ptr().cast_mut(),
        0,
        1,
        0,
        None,
    );
    CopyWindowToVram(WIN_MSG, COPYWIN_FULL);
    (*sSlotMachine).state = SLOTTASK_WAIT_MSG_NO_MORE_COINS;
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_WaitMsg_NoMoreCoins(task: *mut Task) -> u8 {
    if gMain.newKeys as i32 & 3 != 0 {
        ClearDialogWindowAndFrame(WIN_MSG, TRUE);
        (*sSlotMachine).state = SLOTTASK_END;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_EndGame(task: *mut Task) -> u8 {
    SetCoins((*sSlotMachine).coins as u16);
    TryPutFindThatGamerOnAir(GetCoins());
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    (*sSlotMachine).state += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn SlotTask_FreeDataStructures(task: *mut Task) -> u8 {
    if gPaletteFade.active() == 0 {
        SetMainCallback2((*sSlotMachine).prevMainCb);
        Free(sImageTable_DigitalDisplay_Reel as *mut c_void);
        sImageTable_DigitalDisplay_Reel = null_mut();
        Free(sImageTable_DigitalDisplay_Time as *mut c_void);
        sImageTable_DigitalDisplay_Time = null_mut();
        Free(sImageTable_DigitalDisplay_Insert as *mut c_void);
        sImageTable_DigitalDisplay_Insert = null_mut();
        Free(sImageTable_DigitalDisplay_Stop as *mut c_void);
        sImageTable_DigitalDisplay_Stop = null_mut();
        Free(sImageTable_DigitalDisplay_Win as *mut c_void);
        sImageTable_DigitalDisplay_Win = null_mut();
        Free(sImageTable_DigitalDisplay_Lose as *mut c_void);
        sImageTable_DigitalDisplay_Lose = null_mut();
        Free(sImageTable_DigitalDisplay_Bonus as *mut c_void);
        sImageTable_DigitalDisplay_Bonus = null_mut();
        Free(sImageTable_DigitalDisplay_Big as *mut c_void);
        sImageTable_DigitalDisplay_Big = null_mut();
        Free(sImageTable_DigitalDisplay_Reg as *mut c_void);
        sImageTable_DigitalDisplay_Reg = null_mut();
        Free(sImageTable_DigitalDisplay_AButton as *mut c_void);
        sImageTable_DigitalDisplay_AButton = null_mut();
        Free(sImageTable_DigitalDisplay_Smoke as *mut c_void);
        sImageTable_DigitalDisplay_Smoke = null_mut();
        Free(sImageTable_DigitalDisplay_Number as *mut c_void);
        sImageTable_DigitalDisplay_Number = null_mut();
        Free(sImageTable_DigitalDisplay_Pokeball as *mut c_void);
        sImageTable_DigitalDisplay_Pokeball = null_mut();
        Free(sImageTable_DigitalDisplay_DPad as *mut c_void);
        sImageTable_DigitalDisplay_DPad = null_mut();
        if !sImageTable_ReelTimePikachu.is_null() {
            Free(sImageTable_ReelTimePikachu as *mut c_void);
            sImageTable_ReelTimePikachu = null_mut();
        }
        if !sImageTable_ReelTimeMachineAntennae.is_null() {
            Free(sImageTable_ReelTimeMachineAntennae as *mut c_void);
            sImageTable_ReelTimeMachineAntennae = null_mut();
        }
        if !sImageTable_ReelTimeMachine.is_null() {
            Free(sImageTable_ReelTimeMachine as *mut c_void);
            sImageTable_ReelTimeMachine = null_mut();
        }
        if !sImageTable_BrokenReelTimeMachine.is_null() {
            Free(sImageTable_BrokenReelTimeMachine as *mut c_void);
            sImageTable_BrokenReelTimeMachine = null_mut();
        }
        Free(sMenuGfx as *mut c_void);
        sMenuGfx = null_mut();
        Free(sSelectedPikaPowerTile as *mut c_void);
        sSelectedPikaPowerTile = null_mut();
        Free(sReelOverlay_Tilemap as *mut c_void);
        sReelOverlay_Tilemap = null_mut();
        Free(sDigitalDisplayGfxPtr as *mut c_void);
        sDigitalDisplayGfxPtr = null_mut();
        Free(sReelTimeGfxPtr as *mut c_void);
        sReelTimeGfxPtr = null_mut();
        Free(sReelButtonPress_Tilemap as *mut c_void);
        sReelButtonPress_Tilemap = null_mut();
        Free(sReelBackground_Gfx as *mut c_void);
        sReelBackground_Gfx = null_mut();
        Free(sReelBackgroundSpriteSheet as *mut c_void);
        sReelBackgroundSpriteSheet = null_mut();
        Free(sSlotMachineSpritesheetsPtr as *mut c_void);
        sSlotMachineSpritesheetsPtr = null_mut();
        Free(sSlotMachine as *mut c_void);
        sSlotMachine = null_mut();
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DrawMachineBias() {
    let mut whichBias: u8 = 0;
    if (*sSlotMachine).reelTimeSpinsLeft == 0 {
        if (*sSlotMachine).machineBias as i32 & 192 == 0 {
            if ShouldTrySpecialBias() != 0 {
                whichBias = TrySelectBias_Special();
                if whichBias != 3 {
                    (*sSlotMachine).machineBias |= sBiasesSpecial[whichBias] as u8;
                    if whichBias != 1 {
                        return;
                    }
                }
            }
            whichBias = TrySelectBias_Regular();
            if whichBias != 5 {
                (*sSlotMachine).machineBias |= sBiasesRegular[whichBias] as u8;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResetBiasFailure() {
    (*sSlotMachine).didNotFailBias = FALSE;
    if (*sSlotMachine).machineBias != 0 {
        (*sSlotMachine).didNotFailBias = TRUE;
    }
}
pub(crate) unsafe extern "C" fn GetBiasSymbol(mut machineBias: u8) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < 8 {
        if machineBias as i32 & 1 != 0 {
            return sBiasSymbols[i];
        }
        machineBias >>= 1;
        i += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn ShouldTrySpecialBias() -> u8 {
    let mut rval: u8 = Random() as u8;
    if sSpecialDrawOdds[(*sSlotMachine).machineId][(*sSlotMachine).bet as i32 - 1] > rval {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn TrySelectBias_Special() -> u8 {
    let mut whichBias: i16 = 0;
    whichBias = 0;
    while whichBias < 3 {
        let mut rval: i16 = Random() as i16 & 0xff;
        let mut value: i16 =
            sBiasProbabilities_Special[whichBias][(*sSlotMachine).machineId] as i16;
        if value > rval {
            break;
        }
        whichBias += 1;
    }
    return whichBias as u8;
}
pub(crate) unsafe extern "C" fn TrySelectBias_Regular() -> u8 {
    let mut whichBias: i16 = 0;
    whichBias = 0;
    while whichBias < 5 {
        let mut rval: i16 = Random() as i16 & 0xff;
        let mut value: i16 =
            sBiasProbabilities_Regular[whichBias][(*sSlotMachine).machineId] as i16;
        if whichBias == 0 && (*sSlotMachine).luckyGame == TRUE {
            value += 10;
            if value > 0x100 {
                value = 0x100;
            }
        } else if whichBias == 4 && (*sSlotMachine).luckyGame == TRUE {
            value -= 10;
            if value < 0 {
                value = 0;
            }
        }
        if value > rval {
            break;
        }
        whichBias += 1;
    }
    return whichBias as u8;
}
pub(crate) unsafe extern "C" fn GetReelTimeSpinProbability(spins: u8) -> u8 {
    if (*sSlotMachine).luckyGame == FALSE {
        return sReelTimeProbabilities_NormalGame[spins][(*sSlotMachine).pikaPowerBolts];
    } else {
        return sReelTimeProbabilities_LuckyGame[spins][(*sSlotMachine).pikaPowerBolts];
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetReelTimeDraw() {
    let mut rval: u8 = 0;
    let mut spins: i16 = 0;
    (*sSlotMachine).reelTimeDraw = 0;
    rval = Random() as u8;
    if rval < GetReelTimeSpinProbability(0) {
        return;
    }
    spins = 5;
    while spins > 0 {
        rval = Random() as u8;
        if rval < GetReelTimeSpinProbability(spins as u8) {
            break;
        }
        spins -= 1;
    }
    (*sSlotMachine).reelTimeDraw = spins as u8;
}
pub(crate) unsafe extern "C" fn ShouldReelTimeMachineExplode(check: u16) -> u8 {
    let mut rval: u16 = Random() & 0xff;
    if rval < sReelTimeExplodeProbability[check] {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ReelTimeSpeed() -> u16 {
    let mut i: u8 = 0;
    let mut rval: u8 = 0;
    let mut value: u8 = 0;
    if (*sSlotMachine).netCoinLoss >= 300 {
        i = 4;
    } else if (*sSlotMachine).netCoinLoss >= 250 {
        i = 3;
    } else if (*sSlotMachine).netCoinLoss >= 200 {
        i = 2;
    } else if (*sSlotMachine).netCoinLoss >= 150 {
        i = 1;
    }
    rval = (Random() as i32 % 100) as u8;
    value = sReelTimeSpeed_Probabilities[i][0] as u8;
    if rval < value {
        return REEL_HALF_SPEED;
    }
    rval = (Random() as i32 % 100) as u8;
    value = sReelTimeSpeed_Probabilities[i][1] as u8
        + sQuarterSpeed_ProbabilityBoost[(*sSlotMachine).reelTimeSpinsUsed] as u8;
    if rval < value {
        return REEL_QUARTER_SPEED;
    }
    return REEL_NORMAL_SPEED as u16;
}
pub(crate) unsafe extern "C" fn CheckMatch() {
    (*sSlotMachine).matches = 0;
    CheckMatch_CenterRow();
    if (*sSlotMachine).bet > 1 {
        CheckMatch_TopAndBottom();
    }
    if (*sSlotMachine).bet > 2 {
        CheckMatch_Diagonals();
    }
}
pub(crate) unsafe extern "C" fn CheckMatch_CenterRow() {
    let mut sym1: u8 = 0;
    let mut sym2: u8 = 0;
    let mut sym3: u8 = 0;
    let mut r#match: u8 = 0;
    sym1 = GetSymbolAtRest(LEFT_REEL, 2);
    sym2 = GetSymbolAtRest(MIDDLE_REEL, 2);
    sym3 = GetSymbolAtRest(RIGHT_REEL, 2);
    r#match = GetMatchFromSymbols(sym1, sym2, sym3);
    if r#match != MATCH_NONE {
        (*sSlotMachine).payout += sSlotPayouts[r#match] as i16;
        (*sSlotMachine).matches |= sSlotMatchFlags[r#match];
        FlashMatchLine(MATCH_MIDDLE_ROW);
    }
}
pub(crate) unsafe extern "C" fn CheckMatch_TopAndBottom() {
    let mut sym1: u8 = 0;
    let mut sym2: u8 = 0;
    let mut sym3: u8 = 0;
    let mut r#match: u8 = 0;
    sym1 = GetSymbolAtRest(LEFT_REEL, 1);
    sym2 = GetSymbolAtRest(MIDDLE_REEL, 1);
    sym3 = GetSymbolAtRest(RIGHT_REEL, 1);
    r#match = GetMatchFromSymbols(sym1, sym2, sym3);
    if r#match != MATCH_NONE {
        if r#match == MATCH_CHERRY {
            r#match = MATCH_TOPBOT_CHERRY;
        }
        (*sSlotMachine).payout += sSlotPayouts[r#match] as i16;
        (*sSlotMachine).matches |= sSlotMatchFlags[r#match];
        FlashMatchLine(MATCH_TOP_ROW);
    }
    sym1 = GetSymbolAtRest(LEFT_REEL, 3);
    sym2 = GetSymbolAtRest(MIDDLE_REEL, 3);
    sym3 = GetSymbolAtRest(RIGHT_REEL, 3);
    r#match = GetMatchFromSymbols(sym1, sym2, sym3);
    if r#match != MATCH_NONE {
        if r#match == MATCH_CHERRY {
            r#match = MATCH_TOPBOT_CHERRY;
        }
        (*sSlotMachine).payout += sSlotPayouts[r#match] as i16;
        (*sSlotMachine).matches |= sSlotMatchFlags[r#match];
        FlashMatchLine(MATCH_BOTTOM_ROW);
    }
}
pub(crate) unsafe extern "C" fn CheckMatch_Diagonals() {
    let mut sym1: u8 = 0;
    let mut sym2: u8 = 0;
    let mut sym3: u8 = 0;
    let mut r#match: u8 = 0;
    sym1 = GetSymbolAtRest(LEFT_REEL, 1);
    sym2 = GetSymbolAtRest(MIDDLE_REEL, 2);
    sym3 = GetSymbolAtRest(RIGHT_REEL, 3);
    r#match = GetMatchFromSymbols(sym1, sym2, sym3);
    if r#match != MATCH_NONE {
        if r#match != MATCH_CHERRY {
            (*sSlotMachine).payout += sSlotPayouts[r#match] as i16;
            (*sSlotMachine).matches |= sSlotMatchFlags[r#match];
        }
        FlashMatchLine(MATCH_NWSE_DIAG);
    }
    sym1 = GetSymbolAtRest(LEFT_REEL, 3);
    sym2 = GetSymbolAtRest(MIDDLE_REEL, 2);
    sym3 = GetSymbolAtRest(RIGHT_REEL, 1);
    r#match = GetMatchFromSymbols(sym1, sym2, sym3);
    if r#match != MATCH_NONE {
        if r#match != MATCH_CHERRY {
            (*sSlotMachine).payout += sSlotPayouts[r#match] as i16;
            (*sSlotMachine).matches |= sSlotMatchFlags[r#match];
        }
        FlashMatchLine(MATCH_NESW_DIAG);
    }
}
pub(crate) unsafe extern "C" fn GetMatchFromSymbols(sym1: u8, sym2: u8, sym3: u8) -> u8 {
    if sym1 == sym2 && sym1 == sym3 {
        return sSymbolToMatch[sym1];
    }
    if sym1 == SYMBOL_7_RED && sym2 == SYMBOL_7_RED && sym3 == SYMBOL_7_BLUE {
        return MATCH_MIXED_7;
    }
    if sym1 == SYMBOL_7_BLUE && sym2 == SYMBOL_7_BLUE && sym3 == SYMBOL_7_RED {
        return MATCH_MIXED_7;
    }
    if sym1 == SYMBOL_CHERRY {
        return MATCH_CHERRY;
    }
    return MATCH_NONE;
}
pub(crate) unsafe extern "C" fn AwardPayout() {
    Task_Payout(CreateTask(Some(Task_Payout), 4));
}
pub(crate) unsafe extern "C" fn IsFinalTask_Task_Payout() -> u8 {
    if FindTaskIdByFunc(Some(Task_Payout)) == TAIL_SENTINEL {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Task_Payout(taskId: u8) {
    while sPayoutTasks[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn PayoutTask_Init(task: *mut Task) -> u8 {
    if IsMatchLineDoneFlashingBeforePayout() != 0 {
        (*task).data[0] += 1;
        if (*sSlotMachine).payout == 0 {
            (*task).data[0] = PAYOUT_TASK_FREE;
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn PayoutTask_GivePayout(task: *mut Task) -> u8 {
    if ({
        let t1 = (*task).data[1];
        (*task).data[1] -= 1;
        t1
    }) == 0
    {
        if IsFanfareTaskInactive() != 0 {
            PlaySE(SE_PIN);
        }
        (*sSlotMachine).payout -= 1;
        if (*sSlotMachine).coins < MAX_COINS {
            (*sSlotMachine).coins += 1;
        }
        (*task).data[1] = 8;
        if gMain.heldKeys as i32 & A_BUTTON != 0 {
            (*task).data[1] = 4;
        }
    }
    if IsFanfareTaskInactive() != 0 && gMain.newKeys as i32 & START_BUTTON != 0 {
        PlaySE(SE_PIN);
        (*sSlotMachine).coins += (*sSlotMachine).payout;
        if (*sSlotMachine).coins > MAX_COINS {
            (*sSlotMachine).coins = MAX_COINS;
        }
        (*sSlotMachine).payout = 0;
    }
    if (*sSlotMachine).payout == 0 {
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn PayoutTask_Free(task: *mut Task) -> u8 {
    if TryStopMatchLinesFlashing() != 0 {
        DestroyTask(FindTaskIdByFunc(Some(Task_Payout)));
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn GetSymbolAtRest(reel: u8, offset: i16) -> u8 {
    let mut pos: i16 = (((*sSlotMachine).reelPositions[reel] as i32 + offset as i32) % 21) as i16;
    if pos < 0 {
        pos += SYMBOLS_PER_REEL;
    }
    return sReelSymbols[reel][pos];
}
pub(crate) unsafe extern "C" fn GetSymbol(reel: u8, offset: i16) -> u8 {
    let mut inc: i16 = 0;
    let mut pixelOffset: i16 = (*sSlotMachine).reelPixelOffsets[reel] % 24;
    if pixelOffset != 0 {
        inc = -1;
    }
    return GetSymbolAtRest(reel, offset + inc);
}
pub(crate) unsafe extern "C" fn GetReelTimeSymbol(offset: i16) -> u8 {
    let mut newPosition: i16 =
        (((*sSlotMachine).reeltimePosition as i32 + offset as i32) % 6) as i16;
    if newPosition < 0 {
        newPosition += REELTIME_SYMBOLS;
    }
    return sReelTimeSymbols[newPosition];
}
pub(crate) unsafe extern "C" fn AdvanceSlotReel(reelIndex: u8, value: i16) {
    (*sSlotMachine).reelPixelOffsets[reelIndex] += value;
    (*sSlotMachine).reelPixelOffsets[reelIndex] = (*sSlotMachine).reelPixelOffsets[reelIndex] % 504;
    (*sSlotMachine).reelPositions[reelIndex] =
        SYMBOLS_PER_REEL - (*sSlotMachine).reelPixelOffsets[reelIndex] / 24;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AdvanceSlotReelToNextSymbol(reelIndex: u8, mut value: i16) -> i16 {
    let mut offset: i16 = (*sSlotMachine).reelPixelOffsets[reelIndex] % 24;
    if offset != 0 {
        if offset < value {
            value = offset;
        }
        AdvanceSlotReel(reelIndex, value);
        offset = (*sSlotMachine).reelPixelOffsets[reelIndex] % 24;
    }
    return offset;
}
pub(crate) unsafe extern "C" fn AdvanceReeltimeReel(value: i16) {
    (*sSlotMachine).reeltimePixelOffset += value;
    (*sSlotMachine).reeltimePixelOffset = (*sSlotMachine).reeltimePixelOffset % 120;
    (*sSlotMachine).reeltimePosition = REELTIME_SYMBOLS - (*sSlotMachine).reeltimePixelOffset / 20;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AdvanceReeltimeReelToNextSymbol(mut value: i16) -> i16 {
    let mut offset: i16 = (*sSlotMachine).reeltimePixelOffset % 20;
    if offset != 0 {
        if offset < value {
            value = offset;
        }
        AdvanceReeltimeReel(value);
        offset = (*sSlotMachine).reeltimePixelOffset % 20;
    }
    return offset;
}
pub(crate) unsafe extern "C" fn CreateReelTasks() {
    let mut i: u8 = 0;
    i = 0;
    while i < NUM_REELS {
        let mut taskId: u8 = CreateTask(Some(Task_Reel), 2);
        gTasks[taskId].data[15] = i as i16;
        (*sSlotMachine).slotReelTasks[i] = taskId;
        Task_Reel(taskId);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpinSlotReel(reelIndex: u8) {
    gTasks[(*sSlotMachine).slotReelTasks[reelIndex]].data[0] = REEL_TASK_SPIN;
    gTasks[(*sSlotMachine).slotReelTasks[reelIndex]].data[14] = TRUE as i16;
}
pub(crate) unsafe extern "C" fn StopSlotReel(reelIndex: u8) {
    gTasks[(*sSlotMachine).slotReelTasks[reelIndex]].data[0] = REEL_TASK_DECIDE_STOP;
}
pub(crate) unsafe extern "C" fn IsSlotReelMoving(reelIndex: u8) -> u8 {
    return gTasks[(*sSlotMachine).slotReelTasks[reelIndex]].data[14] as u8;
}
pub(crate) unsafe extern "C" fn Task_Reel(taskId: u8) {
    while sReelTasks[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]) != 0 {}
}
pub(crate) unsafe extern "C" fn ReelTask_StayStill(task: *mut Task) -> u8 {
    return FALSE;
}
pub(crate) unsafe extern "C" fn ReelTask_Spin(task: *mut Task) -> u8 {
    AdvanceSlotReel((*task).data[15] as u8, (*sSlotMachine).reelSpeed);
    return FALSE;
}
pub(crate) unsafe extern "C" fn ReelTask_DecideStop(task: *mut Task) -> u8 {
    (*task).data[0] += 1;
    (*sSlotMachine).winnerRows[(*task).data[15]] = 0;
    (*sSlotMachine).reelExtraTurns[(*task).data[15]] = 0;
    if (*sSlotMachine).reelTimeSpinsLeft == 0 {
        if (*sSlotMachine).machineBias == 0
            || (*sSlotMachine).didNotFailBias == 0
            || sDecideStop_Bias[(*task).data[15]].unwrap_unchecked()() == 0
        {
            (*sSlotMachine).didNotFailBias = FALSE;
            sDecideStop_NoBias[(*task).data[15]].unwrap_unchecked()();
        }
    }
    (*task).data[1] = (*sSlotMachine).reelExtraTurns[(*task).data[15]];
    return TRUE;
}
pub(crate) unsafe extern "C" fn ReelTask_MoveToStop(task: *mut Task) -> u8 {
    let mut reelStopShocks: CArray<u16, 5> = zeroed();
    let mut reelPixelPos: i16 = 0;
    memcpy(
        reelStopShocks.as_mut_ptr() as *mut u8,
        sReelStopShocks.as_ptr().cast_mut() as *mut u8,
        10,
    );
    reelPixelPos = (*sSlotMachine).reelPixelOffsets[(*task).data[15]] % 24;
    if reelPixelPos != 0 {
        reelPixelPos =
            AdvanceSlotReelToNextSymbol((*task).data[15] as u8, (*sSlotMachine).reelSpeed);
    } else if (*sSlotMachine).reelExtraTurns[(*task).data[15]] != 0 {
        (*sSlotMachine).reelExtraTurns[(*task).data[15]] -= 1;
        AdvanceSlotReel((*task).data[15] as u8, (*sSlotMachine).reelSpeed);
        reelPixelPos = (*sSlotMachine).reelPixelOffsets[(*task).data[15]] % 24;
    }
    if reelPixelPos == 0 && (*sSlotMachine).reelExtraTurns[(*task).data[15]] == 0 {
        (*task).data[0] += 1;
        (*task).data[1] = reelStopShocks[(*task).data[1]] as i16;
        (*task).data[2] = 0;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ReelTask_ShakingStop(task: *mut Task) -> u8 {
    (*sSlotMachine).reelShockOffsets[(*task).data[15]] = (*task).data[1] as u16;
    (*task).data[1] = -(*task).data[1];
    (*task).data[2] += 1;
    if (*task).data[2] as i32 & 0x3 == 0 {
        (*task).data[1] >>= 1;
    }
    if (*task).data[1] == 0 {
        (*task).data[0] = 0;
        (*task).data[14] = FALSE as i16;
        (*sSlotMachine).reelShockOffsets[(*task).data[15]] = 0;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel1() -> u8 {
    let mut sym2: u8 = GetBiasSymbol((*sSlotMachine).machineBias);
    let mut sym1: u8 = sym2;
    if (*sSlotMachine).machineBias as i32 & 192 != 0 {
        sym1 = SYMBOL_7_RED;
        sym2 = SYMBOL_7_BLUE;
    }
    return sDecideStop_Bias_Reel1_Bets[(*sSlotMachine).bet as i32 - 1].unwrap_unchecked()(
        sym1, sym2,
    );
}
pub(crate) unsafe extern "C" fn EitherSymbolAtPos_Reel1(pos: i16, sym1: u8, sym2: u8) -> u8 {
    let mut sym: u8 = GetSymbol(LEFT_REEL, pos);
    if sym == sym1 || sym == sym2 {
        (*sSlotMachine).biasSymbol = sym;
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn AreCherriesOnScreen_Reel1(turns: i16) -> u8 {
    if GetSymbol(LEFT_REEL, 1 - turns) == SYMBOL_CHERRY
        || GetSymbol(LEFT_REEL, 2 - turns) == SYMBOL_CHERRY
        || GetSymbol(LEFT_REEL, 3 - turns) == SYMBOL_CHERRY
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
pub(crate) unsafe extern "C" fn BiasedTowardCherryOr7s() -> u8 {
    if (*sSlotMachine).machineBias as i32 & 194 != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel1_Bet1(sym1: u8, sym2: u8) -> u8 {
    let mut i: i16 = 0;
    i = 0;
    while i <= MAX_EXTRA_TURNS {
        if EitherSymbolAtPos_Reel1(2 - i, sym1, sym2) != 0 {
            (*sSlotMachine).winnerRows[0] = 2;
            (*sSlotMachine).reelExtraTurns[0] = i;
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel1_Bet2or3(sym1: u8, sym2: u8) -> u8 {
    let mut i: i16 = 0;
    let mut cherry7Bias: u8 = BiasedTowardCherryOr7s();
    if cherry7Bias != 0 || AreCherriesOnScreen_Reel1(0) == 0 {
        i = 1;
        while i <= 3 {
            if EitherSymbolAtPos_Reel1(i, sym1, sym2) != 0 {
                (*sSlotMachine).winnerRows[0] = i;
                (*sSlotMachine).reelExtraTurns[0] = 0;
                return TRUE;
            }
            i += 1;
        }
    }
    i = 1;
    while i <= MAX_EXTRA_TURNS {
        let mut cherry7BiasCopy: u8 = cherry7Bias;
        if cherry7BiasCopy != 0 || AreCherriesOnScreen_Reel1(i) == 0 {
            if EitherSymbolAtPos_Reel1(1 - i, sym1, sym2) != 0 {
                if i == 1 && (cherry7BiasCopy != 0 || AreCherriesOnScreen_Reel1(3) == 0) {
                    (*sSlotMachine).winnerRows[0] = 3;
                    (*sSlotMachine).reelExtraTurns[0] = 3;
                    return TRUE;
                }
                if i <= 3 && (cherry7BiasCopy != 0 || AreCherriesOnScreen_Reel1(i + 1) == 0) {
                    (*sSlotMachine).winnerRows[0] = 2;
                    (*sSlotMachine).reelExtraTurns[0] = i + 1;
                    return TRUE;
                }
                (*sSlotMachine).winnerRows[0] = 1;
                (*sSlotMachine).reelExtraTurns[0] = i;
                return TRUE;
            }
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel2() -> u8 {
    return sDecideStop_Bias_Reel2_Bets[(*sSlotMachine).bet as i32 - 1].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel2_Bet1or2() -> u8 {
    let mut i: i16 = 0;
    let mut reel1BiasRow: i16 = (*sSlotMachine).winnerRows[0];
    i = 0;
    while i <= MAX_EXTRA_TURNS {
        if GetSymbol(MIDDLE_REEL, reel1BiasRow - i) == (*sSlotMachine).biasSymbol {
            (*sSlotMachine).winnerRows[1] = reel1BiasRow;
            (*sSlotMachine).reelExtraTurns[1] = i;
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel2_Bet3() -> u8 {
    let mut i: i16 = 0;
    if DecideStop_Bias_Reel2_Bet1or2() != 0 {
        if (*sSlotMachine).winnerRows[0] != 2
            && (*sSlotMachine).reelExtraTurns[1] > 1
            && (*sSlotMachine).reelExtraTurns[1] != 4
        {
            i = 0;
            while i <= MAX_EXTRA_TURNS {
                if GetSymbol(MIDDLE_REEL, 2 - i) == (*sSlotMachine).biasSymbol {
                    (*sSlotMachine).winnerRows[1] = 2;
                    (*sSlotMachine).reelExtraTurns[1] = i;
                    break;
                }
                i += 1;
            }
        }
        return TRUE;
    }
    if (*sSlotMachine).winnerRows[0] != 2 {
        i = 0;
        while i <= MAX_EXTRA_TURNS {
            if GetSymbol(MIDDLE_REEL, 2 - i) == (*sSlotMachine).biasSymbol {
                (*sSlotMachine).winnerRows[1] = 2;
                (*sSlotMachine).reelExtraTurns[1] = i;
                return TRUE;
            }
            i += 1;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel3() -> u8 {
    let mut biasSymbol: u8 = (*sSlotMachine).biasSymbol;
    if (*sSlotMachine).machineBias as i32 & BIAS_MIXED_7 != 0 {
        biasSymbol = SYMBOL_7_RED;
        if (*sSlotMachine).biasSymbol == SYMBOL_7_RED {
            biasSymbol = SYMBOL_7_BLUE;
        }
    }
    return sDecideStop_Bias_Reel3_Bets[(*sSlotMachine).bet as i32 - 1].unwrap_unchecked()(
        biasSymbol,
    );
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel3_Bet1or2(biasSymbol: u8) -> u8 {
    let mut i: i16 = 0;
    let mut reel2BiasRow: i16 = (*sSlotMachine).winnerRows[1];
    i = 0;
    while i <= MAX_EXTRA_TURNS {
        if GetSymbol(RIGHT_REEL, reel2BiasRow - i) == biasSymbol {
            (*sSlotMachine).winnerRows[2] = reel2BiasRow;
            (*sSlotMachine).reelExtraTurns[2] = i;
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DecideStop_Bias_Reel3_Bet3(biasSymbol: u8) -> u8 {
    let mut i: i16 = 0;
    let mut biasRow: i16 = 0;
    if (*sSlotMachine).winnerRows[0] == (*sSlotMachine).winnerRows[1] {
        return DecideStop_Bias_Reel3_Bet1or2(biasSymbol);
    }
    if (*sSlotMachine).winnerRows[0] == 1 {
        biasRow = 3;
    } else {
        biasRow = 1;
    }
    i = 0;
    while i <= MAX_EXTRA_TURNS {
        if GetSymbol(RIGHT_REEL, biasRow - i) == biasSymbol {
            (*sSlotMachine).reelExtraTurns[2] = i;
            (*sSlotMachine).winnerRows[2] = biasRow;
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel1() {
    let mut i: i16 = 0;
    while AreCherriesOnScreen_Reel1(i) != 0 {
        i += 1;
    }
    (*sSlotMachine).reelExtraTurns[0] = i;
}
pub(crate) unsafe extern "C" fn IfSymbol7_SwitchColor(symbol: *mut u8) -> u8 {
    if *symbol == SYMBOL_7_RED {
        *symbol = SYMBOL_7_BLUE;
        return TRUE;
    }
    if *symbol == SYMBOL_7_BLUE {
        *symbol = SYMBOL_7_RED;
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel2() {
    sDecideStop_NoBias_Reel2_Bets[(*sSlotMachine).bet as i32 - 1].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel2_Bet1() {
    if (*sSlotMachine).winnerRows[0] != 0
        && (*sSlotMachine).machineBias as i32 & BIAS_STRAIGHT_7 != 0
    {
        let mut reel1MiddleSym: u8 = GetSymbol(LEFT_REEL, 2 - (*sSlotMachine).reelExtraTurns[0]);
        if IfSymbol7_SwitchColor(&raw mut reel1MiddleSym) != 0 {
            let mut i: i16 = 0;
            i = 0;
            while i <= MAX_EXTRA_TURNS {
                if reel1MiddleSym == GetSymbol(MIDDLE_REEL, 2 - i) {
                    (*sSlotMachine).winnerRows[1] = 2;
                    (*sSlotMachine).reelExtraTurns[1] = i;
                    break;
                }
                i += 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel2_Bet2() {
    if (*sSlotMachine).winnerRows[0] != 0
        && (*sSlotMachine).machineBias as i32 & BIAS_STRAIGHT_7 != 0
    {
        let mut reel1BiasSym: u8 = GetSymbol(
            LEFT_REEL,
            (*sSlotMachine).winnerRows[0] - (*sSlotMachine).reelExtraTurns[0],
        );
        if IfSymbol7_SwitchColor(&raw mut reel1BiasSym) != 0 {
            let mut i: i16 = 0;
            i = 0;
            while i <= MAX_EXTRA_TURNS {
                if reel1BiasSym == GetSymbol(MIDDLE_REEL, (*sSlotMachine).winnerRows[0] - i) {
                    (*sSlotMachine).winnerRows[1] = (*sSlotMachine).winnerRows[0];
                    (*sSlotMachine).reelExtraTurns[1] = i;
                    break;
                }
                i += 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel2_Bet3() {
    let mut i: i16 = 0;
    let mut j: i16 = 0;
    let mut reel1BiasSym: u8 = 0;
    if (*sSlotMachine).winnerRows[0] != 0
        && (*sSlotMachine).machineBias as i32 & BIAS_STRAIGHT_7 != 0
    {
        if (*sSlotMachine).winnerRows[0] == 2 {
            DecideStop_NoBias_Reel2_Bet2();
            return;
        }
        reel1BiasSym = GetSymbol(
            LEFT_REEL,
            (*sSlotMachine).winnerRows[0] - (*sSlotMachine).reelExtraTurns[0],
        );
        if IfSymbol7_SwitchColor(&raw mut reel1BiasSym) != 0 {
            j = 2;
            if (*sSlotMachine).winnerRows[0] == 3 {
                j = 3;
            }
            i = 0;
            while i < 2 {
                if reel1BiasSym == GetSymbol(MIDDLE_REEL, j) {
                    (*sSlotMachine).winnerRows[1] = j;
                    (*sSlotMachine).reelExtraTurns[1] = 0;
                    return;
                }
                i += 1;
                j -= 1;
            }
            j = 1;
            while j <= MAX_EXTRA_TURNS {
                if reel1BiasSym == GetSymbol(MIDDLE_REEL, (*sSlotMachine).winnerRows[0] - j) {
                    if (*sSlotMachine).winnerRows[0] == 1 {
                        if j <= 2 {
                            (*sSlotMachine).winnerRows[1] = 2;
                            (*sSlotMachine).reelExtraTurns[1] = j + 1;
                        } else {
                            (*sSlotMachine).winnerRows[1] = 1;
                            (*sSlotMachine).reelExtraTurns[1] = j;
                        }
                    } else {
                        if j <= 2 {
                            (*sSlotMachine).winnerRows[1] = 3;
                            (*sSlotMachine).reelExtraTurns[1] = j;
                        } else {
                            (*sSlotMachine).winnerRows[1] = 2;
                            (*sSlotMachine).reelExtraTurns[1] = j - 1;
                        }
                    }
                    return;
                }
                j += 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MismatchedSyms_77(sym1: u8, sym2: u8) -> u8 {
    if sym1 == SYMBOL_7_RED && sym2 == SYMBOL_7_BLUE
        || sym1 == SYMBOL_7_BLUE && sym2 == SYMBOL_7_RED
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
pub(crate) unsafe extern "C" fn MismatchedSyms_777(sym1: u8, sym2: u8, sym3: u8) -> u8 {
    if sym1 == SYMBOL_7_RED && sym2 == SYMBOL_7_BLUE && sym3 == SYMBOL_7_RED
        || sym1 == SYMBOL_7_BLUE && sym2 == SYMBOL_7_RED && sym3 == SYMBOL_7_BLUE
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
pub(crate) unsafe extern "C" fn NeitherMatchNor7Mismatch(sym1: u8, sym2: u8, sym3: u8) -> u8 {
    if sym1 == SYMBOL_7_RED && sym2 == SYMBOL_7_BLUE && sym3 == SYMBOL_7_RED
        || sym1 == SYMBOL_7_BLUE && sym2 == SYMBOL_7_RED && sym3 == SYMBOL_7_BLUE
        || sym1 == SYMBOL_7_RED && sym2 == SYMBOL_7_RED && sym3 == SYMBOL_7_BLUE
        || sym1 == SYMBOL_7_BLUE && sym2 == SYMBOL_7_BLUE && sym3 == SYMBOL_7_RED
        || sym1 == sym2 && sym1 == sym3
    {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel3() {
    sDecideStop_NoBias_Reel3_Bets[(*sSlotMachine).bet as i32 - 1].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel3_Bet1() {
    let mut i: i16 = 0;
    let mut sym1: u8 = GetSymbol(LEFT_REEL, 2 - (*sSlotMachine).reelExtraTurns[0]);
    let mut sym2: u8 = GetSymbol(MIDDLE_REEL, 2 - (*sSlotMachine).reelExtraTurns[1]);
    if sym1 == sym2 {
        loop {
            let mut sym3: u8 = 0;
            if !(sym1
                == ({
                    sym3 = GetSymbol(RIGHT_REEL, 2 - i);
                    sym3
                })
                || sym1 == SYMBOL_7_RED && sym3 == SYMBOL_7_BLUE
                || sym1 == SYMBOL_7_BLUE && sym3 == SYMBOL_7_RED)
            {
                break;
            }
            i += 1;
        }
    } else if MismatchedSyms_77(sym1, sym2) != 0 {
        if (*sSlotMachine).machineBias as i32 & BIAS_STRAIGHT_7 != 0 {
            i = 0;
            while i <= MAX_EXTRA_TURNS {
                if sym1 == GetSymbol(RIGHT_REEL, 2 - i) {
                    (*sSlotMachine).reelExtraTurns[2] = i;
                    return;
                }
                i += 1;
            }
        }
        i = 0;
        loop {
            if sym1 != GetSymbol(RIGHT_REEL, 2 - i) {
                break;
            }
            i += 1;
        }
    }
    (*sSlotMachine).reelExtraTurns[2] = i;
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel3_Bet2() {
    let mut extraTurns: i16 = 0;
    let mut i: i16 = 0;
    let mut sym1: u8 = 0;
    let mut sym2: u8 = 0;
    let mut sym3: u8 = 0;
    if (*sSlotMachine).winnerRows[1] != 0
        && (*sSlotMachine).winnerRows[0] == (*sSlotMachine).winnerRows[1]
        && (*sSlotMachine).machineBias as i32 & BIAS_STRAIGHT_7 != 0
    {
        sym1 = GetSymbol(
            LEFT_REEL,
            (*sSlotMachine).winnerRows[0] - (*sSlotMachine).reelExtraTurns[0],
        );
        sym2 = GetSymbol(
            MIDDLE_REEL,
            (*sSlotMachine).winnerRows[1] - (*sSlotMachine).reelExtraTurns[1],
        );
        if MismatchedSyms_77(sym1, sym2) != 0 {
            i = 0;
            while i <= MAX_EXTRA_TURNS {
                sym3 = GetSymbol(RIGHT_REEL, (*sSlotMachine).winnerRows[1] - i);
                if sym1 == sym3 {
                    extraTurns = i;
                    break;
                }
                i += 1;
            }
        }
    }
    loop {
        let mut numMatches: i16 = 0;
        i = 1;
        numMatches = 0;
        while i <= 3 {
            sym1 = GetSymbol(LEFT_REEL, i - (*sSlotMachine).reelExtraTurns[0]);
            sym2 = GetSymbol(MIDDLE_REEL, i - (*sSlotMachine).reelExtraTurns[1]);
            sym3 = GetSymbol(RIGHT_REEL, i - extraTurns);
            if NeitherMatchNor7Mismatch(sym1, sym2, sym3) == 0
                && !(MismatchedSyms_777(sym1, sym2, sym3) != 0
                    && (*sSlotMachine).machineBias as i32 & BIAS_STRAIGHT_7 != 0)
            {
                numMatches += 1;
                break;
            }
            i += 1;
        }
        if numMatches == 0 {
            break;
        }
        extraTurns += 1;
    }
    (*sSlotMachine).reelExtraTurns[2] = extraTurns;
}
pub(crate) unsafe extern "C" fn DecideStop_NoBias_Reel3_Bet3() {
    let mut sym1: u8 = 0;
    let mut sym2: u8 = 0;
    let mut sym3: u8 = 0;
    let mut row: i16 = 0;
    let mut i: i16 = 0;
    DecideStop_NoBias_Reel3_Bet2();
    if (*sSlotMachine).winnerRows[1] != 0
        && (*sSlotMachine).winnerRows[0] != (*sSlotMachine).winnerRows[1]
        && (*sSlotMachine).machineBias as i32 & BIAS_STRAIGHT_7 != 0
    {
        sym1 = GetSymbol(
            LEFT_REEL,
            (*sSlotMachine).winnerRows[0] - (*sSlotMachine).reelExtraTurns[0],
        );
        sym2 = GetSymbol(
            MIDDLE_REEL,
            (*sSlotMachine).winnerRows[1] - (*sSlotMachine).reelExtraTurns[1],
        );
        if MismatchedSyms_77(sym1, sym2) != 0 {
            row = 1;
            if (*sSlotMachine).winnerRows[0] == 1 {
                row = 3;
            }
            i = 0;
            while i <= MAX_EXTRA_TURNS {
                sym3 = GetSymbol(RIGHT_REEL, row - ((*sSlotMachine).reelExtraTurns[2] + i));
                if sym1 == sym3 {
                    (*sSlotMachine).reelExtraTurns[2] += i;
                    break;
                }
                i += 1;
            }
        }
    }
    loop {
        sym1 = GetSymbol(LEFT_REEL, 1 - (*sSlotMachine).reelExtraTurns[0]);
        sym2 = GetSymbol(MIDDLE_REEL, 2 - (*sSlotMachine).reelExtraTurns[1]);
        sym3 = GetSymbol(RIGHT_REEL, 3 - (*sSlotMachine).reelExtraTurns[2]);
        if NeitherMatchNor7Mismatch(sym1, sym2, sym3) != 0
            || MismatchedSyms_777(sym1, sym2, sym3) != 0
                && (*sSlotMachine).machineBias as i32 & BIAS_STRAIGHT_7 != 0
        {
            break;
        }
        (*sSlotMachine).reelExtraTurns[2] += 1;
    }
    loop {
        sym1 = GetSymbol(LEFT_REEL, 3 - (*sSlotMachine).reelExtraTurns[0]);
        sym2 = GetSymbol(MIDDLE_REEL, 2 - (*sSlotMachine).reelExtraTurns[1]);
        sym3 = GetSymbol(RIGHT_REEL, 1 - (*sSlotMachine).reelExtraTurns[2]);
        if NeitherMatchNor7Mismatch(sym1, sym2, sym3) != 0
            || MismatchedSyms_777(sym1, sym2, sym3) != 0
                && (*sSlotMachine).machineBias as i32 & BIAS_STRAIGHT_7 != 0
        {
            break;
        }
        (*sSlotMachine).reelExtraTurns[2] += 1;
    }
}
pub(crate) unsafe extern "C" fn PressStopReelButton(reelNum: u8) {
    let mut taskId: u8 = CreateTask(Some(Task_PressStopReelButton), 5);
    gTasks[taskId].data[15] = reelNum as i16;
    Task_PressStopReelButton(taskId);
}
pub(crate) unsafe extern "C" fn Task_PressStopReelButton(taskId: u8) {
    sReelStopButtonTasks[gTasks[taskId].data[0]].unwrap_unchecked()(
        &raw mut gTasks[taskId],
        taskId,
    );
}
pub(crate) unsafe extern "C" fn StopReelButton_Press(task: *mut Task, taskId: u8) {
    SetReelButtonTilemap(sReelButtonOffsets[(*task).data[15]], 0x62, 0x63, 0x72, 0x73);
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn StopReelButton_Wait(task: *mut Task, taskId: u8) {
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) > 11
    {
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn StopReelButton_Unpress(task: *mut Task, taskId: u8) {
    SetReelButtonTilemap(sReelButtonOffsets[(*task).data[15]], 0x42, 0x43, 0x52, 0x53);
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn LightenMatchLine(matchLineId: u8) {
    LoadPalette(
        sLitMatchLinePalTable[matchLineId] as *mut c_void,
        sMatchLinePalOffsets[matchLineId] as u16,
        2,
    );
}
pub(crate) unsafe extern "C" fn DarkenMatchLine(matchLineId: u8) {
    LoadPalette(
        sDarkMatchLinePalTable[matchLineId] as *mut c_void,
        sMatchLinePalOffsets[matchLineId] as u16,
        2,
    );
}
pub(crate) unsafe extern "C" fn LightenBetTiles(betVal: u8) {
    let mut i: u8 = 0;
    i = 0;
    while i < sMatchLinesPerBet[betVal] {
        LightenMatchLine(sBetToMatchLineIds[betVal][i]);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DarkenBetTiles(betVal: u8) {
    let mut i: u8 = 0;
    i = 0;
    while i < sMatchLinesPerBet[betVal] {
        DarkenMatchLine(sBetToMatchLineIds[betVal][i]);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateInvisibleFlashMatchLineSprites() {
    let mut i: u8 = 0;
    i = 0;
    while i < 5 {
        let mut spriteId: u8 = CreateInvisibleSprite(Some(SpriteCB_FlashMatchingLines));
        gSprites[spriteId].data[0] = i as i16;
        (*sSlotMachine).flashMatchLineSpriteIds[i] = spriteId;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn FlashMatchLine(matchLineId: u8) {
    let mut sprite: *mut Sprite =
        &raw mut gSprites[(*sSlotMachine).flashMatchLineSpriteIds[matchLineId]];
    (*sprite).data[1] = TRUE as i16;
    (*sprite).data[2] = 4;
    (*sprite).data[3] = 0;
    (*sprite).data[4] = 0;
    (*sprite).data[5] = 2;
    (*sprite).data[7] = FALSE as i16;
}
pub(crate) unsafe extern "C" fn IsMatchLineDoneFlashingBeforePayout() -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < 5 {
        let mut sprite: *mut Sprite = &raw mut gSprites[(*sSlotMachine).flashMatchLineSpriteIds[i]];
        if (*sprite).data[1] != 0 && (*sprite).data[2] != 0 {
            return FALSE;
        }
        i += 1;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn TryStopMatchLinesFlashing() -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < 5 {
        if TryStopMatchLineFlashing((*sSlotMachine).flashMatchLineSpriteIds[i]) == 0 {
            return FALSE;
        }
        i += 1;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn TryStopMatchLineFlashing(spriteId: u8) -> u8 {
    let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
    if (*sprite).data[1] == 0 {
        return TRUE;
    }
    if (*sprite).data[7] != 0 {
        (*sprite).data[1] = FALSE as i16;
    }
    return (*sprite).data[7] as u8;
}
pub(crate) unsafe extern "C" fn SpriteCB_FlashMatchingLines(sprite: *mut Sprite) {
    let mut maxColorChange: i16 = 0;
    if (*sprite).data[1] != 0 {
        if ({
            let t1 = (*sprite).data[3];
            (*sprite).data[3] -= 1;
            t1
        }) == 0
        {
            (*sprite).data[7] = FALSE as i16;
            (*sprite).data[3] = 1;
            (*sprite).data[4] += (*sprite).data[5];
            maxColorChange = 4;
            if (*sprite).data[2] != 0 {
                maxColorChange = 8;
            }
            if (*sprite).data[4] <= 0 {
                (*sprite).data[7] = TRUE as i16;
                (*sprite).data[5] = -(*sprite).data[5];
                if (*sprite).data[2] != 0 {
                    (*sprite).data[2] -= 1;
                }
            } else if (*sprite).data[4] >= maxColorChange {
                (*sprite).data[5] = -(*sprite).data[5];
            }
            if (*sprite).data[2] != 0 {
                (*sprite).data[3] <<= 1;
            }
        }
        MultiplyPaletteRGBComponents(
            sMatchLinePalOffsets[(*sprite).data[0]] as u16,
            (*sprite).data[4] as u8,
            (*sprite).data[4] as u8,
            (*sprite).data[4] as u8,
        );
    }
}
pub(crate) unsafe extern "C" fn FlashSlotMachineLights() {
    let mut taskId: u8 = CreateTask(Some(Task_FlashSlotMachineLights), 6);
    gTasks[taskId].data[3] = 1;
    Task_FlashSlotMachineLights(taskId);
}
pub(crate) unsafe extern "C" fn TryStopSlotMachineLights() -> u8 {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_FlashSlotMachineLights));
    if gTasks[taskId].data[2] == 0 {
        DestroyTask(taskId);
        LoadPalette(*sSlotMachineMenu_Pal as *mut c_void, 16, 32);
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_FlashSlotMachineLights(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if ({
        let t1 = (*task).data[1];
        (*task).data[1] -= 1;
        t1
    }) == 0
    {
        (*task).data[1] = 4;
        (*task).data[2] += (*task).data[3];
        if (*task).data[2] == 0 || (*task).data[2] == 2 {
            (*task).data[3] = -(*task).data[3];
        }
    }
    LoadPalette(
        sFlashingLightsPalTable[(*task).data[2]] as *mut c_void,
        16,
        32,
    );
}
pub(crate) unsafe extern "C" fn CreatePikaPowerBoltTask() {
    (*sSlotMachine).pikaPowerBoltTaskId = CreateTask(Some(Task_CreatePikaPowerBolt), 8);
}
pub(crate) unsafe extern "C" fn AddPikaPowerBolt(bolts: u8) {
    let mut task: *mut Task = &raw mut gTasks[(*sSlotMachine).pikaPowerBoltTaskId];
    ResetPikaPowerBoltTask(task);
    (*task).data[0] = PIKABOLT_TASK_ADD_BOLT;
    (*task).data[1] += 1;
    (*task).data[15] = TRUE as i16;
}
pub(crate) unsafe extern "C" fn ResetPikaPowerBolts() {
    let mut task: *mut Task = &raw mut gTasks[(*sSlotMachine).pikaPowerBoltTaskId];
    ResetPikaPowerBoltTask(task);
    (*task).data[0] = PIKABOLT_TASK_CLEAR_ALL;
    (*task).data[15] = TRUE as i16;
}
pub(crate) unsafe extern "C" fn IsPikaPowerBoltAnimating() -> u8 {
    return gTasks[(*sSlotMachine).pikaPowerBoltTaskId].data[15] as u8;
}
pub(crate) unsafe extern "C" fn Task_CreatePikaPowerBolt(taskId: u8) {
    sPikaPowerBoltTasks[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]);
}
pub(crate) unsafe extern "C" fn PikaPowerBolt_Idle(task: *mut Task) {}
pub(crate) unsafe extern "C" fn PikaPowerBolt_AddBolt(task: *mut Task) {
    (*task).data[2] = CreatePikaPowerBoltSprite(((*task).data[1] << 3) + 20, 20) as i16;
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn PikaPowerBolt_WaitAnim(task: *mut Task) {
    if gSprites[(*task).data[2]].data[7] != 0 {
        let mut r5: i16 = (*task).data[1] + 2;
        let mut r3: i16 = 0;
        let mut r2: i16 = 0;
        if (*task).data[1] == 1 {
            r3 = 1;
            r2 = 1;
        } else if (*task).data[1] == 16 {
            r3 = 2;
            r2 = 2;
        }
        *sSelectedPikaPowerTile.at(r2) = sPikaPowerTileTable[r3][0];
        LoadBgTilemap(
            2,
            sSelectedPikaPowerTile.at(r2) as *mut c_void,
            2,
            r5 as u16 + 0x40,
        );
        DestroyPikaPowerBoltSprite((*task).data[2] as u8);
        (*task).data[0] = PIKABOLT_TASK_IDLE;
        (*task).data[15] = 0;
    }
}
pub(crate) unsafe extern "C" fn PikaPowerBolt_ClearAll(task: *mut Task) {
    let mut r5: i16 = (*task).data[1] + 2;
    let mut r3: i16 = 0;
    let mut r2: i16 = 3;
    if (*task).data[1] == 1 {
        r3 = 1;
        r2 = 1;
    } else if (*task).data[1] == 16 {
        r3 = 2;
        r2 = 2;
    }
    if (*task).data[2] == 0 {
        *sSelectedPikaPowerTile.at(r2) = sPikaPowerTileTable[r3][1];
        LoadBgTilemap(
            2,
            sSelectedPikaPowerTile.at(r2) as *mut c_void,
            2,
            r5 as u16 + 0x40,
        );
        (*task).data[1] -= 1;
    }
    if ({
        (*task).data[2] += 1;
        (*task).data[2]
    }) >= 20
    {
        (*task).data[2] = 0;
    }
    if (*task).data[1] == 0 {
        (*task).data[0] = PIKABOLT_TASK_IDLE;
        (*task).data[15] = 0;
    }
}
pub(crate) unsafe extern "C" fn ResetPikaPowerBoltTask(task: *mut Task) {
    let mut i: u8 = 0;
    i = 2;
    while i < NUM_TASK_DATA {
        (*task).data[i] = 0;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn LoadPikaPowerMeter(bolts: u8) {
    let mut i: i16 = 0;
    let mut r3: i16 = 0;
    let mut r1: i16 = 0;
    let mut r4: i16 = 3;
    i = 0;
    while i < bolts as i16 {
        r3 = 0;
        r1 = 0;
        if i == 0 {
            r3 = 1;
            r1 = 1;
        } else if i == 15 {
            r3 = 2;
            r1 = 2;
        }
        *sSelectedPikaPowerTile.at(r1) = sPikaPowerTileTable[r3][0];
        LoadBgTilemap(
            2,
            sSelectedPikaPowerTile.at(r1) as *mut c_void,
            2,
            r4 as u16 + 0x40,
        );
        i += 1;
        r4 += 1;
    }
    while i < 16 {
        r3 = 0;
        r1 = 3;
        if i == 0 {
            r3 = 1;
            r1 = 1;
        } else if i == 15 {
            r3 = 2;
            r1 = 2;
        }
        *sSelectedPikaPowerTile.at(r1) = sPikaPowerTileTable[r3][1];
        LoadBgTilemap(
            2,
            sSelectedPikaPowerTile.at(r1) as *mut c_void,
            2,
            r4 as u16 + 0x40,
        );
        i += 1;
        r4 += 1;
    }
    gTasks[(*sSlotMachine).pikaPowerBoltTaskId].data[1] = bolts as i16;
}
pub(crate) unsafe extern "C" fn BeginReelTime() {
    let mut taskId: u8 = CreateTask(Some(Task_ReelTime), 7);
    Task_ReelTime(taskId);
}
pub(crate) unsafe extern "C" fn IsReelTimeTaskDone() -> u8 {
    if FindTaskIdByFunc(Some(Task_ReelTime)) == TAIL_SENTINEL {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_ReelTime(taskId: u8) {
    sReelTimeTasks[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]);
}
pub(crate) unsafe extern "C" fn ReelTime_Init(task: *mut Task) {
    (*sSlotMachine).reelTimeSpinsLeft = 0;
    (*sSlotMachine).reeltimePixelOffset = 0;
    (*sSlotMachine).reeltimePosition = 0;
    (*task).data[0] += 1;
    (*task).data[1] = 0;
    (*task).data[2] = 30;
    (*task).data[4] = 1280;
    gSpriteCoordOffsetX = 0;
    gSpriteCoordOffsetY = 0;
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    LoadReelTimeWindowTilemap(REG_OFFSET_BG3VOFS as i16, 0);
    CreateReelTimeMachineSprites();
    CreateReelTimePikachuSprite();
    CreateReelTimeNumberSprites();
    CreateReelTimeShadowSprites();
    CreateReelTimeNumberGapSprite();
    GetReelTimeDraw();
    StopMapMusic();
    PlayNewMapMusic(MUS_ROULETTE);
}
pub(crate) unsafe extern "C" fn ReelTime_WindowEnter(task: *mut Task) {
    let mut r3: i16 = 0;
    gSpriteCoordOffsetX -= 8;
    (*task).data[1] += 8;
    r3 = (((*task).data[1] as i32 + 240 & 0xff) >> 3) as i16;
    SetGpuReg(REG_OFFSET_BG1HOFS, (*task).data[1] as u16 & 0x1ff);
    if r3 != (*task).data[2] && (*task).data[3] <= 18 {
        (*task).data[2] = r3;
        (*task).data[3] = (*task).data[1] >> 3;
        LoadReelTimeWindowTilemap(r3, (*task).data[3]);
    }
    if (*task).data[1] >= 200 {
        (*task).data[0] += 1;
        (*task).data[3] = 0;
    }
    AdvanceReeltimeReel((*task).data[4] >> 8);
}
pub(crate) unsafe extern "C" fn ReelTime_WaitStartPikachu(task: *mut Task) {
    AdvanceReeltimeReel((*task).data[4] >> 8);
    if ({
        (*task).data[5] += 1;
        (*task).data[5]
    }) >= 60
    {
        (*task).data[0] += 1;
        CreateReelTimeBoltSprites();
        CreateReelTimePikachuAuraSprites();
    }
}
pub(crate) unsafe extern "C" fn ReelTime_PikachuSpeedUp1(task: *mut Task) {
    let mut i: i32 = 0;
    let mut pikachuAnimIds: CArray<u8, 4> = zeroed();
    let mut reelTimeBoltDelays: CArray<i16, 4> = zeroed();
    let mut pikachuAuraFlashDelays: CArray<i16, 4> = zeroed();
    memcpy(
        pikachuAnimIds.as_mut_ptr(),
        sReelTimePikachuAnimIds.as_ptr().cast_mut(),
        4,
    );
    memcpy(
        reelTimeBoltDelays.as_mut_ptr() as *mut u8,
        sReelTimeBoltDelays.as_ptr().cast_mut() as *mut u8,
        8,
    );
    memcpy(
        pikachuAuraFlashDelays.as_mut_ptr() as *mut u8,
        sPikachuAuraFlashDelays.as_ptr().cast_mut() as *mut u8,
        8,
    );
    AdvanceReeltimeReel((*task).data[4] >> 8);
    (*task).data[4] -= 4;
    i = 4 - ((*task).data[4] >> 8) as i32;
    SetReelTimeBoltDelay(reelTimeBoltDelays[i]);
    SetReelTimePikachuAuraFlashDelay(pikachuAuraFlashDelays[i]);
    StartSpriteAnimIfDifferent(
        &raw mut gSprites[(*sSlotMachine).reelTimePikachuSpriteId],
        pikachuAnimIds[i],
    );
    if (*task).data[4] <= 0x100 {
        (*task).data[0] += 1;
        (*task).data[4] = 0x100;
        (*task).data[5] = 0;
    }
}
pub(crate) unsafe extern "C" fn ReelTime_PikachuSpeedUp2(task: *mut Task) {
    AdvanceReeltimeReel((*task).data[4] >> 8);
    if ({
        (*task).data[5] += 1;
        (*task).data[5]
    }) >= 80
    {
        (*task).data[0] += 1;
        (*task).data[5] = 0;
        SetReelTimePikachuAuraFlashDelay(2);
        StartSpriteAnimIfDifferent(
            &raw mut gSprites[(*sSlotMachine).reelTimePikachuSpriteId],
            3,
        );
    }
}
pub(crate) unsafe extern "C" fn ReelTime_WaitReel(task: *mut Task) {
    AdvanceReeltimeReel((*task).data[4] >> 8);
    (*task).data[4] = (*task).data[4] as u8 as i16 + 0x80;
    if ({
        (*task).data[5] += 1;
        (*task).data[5]
    }) >= 80
    {
        (*task).data[0] += 1;
        (*task).data[5] = 0;
    }
}
pub(crate) unsafe extern "C" fn ReelTime_CheckExplode(task: *mut Task) {
    AdvanceReeltimeReel((*task).data[4] >> 8);
    (*task).data[4] = (*task).data[4] as u8 as i16 + 0x40;
    if ({
        (*task).data[5] += 1;
        (*task).data[5]
    }) >= 40
    {
        (*task).data[5] = 0;
        if (*sSlotMachine).reelTimeDraw != 0 {
            if (*sSlotMachine).reelTimeSpinsLeft as i16 <= (*task).data[6] {
                (*task).data[0] += 1;
            }
        } else if (*task).data[6] > 3 {
            (*task).data[0] += 1;
        } else if ShouldReelTimeMachineExplode((*task).data[6] as u16) != 0 {
            (*task).data[0] = RT_TASK_EXPLODE;
        }
        (*task).data[6] += 1;
    }
}
pub(crate) unsafe extern "C" fn ReelTime_LandOnOutcome(task: *mut Task) {
    let mut reeltimePixelOffset: i16 = (*sSlotMachine).reeltimePixelOffset % 20;
    if reeltimePixelOffset != 0 {
        reeltimePixelOffset = AdvanceReeltimeReelToNextSymbol((*task).data[4] >> 8);
        (*task).data[4] = (*task).data[4] as u8 as i16 + 0x40;
    } else if GetReelTimeSymbol(1) != (*sSlotMachine).reelTimeDraw {
        AdvanceReeltimeReel((*task).data[4] >> 8);
        reeltimePixelOffset = (*sSlotMachine).reeltimePixelOffset % 20;
        (*task).data[4] = (*task).data[4] as u8 as i16 + 0x40;
    }
    if reeltimePixelOffset == 0 && GetReelTimeSymbol(1) == (*sSlotMachine).reelTimeDraw {
        (*task).data[4] = 0;
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn ReelTime_PikachuReact(task: *mut Task) {
    if ({
        (*task).data[4] += 1;
        (*task).data[4]
    }) >= 60
    {
        StopMapMusic();
        DestroyReelTimeBoltSprites();
        DestroyReelTimePikachuAuraSprites();
        (*task).data[0] += 1;
        if (*sSlotMachine).reelTimeDraw == 0 {
            (*task).data[4] = 0xa0;
            StartSpriteAnimIfDifferent(
                &raw mut gSprites[(*sSlotMachine).reelTimePikachuSpriteId],
                5,
            );
            PlayFanfare(MUS_TOO_BAD);
        } else {
            (*task).data[4] = 0xc0;
            StartSpriteAnimIfDifferent(
                &raw mut gSprites[(*sSlotMachine).reelTimePikachuSpriteId],
                4,
            );
            gSprites[(*sSlotMachine).reelTimePikachuSpriteId].animCmdIndex = 0;
            if (*sSlotMachine).pikaPowerBolts != 0 {
                ResetPikaPowerBolts();
                (*sSlotMachine).pikaPowerBolts = 0;
            }
            PlayFanfare(MUS_SLOTS_WIN);
        }
    }
}
pub(crate) unsafe extern "C" fn ReelTime_WaitClearPikaPower(task: *mut Task) {
    if ((*task).data[4] == 0
        || ({
            (*task).data[4] -= 1;
            (*task).data[4]
        }) == 0)
        && IsPikaPowerBoltAnimating() == 0
    {
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn ReelTime_CloseWindow(task: *mut Task) {
    let mut r4: i16 = 0;
    gSpriteCoordOffsetX -= 8;
    (*task).data[1] += 8;
    (*task).data[3] += 8;
    r4 = (((*task).data[1] as i32 - 8 & 0xff) >> 3) as i16;
    SetGpuReg(REG_OFFSET_BG1HOFS, (*task).data[1] as u16 & 0x1ff);
    if (*task).data[3] >> 3 <= 25 {
        ClearReelTimeWindowTilemap(r4);
    } else {
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn ReelTime_DestroySprites(task: *mut Task) {
    (*sSlotMachine).reelTimeSpinsUsed = 0;
    (*sSlotMachine).reelTimeSpinsLeft = (*sSlotMachine).reelTimeDraw;
    gSpriteCoordOffsetX = 0;
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    (*sSlotMachine).reelSpeed = REEL_NORMAL_SPEED;
    DestroyReelTimePikachuSprite();
    DestroyReelTimeMachineSprites();
    DestroyReelTimeShadowSprites();
    PlayNewMapMusic((*sSlotMachine).backupMapMusic);
    if (*sSlotMachine).reelTimeSpinsLeft == 0 {
        DestroyTask(FindTaskIdByFunc(Some(Task_ReelTime)));
    } else {
        CreateDigitalDisplayScene(DIG_DISPLAY_REEL_TIME);
        (*task).data[1] = ReelTimeSpeed() as i16;
        (*task).data[2] = 0;
        (*task).data[3] = 0;
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn ReelTime_SetReelSpeed(task: *mut Task) {
    if (*sSlotMachine).reelSpeed == (*task).data[1] {
        (*task).data[0] += 1;
    } else if (*sSlotMachine).reelPixelOffsets[0] % 24 == 0
        && ({
            (*task).data[2] += 1;
            (*task).data[2]
        }) as i32
            & 0x07
            == 0
    {
        (*sSlotMachine).reelSpeed >>= 1;
    }
}
pub(crate) unsafe extern "C" fn ReelTime_EndSuccess(task: *mut Task) {
    if IsDigitalDisplayAnimFinished() != 0 {
        DestroyTask(FindTaskIdByFunc(Some(Task_ReelTime)));
    }
}
pub(crate) unsafe extern "C" fn ReelTime_ExplodeMachine(task: *mut Task) {
    DestroyReelTimeMachineSprites();
    DestroyReelTimeBoltSprites();
    DestroyReelTimePikachuAuraSprites();
    CreateReelTimeExplosionSprite();
    gSprites[(*sSlotMachine).reelTimeShadowSpriteIds[0]].set_invisible(TRUE as u16);
    StartSpriteAnimIfDifferent(
        &raw mut gSprites[(*sSlotMachine).reelTimePikachuSpriteId],
        5,
    );
    (*task).data[0] += 1;
    (*task).data[4] = 4;
    (*task).data[5] = 0;
    StopMapMusic();
    PlayFanfare(MUS_TOO_BAD);
    PlaySE(SE_M_EXPLOSION);
}
pub(crate) unsafe extern "C" fn ReelTime_WaitExplode(task: *mut Task) {
    gSpriteCoordOffsetY = (*task).data[4];
    SetGpuReg(REG_OFFSET_BG1VOFS, (*task).data[4] as u16);
    if (*task).data[5] as i32 & 0x01 != 0 {
        (*task).data[4] = -(*task).data[4];
    }
    if ({
        (*task).data[5] += 1;
        (*task).data[5]
    }) as i32
        & 0x1f
        == 0
    {
        (*task).data[4] >>= 1;
    }
    if (*task).data[4] == 0 {
        DestroyReelTimeExplosionSprite();
        CreateReelTimeDuckSprites();
        CreateBrokenReelTimeMachineSprite();
        CreateReelTimeSmokeSprite();
        gSprites[(*sSlotMachine).reelTimeShadowSpriteIds[0]].set_invisible(0);
        (*task).data[0] += 1;
        (*task).data[5] = 0;
    }
}
pub(crate) unsafe extern "C" fn ReelTime_WaitSmoke(task: *mut Task) {
    gSpriteCoordOffsetY = 0;
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    if IsReelTimeSmokeAnimFinished() != 0 {
        (*task).data[0] += 1;
        DestroyReelTimeSmokeSprite();
    }
}
pub(crate) unsafe extern "C" fn ReelTime_EndFailure(task: *mut Task) {
    gSpriteCoordOffsetX = 0;
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    PlayNewMapMusic((*sSlotMachine).backupMapMusic);
    DestroyReelTimePikachuSprite();
    DestroyBrokenReelTimeMachineSprite();
    DestroyReelTimeShadowSprites();
    DestroyReelTimeDuckSprites();
    DestroyTask(FindTaskIdByFunc(Some(Task_ReelTime)));
}
pub(crate) unsafe extern "C" fn LoadReelTimeWindowTilemap(a0: i16, a1: i16) {
    let mut i: i16 = 0;
    i = 4;
    while i < 15 {
        LoadBgTilemap(
            1,
            (&raw const sReelTimeWindow_Tilemap[a1 as i32 + (i as i32 - 4) * 20]).cast_mut()
                as *mut c_void,
            2,
            32 * i as u16 + a0 as u16,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ClearReelTimeWindowTilemap(a0: i16) {
    let mut i: u8 = 0;
    i = 4;
    while i < 15 {
        LoadBgTilemap(
            1,
            sEmptyTilemap.as_ptr().cast_mut() as *mut c_void,
            2,
            32 * i as u16 + a0 as u16,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn OpenInfoBox(digDisplayId: u8) {
    let mut taskId: u8 = CreateTask(Some(Task_InfoBox), 1);
    gTasks[taskId].data[1] = digDisplayId as i16;
    Task_InfoBox(taskId);
}
pub(crate) unsafe extern "C" fn IsInfoBoxClosed() -> u8 {
    if FindTaskIdByFunc(Some(Task_InfoBox)) == TASK_NONE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Task_InfoBox(taskId: u8) {
    sInfoBoxTasks[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]);
}
pub(crate) unsafe extern "C" fn InfoBox_FadeIn(task: *mut Task) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn InfoBox_WaitFade(task: *mut Task) {
    if gPaletteFade.active() == 0 {
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn InfoBox_DrawWindow(task: *mut Task) {
    DestroyDigitalDisplayScene();
    LoadInfoBoxTilemap();
    AddWindow((&raw const *sWindowTemplate_InfoBox).cast_mut());
    PutWindowTilemap(WIN_INFO);
    FillWindowPixelBuffer(WIN_INFO, 0);
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn InfoBox_AddText(task: *mut Task) {
    AddTextPrinterParameterized3(
        WIN_INFO,
        FONT_NORMAL,
        2,
        5,
        sColors_ReeltimeHelp.as_ptr().cast_mut(),
        0,
        gText_ReelTimeHelp.as_ptr().cast_mut(),
    );
    CopyWindowToVram(WIN_INFO, COPYWIN_FULL);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn InfoBox_WaitInput(task: *mut Task) {
    if gMain.newKeys as i32 & 6 != 0 {
        FillWindowPixelBuffer(WIN_INFO, 0);
        ClearWindowTilemap(WIN_INFO);
        CopyWindowToVram(WIN_INFO, COPYWIN_MAP);
        RemoveWindow(WIN_INFO);
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe extern "C" fn InfoBox_LoadSlotMachineTilemap(task: *mut Task) {
    LoadSlotMachineMenuTilemap();
    ShowBg(3);
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn InfoBox_CreateDigitalDisplay(task: *mut Task) {
    CreateDigitalDisplayScene((*task).data[1] as u8);
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn InfoBox_LoadPikaPowerMeter(task: *mut Task) {
    LoadPikaPowerMeter((*sSlotMachine).pikaPowerBolts);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn InfoBox_FreeTask(task: *mut Task) {
    DestroyTask(FindTaskIdByFunc(Some(Task_InfoBox)));
}
pub(crate) unsafe extern "C" fn CreateDigitalDisplayTask() {
    let mut i: u8 = 0;
    let mut task: *mut Task = null_mut();
    i = CreateTask(Some(Task_DigitalDisplay), 3);
    (*sSlotMachine).digDisplayTaskId = i;
    task = &raw mut gTasks[i];
    (*task).data[1] = -1;
    i = 4;
    while i < NUM_TASK_DATA {
        (*task).data[i] = MAX_SPRITES as i16;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateDigitalDisplayScene(id: u8) {
    let mut i: u8 = 0;
    let mut task: *mut Task = null_mut();
    DestroyDigitalDisplayScene();
    task = &raw mut gTasks[(*sSlotMachine).digDisplayTaskId];
    (*task).data[1] = id as i16;
    i = 0;
    while (*sDigitalDisplayScenes[id].at(i)).spriteTemplateId != 255 {
        let mut spriteId: u8 = 0;
        spriteId = CreateStdDigitalDisplaySprite(
            (*sDigitalDisplayScenes[id].at(i)).spriteTemplateId,
            (*sDigitalDisplayScenes[id].at(i)).dispInfoId,
            (*sDigitalDisplayScenes[id].at(i)).spriteId,
        );
        (*task).data[4 + i as i32] = spriteId as i16;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn AddDigitalDisplaySprite(
    templateIdx: u8,
    callback: Option<unsafe extern "C" fn(*mut Sprite)>,
    x: i16,
    y: i16,
    spriteId: i16,
) {
    let mut i: u8 = 0;
    let mut task: *mut Task = &raw mut gTasks[(*sSlotMachine).digDisplayTaskId];
    i = 4;
    while i < NUM_TASK_DATA {
        if (*task).data[i] == MAX_SPRITES as i16 {
            (*task).data[i] =
                CreateDigitalDisplaySprite(templateIdx, callback, x, y, spriteId) as i16;
            break;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DestroyDigitalDisplayScene() {
    let mut i: u8 = 0;
    let mut task: *mut Task = &raw mut gTasks[(*sSlotMachine).digDisplayTaskId];
    if (*task).data[1] as u16 != 0xFFFF {
        sDigitalDisplaySceneExitCallbacks[(*task).data[1]].unwrap_unchecked()();
    }
    i = 4;
    while i < NUM_TASK_DATA {
        if (*task).data[i] != MAX_SPRITES as i16 {
            DestroySprite(&raw mut gSprites[(*task).data[i]]);
            (*task).data[i] = MAX_SPRITES as i16;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn IsDigitalDisplayAnimFinished() -> u8 {
    let mut i: u8 = 0;
    let mut task: *mut Task = &raw mut gTasks[(*sSlotMachine).digDisplayTaskId];
    i = 4;
    while i < NUM_TASK_DATA {
        if (*task).data[i] != MAX_SPRITES as i16 {
            if gSprites[(*task).data[i]].data[7] != 0 {
                return FALSE;
            }
        }
        i += 1;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn Task_DigitalDisplay(taskId: u8) {
    sDigitalDisplayTasks[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId]);
}
pub(crate) unsafe extern "C" fn DigitalDisplay_Idle(task: *mut Task) {}
pub(crate) unsafe extern "C" fn CreateReelSymbolSprites() {
    let mut i: i16 = 0;
    let mut j: i16 = 0;
    let mut x: i16 = 0;
    i = 0;
    x = 0x30;
    while i < 3 {
        j = 0;
        while j < 120 {
            let mut sprite: *mut Sprite = gSprites.as_mut_ptr().at(CreateSprite(
                (&raw const *sSpriteTemplate_ReelSymbol).cast_mut(),
                x,
                0,
                14,
            ));
            (*sprite).oam.set_priority(3);
            (*sprite).data[0] = i;
            (*sprite).data[1] = j;
            (*sprite).data[3] = -1;
            j += 24;
        }
        i += 1;
        x += 0x28;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelSymbol(sprite: *mut Sprite) {
    (*sprite).data[2] = (*sSlotMachine).reelPixelOffsets[(*sprite).data[0]] + (*sprite).data[1];
    (*sprite).data[2] = (*sprite).data[2] % 120;
    (*sprite).y =
        (*sSlotMachine).reelShockOffsets[(*sprite).data[0]] as i16 + 28 + (*sprite).data[2];
    (*sprite).sheetTileStart = GetSpriteTileStartByTag(GetSymbolAtRest(
        (*sprite).data[0] as u8,
        (*sprite).data[2] / 24,
    ) as u16);
    SetSpriteSheetFrameTileNum(sprite);
}
pub(crate) unsafe extern "C" fn CreateCreditPayoutNumberSprites() {
    let mut i: i16 = 0;
    let mut x: i16 = 0;
    x = 203;
    i = 1;
    while i <= MAX_COINS {
        CreateCoinNumberSprite(x, 23, FALSE, i);
        i *= 10;
        x -= 7;
    }
    x = 235;
    i = 1;
    while i <= MAX_COINS {
        CreateCoinNumberSprite(x, 23, TRUE, i);
        i *= 10;
        x -= 7;
    }
}
pub(crate) unsafe extern "C" fn CreateCoinNumberSprite(
    x: i16,
    y: i16,
    isPayout: u8,
    digitMult: i16,
) {
    let mut sprite: *mut Sprite = &raw mut gSprites[CreateSprite(
        (&raw const *sSpriteTemplate_CoinNumber).cast_mut(),
        x,
        y,
        13,
    )];
    (*sprite).oam.set_priority(2);
    (*sprite).data[0] = isPayout as i16;
    (*sprite).data[1] = digitMult;
    (*sprite).data[2] = digitMult * 10;
    (*sprite).data[3] = -1;
}
pub(crate) unsafe extern "C" fn SpriteCB_CoinNumber(sprite: *mut Sprite) {
    let mut tag: u16 = (*sSlotMachine).coins as u16;
    if (*sprite).data[0] != 0 {
        tag = (*sSlotMachine).payout as u16;
    }
    if (*sprite).data[3] as i32 != tag as i32 {
        (*sprite).data[3] = tag as i16;
        tag = rem_i32(tag as i32, (*sprite).data[2] as u16 as i32) as u16;
        tag = div_i32(tag as i32, (*sprite).data[1] as u16 as i32) as u16;
        tag += GFXTAG_NUM_0;
        (*sprite).sheetTileStart = GetSpriteTileStartByTag(tag);
        SetSpriteSheetFrameTileNum(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreateReelBackgroundSprite() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelBackground).cast_mut(),
        88,
        72,
        15,
    );
    gSprites[spriteId].oam.set_priority(3);
    SetSubspriteTables(
        &raw mut gSprites[spriteId],
        sSubspriteTable_ReelBackground.as_ptr().cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn CreateReelTimePikachuSprite() {
    let mut spriteTemplate: SpriteTemplate = zeroed();
    let mut spriteId: u8 = 0;
    if sImageTable_ReelTimePikachu.is_null() {
        sImageTable_ReelTimePikachu = AllocZeroed(40) as *mut SpriteFrameImage;
    }
    (*sImageTable_ReelTimePikachu).data = sReelTimeGfxPtr as *mut c_void;
    (*sImageTable_ReelTimePikachu).size = 0x800;
    (*sImageTable_ReelTimePikachu.at(1)).data = sReelTimeGfxPtr.at(2048) as *mut c_void;
    (*sImageTable_ReelTimePikachu.at(1)).size = 0x800;
    (*sImageTable_ReelTimePikachu.at(2)).data = sReelTimeGfxPtr.at(4096) as *mut c_void;
    (*sImageTable_ReelTimePikachu.at(2)).size = 0x800;
    (*sImageTable_ReelTimePikachu.at(3)).data = sReelTimeGfxPtr.at(6144) as *mut c_void;
    (*sImageTable_ReelTimePikachu.at(3)).size = 0x800;
    (*sImageTable_ReelTimePikachu.at(4)).data = sReelTimeGfxPtr.at(8192) as *mut c_void;
    (*sImageTable_ReelTimePikachu.at(4)).size = 0x800;
    spriteTemplate = *sSpriteTemplate_ReelTimePikachu;
    spriteTemplate.images = sImageTable_ReelTimePikachu;
    spriteId = CreateSprite(&raw mut spriteTemplate, 280, 80, 1);
    gSprites[spriteId].oam.set_priority(1);
    gSprites[spriteId].set_coordOffsetEnabled(TRUE as u16);
    (*sSlotMachine).reelTimePikachuSpriteId = spriteId;
}
pub(crate) unsafe extern "C" fn DestroyReelTimePikachuSprite() {
    DestroySprite(&raw mut gSprites[(*sSlotMachine).reelTimePikachuSpriteId]);
    if !sImageTable_ReelTimePikachu.is_null() {
        Free(sImageTable_ReelTimePikachu as *mut c_void);
        sImageTable_ReelTimePikachu = null_mut();
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimePikachu(sprite: *mut Sprite) {
    (*sprite).y2 = {
        (*sprite).x2 = 0;
        (*sprite).x2
    };
    if (*sprite).animNum == 4 {
        (*sprite).y2 = {
            (*sprite).x2 = 8;
            (*sprite).x2
        };
        if (*sprite).animCmdIndex != 0 && (*sprite).animDelayCounter() != 0
            || (*sprite).animCmdIndex == 0 && (*sprite).animDelayCounter() == 0
        {
            (*sprite).y2 = -8;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimeMachineSprites() {
    let mut spriteTemplate: SpriteTemplate = zeroed();
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    if sImageTable_ReelTimeMachineAntennae.is_null() {
        sImageTable_ReelTimeMachineAntennae = AllocZeroed(8) as *mut SpriteFrameImage;
    }
    (*sImageTable_ReelTimeMachineAntennae).data = sReelTimeGfxPtr.at(10240) as *mut c_void;
    (*sImageTable_ReelTimeMachineAntennae).size = 0x300;
    spriteTemplate = *sSpriteTemplate_ReelTimeMachineAntennae;
    spriteTemplate.images = sImageTable_ReelTimeMachineAntennae;
    spriteId = CreateSprite(&raw mut spriteTemplate, 368, 52, 7);
    sprite = &raw mut gSprites[spriteId];
    (*sprite).oam.set_priority(1);
    (*sprite).set_coordOffsetEnabled(TRUE as u16);
    SetSubspriteTables(
        sprite,
        sSubspriteTable_ReelTimeMachineAntennae.as_ptr().cast_mut(),
    );
    (*sSlotMachine).reelTimeMachineSpriteIds[0] = spriteId;
    if sImageTable_ReelTimeMachine.is_null() {
        sImageTable_ReelTimeMachine = AllocZeroed(8) as *mut SpriteFrameImage;
    }
    (*sImageTable_ReelTimeMachine).data = sReelTimeGfxPtr.at(10240).at(768) as *mut c_void;
    (*sImageTable_ReelTimeMachine).size = 0x500;
    spriteTemplate = *sSpriteTemplate_ReelTimeMachine;
    spriteTemplate.images = sImageTable_ReelTimeMachine;
    spriteId = CreateSprite(&raw mut spriteTemplate, 368, 84, 7);
    sprite = &raw mut gSprites[spriteId];
    (*sprite).oam.set_priority(1);
    (*sprite).set_coordOffsetEnabled(TRUE as u16);
    SetSubspriteTables(sprite, sSubspriteTable_ReelTimeMachine.as_ptr().cast_mut());
    (*sSlotMachine).reelTimeMachineSpriteIds[1] = spriteId;
}
pub(crate) unsafe extern "C" fn CreateBrokenReelTimeMachineSprite() {
    let mut spriteTemplate: SpriteTemplate = zeroed();
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    if sImageTable_BrokenReelTimeMachine.is_null() {
        sImageTable_BrokenReelTimeMachine = AllocZeroed(8) as *mut SpriteFrameImage;
    }
    (*sImageTable_BrokenReelTimeMachine).data = sReelTimeGfxPtr.at(12288) as *mut c_void;
    (*sImageTable_BrokenReelTimeMachine).size = 0x600;
    spriteTemplate = *sSpriteTemplate_BrokenReelTimeMachine;
    spriteTemplate.images = sImageTable_BrokenReelTimeMachine;
    spriteId = CreateSprite(&raw mut spriteTemplate, 168 - gSpriteCoordOffsetX, 80, 7);
    sprite = &raw mut gSprites[spriteId];
    (*sprite).oam.set_priority(1);
    (*sprite).set_coordOffsetEnabled(TRUE as u16);
    SetSubspriteTables(
        sprite,
        sSubspriteTable_BrokenReelTimeMachine.as_ptr().cast_mut(),
    );
    (*sSlotMachine).reelTimeBrokenMachineSpriteId = spriteId;
}
pub(crate) unsafe extern "C" fn CreateReelTimeNumberSprites() {
    let mut i: u8 = 0;
    let mut r5: i16 = 0;
    i = 0;
    r5 = 0;
    while i < 3 {
        let mut spriteId: u8 = CreateSprite(
            (&raw const *sSpriteTemplate_ReelTimeNumbers).cast_mut(),
            368,
            0,
            10,
        );
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).oam.set_priority(1);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).data[7] = r5;
        (*sSlotMachine).reelTimeNumberSpriteIds[i] = spriteId;
        i += 1;
        r5 += 20;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimeNumbers(sprite: *mut Sprite) {
    let mut r0: i16 =
        (*sSlotMachine).reeltimePixelOffset as u16 as i16 + (*sprite).data[7] as u16 as i16;
    r0 = r0 % 40;
    (*sprite).y = r0 + 59;
    StartSpriteAnimIfDifferent(sprite, GetReelTimeSymbol(r0 / 20));
}
pub(crate) unsafe extern "C" fn CreateReelTimeShadowSprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeShadow).cast_mut(),
        368,
        100,
        9,
    );
    let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
    (*sprite).set_coordOffsetEnabled(TRUE as u16);
    (*sprite).oam.set_priority(1);
    SetSubspriteTables(sprite, sSubspriteTable_ReelTimeShadow.as_ptr().cast_mut());
    (*sSlotMachine).reelTimeShadowSpriteIds[0] = spriteId;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeShadow).cast_mut(),
        288,
        104,
        4,
    );
    sprite = &raw mut gSprites[spriteId];
    (*sprite).set_coordOffsetEnabled(TRUE as u16);
    (*sprite).oam.set_priority(1);
    SetSubspriteTables(sprite, sSubspriteTable_ReelTimeShadow.as_ptr().cast_mut());
    (*sSlotMachine).reelTimeShadowSpriteIds[1] = spriteId;
}
pub(crate) unsafe extern "C" fn CreateReelTimeNumberGapSprite() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeNumberGap).cast_mut(),
        368,
        76,
        11,
    );
    let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
    (*sprite).set_coordOffsetEnabled(TRUE as u16);
    (*sprite).oam.set_priority(1);
    SetSubspriteTables(
        sprite,
        sSubspriteTable_ReelTimeNumberGap.as_ptr().cast_mut(),
    );
    (*sSlotMachine).reelTimeNumberGapSpriteId = spriteId;
}
pub(crate) unsafe extern "C" fn DestroyReelTimeMachineSprites() {
    let mut i: u8 = 0;
    DestroySprite(&raw mut gSprites[(*sSlotMachine).reelTimeNumberGapSpriteId]);
    i = 0;
    while i < 2 {
        DestroySprite(&raw mut gSprites[(*sSlotMachine).reelTimeMachineSpriteIds[i]]);
        i += 1;
    }
    if !sImageTable_ReelTimeMachineAntennae.is_null() {
        Free(sImageTable_ReelTimeMachineAntennae as *mut c_void);
        sImageTable_ReelTimeMachineAntennae = null_mut();
    }
    if !sImageTable_ReelTimeMachine.is_null() {
        Free(sImageTable_ReelTimeMachine as *mut c_void);
        sImageTable_ReelTimeMachine = null_mut();
    }
    i = 0;
    while i < 3 {
        DestroySprite(&raw mut gSprites[(*sSlotMachine).reelTimeNumberSpriteIds[i]]);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DestroyReelTimeShadowSprites() {
    let mut i: u8 = 0;
    i = 0;
    while i < 2 {
        DestroySprite(&raw mut gSprites[(*sSlotMachine).reelTimeShadowSpriteIds[i]]);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DestroyBrokenReelTimeMachineSprite() {
    DestroySprite(&raw mut gSprites[(*sSlotMachine).reelTimeBrokenMachineSpriteId]);
    if !sImageTable_BrokenReelTimeMachine.is_null() {
        Free(sImageTable_BrokenReelTimeMachine as *mut c_void);
        sImageTable_BrokenReelTimeMachine = null_mut();
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimeBoltSprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeBolt).cast_mut(),
        152,
        32,
        5,
    );
    let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
    (*sprite).oam.set_priority(1);
    (*sprite).set_hFlip(TRUE as u16);
    (*sSlotMachine).reelTimeBoltSpriteIds[0] = spriteId;
    (*sprite).data[0] = 8;
    (*sprite).data[1] = -1;
    (*sprite).data[2] = -1;
    (*sprite).data[7] = 32;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeBolt).cast_mut(),
        184,
        32,
        5,
    );
    sprite = &raw mut gSprites[spriteId];
    (*sprite).oam.set_priority(1);
    (*sSlotMachine).reelTimeBoltSpriteIds[1] = spriteId;
    (*sprite).data[1] = 1;
    (*sprite).data[2] = -1;
    (*sprite).data[7] = 32;
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimeBolt(sprite: *mut Sprite) {
    if (*sprite).data[0] != 0 {
        (*sprite).data[0] -= 1;
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*sprite).set_invisible(TRUE as u16);
    } else {
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).x2 += (*sprite).data[1];
        (*sprite).y2 += (*sprite).data[2];
        if ({
            (*sprite).data[3] += 1;
            (*sprite).data[3]
        }) >= 8
        {
            (*sprite).data[0] = (*sprite).data[7];
            (*sprite).data[3] = 0;
        }
    }
}
pub(crate) unsafe extern "C" fn SetReelTimeBoltDelay(delay: i16) {
    gSprites[(*sSlotMachine).reelTimeBoltSpriteIds[0]].data[7] = delay;
    gSprites[(*sSlotMachine).reelTimeBoltSpriteIds[1]].data[7] = delay;
}
pub(crate) unsafe extern "C" fn DestroyReelTimeBoltSprites() {
    let mut i: u8 = 0;
    i = 0;
    while i < 2 {
        DestroySprite(&raw mut gSprites[(*sSlotMachine).reelTimeBoltSpriteIds[i]]);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimePikachuAuraSprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimePikachuAura).cast_mut(),
        72,
        80,
        3,
    );
    gSprites[spriteId].oam.set_priority(1);
    gSprites[spriteId].data[0] = TRUE as i16;
    gSprites[spriteId].data[5] = 0;
    gSprites[spriteId].data[6] = 16;
    gSprites[spriteId].data[7] = 8;
    (*sSlotMachine).reelTimePikachuAuraSpriteIds[0] = spriteId;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimePikachuAura).cast_mut(),
        104,
        80,
        3,
    );
    gSprites[spriteId].oam.set_priority(1);
    gSprites[spriteId].set_hFlip(TRUE as u16);
    (*sSlotMachine).reelTimePikachuAuraSpriteIds[1] = spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimePikachuAura(sprite: *mut Sprite) {
    let mut colors: CArray<u8, 2> = CArray([16, 0]);
    if (*sprite).data[0] != 0
        && ({
            (*sprite).data[6] -= 1;
            (*sprite).data[6]
        }) <= 0
    {
        MultiplyInvertedPaletteRGBComponents(
            0x100 + IndexOfSpritePaletteTag(PALTAG_PIKA_AURA) as u16 * 16 + 3,
            colors[(*sprite).data[5]],
            colors[(*sprite).data[5]],
            colors[(*sprite).data[5]],
        );
        (*sprite).data[5] += 1;
        (*sprite).data[5] &= 1;
        (*sprite).data[6] = (*sprite).data[7];
    }
}
pub(crate) unsafe extern "C" fn SetReelTimePikachuAuraFlashDelay(delay: i16) {
    gSprites[(*sSlotMachine).reelTimePikachuAuraSpriteIds[0]].data[7] = delay;
}
pub(crate) unsafe extern "C" fn DestroyReelTimePikachuAuraSprites() {
    let mut i: u8 = 0;
    MultiplyInvertedPaletteRGBComponents(
        0x100 + IndexOfSpritePaletteTag(PALTAG_PIKA_AURA) as u16 * 16 + 3,
        0,
        0,
        0,
    );
    i = 0;
    while i < 2 {
        DestroySprite(&raw mut gSprites[(*sSlotMachine).reelTimePikachuAuraSpriteIds[i]]);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimeExplosionSprite() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeExplosion).cast_mut(),
        168,
        80,
        6,
    );
    gSprites[spriteId].oam.set_priority(1);
    (*sSlotMachine).reelTimeExplosionSpriteId = spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimeExplosion(sprite: *mut Sprite) {
    (*sprite).y2 = gSpriteCoordOffsetY;
}
pub(crate) unsafe extern "C" fn DestroyReelTimeExplosionSprite() {
    DestroySprite(&raw mut gSprites[(*sSlotMachine).reelTimeExplosionSpriteId]);
}
pub(crate) unsafe extern "C" fn CreateReelTimeDuckSprites() {
    let mut i: u8 = 0;
    let mut sp: CArray<u16, 4> = CArray([0, 64, 128, 192]);
    i = 0;
    while i < 4 {
        let mut spriteId: u8 = CreateSprite(
            (&raw const *sSpriteTemplate_ReelTimeDuck).cast_mut(),
            80 - gSpriteCoordOffsetX,
            68,
            0,
        );
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).oam.set_priority(1);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).data[0] = sp[i] as i16;
        (*sSlotMachine).reelTimeDuckSpriteIds[i] = spriteId;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimeDuck(sprite: *mut Sprite) {
    (*sprite).data[0] -= 2;
    (*sprite).data[0] &= 0xff;
    (*sprite).x2 = Cos((*sprite).data[0], 20);
    (*sprite).y2 = Sin((*sprite).data[0], 6);
    (*sprite).subpriority = 0;
    if (*sprite).data[0] >= 0x80 {
        (*sprite).subpriority = 2;
    }
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) >= 16
    {
        (*sprite).set_hFlip((*sprite).hFlip() ^ 1);
        (*sprite).data[1] = 0;
    }
}
pub(crate) unsafe extern "C" fn DestroyReelTimeDuckSprites() {
    let mut i: u8 = 0;
    i = 0;
    while i < 4 {
        DestroySprite(&raw mut gSprites[(*sSlotMachine).reelTimeDuckSpriteIds[i]]);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateReelTimeSmokeSprite() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeSmoke).cast_mut(),
        168,
        60,
        8,
    );
    let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
    (*sprite).oam.set_priority(1);
    (*sprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    InitSpriteAffineAnim(sprite);
    (*sSlotMachine).reelTimeSmokeSpriteId = spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_ReelTimeSmoke(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        if (*sprite).affineAnimEnded() != 0 {
            (*sprite).data[0] += 1;
        }
    } else if (*sprite).data[0] == 1 {
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
        if ({
            (*sprite).data[2] += 1;
            (*sprite).data[2]
        }) >= 24
        {
            (*sprite).data[0] += 1;
            (*sprite).data[2] = 0;
        }
    } else {
        (*sprite).set_invisible(TRUE as u16);
        if ({
            (*sprite).data[2] += 1;
            (*sprite).data[2]
        }) >= 16
        {
            (*sprite).data[7] = TRUE as i16;
        }
    }
    (*sprite).data[1] &= 0xff;
    (*sprite).data[1] += 16;
    (*sprite).y2 -= (*sprite).data[1] >> 8;
}
pub(crate) unsafe extern "C" fn IsReelTimeSmokeAnimFinished() -> u8 {
    return gSprites[(*sSlotMachine).reelTimeSmokeSpriteId].data[7] as u8;
}
pub(crate) unsafe extern "C" fn DestroyReelTimeSmokeSprite() {
    let mut sprite: *mut Sprite = &raw mut gSprites[(*sSlotMachine).reelTimeSmokeSpriteId];
    FreeOamMatrix((*sprite).oam.matrixNum() as u8);
    DestroySprite(sprite);
}
pub(crate) unsafe extern "C" fn CreatePikaPowerBoltSprite(x: i16, y: i16) -> u8 {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_PikaPowerBolt).cast_mut(),
        x,
        y,
        12,
    );
    let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
    (*sprite).oam.set_priority(2);
    (*sprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    InitSpriteAffineAnim(sprite);
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_PikaPowerBolt(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        (*sprite).data[7] = TRUE as i16;
    }
}
pub(crate) unsafe extern "C" fn DestroyPikaPowerBoltSprite(spriteId: u8) {
    let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
    FreeOamMatrix((*sprite).oam.matrixNum() as u8);
    DestroySprite(sprite);
}
pub(crate) unsafe extern "C" fn CreateStdDigitalDisplaySprite(
    templateIdx: u8,
    dispInfoId: u8,
    spriteId: i16,
) -> u8 {
    return CreateDigitalDisplaySprite(
        templateIdx,
        sDigitalDisplay_SpriteCallbacks[dispInfoId],
        sDigitalDisplay_SpriteCoords[dispInfoId][0],
        sDigitalDisplay_SpriteCoords[dispInfoId][1],
        spriteId,
    );
}
pub(crate) unsafe extern "C" fn CreateDigitalDisplaySprite(
    templateIdx: u8,
    callback: Option<unsafe extern "C" fn(*mut Sprite)>,
    x: i16,
    y: i16,
    internalSpriteId: i16,
) -> u8 {
    let mut spriteTemplate: SpriteTemplate = zeroed();
    let mut spriteId: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    spriteTemplate = *sSpriteTemplates_DigitalDisplay[templateIdx];
    spriteTemplate.images = sImageTables_DigitalDisplay[templateIdx];
    spriteId = CreateSprite(&raw mut spriteTemplate, x, y, 16);
    sprite = &raw mut gSprites[spriteId];
    (*sprite).oam.set_priority(3);
    (*sprite).callback = callback;
    (*sprite).data[6] = internalSpriteId;
    (*sprite).data[7] = TRUE as i16;
    if !sSubspriteTables_DigitalDisplay[templateIdx].is_null() {
        SetSubspriteTables(sprite, sSubspriteTables_DigitalDisplay[templateIdx]);
    }
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_Static(sprite: *mut Sprite) {
    (*sprite).data[7] = FALSE as i16;
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_Smoke(sprite: *mut Sprite) {
    let mut targetX: CArray<i16, 4> = CArray([4, -4, 4, -4]);
    let mut targetY: CArray<i16, 4> = CArray([4, 4, -4, -4]);
    if ({
        let t1 = (*sprite).data[1];
        (*sprite).data[1] += 1;
        t1
    }) >= 16
    {
        (*sprite).set_subspriteTableNum((*sprite).subspriteTableNum() ^ 1);
        (*sprite).data[1] = 0;
    }
    (*sprite).x2 = 0;
    (*sprite).y2 = 0;
    if (*sprite).subspriteTableNum() != 0 {
        (*sprite).x2 = targetX[(*sprite).data[6]];
        (*sprite).y2 = targetY[(*sprite).data[6]];
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_SmokeNE(sprite: *mut Sprite) {
    (*sprite).set_hFlip(TRUE as u16);
    SpriteCB_DigitalDisplay_Smoke(sprite);
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_SmokeSW(sprite: *mut Sprite) {
    (*sprite).set_vFlip(TRUE as u16);
    SpriteCB_DigitalDisplay_Smoke(sprite);
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_SmokeSE(sprite: *mut Sprite) {
    (*sprite).set_hFlip(TRUE as u16);
    (*sprite).set_vFlip(TRUE as u16);
    SpriteCB_DigitalDisplay_Smoke(sprite);
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_Reel(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).x += 4;
            if (*sprite).x >= 208 {
                (*sprite).x = 208;
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 90
            {
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            (*sprite).x += 4;
            if (*sprite).x >= 272 {
                (*sprite).data[0] += 1;
            }
        }
        3 => {
            (*sprite).data[7] = FALSE as i16;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_Time(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).x -= 4;
            if (*sprite).x <= 208 {
                (*sprite).x = 208;
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 90
            {
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            (*sprite).x -= 4;
            if (*sprite).x <= 144 {
                (*sprite).data[0] += 1;
            }
        }
        3 => {
            (*sprite).data[7] = FALSE as i16;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_ReelTimeNumber(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            StartSpriteAnim(sprite, (*sSlotMachine).reelTimeSpinsLeft - 1);
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) >= 4
            {
                (*sprite).data[0] += 1;
                (*sprite).data[1] = 0;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            (*sprite).x += 4;
            if (*sprite).x >= 208 {
                (*sprite).x = 208;
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 90
            {
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            (*sprite).x += 4;
            if (*sprite).x >= 248 {
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            (*sprite).data[7] = FALSE as i16;
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_PokeballRocking(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).set_animPaused(TRUE);
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            (*sprite).y += 8;
            if (*sprite).y >= 0x70 {
                (*sprite).y = 0x70;
                (*sprite).data[1] = 16;
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if (*sprite).data[2] == 0 {
                (*sprite).y -= (*sprite).data[1];
                (*sprite).data[1] = -(*sprite).data[1];
                if ({
                    (*sprite).data[3] += 1;
                    (*sprite).data[3]
                }) >= 2
                {
                    (*sprite).data[1] >>= 2;
                    (*sprite).data[3] = 0;
                    if (*sprite).data[1] == 0 {
                        (*sprite).data[0] += 1;
                        (*sprite).data[7] = FALSE as i16;
                        (*sprite).set_animPaused(FALSE);
                    }
                }
            }
            (*sprite).data[2] += 1;
            (*sprite).data[2] &= 0x07;
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_Stop(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 8
            {
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            (*sprite).y += 2;
            if (*sprite).y >= 0x30 {
                (*sprite).y = 0x30;
                (*sprite).data[0] += 1;
                (*sprite).data[7] = FALSE as i16;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_AButtonStop(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).set_invisible(TRUE as u16);
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 0x20
            {
                (*sprite).data[0] += 1;
                (*sprite).data[1] = 5;
                (*sprite).oam.set_mosaic(TRUE as u32);
                (*sprite).set_invisible(FALSE as u16);
                StartSpriteAnim(sprite, 1);
                SetGpuReg(
                    REG_OFFSET_MOSAIC,
                    (((*sprite).data[1] as u16) << 4 | (*sprite).data[1] as u16) << 8,
                );
            }
        }
        1 => {
            (*sprite).data[1] -= (*sprite).data[2] >> 8;
            if (*sprite).data[1] < 0 {
                (*sprite).data[1] = 0;
            }
            SetGpuReg(
                REG_OFFSET_MOSAIC,
                (((*sprite).data[1] as u16) << 4 | (*sprite).data[1] as u16) << 8,
            );
            (*sprite).data[2] &= 0xff;
            (*sprite).data[2] += 0x80;
            if (*sprite).data[1] == 0 {
                (*sprite).data[0] += 1;
                (*sprite).data[7] = FALSE as i16;
                (*sprite).oam.set_mosaic(FALSE as u32);
                StartSpriteAnim(sprite, 0);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_PokeballShining(sprite: *mut Sprite) {
    if (*sprite).data[1] < 3 {
        LoadPalette(
            sPokeballShiningPalTable[(*sprite).data[1]] as *mut c_void,
            0x100 + IndexOfSpritePaletteTag(PALTAG_DIG_DISPLAY) as u16 * 16,
            32,
        );
        if ({
            (*sprite).data[2] += 1;
            (*sprite).data[2]
        }) >= 4
        {
            (*sprite).data[1] += 1;
            (*sprite).data[2] = 0;
        }
    } else {
        LoadPalette(
            sPokeballShiningPalTable[(*sprite).data[1]] as *mut c_void,
            0x100 + IndexOfSpritePaletteTag(PALTAG_DIG_DISPLAY) as u16 * 16,
            32,
        );
        if ({
            (*sprite).data[2] += 1;
            (*sprite).data[2]
        }) >= 25
        {
            (*sprite).data[1] = 0;
            (*sprite).data[2] = 0;
        }
    }
    StartSpriteAnimIfDifferent(sprite, 1);
    (*sprite).data[7] = FALSE as i16;
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_RegBonus(sprite: *mut Sprite) {
    let mut letterXOffset: CArray<i16, 8> = CArray([0, -40, 0, 0, 48, 0, 24, 0]);
    let mut letterYOffset: CArray<i16, 8> = CArray([-32, 0, -32, -48, 0, -48, 0, -48]);
    let mut letterDelay: CArray<i16, 8> = CArray([16, 12, 16, 0, 0, 4, 8, 8]);
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).x2 = letterXOffset[(*sprite).data[6]];
            (*sprite).y2 = letterYOffset[(*sprite).data[6]];
            (*sprite).data[1] = letterDelay[(*sprite).data[6]];
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if ({
                let t2 = (*sprite).data[1];
                (*sprite).data[1] -= 1;
                t2
            }) == 0
            {
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if (*sprite).x2 > 0 {
                (*sprite).x2 -= 4;
            } else if (*sprite).x2 < 0 {
                (*sprite).x2 += 4;
            }
            if (*sprite).y2 > 0 {
                (*sprite).y2 -= 4;
            } else if (*sprite).y2 < 0 {
                (*sprite).y2 += 4;
            }
            if (*sprite).x2 == 0 && (*sprite).y2 == 0 {
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_BigBonus(sprite: *mut Sprite) {
    let mut sp0: CArray<i16, 8> = CArray([160, 192, 224, 104, 80, 64, 48, 24]);
    if (*sprite).data[0] == 0 {
        (*sprite).data[0] += 1;
        (*sprite).data[1] = 12;
    }
    (*sprite).x2 = Cos(sp0[(*sprite).data[6]], (*sprite).data[1]);
    (*sprite).y2 = Sin(sp0[(*sprite).data[6]], (*sprite).data[1]);
    if (*sprite).data[1] != 0 {
        (*sprite).data[1] -= 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DigitalDisplay_AButtonStart(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sSlotMachine).winIn = 47;
            (*sSlotMachine).winOut = 63;
            (*sSlotMachine).win0v = 8328;
            (*sprite).set_invisible(TRUE as u16);
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            (*sprite).data[1] += 2;
            (*sprite).data[2] = (*sprite).data[1] + 176;
            (*sprite).data[3] = DISPLAY_WIDTH as i16 - (*sprite).data[1];
            if (*sprite).data[2] > 208 {
                (*sprite).data[2] = 208;
            }
            if (*sprite).data[3] < 208 {
                (*sprite).data[3] = 208;
            }
            (*sSlotMachine).win0h = ((*sprite).data[2] as u16) << 8 | (*sprite).data[3] as u16;
            if (*sprite).data[1] > 51 {
                (*sprite).data[0] += 1;
                (*sSlotMachine).winIn = 63;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if (*sSlotMachine).bet == 0 {
                break 'l1;
            }
            AddDigitalDisplaySprite(DIG_SPRITE_A_BUTTON, Some(SpriteCallbackDummy), 208, 116, 0);
            (*sSlotMachine).win0h = 49376;
            (*sSlotMachine).win0v = 26752;
            (*sSlotMachine).winIn = 47;
            (*sprite).data[0] += 1;
            (*sprite).data[1] = 0;
        }
        if fall || sw1 == 3 {
            fall = true;
            (*sprite).data[1] += 2;
            (*sprite).data[2] = (*sprite).data[1] + 192;
            (*sprite).data[3] = 224 - (*sprite).data[1];
            if (*sprite).data[2] > 208 {
                (*sprite).data[2] = 208;
            }
            if (*sprite).data[3] < 208 {
                (*sprite).data[3] = 208;
            }
            (*sSlotMachine).win0h = ((*sprite).data[2] as u16) << 8 | (*sprite).data[3] as u16;
            if (*sprite).data[1] > 15 {
                (*sprite).data[0] += 1;
                (*sSlotMachine).winIn = 63;
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn EndDigitalDisplayScene_Dummy() {}
pub(crate) unsafe extern "C" fn EndDigitalDisplayScene_StopReel() {
    SetGpuReg(REG_OFFSET_MOSAIC, 0);
}
pub(crate) unsafe extern "C" fn EndDigitalDisplayScene_Win() {
    LoadPalette(
        *sDigitalDisplay_Pal as *mut c_void,
        0x100 + IndexOfSpritePaletteTag(PALTAG_DIG_DISPLAY) as u16 * 16,
        32,
    );
}
pub(crate) unsafe extern "C" fn EndDigitalDisplayScene_InsertBet() {
    (*sSlotMachine).win0h = DISPLAY_WIDTH;
    (*sSlotMachine).win0v = DISPLAY_HEIGHT;
    (*sSlotMachine).winIn = 63;
    (*sSlotMachine).winOut = 63;
}
pub(crate) unsafe extern "C" fn LoadSlotMachineGfx() {
    let mut i: u8 = 0;
    LoadReelBackground();
    sDigitalDisplayGfxPtr = Alloc(0x3200) as *mut u8;
    LZDecompressWram(
        gSlotMachineDigitalDisplay_Gfx.as_ptr().cast_mut(),
        sDigitalDisplayGfxPtr as *mut c_void,
    );
    sReelTimeGfxPtr = Alloc(0x3600) as *mut u8;
    LZDecompressWram(
        sReelTimeGfx.as_ptr().cast_mut(),
        sReelTimeGfxPtr as *mut c_void,
    );
    sSlotMachineSpritesheetsPtr = AllocZeroed(176) as *mut SpriteSheet;
    i = 0;
    while i < 22 {
        (*sSlotMachineSpritesheetsPtr.at(i)).data = sSlotMachineSpriteSheets[i].data;
        (*sSlotMachineSpritesheetsPtr.at(i)).size = sSlotMachineSpriteSheets[i].size;
        (*sSlotMachineSpritesheetsPtr.at(i)).tag = sSlotMachineSpriteSheets[i].tag;
        i += 1;
    }
    (*sSlotMachineSpritesheetsPtr.at(17)).data = sDigitalDisplayGfxPtr.at(2560) as *mut c_void;
    (*sSlotMachineSpritesheetsPtr.at(18)).data = sDigitalDisplayGfxPtr.at(5120) as *mut c_void;
    (*sSlotMachineSpritesheetsPtr.at(19)).data = sDigitalDisplayGfxPtr.at(5632) as *mut c_void;
    (*sSlotMachineSpritesheetsPtr.at(20)).data = sDigitalDisplayGfxPtr.at(6400) as *mut c_void;
    LoadSpriteSheets(sSlotMachineSpritesheetsPtr);
    LoadSpritePalettes(sSlotMachineSpritePalettes.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn LoadReelBackground() {
    let mut dest: *mut u8 = null_mut();
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    sReelBackgroundSpriteSheet = AllocZeroed(8) as *mut SpriteSheet;
    sReelBackground_Gfx = AllocZeroed(0x2000) as *mut u8;
    dest = sReelBackground_Gfx;
    i = 0;
    while i < 0x40 {
        j = 0;
        while j < 0x20 {
            *dest = *(*sReelBackground_Tilemap).at(j);
            j += 1;
            dest = dest.at(1);
        }
        i += 1;
    }
    (*sReelBackgroundSpriteSheet).data = sReelBackground_Gfx as *mut c_void;
    (*sReelBackgroundSpriteSheet).size = 0x800;
    (*sReelBackgroundSpriteSheet).tag = GFXTAG_REEL_BG;
    LoadSpriteSheet(sReelBackgroundSpriteSheet);
}
pub(crate) unsafe extern "C" fn LoadMenuGfx() {
    sMenuGfx = Alloc(0x2200) as *mut u16;
    LZDecompressWram(
        gSlotMachineMenu_Gfx.as_ptr().cast_mut(),
        sMenuGfx as *mut c_void,
    );
    LoadBgTiles(2, sMenuGfx as *mut c_void, 0x2200, 0);
    LoadPalette(
        gSlotMachineMenu_Pal.as_ptr().cast_mut() as *mut c_void,
        0,
        160,
    );
    LoadPalette(sUnkPalette.as_ptr().cast_mut() as *mut c_void, 208, 32);
}
pub(crate) unsafe extern "C" fn LoadMenuAndReelOverlayTilemaps() {
    LoadSlotMachineMenuTilemap();
    LoadSlotMachineReelOverlay();
}
pub(crate) unsafe extern "C" fn LoadSlotMachineMenuTilemap() {
    LoadBgTilemap(
        2,
        gSlotMachineMenu_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0x500,
        0,
    );
}
pub(crate) unsafe extern "C" fn LoadSlotMachineReelOverlay() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut dx: i16 = 0;
    x = 4;
    while x < 18 {
        dx = 0;
        while dx < 4 {
            LoadBgTilemap(
                3,
                sReelOverlay_Tilemap as *mut c_void,
                2,
                x as u16 + dx as u16 + 160,
            );
            LoadBgTilemap(
                3,
                sReelOverlay_Tilemap.at(1) as *mut c_void,
                2,
                x as u16 + dx as u16 + 416,
            );
            LoadBgTilemap(
                3,
                sReelOverlay_Tilemap.at(2) as *mut c_void,
                2,
                x as u16 + dx as u16 + 192,
            );
            LoadBgTilemap(
                3,
                sReelOverlay_Tilemap.at(3) as *mut c_void,
                2,
                x as u16 + dx as u16 + 384,
            );
            dx += 1;
        }
        LoadBgTilemap(
            3,
            sReelOverlay_Tilemap.at(4) as *mut c_void,
            2,
            x as u16 + 192,
        );
        LoadBgTilemap(
            3,
            sReelOverlay_Tilemap.at(5) as *mut c_void,
            2,
            x as u16 + 384,
        );
        y = 7;
        while y <= 11 {
            LoadBgTilemap(
                3,
                sReelOverlay_Tilemap.at(6) as *mut c_void,
                2,
                x as u16 + y as u16 * 32,
            );
            y += 1;
        }
        x += 5;
    }
}
pub(crate) unsafe extern "C" fn SetReelButtonTilemap(
    offset: i16,
    topLeft: u16,
    topRight: u16,
    bottomLeft: u16,
    bottomRight: u16,
) {
    *sReelButtonPress_Tilemap = topLeft;
    *sReelButtonPress_Tilemap.at(1) = topRight;
    *sReelButtonPress_Tilemap.at(2) = bottomLeft;
    *sReelButtonPress_Tilemap.at(3) = bottomRight;
    LoadBgTilemap(
        2,
        sReelButtonPress_Tilemap as *mut c_void,
        2,
        480 + offset as u16,
    );
    LoadBgTilemap(
        2,
        sReelButtonPress_Tilemap.at(1) as *mut c_void,
        2,
        481 + offset as u16,
    );
    LoadBgTilemap(
        2,
        sReelButtonPress_Tilemap.at(2) as *mut c_void,
        2,
        512 + offset as u16,
    );
    LoadBgTilemap(
        2,
        sReelButtonPress_Tilemap.at(3) as *mut c_void,
        2,
        513 + offset as u16,
    );
}
pub(crate) unsafe extern "C" fn LoadInfoBoxTilemap() {
    LoadBgTilemap(
        2,
        gSlotMachineInfoBox_Tilemap.as_ptr().cast_mut() as *mut c_void,
        0x500,
        0,
    );
    HideBg(3);
}
pub(crate) unsafe extern "C" fn SetDigitalDisplayImagePtrs() {
    sImageTables_DigitalDisplay[0] = sImageTable_DigitalDisplay_Reel;
    sImageTables_DigitalDisplay[1] = sImageTable_DigitalDisplay_Time;
    sImageTables_DigitalDisplay[2] = sImageTable_DigitalDisplay_Insert;
    sImageTables_DigitalDisplay[3] = sImageTable_DigitalDisplay_Win;
    sImageTables_DigitalDisplay[4] = sImageTable_DigitalDisplay_Lose;
    sImageTables_DigitalDisplay[5] = sImageTable_DigitalDisplay_AButton;
    sImageTables_DigitalDisplay[6] = sImageTable_DigitalDisplay_Smoke;
    sImageTables_DigitalDisplay[7] = sImageTable_DigitalDisplay_Number;
    sImageTables_DigitalDisplay[8] = sImageTable_DigitalDisplay_Pokeball;
    sImageTables_DigitalDisplay[9] = sImageTable_DigitalDisplay_DPad;
    sImageTables_DigitalDisplay[10] = sImageTable_DigitalDisplay_Stop;
    sImageTables_DigitalDisplay[11] = sImageTable_DigitalDisplay_Stop;
    sImageTables_DigitalDisplay[12] = sImageTable_DigitalDisplay_Stop;
    sImageTables_DigitalDisplay[13] = sImageTable_DigitalDisplay_Stop;
    sImageTables_DigitalDisplay[14] = sImageTable_DigitalDisplay_Bonus;
    sImageTables_DigitalDisplay[15] = sImageTable_DigitalDisplay_Bonus;
    sImageTables_DigitalDisplay[16] = sImageTable_DigitalDisplay_Bonus;
    sImageTables_DigitalDisplay[17] = sImageTable_DigitalDisplay_Bonus;
    sImageTables_DigitalDisplay[18] = sImageTable_DigitalDisplay_Bonus;
    sImageTables_DigitalDisplay[19] = sImageTable_DigitalDisplay_Big;
    sImageTables_DigitalDisplay[20] = sImageTable_DigitalDisplay_Big;
    sImageTables_DigitalDisplay[21] = sImageTable_DigitalDisplay_Big;
    sImageTables_DigitalDisplay[22] = sImageTable_DigitalDisplay_Reg;
    sImageTables_DigitalDisplay[23] = sImageTable_DigitalDisplay_Reg;
    sImageTables_DigitalDisplay[24] = sImageTable_DigitalDisplay_Reg;
    sImageTables_DigitalDisplay[25] = null_mut();
}
pub(crate) unsafe extern "C" fn AllocDigitalDisplayGfx() {
    sImageTable_DigitalDisplay_Reel = AllocZeroed(8) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_Reel).data = sDigitalDisplayGfxPtr as *mut c_void;
    (*sImageTable_DigitalDisplay_Reel).size = 0x600;
    sImageTable_DigitalDisplay_Time = AllocZeroed(8) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_Time).data = sDigitalDisplayGfxPtr.at(1536) as *mut c_void;
    (*sImageTable_DigitalDisplay_Time).size = 0x200;
    sImageTable_DigitalDisplay_Insert = AllocZeroed(8) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_Insert).data = sDigitalDisplayGfxPtr.at(2048) as *mut c_void;
    (*sImageTable_DigitalDisplay_Insert).size = 0x200;
    sImageTable_DigitalDisplay_Stop = AllocZeroed(8) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_Stop).data = sDigitalDisplayGfxPtr.at(2560) as *mut c_void;
    (*sImageTable_DigitalDisplay_Stop).size = 0x200;
    sImageTable_DigitalDisplay_Win = AllocZeroed(8) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_Win).data = sDigitalDisplayGfxPtr.at(3072) as *mut c_void;
    (*sImageTable_DigitalDisplay_Win).size = 0x300;
    sImageTable_DigitalDisplay_Lose = AllocZeroed(8) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_Lose).data = sDigitalDisplayGfxPtr.at(4096) as *mut c_void;
    (*sImageTable_DigitalDisplay_Lose).size = 0x400;
    sImageTable_DigitalDisplay_Bonus = AllocZeroed(8) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_Bonus).data = sDigitalDisplayGfxPtr.at(5120) as *mut c_void;
    (*sImageTable_DigitalDisplay_Bonus).size = 0x200;
    sImageTable_DigitalDisplay_Big = AllocZeroed(8) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_Big).data = sDigitalDisplayGfxPtr.at(5632) as *mut c_void;
    (*sImageTable_DigitalDisplay_Big).size = 0x300;
    sImageTable_DigitalDisplay_Reg = AllocZeroed(8) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_Reg).data = sDigitalDisplayGfxPtr.at(6400) as *mut c_void;
    (*sImageTable_DigitalDisplay_Reg).size = 0x300;
    sImageTable_DigitalDisplay_AButton = AllocZeroed(16) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_AButton).data = sDigitalDisplayGfxPtr.at(7168) as *mut c_void;
    (*sImageTable_DigitalDisplay_AButton).size = 0x200;
    (*sImageTable_DigitalDisplay_AButton.at(1)).data =
        sDigitalDisplayGfxPtr.at(7680) as *mut c_void;
    (*sImageTable_DigitalDisplay_AButton.at(1)).size = 0x200;
    sImageTable_DigitalDisplay_Smoke = AllocZeroed(8) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_Smoke).data = sDigitalDisplayGfxPtr.at(8192) as *mut c_void;
    (*sImageTable_DigitalDisplay_Smoke).size = 640;
    sImageTable_DigitalDisplay_Number = AllocZeroed(40) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_Number).data = sDigitalDisplayGfxPtr.at(8832) as *mut c_void;
    (*sImageTable_DigitalDisplay_Number).size = 0x80;
    (*sImageTable_DigitalDisplay_Number.at(1)).data = sDigitalDisplayGfxPtr.at(8960) as *mut c_void;
    (*sImageTable_DigitalDisplay_Number.at(1)).size = 0x80;
    (*sImageTable_DigitalDisplay_Number.at(2)).data = sDigitalDisplayGfxPtr.at(9088) as *mut c_void;
    (*sImageTable_DigitalDisplay_Number.at(2)).size = 0x80;
    (*sImageTable_DigitalDisplay_Number.at(3)).data = sDigitalDisplayGfxPtr.at(9216) as *mut c_void;
    (*sImageTable_DigitalDisplay_Number.at(3)).size = 0x80;
    (*sImageTable_DigitalDisplay_Number.at(4)).data = sDigitalDisplayGfxPtr.at(9344) as *mut c_void;
    (*sImageTable_DigitalDisplay_Number.at(4)).size = 0x80;
    sImageTable_DigitalDisplay_Pokeball = AllocZeroed(16) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_Pokeball).data = sDigitalDisplayGfxPtr.at(9728) as *mut c_void;
    (*sImageTable_DigitalDisplay_Pokeball).size = 0x480;
    (*sImageTable_DigitalDisplay_Pokeball.at(1)).data =
        sDigitalDisplayGfxPtr.at(10880) as *mut c_void;
    (*sImageTable_DigitalDisplay_Pokeball.at(1)).size = 0x480;
    sImageTable_DigitalDisplay_DPad = AllocZeroed(16) as *mut SpriteFrameImage;
    (*sImageTable_DigitalDisplay_DPad).data = sDigitalDisplayGfxPtr.at(12032) as *mut c_void;
    (*sImageTable_DigitalDisplay_DPad).size = 0x180;
    (*sImageTable_DigitalDisplay_DPad.at(1)).data = sDigitalDisplayGfxPtr.at(12416) as *mut c_void;
    (*sImageTable_DigitalDisplay_DPad.at(1)).size = 0x180;
}
