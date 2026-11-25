//! Translated from `src/slot_machine.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::agb_main::{SetHBlankCallback, SetVBlankCallback};
use crate::bg::{HideBg, ResetBgsAndClearDma3BusyFlags, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
use crate::coins::{GetCoins, SetCoins};
#[allow(unused_imports)]
use crate::consts::*;
use crate::field_effect::{MultiplyInvertedPaletteRGBComponents, MultiplyPaletteRGBComponents};
use crate::gpu_regs::{EnableInterrupts, SetGpuReg};
use crate::main_menu::CreateYesNoMenuParameterized;
use crate::menu::{
    AddTextPrinterParameterized3, ClearDialogWindowAndFrame, DrawDialogueFrame,
    Menu_ProcessInputNoWrapClearOnChoose,
};
use crate::overworld::IncrementGameStat;
use crate::palette::{
    BeginNormalPaletteFade, LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::random::Random;
use crate::sound::{
    GetCurrentMapMusic, IsFanfareTaskInactive, PlayFanfare, PlayNewMapMusic, PlaySE, StopMapMusic,
};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeOamMatrix, GetSpriteTileStartByTag,
    IndexOfSpritePaletteTag, LoadOam, ProcessSpriteCopyRequests, ResetSpriteData, gOamLimit,
    gSpriteCoordOffsetX, gSpriteCoordOffsetY,
};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{gTasks, task_get, task_set};
use crate::text::DeactivateAllTextPrinters;
use crate::text_window::{LoadMessageBoxGfx, LoadUserWindowBorderGfx};
use crate::trig::{Cos, Sin};
use crate::tv::{
    AlertTVThatPlayerPlayedSlotMachine, IncrementDailySlotsUses, TryPutFindThatGamerOnAir,
};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::{LoadWordFromTwoHalfwords, StoreWordInTwoHalfwords};
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap, RemoveWindow,
};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `CreateInvisibleSprite` with this module's view of its types.
#[inline]
fn CreateInvisibleSprite(a0: Option<unsafe fn(*mut Sprite)>) -> u8 {
    unsafe { crate::sprite::CreateInvisibleSprite(core::mem::transmute(a0)) }
}
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn InitSpriteAffineAnim(a0: *mut Sprite) {
    unsafe {
        crate::sprite::InitSpriteAffineAnim(a0 as _);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
/// `LoadBgTilemap` with this module's view of its types.
#[inline]
unsafe fn LoadBgTilemap(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16 {
    unsafe { crate::bg::LoadBgTilemap(a0, a1 as _, a2, a3) }
}
/// `LoadBgTiles` with this module's view of its types.
#[inline]
unsafe fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16 {
    unsafe { crate::bg::LoadBgTiles(a0, a1 as _, a2, a3) }
}
/// `LoadSpritePalettes` with this module's view of its types.
#[inline]
fn LoadSpritePalettes(a0: *mut SpritePalette) {
    unsafe {
        crate::sprite::LoadSpritePalettes(a0 as _);
    }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
/// `LoadSpriteSheets` with this module's view of its types.
#[inline]
fn LoadSpriteSheets(a0: *mut SpriteSheet) {
    unsafe {
        crate::sprite::LoadSpriteSheets(a0 as _);
    }
}
/// `SetSpriteSheetFrameTileNum` with this module's view of its types.
#[inline]
fn SetSpriteSheetFrameTileNum(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SetSpriteSheetFrameTileNum(a0 as _);
    }
}
/// `SetSubspriteTables` with this module's view of its types.
#[inline]
unsafe fn SetSubspriteTables(a0: *mut Sprite, a1: *mut SubspriteTable) {
    unsafe {
        crate::sprite::SetSubspriteTables(a0 as _, a1 as _);
    }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
/// `StartSpriteAnimIfDifferent` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnimIfDifferent(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const sFlashPal: usize = 0;
const sIsPayout: usize = 0;
const sMatchLineId: usize = 0;
const sState: usize = 0;
const tMachineId: usize = 0;
const tState: usize = 0;
const sDigitMin: usize = 1;
const sFlashing: usize = 1;
const sMoveY: usize = 1;
const sXDir: usize = 1;
const tExitCallback: usize = 1;
const tExtraTurns: usize = 1;
const tNumBolts: usize = 1;
const tReelSpeed: usize = 1;
const tShockMagnitude: usize = 1;
const sDigitMax: usize = 2;
const sFlashState: usize = 2;
const sNumFullFlashes: usize = 2;
const sTimer: usize = 2;
const sYDir: usize = 2;
const tSpriteId: usize = 2;
const tTimer3: usize = 2;
const sCurNum: usize = 3;
const sFlashDir: usize = 3;
const sColor: usize = 4;
const tRtReelSpeed: usize = 4;
const sColorIdx: usize = 5;
const sColorIncr: usize = 5;
const tTimer1: usize = 5;
const sSpriteId: usize = 6;
const tExplodeChecks: usize = 6;
const sAnimFinished: usize = 7;
const sAtOriginalColor: usize = 7;
const sDelay: usize = 7;
const sWaitForAnim: usize = 7;
const tMoving: usize = 14;
const tAnimating: usize = 15;
const tReelId: usize = 15;
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
    pub prevMainCb: Option<unsafe fn()>,
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
static sDecideStop_Bias: Table<CArray<Option<unsafe fn() -> u8>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_Bias).cast());
static sDecideStop_Bias_Reel1_Bets: Table<CArray<Option<unsafe fn(u8, u8) -> u8>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_Bias_Reel1_Bets).cast());
static sDecideStop_Bias_Reel2_Bets: Table<CArray<Option<unsafe fn() -> u8>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_Bias_Reel2_Bets).cast());
static sDecideStop_Bias_Reel3_Bets: Table<CArray<Option<unsafe fn(u8) -> u8>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_Bias_Reel3_Bets).cast());
static sDecideStop_NoBias: Table<CArray<Option<unsafe fn()>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_NoBias).cast());
static sDecideStop_NoBias_Reel2_Bets: Table<CArray<Option<unsafe fn()>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_NoBias_Reel2_Bets).cast());
static sDecideStop_NoBias_Reel3_Bets: Table<CArray<Option<unsafe fn()>, 3>> =
    Table((&raw const crate::data::slot_machine::sDecideStop_NoBias_Reel3_Bets).cast());
static sDigitalDisplaySceneExitCallbacks: Table<CArray<Option<unsafe fn()>, 7>> =
    Table((&raw const crate::data::slot_machine::sDigitalDisplaySceneExitCallbacks).cast());
static sDigitalDisplayScenes: Table<CArray<*mut DigitalDisplaySprite, 7>> =
    Table((&raw const crate::data::slot_machine::sDigitalDisplayScenes).cast());
static sDigitalDisplayTasks: Table<CArray<Option<unsafe fn(*mut Task)>, 1>> =
    Table((&raw const crate::data::slot_machine::sDigitalDisplayTasks).cast());
static sDigitalDisplay_Pal: Table<*mut u16> =
    Table((&raw const crate::data::slot_machine::sDigitalDisplay_Pal).cast());
static sDigitalDisplay_SpriteCallbacks: Table<CArray<Option<unsafe fn(*mut Sprite)>, 35>> =
    Table((&raw const crate::data::slot_machine::sDigitalDisplay_SpriteCallbacks).cast());
static sDigitalDisplay_SpriteCoords: Table<CArray<CArray<i16, 2>, 35>> =
    Table((&raw const crate::data::slot_machine::sDigitalDisplay_SpriteCoords).cast());
static sEmptyTilemap: Table<CArray<u16, 1>> =
    Table((&raw const crate::data::slot_machine::sEmptyTilemap).cast());
static sFlashingLightsPalTable: Table<CArray<*mut u16, 3>> =
    Table((&raw const crate::data::slot_machine::sFlashingLightsPalTable).cast());
static sInfoBoxTasks: Table<CArray<Option<unsafe fn(*mut Task)>, 15>> =
    Table((&raw const crate::data::slot_machine::sInfoBoxTasks).cast());
static sInitialReelPositions: Table<CArray<CArray<i16, 2>, 3>> =
    Table((&raw const crate::data::slot_machine::sInitialReelPositions).cast());
static sLitMatchLinePalTable: Table<CArray<*mut u16, 5>> =
    Table((&raw const crate::data::slot_machine::sLitMatchLinePalTable).cast());
static sMatchLinePalOffsets: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::slot_machine::sMatchLinePalOffsets).cast());
static sMatchLinesPerBet: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::slot_machine::sMatchLinesPerBet).cast());
static sPayoutTasks: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 3>> =
    Table((&raw const crate::data::slot_machine::sPayoutTasks).cast());
static sPikaPowerBoltTasks: Table<CArray<Option<unsafe fn(*mut Task)>, 4>> =
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
static sReelStopButtonTasks: Table<CArray<Option<unsafe fn(*mut Task, u8)>, 3>> =
    Table((&raw const crate::data::slot_machine::sReelStopButtonTasks).cast());
static sReelStopShocks: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::slot_machine::sReelStopShocks).cast());
static sReelSymbols: Table<CArray<CArray<u8, 21>, 3>> =
    Table((&raw const crate::data::slot_machine::sReelSymbols).cast());
static sReelTasks: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 5>> =
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
static sReelTimeTasks: Table<CArray<Option<unsafe fn(*mut Task)>, 19>> =
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
static sSlotTasks: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 29>> =
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

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sMenuGfx: crate::global::Global<*mut u16> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sSelectedPikaPowerTile: crate::global::Global<*mut u16> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sReelOverlay_Tilemap: crate::global::Global<*mut u16> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sDigitalDisplayGfxPtr: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sReelTimeGfxPtr: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sReelButtonPress_Tilemap: crate::global::Global<*mut u16> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sReelBackground_Gfx: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_ReelTimePikachu: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_ReelTimeMachineAntennae: crate::global::Global<
    *mut SpriteFrameImage,
> = crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_ReelTimeMachine: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_BrokenReelTimeMachine: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_Reel: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_Time: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_Insert: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_Stop: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_Win: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_Lose: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_Bonus: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_Big: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_Reg: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_AButton: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_Smoke: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_Number: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_Pokeball: crate::global::Global<
    *mut SpriteFrameImage,
> = crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sImageTable_DigitalDisplay_DPad: crate::global::Global<*mut SpriteFrameImage> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sReelBackgroundSpriteSheet: crate::global::Global<*mut SpriteSheet> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sSlotMachineSpritesheetsPtr: crate::global::Global<*mut SpriteSheet> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sSlotMachine: crate::global::Global<*mut SlotMachine> =
    crate::global::Global::new(null_mut());
pub(crate) static sImageTables_DigitalDisplay: crate::global::Global<
    CArray<*mut SpriteFrameImage, 26>,
> = crate::global::Global::new(unsafe { zeroed() });

/// `AddTextPrinterParameterized` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized(
    a0: u8,
    a1: u8,
    a2: *mut u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe {
        crate::text::AddTextPrinterParameterized(
            a0,
            a1,
            a2 as _,
            a3,
            a4,
            a5,
            core::mem::transmute(a6),
        )
    }
}
/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn Task_FadeToSlotMachine(taskId: u8) {
    match task_get(taskId, tState) {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 if (*gPaletteFade.as_ptr()).active() == 0 => {
            SetMainCallback2(Some(CB2_SlotMachineSetup));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub unsafe fn PlaySlotMachine(machineId: u8, exitCallback: Option<unsafe fn()>) {
    crate::gl!(sSlotMachine = AllocZeroed(104) as *mut SlotMachine);
    PlaySlotMachine_Internal(machineId, exitCallback);
    let taskId: u8 = CreateTask(Some(Task_FadeToSlotMachine), 0);
    task_set(taskId, tState, 0);
}
pub(crate) unsafe fn CB2_SlotMachineSetup() {
    match crate::gl!(gMain.state) {
        0 => {
            SlotMachineSetup_InitBgsWindows();
            InitSlotMachine();
            crate::gl!(gMain.state += 1);
        }
        1 => {
            SlotMachineSetup_InitVRAM();
            crate::gl!(gMain.state += 1);
        }
        2 => {
            SlotMachineSetup_InitOAM();
            SlotMachineSetup_InitGpuRegs();
            crate::gl!(gMain.state += 1);
        }
        3 => {
            SlotMachineSetup_InitPalsSpritesTasks();
            crate::gl!(gMain.state += 1);
        }
        4 => {
            SlotMachineSetup_InitTilemaps();
            crate::gl!(gMain.state += 1);
        }
        5 => {
            SlotMachineSetup_LoadGfxAndTilemaps();
            crate::gl!(gMain.state += 1);
        }
        6 => {
            SlotMachineSetup_InitVBlank();
            crate::gl!(gMain.state += 1);
        }
        7 => {
            BeginNormalPaletteFade(0xffffffff, 0, 0x10, 0, 0);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            crate::gl!(gMain.state += 1);
        }
        8 => {
            AllocDigitalDisplayGfx();
            crate::gl!(gMain.state += 1);
        }
        9 => {
            SetDigitalDisplayImagePtrs();
            crate::gl!(gMain.state += 1);
        }
        10 => {
            CreateSlotMachineSprites();
            CreateGameplayTasks();
            crate::gl!(gMain.state += 1);
        }
        11 => {
            SetMainCallback2(Some(CB2_SlotMachine));
        }
        _ => {}
    }
}
pub(crate) unsafe fn CB2_SlotMachine() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn SlotMachine_VBlankCB() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    SetGpuReg(REG_OFFSET_WIN0H, (*crate::gl!(sSlotMachine)).win0h);
    SetGpuReg(REG_OFFSET_WIN0V, (*crate::gl!(sSlotMachine)).win0v);
    SetGpuReg(REG_OFFSET_WININ, (*crate::gl!(sSlotMachine)).winIn);
    SetGpuReg(REG_OFFSET_WINOUT, (*crate::gl!(sSlotMachine)).winOut);
}
unsafe fn PlaySlotMachine_Internal(machineId: u8, exitCallback: Option<unsafe fn()>) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[CreateTask(Some(SlotMachineDummyTask), 0xFF)];
    (*task).data[tMachineId] = machineId as i16;
    StoreWordInTwoHalfwords(
        &raw mut (*task).data[tExitCallback] as *mut u16,
        core::mem::transmute::<_, usize>(exitCallback) as i32 as u32,
    );
}
unsafe fn SlotMachine_InitFromTask() {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[FindTaskIdByFunc(Some(SlotMachineDummyTask))];
    (*crate::gl!(sSlotMachine)).machineId = (*task).data[tMachineId] as u8;
    LoadWordFromTwoHalfwords(
        &raw mut (*task).data[tExitCallback] as *mut u16,
        &raw mut (*crate::gl!(sSlotMachine)).prevMainCb as *mut u32,
    );
}
pub(crate) unsafe fn SlotMachineDummyTask(taskId: u8) {}
unsafe fn SlotMachineSetup_InitBgsWindows() {
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
unsafe fn SlotMachineSetup_InitVBlank() {
    SetVBlankCallback(Some(SlotMachine_VBlankCB));
    EnableInterrupts(INTR_FLAG_VBLANK);
    SetGpuReg(REG_OFFSET_DISPCNT, 12352);
}
unsafe fn SlotMachineSetup_InitVRAM() {
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
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
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
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
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
unsafe fn SlotMachineSetup_InitOAM() {
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
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
}
unsafe fn SlotMachineSetup_InitGpuRegs() {
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
unsafe fn InitSlotMachine() {
    SlotMachine_InitFromTask();
    (*crate::gl!(sSlotMachine)).state = SLOTTASK_UNFADE;
    (*crate::gl!(sSlotMachine)).pikaPowerBolts = 0;
    (*crate::gl!(sSlotMachine)).luckyGame = Random() as u8 & 1;
    (*crate::gl!(sSlotMachine)).machineBias = 0;
    (*crate::gl!(sSlotMachine)).matches = 0;
    (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft = 0;
    (*crate::gl!(sSlotMachine)).reelTimeSpinsUsed = 0;
    (*crate::gl!(sSlotMachine)).coins = GetCoins() as i16;
    (*crate::gl!(sSlotMachine)).payout = 0;
    (*crate::gl!(sSlotMachine)).netCoinLoss = 0;
    (*crate::gl!(sSlotMachine)).bet = 0;
    (*crate::gl!(sSlotMachine)).currentReel = LEFT_REEL as i16;
    (*crate::gl!(sSlotMachine)).reelSpeed = REEL_NORMAL_SPEED;
    (*crate::gl!(sSlotMachine)).win0h = DISPLAY_WIDTH;
    (*crate::gl!(sSlotMachine)).win0v = DISPLAY_HEIGHT;
    (*crate::gl!(sSlotMachine)).winIn = 63;
    (*crate::gl!(sSlotMachine)).winOut = 63;
    (*crate::gl!(sSlotMachine)).backupMapMusic = GetCurrentMapMusic();
    for i in 0..NUM_REELS {
        (*crate::gl!(sSlotMachine)).reelShockOffsets[i] = 0;
        (*crate::gl!(sSlotMachine)).reelPositions[i] =
            sInitialReelPositions[i][(*crate::gl!(sSlotMachine)).luckyGame] % 21;
        (*crate::gl!(sSlotMachine)).reelPixelOffsets[i] =
            REEL_HEIGHT - (*crate::gl!(sSlotMachine)).reelPositions[i] * REEL_SYMBOL_HEIGHT;
        (*crate::gl!(sSlotMachine)).reelPixelOffsets[i] %= 504;
    }
    AlertTVThatPlayerPlayedSlotMachine(GetCoins());
}
unsafe fn SlotMachineSetup_InitPalsSpritesTasks() {
    ResetPaletteFade();
    ResetSpriteData();
    gOamLimit.set(0x80);
    FreeAllSpritePalettes();
    ResetTasks();
}
unsafe fn SlotMachineSetup_InitTilemaps() {
    crate::gl!(sSelectedPikaPowerTile = Alloc(8) as *mut u16);
    crate::gl!(sReelOverlay_Tilemap = AllocZeroed(14) as *mut u16);
    crate::gl!(sReelButtonPress_Tilemap = AllocZeroed(8) as *mut u16);
    *crate::gl!(sReelOverlay_Tilemap) = 0x2051;
    *(*sReelOverlay_Tilemap.as_ptr()).at(1) = 0x2851;
    *(*sReelOverlay_Tilemap.as_ptr()).at(2) = 0x2061;
    *(*sReelOverlay_Tilemap.as_ptr()).at(3) = 0x2861;
    *(*sReelOverlay_Tilemap.as_ptr()).at(4) = 0x20BE;
    *(*sReelOverlay_Tilemap.as_ptr()).at(5) = 0x28BE;
    *(*sReelOverlay_Tilemap.as_ptr()).at(6) = 0x20BF;
}
unsafe fn SlotMachineSetup_LoadGfxAndTilemaps() {
    LoadMenuGfx();
    LoadMenuAndReelOverlayTilemaps();
    LoadSlotMachineGfx();
    LoadMessageBoxGfx(0, 0x200, 240);
    LoadUserWindowBorderGfx(0, 0x214, 224);
    PutWindowTilemap(WIN_MSG);
}
unsafe fn CreateSlotMachineSprites() {
    CreateReelSymbolSprites();
    CreateCreditPayoutNumberSprites();
    CreateInvisibleFlashMatchLineSprites();
    CreateReelBackgroundSprite();
}
unsafe fn CreateGameplayTasks() {
    CreatePikaPowerBoltTask();
    CreateReelTasks();
    CreateDigitalDisplayTask();
    CreateSlotMachineTasks();
}
unsafe fn CreateSlotMachineTasks() {
    Task_SlotMachine(CreateTask(Some(Task_SlotMachine), 0));
}
pub(crate) unsafe fn Task_SlotMachine(taskId: u8) {
    while sSlotTasks[(*crate::gl!(sSlotMachine)).state].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
pub(crate) unsafe fn SlotTask_UnfadeScreen(task: *mut Task) -> u8 {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    LoadPikaPowerMeter((*crate::gl!(sSlotMachine)).pikaPowerBolts);
    (*crate::gl!(sSlotMachine)).state += 1;
    FALSE
}
pub(crate) unsafe fn SlotTask_WaitUnfade(task: *mut Task) -> u8 {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        (*crate::gl!(sSlotMachine)).state += 1;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_ReadyNewSpin(task: *mut Task) -> u8 {
    (*crate::gl!(sSlotMachine)).payout = 0;
    (*crate::gl!(sSlotMachine)).bet = 0;
    (*crate::gl!(sSlotMachine)).currentReel = LEFT_REEL as i16;
    (*crate::gl!(sSlotMachine)).machineBias &= 192;
    (*crate::gl!(sSlotMachine)).state = SLOTTASK_ASK_INSERT_BET;
    if (*crate::gl!(sSlotMachine)).coins <= 0 {
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_MSG_NO_MORE_COINS;
    } else if (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft != 0 {
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_READY_NEW_RT_SPIN;
        CreateDigitalDisplayScene(DIG_DISPLAY_REEL_TIME);
    }
    TRUE
}
pub(crate) unsafe fn SlotTask_ReadyNewReelTimeSpin(task: *mut Task) -> u8 {
    if IsDigitalDisplayAnimFinished() != 0 {
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_ASK_INSERT_BET;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_AskInsertBet(task: *mut Task) -> u8 {
    CreateDigitalDisplayScene(DIG_DISPLAY_INSERT_BET);
    (*crate::gl!(sSlotMachine)).state = SLOTTASK_BET_INPUT;
    if (*crate::gl!(sSlotMachine)).coins >= MAX_COINS {
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_MSG_MAX_COINS;
    }
    TRUE
}
pub(crate) unsafe fn SlotTask_HandleBetInput(task: *mut Task) -> u8 {
    if crate::gl!(gMain.newKeys) as i32 & SELECT_BUTTON != 0 {
        OpenInfoBox(DIG_DISPLAY_INSERT_BET);
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_WAIT_INFO_BOX;
    } else if crate::gl!(gMain.newKeys) as i32 & R_BUTTON != 0 {
        if (*crate::gl!(sSlotMachine)).coins as i32
            - (MAX_BET as i32 - (*crate::gl!(sSlotMachine)).bet as i32)
            >= 0
        {
            for i in (*crate::gl!(sSlotMachine)).bet..MAX_BET {
                LightenBetTiles(i as u8);
            }
            (*crate::gl!(sSlotMachine)).coins -= MAX_BET - (*crate::gl!(sSlotMachine)).bet;
            (*crate::gl!(sSlotMachine)).bet = MAX_BET;
            (*crate::gl!(sSlotMachine)).state = SLOTTASK_START_SPIN;
            PlaySE(SE_SHOP);
        } else {
            (*crate::gl!(sSlotMachine)).state = SLOTTASK_MSG_NEED_3_COINS;
        }
    } else {
        if crate::gl!(gMain.newKeys) as i32 & DPAD_DOWN != 0
            && (*crate::gl!(sSlotMachine)).coins != 0
        {
            PlaySE(SE_SHOP);
            LightenBetTiles((*crate::gl!(sSlotMachine)).bet as u8);
            (*crate::gl!(sSlotMachine)).coins -= 1;
            (*crate::gl!(sSlotMachine)).bet += 1;
        }
        if (*crate::gl!(sSlotMachine)).bet >= MAX_BET
            || (*crate::gl!(sSlotMachine)).bet != 0
                && crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0
        {
            (*crate::gl!(sSlotMachine)).state = SLOTTASK_START_SPIN;
        }
        if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
            (*crate::gl!(sSlotMachine)).state = SLOTTASK_ASK_QUIT;
        }
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_PrintMsg_Need3Coins(task: *mut Task) -> u8 {
    DrawDialogueFrame(WIN_MSG, FALSE);
    AddTextPrinterParameterized(
        WIN_MSG,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_YouDontHaveThreeCoins).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        None,
    );
    CopyWindowToVram(WIN_MSG, COPYWIN_FULL);
    (*crate::gl!(sSlotMachine)).state = SLOTTASK_WAIT_MSG_NEED_3_COINS;
    FALSE
}
pub(crate) unsafe fn SlotTask_WaitMsg_Need3Coins(task: *mut Task) -> u8 {
    if crate::gl!(gMain.newKeys) as i32 & 3 != 0 {
        ClearDialogWindowAndFrame(WIN_MSG, TRUE);
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_BET_INPUT;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_WaitInfoBox(task: *mut Task) -> u8 {
    if IsInfoBoxClosed() != 0 {
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_BET_INPUT;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_StartSpin(task: *mut Task) -> u8 {
    DrawMachineBias();
    DestroyDigitalDisplayScene();
    SpinSlotReel(LEFT_REEL);
    SpinSlotReel(MIDDLE_REEL);
    SpinSlotReel(RIGHT_REEL);
    IncrementDailySlotsUses();
    (*task).data[0] = 0;
    if (*crate::gl!(sSlotMachine)).machineBias as i32 & BIAS_REELTIME != 0 {
        BeginReelTime();
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_START_RT_SPIN;
    } else {
        CreateDigitalDisplayScene(DIG_DISPLAY_STOP_REEL);
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_RESET_BIAS_FAILURE;
    }
    (*crate::gl!(sSlotMachine)).reelSpeed = REEL_NORMAL_SPEED;
    if (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft != 0 {
        (*crate::gl!(sSlotMachine)).reelSpeed = ReelTimeSpeed() as i16;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_StartReelTimeSpin(task: *mut Task) -> u8 {
    if IsReelTimeTaskDone() != 0 {
        CreateDigitalDisplayScene(DIG_DISPLAY_STOP_REEL);
        (*crate::gl!(sSlotMachine)).machineBias &= 223;
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_RESET_BIAS_FAILURE;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_ResetBiasFailure(task: *mut Task) -> u8 {
    if ({
        (*task).data[0] += 1;
        (*task).data[0]
    }) >= 30
    {
        ResetBiasFailure();
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_WAIT_REEL_STOP;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_WaitReelStop(task: *mut Task) -> u8 {
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        PlaySE(SE_CONTEST_PLACE);
        StopSlotReel((*crate::gl!(sSlotMachine)).currentReel as u8);
        PressStopReelButton((*crate::gl!(sSlotMachine)).currentReel as u8);
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_WAIT_ALL_REELS_STOP;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_WaitAllReelsStop(task: *mut Task) -> u8 {
    if IsSlotReelMoving((*crate::gl!(sSlotMachine)).currentReel as u8) == 0 {
        (*crate::gl!(sSlotMachine)).currentReel += 1;
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_WAIT_REEL_STOP;
        if (*crate::gl!(sSlotMachine)).currentReel >= NUM_REELS as i16 {
            (*crate::gl!(sSlotMachine)).state = SLOTTASK_CHECK_MATCHES;
        }
        return TRUE;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_CheckMatches(task: *mut Task) -> u8 {
    (*crate::gl!(sSlotMachine)).machineBias &= 192;
    CheckMatch();
    if (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft != 0 {
        (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft -= 1;
        (*crate::gl!(sSlotMachine)).reelTimeSpinsUsed += 1;
    }
    if (*crate::gl!(sSlotMachine)).matches != 0 {
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_WAIT_PAYOUT;
        AwardPayout();
        FlashSlotMachineLights();
        if ({
            (*crate::gl!(sSlotMachine)).netCoinLoss -= (*crate::gl!(sSlotMachine)).payout;
            (*crate::gl!(sSlotMachine)).netCoinLoss
        }) < 0
        {
            (*crate::gl!(sSlotMachine)).netCoinLoss = 0;
        }
        if (*crate::gl!(sSlotMachine)).matches as i32 & 384 != 0 {
            PlayFanfare(MUS_SLOTS_JACKPOT);
            CreateDigitalDisplayScene(DIG_DISPLAY_BONUS_BIG);
        } else if (*crate::gl!(sSlotMachine)).matches as i32 & 64 != 0 {
            PlayFanfare(MUS_SLOTS_JACKPOT);
            CreateDigitalDisplayScene(DIG_DISPLAY_BONUS_REG);
        } else {
            PlayFanfare(MUS_SLOTS_WIN);
            CreateDigitalDisplayScene(DIG_DISPLAY_WIN);
        }
        if (*crate::gl!(sSlotMachine)).matches as i32 & 448 != 0 {
            (*crate::gl!(sSlotMachine)).machineBias &= 63;
            if (*crate::gl!(sSlotMachine)).matches as i32 & 384 != 0 {
                (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft = 0;
                (*crate::gl!(sSlotMachine)).reelTimeSpinsUsed = 0;
                (*crate::gl!(sSlotMachine)).luckyGame = FALSE;
                if (*crate::gl!(sSlotMachine)).matches as i32 & 256 != 0 {
                    (*crate::gl!(sSlotMachine)).luckyGame = TRUE;
                }
            }
        }
        if (*crate::gl!(sSlotMachine)).matches as i32 & 32 != 0
            && (*crate::gl!(sSlotMachine)).pikaPowerBolts < 16
        {
            (*crate::gl!(sSlotMachine)).pikaPowerBolts += 1;
            AddPikaPowerBolt((*crate::gl!(sSlotMachine)).pikaPowerBolts);
        }
    } else {
        CreateDigitalDisplayScene(DIG_DISPLAY_LOSE);
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_NO_MATCHES;
        if ({
            (*crate::gl!(sSlotMachine)).netCoinLoss += (*crate::gl!(sSlotMachine)).bet;
            (*crate::gl!(sSlotMachine)).netCoinLoss
        }) > MAX_COINS
        {
            (*crate::gl!(sSlotMachine)).netCoinLoss = MAX_COINS;
        }
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_WaitPayout(task: *mut Task) -> u8 {
    if IsFinalTask_Task_Payout() != 0 {
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_END_PAYOUT;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_EndPayout(task: *mut Task) -> u8 {
    if TryStopSlotMachineLights() != 0 {
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_RESET_BET_TILES;
        if (*crate::gl!(sSlotMachine)).matches as i32 & 384 != 0 {
            IncrementGameStat(GAME_STAT_SLOT_JACKPOTS);
        }
        if (*crate::gl!(sSlotMachine)).matches as i32 & 4 != 0 {
            (*crate::gl!(sSlotMachine)).currentReel = LEFT_REEL as i16;
            (*crate::gl!(sSlotMachine)).state = SLOTTASK_START_SPIN;
        }
        if (*crate::gl!(sSlotMachine)).matches as i32 & 32 != 0 {
            (*crate::gl!(sSlotMachine)).state = SLOTTASK_MATCHED_POWER;
        }
        if (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft != 0
            && (*crate::gl!(sSlotMachine)).matches as i32 & 4 != 0
        {
            CreateDigitalDisplayScene(DIG_DISPLAY_REEL_TIME);
            (*crate::gl!(sSlotMachine)).state = SLOTTASK_WAIT_RT_ANIM;
        }
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_MatchedPower(task: *mut Task) -> u8 {
    if IsPikaPowerBoltAnimating() == 0 {
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_RESET_BET_TILES;
        if (*crate::gl!(sSlotMachine)).matches as i32 & 4 != 0 {
            (*crate::gl!(sSlotMachine)).state = SLOTTASK_START_SPIN;
            if (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft != 0 {
                CreateDigitalDisplayScene(DIG_DISPLAY_REEL_TIME);
                (*crate::gl!(sSlotMachine)).state = SLOTTASK_WAIT_RT_ANIM;
            }
        }
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_WaitReelTimeAnim(task: *mut Task) -> u8 {
    if IsDigitalDisplayAnimFinished() != 0 {
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_RESET_BET_TILES;
        if (*crate::gl!(sSlotMachine)).matches as i32 & 4 != 0 {
            (*crate::gl!(sSlotMachine)).state = SLOTTASK_START_SPIN;
        }
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_ResetBetTiles(task: *mut Task) -> u8 {
    DarkenBetTiles(0);
    DarkenBetTiles(1);
    DarkenBetTiles(2);
    (*crate::gl!(sSlotMachine)).state = SLOTTASK_READY_NEW_SPIN;
    FALSE
}
pub(crate) unsafe fn SlotTask_NoMatches(task: *mut Task) -> u8 {
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) > 64
    {
        (*task).data[1] = 0;
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_RESET_BET_TILES;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_AskQuit(task: *mut Task) -> u8 {
    DrawDialogueFrame(WIN_MSG, FALSE);
    AddTextPrinterParameterized(
        WIN_MSG,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_QuitTheGame).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        None,
    );
    CopyWindowToVram(WIN_MSG, COPYWIN_FULL);
    CreateYesNoMenuParameterized(0x15, 7, 0x214, 0x180, 0xE, 0xF);
    (*crate::gl!(sSlotMachine)).state = SLOTTASK_HANDLE_QUIT_INPUT;
    FALSE
}
pub(crate) unsafe fn SlotTask_HandleQuitInput(task: *mut Task) -> u8 {
    let input: i8 = Menu_ProcessInputNoWrapClearOnChoose();
    if input == 0 {
        ClearDialogWindowAndFrame(WIN_MSG, TRUE);
        DarkenBetTiles(0);
        DarkenBetTiles(1);
        DarkenBetTiles(2);
        (*crate::gl!(sSlotMachine)).coins += (*crate::gl!(sSlotMachine)).bet;
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_END;
    } else if input == 1 || input == MENU_B_PRESSED {
        ClearDialogWindowAndFrame(WIN_MSG, TRUE);
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_BET_INPUT;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_PrintMsg_MaxCoins(task: *mut Task) -> u8 {
    DrawDialogueFrame(WIN_MSG, FALSE);
    AddTextPrinterParameterized(
        WIN_MSG,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_YouveGot9999Coins).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        None,
    );
    CopyWindowToVram(WIN_MSG, COPYWIN_FULL);
    (*crate::gl!(sSlotMachine)).state = SLOTTASK_WAIT_MSG_MAX_COINS;
    FALSE
}
pub(crate) unsafe fn SlotTask_WaitMsg_MaxCoins(task: *mut Task) -> u8 {
    if crate::gl!(gMain.newKeys) as i32 & 3 != 0 {
        ClearDialogWindowAndFrame(WIN_MSG, TRUE);
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_BET_INPUT;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_PrintMsg_NoMoreCoins(task: *mut Task) -> u8 {
    DrawDialogueFrame(WIN_MSG, FALSE);
    AddTextPrinterParameterized(
        WIN_MSG,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_YouveRunOutOfCoins).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        None,
    );
    CopyWindowToVram(WIN_MSG, COPYWIN_FULL);
    (*crate::gl!(sSlotMachine)).state = SLOTTASK_WAIT_MSG_NO_MORE_COINS;
    FALSE
}
pub(crate) unsafe fn SlotTask_WaitMsg_NoMoreCoins(task: *mut Task) -> u8 {
    if crate::gl!(gMain.newKeys) as i32 & 3 != 0 {
        ClearDialogWindowAndFrame(WIN_MSG, TRUE);
        (*crate::gl!(sSlotMachine)).state = SLOTTASK_END;
    }
    FALSE
}
pub(crate) unsafe fn SlotTask_EndGame(task: *mut Task) -> u8 {
    SetCoins((*crate::gl!(sSlotMachine)).coins as u16);
    TryPutFindThatGamerOnAir(GetCoins());
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    (*crate::gl!(sSlotMachine)).state += 1;
    FALSE
}
pub(crate) unsafe fn SlotTask_FreeDataStructures(task: *mut Task) -> u8 {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        SetMainCallback2((*crate::gl!(sSlotMachine)).prevMainCb);
        Free(crate::gl!(sImageTable_DigitalDisplay_Reel) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_Reel = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_Time) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_Time = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_Insert) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_Insert = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_Stop) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_Stop = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_Win) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_Win = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_Lose) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_Lose = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_Bonus) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_Bonus = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_Big) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_Big = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_Reg) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_Reg = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_AButton) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_AButton = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_Smoke) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_Smoke = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_Number) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_Number = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_Pokeball) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_Pokeball = null_mut());
        Free(crate::gl!(sImageTable_DigitalDisplay_DPad) as *mut c_void);
        crate::gl!(sImageTable_DigitalDisplay_DPad = null_mut());
        if !(*sImageTable_ReelTimePikachu.as_ptr()).is_null() {
            Free(crate::gl!(sImageTable_ReelTimePikachu) as *mut c_void);
            crate::gl!(sImageTable_ReelTimePikachu = null_mut());
        }
        if !(*sImageTable_ReelTimeMachineAntennae.as_ptr()).is_null() {
            Free(crate::gl!(sImageTable_ReelTimeMachineAntennae) as *mut c_void);
            crate::gl!(sImageTable_ReelTimeMachineAntennae = null_mut());
        }
        if !(*sImageTable_ReelTimeMachine.as_ptr()).is_null() {
            Free(crate::gl!(sImageTable_ReelTimeMachine) as *mut c_void);
            crate::gl!(sImageTable_ReelTimeMachine = null_mut());
        }
        if !(*sImageTable_BrokenReelTimeMachine.as_ptr()).is_null() {
            Free(crate::gl!(sImageTable_BrokenReelTimeMachine) as *mut c_void);
            crate::gl!(sImageTable_BrokenReelTimeMachine = null_mut());
        }
        Free(crate::gl!(sMenuGfx) as *mut c_void);
        crate::gl!(sMenuGfx = null_mut());
        Free(crate::gl!(sSelectedPikaPowerTile) as *mut c_void);
        crate::gl!(sSelectedPikaPowerTile = null_mut());
        Free(crate::gl!(sReelOverlay_Tilemap) as *mut c_void);
        crate::gl!(sReelOverlay_Tilemap = null_mut());
        Free(crate::gl!(sDigitalDisplayGfxPtr) as *mut c_void);
        crate::gl!(sDigitalDisplayGfxPtr = null_mut());
        Free(crate::gl!(sReelTimeGfxPtr) as *mut c_void);
        crate::gl!(sReelTimeGfxPtr = null_mut());
        Free(crate::gl!(sReelButtonPress_Tilemap) as *mut c_void);
        crate::gl!(sReelButtonPress_Tilemap = null_mut());
        Free(crate::gl!(sReelBackground_Gfx) as *mut c_void);
        crate::gl!(sReelBackground_Gfx = null_mut());
        Free(crate::gl!(sReelBackgroundSpriteSheet) as *mut c_void);
        crate::gl!(sReelBackgroundSpriteSheet = null_mut());
        Free(crate::gl!(sSlotMachineSpritesheetsPtr) as *mut c_void);
        crate::gl!(sSlotMachineSpritesheetsPtr = null_mut());
        Free(crate::gl!(sSlotMachine) as *mut c_void);
        crate::gl!(sSlotMachine = null_mut());
    }
    FALSE
}
unsafe fn DrawMachineBias() {
    let mut whichBias: u8 = 0;
    if (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft == 0
        && (*crate::gl!(sSlotMachine)).machineBias as i32 & 192 == 0
    {
        if ShouldTrySpecialBias() != 0 {
            whichBias = TrySelectBias_Special();
            if whichBias != 3 {
                (*crate::gl!(sSlotMachine)).machineBias |= sBiasesSpecial[whichBias] as u8;
                if whichBias != 1 {
                    return;
                }
            }
        }
        whichBias = TrySelectBias_Regular();
        if whichBias != 5 {
            (*crate::gl!(sSlotMachine)).machineBias |= sBiasesRegular[whichBias] as u8;
        }
    }
}
unsafe fn ResetBiasFailure() {
    (*crate::gl!(sSlotMachine)).didNotFailBias = FALSE;
    if (*crate::gl!(sSlotMachine)).machineBias != 0 {
        (*crate::gl!(sSlotMachine)).didNotFailBias = TRUE;
    }
}
unsafe fn GetBiasSymbol(mut machineBias: u8) -> u8 {
    for i in 0..8u8 {
        if machineBias as i32 & 1 != 0 {
            return sBiasSymbols[i];
        }
        machineBias >>= 1;
    }
    0
}
unsafe fn ShouldTrySpecialBias() -> u8 {
    let rval: u8 = Random() as u8;
    if sSpecialDrawOdds[(*crate::gl!(sSlotMachine)).machineId]
        [(*crate::gl!(sSlotMachine)).bet as i32 - 1]
        > rval
    {
        return TRUE;
    }
    FALSE
}
unsafe fn TrySelectBias_Special() -> u8 {
    let mut whichBias: i16 = 0;
    while whichBias < 3 {
        let rval: i16 = Random() as i16 & 0xff;
        let value: i16 =
            sBiasProbabilities_Special[whichBias][(*crate::gl!(sSlotMachine)).machineId] as i16;
        if value > rval {
            break;
        }
        whichBias += 1;
    }
    whichBias as u8
}
unsafe fn TrySelectBias_Regular() -> u8 {
    let mut whichBias: i16 = 0;
    while whichBias < 5 {
        let rval: i16 = Random() as i16 & 0xff;
        let mut value: i16 =
            sBiasProbabilities_Regular[whichBias][(*crate::gl!(sSlotMachine)).machineId] as i16;
        if whichBias == 0 && (*crate::gl!(sSlotMachine)).luckyGame == TRUE {
            value += 10;
            if value > 0x100 {
                value = 0x100;
            }
        } else if whichBias == 4 && (*crate::gl!(sSlotMachine)).luckyGame == TRUE {
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
    whichBias as u8
}
unsafe fn GetReelTimeSpinProbability(spins: u8) -> u8 {
    if (*crate::gl!(sSlotMachine)).luckyGame == FALSE {
        return sReelTimeProbabilities_NormalGame[spins]
            [(*crate::gl!(sSlotMachine)).pikaPowerBolts];
    } else {
        return sReelTimeProbabilities_LuckyGame[spins][(*crate::gl!(sSlotMachine)).pikaPowerBolts];
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetReelTimeDraw() {
    (*crate::gl!(sSlotMachine)).reelTimeDraw = 0;
    let mut rval: u8 = Random() as u8;
    if rval < GetReelTimeSpinProbability(0) {
        return;
    }
    let mut spins: i16 = 5;
    while spins > 0 {
        rval = Random() as u8;
        if rval < GetReelTimeSpinProbability(spins as u8) {
            break;
        }
        spins -= 1;
    }
    (*crate::gl!(sSlotMachine)).reelTimeDraw = spins as u8;
}
unsafe fn ShouldReelTimeMachineExplode(check: u16) -> u8 {
    let rval: u16 = Random() & 0xff;
    if rval < sReelTimeExplodeProbability[check] {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn ReelTimeSpeed() -> u16 {
    let mut i: u8 = 0;
    if (*crate::gl!(sSlotMachine)).netCoinLoss >= 300 {
        i = 4;
    } else if (*crate::gl!(sSlotMachine)).netCoinLoss >= 250 {
        i = 3;
    } else if (*crate::gl!(sSlotMachine)).netCoinLoss >= 200 {
        i = 2;
    } else if (*crate::gl!(sSlotMachine)).netCoinLoss >= 150 {
        i = 1;
    }
    let mut rval: u8 = (Random() as i32 % 100) as u8;
    let mut value: u8 = sReelTimeSpeed_Probabilities[i][0] as u8;
    if rval < value {
        return REEL_HALF_SPEED;
    }
    rval = (Random() as i32 % 100) as u8;
    value = sReelTimeSpeed_Probabilities[i][1] as u8
        + sQuarterSpeed_ProbabilityBoost[(*crate::gl!(sSlotMachine)).reelTimeSpinsUsed] as u8;
    if rval < value {
        return REEL_QUARTER_SPEED;
    }
    REEL_NORMAL_SPEED as u16
}
unsafe fn CheckMatch() {
    (*crate::gl!(sSlotMachine)).matches = 0;
    CheckMatch_CenterRow();
    if (*crate::gl!(sSlotMachine)).bet > 1 {
        CheckMatch_TopAndBottom();
    }
    if (*crate::gl!(sSlotMachine)).bet > 2 {
        CheckMatch_Diagonals();
    }
}
unsafe fn CheckMatch_CenterRow() {
    let sym1: u8 = GetSymbolAtRest(LEFT_REEL, 2);
    let sym2: u8 = GetSymbolAtRest(MIDDLE_REEL, 2);
    let sym3: u8 = GetSymbolAtRest(RIGHT_REEL, 2);
    let r#match: u8 = GetMatchFromSymbols(sym1, sym2, sym3);
    if r#match != MATCH_NONE {
        (*crate::gl!(sSlotMachine)).payout += sSlotPayouts[r#match] as i16;
        (*crate::gl!(sSlotMachine)).matches |= sSlotMatchFlags[r#match];
        FlashMatchLine(MATCH_MIDDLE_ROW);
    }
}
unsafe fn CheckMatch_TopAndBottom() {
    let mut sym1: u8 = GetSymbolAtRest(LEFT_REEL, 1);
    let mut sym2: u8 = GetSymbolAtRest(MIDDLE_REEL, 1);
    let mut sym3: u8 = GetSymbolAtRest(RIGHT_REEL, 1);
    let mut r#match: u8 = GetMatchFromSymbols(sym1, sym2, sym3);
    if r#match != MATCH_NONE {
        if r#match == MATCH_CHERRY {
            r#match = MATCH_TOPBOT_CHERRY;
        }
        (*crate::gl!(sSlotMachine)).payout += sSlotPayouts[r#match] as i16;
        (*crate::gl!(sSlotMachine)).matches |= sSlotMatchFlags[r#match];
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
        (*crate::gl!(sSlotMachine)).payout += sSlotPayouts[r#match] as i16;
        (*crate::gl!(sSlotMachine)).matches |= sSlotMatchFlags[r#match];
        FlashMatchLine(MATCH_BOTTOM_ROW);
    }
}
unsafe fn CheckMatch_Diagonals() {
    let mut sym1: u8 = GetSymbolAtRest(LEFT_REEL, 1);
    let mut sym2: u8 = GetSymbolAtRest(MIDDLE_REEL, 2);
    let mut sym3: u8 = GetSymbolAtRest(RIGHT_REEL, 3);
    let mut r#match: u8 = GetMatchFromSymbols(sym1, sym2, sym3);
    if r#match != MATCH_NONE {
        if r#match != MATCH_CHERRY {
            (*crate::gl!(sSlotMachine)).payout += sSlotPayouts[r#match] as i16;
            (*crate::gl!(sSlotMachine)).matches |= sSlotMatchFlags[r#match];
        }
        FlashMatchLine(MATCH_NWSE_DIAG);
    }
    sym1 = GetSymbolAtRest(LEFT_REEL, 3);
    sym2 = GetSymbolAtRest(MIDDLE_REEL, 2);
    sym3 = GetSymbolAtRest(RIGHT_REEL, 1);
    r#match = GetMatchFromSymbols(sym1, sym2, sym3);
    if r#match != MATCH_NONE {
        if r#match != MATCH_CHERRY {
            (*crate::gl!(sSlotMachine)).payout += sSlotPayouts[r#match] as i16;
            (*crate::gl!(sSlotMachine)).matches |= sSlotMatchFlags[r#match];
        }
        FlashMatchLine(MATCH_NESW_DIAG);
    }
}
fn GetMatchFromSymbols(sym1: u8, sym2: u8, sym3: u8) -> u8 {
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
    MATCH_NONE
}
unsafe fn AwardPayout() {
    Task_Payout(CreateTask(Some(Task_Payout), 4));
}
unsafe fn IsFinalTask_Task_Payout() -> u8 {
    if FindTaskIdByFunc(Some(Task_Payout)) == TAIL_SENTINEL {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Task_Payout(taskId: u8) {
    while sPayoutTasks[task_get(taskId, 0)].unwrap_unchecked()(&raw mut (*gTasks.as_ptr())[taskId])
        != 0
    {}
}
pub(crate) unsafe fn PayoutTask_Init(task: *mut Task) -> u8 {
    if IsMatchLineDoneFlashingBeforePayout() != 0 {
        (*task).data[tState] += 1;
        if (*crate::gl!(sSlotMachine)).payout == 0 {
            (*task).data[tState] = PAYOUT_TASK_FREE;
            return TRUE;
        }
    }
    FALSE
}
pub(crate) unsafe fn PayoutTask_GivePayout(task: *mut Task) -> u8 {
    if ({
        let t1 = (*task).data[1];
        (*task).data[1] -= 1;
        t1
    }) == 0
    {
        if IsFanfareTaskInactive() != 0 {
            PlaySE(SE_PIN);
        }
        (*crate::gl!(sSlotMachine)).payout -= 1;
        if (*crate::gl!(sSlotMachine)).coins < MAX_COINS {
            (*crate::gl!(sSlotMachine)).coins += 1;
        }
        (*task).data[1] = 8;
        if crate::gl!(gMain.heldKeys) as i32 & A_BUTTON != 0 {
            (*task).data[1] = 4;
        }
    }
    if IsFanfareTaskInactive() != 0 && crate::gl!(gMain.newKeys) as i32 & START_BUTTON != 0 {
        PlaySE(SE_PIN);
        (*crate::gl!(sSlotMachine)).coins += (*crate::gl!(sSlotMachine)).payout;
        if (*crate::gl!(sSlotMachine)).coins > MAX_COINS {
            (*crate::gl!(sSlotMachine)).coins = MAX_COINS;
        }
        (*crate::gl!(sSlotMachine)).payout = 0;
    }
    if (*crate::gl!(sSlotMachine)).payout == 0 {
        (*task).data[tState] += 1;
    }
    FALSE
}
pub(crate) unsafe fn PayoutTask_Free(task: *mut Task) -> u8 {
    if TryStopMatchLinesFlashing() != 0 {
        DestroyTask(FindTaskIdByFunc(Some(Task_Payout)));
    }
    FALSE
}
unsafe fn GetSymbolAtRest(reel: u8, offset: i16) -> u8 {
    let mut pos: i16 =
        (((*crate::gl!(sSlotMachine)).reelPositions[reel] as i32 + offset as i32) % 21) as i16;
    if pos < 0 {
        pos += SYMBOLS_PER_REEL;
    }
    sReelSymbols[reel][pos]
}
unsafe fn GetSymbol(reel: u8, offset: i16) -> u8 {
    let mut inc: i16 = 0;
    let pixelOffset: i16 = (*crate::gl!(sSlotMachine)).reelPixelOffsets[reel] % 24;
    if pixelOffset != 0 {
        inc = -1;
    }
    GetSymbolAtRest(reel, offset + inc)
}
unsafe fn GetReelTimeSymbol(offset: i16) -> u8 {
    let mut newPosition: i16 =
        (((*crate::gl!(sSlotMachine)).reeltimePosition as i32 + offset as i32) % 6) as i16;
    if newPosition < 0 {
        newPosition += REELTIME_SYMBOLS;
    }
    sReelTimeSymbols[newPosition]
}
unsafe fn AdvanceSlotReel(reelIndex: u8, value: i16) {
    (*crate::gl!(sSlotMachine)).reelPixelOffsets[reelIndex] += value;
    (*crate::gl!(sSlotMachine)).reelPixelOffsets[reelIndex] %= 504;
    (*crate::gl!(sSlotMachine)).reelPositions[reelIndex] =
        SYMBOLS_PER_REEL - (*crate::gl!(sSlotMachine)).reelPixelOffsets[reelIndex] / 24;
}
pub unsafe fn AdvanceSlotReelToNextSymbol(reelIndex: u8, mut value: i16) -> i16 {
    let mut offset: i16 = (*crate::gl!(sSlotMachine)).reelPixelOffsets[reelIndex] % 24;
    if offset != 0 {
        if offset < value {
            value = offset;
        }
        AdvanceSlotReel(reelIndex, value);
        offset = (*crate::gl!(sSlotMachine)).reelPixelOffsets[reelIndex] % 24;
    }
    offset
}
unsafe fn AdvanceReeltimeReel(value: i16) {
    (*crate::gl!(sSlotMachine)).reeltimePixelOffset += value;
    (*crate::gl!(sSlotMachine)).reeltimePixelOffset %= 120;
    (*crate::gl!(sSlotMachine)).reeltimePosition =
        REELTIME_SYMBOLS - (*crate::gl!(sSlotMachine)).reeltimePixelOffset / 20;
}
pub unsafe fn AdvanceReeltimeReelToNextSymbol(mut value: i16) -> i16 {
    let mut offset: i16 = (*crate::gl!(sSlotMachine)).reeltimePixelOffset % 20;
    if offset != 0 {
        if offset < value {
            value = offset;
        }
        AdvanceReeltimeReel(value);
        offset = (*crate::gl!(sSlotMachine)).reeltimePixelOffset % 20;
    }
    offset
}
unsafe fn CreateReelTasks() {
    for i in 0..NUM_REELS {
        let taskId: u8 = CreateTask(Some(Task_Reel), 2);
        task_set(taskId, tReelId, i as i16);
        (*crate::gl!(sSlotMachine)).slotReelTasks[i] = taskId;
        Task_Reel(taskId);
    }
}
unsafe fn SpinSlotReel(reelIndex: u8) {
    task_set(
        (*crate::gl!(sSlotMachine)).slotReelTasks[reelIndex],
        tState,
        REEL_TASK_SPIN,
    );
    task_set(
        (*crate::gl!(sSlotMachine)).slotReelTasks[reelIndex],
        tMoving,
        TRUE as i16,
    );
}
unsafe fn StopSlotReel(reelIndex: u8) {
    task_set(
        (*crate::gl!(sSlotMachine)).slotReelTasks[reelIndex],
        tState,
        REEL_TASK_DECIDE_STOP,
    );
}
unsafe fn IsSlotReelMoving(reelIndex: u8) -> u8 {
    task_get(
        (*crate::gl!(sSlotMachine)).slotReelTasks[reelIndex],
        tMoving,
    ) as u8
}
pub(crate) unsafe fn Task_Reel(taskId: u8) {
    while sReelTasks[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
pub(crate) fn ReelTask_StayStill(task: *mut Task) -> u8 {
    FALSE
}
pub(crate) unsafe fn ReelTask_Spin(task: *mut Task) -> u8 {
    AdvanceSlotReel(
        (*task).data[tReelId] as u8,
        (*crate::gl!(sSlotMachine)).reelSpeed,
    );
    FALSE
}
pub(crate) unsafe fn ReelTask_DecideStop(task: *mut Task) -> u8 {
    (*task).data[tState] += 1;
    (*crate::gl!(sSlotMachine)).winnerRows[(*task).data[tReelId]] = 0;
    (*crate::gl!(sSlotMachine)).reelExtraTurns[(*task).data[tReelId]] = 0;
    if (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft == 0
        && ((*crate::gl!(sSlotMachine)).machineBias == 0
            || (*crate::gl!(sSlotMachine)).didNotFailBias == 0
            || sDecideStop_Bias[(*task).data[tReelId]].unwrap_unchecked()() == 0)
    {
        (*crate::gl!(sSlotMachine)).didNotFailBias = FALSE;
        sDecideStop_NoBias[(*task).data[tReelId]].unwrap_unchecked()();
    }
    (*task).data[tExtraTurns] = (*crate::gl!(sSlotMachine)).reelExtraTurns[(*task).data[tReelId]];
    TRUE
}
pub(crate) unsafe fn ReelTask_MoveToStop(task: *mut Task) -> u8 {
    let mut reelStopShocks: CArray<u16, 5> = zeroed();
    memcpy(
        reelStopShocks.as_mut_ptr() as *mut u8,
        sReelStopShocks.as_ptr().cast_mut() as *mut u8,
        10,
    );
    let mut reelPixelPos: i16 =
        (*crate::gl!(sSlotMachine)).reelPixelOffsets[(*task).data[tReelId]] % 24;
    if reelPixelPos != 0 {
        reelPixelPos = AdvanceSlotReelToNextSymbol(
            (*task).data[tReelId] as u8,
            (*crate::gl!(sSlotMachine)).reelSpeed,
        );
    } else if (*crate::gl!(sSlotMachine)).reelExtraTurns[(*task).data[tReelId]] != 0 {
        (*crate::gl!(sSlotMachine)).reelExtraTurns[(*task).data[tReelId]] -= 1;
        AdvanceSlotReel(
            (*task).data[tReelId] as u8,
            (*crate::gl!(sSlotMachine)).reelSpeed,
        );
        reelPixelPos = (*crate::gl!(sSlotMachine)).reelPixelOffsets[(*task).data[tReelId]] % 24;
    }
    if reelPixelPos == 0 && (*crate::gl!(sSlotMachine)).reelExtraTurns[(*task).data[tReelId]] == 0 {
        (*task).data[tState] += 1;
        (*task).data[1] = reelStopShocks[(*task).data[1]] as i16;
        (*task).data[2] = 0;
    }
    FALSE
}
pub(crate) unsafe fn ReelTask_ShakingStop(task: *mut Task) -> u8 {
    (*crate::gl!(sSlotMachine)).reelShockOffsets[(*task).data[tReelId]] =
        (*task).data[tShockMagnitude] as u16;
    (*task).data[tShockMagnitude] = -(*task).data[tShockMagnitude];
    (*task).data[2] += 1;
    if (*task).data[2] as i32 & 0x3 == 0 {
        (*task).data[tShockMagnitude] >>= 1;
    }
    if (*task).data[tShockMagnitude] == 0 {
        (*task).data[tState] = 0;
        (*task).data[tMoving] = FALSE as i16;
        (*crate::gl!(sSlotMachine)).reelShockOffsets[(*task).data[tReelId]] = 0;
    }
    FALSE
}
pub(crate) unsafe fn DecideStop_Bias_Reel1() -> u8 {
    let mut sym2: u8 = GetBiasSymbol((*crate::gl!(sSlotMachine)).machineBias);
    let mut sym1: u8 = sym2;
    if (*crate::gl!(sSlotMachine)).machineBias as i32 & 192 != 0 {
        sym1 = SYMBOL_7_RED;
        sym2 = SYMBOL_7_BLUE;
    }
    sDecideStop_Bias_Reel1_Bets[(*crate::gl!(sSlotMachine)).bet as i32 - 1].unwrap_unchecked()(
        sym1, sym2,
    )
}
unsafe fn EitherSymbolAtPos_Reel1(pos: i16, sym1: u8, sym2: u8) -> u8 {
    let sym: u8 = GetSymbol(LEFT_REEL, pos);
    if sym == sym1 || sym == sym2 {
        (*crate::gl!(sSlotMachine)).biasSymbol = sym;
        return TRUE;
    }
    FALSE
}
unsafe fn AreCherriesOnScreen_Reel1(turns: i16) -> u8 {
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
        0
    }
}
unsafe fn BiasedTowardCherryOr7s() -> u8 {
    if (*crate::gl!(sSlotMachine)).machineBias as i32 & 194 != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn DecideStop_Bias_Reel1_Bet1(sym1: u8, sym2: u8) -> u8 {
    let mut i: i16 = 0;
    while i <= MAX_EXTRA_TURNS {
        if EitherSymbolAtPos_Reel1(2 - i, sym1, sym2) != 0 {
            (*crate::gl!(sSlotMachine)).winnerRows[0] = 2;
            (*crate::gl!(sSlotMachine)).reelExtraTurns[0] = i;
            return TRUE;
        }
        i += 1;
    }
    FALSE
}
pub(crate) unsafe fn DecideStop_Bias_Reel1_Bet2or3(sym1: u8, sym2: u8) -> u8 {
    let mut i: i16 = 0;
    let cherry7Bias: u8 = BiasedTowardCherryOr7s();
    if cherry7Bias != 0 || AreCherriesOnScreen_Reel1(0) == 0 {
        for i in 1..=3i16 {
            if EitherSymbolAtPos_Reel1(i, sym1, sym2) != 0 {
                (*crate::gl!(sSlotMachine)).winnerRows[0] = i;
                (*crate::gl!(sSlotMachine)).reelExtraTurns[0] = 0;
                return TRUE;
            }
        }
    }
    i = 1;
    while i <= MAX_EXTRA_TURNS {
        let cherry7BiasCopy: u8 = cherry7Bias;
        if (cherry7BiasCopy != 0 || AreCherriesOnScreen_Reel1(i) == 0)
            && EitherSymbolAtPos_Reel1(1 - i, sym1, sym2) != 0
        {
            if i == 1 && (cherry7BiasCopy != 0 || AreCherriesOnScreen_Reel1(3) == 0) {
                (*crate::gl!(sSlotMachine)).winnerRows[0] = 3;
                (*crate::gl!(sSlotMachine)).reelExtraTurns[0] = 3;
                return TRUE;
            }
            if i <= 3 && (cherry7BiasCopy != 0 || AreCherriesOnScreen_Reel1(i + 1) == 0) {
                (*crate::gl!(sSlotMachine)).winnerRows[0] = 2;
                (*crate::gl!(sSlotMachine)).reelExtraTurns[0] = i + 1;
                return TRUE;
            }
            (*crate::gl!(sSlotMachine)).winnerRows[0] = 1;
            (*crate::gl!(sSlotMachine)).reelExtraTurns[0] = i;
            return TRUE;
        }
        i += 1;
    }
    FALSE
}
pub(crate) unsafe fn DecideStop_Bias_Reel2() -> u8 {
    sDecideStop_Bias_Reel2_Bets[(*crate::gl!(sSlotMachine)).bet as i32 - 1].unwrap_unchecked()()
}
pub(crate) unsafe fn DecideStop_Bias_Reel2_Bet1or2() -> u8 {
    let reel1BiasRow: i16 = (*crate::gl!(sSlotMachine)).winnerRows[0];
    let mut i: i16 = 0;
    while i <= MAX_EXTRA_TURNS {
        if GetSymbol(MIDDLE_REEL, reel1BiasRow - i) == (*crate::gl!(sSlotMachine)).biasSymbol {
            (*crate::gl!(sSlotMachine)).winnerRows[1] = reel1BiasRow;
            (*crate::gl!(sSlotMachine)).reelExtraTurns[1] = i;
            return TRUE;
        }
        i += 1;
    }
    FALSE
}
pub(crate) unsafe fn DecideStop_Bias_Reel2_Bet3() -> u8 {
    let mut i: i16 = 0;
    if DecideStop_Bias_Reel2_Bet1or2() != 0 {
        if (*crate::gl!(sSlotMachine)).winnerRows[0] != 2
            && (*crate::gl!(sSlotMachine)).reelExtraTurns[1] > 1
            && (*crate::gl!(sSlotMachine)).reelExtraTurns[1] != 4
        {
            i = 0;
            while i <= MAX_EXTRA_TURNS {
                if GetSymbol(MIDDLE_REEL, 2 - i) == (*crate::gl!(sSlotMachine)).biasSymbol {
                    (*crate::gl!(sSlotMachine)).winnerRows[1] = 2;
                    (*crate::gl!(sSlotMachine)).reelExtraTurns[1] = i;
                    break;
                }
                i += 1;
            }
        }
        return TRUE;
    }
    if (*crate::gl!(sSlotMachine)).winnerRows[0] != 2 {
        i = 0;
        while i <= MAX_EXTRA_TURNS {
            if GetSymbol(MIDDLE_REEL, 2 - i) == (*crate::gl!(sSlotMachine)).biasSymbol {
                (*crate::gl!(sSlotMachine)).winnerRows[1] = 2;
                (*crate::gl!(sSlotMachine)).reelExtraTurns[1] = i;
                return TRUE;
            }
            i += 1;
        }
    }
    FALSE
}
pub(crate) unsafe fn DecideStop_Bias_Reel3() -> u8 {
    let mut biasSymbol: u8 = (*crate::gl!(sSlotMachine)).biasSymbol;
    if (*crate::gl!(sSlotMachine)).machineBias as i32 & BIAS_MIXED_7 != 0 {
        biasSymbol = SYMBOL_7_RED;
        if (*crate::gl!(sSlotMachine)).biasSymbol == SYMBOL_7_RED {
            biasSymbol = SYMBOL_7_BLUE;
        }
    }
    sDecideStop_Bias_Reel3_Bets[(*crate::gl!(sSlotMachine)).bet as i32 - 1].unwrap_unchecked()(
        biasSymbol,
    )
}
pub(crate) unsafe fn DecideStop_Bias_Reel3_Bet1or2(biasSymbol: u8) -> u8 {
    let reel2BiasRow: i16 = (*crate::gl!(sSlotMachine)).winnerRows[1];
    let mut i: i16 = 0;
    while i <= MAX_EXTRA_TURNS {
        if GetSymbol(RIGHT_REEL, reel2BiasRow - i) == biasSymbol {
            (*crate::gl!(sSlotMachine)).winnerRows[2] = reel2BiasRow;
            (*crate::gl!(sSlotMachine)).reelExtraTurns[2] = i;
            return TRUE;
        }
        i += 1;
    }
    FALSE
}
pub(crate) unsafe fn DecideStop_Bias_Reel3_Bet3(biasSymbol: u8) -> u8 {
    let mut biasRow: i16 = 0;
    if (*crate::gl!(sSlotMachine)).winnerRows[0] == (*crate::gl!(sSlotMachine)).winnerRows[1] {
        return DecideStop_Bias_Reel3_Bet1or2(biasSymbol);
    }
    if (*crate::gl!(sSlotMachine)).winnerRows[0] == 1 {
        biasRow = 3;
    } else {
        biasRow = 1;
    }
    let mut i: i16 = 0;
    while i <= MAX_EXTRA_TURNS {
        if GetSymbol(RIGHT_REEL, biasRow - i) == biasSymbol {
            (*crate::gl!(sSlotMachine)).reelExtraTurns[2] = i;
            (*crate::gl!(sSlotMachine)).winnerRows[2] = biasRow;
            return TRUE;
        }
        i += 1;
    }
    FALSE
}
pub(crate) unsafe fn DecideStop_NoBias_Reel1() {
    let mut i: i16 = 0;
    while AreCherriesOnScreen_Reel1(i) != 0 {
        i += 1;
    }
    (*crate::gl!(sSlotMachine)).reelExtraTurns[0] = i;
}
unsafe fn IfSymbol7_SwitchColor(symbol: *mut u8) -> u8 {
    if *symbol == SYMBOL_7_RED {
        *symbol = SYMBOL_7_BLUE;
        return TRUE;
    }
    if *symbol == SYMBOL_7_BLUE {
        *symbol = SYMBOL_7_RED;
        return TRUE;
    }
    FALSE
}
pub(crate) unsafe fn DecideStop_NoBias_Reel2() {
    sDecideStop_NoBias_Reel2_Bets[(*crate::gl!(sSlotMachine)).bet as i32 - 1].unwrap_unchecked()();
}
pub(crate) unsafe fn DecideStop_NoBias_Reel2_Bet1() {
    if (*crate::gl!(sSlotMachine)).winnerRows[0] != 0
        && (*crate::gl!(sSlotMachine)).machineBias as i32 & BIAS_STRAIGHT_7 != 0
    {
        let mut reel1MiddleSym: u8 =
            GetSymbol(LEFT_REEL, 2 - (*crate::gl!(sSlotMachine)).reelExtraTurns[0]);
        if IfSymbol7_SwitchColor(&raw mut reel1MiddleSym) != 0 {
            let mut i: i16 = 0;
            while i <= MAX_EXTRA_TURNS {
                if reel1MiddleSym == GetSymbol(MIDDLE_REEL, 2 - i) {
                    (*crate::gl!(sSlotMachine)).winnerRows[1] = 2;
                    (*crate::gl!(sSlotMachine)).reelExtraTurns[1] = i;
                    break;
                }
                i += 1;
            }
        }
    }
}
pub(crate) unsafe fn DecideStop_NoBias_Reel2_Bet2() {
    if (*crate::gl!(sSlotMachine)).winnerRows[0] != 0
        && (*crate::gl!(sSlotMachine)).machineBias as i32 & BIAS_STRAIGHT_7 != 0
    {
        let mut reel1BiasSym: u8 = GetSymbol(
            LEFT_REEL,
            (*crate::gl!(sSlotMachine)).winnerRows[0]
                - (*crate::gl!(sSlotMachine)).reelExtraTurns[0],
        );
        if IfSymbol7_SwitchColor(&raw mut reel1BiasSym) != 0 {
            let mut i: i16 = 0;
            while i <= MAX_EXTRA_TURNS {
                if reel1BiasSym
                    == GetSymbol(MIDDLE_REEL, (*crate::gl!(sSlotMachine)).winnerRows[0] - i)
                {
                    (*crate::gl!(sSlotMachine)).winnerRows[1] =
                        (*crate::gl!(sSlotMachine)).winnerRows[0];
                    (*crate::gl!(sSlotMachine)).reelExtraTurns[1] = i;
                    break;
                }
                i += 1;
            }
        }
    }
}
pub(crate) unsafe fn DecideStop_NoBias_Reel2_Bet3() {
    let mut i: i16 = 0;
    let mut j: i16 = 0;
    let mut reel1BiasSym: u8 = 0;
    if (*crate::gl!(sSlotMachine)).winnerRows[0] != 0
        && (*crate::gl!(sSlotMachine)).machineBias as i32 & BIAS_STRAIGHT_7 != 0
    {
        if (*crate::gl!(sSlotMachine)).winnerRows[0] == 2 {
            DecideStop_NoBias_Reel2_Bet2();
            return;
        }
        reel1BiasSym = GetSymbol(
            LEFT_REEL,
            (*crate::gl!(sSlotMachine)).winnerRows[0]
                - (*crate::gl!(sSlotMachine)).reelExtraTurns[0],
        );
        if IfSymbol7_SwitchColor(&raw mut reel1BiasSym) != 0 {
            j = 2;
            if (*crate::gl!(sSlotMachine)).winnerRows[0] == 3 {
                j = 3;
            }
            i = 0;
            while i < 2 {
                if reel1BiasSym == GetSymbol(MIDDLE_REEL, j) {
                    (*crate::gl!(sSlotMachine)).winnerRows[1] = j;
                    (*crate::gl!(sSlotMachine)).reelExtraTurns[1] = 0;
                    return;
                }
                i += 1;
                j -= 1;
            }
            j = 1;
            while j <= MAX_EXTRA_TURNS {
                if reel1BiasSym
                    == GetSymbol(MIDDLE_REEL, (*crate::gl!(sSlotMachine)).winnerRows[0] - j)
                {
                    if (*crate::gl!(sSlotMachine)).winnerRows[0] == 1 {
                        if j <= 2 {
                            (*crate::gl!(sSlotMachine)).winnerRows[1] = 2;
                            (*crate::gl!(sSlotMachine)).reelExtraTurns[1] = j + 1;
                        } else {
                            (*crate::gl!(sSlotMachine)).winnerRows[1] = 1;
                            (*crate::gl!(sSlotMachine)).reelExtraTurns[1] = j;
                        }
                    } else {
                        if j <= 2 {
                            (*crate::gl!(sSlotMachine)).winnerRows[1] = 3;
                            (*crate::gl!(sSlotMachine)).reelExtraTurns[1] = j;
                        } else {
                            (*crate::gl!(sSlotMachine)).winnerRows[1] = 2;
                            (*crate::gl!(sSlotMachine)).reelExtraTurns[1] = j - 1;
                        }
                    }
                    return;
                }
                j += 1;
            }
        }
    }
}
fn MismatchedSyms_77(sym1: u8, sym2: u8) -> u8 {
    if sym1 == SYMBOL_7_RED && sym2 == SYMBOL_7_BLUE
        || sym1 == SYMBOL_7_BLUE && sym2 == SYMBOL_7_RED
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
fn MismatchedSyms_777(sym1: u8, sym2: u8, sym3: u8) -> u8 {
    if sym1 == SYMBOL_7_RED && sym2 == SYMBOL_7_BLUE && sym3 == SYMBOL_7_RED
        || sym1 == SYMBOL_7_BLUE && sym2 == SYMBOL_7_RED && sym3 == SYMBOL_7_BLUE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
fn NeitherMatchNor7Mismatch(sym1: u8, sym2: u8, sym3: u8) -> u8 {
    if sym1 == SYMBOL_7_RED && sym2 == SYMBOL_7_BLUE && sym3 == SYMBOL_7_RED
        || sym1 == SYMBOL_7_BLUE && sym2 == SYMBOL_7_RED && sym3 == SYMBOL_7_BLUE
        || sym1 == SYMBOL_7_RED && sym2 == SYMBOL_7_RED && sym3 == SYMBOL_7_BLUE
        || sym1 == SYMBOL_7_BLUE && sym2 == SYMBOL_7_BLUE && sym3 == SYMBOL_7_RED
        || sym1 == sym2 && sym1 == sym3
    {
        return FALSE;
    }
    TRUE
}
pub(crate) unsafe fn DecideStop_NoBias_Reel3() {
    sDecideStop_NoBias_Reel3_Bets[(*crate::gl!(sSlotMachine)).bet as i32 - 1].unwrap_unchecked()();
}
pub(crate) unsafe fn DecideStop_NoBias_Reel3_Bet1() {
    let mut i: i16 = 0;
    let sym1: u8 = GetSymbol(LEFT_REEL, 2 - (*crate::gl!(sSlotMachine)).reelExtraTurns[0]);
    let sym2: u8 = GetSymbol(
        MIDDLE_REEL,
        2 - (*crate::gl!(sSlotMachine)).reelExtraTurns[1],
    );
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
        if (*crate::gl!(sSlotMachine)).machineBias as i32 & BIAS_STRAIGHT_7 != 0 {
            i = 0;
            while i <= MAX_EXTRA_TURNS {
                if sym1 == GetSymbol(RIGHT_REEL, 2 - i) {
                    (*crate::gl!(sSlotMachine)).reelExtraTurns[2] = i;
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
    (*crate::gl!(sSlotMachine)).reelExtraTurns[2] = i;
}
pub(crate) unsafe fn DecideStop_NoBias_Reel3_Bet2() {
    let mut extraTurns: i16 = 0;
    let mut i: i16 = 0;
    let mut sym1: u8 = 0;
    let mut sym2: u8 = 0;
    let mut sym3: u8 = 0;
    if (*crate::gl!(sSlotMachine)).winnerRows[1] != 0
        && (*crate::gl!(sSlotMachine)).winnerRows[0] == (*crate::gl!(sSlotMachine)).winnerRows[1]
        && (*crate::gl!(sSlotMachine)).machineBias as i32 & BIAS_STRAIGHT_7 != 0
    {
        sym1 = GetSymbol(
            LEFT_REEL,
            (*crate::gl!(sSlotMachine)).winnerRows[0]
                - (*crate::gl!(sSlotMachine)).reelExtraTurns[0],
        );
        sym2 = GetSymbol(
            MIDDLE_REEL,
            (*crate::gl!(sSlotMachine)).winnerRows[1]
                - (*crate::gl!(sSlotMachine)).reelExtraTurns[1],
        );
        if MismatchedSyms_77(sym1, sym2) != 0 {
            i = 0;
            while i <= MAX_EXTRA_TURNS {
                sym3 = GetSymbol(RIGHT_REEL, (*crate::gl!(sSlotMachine)).winnerRows[1] - i);
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
        for i in 1..=3i16 {
            sym1 = GetSymbol(LEFT_REEL, i - (*crate::gl!(sSlotMachine)).reelExtraTurns[0]);
            sym2 = GetSymbol(
                MIDDLE_REEL,
                i - (*crate::gl!(sSlotMachine)).reelExtraTurns[1],
            );
            sym3 = GetSymbol(RIGHT_REEL, i - extraTurns);
            if NeitherMatchNor7Mismatch(sym1, sym2, sym3) == 0
                && !(MismatchedSyms_777(sym1, sym2, sym3) != 0
                    && (*crate::gl!(sSlotMachine)).machineBias as i32 & BIAS_STRAIGHT_7 != 0)
            {
                numMatches += 1;
                break;
            }
        }
        if numMatches == 0 {
            break;
        }
        extraTurns += 1;
    }
    (*crate::gl!(sSlotMachine)).reelExtraTurns[2] = extraTurns;
}
pub(crate) unsafe fn DecideStop_NoBias_Reel3_Bet3() {
    let mut sym1: u8 = 0;
    let mut sym2: u8 = 0;
    let mut sym3: u8 = 0;
    let mut row: i16 = 0;
    let mut i: i16 = 0;
    DecideStop_NoBias_Reel3_Bet2();
    if (*crate::gl!(sSlotMachine)).winnerRows[1] != 0
        && (*crate::gl!(sSlotMachine)).winnerRows[0] != (*crate::gl!(sSlotMachine)).winnerRows[1]
        && (*crate::gl!(sSlotMachine)).machineBias as i32 & BIAS_STRAIGHT_7 != 0
    {
        sym1 = GetSymbol(
            LEFT_REEL,
            (*crate::gl!(sSlotMachine)).winnerRows[0]
                - (*crate::gl!(sSlotMachine)).reelExtraTurns[0],
        );
        sym2 = GetSymbol(
            MIDDLE_REEL,
            (*crate::gl!(sSlotMachine)).winnerRows[1]
                - (*crate::gl!(sSlotMachine)).reelExtraTurns[1],
        );
        if MismatchedSyms_77(sym1, sym2) != 0 {
            row = 1;
            if (*crate::gl!(sSlotMachine)).winnerRows[0] == 1 {
                row = 3;
            }
            i = 0;
            while i <= MAX_EXTRA_TURNS {
                sym3 = GetSymbol(
                    RIGHT_REEL,
                    row - ((*crate::gl!(sSlotMachine)).reelExtraTurns[2] + i),
                );
                if sym1 == sym3 {
                    (*crate::gl!(sSlotMachine)).reelExtraTurns[2] += i;
                    break;
                }
                i += 1;
            }
        }
    }
    loop {
        sym1 = GetSymbol(LEFT_REEL, 1 - (*crate::gl!(sSlotMachine)).reelExtraTurns[0]);
        sym2 = GetSymbol(
            MIDDLE_REEL,
            2 - (*crate::gl!(sSlotMachine)).reelExtraTurns[1],
        );
        sym3 = GetSymbol(
            RIGHT_REEL,
            3 - (*crate::gl!(sSlotMachine)).reelExtraTurns[2],
        );
        if NeitherMatchNor7Mismatch(sym1, sym2, sym3) != 0
            || MismatchedSyms_777(sym1, sym2, sym3) != 0
                && (*crate::gl!(sSlotMachine)).machineBias as i32 & BIAS_STRAIGHT_7 != 0
        {
            break;
        }
        (*crate::gl!(sSlotMachine)).reelExtraTurns[2] += 1;
    }
    loop {
        sym1 = GetSymbol(LEFT_REEL, 3 - (*crate::gl!(sSlotMachine)).reelExtraTurns[0]);
        sym2 = GetSymbol(
            MIDDLE_REEL,
            2 - (*crate::gl!(sSlotMachine)).reelExtraTurns[1],
        );
        sym3 = GetSymbol(
            RIGHT_REEL,
            1 - (*crate::gl!(sSlotMachine)).reelExtraTurns[2],
        );
        if NeitherMatchNor7Mismatch(sym1, sym2, sym3) != 0
            || MismatchedSyms_777(sym1, sym2, sym3) != 0
                && (*crate::gl!(sSlotMachine)).machineBias as i32 & BIAS_STRAIGHT_7 != 0
        {
            break;
        }
        (*crate::gl!(sSlotMachine)).reelExtraTurns[2] += 1;
    }
}
unsafe fn PressStopReelButton(reelNum: u8) {
    let taskId: u8 = CreateTask(Some(Task_PressStopReelButton), 5);
    task_set(taskId, 15, reelNum as i16);
    Task_PressStopReelButton(taskId);
}
pub(crate) unsafe fn Task_PressStopReelButton(taskId: u8) {
    sReelStopButtonTasks[task_get(taskId, 0)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
        taskId,
    );
}
pub(crate) unsafe fn StopReelButton_Press(task: *mut Task, taskId: u8) {
    SetReelButtonTilemap(sReelButtonOffsets[(*task).data[15]], 0x62, 0x63, 0x72, 0x73);
    (*task).data[0] += 1;
}
pub(crate) unsafe fn StopReelButton_Wait(task: *mut Task, taskId: u8) {
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) > 11
    {
        (*task).data[0] += 1;
    }
}
pub(crate) unsafe fn StopReelButton_Unpress(task: *mut Task, taskId: u8) {
    SetReelButtonTilemap(sReelButtonOffsets[(*task).data[15]], 0x42, 0x43, 0x52, 0x53);
    DestroyTask(taskId);
}
unsafe fn LightenMatchLine(matchLineId: u8) {
    LoadPalette(
        sLitMatchLinePalTable[matchLineId] as *mut c_void,
        sMatchLinePalOffsets[matchLineId] as u16,
        2,
    );
}
unsafe fn DarkenMatchLine(matchLineId: u8) {
    LoadPalette(
        sDarkMatchLinePalTable[matchLineId] as *mut c_void,
        sMatchLinePalOffsets[matchLineId] as u16,
        2,
    );
}
unsafe fn LightenBetTiles(betVal: u8) {
    let mut i: u8 = 0;
    while i < sMatchLinesPerBet[betVal] {
        LightenMatchLine(sBetToMatchLineIds[betVal][i]);
        i += 1;
    }
}
unsafe fn DarkenBetTiles(betVal: u8) {
    let mut i: u8 = 0;
    while i < sMatchLinesPerBet[betVal] {
        DarkenMatchLine(sBetToMatchLineIds[betVal][i]);
        i += 1;
    }
}
unsafe fn CreateInvisibleFlashMatchLineSprites() {
    for i in 0..5u8 {
        let spriteId: u8 = CreateInvisibleSprite(Some(SpriteCB_FlashMatchingLines));
        {
            let gl_i0 = sMatchLineId;
            crate::gl!(gSprites[spriteId].data[gl_i0] = i as i16)
        };
        (*crate::gl!(sSlotMachine)).flashMatchLineSpriteIds[i] = spriteId;
    }
}
unsafe fn FlashMatchLine(matchLineId: u8) {
    let sprite: *mut Sprite = {
        let gl_i1 = (*crate::gl!(sSlotMachine)).flashMatchLineSpriteIds[matchLineId];
        crate::gl!(&raw gSprites[gl_i1])
    };
    (*sprite).data[sFlashing] = TRUE as i16;
    (*sprite).data[sNumFullFlashes] = 4;
    (*sprite).data[3] = 0;
    (*sprite).data[sColor] = 0;
    (*sprite).data[sColorIncr] = 2;
    (*sprite).data[sAtOriginalColor] = FALSE as i16;
}
unsafe fn IsMatchLineDoneFlashingBeforePayout() -> u8 {
    for i in 0..5u8 {
        let sprite: *mut Sprite = {
            let gl_i2 = (*crate::gl!(sSlotMachine)).flashMatchLineSpriteIds[i];
            crate::gl!(&raw gSprites[gl_i2])
        };
        if (*sprite).data[sFlashing] != 0 && (*sprite).data[sNumFullFlashes] != 0 {
            return FALSE;
        }
    }
    TRUE
}
unsafe fn TryStopMatchLinesFlashing() -> u8 {
    for i in 0..5u8 {
        if TryStopMatchLineFlashing((*crate::gl!(sSlotMachine)).flashMatchLineSpriteIds[i]) == 0 {
            return FALSE;
        }
    }
    TRUE
}
unsafe fn TryStopMatchLineFlashing(spriteId: u8) -> u8 {
    let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
    if (*sprite).data[sFlashing] == 0 {
        return TRUE;
    }
    if (*sprite).data[sAtOriginalColor] != 0 {
        (*sprite).data[sFlashing] = FALSE as i16;
    }
    (*sprite).data[sAtOriginalColor] as u8
}
pub(crate) unsafe fn SpriteCB_FlashMatchingLines(sprite: *mut Sprite) {
    let mut maxColorChange: i16 = 0;
    if (*sprite).data[sFlashing] != 0 {
        if ({
            let t1 = (*sprite).data[3];
            (*sprite).data[3] -= 1;
            t1
        }) == 0
        {
            (*sprite).data[sAtOriginalColor] = FALSE as i16;
            (*sprite).data[3] = 1;
            (*sprite).data[sColor] += (*sprite).data[sColorIncr];
            maxColorChange = 4;
            if (*sprite).data[sNumFullFlashes] != 0 {
                maxColorChange = 8;
            }
            if (*sprite).data[sColor] <= 0 {
                (*sprite).data[sAtOriginalColor] = TRUE as i16;
                (*sprite).data[sColorIncr] = -(*sprite).data[sColorIncr];
                if (*sprite).data[sNumFullFlashes] != 0 {
                    (*sprite).data[sNumFullFlashes] -= 1;
                }
            } else if (*sprite).data[sColor] >= maxColorChange {
                (*sprite).data[sColorIncr] = -(*sprite).data[sColorIncr];
            }
            if (*sprite).data[sNumFullFlashes] != 0 {
                (*sprite).data[3] <<= 1;
            }
        }
        MultiplyPaletteRGBComponents(
            sMatchLinePalOffsets[(*sprite).data[sMatchLineId]] as u16,
            (*sprite).data[sColor] as u8,
            (*sprite).data[sColor] as u8,
            (*sprite).data[sColor] as u8,
        );
    }
}
unsafe fn FlashSlotMachineLights() {
    let taskId: u8 = CreateTask(Some(Task_FlashSlotMachineLights), 6);
    task_set(taskId, sFlashDir, 1);
    Task_FlashSlotMachineLights(taskId);
}
unsafe fn TryStopSlotMachineLights() -> u8 {
    let taskId: u8 = FindTaskIdByFunc(Some(Task_FlashSlotMachineLights));
    if task_get(taskId, sFlashState) == 0 {
        DestroyTask(taskId);
        LoadPalette(*sSlotMachineMenu_Pal as *mut c_void, 16, 32);
        return TRUE;
    }
    FALSE
}
pub(crate) unsafe fn Task_FlashSlotMachineLights(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if ({
        let t1 = (*task).data[1];
        (*task).data[1] -= 1;
        t1
    }) == 0
    {
        (*task).data[1] = 4;
        (*task).data[sFlashState] += (*task).data[sFlashDir];
        if (*task).data[sFlashState] == 0 || (*task).data[sFlashState] == 2 {
            (*task).data[sFlashDir] = -(*task).data[sFlashDir];
        }
    }
    LoadPalette(
        sFlashingLightsPalTable[(*task).data[sFlashState]] as *mut c_void,
        16,
        32,
    );
}
unsafe fn CreatePikaPowerBoltTask() {
    (*crate::gl!(sSlotMachine)).pikaPowerBoltTaskId = CreateTask(Some(Task_CreatePikaPowerBolt), 8);
}
unsafe fn AddPikaPowerBolt(bolts: u8) {
    let task: *mut Task =
        &raw mut (*gTasks.as_ptr())[(*crate::gl!(sSlotMachine)).pikaPowerBoltTaskId];
    ResetPikaPowerBoltTask(task);
    (*task).data[tState] = PIKABOLT_TASK_ADD_BOLT;
    (*task).data[tNumBolts] += 1;
    (*task).data[tAnimating] = TRUE as i16;
}
unsafe fn ResetPikaPowerBolts() {
    let task: *mut Task =
        &raw mut (*gTasks.as_ptr())[(*crate::gl!(sSlotMachine)).pikaPowerBoltTaskId];
    ResetPikaPowerBoltTask(task);
    (*task).data[tState] = PIKABOLT_TASK_CLEAR_ALL;
    (*task).data[tAnimating] = TRUE as i16;
}
unsafe fn IsPikaPowerBoltAnimating() -> u8 {
    task_get((*crate::gl!(sSlotMachine)).pikaPowerBoltTaskId, tAnimating) as u8
}
pub(crate) unsafe fn Task_CreatePikaPowerBolt(taskId: u8) {
    sPikaPowerBoltTasks[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    );
}
pub(crate) fn PikaPowerBolt_Idle(task: *mut Task) {}
pub(crate) unsafe fn PikaPowerBolt_AddBolt(task: *mut Task) {
    (*task).data[tSpriteId] =
        CreatePikaPowerBoltSprite(((*task).data[tNumBolts] << 3) + 20, 20) as i16;
    (*task).data[tState] += 1;
}
pub(crate) unsafe fn PikaPowerBolt_WaitAnim(task: *mut Task) {
    if ({
        let gl_i3 = (*task).data[tSpriteId];
        crate::gl!(gSprites[gl_i3].data[7])
    }) != 0
    {
        let r5: i16 = (*task).data[tNumBolts] + 2;
        let mut r3: i16 = 0;
        let mut r2: i16 = 0;
        if (*task).data[tNumBolts] == 1 {
            r3 = 1;
            r2 = 1;
        } else if (*task).data[tNumBolts] == 16 {
            r3 = 2;
            r2 = 2;
        }
        *(*sSelectedPikaPowerTile.as_ptr()).at(r2) = sPikaPowerTileTable[r3][0];
        LoadBgTilemap(
            2,
            (*sSelectedPikaPowerTile.as_ptr()).at(r2) as *mut c_void,
            2,
            r5 as u16 + 0x40,
        );
        DestroyPikaPowerBoltSprite((*task).data[tSpriteId] as u8);
        (*task).data[tState] = PIKABOLT_TASK_IDLE;
        (*task).data[tAnimating] = 0;
    }
}
pub(crate) unsafe fn PikaPowerBolt_ClearAll(task: *mut Task) {
    let r5: i16 = (*task).data[tNumBolts] + 2;
    let mut r3: i16 = 0;
    let mut r2: i16 = 3;
    if (*task).data[tNumBolts] == 1 {
        r3 = 1;
        r2 = 1;
    } else if (*task).data[tNumBolts] == 16 {
        r3 = 2;
        r2 = 2;
    }
    if (*task).data[2] == 0 {
        *(*sSelectedPikaPowerTile.as_ptr()).at(r2) = sPikaPowerTileTable[r3][1];
        LoadBgTilemap(
            2,
            (*sSelectedPikaPowerTile.as_ptr()).at(r2) as *mut c_void,
            2,
            r5 as u16 + 0x40,
        );
        (*task).data[tNumBolts] -= 1;
    }
    if ({
        (*task).data[2] += 1;
        (*task).data[2]
    }) >= 20
    {
        (*task).data[2] = 0;
    }
    if (*task).data[tNumBolts] == 0 {
        (*task).data[tState] = PIKABOLT_TASK_IDLE;
        (*task).data[tAnimating] = 0;
    }
}
unsafe fn ResetPikaPowerBoltTask(task: *mut Task) {
    for i in 2..NUM_TASK_DATA {
        (*task).data[i] = 0;
    }
}
unsafe fn LoadPikaPowerMeter(bolts: u8) {
    let mut r3: i16 = 0;
    let mut r1: i16 = 0;
    let mut r4: i16 = 3;
    let mut i: i16 = 0;
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
        *(*sSelectedPikaPowerTile.as_ptr()).at(r1) = sPikaPowerTileTable[r3][0];
        LoadBgTilemap(
            2,
            (*sSelectedPikaPowerTile.as_ptr()).at(r1) as *mut c_void,
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
        *(*sSelectedPikaPowerTile.as_ptr()).at(r1) = sPikaPowerTileTable[r3][1];
        LoadBgTilemap(
            2,
            (*sSelectedPikaPowerTile.as_ptr()).at(r1) as *mut c_void,
            2,
            r4 as u16 + 0x40,
        );
        i += 1;
        r4 += 1;
    }
    task_set(
        (*crate::gl!(sSlotMachine)).pikaPowerBoltTaskId,
        1,
        bolts as i16,
    );
}
unsafe fn BeginReelTime() {
    let taskId: u8 = CreateTask(Some(Task_ReelTime), 7);
    Task_ReelTime(taskId);
}
unsafe fn IsReelTimeTaskDone() -> u8 {
    if FindTaskIdByFunc(Some(Task_ReelTime)) == TAIL_SENTINEL {
        return TRUE;
    }
    FALSE
}
pub(crate) unsafe fn Task_ReelTime(taskId: u8) {
    sReelTimeTasks[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    );
}
pub(crate) unsafe fn ReelTime_Init(task: *mut Task) {
    (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft = 0;
    (*crate::gl!(sSlotMachine)).reeltimePixelOffset = 0;
    (*crate::gl!(sSlotMachine)).reeltimePosition = 0;
    (*task).data[tState] += 1;
    (*task).data[1] = 0;
    (*task).data[2] = 30;
    (*task).data[tRtReelSpeed] = 1280;
    gSpriteCoordOffsetX.set(0);
    gSpriteCoordOffsetY.set(0);
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
pub(crate) unsafe fn ReelTime_WindowEnter(task: *mut Task) {
    gSpriteCoordOffsetX.set(gSpriteCoordOffsetX.get() - (8));
    (*task).data[1] += 8;
    let r3: i16 = ((((*task).data[1] as i32 + 240) & 0xff) >> 3) as i16;
    SetGpuReg(REG_OFFSET_BG1HOFS, (*task).data[1] as u16 & 0x1ff);
    if r3 != (*task).data[2] && (*task).data[3] <= 18 {
        (*task).data[2] = r3;
        (*task).data[3] = (*task).data[1] >> 3;
        LoadReelTimeWindowTilemap(r3, (*task).data[3]);
    }
    if (*task).data[1] >= 200 {
        (*task).data[tState] += 1;
        (*task).data[3] = 0;
    }
    AdvanceReeltimeReel((*task).data[tRtReelSpeed] >> 8);
}
pub(crate) unsafe fn ReelTime_WaitStartPikachu(task: *mut Task) {
    AdvanceReeltimeReel((*task).data[tRtReelSpeed] >> 8);
    if ({
        (*task).data[tTimer1] += 1;
        (*task).data[tTimer1]
    }) >= 60
    {
        (*task).data[tState] += 1;
        CreateReelTimeBoltSprites();
        CreateReelTimePikachuAuraSprites();
    }
}
pub(crate) unsafe fn ReelTime_PikachuSpeedUp1(task: *mut Task) {
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
    AdvanceReeltimeReel((*task).data[tRtReelSpeed] >> 8);
    (*task).data[tRtReelSpeed] -= 4;
    let i: i32 = 4 - ((*task).data[tRtReelSpeed] >> 8) as i32;
    SetReelTimeBoltDelay(reelTimeBoltDelays[i]);
    SetReelTimePikachuAuraFlashDelay(pikachuAuraFlashDelays[i]);
    StartSpriteAnimIfDifferent(
        {
            let gl_i4 = (*crate::gl!(sSlotMachine)).reelTimePikachuSpriteId;
            crate::gl!(&raw gSprites[gl_i4])
        },
        pikachuAnimIds[i],
    );
    if (*task).data[tRtReelSpeed] <= 0x100 {
        (*task).data[tState] += 1;
        (*task).data[tRtReelSpeed] = 0x100;
        (*task).data[tTimer1] = 0;
    }
}
pub(crate) unsafe fn ReelTime_PikachuSpeedUp2(task: *mut Task) {
    AdvanceReeltimeReel((*task).data[tRtReelSpeed] >> 8);
    if ({
        (*task).data[tTimer1] += 1;
        (*task).data[tTimer1]
    }) >= 80
    {
        (*task).data[tState] += 1;
        (*task).data[tTimer1] = 0;
        SetReelTimePikachuAuraFlashDelay(2);
        StartSpriteAnimIfDifferent(
            {
                let gl_i5 = (*crate::gl!(sSlotMachine)).reelTimePikachuSpriteId;
                crate::gl!(&raw gSprites[gl_i5])
            },
            3,
        );
    }
}
pub(crate) unsafe fn ReelTime_WaitReel(task: *mut Task) {
    AdvanceReeltimeReel((*task).data[tRtReelSpeed] >> 8);
    (*task).data[tRtReelSpeed] = (*task).data[tRtReelSpeed] as u8 as i16 + 0x80;
    if ({
        (*task).data[tTimer1] += 1;
        (*task).data[tTimer1]
    }) >= 80
    {
        (*task).data[tState] += 1;
        (*task).data[tTimer1] = 0;
    }
}
pub(crate) unsafe fn ReelTime_CheckExplode(task: *mut Task) {
    AdvanceReeltimeReel((*task).data[tRtReelSpeed] >> 8);
    (*task).data[tRtReelSpeed] = (*task).data[tRtReelSpeed] as u8 as i16 + 0x40;
    if ({
        (*task).data[tTimer1] += 1;
        (*task).data[tTimer1]
    }) >= 40
    {
        (*task).data[tTimer1] = 0;
        if (*crate::gl!(sSlotMachine)).reelTimeDraw != 0 {
            if (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft as i16 <= (*task).data[tExplodeChecks]
            {
                (*task).data[tState] += 1;
            }
        } else if (*task).data[tExplodeChecks] > 3 {
            (*task).data[tState] += 1;
        } else if ShouldReelTimeMachineExplode((*task).data[tExplodeChecks] as u16) != 0 {
            (*task).data[tState] = RT_TASK_EXPLODE;
        }
        (*task).data[tExplodeChecks] += 1;
    }
}
pub(crate) unsafe fn ReelTime_LandOnOutcome(task: *mut Task) {
    let mut reeltimePixelOffset: i16 = (*crate::gl!(sSlotMachine)).reeltimePixelOffset % 20;
    if reeltimePixelOffset != 0 {
        reeltimePixelOffset = AdvanceReeltimeReelToNextSymbol((*task).data[tRtReelSpeed] >> 8);
        (*task).data[tRtReelSpeed] = (*task).data[tRtReelSpeed] as u8 as i16 + 0x40;
    } else if GetReelTimeSymbol(1) != (*crate::gl!(sSlotMachine)).reelTimeDraw {
        AdvanceReeltimeReel((*task).data[tRtReelSpeed] >> 8);
        reeltimePixelOffset = (*crate::gl!(sSlotMachine)).reeltimePixelOffset % 20;
        (*task).data[tRtReelSpeed] = (*task).data[tRtReelSpeed] as u8 as i16 + 0x40;
    }
    if reeltimePixelOffset == 0 && GetReelTimeSymbol(1) == (*crate::gl!(sSlotMachine)).reelTimeDraw
    {
        (*task).data[tRtReelSpeed] = 0;
        (*task).data[tState] += 1;
    }
}
pub(crate) unsafe fn ReelTime_PikachuReact(task: *mut Task) {
    if ({
        (*task).data[4] += 1;
        (*task).data[4]
    }) >= 60
    {
        StopMapMusic();
        DestroyReelTimeBoltSprites();
        DestroyReelTimePikachuAuraSprites();
        (*task).data[tState] += 1;
        if (*crate::gl!(sSlotMachine)).reelTimeDraw == 0 {
            (*task).data[4] = 0xa0;
            StartSpriteAnimIfDifferent(
                {
                    let gl_i6 = (*crate::gl!(sSlotMachine)).reelTimePikachuSpriteId;
                    crate::gl!(&raw gSprites[gl_i6])
                },
                5,
            );
            PlayFanfare(MUS_TOO_BAD);
        } else {
            (*task).data[4] = 0xc0;
            StartSpriteAnimIfDifferent(
                {
                    let gl_i7 = (*crate::gl!(sSlotMachine)).reelTimePikachuSpriteId;
                    crate::gl!(&raw gSprites[gl_i7])
                },
                4,
            );
            {
                let gl_i8 = (*crate::gl!(sSlotMachine)).reelTimePikachuSpriteId;
                crate::gl!(gSprites[gl_i8].animCmdIndex = 0)
            };
            if (*crate::gl!(sSlotMachine)).pikaPowerBolts != 0 {
                ResetPikaPowerBolts();
                (*crate::gl!(sSlotMachine)).pikaPowerBolts = 0;
            }
            PlayFanfare(MUS_SLOTS_WIN);
        }
    }
}
pub(crate) unsafe fn ReelTime_WaitClearPikaPower(task: *mut Task) {
    if ((*task).data[4] == 0
        || ({
            (*task).data[4] -= 1;
            (*task).data[4]
        }) == 0)
        && IsPikaPowerBoltAnimating() == 0
    {
        (*task).data[tState] += 1;
    }
}
pub(crate) unsafe fn ReelTime_CloseWindow(task: *mut Task) {
    gSpriteCoordOffsetX.set(gSpriteCoordOffsetX.get() - (8));
    (*task).data[1] += 8;
    (*task).data[3] += 8;
    let r4: i16 = ((((*task).data[1] as i32 - 8) & 0xff) >> 3) as i16;
    SetGpuReg(REG_OFFSET_BG1HOFS, (*task).data[1] as u16 & 0x1ff);
    if (*task).data[3] >> 3 <= 25 {
        ClearReelTimeWindowTilemap(r4);
    } else {
        (*task).data[tState] += 1;
    }
}
pub(crate) unsafe fn ReelTime_DestroySprites(task: *mut Task) {
    (*crate::gl!(sSlotMachine)).reelTimeSpinsUsed = 0;
    (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft = (*crate::gl!(sSlotMachine)).reelTimeDraw;
    gSpriteCoordOffsetX.set(0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    (*crate::gl!(sSlotMachine)).reelSpeed = REEL_NORMAL_SPEED;
    DestroyReelTimePikachuSprite();
    DestroyReelTimeMachineSprites();
    DestroyReelTimeShadowSprites();
    PlayNewMapMusic((*crate::gl!(sSlotMachine)).backupMapMusic);
    if (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft == 0 {
        DestroyTask(FindTaskIdByFunc(Some(Task_ReelTime)));
    } else {
        CreateDigitalDisplayScene(DIG_DISPLAY_REEL_TIME);
        (*task).data[tReelSpeed] = ReelTimeSpeed() as i16;
        (*task).data[tTimer3] = 0;
        (*task).data[3] = 0;
        (*task).data[tState] += 1;
    }
}
pub(crate) unsafe fn ReelTime_SetReelSpeed(task: *mut Task) {
    if (*crate::gl!(sSlotMachine)).reelSpeed == (*task).data[tReelSpeed] {
        (*task).data[tState] += 1;
    } else if (*crate::gl!(sSlotMachine)).reelPixelOffsets[0] % 24 == 0
        && ({
            (*task).data[tTimer3] += 1;
            (*task).data[tTimer3]
        }) as i32
            & 0x07
            == 0
    {
        (*crate::gl!(sSlotMachine)).reelSpeed >>= 1;
    }
}
pub(crate) unsafe fn ReelTime_EndSuccess(task: *mut Task) {
    if IsDigitalDisplayAnimFinished() != 0 {
        DestroyTask(FindTaskIdByFunc(Some(Task_ReelTime)));
    }
}
pub(crate) unsafe fn ReelTime_ExplodeMachine(task: *mut Task) {
    DestroyReelTimeMachineSprites();
    DestroyReelTimeBoltSprites();
    DestroyReelTimePikachuAuraSprites();
    CreateReelTimeExplosionSprite();
    (*gSprites.as_ptr())[(*crate::gl!(sSlotMachine)).reelTimeShadowSpriteIds[0]]
        .set_invisible(TRUE as u16);
    StartSpriteAnimIfDifferent(
        {
            let gl_i9 = (*crate::gl!(sSlotMachine)).reelTimePikachuSpriteId;
            crate::gl!(&raw gSprites[gl_i9])
        },
        5,
    );
    (*task).data[tState] += 1;
    (*task).data[4] = 4;
    (*task).data[tTimer1] = 0;
    StopMapMusic();
    PlayFanfare(MUS_TOO_BAD);
    PlaySE(SE_M_EXPLOSION);
}
pub(crate) unsafe fn ReelTime_WaitExplode(task: *mut Task) {
    gSpriteCoordOffsetY.set((*task).data[4]);
    SetGpuReg(REG_OFFSET_BG1VOFS, (*task).data[4] as u16);
    if (*task).data[tTimer1] as i32 & 0x01 != 0 {
        (*task).data[4] = -(*task).data[4];
    }
    if ({
        (*task).data[tTimer1] += 1;
        (*task).data[tTimer1]
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
        (*gSprites.as_ptr())[(*crate::gl!(sSlotMachine)).reelTimeShadowSpriteIds[0]]
            .set_invisible(0);
        (*task).data[tState] += 1;
        (*task).data[tTimer1] = 0;
    }
}
pub(crate) unsafe fn ReelTime_WaitSmoke(task: *mut Task) {
    gSpriteCoordOffsetY.set(0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    if IsReelTimeSmokeAnimFinished() != 0 {
        (*task).data[tState] += 1;
        DestroyReelTimeSmokeSprite();
    }
}
pub(crate) unsafe fn ReelTime_EndFailure(task: *mut Task) {
    gSpriteCoordOffsetX.set(0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    PlayNewMapMusic((*crate::gl!(sSlotMachine)).backupMapMusic);
    DestroyReelTimePikachuSprite();
    DestroyBrokenReelTimeMachineSprite();
    DestroyReelTimeShadowSprites();
    DestroyReelTimeDuckSprites();
    DestroyTask(FindTaskIdByFunc(Some(Task_ReelTime)));
}
unsafe fn LoadReelTimeWindowTilemap(a0: i16, a1: i16) {
    for i in 4..15i16 {
        LoadBgTilemap(
            1,
            (&raw const sReelTimeWindow_Tilemap[a1 as i32 + (i as i32 - 4) * 20]).cast_mut()
                as *mut c_void,
            2,
            32 * i as u16 + a0 as u16,
        );
    }
}
unsafe fn ClearReelTimeWindowTilemap(a0: i16) {
    for i in 4..15u8 {
        LoadBgTilemap(
            1,
            sEmptyTilemap.as_ptr().cast_mut() as *mut c_void,
            2,
            32 * i as u16 + a0 as u16,
        );
    }
}
unsafe fn OpenInfoBox(digDisplayId: u8) {
    let taskId: u8 = CreateTask(Some(Task_InfoBox), 1);
    task_set(taskId, 1, digDisplayId as i16);
    Task_InfoBox(taskId);
}
unsafe fn IsInfoBoxClosed() -> u8 {
    if FindTaskIdByFunc(Some(Task_InfoBox)) == TASK_NONE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Task_InfoBox(taskId: u8) {
    sInfoBoxTasks[task_get(taskId, tState)].unwrap_unchecked()(&raw mut (*gTasks.as_ptr())[taskId]);
}
pub(crate) unsafe fn InfoBox_FadeIn(task: *mut Task) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    (*task).data[tState] += 1;
}
pub(crate) unsafe fn InfoBox_WaitFade(task: *mut Task) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        (*task).data[tState] += 1;
    }
}
pub(crate) unsafe fn InfoBox_DrawWindow(task: *mut Task) {
    DestroyDigitalDisplayScene();
    LoadInfoBoxTilemap();
    AddWindow((&raw const *sWindowTemplate_InfoBox).cast_mut());
    PutWindowTilemap(WIN_INFO);
    FillWindowPixelBuffer(WIN_INFO, 0);
    (*task).data[tState] += 1;
}
pub(crate) unsafe fn InfoBox_AddText(task: *mut Task) {
    AddTextPrinterParameterized3(
        WIN_INFO,
        FONT_NORMAL,
        2,
        5,
        sColors_ReeltimeHelp.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_ReelTimeHelp).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    CopyWindowToVram(WIN_INFO, COPYWIN_FULL);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    (*task).data[tState] += 1;
}
pub(crate) unsafe fn InfoBox_WaitInput(task: *mut Task) {
    if crate::gl!(gMain.newKeys) as i32 & 6 != 0 {
        FillWindowPixelBuffer(WIN_INFO, 0);
        ClearWindowTilemap(WIN_INFO);
        CopyWindowToVram(WIN_INFO, COPYWIN_MAP);
        RemoveWindow(WIN_INFO);
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        (*task).data[tState] += 1;
    }
}
pub(crate) unsafe fn InfoBox_LoadSlotMachineTilemap(task: *mut Task) {
    LoadSlotMachineMenuTilemap();
    ShowBg(3);
    (*task).data[tState] += 1;
}
pub(crate) unsafe fn InfoBox_CreateDigitalDisplay(task: *mut Task) {
    CreateDigitalDisplayScene((*task).data[1] as u8);
    (*task).data[tState] += 1;
}
pub(crate) unsafe fn InfoBox_LoadPikaPowerMeter(task: *mut Task) {
    LoadPikaPowerMeter((*crate::gl!(sSlotMachine)).pikaPowerBolts);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    (*task).data[tState] += 1;
}
pub(crate) unsafe fn InfoBox_FreeTask(task: *mut Task) {
    DestroyTask(FindTaskIdByFunc(Some(Task_InfoBox)));
}
unsafe fn CreateDigitalDisplayTask() {
    let i: u8 = CreateTask(Some(Task_DigitalDisplay), 3);
    (*crate::gl!(sSlotMachine)).digDisplayTaskId = i;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[i];
    (*task).data[1] = -1;
    for i in 4..NUM_TASK_DATA {
        (*task).data[i] = MAX_SPRITES as i16;
    }
}
unsafe fn CreateDigitalDisplayScene(id: u8) {
    DestroyDigitalDisplayScene();
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[(*crate::gl!(sSlotMachine)).digDisplayTaskId];
    (*task).data[1] = id as i16;
    let mut i: u8 = 0;
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
unsafe fn AddDigitalDisplaySprite(
    templateIdx: u8,
    callback: Option<unsafe fn(*mut Sprite)>,
    x: i16,
    y: i16,
    spriteId: i16,
) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[(*crate::gl!(sSlotMachine)).digDisplayTaskId];
    for i in 4..NUM_TASK_DATA {
        if (*task).data[i] == MAX_SPRITES as i16 {
            (*task).data[i] =
                CreateDigitalDisplaySprite(templateIdx, callback, x, y, spriteId) as i16;
            break;
        }
    }
}
unsafe fn DestroyDigitalDisplayScene() {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[(*crate::gl!(sSlotMachine)).digDisplayTaskId];
    if (*task).data[1] as u16 != 0xFFFF {
        sDigitalDisplaySceneExitCallbacks[(*task).data[1]].unwrap_unchecked()();
    }
    for i in 4..NUM_TASK_DATA {
        if (*task).data[i] != MAX_SPRITES as i16 {
            DestroySprite({
                let gl_i10 = (*task).data[i];
                crate::gl!(&raw gSprites[gl_i10])
            });
            (*task).data[i] = MAX_SPRITES as i16;
        }
    }
}
unsafe fn IsDigitalDisplayAnimFinished() -> u8 {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[(*crate::gl!(sSlotMachine)).digDisplayTaskId];
    for i in 4..NUM_TASK_DATA {
        if (*task).data[i] != MAX_SPRITES as i16
            && ({
                let gl_i11 = (*task).data[i];
                let gl_i12 = sWaitForAnim;
                crate::gl!(gSprites[gl_i11].data[gl_i12])
            }) != 0
        {
            return FALSE;
        }
    }
    TRUE
}
pub(crate) unsafe fn Task_DigitalDisplay(taskId: u8) {
    sDigitalDisplayTasks[task_get(taskId, 0)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    );
}
pub(crate) fn DigitalDisplay_Idle(task: *mut Task) {}
unsafe fn CreateReelSymbolSprites() {
    let mut j: i16 = 0;
    let mut i: i16 = 0;
    let mut x: i16 = 0x30;
    while i < 3 {
        j = 0;
        while j < 120 {
            let sprite: *mut Sprite = (*gSprites.as_ptr()).as_mut_ptr().at(CreateSprite(
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
pub(crate) unsafe fn SpriteCB_ReelSymbol(sprite: *mut Sprite) {
    (*sprite).data[2] =
        (*crate::gl!(sSlotMachine)).reelPixelOffsets[(*sprite).data[0]] + (*sprite).data[1];
    (*sprite).data[2] %= 120;
    (*sprite).y = (*crate::gl!(sSlotMachine)).reelShockOffsets[(*sprite).data[0]] as i16
        + 28
        + (*sprite).data[2];
    (*sprite).sheetTileStart = GetSpriteTileStartByTag(GetSymbolAtRest(
        (*sprite).data[0] as u8,
        (*sprite).data[2] / 24,
    ) as u16);
    SetSpriteSheetFrameTileNum(sprite);
}
unsafe fn CreateCreditPayoutNumberSprites() {
    let mut x: i16 = 203;
    let mut i: i16 = 1;
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
unsafe fn CreateCoinNumberSprite(x: i16, y: i16, isPayout: u8, digitMult: i16) {
    let sprite: *mut Sprite = {
        let gl_i13 = CreateSprite(
            (&raw const *sSpriteTemplate_CoinNumber).cast_mut(),
            x,
            y,
            13,
        );
        crate::gl!(&raw gSprites[gl_i13])
    };
    (*sprite).oam.set_priority(2);
    (*sprite).data[sIsPayout] = isPayout as i16;
    (*sprite).data[sDigitMin] = digitMult;
    (*sprite).data[sDigitMax] = digitMult * 10;
    (*sprite).data[sCurNum] = -1;
}
pub(crate) unsafe fn SpriteCB_CoinNumber(sprite: *mut Sprite) {
    let mut tag: u16 = (*crate::gl!(sSlotMachine)).coins as u16;
    if (*sprite).data[sIsPayout] != 0 {
        tag = (*crate::gl!(sSlotMachine)).payout as u16;
    }
    if (*sprite).data[sCurNum] as i32 != tag as i32 {
        (*sprite).data[sCurNum] = tag as i16;
        tag = rem_i32(tag as i32, (*sprite).data[sDigitMax] as u16 as i32) as u16;
        tag = div_i32(tag as i32, (*sprite).data[sDigitMin] as u16 as i32) as u16;
        tag += GFXTAG_NUM_0;
        (*sprite).sheetTileStart = GetSpriteTileStartByTag(tag);
        SetSpriteSheetFrameTileNum(sprite);
    }
}
unsafe fn CreateReelBackgroundSprite() {
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelBackground).cast_mut(),
        88,
        72,
        15,
    );
    (*gSprites.as_ptr())[spriteId].oam.set_priority(3);
    SetSubspriteTables(
        crate::gl!(&raw gSprites[spriteId]),
        sSubspriteTable_ReelBackground.as_ptr().cast_mut(),
    );
}
unsafe fn CreateReelTimePikachuSprite() {
    if (*sImageTable_ReelTimePikachu.as_ptr()).is_null() {
        crate::gl!(sImageTable_ReelTimePikachu = AllocZeroed(40) as *mut SpriteFrameImage);
    }
    (*crate::gl!(sImageTable_ReelTimePikachu)).data = crate::gl!(sReelTimeGfxPtr) as *mut c_void;
    (*crate::gl!(sImageTable_ReelTimePikachu)).size = 0x800;
    (*(*sImageTable_ReelTimePikachu.as_ptr()).at(1)).data =
        (*sReelTimeGfxPtr.as_ptr()).at(2048) as *mut c_void;
    (*(*sImageTable_ReelTimePikachu.as_ptr()).at(1)).size = 0x800;
    (*(*sImageTable_ReelTimePikachu.as_ptr()).at(2)).data =
        (*sReelTimeGfxPtr.as_ptr()).at(4096) as *mut c_void;
    (*(*sImageTable_ReelTimePikachu.as_ptr()).at(2)).size = 0x800;
    (*(*sImageTable_ReelTimePikachu.as_ptr()).at(3)).data =
        (*sReelTimeGfxPtr.as_ptr()).at(6144) as *mut c_void;
    (*(*sImageTable_ReelTimePikachu.as_ptr()).at(3)).size = 0x800;
    (*(*sImageTable_ReelTimePikachu.as_ptr()).at(4)).data =
        (*sReelTimeGfxPtr.as_ptr()).at(8192) as *mut c_void;
    (*(*sImageTable_ReelTimePikachu.as_ptr()).at(4)).size = 0x800;
    let mut spriteTemplate: SpriteTemplate = *sSpriteTemplate_ReelTimePikachu;
    spriteTemplate.images = crate::gl!(sImageTable_ReelTimePikachu);
    let spriteId: u8 = CreateSprite(&raw mut spriteTemplate, 280, 80, 1);
    (*gSprites.as_ptr())[spriteId].oam.set_priority(1);
    (*gSprites.as_ptr())[spriteId].set_coordOffsetEnabled(TRUE as u16);
    (*crate::gl!(sSlotMachine)).reelTimePikachuSpriteId = spriteId;
}
unsafe fn DestroyReelTimePikachuSprite() {
    DestroySprite({
        let gl_i14 = (*crate::gl!(sSlotMachine)).reelTimePikachuSpriteId;
        crate::gl!(&raw gSprites[gl_i14])
    });
    if !(*sImageTable_ReelTimePikachu.as_ptr()).is_null() {
        Free(crate::gl!(sImageTable_ReelTimePikachu) as *mut c_void);
        crate::gl!(sImageTable_ReelTimePikachu = null_mut());
    }
}
pub(crate) unsafe fn SpriteCB_ReelTimePikachu(sprite: *mut Sprite) {
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
unsafe fn CreateReelTimeMachineSprites() {
    if (*sImageTable_ReelTimeMachineAntennae.as_ptr()).is_null() {
        crate::gl!(sImageTable_ReelTimeMachineAntennae = AllocZeroed(8) as *mut SpriteFrameImage);
    }
    (*crate::gl!(sImageTable_ReelTimeMachineAntennae)).data =
        (*sReelTimeGfxPtr.as_ptr()).at(10240) as *mut c_void;
    (*crate::gl!(sImageTable_ReelTimeMachineAntennae)).size = 0x300;
    let mut spriteTemplate: SpriteTemplate = *sSpriteTemplate_ReelTimeMachineAntennae;
    spriteTemplate.images = crate::gl!(sImageTable_ReelTimeMachineAntennae);
    let mut spriteId: u8 = CreateSprite(&raw mut spriteTemplate, 368, 52, 7);
    let mut sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
    (*sprite).oam.set_priority(1);
    (*sprite).set_coordOffsetEnabled(TRUE as u16);
    SetSubspriteTables(
        sprite,
        sSubspriteTable_ReelTimeMachineAntennae.as_ptr().cast_mut(),
    );
    (*crate::gl!(sSlotMachine)).reelTimeMachineSpriteIds[0] = spriteId;
    if (*sImageTable_ReelTimeMachine.as_ptr()).is_null() {
        crate::gl!(sImageTable_ReelTimeMachine = AllocZeroed(8) as *mut SpriteFrameImage);
    }
    (*crate::gl!(sImageTable_ReelTimeMachine)).data =
        (*sReelTimeGfxPtr.as_ptr()).at(10240).at(768) as *mut c_void;
    (*crate::gl!(sImageTable_ReelTimeMachine)).size = 0x500;
    spriteTemplate = *sSpriteTemplate_ReelTimeMachine;
    spriteTemplate.images = crate::gl!(sImageTable_ReelTimeMachine);
    spriteId = CreateSprite(&raw mut spriteTemplate, 368, 84, 7);
    sprite = crate::gl!(&raw gSprites[spriteId]);
    (*sprite).oam.set_priority(1);
    (*sprite).set_coordOffsetEnabled(TRUE as u16);
    SetSubspriteTables(sprite, sSubspriteTable_ReelTimeMachine.as_ptr().cast_mut());
    (*crate::gl!(sSlotMachine)).reelTimeMachineSpriteIds[1] = spriteId;
}
unsafe fn CreateBrokenReelTimeMachineSprite() {
    if (*sImageTable_BrokenReelTimeMachine.as_ptr()).is_null() {
        crate::gl!(sImageTable_BrokenReelTimeMachine = AllocZeroed(8) as *mut SpriteFrameImage);
    }
    (*crate::gl!(sImageTable_BrokenReelTimeMachine)).data =
        (*sReelTimeGfxPtr.as_ptr()).at(12288) as *mut c_void;
    (*crate::gl!(sImageTable_BrokenReelTimeMachine)).size = 0x600;
    let mut spriteTemplate: SpriteTemplate = *sSpriteTemplate_BrokenReelTimeMachine;
    spriteTemplate.images = crate::gl!(sImageTable_BrokenReelTimeMachine);
    let spriteId: u8 = CreateSprite(
        &raw mut spriteTemplate,
        168 - gSpriteCoordOffsetX.get(),
        80,
        7,
    );
    let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
    (*sprite).oam.set_priority(1);
    (*sprite).set_coordOffsetEnabled(TRUE as u16);
    SetSubspriteTables(
        sprite,
        sSubspriteTable_BrokenReelTimeMachine.as_ptr().cast_mut(),
    );
    (*crate::gl!(sSlotMachine)).reelTimeBrokenMachineSpriteId = spriteId;
}
unsafe fn CreateReelTimeNumberSprites() {
    let mut i: u8 = 0;
    let mut r5: i16 = 0;
    while i < 3 {
        let spriteId: u8 = CreateSprite(
            (&raw const *sSpriteTemplate_ReelTimeNumbers).cast_mut(),
            368,
            0,
            10,
        );
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).oam.set_priority(1);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).data[7] = r5;
        (*crate::gl!(sSlotMachine)).reelTimeNumberSpriteIds[i] = spriteId;
        i += 1;
        r5 += 20;
    }
}
pub(crate) unsafe fn SpriteCB_ReelTimeNumbers(sprite: *mut Sprite) {
    let mut r0: i16 = (*crate::gl!(sSlotMachine)).reeltimePixelOffset as u16 as i16
        + (*sprite).data[7] as u16 as i16;
    r0 %= 40;
    (*sprite).y = r0 + 59;
    StartSpriteAnimIfDifferent(sprite, GetReelTimeSymbol(r0 / 20));
}
unsafe fn CreateReelTimeShadowSprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeShadow).cast_mut(),
        368,
        100,
        9,
    );
    let mut sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
    (*sprite).set_coordOffsetEnabled(TRUE as u16);
    (*sprite).oam.set_priority(1);
    SetSubspriteTables(sprite, sSubspriteTable_ReelTimeShadow.as_ptr().cast_mut());
    (*crate::gl!(sSlotMachine)).reelTimeShadowSpriteIds[0] = spriteId;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeShadow).cast_mut(),
        288,
        104,
        4,
    );
    sprite = crate::gl!(&raw gSprites[spriteId]);
    (*sprite).set_coordOffsetEnabled(TRUE as u16);
    (*sprite).oam.set_priority(1);
    SetSubspriteTables(sprite, sSubspriteTable_ReelTimeShadow.as_ptr().cast_mut());
    (*crate::gl!(sSlotMachine)).reelTimeShadowSpriteIds[1] = spriteId;
}
unsafe fn CreateReelTimeNumberGapSprite() {
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeNumberGap).cast_mut(),
        368,
        76,
        11,
    );
    let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
    (*sprite).set_coordOffsetEnabled(TRUE as u16);
    (*sprite).oam.set_priority(1);
    SetSubspriteTables(
        sprite,
        sSubspriteTable_ReelTimeNumberGap.as_ptr().cast_mut(),
    );
    (*crate::gl!(sSlotMachine)).reelTimeNumberGapSpriteId = spriteId;
}
unsafe fn DestroyReelTimeMachineSprites() {
    DestroySprite({
        let gl_i15 = (*crate::gl!(sSlotMachine)).reelTimeNumberGapSpriteId;
        crate::gl!(&raw gSprites[gl_i15])
    });
    for i in 0..2u8 {
        DestroySprite({
            let gl_i16 = (*crate::gl!(sSlotMachine)).reelTimeMachineSpriteIds[i];
            crate::gl!(&raw gSprites[gl_i16])
        });
    }
    if !(*sImageTable_ReelTimeMachineAntennae.as_ptr()).is_null() {
        Free(crate::gl!(sImageTable_ReelTimeMachineAntennae) as *mut c_void);
        crate::gl!(sImageTable_ReelTimeMachineAntennae = null_mut());
    }
    if !(*sImageTable_ReelTimeMachine.as_ptr()).is_null() {
        Free(crate::gl!(sImageTable_ReelTimeMachine) as *mut c_void);
        crate::gl!(sImageTable_ReelTimeMachine = null_mut());
    }
    for i in 0..3u8 {
        DestroySprite({
            let gl_i17 = (*crate::gl!(sSlotMachine)).reelTimeNumberSpriteIds[i];
            crate::gl!(&raw gSprites[gl_i17])
        });
    }
}
unsafe fn DestroyReelTimeShadowSprites() {
    for i in 0..2u8 {
        DestroySprite({
            let gl_i18 = (*crate::gl!(sSlotMachine)).reelTimeShadowSpriteIds[i];
            crate::gl!(&raw gSprites[gl_i18])
        });
    }
}
unsafe fn DestroyBrokenReelTimeMachineSprite() {
    DestroySprite({
        let gl_i19 = (*crate::gl!(sSlotMachine)).reelTimeBrokenMachineSpriteId;
        crate::gl!(&raw gSprites[gl_i19])
    });
    if !(*sImageTable_BrokenReelTimeMachine.as_ptr()).is_null() {
        Free(crate::gl!(sImageTable_BrokenReelTimeMachine) as *mut c_void);
        crate::gl!(sImageTable_BrokenReelTimeMachine = null_mut());
    }
}
unsafe fn CreateReelTimeBoltSprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeBolt).cast_mut(),
        152,
        32,
        5,
    );
    let mut sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
    (*sprite).oam.set_priority(1);
    (*sprite).set_hFlip(TRUE as u16);
    (*crate::gl!(sSlotMachine)).reelTimeBoltSpriteIds[0] = spriteId;
    (*sprite).data[0] = 8;
    (*sprite).data[sXDir] = -1;
    (*sprite).data[sYDir] = -1;
    (*sprite).data[sDelay] = 32;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeBolt).cast_mut(),
        184,
        32,
        5,
    );
    sprite = crate::gl!(&raw gSprites[spriteId]);
    (*sprite).oam.set_priority(1);
    (*crate::gl!(sSlotMachine)).reelTimeBoltSpriteIds[1] = spriteId;
    (*sprite).data[sXDir] = 1;
    (*sprite).data[sYDir] = -1;
    (*sprite).data[sDelay] = 32;
}
pub(crate) unsafe fn SpriteCB_ReelTimeBolt(sprite: *mut Sprite) {
    if (*sprite).data[0] != 0 {
        (*sprite).data[0] -= 1;
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*sprite).set_invisible(TRUE as u16);
    } else {
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).x2 += (*sprite).data[sXDir];
        (*sprite).y2 += (*sprite).data[sYDir];
        if ({
            (*sprite).data[3] += 1;
            (*sprite).data[3]
        }) >= 8
        {
            (*sprite).data[0] = (*sprite).data[sDelay];
            (*sprite).data[3] = 0;
        }
    }
}
unsafe fn SetReelTimeBoltDelay(delay: i16) {
    {
        let gl_i20 = (*crate::gl!(sSlotMachine)).reelTimeBoltSpriteIds[0];
        let gl_i21 = sDelay;
        crate::gl!(gSprites[gl_i20].data[gl_i21] = delay)
    };
    {
        let gl_i22 = (*crate::gl!(sSlotMachine)).reelTimeBoltSpriteIds[1];
        let gl_i23 = sDelay;
        crate::gl!(gSprites[gl_i22].data[gl_i23] = delay)
    };
}
unsafe fn DestroyReelTimeBoltSprites() {
    for i in 0..2u8 {
        DestroySprite({
            let gl_i24 = (*crate::gl!(sSlotMachine)).reelTimeBoltSpriteIds[i];
            crate::gl!(&raw gSprites[gl_i24])
        });
    }
}
unsafe fn CreateReelTimePikachuAuraSprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimePikachuAura).cast_mut(),
        72,
        80,
        3,
    );
    (*gSprites.as_ptr())[spriteId].oam.set_priority(1);
    {
        let gl_i25 = sFlashPal;
        crate::gl!(gSprites[spriteId].data[gl_i25] = TRUE as i16)
    };
    {
        let gl_i26 = sColorIdx;
        crate::gl!(gSprites[spriteId].data[gl_i26] = 0)
    };
    crate::gl!(gSprites[spriteId].data[6] = 16);
    {
        let gl_i27 = sDelay;
        crate::gl!(gSprites[spriteId].data[gl_i27] = 8)
    };
    (*crate::gl!(sSlotMachine)).reelTimePikachuAuraSpriteIds[0] = spriteId;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimePikachuAura).cast_mut(),
        104,
        80,
        3,
    );
    (*gSprites.as_ptr())[spriteId].oam.set_priority(1);
    (*gSprites.as_ptr())[spriteId].set_hFlip(TRUE as u16);
    (*crate::gl!(sSlotMachine)).reelTimePikachuAuraSpriteIds[1] = spriteId;
}
pub(crate) unsafe fn SpriteCB_ReelTimePikachuAura(sprite: *mut Sprite) {
    let colors: CArray<u8, 2> = CArray([16, 0]);
    if (*sprite).data[sFlashPal] != 0
        && ({
            (*sprite).data[6] -= 1;
            (*sprite).data[6]
        }) <= 0
    {
        MultiplyInvertedPaletteRGBComponents(
            0x100 + IndexOfSpritePaletteTag(PALTAG_PIKA_AURA) as u16 * 16 + 3,
            colors[(*sprite).data[sColorIdx]],
            colors[(*sprite).data[sColorIdx]],
            colors[(*sprite).data[sColorIdx]],
        );
        (*sprite).data[sColorIdx] += 1;
        (*sprite).data[sColorIdx] &= 1;
        (*sprite).data[6] = (*sprite).data[sDelay];
    }
}
unsafe fn SetReelTimePikachuAuraFlashDelay(delay: i16) {
    {
        let gl_i28 = (*crate::gl!(sSlotMachine)).reelTimePikachuAuraSpriteIds[0];
        let gl_i29 = sDelay;
        crate::gl!(gSprites[gl_i28].data[gl_i29] = delay)
    };
}
unsafe fn DestroyReelTimePikachuAuraSprites() {
    MultiplyInvertedPaletteRGBComponents(
        0x100 + IndexOfSpritePaletteTag(PALTAG_PIKA_AURA) as u16 * 16 + 3,
        0,
        0,
        0,
    );
    for i in 0..2u8 {
        DestroySprite({
            let gl_i30 = (*crate::gl!(sSlotMachine)).reelTimePikachuAuraSpriteIds[i];
            crate::gl!(&raw gSprites[gl_i30])
        });
    }
}
unsafe fn CreateReelTimeExplosionSprite() {
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeExplosion).cast_mut(),
        168,
        80,
        6,
    );
    (*gSprites.as_ptr())[spriteId].oam.set_priority(1);
    (*crate::gl!(sSlotMachine)).reelTimeExplosionSpriteId = spriteId;
}
pub(crate) unsafe fn SpriteCB_ReelTimeExplosion(sprite: *mut Sprite) {
    (*sprite).y2 = gSpriteCoordOffsetY.get();
}
unsafe fn DestroyReelTimeExplosionSprite() {
    DestroySprite({
        let gl_i31 = (*crate::gl!(sSlotMachine)).reelTimeExplosionSpriteId;
        crate::gl!(&raw gSprites[gl_i31])
    });
}
unsafe fn CreateReelTimeDuckSprites() {
    let sp: CArray<u16, 4> = CArray([0, 64, 128, 192]);
    for i in 0..4u8 {
        let spriteId: u8 = CreateSprite(
            (&raw const *sSpriteTemplate_ReelTimeDuck).cast_mut(),
            80 - gSpriteCoordOffsetX.get(),
            68,
            0,
        );
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).oam.set_priority(1);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).data[0] = sp[i] as i16;
        (*crate::gl!(sSlotMachine)).reelTimeDuckSpriteIds[i] = spriteId;
    }
}
pub(crate) unsafe fn SpriteCB_ReelTimeDuck(sprite: *mut Sprite) {
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
unsafe fn DestroyReelTimeDuckSprites() {
    for i in 0..4u8 {
        DestroySprite({
            let gl_i32 = (*crate::gl!(sSlotMachine)).reelTimeDuckSpriteIds[i];
            crate::gl!(&raw gSprites[gl_i32])
        });
    }
}
unsafe fn CreateReelTimeSmokeSprite() {
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ReelTimeSmoke).cast_mut(),
        168,
        60,
        8,
    );
    let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
    (*sprite).oam.set_priority(1);
    (*sprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    InitSpriteAffineAnim(sprite);
    (*crate::gl!(sSlotMachine)).reelTimeSmokeSpriteId = spriteId;
}
pub(crate) unsafe fn SpriteCB_ReelTimeSmoke(sprite: *mut Sprite) {
    if (*sprite).data[sState] == 0 {
        if (*sprite).affineAnimEnded() != 0 {
            (*sprite).data[sState] += 1;
        }
    } else if (*sprite).data[sState] == 1 {
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
        if ({
            (*sprite).data[sTimer] += 1;
            (*sprite).data[sTimer]
        }) >= 24
        {
            (*sprite).data[sState] += 1;
            (*sprite).data[sTimer] = 0;
        }
    } else {
        (*sprite).set_invisible(TRUE as u16);
        if ({
            (*sprite).data[sTimer] += 1;
            (*sprite).data[sTimer]
        }) >= 16
        {
            (*sprite).data[sAnimFinished] = TRUE as i16;
        }
    }
    (*sprite).data[sMoveY] &= 0xff;
    (*sprite).data[sMoveY] += 16;
    (*sprite).y2 -= (*sprite).data[sMoveY] >> 8;
}
unsafe fn IsReelTimeSmokeAnimFinished() -> u8 {
    ({
        let gl_i33 = (*crate::gl!(sSlotMachine)).reelTimeSmokeSpriteId;
        let gl_i34 = sAnimFinished;
        crate::gl!(gSprites[gl_i33].data[gl_i34])
    }) as u8
}
unsafe fn DestroyReelTimeSmokeSprite() {
    let sprite: *mut Sprite = {
        let gl_i35 = (*crate::gl!(sSlotMachine)).reelTimeSmokeSpriteId;
        crate::gl!(&raw gSprites[gl_i35])
    };
    FreeOamMatrix((*sprite).oam.matrixNum() as u8);
    DestroySprite(sprite);
}
unsafe fn CreatePikaPowerBoltSprite(x: i16, y: i16) -> u8 {
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_PikaPowerBolt).cast_mut(),
        x,
        y,
        12,
    );
    let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
    (*sprite).oam.set_priority(2);
    (*sprite).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
    InitSpriteAffineAnim(sprite);
    spriteId
}
pub(crate) unsafe fn SpriteCB_PikaPowerBolt(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        (*sprite).data[7] = TRUE as i16;
    }
}
unsafe fn DestroyPikaPowerBoltSprite(spriteId: u8) {
    let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
    FreeOamMatrix((*sprite).oam.matrixNum() as u8);
    DestroySprite(sprite);
}
unsafe fn CreateStdDigitalDisplaySprite(templateIdx: u8, dispInfoId: u8, spriteId: i16) -> u8 {
    CreateDigitalDisplaySprite(
        templateIdx,
        sDigitalDisplay_SpriteCallbacks[dispInfoId],
        sDigitalDisplay_SpriteCoords[dispInfoId][0],
        sDigitalDisplay_SpriteCoords[dispInfoId][1],
        spriteId,
    )
}
unsafe fn CreateDigitalDisplaySprite(
    templateIdx: u8,
    callback: Option<unsafe fn(*mut Sprite)>,
    x: i16,
    y: i16,
    internalSpriteId: i16,
) -> u8 {
    let mut spriteTemplate: SpriteTemplate = *sSpriteTemplates_DigitalDisplay[templateIdx];
    spriteTemplate.images = crate::gl!(sImageTables_DigitalDisplay[templateIdx]);
    let spriteId: u8 = CreateSprite(&raw mut spriteTemplate, x, y, 16);
    let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
    (*sprite).oam.set_priority(3);
    (*sprite).callback = callback;
    (*sprite).data[sSpriteId] = internalSpriteId;
    (*sprite).data[sWaitForAnim] = TRUE as i16;
    if !sSubspriteTables_DigitalDisplay[templateIdx].is_null() {
        SetSubspriteTables(sprite, sSubspriteTables_DigitalDisplay[templateIdx]);
    }
    spriteId
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_Static(sprite: *mut Sprite) {
    (*sprite).data[sWaitForAnim] = FALSE as i16;
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_Smoke(sprite: *mut Sprite) {
    let targetX: CArray<i16, 4> = CArray([4, -4, 4, -4]);
    let targetY: CArray<i16, 4> = CArray([4, 4, -4, -4]);
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
        (*sprite).x2 = targetX[(*sprite).data[sSpriteId]];
        (*sprite).y2 = targetY[(*sprite).data[sSpriteId]];
    }
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_SmokeNE(sprite: *mut Sprite) {
    (*sprite).set_hFlip(TRUE as u16);
    SpriteCB_DigitalDisplay_Smoke(sprite);
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_SmokeSW(sprite: *mut Sprite) {
    (*sprite).set_vFlip(TRUE as u16);
    SpriteCB_DigitalDisplay_Smoke(sprite);
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_SmokeSE(sprite: *mut Sprite) {
    (*sprite).set_hFlip(TRUE as u16);
    (*sprite).set_vFlip(TRUE as u16);
    SpriteCB_DigitalDisplay_Smoke(sprite);
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_Reel(sprite: *mut Sprite) {
    match (*sprite).data[sState] {
        0 => {
            (*sprite).x += 4;
            if (*sprite).x >= 208 {
                (*sprite).x = 208;
                (*sprite).data[sState] += 1;
            }
        }
        1 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 90
            {
                (*sprite).data[sState] += 1;
            }
        }
        2 => {
            (*sprite).x += 4;
            if (*sprite).x >= 272 {
                (*sprite).data[sState] += 1;
            }
        }
        3 => {
            (*sprite).data[sWaitForAnim] = FALSE as i16;
        }
        _ => {}
    }
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_Time(sprite: *mut Sprite) {
    match (*sprite).data[sState] {
        0 => {
            (*sprite).x -= 4;
            if (*sprite).x <= 208 {
                (*sprite).x = 208;
                (*sprite).data[sState] += 1;
            }
        }
        1 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 90
            {
                (*sprite).data[sState] += 1;
            }
        }
        2 => {
            (*sprite).x -= 4;
            if (*sprite).x <= 144 {
                (*sprite).data[sState] += 1;
            }
        }
        3 => {
            (*sprite).data[sWaitForAnim] = FALSE as i16;
        }
        _ => {}
    }
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_ReelTimeNumber(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[sState];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            StartSpriteAnim(sprite, (*crate::gl!(sSlotMachine)).reelTimeSpinsLeft - 1);
            (*sprite).data[sState] += 1;
        }
        if fall || sw1 == 1 {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) >= 4
            {
                (*sprite).data[sState] += 1;
                (*sprite).data[1] = 0;
            }
            break 'l1;
        }
        if sw1 == 2 {
            (*sprite).x += 4;
            if (*sprite).x >= 208 {
                (*sprite).x = 208;
                (*sprite).data[sState] += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 90
            {
                (*sprite).data[sState] += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            (*sprite).x += 4;
            if (*sprite).x >= 248 {
                (*sprite).data[sState] += 1;
            }
            break 'l1;
        }
        if sw1 == 5 {
            (*sprite).data[sWaitForAnim] = FALSE as i16;
            break 'l1;
        }
    }
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_PokeballRocking(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[sState];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).set_animPaused(TRUE);
            (*sprite).data[sState] += 1;
        }
        if fall || sw1 == 1 {
            (*sprite).y += 8;
            if (*sprite).y >= 0x70 {
                (*sprite).y = 0x70;
                (*sprite).data[1] = 16;
                (*sprite).data[sState] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
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
                        (*sprite).data[sState] += 1;
                        (*sprite).data[sWaitForAnim] = FALSE as i16;
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
pub(crate) unsafe fn SpriteCB_DigitalDisplay_Stop(sprite: *mut Sprite) {
    match (*sprite).data[sState] {
        0 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 8
            {
                (*sprite).data[sState] += 1;
            }
        }
        1 => {
            (*sprite).y += 2;
            if (*sprite).y >= 0x30 {
                (*sprite).y = 0x30;
                (*sprite).data[sState] += 1;
                (*sprite).data[sWaitForAnim] = FALSE as i16;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_AButtonStop(sprite: *mut Sprite) {
    match (*sprite).data[sState] {
        0 => {
            (*sprite).set_invisible(TRUE as u16);
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 0x20
            {
                (*sprite).data[sState] += 1;
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
                (*sprite).data[sState] += 1;
                (*sprite).data[sWaitForAnim] = FALSE as i16;
                (*sprite).oam.set_mosaic(FALSE as u32);
                StartSpriteAnim(sprite, 0);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_PokeballShining(sprite: *mut Sprite) {
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
    (*sprite).data[sWaitForAnim] = FALSE as i16;
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_RegBonus(sprite: *mut Sprite) {
    let letterXOffset: CArray<i16, 8> = CArray([0, -40, 0, 0, 48, 0, 24, 0]);
    let letterYOffset: CArray<i16, 8> = CArray([-32, 0, -32, -48, 0, -48, 0, -48]);
    let letterDelay: CArray<i16, 8> = CArray([16, 12, 16, 0, 0, 4, 8, 8]);
    'l1: {
        let sw1: i16 = (*sprite).data[sState];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).x2 = letterXOffset[(*sprite).data[sSpriteId]];
            (*sprite).y2 = letterYOffset[(*sprite).data[sSpriteId]];
            (*sprite).data[1] = letterDelay[(*sprite).data[sSpriteId]];
            (*sprite).data[sState] += 1;
        }
        if fall || sw1 == 1 {
            if ({
                let t2 = (*sprite).data[1];
                (*sprite).data[1] -= 1;
                t2
            }) == 0
            {
                (*sprite).data[sState] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
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
                (*sprite).data[sState] += 1;
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_BigBonus(sprite: *mut Sprite) {
    let sp0: CArray<i16, 8> = CArray([160, 192, 224, 104, 80, 64, 48, 24]);
    if (*sprite).data[sState] == 0 {
        (*sprite).data[sState] += 1;
        (*sprite).data[1] = 12;
    }
    (*sprite).x2 = Cos(sp0[(*sprite).data[sSpriteId]], (*sprite).data[1]);
    (*sprite).y2 = Sin(sp0[(*sprite).data[sSpriteId]], (*sprite).data[1]);
    if (*sprite).data[1] != 0 {
        (*sprite).data[1] -= 1;
    }
}
pub(crate) unsafe fn SpriteCB_DigitalDisplay_AButtonStart(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[sState];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*crate::gl!(sSlotMachine)).winIn = 47;
            (*crate::gl!(sSlotMachine)).winOut = 63;
            (*crate::gl!(sSlotMachine)).win0v = 8328;
            (*sprite).set_invisible(TRUE as u16);
            (*sprite).data[sState] += 1;
        }
        if fall || sw1 == 1 {
            (*sprite).data[1] += 2;
            (*sprite).data[2] = (*sprite).data[1] + 176;
            (*sprite).data[3] = DISPLAY_WIDTH as i16 - (*sprite).data[1];
            if (*sprite).data[2] > 208 {
                (*sprite).data[2] = 208;
            }
            if (*sprite).data[3] < 208 {
                (*sprite).data[3] = 208;
            }
            (*crate::gl!(sSlotMachine)).win0h =
                ((*sprite).data[2] as u16) << 8 | (*sprite).data[3] as u16;
            if (*sprite).data[1] > 51 {
                (*sprite).data[sState] += 1;
                (*crate::gl!(sSlotMachine)).winIn = 63;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if (*crate::gl!(sSlotMachine)).bet == 0 {
                break 'l1;
            }
            AddDigitalDisplaySprite(DIG_SPRITE_A_BUTTON, Some(SpriteCallbackDummy), 208, 116, 0);
            (*crate::gl!(sSlotMachine)).win0h = 49376;
            (*crate::gl!(sSlotMachine)).win0v = 26752;
            (*crate::gl!(sSlotMachine)).winIn = 47;
            (*sprite).data[sState] += 1;
            (*sprite).data[1] = 0;
        }
        if fall || sw1 == 3 {
            (*sprite).data[1] += 2;
            (*sprite).data[2] = (*sprite).data[1] + 192;
            (*sprite).data[3] = 224 - (*sprite).data[1];
            if (*sprite).data[2] > 208 {
                (*sprite).data[2] = 208;
            }
            if (*sprite).data[3] < 208 {
                (*sprite).data[3] = 208;
            }
            (*crate::gl!(sSlotMachine)).win0h =
                ((*sprite).data[2] as u16) << 8 | (*sprite).data[3] as u16;
            if (*sprite).data[1] > 15 {
                (*sprite).data[sState] += 1;
                (*crate::gl!(sSlotMachine)).winIn = 63;
            }
            break 'l1;
        }
    }
}
pub(crate) fn EndDigitalDisplayScene_Dummy() {}
pub(crate) unsafe fn EndDigitalDisplayScene_StopReel() {
    SetGpuReg(REG_OFFSET_MOSAIC, 0);
}
pub(crate) unsafe fn EndDigitalDisplayScene_Win() {
    LoadPalette(
        *sDigitalDisplay_Pal as *mut c_void,
        0x100 + IndexOfSpritePaletteTag(PALTAG_DIG_DISPLAY) as u16 * 16,
        32,
    );
}
pub(crate) unsafe fn EndDigitalDisplayScene_InsertBet() {
    (*crate::gl!(sSlotMachine)).win0h = DISPLAY_WIDTH;
    (*crate::gl!(sSlotMachine)).win0v = DISPLAY_HEIGHT;
    (*crate::gl!(sSlotMachine)).winIn = 63;
    (*crate::gl!(sSlotMachine)).winOut = 63;
}
unsafe fn LoadSlotMachineGfx() {
    LoadReelBackground();
    crate::gl!(sDigitalDisplayGfxPtr = Alloc(0x3200) as *mut u8);
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gSlotMachineDigitalDisplay_Gfx)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        crate::gl!(sDigitalDisplayGfxPtr) as *mut c_void,
    );
    crate::gl!(sReelTimeGfxPtr = Alloc(0x3600) as *mut u8);
    LZDecompressWram(
        sReelTimeGfx.as_ptr().cast_mut(),
        crate::gl!(sReelTimeGfxPtr) as *mut c_void,
    );
    crate::gl!(sSlotMachineSpritesheetsPtr = AllocZeroed(176) as *mut SpriteSheet);
    for i in 0..22u8 {
        (*(*sSlotMachineSpritesheetsPtr.as_ptr()).at(i)).data = sSlotMachineSpriteSheets[i].data;
        (*(*sSlotMachineSpritesheetsPtr.as_ptr()).at(i)).size = sSlotMachineSpriteSheets[i].size;
        (*(*sSlotMachineSpritesheetsPtr.as_ptr()).at(i)).tag = sSlotMachineSpriteSheets[i].tag;
    }
    (*(*sSlotMachineSpritesheetsPtr.as_ptr()).at(17)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(2560) as *mut c_void;
    (*(*sSlotMachineSpritesheetsPtr.as_ptr()).at(18)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(5120) as *mut c_void;
    (*(*sSlotMachineSpritesheetsPtr.as_ptr()).at(19)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(5632) as *mut c_void;
    (*(*sSlotMachineSpritesheetsPtr.as_ptr()).at(20)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(6400) as *mut c_void;
    LoadSpriteSheets(crate::gl!(sSlotMachineSpritesheetsPtr));
    LoadSpritePalettes(sSlotMachineSpritePalettes.as_ptr().cast_mut());
}
unsafe fn LoadReelBackground() {
    let mut j: u8 = 0;
    crate::gl!(sReelBackgroundSpriteSheet = AllocZeroed(8) as *mut SpriteSheet);
    crate::gl!(sReelBackground_Gfx = AllocZeroed(0x2000) as *mut u8);
    let mut dest: *mut u8 = crate::gl!(sReelBackground_Gfx);
    for i in 0..0x40 {
        j = 0;
        while j < 0x20 {
            *dest = *(*sReelBackground_Tilemap).at(j);
            j += 1;
            dest = dest.at(1);
        }
    }
    (*crate::gl!(sReelBackgroundSpriteSheet)).data = crate::gl!(sReelBackground_Gfx) as *mut c_void;
    (*crate::gl!(sReelBackgroundSpriteSheet)).size = 0x800;
    (*crate::gl!(sReelBackgroundSpriteSheet)).tag = GFXTAG_REEL_BG;
    LoadSpriteSheet(crate::gl!(sReelBackgroundSpriteSheet));
}
unsafe fn LoadMenuGfx() {
    crate::gl!(sMenuGfx = Alloc(0x2200) as *mut u16);
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gSlotMachineMenu_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        crate::gl!(sMenuGfx) as *mut c_void,
    );
    LoadBgTiles(2, crate::gl!(sMenuGfx) as *mut c_void, 0x2200, 0);
    LoadPalette(
        (*(&raw const crate::data::graphics::gSlotMachineMenu_Pal).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        160,
    );
    LoadPalette(sUnkPalette.as_ptr().cast_mut() as *mut c_void, 208, 32);
}
unsafe fn LoadMenuAndReelOverlayTilemaps() {
    LoadSlotMachineMenuTilemap();
    LoadSlotMachineReelOverlay();
}
unsafe fn LoadSlotMachineMenuTilemap() {
    LoadBgTilemap(
        2,
        (*(&raw const crate::data::graphics::gSlotMachineMenu_Tilemap).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0x500,
        0,
    );
}
unsafe fn LoadSlotMachineReelOverlay() {
    let mut x: i16 = 4;
    while x < 18 {
        for dx in 0..4i16 {
            LoadBgTilemap(
                3,
                crate::gl!(sReelOverlay_Tilemap) as *mut c_void,
                2,
                x as u16 + dx as u16 + 160,
            );
            LoadBgTilemap(
                3,
                (*sReelOverlay_Tilemap.as_ptr()).at(1) as *mut c_void,
                2,
                x as u16 + dx as u16 + 416,
            );
            LoadBgTilemap(
                3,
                (*sReelOverlay_Tilemap.as_ptr()).at(2) as *mut c_void,
                2,
                x as u16 + dx as u16 + 192,
            );
            LoadBgTilemap(
                3,
                (*sReelOverlay_Tilemap.as_ptr()).at(3) as *mut c_void,
                2,
                x as u16 + dx as u16 + 384,
            );
        }
        LoadBgTilemap(
            3,
            (*sReelOverlay_Tilemap.as_ptr()).at(4) as *mut c_void,
            2,
            x as u16 + 192,
        );
        LoadBgTilemap(
            3,
            (*sReelOverlay_Tilemap.as_ptr()).at(5) as *mut c_void,
            2,
            x as u16 + 384,
        );
        for y in 7..=11i16 {
            LoadBgTilemap(
                3,
                (*sReelOverlay_Tilemap.as_ptr()).at(6) as *mut c_void,
                2,
                x as u16 + y as u16 * 32,
            );
        }
        x += 5;
    }
}
unsafe fn SetReelButtonTilemap(
    offset: i16,
    topLeft: u16,
    topRight: u16,
    bottomLeft: u16,
    bottomRight: u16,
) {
    *crate::gl!(sReelButtonPress_Tilemap) = topLeft;
    *(*sReelButtonPress_Tilemap.as_ptr()).at(1) = topRight;
    *(*sReelButtonPress_Tilemap.as_ptr()).at(2) = bottomLeft;
    *(*sReelButtonPress_Tilemap.as_ptr()).at(3) = bottomRight;
    LoadBgTilemap(
        2,
        crate::gl!(sReelButtonPress_Tilemap) as *mut c_void,
        2,
        480 + offset as u16,
    );
    LoadBgTilemap(
        2,
        (*sReelButtonPress_Tilemap.as_ptr()).at(1) as *mut c_void,
        2,
        481 + offset as u16,
    );
    LoadBgTilemap(
        2,
        (*sReelButtonPress_Tilemap.as_ptr()).at(2) as *mut c_void,
        2,
        512 + offset as u16,
    );
    LoadBgTilemap(
        2,
        (*sReelButtonPress_Tilemap.as_ptr()).at(3) as *mut c_void,
        2,
        513 + offset as u16,
    );
}
unsafe fn LoadInfoBoxTilemap() {
    LoadBgTilemap(
        2,
        (*(&raw const crate::data::graphics::gSlotMachineInfoBox_Tilemap).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0x500,
        0,
    );
    HideBg(3);
}
fn SetDigitalDisplayImagePtrs() {
    crate::gl!(sImageTables_DigitalDisplay[0] = crate::gl!(sImageTable_DigitalDisplay_Reel));
    crate::gl!(sImageTables_DigitalDisplay[1] = crate::gl!(sImageTable_DigitalDisplay_Time));
    crate::gl!(sImageTables_DigitalDisplay[2] = crate::gl!(sImageTable_DigitalDisplay_Insert));
    crate::gl!(sImageTables_DigitalDisplay[3] = crate::gl!(sImageTable_DigitalDisplay_Win));
    crate::gl!(sImageTables_DigitalDisplay[4] = crate::gl!(sImageTable_DigitalDisplay_Lose));
    crate::gl!(sImageTables_DigitalDisplay[5] = crate::gl!(sImageTable_DigitalDisplay_AButton));
    crate::gl!(sImageTables_DigitalDisplay[6] = crate::gl!(sImageTable_DigitalDisplay_Smoke));
    crate::gl!(sImageTables_DigitalDisplay[7] = crate::gl!(sImageTable_DigitalDisplay_Number));
    crate::gl!(sImageTables_DigitalDisplay[8] = crate::gl!(sImageTable_DigitalDisplay_Pokeball));
    crate::gl!(sImageTables_DigitalDisplay[9] = crate::gl!(sImageTable_DigitalDisplay_DPad));
    crate::gl!(sImageTables_DigitalDisplay[10] = crate::gl!(sImageTable_DigitalDisplay_Stop));
    crate::gl!(sImageTables_DigitalDisplay[11] = crate::gl!(sImageTable_DigitalDisplay_Stop));
    crate::gl!(sImageTables_DigitalDisplay[12] = crate::gl!(sImageTable_DigitalDisplay_Stop));
    crate::gl!(sImageTables_DigitalDisplay[13] = crate::gl!(sImageTable_DigitalDisplay_Stop));
    crate::gl!(sImageTables_DigitalDisplay[14] = crate::gl!(sImageTable_DigitalDisplay_Bonus));
    crate::gl!(sImageTables_DigitalDisplay[15] = crate::gl!(sImageTable_DigitalDisplay_Bonus));
    crate::gl!(sImageTables_DigitalDisplay[16] = crate::gl!(sImageTable_DigitalDisplay_Bonus));
    crate::gl!(sImageTables_DigitalDisplay[17] = crate::gl!(sImageTable_DigitalDisplay_Bonus));
    crate::gl!(sImageTables_DigitalDisplay[18] = crate::gl!(sImageTable_DigitalDisplay_Bonus));
    crate::gl!(sImageTables_DigitalDisplay[19] = crate::gl!(sImageTable_DigitalDisplay_Big));
    crate::gl!(sImageTables_DigitalDisplay[20] = crate::gl!(sImageTable_DigitalDisplay_Big));
    crate::gl!(sImageTables_DigitalDisplay[21] = crate::gl!(sImageTable_DigitalDisplay_Big));
    crate::gl!(sImageTables_DigitalDisplay[22] = crate::gl!(sImageTable_DigitalDisplay_Reg));
    crate::gl!(sImageTables_DigitalDisplay[23] = crate::gl!(sImageTable_DigitalDisplay_Reg));
    crate::gl!(sImageTables_DigitalDisplay[24] = crate::gl!(sImageTable_DigitalDisplay_Reg));
    crate::gl!(sImageTables_DigitalDisplay[25] = null_mut());
}
unsafe fn AllocDigitalDisplayGfx() {
    crate::gl!(sImageTable_DigitalDisplay_Reel = AllocZeroed(8) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_Reel)).data =
        crate::gl!(sDigitalDisplayGfxPtr) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_Reel)).size = 0x600;
    crate::gl!(sImageTable_DigitalDisplay_Time = AllocZeroed(8) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_Time)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(1536) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_Time)).size = 0x200;
    crate::gl!(sImageTable_DigitalDisplay_Insert = AllocZeroed(8) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_Insert)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(2048) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_Insert)).size = 0x200;
    crate::gl!(sImageTable_DigitalDisplay_Stop = AllocZeroed(8) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_Stop)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(2560) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_Stop)).size = 0x200;
    crate::gl!(sImageTable_DigitalDisplay_Win = AllocZeroed(8) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_Win)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(3072) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_Win)).size = 0x300;
    crate::gl!(sImageTable_DigitalDisplay_Lose = AllocZeroed(8) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_Lose)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(4096) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_Lose)).size = 0x400;
    crate::gl!(sImageTable_DigitalDisplay_Bonus = AllocZeroed(8) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_Bonus)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(5120) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_Bonus)).size = 0x200;
    crate::gl!(sImageTable_DigitalDisplay_Big = AllocZeroed(8) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_Big)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(5632) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_Big)).size = 0x300;
    crate::gl!(sImageTable_DigitalDisplay_Reg = AllocZeroed(8) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_Reg)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(6400) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_Reg)).size = 0x300;
    crate::gl!(sImageTable_DigitalDisplay_AButton = AllocZeroed(16) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_AButton)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(7168) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_AButton)).size = 0x200;
    (*(*sImageTable_DigitalDisplay_AButton.as_ptr()).at(1)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(7680) as *mut c_void;
    (*(*sImageTable_DigitalDisplay_AButton.as_ptr()).at(1)).size = 0x200;
    crate::gl!(sImageTable_DigitalDisplay_Smoke = AllocZeroed(8) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_Smoke)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(8192) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_Smoke)).size = 640;
    crate::gl!(sImageTable_DigitalDisplay_Number = AllocZeroed(40) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_Number)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(8832) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_Number)).size = 0x80;
    (*(*sImageTable_DigitalDisplay_Number.as_ptr()).at(1)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(8960) as *mut c_void;
    (*(*sImageTable_DigitalDisplay_Number.as_ptr()).at(1)).size = 0x80;
    (*(*sImageTable_DigitalDisplay_Number.as_ptr()).at(2)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(9088) as *mut c_void;
    (*(*sImageTable_DigitalDisplay_Number.as_ptr()).at(2)).size = 0x80;
    (*(*sImageTable_DigitalDisplay_Number.as_ptr()).at(3)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(9216) as *mut c_void;
    (*(*sImageTable_DigitalDisplay_Number.as_ptr()).at(3)).size = 0x80;
    (*(*sImageTable_DigitalDisplay_Number.as_ptr()).at(4)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(9344) as *mut c_void;
    (*(*sImageTable_DigitalDisplay_Number.as_ptr()).at(4)).size = 0x80;
    crate::gl!(sImageTable_DigitalDisplay_Pokeball = AllocZeroed(16) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_Pokeball)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(9728) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_Pokeball)).size = 0x480;
    (*(*sImageTable_DigitalDisplay_Pokeball.as_ptr()).at(1)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(10880) as *mut c_void;
    (*(*sImageTable_DigitalDisplay_Pokeball.as_ptr()).at(1)).size = 0x480;
    crate::gl!(sImageTable_DigitalDisplay_DPad = AllocZeroed(16) as *mut SpriteFrameImage);
    (*crate::gl!(sImageTable_DigitalDisplay_DPad)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(12032) as *mut c_void;
    (*crate::gl!(sImageTable_DigitalDisplay_DPad)).size = 0x180;
    (*(*sImageTable_DigitalDisplay_DPad.as_ptr()).at(1)).data =
        (*sDigitalDisplayGfxPtr.as_ptr()).at(12416) as *mut c_void;
    (*(*sImageTable_DigitalDisplay_DPad.as_ptr()).at(1)).size = 0x180;
}
