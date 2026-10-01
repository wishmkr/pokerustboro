//! Translated from `src/battle_main.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::agb_main::{SetHBlankCallback, SetVBlankCallback};
use crate::battle_ai_script_commands::{ClearBattlerAbilityHistory, ClearBattlerMoveHistory};
use crate::battle_anim_mons::{GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide};
use crate::battle_arena::BattleArena_InitPoints;
use crate::battle_bg::{
    DrawBattleEntryBackground, InitBattleBgsVideo, InitLinkBattleVsScreen, LoadBattleMenuWindowGfx,
    LoadBattleTextboxAndBackground, LoadChosenBattleElement,
};
use crate::battle_controllers::{
    BtlController_EmitChooseAction, BtlController_EmitChooseItem, BtlController_EmitChooseMove,
    BtlController_EmitChoosePokemon, BtlController_EmitDrawPartyStatusSummary,
    BtlController_EmitDrawTrainerPic, BtlController_EmitEndBounceEffect,
    BtlController_EmitGetMonData, BtlController_EmitIntroSlide,
    BtlController_EmitIntroTrainerBallThrow, BtlController_EmitLinkStandbyMsg,
    BtlController_EmitLoadMonSprite, BtlController_EmitSwitchInAnim, HandleLinkBattleSetup,
    InitBattleControllers, SetUpBattleVarsAndBirchZigzagoon,
};
use crate::battle_gfx_sfx_util::{
    AllocateBattleSpritesData, AllocateMonSpritesGfx, BattleInitAllSprites, BattleStopLowHpSound,
    FillAroundBattleWindows, FreeBattleSpritesData, FreeMonSpritesGfx,
};
use crate::battle_interface::SetHealthboxSpriteVisible;
use crate::battle_message::{
    BattlePutTextOnWindow, BattleStringExpandPlaceholdersToDisplayedString,
};
use crate::battle_script_commands::{
    BattleCreateYesNoCursorAt, BattleDestroyYesNoCursorAt, HandleBattleWindow,
};
use crate::battle_setup::{
    BattleSetup_GetEnvironmentId, gPartnerTrainerId, gTrainerBattleOpponent_A,
    gTrainerBattleOpponent_B,
};
use crate::battle_tower::{
    GetBattleTowerTrainerLanguage, GetFrontierTrainerName, TrySetLinkBattleTowerEnemyPartyLevel,
};
use crate::battle_util::{
    AbilityBattleEffects, AreAllMovesUnusable, BattleScriptExecute, CancelMultiTurnMoves,
    DoBattlerEndTurnEffects, DoFieldEndTurnEffects, HandleFaintedMonActions,
    HandleWishPerishSongOnTurnEnd, ItemBattleEffects, MarkBattlerForControllerExec,
    PrepareStringBattle, ResetSentPokesToOpponentValue, TryClearRageStatuses,
    TrySetCantSelectMoveBattleScript,
};
use crate::battle_util2::{
    AllocateBattleResources, FreeBattleResources, SwitchPartyOrderInGameMulti,
};
use crate::berry::{GetBerryInfo, IsEnigmaBerryValid, ItemIdToBerryType};
use crate::bg::{IsDma3ManagerBusyWithBgCopy, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
use crate::cable_club::Task_ReconnectWithLinkPlayers;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::FlagGet;
use crate::evolution_scene::{EvolutionScene, gCB2_AfterEvolution};
use crate::ffi::gSpecialVar_Result;
use crate::gpu_regs::SetGpuReg;
use crate::international_string_util::PadNameString;
use crate::item::{GetItemHoldEffect, GetItemHoldEffectParam};
use crate::link::gBlockRecvBuffer;
use crate::link::{
    BitmaskAllOtherLinkPlayers, GetBlockReceivedStatus, GetMultiplayerId, IsLinkTaskFinished,
    ResetBlockReceivedFlags, SendBlock, SetCloseLinkCallback, SetLinkStandbyCallback, gLinkPlayers,
    gReceivedRemoteLinkPlayers, gWirelessCommType,
};
use crate::link_rfu_2::IsLinkRfuTaskFinished;
use crate::link_rfu_3::{
    CreateWirelessStatusIndicatorSprite, LoadWirelessStatusIndicatorSpriteGfx,
};
use crate::load_save::MoveSaveBlocks_ResetHeap;
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::m4a::{gMPlayInfo_SE1, gMPlayInfo_SE2, m4aMPlayStop, m4aSongNumStop};
use crate::palette::{
    BeginFastPaletteFade, BeginNormalPaletteFade, LoadCompressedPalette, ResetPaletteFade,
    ResetPaletteFadeControl, TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::party_menu::gBattlePartyCurrentOrder;
use crate::party_menu::{
    GetPartyIdFromBattlePartyId, ShowPartyMenuToShowcaseMultiBattleParty, SwitchPartyMonSlots,
};
use crate::pokeball::StartHealthboxSlideIn;
use crate::pokemon::{
    AdjustFriendship, BattleAnimateBackSprite, BattleAnimateFrontSprite, CalculatePPWithBonus,
    CreateMon, GetAbilityBySpecies, GetEvolutionTargetSpecies, GetMonData2, GetMonData3,
    GetMonGender, HandleSetPokedexFlag, HasTwoFramesAnimation, IsMonShiny,
    IsPlayerPartyAndPokemonStorageFull, PartySpreadPokerus, RandomlyGivePartyPokerus,
    SetDeoxysStats, SetMonData, SetWildMonHeldItem, SpeciesToNationalPokedexNum,
    ZeroEnemyPartyMons, gEnemyParty, gPlayerParty,
};
use crate::random::Random;
use crate::recorded_battle::{
    GetBattleSceneInRecordedBattle, MoveRecordedBattleToSaveData, RecordedBattle_CanStopPlayback,
    RecordedBattle_CheckMovesetChanges, RecordedBattle_ClearBattlerAction,
    RecordedBattle_ClearFrontierPassFlag, RecordedBattle_CopyBattlerMoves,
    RecordedBattle_GetFrontierPassFlag, RecordedBattle_SetBattlerAction,
    RecordedBattle_SetFrontierPassFlagFromHword, RecordedBattle_SetPlaybackFinished,
    RecordedBattle_SetTrainerInfo, gBattlePalaceMoveSelectionRngValue, gRecordedBattleRngSeed,
};
use crate::roamer::SetRoamerInactive;
use crate::scanline_effect::gScanlineEffect;
use crate::scanline_effect::{ScanlineEffect_Clear, ScanlineEffect_InitHBlankDmaTransfer};
use crate::sound::{FadeOutMapMusic, PlayBGM, PlaySE, StopCryAndClearCrySongs};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeSpritePaletteByTag,
    FreeSpriteTilesByTag, LoadOam, ProcessSpriteCopyRequests, ResetSpriteData,
    gReservedSpritePaletteCount,
};
use crate::string_util::StringCompareWithoutExtCtrlCodes;
use crate::string_util::StripExtCtrlCodes;
use crate::task::{ResetTasks, RunTasks};
use crate::task::{task_get, task_set};
use crate::text::{IsTextPrinterActive, RunTextPrinters};
use crate::trig::Sin;
use crate::tv::{TryPutBreakingNewsOnAir, TryPutPokemonTodayOnAir};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
use crate::window::FreeAllWindowBuffers;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateInvisibleSpriteWithCallback` with this module's view of its types.
#[inline]
unsafe fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe fn(*mut Sprite)>) -> u8 {
    unsafe { crate::util::CreateInvisibleSpriteWithCallback(core::mem::transmute(a0)) }
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
    }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `ScanlineEffect_SetParams` with this module's view of its types.
#[inline]
unsafe fn ScanlineEffect_SetParams(a0: ScanlineEffectParams) {
    unsafe {
        crate::scanline_effect::ScanlineEffect_SetParams(core::mem::transmute(a0));
    }
}
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
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
/// `UpdateRoamerHPStatus` with this module's view of its types.
#[inline]
unsafe fn UpdateRoamerHPStatus(a0: *mut Pokemon) {
    unsafe {
        crate::roamer::UpdateRoamerHPStatus(a0 as _);
    }
}
// The C's names for task and sprite data slots.
const sBattler: usize = 0;
const sSinIndex: usize = 0;
const sState: usize = 0;
const sDelta: usize = 1;
const sSpeedX: usize = 1;
const sAmplitude: usize = 2;
const sSpeciesId: usize = 2;
const sSpeedY: usize = 2;
const sBouncerSpriteId: usize = 3;
const sNumFlickers: usize = 3;
const sDelay: usize = 4;
const sWhich: usize = 4;
// Data tables (translate with cdata.py): sIntroScanlineParams16Bit sIntroScanlineParams32Bit gUnusedBattleInitSprite sText_ShedinjaJpnName gOamData_BattleSpriteOpponentSide gOamData_BattleSpritePlayerSide sAnim_Unused sAnims_Unused sAffineAnim_Unused sAffineAnims_Unused sCenterToCornerVecXs gTypeEffectiveness gTypeNames gTrainerMoneyTable sNoneDescription sStenchDescription sDrizzleDescription sSpeedBoostDescription sBattleArmorDescription sSturdyDescription sDampDescription sLimberDescription sSandVeilDescription sStaticDescription sVoltAbsorbDescription sWaterAbsorbDescription sObliviousDescription sCloudNineDescription sCompoundEyesDescription sInsomniaDescription sColorChangeDescription sImmunityDescription sFlashFireDescription sShieldDustDescription sOwnTempoDescription sSuctionCupsDescription sIntimidateDescription sShadowTagDescription sRoughSkinDescription sWonderGuardDescription sLevitateDescription sEffectSporeDescription sSynchronizeDescription sClearBodyDescription sNaturalCureDescription sLightningRodDescription sSereneGraceDescription sSwiftSwimDescription sChlorophyllDescription sIlluminateDescription sTraceDescription sHugePowerDescription sPoisonPointDescription sInnerFocusDescription sMagmaArmorDescription sWaterVeilDescription sMagnetPullDescription sSoundproofDescription sRainDishDescription sSandStreamDescription sPressureDescription sThickFatDescription sEarlyBirdDescription sFlameBodyDescription sRunAwayDescription sKeenEyeDescription sHyperCutterDescription sPickupDescription sTruantDescription sHustleDescription sCuteCharmDescription sPlusDescription sMinusDescription sForecastDescription sStickyHoldDescription sShedSkinDescription sGutsDescription sMarvelScaleDescription sLiquidOozeDescription sOvergrowDescription sBlazeDescription sTorrentDescription sSwarmDescription sRockHeadDescription sDroughtDescription sArenaTrapDescription sVitalSpiritDescription sWhiteSmokeDescription sPurePowerDescription sShellArmorDescription sCacophonyDescription sAirLockDescription gAbilityNames gAbilityDescriptionPointers sTurnActionsFuncsTable sEndTurnFuncsTable gStatusConditionString_PoisonJpn gStatusConditionString_SleepJpn gStatusConditionString_ParalysisJpn gStatusConditionString_BurnJpn gStatusConditionString_IceJpn gStatusConditionString_ConfusionJpn gStatusConditionString_LoveJpn gStatusConditionStringsTable

const STATE_ASK_RECORD: u8 = 3;
const STATE_BEFORE_ACTION_CHOSEN: u8 = 1;
const STATE_END: u8 = 9;
const STATE_END_RECORD_NO: u8 = 7;
const STATE_END_RECORD_YES: u8 = 12;
const STATE_HANDLE_YES_NO: u8 = 5;
const STATE_INIT: u8 = 0;
const STATE_LINK: u8 = 1;
const STATE_PRINT_YES_NO: u8 = 4;
const STATE_RECORD_NO: u8 = 6;
const STATE_RECORD_WAIT: u8 = 11;
const STATE_RECORD_YES: u8 = 10;
const STATE_SELECTION_SCRIPT: u8 = 6;
const STATE_SELECTION_SCRIPT_MAY_RUN: u8 = 8;
const STATE_TURN_START_RECORD: u8 = 0;
const STATE_WAIT_ACTION_CASE_CHOSEN: u8 = 3;
const STATE_WAIT_ACTION_CHOSEN: u8 = 2;
const STATE_WAIT_ACTION_CONFIRMED: u8 = 5;
const STATE_WAIT_ACTION_CONFIRMED_STANDBY: u8 = 4;
const STATE_WAIT_END: u8 = 8;
const STATE_WAIT_LINK: u8 = 2;
const STATE_WAIT_SET_BEFORE_ACTION: u8 = 7;

static sCenterToCornerVecXs: Table<CArray<i8, 8>> =
    Table((&raw const crate::data::battle_main::sCenterToCornerVecXs).cast());
static sEndTurnFuncsTable: Table<CArray<Option<unsafe fn()>, 11>> =
    Table((&raw const crate::data::battle_main::sEndTurnFuncsTable).cast());
static sIntroScanlineParams16Bit: Table<ScanlineEffectParams> =
    Table((&raw const crate::data::battle_main::sIntroScanlineParams16Bit).cast());
static sText_ShedinjaJpnName: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::battle_main::sText_ShedinjaJpnName).cast());
static sTurnActionsFuncsTable: Table<CArray<Option<unsafe fn()>, 14>> =
    Table((&raw const crate::data::battle_main::sTurnActionsFuncsTable).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG0_X: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG0_Y: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG1_X: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG1_Y: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG2_X: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG2_Y: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG3_X: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG3_Y: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_WIN0H: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_WIN0V: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_WIN1H: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_WIN1V: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gDisplayedStringBattle: Aligned<CArray<u8, 300>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleTextBuff1: Aligned<CArray<u8, 16>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleTextBuff2: Aligned<CArray<u8, 16>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleTextBuff3: Aligned<CArray<u8, 16>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFlickerArray: CArray<u32, 25> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleTypeFlags: u32 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleEnvironment: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gUnusedFirstBattleVar1: u32 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gMultiPartnerParty: CArray<MultiPartnerMenuPokemon, 3> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMultiPartnerPartyBuffer: *mut MultiPartnerMenuPokemon = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimBgTileBuffer: *mut u8 = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimBgTilemapBuffer: *mut u8 = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleBufferA: Aligned<CArray<CArray<u8, 512>, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleBufferB: Aligned<CArray<CArray<u8, 512>, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gActiveBattler: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleControllerExecFlags: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlersCount: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerPartyIndexes: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerPositions: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gActionsByTurnOrder: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerByTurnOrder: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub static mut gCurrentTurnActionNumber: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gCurrentActionFuncId: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleMons: CArray<BattlePokemon, 4> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerSpriteIds: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub static mut gCurrMovePos: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gChosenMovePos: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurrentMove: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gChosenMove: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gCalledMove: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleMoveDamage: i32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gHpDealt: i32 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gBideDmg: CArray<i32, 4> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastUsedItem: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gLastUsedAbility: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerAttacker: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerTarget: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerFainted: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gEffectBattler: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gPotentialItemEffectBattler: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gAbsentBattlerFlags: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCritMultiplier: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gMultiHitCounter: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlescriptCurrInstr: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub static gUnusedBattleMainVar: crate::global::Global<u32> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gChosenActionByBattler: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub static mut gSelectionBattleScripts: CArray<*mut u8, 4> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub static mut gPalaceSelectionBattleScripts: CArray<*mut u8, 4> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastPrintedMoves: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastMoves: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastLandedMoves: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastHitByType: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastResultingMoves: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLockedMoves: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastHitBy: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gChosenMoveByBattler: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMoveResultFlags: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gHitMarker: u32 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnusedBattlersArray: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBideTarget: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub static mut gUnusedFirstBattleVar2: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSideStatuses: Aligned<CArray<u16, 2>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub static mut gSideTimers: CArray<SideTimer, 2> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub static mut gStatuses3: CArray<u32, 4> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub static mut gDisableStructs: CArray<DisableStruct, 4> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub static mut gPauseCounterBattle: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gPaydayMoney: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static gRandomTurnNumber: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleCommunication: Aligned<CArray<u8, 8>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleOutcome: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gProtectStructs: CArray<ProtectStruct, 4> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub static mut gSpecialStatuses: CArray<SpecialStatus, 4> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleWeather: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gWishFutureKnock: WishFutureKnock = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub static mut gIntroSlideFlags: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSentPokesToOpponent: Aligned<CArray<u8, 2>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gDynamicBasePower: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gExpShareExp: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gEnigmaBerries: CArray<BattleEnigmaBerry, 4> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleScripting: BattleScripting = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleStruct: *mut BattleStruct = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkBattleSendBuffer: *mut u8 = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkBattleRecvBuffer: *mut u8 = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleResources: *mut BattleResources = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gActionSelectionCursor: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMoveSelectionCursor: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerStatusSummaryTaskId: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerInMenuId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gDoingBattleAnim: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gTransformedPersonalities: CArray<u32, 4> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerDpadHoldFrames: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleSpritesDataPtr: *mut BattleSpriteData = null_mut();
#[unsafe(link_section = "ewram_data")]
pub static mut gMonSpritesGfxPtr: *mut MonSpritesGfx = null_mut();
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleControllerOpponentHealthboxData: *mut BattleHealthboxInfo = null_mut();
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleControllerOpponentFlankHealthboxData: *mut BattleHealthboxInfo = null_mut();
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleMovePower: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gMoveToLearn: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleMonForms: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "common_data")]
pub static mut gPreBattleCallback1: Option<unsafe fn()> = None;
#[unsafe(link_section = "common_data")]
pub static mut gBattleMainFunc: Option<unsafe fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBattleResults: BattleResults = unsafe { zeroed() };
#[unsafe(link_section = "common_data")]
pub static mut gLeveledUpInBattle: u8 = 0;
#[unsafe(link_section = "common_data")]
pub static mut gBattlerControllerFuncs: CArray<Option<unsafe fn()>, 4> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gHealthboxSpriteIds: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "common_data")]
pub static mut gMultiUsePlayerCursor: u8 = 0;
#[unsafe(link_section = "common_data")]
pub static mut gNumberOfMovesToChoose: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBattleControllerData: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });

/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
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

#[unsafe(no_mangle)]
pub unsafe fn CB2_InitBattle() {
    MoveSaveBlocks_ResetHeap();
    AllocateBattleResources();
    AllocateBattleSpritesData();
    AllocateMonSpritesGfx();
    RecordedBattle_ClearFrontierPassFlag();
    if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            CB2_InitBattleInternal();
        } else if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER == 0 {
            HandleLinkBattleSetup();
            SetMainCallback2(Some(CB2_PreInitMultiBattle));
        } else {
            SetMainCallback2(Some(CB2_PreInitIngamePlayerPartnerBattle));
        }
        gBattleCommunication[0] = 0;
    } else {
        CB2_InitBattleInternal();
    }
}
pub(crate) unsafe fn CB2_InitBattleInternal() {
    let mut i: i32 = 0;
    SetHBlankCallback(None);
    SetVBlankCallback(None);
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
    SetGpuReg(REG_OFFSET_MOSAIC, 0);
    SetGpuReg(REG_OFFSET_WIN0H, DISPLAY_WIDTH);
    SetGpuReg(REG_OFFSET_WIN0V, 20561);
    SetGpuReg(REG_OFFSET_WININ, 0);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    gBattle_WIN0H = DISPLAY_WIDTH;
    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0
        && gPartnerTrainerId != TRAINER_STEVEN_PARTNER
    {
        gBattle_WIN0V = 159;
        gBattle_WIN1H = DISPLAY_WIDTH;
        gBattle_WIN1V = 32;
    } else {
        gBattle_WIN0V = 20561;
        ScanlineEffect_Clear();
        i = 0;
        while i < 80 {
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[0][i] = 0xF0;
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[1][i] = 0xF0;
            i += 1;
        }
        while i < DISPLAY_HEIGHT as i32 {
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[0][i] = 0xFF10;
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[1][i] = 0xFF10;
            i += 1;
        }
        ScanlineEffect_SetParams(*sIntroScanlineParams16Bit);
    }
    ResetPaletteFade();
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    gBattle_BG2_Y = 0;
    gBattle_BG3_X = 0;
    gBattle_BG3_Y = 0;
    gBattleEnvironment = BattleSetup_GetEnvironmentId();
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
        gBattleEnvironment = BATTLE_ENVIRONMENT_BUILDING;
    }
    InitBattleBgsVideo();
    LoadBattleTextboxAndBackground();
    ResetSpriteData();
    ResetTasks();
    DrawBattleEntryBackground();
    FreeAllSpritePalettes();
    gReservedSpritePaletteCount = MAX_BATTLERS_COUNT;
    SetVBlankCallback(Some(VBlankCB_Battle));
    SetUpBattleVarsAndBirchZigzagoon();
    if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 && gBattleTypeFlags & BATTLE_TYPE_BATTLE_TOWER != 0
    {
        SetMainCallback2(Some(CB2_HandleStartMultiPartnerBattle));
    } else if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
        && gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0
    {
        SetMainCallback2(Some(CB2_HandleStartMultiPartnerBattle));
    } else if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        SetMainCallback2(Some(CB2_HandleStartMultiBattle));
    } else {
        SetMainCallback2(Some(CB2_HandleStartBattle));
    }
    if gBattleTypeFlags & 0x1000002 == 0 {
        CreateNPCTrainerParty(&raw mut gEnemyParty[0], gTrainerBattleOpponent_A, TRUE);
        if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
            CreateNPCTrainerParty(&raw mut gEnemyParty[3], gTrainerBattleOpponent_B, FALSE);
        }
        SetWildMonHeldItem();
    }
    gMain.set_inBattle(TRUE);
    (*gSaveBlock2Ptr).frontier.set_disableRecordBattle(FALSE);
    for i in 0..PARTY_SIZE {
        AdjustFriendship(&raw mut gPlayerParty[i], FRIENDSHIP_EVENT_LEAGUE_BATTLE);
    }
    gBattleCommunication[0] = 0;
}
unsafe fn BufferPartyVsScreenHealth_AtStart() {
    let mut flags: u16 = 0;
    for i in 0..6i32 {
        'l1: {
            let species: u16 =
                GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) as u16;
            let hp: u16 = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) as u16;
            let status: u32 = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_STATUS);
            if species == 0 {
                break 'l1;
            }
            if species != 412 && hp != 0 && status == 0 {
                flags |= shl_i32(1, i as u32 * 2) as u16;
            }
            if species == 0 {
                break 'l1;
            }
            if hp != 0 && (species == 412 || status != 0) {
                flags |= shl_i32(2, i as u32 * 2) as u16;
            }
            if species == 0 {
                break 'l1;
            }
            if species != 412 && hp == 0 {
                flags |= shl_i32(3, i as u32 * 2) as u16;
            }
        }
    }
    (*gBattleStruct)
        .multiBuffer
        .linkBattlerHeader
        .vsScreenHealthFlagsLo = flags as u8;
    (*gBattleStruct)
        .multiBuffer
        .linkBattlerHeader
        .vsScreenHealthFlagsHi = (flags >> 8) as u8;
    (*gBattleStruct)
        .multiBuffer
        .linkBattlerHeader
        .vsScreenHealthFlagsHi |= FlagGet(FLAG_SYS_FRONTIER_PASS) << 7;
}
unsafe fn SetPlayerBerryDataInBattleStruct() {
    let mut i: i32 = 0;
    let battleStruct: *mut BattleStruct = gBattleStruct;
    let battleBerry: *mut BattleEnigmaBerry = &raw mut (*battleStruct)
        .multiBuffer
        .linkBattlerHeader
        .battleEnigmaBerry;
    if IsEnigmaBerryValid() == TRUE as u32 {
        i = 0;
        while i < BERRY_NAME_LENGTH {
            (*battleBerry).name[i] = (*gSaveBlock1Ptr).enigmaBerry.berry.name[i];
            i += 1;
        }
        (*battleBerry).name[i] = EOS;
        for i in 0..BERRY_ITEM_EFFECT_COUNT {
            (*battleBerry).itemEffect[i] = (*gSaveBlock1Ptr).enigmaBerry.itemEffect[i];
        }
        (*battleBerry).holdEffect = (*gSaveBlock1Ptr).enigmaBerry.holdEffect;
        (*battleBerry).holdEffectParam = (*gSaveBlock1Ptr).enigmaBerry.holdEffectParam;
    } else {
        let berryData: *mut Berry = GetBerryInfo(ItemIdToBerryType(ITEM_ENIGMA_BERRY));
        i = 0;
        while i < BERRY_NAME_LENGTH {
            (*battleBerry).name[i] = (*berryData).name[i];
            i += 1;
        }
        (*battleBerry).name[i] = EOS;
        for i in 0..BERRY_ITEM_EFFECT_COUNT {
            (*battleBerry).itemEffect[i] = 0;
        }
        (*battleBerry).holdEffect = HOLD_EFFECT_NONE;
        (*battleBerry).holdEffectParam = 0;
    }
}
unsafe fn SetAllPlayersBerryData() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_LINK == 0 {
        if IsEnigmaBerryValid() == TRUE as u32 {
            i = 0;
            while i < BERRY_NAME_LENGTH {
                gEnigmaBerries[0].name[i] = (*gSaveBlock1Ptr).enigmaBerry.berry.name[i];
                gEnigmaBerries[2].name[i] = (*gSaveBlock1Ptr).enigmaBerry.berry.name[i];
                i += 1;
            }
            gEnigmaBerries[0].name[i] = EOS;
            gEnigmaBerries[2].name[i] = EOS;
            for i in 0..BERRY_ITEM_EFFECT_COUNT {
                gEnigmaBerries[0].itemEffect[i] = (*gSaveBlock1Ptr).enigmaBerry.itemEffect[i];
                gEnigmaBerries[2].itemEffect[i] = (*gSaveBlock1Ptr).enigmaBerry.itemEffect[i];
            }
            gEnigmaBerries[0].holdEffect = (*gSaveBlock1Ptr).enigmaBerry.holdEffect;
            gEnigmaBerries[2].holdEffect = (*gSaveBlock1Ptr).enigmaBerry.holdEffect;
            gEnigmaBerries[0].holdEffectParam = (*gSaveBlock1Ptr).enigmaBerry.holdEffectParam;
            gEnigmaBerries[2].holdEffectParam = (*gSaveBlock1Ptr).enigmaBerry.holdEffectParam;
        } else {
            let berryData: *mut Berry = GetBerryInfo(ItemIdToBerryType(ITEM_ENIGMA_BERRY));
            i = 0;
            while i < BERRY_NAME_LENGTH {
                gEnigmaBerries[0].name[i] = (*berryData).name[i];
                gEnigmaBerries[2].name[i] = (*berryData).name[i];
                i += 1;
            }
            gEnigmaBerries[0].name[i] = EOS;
            gEnigmaBerries[2].name[i] = EOS;
            for i in 0..BERRY_ITEM_EFFECT_COUNT {
                gEnigmaBerries[0].itemEffect[i] = 0;
                gEnigmaBerries[2].itemEffect[i] = 0;
            }
            gEnigmaBerries[0].holdEffect = 0;
            gEnigmaBerries[2].holdEffect = HOLD_EFFECT_NONE;
            gEnigmaBerries[0].holdEffectParam = 0;
            gEnigmaBerries[2].holdEffectParam = 0;
        }
    } else {
        let mut numPlayers: i32 = 0;
        let mut src: *mut BattleEnigmaBerry = null_mut();
        let mut battler: u8 = 0;
        if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
            if gBattleTypeFlags & BATTLE_TYPE_BATTLE_TOWER != 0 {
                numPlayers = 2;
            } else {
                numPlayers = 4;
            }
            for i in 0..numPlayers {
                src = gBlockRecvBuffer[i].as_mut_ptr().at(2) as *mut BattleEnigmaBerry;
                battler = gLinkPlayers[i].id as u8;
                j = 0;
                while j < BERRY_NAME_LENGTH {
                    gEnigmaBerries[battler].name[j] = (*src).name[j];
                    j += 1;
                }
                gEnigmaBerries[battler].name[j] = EOS;
                for j in 0..BERRY_ITEM_EFFECT_COUNT {
                    gEnigmaBerries[battler].itemEffect[j] = (*src).itemEffect[j];
                }
                gEnigmaBerries[battler].holdEffect = (*src).holdEffect;
                gEnigmaBerries[battler].holdEffectParam = (*src).holdEffectParam;
            }
        } else {
            for i in 0..2i32 {
                src = gBlockRecvBuffer[i].as_mut_ptr().at(2) as *mut BattleEnigmaBerry;
                j = 0;
                while j < BERRY_NAME_LENGTH {
                    gEnigmaBerries[i].name[j] = (*src).name[j];
                    gEnigmaBerries[i + 2].name[j] = (*src).name[j];
                    j += 1;
                }
                gEnigmaBerries[i].name[j] = EOS;
                gEnigmaBerries[i + 2].name[j] = EOS;
                for j in 0..BERRY_ITEM_EFFECT_COUNT {
                    gEnigmaBerries[i].itemEffect[j] = (*src).itemEffect[j];
                    gEnigmaBerries[i + 2].itemEffect[j] = (*src).itemEffect[j];
                }
                gEnigmaBerries[i].holdEffect = (*src).holdEffect;
                gEnigmaBerries[i + 2].holdEffect = (*src).holdEffect;
                gEnigmaBerries[i].holdEffectParam = (*src).holdEffectParam;
                gEnigmaBerries[i + 2].holdEffectParam = (*src).holdEffectParam;
            }
        }
    }
}
unsafe fn FindLinkBattleMaster(numPlayers: u8, multiPlayerId: u8) {
    let mut found: u8 = 0;
    if gBlockRecvBuffer[0][0] == 0x100 {
        if multiPlayerId == 0 {
            gBattleTypeFlags |= 12;
        } else {
            gBattleTypeFlags |= BATTLE_TYPE_TRAINER;
        }
        found += 1;
    }
    if found == 0 {
        let mut i: i32 = 0;
        while i < numPlayers as i32 {
            if gBlockRecvBuffer[0][0] != gBlockRecvBuffer[i][0] {
                break;
            }
            i += 1;
        }
        if i == numPlayers as i32 {
            if multiPlayerId == 0 {
                gBattleTypeFlags |= 12;
            } else {
                gBattleTypeFlags |= BATTLE_TYPE_TRAINER;
            }
            found += 1;
        }
        if found == 0 {
            i = 0;
            while i < numPlayers as i32 {
                if gBlockRecvBuffer[i][0] == 0x300
                    && i != multiPlayerId as i32
                    && i < multiPlayerId as i32
                {
                    break;
                }
                if gBlockRecvBuffer[i][0] > 0x300 && i != multiPlayerId as i32 {
                    break;
                }
                i += 1;
            }
            if i == numPlayers as i32 {
                gBattleTypeFlags |= 12;
            } else {
                gBattleTypeFlags |= BATTLE_TYPE_TRAINER;
            }
        }
    }
}
pub(crate) unsafe fn CB2_HandleStartBattle() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    let playerMultiplayerId: u8 = GetMultiplayerId();
    gBattleScripting.multiplayerId = playerMultiplayerId;
    let enemyMultiplayerId: u8 = playerMultiplayerId ^ BIT_SIDE;
    'l1: {
        let sw1: u8 = gBattleCommunication[0];
        let mut fall = false;
        if sw1 == 0 {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                ShowBg(0);
                ShowBg(1);
                ShowBg(2);
                ShowBg(3);
                FillAroundBattleWindows();
                gBattleCommunication[0] = 1;
            }
            if gWirelessCommType != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
            }
            break 'l1;
        }
        if sw1 == 1 {
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                if gReceivedRemoteLinkPlayers != 0 {
                    if IsLinkTaskFinished() != 0 {
                        (*gBattleStruct)
                            .multiBuffer
                            .linkBattlerHeader
                            .versionSignatureLo = 0;
                        (*gBattleStruct)
                            .multiBuffer
                            .linkBattlerHeader
                            .versionSignatureHi = 3;
                        BufferPartyVsScreenHealth_AtStart();
                        SetPlayerBerryDataInBattleStruct();
                        if gTrainerBattleOpponent_A == TRAINER_UNION_ROOM {
                            gLinkPlayers[0].id = 0;
                            gLinkPlayers[1].id = 1;
                        }
                        SendBlock(
                            BitmaskAllOtherLinkPlayers(),
                            &raw mut (*gBattleStruct).multiBuffer.linkBattlerHeader as *mut c_void,
                            32,
                        );
                        gBattleCommunication[0] = 2;
                    }
                    if gWirelessCommType != 0 {
                        CreateWirelessStatusIndicatorSprite(0, 0);
                    }
                }
            } else {
                if gBattleTypeFlags & BATTLE_TYPE_RECORDED == 0 {
                    gBattleTypeFlags |= BATTLE_TYPE_IS_MASTER;
                }
                gBattleCommunication[0] = 15;
                SetAllPlayersBerryData();
            }
            break 'l1;
        }
        if sw1 == 2 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                FindLinkBattleMaster(2, playerMultiplayerId);
                SetAllPlayersBerryData();
                let taskId: u8 = CreateTask(Some(InitLinkBattleVsScreen), 0);
                task_set(taskId, 1, 0x10E);
                task_set(taskId, 2, 0x5A);
                task_set(taskId, 5, 0);
                task_set(
                    taskId,
                    3,
                    (*gBattleStruct)
                        .multiBuffer
                        .linkBattlerHeader
                        .vsScreenHealthFlagsLo as i16
                        | ((*gBattleStruct)
                            .multiBuffer
                            .linkBattlerHeader
                            .vsScreenHealthFlagsHi as i16)
                            << 8,
                );
                task_set(taskId, 4, gBlockRecvBuffer[enemyMultiplayerId][1] as i16);
                RecordedBattle_SetFrontierPassFlagFromHword(
                    gBlockRecvBuffer[playerMultiplayerId][1],
                );
                RecordedBattle_SetFrontierPassFlagFromHword(
                    gBlockRecvBuffer[enemyMultiplayerId][1],
                );
                SetDeoxysStats();
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    gPlayerParty.as_mut_ptr() as *mut c_void,
                    200,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                memcpy(
                    gEnemyParty.as_mut_ptr() as *mut u8,
                    gBlockRecvBuffer[enemyMultiplayerId].as_mut_ptr() as *mut u8,
                    200,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 7 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    &raw mut gPlayerParty[2] as *mut c_void,
                    200,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 8 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                memcpy(
                    &raw mut gEnemyParty[2] as *mut u8,
                    gBlockRecvBuffer[enemyMultiplayerId].as_mut_ptr() as *mut u8,
                    200,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 11 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    &raw mut gPlayerParty[4] as *mut c_void,
                    200,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 12 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                memcpy(
                    &raw mut gEnemyParty[4] as *mut u8,
                    gBlockRecvBuffer[enemyMultiplayerId].as_mut_ptr() as *mut u8,
                    200,
                );
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[0]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[1]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[2]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[3]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[4]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[5]);
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 15 {
            InitBattleControllers();
            RecordedBattle_SetTrainerInfo();
            gBattleCommunication[1] = 0;
            gBattleCommunication[2] = 0;
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                let mut i: i32 = 0;
                while i < 2 && gLinkPlayers[i].version as i32 & 0xFF == VERSION_EMERALD as i32 {
                    i += 1;
                }
                if i == 2 {
                    gBattleCommunication[0] = 16;
                } else {
                    gBattleCommunication[0] = 18;
                }
            } else {
                gBattleCommunication[0] = 18;
            }
            break 'l1;
        }
        if sw1 == 16 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    &raw mut gRecordedBattleRngSeed as *mut c_void,
                    4,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 17 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if gBattleTypeFlags & BATTLE_TYPE_IS_MASTER == 0 {
                    memcpy(
                        &raw mut gRecordedBattleRngSeed as *mut u8,
                        gBlockRecvBuffer[enemyMultiplayerId].as_mut_ptr() as *mut u8,
                        4,
                    );
                }
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 18 {
            if BattleInitAllSprites(
                &raw mut gBattleCommunication[1],
                &raw mut gBattleCommunication[2],
            ) != 0
            {
                gPreBattleCallback1 = gMain.callback1;
                gMain.callback1 = Some(BattleMainCB1);
                SetMainCallback2(Some(BattleMainCB2));
                if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                    gBattleTypeFlags |= BATTLE_TYPE_LINK_IN_BATTLE;
                }
            }
            break 'l1;
        }
        if sw1 == 5 || sw1 == 9 || sw1 == 13 {
            fall = true;
            gBattleCommunication[0] += 1;
            gBattleCommunication[1] = 1;
        }
        if fall || sw1 == 6 || sw1 == 10 || sw1 == 14 {
            if ({
                gBattleCommunication[1] -= 1;
                gBattleCommunication[1]
            }) == 0
            {
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe fn CB2_HandleStartMultiPartnerBattle() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    let playerMultiplayerId: u8 = GetMultiplayerId();
    gBattleScripting.multiplayerId = playerMultiplayerId;
    let partnerMultiplayerId: u8 = playerMultiplayerId ^ BIT_SIDE;
    'l1: {
        let sw1: u8 = gBattleCommunication[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                ShowBg(0);
                ShowBg(1);
                ShowBg(2);
                ShowBg(3);
                FillAroundBattleWindows();
                gBattleCommunication[0] = 1;
            }
            if gWirelessCommType != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
            }
        }
        if fall || sw1 == 1 {
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                if gReceivedRemoteLinkPlayers != 0 {
                    let mut language: u8 = 0;
                    gLinkPlayers[0].id = 0;
                    gLinkPlayers[1].id = 2;
                    gLinkPlayers[2].id = 1;
                    gLinkPlayers[3].id = 3;
                    GetFrontierTrainerName(
                        gLinkPlayers[2].name.as_mut_ptr(),
                        gTrainerBattleOpponent_A,
                    );
                    GetFrontierTrainerName(
                        gLinkPlayers[3].name.as_mut_ptr(),
                        gTrainerBattleOpponent_B,
                    );
                    GetBattleTowerTrainerLanguage(&raw mut language, gTrainerBattleOpponent_A);
                    gLinkPlayers[2].language = language as u16;
                    GetBattleTowerTrainerLanguage(&raw mut language, gTrainerBattleOpponent_B);
                    gLinkPlayers[3].language = language as u16;
                    if IsLinkTaskFinished() != 0 {
                        (*gBattleStruct)
                            .multiBuffer
                            .linkBattlerHeader
                            .versionSignatureLo = 0;
                        (*gBattleStruct)
                            .multiBuffer
                            .linkBattlerHeader
                            .versionSignatureHi = 3;
                        BufferPartyVsScreenHealth_AtStart();
                        SetPlayerBerryDataInBattleStruct();
                        SendBlock(
                            BitmaskAllOtherLinkPlayers(),
                            &raw mut (*gBattleStruct).multiBuffer.linkBattlerHeader as *mut c_void,
                            32,
                        );
                        gBattleCommunication[0] = 2;
                    }
                    if gWirelessCommType != 0 {
                        CreateWirelessStatusIndicatorSprite(0, 0);
                    }
                }
            } else {
                if gBattleTypeFlags & BATTLE_TYPE_RECORDED == 0 {
                    gBattleTypeFlags |= BATTLE_TYPE_IS_MASTER;
                }
                gBattleCommunication[0] = 13;
                SetAllPlayersBerryData();
            }
            break 'l1;
        }
        if sw1 == 2 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                FindLinkBattleMaster(2, playerMultiplayerId);
                SetAllPlayersBerryData();
                let taskId: u8 = CreateTask(Some(InitLinkBattleVsScreen), 0);
                task_set(taskId, 1, 0x10E);
                task_set(taskId, 2, 0x5A);
                task_set(taskId, 5, 0);
                task_set(taskId, 3, 0x145);
                task_set(taskId, 4, 0x145);
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    gPlayerParty.as_mut_ptr() as *mut c_void,
                    200,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if gLinkPlayers[playerMultiplayerId].id != 0 {
                    memcpy(
                        gPlayerParty.as_mut_ptr() as *mut u8,
                        gBlockRecvBuffer[partnerMultiplayerId].as_mut_ptr() as *mut u8,
                        200,
                    );
                    memcpy(
                        &raw mut gPlayerParty[3] as *mut u8,
                        gBlockRecvBuffer[playerMultiplayerId].as_mut_ptr() as *mut u8,
                        200,
                    );
                } else {
                    memcpy(
                        gPlayerParty.as_mut_ptr() as *mut u8,
                        gBlockRecvBuffer[playerMultiplayerId].as_mut_ptr() as *mut u8,
                        200,
                    );
                    memcpy(
                        &raw mut gPlayerParty[3] as *mut u8,
                        gBlockRecvBuffer[partnerMultiplayerId].as_mut_ptr() as *mut u8,
                        200,
                    );
                }
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 5 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    &raw mut gPlayerParty[2] as *mut c_void,
                    100,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 6 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if gLinkPlayers[playerMultiplayerId].id != 0 {
                    memcpy(
                        &raw mut gPlayerParty[2] as *mut u8,
                        gBlockRecvBuffer[partnerMultiplayerId].as_mut_ptr() as *mut u8,
                        100,
                    );
                    memcpy(
                        &raw mut gPlayerParty[5] as *mut u8,
                        gBlockRecvBuffer[playerMultiplayerId].as_mut_ptr() as *mut u8,
                        100,
                    );
                } else {
                    memcpy(
                        &raw mut gPlayerParty[2] as *mut u8,
                        gBlockRecvBuffer[playerMultiplayerId].as_mut_ptr() as *mut u8,
                        100,
                    );
                    memcpy(
                        &raw mut gPlayerParty[5] as *mut u8,
                        gBlockRecvBuffer[partnerMultiplayerId].as_mut_ptr() as *mut u8,
                        100,
                    );
                }
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 7 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    gEnemyParty.as_mut_ptr() as *mut c_void,
                    200,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 8 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if GetMultiplayerId() != 0 {
                    memcpy(
                        gEnemyParty.as_mut_ptr() as *mut u8,
                        gBlockRecvBuffer[0].as_mut_ptr() as *mut u8,
                        200,
                    );
                }
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 9 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    &raw mut gEnemyParty[2] as *mut c_void,
                    200,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 10 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if GetMultiplayerId() != 0 {
                    memcpy(
                        &raw mut gEnemyParty[2] as *mut u8,
                        gBlockRecvBuffer[0].as_mut_ptr() as *mut u8,
                        200,
                    );
                }
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 11 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    &raw mut gEnemyParty[4] as *mut c_void,
                    200,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 12 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if GetMultiplayerId() != 0 {
                    memcpy(
                        &raw mut gEnemyParty[4] as *mut u8,
                        gBlockRecvBuffer[0].as_mut_ptr() as *mut u8,
                        200,
                    );
                }
                TryCorrectShedinjaLanguage(&raw mut gPlayerParty[0]);
                TryCorrectShedinjaLanguage(&raw mut gPlayerParty[1]);
                TryCorrectShedinjaLanguage(&raw mut gPlayerParty[2]);
                TryCorrectShedinjaLanguage(&raw mut gPlayerParty[3]);
                TryCorrectShedinjaLanguage(&raw mut gPlayerParty[4]);
                TryCorrectShedinjaLanguage(&raw mut gPlayerParty[5]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[0]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[1]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[2]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[3]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[4]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[5]);
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 13 {
            InitBattleControllers();
            RecordedBattle_SetTrainerInfo();
            gBattleCommunication[1] = 0;
            gBattleCommunication[2] = 0;
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                gBattleCommunication[0] = 14;
            } else {
                gBattleCommunication[0] = 16;
            }
            break 'l1;
        }
        if sw1 == 14 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    &raw mut gRecordedBattleRngSeed as *mut c_void,
                    4,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 15 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if gBattleTypeFlags & BATTLE_TYPE_IS_MASTER == 0 {
                    memcpy(
                        &raw mut gRecordedBattleRngSeed as *mut u8,
                        gBlockRecvBuffer[partnerMultiplayerId].as_mut_ptr() as *mut u8,
                        4,
                    );
                }
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 16 {
            if BattleInitAllSprites(
                &raw mut gBattleCommunication[1],
                &raw mut gBattleCommunication[2],
            ) != 0
            {
                TrySetLinkBattleTowerEnemyPartyLevel();
                gPreBattleCallback1 = gMain.callback1;
                gMain.callback1 = Some(BattleMainCB1);
                SetMainCallback2(Some(BattleMainCB2));
                if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                    gBattleTypeFlags |= BATTLE_TYPE_LINK_IN_BATTLE;
                }
            }
            break 'l1;
        }
    }
}
unsafe fn SetMultiPartnerMenuParty(offset: u8) {
    for i in 0..MULTI_PARTY_SIZE {
        gMultiPartnerParty[i].species =
            GetMonData2(&raw mut gPlayerParty[offset as i32 + i], MON_DATA_SPECIES) as u16;
        gMultiPartnerParty[i].heldItem =
            GetMonData2(&raw mut gPlayerParty[offset as i32 + i], MON_DATA_HELD_ITEM) as u16;
        GetMonData3(
            &raw mut gPlayerParty[offset as i32 + i],
            MON_DATA_NICKNAME,
            gMultiPartnerParty[i].nickname.as_mut_ptr(),
        );
        gMultiPartnerParty[i].level =
            GetMonData2(&raw mut gPlayerParty[offset as i32 + i], MON_DATA_LEVEL) as u8;
        gMultiPartnerParty[i].hp =
            GetMonData2(&raw mut gPlayerParty[offset as i32 + i], MON_DATA_HP) as u16;
        gMultiPartnerParty[i].maxhp =
            GetMonData2(&raw mut gPlayerParty[offset as i32 + i], MON_DATA_MAX_HP) as u16;
        gMultiPartnerParty[i].status =
            GetMonData2(&raw mut gPlayerParty[offset as i32 + i], MON_DATA_STATUS);
        gMultiPartnerParty[i].personality = GetMonData2(
            &raw mut gPlayerParty[offset as i32 + i],
            MON_DATA_PERSONALITY,
        );
        gMultiPartnerParty[i].gender = GetMonGender(&raw mut gPlayerParty[offset as i32 + i]);
        StripExtCtrlCodes(gMultiPartnerParty[i].nickname.as_mut_ptr());
        if GetMonData2(&raw mut gPlayerParty[offset as i32 + i], MON_DATA_LANGUAGE)
            != LANGUAGE_JAPANESE as u32
        {
            PadNameString(gMultiPartnerParty[i].nickname.as_mut_ptr(), CHAR_SPACE);
        }
    }
    memcpy(
        sMultiPartnerPartyBuffer as *mut u8,
        gMultiPartnerParty.as_mut_ptr() as *mut u8,
        96,
    );
}
pub(crate) unsafe fn CB2_PreInitMultiBattle() {
    let mut numPlayers: i32 = MAX_BATTLERS_COUNT as i32;
    let mut blockMask: u8 = 0xF;
    let mut savedBattleTypeFlags: *mut u32 = null_mut();
    let mut savedCallback: *mut Option<unsafe fn()> = null_mut();
    if gBattleTypeFlags & BATTLE_TYPE_BATTLE_TOWER != 0 {
        numPlayers = 2;
        blockMask = 3;
    }
    let playerMultiplierId: u8 = GetMultiplayerId();
    gBattleScripting.multiplayerId = playerMultiplierId;
    savedCallback = &raw mut (*gBattleStruct).savedCallback;
    savedBattleTypeFlags = &raw mut (*gBattleStruct).savedBattleTypeFlags;
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    match gBattleCommunication[0] {
        0 => {
            if gReceivedRemoteLinkPlayers != 0 && IsLinkTaskFinished() != 0 {
                sMultiPartnerPartyBuffer = Alloc(96) as *mut MultiPartnerMenuPokemon;
                SetMultiPartnerMenuParty(0);
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    sMultiPartnerPartyBuffer as *mut c_void,
                    96,
                );
                gBattleCommunication[0] += 1;
            }
        }
        1 => {
            if GetBlockReceivedStatus() as i32 & blockMask as i32 == blockMask as i32 {
                ResetBlockReceivedFlags();
                for i in 0..numPlayers {
                    'l2: {
                        if i == playerMultiplierId as i32 {
                            break 'l2;
                        }
                        if numPlayers == MAX_LINK_PLAYERS {
                            if gLinkPlayers[i].id as i32 & 1 == 0
                                && gLinkPlayers[playerMultiplierId].id as i32 & 1 == 0
                                || gLinkPlayers[i].id as i32 & 1 != 0
                                    && gLinkPlayers[playerMultiplierId].id as i32 & 1 != 0
                            {
                                memcpy(
                                    gMultiPartnerParty.as_mut_ptr() as *mut u8,
                                    gBlockRecvBuffer[i].as_mut_ptr() as *mut u8,
                                    96,
                                );
                            }
                        } else {
                            memcpy(
                                gMultiPartnerParty.as_mut_ptr() as *mut u8,
                                gBlockRecvBuffer[i].as_mut_ptr() as *mut u8,
                                96,
                            );
                        }
                    }
                }
                gBattleCommunication[0] += 1;
                *savedCallback = gMain.savedCallback;
                *savedBattleTypeFlags = gBattleTypeFlags;
                gMain.savedCallback = Some(CB2_PreInitMultiBattle);
                ShowPartyMenuToShowcaseMultiBattleParty();
            }
        }
        2 => {
            if IsLinkTaskFinished() != 0 && gPaletteFade.active() == 0 {
                gBattleCommunication[0] += 1;
                if gWirelessCommType != 0 {
                    SetLinkStandbyCallback();
                } else {
                    SetCloseLinkCallback();
                }
            }
        }
        3 => {
            if gWirelessCommType != 0 {
                if IsLinkRfuTaskFinished() != 0 {
                    gBattleTypeFlags = *savedBattleTypeFlags;
                    gMain.savedCallback = *savedCallback;
                    SetMainCallback2(Some(CB2_InitBattleInternal));
                    Free(sMultiPartnerPartyBuffer as *mut c_void);
                    sMultiPartnerPartyBuffer = null_mut();
                }
            } else if gReceivedRemoteLinkPlayers == 0 {
                gBattleTypeFlags = *savedBattleTypeFlags;
                gMain.savedCallback = *savedCallback;
                SetMainCallback2(Some(CB2_InitBattleInternal));
                Free(sMultiPartnerPartyBuffer as *mut c_void);
                sMultiPartnerPartyBuffer = null_mut();
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn CB2_PreInitIngamePlayerPartnerBattle() {
    let mut savedBattleTypeFlags: *mut u32 = null_mut();
    let mut savedCallback: *mut Option<unsafe fn()> = null_mut();
    savedCallback = &raw mut (*gBattleStruct).savedCallback;
    savedBattleTypeFlags = &raw mut (*gBattleStruct).savedBattleTypeFlags;
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    match gBattleCommunication[0] {
        0 => {
            sMultiPartnerPartyBuffer = Alloc(96) as *mut MultiPartnerMenuPokemon;
            SetMultiPartnerMenuParty(MULTI_PARTY_SIZE as u8);
            gBattleCommunication[0] += 1;
            *savedCallback = gMain.savedCallback;
            *savedBattleTypeFlags = gBattleTypeFlags;
            gMain.savedCallback = Some(CB2_PreInitIngamePlayerPartnerBattle);
            ShowPartyMenuToShowcaseMultiBattleParty();
        }
        1 if gPaletteFade.active() == 0 => {
            gBattleCommunication[0] = 2;
            gBattleTypeFlags = *savedBattleTypeFlags;
            gMain.savedCallback = *savedCallback;
            SetMainCallback2(Some(CB2_InitBattleInternal));
            Free(sMultiPartnerPartyBuffer as *mut c_void);
            sMultiPartnerPartyBuffer = null_mut();
        }
        _ => {}
    }
}
pub(crate) unsafe fn CB2_HandleStartMultiBattle() {
    let mut id: i32 = 0;
    let mut var: u8 = 0;
    let playerMultiplayerId: u8 = GetMultiplayerId();
    gBattleScripting.multiplayerId = playerMultiplayerId;
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    'l1: {
        let sw1: u8 = gBattleCommunication[0];
        let mut fall = false;
        if sw1 == 0 {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                ShowBg(0);
                ShowBg(1);
                ShowBg(2);
                ShowBg(3);
                FillAroundBattleWindows();
                gBattleCommunication[0] = 1;
            }
            if gWirelessCommType != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
            }
            break 'l1;
        }
        if sw1 == 1 {
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                if gReceivedRemoteLinkPlayers != 0 {
                    if IsLinkTaskFinished() != 0 {
                        (*gBattleStruct)
                            .multiBuffer
                            .linkBattlerHeader
                            .versionSignatureLo = 0;
                        (*gBattleStruct)
                            .multiBuffer
                            .linkBattlerHeader
                            .versionSignatureHi = 3;
                        BufferPartyVsScreenHealth_AtStart();
                        SetPlayerBerryDataInBattleStruct();
                        SendBlock(
                            BitmaskAllOtherLinkPlayers(),
                            &raw mut (*gBattleStruct).multiBuffer.linkBattlerHeader as *mut c_void,
                            32,
                        );
                        gBattleCommunication[0] += 1;
                    }
                    if gWirelessCommType != 0 {
                        CreateWirelessStatusIndicatorSprite(0, 0);
                    }
                }
            } else {
                if gBattleTypeFlags & BATTLE_TYPE_RECORDED == 0 {
                    gBattleTypeFlags |= BATTLE_TYPE_IS_MASTER;
                }
                gBattleCommunication[0] = 7;
                SetAllPlayersBerryData();
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if GetBlockReceivedStatus() as i32 & 0xF == 0xF {
                ResetBlockReceivedFlags();
                FindLinkBattleMaster(4, playerMultiplayerId);
                SetAllPlayersBerryData();
                SetDeoxysStats();
                var = CreateTask(Some(InitLinkBattleVsScreen), 0);
                task_set(var, 1, 0x10E);
                task_set(var, 2, 0x5A);
                task_set(var, 5, 0);
                task_set(var, 3, 0);
                task_set(var, 4, 0);
                for id in 0..MAX_LINK_PLAYERS {
                    RecordedBattle_SetFrontierPassFlagFromHword(gBlockRecvBuffer[id][1]);
                    match gLinkPlayers[id].id {
                        0 => {
                            task_set(
                                var,
                                3,
                                task_get(var, 3) | (gBlockRecvBuffer[id][1] as i16 & 0x3F),
                            );
                        }
                        1 => {
                            task_set(
                                var,
                                4,
                                task_get(var, 4) | (gBlockRecvBuffer[id][1] as i16 & 0x3F),
                            );
                        }
                        2 => {
                            task_set(
                                var,
                                3,
                                task_get(var, 3) | ((gBlockRecvBuffer[id][1] as i16 & 0x3F) << 6),
                            );
                        }
                        3 => {
                            task_set(
                                var,
                                4,
                                task_get(var, 4) | ((gBlockRecvBuffer[id][1] as i16 & 0x3F) << 6),
                            );
                        }
                        _ => {}
                    }
                }
                ZeroEnemyPartyMons();
                gBattleCommunication[0] += 1;
            } else {
                break 'l1;
            }
        }
        if fall || sw1 == 3 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    gPlayerParty.as_mut_ptr() as *mut c_void,
                    200,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            if GetBlockReceivedStatus() as i32 & 0xF == 0xF {
                ResetBlockReceivedFlags();
                for id in 0..MAX_LINK_PLAYERS {
                    if id == playerMultiplayerId as i32 {
                        match gLinkPlayers[id].id {
                            0 | 3 => {
                                memcpy(
                                    gPlayerParty.as_mut_ptr() as *mut u8,
                                    gBlockRecvBuffer[id].as_mut_ptr() as *mut u8,
                                    200,
                                );
                            }
                            1 | 2 => {
                                memcpy(
                                    gPlayerParty.as_mut_ptr().at(3) as *mut u8,
                                    gBlockRecvBuffer[id].as_mut_ptr() as *mut u8,
                                    200,
                                );
                            }
                            _ => {}
                        }
                    } else {
                        if gLinkPlayers[id].id as i32 & 1 == 0
                            && gLinkPlayers[playerMultiplayerId].id as i32 & 1 == 0
                            || gLinkPlayers[id].id as i32 & 1 != 0
                                && gLinkPlayers[playerMultiplayerId].id as i32 & 1 != 0
                        {
                            match gLinkPlayers[id].id {
                                0 | 3 => {
                                    memcpy(
                                        gPlayerParty.as_mut_ptr() as *mut u8,
                                        gBlockRecvBuffer[id].as_mut_ptr() as *mut u8,
                                        200,
                                    );
                                }
                                1 | 2 => {
                                    memcpy(
                                        gPlayerParty.as_mut_ptr().at(3) as *mut u8,
                                        gBlockRecvBuffer[id].as_mut_ptr() as *mut u8,
                                        200,
                                    );
                                }
                                _ => {}
                            }
                        } else {
                            match gLinkPlayers[id].id {
                                0 | 3 => {
                                    memcpy(
                                        gEnemyParty.as_mut_ptr() as *mut u8,
                                        gBlockRecvBuffer[id].as_mut_ptr() as *mut u8,
                                        200,
                                    );
                                }
                                1 | 2 => {
                                    memcpy(
                                        gEnemyParty.as_mut_ptr().at(3) as *mut u8,
                                        gBlockRecvBuffer[id].as_mut_ptr() as *mut u8,
                                        200,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                }
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 5 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    gPlayerParty.as_mut_ptr().at(2) as *mut c_void,
                    100,
                );
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 6 {
            if GetBlockReceivedStatus() as i32 & 0xF == 0xF {
                ResetBlockReceivedFlags();
                for id in 0..MAX_LINK_PLAYERS {
                    if id == playerMultiplayerId as i32 {
                        match gLinkPlayers[id].id {
                            0 | 3 => {
                                memcpy(
                                    gPlayerParty.as_mut_ptr().at(2) as *mut u8,
                                    gBlockRecvBuffer[id].as_mut_ptr() as *mut u8,
                                    100,
                                );
                            }
                            1 | 2 => {
                                memcpy(
                                    gPlayerParty.as_mut_ptr().at(5) as *mut u8,
                                    gBlockRecvBuffer[id].as_mut_ptr() as *mut u8,
                                    100,
                                );
                            }
                            _ => {}
                        }
                    } else {
                        if gLinkPlayers[id].id as i32 & 1 == 0
                            && gLinkPlayers[playerMultiplayerId].id as i32 & 1 == 0
                            || gLinkPlayers[id].id as i32 & 1 != 0
                                && gLinkPlayers[playerMultiplayerId].id as i32 & 1 != 0
                        {
                            match gLinkPlayers[id].id {
                                0 | 3 => {
                                    memcpy(
                                        gPlayerParty.as_mut_ptr().at(2) as *mut u8,
                                        gBlockRecvBuffer[id].as_mut_ptr() as *mut u8,
                                        100,
                                    );
                                }
                                1 | 2 => {
                                    memcpy(
                                        gPlayerParty.as_mut_ptr().at(5) as *mut u8,
                                        gBlockRecvBuffer[id].as_mut_ptr() as *mut u8,
                                        100,
                                    );
                                }
                                _ => {}
                            }
                        } else {
                            match gLinkPlayers[id].id {
                                0 | 3 => {
                                    memcpy(
                                        gEnemyParty.as_mut_ptr().at(2) as *mut u8,
                                        gBlockRecvBuffer[id].as_mut_ptr() as *mut u8,
                                        100,
                                    );
                                }
                                1 | 2 => {
                                    memcpy(
                                        gEnemyParty.as_mut_ptr().at(5) as *mut u8,
                                        gBlockRecvBuffer[id].as_mut_ptr() as *mut u8,
                                        100,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                }
                TryCorrectShedinjaLanguage(&raw mut gPlayerParty[0]);
                TryCorrectShedinjaLanguage(&raw mut gPlayerParty[1]);
                TryCorrectShedinjaLanguage(&raw mut gPlayerParty[2]);
                TryCorrectShedinjaLanguage(&raw mut gPlayerParty[3]);
                TryCorrectShedinjaLanguage(&raw mut gPlayerParty[4]);
                TryCorrectShedinjaLanguage(&raw mut gPlayerParty[5]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[0]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[1]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[2]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[3]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[4]);
                TryCorrectShedinjaLanguage(&raw mut gEnemyParty[5]);
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 7 {
            InitBattleControllers();
            RecordedBattle_SetTrainerInfo();
            gBattleCommunication[1] = 0;
            gBattleCommunication[2] = 0;
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                id = 0;
                while id < MAX_LINK_PLAYERS
                    && gLinkPlayers[id].version as i32 & 0xFF == VERSION_EMERALD as i32
                {
                    id += 1;
                }
                if id == MAX_LINK_PLAYERS {
                    gBattleCommunication[0] = 8;
                } else {
                    gBattleCommunication[0] = 10;
                }
            } else {
                gBattleCommunication[0] = 10;
            }
            break 'l1;
        }
        if sw1 == 8 {
            if IsLinkTaskFinished() != 0 {
                let ptr: *mut u32 = (*gBattleStruct).multiBuffer.battleVideo.as_mut_ptr();
                *ptr = gBattleTypeFlags;
                *ptr.at(1) = gRecordedBattleRngSeed;
                SendBlock(BitmaskAllOtherLinkPlayers(), ptr as *mut c_void, 8);
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 9 {
            if GetBlockReceivedStatus() as i32 & 0xF == 0xF {
                ResetBlockReceivedFlags();
                for var in 0..4u8 {
                    let blockValue: u32 = gBlockRecvBuffer[var][0] as u32;
                    if blockValue & 4 != 0 {
                        memcpy(
                            &raw mut gRecordedBattleRngSeed as *mut u8,
                            &raw mut gBlockRecvBuffer[var][2] as *mut u8,
                            4,
                        );
                        break;
                    }
                }
                gBattleCommunication[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 10 {
            if BattleInitAllSprites(
                &raw mut gBattleCommunication[1],
                &raw mut gBattleCommunication[2],
            ) != 0
            {
                gPreBattleCallback1 = gMain.callback1;
                gMain.callback1 = Some(BattleMainCB1);
                SetMainCallback2(Some(BattleMainCB2));
                if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                    gTrainerBattleOpponent_A = TRAINER_LINK_OPPONENT;
                    gBattleTypeFlags |= BATTLE_TYPE_LINK_IN_BATTLE;
                }
            }
            break 'l1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn BattleMainCB2() {
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
    RunTasks();
    if gMain.heldKeys as i32 & B_BUTTON != 0
        && gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0
        && RecordedBattle_CanStopPlayback() != 0
    {
        gSpecialVar_Result = ({
            gBattleOutcome = B_OUTCOME_PLAYER_TELEPORTED;
            gBattleOutcome
        }) as u16;
        ResetPaletteFadeControl();
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        SetMainCallback2(Some(CB2_QuitRecordedBattle));
    }
}
unsafe fn FreeRestoreBattleData() {
    gMain.callback1 = gPreBattleCallback1;
    gScanlineEffect.state = 3;
    gMain.set_inBattle(FALSE);
    ZeroEnemyPartyMons();
    m4aSongNumStop(SE_LOW_HEALTH);
    FreeMonSpritesGfx();
    FreeBattleSpritesData();
    FreeBattleResources();
}
pub unsafe fn CB2_QuitRecordedBattle() {
    UpdatePaletteFade();
    if gPaletteFade.active() == 0 {
        m4aMPlayStop(&raw mut gMPlayInfo_SE1);
        m4aMPlayStop(&raw mut gMPlayInfo_SE2);
        FreeRestoreBattleData();
        FreeAllWindowBuffers();
        SetMainCallback2(gMain.savedCallback);
    }
}
pub(crate) unsafe fn SpriteCB_UnusedBattleInit(sprite: *mut Sprite) {
    (*sprite).data[sState] = 0;
    (*sprite).callback = Some(SpriteCB_UnusedBattleInit_Main);
}
pub(crate) unsafe fn SpriteCB_UnusedBattleInit_Main(sprite: *mut Sprite) {
    let arr: *mut u16 = (*(&raw const crate::decompress::gDecompressionBuffer)
        .cast::<CArray<u8, 16384>>()
        .cast_mut())
    .as_mut_ptr() as *mut u16;
    'l1: {
        let sw1: i16 = (*sprite).data[sState];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).data[sState] += 1;
            (*sprite).data[1] = 0;
            (*sprite).data[2] = 0x281;
            (*sprite).data[3] = 0;
            (*sprite).data[sDelay] = 1;
        }
        if fall || sw1 == 1 {
            (*sprite).data[sDelay] -= 1;
            if (*sprite).data[sDelay] == 0 {
                (*sprite).data[sDelay] = 2;
                let r2: i32 = (*sprite).data[1] as i32 + (*sprite).data[3] as i32 * 32;
                let r0: i32 = (*sprite).data[2] as i32 - (*sprite).data[3] as i32 * 32;
                let mut i: i32 = 0;
                while i < 29 {
                    *arr.at(r2 + i) = 0x3D;
                    *arr.at(r0 + i) = 0x3D;
                    i += 2;
                }
                (*sprite).data[3] += 1;
                if (*sprite).data[3] == 21 {
                    (*sprite).data[sState] += 1;
                    (*sprite).data[1] = 32;
                }
            }
            break 'l1;
        }
        if sw1 == 2 {
            (*sprite).data[1] -= 1;
            if (*sprite).data[1] == 20 {
                SetMainCallback2(Some(CB2_InitBattle));
            }
            break 'l1;
        }
    }
}
unsafe fn CreateNPCTrainerParty(party: *mut Pokemon, trainerNum: u16, firstTrainer: u8) -> u8 {
    let mut nameHash: u32 = 0;
    let mut personalityValue: u32 = 0;
    let mut fixedIV: u8 = 0;
    let mut j: i32 = 0;
    let mut monsCount: u8 = 0;
    if trainerNum == TRAINER_SECRET_BASE {
        return 0;
    }
    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 && gBattleTypeFlags & 0x43f0900 == 0 {
        if firstTrainer == TRUE {
            ZeroEnemyPartyMons();
        }
        if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
            if (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
                [trainerNum]
                .partySize
                > 3
            {
                monsCount = 3;
            } else {
                monsCount = (*(&raw const crate::data::data_tables::gTrainers)
                    .cast::<CArray<Trainer, 0>>())[trainerNum]
                    .partySize;
            }
        } else {
            monsCount = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[trainerNum]
                .partySize;
        }
        for i in 0..(monsCount as i32) {
            if (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
                [trainerNum]
                .doubleBattle
                == TRUE
            {
                personalityValue = 0x80;
            } else if (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[trainerNum]
                .encounterMusic_gender as i32
                & F_TRAINER_FEMALE
                != 0
            {
                personalityValue = 0x78;
            } else {
                personalityValue = 0x88;
            }
            j = 0;
            while (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
                [trainerNum]
                .trainerName[j]
                != EOS
            {
                nameHash += (*(&raw const crate::data::data_tables::gTrainers)
                    .cast::<CArray<Trainer, 0>>())[trainerNum]
                    .trainerName[j] as u32;
                j += 1;
            }
            'l3: {
                match (*(&raw const crate::data::data_tables::gTrainers)
                    .cast::<CArray<Trainer, 0>>())[trainerNum]
                    .partyFlags
                {
                    0 => {
                        let partyData: *mut TrainerMonNoItemDefaultMoves =
                            (*(&raw const crate::data::data_tables::gTrainers)
                                .cast::<CArray<Trainer, 0>>())[trainerNum]
                                .party
                                .NoItemDefaultMoves;
                        j = 0;
                        while (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<
                            CArray<u8, 11>,
                            0,
                        >>(
                        ))[(*partyData.at(i)).species][j]
                            != EOS
                        {
                            nameHash += (*(&raw const crate::data::data_tables::gSpeciesNames)
                                .cast::<CArray<CArray<u8, 11>, 0>>())[(*partyData.at(i)).species][j]
                                as u32;
                            j += 1;
                        }
                        personalityValue += nameHash << 8;
                        fixedIV =
                            ((*partyData.at(i)).iv as i32 * MAX_PER_STAT_IVS as i32 / 255) as u8;
                        CreateMon(
                            party.at(i),
                            (*partyData.at(i)).species,
                            (*partyData.at(i)).lvl,
                            fixedIV,
                            TRUE,
                            personalityValue,
                            OT_ID_RANDOM_NO_SHINY,
                            0,
                        );
                        break 'l3;
                    }
                    F_TRAINER_PARTY_CUSTOM_MOVESET => {
                        let partyData: *mut TrainerMonNoItemCustomMoves =
                            (*(&raw const crate::data::data_tables::gTrainers)
                                .cast::<CArray<Trainer, 0>>())[trainerNum]
                                .party
                                .NoItemCustomMoves;
                        j = 0;
                        while (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<
                            CArray<u8, 11>,
                            0,
                        >>(
                        ))[(*partyData.at(i)).species][j]
                            != EOS
                        {
                            nameHash += (*(&raw const crate::data::data_tables::gSpeciesNames)
                                .cast::<CArray<CArray<u8, 11>, 0>>())[(*partyData.at(i)).species][j]
                                as u32;
                            j += 1;
                        }
                        personalityValue += nameHash << 8;
                        fixedIV =
                            ((*partyData.at(i)).iv as i32 * MAX_PER_STAT_IVS as i32 / 255) as u8;
                        CreateMon(
                            party.at(i),
                            (*partyData.at(i)).species,
                            (*partyData.at(i)).lvl,
                            fixedIV,
                            TRUE,
                            personalityValue,
                            OT_ID_RANDOM_NO_SHINY,
                            0,
                        );
                        for j in 0..MAX_MON_MOVES {
                            SetMonData(
                                party.at(i),
                                MON_DATA_MOVE1 + j,
                                &raw mut (*partyData.at(i)).moves[j] as *mut c_void,
                            );
                            SetMonData(
                                party.at(i),
                                MON_DATA_PP1 + j,
                                (&raw const (*(&raw const crate::data::pokemon::gBattleMoves)
                                    .cast::<CArray<BattleMove, 0>>())[(*partyData.at(i)).moves[j]]
                                    .pp)
                                    .cast_mut() as *mut c_void,
                            );
                        }
                        break 'l3;
                    }
                    F_TRAINER_PARTY_HELD_ITEM => {
                        let partyData: *mut TrainerMonItemDefaultMoves =
                            (*(&raw const crate::data::data_tables::gTrainers)
                                .cast::<CArray<Trainer, 0>>())[trainerNum]
                                .party
                                .ItemDefaultMoves;
                        j = 0;
                        while (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<
                            CArray<u8, 11>,
                            0,
                        >>(
                        ))[(*partyData.at(i)).species][j]
                            != EOS
                        {
                            nameHash += (*(&raw const crate::data::data_tables::gSpeciesNames)
                                .cast::<CArray<CArray<u8, 11>, 0>>())[(*partyData.at(i)).species][j]
                                as u32;
                            j += 1;
                        }
                        personalityValue += nameHash << 8;
                        fixedIV =
                            ((*partyData.at(i)).iv as i32 * MAX_PER_STAT_IVS as i32 / 255) as u8;
                        CreateMon(
                            party.at(i),
                            (*partyData.at(i)).species,
                            (*partyData.at(i)).lvl,
                            fixedIV,
                            TRUE,
                            personalityValue,
                            OT_ID_RANDOM_NO_SHINY,
                            0,
                        );
                        SetMonData(
                            party.at(i),
                            MON_DATA_HELD_ITEM,
                            &raw mut (*partyData.at(i)).heldItem as *mut c_void,
                        );
                        break 'l3;
                    }
                    3 => {
                        let partyData: *mut TrainerMonItemCustomMoves =
                            (*(&raw const crate::data::data_tables::gTrainers)
                                .cast::<CArray<Trainer, 0>>())[trainerNum]
                                .party
                                .ItemCustomMoves;
                        j = 0;
                        while (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<
                            CArray<u8, 11>,
                            0,
                        >>(
                        ))[(*partyData.at(i)).species][j]
                            != EOS
                        {
                            nameHash += (*(&raw const crate::data::data_tables::gSpeciesNames)
                                .cast::<CArray<CArray<u8, 11>, 0>>())[(*partyData.at(i)).species][j]
                                as u32;
                            j += 1;
                        }
                        personalityValue += nameHash << 8;
                        fixedIV =
                            ((*partyData.at(i)).iv as i32 * MAX_PER_STAT_IVS as i32 / 255) as u8;
                        CreateMon(
                            party.at(i),
                            (*partyData.at(i)).species,
                            (*partyData.at(i)).lvl,
                            fixedIV,
                            TRUE,
                            personalityValue,
                            OT_ID_RANDOM_NO_SHINY,
                            0,
                        );
                        SetMonData(
                            party.at(i),
                            MON_DATA_HELD_ITEM,
                            &raw mut (*partyData.at(i)).heldItem as *mut c_void,
                        );
                        for j in 0..MAX_MON_MOVES {
                            SetMonData(
                                party.at(i),
                                MON_DATA_MOVE1 + j,
                                &raw mut (*partyData.at(i)).moves[j] as *mut c_void,
                            );
                            SetMonData(
                                party.at(i),
                                MON_DATA_PP1 + j,
                                (&raw const (*(&raw const crate::data::pokemon::gBattleMoves)
                                    .cast::<CArray<BattleMove, 0>>())[(*partyData.at(i)).moves[j]]
                                    .pp)
                                    .cast_mut() as *mut c_void,
                            );
                        }
                        break 'l3;
                    }
                    _ => {}
                }
            }
        }
        gBattleTypeFlags |= (*(&raw const crate::data::data_tables::gTrainers)
            .cast::<CArray<Trainer, 0>>())[trainerNum]
            .doubleBattle as u32;
    }
    (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())[trainerNum]
        .partySize
}
unsafe fn HBlankCB_Battle() {
    if (67108870_usize as *mut u16).read_volatile() < DISPLAY_HEIGHT
        && (67108870_usize as *mut u16).read_volatile() >= 111
    {
        SetGpuReg(REG_OFFSET_BG0CNT, 38912);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn VBlankCB_Battle() {
    if gBattleTypeFlags & 0x13f0102 == 0 {
        Random();
    }
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
pub fn SpriteCB_VsLetterDummy(sprite: *mut Sprite) {}
pub(crate) unsafe fn SpriteCB_VsLetter(sprite: *mut Sprite) {
    if (*sprite).data[0] != 0 {
        (*sprite).x = (*sprite).data[1] + (((*sprite).data[2] as i32 & 0xFF00) >> 8) as i16;
    } else {
        (*sprite).x = (*sprite).data[1] - (((*sprite).data[2] as i32 & 0xFF00) >> 8) as i16;
    }
    (*sprite).data[2] += 0x180;
    if (*sprite).affineAnimEnded() != 0 {
        FreeSpriteTilesByTag(ANIM_SPRITES_START);
        FreeSpritePaletteByTag(ANIM_SPRITES_START);
        FreeSpriteOamMatrix(sprite);
        DestroySprite(sprite);
    }
}
pub unsafe fn SpriteCB_VsLetterInit(sprite: *mut Sprite) {
    StartSpriteAffineAnim(sprite, 1);
    (*sprite).callback = Some(SpriteCB_VsLetter);
    PlaySE(SE_MUGSHOT);
}
unsafe fn BufferPartyVsScreenHealth_AtEnd(taskId: u8) {
    let mut party1: *mut Pokemon = null_mut();
    let mut party2: *mut Pokemon = null_mut();
    let multiplayerId: u8 = gBattleScripting.multiplayerId;
    if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        match gLinkPlayers[multiplayerId].id {
            0 | 2 => {
                party1 = gPlayerParty.as_mut_ptr();
                party2 = gEnemyParty.as_mut_ptr();
            }
            1 | 3 => {
                party1 = gEnemyParty.as_mut_ptr();
                party2 = gPlayerParty.as_mut_ptr();
            }
            _ => {}
        }
    } else {
        party1 = gPlayerParty.as_mut_ptr();
        party2 = gEnemyParty.as_mut_ptr();
    }
    let mut flags: u32 = 0;
    for i in 0..6i32 {
        'l2: {
            let species: u16 = GetMonData2(party1.at(i), MON_DATA_SPECIES_OR_EGG) as u16;
            let hp: u16 = GetMonData2(party1.at(i), MON_DATA_HP) as u16;
            let status: u32 = GetMonData2(party1.at(i), MON_DATA_STATUS);
            if species == 0 {
                break 'l2;
            }
            if species != 412 && hp != 0 && status == 0 {
                flags |= shl_i32(1, i as u32 * 2) as u32;
            }
            if species == 0 {
                break 'l2;
            }
            if hp != 0 && (species == 412 || status != 0) {
                flags |= shl_i32(2, i as u32 * 2) as u32;
            }
            if species == 0 {
                break 'l2;
            }
            if species != 412 && hp == 0 {
                flags |= shl_i32(3, i as u32 * 2) as u32;
            }
        }
    }
    task_set(taskId, 3, flags as i16);
    flags = 0;
    for i in 0..6i32 {
        'l4: {
            let species: u16 = GetMonData2(party2.at(i), MON_DATA_SPECIES_OR_EGG) as u16;
            let hp: u16 = GetMonData2(party2.at(i), MON_DATA_HP) as u16;
            let status: u32 = GetMonData2(party2.at(i), MON_DATA_STATUS);
            if species == 0 {
                break 'l4;
            }
            if species != 412 && hp != 0 && status == 0 {
                flags |= shl_i32(1, i as u32 * 2) as u32;
            }
            if species == 0 {
                break 'l4;
            }
            if hp != 0 && (species == 412 || status != 0) {
                flags |= shl_i32(2, i as u32 * 2) as u32;
            }
            if species == 0 {
                break 'l4;
            }
            if species != 412 && hp == 0 {
                flags |= shl_i32(3, i as u32 * 2) as u32;
            }
        }
    }
    task_set(taskId, 4, flags as i16);
}
pub unsafe fn CB2_InitEndLinkBattle() {
    let mut i: i32 = 0;
    let mut taskId: u8 = 0;
    SetHBlankCallback(None);
    SetVBlankCallback(None);
    gBattleTypeFlags &= 0xffffffdf;
    if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
        SetMainCallback2(gMain.savedCallback);
        FreeBattleResources();
        FreeBattleSpritesData();
        FreeMonSpritesGfx();
    } else {
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
        SetGpuReg(REG_OFFSET_MOSAIC, 0);
        SetGpuReg(REG_OFFSET_WIN0H, DISPLAY_WIDTH);
        SetGpuReg(REG_OFFSET_WIN0V, 20561);
        SetGpuReg(REG_OFFSET_WININ, 0);
        SetGpuReg(REG_OFFSET_WINOUT, 0);
        gBattle_WIN0H = DISPLAY_WIDTH;
        gBattle_WIN0V = 20561;
        ScanlineEffect_Clear();
        i = 0;
        while i < 80 {
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[0][i] = 0xF0;
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[1][i] = 0xF0;
            i += 1;
        }
        while i < 160 {
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[0][i] = 0xFF10;
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[1][i] = 0xFF10;
            i += 1;
        }
        ResetPaletteFade();
        gBattle_BG0_X = 0;
        gBattle_BG0_Y = 0;
        gBattle_BG1_X = 0;
        gBattle_BG1_Y = 0;
        gBattle_BG2_X = 0;
        gBattle_BG2_Y = 0;
        gBattle_BG3_X = 0;
        gBattle_BG3_Y = 0;
        InitBattleBgsVideo();
        LoadCompressedPalette(
            (*(&raw const crate::data::graphics::gBattleTextboxPalette).cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
            64,
        );
        LoadBattleMenuWindowGfx();
        ResetSpriteData();
        ResetTasks();
        DrawBattleEntryBackground();
        SetGpuReg(REG_OFFSET_WINOUT, 55);
        FreeAllSpritePalettes();
        gReservedSpritePaletteCount = MAX_BATTLERS_COUNT;
        SetVBlankCallback(Some(VBlankCB_Battle));
        taskId = CreateTask(Some(InitLinkBattleVsScreen), 0);
        task_set(taskId, 1, 0x10E);
        task_set(taskId, 2, 0x5A);
        task_set(taskId, 5, 1);
        BufferPartyVsScreenHealth_AtEnd(taskId);
        SetMainCallback2(Some(CB2_EndLinkBattle));
        gBattleCommunication[0] = 0;
    }
}
pub(crate) unsafe fn CB2_EndLinkBattle() {
    EndLinkBattleInSteps();
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
    RunTasks();
}
unsafe fn EndLinkBattleInSteps() {
    let mut i: i32 = 0;
    match gBattleCommunication[0] {
        0 => {
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            gBattleCommunication[1] = 0xFF;
            gBattleCommunication[0] += 1;
        }
        1 => {
            if ({
                gBattleCommunication[1] -= 1;
                gBattleCommunication[1]
            }) == 0
            {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                gBattleCommunication[0] += 1;
            }
        }
        2 => {
            if gPaletteFade.active() == 0 {
                let mut battlerCount: u8 = 0;
                gMain.set_anyLinkBattlerHasFrontierPass(RecordedBattle_GetFrontierPassFlag());
                if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                    battlerCount = 4;
                } else {
                    battlerCount = 2;
                }
                i = 0;
                while i < battlerCount as i32
                    && gLinkPlayers[i].version as i32 & 0xFF == VERSION_EMERALD as i32
                {
                    i += 1;
                }
                if (*gSaveBlock2Ptr).frontier.disableRecordBattle() == 0 && i == battlerCount as i32
                {
                    if FlagGet(FLAG_SYS_FRONTIER_PASS) != 0 {
                        FreeAllWindowBuffers();
                        SetMainCallback2(Some(CB2_InitAskRecordBattle));
                    } else if gMain.anyLinkBattlerHasFrontierPass() == 0 {
                        SetMainCallback2(gMain.savedCallback);
                        FreeBattleResources();
                        FreeBattleSpritesData();
                        FreeMonSpritesGfx();
                    } else if gReceivedRemoteLinkPlayers == 0 {
                        CreateTask(Some(Task_ReconnectWithLinkPlayers), 5);
                        gBattleCommunication[0] += 1;
                    } else {
                        gBattleCommunication[0] += 1;
                    }
                } else {
                    SetMainCallback2(gMain.savedCallback);
                    FreeBattleResources();
                    FreeBattleSpritesData();
                    FreeMonSpritesGfx();
                }
            }
        }
        3 => {
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
            for i in 0..2i32 {
                LoadChosenBattleElement(i as u8);
            }
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            gBattleCommunication[0] += 1;
        }
        4 => {
            if gPaletteFade.active() == 0 {
                gBattleCommunication[0] += 1;
            }
        }
        5 => {
            if FuncIsActiveTask(Some(Task_ReconnectWithLinkPlayers)) == 0 {
                gBattleCommunication[0] += 1;
            }
        }
        6 => {
            if IsLinkTaskFinished() == TRUE {
                SetLinkStandbyCallback();
                BattlePutTextOnWindow(
                    (*crate::asmdata::gText_LinkStandby3.cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    B_WIN_MSG,
                );
                gBattleCommunication[0] += 1;
            }
        }
        7 => {
            if IsTextPrinterActive(B_WIN_MSG) == 0 && IsLinkTaskFinished() == TRUE {
                gBattleCommunication[0] += 1;
            }
        }
        8 => {
            if gWirelessCommType == 0 {
                SetCloseLinkCallback();
            }
            gBattleCommunication[0] += 1;
        }
        9 if (gMain.anyLinkBattlerHasFrontierPass() == 0
            || gWirelessCommType != 0
            || gReceivedRemoteLinkPlayers != 1) =>
        {
            gMain.set_anyLinkBattlerHasFrontierPass(FALSE);
            SetMainCallback2(gMain.savedCallback);
            FreeBattleResources();
            FreeBattleSpritesData();
            FreeMonSpritesGfx();
        }
        _ => {}
    }
}
pub unsafe fn GetBattleBgTemplateData(arrayId: u8, caseId: u8) -> u32 {
    let mut ret: u32 = 0;
    match caseId {
        0 => {
            ret = (*(&raw const crate::data::battle_bg::gBattleBgTemplates)
                .cast::<CArray<BgTemplate, 0>>())[arrayId]
                .bg() as u32;
        }
        1 => {
            ret = (*(&raw const crate::data::battle_bg::gBattleBgTemplates)
                .cast::<CArray<BgTemplate, 0>>())[arrayId]
                .charBaseIndex() as u32;
        }
        2 => {
            ret = (*(&raw const crate::data::battle_bg::gBattleBgTemplates)
                .cast::<CArray<BgTemplate, 0>>())[arrayId]
                .mapBaseIndex() as u32;
        }
        3 => {
            ret = (*(&raw const crate::data::battle_bg::gBattleBgTemplates)
                .cast::<CArray<BgTemplate, 0>>())[arrayId]
                .screenSize() as u32;
        }
        4 => {
            ret = (*(&raw const crate::data::battle_bg::gBattleBgTemplates)
                .cast::<CArray<BgTemplate, 0>>())[arrayId]
                .paletteMode() as u32;
        }
        5 => {
            ret = (*(&raw const crate::data::battle_bg::gBattleBgTemplates)
                .cast::<CArray<BgTemplate, 0>>())[arrayId]
                .priority() as u32;
        }
        6 => {
            ret = (*(&raw const crate::data::battle_bg::gBattleBgTemplates)
                .cast::<CArray<BgTemplate, 0>>())[arrayId]
                .baseTile() as u32;
        }
        _ => {}
    }
    ret
}
pub(crate) unsafe fn CB2_InitAskRecordBattle() {
    SetHBlankCallback(None);
    SetVBlankCallback(None);
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
    ResetPaletteFade();
    gBattle_BG0_X = 0;
    gBattle_BG0_Y = 0;
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    gBattle_BG2_X = 0;
    gBattle_BG2_Y = 0;
    gBattle_BG3_X = 0;
    gBattle_BG3_Y = 0;
    InitBattleBgsVideo();
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    LoadBattleMenuWindowGfx();
    for i in 0..2i32 {
        LoadChosenBattleElement(i as u8);
    }
    ResetSpriteData();
    ResetTasks();
    FreeAllSpritePalettes();
    gReservedSpritePaletteCount = MAX_BATTLERS_COUNT;
    SetVBlankCallback(Some(VBlankCB_Battle));
    SetMainCallback2(Some(CB2_AskRecordBattle));
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
    gBattleCommunication[0] = 0;
}
pub(crate) unsafe fn CB2_AskRecordBattle() {
    AskRecordBattle();
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
    RunTasks();
}
unsafe fn AskRecordBattle() {
    match gBattleCommunication[0] {
        STATE_INIT => {
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            gBattleCommunication[0] += 1;
        }
        STATE_LINK => {
            if gMain.anyLinkBattlerHasFrontierPass() != 0 && gReceivedRemoteLinkPlayers == 0 {
                CreateTask(Some(Task_ReconnectWithLinkPlayers), 5);
            }
            gBattleCommunication[0] += 1;
        }
        STATE_WAIT_LINK => {
            if FuncIsActiveTask(Some(Task_ReconnectWithLinkPlayers)) == 0 {
                gBattleCommunication[0] += 1;
            }
        }
        STATE_ASK_RECORD => {
            if gPaletteFade.active() == 0 {
                BattlePutTextOnWindow(
                    (*(&raw const crate::data::battle_message::gText_RecordBattleToPass)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    B_WIN_MSG,
                );
                gBattleCommunication[0] += 1;
            }
        }
        STATE_PRINT_YES_NO => {
            if IsTextPrinterActive(B_WIN_MSG) == 0 {
                HandleBattleWindow(24, 8, 29, 13, 0);
                BattlePutTextOnWindow(
                    (*(&raw const crate::data::battle_message::gText_BattleYesNoChoice)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    B_WIN_YESNO,
                );
                gBattleCommunication[1] = 1;
                BattleCreateYesNoCursorAt(1);
                gBattleCommunication[0] += 1;
            }
        }
        STATE_HANDLE_YES_NO => {
            if gMain.newKeys as i32 & DPAD_UP != 0 {
                if gBattleCommunication[1] != 0 {
                    PlaySE(SE_SELECT);
                    BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                    gBattleCommunication[1] = 0;
                    BattleCreateYesNoCursorAt(0);
                }
            } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
                if gBattleCommunication[1] == 0 {
                    PlaySE(SE_SELECT);
                    BattleDestroyYesNoCursorAt(gBattleCommunication[1]);
                    gBattleCommunication[1] = 1;
                    BattleCreateYesNoCursorAt(1);
                }
            } else if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if gBattleCommunication[1] == 0 {
                    HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                    gBattleCommunication[1] = MoveRecordedBattleToSaveData() as u8;
                    gBattleCommunication[0] = STATE_RECORD_YES;
                } else {
                    gBattleCommunication[0] += 1;
                }
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                gBattleCommunication[0] += 1;
            }
        }
        STATE_RECORD_NO => {
            if IsLinkTaskFinished() == TRUE {
                HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                if gMain.anyLinkBattlerHasFrontierPass() != 0 {
                    SetLinkStandbyCallback();
                    BattlePutTextOnWindow(
                        (*crate::asmdata::gText_LinkStandby3.cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        B_WIN_MSG,
                    );
                }
                gBattleCommunication[0] += 1;
            }
        }
        STATE_WAIT_END => {
            if ({
                gBattleCommunication[1] -= 1;
                gBattleCommunication[1]
            }) == 0
            {
                if gMain.anyLinkBattlerHasFrontierPass() != 0 && gWirelessCommType == 0 {
                    SetCloseLinkCallback();
                }
                gBattleCommunication[0] += 1;
            }
        }
        STATE_END => {
            if gMain.anyLinkBattlerHasFrontierPass() == 0
                || gWirelessCommType != 0
                || gReceivedRemoteLinkPlayers != 1
            {
                gMain.set_anyLinkBattlerHasFrontierPass(FALSE);
                if gPaletteFade.active() == 0 {
                    SetMainCallback2(gMain.savedCallback);
                    FreeBattleResources();
                    FreeBattleSpritesData();
                    FreeMonSpritesGfx();
                }
            }
        }
        STATE_RECORD_YES => {
            if gBattleCommunication[1] == 1 {
                PlaySE(SE_SAVE);
                BattleStringExpandPlaceholdersToDisplayedString(
                    (*(&raw const crate::data::battle_message::gText_BattleRecordedOnPass)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                gBattleCommunication[1] = 128;
                gBattleCommunication[0] += 1;
            } else {
                BattleStringExpandPlaceholdersToDisplayedString(
                    (*crate::asmdata::BattleFrontier_BattleTowerBattleRoom_Text_RecordCouldntBeSaved.cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
                BattlePutTextOnWindow(gDisplayedStringBattle.as_mut_ptr(), B_WIN_MSG);
                gBattleCommunication[1] = 128;
                gBattleCommunication[0] += 1;
            }
        }
        STATE_RECORD_WAIT => {
            if IsLinkTaskFinished() == 1
                && IsTextPrinterActive(0) == 0
                && ({
                    gBattleCommunication[1] -= 1;
                    gBattleCommunication[1]
                }) == 0
            {
                if gMain.anyLinkBattlerHasFrontierPass() != 0 {
                    SetLinkStandbyCallback();
                    BattlePutTextOnWindow(
                        (*crate::asmdata::gText_LinkStandby3.cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        B_WIN_MSG,
                    );
                }
                gBattleCommunication[0] += 1;
            }
        }
        STATE_END_RECORD_YES | STATE_END_RECORD_NO if IsTextPrinterActive(B_WIN_MSG) == 0 => {
            if gMain.anyLinkBattlerHasFrontierPass() != 0 {
                if IsLinkTaskFinished() == TRUE {
                    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                    gBattleCommunication[1] = 32;
                    gBattleCommunication[0] = STATE_WAIT_END;
                }
            } else {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                gBattleCommunication[1] = 32;
                gBattleCommunication[0] = STATE_WAIT_END;
            }
        }
        _ => {}
    }
}
unsafe fn TryCorrectShedinjaLanguage(mon: *mut Pokemon) {
    let mut nickname: CArray<u8, 11> = zeroed();
    let mut language: u8 = LANGUAGE_JAPANESE;
    if GetMonData2(mon, MON_DATA_SPECIES) == SPECIES_SHEDINJA as u32
        && GetMonData2(mon, MON_DATA_LANGUAGE) != language as u32
    {
        GetMonData3(mon, MON_DATA_NICKNAME, nickname.as_mut_ptr());
        if StringCompareWithoutExtCtrlCodes(
            nickname.as_mut_ptr(),
            sText_ShedinjaJpnName.as_ptr().cast_mut(),
        ) == 0
        {
            SetMonData(mon, MON_DATA_LANGUAGE, &raw mut language as *mut c_void);
        }
    }
}
pub unsafe fn GetBattleWindowTemplatePixelWidth(windowsType: u32, tableId: u32) -> u32 {
    (*(*(&raw const crate::data::battle_bg::gBattleWindowTemplates)
        .cast::<CArray<*mut WindowTemplate, 0>>())[windowsType]
        .at(tableId))
    .width as u32
        * 8
}
pub unsafe fn SpriteCB_WildMon(sprite: *mut Sprite) {
    (*sprite).callback = Some(SpriteCB_MoveWildMonToRight);
    StartSpriteAnimIfDifferent(sprite, 0);
    BeginNormalPaletteFade(0x20000, 0, 10, 10, 8456);
}
pub(crate) unsafe fn SpriteCB_MoveWildMonToRight(sprite: *mut Sprite) {
    if gIntroSlideFlags as i32 & 1 == 0 {
        (*sprite).x2 += 2;
        if (*sprite).x2 == 0 {
            (*sprite).callback = Some(SpriteCB_WildMonShowHealthbox);
        }
    }
}
pub(crate) unsafe fn SpriteCB_WildMonShowHealthbox(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        StartHealthboxSlideIn((*sprite).data[sBattler] as u8);
        SetHealthboxSpriteVisible(gHealthboxSpriteIds[(*sprite).data[sBattler]]);
        (*sprite).callback = Some(SpriteCB_WildMonAnimate);
        StartSpriteAnimIfDifferent(sprite, 0);
        BeginNormalPaletteFade(0x20000, 0, 10, 0, 8456);
    }
}
pub(crate) unsafe fn SpriteCB_WildMonAnimate(sprite: *mut Sprite) {
    if gPaletteFade.active() == 0 {
        BattleAnimateFrontSprite(sprite, (*sprite).data[sSpeciesId] as u16, FALSE, 1);
    }
}
pub unsafe fn SpriteCallbackDummy_2(sprite: *mut Sprite) {}
unsafe fn SpriteCB_InitFlicker(sprite: *mut Sprite) {
    (*sprite).data[sNumFlickers] = 6;
    (*sprite).data[sDelay] = 1;
    (*sprite).callback = Some(SpriteCB_Flicker);
}
pub(crate) unsafe fn SpriteCB_Flicker(sprite: *mut Sprite) {
    (*sprite).data[sDelay] -= 1;
    if (*sprite).data[sDelay] == 0 {
        (*sprite).data[sDelay] = 8;
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
        (*sprite).data[sNumFlickers] -= 1;
        if (*sprite).data[sNumFlickers] == 0 {
            (*sprite).set_invisible(FALSE as u16);
            (*sprite).callback = Some(SpriteCallbackDummy_2);
            sFlickerArray[0] = 0;
        }
    }
}
pub unsafe fn SpriteCB_FaintOpponentMon(sprite: *mut Sprite) {
    let battler: u8 = (*sprite).data[sBattler] as u8;
    let mut species: u16 = 0;
    let mut yOffset: u8 = 0;
    if (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies != 0 {
        species = (*(*gBattleSpritesDataPtr).battlerData.at(battler)).transformSpecies;
    } else {
        species = (*sprite).data[sSpeciesId] as u16;
    }
    GetMonData2(
        &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
        MON_DATA_PERSONALITY,
    );
    if species == SPECIES_UNOWN {
        let personalityValue: u32 = GetMonData2(
            &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
            MON_DATA_PERSONALITY,
        );
        let unownForm: u16 = (((personalityValue & 0x03000000) >> 18
            | (personalityValue & 0x00030000) >> 12
            | (personalityValue & 0x00000300) >> 6
            | (personalityValue & 0x00000003))
            % 28) as u16;
        let mut unownSpecies: u16 = 0;
        if unownForm == 0 {
            unownSpecies = SPECIES_UNOWN;
        } else {
            unownSpecies = NUM_SPECIES + unownForm;
        }
        yOffset = (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
            .cast::<CArray<MonCoords, 0>>())[unownSpecies]
            .y_offset;
    } else if species == SPECIES_CASTFORM {
        yOffset = (*(&raw const crate::data::battle_anim_mons::gCastformFrontSpriteCoords)
            .cast::<CArray<MonCoords, 0>>())[gBattleMonForms[battler]]
            .y_offset;
    } else if species > NUM_SPECIES {
        yOffset = (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
            .cast::<CArray<MonCoords, 0>>())[0]
            .y_offset;
    } else {
        yOffset = (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
            .cast::<CArray<MonCoords, 0>>())[species]
            .y_offset;
    }
    (*sprite).data[3] = 8 - (yOffset as i32 / 8) as i16;
    (*sprite).data[4] = 1;
    (*sprite).callback = Some(SpriteCB_AnimFaintOpponent);
}
pub(crate) unsafe fn SpriteCB_AnimFaintOpponent(sprite: *mut Sprite) {
    if ({
        (*sprite).data[4] -= 1;
        (*sprite).data[4]
    }) == 0
    {
        (*sprite).data[4] = 2;
        (*sprite).y2 += 8;
        if ({
            (*sprite).data[3] -= 1;
            (*sprite).data[3]
        }) < 0
        {
            FreeSpriteOamMatrix(sprite);
            DestroySprite(sprite);
        } else {
            let mut dst: *mut u8 = (*gMonSpritesGfxPtr).sprites.byte
                [GetBattlerPosition((*sprite).data[sBattler] as u8)]
            .at((gBattleMonForms[(*sprite).data[sBattler]] as i32) << 11)
            .at(((*sprite).data[3] as i32) << 8);
            for i in 0..0x100 {
                *({
                    let t3 = dst;
                    dst = dst.at(1);
                    t3
                }) = 0;
            }
            StartSpriteAnim(sprite, gBattleMonForms[(*sprite).data[sBattler]]);
        }
    }
}
pub unsafe fn SpriteCB_ShowAsMoveTarget(sprite: *mut Sprite) {
    (*sprite).data[3] = 8;
    (*sprite).data[4] = (*sprite).invisible() as i16;
    (*sprite).callback = Some(SpriteCB_BlinkVisible);
}
pub(crate) unsafe fn SpriteCB_BlinkVisible(sprite: *mut Sprite) {
    if ({
        (*sprite).data[3] -= 1;
        (*sprite).data[3]
    }) == 0
    {
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
        (*sprite).data[3] = 8;
    }
}
pub unsafe fn SpriteCB_HideAsMoveTarget(sprite: *mut Sprite) {
    (*sprite).set_invisible((*sprite).data[4] as u16);
    (*sprite).data[4] = FALSE as i16;
    (*sprite).callback = Some(SpriteCallbackDummy_2);
}
pub unsafe fn SpriteCB_OpponentMonFromBall(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        if (gHitMarker & HITMARKER_NO_ANIMATIONS == 0 || gBattleTypeFlags & 0x2000002 != 0)
            && HasTwoFramesAnimation((*sprite).data[sSpeciesId] as u16) != 0
        {
            StartSpriteAnim(sprite, 1);
        }
        BattleAnimateFrontSprite(sprite, (*sprite).data[sSpeciesId] as u16, 1, 1);
    }
}
pub unsafe fn SpriteCB_BattleSpriteStartSlideLeft(sprite: *mut Sprite) {
    (*sprite).callback = Some(SpriteCB_BattleSpriteSlideLeft);
}
pub(crate) unsafe fn SpriteCB_BattleSpriteSlideLeft(sprite: *mut Sprite) {
    if gIntroSlideFlags as i32 & 1 == 0 {
        (*sprite).x2 -= 2;
        if (*sprite).x2 == 0 {
            (*sprite).callback = Some(SpriteCB_Idle);
            (*sprite).data[1] = 0;
        }
    }
}
unsafe fn SetIdleSpriteCallback(sprite: *mut Sprite) {
    (*sprite).callback = Some(SpriteCB_Idle);
}
pub(crate) unsafe fn SpriteCB_Idle(sprite: *mut Sprite) {}
pub unsafe fn SpriteCB_FaintSlideAnim(sprite: *mut Sprite) {
    if gIntroSlideFlags as i32 & 1 == 0 {
        (*sprite).x2 += (*sprite).data[sSpeedX];
        (*sprite).y2 += (*sprite).data[sSpeedY];
    }
}
pub unsafe fn DoBounceEffect(battler: u8, which: u8, delta: i8, amplitude: i8) {
    let mut bouncerSpriteId: u8 = 0;
    match which {
        BOUNCE_MON => {
            if (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).battlerIsBouncing() != 0 {
                return;
            }
        }
        _ => {
            if (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).healthboxIsBouncing() != 0 {
                return;
            }
        }
    }
    let invisibleSpriteId: u8 = CreateInvisibleSpriteWithCallback(Some(SpriteCB_BounceEffect));
    if which == BOUNCE_HEALTHBOX {
        bouncerSpriteId = gHealthboxSpriteIds[battler];
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).healthboxBounceSpriteId =
            invisibleSpriteId;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_healthboxIsBouncing(1);
        gSprites[invisibleSpriteId].data[sSinIndex] = 128;
    } else {
        bouncerSpriteId = gBattlerSpriteIds[battler];
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).battlerBounceSpriteId =
            invisibleSpriteId;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_battlerIsBouncing(1);
        gSprites[invisibleSpriteId].data[sSinIndex] = 192;
    }
    gSprites[invisibleSpriteId].data[sDelta] = delta as i16;
    gSprites[invisibleSpriteId].data[sAmplitude] = amplitude as i16;
    gSprites[invisibleSpriteId].data[sBouncerSpriteId] = bouncerSpriteId as i16;
    gSprites[invisibleSpriteId].data[sWhich] = which as i16;
    gSprites[bouncerSpriteId].x2 = 0;
    gSprites[bouncerSpriteId].y2 = 0;
}
pub unsafe fn EndBounceEffect(battler: u8, which: u8) {
    let mut bouncerSpriteId: u8 = 0;
    if which == BOUNCE_HEALTHBOX {
        if (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).healthboxIsBouncing() == 0 {
            return;
        }
        bouncerSpriteId = gSprites
            [(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).healthboxBounceSpriteId]
            .data[sBouncerSpriteId] as u8;
        DestroySprite(
            &raw mut gSprites
                [(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).healthboxBounceSpriteId],
        );
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_healthboxIsBouncing(0);
    } else {
        if (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).battlerIsBouncing() == 0 {
            return;
        }
        bouncerSpriteId = gSprites
            [(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).battlerBounceSpriteId]
            .data[sBouncerSpriteId] as u8;
        DestroySprite(
            &raw mut gSprites
                [(*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).battlerBounceSpriteId],
        );
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_battlerIsBouncing(0);
    }
    gSprites[bouncerSpriteId].x2 = 0;
    gSprites[bouncerSpriteId].y2 = 0;
}
pub(crate) unsafe fn SpriteCB_BounceEffect(sprite: *mut Sprite) {
    let bouncerSpriteId: u8 = (*sprite).data[sBouncerSpriteId] as u8;
    let mut index: i32 = 0;
    if (*sprite).data[sWhich] == BOUNCE_HEALTHBOX as i16 {
        index = (*sprite).data[sSinIndex] as i32;
    } else {
        index = (*sprite).data[sSinIndex] as i32;
    }
    gSprites[bouncerSpriteId].y2 =
        Sin(index as i16, (*sprite).data[sAmplitude]) + (*sprite).data[sAmplitude];
    (*sprite).data[sSinIndex] = ((*sprite).data[sSinIndex] + (*sprite).data[sDelta]) & 0xFF;
}
pub unsafe fn SpriteCB_PlayerMonFromBall(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        BattleAnimateBackSprite(sprite, (*sprite).data[sSpeciesId] as u16);
    }
}
pub(crate) unsafe fn SpriteCB_TrainerThrowObject_Main(sprite: *mut Sprite) {
    AnimSetCenterToCornerVecX(sprite);
    if (*sprite).animEnded() != 0 {
        (*sprite).callback = Some(SpriteCB_Idle);
    }
}
pub unsafe fn SpriteCB_TrainerThrowObject(sprite: *mut Sprite) {
    StartSpriteAnim(sprite, 1);
    (*sprite).callback = Some(SpriteCB_TrainerThrowObject_Main);
}
pub unsafe fn AnimSetCenterToCornerVecX(sprite: *mut Sprite) {
    if (*sprite).animDelayCounter() == 0 {
        (*sprite).centerToCornerVecX = sCenterToCornerVecXs[(*sprite).animCmdIndex];
    }
}
pub unsafe fn BeginBattleIntroDummy() {}
pub unsafe fn BeginBattleIntro() {
    BattleStartClearSetData();
    gBattleCommunication[1] = 0;
    gBattleMainFunc = Some(BattleIntroGetMonsData);
}
pub(crate) unsafe fn BattleMainCB1() {
    gBattleMainFunc.unwrap_unchecked()();
    gActiveBattler = 0;
    while gActiveBattler < gBattlersCount {
        gBattlerControllerFuncs[gActiveBattler].unwrap_unchecked()();
        gActiveBattler += 1;
    }
}
unsafe fn BattleStartClearSetData() {
    let mut dataPtr: *mut u8 = null_mut();
    TurnValuesCleanUp(FALSE);
    SpecialStatusesClear();
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        gStatuses3[i] = 0;
        dataPtr = &raw mut gDisableStructs[i] as *mut u8;
        for j in 0..28u32 {
            *dataPtr.at(j) = 0;
        }
        gDisableStructs[i].isFirstTurn = 2;
        sUnusedBattlersArray[i] = 0;
        gLastMoves[i] = MOVE_NONE;
        gLastLandedMoves[i] = MOVE_NONE;
        gLastHitByType[i] = 0;
        gLastResultingMoves[i] = MOVE_NONE;
        gLastHitBy[i] = 0xFF;
        gLockedMoves[i] = MOVE_NONE;
        gLastPrintedMoves[i] = MOVE_NONE;
        (*(*gBattleResources).flags).flags[i] = 0;
        gPalaceSelectionBattleScripts[i] = null_mut();
    }
    let mut i: i32 = 0;
    while i < 2 {
        gSideStatuses[i] = 0;
        dataPtr = &raw mut gSideTimers[i] as *mut u8;
        for j in 0..12u32 {
            *dataPtr.at(j) = 0;
        }
        i += 1;
    }
    gBattlerAttacker = 0;
    gBattlerTarget = 0;
    gBattleWeather = 0;
    dataPtr = &raw mut gWishFutureKnock as *mut u8;
    for i in 0..44i32 {
        *dataPtr.at(i) = 0;
    }
    gHitMarker = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED == 0 {
        if gBattleTypeFlags & BATTLE_TYPE_LINK == 0
            && (*gSaveBlock2Ptr).optionsBattleSceneOff() == TRUE as u16
        {
            gHitMarker |= HITMARKER_NO_ANIMATIONS;
        }
    } else if gBattleTypeFlags & 0x2000002 == 0 && GetBattleSceneInRecordedBattle() != 0 {
        gHitMarker |= HITMARKER_NO_ANIMATIONS;
    }
    gBattleScripting.battleStyle = (*gSaveBlock2Ptr).optionsBattleStyle() as u8;
    gMultiHitCounter = 0;
    gBattleOutcome = 0;
    gBattleControllerExecFlags = 0;
    gPaydayMoney = 0;
    (*(*gBattleResources).battleScriptsStack).size = 0;
    (*(*gBattleResources).battleCallbackStack).size = 0;
    for i in 0..BATTLE_COMMUNICATION_ENTRIES_COUNT {
        gBattleCommunication[i] = 0;
    }
    gPauseCounterBattle = 0;
    gBattleMoveDamage = 0;
    gIntroSlideFlags = 0;
    gBattleScripting.animTurn = 0;
    gBattleScripting.animTargetsHit = 0;
    gLeveledUpInBattle = 0;
    gAbsentBattlerFlags = 0;
    (*gBattleStruct).runTries = 0;
    (*gBattleStruct).safariGoNearCounter = 0;
    (*gBattleStruct).safariPkblThrowCounter = 0;
    (*gBattleStruct).safariCatchFactor = ((*(&raw const crate::data::pokemon::gSpeciesInfo)
        .cast::<CArray<SpeciesInfo, 0>>())[GetMonData2(&raw mut gEnemyParty[0], MON_DATA_SPECIES)]
    .catchRate as i32
        * 100
        / 1275) as u8;
    (*gBattleStruct).safariEscapeFactor = 3;
    (*gBattleStruct).wildVictorySong = 0;
    (*gBattleStruct).moneyMultiplier = 1;
    for i in 0..8i32 {
        *(*gBattleStruct).lastTakenMove.as_mut_ptr().at(i) = MOVE_NONE as u8;
        *((*gBattleStruct).usedHeldItems.as_mut_ptr() as *mut u8).at(i) = ITEM_NONE as u8;
        *((*gBattleStruct).choicedMove.as_mut_ptr() as *mut u8).at(i) = MOVE_NONE as u8;
        *((*gBattleStruct).changedItems.as_mut_ptr() as *mut u8).at(i) = ITEM_NONE as u8;
        *(*gBattleStruct).lastTakenMoveFrom.as_mut_ptr().at(i) = 0;
        *(*gBattleStruct).lastTakenMoveFrom.as_mut_ptr().at(i + 8) = 0;
        *(*gBattleStruct).lastTakenMoveFrom.as_mut_ptr().at(i + 16) = 0;
        *(*gBattleStruct).lastTakenMoveFrom.as_mut_ptr().at(i + 24) = 0;
    }
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        *(*gBattleStruct).AI_monToSwitchIntoId.as_mut_ptr().at(i) = PARTY_SIZE as u8;
    }
    (*gBattleStruct).givenExpMons = 0;
    (*gBattleStruct).palaceFlags = 0;
    gRandomTurnNumber.set(Random());
    dataPtr = &raw mut gBattleResults as *mut u8;
    for i in 0..68i32 {
        *dataPtr.at(i) = 0;
    }
    gBattleResults.set_shinyWildMon(IsMonShiny(&raw mut gEnemyParty[0]));
    (*gBattleStruct).arenaLostPlayerMons = 0;
    (*gBattleStruct).arenaLostOpponentMons = 0;
}
pub unsafe fn SwitchInClearSetData() {
    let disableStructCopy: DisableStruct = gDisableStructs[gActiveBattler];
    let mut i: i32 = 0;
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [gCurrentMove]
        .effect
        != EFFECT_BATON_PASS
    {
        for i in 0..NUM_BATTLE_STATS {
            gBattleMons[gActiveBattler].statStages[i] = DEFAULT_STAT_STAGE;
        }
        for i in 0..(gBattlersCount as i32) {
            if gBattleMons[i].status2 & STATUS2_ESCAPE_PREVENTION != 0
                && gDisableStructs[i].battlerPreventingEscape == gActiveBattler
            {
                gBattleMons[i].status2 &= 0xfbffffff;
            }
            if gStatuses3[i] & STATUS3_ALWAYS_HITS != 0
                && gDisableStructs[i].battlerWithSureHit == gActiveBattler
            {
                gStatuses3[i] &= 0xffffffe7;
                gDisableStructs[i].battlerWithSureHit = 0;
            }
        }
    }
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [gCurrentMove]
        .effect
        == EFFECT_BATON_PASS
    {
        gBattleMons[gActiveBattler].status2 &= 0x15100007;
        gStatuses3[gActiveBattler] &= 0x3043f;
        i = 0;
        while i < gBattlersCount as i32 {
            if GetBattlerSide(gActiveBattler) != GetBattlerSide(i as u8)
                && gStatuses3[i] & STATUS3_ALWAYS_HITS != 0
                && gDisableStructs[i].battlerWithSureHit == gActiveBattler
            {
                gStatuses3[i] &= 0xffffffe7;
                gStatuses3[i] |= 16;
            }
            i += 1;
        }
    } else {
        gBattleMons[gActiveBattler].status2 = 0;
        gStatuses3[gActiveBattler] = 0;
    }
    i = 0;
    while i < gBattlersCount as i32 {
        if gBattleMons[i].status2
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler] << 16
            != 0
        {
            gBattleMons[i].status2 &= !(gBitTable[gActiveBattler] << 16);
        }
        if gBattleMons[i].status2 & STATUS2_WRAPPED != 0
            && *(*gBattleStruct).wrappedBy.as_mut_ptr().at(i) == gActiveBattler
        {
            gBattleMons[i].status2 &= 0xffff1fff;
        }
        i += 1;
    }
    gActionSelectionCursor[gActiveBattler] = 0;
    gMoveSelectionCursor[gActiveBattler] = 0;
    let ptr: *mut u8 = &raw mut gDisableStructs[gActiveBattler] as *mut u8;
    for i in 0..28i32 {
        *ptr.at(i) = 0;
    }
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [gCurrentMove]
        .effect
        == EFFECT_BATON_PASS
    {
        gDisableStructs[gActiveBattler].substituteHP = disableStructCopy.substituteHP;
        gDisableStructs[gActiveBattler].battlerWithSureHit = disableStructCopy.battlerWithSureHit;
        gDisableStructs[gActiveBattler].set_perishSongTimer(disableStructCopy.perishSongTimer());
        gDisableStructs[gActiveBattler]
            .set_perishSongTimerStartValue(disableStructCopy.perishSongTimerStartValue());
        gDisableStructs[gActiveBattler].battlerPreventingEscape =
            disableStructCopy.battlerPreventingEscape;
    }
    gMoveResultFlags = 0;
    gDisableStructs[gActiveBattler].isFirstTurn = 2;
    gDisableStructs[gActiveBattler].set_truantSwitchInHack(disableStructCopy.truantSwitchInHack());
    gLastMoves[gActiveBattler] = MOVE_NONE;
    gLastLandedMoves[gActiveBattler] = MOVE_NONE;
    gLastHitByType[gActiveBattler] = 0;
    gLastResultingMoves[gActiveBattler] = MOVE_NONE;
    gLastPrintedMoves[gActiveBattler] = MOVE_NONE;
    gLastHitBy[gActiveBattler] = 0xFF;
    *(*gBattleStruct)
        .lastTakenMove
        .as_mut_ptr()
        .at(gActiveBattler as i32 * 2) = 0;
    *(*gBattleStruct)
        .lastTakenMove
        .as_mut_ptr()
        .at(gActiveBattler as i32 * 2)
        .at(1) = MOVE_NONE as u8;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(gActiveBattler as i32 * 8) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(gActiveBattler as i32 * 8)
        .at(1) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(2 + gActiveBattler as i32 * 8) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(2 + gActiveBattler as i32 * 8)
        .at(1) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(4 + gActiveBattler as i32 * 8) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(4 + gActiveBattler as i32 * 8)
        .at(1) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(6 + gActiveBattler as i32 * 8) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(6 + gActiveBattler as i32 * 8)
        .at(1) = 0;
    (*gBattleStruct).palaceFlags &= !(gBitTable[gActiveBattler] as u8);
    i = 0;
    while i < gBattlersCount as i32 {
        if i != gActiveBattler as i32 && GetBattlerSide(i as u8) != GetBattlerSide(gActiveBattler) {
            *(*gBattleStruct).lastTakenMove.as_mut_ptr().at(i * 2) = 0;
            *(*gBattleStruct).lastTakenMove.as_mut_ptr().at(i * 2).at(1) = MOVE_NONE as u8;
        }
        *(*gBattleStruct)
            .lastTakenMoveFrom
            .as_mut_ptr()
            .at(i * 8 + gActiveBattler as i32 * 2) = 0;
        *(*gBattleStruct)
            .lastTakenMoveFrom
            .as_mut_ptr()
            .at(i * 8 + gActiveBattler as i32 * 2)
            .at(1) = 0;
        i += 1;
    }
    *(&raw mut (*gBattleStruct).choicedMove[gActiveBattler] as *mut u8) = 0;
    *(&raw mut (*gBattleStruct).choicedMove[gActiveBattler] as *mut u8).at(1) = MOVE_NONE as u8;
    (*(*gBattleResources).flags).flags[gActiveBattler] = 0;
    gCurrentMove = MOVE_NONE;
    (*gBattleStruct).arenaTurnCounter = 0xFF;
    ClearBattlerMoveHistory(gActiveBattler);
    ClearBattlerAbilityHistory(gActiveBattler);
}
pub unsafe fn FaintClearSetData() {
    for i in 0..NUM_BATTLE_STATS {
        gBattleMons[gActiveBattler].statStages[i] = DEFAULT_STAT_STAGE;
    }
    gBattleMons[gActiveBattler].status2 = 0;
    gStatuses3[gActiveBattler] = 0;
    let mut i: i32 = 0;
    while i < gBattlersCount as i32 {
        if gBattleMons[i].status2 & STATUS2_ESCAPE_PREVENTION != 0
            && gDisableStructs[i].battlerPreventingEscape == gActiveBattler
        {
            gBattleMons[i].status2 &= 0xfbffffff;
        }
        if gBattleMons[i].status2
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler] << 16
            != 0
        {
            gBattleMons[i].status2 &= !(gBitTable[gActiveBattler] << 16);
        }
        if gBattleMons[i].status2 & STATUS2_WRAPPED != 0
            && *(*gBattleStruct).wrappedBy.as_mut_ptr().at(i) == gActiveBattler
        {
            gBattleMons[i].status2 &= 0xffff1fff;
        }
        i += 1;
    }
    gActionSelectionCursor[gActiveBattler] = 0;
    gMoveSelectionCursor[gActiveBattler] = 0;
    let ptr: *mut u8 = &raw mut gDisableStructs[gActiveBattler] as *mut u8;
    for i in 0..28i32 {
        *ptr.at(i) = 0;
    }
    gProtectStructs[gActiveBattler].set_protected(FALSE as u32);
    gProtectStructs[gActiveBattler].set_endured(FALSE as u32);
    gProtectStructs[gActiveBattler].set_noValidMoves(FALSE as u32);
    gProtectStructs[gActiveBattler].set_helpingHand(FALSE as u32);
    gProtectStructs[gActiveBattler].set_bounceMove(FALSE as u32);
    gProtectStructs[gActiveBattler].set_stealMove(FALSE as u32);
    gProtectStructs[gActiveBattler].set_flag0Unknown(FALSE as u32);
    gProtectStructs[gActiveBattler].set_prlzImmobility(FALSE as u32);
    gProtectStructs[gActiveBattler].set_confusionSelfDmg(FALSE as u32);
    gProtectStructs[gActiveBattler].set_targetNotAffected(FALSE as u32);
    gProtectStructs[gActiveBattler].set_chargingTurn(FALSE as u32);
    gProtectStructs[gActiveBattler].set_fleeType(0);
    gProtectStructs[gActiveBattler].set_usedImprisonedMove(FALSE as u32);
    gProtectStructs[gActiveBattler].set_loveImmobility(FALSE as u32);
    gProtectStructs[gActiveBattler].set_usedDisabledMove(FALSE as u32);
    gProtectStructs[gActiveBattler].set_usedTauntedMove(FALSE as u32);
    gProtectStructs[gActiveBattler].set_flag2Unknown(FALSE as u32);
    gProtectStructs[gActiveBattler].set_flinchImmobility(FALSE as u32);
    gProtectStructs[gActiveBattler].set_notFirstStrike(FALSE as u32);
    gDisableStructs[gActiveBattler].isFirstTurn = 2;
    gLastMoves[gActiveBattler] = MOVE_NONE;
    gLastLandedMoves[gActiveBattler] = MOVE_NONE;
    gLastHitByType[gActiveBattler] = 0;
    gLastResultingMoves[gActiveBattler] = MOVE_NONE;
    gLastPrintedMoves[gActiveBattler] = MOVE_NONE;
    gLastHitBy[gActiveBattler] = 0xFF;
    *(&raw mut (*gBattleStruct).choicedMove[gActiveBattler] as *mut u8) = 0;
    *(&raw mut (*gBattleStruct).choicedMove[gActiveBattler] as *mut u8).at(1) = MOVE_NONE as u8;
    *(*gBattleStruct)
        .lastTakenMove
        .as_mut_ptr()
        .at(gActiveBattler as i32 * 2) = 0;
    *(*gBattleStruct)
        .lastTakenMove
        .as_mut_ptr()
        .at(gActiveBattler as i32 * 2)
        .at(1) = MOVE_NONE as u8;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(gActiveBattler as i32 * 8) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(gActiveBattler as i32 * 8)
        .at(1) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(2 + gActiveBattler as i32 * 8) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(2 + gActiveBattler as i32 * 8)
        .at(1) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(4 + gActiveBattler as i32 * 8) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(4 + gActiveBattler as i32 * 8)
        .at(1) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(6 + gActiveBattler as i32 * 8) = 0;
    *(*gBattleStruct)
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(6 + gActiveBattler as i32 * 8)
        .at(1) = 0;
    (*gBattleStruct).palaceFlags &= !(gBitTable[gActiveBattler] as u8);
    i = 0;
    while i < gBattlersCount as i32 {
        if i != gActiveBattler as i32 && GetBattlerSide(i as u8) != GetBattlerSide(gActiveBattler) {
            *(*gBattleStruct).lastTakenMove.as_mut_ptr().at(i * 2) = 0;
            *(*gBattleStruct).lastTakenMove.as_mut_ptr().at(i * 2).at(1) = MOVE_NONE as u8;
        }
        *(*gBattleStruct)
            .lastTakenMoveFrom
            .as_mut_ptr()
            .at(i * 8 + gActiveBattler as i32 * 2) = 0;
        *(*gBattleStruct)
            .lastTakenMoveFrom
            .as_mut_ptr()
            .at(i * 8 + gActiveBattler as i32 * 2)
            .at(1) = 0;
        i += 1;
    }
    (*(*gBattleResources).flags).flags[gActiveBattler] = 0;
    gBattleMons[gActiveBattler].types[0] = (*(&raw const crate::data::pokemon::gSpeciesInfo)
        .cast::<CArray<SpeciesInfo, 0>>())[gBattleMons[gActiveBattler].species]
        .types[0];
    gBattleMons[gActiveBattler].types[1] = (*(&raw const crate::data::pokemon::gSpeciesInfo)
        .cast::<CArray<SpeciesInfo, 0>>())[gBattleMons[gActiveBattler].species]
        .types[1];
    ClearBattlerMoveHistory(gActiveBattler);
    ClearBattlerAbilityHistory(gActiveBattler);
}
pub(crate) unsafe fn BattleIntroGetMonsData() {
    match gBattleCommunication[0] {
        0 => {
            gActiveBattler = gBattleCommunication[1];
            BtlController_EmitGetMonData(B_COMM_TO_CONTROLLER, REQUEST_ALL_BATTLE, 0);
            MarkBattlerForControllerExec(gActiveBattler);
            gBattleCommunication[0] += 1;
        }
        1 if gBattleControllerExecFlags == 0 => {
            gBattleCommunication[1] += 1;
            if gBattleCommunication[1] == gBattlersCount {
                gBattleMainFunc = Some(BattleIntroPrepareBackgroundSlide);
            } else {
                gBattleCommunication[0] = 0;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn BattleIntroPrepareBackgroundSlide() {
    if gBattleControllerExecFlags == 0 {
        gActiveBattler = GetBattlerAtPosition(0);
        BtlController_EmitIntroSlide(B_COMM_TO_CONTROLLER, gBattleEnvironment);
        MarkBattlerForControllerExec(gActiveBattler);
        gBattleMainFunc = Some(BattleIntroDrawTrainersOrMonsSprites);
        gBattleCommunication[0] = 0;
        gBattleCommunication[1] = 0;
    }
}
pub(crate) unsafe fn BattleIntroDrawTrainersOrMonsSprites() {
    let mut ptr: *mut u8 = null_mut();
    let mut i: i32 = 0;
    if gBattleControllerExecFlags != 0 {
        return;
    }
    gActiveBattler = 0;
    while gActiveBattler < gBattlersCount {
        if gBattleTypeFlags & BATTLE_TYPE_SAFARI != 0
            && GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER
        {
            ptr = &raw mut gBattleMons[gActiveBattler] as *mut u8;
            for i in 0..88i32 {
                *ptr.at(i) = 0;
            }
        } else {
            let mut hpOnSwitchout: *mut u16 = null_mut();
            ptr = &raw mut gBattleMons[gActiveBattler] as *mut u8;
            i = 0;
            while i < 88 {
                *ptr.at(i) = gBattleBufferB[gActiveBattler][4 + i];
                i += 1;
            }
            gBattleMons[gActiveBattler].types[0] =
                (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                    [gBattleMons[gActiveBattler].species]
                    .types[0];
            gBattleMons[gActiveBattler].types[1] =
                (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                    [gBattleMons[gActiveBattler].species]
                    .types[1];
            gBattleMons[gActiveBattler].ability = GetAbilityBySpecies(
                gBattleMons[gActiveBattler].species,
                gBattleMons[gActiveBattler].abilityNum() as u8,
            );
            hpOnSwitchout = &raw mut (*gBattleStruct).hpOnSwitchout[GetBattlerSide(gActiveBattler)];
            *hpOnSwitchout = gBattleMons[gActiveBattler].hp;
            for i in 0..NUM_BATTLE_STATS {
                gBattleMons[gActiveBattler].statStages[i] = DEFAULT_STAT_STAGE;
            }
            gBattleMons[gActiveBattler].status2 = 0;
        }
        if GetBattlerPosition(gActiveBattler) == B_POSITION_PLAYER_LEFT {
            BtlController_EmitDrawTrainerPic(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler);
        }
        if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
            if GetBattlerPosition(gActiveBattler) == B_POSITION_OPPONENT_LEFT {
                BtlController_EmitDrawTrainerPic(B_COMM_TO_CONTROLLER);
                MarkBattlerForControllerExec(gActiveBattler);
            }
            if GetBattlerSide(gActiveBattler) == B_SIDE_OPPONENT
                && gBattleTypeFlags & 0x63f0902 == 0
            {
                HandleSetPokedexFlag(
                    SpeciesToNationalPokedexNum(gBattleMons[gActiveBattler].species),
                    FLAG_SET_SEEN,
                    gBattleMons[gActiveBattler].personality,
                );
            }
        } else {
            if GetBattlerSide(gActiveBattler) == B_SIDE_OPPONENT {
                if gBattleTypeFlags & 0x63f0902 == 0 {
                    HandleSetPokedexFlag(
                        SpeciesToNationalPokedexNum(gBattleMons[gActiveBattler].species),
                        FLAG_SET_SEEN,
                        gBattleMons[gActiveBattler].personality,
                    );
                }
                BtlController_EmitLoadMonSprite(B_COMM_TO_CONTROLLER);
                MarkBattlerForControllerExec(gActiveBattler);
                gBattleResults.lastOpponentSpecies = GetMonData3(
                    &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
                    MON_DATA_SPECIES,
                    null_mut(),
                ) as u16;
            }
        }
        if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
            && (GetBattlerPosition(gActiveBattler) == B_POSITION_PLAYER_RIGHT
                || GetBattlerPosition(gActiveBattler) == B_POSITION_OPPONENT_RIGHT)
        {
            BtlController_EmitDrawTrainerPic(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler);
        }
        if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0
            && GetBattlerPosition(gActiveBattler) == B_POSITION_OPPONENT_RIGHT
        {
            BtlController_EmitDrawTrainerPic(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler);
        }
        if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0 {
            BattleArena_InitPoints();
        }
        gActiveBattler += 1;
    }
    gBattleMainFunc = Some(BattleIntroDrawPartySummaryScreens);
}
pub(crate) unsafe fn BattleIntroDrawPartySummaryScreens() {
    let mut i: i32 = 0;
    let mut hpStatus: CArray<HpAndStatus, 6> = zeroed();
    if gBattleControllerExecFlags != 0 {
        return;
    }
    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
        i = 0;
        while i < PARTY_SIZE {
            if GetMonData2(&raw mut gEnemyParty[i], MON_DATA_SPECIES_OR_EGG) == SPECIES_NONE as u32
                || GetMonData2(&raw mut gEnemyParty[i], MON_DATA_SPECIES_OR_EGG) == SPECIES_EGG
            {
                hpStatus[i].hp = HP_EMPTY_SLOT;
                hpStatus[i].status = 0;
            } else {
                hpStatus[i].hp = GetMonData2(&raw mut gEnemyParty[i], MON_DATA_HP) as u16;
                hpStatus[i].status = GetMonData2(&raw mut gEnemyParty[i], MON_DATA_STATUS);
            }
            i += 1;
        }
        gActiveBattler = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
        BtlController_EmitDrawPartyStatusSummary(
            B_COMM_TO_CONTROLLER,
            hpStatus.as_mut_ptr(),
            PARTY_SUMM_SKIP_DRAW_DELAY,
        );
        MarkBattlerForControllerExec(gActiveBattler);
        for i in 0..PARTY_SIZE {
            if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) == SPECIES_NONE as u32
                || GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) == SPECIES_EGG
            {
                hpStatus[i].hp = HP_EMPTY_SLOT;
                hpStatus[i].status = 0;
            } else {
                hpStatus[i].hp = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) as u16;
                hpStatus[i].status = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_STATUS);
            }
        }
        gActiveBattler = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
        BtlController_EmitDrawPartyStatusSummary(
            B_COMM_TO_CONTROLLER,
            hpStatus.as_mut_ptr(),
            PARTY_SUMM_SKIP_DRAW_DELAY,
        );
        MarkBattlerForControllerExec(gActiveBattler);
        gBattleMainFunc = Some(BattleIntroPrintTrainerWantsToBattle);
    } else {
        for i in 0..PARTY_SIZE {
            if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) == SPECIES_NONE as u32
                || GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) == SPECIES_EGG
            {
                hpStatus[i].hp = HP_EMPTY_SLOT;
                hpStatus[i].status = 0;
            } else {
                hpStatus[i].hp = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) as u16;
                hpStatus[i].status = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_STATUS);
            }
        }
        gBattleMainFunc = Some(BattleIntroPrintWildMonAttacked);
    }
}
pub(crate) unsafe fn BattleIntroPrintTrainerWantsToBattle() {
    if gBattleControllerExecFlags == 0 {
        gActiveBattler = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
        PrepareStringBattle(STRINGID_INTROMSG, gActiveBattler);
        gBattleMainFunc = Some(BattleIntroPrintOpponentSendsOut);
    }
}
pub(crate) unsafe fn BattleIntroPrintWildMonAttacked() {
    if gBattleControllerExecFlags == 0 {
        gBattleMainFunc = Some(BattleIntroPrintPlayerSendsOut);
        PrepareStringBattle(0, 0);
    }
}
pub(crate) unsafe fn BattleIntroPrintOpponentSendsOut() {
    let mut position: u32 = 0;
    if gBattleControllerExecFlags != 0 {
        return;
    }
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED == 0 {
        position = B_POSITION_OPPONENT_LEFT as u32;
    } else if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
        if gBattleTypeFlags & 0x80000000 != 0 {
            position = B_POSITION_OPPONENT_LEFT as u32;
        } else {
            position = B_POSITION_PLAYER_LEFT as u32;
        }
    } else {
        position = B_POSITION_OPPONENT_LEFT as u32;
    }
    PrepareStringBattle(STRINGID_INTROSENDOUT, GetBattlerAtPosition(position as u8));
    gBattleMainFunc = Some(BattleIntroOpponent1SendsOutMonAnimation);
}
pub(crate) unsafe fn BattleIntroOpponent2SendsOutMonAnimation() {
    let mut position: u32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED == 0 {
        position = B_POSITION_OPPONENT_RIGHT as u32;
    } else if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
        if gBattleTypeFlags & 0x80000000 != 0 {
            position = B_POSITION_OPPONENT_RIGHT as u32;
        } else {
            position = B_POSITION_PLAYER_RIGHT as u32;
        }
    } else {
        position = B_POSITION_OPPONENT_RIGHT as u32;
    }
    gActiveBattler = 0;
    while gActiveBattler < gBattlersCount {
        if GetBattlerPosition(gActiveBattler) as u32 == position {
            BtlController_EmitIntroTrainerBallThrow(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler);
        }
        gActiveBattler += 1;
    }
    gBattleMainFunc = Some(BattleIntroRecordMonsToDex);
}
pub(crate) unsafe fn BattleIntroOpponent1SendsOutMonAnimation() {
    let mut position: u32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
            if gBattleTypeFlags & 0x80000000 != 0 {
                position = B_POSITION_OPPONENT_LEFT as u32;
            } else {
                position = B_POSITION_PLAYER_LEFT as u32;
            }
        } else {
            position = B_POSITION_OPPONENT_LEFT as u32;
        }
    } else {
        position = B_POSITION_OPPONENT_LEFT as u32;
    }
    if gBattleControllerExecFlags != 0 {
        return;
    }
    gActiveBattler = 0;
    while gActiveBattler < gBattlersCount {
        if GetBattlerPosition(gActiveBattler) as u32 == position {
            BtlController_EmitIntroTrainerBallThrow(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler);
            if gBattleTypeFlags & 32832 != 0 {
                gBattleMainFunc = Some(BattleIntroOpponent2SendsOutMonAnimation);
                return;
            }
        }
        gActiveBattler += 1;
    }
    gBattleMainFunc = Some(BattleIntroRecordMonsToDex);
}
pub(crate) unsafe fn BattleIntroRecordMonsToDex() {
    if gBattleControllerExecFlags == 0 {
        gActiveBattler = 0;
        while gActiveBattler < gBattlersCount {
            if GetBattlerSide(gActiveBattler) == B_SIDE_OPPONENT
                && gBattleTypeFlags & 0x63f0902 == 0
            {
                HandleSetPokedexFlag(
                    SpeciesToNationalPokedexNum(gBattleMons[gActiveBattler].species),
                    FLAG_SET_SEEN,
                    gBattleMons[gActiveBattler].personality,
                );
            }
            gActiveBattler += 1;
        }
        gBattleMainFunc = Some(BattleIntroPrintPlayerSendsOut);
    }
}
unsafe fn BattleIntroSkipRecordMonsToDex() {
    if gBattleControllerExecFlags == 0 {
        gBattleMainFunc = Some(BattleIntroPrintPlayerSendsOut);
    }
}
pub(crate) unsafe fn BattleIntroPrintPlayerSendsOut() {
    if gBattleControllerExecFlags == 0 {
        let mut position: u8 = 0;
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED == 0 {
            position = B_POSITION_PLAYER_LEFT;
        } else if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
            if gBattleTypeFlags & 0x80000000 != 0 {
                position = B_POSITION_PLAYER_LEFT;
            } else {
                position = B_POSITION_OPPONENT_LEFT;
            }
        } else {
            position = B_POSITION_PLAYER_LEFT;
        }
        if gBattleTypeFlags & BATTLE_TYPE_SAFARI == 0 {
            PrepareStringBattle(STRINGID_INTROSENDOUT, GetBattlerAtPosition(position));
        }
        gBattleMainFunc = Some(BattleIntroPlayer1SendsOutMonAnimation);
    }
}
pub(crate) unsafe fn BattleIntroPlayer2SendsOutMonAnimation() {
    let mut position: u32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED == 0 {
        position = B_POSITION_PLAYER_RIGHT as u32;
    } else if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
        if gBattleTypeFlags & 0x80000000 != 0 {
            position = B_POSITION_PLAYER_RIGHT as u32;
        } else {
            position = B_POSITION_OPPONENT_RIGHT as u32;
        }
    } else {
        position = B_POSITION_PLAYER_RIGHT as u32;
    }
    gActiveBattler = 0;
    while gActiveBattler < gBattlersCount {
        if GetBattlerPosition(gActiveBattler) as u32 == position {
            BtlController_EmitIntroTrainerBallThrow(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler);
        }
        gActiveBattler += 1;
    }
    (*gBattleStruct).switchInAbilitiesCounter = 0;
    (*gBattleStruct).switchInItemsCounter = 0;
    (*gBattleStruct).overworldWeatherDone = FALSE;
    gBattleMainFunc = Some(TryDoEventsBeforeFirstTurn);
}
pub(crate) unsafe fn BattleIntroPlayer1SendsOutMonAnimation() {
    let mut position: u32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED == 0 {
        position = B_POSITION_PLAYER_LEFT as u32;
    } else if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
        if gBattleTypeFlags & 0x80000000 != 0 {
            position = B_POSITION_PLAYER_LEFT as u32;
        } else {
            position = B_POSITION_OPPONENT_LEFT as u32;
        }
    } else {
        position = B_POSITION_PLAYER_LEFT as u32;
    }
    if gBattleControllerExecFlags != 0 {
        return;
    }
    gActiveBattler = 0;
    while gActiveBattler < gBattlersCount {
        if GetBattlerPosition(gActiveBattler) as u32 == position {
            BtlController_EmitIntroTrainerBallThrow(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler);
            if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                gBattleMainFunc = Some(BattleIntroPlayer2SendsOutMonAnimation);
                return;
            }
        }
        gActiveBattler += 1;
    }
    (*gBattleStruct).switchInAbilitiesCounter = 0;
    (*gBattleStruct).switchInItemsCounter = 0;
    (*gBattleStruct).overworldWeatherDone = FALSE;
    gBattleMainFunc = Some(TryDoEventsBeforeFirstTurn);
}
unsafe fn BattleIntroSwitchInPlayerMons() {
    if gBattleControllerExecFlags == 0 {
        gActiveBattler = 0;
        while gActiveBattler < gBattlersCount {
            if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
                BtlController_EmitSwitchInAnim(
                    B_COMM_TO_CONTROLLER,
                    gBattlerPartyIndexes[gActiveBattler] as u8,
                    FALSE,
                );
                MarkBattlerForControllerExec(gActiveBattler);
            }
            gActiveBattler += 1;
        }
        (*gBattleStruct).switchInAbilitiesCounter = 0;
        (*gBattleStruct).switchInItemsCounter = 0;
        (*gBattleStruct).overworldWeatherDone = FALSE;
        gBattleMainFunc = Some(TryDoEventsBeforeFirstTurn);
    }
}
pub(crate) unsafe fn TryDoEventsBeforeFirstTurn() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut effect: u8 = 0;
    if gBattleControllerExecFlags != 0 {
        return;
    }
    if (*gBattleStruct).switchInAbilitiesCounter == 0 {
        for i in 0..(gBattlersCount as i32) {
            gBattlerByTurnOrder[i] = i as u8;
        }
        i = 0;
        while i < gBattlersCount as i32 - 1 {
            j = i + 1;
            while j < gBattlersCount as i32 {
                if GetWhoStrikesFirst(gBattlerByTurnOrder[i], gBattlerByTurnOrder[j], TRUE) != 0 {
                    SwapTurnOrder(i as u8, j as u8);
                }
                j += 1;
            }
            i += 1;
        }
    }
    if (*gBattleStruct).overworldWeatherDone == 0
        && AbilityBattleEffects(0, 0, 0, ABILITYEFFECT_SWITCH_IN_WEATHER, 0) != 0
    {
        (*gBattleStruct).overworldWeatherDone = TRUE;
        return;
    }
    while (*gBattleStruct).switchInAbilitiesCounter < gBattlersCount {
        if AbilityBattleEffects(
            0,
            gBattlerByTurnOrder[(*gBattleStruct).switchInAbilitiesCounter],
            0,
            0,
            0,
        ) != 0
        {
            effect += 1;
        }
        (*gBattleStruct).switchInAbilitiesCounter += 1;
        if effect != 0 {
            return;
        }
    }
    if AbilityBattleEffects(ABILITYEFFECT_INTIMIDATE1, 0, 0, 0, 0) != 0 {
        return;
    }
    if AbilityBattleEffects(ABILITYEFFECT_TRACE, 0, 0, 0, 0) != 0 {
        return;
    }
    while (*gBattleStruct).switchInItemsCounter < gBattlersCount {
        if ItemBattleEffects(
            0,
            gBattlerByTurnOrder[(*gBattleStruct).switchInItemsCounter],
            0,
        ) != 0
        {
            effect += 1;
        }
        (*gBattleStruct).switchInItemsCounter += 1;
        if effect != 0 {
            return;
        }
    }
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        *(*gBattleStruct).monToSwitchIntoId.as_mut_ptr().at(i) = PARTY_SIZE as u8;
        gChosenActionByBattler[i] = B_ACTION_NONE;
        gChosenMoveByBattler[i] = MOVE_NONE;
    }
    TurnValuesCleanUp(FALSE);
    SpecialStatusesClear();
    (*gBattleStruct).absentBattlerFlags = gAbsentBattlerFlags;
    BattlePutTextOnWindow(
        (*(&raw const crate::data::battle_message::gText_EmptyString3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        B_WIN_MSG,
    );
    gBattleMainFunc = Some(HandleTurnActionSelectionState);
    ResetSentPokesToOpponentValue();
    for i in 0..BATTLE_COMMUNICATION_ENTRIES_COUNT {
        gBattleCommunication[i] = 0;
    }
    for i in 0..(gBattlersCount as i32) {
        gBattleMons[i].status2 &= 0xfffffff7;
    }
    (*gBattleStruct).turnEffectsTracker = 0;
    (*gBattleStruct).turnEffectsBattlerId = 0;
    (*gBattleStruct).wishPerishSongState = 0;
    (*gBattleStruct).wishPerishSongBattlerId = 0;
    gBattleScripting.moveendState = 0;
    (*gBattleStruct).faintedActionsState = 0;
    (*gBattleStruct).turnCountersTracker = 0;
    gMoveResultFlags = 0;
    gRandomTurnNumber.set(Random());
    if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0 {
        StopCryAndClearCrySongs();
        BattleScriptExecute(
            (*crate::asmdata::BattleScript_ArenaTurnBeginning.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe fn HandleEndTurn_ContinueBattle() {
    let mut i: i32 = 0;
    if gBattleControllerExecFlags == 0 {
        gBattleMainFunc = Some(BattleTurnPassed);
        for i in 0..BATTLE_COMMUNICATION_ENTRIES_COUNT {
            gBattleCommunication[i] = 0;
        }
        i = 0;
        while i < gBattlersCount as i32 {
            gBattleMons[i].status2 &= 0xfffffff7;
            if gBattleMons[i].status1 & STATUS1_SLEEP != 0
                && gBattleMons[i].status2 & STATUS2_MULTIPLETURNS != 0
            {
                CancelMultiTurnMoves(i as u8);
            }
            i += 1;
        }
        (*gBattleStruct).turnEffectsTracker = 0;
        (*gBattleStruct).turnEffectsBattlerId = 0;
        (*gBattleStruct).wishPerishSongState = 0;
        (*gBattleStruct).wishPerishSongBattlerId = 0;
        (*gBattleStruct).turnCountersTracker = 0;
        gMoveResultFlags = 0;
    }
}
pub unsafe fn BattleTurnPassed() {
    TurnValuesCleanUp(TRUE);
    if gBattleOutcome == 0 {
        if DoFieldEndTurnEffects() != 0 {
            return;
        }
        if DoBattlerEndTurnEffects() != 0 {
            return;
        }
    }
    if HandleFaintedMonActions() != 0 {
        return;
    }
    (*gBattleStruct).faintedActionsState = 0;
    if HandleWishPerishSongOnTurnEnd() != 0 {
        return;
    }
    TurnValuesCleanUp(FALSE);
    gHitMarker &= 0xfffffdff;
    gHitMarker &= 0xfff7ffff;
    gHitMarker &= 0xffbfffff;
    gHitMarker &= 0xffefffff;
    gBattleScripting.animTurn = 0;
    gBattleScripting.animTargetsHit = 0;
    gBattleScripting.moveendState = 0;
    gBattleMoveDamage = 0;
    gMoveResultFlags = 0;
    for i in 0..5i32 {
        gBattleCommunication[i] = 0;
    }
    if gBattleOutcome != 0 {
        gCurrentActionFuncId = B_ACTION_FINISHED;
        gBattleMainFunc = Some(RunTurnActionsFunctions);
        return;
    }
    if gBattleResults.battleTurnCounter < 0xFF {
        gBattleResults.battleTurnCounter += 1;
        (*gBattleStruct).arenaTurnCounter += 1;
    }
    let mut i: i32 = 0;
    while i < gBattlersCount as i32 {
        gChosenActionByBattler[i] = B_ACTION_NONE;
        gChosenMoveByBattler[i] = MOVE_NONE;
        i += 1;
    }
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        *(*gBattleStruct).monToSwitchIntoId.as_mut_ptr().at(i) = PARTY_SIZE as u8;
    }
    (*gBattleStruct).absentBattlerFlags = gAbsentBattlerFlags;
    BattlePutTextOnWindow(
        (*(&raw const crate::data::battle_message::gText_EmptyString3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        B_WIN_MSG,
    );
    gBattleMainFunc = Some(HandleTurnActionSelectionState);
    gRandomTurnNumber.set(Random());
    if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0 {
        BattleScriptExecute(
            (*crate::asmdata::BattleScript_PalacePrintFlavorText.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else if gBattleTypeFlags & BATTLE_TYPE_ARENA != 0 && (*gBattleStruct).arenaTurnCounter == 0 {
        BattleScriptExecute(
            (*crate::asmdata::BattleScript_ArenaTurnBeginning.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
}
pub unsafe fn IsRunningFromBattleImpossible() -> u8 {
    let mut holdEffect: u8 = 0;
    if gBattleMons[gActiveBattler].item == ITEM_ENIGMA_BERRY {
        holdEffect = gEnigmaBerries[gActiveBattler].holdEffect;
    } else {
        holdEffect = GetItemHoldEffect(gBattleMons[gActiveBattler].item);
    }
    gPotentialItemEffectBattler = gActiveBattler;
    if holdEffect == HOLD_EFFECT_CAN_ALWAYS_RUN {
        return BATTLE_RUN_SUCCESS;
    }
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        return BATTLE_RUN_SUCCESS;
    }
    if gBattleMons[gActiveBattler].ability == ABILITY_RUN_AWAY {
        return BATTLE_RUN_SUCCESS;
    }
    let side: u8 = GetBattlerSide(gActiveBattler);
    let mut i: i32 = 0;
    while i < gBattlersCount as i32 {
        if side != GetBattlerSide(i as u8) && gBattleMons[i].ability == ABILITY_SHADOW_TAG {
            gBattleScripting.battler = i as u8;
            gLastUsedAbility = gBattleMons[i].ability;
            gBattleCommunication[5] = B_MSG_PREVENTS_ESCAPE;
            return BATTLE_RUN_FAILURE;
        }
        if side != GetBattlerSide(i as u8)
            && gBattleMons[gActiveBattler].ability != ABILITY_LEVITATE
            && !(gBattleMons[gActiveBattler].types[0] == TYPE_FLYING
                || gBattleMons[gActiveBattler].types[1] == TYPE_FLYING)
            && gBattleMons[i].ability == ABILITY_ARENA_TRAP
        {
            gBattleScripting.battler = i as u8;
            gLastUsedAbility = gBattleMons[i].ability;
            gBattleCommunication[5] = B_MSG_PREVENTS_ESCAPE;
            return BATTLE_RUN_FAILURE;
        }
        i += 1;
    }
    i = AbilityBattleEffects(
        ABILITYEFFECT_CHECK_FIELD_EXCEPT_BATTLER,
        gActiveBattler,
        ABILITY_MAGNET_PULL,
        0,
        0,
    ) as i32;
    if i != 0
        && (gBattleMons[gActiveBattler].types[0] == TYPE_STEEL
            || gBattleMons[gActiveBattler].types[1] == TYPE_STEEL)
    {
        gBattleScripting.battler = i as u8 - 1;
        gLastUsedAbility = gBattleMons[i - 1].ability;
        gBattleCommunication[5] = B_MSG_PREVENTS_ESCAPE;
        return BATTLE_RUN_FAILURE;
    }
    if gBattleMons[gActiveBattler].status2 & 0x400e000 != 0
        || gStatuses3[gActiveBattler] & STATUS3_ROOTED != 0
    {
        gBattleCommunication[5] = B_MSG_CANT_ESCAPE;
        return BATTLE_RUN_FORBIDDEN;
    }
    if gBattleTypeFlags & BATTLE_TYPE_FIRST_BATTLE != 0 {
        gBattleCommunication[5] = B_MSG_DONT_LEAVE_BIRCH;
        return BATTLE_RUN_FORBIDDEN;
    }
    BATTLE_RUN_SUCCESS
}
pub unsafe fn SwitchPartyOrder(battler: u8) {
    let mut i: i32 = 0;
    while i < 3 {
        gBattlePartyCurrentOrder[i] = *((*gBattleStruct).battlerPartyOrders.as_mut_ptr()
            as *mut u8)
            .at(battler as i32 * 3 + i);
        i += 1;
    }
    let partyId1: u8 = GetPartyIdFromBattlePartyId(gBattlerPartyIndexes[battler] as u8);
    let partyId2: u8 =
        GetPartyIdFromBattlePartyId(*(*gBattleStruct).monToSwitchIntoId.as_mut_ptr().at(battler));
    SwitchPartyMonSlots(partyId1, partyId2);
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
        for i in 0..3i32 {
            *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
                .at(battler as i32 * 3 + i) = gBattlePartyCurrentOrder[i];
            *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
                .at((battler as i32 ^ 2) * 3 + i) = gBattlePartyCurrentOrder[i];
        }
    } else {
        for i in 0..3i32 {
            *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
                .at(battler as i32 * 3 + i) = gBattlePartyCurrentOrder[i];
        }
    }
}
pub(crate) unsafe fn HandleTurnActionSelectionState() {
    let mut i: i32 = 0;
    gBattleCommunication[4] = 0;
    gActiveBattler = 0;
    while gActiveBattler < gBattlersCount {
        let position: u8 = GetBattlerPosition(gActiveBattler);
        match gBattleCommunication[gActiveBattler] {
            STATE_TURN_START_RECORD => {
                RecordedBattle_CopyBattlerMoves();
                gBattleCommunication[gActiveBattler] = STATE_BEFORE_ACTION_CHOSEN;
            }
            STATE_BEFORE_ACTION_CHOSEN => {
                *(*gBattleStruct)
                    .monToSwitchIntoId
                    .as_mut_ptr()
                    .at(gActiveBattler) = PARTY_SIZE as u8;
                if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
                    || position as i32 & BIT_FLANK as i32 == B_FLANK_LEFT
                    || (*gBattleStruct).absentBattlerFlags as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                            [GetBattlerAtPosition(position ^ 2)]
                        != 0
                    || gBattleCommunication[GetBattlerAtPosition(position ^ 2)]
                        == STATE_WAIT_ACTION_CONFIRMED
                {
                    if (*gBattleStruct).absentBattlerFlags as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                            [gActiveBattler]
                        != 0
                    {
                        gChosenActionByBattler[gActiveBattler] = B_ACTION_NOTHING_FAINTED;
                        if gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
                            gBattleCommunication[gActiveBattler] = STATE_WAIT_ACTION_CONFIRMED;
                        } else {
                            gBattleCommunication[gActiveBattler] =
                                STATE_WAIT_ACTION_CONFIRMED_STANDBY;
                        }
                    } else {
                        if gBattleMons[gActiveBattler].status2 & STATUS2_MULTIPLETURNS != 0
                            || gBattleMons[gActiveBattler].status2 & STATUS2_RECHARGE != 0
                        {
                            gChosenActionByBattler[gActiveBattler] = B_ACTION_USE_MOVE;
                            gBattleCommunication[gActiveBattler] =
                                STATE_WAIT_ACTION_CONFIRMED_STANDBY;
                        } else {
                            BtlController_EmitChooseAction(
                                B_COMM_TO_CONTROLLER,
                                gChosenActionByBattler[0],
                                gBattleBufferB[0][1] as u16 | (gBattleBufferB[0][2] as u16) << 8,
                            );
                            MarkBattlerForControllerExec(gActiveBattler);
                            gBattleCommunication[gActiveBattler] += 1;
                        }
                    }
                }
            }
            STATE_WAIT_ACTION_CHOSEN => {
                if gBattleControllerExecFlags
                    & (gBitTable[gActiveBattler]
                        | 0xf0000000
                        | gBitTable[gActiveBattler] << 4
                        | gBitTable[gActiveBattler] << 8
                        | gBitTable[gActiveBattler] << 12)
                    == 0
                {
                    RecordedBattle_SetBattlerAction(
                        gActiveBattler,
                        gBattleBufferB[gActiveBattler][1],
                    );
                    gChosenActionByBattler[gActiveBattler] = gBattleBufferB[gActiveBattler][1];
                    match gBattleBufferB[gActiveBattler][1] {
                        B_ACTION_USE_MOVE => {
                            if AreAllMovesUnusable() != 0 {
                                gBattleCommunication[gActiveBattler] = STATE_SELECTION_SCRIPT;
                                *(*gBattleStruct)
                                    .selectionScriptFinished
                                    .as_mut_ptr()
                                    .at(gActiveBattler) = FALSE;
                                *(*gBattleStruct)
                                    .stateIdAfterSelScript
                                    .as_mut_ptr()
                                    .at(gActiveBattler) = STATE_WAIT_ACTION_CONFIRMED_STANDBY;
                                *(*gBattleStruct).moveTarget.as_mut_ptr().at(gActiveBattler) =
                                    gBattleBufferB[gActiveBattler][3];
                                return;
                            } else if gDisableStructs[gActiveBattler].encoredMove != 0 {
                                gChosenMoveByBattler[gActiveBattler] =
                                    gDisableStructs[gActiveBattler].encoredMove;
                                *(*gBattleStruct)
                                    .chosenMovePositions
                                    .as_mut_ptr()
                                    .at(gActiveBattler) =
                                    gDisableStructs[gActiveBattler].encoredMovePos;
                                gBattleCommunication[gActiveBattler] =
                                    STATE_WAIT_ACTION_CONFIRMED_STANDBY;
                                return;
                            } else {
                                let mut moveInfo: ChooseMoveStruct = zeroed();
                                moveInfo.species = gBattleMons[gActiveBattler].species;
                                moveInfo.monTypes[0] = gBattleMons[gActiveBattler].types[0];
                                moveInfo.monTypes[1] = gBattleMons[gActiveBattler].types[1];
                                for i in 0..MAX_MON_MOVES {
                                    moveInfo.moves[i] = gBattleMons[gActiveBattler].moves[i];
                                    moveInfo.currentPP[i] = gBattleMons[gActiveBattler].pp[i];
                                    moveInfo.maxPP[i] = CalculatePPWithBonus(
                                        gBattleMons[gActiveBattler].moves[i],
                                        gBattleMons[gActiveBattler].ppBonuses,
                                        i as u8,
                                    );
                                }
                                BtlController_EmitChooseMove(
                                    B_COMM_TO_CONTROLLER,
                                    (gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0) as u8,
                                    0,
                                    &raw mut moveInfo,
                                );
                                MarkBattlerForControllerExec(gActiveBattler);
                            }
                        }
                        B_ACTION_USE_ITEM => {
                            if gBattleTypeFlags & 0x21f0902 != 0 {
                                RecordedBattle_ClearBattlerAction(gActiveBattler, 1);
                                gSelectionBattleScripts[gActiveBattler] =
                                    (*crate::asmdata::BattleScript_ActionSelectionItemsCantBeUsed
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut();
                                gBattleCommunication[gActiveBattler] = STATE_SELECTION_SCRIPT;
                                *(*gBattleStruct)
                                    .selectionScriptFinished
                                    .as_mut_ptr()
                                    .at(gActiveBattler) = FALSE;
                                *(*gBattleStruct)
                                    .stateIdAfterSelScript
                                    .as_mut_ptr()
                                    .at(gActiveBattler) = STATE_BEFORE_ACTION_CHOSEN;
                                return;
                            } else {
                                BtlController_EmitChooseItem(
                                    B_COMM_TO_CONTROLLER,
                                    (*gBattleStruct).battlerPartyOrders[gActiveBattler]
                                        .as_mut_ptr(),
                                );
                                MarkBattlerForControllerExec(gActiveBattler);
                            }
                        }
                        B_ACTION_SWITCH => {
                            *(*gBattleStruct)
                                .battlerPartyIndexes
                                .as_mut_ptr()
                                .at(gActiveBattler) = gBattlerPartyIndexes[gActiveBattler] as u8;
                            if gBattleMons[gActiveBattler].status2 & 0x400e000 != 0
                                || gBattleTypeFlags & BATTLE_TYPE_ARENA != 0
                                || gStatuses3[gActiveBattler] & STATUS3_ROOTED != 0
                            {
                                BtlController_EmitChoosePokemon(
                                    B_COMM_TO_CONTROLLER,
                                    PARTY_ACTION_CANT_SWITCH,
                                    PARTY_SIZE as u8,
                                    ABILITY_NONE,
                                    (*gBattleStruct).battlerPartyOrders[gActiveBattler]
                                        .as_mut_ptr(),
                                );
                            } else if ({
                                i = AbilityBattleEffects(
                                    12,
                                    gActiveBattler,
                                    ABILITY_SHADOW_TAG,
                                    0,
                                    0,
                                ) as i32;
                                i
                            }) != 0
                                || ({
                                    i = AbilityBattleEffects(
                                        12,
                                        gActiveBattler,
                                        ABILITY_ARENA_TRAP,
                                        0,
                                        0,
                                    ) as i32;
                                    i
                                }) != 0
                                    && !(gBattleMons[gActiveBattler].types[0] == TYPE_FLYING
                                        || gBattleMons[gActiveBattler].types[1] == TYPE_FLYING)
                                    && gBattleMons[gActiveBattler].ability != ABILITY_LEVITATE
                                || ({
                                    i = AbilityBattleEffects(
                                        ABILITYEFFECT_CHECK_FIELD_EXCEPT_BATTLER,
                                        gActiveBattler,
                                        ABILITY_MAGNET_PULL,
                                        0,
                                        0,
                                    ) as i32;
                                    i
                                }) != 0
                                    && (gBattleMons[gActiveBattler].types[0] == TYPE_STEEL
                                        || gBattleMons[gActiveBattler].types[1] == TYPE_STEEL)
                            {
                                BtlController_EmitChoosePokemon(
                                    B_COMM_TO_CONTROLLER,
                                    (i as u8 - 1) << 4 | 4,
                                    PARTY_SIZE as u8,
                                    gLastUsedAbility,
                                    (*gBattleStruct).battlerPartyOrders[gActiveBattler]
                                        .as_mut_ptr(),
                                );
                            } else {
                                if gActiveBattler == 2 && gChosenActionByBattler[0] == 2 {
                                    BtlController_EmitChoosePokemon(
                                        B_COMM_TO_CONTROLLER,
                                        0,
                                        *(*gBattleStruct).monToSwitchIntoId.as_mut_ptr(),
                                        0,
                                        (*gBattleStruct).battlerPartyOrders[gActiveBattler]
                                            .as_mut_ptr(),
                                    );
                                } else if gActiveBattler == 3
                                    && gChosenActionByBattler[1] == B_ACTION_SWITCH
                                {
                                    BtlController_EmitChoosePokemon(
                                        B_COMM_TO_CONTROLLER,
                                        0,
                                        *(*gBattleStruct).monToSwitchIntoId.as_mut_ptr().at(1),
                                        0,
                                        (*gBattleStruct).battlerPartyOrders[gActiveBattler]
                                            .as_mut_ptr(),
                                    );
                                } else {
                                    BtlController_EmitChoosePokemon(
                                        B_COMM_TO_CONTROLLER,
                                        0,
                                        PARTY_SIZE as u8,
                                        0,
                                        (*gBattleStruct).battlerPartyOrders[gActiveBattler]
                                            .as_mut_ptr(),
                                    );
                                }
                            }
                            MarkBattlerForControllerExec(gActiveBattler);
                        }
                        B_ACTION_SAFARI_BALL => {
                            if IsPlayerPartyAndPokemonStorageFull() != 0 {
                                gSelectionBattleScripts[gActiveBattler] =
                                    (*crate::asmdata::BattleScript_PrintFullBox
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut();
                                gBattleCommunication[gActiveBattler] = STATE_SELECTION_SCRIPT;
                                *(*gBattleStruct)
                                    .selectionScriptFinished
                                    .as_mut_ptr()
                                    .at(gActiveBattler) = FALSE;
                                *(*gBattleStruct)
                                    .stateIdAfterSelScript
                                    .as_mut_ptr()
                                    .at(gActiveBattler) = STATE_BEFORE_ACTION_CHOSEN;
                                return;
                            }
                        }
                        B_ACTION_SAFARI_POKEBLOCK => {
                            BtlController_EmitChooseItem(
                                B_COMM_TO_CONTROLLER,
                                (*gBattleStruct).battlerPartyOrders[gActiveBattler].as_mut_ptr(),
                            );
                            MarkBattlerForControllerExec(gActiveBattler);
                        }
                        B_ACTION_CANCEL_PARTNER => {
                            gBattleCommunication[gActiveBattler] = STATE_WAIT_SET_BEFORE_ACTION;
                            gBattleCommunication
                                [GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2)] =
                                STATE_BEFORE_ACTION_CHOSEN;
                            RecordedBattle_ClearBattlerAction(gActiveBattler, 1);
                            if gBattleMons
                                [GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2)]
                            .status2
                                & STATUS2_MULTIPLETURNS
                                != 0
                                || gBattleMons
                                    [GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2)]
                                .status2
                                    & STATUS2_RECHARGE
                                    != 0
                            {
                                BtlController_EmitEndBounceEffect(B_COMM_TO_CONTROLLER);
                                MarkBattlerForControllerExec(gActiveBattler);
                                return;
                            } else if gChosenActionByBattler[GetBattlerAtPosition(
                                GetBattlerPosition(gActiveBattler) ^ B_ACTION_SWITCH,
                            )] == B_ACTION_SWITCH
                            {
                                RecordedBattle_ClearBattlerAction(
                                    GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2),
                                    2,
                                );
                            } else if gChosenActionByBattler
                                [GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2)]
                                == B_ACTION_RUN
                            {
                                RecordedBattle_ClearBattlerAction(
                                    GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2),
                                    1,
                                );
                            } else if gChosenActionByBattler
                                [GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2)]
                                == B_ACTION_USE_MOVE
                                && (gProtectStructs
                                    [GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2)]
                                .noValidMoves()
                                    != 0
                                    || gDisableStructs[GetBattlerAtPosition(
                                        GetBattlerPosition(gActiveBattler) ^ 2,
                                    )]
                                    .encoredMove
                                        != 0)
                            {
                                RecordedBattle_ClearBattlerAction(
                                    GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2),
                                    1,
                                );
                            } else if gBattleTypeFlags & BATTLE_TYPE_PALACE != 0
                                && gChosenActionByBattler
                                    [GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2)]
                                    == B_ACTION_USE_MOVE
                            {
                                (*crate::random::gRngValue.as_ptr().cast::<u32>()) =
                                    gBattlePalaceMoveSelectionRngValue;
                                RecordedBattle_ClearBattlerAction(
                                    GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2),
                                    1,
                                );
                            } else {
                                RecordedBattle_ClearBattlerAction(
                                    GetBattlerAtPosition(GetBattlerPosition(gActiveBattler) ^ 2),
                                    3,
                                );
                            }
                            BtlController_EmitEndBounceEffect(B_COMM_TO_CONTROLLER);
                            MarkBattlerForControllerExec(gActiveBattler);
                            return;
                        }
                        _ => {}
                    }
                    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0
                        && gBattleTypeFlags & 0x43f0100 != 0
                        && gBattleBufferB[gActiveBattler][1] == B_ACTION_RUN
                    {
                        gSelectionBattleScripts[gActiveBattler] =
                            (*crate::asmdata::BattleScript_AskIfWantsToForfeitMatch
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        gBattleCommunication[gActiveBattler] = STATE_SELECTION_SCRIPT_MAY_RUN;
                        *(*gBattleStruct)
                            .selectionScriptFinished
                            .as_mut_ptr()
                            .at(gActiveBattler) = FALSE;
                        *(*gBattleStruct)
                            .stateIdAfterSelScript
                            .as_mut_ptr()
                            .at(gActiveBattler) = STATE_BEFORE_ACTION_CHOSEN;
                        return;
                    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0
                        && gBattleTypeFlags & 0x2000002 == 0
                        && gBattleBufferB[gActiveBattler][1] == B_ACTION_RUN
                    {
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_PrintCantRunFromTrainer
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        gBattleCommunication[gActiveBattler] = STATE_BEFORE_ACTION_CHOSEN;
                    } else if IsRunningFromBattleImpossible() != BATTLE_RUN_SUCCESS
                        && gBattleBufferB[gActiveBattler][1] == B_ACTION_RUN
                    {
                        gSelectionBattleScripts[gActiveBattler] =
                            (*crate::asmdata::BattleScript_PrintCantEscapeFromBattle
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                        gBattleCommunication[gActiveBattler] = STATE_SELECTION_SCRIPT;
                        *(*gBattleStruct)
                            .selectionScriptFinished
                            .as_mut_ptr()
                            .at(gActiveBattler) = FALSE;
                        *(*gBattleStruct)
                            .stateIdAfterSelScript
                            .as_mut_ptr()
                            .at(gActiveBattler) = STATE_BEFORE_ACTION_CHOSEN;
                        return;
                    } else {
                        gBattleCommunication[gActiveBattler] += 1;
                    }
                }
            }
            STATE_WAIT_ACTION_CASE_CHOSEN => {
                if gBattleControllerExecFlags
                    & (gBitTable[gActiveBattler]
                        | 0xf0000000
                        | gBitTable[gActiveBattler] << 4
                        | gBitTable[gActiveBattler] << 8
                        | gBitTable[gActiveBattler] << 12)
                    == 0
                {
                    match gChosenActionByBattler[gActiveBattler] {
                        B_ACTION_USE_MOVE => match gBattleBufferB[gActiveBattler][1] {
                            3..=9 => {
                                gChosenActionByBattler[gActiveBattler] =
                                    gBattleBufferB[gActiveBattler][1];
                                return;
                            }
                            15 => {
                                gChosenActionByBattler[gActiveBattler] = B_ACTION_SWITCH;
                                UpdateBattlerPartyOrdersOnSwitch();
                                return;
                            }
                            _ => {
                                RecordedBattle_CheckMovesetChanges(B_RECORD_MODE_PLAYBACK);
                                if gBattleBufferB[gActiveBattler][2] as i32
                                    | (gBattleBufferB[gActiveBattler][3] as i32) << 8
                                    == 0xFFFF
                                {
                                    gBattleCommunication[gActiveBattler] =
                                        STATE_BEFORE_ACTION_CHOSEN;
                                    RecordedBattle_ClearBattlerAction(gActiveBattler, 1);
                                } else if TrySetCantSelectMoveBattleScript() != 0 {
                                    RecordedBattle_ClearBattlerAction(gActiveBattler, 1);
                                    gBattleCommunication[gActiveBattler] = STATE_SELECTION_SCRIPT;
                                    *(*gBattleStruct)
                                        .selectionScriptFinished
                                        .as_mut_ptr()
                                        .at(gActiveBattler) = FALSE;
                                    gBattleBufferB[gActiveBattler][1] = B_ACTION_USE_MOVE;
                                    *(*gBattleStruct)
                                        .stateIdAfterSelScript
                                        .as_mut_ptr()
                                        .at(gActiveBattler) = STATE_WAIT_ACTION_CHOSEN;
                                    return;
                                } else {
                                    if gBattleTypeFlags & BATTLE_TYPE_PALACE == 0 {
                                        RecordedBattle_SetBattlerAction(
                                            gActiveBattler,
                                            gBattleBufferB[gActiveBattler][2],
                                        );
                                        RecordedBattle_SetBattlerAction(
                                            gActiveBattler,
                                            gBattleBufferB[gActiveBattler][3],
                                        );
                                    }
                                    *(*gBattleStruct)
                                        .chosenMovePositions
                                        .as_mut_ptr()
                                        .at(gActiveBattler) = gBattleBufferB[gActiveBattler][2];
                                    gChosenMoveByBattler[gActiveBattler] =
                                        gBattleMons[gActiveBattler].moves[*(*gBattleStruct)
                                            .chosenMovePositions
                                            .as_mut_ptr()
                                            .at(gActiveBattler)];
                                    *(*gBattleStruct).moveTarget.as_mut_ptr().at(gActiveBattler) =
                                        gBattleBufferB[gActiveBattler][3];
                                    gBattleCommunication[gActiveBattler] += 1;
                                }
                            }
                        },
                        B_ACTION_USE_ITEM => {
                            if gBattleBufferB[gActiveBattler][1] as i32
                                | (gBattleBufferB[gActiveBattler][2] as i32) << 8
                                == 0
                            {
                                gBattleCommunication[gActiveBattler] = STATE_BEFORE_ACTION_CHOSEN;
                            } else {
                                gLastUsedItem = gBattleBufferB[gActiveBattler][1] as u16
                                    | (gBattleBufferB[gActiveBattler][2] as u16) << 8;
                                gBattleCommunication[gActiveBattler] += 1;
                            }
                        }
                        B_ACTION_SWITCH => {
                            if gBattleBufferB[gActiveBattler][1] == PARTY_SIZE as u8 {
                                gBattleCommunication[gActiveBattler] = STATE_BEFORE_ACTION_CHOSEN;
                                RecordedBattle_ClearBattlerAction(gActiveBattler, 1);
                            } else {
                                UpdateBattlerPartyOrdersOnSwitch();
                                gBattleCommunication[gActiveBattler] += 1;
                            }
                        }
                        B_ACTION_RUN => {
                            gHitMarker |= HITMARKER_RUN;
                            gBattleCommunication[gActiveBattler] += 1;
                        }
                        B_ACTION_SAFARI_WATCH_CAREFULLY => {
                            gBattleCommunication[gActiveBattler] += 1;
                        }
                        B_ACTION_SAFARI_BALL => {
                            gBattleCommunication[gActiveBattler] += 1;
                        }
                        B_ACTION_SAFARI_POKEBLOCK => {
                            if gBattleBufferB[gActiveBattler][1] as i32
                                | (gBattleBufferB[gActiveBattler][2] as i32) << 8
                                != 0
                            {
                                gBattleCommunication[gActiveBattler] += 1;
                            } else {
                                gBattleCommunication[gActiveBattler] = STATE_BEFORE_ACTION_CHOSEN;
                            }
                        }
                        B_ACTION_SAFARI_GO_NEAR => {
                            gBattleCommunication[gActiveBattler] += 1;
                        }
                        B_ACTION_SAFARI_RUN => {
                            gHitMarker |= HITMARKER_RUN;
                            gBattleCommunication[gActiveBattler] += 1;
                        }
                        B_ACTION_WALLY_THROW => {
                            gBattleCommunication[gActiveBattler] += 1;
                        }
                        _ => {}
                    }
                }
            }
            STATE_WAIT_ACTION_CONFIRMED_STANDBY => {
                if gBattleControllerExecFlags
                    & (gBitTable[gActiveBattler]
                        | 0xf0000000
                        | gBitTable[gActiveBattler] << 4
                        | gBitTable[gActiveBattler] << 8
                        | gBitTable[gActiveBattler] << 12)
                    == 0
                {
                    if AllAtActionConfirmed() != 0 {
                        i = TRUE as i32;
                    } else {
                        i = FALSE as i32;
                    }
                    if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
                        || gBattleTypeFlags & BATTLE_TYPE_DOUBLE == 0
                        || position as i32 & BIT_FLANK as i32 != B_FLANK_LEFT
                        || (*gBattleStruct).absentBattlerFlags as u32
                            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                                [GetBattlerAtPosition(position ^ 2)]
                            != 0
                    {
                        BtlController_EmitLinkStandbyMsg(
                            B_COMM_TO_CONTROLLER,
                            LINK_STANDBY_MSG_STOP_BOUNCE,
                            i as u32,
                        );
                    } else {
                        BtlController_EmitLinkStandbyMsg(
                            B_COMM_TO_CONTROLLER,
                            LINK_STANDBY_STOP_BOUNCE_ONLY,
                            i as u32,
                        );
                    }
                    MarkBattlerForControllerExec(gActiveBattler);
                    gBattleCommunication[gActiveBattler] += 1;
                }
            }
            STATE_WAIT_ACTION_CONFIRMED => {
                if gBattleControllerExecFlags
                    & (gBitTable[gActiveBattler]
                        | 0xf0000000
                        | gBitTable[gActiveBattler] << 4
                        | gBitTable[gActiveBattler] << 8
                        | gBitTable[gActiveBattler] << 12)
                    == 0
                {
                    gBattleCommunication[4] += 1;
                }
            }
            STATE_SELECTION_SCRIPT => {
                if *(*gBattleStruct)
                    .selectionScriptFinished
                    .as_mut_ptr()
                    .at(gActiveBattler)
                    != 0
                {
                    gBattleCommunication[gActiveBattler] = *(*gBattleStruct)
                        .stateIdAfterSelScript
                        .as_mut_ptr()
                        .at(gActiveBattler);
                } else {
                    gBattlerAttacker = gActiveBattler;
                    gBattlescriptCurrInstr = gSelectionBattleScripts[gActiveBattler];
                    if gBattleControllerExecFlags
                        & (gBitTable[gActiveBattler]
                            | 0xf0000000
                            | gBitTable[gActiveBattler] << 4
                            | gBitTable[gActiveBattler] << 8
                            | gBitTable[gActiveBattler] << 12)
                        == 0
                    {
                        (*(&raw const crate::data::battle_script_commands::gBattleScriptingCommandsTable).cast::<CArray<Option<unsafe fn()>, 0>>())[*gBattlescriptCurrInstr].unwrap_unchecked()();
                    }
                    gSelectionBattleScripts[gActiveBattler] = gBattlescriptCurrInstr;
                }
            }
            STATE_WAIT_SET_BEFORE_ACTION => {
                if gBattleControllerExecFlags
                    & (gBitTable[gActiveBattler]
                        | 0xf0000000
                        | gBitTable[gActiveBattler] << 4
                        | gBitTable[gActiveBattler] << 8
                        | gBitTable[gActiveBattler] << 12)
                    == 0
                {
                    gBattleCommunication[gActiveBattler] = STATE_BEFORE_ACTION_CHOSEN;
                }
            }
            STATE_SELECTION_SCRIPT_MAY_RUN => {
                if *(*gBattleStruct)
                    .selectionScriptFinished
                    .as_mut_ptr()
                    .at(gActiveBattler)
                    != 0
                {
                    if gBattleBufferB[gActiveBattler][1] == B_ACTION_NOTHING_FAINTED {
                        gHitMarker |= HITMARKER_RUN;
                        gChosenActionByBattler[gActiveBattler] = B_ACTION_RUN;
                        gBattleCommunication[gActiveBattler] = STATE_WAIT_ACTION_CONFIRMED_STANDBY;
                    } else {
                        RecordedBattle_ClearBattlerAction(gActiveBattler, 1);
                        gBattleCommunication[gActiveBattler] = *(*gBattleStruct)
                            .stateIdAfterSelScript
                            .as_mut_ptr()
                            .at(gActiveBattler);
                    }
                } else {
                    gBattlerAttacker = gActiveBattler;
                    gBattlescriptCurrInstr = gSelectionBattleScripts[gActiveBattler];
                    if gBattleControllerExecFlags
                        & (gBitTable[gActiveBattler]
                            | 0xf0000000
                            | gBitTable[gActiveBattler] << 4
                            | gBitTable[gActiveBattler] << 8
                            | gBitTable[gActiveBattler] << 12)
                        == 0
                    {
                        (*(&raw const crate::data::battle_script_commands::gBattleScriptingCommandsTable).cast::<CArray<Option<unsafe fn()>, 0>>())[*gBattlescriptCurrInstr].unwrap_unchecked()();
                    }
                    gSelectionBattleScripts[gActiveBattler] = gBattlescriptCurrInstr;
                }
            }
            _ => {}
        }
        gActiveBattler += 1;
    }
    if gBattleCommunication[4] == gBattlersCount {
        RecordedBattle_CheckMovesetChanges(B_RECORD_MODE_RECORDING);
        gBattleMainFunc = Some(SetActionsAndBattlersTurnOrder);
        if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 {
            i = 0;
            while i < gBattlersCount as i32 {
                if gChosenActionByBattler[i] == B_ACTION_SWITCH {
                    SwitchPartyOrderInGameMulti(
                        i as u8,
                        *(*gBattleStruct).monToSwitchIntoId.as_mut_ptr().at(i),
                    );
                }
                i += 1;
            }
        }
    }
}
unsafe fn AllAtActionConfirmed() -> u8 {
    let mut count: i32 = 0;
    for i in 0..(gBattlersCount as i32) {
        if gBattleCommunication[i] == STATE_WAIT_ACTION_CONFIRMED {
            count += 1;
        }
    }
    if count + 1 == gBattlersCount as i32 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn UpdateBattlerPartyOrdersOnSwitch() {
    *(*gBattleStruct)
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler) = gBattleBufferB[gActiveBattler][1];
    RecordedBattle_SetBattlerAction(gActiveBattler, gBattleBufferB[gActiveBattler][1]);
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 && gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
        *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
            .at(gActiveBattler as i32 * 3) &= 0xF;
        *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
            .at(gActiveBattler as i32 * 3) |= gBattleBufferB[gActiveBattler][2] & 0xF0;
        *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
            .at(gActiveBattler as i32 * 3)
            .at(1) = gBattleBufferB[gActiveBattler][3];
        *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
            .at((gActiveBattler as i32 ^ 2) * 3) &= 0xF0;
        *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
            .at((gActiveBattler as i32 ^ 2) * 3) |=
            ((gBattleBufferB[gActiveBattler][2] as i32 & 0xF0) >> 4) as u8;
        *((*gBattleStruct).battlerPartyOrders.as_mut_ptr() as *mut u8)
            .at((gActiveBattler as i32 ^ 2) * 3)
            .at(2) = gBattleBufferB[gActiveBattler][3];
    }
}
pub unsafe fn SwapTurnOrder(id1: u8, id2: u8) {
    let mut temp: u32 = gActionsByTurnOrder[id1] as u32;
    gActionsByTurnOrder[id1] = gActionsByTurnOrder[id2];
    gActionsByTurnOrder[id2] = temp as u8;
    temp = gBattlerByTurnOrder[id1] as u32;
    gBattlerByTurnOrder[id1] = gBattlerByTurnOrder[id2];
    gBattlerByTurnOrder[id2] = temp as u8;
}
pub unsafe fn GetWhoStrikesFirst(battler1: u8, battler2: u8, ignoreChosenMoves: u8) -> u8 {
    let mut strikesFirst: u8 = 0;
    let mut speedMultiplierBattler1: u8 = 0;
    let mut speedMultiplierBattler2: u8 = 0;
    let mut holdEffect: u8 = 0;
    let mut holdEffectParam: u8 = 0;
    let mut moveBattler1: u16 = 0;
    let mut moveBattler2: u16 = 0;
    if AbilityBattleEffects(19, 0, 13, 0, 0) == 0 && AbilityBattleEffects(19, 0, 77, 0, 0) == 0 {
        if gBattleMons[battler1].ability == ABILITY_SWIFT_SWIM
            && gBattleWeather as i32 & B_WEATHER_RAIN != 0
            || gBattleMons[battler1].ability == ABILITY_CHLOROPHYLL
                && gBattleWeather as i32 & B_WEATHER_SUN != 0
        {
            speedMultiplierBattler1 = 2;
        } else {
            speedMultiplierBattler1 = 1;
        }
        if gBattleMons[battler2].ability == ABILITY_SWIFT_SWIM
            && gBattleWeather as i32 & B_WEATHER_RAIN != 0
            || gBattleMons[battler2].ability == ABILITY_CHLOROPHYLL
                && gBattleWeather as i32 & B_WEATHER_SUN != 0
        {
            speedMultiplierBattler2 = 2;
        } else {
            speedMultiplierBattler2 = 1;
        }
    } else {
        speedMultiplierBattler1 = 1;
        speedMultiplierBattler2 = 1;
    }
    let mut speedBattler1: u32 = div_i32(
        gBattleMons[battler1].speed as i32
            * speedMultiplierBattler1 as i32
            * (*(&raw const crate::data::pokemon::gStatStageRatios)
                .cast::<CArray<CArray<u8, 2>, 13>>())[gBattleMons[battler1].statStages[3]][0]
                as i32,
        (*(&raw const crate::data::pokemon::gStatStageRatios).cast::<CArray<CArray<u8, 2>, 13>>())
            [gBattleMons[battler1].statStages[3]][1] as i32,
    ) as u32;
    if gBattleMons[battler1].item == ITEM_ENIGMA_BERRY {
        holdEffect = gEnigmaBerries[battler1].holdEffect;
        holdEffectParam = gEnigmaBerries[battler1].holdEffectParam;
    } else {
        holdEffect = GetItemHoldEffect(gBattleMons[battler1].item);
        holdEffectParam = GetItemHoldEffectParam(gBattleMons[battler1].item);
    }
    if gBattleTypeFlags & 0x23f0102 == 0
        && FlagGet(FLAG_BADGE03_GET) != 0
        && GetBattlerSide(battler1) == B_SIDE_PLAYER
    {
        speedBattler1 = speedBattler1 * 110 / 100;
    }
    if holdEffect == HOLD_EFFECT_MACHO_BRACE {
        speedBattler1 /= 2;
    }
    if gBattleMons[battler1].status1 & STATUS1_PARALYSIS != 0 {
        speedBattler1 /= 4;
    }
    if holdEffect == HOLD_EFFECT_QUICK_CLAW
        && (gRandomTurnNumber.get() as i32) < 0xFFFF * holdEffectParam as i32 / 100
    {
        speedBattler1 = 0xffffffff;
    }
    let mut speedBattler2: u32 = div_i32(
        gBattleMons[battler2].speed as i32
            * speedMultiplierBattler2 as i32
            * (*(&raw const crate::data::pokemon::gStatStageRatios)
                .cast::<CArray<CArray<u8, 2>, 13>>())[gBattleMons[battler2].statStages[3]][0]
                as i32,
        (*(&raw const crate::data::pokemon::gStatStageRatios).cast::<CArray<CArray<u8, 2>, 13>>())
            [gBattleMons[battler2].statStages[3]][1] as i32,
    ) as u32;
    if gBattleMons[battler2].item == ITEM_ENIGMA_BERRY {
        holdEffect = gEnigmaBerries[battler2].holdEffect;
        holdEffectParam = gEnigmaBerries[battler2].holdEffectParam;
    } else {
        holdEffect = GetItemHoldEffect(gBattleMons[battler2].item);
        holdEffectParam = GetItemHoldEffectParam(gBattleMons[battler2].item);
    }
    if gBattleTypeFlags & 0x23f0102 == 0
        && FlagGet(FLAG_BADGE03_GET) != 0
        && GetBattlerSide(battler2) == B_SIDE_PLAYER
    {
        speedBattler2 = speedBattler2 * 110 / 100;
    }
    if holdEffect == HOLD_EFFECT_MACHO_BRACE {
        speedBattler2 /= 2;
    }
    if gBattleMons[battler2].status1 & STATUS1_PARALYSIS != 0 {
        speedBattler2 /= 4;
    }
    if holdEffect == HOLD_EFFECT_QUICK_CLAW
        && (gRandomTurnNumber.get() as i32) < 0xFFFF * holdEffectParam as i32 / 100
    {
        speedBattler2 = 0xffffffff;
    }
    if ignoreChosenMoves != 0 {
        moveBattler1 = MOVE_NONE;
        moveBattler2 = MOVE_NONE;
    } else {
        if gChosenActionByBattler[battler1] == B_ACTION_USE_MOVE {
            if gProtectStructs[battler1].noValidMoves() != 0 {
                moveBattler1 = MOVE_STRUGGLE;
            } else {
                moveBattler1 = gBattleMons[battler1].moves[*(*gBattleStruct)
                    .chosenMovePositions
                    .as_mut_ptr()
                    .at(battler1)];
            }
        } else {
            moveBattler1 = MOVE_NONE;
        }
        if gChosenActionByBattler[battler2] == B_ACTION_USE_MOVE {
            if gProtectStructs[battler2].noValidMoves() != 0 {
                moveBattler2 = MOVE_STRUGGLE;
            } else {
                moveBattler2 = gBattleMons[battler2].moves[*(*gBattleStruct)
                    .chosenMovePositions
                    .as_mut_ptr()
                    .at(battler2)];
            }
        } else {
            moveBattler2 = MOVE_NONE;
        }
    }
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [moveBattler1]
        .priority
        != 0
        || (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [moveBattler2]
            .priority
            != 0
    {
        if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [moveBattler1]
            .priority
            == (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [moveBattler2]
                .priority
        {
            if speedBattler1 == speedBattler2 && Random() as i32 & 1 != 0 {
                strikesFirst = 2;
            } else if speedBattler1 < speedBattler2 {
                strikesFirst = 1;
            }
        } else if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [moveBattler1]
            .priority
            < (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [moveBattler2]
                .priority
        {
            strikesFirst = 1;
        }
    } else {
        if speedBattler1 == speedBattler2 && Random() as i32 & 1 != 0 {
            strikesFirst = 2;
        } else if speedBattler1 < speedBattler2 {
            strikesFirst = 1;
        }
    }
    strikesFirst
}
pub(crate) unsafe fn SetActionsAndBattlersTurnOrder() {
    let mut turnOrderId: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_SAFARI != 0 {
        gActiveBattler = 0;
        while gActiveBattler < gBattlersCount {
            gActionsByTurnOrder[turnOrderId] = gChosenActionByBattler[gActiveBattler];
            gBattlerByTurnOrder[turnOrderId] = gActiveBattler;
            turnOrderId += 1;
            gActiveBattler += 1;
        }
    } else {
        if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
            gActiveBattler = 0;
            while gActiveBattler < gBattlersCount {
                if gChosenActionByBattler[gActiveBattler] == B_ACTION_RUN {
                    turnOrderId = 5;
                    break;
                }
                gActiveBattler += 1;
            }
        } else {
            if gChosenActionByBattler[0] == B_ACTION_RUN {
                gActiveBattler = 0;
                turnOrderId = 5;
            }
            if gChosenActionByBattler[2] == B_ACTION_RUN {
                gActiveBattler = 2;
                turnOrderId = 5;
            }
        }
        if turnOrderId == 5 {
            gActionsByTurnOrder[0] = gChosenActionByBattler[gActiveBattler];
            gBattlerByTurnOrder[0] = gActiveBattler;
            turnOrderId = 1;
            for i in 0..(gBattlersCount as i32) {
                if i != gActiveBattler as i32 {
                    gActionsByTurnOrder[turnOrderId] = gChosenActionByBattler[i];
                    gBattlerByTurnOrder[turnOrderId] = i as u8;
                    turnOrderId += 1;
                }
            }
            gBattleMainFunc = Some(CheckFocusPunch_ClearVarsBeforeTurnStarts);
            (*gBattleStruct).focusPunchBattlerId = 0;
            return;
        } else {
            gActiveBattler = 0;
            while gActiveBattler < gBattlersCount {
                if gChosenActionByBattler[gActiveBattler] == B_ACTION_USE_ITEM
                    || gChosenActionByBattler[gActiveBattler] == B_ACTION_SWITCH
                {
                    gActionsByTurnOrder[turnOrderId] = gChosenActionByBattler[gActiveBattler];
                    gBattlerByTurnOrder[turnOrderId] = gActiveBattler;
                    turnOrderId += 1;
                }
                gActiveBattler += 1;
            }
            gActiveBattler = 0;
            while gActiveBattler < gBattlersCount {
                if gChosenActionByBattler[gActiveBattler] != B_ACTION_USE_ITEM
                    && gChosenActionByBattler[gActiveBattler] != B_ACTION_SWITCH
                {
                    gActionsByTurnOrder[turnOrderId] = gChosenActionByBattler[gActiveBattler];
                    gBattlerByTurnOrder[turnOrderId] = gActiveBattler;
                    turnOrderId += 1;
                }
                gActiveBattler += 1;
            }
            i = 0;
            while i < gBattlersCount as i32 - 1 {
                j = i + 1;
                while j < gBattlersCount as i32 {
                    let battler1: u8 = gBattlerByTurnOrder[i];
                    let battler2: u8 = gBattlerByTurnOrder[j];
                    if gActionsByTurnOrder[i] != B_ACTION_USE_ITEM
                        && gActionsByTurnOrder[j] != B_ACTION_USE_ITEM
                        && gActionsByTurnOrder[i] != B_ACTION_SWITCH
                        && gActionsByTurnOrder[j] != B_ACTION_SWITCH
                        && GetWhoStrikesFirst(battler1, battler2, FALSE) != 0
                    {
                        SwapTurnOrder(i as u8, j as u8);
                    }
                    j += 1;
                }
                i += 1;
            }
        }
    }
    gBattleMainFunc = Some(CheckFocusPunch_ClearVarsBeforeTurnStarts);
    (*gBattleStruct).focusPunchBattlerId = 0;
}
unsafe fn TurnValuesCleanUp(var0: u8) {
    let mut dataPtr: *mut u8 = null_mut();
    gActiveBattler = 0;
    while gActiveBattler < gBattlersCount {
        if var0 != 0 {
            gProtectStructs[gActiveBattler].set_protected(0);
            gProtectStructs[gActiveBattler].set_endured(0);
        } else {
            dataPtr = &raw mut gProtectStructs[gActiveBattler] as *mut u8;
            for i in 0..16i32 {
                *dataPtr.at(i) = 0;
            }
            if gDisableStructs[gActiveBattler].isFirstTurn != 0 {
                gDisableStructs[gActiveBattler].isFirstTurn -= 1;
            }
            if gDisableStructs[gActiveBattler].rechargeTimer != 0 {
                gDisableStructs[gActiveBattler].rechargeTimer -= 1;
                if gDisableStructs[gActiveBattler].rechargeTimer == 0 {
                    gBattleMons[gActiveBattler].status2 &= 0xffbfffff;
                }
            }
        }
        if gDisableStructs[gActiveBattler].substituteHP == 0 {
            gBattleMons[gActiveBattler].status2 &= 0xfeffffff;
        }
        gActiveBattler += 1;
    }
    gSideTimers[0].followmeTimer = 0;
    gSideTimers[1].followmeTimer = 0;
}
pub unsafe fn SpecialStatusesClear() {
    gActiveBattler = 0;
    while gActiveBattler < gBattlersCount {
        let dataPtr: *mut u8 = &raw mut gSpecialStatuses[gActiveBattler] as *mut u8;
        for i in 0..20i32 {
            *dataPtr.at(i) = 0;
        }
        gActiveBattler += 1;
    }
}
pub(crate) unsafe fn CheckFocusPunch_ClearVarsBeforeTurnStarts() {
    if gHitMarker & HITMARKER_RUN == 0 {
        while (*gBattleStruct).focusPunchBattlerId < gBattlersCount {
            gActiveBattler = {
                gBattlerAttacker = (*gBattleStruct).focusPunchBattlerId;
                gBattlerAttacker
            };
            (*gBattleStruct).focusPunchBattlerId += 1;
            if gChosenMoveByBattler[gActiveBattler] == MOVE_FOCUS_PUNCH
                && gBattleMons[gActiveBattler].status1 & STATUS1_SLEEP == 0
                && gDisableStructs[gBattlerAttacker].truantCounter() == 0
                && gProtectStructs[gActiveBattler].noValidMoves() == 0
            {
                BattleScriptExecute(
                    (*crate::asmdata::BattleScript_FocusPunchSetUp.cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
                return;
            }
        }
    }
    TryClearRageStatuses();
    gCurrentTurnActionNumber = 0;
    gCurrentActionFuncId = gActionsByTurnOrder[gCurrentTurnActionNumber];
    gDynamicBasePower = 0;
    (*gBattleStruct).dynamicMoveType = 0;
    gBattleMainFunc = Some(RunTurnActionsFunctions);
    gBattleCommunication[3] = 0;
    gBattleCommunication[4] = 0;
    gBattleScripting.multihitMoveEffect = 0;
    (*(*gBattleResources).battleScriptsStack).size = 0;
}
pub(crate) unsafe fn RunTurnActionsFunctions() {
    if gBattleOutcome != 0 {
        gCurrentActionFuncId = B_ACTION_FINISHED;
    }
    (*gBattleStruct).savedTurnActionNumber = gCurrentTurnActionNumber;
    sTurnActionsFuncsTable[gCurrentActionFuncId].unwrap_unchecked()();
    if gCurrentTurnActionNumber >= gBattlersCount {
        gHitMarker &= 0xffefffff;
        gBattleMainFunc = sEndTurnFuncsTable[gBattleOutcome as i32 & 0x7F];
    } else {
        if (*gBattleStruct).savedTurnActionNumber != gCurrentTurnActionNumber {
            gHitMarker &= 0xfffffdff;
            gHitMarker &= 0xfff7ffff;
        }
    }
}
pub(crate) unsafe fn HandleEndTurn_BattleWon() {
    gCurrentActionFuncId = 0;
    if gBattleTypeFlags & 0x2000002 != 0 {
        gSpecialVar_Result = gBattleOutcome as u16;
        gBattleTextBuff1[0] = gBattleOutcome;
        gBattlerAttacker = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_LinkBattleWonOrLost
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
        gBattleOutcome &= 127;
    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 && gBattleTypeFlags & 0x43f0900 != 0 {
        BattleStopLowHpSound();
        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_FrontierTrainerBattleWon
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
        if gTrainerBattleOpponent_A == TRAINER_FRONTIER_BRAIN {
            PlayBGM(MUS_VICTORY_GYM_LEADER);
        } else {
            PlayBGM(MUS_VICTORY_TRAINER);
        }
    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0
        && gBattleTypeFlags & BATTLE_TYPE_LINK == 0
    {
        BattleStopLowHpSound();
        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_LocalTrainerBattleWon
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
        match (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
            [gTrainerBattleOpponent_A]
            .trainerClass
        {
            TRAINER_CLASS_ELITE_FOUR | TRAINER_CLASS_CHAMPION => {
                PlayBGM(MUS_VICTORY_LEAGUE);
            }
            TRAINER_CLASS_TEAM_AQUA
            | TRAINER_CLASS_TEAM_MAGMA
            | TRAINER_CLASS_AQUA_ADMIN
            | TRAINER_CLASS_AQUA_LEADER
            | TRAINER_CLASS_MAGMA_ADMIN
            | TRAINER_CLASS_MAGMA_LEADER => {
                PlayBGM(MUS_VICTORY_AQUA_MAGMA);
            }
            TRAINER_CLASS_LEADER => {
                PlayBGM(MUS_VICTORY_GYM_LEADER);
            }
            _ => {
                PlayBGM(MUS_VICTORY_TRAINER);
            }
        }
    } else {
        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PayDayMoneyAndPickUpItems
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    }
    gBattleMainFunc = Some(HandleEndTurn_FinishBattle);
}
pub(crate) unsafe fn HandleEndTurn_BattleLost() {
    gCurrentActionFuncId = 0;
    if gBattleTypeFlags & 0x2000002 != 0 {
        if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
            if gBattleOutcome as i32 & B_OUTCOME_LINK_BATTLE_RAN != 0 {
                gBattlescriptCurrInstr =
                    (*crate::asmdata::BattleScript_PrintPlayerForfeitedLinkBattle
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut();
                gBattleOutcome &= 127;
                (*gSaveBlock2Ptr).frontier.set_disableRecordBattle(TRUE);
            } else {
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_FrontierLinkBattleLost
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
                gBattleOutcome &= 127;
            }
        } else {
            gBattleTextBuff1[0] = gBattleOutcome;
            gBattlerAttacker = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_LinkBattleWonOrLost
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
            gBattleOutcome &= 127;
        }
    } else {
        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_LocalBattleLost
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    }
    gBattleMainFunc = Some(HandleEndTurn_FinishBattle);
}
pub(crate) unsafe fn HandleEndTurn_RanFromBattle() {
    gCurrentActionFuncId = 0;
    if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 && gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PrintPlayerForfeited
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
        gBattleOutcome = B_OUTCOME_FORFEITED;
        (*gSaveBlock2Ptr).frontier.set_disableRecordBattle(TRUE);
    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER_HILL != 0 {
        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PrintPlayerForfeited
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
        gBattleOutcome = B_OUTCOME_FORFEITED;
    } else {
        match gProtectStructs[gBattlerAttacker].fleeType() {
            FLEE_ITEM => {
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SmokeBallEscape
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            }
            FLEE_ABILITY => {
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_RanAwayUsingMonAbility
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            }
            _ => {
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_GotAwaySafely
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
            }
        }
    }
    gBattleMainFunc = Some(HandleEndTurn_FinishBattle);
}
pub(crate) unsafe fn HandleEndTurn_MonFled() {
    gCurrentActionFuncId = 0;
    gBattleTextBuff1[0] = 0xFD;
    gBattleTextBuff1[1] = 7;
    gBattleTextBuff1[2] = gBattlerAttacker;
    gBattleTextBuff1[3] = gBattlerPartyIndexes[gBattlerAttacker] as u8;
    gBattleTextBuff1[4] = 0xFF;
    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_WildMonFled.cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    gBattleMainFunc = Some(HandleEndTurn_FinishBattle);
}
pub(crate) unsafe fn HandleEndTurn_FinishBattle() {
    if gCurrentActionFuncId == B_ACTION_TRY_FINISH || gCurrentActionFuncId == B_ACTION_FINISHED {
        if gBattleTypeFlags & 0x23f0b92 == 0 {
            gActiveBattler = 0;
            while gActiveBattler < gBattlersCount {
                if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
                    if gBattleResults.playerMon1Species == SPECIES_NONE {
                        gBattleResults.playerMon1Species = GetMonData3(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                            MON_DATA_SPECIES,
                            null_mut(),
                        ) as u16;
                        GetMonData3(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                            MON_DATA_NICKNAME,
                            gBattleResults.playerMon1Name.as_mut_ptr(),
                        );
                    } else {
                        gBattleResults.playerMon2Species = GetMonData3(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                            MON_DATA_SPECIES,
                            null_mut(),
                        ) as u16;
                        GetMonData3(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                            MON_DATA_NICKNAME,
                            gBattleResults.playerMon2Name.as_mut_ptr(),
                        );
                    }
                }
                gActiveBattler += 1;
            }
            TryPutPokemonTodayOnAir();
        }
        if gBattleTypeFlags & 0x23f0b9a == 0 && gBattleResults.shinyWildMon() != 0 {
            TryPutBreakingNewsOnAir();
        }
        RecordedBattle_SetPlaybackFinished();
        BeginFastPaletteFade(3);
        FadeOutMapMusic(5);
        gBattleMainFunc = Some(FreeResetData_ReturnToOvOrDoEvolutions);
        gCB2_AfterEvolution = Some(BattleMainCB2);
    } else {
        if gBattleControllerExecFlags == 0 {
            (*(&raw const crate::data::battle_script_commands::gBattleScriptingCommandsTable)
                .cast::<CArray<Option<unsafe fn()>, 0>>())[*gBattlescriptCurrInstr]
                .unwrap_unchecked()();
        }
    }
}
pub(crate) unsafe fn FreeResetData_ReturnToOvOrDoEvolutions() {
    if gPaletteFade.active() == 0 {
        ResetSpriteData();
        if gLeveledUpInBattle == 0 || gBattleOutcome != B_OUTCOME_WON {
            gBattleMainFunc = Some(ReturnFromBattleToOverworld);
            return;
        } else {
            gBattleMainFunc = Some(TryEvolvePokemon);
        }
    }
    FreeAllWindowBuffers();
    if gBattleTypeFlags & BATTLE_TYPE_LINK == 0 {
        FreeMonSpritesGfx();
        FreeBattleResources();
        FreeBattleSpritesData();
    }
}
pub(crate) unsafe fn TryEvolvePokemon() {
    while gLeveledUpInBattle != 0 {
        for i in 0..PARTY_SIZE {
            if gLeveledUpInBattle as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                != 0
            {
                let mut levelUpBits: u8 = gLeveledUpInBattle;
                levelUpBits &= !(gBitTable[i] as u8);
                gLeveledUpInBattle = levelUpBits;
                let species: u16 = GetEvolutionTargetSpecies(
                    &raw mut gPlayerParty[i],
                    EVO_MODE_NORMAL,
                    levelUpBits as u16,
                );
                if species != SPECIES_NONE {
                    FreeAllWindowBuffers();
                    gBattleMainFunc = Some(WaitForEvoSceneToFinish);
                    EvolutionScene(&raw mut gPlayerParty[i], species, TRUE, i as u8);
                    return;
                }
            }
        }
    }
    gBattleMainFunc = Some(ReturnFromBattleToOverworld);
}
pub(crate) unsafe fn WaitForEvoSceneToFinish() {
    if gMain.callback2 == Some(BattleMainCB2 as unsafe fn()) {
        gBattleMainFunc = Some(TryEvolvePokemon);
    }
}
pub(crate) unsafe fn ReturnFromBattleToOverworld() {
    if gBattleTypeFlags & BATTLE_TYPE_LINK == 0 {
        RandomlyGivePartyPokerus(gPlayerParty.as_mut_ptr());
        PartySpreadPokerus(gPlayerParty.as_mut_ptr());
    }
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 && gReceivedRemoteLinkPlayers != 0 {
        return;
    }
    gSpecialVar_Result = gBattleOutcome as u16;
    gMain.set_inBattle(FALSE);
    gMain.callback1 = gPreBattleCallback1;
    if gBattleTypeFlags & BATTLE_TYPE_ROAMER != 0 {
        UpdateRoamerHPStatus(&raw mut gEnemyParty[0]);
        if gBattleOutcome as i32 & B_OUTCOME_WON as i32 != 0 || gBattleOutcome == B_OUTCOME_CAUGHT {
            SetRoamerInactive();
        }
    }
    m4aSongNumStop(SE_LOW_HEALTH);
    SetMainCallback2(gMain.savedCallback);
}
pub unsafe fn RunBattleScriptCommands_PopCallbacksStack() {
    if gCurrentActionFuncId == B_ACTION_TRY_FINISH || gCurrentActionFuncId == B_ACTION_FINISHED {
        if (*(*gBattleResources).battleCallbackStack).size != 0 {
            (*(*gBattleResources).battleCallbackStack).size -= 1;
        }
        gBattleMainFunc = (*(*gBattleResources).battleCallbackStack).function
            [(*(*gBattleResources).battleCallbackStack).size];
    } else {
        if gBattleControllerExecFlags == 0 {
            (*(&raw const crate::data::battle_script_commands::gBattleScriptingCommandsTable)
                .cast::<CArray<Option<unsafe fn()>, 0>>())[*gBattlescriptCurrInstr]
                .unwrap_unchecked()();
        }
    }
}
pub unsafe fn RunBattleScriptCommands() {
    if gBattleControllerExecFlags == 0 {
        (*(&raw const crate::data::battle_script_commands::gBattleScriptingCommandsTable)
            .cast::<CArray<Option<unsafe fn()>, 0>>())[*gBattlescriptCurrInstr]
            .unwrap_unchecked()();
    }
}
