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
fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe fn(*mut Sprite)>) -> u8 {
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
fn ScanlineEffect_SetParams(a0: ScanlineEffectParams) {
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
fn UpdateRoamerHPStatus(a0: *mut Pokemon) {
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
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattle_BG0_X: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattle_BG0_Y: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattle_BG1_X: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattle_BG1_Y: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattle_BG2_X: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattle_BG2_Y: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattle_BG3_X: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattle_BG3_Y: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattle_WIN0H: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattle_WIN0V: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattle_WIN1H: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattle_WIN1V: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gDisplayedStringBattle: crate::global::Global<Aligned<CArray<u8, 300>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleTextBuff1: crate::global::Global<Aligned<CArray<u8, 16>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleTextBuff2: crate::global::Global<Aligned<CArray<u8, 16>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleTextBuff3: crate::global::Global<Aligned<CArray<u8, 16>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sFlickerArray: crate::global::Global<CArray<u32, 25>> =
    crate::global::Global::new(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleTypeFlags: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleEnvironment: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gUnusedFirstBattleVar1: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gMultiPartnerParty: crate::global::Global<CArray<MultiPartnerMenuPokemon, 3>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sMultiPartnerPartyBuffer: crate::global::Global<*mut MultiPartnerMenuPokemon> =
    crate::global::Global::new(null_mut());
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleAnimBgTileBuffer: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleAnimBgTilemapBuffer: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleBufferA: crate::global::Global<Aligned<CArray<CArray<u8, 512>, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleBufferB: crate::global::Global<Aligned<CArray<CArray<u8, 512>, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gActiveBattler: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleControllerExecFlags: crate::global::Global<u32> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlersCount: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlerPartyIndexes: crate::global::Global<Aligned<CArray<u16, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlerPositions: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gActionsByTurnOrder: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlerByTurnOrder: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gCurrentTurnActionNumber: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gCurrentActionFuncId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleMons: crate::global::Global<CArray<BattlePokemon, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlerSpriteIds: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gCurrMovePos: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gChosenMovePos: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gCurrentMove: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gChosenMove: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gCalledMove: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleMoveDamage: crate::global::Global<i32> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gHpDealt: crate::global::Global<i32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBideDmg: crate::global::Global<CArray<i32, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLastUsedItem: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLastUsedAbility: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlerAttacker: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlerTarget: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlerFainted: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gEffectBattler: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gPotentialItemEffectBattler: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gAbsentBattlerFlags: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gCritMultiplier: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gMultiHitCounter: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlescriptCurrInstr: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gUnusedBattleMainVar: crate::global::Global<u32> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gChosenActionByBattler: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gSelectionBattleScripts: crate::global::Global<CArray<*mut u8, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gPalaceSelectionBattleScripts: crate::global::Global<CArray<*mut u8, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLastPrintedMoves: crate::global::Global<Aligned<CArray<u16, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLastMoves: crate::global::Global<Aligned<CArray<u16, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLastLandedMoves: crate::global::Global<Aligned<CArray<u16, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLastHitByType: crate::global::Global<Aligned<CArray<u16, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLastResultingMoves: crate::global::Global<Aligned<CArray<u16, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLockedMoves: crate::global::Global<Aligned<CArray<u16, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLastHitBy: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gChosenMoveByBattler: crate::global::Global<Aligned<CArray<u16, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gMoveResultFlags: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gHitMarker: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sUnusedBattlersArray: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBideTarget: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gUnusedFirstBattleVar2: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gSideStatuses: crate::global::Global<Aligned<CArray<u16, 2>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gSideTimers: crate::global::Global<CArray<SideTimer, 2>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gStatuses3: crate::global::Global<CArray<u32, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gDisableStructs: crate::global::Global<CArray<DisableStruct, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gPauseCounterBattle: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gPaydayMoney: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gRandomTurnNumber: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleCommunication: crate::global::Global<Aligned<CArray<u8, 8>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleOutcome: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gProtectStructs: crate::global::Global<CArray<ProtectStruct, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gSpecialStatuses: crate::global::Global<CArray<SpecialStatus, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleWeather: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gWishFutureKnock: crate::global::Global<WishFutureKnock> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gIntroSlideFlags: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gSentPokesToOpponent: crate::global::Global<Aligned<CArray<u8, 2>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gDynamicBasePower: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gExpShareExp: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gEnigmaBerries: crate::global::Global<CArray<BattleEnigmaBerry, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleScripting: crate::global::Global<BattleScripting> =
    crate::global::Global::new(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleStruct: crate::global::Global<*mut BattleStruct> =
    crate::global::Global::new(null_mut());
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLinkBattleSendBuffer: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLinkBattleRecvBuffer: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleResources: crate::global::Global<*mut BattleResources> =
    crate::global::Global::new(null_mut());
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gActionSelectionCursor: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gMoveSelectionCursor: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlerStatusSummaryTaskId: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlerInMenuId: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gDoingBattleAnim: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gTransformedPersonalities: crate::global::Global<CArray<u32, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gPlayerDpadHoldFrames: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleSpritesDataPtr: crate::global::Global<*mut BattleSpriteData> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gMonSpritesGfxPtr: crate::global::Global<*mut MonSpritesGfx> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleControllerOpponentHealthboxData: crate::global::Global<*mut BattleHealthboxInfo> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleControllerOpponentFlankHealthboxData: crate::global::Global<
    *mut BattleHealthboxInfo,
> = crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleMovePower: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gMoveToLearn: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattleMonForms: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gPreBattleCallback1: crate::global::Global<Option<unsafe fn()>> =
    crate::global::Global::new(None);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gBattleMainFunc: crate::global::Global<Option<unsafe fn()>> =
    crate::global::Global::new(None);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gBattleResults: crate::global::Global<BattleResults> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLeveledUpInBattle: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gBattlerControllerFuncs: crate::global::Global<CArray<Option<unsafe fn()>, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gHealthboxSpriteIds: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gMultiUsePlayerCursor: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gNumberOfMovesToChoose: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gBattleControllerData: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));

/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
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

#[unsafe(no_mangle)]
pub unsafe fn CB2_InitBattle() {
    MoveSaveBlocks_ResetHeap();
    AllocateBattleResources();
    AllocateBattleSpritesData();
    AllocateMonSpritesGfx();
    RecordedBattle_ClearFrontierPassFlag();
    if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED != 0 {
            CB2_InitBattleInternal();
        } else if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER == 0 {
            HandleLinkBattleSetup();
            SetMainCallback2(Some(CB2_PreInitMultiBattle));
        } else {
            SetMainCallback2(Some(CB2_PreInitIngamePlayerPartnerBattle));
        }
        crate::gl!(gBattleCommunication[0] = 0);
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
    gBattle_WIN0H.set(DISPLAY_WIDTH);
    if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0
        && gPartnerTrainerId.get() != TRAINER_STEVEN_PARTNER
    {
        gBattle_WIN0V.set(159);
        gBattle_WIN1H.set(DISPLAY_WIDTH);
        gBattle_WIN1V.set(32);
    } else {
        gBattle_WIN0V.set(20561);
        ScanlineEffect_Clear();
        i = 0;
        while i < 80 {
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = 0xF0;
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = 0xF0;
            i += 1;
        }
        while i < DISPLAY_HEIGHT as i32 {
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = 0xFF10;
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = 0xFF10;
            i += 1;
        }
        ScanlineEffect_SetParams(*sIntroScanlineParams16Bit);
    }
    ResetPaletteFade();
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    gBattle_BG1_X.set(0);
    gBattle_BG1_Y.set(0);
    gBattle_BG2_X.set(0);
    gBattle_BG2_Y.set(0);
    gBattle_BG3_X.set(0);
    gBattle_BG3_Y.set(0);
    gBattleEnvironment.set(BattleSetup_GetEnvironmentId());
    if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED != 0 {
        gBattleEnvironment.set(BATTLE_ENVIRONMENT_BUILDING);
    }
    InitBattleBgsVideo();
    LoadBattleTextboxAndBackground();
    ResetSpriteData();
    ResetTasks();
    DrawBattleEntryBackground();
    FreeAllSpritePalettes();
    gReservedSpritePaletteCount.set(MAX_BATTLERS_COUNT);
    SetVBlankCallback(Some(VBlankCB_Battle));
    SetUpBattleVarsAndBirchZigzagoon();
    if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_BATTLE_TOWER != 0
    {
        SetMainCallback2(Some(CB2_HandleStartMultiPartnerBattle));
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0
    {
        SetMainCallback2(Some(CB2_HandleStartMultiPartnerBattle));
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
        SetMainCallback2(Some(CB2_HandleStartMultiBattle));
    } else {
        SetMainCallback2(Some(CB2_HandleStartBattle));
    }
    if gBattleTypeFlags.get() & 0x1000002 == 0 {
        CreateNPCTrainerParty(
            crate::gl!(&raw gEnemyParty[0]),
            gTrainerBattleOpponent_A.get(),
            TRUE,
        );
        if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0 {
            CreateNPCTrainerParty(
                crate::gl!(&raw gEnemyParty[3]),
                gTrainerBattleOpponent_B.get(),
                FALSE,
            );
        }
        SetWildMonHeldItem();
    }
    (*gMain.as_ptr()).set_inBattle(TRUE);
    (*crate::gl!(gSaveBlock2Ptr))
        .frontier
        .set_disableRecordBattle(FALSE);
    for i in 0..PARTY_SIZE {
        AdjustFriendship(
            crate::gl!(&raw gPlayerParty[i]),
            FRIENDSHIP_EVENT_LEAGUE_BATTLE,
        );
    }
    crate::gl!(gBattleCommunication[0] = 0);
}
unsafe fn BufferPartyVsScreenHealth_AtStart() {
    let mut flags: u16 = 0;
    for i in 0..6i32 {
        'l1: {
            let species: u16 =
                GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES_OR_EGG) as u16;
            let hp: u16 = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HP) as u16;
            let status: u32 = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_STATUS);
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
    (*crate::gl!(gBattleStruct))
        .multiBuffer
        .linkBattlerHeader
        .vsScreenHealthFlagsLo = flags as u8;
    (*crate::gl!(gBattleStruct))
        .multiBuffer
        .linkBattlerHeader
        .vsScreenHealthFlagsHi = (flags >> 8) as u8;
    (*crate::gl!(gBattleStruct))
        .multiBuffer
        .linkBattlerHeader
        .vsScreenHealthFlagsHi |= FlagGet(FLAG_SYS_FRONTIER_PASS) << 7;
}
unsafe fn SetPlayerBerryDataInBattleStruct() {
    let mut i: i32 = 0;
    let battleStruct: *mut BattleStruct = crate::gl!(gBattleStruct);
    let battleBerry: *mut BattleEnigmaBerry = &raw mut (*battleStruct)
        .multiBuffer
        .linkBattlerHeader
        .battleEnigmaBerry;
    if IsEnigmaBerryValid() == TRUE as u32 {
        i = 0;
        while i < BERRY_NAME_LENGTH {
            (*battleBerry).name[i] = (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.berry.name[i];
            i += 1;
        }
        (*battleBerry).name[i] = EOS;
        for i in 0..BERRY_ITEM_EFFECT_COUNT {
            (*battleBerry).itemEffect[i] = (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.itemEffect[i];
        }
        (*battleBerry).holdEffect = (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.holdEffect;
        (*battleBerry).holdEffectParam = (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.holdEffectParam;
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
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK == 0 {
        if IsEnigmaBerryValid() == TRUE as u32 {
            i = 0;
            while i < BERRY_NAME_LENGTH {
                crate::gl!(
                    gEnigmaBerries[0].name[i] =
                        (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.berry.name[i]
                );
                crate::gl!(
                    gEnigmaBerries[2].name[i] =
                        (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.berry.name[i]
                );
                i += 1;
            }
            crate::gl!(gEnigmaBerries[0].name[i] = EOS);
            crate::gl!(gEnigmaBerries[2].name[i] = EOS);
            for i in 0..BERRY_ITEM_EFFECT_COUNT {
                crate::gl!(
                    gEnigmaBerries[0].itemEffect[i] =
                        (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.itemEffect[i]
                );
                crate::gl!(
                    gEnigmaBerries[2].itemEffect[i] =
                        (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.itemEffect[i]
                );
            }
            crate::gl!(
                gEnigmaBerries[0].holdEffect = (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.holdEffect
            );
            crate::gl!(
                gEnigmaBerries[2].holdEffect = (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.holdEffect
            );
            crate::gl!(
                gEnigmaBerries[0].holdEffectParam =
                    (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.holdEffectParam
            );
            crate::gl!(
                gEnigmaBerries[2].holdEffectParam =
                    (*crate::gl!(gSaveBlock1Ptr)).enigmaBerry.holdEffectParam
            );
        } else {
            let berryData: *mut Berry = GetBerryInfo(ItemIdToBerryType(ITEM_ENIGMA_BERRY));
            i = 0;
            while i < BERRY_NAME_LENGTH {
                crate::gl!(gEnigmaBerries[0].name[i] = (*berryData).name[i]);
                crate::gl!(gEnigmaBerries[2].name[i] = (*berryData).name[i]);
                i += 1;
            }
            crate::gl!(gEnigmaBerries[0].name[i] = EOS);
            crate::gl!(gEnigmaBerries[2].name[i] = EOS);
            for i in 0..BERRY_ITEM_EFFECT_COUNT {
                crate::gl!(gEnigmaBerries[0].itemEffect[i] = 0);
                crate::gl!(gEnigmaBerries[2].itemEffect[i] = 0);
            }
            crate::gl!(gEnigmaBerries[0].holdEffect = 0);
            crate::gl!(gEnigmaBerries[2].holdEffect = HOLD_EFFECT_NONE);
            crate::gl!(gEnigmaBerries[0].holdEffectParam = 0);
            crate::gl!(gEnigmaBerries[2].holdEffectParam = 0);
        }
    } else {
        let mut numPlayers: i32 = 0;
        let mut src: *mut BattleEnigmaBerry = null_mut();
        let mut battler: u8 = 0;
        if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
            if gBattleTypeFlags.get() & BATTLE_TYPE_BATTLE_TOWER != 0 {
                numPlayers = 2;
            } else {
                numPlayers = 4;
            }
            for i in 0..numPlayers {
                src = (*gBlockRecvBuffer.as_ptr())[i].as_mut_ptr().at(2) as *mut BattleEnigmaBerry;
                battler = crate::gl!(gLinkPlayers[i].id) as u8;
                j = 0;
                while j < BERRY_NAME_LENGTH {
                    crate::gl!(gEnigmaBerries[battler].name[j] = (*src).name[j]);
                    j += 1;
                }
                crate::gl!(gEnigmaBerries[battler].name[j] = EOS);
                for j in 0..BERRY_ITEM_EFFECT_COUNT {
                    crate::gl!(gEnigmaBerries[battler].itemEffect[j] = (*src).itemEffect[j]);
                }
                crate::gl!(gEnigmaBerries[battler].holdEffect = (*src).holdEffect);
                crate::gl!(gEnigmaBerries[battler].holdEffectParam = (*src).holdEffectParam);
            }
        } else {
            for i in 0..2i32 {
                src = (*gBlockRecvBuffer.as_ptr())[i].as_mut_ptr().at(2) as *mut BattleEnigmaBerry;
                j = 0;
                while j < BERRY_NAME_LENGTH {
                    crate::gl!(gEnigmaBerries[i].name[j] = (*src).name[j]);
                    {
                        let gl_v = (*src).name[j];
                        let gl_i0 = i + 2;
                        crate::gl!(gEnigmaBerries[gl_i0].name[j] = gl_v)
                    };
                    j += 1;
                }
                crate::gl!(gEnigmaBerries[i].name[j] = EOS);
                {
                    let gl_v = EOS;
                    let gl_i1 = i + 2;
                    crate::gl!(gEnigmaBerries[gl_i1].name[j] = gl_v)
                };
                for j in 0..BERRY_ITEM_EFFECT_COUNT {
                    crate::gl!(gEnigmaBerries[i].itemEffect[j] = (*src).itemEffect[j]);
                    {
                        let gl_v = (*src).itemEffect[j];
                        let gl_i2 = i + 2;
                        crate::gl!(gEnigmaBerries[gl_i2].itemEffect[j] = gl_v)
                    };
                }
                crate::gl!(gEnigmaBerries[i].holdEffect = (*src).holdEffect);
                {
                    let gl_v = (*src).holdEffect;
                    let gl_i3 = i + 2;
                    crate::gl!(gEnigmaBerries[gl_i3].holdEffect = gl_v)
                };
                crate::gl!(gEnigmaBerries[i].holdEffectParam = (*src).holdEffectParam);
                {
                    let gl_v = (*src).holdEffectParam;
                    let gl_i4 = i + 2;
                    crate::gl!(gEnigmaBerries[gl_i4].holdEffectParam = gl_v)
                };
            }
        }
    }
}
fn FindLinkBattleMaster(numPlayers: u8, multiPlayerId: u8) {
    let mut found: u8 = 0;
    if crate::gl!(gBlockRecvBuffer[0][0]) == 0x100 {
        if multiPlayerId == 0 {
            gBattleTypeFlags.set(gBattleTypeFlags.get() | (12));
        } else {
            gBattleTypeFlags.set(gBattleTypeFlags.get() | (BATTLE_TYPE_TRAINER));
        }
        found += 1;
    }
    if found == 0 {
        let mut i: i32 = 0;
        while i < numPlayers as i32 {
            if crate::gl!(gBlockRecvBuffer[0][0]) != crate::gl!(gBlockRecvBuffer[i][0]) {
                break;
            }
            i += 1;
        }
        if i == numPlayers as i32 {
            if multiPlayerId == 0 {
                gBattleTypeFlags.set(gBattleTypeFlags.get() | (12));
            } else {
                gBattleTypeFlags.set(gBattleTypeFlags.get() | (BATTLE_TYPE_TRAINER));
            }
            found += 1;
        }
        if found == 0 {
            i = 0;
            while i < numPlayers as i32 {
                if crate::gl!(gBlockRecvBuffer[i][0]) == 0x300
                    && i != multiPlayerId as i32
                    && i < multiPlayerId as i32
                {
                    break;
                }
                if crate::gl!(gBlockRecvBuffer[i][0]) > 0x300 && i != multiPlayerId as i32 {
                    break;
                }
                i += 1;
            }
            if i == numPlayers as i32 {
                gBattleTypeFlags.set(gBattleTypeFlags.get() | (12));
            } else {
                gBattleTypeFlags.set(gBattleTypeFlags.get() | (BATTLE_TYPE_TRAINER));
            }
        }
    }
}
pub(crate) unsafe fn CB2_HandleStartBattle() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    let playerMultiplayerId: u8 = GetMultiplayerId();
    crate::gl!(gBattleScripting.multiplayerId = playerMultiplayerId);
    let enemyMultiplayerId: u8 = playerMultiplayerId ^ BIT_SIDE;
    'l1: {
        let sw1: u8 = crate::gl!(gBattleCommunication[0]);
        let mut fall = false;
        if sw1 == 0 {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                ShowBg(0);
                ShowBg(1);
                ShowBg(2);
                ShowBg(3);
                FillAroundBattleWindows();
                crate::gl!(gBattleCommunication[0] = 1);
            }
            if gWirelessCommType.get() != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
            }
            break 'l1;
        }
        if sw1 == 1 {
            if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
                if gReceivedRemoteLinkPlayers.get() != 0 {
                    if IsLinkTaskFinished() != 0 {
                        (*crate::gl!(gBattleStruct))
                            .multiBuffer
                            .linkBattlerHeader
                            .versionSignatureLo = 0;
                        (*crate::gl!(gBattleStruct))
                            .multiBuffer
                            .linkBattlerHeader
                            .versionSignatureHi = 3;
                        BufferPartyVsScreenHealth_AtStart();
                        SetPlayerBerryDataInBattleStruct();
                        if gTrainerBattleOpponent_A.get() == TRAINER_UNION_ROOM {
                            crate::gl!(gLinkPlayers[0].id = 0);
                            crate::gl!(gLinkPlayers[1].id = 1);
                        }
                        SendBlock(
                            BitmaskAllOtherLinkPlayers(),
                            &raw mut (*crate::gl!(gBattleStruct)).multiBuffer.linkBattlerHeader
                                as *mut c_void,
                            32,
                        );
                        crate::gl!(gBattleCommunication[0] = 2);
                    }
                    if gWirelessCommType.get() != 0 {
                        CreateWirelessStatusIndicatorSprite(0, 0);
                    }
                }
            } else {
                if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED == 0 {
                    gBattleTypeFlags.set(gBattleTypeFlags.get() | (BATTLE_TYPE_IS_MASTER));
                }
                crate::gl!(gBattleCommunication[0] = 15);
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
                    (*crate::gl!(gBattleStruct))
                        .multiBuffer
                        .linkBattlerHeader
                        .vsScreenHealthFlagsLo as i16
                        | ((*crate::gl!(gBattleStruct))
                            .multiBuffer
                            .linkBattlerHeader
                            .vsScreenHealthFlagsHi as i16)
                            << 8,
                );
                task_set(
                    taskId,
                    4,
                    crate::gl!(gBlockRecvBuffer[enemyMultiplayerId][1]) as i16,
                );
                RecordedBattle_SetFrontierPassFlagFromHword(crate::gl!(
                    gBlockRecvBuffer[playerMultiplayerId][1]
                ));
                RecordedBattle_SetFrontierPassFlagFromHword(crate::gl!(
                    gBlockRecvBuffer[enemyMultiplayerId][1]
                ));
                SetDeoxysStats();
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 3 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut c_void,
                    200,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 4 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                memcpy(
                    (*gEnemyParty.as_ptr()).as_mut_ptr() as *mut u8,
                    (*gBlockRecvBuffer.as_ptr())[enemyMultiplayerId].as_mut_ptr() as *mut u8,
                    200,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 7 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    crate::gl!(&raw gPlayerParty[2]) as *mut c_void,
                    200,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 8 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                memcpy(
                    crate::gl!(&raw gEnemyParty[2]) as *mut u8,
                    (*gBlockRecvBuffer.as_ptr())[enemyMultiplayerId].as_mut_ptr() as *mut u8,
                    200,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 11 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    crate::gl!(&raw gPlayerParty[4]) as *mut c_void,
                    200,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 12 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                memcpy(
                    crate::gl!(&raw gEnemyParty[4]) as *mut u8,
                    (*gBlockRecvBuffer.as_ptr())[enemyMultiplayerId].as_mut_ptr() as *mut u8,
                    200,
                );
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[0]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[1]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[2]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[3]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[4]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[5]));
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 15 {
            InitBattleControllers();
            RecordedBattle_SetTrainerInfo();
            crate::gl!(gBattleCommunication[1] = 0);
            crate::gl!(gBattleCommunication[2] = 0);
            if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
                let mut i: i32 = 0;
                while i < 2
                    && crate::gl!(gLinkPlayers[i].version) as i32 & 0xFF == VERSION_EMERALD as i32
                {
                    i += 1;
                }
                if i == 2 {
                    crate::gl!(gBattleCommunication[0] = 16);
                } else {
                    crate::gl!(gBattleCommunication[0] = 18);
                }
            } else {
                crate::gl!(gBattleCommunication[0] = 18);
            }
            break 'l1;
        }
        if sw1 == 16 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    gRecordedBattleRngSeed.as_ptr() as *mut c_void,
                    4,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 17 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if gBattleTypeFlags.get() & BATTLE_TYPE_IS_MASTER == 0 {
                    memcpy(
                        gRecordedBattleRngSeed.as_ptr() as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[enemyMultiplayerId].as_mut_ptr() as *mut u8,
                        4,
                    );
                }
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 18 {
            if BattleInitAllSprites(
                crate::gl!(&raw gBattleCommunication[1]),
                crate::gl!(&raw gBattleCommunication[2]),
            ) != 0
            {
                crate::gl!(gPreBattleCallback1 = crate::gl!(gMain.callback1));
                crate::gl!(gMain.callback1 = Some(BattleMainCB1));
                SetMainCallback2(Some(BattleMainCB2));
                if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
                    gBattleTypeFlags.set(gBattleTypeFlags.get() | (BATTLE_TYPE_LINK_IN_BATTLE));
                }
            }
            break 'l1;
        }
        if sw1 == 5 || sw1 == 9 || sw1 == 13 {
            fall = true;
            crate::gl!(gBattleCommunication[0] += 1);
            crate::gl!(gBattleCommunication[1] = 1);
        }
        if fall || sw1 == 6 || sw1 == 10 || sw1 == 14 {
            if ({
                crate::gl!(gBattleCommunication[1] -= 1);
                crate::gl!(gBattleCommunication[1])
            }) == 0
            {
                crate::gl!(gBattleCommunication[0] += 1);
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
    crate::gl!(gBattleScripting.multiplayerId = playerMultiplayerId);
    let partnerMultiplayerId: u8 = playerMultiplayerId ^ BIT_SIDE;
    'l1: {
        let sw1: u8 = crate::gl!(gBattleCommunication[0]);
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                ShowBg(0);
                ShowBg(1);
                ShowBg(2);
                ShowBg(3);
                FillAroundBattleWindows();
                crate::gl!(gBattleCommunication[0] = 1);
            }
            if gWirelessCommType.get() != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
            }
        }
        if fall || sw1 == 1 {
            if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
                if gReceivedRemoteLinkPlayers.get() != 0 {
                    let mut language: u8 = 0;
                    crate::gl!(gLinkPlayers[0].id = 0);
                    crate::gl!(gLinkPlayers[1].id = 2);
                    crate::gl!(gLinkPlayers[2].id = 1);
                    crate::gl!(gLinkPlayers[3].id = 3);
                    GetFrontierTrainerName(
                        (*gLinkPlayers.as_ptr())[2].name.as_mut_ptr(),
                        gTrainerBattleOpponent_A.get(),
                    );
                    GetFrontierTrainerName(
                        (*gLinkPlayers.as_ptr())[3].name.as_mut_ptr(),
                        gTrainerBattleOpponent_B.get(),
                    );
                    GetBattleTowerTrainerLanguage(
                        &raw mut language,
                        gTrainerBattleOpponent_A.get(),
                    );
                    crate::gl!(gLinkPlayers[2].language = language as u16);
                    GetBattleTowerTrainerLanguage(
                        &raw mut language,
                        gTrainerBattleOpponent_B.get(),
                    );
                    crate::gl!(gLinkPlayers[3].language = language as u16);
                    if IsLinkTaskFinished() != 0 {
                        (*crate::gl!(gBattleStruct))
                            .multiBuffer
                            .linkBattlerHeader
                            .versionSignatureLo = 0;
                        (*crate::gl!(gBattleStruct))
                            .multiBuffer
                            .linkBattlerHeader
                            .versionSignatureHi = 3;
                        BufferPartyVsScreenHealth_AtStart();
                        SetPlayerBerryDataInBattleStruct();
                        SendBlock(
                            BitmaskAllOtherLinkPlayers(),
                            &raw mut (*crate::gl!(gBattleStruct)).multiBuffer.linkBattlerHeader
                                as *mut c_void,
                            32,
                        );
                        crate::gl!(gBattleCommunication[0] = 2);
                    }
                    if gWirelessCommType.get() != 0 {
                        CreateWirelessStatusIndicatorSprite(0, 0);
                    }
                }
            } else {
                if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED == 0 {
                    gBattleTypeFlags.set(gBattleTypeFlags.get() | (BATTLE_TYPE_IS_MASTER));
                }
                crate::gl!(gBattleCommunication[0] = 13);
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
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 3 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut c_void,
                    200,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 4 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if crate::gl!(gLinkPlayers[playerMultiplayerId].id) != 0 {
                    memcpy(
                        (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[partnerMultiplayerId].as_mut_ptr() as *mut u8,
                        200,
                    );
                    memcpy(
                        crate::gl!(&raw gPlayerParty[3]) as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[playerMultiplayerId].as_mut_ptr() as *mut u8,
                        200,
                    );
                } else {
                    memcpy(
                        (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[playerMultiplayerId].as_mut_ptr() as *mut u8,
                        200,
                    );
                    memcpy(
                        crate::gl!(&raw gPlayerParty[3]) as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[partnerMultiplayerId].as_mut_ptr() as *mut u8,
                        200,
                    );
                }
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 5 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    crate::gl!(&raw gPlayerParty[2]) as *mut c_void,
                    100,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 6 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if crate::gl!(gLinkPlayers[playerMultiplayerId].id) != 0 {
                    memcpy(
                        crate::gl!(&raw gPlayerParty[2]) as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[partnerMultiplayerId].as_mut_ptr() as *mut u8,
                        100,
                    );
                    memcpy(
                        crate::gl!(&raw gPlayerParty[5]) as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[playerMultiplayerId].as_mut_ptr() as *mut u8,
                        100,
                    );
                } else {
                    memcpy(
                        crate::gl!(&raw gPlayerParty[2]) as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[playerMultiplayerId].as_mut_ptr() as *mut u8,
                        100,
                    );
                    memcpy(
                        crate::gl!(&raw gPlayerParty[5]) as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[partnerMultiplayerId].as_mut_ptr() as *mut u8,
                        100,
                    );
                }
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 7 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    (*gEnemyParty.as_ptr()).as_mut_ptr() as *mut c_void,
                    200,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 8 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if GetMultiplayerId() != 0 {
                    memcpy(
                        (*gEnemyParty.as_ptr()).as_mut_ptr() as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[0].as_mut_ptr() as *mut u8,
                        200,
                    );
                }
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 9 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    crate::gl!(&raw gEnemyParty[2]) as *mut c_void,
                    200,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 10 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if GetMultiplayerId() != 0 {
                    memcpy(
                        crate::gl!(&raw gEnemyParty[2]) as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[0].as_mut_ptr() as *mut u8,
                        200,
                    );
                }
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 11 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    crate::gl!(&raw gEnemyParty[4]) as *mut c_void,
                    200,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 12 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if GetMultiplayerId() != 0 {
                    memcpy(
                        crate::gl!(&raw gEnemyParty[4]) as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[0].as_mut_ptr() as *mut u8,
                        200,
                    );
                }
                TryCorrectShedinjaLanguage(crate::gl!(&raw gPlayerParty[0]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gPlayerParty[1]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gPlayerParty[2]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gPlayerParty[3]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gPlayerParty[4]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gPlayerParty[5]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[0]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[1]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[2]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[3]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[4]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[5]));
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 13 {
            InitBattleControllers();
            RecordedBattle_SetTrainerInfo();
            crate::gl!(gBattleCommunication[1] = 0);
            crate::gl!(gBattleCommunication[2] = 0);
            if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
                crate::gl!(gBattleCommunication[0] = 14);
            } else {
                crate::gl!(gBattleCommunication[0] = 16);
            }
            break 'l1;
        }
        if sw1 == 14 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    gRecordedBattleRngSeed.as_ptr() as *mut c_void,
                    4,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 15 {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if gBattleTypeFlags.get() & BATTLE_TYPE_IS_MASTER == 0 {
                    memcpy(
                        gRecordedBattleRngSeed.as_ptr() as *mut u8,
                        (*gBlockRecvBuffer.as_ptr())[partnerMultiplayerId].as_mut_ptr() as *mut u8,
                        4,
                    );
                }
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 16 {
            if BattleInitAllSprites(
                crate::gl!(&raw gBattleCommunication[1]),
                crate::gl!(&raw gBattleCommunication[2]),
            ) != 0
            {
                TrySetLinkBattleTowerEnemyPartyLevel();
                crate::gl!(gPreBattleCallback1 = crate::gl!(gMain.callback1));
                crate::gl!(gMain.callback1 = Some(BattleMainCB1));
                SetMainCallback2(Some(BattleMainCB2));
                if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
                    gBattleTypeFlags.set(gBattleTypeFlags.get() | (BATTLE_TYPE_LINK_IN_BATTLE));
                }
            }
            break 'l1;
        }
    }
}
unsafe fn SetMultiPartnerMenuParty(offset: u8) {
    for i in 0..MULTI_PARTY_SIZE {
        crate::gl!(
            gMultiPartnerParty[i].species = GetMonData2(
                {
                    let gl_i0 = offset as i32 + i;
                    crate::gl!(&raw gPlayerParty[gl_i0])
                },
                MON_DATA_SPECIES
            ) as u16
        );
        crate::gl!(
            gMultiPartnerParty[i].heldItem = GetMonData2(
                {
                    let gl_i1 = offset as i32 + i;
                    crate::gl!(&raw gPlayerParty[gl_i1])
                },
                MON_DATA_HELD_ITEM
            ) as u16
        );
        GetMonData3(
            {
                let gl_i2 = offset as i32 + i;
                crate::gl!(&raw gPlayerParty[gl_i2])
            },
            MON_DATA_NICKNAME,
            (*gMultiPartnerParty.as_ptr())[i].nickname.as_mut_ptr(),
        );
        crate::gl!(
            gMultiPartnerParty[i].level = GetMonData2(
                {
                    let gl_i3 = offset as i32 + i;
                    crate::gl!(&raw gPlayerParty[gl_i3])
                },
                MON_DATA_LEVEL
            ) as u8
        );
        crate::gl!(
            gMultiPartnerParty[i].hp = GetMonData2(
                {
                    let gl_i4 = offset as i32 + i;
                    crate::gl!(&raw gPlayerParty[gl_i4])
                },
                MON_DATA_HP
            ) as u16
        );
        crate::gl!(
            gMultiPartnerParty[i].maxhp = GetMonData2(
                {
                    let gl_i5 = offset as i32 + i;
                    crate::gl!(&raw gPlayerParty[gl_i5])
                },
                MON_DATA_MAX_HP
            ) as u16
        );
        crate::gl!(
            gMultiPartnerParty[i].status = GetMonData2(
                {
                    let gl_i6 = offset as i32 + i;
                    crate::gl!(&raw gPlayerParty[gl_i6])
                },
                MON_DATA_STATUS
            )
        );
        crate::gl!(
            gMultiPartnerParty[i].personality = GetMonData2(
                {
                    let gl_i7 = offset as i32 + i;
                    crate::gl!(&raw gPlayerParty[gl_i7])
                },
                MON_DATA_PERSONALITY,
            )
        );
        crate::gl!(
            gMultiPartnerParty[i].gender = GetMonGender({
                let gl_i8 = offset as i32 + i;
                crate::gl!(&raw gPlayerParty[gl_i8])
            })
        );
        StripExtCtrlCodes((*gMultiPartnerParty.as_ptr())[i].nickname.as_mut_ptr());
        if GetMonData2(
            {
                let gl_i9 = offset as i32 + i;
                crate::gl!(&raw gPlayerParty[gl_i9])
            },
            MON_DATA_LANGUAGE,
        ) != LANGUAGE_JAPANESE as u32
        {
            PadNameString(
                (*gMultiPartnerParty.as_ptr())[i].nickname.as_mut_ptr(),
                CHAR_SPACE,
            );
        }
    }
    memcpy(
        crate::gl!(sMultiPartnerPartyBuffer) as *mut u8,
        (*gMultiPartnerParty.as_ptr()).as_mut_ptr() as *mut u8,
        96,
    );
}
pub(crate) unsafe fn CB2_PreInitMultiBattle() {
    let mut numPlayers: i32 = MAX_BATTLERS_COUNT as i32;
    let mut blockMask: u8 = 0xF;
    let mut savedBattleTypeFlags: *mut u32 = null_mut();
    let mut savedCallback: *mut Option<unsafe fn()> = null_mut();
    if gBattleTypeFlags.get() & BATTLE_TYPE_BATTLE_TOWER != 0 {
        numPlayers = 2;
        blockMask = 3;
    }
    let playerMultiplierId: u8 = GetMultiplayerId();
    crate::gl!(gBattleScripting.multiplayerId = playerMultiplierId);
    savedCallback = &raw mut (*crate::gl!(gBattleStruct)).savedCallback;
    savedBattleTypeFlags = &raw mut (*crate::gl!(gBattleStruct)).savedBattleTypeFlags;
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    match crate::gl!(gBattleCommunication[0]) {
        0 => {
            if gReceivedRemoteLinkPlayers.get() != 0 && IsLinkTaskFinished() != 0 {
                crate::gl!(sMultiPartnerPartyBuffer = Alloc(96) as *mut MultiPartnerMenuPokemon);
                SetMultiPartnerMenuParty(0);
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    crate::gl!(sMultiPartnerPartyBuffer) as *mut c_void,
                    96,
                );
                crate::gl!(gBattleCommunication[0] += 1);
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
                            if crate::gl!(gLinkPlayers[i].id) as i32 & 1 == 0
                                && crate::gl!(gLinkPlayers[playerMultiplierId].id) as i32 & 1 == 0
                                || crate::gl!(gLinkPlayers[i].id) as i32 & 1 != 0
                                    && crate::gl!(gLinkPlayers[playerMultiplierId].id) as i32 & 1
                                        != 0
                            {
                                memcpy(
                                    (*gMultiPartnerParty.as_ptr()).as_mut_ptr() as *mut u8,
                                    (*gBlockRecvBuffer.as_ptr())[i].as_mut_ptr() as *mut u8,
                                    96,
                                );
                            }
                        } else {
                            memcpy(
                                (*gMultiPartnerParty.as_ptr()).as_mut_ptr() as *mut u8,
                                (*gBlockRecvBuffer.as_ptr())[i].as_mut_ptr() as *mut u8,
                                96,
                            );
                        }
                    }
                }
                crate::gl!(gBattleCommunication[0] += 1);
                *savedCallback = crate::gl!(gMain.savedCallback);
                *savedBattleTypeFlags = gBattleTypeFlags.get();
                crate::gl!(gMain.savedCallback = Some(CB2_PreInitMultiBattle));
                ShowPartyMenuToShowcaseMultiBattleParty();
            }
        }
        2 => {
            if IsLinkTaskFinished() != 0 && (*gPaletteFade.as_ptr()).active() == 0 {
                crate::gl!(gBattleCommunication[0] += 1);
                if gWirelessCommType.get() != 0 {
                    SetLinkStandbyCallback();
                } else {
                    SetCloseLinkCallback();
                }
            }
        }
        3 => {
            if gWirelessCommType.get() != 0 {
                if IsLinkRfuTaskFinished() != 0 {
                    gBattleTypeFlags.set(*savedBattleTypeFlags);
                    crate::gl!(gMain.savedCallback = *savedCallback);
                    SetMainCallback2(Some(CB2_InitBattleInternal));
                    Free(crate::gl!(sMultiPartnerPartyBuffer) as *mut c_void);
                    crate::gl!(sMultiPartnerPartyBuffer = null_mut());
                }
            } else if gReceivedRemoteLinkPlayers.get() == 0 {
                gBattleTypeFlags.set(*savedBattleTypeFlags);
                crate::gl!(gMain.savedCallback = *savedCallback);
                SetMainCallback2(Some(CB2_InitBattleInternal));
                Free(crate::gl!(sMultiPartnerPartyBuffer) as *mut c_void);
                crate::gl!(sMultiPartnerPartyBuffer = null_mut());
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn CB2_PreInitIngamePlayerPartnerBattle() {
    let mut savedBattleTypeFlags: *mut u32 = null_mut();
    let mut savedCallback: *mut Option<unsafe fn()> = null_mut();
    savedCallback = &raw mut (*crate::gl!(gBattleStruct)).savedCallback;
    savedBattleTypeFlags = &raw mut (*crate::gl!(gBattleStruct)).savedBattleTypeFlags;
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    match crate::gl!(gBattleCommunication[0]) {
        0 => {
            crate::gl!(sMultiPartnerPartyBuffer = Alloc(96) as *mut MultiPartnerMenuPokemon);
            SetMultiPartnerMenuParty(MULTI_PARTY_SIZE as u8);
            crate::gl!(gBattleCommunication[0] += 1);
            *savedCallback = crate::gl!(gMain.savedCallback);
            *savedBattleTypeFlags = gBattleTypeFlags.get();
            crate::gl!(gMain.savedCallback = Some(CB2_PreInitIngamePlayerPartnerBattle));
            ShowPartyMenuToShowcaseMultiBattleParty();
        }
        1 if (*gPaletteFade.as_ptr()).active() == 0 => {
            crate::gl!(gBattleCommunication[0] = 2);
            gBattleTypeFlags.set(*savedBattleTypeFlags);
            crate::gl!(gMain.savedCallback = *savedCallback);
            SetMainCallback2(Some(CB2_InitBattleInternal));
            Free(crate::gl!(sMultiPartnerPartyBuffer) as *mut c_void);
            crate::gl!(sMultiPartnerPartyBuffer = null_mut());
        }
        _ => {}
    }
}
pub(crate) unsafe fn CB2_HandleStartMultiBattle() {
    let mut id: i32 = 0;
    let mut var: u8 = 0;
    let playerMultiplayerId: u8 = GetMultiplayerId();
    crate::gl!(gBattleScripting.multiplayerId = playerMultiplayerId);
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    'l1: {
        let sw1: u8 = crate::gl!(gBattleCommunication[0]);
        let mut fall = false;
        if sw1 == 0 {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                ShowBg(0);
                ShowBg(1);
                ShowBg(2);
                ShowBg(3);
                FillAroundBattleWindows();
                crate::gl!(gBattleCommunication[0] = 1);
            }
            if gWirelessCommType.get() != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
            }
            break 'l1;
        }
        if sw1 == 1 {
            if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
                if gReceivedRemoteLinkPlayers.get() != 0 {
                    if IsLinkTaskFinished() != 0 {
                        (*crate::gl!(gBattleStruct))
                            .multiBuffer
                            .linkBattlerHeader
                            .versionSignatureLo = 0;
                        (*crate::gl!(gBattleStruct))
                            .multiBuffer
                            .linkBattlerHeader
                            .versionSignatureHi = 3;
                        BufferPartyVsScreenHealth_AtStart();
                        SetPlayerBerryDataInBattleStruct();
                        SendBlock(
                            BitmaskAllOtherLinkPlayers(),
                            &raw mut (*crate::gl!(gBattleStruct)).multiBuffer.linkBattlerHeader
                                as *mut c_void,
                            32,
                        );
                        crate::gl!(gBattleCommunication[0] += 1);
                    }
                    if gWirelessCommType.get() != 0 {
                        CreateWirelessStatusIndicatorSprite(0, 0);
                    }
                }
            } else {
                if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED == 0 {
                    gBattleTypeFlags.set(gBattleTypeFlags.get() | (BATTLE_TYPE_IS_MASTER));
                }
                crate::gl!(gBattleCommunication[0] = 7);
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
                    RecordedBattle_SetFrontierPassFlagFromHword(crate::gl!(
                        gBlockRecvBuffer[id][1]
                    ));
                    match crate::gl!(gLinkPlayers[id].id) {
                        0 => {
                            task_set(
                                var,
                                3,
                                task_get(var, 3)
                                    | (crate::gl!(gBlockRecvBuffer[id][1]) as i16 & 0x3F),
                            );
                        }
                        1 => {
                            task_set(
                                var,
                                4,
                                task_get(var, 4)
                                    | (crate::gl!(gBlockRecvBuffer[id][1]) as i16 & 0x3F),
                            );
                        }
                        2 => {
                            task_set(
                                var,
                                3,
                                task_get(var, 3)
                                    | ((crate::gl!(gBlockRecvBuffer[id][1]) as i16 & 0x3F) << 6),
                            );
                        }
                        3 => {
                            task_set(
                                var,
                                4,
                                task_get(var, 4)
                                    | ((crate::gl!(gBlockRecvBuffer[id][1]) as i16 & 0x3F) << 6),
                            );
                        }
                        _ => {}
                    }
                }
                ZeroEnemyPartyMons();
                crate::gl!(gBattleCommunication[0] += 1);
            } else {
                break 'l1;
            }
        }
        if fall || sw1 == 3 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut c_void,
                    200,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 4 {
            if GetBlockReceivedStatus() as i32 & 0xF == 0xF {
                ResetBlockReceivedFlags();
                for id in 0..MAX_LINK_PLAYERS {
                    if id == playerMultiplayerId as i32 {
                        match crate::gl!(gLinkPlayers[id].id) {
                            0 | 3 => {
                                memcpy(
                                    (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut u8,
                                    (*gBlockRecvBuffer.as_ptr())[id].as_mut_ptr() as *mut u8,
                                    200,
                                );
                            }
                            1 | 2 => {
                                memcpy(
                                    (*gPlayerParty.as_ptr()).as_mut_ptr().at(3) as *mut u8,
                                    (*gBlockRecvBuffer.as_ptr())[id].as_mut_ptr() as *mut u8,
                                    200,
                                );
                            }
                            _ => {}
                        }
                    } else {
                        if crate::gl!(gLinkPlayers[id].id) as i32 & 1 == 0
                            && crate::gl!(gLinkPlayers[playerMultiplayerId].id) as i32 & 1 == 0
                            || crate::gl!(gLinkPlayers[id].id) as i32 & 1 != 0
                                && crate::gl!(gLinkPlayers[playerMultiplayerId].id) as i32 & 1 != 0
                        {
                            match crate::gl!(gLinkPlayers[id].id) {
                                0 | 3 => {
                                    memcpy(
                                        (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut u8,
                                        (*gBlockRecvBuffer.as_ptr())[id].as_mut_ptr() as *mut u8,
                                        200,
                                    );
                                }
                                1 | 2 => {
                                    memcpy(
                                        (*gPlayerParty.as_ptr()).as_mut_ptr().at(3) as *mut u8,
                                        (*gBlockRecvBuffer.as_ptr())[id].as_mut_ptr() as *mut u8,
                                        200,
                                    );
                                }
                                _ => {}
                            }
                        } else {
                            match crate::gl!(gLinkPlayers[id].id) {
                                0 | 3 => {
                                    memcpy(
                                        (*gEnemyParty.as_ptr()).as_mut_ptr() as *mut u8,
                                        (*gBlockRecvBuffer.as_ptr())[id].as_mut_ptr() as *mut u8,
                                        200,
                                    );
                                }
                                1 | 2 => {
                                    memcpy(
                                        (*gEnemyParty.as_ptr()).as_mut_ptr().at(3) as *mut u8,
                                        (*gBlockRecvBuffer.as_ptr())[id].as_mut_ptr() as *mut u8,
                                        200,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                }
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 5 {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    (*gPlayerParty.as_ptr()).as_mut_ptr().at(2) as *mut c_void,
                    100,
                );
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 6 {
            if GetBlockReceivedStatus() as i32 & 0xF == 0xF {
                ResetBlockReceivedFlags();
                for id in 0..MAX_LINK_PLAYERS {
                    if id == playerMultiplayerId as i32 {
                        match crate::gl!(gLinkPlayers[id].id) {
                            0 | 3 => {
                                memcpy(
                                    (*gPlayerParty.as_ptr()).as_mut_ptr().at(2) as *mut u8,
                                    (*gBlockRecvBuffer.as_ptr())[id].as_mut_ptr() as *mut u8,
                                    100,
                                );
                            }
                            1 | 2 => {
                                memcpy(
                                    (*gPlayerParty.as_ptr()).as_mut_ptr().at(5) as *mut u8,
                                    (*gBlockRecvBuffer.as_ptr())[id].as_mut_ptr() as *mut u8,
                                    100,
                                );
                            }
                            _ => {}
                        }
                    } else {
                        if crate::gl!(gLinkPlayers[id].id) as i32 & 1 == 0
                            && crate::gl!(gLinkPlayers[playerMultiplayerId].id) as i32 & 1 == 0
                            || crate::gl!(gLinkPlayers[id].id) as i32 & 1 != 0
                                && crate::gl!(gLinkPlayers[playerMultiplayerId].id) as i32 & 1 != 0
                        {
                            match crate::gl!(gLinkPlayers[id].id) {
                                0 | 3 => {
                                    memcpy(
                                        (*gPlayerParty.as_ptr()).as_mut_ptr().at(2) as *mut u8,
                                        (*gBlockRecvBuffer.as_ptr())[id].as_mut_ptr() as *mut u8,
                                        100,
                                    );
                                }
                                1 | 2 => {
                                    memcpy(
                                        (*gPlayerParty.as_ptr()).as_mut_ptr().at(5) as *mut u8,
                                        (*gBlockRecvBuffer.as_ptr())[id].as_mut_ptr() as *mut u8,
                                        100,
                                    );
                                }
                                _ => {}
                            }
                        } else {
                            match crate::gl!(gLinkPlayers[id].id) {
                                0 | 3 => {
                                    memcpy(
                                        (*gEnemyParty.as_ptr()).as_mut_ptr().at(2) as *mut u8,
                                        (*gBlockRecvBuffer.as_ptr())[id].as_mut_ptr() as *mut u8,
                                        100,
                                    );
                                }
                                1 | 2 => {
                                    memcpy(
                                        (*gEnemyParty.as_ptr()).as_mut_ptr().at(5) as *mut u8,
                                        (*gBlockRecvBuffer.as_ptr())[id].as_mut_ptr() as *mut u8,
                                        100,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                }
                TryCorrectShedinjaLanguage(crate::gl!(&raw gPlayerParty[0]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gPlayerParty[1]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gPlayerParty[2]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gPlayerParty[3]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gPlayerParty[4]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gPlayerParty[5]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[0]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[1]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[2]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[3]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[4]));
                TryCorrectShedinjaLanguage(crate::gl!(&raw gEnemyParty[5]));
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 7 {
            InitBattleControllers();
            RecordedBattle_SetTrainerInfo();
            crate::gl!(gBattleCommunication[1] = 0);
            crate::gl!(gBattleCommunication[2] = 0);
            if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
                id = 0;
                while id < MAX_LINK_PLAYERS
                    && crate::gl!(gLinkPlayers[id].version) as i32 & 0xFF == VERSION_EMERALD as i32
                {
                    id += 1;
                }
                if id == MAX_LINK_PLAYERS {
                    crate::gl!(gBattleCommunication[0] = 8);
                } else {
                    crate::gl!(gBattleCommunication[0] = 10);
                }
            } else {
                crate::gl!(gBattleCommunication[0] = 10);
            }
            break 'l1;
        }
        if sw1 == 8 {
            if IsLinkTaskFinished() != 0 {
                let ptr: *mut u32 = (*crate::gl!(gBattleStruct))
                    .multiBuffer
                    .battleVideo
                    .as_mut_ptr();
                *ptr = gBattleTypeFlags.get();
                *ptr.at(1) = gRecordedBattleRngSeed.get();
                SendBlock(BitmaskAllOtherLinkPlayers(), ptr as *mut c_void, 8);
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 9 {
            if GetBlockReceivedStatus() as i32 & 0xF == 0xF {
                ResetBlockReceivedFlags();
                for var in 0..4u8 {
                    let blockValue: u32 = crate::gl!(gBlockRecvBuffer[var][0]) as u32;
                    if blockValue & 4 != 0 {
                        memcpy(
                            gRecordedBattleRngSeed.as_ptr() as *mut u8,
                            crate::gl!(&raw gBlockRecvBuffer[var][2]) as *mut u8,
                            4,
                        );
                        break;
                    }
                }
                crate::gl!(gBattleCommunication[0] += 1);
            }
            break 'l1;
        }
        if sw1 == 10 {
            if BattleInitAllSprites(
                crate::gl!(&raw gBattleCommunication[1]),
                crate::gl!(&raw gBattleCommunication[2]),
            ) != 0
            {
                crate::gl!(gPreBattleCallback1 = crate::gl!(gMain.callback1));
                crate::gl!(gMain.callback1 = Some(BattleMainCB1));
                SetMainCallback2(Some(BattleMainCB2));
                if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
                    gTrainerBattleOpponent_A.set(TRAINER_LINK_OPPONENT);
                    gBattleTypeFlags.set(gBattleTypeFlags.get() | (BATTLE_TYPE_LINK_IN_BATTLE));
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
    if crate::gl!(gMain.heldKeys) as i32 & B_BUTTON != 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED != 0
        && RecordedBattle_CanStopPlayback() != 0
    {
        gSpecialVar_Result = ({
            gBattleOutcome.set(B_OUTCOME_PLAYER_TELEPORTED);
            gBattleOutcome.get()
        }) as u16;
        ResetPaletteFadeControl();
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        SetMainCallback2(Some(CB2_QuitRecordedBattle));
    }
}
unsafe fn FreeRestoreBattleData() {
    crate::gl!(gMain.callback1 = crate::gl!(gPreBattleCallback1));
    crate::gl!(gScanlineEffect.state = 3);
    (*gMain.as_ptr()).set_inBattle(FALSE);
    ZeroEnemyPartyMons();
    m4aSongNumStop(SE_LOW_HEALTH);
    FreeMonSpritesGfx();
    FreeBattleSpritesData();
    FreeBattleResources();
}
pub unsafe fn CB2_QuitRecordedBattle() {
    UpdatePaletteFade();
    if (*gPaletteFade.as_ptr()).active() == 0 {
        m4aMPlayStop(crate::gl!(&raw gMPlayInfo_SE1));
        m4aMPlayStop(crate::gl!(&raw gMPlayInfo_SE2));
        FreeRestoreBattleData();
        FreeAllWindowBuffers();
        SetMainCallback2(crate::gl!(gMain.savedCallback));
    }
}
pub(crate) unsafe fn SpriteCB_UnusedBattleInit(sprite: *mut Sprite) {
    (*sprite).data[sState] = 0;
    (*sprite).callback = Some(SpriteCB_UnusedBattleInit_Main);
}
pub(crate) unsafe fn SpriteCB_UnusedBattleInit_Main(sprite: *mut Sprite) {
    let arr: *mut u16 = (*(crate::gl!(&raw @at crate::decompress::gDecompressionBuffer;)
        .cast_const())
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
    if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0 && gBattleTypeFlags.get() & 0x43f0900 == 0
    {
        if firstTrainer == TRUE {
            ZeroEnemyPartyMons();
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0 {
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
        {
            let rhs = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[trainerNum]
                .doubleBattle as u32;
            gBattleTypeFlags.set(gBattleTypeFlags.get() | rhs)
        };
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
    if gBattleTypeFlags.get() & 0x13f0102 == 0 {
        Random();
    }
    SetGpuReg(REG_OFFSET_BG0HOFS, gBattle_BG0_X.get());
    SetGpuReg(REG_OFFSET_BG0VOFS, gBattle_BG0_Y.get());
    SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X.get());
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y.get());
    SetGpuReg(REG_OFFSET_BG2HOFS, gBattle_BG2_X.get());
    SetGpuReg(REG_OFFSET_BG2VOFS, gBattle_BG2_Y.get());
    SetGpuReg(REG_OFFSET_BG3HOFS, gBattle_BG3_X.get());
    SetGpuReg(REG_OFFSET_BG3VOFS, gBattle_BG3_Y.get());
    SetGpuReg(REG_OFFSET_WIN0H, gBattle_WIN0H.get());
    SetGpuReg(REG_OFFSET_WIN0V, gBattle_WIN0V.get());
    SetGpuReg(REG_OFFSET_WIN1H, gBattle_WIN1H.get());
    SetGpuReg(REG_OFFSET_WIN1V, gBattle_WIN1V.get());
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
    let multiplayerId: u8 = crate::gl!(gBattleScripting.multiplayerId);
    if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
        match crate::gl!(gLinkPlayers[multiplayerId].id) {
            0 | 2 => {
                party1 = (*gPlayerParty.as_ptr()).as_mut_ptr();
                party2 = (*gEnemyParty.as_ptr()).as_mut_ptr();
            }
            1 | 3 => {
                party1 = (*gEnemyParty.as_ptr()).as_mut_ptr();
                party2 = (*gPlayerParty.as_ptr()).as_mut_ptr();
            }
            _ => {}
        }
    } else {
        party1 = (*gPlayerParty.as_ptr()).as_mut_ptr();
        party2 = (*gEnemyParty.as_ptr()).as_mut_ptr();
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
    gBattleTypeFlags.set(gBattleTypeFlags.get() & (0xffffffdf));
    if gBattleTypeFlags.get() & BATTLE_TYPE_FRONTIER != 0 {
        SetMainCallback2(crate::gl!(gMain.savedCallback));
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
        gBattle_WIN0H.set(DISPLAY_WIDTH);
        gBattle_WIN0V.set(20561);
        ScanlineEffect_Clear();
        i = 0;
        while i < 80 {
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = 0xF0;
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = 0xF0;
            i += 1;
        }
        while i < 160 {
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = 0xFF10;
            (*(crate::gl!(&raw @at crate::scanline_effect::gScanlineEffectRegBuffers;)
                .cast_const())
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = 0xFF10;
            i += 1;
        }
        ResetPaletteFade();
        gBattle_BG0_X.set(0);
        gBattle_BG0_Y.set(0);
        gBattle_BG1_X.set(0);
        gBattle_BG1_Y.set(0);
        gBattle_BG2_X.set(0);
        gBattle_BG2_Y.set(0);
        gBattle_BG3_X.set(0);
        gBattle_BG3_Y.set(0);
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
        gReservedSpritePaletteCount.set(MAX_BATTLERS_COUNT);
        SetVBlankCallback(Some(VBlankCB_Battle));
        taskId = CreateTask(Some(InitLinkBattleVsScreen), 0);
        task_set(taskId, 1, 0x10E);
        task_set(taskId, 2, 0x5A);
        task_set(taskId, 5, 1);
        BufferPartyVsScreenHealth_AtEnd(taskId);
        SetMainCallback2(Some(CB2_EndLinkBattle));
        crate::gl!(gBattleCommunication[0] = 0);
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
    match crate::gl!(gBattleCommunication[0]) {
        0 => {
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            crate::gl!(gBattleCommunication[1] = 0xFF);
            crate::gl!(gBattleCommunication[0] += 1);
        }
        1 => {
            if ({
                crate::gl!(gBattleCommunication[1] -= 1);
                crate::gl!(gBattleCommunication[1])
            }) == 0
            {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        2 => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                let mut battlerCount: u8 = 0;
                (*gMain.as_ptr())
                    .set_anyLinkBattlerHasFrontierPass(RecordedBattle_GetFrontierPassFlag());
                if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
                    battlerCount = 4;
                } else {
                    battlerCount = 2;
                }
                i = 0;
                while i < battlerCount as i32
                    && crate::gl!(gLinkPlayers[i].version) as i32 & 0xFF == VERSION_EMERALD as i32
                {
                    i += 1;
                }
                if (*crate::gl!(gSaveBlock2Ptr)).frontier.disableRecordBattle() == 0
                    && i == battlerCount as i32
                {
                    if FlagGet(FLAG_SYS_FRONTIER_PASS) != 0 {
                        FreeAllWindowBuffers();
                        SetMainCallback2(Some(CB2_InitAskRecordBattle));
                    } else if (*gMain.as_ptr()).anyLinkBattlerHasFrontierPass() == 0 {
                        SetMainCallback2(crate::gl!(gMain.savedCallback));
                        FreeBattleResources();
                        FreeBattleSpritesData();
                        FreeMonSpritesGfx();
                    } else if gReceivedRemoteLinkPlayers.get() == 0 {
                        CreateTask(Some(Task_ReconnectWithLinkPlayers), 5);
                        crate::gl!(gBattleCommunication[0] += 1);
                    } else {
                        crate::gl!(gBattleCommunication[0] += 1);
                    }
                } else {
                    SetMainCallback2(crate::gl!(gMain.savedCallback));
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
            crate::gl!(gBattleCommunication[0] += 1);
        }
        4 => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        5 => {
            if FuncIsActiveTask(Some(Task_ReconnectWithLinkPlayers)) == 0 {
                crate::gl!(gBattleCommunication[0] += 1);
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
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        7 => {
            if IsTextPrinterActive(B_WIN_MSG) == 0 && IsLinkTaskFinished() == TRUE {
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        8 => {
            if gWirelessCommType.get() == 0 {
                SetCloseLinkCallback();
            }
            crate::gl!(gBattleCommunication[0] += 1);
        }
        9 if ((*gMain.as_ptr()).anyLinkBattlerHasFrontierPass() == 0
            || gWirelessCommType.get() != 0
            || gReceivedRemoteLinkPlayers.get() != 1) =>
        {
            (*gMain.as_ptr()).set_anyLinkBattlerHasFrontierPass(FALSE);
            SetMainCallback2(crate::gl!(gMain.savedCallback));
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
    gBattle_BG0_X.set(0);
    gBattle_BG0_Y.set(0);
    gBattle_BG1_X.set(0);
    gBattle_BG1_Y.set(0);
    gBattle_BG2_X.set(0);
    gBattle_BG2_Y.set(0);
    gBattle_BG3_X.set(0);
    gBattle_BG3_Y.set(0);
    InitBattleBgsVideo();
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    LoadBattleMenuWindowGfx();
    for i in 0..2i32 {
        LoadChosenBattleElement(i as u8);
    }
    ResetSpriteData();
    ResetTasks();
    FreeAllSpritePalettes();
    gReservedSpritePaletteCount.set(MAX_BATTLERS_COUNT);
    SetVBlankCallback(Some(VBlankCB_Battle));
    SetMainCallback2(Some(CB2_AskRecordBattle));
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
    crate::gl!(gBattleCommunication[0] = 0);
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
    match crate::gl!(gBattleCommunication[0]) {
        STATE_INIT => {
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            crate::gl!(gBattleCommunication[0] += 1);
        }
        STATE_LINK => {
            if (*gMain.as_ptr()).anyLinkBattlerHasFrontierPass() != 0
                && gReceivedRemoteLinkPlayers.get() == 0
            {
                CreateTask(Some(Task_ReconnectWithLinkPlayers), 5);
            }
            crate::gl!(gBattleCommunication[0] += 1);
        }
        STATE_WAIT_LINK => {
            if FuncIsActiveTask(Some(Task_ReconnectWithLinkPlayers)) == 0 {
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        STATE_ASK_RECORD => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                BattlePutTextOnWindow(
                    (*(&raw const crate::data::battle_message::gText_RecordBattleToPass)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    B_WIN_MSG,
                );
                crate::gl!(gBattleCommunication[0] += 1);
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
                crate::gl!(gBattleCommunication[1] = 1);
                BattleCreateYesNoCursorAt(1);
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        STATE_HANDLE_YES_NO => {
            if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0 {
                if crate::gl!(gBattleCommunication[1]) != 0 {
                    PlaySE(SE_SELECT);
                    BattleDestroyYesNoCursorAt(crate::gl!(gBattleCommunication[1]));
                    crate::gl!(gBattleCommunication[1] = 0);
                    BattleCreateYesNoCursorAt(0);
                }
            } else if crate::gl!(gMain.newKeys) as i32 & DPAD_DOWN != 0 {
                if crate::gl!(gBattleCommunication[1]) == 0 {
                    PlaySE(SE_SELECT);
                    BattleDestroyYesNoCursorAt(crate::gl!(gBattleCommunication[1]));
                    crate::gl!(gBattleCommunication[1] = 1);
                    BattleCreateYesNoCursorAt(1);
                }
            } else if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if crate::gl!(gBattleCommunication[1]) == 0 {
                    HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                    crate::gl!(gBattleCommunication[1] = MoveRecordedBattleToSaveData() as u8);
                    crate::gl!(gBattleCommunication[0] = STATE_RECORD_YES);
                } else {
                    crate::gl!(gBattleCommunication[0] += 1);
                }
            } else if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        STATE_RECORD_NO => {
            if IsLinkTaskFinished() == TRUE {
                HandleBattleWindow(24, 8, 29, 13, WINDOW_CLEAR);
                if (*gMain.as_ptr()).anyLinkBattlerHasFrontierPass() != 0 {
                    SetLinkStandbyCallback();
                    BattlePutTextOnWindow(
                        (*crate::asmdata::gText_LinkStandby3.cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        B_WIN_MSG,
                    );
                }
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        STATE_WAIT_END => {
            if ({
                crate::gl!(gBattleCommunication[1] -= 1);
                crate::gl!(gBattleCommunication[1])
            }) == 0
            {
                if (*gMain.as_ptr()).anyLinkBattlerHasFrontierPass() != 0
                    && gWirelessCommType.get() == 0
                {
                    SetCloseLinkCallback();
                }
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        STATE_END => {
            if (*gMain.as_ptr()).anyLinkBattlerHasFrontierPass() == 0
                || gWirelessCommType.get() != 0
                || gReceivedRemoteLinkPlayers.get() != 1
            {
                (*gMain.as_ptr()).set_anyLinkBattlerHasFrontierPass(FALSE);
                if (*gPaletteFade.as_ptr()).active() == 0 {
                    SetMainCallback2(crate::gl!(gMain.savedCallback));
                    FreeBattleResources();
                    FreeBattleSpritesData();
                    FreeMonSpritesGfx();
                }
            }
        }
        STATE_RECORD_YES => {
            if crate::gl!(gBattleCommunication[1]) == 1 {
                PlaySE(SE_SAVE);
                BattleStringExpandPlaceholdersToDisplayedString(
                    (*(&raw const crate::data::battle_message::gText_BattleRecordedOnPass)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                BattlePutTextOnWindow((*gDisplayedStringBattle.as_ptr()).as_mut_ptr(), B_WIN_MSG);
                crate::gl!(gBattleCommunication[1] = 128);
                crate::gl!(gBattleCommunication[0] += 1);
            } else {
                BattleStringExpandPlaceholdersToDisplayedString(
                    (*crate::asmdata::BattleFrontier_BattleTowerBattleRoom_Text_RecordCouldntBeSaved.cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
                BattlePutTextOnWindow((*gDisplayedStringBattle.as_ptr()).as_mut_ptr(), B_WIN_MSG);
                crate::gl!(gBattleCommunication[1] = 128);
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        STATE_RECORD_WAIT => {
            if IsLinkTaskFinished() == 1
                && IsTextPrinterActive(0) == 0
                && ({
                    crate::gl!(gBattleCommunication[1] -= 1);
                    crate::gl!(gBattleCommunication[1])
                }) == 0
            {
                if (*gMain.as_ptr()).anyLinkBattlerHasFrontierPass() != 0 {
                    SetLinkStandbyCallback();
                    BattlePutTextOnWindow(
                        (*crate::asmdata::gText_LinkStandby3.cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        B_WIN_MSG,
                    );
                }
                crate::gl!(gBattleCommunication[0] += 1);
            }
        }
        STATE_END_RECORD_YES | STATE_END_RECORD_NO if IsTextPrinterActive(B_WIN_MSG) == 0 => {
            if (*gMain.as_ptr()).anyLinkBattlerHasFrontierPass() != 0 {
                if IsLinkTaskFinished() == TRUE {
                    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                    crate::gl!(gBattleCommunication[1] = 32);
                    crate::gl!(gBattleCommunication[0] = STATE_WAIT_END);
                }
            } else {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                crate::gl!(gBattleCommunication[1] = 32);
                crate::gl!(gBattleCommunication[0] = STATE_WAIT_END);
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
    if gIntroSlideFlags.get() as i32 & 1 == 0 {
        (*sprite).x2 += 2;
        if (*sprite).x2 == 0 {
            (*sprite).callback = Some(SpriteCB_WildMonShowHealthbox);
        }
    }
}
pub(crate) unsafe fn SpriteCB_WildMonShowHealthbox(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        StartHealthboxSlideIn((*sprite).data[sBattler] as u8);
        SetHealthboxSpriteVisible({
            let gl_i10 = (*sprite).data[sBattler];
            crate::gl!(gHealthboxSpriteIds[gl_i10])
        });
        (*sprite).callback = Some(SpriteCB_WildMonAnimate);
        StartSpriteAnimIfDifferent(sprite, 0);
        BeginNormalPaletteFade(0x20000, 0, 10, 0, 8456);
    }
}
pub(crate) unsafe fn SpriteCB_WildMonAnimate(sprite: *mut Sprite) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
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
            crate::gl!(sFlickerArray[0] = 0);
        }
    }
}
pub unsafe fn SpriteCB_FaintOpponentMon(sprite: *mut Sprite) {
    let battler: u8 = (*sprite).data[sBattler] as u8;
    let mut species: u16 = 0;
    let mut yOffset: u8 = 0;
    if (*(*crate::gl!(gBattleSpritesDataPtr)).battlerData.at(battler)).transformSpecies != 0 {
        species = (*(*crate::gl!(gBattleSpritesDataPtr)).battlerData.at(battler)).transformSpecies;
    } else {
        species = (*sprite).data[sSpeciesId] as u16;
    }
    GetMonData2(
        &raw mut (*gEnemyParty.as_ptr())[crate::gl!(gBattlerPartyIndexes[battler])],
        MON_DATA_PERSONALITY,
    );
    if species == SPECIES_UNOWN {
        let personalityValue: u32 = GetMonData2(
            &raw mut (*gEnemyParty.as_ptr())[crate::gl!(gBattlerPartyIndexes[battler])],
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
            .cast::<CArray<MonCoords, 0>>())[crate::gl!(gBattleMonForms[battler])]
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
            let mut dst: *mut u8 = (*crate::gl!(gMonSpritesGfxPtr)).sprites.byte
                [GetBattlerPosition((*sprite).data[sBattler] as u8)]
            .at((({
                let gl_i11 = (*sprite).data[sBattler];
                crate::gl!(gBattleMonForms[gl_i11])
            }) as i32)
                << 11)
            .at(((*sprite).data[3] as i32) << 8);
            for i in 0..0x100 {
                *({
                    let t3 = dst;
                    dst = dst.at(1);
                    t3
                }) = 0;
            }
            StartSpriteAnim(sprite, {
                let gl_i12 = (*sprite).data[sBattler];
                crate::gl!(gBattleMonForms[gl_i12])
            });
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
        if (gHitMarker.get() & HITMARKER_NO_ANIMATIONS == 0
            || gBattleTypeFlags.get() & 0x2000002 != 0)
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
    if gIntroSlideFlags.get() as i32 & 1 == 0 {
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
    if gIntroSlideFlags.get() as i32 & 1 == 0 {
        (*sprite).x2 += (*sprite).data[sSpeedX];
        (*sprite).y2 += (*sprite).data[sSpeedY];
    }
}
pub unsafe fn DoBounceEffect(battler: u8, which: u8, delta: i8, amplitude: i8) {
    let mut bouncerSpriteId: u8 = 0;
    match which {
        BOUNCE_MON => {
            if (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(battler))
            .battlerIsBouncing()
                != 0
            {
                return;
            }
        }
        _ => {
            if (*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(battler))
            .healthboxIsBouncing()
                != 0
            {
                return;
            }
        }
    }
    let invisibleSpriteId: u8 = CreateInvisibleSpriteWithCallback(Some(SpriteCB_BounceEffect));
    if which == BOUNCE_HEALTHBOX {
        bouncerSpriteId = crate::gl!(gHealthboxSpriteIds[battler]);
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .healthboxBounceSpriteId = invisibleSpriteId;
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .set_healthboxIsBouncing(1);
        {
            let gl_i13 = sSinIndex;
            crate::gl!(gSprites[invisibleSpriteId].data[gl_i13] = 128)
        };
    } else {
        bouncerSpriteId = crate::gl!(gBattlerSpriteIds[battler]);
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .battlerBounceSpriteId = invisibleSpriteId;
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .set_battlerIsBouncing(1);
        {
            let gl_i14 = sSinIndex;
            crate::gl!(gSprites[invisibleSpriteId].data[gl_i14] = 192)
        };
    }
    {
        let gl_i15 = sDelta;
        crate::gl!(gSprites[invisibleSpriteId].data[gl_i15] = delta as i16)
    };
    {
        let gl_i16 = sAmplitude;
        crate::gl!(gSprites[invisibleSpriteId].data[gl_i16] = amplitude as i16)
    };
    {
        let gl_i17 = sBouncerSpriteId;
        crate::gl!(gSprites[invisibleSpriteId].data[gl_i17] = bouncerSpriteId as i16)
    };
    {
        let gl_i18 = sWhich;
        crate::gl!(gSprites[invisibleSpriteId].data[gl_i18] = which as i16)
    };
    crate::gl!(gSprites[bouncerSpriteId].x2 = 0);
    crate::gl!(gSprites[bouncerSpriteId].y2 = 0);
}
pub unsafe fn EndBounceEffect(battler: u8, which: u8) {
    let mut bouncerSpriteId: u8 = 0;
    if which == BOUNCE_HEALTHBOX {
        if (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .healthboxIsBouncing()
            == 0
        {
            return;
        }
        bouncerSpriteId = (*gSprites.as_ptr())[(*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .healthboxBounceSpriteId]
            .data[sBouncerSpriteId] as u8;
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[(*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(battler))
            .healthboxBounceSpriteId],
        );
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .set_healthboxIsBouncing(0);
    } else {
        if (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .battlerIsBouncing()
            == 0
        {
            return;
        }
        bouncerSpriteId = (*gSprites.as_ptr())[(*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .battlerBounceSpriteId]
            .data[sBouncerSpriteId] as u8;
        DestroySprite(
            &raw mut (*gSprites.as_ptr())[(*(*crate::gl!(gBattleSpritesDataPtr))
                .healthBoxesData
                .at(battler))
            .battlerBounceSpriteId],
        );
        (*(*crate::gl!(gBattleSpritesDataPtr))
            .healthBoxesData
            .at(battler))
        .set_battlerIsBouncing(0);
    }
    crate::gl!(gSprites[bouncerSpriteId].x2 = 0);
    crate::gl!(gSprites[bouncerSpriteId].y2 = 0);
}
pub(crate) unsafe fn SpriteCB_BounceEffect(sprite: *mut Sprite) {
    let bouncerSpriteId: u8 = (*sprite).data[sBouncerSpriteId] as u8;
    let mut index: i32 = 0;
    if (*sprite).data[sWhich] == BOUNCE_HEALTHBOX as i16 {
        index = (*sprite).data[sSinIndex] as i32;
    } else {
        index = (*sprite).data[sSinIndex] as i32;
    }
    crate::gl!(
        gSprites[bouncerSpriteId].y2 =
            Sin(index as i16, (*sprite).data[sAmplitude]) + (*sprite).data[sAmplitude]
    );
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
pub fn BeginBattleIntroDummy() {}
pub unsafe fn BeginBattleIntro() {
    BattleStartClearSetData();
    crate::gl!(gBattleCommunication[1] = 0);
    crate::gl!(gBattleMainFunc = Some(BattleIntroGetMonsData));
}
pub(crate) unsafe fn BattleMainCB1() {
    (*gBattleMainFunc.as_ptr()).unwrap_unchecked()();
    gActiveBattler.set(0);
    while gActiveBattler.get() < gBattlersCount.get() {
        (*gBattlerControllerFuncs.as_ptr())[gActiveBattler.get()].unwrap_unchecked()();
        gActiveBattler.set(gActiveBattler.get() + (1));
    }
}
unsafe fn BattleStartClearSetData() {
    let mut dataPtr: *mut u8 = null_mut();
    TurnValuesCleanUp(FALSE);
    SpecialStatusesClear();
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        crate::gl!(gStatuses3[i] = 0);
        dataPtr = crate::gl!(&raw gDisableStructs[i]) as *mut u8;
        for j in 0..28u32 {
            *dataPtr.at(j) = 0;
        }
        crate::gl!(gDisableStructs[i].isFirstTurn = 2);
        crate::gl!(sUnusedBattlersArray[i] = 0);
        crate::gl!(gLastMoves[i] = MOVE_NONE);
        crate::gl!(gLastLandedMoves[i] = MOVE_NONE);
        crate::gl!(gLastHitByType[i] = 0);
        crate::gl!(gLastResultingMoves[i] = MOVE_NONE);
        crate::gl!(gLastHitBy[i] = 0xFF);
        crate::gl!(gLockedMoves[i] = MOVE_NONE);
        crate::gl!(gLastPrintedMoves[i] = MOVE_NONE);
        (*(*crate::gl!(gBattleResources)).flags).flags[i] = 0;
        crate::gl!(gPalaceSelectionBattleScripts[i] = null_mut());
    }
    let mut i: i32 = 0;
    while i < 2 {
        crate::gl!(gSideStatuses[i] = 0);
        dataPtr = crate::gl!(&raw gSideTimers[i]) as *mut u8;
        for j in 0..12u32 {
            *dataPtr.at(j) = 0;
        }
        i += 1;
    }
    gBattlerAttacker.set(0);
    gBattlerTarget.set(0);
    gBattleWeather.set(0);
    dataPtr = crate::gl!(&raw gWishFutureKnock) as *mut u8;
    for i in 0..44i32 {
        *dataPtr.at(i) = 0;
    }
    gHitMarker.set(0);
    if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED == 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_LINK == 0
            && (*crate::gl!(gSaveBlock2Ptr)).optionsBattleSceneOff() == TRUE as u16
        {
            gHitMarker.set(gHitMarker.get() | (HITMARKER_NO_ANIMATIONS));
        }
    } else if gBattleTypeFlags.get() & 0x2000002 == 0 && GetBattleSceneInRecordedBattle() != 0 {
        gHitMarker.set(gHitMarker.get() | (HITMARKER_NO_ANIMATIONS));
    }
    crate::gl!(
        gBattleScripting.battleStyle = (*crate::gl!(gSaveBlock2Ptr)).optionsBattleStyle() as u8
    );
    gMultiHitCounter.set(0);
    gBattleOutcome.set(0);
    gBattleControllerExecFlags.set(0);
    gPaydayMoney.set(0);
    (*(*crate::gl!(gBattleResources)).battleScriptsStack).size = 0;
    (*(*crate::gl!(gBattleResources)).battleCallbackStack).size = 0;
    for i in 0..BATTLE_COMMUNICATION_ENTRIES_COUNT {
        crate::gl!(gBattleCommunication[i] = 0);
    }
    gPauseCounterBattle.set(0);
    gBattleMoveDamage.set(0);
    gIntroSlideFlags.set(0);
    crate::gl!(gBattleScripting.animTurn = 0);
    crate::gl!(gBattleScripting.animTargetsHit = 0);
    gLeveledUpInBattle.set(0);
    gAbsentBattlerFlags.set(0);
    (*crate::gl!(gBattleStruct)).runTries = 0;
    (*crate::gl!(gBattleStruct)).safariGoNearCounter = 0;
    (*crate::gl!(gBattleStruct)).safariPkblThrowCounter = 0;
    (*crate::gl!(gBattleStruct)).safariCatchFactor =
        ((*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
            [GetMonData2(crate::gl!(&raw gEnemyParty[0]), MON_DATA_SPECIES)]
        .catchRate as i32
            * 100
            / 1275) as u8;
    (*crate::gl!(gBattleStruct)).safariEscapeFactor = 3;
    (*crate::gl!(gBattleStruct)).wildVictorySong = 0;
    (*crate::gl!(gBattleStruct)).moneyMultiplier = 1;
    for i in 0..8i32 {
        *(*crate::gl!(gBattleStruct))
            .lastTakenMove
            .as_mut_ptr()
            .at(i) = MOVE_NONE as u8;
        *((*crate::gl!(gBattleStruct)).usedHeldItems.as_mut_ptr() as *mut u8).at(i) =
            ITEM_NONE as u8;
        *((*crate::gl!(gBattleStruct)).choicedMove.as_mut_ptr() as *mut u8).at(i) = MOVE_NONE as u8;
        *((*crate::gl!(gBattleStruct)).changedItems.as_mut_ptr() as *mut u8).at(i) =
            ITEM_NONE as u8;
        *(*crate::gl!(gBattleStruct))
            .lastTakenMoveFrom
            .as_mut_ptr()
            .at(i) = 0;
        *(*crate::gl!(gBattleStruct))
            .lastTakenMoveFrom
            .as_mut_ptr()
            .at(i + 8) = 0;
        *(*crate::gl!(gBattleStruct))
            .lastTakenMoveFrom
            .as_mut_ptr()
            .at(i + 16) = 0;
        *(*crate::gl!(gBattleStruct))
            .lastTakenMoveFrom
            .as_mut_ptr()
            .at(i + 24) = 0;
    }
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        *(*crate::gl!(gBattleStruct))
            .AI_monToSwitchIntoId
            .as_mut_ptr()
            .at(i) = PARTY_SIZE as u8;
    }
    (*crate::gl!(gBattleStruct)).givenExpMons = 0;
    (*crate::gl!(gBattleStruct)).palaceFlags = 0;
    gRandomTurnNumber.set(Random());
    dataPtr = crate::gl!(&raw gBattleResults) as *mut u8;
    for i in 0..68i32 {
        *dataPtr.at(i) = 0;
    }
    (*gBattleResults.as_ptr()).set_shinyWildMon(IsMonShiny(crate::gl!(&raw gEnemyParty[0])));
    (*crate::gl!(gBattleStruct)).arenaLostPlayerMons = 0;
    (*crate::gl!(gBattleStruct)).arenaLostOpponentMons = 0;
}
pub unsafe fn SwitchInClearSetData() {
    let disableStructCopy: DisableStruct = {
        let gl_i19 = gActiveBattler.get();
        crate::gl!(gDisableStructs[gl_i19])
    };
    let mut i: i32 = 0;
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [gCurrentMove.get()]
    .effect
        != EFFECT_BATON_PASS
    {
        for i in 0..NUM_BATTLE_STATS {
            {
                let gl_i20 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i20].statStages[i] = DEFAULT_STAT_STAGE)
            };
        }
        for i in 0..(gBattlersCount.get() as i32) {
            if crate::gl!(gBattleMons[i].status2) & STATUS2_ESCAPE_PREVENTION != 0
                && crate::gl!(gDisableStructs[i].battlerPreventingEscape) == gActiveBattler.get()
            {
                crate::gl!(gBattleMons[i].status2 &= 0xfbffffff);
            }
            if crate::gl!(gStatuses3[i]) & STATUS3_ALWAYS_HITS != 0
                && crate::gl!(gDisableStructs[i].battlerWithSureHit) == gActiveBattler.get()
            {
                crate::gl!(gStatuses3[i] &= 0xffffffe7);
                crate::gl!(gDisableStructs[i].battlerWithSureHit = 0);
            }
        }
    }
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [gCurrentMove.get()]
    .effect
        == EFFECT_BATON_PASS
    {
        {
            let gl_i21 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i21].status2 &= 0x15100007)
        };
        {
            let gl_v = 0x3043f;
            let gl_i5 = gActiveBattler.get();
            crate::gl!(gStatuses3[gl_i5] &= gl_v)
        };
        i = 0;
        while i < gBattlersCount.get() as i32 {
            if GetBattlerSide(gActiveBattler.get()) != GetBattlerSide(i as u8)
                && crate::gl!(gStatuses3[i]) & STATUS3_ALWAYS_HITS != 0
                && crate::gl!(gDisableStructs[i].battlerWithSureHit) == gActiveBattler.get()
            {
                crate::gl!(gStatuses3[i] &= 0xffffffe7);
                crate::gl!(gStatuses3[i] |= 16);
            }
            i += 1;
        }
    } else {
        {
            let gl_i22 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i22].status2 = 0)
        };
        {
            let gl_v = 0;
            let gl_i6 = gActiveBattler.get();
            crate::gl!(gStatuses3[gl_i6] = gl_v)
        };
    }
    i = 0;
    while i < gBattlersCount.get() as i32 {
        if crate::gl!(gBattleMons[i].status2)
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
                << 16
            != 0
        {
            crate::gl!(gBattleMons[i].status2 &= !(gBitTable[gActiveBattler.get()] << 16));
        }
        if crate::gl!(gBattleMons[i].status2) & STATUS2_WRAPPED != 0
            && *(*crate::gl!(gBattleStruct)).wrappedBy.as_mut_ptr().at(i) == gActiveBattler.get()
        {
            crate::gl!(gBattleMons[i].status2 &= 0xffff1fff);
        }
        i += 1;
    }
    {
        let gl_i23 = gActiveBattler.get();
        crate::gl!(gActionSelectionCursor[gl_i23] = 0)
    };
    {
        let gl_i24 = gActiveBattler.get();
        crate::gl!(gMoveSelectionCursor[gl_i24] = 0)
    };
    let ptr: *mut u8 = ({
        let gl_i25 = gActiveBattler.get();
        crate::gl!(&raw gDisableStructs[gl_i25])
    }) as *mut u8;
    for i in 0..28i32 {
        *ptr.at(i) = 0;
    }
    if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
        [gCurrentMove.get()]
    .effect
        == EFFECT_BATON_PASS
    {
        {
            let gl_i26 = gActiveBattler.get();
            crate::gl!(gDisableStructs[gl_i26].substituteHP = disableStructCopy.substituteHP)
        };
        {
            let gl_i27 = gActiveBattler.get();
            crate::gl!(
                gDisableStructs[gl_i27].battlerWithSureHit = disableStructCopy.battlerWithSureHit
            )
        };
        (*gDisableStructs.as_ptr())[gActiveBattler.get()]
            .set_perishSongTimer(disableStructCopy.perishSongTimer());
        (*gDisableStructs.as_ptr())[gActiveBattler.get()]
            .set_perishSongTimerStartValue(disableStructCopy.perishSongTimerStartValue());
        {
            let gl_i28 = gActiveBattler.get();
            crate::gl!(
                gDisableStructs[gl_i28].battlerPreventingEscape =
                    disableStructCopy.battlerPreventingEscape
            )
        };
    }
    gMoveResultFlags.set(0);
    {
        let gl_i29 = gActiveBattler.get();
        crate::gl!(gDisableStructs[gl_i29].isFirstTurn = 2)
    };
    (*gDisableStructs.as_ptr())[gActiveBattler.get()]
        .set_truantSwitchInHack(disableStructCopy.truantSwitchInHack());
    {
        let gl_v = MOVE_NONE;
        let gl_i7 = gActiveBattler.get();
        crate::gl!(gLastMoves[gl_i7] = gl_v)
    };
    {
        let gl_v = MOVE_NONE;
        let gl_i8 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i8] = gl_v)
    };
    {
        let gl_v = 0;
        let gl_i9 = gActiveBattler.get();
        crate::gl!(gLastHitByType[gl_i9] = gl_v)
    };
    {
        let gl_v = MOVE_NONE;
        let gl_i10 = gActiveBattler.get();
        crate::gl!(gLastResultingMoves[gl_i10] = gl_v)
    };
    {
        let gl_v = MOVE_NONE;
        let gl_i11 = gActiveBattler.get();
        crate::gl!(gLastPrintedMoves[gl_i11] = gl_v)
    };
    {
        let gl_v = 0xFF;
        let gl_i12 = gActiveBattler.get();
        crate::gl!(gLastHitBy[gl_i12] = gl_v)
    };
    *(*crate::gl!(gBattleStruct))
        .lastTakenMove
        .as_mut_ptr()
        .at(gActiveBattler.get() as i32 * 2) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMove
        .as_mut_ptr()
        .at(gActiveBattler.get() as i32 * 2)
        .at(1) = MOVE_NONE as u8;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(gActiveBattler.get() as i32 * 8) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(gActiveBattler.get() as i32 * 8)
        .at(1) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(2 + gActiveBattler.get() as i32 * 8) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(2 + gActiveBattler.get() as i32 * 8)
        .at(1) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(4 + gActiveBattler.get() as i32 * 8) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(4 + gActiveBattler.get() as i32 * 8)
        .at(1) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(6 + gActiveBattler.get() as i32 * 8) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(6 + gActiveBattler.get() as i32 * 8)
        .at(1) = 0;
    (*crate::gl!(gBattleStruct)).palaceFlags &= !(gBitTable[gActiveBattler.get()] as u8);
    i = 0;
    while i < gBattlersCount.get() as i32 {
        if i != gActiveBattler.get() as i32
            && GetBattlerSide(i as u8) != GetBattlerSide(gActiveBattler.get())
        {
            *(*crate::gl!(gBattleStruct))
                .lastTakenMove
                .as_mut_ptr()
                .at(i * 2) = 0;
            *(*crate::gl!(gBattleStruct))
                .lastTakenMove
                .as_mut_ptr()
                .at(i * 2)
                .at(1) = MOVE_NONE as u8;
        }
        *(*crate::gl!(gBattleStruct))
            .lastTakenMoveFrom
            .as_mut_ptr()
            .at(i * 8 + gActiveBattler.get() as i32 * 2) = 0;
        *(*crate::gl!(gBattleStruct))
            .lastTakenMoveFrom
            .as_mut_ptr()
            .at(i * 8 + gActiveBattler.get() as i32 * 2)
            .at(1) = 0;
        i += 1;
    }
    *(&raw mut (*crate::gl!(gBattleStruct)).choicedMove[gActiveBattler.get()] as *mut u8) = 0;
    *(&raw mut (*crate::gl!(gBattleStruct)).choicedMove[gActiveBattler.get()] as *mut u8).at(1) =
        MOVE_NONE as u8;
    (*(*crate::gl!(gBattleResources)).flags).flags[gActiveBattler.get()] = 0;
    gCurrentMove.set(MOVE_NONE);
    (*crate::gl!(gBattleStruct)).arenaTurnCounter = 0xFF;
    ClearBattlerMoveHistory(gActiveBattler.get());
    ClearBattlerAbilityHistory(gActiveBattler.get());
}
pub unsafe fn FaintClearSetData() {
    for i in 0..NUM_BATTLE_STATS {
        {
            let gl_i30 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i30].statStages[i] = DEFAULT_STAT_STAGE)
        };
    }
    {
        let gl_i31 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i31].status2 = 0)
    };
    {
        let gl_v = 0;
        let gl_i13 = gActiveBattler.get();
        crate::gl!(gStatuses3[gl_i13] = gl_v)
    };
    let mut i: i32 = 0;
    while i < gBattlersCount.get() as i32 {
        if crate::gl!(gBattleMons[i].status2) & STATUS2_ESCAPE_PREVENTION != 0
            && crate::gl!(gDisableStructs[i].battlerPreventingEscape) == gActiveBattler.get()
        {
            crate::gl!(gBattleMons[i].status2 &= 0xfbffffff);
        }
        if crate::gl!(gBattleMons[i].status2)
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[gActiveBattler.get()]
                << 16
            != 0
        {
            crate::gl!(gBattleMons[i].status2 &= !(gBitTable[gActiveBattler.get()] << 16));
        }
        if crate::gl!(gBattleMons[i].status2) & STATUS2_WRAPPED != 0
            && *(*crate::gl!(gBattleStruct)).wrappedBy.as_mut_ptr().at(i) == gActiveBattler.get()
        {
            crate::gl!(gBattleMons[i].status2 &= 0xffff1fff);
        }
        i += 1;
    }
    {
        let gl_i32 = gActiveBattler.get();
        crate::gl!(gActionSelectionCursor[gl_i32] = 0)
    };
    {
        let gl_i33 = gActiveBattler.get();
        crate::gl!(gMoveSelectionCursor[gl_i33] = 0)
    };
    let ptr: *mut u8 = ({
        let gl_i34 = gActiveBattler.get();
        crate::gl!(&raw gDisableStructs[gl_i34])
    }) as *mut u8;
    for i in 0..28i32 {
        *ptr.at(i) = 0;
    }
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_protected(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_endured(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_noValidMoves(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_helpingHand(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_bounceMove(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_stealMove(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_flag0Unknown(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_prlzImmobility(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_confusionSelfDmg(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_targetNotAffected(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_chargingTurn(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_fleeType(0);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_usedImprisonedMove(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_loveImmobility(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_usedDisabledMove(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_usedTauntedMove(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_flag2Unknown(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_flinchImmobility(FALSE as u32);
    (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_notFirstStrike(FALSE as u32);
    {
        let gl_i35 = gActiveBattler.get();
        crate::gl!(gDisableStructs[gl_i35].isFirstTurn = 2)
    };
    {
        let gl_v = MOVE_NONE;
        let gl_i14 = gActiveBattler.get();
        crate::gl!(gLastMoves[gl_i14] = gl_v)
    };
    {
        let gl_v = MOVE_NONE;
        let gl_i15 = gActiveBattler.get();
        crate::gl!(gLastLandedMoves[gl_i15] = gl_v)
    };
    {
        let gl_v = 0;
        let gl_i16 = gActiveBattler.get();
        crate::gl!(gLastHitByType[gl_i16] = gl_v)
    };
    {
        let gl_v = MOVE_NONE;
        let gl_i17 = gActiveBattler.get();
        crate::gl!(gLastResultingMoves[gl_i17] = gl_v)
    };
    {
        let gl_v = MOVE_NONE;
        let gl_i18 = gActiveBattler.get();
        crate::gl!(gLastPrintedMoves[gl_i18] = gl_v)
    };
    {
        let gl_v = 0xFF;
        let gl_i19 = gActiveBattler.get();
        crate::gl!(gLastHitBy[gl_i19] = gl_v)
    };
    *(&raw mut (*crate::gl!(gBattleStruct)).choicedMove[gActiveBattler.get()] as *mut u8) = 0;
    *(&raw mut (*crate::gl!(gBattleStruct)).choicedMove[gActiveBattler.get()] as *mut u8).at(1) =
        MOVE_NONE as u8;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMove
        .as_mut_ptr()
        .at(gActiveBattler.get() as i32 * 2) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMove
        .as_mut_ptr()
        .at(gActiveBattler.get() as i32 * 2)
        .at(1) = MOVE_NONE as u8;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(gActiveBattler.get() as i32 * 8) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(gActiveBattler.get() as i32 * 8)
        .at(1) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(2 + gActiveBattler.get() as i32 * 8) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(2 + gActiveBattler.get() as i32 * 8)
        .at(1) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(4 + gActiveBattler.get() as i32 * 8) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(4 + gActiveBattler.get() as i32 * 8)
        .at(1) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(6 + gActiveBattler.get() as i32 * 8) = 0;
    *(*crate::gl!(gBattleStruct))
        .lastTakenMoveFrom
        .as_mut_ptr()
        .at(6 + gActiveBattler.get() as i32 * 8)
        .at(1) = 0;
    (*crate::gl!(gBattleStruct)).palaceFlags &= !(gBitTable[gActiveBattler.get()] as u8);
    i = 0;
    while i < gBattlersCount.get() as i32 {
        if i != gActiveBattler.get() as i32
            && GetBattlerSide(i as u8) != GetBattlerSide(gActiveBattler.get())
        {
            *(*crate::gl!(gBattleStruct))
                .lastTakenMove
                .as_mut_ptr()
                .at(i * 2) = 0;
            *(*crate::gl!(gBattleStruct))
                .lastTakenMove
                .as_mut_ptr()
                .at(i * 2)
                .at(1) = MOVE_NONE as u8;
        }
        *(*crate::gl!(gBattleStruct))
            .lastTakenMoveFrom
            .as_mut_ptr()
            .at(i * 8 + gActiveBattler.get() as i32 * 2) = 0;
        *(*crate::gl!(gBattleStruct))
            .lastTakenMoveFrom
            .as_mut_ptr()
            .at(i * 8 + gActiveBattler.get() as i32 * 2)
            .at(1) = 0;
        i += 1;
    }
    (*(*crate::gl!(gBattleResources)).flags).flags[gActiveBattler.get()] = 0;
    {
        let gl_v = (*(&raw const crate::data::pokemon::gSpeciesInfo)
            .cast::<CArray<SpeciesInfo, 0>>())[{
            let gl_i37 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i37].species)
        }]
        .types[0];
        let gl_i36 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i36].types[0] = gl_v)
    };
    {
        let gl_v = (*(&raw const crate::data::pokemon::gSpeciesInfo)
            .cast::<CArray<SpeciesInfo, 0>>())[{
            let gl_i39 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i39].species)
        }]
        .types[1];
        let gl_i38 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i38].types[1] = gl_v)
    };
    ClearBattlerMoveHistory(gActiveBattler.get());
    ClearBattlerAbilityHistory(gActiveBattler.get());
}
pub(crate) unsafe fn BattleIntroGetMonsData() {
    match crate::gl!(gBattleCommunication[0]) {
        0 => {
            gActiveBattler.set(crate::gl!(gBattleCommunication[1]));
            BtlController_EmitGetMonData(B_COMM_TO_CONTROLLER, REQUEST_ALL_BATTLE, 0);
            MarkBattlerForControllerExec(gActiveBattler.get());
            crate::gl!(gBattleCommunication[0] += 1);
        }
        1 if gBattleControllerExecFlags.get() == 0 => {
            crate::gl!(gBattleCommunication[1] += 1);
            if crate::gl!(gBattleCommunication[1]) == gBattlersCount.get() {
                crate::gl!(gBattleMainFunc = Some(BattleIntroPrepareBackgroundSlide));
            } else {
                crate::gl!(gBattleCommunication[0] = 0);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn BattleIntroPrepareBackgroundSlide() {
    if gBattleControllerExecFlags.get() == 0 {
        gActiveBattler.set(GetBattlerAtPosition(0));
        BtlController_EmitIntroSlide(B_COMM_TO_CONTROLLER, gBattleEnvironment.get());
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattleMainFunc = Some(BattleIntroDrawTrainersOrMonsSprites));
        crate::gl!(gBattleCommunication[0] = 0);
        crate::gl!(gBattleCommunication[1] = 0);
    }
}
pub(crate) unsafe fn BattleIntroDrawTrainersOrMonsSprites() {
    let mut ptr: *mut u8 = null_mut();
    let mut i: i32 = 0;
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    gActiveBattler.set(0);
    while gActiveBattler.get() < gBattlersCount.get() {
        if gBattleTypeFlags.get() & BATTLE_TYPE_SAFARI != 0
            && GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER
        {
            ptr = ({
                let gl_i40 = gActiveBattler.get();
                crate::gl!(&raw gBattleMons[gl_i40])
            }) as *mut u8;
            for i in 0..88i32 {
                *ptr.at(i) = 0;
            }
        } else {
            let mut hpOnSwitchout: *mut u16 = null_mut();
            ptr = ({
                let gl_i41 = gActiveBattler.get();
                crate::gl!(&raw gBattleMons[gl_i41])
            }) as *mut u8;
            i = 0;
            while i < 88 {
                *ptr.at(i) = {
                    let gl_i20 = gActiveBattler.get();
                    let gl_i21 = 4 + i;
                    crate::gl!(gBattleBufferB[gl_i20][gl_i21])
                };
                i += 1;
            }
            {
                let gl_v = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[{
                    let gl_i43 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i43].species)
                }]
                .types[0];
                let gl_i42 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i42].types[0] = gl_v)
            };
            {
                let gl_v = (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[{
                    let gl_i45 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i45].species)
                }]
                .types[1];
                let gl_i44 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i44].types[1] = gl_v)
            };
            {
                let gl_v = GetAbilityBySpecies(
                    {
                        let gl_i47 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i47].species)
                    },
                    (*gBattleMons.as_ptr())[gActiveBattler.get()].abilityNum() as u8,
                );
                let gl_i46 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i46].ability = gl_v)
            };
            hpOnSwitchout = &raw mut (*crate::gl!(gBattleStruct)).hpOnSwitchout
                [GetBattlerSide(gActiveBattler.get())];
            *hpOnSwitchout = {
                let gl_i48 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i48].hp)
            };
            for i in 0..NUM_BATTLE_STATS {
                {
                    let gl_i49 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i49].statStages[i] = DEFAULT_STAT_STAGE)
                };
            }
            {
                let gl_i50 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i50].status2 = 0)
            };
        }
        if GetBattlerPosition(gActiveBattler.get()) == B_POSITION_PLAYER_LEFT {
            BtlController_EmitDrawTrainerPic(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0 {
            if GetBattlerPosition(gActiveBattler.get()) == B_POSITION_OPPONENT_LEFT {
                BtlController_EmitDrawTrainerPic(B_COMM_TO_CONTROLLER);
                MarkBattlerForControllerExec(gActiveBattler.get());
            }
            if GetBattlerSide(gActiveBattler.get()) == B_SIDE_OPPONENT
                && gBattleTypeFlags.get() & 0x63f0902 == 0
            {
                HandleSetPokedexFlag(
                    SpeciesToNationalPokedexNum({
                        let gl_i51 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i51].species)
                    }),
                    FLAG_SET_SEEN,
                    {
                        let gl_i52 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i52].personality)
                    },
                );
            }
        } else {
            if GetBattlerSide(gActiveBattler.get()) == B_SIDE_OPPONENT {
                if gBattleTypeFlags.get() & 0x63f0902 == 0 {
                    HandleSetPokedexFlag(
                        SpeciesToNationalPokedexNum({
                            let gl_i53 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i53].species)
                        }),
                        FLAG_SET_SEEN,
                        {
                            let gl_i54 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i54].personality)
                        },
                    );
                }
                BtlController_EmitLoadMonSprite(B_COMM_TO_CONTROLLER);
                MarkBattlerForControllerExec(gActiveBattler.get());
                crate::gl!(
                    gBattleResults.lastOpponentSpecies = GetMonData3(
                        &raw mut (*gEnemyParty.as_ptr())[{
                            let gl_i55 = gActiveBattler.get();
                            crate::gl!(gBattlerPartyIndexes[gl_i55])
                        }],
                        MON_DATA_SPECIES,
                        null_mut(),
                    ) as u16
                );
            }
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
            && (GetBattlerPosition(gActiveBattler.get()) == B_POSITION_PLAYER_RIGHT
                || GetBattlerPosition(gActiveBattler.get()) == B_POSITION_OPPONENT_RIGHT)
        {
            BtlController_EmitDrawTrainerPic(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0
            && GetBattlerPosition(gActiveBattler.get()) == B_POSITION_OPPONENT_RIGHT
        {
            BtlController_EmitDrawTrainerPic(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_ARENA != 0 {
            BattleArena_InitPoints();
        }
        gActiveBattler.set(gActiveBattler.get() + (1));
    }
    crate::gl!(gBattleMainFunc = Some(BattleIntroDrawPartySummaryScreens));
}
pub(crate) unsafe fn BattleIntroDrawPartySummaryScreens() {
    let mut i: i32 = 0;
    let mut hpStatus: CArray<HpAndStatus, 6> = zeroed();
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0 {
        i = 0;
        while i < PARTY_SIZE {
            if GetMonData2(crate::gl!(&raw gEnemyParty[i]), MON_DATA_SPECIES_OR_EGG)
                == SPECIES_NONE as u32
                || GetMonData2(crate::gl!(&raw gEnemyParty[i]), MON_DATA_SPECIES_OR_EGG)
                    == SPECIES_EGG
            {
                hpStatus[i].hp = HP_EMPTY_SLOT;
                hpStatus[i].status = 0;
            } else {
                hpStatus[i].hp = GetMonData2(crate::gl!(&raw gEnemyParty[i]), MON_DATA_HP) as u16;
                hpStatus[i].status = GetMonData2(crate::gl!(&raw gEnemyParty[i]), MON_DATA_STATUS);
            }
            i += 1;
        }
        gActiveBattler.set(GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT));
        BtlController_EmitDrawPartyStatusSummary(
            B_COMM_TO_CONTROLLER,
            hpStatus.as_mut_ptr(),
            PARTY_SUMM_SKIP_DRAW_DELAY,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
        for i in 0..PARTY_SIZE {
            if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES_OR_EGG)
                == SPECIES_NONE as u32
                || GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES_OR_EGG)
                    == SPECIES_EGG
            {
                hpStatus[i].hp = HP_EMPTY_SLOT;
                hpStatus[i].status = 0;
            } else {
                hpStatus[i].hp = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HP) as u16;
                hpStatus[i].status = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_STATUS);
            }
        }
        gActiveBattler.set(GetBattlerAtPosition(B_POSITION_PLAYER_LEFT));
        BtlController_EmitDrawPartyStatusSummary(
            B_COMM_TO_CONTROLLER,
            hpStatus.as_mut_ptr(),
            PARTY_SUMM_SKIP_DRAW_DELAY,
        );
        MarkBattlerForControllerExec(gActiveBattler.get());
        crate::gl!(gBattleMainFunc = Some(BattleIntroPrintTrainerWantsToBattle));
    } else {
        for i in 0..PARTY_SIZE {
            if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES_OR_EGG)
                == SPECIES_NONE as u32
                || GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES_OR_EGG)
                    == SPECIES_EGG
            {
                hpStatus[i].hp = HP_EMPTY_SLOT;
                hpStatus[i].status = 0;
            } else {
                hpStatus[i].hp = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HP) as u16;
                hpStatus[i].status = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_STATUS);
            }
        }
        crate::gl!(gBattleMainFunc = Some(BattleIntroPrintWildMonAttacked));
    }
}
pub(crate) unsafe fn BattleIntroPrintTrainerWantsToBattle() {
    if gBattleControllerExecFlags.get() == 0 {
        gActiveBattler.set(GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT));
        PrepareStringBattle(STRINGID_INTROMSG, gActiveBattler.get());
        crate::gl!(gBattleMainFunc = Some(BattleIntroPrintOpponentSendsOut));
    }
}
pub(crate) unsafe fn BattleIntroPrintWildMonAttacked() {
    if gBattleControllerExecFlags.get() == 0 {
        crate::gl!(gBattleMainFunc = Some(BattleIntroPrintPlayerSendsOut));
        PrepareStringBattle(0, 0);
    }
}
pub(crate) unsafe fn BattleIntroPrintOpponentSendsOut() {
    let mut position: u32 = 0;
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED == 0 {
        position = B_POSITION_OPPONENT_LEFT as u32;
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0 {
        if gBattleTypeFlags.get() & 0x80000000 != 0 {
            position = B_POSITION_OPPONENT_LEFT as u32;
        } else {
            position = B_POSITION_PLAYER_LEFT as u32;
        }
    } else {
        position = B_POSITION_OPPONENT_LEFT as u32;
    }
    PrepareStringBattle(STRINGID_INTROSENDOUT, GetBattlerAtPosition(position as u8));
    crate::gl!(gBattleMainFunc = Some(BattleIntroOpponent1SendsOutMonAnimation));
}
pub(crate) unsafe fn BattleIntroOpponent2SendsOutMonAnimation() {
    let mut position: u32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED == 0 {
        position = B_POSITION_OPPONENT_RIGHT as u32;
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0 {
        if gBattleTypeFlags.get() & 0x80000000 != 0 {
            position = B_POSITION_OPPONENT_RIGHT as u32;
        } else {
            position = B_POSITION_PLAYER_RIGHT as u32;
        }
    } else {
        position = B_POSITION_OPPONENT_RIGHT as u32;
    }
    gActiveBattler.set(0);
    while gActiveBattler.get() < gBattlersCount.get() {
        if GetBattlerPosition(gActiveBattler.get()) as u32 == position {
            BtlController_EmitIntroTrainerBallThrow(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
        gActiveBattler.set(gActiveBattler.get() + (1));
    }
    crate::gl!(gBattleMainFunc = Some(BattleIntroRecordMonsToDex));
}
pub(crate) unsafe fn BattleIntroOpponent1SendsOutMonAnimation() {
    let mut position: u32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED != 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0 {
            if gBattleTypeFlags.get() & 0x80000000 != 0 {
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
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    gActiveBattler.set(0);
    while gActiveBattler.get() < gBattlersCount.get() {
        if GetBattlerPosition(gActiveBattler.get()) as u32 == position {
            BtlController_EmitIntroTrainerBallThrow(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler.get());
            if gBattleTypeFlags.get() & 32832 != 0 {
                crate::gl!(gBattleMainFunc = Some(BattleIntroOpponent2SendsOutMonAnimation));
                return;
            }
        }
        gActiveBattler.set(gActiveBattler.get() + (1));
    }
    crate::gl!(gBattleMainFunc = Some(BattleIntroRecordMonsToDex));
}
pub(crate) unsafe fn BattleIntroRecordMonsToDex() {
    if gBattleControllerExecFlags.get() == 0 {
        gActiveBattler.set(0);
        while gActiveBattler.get() < gBattlersCount.get() {
            if GetBattlerSide(gActiveBattler.get()) == B_SIDE_OPPONENT
                && gBattleTypeFlags.get() & 0x63f0902 == 0
            {
                HandleSetPokedexFlag(
                    SpeciesToNationalPokedexNum({
                        let gl_i56 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i56].species)
                    }),
                    FLAG_SET_SEEN,
                    {
                        let gl_i57 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i57].personality)
                    },
                );
            }
            gActiveBattler.set(gActiveBattler.get() + (1));
        }
        crate::gl!(gBattleMainFunc = Some(BattleIntroPrintPlayerSendsOut));
    }
}
fn BattleIntroSkipRecordMonsToDex() {
    if gBattleControllerExecFlags.get() == 0 {
        crate::gl!(gBattleMainFunc = Some(BattleIntroPrintPlayerSendsOut));
    }
}
pub(crate) unsafe fn BattleIntroPrintPlayerSendsOut() {
    if gBattleControllerExecFlags.get() == 0 {
        let mut position: u8 = 0;
        if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED == 0 {
            position = B_POSITION_PLAYER_LEFT;
        } else if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0 {
            if gBattleTypeFlags.get() & 0x80000000 != 0 {
                position = B_POSITION_PLAYER_LEFT;
            } else {
                position = B_POSITION_OPPONENT_LEFT;
            }
        } else {
            position = B_POSITION_PLAYER_LEFT;
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_SAFARI == 0 {
            PrepareStringBattle(STRINGID_INTROSENDOUT, GetBattlerAtPosition(position));
        }
        crate::gl!(gBattleMainFunc = Some(BattleIntroPlayer1SendsOutMonAnimation));
    }
}
pub(crate) unsafe fn BattleIntroPlayer2SendsOutMonAnimation() {
    let mut position: u32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED == 0 {
        position = B_POSITION_PLAYER_RIGHT as u32;
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0 {
        if gBattleTypeFlags.get() & 0x80000000 != 0 {
            position = B_POSITION_PLAYER_RIGHT as u32;
        } else {
            position = B_POSITION_OPPONENT_RIGHT as u32;
        }
    } else {
        position = B_POSITION_PLAYER_RIGHT as u32;
    }
    gActiveBattler.set(0);
    while gActiveBattler.get() < gBattlersCount.get() {
        if GetBattlerPosition(gActiveBattler.get()) as u32 == position {
            BtlController_EmitIntroTrainerBallThrow(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler.get());
        }
        gActiveBattler.set(gActiveBattler.get() + (1));
    }
    (*crate::gl!(gBattleStruct)).switchInAbilitiesCounter = 0;
    (*crate::gl!(gBattleStruct)).switchInItemsCounter = 0;
    (*crate::gl!(gBattleStruct)).overworldWeatherDone = FALSE;
    crate::gl!(gBattleMainFunc = Some(TryDoEventsBeforeFirstTurn));
}
pub(crate) unsafe fn BattleIntroPlayer1SendsOutMonAnimation() {
    let mut position: u32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED == 0 {
        position = B_POSITION_PLAYER_LEFT as u32;
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0 {
        if gBattleTypeFlags.get() & 0x80000000 != 0 {
            position = B_POSITION_PLAYER_LEFT as u32;
        } else {
            position = B_POSITION_OPPONENT_LEFT as u32;
        }
    } else {
        position = B_POSITION_PLAYER_LEFT as u32;
    }
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    gActiveBattler.set(0);
    while gActiveBattler.get() < gBattlersCount.get() {
        if GetBattlerPosition(gActiveBattler.get()) as u32 == position {
            BtlController_EmitIntroTrainerBallThrow(B_COMM_TO_CONTROLLER);
            MarkBattlerForControllerExec(gActiveBattler.get());
            if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
                crate::gl!(gBattleMainFunc = Some(BattleIntroPlayer2SendsOutMonAnimation));
                return;
            }
        }
        gActiveBattler.set(gActiveBattler.get() + (1));
    }
    (*crate::gl!(gBattleStruct)).switchInAbilitiesCounter = 0;
    (*crate::gl!(gBattleStruct)).switchInItemsCounter = 0;
    (*crate::gl!(gBattleStruct)).overworldWeatherDone = FALSE;
    crate::gl!(gBattleMainFunc = Some(TryDoEventsBeforeFirstTurn));
}
unsafe fn BattleIntroSwitchInPlayerMons() {
    if gBattleControllerExecFlags.get() == 0 {
        gActiveBattler.set(0);
        while gActiveBattler.get() < gBattlersCount.get() {
            if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
                BtlController_EmitSwitchInAnim(
                    B_COMM_TO_CONTROLLER,
                    ({
                        let gl_i58 = gActiveBattler.get();
                        crate::gl!(gBattlerPartyIndexes[gl_i58])
                    }) as u8,
                    FALSE,
                );
                MarkBattlerForControllerExec(gActiveBattler.get());
            }
            gActiveBattler.set(gActiveBattler.get() + (1));
        }
        (*crate::gl!(gBattleStruct)).switchInAbilitiesCounter = 0;
        (*crate::gl!(gBattleStruct)).switchInItemsCounter = 0;
        (*crate::gl!(gBattleStruct)).overworldWeatherDone = FALSE;
        crate::gl!(gBattleMainFunc = Some(TryDoEventsBeforeFirstTurn));
    }
}
pub(crate) unsafe fn TryDoEventsBeforeFirstTurn() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut effect: u8 = 0;
    if gBattleControllerExecFlags.get() != 0 {
        return;
    }
    if (*crate::gl!(gBattleStruct)).switchInAbilitiesCounter == 0 {
        for i in 0..(gBattlersCount.get() as i32) {
            crate::gl!(gBattlerByTurnOrder[i] = i as u8);
        }
        i = 0;
        while i < gBattlersCount.get() as i32 - 1 {
            j = i + 1;
            while j < gBattlersCount.get() as i32 {
                if GetWhoStrikesFirst(
                    crate::gl!(gBattlerByTurnOrder[i]),
                    crate::gl!(gBattlerByTurnOrder[j]),
                    TRUE,
                ) != 0
                {
                    SwapTurnOrder(i as u8, j as u8);
                }
                j += 1;
            }
            i += 1;
        }
    }
    if (*crate::gl!(gBattleStruct)).overworldWeatherDone == 0
        && AbilityBattleEffects(0, 0, 0, ABILITYEFFECT_SWITCH_IN_WEATHER, 0) != 0
    {
        (*crate::gl!(gBattleStruct)).overworldWeatherDone = TRUE;
        return;
    }
    while (*crate::gl!(gBattleStruct)).switchInAbilitiesCounter < gBattlersCount.get() {
        if AbilityBattleEffects(
            0,
            {
                let gl_i22 = (*crate::gl!(gBattleStruct)).switchInAbilitiesCounter;
                crate::gl!(gBattlerByTurnOrder[gl_i22])
            },
            0,
            0,
            0,
        ) != 0
        {
            effect += 1;
        }
        (*crate::gl!(gBattleStruct)).switchInAbilitiesCounter += 1;
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
    while (*crate::gl!(gBattleStruct)).switchInItemsCounter < gBattlersCount.get() {
        if ItemBattleEffects(
            0,
            {
                let gl_i23 = (*crate::gl!(gBattleStruct)).switchInItemsCounter;
                crate::gl!(gBattlerByTurnOrder[gl_i23])
            },
            0,
        ) != 0
        {
            effect += 1;
        }
        (*crate::gl!(gBattleStruct)).switchInItemsCounter += 1;
        if effect != 0 {
            return;
        }
    }
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        *(*crate::gl!(gBattleStruct))
            .monToSwitchIntoId
            .as_mut_ptr()
            .at(i) = PARTY_SIZE as u8;
        crate::gl!(gChosenActionByBattler[i] = B_ACTION_NONE);
        crate::gl!(gChosenMoveByBattler[i] = MOVE_NONE);
    }
    TurnValuesCleanUp(FALSE);
    SpecialStatusesClear();
    (*crate::gl!(gBattleStruct)).absentBattlerFlags = gAbsentBattlerFlags.get();
    BattlePutTextOnWindow(
        (*(&raw const crate::data::battle_message::gText_EmptyString3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        B_WIN_MSG,
    );
    crate::gl!(gBattleMainFunc = Some(HandleTurnActionSelectionState));
    ResetSentPokesToOpponentValue();
    for i in 0..BATTLE_COMMUNICATION_ENTRIES_COUNT {
        crate::gl!(gBattleCommunication[i] = 0);
    }
    for i in 0..(gBattlersCount.get() as i32) {
        crate::gl!(gBattleMons[i].status2 &= 0xfffffff7);
    }
    (*crate::gl!(gBattleStruct)).turnEffectsTracker = 0;
    (*crate::gl!(gBattleStruct)).turnEffectsBattlerId = 0;
    (*crate::gl!(gBattleStruct)).wishPerishSongState = 0;
    (*crate::gl!(gBattleStruct)).wishPerishSongBattlerId = 0;
    crate::gl!(gBattleScripting.moveendState = 0);
    (*crate::gl!(gBattleStruct)).faintedActionsState = 0;
    (*crate::gl!(gBattleStruct)).turnCountersTracker = 0;
    gMoveResultFlags.set(0);
    gRandomTurnNumber.set(Random());
    if gBattleTypeFlags.get() & BATTLE_TYPE_ARENA != 0 {
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
    if gBattleControllerExecFlags.get() == 0 {
        crate::gl!(gBattleMainFunc = Some(BattleTurnPassed));
        for i in 0..BATTLE_COMMUNICATION_ENTRIES_COUNT {
            crate::gl!(gBattleCommunication[i] = 0);
        }
        i = 0;
        while i < gBattlersCount.get() as i32 {
            crate::gl!(gBattleMons[i].status2 &= 0xfffffff7);
            if crate::gl!(gBattleMons[i].status1) & STATUS1_SLEEP != 0
                && crate::gl!(gBattleMons[i].status2) & STATUS2_MULTIPLETURNS != 0
            {
                CancelMultiTurnMoves(i as u8);
            }
            i += 1;
        }
        (*crate::gl!(gBattleStruct)).turnEffectsTracker = 0;
        (*crate::gl!(gBattleStruct)).turnEffectsBattlerId = 0;
        (*crate::gl!(gBattleStruct)).wishPerishSongState = 0;
        (*crate::gl!(gBattleStruct)).wishPerishSongBattlerId = 0;
        (*crate::gl!(gBattleStruct)).turnCountersTracker = 0;
        gMoveResultFlags.set(0);
    }
}
pub unsafe fn BattleTurnPassed() {
    TurnValuesCleanUp(TRUE);
    if gBattleOutcome.get() == 0 {
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
    (*crate::gl!(gBattleStruct)).faintedActionsState = 0;
    if HandleWishPerishSongOnTurnEnd() != 0 {
        return;
    }
    TurnValuesCleanUp(FALSE);
    gHitMarker.set(gHitMarker.get() & (0xfffffdff));
    gHitMarker.set(gHitMarker.get() & (0xfff7ffff));
    gHitMarker.set(gHitMarker.get() & (0xffbfffff));
    gHitMarker.set(gHitMarker.get() & (0xffefffff));
    crate::gl!(gBattleScripting.animTurn = 0);
    crate::gl!(gBattleScripting.animTargetsHit = 0);
    crate::gl!(gBattleScripting.moveendState = 0);
    gBattleMoveDamage.set(0);
    gMoveResultFlags.set(0);
    for i in 0..5i32 {
        crate::gl!(gBattleCommunication[i] = 0);
    }
    if gBattleOutcome.get() != 0 {
        gCurrentActionFuncId.set(B_ACTION_FINISHED);
        crate::gl!(gBattleMainFunc = Some(RunTurnActionsFunctions));
        return;
    }
    if crate::gl!(gBattleResults.battleTurnCounter) < 0xFF {
        crate::gl!(gBattleResults.battleTurnCounter += 1);
        (*crate::gl!(gBattleStruct)).arenaTurnCounter += 1;
    }
    let mut i: i32 = 0;
    while i < gBattlersCount.get() as i32 {
        crate::gl!(gChosenActionByBattler[i] = B_ACTION_NONE);
        crate::gl!(gChosenMoveByBattler[i] = MOVE_NONE);
        i += 1;
    }
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        *(*crate::gl!(gBattleStruct))
            .monToSwitchIntoId
            .as_mut_ptr()
            .at(i) = PARTY_SIZE as u8;
    }
    (*crate::gl!(gBattleStruct)).absentBattlerFlags = gAbsentBattlerFlags.get();
    BattlePutTextOnWindow(
        (*(&raw const crate::data::battle_message::gText_EmptyString3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        B_WIN_MSG,
    );
    crate::gl!(gBattleMainFunc = Some(HandleTurnActionSelectionState));
    gRandomTurnNumber.set(Random());
    if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 {
        BattleScriptExecute(
            (*crate::asmdata::BattleScript_PalacePrintFlavorText.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_ARENA != 0
        && (*crate::gl!(gBattleStruct)).arenaTurnCounter == 0
    {
        BattleScriptExecute(
            (*crate::asmdata::BattleScript_ArenaTurnBeginning.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
}
pub unsafe fn IsRunningFromBattleImpossible() -> u8 {
    let mut holdEffect: u8 = 0;
    if ({
        let gl_i59 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i59].item)
    }) == ITEM_ENIGMA_BERRY
    {
        holdEffect = {
            let gl_i24 = gActiveBattler.get();
            crate::gl!(gEnigmaBerries[gl_i24].holdEffect)
        };
    } else {
        holdEffect = GetItemHoldEffect({
            let gl_i60 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i60].item)
        });
    }
    gPotentialItemEffectBattler.set(gActiveBattler.get());
    if holdEffect == HOLD_EFFECT_CAN_ALWAYS_RUN {
        return BATTLE_RUN_SUCCESS;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        return BATTLE_RUN_SUCCESS;
    }
    if ({
        let gl_i61 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i61].ability)
    }) == ABILITY_RUN_AWAY
    {
        return BATTLE_RUN_SUCCESS;
    }
    let side: u8 = GetBattlerSide(gActiveBattler.get());
    let mut i: i32 = 0;
    while i < gBattlersCount.get() as i32 {
        if side != GetBattlerSide(i as u8)
            && crate::gl!(gBattleMons[i].ability) == ABILITY_SHADOW_TAG
        {
            crate::gl!(gBattleScripting.battler = i as u8);
            gLastUsedAbility.set(crate::gl!(gBattleMons[i].ability));
            crate::gl!(gBattleCommunication[5] = B_MSG_PREVENTS_ESCAPE);
            return BATTLE_RUN_FAILURE;
        }
        if side != GetBattlerSide(i as u8)
            && ({
                let gl_i62 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i62].ability)
            }) != ABILITY_LEVITATE
            && !(({
                let gl_i63 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i63].types[0])
            }) == TYPE_FLYING
                || ({
                    let gl_i64 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i64].types[1])
                }) == TYPE_FLYING)
            && crate::gl!(gBattleMons[i].ability) == ABILITY_ARENA_TRAP
        {
            crate::gl!(gBattleScripting.battler = i as u8);
            gLastUsedAbility.set(crate::gl!(gBattleMons[i].ability));
            crate::gl!(gBattleCommunication[5] = B_MSG_PREVENTS_ESCAPE);
            return BATTLE_RUN_FAILURE;
        }
        i += 1;
    }
    i = AbilityBattleEffects(
        ABILITYEFFECT_CHECK_FIELD_EXCEPT_BATTLER,
        gActiveBattler.get(),
        ABILITY_MAGNET_PULL,
        0,
        0,
    ) as i32;
    if i != 0
        && (({
            let gl_i65 = gActiveBattler.get();
            crate::gl!(gBattleMons[gl_i65].types[0])
        }) == TYPE_STEEL
            || ({
                let gl_i66 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i66].types[1])
            }) == TYPE_STEEL)
    {
        crate::gl!(gBattleScripting.battler = i as u8 - 1);
        gLastUsedAbility.set({
            let gl_i67 = i - 1;
            crate::gl!(gBattleMons[gl_i67].ability)
        });
        crate::gl!(gBattleCommunication[5] = B_MSG_PREVENTS_ESCAPE);
        return BATTLE_RUN_FAILURE;
    }
    if ({
        let gl_i68 = gActiveBattler.get();
        crate::gl!(gBattleMons[gl_i68].status2)
    }) & 0x400e000
        != 0
        || {
            let gl_i25 = gActiveBattler.get();
            crate::gl!(gStatuses3[gl_i25])
        } & STATUS3_ROOTED
            != 0
    {
        crate::gl!(gBattleCommunication[5] = B_MSG_CANT_ESCAPE);
        return BATTLE_RUN_FORBIDDEN;
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_FIRST_BATTLE != 0 {
        crate::gl!(gBattleCommunication[5] = B_MSG_DONT_LEAVE_BIRCH);
        return BATTLE_RUN_FORBIDDEN;
    }
    BATTLE_RUN_SUCCESS
}
pub unsafe fn SwitchPartyOrder(battler: u8) {
    let mut i: i32 = 0;
    while i < 3 {
        crate::gl!(
            gBattlePartyCurrentOrder[i] =
                *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
                    .at(battler as i32 * 3 + i)
        );
        i += 1;
    }
    let partyId1: u8 = GetPartyIdFromBattlePartyId(crate::gl!(gBattlerPartyIndexes[battler]) as u8);
    let partyId2: u8 = GetPartyIdFromBattlePartyId(
        *(*crate::gl!(gBattleStruct))
            .monToSwitchIntoId
            .as_mut_ptr()
            .at(battler),
    );
    SwitchPartyMonSlots(partyId1, partyId2);
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0 {
        for i in 0..3i32 {
            *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
                .at(battler as i32 * 3 + i) = crate::gl!(gBattlePartyCurrentOrder[i]);
            *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
                .at((battler as i32 ^ 2) * 3 + i) = crate::gl!(gBattlePartyCurrentOrder[i]);
        }
    } else {
        for i in 0..3i32 {
            *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
                .at(battler as i32 * 3 + i) = crate::gl!(gBattlePartyCurrentOrder[i]);
        }
    }
}
pub(crate) unsafe fn HandleTurnActionSelectionState() {
    let mut i: i32 = 0;
    crate::gl!(gBattleCommunication[4] = 0);
    gActiveBattler.set(0);
    while gActiveBattler.get() < gBattlersCount.get() {
        let position: u8 = GetBattlerPosition(gActiveBattler.get());
        match {
            let gl_i26 = gActiveBattler.get();
            crate::gl!(gBattleCommunication[gl_i26])
        } {
            STATE_TURN_START_RECORD => {
                RecordedBattle_CopyBattlerMoves();
                {
                    let gl_v = STATE_BEFORE_ACTION_CHOSEN;
                    let gl_i27 = gActiveBattler.get();
                    crate::gl!(gBattleCommunication[gl_i27] = gl_v)
                };
            }
            STATE_BEFORE_ACTION_CHOSEN => {
                *(*crate::gl!(gBattleStruct))
                    .monToSwitchIntoId
                    .as_mut_ptr()
                    .at(gActiveBattler.get()) = PARTY_SIZE as u8;
                if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
                    || position as i32 & BIT_FLANK as i32 == B_FLANK_LEFT
                    || (*crate::gl!(gBattleStruct)).absentBattlerFlags as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                            [GetBattlerAtPosition(position ^ 2)]
                        != 0
                    || {
                        let gl_i28 = GetBattlerAtPosition(position ^ 2);
                        crate::gl!(gBattleCommunication[gl_i28])
                    } == STATE_WAIT_ACTION_CONFIRMED
                {
                    if (*crate::gl!(gBattleStruct)).absentBattlerFlags as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
                            [gActiveBattler.get()]
                        != 0
                    {
                        {
                            let gl_v = B_ACTION_NOTHING_FAINTED;
                            let gl_i29 = gActiveBattler.get();
                            crate::gl!(gChosenActionByBattler[gl_i29] = gl_v)
                        };
                        if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI == 0 {
                            {
                                let gl_v = STATE_WAIT_ACTION_CONFIRMED;
                                let gl_i30 = gActiveBattler.get();
                                crate::gl!(gBattleCommunication[gl_i30] = gl_v)
                            };
                        } else {
                            {
                                let gl_v = STATE_WAIT_ACTION_CONFIRMED_STANDBY;
                                let gl_i31 = gActiveBattler.get();
                                crate::gl!(gBattleCommunication[gl_i31] = gl_v)
                            };
                        }
                    } else {
                        if ({
                            let gl_i69 = gActiveBattler.get();
                            crate::gl!(gBattleMons[gl_i69].status2)
                        }) & STATUS2_MULTIPLETURNS
                            != 0
                            || ({
                                let gl_i70 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i70].status2)
                            }) & STATUS2_RECHARGE
                                != 0
                        {
                            {
                                let gl_v = B_ACTION_USE_MOVE;
                                let gl_i32 = gActiveBattler.get();
                                crate::gl!(gChosenActionByBattler[gl_i32] = gl_v)
                            };
                            {
                                let gl_v = STATE_WAIT_ACTION_CONFIRMED_STANDBY;
                                let gl_i33 = gActiveBattler.get();
                                crate::gl!(gBattleCommunication[gl_i33] = gl_v)
                            };
                        } else {
                            BtlController_EmitChooseAction(
                                B_COMM_TO_CONTROLLER,
                                crate::gl!(gChosenActionByBattler[0]),
                                crate::gl!(gBattleBufferB[0][1]) as u16
                                    | (crate::gl!(gBattleBufferB[0][2]) as u16) << 8,
                            );
                            MarkBattlerForControllerExec(gActiveBattler.get());
                            {
                                let gl_v = 1;
                                let gl_i34 = gActiveBattler.get();
                                crate::gl!(gBattleCommunication[gl_i34] += gl_v)
                            };
                        }
                    }
                }
            }
            STATE_WAIT_ACTION_CHOSEN => {
                if gBattleControllerExecFlags.get()
                    & (gBitTable[gActiveBattler.get()]
                        | 0xf0000000
                        | gBitTable[gActiveBattler.get()] << 4
                        | gBitTable[gActiveBattler.get()] << 8
                        | gBitTable[gActiveBattler.get()] << 12)
                    == 0
                {
                    RecordedBattle_SetBattlerAction(gActiveBattler.get(), {
                        let gl_i35 = gActiveBattler.get();
                        crate::gl!(gBattleBufferB[gl_i35][1])
                    });
                    {
                        let gl_v = {
                            let gl_i37 = gActiveBattler.get();
                            crate::gl!(gBattleBufferB[gl_i37][1])
                        };
                        let gl_i36 = gActiveBattler.get();
                        crate::gl!(gChosenActionByBattler[gl_i36] = gl_v)
                    };
                    match {
                        let gl_i38 = gActiveBattler.get();
                        crate::gl!(gBattleBufferB[gl_i38][1])
                    } {
                        B_ACTION_USE_MOVE => {
                            if AreAllMovesUnusable() != 0 {
                                {
                                    let gl_v = STATE_SELECTION_SCRIPT;
                                    let gl_i39 = gActiveBattler.get();
                                    crate::gl!(gBattleCommunication[gl_i39] = gl_v)
                                };
                                *(*crate::gl!(gBattleStruct))
                                    .selectionScriptFinished
                                    .as_mut_ptr()
                                    .at(gActiveBattler.get()) = FALSE;
                                *(*crate::gl!(gBattleStruct))
                                    .stateIdAfterSelScript
                                    .as_mut_ptr()
                                    .at(gActiveBattler.get()) = STATE_WAIT_ACTION_CONFIRMED_STANDBY;
                                *(*crate::gl!(gBattleStruct))
                                    .moveTarget
                                    .as_mut_ptr()
                                    .at(gActiveBattler.get()) = {
                                    let gl_i40 = gActiveBattler.get();
                                    crate::gl!(gBattleBufferB[gl_i40][3])
                                };
                                return;
                            } else if ({
                                let gl_i71 = gActiveBattler.get();
                                crate::gl!(gDisableStructs[gl_i71].encoredMove)
                            }) != 0
                            {
                                {
                                    let gl_v = {
                                        let gl_i72 = gActiveBattler.get();
                                        crate::gl!(gDisableStructs[gl_i72].encoredMove)
                                    };
                                    let gl_i41 = gActiveBattler.get();
                                    crate::gl!(gChosenMoveByBattler[gl_i41] = gl_v)
                                };
                                *(*crate::gl!(gBattleStruct))
                                    .chosenMovePositions
                                    .as_mut_ptr()
                                    .at(gActiveBattler.get()) = {
                                    let gl_i73 = gActiveBattler.get();
                                    crate::gl!(gDisableStructs[gl_i73].encoredMovePos)
                                };
                                {
                                    let gl_v = STATE_WAIT_ACTION_CONFIRMED_STANDBY;
                                    let gl_i42 = gActiveBattler.get();
                                    crate::gl!(gBattleCommunication[gl_i42] = gl_v)
                                };
                                return;
                            } else {
                                let mut moveInfo: ChooseMoveStruct = zeroed();
                                moveInfo.species = {
                                    let gl_i74 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i74].species)
                                };
                                moveInfo.monTypes[0] = {
                                    let gl_i75 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i75].types[0])
                                };
                                moveInfo.monTypes[1] = {
                                    let gl_i76 = gActiveBattler.get();
                                    crate::gl!(gBattleMons[gl_i76].types[1])
                                };
                                for i in 0..MAX_MON_MOVES {
                                    moveInfo.moves[i] = {
                                        let gl_i77 = gActiveBattler.get();
                                        crate::gl!(gBattleMons[gl_i77].moves[i])
                                    };
                                    moveInfo.currentPP[i] = {
                                        let gl_i78 = gActiveBattler.get();
                                        crate::gl!(gBattleMons[gl_i78].pp[i])
                                    };
                                    moveInfo.maxPP[i] = CalculatePPWithBonus(
                                        {
                                            let gl_i79 = gActiveBattler.get();
                                            crate::gl!(gBattleMons[gl_i79].moves[i])
                                        },
                                        {
                                            let gl_i80 = gActiveBattler.get();
                                            crate::gl!(gBattleMons[gl_i80].ppBonuses)
                                        },
                                        i as u8,
                                    );
                                }
                                BtlController_EmitChooseMove(
                                    B_COMM_TO_CONTROLLER,
                                    (gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0) as u8,
                                    0,
                                    &raw mut moveInfo,
                                );
                                MarkBattlerForControllerExec(gActiveBattler.get());
                            }
                        }
                        B_ACTION_USE_ITEM => {
                            if gBattleTypeFlags.get() & 0x21f0902 != 0 {
                                RecordedBattle_ClearBattlerAction(gActiveBattler.get(), 1);
                                {
                                    let gl_v = (*crate::asmdata::BattleScript_ActionSelectionItemsCantBeUsed
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut();
                                    let gl_i43 = gActiveBattler.get();
                                    crate::gl!(gSelectionBattleScripts[gl_i43] = gl_v)
                                };
                                {
                                    let gl_v = STATE_SELECTION_SCRIPT;
                                    let gl_i44 = gActiveBattler.get();
                                    crate::gl!(gBattleCommunication[gl_i44] = gl_v)
                                };
                                *(*crate::gl!(gBattleStruct))
                                    .selectionScriptFinished
                                    .as_mut_ptr()
                                    .at(gActiveBattler.get()) = FALSE;
                                *(*crate::gl!(gBattleStruct))
                                    .stateIdAfterSelScript
                                    .as_mut_ptr()
                                    .at(gActiveBattler.get()) = STATE_BEFORE_ACTION_CHOSEN;
                                return;
                            } else {
                                BtlController_EmitChooseItem(
                                    B_COMM_TO_CONTROLLER,
                                    (*crate::gl!(gBattleStruct)).battlerPartyOrders
                                        [gActiveBattler.get()]
                                    .as_mut_ptr(),
                                );
                                MarkBattlerForControllerExec(gActiveBattler.get());
                            }
                        }
                        B_ACTION_SWITCH => {
                            *(*crate::gl!(gBattleStruct))
                                .battlerPartyIndexes
                                .as_mut_ptr()
                                .at(gActiveBattler.get()) = ({
                                let gl_i81 = gActiveBattler.get();
                                crate::gl!(gBattlerPartyIndexes[gl_i81])
                            }) as u8;
                            if ({
                                let gl_i82 = gActiveBattler.get();
                                crate::gl!(gBattleMons[gl_i82].status2)
                            }) & 0x400e000
                                != 0
                                || gBattleTypeFlags.get() & BATTLE_TYPE_ARENA != 0
                                || {
                                    let gl_i45 = gActiveBattler.get();
                                    crate::gl!(gStatuses3[gl_i45])
                                } & STATUS3_ROOTED
                                    != 0
                            {
                                BtlController_EmitChoosePokemon(
                                    B_COMM_TO_CONTROLLER,
                                    PARTY_ACTION_CANT_SWITCH,
                                    PARTY_SIZE as u8,
                                    ABILITY_NONE,
                                    (*crate::gl!(gBattleStruct)).battlerPartyOrders
                                        [gActiveBattler.get()]
                                    .as_mut_ptr(),
                                );
                            } else if ({
                                i = AbilityBattleEffects(
                                    12,
                                    gActiveBattler.get(),
                                    ABILITY_SHADOW_TAG,
                                    0,
                                    0,
                                ) as i32;
                                i
                            }) != 0
                                || ({
                                    i = AbilityBattleEffects(
                                        12,
                                        gActiveBattler.get(),
                                        ABILITY_ARENA_TRAP,
                                        0,
                                        0,
                                    ) as i32;
                                    i
                                }) != 0
                                    && !(({
                                        let gl_i83 = gActiveBattler.get();
                                        crate::gl!(gBattleMons[gl_i83].types[0])
                                    }) == TYPE_FLYING
                                        || ({
                                            let gl_i84 = gActiveBattler.get();
                                            crate::gl!(gBattleMons[gl_i84].types[1])
                                        }) == TYPE_FLYING)
                                    && ({
                                        let gl_i85 = gActiveBattler.get();
                                        crate::gl!(gBattleMons[gl_i85].ability)
                                    }) != ABILITY_LEVITATE
                                || ({
                                    i = AbilityBattleEffects(
                                        ABILITYEFFECT_CHECK_FIELD_EXCEPT_BATTLER,
                                        gActiveBattler.get(),
                                        ABILITY_MAGNET_PULL,
                                        0,
                                        0,
                                    ) as i32;
                                    i
                                }) != 0
                                    && (({
                                        let gl_i86 = gActiveBattler.get();
                                        crate::gl!(gBattleMons[gl_i86].types[0])
                                    }) == TYPE_STEEL
                                        || ({
                                            let gl_i87 = gActiveBattler.get();
                                            crate::gl!(gBattleMons[gl_i87].types[1])
                                        }) == TYPE_STEEL)
                            {
                                BtlController_EmitChoosePokemon(
                                    B_COMM_TO_CONTROLLER,
                                    (i as u8 - 1) << 4 | 4,
                                    PARTY_SIZE as u8,
                                    gLastUsedAbility.get(),
                                    (*crate::gl!(gBattleStruct)).battlerPartyOrders
                                        [gActiveBattler.get()]
                                    .as_mut_ptr(),
                                );
                            } else {
                                if gActiveBattler.get() == 2
                                    && crate::gl!(gChosenActionByBattler[0]) == 2
                                {
                                    BtlController_EmitChoosePokemon(
                                        B_COMM_TO_CONTROLLER,
                                        0,
                                        *(*crate::gl!(gBattleStruct))
                                            .monToSwitchIntoId
                                            .as_mut_ptr(),
                                        0,
                                        (*crate::gl!(gBattleStruct)).battlerPartyOrders
                                            [gActiveBattler.get()]
                                        .as_mut_ptr(),
                                    );
                                } else if gActiveBattler.get() == 3
                                    && crate::gl!(gChosenActionByBattler[1]) == B_ACTION_SWITCH
                                {
                                    BtlController_EmitChoosePokemon(
                                        B_COMM_TO_CONTROLLER,
                                        0,
                                        *(*crate::gl!(gBattleStruct))
                                            .monToSwitchIntoId
                                            .as_mut_ptr()
                                            .at(1),
                                        0,
                                        (*crate::gl!(gBattleStruct)).battlerPartyOrders
                                            [gActiveBattler.get()]
                                        .as_mut_ptr(),
                                    );
                                } else {
                                    BtlController_EmitChoosePokemon(
                                        B_COMM_TO_CONTROLLER,
                                        0,
                                        PARTY_SIZE as u8,
                                        0,
                                        (*crate::gl!(gBattleStruct)).battlerPartyOrders
                                            [gActiveBattler.get()]
                                        .as_mut_ptr(),
                                    );
                                }
                            }
                            MarkBattlerForControllerExec(gActiveBattler.get());
                        }
                        B_ACTION_SAFARI_BALL => {
                            if IsPlayerPartyAndPokemonStorageFull() != 0 {
                                {
                                    let gl_v = (*crate::asmdata::BattleScript_PrintFullBox
                                        .cast::<CArray<u8, 0>>())
                                    .as_ptr()
                                    .cast_mut();
                                    let gl_i46 = gActiveBattler.get();
                                    crate::gl!(gSelectionBattleScripts[gl_i46] = gl_v)
                                };
                                {
                                    let gl_v = STATE_SELECTION_SCRIPT;
                                    let gl_i47 = gActiveBattler.get();
                                    crate::gl!(gBattleCommunication[gl_i47] = gl_v)
                                };
                                *(*crate::gl!(gBattleStruct))
                                    .selectionScriptFinished
                                    .as_mut_ptr()
                                    .at(gActiveBattler.get()) = FALSE;
                                *(*crate::gl!(gBattleStruct))
                                    .stateIdAfterSelScript
                                    .as_mut_ptr()
                                    .at(gActiveBattler.get()) = STATE_BEFORE_ACTION_CHOSEN;
                                return;
                            }
                        }
                        B_ACTION_SAFARI_POKEBLOCK => {
                            BtlController_EmitChooseItem(
                                B_COMM_TO_CONTROLLER,
                                (*crate::gl!(gBattleStruct)).battlerPartyOrders
                                    [gActiveBattler.get()]
                                .as_mut_ptr(),
                            );
                            MarkBattlerForControllerExec(gActiveBattler.get());
                        }
                        B_ACTION_CANCEL_PARTNER => {
                            {
                                let gl_v = STATE_WAIT_SET_BEFORE_ACTION;
                                let gl_i48 = gActiveBattler.get();
                                crate::gl!(gBattleCommunication[gl_i48] = gl_v)
                            };
                            {
                                let gl_v = STATE_BEFORE_ACTION_CHOSEN;
                                let gl_i49 = GetBattlerAtPosition(
                                    GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                );
                                crate::gl!(gBattleCommunication[gl_i49] = gl_v)
                            };
                            RecordedBattle_ClearBattlerAction(gActiveBattler.get(), 1);
                            if ({
                                let gl_i88 = GetBattlerAtPosition(
                                    GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                );
                                crate::gl!(gBattleMons[gl_i88].status2)
                            }) & STATUS2_MULTIPLETURNS
                                != 0
                                || ({
                                    let gl_i89 = GetBattlerAtPosition(
                                        GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                    );
                                    crate::gl!(gBattleMons[gl_i89].status2)
                                }) & STATUS2_RECHARGE
                                    != 0
                            {
                                BtlController_EmitEndBounceEffect(B_COMM_TO_CONTROLLER);
                                MarkBattlerForControllerExec(gActiveBattler.get());
                                return;
                            } else if {
                                let gl_i50 = GetBattlerAtPosition(
                                    GetBattlerPosition(gActiveBattler.get()) ^ B_ACTION_SWITCH,
                                );
                                crate::gl!(gChosenActionByBattler[gl_i50])
                            } == B_ACTION_SWITCH
                            {
                                RecordedBattle_ClearBattlerAction(
                                    GetBattlerAtPosition(
                                        GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                    ),
                                    2,
                                );
                            } else if {
                                let gl_i51 = GetBattlerAtPosition(
                                    GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                );
                                crate::gl!(gChosenActionByBattler[gl_i51])
                            } == B_ACTION_RUN
                            {
                                RecordedBattle_ClearBattlerAction(
                                    GetBattlerAtPosition(
                                        GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                    ),
                                    1,
                                );
                            } else if {
                                let gl_i52 = GetBattlerAtPosition(
                                    GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                );
                                crate::gl!(gChosenActionByBattler[gl_i52])
                            } == B_ACTION_USE_MOVE
                                && ((*gProtectStructs.as_ptr())[GetBattlerAtPosition(
                                    GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                )]
                                .noValidMoves()
                                    != 0
                                    || ({
                                        let gl_i90 = GetBattlerAtPosition(
                                            GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                        );
                                        crate::gl!(gDisableStructs[gl_i90].encoredMove)
                                    }) != 0)
                            {
                                RecordedBattle_ClearBattlerAction(
                                    GetBattlerAtPosition(
                                        GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                    ),
                                    1,
                                );
                            } else if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE != 0 && {
                                let gl_i53 = GetBattlerAtPosition(
                                    GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                );
                                crate::gl!(gChosenActionByBattler[gl_i53])
                            }
                                == B_ACTION_USE_MOVE
                            {
                                (*crate::random::gRngValue.as_ptr().cast::<u32>()) =
                                    gBattlePalaceMoveSelectionRngValue.get();
                                RecordedBattle_ClearBattlerAction(
                                    GetBattlerAtPosition(
                                        GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                    ),
                                    1,
                                );
                            } else {
                                RecordedBattle_ClearBattlerAction(
                                    GetBattlerAtPosition(
                                        GetBattlerPosition(gActiveBattler.get()) ^ 2,
                                    ),
                                    3,
                                );
                            }
                            BtlController_EmitEndBounceEffect(B_COMM_TO_CONTROLLER);
                            MarkBattlerForControllerExec(gActiveBattler.get());
                            return;
                        }
                        _ => {}
                    }
                    if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0
                        && gBattleTypeFlags.get() & 0x43f0100 != 0
                        && {
                            let gl_i54 = gActiveBattler.get();
                            crate::gl!(gBattleBufferB[gl_i54][1])
                        } == B_ACTION_RUN
                    {
                        {
                            let gl_v = (*crate::asmdata::BattleScript_AskIfWantsToForfeitMatch
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                            let gl_i55 = gActiveBattler.get();
                            crate::gl!(gSelectionBattleScripts[gl_i55] = gl_v)
                        };
                        {
                            let gl_v = STATE_SELECTION_SCRIPT_MAY_RUN;
                            let gl_i56 = gActiveBattler.get();
                            crate::gl!(gBattleCommunication[gl_i56] = gl_v)
                        };
                        *(*crate::gl!(gBattleStruct))
                            .selectionScriptFinished
                            .as_mut_ptr()
                            .at(gActiveBattler.get()) = FALSE;
                        *(*crate::gl!(gBattleStruct))
                            .stateIdAfterSelScript
                            .as_mut_ptr()
                            .at(gActiveBattler.get()) = STATE_BEFORE_ACTION_CHOSEN;
                        return;
                    } else if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0
                        && gBattleTypeFlags.get() & 0x2000002 == 0
                        && {
                            let gl_i57 = gActiveBattler.get();
                            crate::gl!(gBattleBufferB[gl_i57][1])
                        } == B_ACTION_RUN
                    {
                        BattleScriptExecute(
                            (*crate::asmdata::BattleScript_PrintCantRunFromTrainer
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                        );
                        {
                            let gl_v = STATE_BEFORE_ACTION_CHOSEN;
                            let gl_i58 = gActiveBattler.get();
                            crate::gl!(gBattleCommunication[gl_i58] = gl_v)
                        };
                    } else if IsRunningFromBattleImpossible() != BATTLE_RUN_SUCCESS && {
                        let gl_i59 = gActiveBattler.get();
                        crate::gl!(gBattleBufferB[gl_i59][1])
                    }
                        == B_ACTION_RUN
                    {
                        {
                            let gl_v = (*crate::asmdata::BattleScript_PrintCantEscapeFromBattle
                                .cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut();
                            let gl_i60 = gActiveBattler.get();
                            crate::gl!(gSelectionBattleScripts[gl_i60] = gl_v)
                        };
                        {
                            let gl_v = STATE_SELECTION_SCRIPT;
                            let gl_i61 = gActiveBattler.get();
                            crate::gl!(gBattleCommunication[gl_i61] = gl_v)
                        };
                        *(*crate::gl!(gBattleStruct))
                            .selectionScriptFinished
                            .as_mut_ptr()
                            .at(gActiveBattler.get()) = FALSE;
                        *(*crate::gl!(gBattleStruct))
                            .stateIdAfterSelScript
                            .as_mut_ptr()
                            .at(gActiveBattler.get()) = STATE_BEFORE_ACTION_CHOSEN;
                        return;
                    } else {
                        {
                            let gl_v = 1;
                            let gl_i62 = gActiveBattler.get();
                            crate::gl!(gBattleCommunication[gl_i62] += gl_v)
                        };
                    }
                }
            }
            STATE_WAIT_ACTION_CASE_CHOSEN => {
                if gBattleControllerExecFlags.get()
                    & (gBitTable[gActiveBattler.get()]
                        | 0xf0000000
                        | gBitTable[gActiveBattler.get()] << 4
                        | gBitTable[gActiveBattler.get()] << 8
                        | gBitTable[gActiveBattler.get()] << 12)
                    == 0
                {
                    match {
                        let gl_i63 = gActiveBattler.get();
                        crate::gl!(gChosenActionByBattler[gl_i63])
                    } {
                        B_ACTION_USE_MOVE => match {
                            let gl_i64 = gActiveBattler.get();
                            crate::gl!(gBattleBufferB[gl_i64][1])
                        } {
                            3..=9 => {
                                {
                                    let gl_v = {
                                        let gl_i66 = gActiveBattler.get();
                                        crate::gl!(gBattleBufferB[gl_i66][1])
                                    };
                                    let gl_i65 = gActiveBattler.get();
                                    crate::gl!(gChosenActionByBattler[gl_i65] = gl_v)
                                };
                                return;
                            }
                            15 => {
                                {
                                    let gl_v = B_ACTION_SWITCH;
                                    let gl_i67 = gActiveBattler.get();
                                    crate::gl!(gChosenActionByBattler[gl_i67] = gl_v)
                                };
                                UpdateBattlerPartyOrdersOnSwitch();
                                return;
                            }
                            _ => {
                                RecordedBattle_CheckMovesetChanges(B_RECORD_MODE_PLAYBACK);
                                if {
                                    let gl_i68 = gActiveBattler.get();
                                    crate::gl!(gBattleBufferB[gl_i68][2])
                                } as i32
                                    | ({
                                        let gl_i69 = gActiveBattler.get();
                                        crate::gl!(gBattleBufferB[gl_i69][3])
                                    } as i32)
                                        << 8
                                    == 0xFFFF
                                {
                                    {
                                        let gl_v = STATE_BEFORE_ACTION_CHOSEN;
                                        let gl_i70 = gActiveBattler.get();
                                        crate::gl!(gBattleCommunication[gl_i70] = gl_v)
                                    };
                                    RecordedBattle_ClearBattlerAction(gActiveBattler.get(), 1);
                                } else if TrySetCantSelectMoveBattleScript() != 0 {
                                    RecordedBattle_ClearBattlerAction(gActiveBattler.get(), 1);
                                    {
                                        let gl_v = STATE_SELECTION_SCRIPT;
                                        let gl_i71 = gActiveBattler.get();
                                        crate::gl!(gBattleCommunication[gl_i71] = gl_v)
                                    };
                                    *(*crate::gl!(gBattleStruct))
                                        .selectionScriptFinished
                                        .as_mut_ptr()
                                        .at(gActiveBattler.get()) = FALSE;
                                    {
                                        let gl_v = B_ACTION_USE_MOVE;
                                        let gl_i72 = gActiveBattler.get();
                                        crate::gl!(gBattleBufferB[gl_i72][1] = gl_v)
                                    };
                                    *(*crate::gl!(gBattleStruct))
                                        .stateIdAfterSelScript
                                        .as_mut_ptr()
                                        .at(gActiveBattler.get()) = STATE_WAIT_ACTION_CHOSEN;
                                    return;
                                } else {
                                    if gBattleTypeFlags.get() & BATTLE_TYPE_PALACE == 0 {
                                        RecordedBattle_SetBattlerAction(gActiveBattler.get(), {
                                            let gl_i73 = gActiveBattler.get();
                                            crate::gl!(gBattleBufferB[gl_i73][2])
                                        });
                                        RecordedBattle_SetBattlerAction(gActiveBattler.get(), {
                                            let gl_i74 = gActiveBattler.get();
                                            crate::gl!(gBattleBufferB[gl_i74][3])
                                        });
                                    }
                                    *(*crate::gl!(gBattleStruct))
                                        .chosenMovePositions
                                        .as_mut_ptr()
                                        .at(gActiveBattler.get()) = {
                                        let gl_i75 = gActiveBattler.get();
                                        crate::gl!(gBattleBufferB[gl_i75][2])
                                    };
                                    {
                                        let gl_v = (*gBattleMons.as_ptr())[gActiveBattler.get()]
                                            .moves[*(*crate::gl!(gBattleStruct))
                                            .chosenMovePositions
                                            .as_mut_ptr()
                                            .at(gActiveBattler.get())];
                                        let gl_i76 = gActiveBattler.get();
                                        crate::gl!(gChosenMoveByBattler[gl_i76] = gl_v)
                                    };
                                    *(*crate::gl!(gBattleStruct))
                                        .moveTarget
                                        .as_mut_ptr()
                                        .at(gActiveBattler.get()) = {
                                        let gl_i77 = gActiveBattler.get();
                                        crate::gl!(gBattleBufferB[gl_i77][3])
                                    };
                                    {
                                        let gl_v = 1;
                                        let gl_i78 = gActiveBattler.get();
                                        crate::gl!(gBattleCommunication[gl_i78] += gl_v)
                                    };
                                }
                            }
                        },
                        B_ACTION_USE_ITEM => {
                            if {
                                let gl_i79 = gActiveBattler.get();
                                crate::gl!(gBattleBufferB[gl_i79][1])
                            } as i32
                                | ({
                                    let gl_i80 = gActiveBattler.get();
                                    crate::gl!(gBattleBufferB[gl_i80][2])
                                } as i32)
                                    << 8
                                == 0
                            {
                                {
                                    let gl_v = STATE_BEFORE_ACTION_CHOSEN;
                                    let gl_i81 = gActiveBattler.get();
                                    crate::gl!(gBattleCommunication[gl_i81] = gl_v)
                                };
                            } else {
                                gLastUsedItem.set(
                                    {
                                        let gl_i82 = gActiveBattler.get();
                                        crate::gl!(gBattleBufferB[gl_i82][1])
                                    } as u16
                                        | ({
                                            let gl_i83 = gActiveBattler.get();
                                            crate::gl!(gBattleBufferB[gl_i83][2])
                                        } as u16)
                                            << 8,
                                );
                                {
                                    let gl_v = 1;
                                    let gl_i84 = gActiveBattler.get();
                                    crate::gl!(gBattleCommunication[gl_i84] += gl_v)
                                };
                            }
                        }
                        B_ACTION_SWITCH => {
                            if {
                                let gl_i85 = gActiveBattler.get();
                                crate::gl!(gBattleBufferB[gl_i85][1])
                            } == PARTY_SIZE as u8
                            {
                                {
                                    let gl_v = STATE_BEFORE_ACTION_CHOSEN;
                                    let gl_i86 = gActiveBattler.get();
                                    crate::gl!(gBattleCommunication[gl_i86] = gl_v)
                                };
                                RecordedBattle_ClearBattlerAction(gActiveBattler.get(), 1);
                            } else {
                                UpdateBattlerPartyOrdersOnSwitch();
                                {
                                    let gl_v = 1;
                                    let gl_i87 = gActiveBattler.get();
                                    crate::gl!(gBattleCommunication[gl_i87] += gl_v)
                                };
                            }
                        }
                        B_ACTION_RUN => {
                            gHitMarker.set(gHitMarker.get() | (HITMARKER_RUN));
                            {
                                let gl_v = 1;
                                let gl_i88 = gActiveBattler.get();
                                crate::gl!(gBattleCommunication[gl_i88] += gl_v)
                            };
                        }
                        B_ACTION_SAFARI_WATCH_CAREFULLY => {
                            {
                                let gl_v = 1;
                                let gl_i89 = gActiveBattler.get();
                                crate::gl!(gBattleCommunication[gl_i89] += gl_v)
                            };
                        }
                        B_ACTION_SAFARI_BALL => {
                            {
                                let gl_v = 1;
                                let gl_i90 = gActiveBattler.get();
                                crate::gl!(gBattleCommunication[gl_i90] += gl_v)
                            };
                        }
                        B_ACTION_SAFARI_POKEBLOCK => {
                            if {
                                let gl_i91 = gActiveBattler.get();
                                crate::gl!(gBattleBufferB[gl_i91][1])
                            } as i32
                                | ({
                                    let gl_i92 = gActiveBattler.get();
                                    crate::gl!(gBattleBufferB[gl_i92][2])
                                } as i32)
                                    << 8
                                != 0
                            {
                                {
                                    let gl_v = 1;
                                    let gl_i93 = gActiveBattler.get();
                                    crate::gl!(gBattleCommunication[gl_i93] += gl_v)
                                };
                            } else {
                                {
                                    let gl_v = STATE_BEFORE_ACTION_CHOSEN;
                                    let gl_i94 = gActiveBattler.get();
                                    crate::gl!(gBattleCommunication[gl_i94] = gl_v)
                                };
                            }
                        }
                        B_ACTION_SAFARI_GO_NEAR => {
                            {
                                let gl_v = 1;
                                let gl_i95 = gActiveBattler.get();
                                crate::gl!(gBattleCommunication[gl_i95] += gl_v)
                            };
                        }
                        B_ACTION_SAFARI_RUN => {
                            gHitMarker.set(gHitMarker.get() | (HITMARKER_RUN));
                            {
                                let gl_v = 1;
                                let gl_i96 = gActiveBattler.get();
                                crate::gl!(gBattleCommunication[gl_i96] += gl_v)
                            };
                        }
                        B_ACTION_WALLY_THROW => {
                            {
                                let gl_v = 1;
                                let gl_i97 = gActiveBattler.get();
                                crate::gl!(gBattleCommunication[gl_i97] += gl_v)
                            };
                        }
                        _ => {}
                    }
                }
            }
            STATE_WAIT_ACTION_CONFIRMED_STANDBY => {
                if gBattleControllerExecFlags.get()
                    & (gBitTable[gActiveBattler.get()]
                        | 0xf0000000
                        | gBitTable[gActiveBattler.get()] << 4
                        | gBitTable[gActiveBattler.get()] << 8
                        | gBitTable[gActiveBattler.get()] << 12)
                    == 0
                {
                    if AllAtActionConfirmed() != 0 {
                        i = TRUE as i32;
                    } else {
                        i = FALSE as i32;
                    }
                    if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
                        || gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE == 0
                        || position as i32 & BIT_FLANK as i32 != B_FLANK_LEFT
                        || (*crate::gl!(gBattleStruct)).absentBattlerFlags as u32
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
                    MarkBattlerForControllerExec(gActiveBattler.get());
                    {
                        let gl_v = 1;
                        let gl_i98 = gActiveBattler.get();
                        crate::gl!(gBattleCommunication[gl_i98] += gl_v)
                    };
                }
            }
            STATE_WAIT_ACTION_CONFIRMED => {
                if gBattleControllerExecFlags.get()
                    & (gBitTable[gActiveBattler.get()]
                        | 0xf0000000
                        | gBitTable[gActiveBattler.get()] << 4
                        | gBitTable[gActiveBattler.get()] << 8
                        | gBitTable[gActiveBattler.get()] << 12)
                    == 0
                {
                    crate::gl!(gBattleCommunication[4] += 1);
                }
            }
            STATE_SELECTION_SCRIPT => {
                if *(*crate::gl!(gBattleStruct))
                    .selectionScriptFinished
                    .as_mut_ptr()
                    .at(gActiveBattler.get())
                    != 0
                {
                    {
                        let gl_v = *(*crate::gl!(gBattleStruct))
                            .stateIdAfterSelScript
                            .as_mut_ptr()
                            .at(gActiveBattler.get());
                        let gl_i99 = gActiveBattler.get();
                        crate::gl!(gBattleCommunication[gl_i99] = gl_v)
                    };
                } else {
                    gBattlerAttacker.set(gActiveBattler.get());
                    crate::gl!(
                        gBattlescriptCurrInstr = {
                            let gl_i100 = gActiveBattler.get();
                            crate::gl!(gSelectionBattleScripts[gl_i100])
                        }
                    );
                    if gBattleControllerExecFlags.get()
                        & (gBitTable[gActiveBattler.get()]
                            | 0xf0000000
                            | gBitTable[gActiveBattler.get()] << 4
                            | gBitTable[gActiveBattler.get()] << 8
                            | gBitTable[gActiveBattler.get()] << 12)
                        == 0
                    {
                        (*(&raw const crate::data::battle_script_commands::gBattleScriptingCommandsTable).cast::<CArray<Option<unsafe fn()>, 0>>())[*crate::gl!(gBattlescriptCurrInstr)].unwrap_unchecked()();
                    }
                    {
                        let gl_v = crate::gl!(gBattlescriptCurrInstr);
                        let gl_i101 = gActiveBattler.get();
                        crate::gl!(gSelectionBattleScripts[gl_i101] = gl_v)
                    };
                }
            }
            STATE_WAIT_SET_BEFORE_ACTION => {
                if gBattleControllerExecFlags.get()
                    & (gBitTable[gActiveBattler.get()]
                        | 0xf0000000
                        | gBitTable[gActiveBattler.get()] << 4
                        | gBitTable[gActiveBattler.get()] << 8
                        | gBitTable[gActiveBattler.get()] << 12)
                    == 0
                {
                    {
                        let gl_v = STATE_BEFORE_ACTION_CHOSEN;
                        let gl_i102 = gActiveBattler.get();
                        crate::gl!(gBattleCommunication[gl_i102] = gl_v)
                    };
                }
            }
            STATE_SELECTION_SCRIPT_MAY_RUN => {
                if *(*crate::gl!(gBattleStruct))
                    .selectionScriptFinished
                    .as_mut_ptr()
                    .at(gActiveBattler.get())
                    != 0
                {
                    if {
                        let gl_i103 = gActiveBattler.get();
                        crate::gl!(gBattleBufferB[gl_i103][1])
                    } == B_ACTION_NOTHING_FAINTED
                    {
                        gHitMarker.set(gHitMarker.get() | (HITMARKER_RUN));
                        {
                            let gl_v = B_ACTION_RUN;
                            let gl_i104 = gActiveBattler.get();
                            crate::gl!(gChosenActionByBattler[gl_i104] = gl_v)
                        };
                        {
                            let gl_v = STATE_WAIT_ACTION_CONFIRMED_STANDBY;
                            let gl_i105 = gActiveBattler.get();
                            crate::gl!(gBattleCommunication[gl_i105] = gl_v)
                        };
                    } else {
                        RecordedBattle_ClearBattlerAction(gActiveBattler.get(), 1);
                        {
                            let gl_v = *(*crate::gl!(gBattleStruct))
                                .stateIdAfterSelScript
                                .as_mut_ptr()
                                .at(gActiveBattler.get());
                            let gl_i106 = gActiveBattler.get();
                            crate::gl!(gBattleCommunication[gl_i106] = gl_v)
                        };
                    }
                } else {
                    gBattlerAttacker.set(gActiveBattler.get());
                    crate::gl!(
                        gBattlescriptCurrInstr = {
                            let gl_i107 = gActiveBattler.get();
                            crate::gl!(gSelectionBattleScripts[gl_i107])
                        }
                    );
                    if gBattleControllerExecFlags.get()
                        & (gBitTable[gActiveBattler.get()]
                            | 0xf0000000
                            | gBitTable[gActiveBattler.get()] << 4
                            | gBitTable[gActiveBattler.get()] << 8
                            | gBitTable[gActiveBattler.get()] << 12)
                        == 0
                    {
                        (*(&raw const crate::data::battle_script_commands::gBattleScriptingCommandsTable).cast::<CArray<Option<unsafe fn()>, 0>>())[*crate::gl!(gBattlescriptCurrInstr)].unwrap_unchecked()();
                    }
                    {
                        let gl_v = crate::gl!(gBattlescriptCurrInstr);
                        let gl_i108 = gActiveBattler.get();
                        crate::gl!(gSelectionBattleScripts[gl_i108] = gl_v)
                    };
                }
            }
            _ => {}
        }
        gActiveBattler.set(gActiveBattler.get() + (1));
    }
    if crate::gl!(gBattleCommunication[4]) == gBattlersCount.get() {
        RecordedBattle_CheckMovesetChanges(B_RECORD_MODE_RECORDING);
        crate::gl!(gBattleMainFunc = Some(SetActionsAndBattlersTurnOrder));
        if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0 {
            i = 0;
            while i < gBattlersCount.get() as i32 {
                if crate::gl!(gChosenActionByBattler[i]) == B_ACTION_SWITCH {
                    SwitchPartyOrderInGameMulti(
                        i as u8,
                        *(*crate::gl!(gBattleStruct))
                            .monToSwitchIntoId
                            .as_mut_ptr()
                            .at(i),
                    );
                }
                i += 1;
            }
        }
    }
}
fn AllAtActionConfirmed() -> u8 {
    let mut count: i32 = 0;
    for i in 0..(gBattlersCount.get() as i32) {
        if crate::gl!(gBattleCommunication[i]) == STATE_WAIT_ACTION_CONFIRMED {
            count += 1;
        }
    }
    if count + 1 == gBattlersCount.get() as i32 {
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
    *(*crate::gl!(gBattleStruct))
        .monToSwitchIntoId
        .as_mut_ptr()
        .at(gActiveBattler.get()) = {
        let gl_i109 = gActiveBattler.get();
        crate::gl!(gBattleBufferB[gl_i109][1])
    };
    RecordedBattle_SetBattlerAction(gActiveBattler.get(), {
        let gl_i110 = gActiveBattler.get();
        crate::gl!(gBattleBufferB[gl_i110][1])
    });
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
    {
        *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
            .at(gActiveBattler.get() as i32 * 3) &= 0xF;
        *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
            .at(gActiveBattler.get() as i32 * 3) |= {
            let gl_i111 = gActiveBattler.get();
            crate::gl!(gBattleBufferB[gl_i111][2])
        } & 0xF0;
        *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
            .at(gActiveBattler.get() as i32 * 3)
            .at(1) = {
            let gl_i112 = gActiveBattler.get();
            crate::gl!(gBattleBufferB[gl_i112][3])
        };
        *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
            .at((gActiveBattler.get() as i32 ^ 2) * 3) &= 0xF0;
        *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
            .at((gActiveBattler.get() as i32 ^ 2) * 3) |= (({
            let gl_i113 = gActiveBattler.get();
            crate::gl!(gBattleBufferB[gl_i113][2])
        } as i32
            & 0xF0)
            >> 4) as u8;
        *((*crate::gl!(gBattleStruct)).battlerPartyOrders.as_mut_ptr() as *mut u8)
            .at((gActiveBattler.get() as i32 ^ 2) * 3)
            .at(2) = {
            let gl_i114 = gActiveBattler.get();
            crate::gl!(gBattleBufferB[gl_i114][3])
        };
    }
}
pub fn SwapTurnOrder(id1: u8, id2: u8) {
    let mut temp: u32 = crate::gl!(gActionsByTurnOrder[id1]) as u32;
    crate::gl!(gActionsByTurnOrder[id1] = crate::gl!(gActionsByTurnOrder[id2]));
    crate::gl!(gActionsByTurnOrder[id2] = temp as u8);
    temp = crate::gl!(gBattlerByTurnOrder[id1]) as u32;
    crate::gl!(gBattlerByTurnOrder[id1] = crate::gl!(gBattlerByTurnOrder[id2]));
    crate::gl!(gBattlerByTurnOrder[id2] = temp as u8);
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
        if crate::gl!(gBattleMons[battler1].ability) == ABILITY_SWIFT_SWIM
            && gBattleWeather.get() as i32 & B_WEATHER_RAIN != 0
            || crate::gl!(gBattleMons[battler1].ability) == ABILITY_CHLOROPHYLL
                && gBattleWeather.get() as i32 & B_WEATHER_SUN != 0
        {
            speedMultiplierBattler1 = 2;
        } else {
            speedMultiplierBattler1 = 1;
        }
        if crate::gl!(gBattleMons[battler2].ability) == ABILITY_SWIFT_SWIM
            && gBattleWeather.get() as i32 & B_WEATHER_RAIN != 0
            || crate::gl!(gBattleMons[battler2].ability) == ABILITY_CHLOROPHYLL
                && gBattleWeather.get() as i32 & B_WEATHER_SUN != 0
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
        crate::gl!(gBattleMons[battler1].speed) as i32
            * speedMultiplierBattler1 as i32
            * (*(&raw const crate::data::pokemon::gStatStageRatios)
                .cast::<CArray<CArray<u8, 2>, 13>>())
                [crate::gl!(gBattleMons[battler1].statStages[3])][0] as i32,
        (*(&raw const crate::data::pokemon::gStatStageRatios).cast::<CArray<CArray<u8, 2>, 13>>())
            [crate::gl!(gBattleMons[battler1].statStages[3])][1] as i32,
    ) as u32;
    if crate::gl!(gBattleMons[battler1].item) == ITEM_ENIGMA_BERRY {
        holdEffect = crate::gl!(gEnigmaBerries[battler1].holdEffect);
        holdEffectParam = crate::gl!(gEnigmaBerries[battler1].holdEffectParam);
    } else {
        holdEffect = GetItemHoldEffect(crate::gl!(gBattleMons[battler1].item));
        holdEffectParam = GetItemHoldEffectParam(crate::gl!(gBattleMons[battler1].item));
    }
    if gBattleTypeFlags.get() & 0x23f0102 == 0
        && FlagGet(FLAG_BADGE03_GET) != 0
        && GetBattlerSide(battler1) == B_SIDE_PLAYER
    {
        speedBattler1 = speedBattler1 * 110 / 100;
    }
    if holdEffect == HOLD_EFFECT_MACHO_BRACE {
        speedBattler1 /= 2;
    }
    if crate::gl!(gBattleMons[battler1].status1) & STATUS1_PARALYSIS != 0 {
        speedBattler1 /= 4;
    }
    if holdEffect == HOLD_EFFECT_QUICK_CLAW
        && (gRandomTurnNumber.get() as i32) < 0xFFFF * holdEffectParam as i32 / 100
    {
        speedBattler1 = 0xffffffff;
    }
    let mut speedBattler2: u32 = div_i32(
        crate::gl!(gBattleMons[battler2].speed) as i32
            * speedMultiplierBattler2 as i32
            * (*(&raw const crate::data::pokemon::gStatStageRatios)
                .cast::<CArray<CArray<u8, 2>, 13>>())
                [crate::gl!(gBattleMons[battler2].statStages[3])][0] as i32,
        (*(&raw const crate::data::pokemon::gStatStageRatios).cast::<CArray<CArray<u8, 2>, 13>>())
            [crate::gl!(gBattleMons[battler2].statStages[3])][1] as i32,
    ) as u32;
    if crate::gl!(gBattleMons[battler2].item) == ITEM_ENIGMA_BERRY {
        holdEffect = crate::gl!(gEnigmaBerries[battler2].holdEffect);
        holdEffectParam = crate::gl!(gEnigmaBerries[battler2].holdEffectParam);
    } else {
        holdEffect = GetItemHoldEffect(crate::gl!(gBattleMons[battler2].item));
        holdEffectParam = GetItemHoldEffectParam(crate::gl!(gBattleMons[battler2].item));
    }
    if gBattleTypeFlags.get() & 0x23f0102 == 0
        && FlagGet(FLAG_BADGE03_GET) != 0
        && GetBattlerSide(battler2) == B_SIDE_PLAYER
    {
        speedBattler2 = speedBattler2 * 110 / 100;
    }
    if holdEffect == HOLD_EFFECT_MACHO_BRACE {
        speedBattler2 /= 2;
    }
    if crate::gl!(gBattleMons[battler2].status1) & STATUS1_PARALYSIS != 0 {
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
        if crate::gl!(gChosenActionByBattler[battler1]) == B_ACTION_USE_MOVE {
            if (*gProtectStructs.as_ptr())[battler1].noValidMoves() != 0 {
                moveBattler1 = MOVE_STRUGGLE;
            } else {
                moveBattler1 =
                    (*gBattleMons.as_ptr())[battler1].moves[*(*crate::gl!(gBattleStruct))
                        .chosenMovePositions
                        .as_mut_ptr()
                        .at(battler1)];
            }
        } else {
            moveBattler1 = MOVE_NONE;
        }
        if crate::gl!(gChosenActionByBattler[battler2]) == B_ACTION_USE_MOVE {
            if (*gProtectStructs.as_ptr())[battler2].noValidMoves() != 0 {
                moveBattler2 = MOVE_STRUGGLE;
            } else {
                moveBattler2 =
                    (*gBattleMons.as_ptr())[battler2].moves[*(*crate::gl!(gBattleStruct))
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
    if gBattleTypeFlags.get() & BATTLE_TYPE_SAFARI != 0 {
        gActiveBattler.set(0);
        while gActiveBattler.get() < gBattlersCount.get() {
            crate::gl!(
                gActionsByTurnOrder[turnOrderId] = {
                    let gl_i115 = gActiveBattler.get();
                    crate::gl!(gChosenActionByBattler[gl_i115])
                }
            );
            crate::gl!(gBattlerByTurnOrder[turnOrderId] = gActiveBattler.get());
            turnOrderId += 1;
            gActiveBattler.set(gActiveBattler.get() + (1));
        }
    } else {
        if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
            gActiveBattler.set(0);
            while gActiveBattler.get() < gBattlersCount.get() {
                if {
                    let gl_i116 = gActiveBattler.get();
                    crate::gl!(gChosenActionByBattler[gl_i116])
                } == B_ACTION_RUN
                {
                    turnOrderId = 5;
                    break;
                }
                gActiveBattler.set(gActiveBattler.get() + (1));
            }
        } else {
            if crate::gl!(gChosenActionByBattler[0]) == B_ACTION_RUN {
                gActiveBattler.set(0);
                turnOrderId = 5;
            }
            if crate::gl!(gChosenActionByBattler[2]) == B_ACTION_RUN {
                gActiveBattler.set(2);
                turnOrderId = 5;
            }
        }
        if turnOrderId == 5 {
            crate::gl!(
                gActionsByTurnOrder[0] = {
                    let gl_i117 = gActiveBattler.get();
                    crate::gl!(gChosenActionByBattler[gl_i117])
                }
            );
            crate::gl!(gBattlerByTurnOrder[0] = gActiveBattler.get());
            turnOrderId = 1;
            for i in 0..(gBattlersCount.get() as i32) {
                if i != gActiveBattler.get() as i32 {
                    crate::gl!(
                        gActionsByTurnOrder[turnOrderId] = crate::gl!(gChosenActionByBattler[i])
                    );
                    crate::gl!(gBattlerByTurnOrder[turnOrderId] = i as u8);
                    turnOrderId += 1;
                }
            }
            crate::gl!(gBattleMainFunc = Some(CheckFocusPunch_ClearVarsBeforeTurnStarts));
            (*crate::gl!(gBattleStruct)).focusPunchBattlerId = 0;
            return;
        } else {
            gActiveBattler.set(0);
            while gActiveBattler.get() < gBattlersCount.get() {
                if {
                    let gl_i118 = gActiveBattler.get();
                    crate::gl!(gChosenActionByBattler[gl_i118])
                } == B_ACTION_USE_ITEM
                    || {
                        let gl_i119 = gActiveBattler.get();
                        crate::gl!(gChosenActionByBattler[gl_i119])
                    } == B_ACTION_SWITCH
                {
                    crate::gl!(
                        gActionsByTurnOrder[turnOrderId] = {
                            let gl_i120 = gActiveBattler.get();
                            crate::gl!(gChosenActionByBattler[gl_i120])
                        }
                    );
                    crate::gl!(gBattlerByTurnOrder[turnOrderId] = gActiveBattler.get());
                    turnOrderId += 1;
                }
                gActiveBattler.set(gActiveBattler.get() + (1));
            }
            gActiveBattler.set(0);
            while gActiveBattler.get() < gBattlersCount.get() {
                if {
                    let gl_i121 = gActiveBattler.get();
                    crate::gl!(gChosenActionByBattler[gl_i121])
                } != B_ACTION_USE_ITEM
                    && {
                        let gl_i122 = gActiveBattler.get();
                        crate::gl!(gChosenActionByBattler[gl_i122])
                    } != B_ACTION_SWITCH
                {
                    crate::gl!(
                        gActionsByTurnOrder[turnOrderId] = {
                            let gl_i123 = gActiveBattler.get();
                            crate::gl!(gChosenActionByBattler[gl_i123])
                        }
                    );
                    crate::gl!(gBattlerByTurnOrder[turnOrderId] = gActiveBattler.get());
                    turnOrderId += 1;
                }
                gActiveBattler.set(gActiveBattler.get() + (1));
            }
            i = 0;
            while i < gBattlersCount.get() as i32 - 1 {
                j = i + 1;
                while j < gBattlersCount.get() as i32 {
                    let battler1: u8 = crate::gl!(gBattlerByTurnOrder[i]);
                    let battler2: u8 = crate::gl!(gBattlerByTurnOrder[j]);
                    if crate::gl!(gActionsByTurnOrder[i]) != B_ACTION_USE_ITEM
                        && crate::gl!(gActionsByTurnOrder[j]) != B_ACTION_USE_ITEM
                        && crate::gl!(gActionsByTurnOrder[i]) != B_ACTION_SWITCH
                        && crate::gl!(gActionsByTurnOrder[j]) != B_ACTION_SWITCH
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
    crate::gl!(gBattleMainFunc = Some(CheckFocusPunch_ClearVarsBeforeTurnStarts));
    (*crate::gl!(gBattleStruct)).focusPunchBattlerId = 0;
}
unsafe fn TurnValuesCleanUp(var0: u8) {
    let mut dataPtr: *mut u8 = null_mut();
    gActiveBattler.set(0);
    while gActiveBattler.get() < gBattlersCount.get() {
        if var0 != 0 {
            (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_protected(0);
            (*gProtectStructs.as_ptr())[gActiveBattler.get()].set_endured(0);
        } else {
            dataPtr = {
                let gl_i124 = gActiveBattler.get();
                crate::gl!(&raw gProtectStructs[gl_i124])
            } as *mut u8;
            for i in 0..16i32 {
                *dataPtr.at(i) = 0;
            }
            if ({
                let gl_i92 = gActiveBattler.get();
                crate::gl!(gDisableStructs[gl_i92].isFirstTurn)
            }) != 0
            {
                {
                    let gl_i93 = gActiveBattler.get();
                    crate::gl!(gDisableStructs[gl_i93].isFirstTurn -= 1)
                };
            }
            if ({
                let gl_i94 = gActiveBattler.get();
                crate::gl!(gDisableStructs[gl_i94].rechargeTimer)
            }) != 0
            {
                {
                    let gl_i95 = gActiveBattler.get();
                    crate::gl!(gDisableStructs[gl_i95].rechargeTimer -= 1)
                };
                if ({
                    let gl_i96 = gActiveBattler.get();
                    crate::gl!(gDisableStructs[gl_i96].rechargeTimer)
                }) == 0
                {
                    {
                        let gl_i97 = gActiveBattler.get();
                        crate::gl!(gBattleMons[gl_i97].status2 &= 0xffbfffff)
                    };
                }
            }
        }
        if ({
            let gl_i98 = gActiveBattler.get();
            crate::gl!(gDisableStructs[gl_i98].substituteHP)
        }) == 0
        {
            {
                let gl_i99 = gActiveBattler.get();
                crate::gl!(gBattleMons[gl_i99].status2 &= 0xfeffffff)
            };
        }
        gActiveBattler.set(gActiveBattler.get() + (1));
    }
    crate::gl!(gSideTimers[0].followmeTimer = 0);
    crate::gl!(gSideTimers[1].followmeTimer = 0);
}
pub unsafe fn SpecialStatusesClear() {
    gActiveBattler.set(0);
    while gActiveBattler.get() < gBattlersCount.get() {
        let dataPtr: *mut u8 = {
            let gl_i125 = gActiveBattler.get();
            crate::gl!(&raw gSpecialStatuses[gl_i125])
        } as *mut u8;
        for i in 0..20i32 {
            *dataPtr.at(i) = 0;
        }
        gActiveBattler.set(gActiveBattler.get() + (1));
    }
}
pub(crate) unsafe fn CheckFocusPunch_ClearVarsBeforeTurnStarts() {
    if gHitMarker.get() & HITMARKER_RUN == 0 {
        while (*crate::gl!(gBattleStruct)).focusPunchBattlerId < gBattlersCount.get() {
            gActiveBattler.set({
                gBattlerAttacker.set((*crate::gl!(gBattleStruct)).focusPunchBattlerId);
                gBattlerAttacker.get()
            });
            (*crate::gl!(gBattleStruct)).focusPunchBattlerId += 1;
            if {
                let gl_i126 = gActiveBattler.get();
                crate::gl!(gChosenMoveByBattler[gl_i126])
            } == MOVE_FOCUS_PUNCH
                && ({
                    let gl_i100 = gActiveBattler.get();
                    crate::gl!(gBattleMons[gl_i100].status1)
                }) & STATUS1_SLEEP
                    == 0
                && (*gDisableStructs.as_ptr())[gBattlerAttacker.get()].truantCounter() == 0
                && (*gProtectStructs.as_ptr())[gActiveBattler.get()].noValidMoves() == 0
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
    gCurrentTurnActionNumber.set(0);
    gCurrentActionFuncId.set({
        let gl_i127 = gCurrentTurnActionNumber.get();
        crate::gl!(gActionsByTurnOrder[gl_i127])
    });
    gDynamicBasePower.set(0);
    (*crate::gl!(gBattleStruct)).dynamicMoveType = 0;
    crate::gl!(gBattleMainFunc = Some(RunTurnActionsFunctions));
    crate::gl!(gBattleCommunication[3] = 0);
    crate::gl!(gBattleCommunication[4] = 0);
    crate::gl!(gBattleScripting.multihitMoveEffect = 0);
    (*(*crate::gl!(gBattleResources)).battleScriptsStack).size = 0;
}
pub(crate) unsafe fn RunTurnActionsFunctions() {
    if gBattleOutcome.get() != 0 {
        gCurrentActionFuncId.set(B_ACTION_FINISHED);
    }
    (*crate::gl!(gBattleStruct)).savedTurnActionNumber = gCurrentTurnActionNumber.get();
    sTurnActionsFuncsTable[gCurrentActionFuncId.get()].unwrap_unchecked()();
    if gCurrentTurnActionNumber.get() >= gBattlersCount.get() {
        gHitMarker.set(gHitMarker.get() & (0xffefffff));
        crate::gl!(gBattleMainFunc = sEndTurnFuncsTable[gBattleOutcome.get() as i32 & 0x7F]);
    } else {
        if (*crate::gl!(gBattleStruct)).savedTurnActionNumber != gCurrentTurnActionNumber.get() {
            gHitMarker.set(gHitMarker.get() & (0xfffffdff));
            gHitMarker.set(gHitMarker.get() & (0xfff7ffff));
        }
    }
}
pub(crate) unsafe fn HandleEndTurn_BattleWon() {
    gCurrentActionFuncId.set(0);
    if gBattleTypeFlags.get() & 0x2000002 != 0 {
        gSpecialVar_Result = gBattleOutcome.get() as u16;
        crate::gl!(gBattleTextBuff1[0] = gBattleOutcome.get());
        gBattlerAttacker.set(GetBattlerAtPosition(B_POSITION_PLAYER_LEFT));
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_LinkBattleWonOrLost
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        gBattleOutcome.set(gBattleOutcome.get() & (127));
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0
        && gBattleTypeFlags.get() & 0x43f0900 != 0
    {
        BattleStopLowHpSound();
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_FrontierTrainerBattleWon
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        if gTrainerBattleOpponent_A.get() == TRAINER_FRONTIER_BRAIN {
            PlayBGM(MUS_VICTORY_GYM_LEADER);
        } else {
            PlayBGM(MUS_VICTORY_TRAINER);
        }
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_LINK == 0
    {
        BattleStopLowHpSound();
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_LocalTrainerBattleWon
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        match (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
            [gTrainerBattleOpponent_A.get()]
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
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PayDayMoneyAndPickUpItems
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    }
    crate::gl!(gBattleMainFunc = Some(HandleEndTurn_FinishBattle));
}
pub(crate) unsafe fn HandleEndTurn_BattleLost() {
    gCurrentActionFuncId.set(0);
    if gBattleTypeFlags.get() & 0x2000002 != 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_FRONTIER != 0 {
            if gBattleOutcome.get() as i32 & B_OUTCOME_LINK_BATTLE_RAN != 0 {
                crate::gl!(
                    gBattlescriptCurrInstr =
                        (*crate::asmdata::BattleScript_PrintPlayerForfeitedLinkBattle
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut()
                );
                gBattleOutcome.set(gBattleOutcome.get() & (127));
                (*crate::gl!(gSaveBlock2Ptr))
                    .frontier
                    .set_disableRecordBattle(TRUE);
            } else {
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_FrontierLinkBattleLost
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
                gBattleOutcome.set(gBattleOutcome.get() & (127));
            }
        } else {
            crate::gl!(gBattleTextBuff1[0] = gBattleOutcome.get());
            gBattlerAttacker.set(GetBattlerAtPosition(B_POSITION_PLAYER_LEFT));
            crate::gl!(
                gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_LinkBattleWonOrLost
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            );
            gBattleOutcome.set(gBattleOutcome.get() & (127));
        }
    } else {
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_LocalBattleLost
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
    }
    crate::gl!(gBattleMainFunc = Some(HandleEndTurn_FinishBattle));
}
pub(crate) unsafe fn HandleEndTurn_RanFromBattle() {
    gCurrentActionFuncId.set(0);
    if gBattleTypeFlags.get() & BATTLE_TYPE_FRONTIER != 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0
    {
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PrintPlayerForfeited
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        gBattleOutcome.set(B_OUTCOME_FORFEITED);
        (*crate::gl!(gSaveBlock2Ptr))
            .frontier
            .set_disableRecordBattle(TRUE);
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER_HILL != 0 {
        crate::gl!(
            gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_PrintPlayerForfeited
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        );
        gBattleOutcome.set(B_OUTCOME_FORFEITED);
    } else {
        match (*gProtectStructs.as_ptr())[gBattlerAttacker.get()].fleeType() {
            FLEE_ITEM => {
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_SmokeBallEscape
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
            }
            FLEE_ABILITY => {
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_RanAwayUsingMonAbility
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
            }
            _ => {
                crate::gl!(
                    gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_GotAwaySafely
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                );
            }
        }
    }
    crate::gl!(gBattleMainFunc = Some(HandleEndTurn_FinishBattle));
}
pub(crate) unsafe fn HandleEndTurn_MonFled() {
    gCurrentActionFuncId.set(0);
    crate::gl!(gBattleTextBuff1[0] = 0xFD);
    crate::gl!(gBattleTextBuff1[1] = 7);
    crate::gl!(gBattleTextBuff1[2] = gBattlerAttacker.get());
    crate::gl!(
        gBattleTextBuff1[3] = ({
            let gl_i101 = gBattlerAttacker.get();
            crate::gl!(gBattlerPartyIndexes[gl_i101])
        }) as u8
    );
    crate::gl!(gBattleTextBuff1[4] = 0xFF);
    crate::gl!(
        gBattlescriptCurrInstr = (*crate::asmdata::BattleScript_WildMonFled
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut()
    );
    crate::gl!(gBattleMainFunc = Some(HandleEndTurn_FinishBattle));
}
pub(crate) unsafe fn HandleEndTurn_FinishBattle() {
    if gCurrentActionFuncId.get() == B_ACTION_TRY_FINISH
        || gCurrentActionFuncId.get() == B_ACTION_FINISHED
    {
        if gBattleTypeFlags.get() & 0x23f0b92 == 0 {
            gActiveBattler.set(0);
            while gActiveBattler.get() < gBattlersCount.get() {
                if GetBattlerSide(gActiveBattler.get()) == B_SIDE_PLAYER {
                    if crate::gl!(gBattleResults.playerMon1Species) == SPECIES_NONE {
                        crate::gl!(
                            gBattleResults.playerMon1Species = GetMonData3(
                                &raw mut (*gPlayerParty.as_ptr())[{
                                    let gl_i102 = gActiveBattler.get();
                                    crate::gl!(gBattlerPartyIndexes[gl_i102])
                                }],
                                MON_DATA_SPECIES,
                                null_mut(),
                            ) as u16
                        );
                        GetMonData3(
                            &raw mut (*gPlayerParty.as_ptr())[{
                                let gl_i103 = gActiveBattler.get();
                                crate::gl!(gBattlerPartyIndexes[gl_i103])
                            }],
                            MON_DATA_NICKNAME,
                            (*gBattleResults.as_ptr()).playerMon1Name.as_mut_ptr(),
                        );
                    } else {
                        crate::gl!(
                            gBattleResults.playerMon2Species = GetMonData3(
                                &raw mut (*gPlayerParty.as_ptr())[{
                                    let gl_i104 = gActiveBattler.get();
                                    crate::gl!(gBattlerPartyIndexes[gl_i104])
                                }],
                                MON_DATA_SPECIES,
                                null_mut(),
                            ) as u16
                        );
                        GetMonData3(
                            &raw mut (*gPlayerParty.as_ptr())[{
                                let gl_i105 = gActiveBattler.get();
                                crate::gl!(gBattlerPartyIndexes[gl_i105])
                            }],
                            MON_DATA_NICKNAME,
                            (*gBattleResults.as_ptr()).playerMon2Name.as_mut_ptr(),
                        );
                    }
                }
                gActiveBattler.set(gActiveBattler.get() + (1));
            }
            TryPutPokemonTodayOnAir();
        }
        if gBattleTypeFlags.get() & 0x23f0b9a == 0 && (*gBattleResults.as_ptr()).shinyWildMon() != 0
        {
            TryPutBreakingNewsOnAir();
        }
        RecordedBattle_SetPlaybackFinished();
        BeginFastPaletteFade(3);
        FadeOutMapMusic(5);
        crate::gl!(gBattleMainFunc = Some(FreeResetData_ReturnToOvOrDoEvolutions));
        crate::gl!(gCB2_AfterEvolution = Some(BattleMainCB2));
    } else {
        if gBattleControllerExecFlags.get() == 0 {
            (*(&raw const crate::data::battle_script_commands::gBattleScriptingCommandsTable)
                .cast::<CArray<Option<unsafe fn()>, 0>>())[*crate::gl!(gBattlescriptCurrInstr)]
            .unwrap_unchecked()();
        }
    }
}
pub(crate) unsafe fn FreeResetData_ReturnToOvOrDoEvolutions() {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        ResetSpriteData();
        if gLeveledUpInBattle.get() == 0 || gBattleOutcome.get() != B_OUTCOME_WON {
            crate::gl!(gBattleMainFunc = Some(ReturnFromBattleToOverworld));
            return;
        } else {
            crate::gl!(gBattleMainFunc = Some(TryEvolvePokemon));
        }
    }
    FreeAllWindowBuffers();
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK == 0 {
        FreeMonSpritesGfx();
        FreeBattleResources();
        FreeBattleSpritesData();
    }
}
pub(crate) unsafe fn TryEvolvePokemon() {
    while gLeveledUpInBattle.get() != 0 {
        for i in 0..PARTY_SIZE {
            if gLeveledUpInBattle.get() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                != 0
            {
                let mut levelUpBits: u8 = gLeveledUpInBattle.get();
                levelUpBits &= !(gBitTable[i] as u8);
                gLeveledUpInBattle.set(levelUpBits);
                let species: u16 = GetEvolutionTargetSpecies(
                    crate::gl!(&raw gPlayerParty[i]),
                    EVO_MODE_NORMAL,
                    levelUpBits as u16,
                );
                if species != SPECIES_NONE {
                    FreeAllWindowBuffers();
                    crate::gl!(gBattleMainFunc = Some(WaitForEvoSceneToFinish));
                    EvolutionScene(crate::gl!(&raw gPlayerParty[i]), species, TRUE, i as u8);
                    return;
                }
            }
        }
    }
    crate::gl!(gBattleMainFunc = Some(ReturnFromBattleToOverworld));
}
pub(crate) fn WaitForEvoSceneToFinish() {
    if crate::gl!(gMain.callback2) == Some(BattleMainCB2 as unsafe fn()) {
        crate::gl!(gBattleMainFunc = Some(TryEvolvePokemon));
    }
}
pub(crate) unsafe fn ReturnFromBattleToOverworld() {
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK == 0 {
        RandomlyGivePartyPokerus((*gPlayerParty.as_ptr()).as_mut_ptr());
        PartySpreadPokerus((*gPlayerParty.as_ptr()).as_mut_ptr());
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 && gReceivedRemoteLinkPlayers.get() != 0 {
        return;
    }
    gSpecialVar_Result = gBattleOutcome.get() as u16;
    (*gMain.as_ptr()).set_inBattle(FALSE);
    crate::gl!(gMain.callback1 = crate::gl!(gPreBattleCallback1));
    if gBattleTypeFlags.get() & BATTLE_TYPE_ROAMER != 0 {
        UpdateRoamerHPStatus(crate::gl!(&raw gEnemyParty[0]));
        if gBattleOutcome.get() as i32 & B_OUTCOME_WON as i32 != 0
            || gBattleOutcome.get() == B_OUTCOME_CAUGHT
        {
            SetRoamerInactive();
        }
    }
    m4aSongNumStop(SE_LOW_HEALTH);
    SetMainCallback2(crate::gl!(gMain.savedCallback));
}
pub unsafe fn RunBattleScriptCommands_PopCallbacksStack() {
    if gCurrentActionFuncId.get() == B_ACTION_TRY_FINISH
        || gCurrentActionFuncId.get() == B_ACTION_FINISHED
    {
        if (*(*crate::gl!(gBattleResources)).battleCallbackStack).size != 0 {
            (*(*crate::gl!(gBattleResources)).battleCallbackStack).size -= 1;
        }
        crate::gl!(
            gBattleMainFunc = (*(*crate::gl!(gBattleResources)).battleCallbackStack).function
                [(*(*crate::gl!(gBattleResources)).battleCallbackStack).size]
        );
    } else {
        if gBattleControllerExecFlags.get() == 0 {
            (*(&raw const crate::data::battle_script_commands::gBattleScriptingCommandsTable)
                .cast::<CArray<Option<unsafe fn()>, 0>>())[*crate::gl!(gBattlescriptCurrInstr)]
            .unwrap_unchecked()();
        }
    }
}
pub unsafe fn RunBattleScriptCommands() {
    if gBattleControllerExecFlags.get() == 0 {
        (*(&raw const crate::data::battle_script_commands::gBattleScriptingCommandsTable)
            .cast::<CArray<Option<unsafe fn()>, 0>>())[*crate::gl!(gBattlescriptCurrInstr)]
        .unwrap_unchecked()();
    }
}
