//! Translated from `src/battle_main.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sIntroScanlineParams16Bit sIntroScanlineParams32Bit gUnusedBattleInitSprite sText_ShedinjaJpnName gOamData_BattleSpriteOpponentSide gOamData_BattleSpritePlayerSide sAnim_Unused sAnims_Unused sAffineAnim_Unused sAffineAnims_Unused sCenterToCornerVecXs gTypeEffectiveness gTypeNames gTrainerMoneyTable sNoneDescription sStenchDescription sDrizzleDescription sSpeedBoostDescription sBattleArmorDescription sSturdyDescription sDampDescription sLimberDescription sSandVeilDescription sStaticDescription sVoltAbsorbDescription sWaterAbsorbDescription sObliviousDescription sCloudNineDescription sCompoundEyesDescription sInsomniaDescription sColorChangeDescription sImmunityDescription sFlashFireDescription sShieldDustDescription sOwnTempoDescription sSuctionCupsDescription sIntimidateDescription sShadowTagDescription sRoughSkinDescription sWonderGuardDescription sLevitateDescription sEffectSporeDescription sSynchronizeDescription sClearBodyDescription sNaturalCureDescription sLightningRodDescription sSereneGraceDescription sSwiftSwimDescription sChlorophyllDescription sIlluminateDescription sTraceDescription sHugePowerDescription sPoisonPointDescription sInnerFocusDescription sMagmaArmorDescription sWaterVeilDescription sMagnetPullDescription sSoundproofDescription sRainDishDescription sSandStreamDescription sPressureDescription sThickFatDescription sEarlyBirdDescription sFlameBodyDescription sRunAwayDescription sKeenEyeDescription sHyperCutterDescription sPickupDescription sTruantDescription sHustleDescription sCuteCharmDescription sPlusDescription sMinusDescription sForecastDescription sStickyHoldDescription sShedSkinDescription sGutsDescription sMarvelScaleDescription sLiquidOozeDescription sOvergrowDescription sBlazeDescription sTorrentDescription sSwarmDescription sRockHeadDescription sDroughtDescription sArenaTrapDescription sVitalSpiritDescription sWhiteSmokeDescription sPurePowerDescription sShellArmorDescription sCacophonyDescription sAirLockDescription gAbilityNames gAbilityDescriptionPointers sTurnActionsFuncsTable sEndTurnFuncsTable gStatusConditionString_PoisonJpn gStatusConditionString_SleepJpn gStatusConditionString_ParalysisJpn gStatusConditionString_BurnJpn gStatusConditionString_IceJpn gStatusConditionString_ConfusionJpn gStatusConditionString_LoveJpn gStatusConditionStringsTable
#[allow(unused_imports)]
use crate::data::battle_main::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG0_X: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG0_Y: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG1_X: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG1_Y: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG2_X: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG2_Y: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG3_X: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_BG3_Y: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_WIN0H: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_WIN0V: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_WIN1H: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattle_WIN1V: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gDisplayedStringBattle: crate::ffi::Align4<[u8; 300]> = crate::ffi::Align4([0; 300]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleTextBuff1: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleTextBuff2: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleTextBuff3: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFlickerArray: crate::ffi::Align4<[u8; 100]> = crate::ffi::Align4([0; 100]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleTypeFlags: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleEnvironment: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gUnusedFirstBattleVar1: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMultiPartnerParty: crate::ffi::Align4<[u8; 96]> = crate::ffi::Align4([0; 96]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMultiPartnerPartyBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimBgTileBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleAnimBgTilemapBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleBufferA: crate::ffi::Align4<[u8; 2048]> = crate::ffi::Align4([0; 2048]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleBufferB: crate::ffi::Align4<[u8; 2048]> = crate::ffi::Align4([0; 2048]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gActiveBattler: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleControllerExecFlags: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlersCount: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerPartyIndexes: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerPositions: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gActionsByTurnOrder: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerByTurnOrder: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurrentTurnActionNumber: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurrentActionFuncId: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleMons: crate::ffi::Align4<[u8; 352]> = crate::ffi::Align4([0; 352]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerSpriteIds: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurrMovePos: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gChosenMovePos: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurrentMove: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gChosenMove: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCalledMove: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleMoveDamage: i32 = 0i32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gHpDealt: i32 = 0i32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBideDmg: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastUsedItem: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastUsedAbility: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerAttacker: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerTarget: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerFainted: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gEffectBattler: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPotentialItemEffectBattler: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gAbsentBattlerFlags: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCritMultiplier: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMultiHitCounter: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlescriptCurrInstr: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gUnusedBattleMainVar: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gChosenActionByBattler: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSelectionBattleScripts: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPalaceSelectionBattleScripts: crate::ffi::Align4<[u8; 16]> =
    crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastPrintedMoves: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastMoves: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastLandedMoves: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastHitByType: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastResultingMoves: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLockedMoves: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastHitBy: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gChosenMoveByBattler: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMoveResultFlags: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gHitMarker: u32 = 0u32;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnusedBattlersArray: crate::ffi::Align4<[u8; 4]> =
    crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBideTarget: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gUnusedFirstBattleVar2: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSideStatuses: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSideTimers: crate::ffi::Align4<[u8; 24]> = crate::ffi::Align4([0; 24]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gStatuses3: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gDisableStructs: crate::ffi::Align4<[u8; 112]> = crate::ffi::Align4([0; 112]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPauseCounterBattle: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPaydayMoney: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gRandomTurnNumber: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleCommunication: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleOutcome: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gProtectStructs: crate::ffi::Align4<[u8; 64]> = crate::ffi::Align4([0; 64]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSpecialStatuses: crate::ffi::Align4<[u8; 80]> = crate::ffi::Align4([0; 80]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleWeather: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gWishFutureKnock: crate::ffi::Align4<[u8; 44]> = crate::ffi::Align4([0; 44]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gIntroSlideFlags: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSentPokesToOpponent: crate::ffi::Align4<[u8; 2]> = crate::ffi::Align4([0; 2]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gDynamicBasePower: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gExpShareExp: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gEnigmaBerries: crate::ffi::Align4<[u8; 112]> = crate::ffi::Align4([0; 112]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleScripting: crate::ffi::Align4<[u8; 40]> = crate::ffi::Align4([0; 40]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleStruct: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkBattleSendBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkBattleRecvBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleResources: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gActionSelectionCursor: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMoveSelectionCursor: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerStatusSummaryTaskId: crate::ffi::Align4<[u8; 4]> =
    crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlerInMenuId: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gDoingBattleAnim: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTransformedPersonalities: crate::ffi::Align4<[u8; 16]> =
    crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerDpadHoldFrames: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleSpritesDataPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMonSpritesGfxPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleControllerOpponentHealthboxData: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleControllerOpponentFlankHealthboxData: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleMovePower: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMoveToLearn: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleMonForms: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPreBattleCallback1: Option<unsafe extern "C" fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBattleMainFunc: Option<unsafe extern "C" fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBattleResults: crate::ffi::Align4<[u8; 68]> = crate::ffi::Align4([0; 68]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLeveledUpInBattle: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBattlerControllerFuncs: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gHealthboxSpriteIds: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMultiUsePlayerCursor: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gNumberOfMovesToChoose: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBattleControllerData: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);

unsafe extern "C" {
    static mut BattleFrontier_BattleTowerBattleRoom_Text_RecordCouldntBeSaved: u8;
    static mut BattleScript_ActionSelectionItemsCantBeUsed: u8;
    static mut BattleScript_ArenaTurnBeginning: u8;
    static mut BattleScript_AskIfWantsToForfeitMatch: u8;
    static mut BattleScript_FocusPunchSetUp: u8;
    static mut BattleScript_FrontierLinkBattleLost: u8;
    static mut BattleScript_FrontierTrainerBattleWon: u8;
    static mut BattleScript_GotAwaySafely: u8;
    static mut BattleScript_LinkBattleWonOrLost: u8;
    static mut BattleScript_LocalBattleLost: u8;
    static mut BattleScript_LocalTrainerBattleWon: u8;
    static mut BattleScript_PalacePrintFlavorText: u8;
    static mut BattleScript_PayDayMoneyAndPickUpItems: u8;
    static mut BattleScript_PrintCantEscapeFromBattle: u8;
    static mut BattleScript_PrintCantRunFromTrainer: u8;
    static mut BattleScript_PrintFullBox: u8;
    static mut BattleScript_PrintPlayerForfeited: u8;
    static mut BattleScript_PrintPlayerForfeitedLinkBattle: u8;
    static mut BattleScript_RanAwayUsingMonAbility: u8;
    static mut BattleScript_SmokeBallEscape: u8;
    static mut BattleScript_WildMonFled: u8;
    static mut gBattleBgTemplates: u8;
    static mut gBattleMoves: u8;
    static mut gBattlePalaceMoveSelectionRngValue: u8;
    static mut gBattlePartyCurrentOrder: u8;
    static mut gBattleScriptingCommandsTable: u8;
    static mut gBattleTextboxPalette: u8;
    static mut gBattleWindowTemplates: u8;
    static mut gBitTable: u8;
    static mut gBlockRecvBuffer: u8;
    static mut gCB2_AfterEvolution: u8;
    static mut gCastformFrontSpriteCoords: u8;
    static mut gDecompressionBuffer: u8;
    static mut gEnemyParty: u8;
    static mut gLinkPlayers: u8;
    static mut gMPlayInfo_SE1: u8;
    static mut gMPlayInfo_SE2: u8;
    static mut gMain: u8;
    static mut gMonFrontPicCoords: u8;
    static mut gPaletteFade: u8;
    static mut gPartnerTrainerId: u8;
    static mut gPlayerParty: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecordedBattleRngSeed: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gRngValue: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gScanlineEffect: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesInfo: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gStatStageRatios: u8;
    static mut gTasks: u8;
    static mut gText_BattleRecordedOnPass: u8;
    static mut gText_BattleYesNoChoice: u8;
    static mut gText_EmptyString3: u8;
    static mut gText_LinkStandby3: u8;
    static mut gText_RecordBattleToPass: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainerBattleOpponent_B: u8;
    static mut gTrainers: u8;
    static mut gWirelessCommType: u8;
    fn AbilityBattleEffects(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn AdjustFriendship(a0: *mut u8, a1: u8);
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocateBattleResources();
    fn AllocateBattleSpritesData();
    fn AllocateMonSpritesGfx();
    fn AnimateSprites();
    fn AreAllMovesUnusable() -> u8;
    fn BattleAnimateBackSprite(a0: *mut u8, a1: u16);
    fn BattleAnimateFrontSprite(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn BattleArena_InitPoints();
    fn BattleCreateYesNoCursorAt(a0: u8);
    fn BattleDestroyYesNoCursorAt(a0: u8);
    fn BattleInitAllSprites(a0: *mut u8, a1: *mut u8) -> u8;
    fn BattlePutTextOnWindow(a0: *mut u8, a1: u8);
    fn BattleScriptExecute(a0: *mut u8);
    fn BattleSetup_GetEnvironmentId() -> u8;
    fn BattleStopLowHpSound();
    fn BattleStringExpandPlaceholdersToDisplayedString(a0: *mut u8) -> u32;
    fn BeginFastPaletteFade(a0: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BitmaskAllOtherLinkPlayers() -> u8;
    fn BtlController_EmitChooseAction(a0: u8, a1: u8, a2: u16);
    fn BtlController_EmitChooseItem(a0: u8, a1: *mut u8);
    fn BtlController_EmitChooseMove(a0: u8, a1: u8, a2: u8, a3: *mut u8);
    fn BtlController_EmitChoosePokemon(a0: u8, a1: u8, a2: u8, a3: u8, a4: *mut u8);
    fn BtlController_EmitDrawPartyStatusSummary(a0: u8, a1: *mut u8, a2: u8);
    fn BtlController_EmitDrawTrainerPic(a0: u8);
    fn BtlController_EmitEndBounceEffect(a0: u8);
    fn BtlController_EmitGetMonData(a0: u8, a1: u8, a2: u8);
    fn BtlController_EmitIntroSlide(a0: u8, a1: u8);
    fn BtlController_EmitIntroTrainerBallThrow(a0: u8);
    fn BtlController_EmitLinkStandbyMsg(a0: u8, a1: u8, a2: u32);
    fn BtlController_EmitLoadMonSprite(a0: u8);
    fn BtlController_EmitSwitchInAnim(a0: u8, a1: u8, a2: u8);
    fn BuildOamBuffer();
    fn CalculatePPWithBonus(a0: u16, a1: u8, a2: u8) -> u8;
    fn CancelMultiTurnMoves(a0: u8);
    fn ClearBattlerAbilityHistory(a0: u8);
    fn ClearBattlerMoveHistory(a0: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut u8)>) -> u8;
    fn CreateMon(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DestroySprite(a0: *mut u8);
    fn DoBattlerEndTurnEffects() -> u8;
    fn DoFieldEndTurnEffects() -> u8;
    fn DrawBattleEntryBackground();
    fn EvolutionScene(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn FadeOutMapMusic(a0: u8);
    fn FillAroundBattleWindows();
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeBattleResources();
    fn FreeBattleSpritesData();
    fn FreeMonSpritesGfx();
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetAbilityBySpecies(a0: u16, a1: u8) -> u8;
    fn GetBattleSceneInRecordedBattle() -> u8;
    fn GetBattleTowerTrainerLanguage(a0: *mut u8, a1: u16);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBerryInfo(a0: u8) -> *mut u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetEvolutionTargetSpecies(a0: *mut u8, a1: u8, a2: u16) -> u16;
    fn GetFrontierTrainerName(a0: *mut u8, a1: u16);
    fn GetItemHoldEffect(a0: u16) -> u8;
    fn GetItemHoldEffectParam(a0: u16) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut u8) -> u8;
    fn GetMultiplayerId() -> u8;
    fn GetPartyIdFromBattlePartyId(a0: u8) -> u8;
    fn HandleBattleWindow(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn HandleFaintedMonActions() -> u8;
    fn HandleLinkBattleSetup();
    fn HandleSetPokedexFlag(a0: u16, a1: u8, a2: u32);
    fn HandleWishPerishSongOnTurnEnd() -> u8;
    fn HasTwoFramesAnimation(a0: u16) -> u8;
    fn InitBattleBgsVideo();
    fn InitBattleControllers();
    fn InitLinkBattleVsScreen(a0: u8);
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsEnigmaBerryValid() -> u32;
    fn IsLinkRfuTaskFinished() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn IsMonShiny(a0: *mut u8) -> u8;
    fn IsPlayerPartyAndPokemonStorageFull() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn ItemBattleEffects(a0: u8, a1: u8, a2: u8) -> u8;
    fn ItemIdToBerryType(a0: u16) -> u8;
    fn LoadBattleMenuWindowGfx();
    fn LoadBattleTextboxAndBackground();
    fn LoadChosenBattleElement(a0: u8) -> u8;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadOam();
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn MarkBattlerForControllerExec(a0: u8);
    fn MoveRecordedBattleToSaveData() -> u32;
    fn MoveSaveBlocks_ResetHeap();
    fn PadNameString(a0: *mut u8, a1: u8);
    fn PartySpreadPokerus(a0: *mut u8);
    fn PlayBGM(a0: u16);
    fn PlaySE(a0: u16);
    fn PrepareStringBattle(a0: u16, a1: u8);
    fn ProcessSpriteCopyRequests();
    fn Random() -> u16;
    fn RandomlyGivePartyPokerus(a0: *mut u8);
    fn RecordedBattle_CanStopPlayback() -> u8;
    fn RecordedBattle_CheckMovesetChanges(a0: u8);
    fn RecordedBattle_ClearBattlerAction(a0: u8, a1: u8);
    fn RecordedBattle_ClearFrontierPassFlag();
    fn RecordedBattle_CopyBattlerMoves();
    fn RecordedBattle_GetFrontierPassFlag() -> u8;
    fn RecordedBattle_SetBattlerAction(a0: u8, a1: u8);
    fn RecordedBattle_SetFrontierPassFlagFromHword(a0: u16);
    fn RecordedBattle_SetPlaybackFinished();
    fn RecordedBattle_SetTrainerInfo();
    fn ResetBlockReceivedFlags();
    fn ResetPaletteFade();
    fn ResetPaletteFadeControl();
    fn ResetSentPokesToOpponentValue();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn RunTextPrinters();
    fn ScanlineEffect_Clear();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
    fn SetCloseLinkCallback();
    fn SetDeoxysStats();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetHealthboxSpriteVisible(a0: u8);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetRoamerInactive();
    fn SetUpBattleVarsAndBirchZigzagoon();
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWildMonHeldItem();
    fn ShowBg(a0: u8);
    fn ShowPartyMenuToShowcaseMultiBattleParty();
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn StartHealthboxSlideIn(a0: u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut u8, a1: u8);
    fn StopCryAndClearCrySongs();
    fn StringCompareWithoutExtCtrlCodes(a0: *mut u8, a1: *mut u8) -> i32;
    fn StripExtCtrlCodes(a0: *mut u8);
    fn SwitchPartyMonSlots(a0: u8, a1: u8);
    fn SwitchPartyOrderInGameMulti(a0: u8, a1: u8);
    fn Task_ReconnectWithLinkPlayers(a0: u8);
    fn TransferPlttBuffer();
    fn TryClearRageStatuses();
    fn TryPutBreakingNewsOnAir();
    fn TryPutPokemonTodayOnAir();
    fn TrySetCantSelectMoveBattleScript() -> u8;
    fn TrySetLinkBattleTowerEnemyPartyLevel();
    fn UpdatePaletteFade() -> u8;
    fn UpdateRoamerHPStatus(a0: *mut u8);
    fn ZeroEnemyPartyMons();
    fn m4aMPlayStop(a0: *mut u8);
    fn m4aSongNumStop(a0: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitBattle() {
    unsafe {
        MoveSaveBlocks_ResetHeap();
        AllocateBattleResources();
        AllocateBattleSpritesData();
        AllocateMonSpritesGfx();
        RecordedBattle_ClearFrontierPassFlag();
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 64u32) != 0 {
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 16777216u32) != 0
            {
                CB2_InitBattleInternal();
            } else {
                if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 4194304u32)
                    != 0)
                {
                    HandleLinkBattleSetup();
                    SetMainCallback2(Some(CB2_PreInitMultiBattle));
                } else {
                    SetMainCallback2(Some(CB2_PreInitIngamePlayerPartnerBattle));
                }
            }
            (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(0u8);
        } else {
            CB2_InitBattleInternal();
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_InitBattleInternal() {
    unsafe {
        let mut i: i32 = 0i32;
        SetHBlankCallback(None);
        SetVBlankCallback(None);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((100663296i32) as usize as *mut u8),
                                ((83886080i32
                                    | (crate::c::div_i32(98304i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        SetGpuReg(76u8, 0u16);
        SetGpuReg(64u8, 240u16);
        SetGpuReg(
            68u8,
            (((crate::c::div_i32(160i32, 2i32) << 8)
                | (crate::c::div_i32(160i32, 2i32)).wrapping_add(1i32)) as u16),
        );
        SetGpuReg(72u8, 0u16);
        SetGpuReg(74u8, 0u16);
        ((&raw mut gBattle_WIN0H).cast::<u8>().cast::<u16>()).write(240u16);
        if ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 4194304u32) != 0)
            && (((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32) != 3075i32)
        {
            ((&raw mut gBattle_WIN0V).cast::<u8>().cast::<u16>()).write(159u16);
            ((&raw mut gBattle_WIN1H).cast::<u8>().cast::<u16>()).write(240u16);
            ((&raw mut gBattle_WIN1V).cast::<u8>().cast::<u16>()).write(32u16);
        } else {
            ((&raw mut gBattle_WIN0V).cast::<u8>().cast::<u16>()).write(
                (((crate::c::div_i32(160i32, 2i32) << 8)
                    | (crate::c::div_i32(160i32, 2i32)).wrapping_add(1i32))
                    as u16),
            );
            ScanlineEffect_Clear();
            {
                i = 0i32;
                'l5: loop {
                    if !(i < crate::c::div_i32(160i32, 2i32)) {
                        break 'l5;
                    }
                    'l6: {
                        ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .wrapping_offset((i) as isize))
                        .write(240u16);
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write(240u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                'l7: loop {
                    if !(i < 160i32) {
                        break 'l7;
                    }
                    'l8: {
                        ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .wrapping_offset((i) as isize))
                        .write(65296u16);
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write(65296u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ScanlineEffect_SetParams(
                (&raw const sIntroScanlineParams16Bit)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<12>>()
                    .read_unaligned(),
            );
        }
        ResetPaletteFade();
        ((&raw mut gBattle_BG0_X).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_X).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_X).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_Y).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG3_X).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG3_Y).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattleEnvironment).cast::<u8>().cast::<u8>())
            .write(BattleSetup_GetEnvironmentId());
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 16777216u32) != 0 {
            ((&raw mut gBattleEnvironment).cast::<u8>().cast::<u8>()).write(8u8);
        }
        InitBattleBgsVideo();
        LoadBattleTextboxAndBackground();
        ResetSpriteData();
        ResetTasks();
        DrawBattleEntryBackground();
        FreeAllSpritePalettes();
        ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(4u8);
        SetVBlankCallback(Some(VBlankCB_Battle));
        SetUpBattleVarsAndBirchZigzagoon();
        if ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 64u32) != 0)
            && ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 256u32) != 0)
        {
            SetMainCallback2(Some(CB2_HandleStartMultiPartnerBattle));
        } else {
            if ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 64u32) != 0)
                && ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 4194304u32)
                    != 0)
            {
                SetMainCallback2(Some(CB2_HandleStartMultiPartnerBattle));
            } else {
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 64u32) != 0 {
                    SetMainCallback2(Some(CB2_HandleStartMultiBattle));
                } else {
                    SetMainCallback2(Some(CB2_HandleStartBattle));
                }
            }
        }
        if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 16777218u32) != 0) {
            CreateNPCTrainerParty(
                (&raw mut gEnemyParty).cast::<u8>(),
                ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                1u8,
            );
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 32768u32) != 0 {
                CreateNPCTrainerParty(
                    ((&raw mut gEnemyParty).cast::<u8>())
                        .wrapping_offset((crate::c::div_i32(6i32, 2i32)) as isize * 100),
                    ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
                    0u8,
                );
            }
            SetWildMonHeldItem();
        }
        crate::c::bf_write(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            (1u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            3,
            1,
            (0u8) as i32,
        );
        {
            i = 0i32;
            'l9: loop {
                if !(i < 6i32) {
                    break 'l9;
                }
                'l10: {
                    AdjustFriendship(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        3u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn BufferPartyVsScreenHealth_AtStart() {
    unsafe {
        let mut flags: u16 = 0u16;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut species: u16 = ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        65i32,
                    )) as u16);
                    let mut hp: u16 = ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        57i32,
                    )) as u16);
                    let mut status: u32 = GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        55i32,
                    );
                    if ((species) as i32) == 0i32 {
                        break 'l2;
                    }
                    if ((((species) as i32) != 412i32) && (((hp) as i32) != 0i32))
                        && (status == 0u32)
                    {
                        flags = ((((flags) as i32)
                            | crate::c::shl_i32(1i32, (((i).wrapping_mul(2i32)) as u32)))
                            as u16);
                    }
                    if ((species) as i32) == 0i32 {
                        break 'l2;
                    }
                    if (((hp) as i32) != 0i32)
                        && ((((species) as i32) == 412i32) || (status != 0u32))
                    {
                        flags = ((((flags) as i32)
                            | crate::c::shl_i32(2i32, (((i).wrapping_mul(2i32)) as u32)))
                            as u16);
                    }
                    if ((species) as i32) == 0i32 {
                        break 'l2;
                    }
                    if (((species) as i32) != 412i32) && (((hp) as i32) == 0i32) {
                        flags = ((((flags) as i32)
                            | crate::c::shl_i32(3i32, (((i).wrapping_mul(2i32)) as u32)))
                            as u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(384))
            .wrapping_add(2))
        .write(((flags) as u8));
        (((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(384))
            .wrapping_add(3))
        .write(((((flags) as i32) >> 8) as u8));
        let __p1 = ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(384))
        .wrapping_add(3);
        (__p1).write((((((__p1).read()) as i32) | (((FlagGet(2258u16)) as i32) << 7)) as u8));
    }
}
pub(crate) unsafe extern "C" fn SetPlayerBerryDataInBattleStruct() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut battleStruct: *mut u8 =
            ((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read();
        let mut battleBerry: *mut u8 = ((battleStruct).wrapping_add(384)).wrapping_add(4);
        if IsEnigmaBerryValid() == 1u32 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((battleBerry).cast::<u8>()).wrapping_offset((i) as isize)).write(
                            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(12792))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((battleBerry).cast::<u8>()).wrapping_offset((i) as isize)).write(255u8);
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 18i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((((battleBerry).wrapping_add(8)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .write(
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(12792))
                            .wrapping_add(28))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((battleBerry).wrapping_add(7)).write(
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792))
                    .wrapping_add(46))
                .read(),
            );
            ((battleBerry).wrapping_add(26)).write(
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792))
                    .wrapping_add(47))
                .read(),
            );
        } else {
            let mut berryData: *mut u8 = GetBerryInfo(ItemIdToBerryType(175u16));
            {
                i = 0i32;
                'l5: loop {
                    if !(i < 6i32) {
                        break 'l5;
                    }
                    'l6: {
                        (((battleBerry).cast::<u8>()).wrapping_offset((i) as isize)).write(
                            (((berryData).cast::<u8>()).wrapping_offset((i) as isize)).read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((battleBerry).cast::<u8>()).wrapping_offset((i) as isize)).write(255u8);
            {
                i = 0i32;
                'l7: loop {
                    if !(i < 18i32) {
                        break 'l7;
                    }
                    'l8: {
                        ((((battleBerry).wrapping_add(8)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((battleBerry).wrapping_add(7)).write(0u8);
            ((battleBerry).wrapping_add(26)).write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn SetAllPlayersBerryData() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0) {
            if IsEnigmaBerryValid() == 1u32 {
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i < 6i32) {
                            break 'l1;
                        }
                        'l2: {
                            (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(12792))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                            ((((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(56))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(12792))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>()).cast::<u8>())
                    .wrapping_offset((i) as isize))
                .write(255u8);
                ((((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>()).wrapping_offset(56))
                    .cast::<u8>())
                .wrapping_offset((i) as isize))
                .write(255u8);
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < 18i32) {
                            break 'l3;
                        }
                        'l4: {
                            ((((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(12792))
                                .wrapping_add(28))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                            (((((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(56))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(12792))
                                .wrapping_add(28))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>()).wrapping_add(7)).write(
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792))
                        .wrapping_add(46))
                    .read(),
                );
                (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>()).wrapping_offset(56))
                    .wrapping_add(7))
                .write(
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792))
                        .wrapping_add(46))
                    .read(),
                );
                ((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>()).wrapping_add(26)).write(
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792))
                        .wrapping_add(47))
                    .read(),
                );
                (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>()).wrapping_offset(56))
                    .wrapping_add(26))
                .write(
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792))
                        .wrapping_add(47))
                    .read(),
                );
            } else {
                let mut berryData: *mut u8 = GetBerryInfo(ItemIdToBerryType(175u16));
                {
                    i = 0i32;
                    'l5: loop {
                        if !(i < 6i32) {
                            break 'l5;
                        }
                        'l6: {
                            (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(
                                (((berryData).cast::<u8>()).wrapping_offset((i) as isize)).read(),
                            );
                            ((((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(56))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(
                                (((berryData).cast::<u8>()).wrapping_offset((i) as isize)).read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>()).cast::<u8>())
                    .wrapping_offset((i) as isize))
                .write(255u8);
                ((((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>()).wrapping_offset(56))
                    .cast::<u8>())
                .wrapping_offset((i) as isize))
                .write(255u8);
                {
                    i = 0i32;
                    'l7: loop {
                        if !(i < 18i32) {
                            break 'l7;
                        }
                        'l8: {
                            ((((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(0u8);
                            (((((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(56))
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(0u8);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>()).wrapping_add(7))
                    .write(0u8);
                (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>()).wrapping_offset(56))
                    .wrapping_add(7))
                .write(0u8);
                ((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>()).wrapping_add(26))
                    .write(0u8);
                (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>()).wrapping_offset(56))
                    .wrapping_add(26))
                .write(0u8);
            }
        } else {
            let mut numPlayers: i32 = 0i32;
            let mut src: *mut u8 = core::ptr::null_mut();
            let mut battler: u8 = 0u8;
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 64u32) != 0 {
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 256u32) != 0 {
                    numPlayers = 2i32;
                } else {
                    numPlayers = 4i32;
                }
                {
                    i = 0i32;
                    'l9: loop {
                        if !(i < numPlayers) {
                            break 'l9;
                        }
                        'l10: {
                            src = (((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset((i) as isize * 256))
                            .cast::<u16>())
                            .wrapping_offset(2))
                            .cast::<u8>();
                            battler = ((((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(24)
                            .cast::<u16>())
                            .read()) as u8);
                            {
                                j = 0i32;
                                'l11: loop {
                                    if !(j < 6i32) {
                                        break 'l11;
                                    }
                                    'l12: {
                                        ((((((&raw mut gEnigmaBerries).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 28))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .write(
                                            (((src).cast::<u8>()).wrapping_offset((j) as isize))
                                                .read(),
                                        );
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                            ((((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 28))
                            .cast::<u8>())
                            .wrapping_offset((j) as isize))
                            .write(255u8);
                            {
                                j = 0i32;
                                'l13: loop {
                                    if !(j < 18i32) {
                                        break 'l13;
                                    }
                                    'l14: {
                                        (((((((&raw mut gEnigmaBerries).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 28))
                                        .wrapping_add(8))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .write(
                                            ((((src).wrapping_add(8)).cast::<u8>())
                                                .wrapping_offset((j) as isize))
                                            .read(),
                                        );
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                            (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 28))
                            .wrapping_add(7))
                            .write(((src).wrapping_add(7)).read());
                            (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize * 28))
                            .wrapping_add(26))
                            .write(((src).wrapping_add(26)).read());
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                {
                    i = 0i32;
                    'l15: loop {
                        if !(i < 2i32) {
                            break 'l15;
                        }
                        'l16: {
                            src = (((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset((i) as isize * 256))
                            .cast::<u16>())
                            .wrapping_offset(2))
                            .cast::<u8>();
                            {
                                j = 0i32;
                                'l17: loop {
                                    if !(j < 6i32) {
                                        break 'l17;
                                    }
                                    'l18: {
                                        ((((((&raw mut gEnigmaBerries).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .write(
                                            (((src).cast::<u8>()).wrapping_offset((j) as isize))
                                                .read(),
                                        );
                                        ((((((&raw mut gEnigmaBerries).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(((i).wrapping_add(2i32)) as isize * 28))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .write(
                                            (((src).cast::<u8>()).wrapping_offset((j) as isize))
                                                .read(),
                                        );
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                            ((((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .cast::<u8>())
                            .wrapping_offset((j) as isize))
                            .write(255u8);
                            ((((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i).wrapping_add(2i32)) as isize * 28))
                            .cast::<u8>())
                            .wrapping_offset((j) as isize))
                            .write(255u8);
                            {
                                j = 0i32;
                                'l19: loop {
                                    if !(j < 18i32) {
                                        break 'l19;
                                    }
                                    'l20: {
                                        (((((((&raw mut gEnigmaBerries).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                        .wrapping_add(8))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .write(
                                            ((((src).wrapping_add(8)).cast::<u8>())
                                                .wrapping_offset((j) as isize))
                                            .read(),
                                        );
                                        (((((((&raw mut gEnigmaBerries).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((i).wrapping_add(2i32)) as isize * 28,
                                        ))
                                        .wrapping_add(8))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .write(
                                            ((((src).wrapping_add(8)).cast::<u8>())
                                                .wrapping_offset((j) as isize))
                                            .read(),
                                        );
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                            (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(7))
                            .write(((src).wrapping_add(7)).read());
                            (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i).wrapping_add(2i32)) as isize * 28))
                            .wrapping_add(7))
                            .write(((src).wrapping_add(7)).read());
                            (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(26))
                            .write(((src).wrapping_add(26)).read());
                            (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i).wrapping_add(2i32)) as isize * 28))
                            .wrapping_add(26))
                            .write(((src).wrapping_add(26)).read());
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FindLinkBattleMaster(numPlayers: u8, multiPlayerId: u8) {
    unsafe {
        let mut numPlayers = numPlayers;
        let mut multiPlayerId = multiPlayerId;
        let mut found: u8 = 0u8;
        if (((((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).read()) as i32) == 256i32 {
            if ((multiPlayerId) as i32) == 0i32 {
                let __p1 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
                (__p1).write(((__p1).read() | 12u32));
            } else {
                let __p2 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
                (__p2).write(((__p2).read() | 8u32));
            }
            found = (found).wrapping_add(1);
        }
        if ((found) as i32) == 0i32 {
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((numPlayers) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).read())
                            as i32)
                            != ((((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset((i) as isize * 256))
                            .cast::<u16>())
                            .read()) as i32)
                        {
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if i == ((numPlayers) as i32) {
                if ((multiPlayerId) as i32) == 0i32 {
                    let __p3 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
                    (__p3).write(((__p3).read() | 12u32));
                } else {
                    let __p4 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
                    (__p4).write(((__p4).read() | 8u32));
                }
                found = (found).wrapping_add(1);
            }
            if ((found) as i32) == 0i32 {
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < ((numPlayers) as i32)) {
                            break 'l3;
                        }
                        'l4: {
                            if (((((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset((i) as isize * 256))
                            .cast::<u16>())
                            .read()) as i32)
                                == 768i32)
                                && (i != ((multiPlayerId) as i32))
                            {
                                if i < ((multiPlayerId) as i32) {
                                    break 'l3;
                                }
                            }
                            if (((((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset((i) as isize * 256))
                            .cast::<u16>())
                            .read()) as i32)
                                > 768i32)
                                && (i != ((multiPlayerId) as i32))
                            {
                                break 'l3;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i == ((numPlayers) as i32) {
                    let __p5 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
                    (__p5).write(((__p5).read() | 12u32));
                } else {
                    let __p6 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
                    (__p6).write(((__p6).read() | 8u32));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_HandleStartBattle() {
    unsafe {
        let mut playerMultiplayerId: u8 = 0u8;
        let mut enemyMultiplayerId: u8 = 0u8;
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        playerMultiplayerId = GetMultiplayerId();
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).write(playerMultiplayerId);
        enemyMultiplayerId = ((((playerMultiplayerId) as i32) ^ 1i32) as u8);
        'l1: {
            let __sw1 =
                (((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ShowBg(0u8);
                    ShowBg(1u8);
                    ShowBg(2u8);
                    ShowBg(3u8);
                    FillAroundBattleWindows();
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(1u8);
                }
                if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                    LoadWirelessStatusIndicatorSpriteGfx();
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0 {
                    if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                        if (IsLinkTaskFinished()) != 0 {
                            ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(384))
                            .write(0u8);
                            (((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(384))
                            .wrapping_add(1))
                            .write(3u8);
                            BufferPartyVsScreenHealth_AtStart();
                            SetPlayerBerryDataInBattleStruct();
                            if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                                == 3072i32
                            {
                                (((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_add(24)
                                    .cast::<u16>())
                                .write(0u16);
                                ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                                    .wrapping_add(24)
                                    .cast::<u16>())
                                .write(1u16);
                            }
                            SendBlock(
                                BitmaskAllOtherLinkPlayers(),
                                ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(384)),
                                32u16,
                            );
                            (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                .write(2u8);
                        }
                        if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                            CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                        }
                    }
                } else {
                    if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read()
                        & 16777216u32)
                        != 0)
                    {
                        let __p2 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
                        (__p2).write(((__p2).read() | 4u32));
                    }
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(15u8);
                    SetAllPlayersBerryData();
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    let mut taskId: u8 = 0u8;
                    ResetBlockReceivedFlags();
                    FindLinkBattleMaster(2u8, playerMultiplayerId);
                    SetAllPlayersBerryData();
                    taskId = CreateTask(Some(InitLinkBattleVsScreen), 0u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(270i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(90i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(
                        (((((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(384))
                        .wrapping_add(2))
                        .read()) as i32)
                            | ((((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(384))
                            .wrapping_add(3))
                            .read()) as i32)
                                << 8)) as i16),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(
                        (((((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset(((enemyMultiplayerId) as i32) as isize * 256))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read()) as i16),
                    );
                    RecordedBattle_SetFrontierPassFlagFromHword(
                        (((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset(((playerMultiplayerId) as i32) as isize * 256))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read(),
                    );
                    RecordedBattle_SetFrontierPassFlagFromHword(
                        (((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset(((enemyMultiplayerId) as i32) as isize * 256))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read(),
                    );
                    SetDeoxysStats();
                    let __p3 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        (&raw mut gPlayerParty).cast::<u8>(),
                        200u16,
                    );
                    let __p4 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    crate::c::memcpy(
                        (&raw mut gEnemyParty).cast::<u8>(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset(((enemyMultiplayerId) as i32) as isize * 256))
                        .cast::<u16>())
                        .cast::<u8>(),
                        200u32,
                    );
                    let __p5 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(200),
                        200u16,
                    );
                    let __p6 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    crate::c::memcpy(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(200),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset(((enemyMultiplayerId) as i32) as isize * 256))
                        .cast::<u16>())
                        .cast::<u8>(),
                        200u32,
                    );
                    let __p7 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(400),
                        200u16,
                    );
                    let __p8 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    crate::c::memcpy(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(400),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset(((enemyMultiplayerId) as i32) as isize * 256))
                        .cast::<u16>())
                        .cast::<u8>(),
                        200u32,
                    );
                    TryCorrectShedinjaLanguage((&raw mut gEnemyParty).cast::<u8>());
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(100),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(200),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(300),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(400),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(500),
                    );
                    let __p9 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 15i32 {
                __fall = true;
                InitBattleControllers();
                RecordedBattle_SetTrainerInfo();
                ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                    .write(0u8);
                ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
                    .write(0u8);
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0 {
                    let mut i: i32 = 0i32;
                    {
                        i = 0i32;
                        'l2: loop {
                            if !((i < 2i32)
                                && ((((((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset((i) as isize * 28))
                                .cast::<u16>())
                                .read()) as i32)
                                    & 255i32)
                                    == 3i32))
                            {
                                break 'l2;
                            }
                            'l3: {}
                            i = (i).wrapping_add(1);
                        }
                    }
                    if i == 2i32 {
                        (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(16u8);
                    } else {
                        (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(18u8);
                    }
                } else {
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(18u8);
                }
                break 'l1;
            }
            if __sw1 == 16i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        ((&raw mut gRecordedBattleRngSeed).cast::<u32>()).cast::<u8>(),
                        4u16,
                    );
                    let __p10 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 17i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 4u32)
                        != 0)
                    {
                        crate::c::memcpy(
                            ((&raw mut gRecordedBattleRngSeed).cast::<u32>()).cast::<u8>(),
                            ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset(((enemyMultiplayerId) as i32) as isize * 256))
                            .cast::<u16>())
                            .cast::<u8>(),
                            4u32,
                        );
                    }
                    let __p11 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 18i32 {
                __fall = true;
                if (BattleInitAllSprites(
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1),
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(2),
                )) != 0
                {
                    ((&raw mut gPreBattleCallback1)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(
                        (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                            .read(),
                    );
                    (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(BattleMainCB1));
                    SetMainCallback2(Some(BattleMainCB2));
                    if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0
                    {
                        let __p12 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
                        (__p12).write(((__p12).read() | 32u32));
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 || __sw1 == 9i32 || __sw1 == 13i32 {
                __fall = true;
                let __p13 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                (__p13).write(((__p13).read()).wrapping_add(1));
                ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                    .write(1u8);
            }
            if __fall || __sw1 == 6i32 || __sw1 == 10i32 || __sw1 == 14i32 {
                __fall = true;
                if (({
                    let __p14 = (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1);
                    let __t15 = ((__p14).read()).wrapping_sub(1);
                    (__p14).write(__t15);
                    __t15
                }) as i32)
                    == 0i32
                {
                    let __p16 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p16).write(((__p16).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_HandleStartMultiPartnerBattle() {
    unsafe {
        let mut playerMultiplayerId: u8 = 0u8;
        let mut partnerMultiplayerId: u8 = 0u8;
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        playerMultiplayerId = GetMultiplayerId();
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).write(playerMultiplayerId);
        partnerMultiplayerId = ((((playerMultiplayerId) as i32) ^ 1i32) as u8);
        'l1: {
            let __sw1 =
                (((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ShowBg(0u8);
                    ShowBg(1u8);
                    ShowBg(2u8);
                    ShowBg(3u8);
                    FillAroundBattleWindows();
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(1u8);
                }
                if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                    LoadWirelessStatusIndicatorSpriteGfx();
                }
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0 {
                    if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                        let mut language: u8 = 0u8;
                        (((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_add(24)
                            .cast::<u16>())
                        .write(0u16);
                        ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .wrapping_add(24)
                            .cast::<u16>())
                        .write(2u16);
                        ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56))
                            .wrapping_add(24)
                            .cast::<u16>())
                        .write(1u16);
                        ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(84))
                            .wrapping_add(24)
                            .cast::<u16>())
                        .write(3u16);
                        GetFrontierTrainerName(
                            ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56))
                                .wrapping_add(8))
                            .cast::<u8>(),
                            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                        );
                        GetFrontierTrainerName(
                            ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(84))
                                .wrapping_add(8))
                            .cast::<u8>(),
                            ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
                        );
                        GetBattleTowerTrainerLanguage(
                            &raw mut language,
                            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                        );
                        ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56))
                            .wrapping_add(26)
                            .cast::<u16>())
                        .write(((language) as u16));
                        GetBattleTowerTrainerLanguage(
                            &raw mut language,
                            ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
                        );
                        ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(84))
                            .wrapping_add(26)
                            .cast::<u16>())
                        .write(((language) as u16));
                        if (IsLinkTaskFinished()) != 0 {
                            ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(384))
                            .write(0u8);
                            (((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(384))
                            .wrapping_add(1))
                            .write(3u8);
                            BufferPartyVsScreenHealth_AtStart();
                            SetPlayerBerryDataInBattleStruct();
                            SendBlock(
                                BitmaskAllOtherLinkPlayers(),
                                ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(384)),
                                32u16,
                            );
                            (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                .write(2u8);
                        }
                        if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                            CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                        }
                    }
                } else {
                    if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read()
                        & 16777216u32)
                        != 0)
                    {
                        let __p2 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
                        (__p2).write(((__p2).read() | 4u32));
                    }
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(13u8);
                    SetAllPlayersBerryData();
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    let mut taskId: u8 = 0u8;
                    ResetBlockReceivedFlags();
                    FindLinkBattleMaster(2u8, playerMultiplayerId);
                    SetAllPlayersBerryData();
                    taskId = CreateTask(Some(InitLinkBattleVsScreen), 0u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(270i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(90i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(325i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(325i16);
                    let __p3 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        (&raw mut gPlayerParty).cast::<u8>(),
                        200u16,
                    );
                    let __p4 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    if ((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((playerMultiplayerId) as i32) as isize * 28))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        crate::c::memcpy(
                            (&raw mut gPlayerParty).cast::<u8>(),
                            ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset(((partnerMultiplayerId) as i32) as isize * 256))
                            .cast::<u16>())
                            .cast::<u8>(),
                            200u32,
                        );
                        crate::c::memcpy(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((crate::c::div_i32(6i32, 2i32)) as isize * 100),
                            ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset(((playerMultiplayerId) as i32) as isize * 256))
                            .cast::<u16>())
                            .cast::<u8>(),
                            200u32,
                        );
                    } else {
                        crate::c::memcpy(
                            (&raw mut gPlayerParty).cast::<u8>(),
                            ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset(((playerMultiplayerId) as i32) as isize * 256))
                            .cast::<u16>())
                            .cast::<u8>(),
                            200u32,
                        );
                        crate::c::memcpy(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((crate::c::div_i32(6i32, 2i32)) as isize * 100),
                            ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset(((partnerMultiplayerId) as i32) as isize * 256))
                            .cast::<u16>())
                            .cast::<u8>(),
                            200u32,
                        );
                    }
                    let __p5 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(200),
                        100u16,
                    );
                    let __p6 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    if ((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((playerMultiplayerId) as i32) as isize * 28))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        crate::c::memcpy(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(200),
                            ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset(((partnerMultiplayerId) as i32) as isize * 256))
                            .cast::<u16>())
                            .cast::<u8>(),
                            100u32,
                        );
                        crate::c::memcpy(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((2i32).wrapping_add(crate::c::div_i32(6i32, 2i32))) as isize * 100,
                            ),
                            ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset(((playerMultiplayerId) as i32) as isize * 256))
                            .cast::<u16>())
                            .cast::<u8>(),
                            100u32,
                        );
                    } else {
                        crate::c::memcpy(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(200),
                            ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset(((playerMultiplayerId) as i32) as isize * 256))
                            .cast::<u16>())
                            .cast::<u8>(),
                            100u32,
                        );
                        crate::c::memcpy(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((2i32).wrapping_add(crate::c::div_i32(6i32, 2i32))) as isize * 100,
                            ),
                            ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset(((partnerMultiplayerId) as i32) as isize * 256))
                            .cast::<u16>())
                            .cast::<u8>(),
                            100u32,
                        );
                    }
                    let __p7 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        (&raw mut gEnemyParty).cast::<u8>(),
                        200u16,
                    );
                    let __p8 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    if ((GetMultiplayerId()) as i32) != 0i32 {
                        crate::c::memcpy(
                            (&raw mut gEnemyParty).cast::<u8>(),
                            (((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).cast::<u8>(),
                            200u32,
                        );
                    }
                    let __p9 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(200),
                        200u16,
                    );
                    let __p10 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    if ((GetMultiplayerId()) as i32) != 0i32 {
                        crate::c::memcpy(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(200),
                            (((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).cast::<u8>(),
                            200u32,
                        );
                    }
                    let __p11 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(400),
                        200u16,
                    );
                    let __p12 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    if ((GetMultiplayerId()) as i32) != 0i32 {
                        crate::c::memcpy(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(400),
                            (((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).cast::<u8>(),
                            200u32,
                        );
                    }
                    TryCorrectShedinjaLanguage((&raw mut gPlayerParty).cast::<u8>());
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(100),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(200),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(300),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(400),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(500),
                    );
                    TryCorrectShedinjaLanguage((&raw mut gEnemyParty).cast::<u8>());
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(100),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(200),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(300),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(400),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(500),
                    );
                    let __p13 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p13).write(((__p13).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                __fall = true;
                InitBattleControllers();
                RecordedBattle_SetTrainerInfo();
                ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                    .write(0u8);
                ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
                    .write(0u8);
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0 {
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(14u8);
                } else {
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(16u8);
                }
                break 'l1;
            }
            if __sw1 == 14i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        ((&raw mut gRecordedBattleRngSeed).cast::<u32>()).cast::<u8>(),
                        4u16,
                    );
                    let __p14 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 15i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 4u32)
                        != 0)
                    {
                        crate::c::memcpy(
                            ((&raw mut gRecordedBattleRngSeed).cast::<u32>()).cast::<u8>(),
                            ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                .wrapping_offset(((partnerMultiplayerId) as i32) as isize * 256))
                            .cast::<u16>())
                            .cast::<u8>(),
                            4u32,
                        );
                    }
                    let __p15 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p15).write(((__p15).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 16i32 {
                __fall = true;
                if (BattleInitAllSprites(
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1),
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(2),
                )) != 0
                {
                    TrySetLinkBattleTowerEnemyPartyLevel();
                    ((&raw mut gPreBattleCallback1)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(
                        (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                            .read(),
                    );
                    (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(BattleMainCB1));
                    SetMainCallback2(Some(BattleMainCB2));
                    if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0
                    {
                        let __p16 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
                        (__p16).write(((__p16).read() | 32u32));
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetMultiPartnerMenuParty(offset: u8) {
    unsafe {
        let mut offset = offset;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < crate::c::div_i32(6i32, 2i32)) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 32))
                    .cast::<u16>())
                    .write(
                        ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                (((offset) as i32).wrapping_add(i)) as isize * 100,
                            ),
                            11i32,
                        )) as u16),
                    );
                    (((((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 32))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .write(
                        ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                (((offset) as i32).wrapping_add(i)) as isize * 100,
                            ),
                            12i32,
                        )) as u16),
                    );
                    GetMonData3(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset((((offset) as i32).wrapping_add(i)) as isize * 100),
                        2i32,
                        (((((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 32))
                        .wrapping_add(4))
                        .cast::<u8>(),
                    );
                    (((((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 32))
                    .wrapping_add(15))
                    .write(
                        ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                (((offset) as i32).wrapping_add(i)) as isize * 100,
                            ),
                            56i32,
                        )) as u8),
                    );
                    (((((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 32))
                    .wrapping_add(16)
                    .cast::<u16>())
                    .write(
                        ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                (((offset) as i32).wrapping_add(i)) as isize * 100,
                            ),
                            57i32,
                        )) as u16),
                    );
                    (((((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 32))
                    .wrapping_add(18)
                    .cast::<u16>())
                    .write(
                        ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                (((offset) as i32).wrapping_add(i)) as isize * 100,
                            ),
                            58i32,
                        )) as u16),
                    );
                    (((((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 32))
                    .wrapping_add(20)
                    .cast::<u32>())
                    .write(GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset((((offset) as i32).wrapping_add(i)) as isize * 100),
                        55i32,
                    ));
                    (((((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 32))
                    .wrapping_add(24)
                    .cast::<u32>())
                    .write(GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset((((offset) as i32).wrapping_add(i)) as isize * 100),
                        0i32,
                    ));
                    (((((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 32))
                    .wrapping_add(28))
                    .write(GetMonGender(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset((((offset) as i32).wrapping_add(i)) as isize * 100),
                    ));
                    StripExtCtrlCodes(
                        (((((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 32))
                        .wrapping_add(4))
                        .cast::<u8>(),
                    );
                    if GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset((((offset) as i32).wrapping_add(i)) as isize * 100),
                        3i32,
                    ) != 1u32
                    {
                        PadNameString(
                            (((((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 32))
                            .wrapping_add(4))
                            .cast::<u8>(),
                            0u8,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::memcpy(
            ((&raw mut sMultiPartnerPartyBuffer)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
            ((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>(),
            96u32,
        );
    }
}
pub(crate) unsafe extern "C" fn CB2_PreInitMultiBattle() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut playerMultiplierId: u8 = 0u8;
        let mut numPlayers: i32 = 4i32;
        let mut blockMask: u8 = 15u8;
        let mut savedBattleTypeFlags: *mut u32 = core::ptr::null_mut();
        let mut savedCallback: *mut Option<unsafe extern "C" fn()> = core::ptr::null_mut();
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 256u32) != 0 {
            numPlayers = 2i32;
            blockMask = 3u8;
        }
        playerMultiplierId = GetMultiplayerId();
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).write(playerMultiplierId);
        savedCallback = (((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(180)
            .cast::<Option<unsafe extern "C" fn()>>();
        savedBattleTypeFlags = (((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(172)
            .cast::<u32>();
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        'l1: {
            let __sw1 =
                (((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).read()) as i32);
            if __sw1 == 0i32 {
                if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
                    && ((IsLinkTaskFinished()) != 0)
                {
                    ((&raw mut sMultiPartnerPartyBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .write(Alloc(96u32));
                    SetMultiPartnerMenuParty(0u8);
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        ((&raw mut sMultiPartnerPartyBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read(),
                        96u16,
                    );
                    let __p2 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((GetBlockReceivedStatus()) as i32) & ((blockMask) as i32))
                    == ((blockMask) as i32)
                {
                    ResetBlockReceivedFlags();
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < numPlayers) {
                                break 'l2;
                            }
                            'l3: {
                                if i == ((playerMultiplierId) as i32) {
                                    break 'l3;
                                }
                                if numPlayers == 4i32 {
                                    if ((!((((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                    .wrapping_add(24)
                                    .cast::<u16>())
                                    .read())
                                        as i32)
                                        & 1i32)
                                        != 0))
                                        && (!((((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset(
                                                ((playerMultiplierId) as i32) as isize * 28,
                                            ))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read())
                                            as i32)
                                            & 1i32)
                                            != 0)))
                                        || (((((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset((i) as isize * 28))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read())
                                            as i32)
                                            & 1i32)
                                            != 0)
                                            && ((((((((&raw mut gLinkPlayers).cast::<u8>())
                                                .wrapping_offset(
                                                    ((playerMultiplierId) as i32) as isize * 28,
                                                ))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                & 1i32)
                                                != 0))
                                    {
                                        crate::c::memcpy(
                                            ((&raw mut gMultiPartnerParty).cast::<u8>())
                                                .cast::<u8>(),
                                            ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                .wrapping_offset((i) as isize * 256))
                                            .cast::<u16>())
                                            .cast::<u8>(),
                                            96u32,
                                        );
                                    }
                                } else {
                                    crate::c::memcpy(
                                        ((&raw mut gMultiPartnerParty).cast::<u8>()).cast::<u8>(),
                                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                            .wrapping_offset((i) as isize * 256))
                                        .cast::<u16>())
                                        .cast::<u8>(),
                                        96u32,
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    let __p3 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    (savedCallback).write(
                        (((&raw mut gMain).cast::<u8>())
                            .wrapping_add(8)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                    (savedBattleTypeFlags)
                        .write(((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read());
                    (((&raw mut gMain).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(CB2_PreInitMultiBattle));
                    ShowPartyMenuToShowcaseMultiBattleParty();
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((IsLinkTaskFinished()) != 0)
                    && (!((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                        7,
                        1,
                        false,
                    ) as u16)
                        != 0))
                {
                    let __p4 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                        SetLinkStandbyCallback();
                    } else {
                        SetCloseLinkCallback();
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                    if (IsLinkRfuTaskFinished()) != 0 {
                        ((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>())
                            .write((savedBattleTypeFlags).read());
                        (((&raw mut gMain).cast::<u8>())
                            .wrapping_add(8)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .write((savedCallback).read());
                        SetMainCallback2(Some(CB2_InitBattleInternal));
                        {
                            Free(
                                ((&raw mut sMultiPartnerPartyBuffer)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read(),
                            );
                            ((&raw mut sMultiPartnerPartyBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                        }
                    }
                } else {
                    if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32)
                        == 0i32
                    {
                        ((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>())
                            .write((savedBattleTypeFlags).read());
                        (((&raw mut gMain).cast::<u8>())
                            .wrapping_add(8)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .write((savedCallback).read());
                        SetMainCallback2(Some(CB2_InitBattleInternal));
                        {
                            Free(
                                ((&raw mut sMultiPartnerPartyBuffer)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read(),
                            );
                            ((&raw mut sMultiPartnerPartyBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                        }
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_PreInitIngamePlayerPartnerBattle() {
    unsafe {
        let mut savedBattleTypeFlags: *mut u32 = core::ptr::null_mut();
        let mut savedCallback: *mut Option<unsafe extern "C" fn()> = core::ptr::null_mut();
        savedCallback = (((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(180)
            .cast::<Option<unsafe extern "C" fn()>>();
        savedBattleTypeFlags = (((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(172)
            .cast::<u32>();
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        'l1: {
            let __sw1 =
                (((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut sMultiPartnerPartyBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(Alloc(96u32));
                SetMultiPartnerMenuParty(((crate::c::div_i32(6i32, 2i32)) as u8));
                let __p2 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                (savedCallback).write(
                    (((&raw mut gMain).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
                (savedBattleTypeFlags)
                    .write(((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read());
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(CB2_PreInitIngamePlayerPartnerBattle));
                ShowPartyMenuToShowcaseMultiBattleParty();
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(2u8);
                    ((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>())
                        .write((savedBattleTypeFlags).read());
                    (((&raw mut gMain).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write((savedCallback).read());
                    SetMainCallback2(Some(CB2_InitBattleInternal));
                    {
                        Free(
                            ((&raw mut sMultiPartnerPartyBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sMultiPartnerPartyBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_HandleStartMultiBattle() {
    unsafe {
        let mut playerMultiplayerId: u8 = 0u8;
        let mut id: i32 = 0i32;
        let mut var: u8 = 0u8;
        playerMultiplayerId = GetMultiplayerId();
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).write(playerMultiplayerId);
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        'l1: {
            let __sw1 =
                (((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ShowBg(0u8);
                    ShowBg(1u8);
                    ShowBg(2u8);
                    ShowBg(3u8);
                    FillAroundBattleWindows();
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(1u8);
                }
                if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                    LoadWirelessStatusIndicatorSpriteGfx();
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0 {
                    if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                        if (IsLinkTaskFinished()) != 0 {
                            ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(384))
                            .write(0u8);
                            (((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(384))
                            .wrapping_add(1))
                            .write(3u8);
                            BufferPartyVsScreenHealth_AtStart();
                            SetPlayerBerryDataInBattleStruct();
                            SendBlock(
                                BitmaskAllOtherLinkPlayers(),
                                ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(384)),
                                32u16,
                            );
                            let __p2 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                            (__p2).write(((__p2).read()).wrapping_add(1));
                        }
                        if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                            CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                        }
                    }
                } else {
                    if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read()
                        & 16777216u32)
                        != 0)
                    {
                        let __p3 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
                        (__p3).write(((__p3).read() | 4u32));
                    }
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(7u8);
                    SetAllPlayersBerryData();
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 15i32) == 15i32 {
                    ResetBlockReceivedFlags();
                    FindLinkBattleMaster(4u8, playerMultiplayerId);
                    SetAllPlayersBerryData();
                    SetDeoxysStats();
                    var = CreateTask(Some(InitLinkBattleVsScreen), 0u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((var) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(270i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((var) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(90i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((var) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((var) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((var) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(0i16);
                    {
                        id = 0i32;
                        'l2: loop {
                            if !(id < 4i32) {
                                break 'l2;
                            }
                            'l3: {
                                RecordedBattle_SetFrontierPassFlagFromHword(
                                    (((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                        .wrapping_offset((id) as isize * 256))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read(),
                                );
                                'l4: {
                                    let __sw4 = ((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((id) as isize * 28))
                                    .wrapping_add(24)
                                    .cast::<u16>())
                                    .read())
                                        as i32);
                                    if __sw4 == 0i32 {
                                        let __p5 = (((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((var) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(3);
                                        (__p5).write(
                                            (((((__p5).read()) as i32)
                                                | ((((((((&raw mut gBlockRecvBuffer)
                                                    .cast::<u8>())
                                                .wrapping_offset((id) as isize * 256))
                                                .cast::<u16>())
                                                .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    & 63i32))
                                                as i16),
                                        );
                                        break 'l4;
                                    }
                                    if __sw4 == 1i32 {
                                        let __p6 = (((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((var) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(4);
                                        (__p6).write(
                                            (((((__p6).read()) as i32)
                                                | ((((((((&raw mut gBlockRecvBuffer)
                                                    .cast::<u8>())
                                                .wrapping_offset((id) as isize * 256))
                                                .cast::<u16>())
                                                .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    & 63i32))
                                                as i16),
                                        );
                                        break 'l4;
                                    }
                                    if __sw4 == 2i32 {
                                        let __p7 = (((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((var) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(3);
                                        (__p7).write(
                                            (((((__p7).read()) as i32)
                                                | (((((((((&raw mut gBlockRecvBuffer)
                                                    .cast::<u8>())
                                                .wrapping_offset((id) as isize * 256))
                                                .cast::<u16>())
                                                .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    & 63i32)
                                                    << 6))
                                                as i16),
                                        );
                                        break 'l4;
                                    }
                                    if __sw4 == 3i32 {
                                        let __p8 = (((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((var) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(4);
                                        (__p8).write(
                                            (((((__p8).read()) as i32)
                                                | (((((((((&raw mut gBlockRecvBuffer)
                                                    .cast::<u8>())
                                                .wrapping_offset((id) as isize * 256))
                                                .cast::<u16>())
                                                .wrapping_offset(1))
                                                .read())
                                                    as i32)
                                                    & 63i32)
                                                    << 6))
                                                as i16),
                                        );
                                        break 'l4;
                                    }
                                }
                            }
                            id = (id).wrapping_add(1);
                        }
                    }
                    ZeroEnemyPartyMons();
                    let __p9 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                } else {
                    break 'l1;
                }
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        (&raw mut gPlayerParty).cast::<u8>(),
                        200u16,
                    );
                    let __p10 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 15i32) == 15i32 {
                    ResetBlockReceivedFlags();
                    {
                        id = 0i32;
                        'l5: loop {
                            if !(id < 4i32) {
                                break 'l5;
                            }
                            'l6: {
                                if id == ((playerMultiplayerId) as i32) {
                                    'l7: {
                                        let __sw11 = ((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset((id) as isize * 28))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read())
                                            as i32);
                                        if __sw11 == 0i32 || __sw11 == 3i32 {
                                            crate::c::memcpy(
                                                (&raw mut gPlayerParty).cast::<u8>(),
                                                ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                    .wrapping_offset((id) as isize * 256))
                                                .cast::<u16>())
                                                .cast::<u8>(),
                                                200u32,
                                            );
                                            break 'l7;
                                        }
                                        if __sw11 == 1i32 || __sw11 == 2i32 {
                                            crate::c::memcpy(
                                                ((&raw mut gPlayerParty).cast::<u8>())
                                                    .wrapping_offset(
                                                        (crate::c::div_i32(6i32, 2i32)) as isize
                                                            * 100,
                                                    ),
                                                ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                    .wrapping_offset((id) as isize * 256))
                                                .cast::<u16>())
                                                .cast::<u8>(),
                                                200u32,
                                            );
                                            break 'l7;
                                        }
                                    }
                                } else {
                                    if ((!((((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((id) as isize * 28))
                                    .wrapping_add(24)
                                    .cast::<u16>())
                                    .read())
                                        as i32)
                                        & 1i32)
                                        != 0))
                                        && (!((((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset(
                                                ((playerMultiplayerId) as i32) as isize * 28,
                                            ))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read())
                                            as i32)
                                            & 1i32)
                                            != 0)))
                                        || (((((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset((id) as isize * 28))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read())
                                            as i32)
                                            & 1i32)
                                            != 0)
                                            && ((((((((&raw mut gLinkPlayers).cast::<u8>())
                                                .wrapping_offset(
                                                    ((playerMultiplayerId) as i32) as isize * 28,
                                                ))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                & 1i32)
                                                != 0))
                                    {
                                        'l8: {
                                            let __sw12 = ((((((&raw mut gLinkPlayers)
                                                .cast::<u8>())
                                            .wrapping_offset((id) as isize * 28))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32);
                                            if __sw12 == 0i32 || __sw12 == 3i32 {
                                                crate::c::memcpy(
                                                    (&raw mut gPlayerParty).cast::<u8>(),
                                                    ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                        .wrapping_offset((id) as isize * 256))
                                                    .cast::<u16>())
                                                    .cast::<u8>(),
                                                    200u32,
                                                );
                                                break 'l8;
                                            }
                                            if __sw12 == 1i32 || __sw12 == 2i32 {
                                                crate::c::memcpy(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset(
                                                            (crate::c::div_i32(6i32, 2i32))
                                                                as isize
                                                                * 100,
                                                        ),
                                                    ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                        .wrapping_offset((id) as isize * 256))
                                                    .cast::<u16>())
                                                    .cast::<u8>(),
                                                    200u32,
                                                );
                                                break 'l8;
                                            }
                                        }
                                    } else {
                                        'l9: {
                                            let __sw13 = ((((((&raw mut gLinkPlayers)
                                                .cast::<u8>())
                                            .wrapping_offset((id) as isize * 28))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32);
                                            if __sw13 == 0i32 || __sw13 == 3i32 {
                                                crate::c::memcpy(
                                                    (&raw mut gEnemyParty).cast::<u8>(),
                                                    ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                        .wrapping_offset((id) as isize * 256))
                                                    .cast::<u16>())
                                                    .cast::<u8>(),
                                                    200u32,
                                                );
                                                break 'l9;
                                            }
                                            if __sw13 == 1i32 || __sw13 == 2i32 {
                                                crate::c::memcpy(
                                                    ((&raw mut gEnemyParty).cast::<u8>())
                                                        .wrapping_offset(
                                                            (crate::c::div_i32(6i32, 2i32))
                                                                as isize
                                                                * 100,
                                                        ),
                                                    ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                        .wrapping_offset((id) as isize * 256))
                                                    .cast::<u16>())
                                                    .cast::<u8>(),
                                                    200u32,
                                                );
                                                break 'l9;
                                            }
                                        }
                                    }
                                }
                            }
                            id = (id).wrapping_add(1);
                        }
                    }
                    let __p14 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(200),
                        100u16,
                    );
                    let __p15 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p15).write(((__p15).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 15i32) == 15i32 {
                    ResetBlockReceivedFlags();
                    {
                        id = 0i32;
                        'l10: loop {
                            if !(id < 4i32) {
                                break 'l10;
                            }
                            'l11: {
                                if id == ((playerMultiplayerId) as i32) {
                                    'l12: {
                                        let __sw16 = ((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset((id) as isize * 28))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read())
                                            as i32);
                                        if __sw16 == 0i32 || __sw16 == 3i32 {
                                            crate::c::memcpy(
                                                ((&raw mut gPlayerParty).cast::<u8>())
                                                    .wrapping_offset(200),
                                                ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                    .wrapping_offset((id) as isize * 256))
                                                .cast::<u16>())
                                                .cast::<u8>(),
                                                100u32,
                                            );
                                            break 'l12;
                                        }
                                        if __sw16 == 1i32 || __sw16 == 2i32 {
                                            crate::c::memcpy(
                                                ((&raw mut gPlayerParty).cast::<u8>())
                                                    .wrapping_offset(500),
                                                ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                    .wrapping_offset((id) as isize * 256))
                                                .cast::<u16>())
                                                .cast::<u8>(),
                                                100u32,
                                            );
                                            break 'l12;
                                        }
                                    }
                                } else {
                                    if ((!((((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((id) as isize * 28))
                                    .wrapping_add(24)
                                    .cast::<u16>())
                                    .read())
                                        as i32)
                                        & 1i32)
                                        != 0))
                                        && (!((((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset(
                                                ((playerMultiplayerId) as i32) as isize * 28,
                                            ))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read())
                                            as i32)
                                            & 1i32)
                                            != 0)))
                                        || (((((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset((id) as isize * 28))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read())
                                            as i32)
                                            & 1i32)
                                            != 0)
                                            && ((((((((&raw mut gLinkPlayers).cast::<u8>())
                                                .wrapping_offset(
                                                    ((playerMultiplayerId) as i32) as isize * 28,
                                                ))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                & 1i32)
                                                != 0))
                                    {
                                        'l13: {
                                            let __sw17 = ((((((&raw mut gLinkPlayers)
                                                .cast::<u8>())
                                            .wrapping_offset((id) as isize * 28))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32);
                                            if __sw17 == 0i32 || __sw17 == 3i32 {
                                                crate::c::memcpy(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset(200),
                                                    ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                        .wrapping_offset((id) as isize * 256))
                                                    .cast::<u16>())
                                                    .cast::<u8>(),
                                                    100u32,
                                                );
                                                break 'l13;
                                            }
                                            if __sw17 == 1i32 || __sw17 == 2i32 {
                                                crate::c::memcpy(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset(500),
                                                    ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                        .wrapping_offset((id) as isize * 256))
                                                    .cast::<u16>())
                                                    .cast::<u8>(),
                                                    100u32,
                                                );
                                                break 'l13;
                                            }
                                        }
                                    } else {
                                        'l14: {
                                            let __sw18 = ((((((&raw mut gLinkPlayers)
                                                .cast::<u8>())
                                            .wrapping_offset((id) as isize * 28))
                                            .wrapping_add(24)
                                            .cast::<u16>())
                                            .read())
                                                as i32);
                                            if __sw18 == 0i32 || __sw18 == 3i32 {
                                                crate::c::memcpy(
                                                    ((&raw mut gEnemyParty).cast::<u8>())
                                                        .wrapping_offset(200),
                                                    ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                        .wrapping_offset((id) as isize * 256))
                                                    .cast::<u16>())
                                                    .cast::<u8>(),
                                                    100u32,
                                                );
                                                break 'l14;
                                            }
                                            if __sw18 == 1i32 || __sw18 == 2i32 {
                                                crate::c::memcpy(
                                                    ((&raw mut gEnemyParty).cast::<u8>())
                                                        .wrapping_offset(500),
                                                    ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                        .wrapping_offset((id) as isize * 256))
                                                    .cast::<u16>())
                                                    .cast::<u8>(),
                                                    100u32,
                                                );
                                                break 'l14;
                                            }
                                        }
                                    }
                                }
                            }
                            id = (id).wrapping_add(1);
                        }
                    }
                    TryCorrectShedinjaLanguage((&raw mut gPlayerParty).cast::<u8>());
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(100),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(200),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(300),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(400),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(500),
                    );
                    TryCorrectShedinjaLanguage((&raw mut gEnemyParty).cast::<u8>());
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(100),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(200),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(300),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(400),
                    );
                    TryCorrectShedinjaLanguage(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(500),
                    );
                    let __p19 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p19).write(((__p19).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                InitBattleControllers();
                RecordedBattle_SetTrainerInfo();
                ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                    .write(0u8);
                ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
                    .write(0u8);
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0 {
                    {
                        id = 0i32;
                        'l15: loop {
                            if !((id < 4i32)
                                && ((((((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset((id) as isize * 28))
                                .cast::<u16>())
                                .read()) as i32)
                                    & 255i32)
                                    == 3i32))
                            {
                                break 'l15;
                            }
                            'l16: {}
                            id = (id).wrapping_add(1);
                        }
                    }
                    if id == 4i32 {
                        (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(8u8);
                    } else {
                        (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(10u8);
                    }
                } else {
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(10u8);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    let mut ptr: *mut u32 =
                        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(384))
                        .cast::<u32>();
                    (ptr).write(((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read());
                    ((ptr).wrapping_offset(1))
                        .write(((&raw mut gRecordedBattleRngSeed).cast::<u32>()).read());
                    SendBlock(BitmaskAllOtherLinkPlayers(), (ptr).cast::<u8>(), 8u16);
                    let __p20 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p20).write(((__p20).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                __fall = true;
                if (((GetBlockReceivedStatus()) as i32) & 15i32) == 15i32 {
                    ResetBlockReceivedFlags();
                    {
                        var = 0u8;
                        'l17: loop {
                            if !(((var) as i32) < 4i32) {
                                break 'l17;
                            }
                            'l18: {
                                let mut blockValue: u32 =
                                    ((((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                        .wrapping_offset(((var) as i32) as isize * 256))
                                    .cast::<u16>())
                                    .read()) as u32);
                                if (blockValue & 4u32) != 0 {
                                    crate::c::memcpy(
                                        ((&raw mut gRecordedBattleRngSeed).cast::<u32>())
                                            .cast::<u8>(),
                                        (((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                            .wrapping_offset(((var) as i32) as isize * 256))
                                        .cast::<u16>())
                                        .wrapping_offset(2))
                                        .cast::<u8>(),
                                        4u32,
                                    );
                                    break 'l17;
                                }
                            }
                            var = (var).wrapping_add(1);
                        }
                    }
                    let __p21 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p21).write(((__p21).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                if (BattleInitAllSprites(
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1),
                    (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(2),
                )) != 0
                {
                    ((&raw mut gPreBattleCallback1)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(
                        (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                            .read(),
                    );
                    (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(BattleMainCB1));
                    SetMainCallback2(Some(BattleMainCB2));
                    if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0
                    {
                        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(2048u16);
                        let __p22 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
                        (__p22).write(((__p22).read() | 32u32));
                    }
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleMainCB2() {
    unsafe {
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
        RunTasks();
        if ((((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0)
            && ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 16777216u32)
                != 0))
            && ((RecordedBattle_CanStopPlayback()) != 0)
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                (({
                    let __v1 = 5u8;
                    ((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).write(__v1);
                    __v1
                }) as u16),
            );
            ResetPaletteFadeControl();
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
            SetMainCallback2(Some(CB2_QuitRecordedBattle));
        }
    }
}
pub(crate) unsafe extern "C" fn FreeRestoreBattleData() {
    unsafe {
        (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(
            ((&raw mut gPreBattleCallback1)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
        (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
        crate::c::bf_write(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            (0u8) as i32,
        );
        ZeroEnemyPartyMons();
        m4aSongNumStop(90u16);
        FreeMonSpritesGfx();
        FreeBattleSpritesData();
        FreeBattleResources();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_QuitRecordedBattle() {
    unsafe {
        UpdatePaletteFade();
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            m4aMPlayStop((&raw mut gMPlayInfo_SE1).cast::<u8>());
            m4aMPlayStop((&raw mut gMPlayInfo_SE2).cast::<u8>());
            FreeRestoreBattleData();
            FreeAllWindowBuffers();
            SetMainCallback2(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnusedBattleInit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_UnusedBattleInit_Main));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnusedBattleInit_Main(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut arr: *mut u16 = ((&raw mut gDecompressionBuffer).cast::<u8>()).cast::<u16>();
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(641i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p3).write(((__p3).read()).wrapping_sub(1));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    == 0i32
                {
                    let mut i: i32 = 0i32;
                    let mut r2: i32 = 0i32;
                    let mut r0: i32 = 0i32;
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(2i16);
                    r2 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_add(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                                .read()) as i32)
                                .wrapping_mul(32i32),
                        );
                    r0 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_sub(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                                .read()) as i32)
                                .wrapping_mul(32i32),
                        );
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 29i32) {
                                break 'l2;
                            }
                            'l3: {
                                ((arr).wrapping_offset(((r2).wrapping_add(i)) as isize))
                                    .write(61u16);
                                ((arr).wrapping_offset(((r0).wrapping_add(i)) as isize))
                                    .write(61u16);
                            }
                            i = (i).wrapping_add(2i32);
                        }
                    }
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        == 21i32
                    {
                        let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                            .write(32i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p6).write(((__p6).read()).wrapping_sub(1));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    == 20i32
                {
                    SetMainCallback2(Some(CB2_InitBattle));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateNPCTrainerParty(
    party: *mut u8,
    trainerNum: u16,
    firstTrainer: u8,
) -> u8 {
    unsafe {
        let mut party = party;
        let mut trainerNum = trainerNum;
        let mut firstTrainer = firstTrainer;
        let mut nameHash: u32 = 0u32;
        let mut personalityValue: u32 = 0u32;
        let mut fixedIV: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut monsCount: u8 = 0u8;
        if ((trainerNum) as i32) == 1024i32 {
            return 0u8;
        }
        if ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 8u32) != 0)
            && (!((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 71239936u32)
                != 0))
        {
            if ((firstTrainer) as i32) == 1i32 {
                ZeroEnemyPartyMons();
            }
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 32768u32) != 0 {
                if ((((((&raw mut gTrainers).cast::<u8>())
                    .wrapping_offset(((trainerNum) as i32) as isize * 40))
                .wrapping_add(32))
                .read()) as i32)
                    > crate::c::div_i32(6i32, 2i32)
                {
                    monsCount = ((crate::c::div_i32(6i32, 2i32)) as u8);
                } else {
                    monsCount = ((((&raw mut gTrainers).cast::<u8>())
                        .wrapping_offset(((trainerNum) as i32) as isize * 40))
                    .wrapping_add(32))
                    .read();
                }
            } else {
                monsCount = ((((&raw mut gTrainers).cast::<u8>())
                    .wrapping_offset(((trainerNum) as i32) as isize * 40))
                .wrapping_add(32))
                .read();
            }
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((monsCount) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((&raw mut gTrainers).cast::<u8>())
                            .wrapping_offset(((trainerNum) as i32) as isize * 40))
                        .wrapping_add(24))
                        .read()) as i32)
                            == 1i32
                        {
                            personalityValue = 128u32;
                        } else {
                            if (((((((&raw mut gTrainers).cast::<u8>())
                                .wrapping_offset(((trainerNum) as i32) as isize * 40))
                            .wrapping_add(2))
                            .read()) as i32)
                                & 128i32)
                                != 0
                            {
                                personalityValue = 120u32;
                            } else {
                                personalityValue = 136u32;
                            }
                        }
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(((((((((&raw mut gTrainers).cast::<u8>())
                                    .wrapping_offset(((trainerNum) as i32) as isize * 40))
                                .wrapping_add(4))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .read()) as i32)
                                    != 255i32)
                                {
                                    break 'l3;
                                }
                                'l4: {
                                    nameHash = (nameHash).wrapping_add(
                                        ((((((((&raw mut gTrainers).cast::<u8>())
                                            .wrapping_offset(
                                                ((trainerNum) as i32) as isize * 40,
                                            ))
                                        .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as u32),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        'l5: {
                            let __sw1 = (((((&raw mut gTrainers).cast::<u8>())
                                .wrapping_offset(((trainerNum) as i32) as isize * 40))
                            .read()) as i32);
                            if __sw1 == 0i32 {
                                {
                                    let mut partyData: *mut u8 = (((((&raw mut gTrainers)
                                        .cast::<u8>())
                                    .wrapping_offset(((trainerNum) as i32) as isize * 40))
                                    .wrapping_add(36))
                                    .cast::<*mut u8>())
                                    .read();
                                    {
                                        j = 0i32;
                                        'l6: loop {
                                            if !((((((((&raw mut gSpeciesNames).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((partyData)
                                                        .wrapping_offset((i) as isize * 8))
                                                    .wrapping_add(4)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 11,
                                                ))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                                != 255i32)
                                            {
                                                break 'l6;
                                            }
                                            'l7: {
                                                nameHash = (nameHash).wrapping_add(
                                                    (((((((&raw mut gSpeciesNames).cast::<u8>())
                                                        .wrapping_offset(
                                                            (((((partyData).wrapping_offset(
                                                                (i) as isize * 8,
                                                            ))
                                                            .wrapping_add(4)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 11,
                                                        ))
                                                    .cast::<u8>())
                                                    .wrapping_offset((j) as isize))
                                                    .read())
                                                        as u32),
                                                );
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                    personalityValue =
                                        (personalityValue).wrapping_add((nameHash << 8));
                                    fixedIV = ((crate::c::div_i32(
                                        (((((partyData).wrapping_offset((i) as isize * 8))
                                            .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_mul(31i32),
                                        255i32,
                                    )) as u8);
                                    CreateMon(
                                        (party).wrapping_offset((i) as isize * 100),
                                        (((partyData).wrapping_offset((i) as isize * 8))
                                            .wrapping_add(4)
                                            .cast::<u16>())
                                        .read(),
                                        (((partyData).wrapping_offset((i) as isize * 8))
                                            .wrapping_add(2))
                                        .read(),
                                        fixedIV,
                                        1u8,
                                        personalityValue,
                                        2u8,
                                        0u32,
                                    );
                                    break 'l5;
                                }
                            }
                            if __sw1 == 1i32 {
                                {
                                    let mut partyData: *mut u8 = (((((&raw mut gTrainers)
                                        .cast::<u8>())
                                    .wrapping_offset(((trainerNum) as i32) as isize * 40))
                                    .wrapping_add(36))
                                    .cast::<*mut u8>())
                                    .read();
                                    {
                                        j = 0i32;
                                        'l8: loop {
                                            if !((((((((&raw mut gSpeciesNames).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((partyData)
                                                        .wrapping_offset((i) as isize * 16))
                                                    .wrapping_add(4)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 11,
                                                ))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                                != 255i32)
                                            {
                                                break 'l8;
                                            }
                                            'l9: {
                                                nameHash = (nameHash).wrapping_add(
                                                    (((((((&raw mut gSpeciesNames).cast::<u8>())
                                                        .wrapping_offset(
                                                            (((((partyData).wrapping_offset(
                                                                (i) as isize * 16,
                                                            ))
                                                            .wrapping_add(4)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 11,
                                                        ))
                                                    .cast::<u8>())
                                                    .wrapping_offset((j) as isize))
                                                    .read())
                                                        as u32),
                                                );
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                    personalityValue =
                                        (personalityValue).wrapping_add((nameHash << 8));
                                    fixedIV = ((crate::c::div_i32(
                                        (((((partyData).wrapping_offset((i) as isize * 16))
                                            .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_mul(31i32),
                                        255i32,
                                    )) as u8);
                                    CreateMon(
                                        (party).wrapping_offset((i) as isize * 100),
                                        (((partyData).wrapping_offset((i) as isize * 16))
                                            .wrapping_add(4)
                                            .cast::<u16>())
                                        .read(),
                                        (((partyData).wrapping_offset((i) as isize * 16))
                                            .wrapping_add(2))
                                        .read(),
                                        fixedIV,
                                        1u8,
                                        personalityValue,
                                        2u8,
                                        0u32,
                                    );
                                    {
                                        j = 0i32;
                                        'l10: loop {
                                            if !(j < 4i32) {
                                                break 'l10;
                                            }
                                            'l11: {
                                                SetMonData(
                                                    (party).wrapping_offset((i) as isize * 100),
                                                    (13i32).wrapping_add(j),
                                                    (((((partyData)
                                                        .wrapping_offset((i) as isize * 16))
                                                    .wrapping_add(6))
                                                    .cast::<u16>())
                                                    .wrapping_offset((j) as isize))
                                                    .cast::<u8>(),
                                                );
                                                SetMonData(
                                                    (party).wrapping_offset((i) as isize * 100),
                                                    (17i32).wrapping_add(j),
                                                    (((&raw mut gBattleMoves).cast::<u8>())
                                                        .wrapping_offset(
                                                            (((((((partyData).wrapping_offset(
                                                                (i) as isize * 16,
                                                            ))
                                                            .wrapping_add(6))
                                                            .cast::<u16>())
                                                            .wrapping_offset((j) as isize))
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 12,
                                                        ))
                                                    .wrapping_add(4),
                                                );
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                    break 'l5;
                                }
                            }
                            if __sw1 == 2i32 {
                                {
                                    let mut partyData: *mut u8 = (((((&raw mut gTrainers)
                                        .cast::<u8>())
                                    .wrapping_offset(((trainerNum) as i32) as isize * 40))
                                    .wrapping_add(36))
                                    .cast::<*mut u8>())
                                    .read();
                                    {
                                        j = 0i32;
                                        'l12: loop {
                                            if !((((((((&raw mut gSpeciesNames).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((partyData)
                                                        .wrapping_offset((i) as isize * 8))
                                                    .wrapping_add(4)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 11,
                                                ))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                                != 255i32)
                                            {
                                                break 'l12;
                                            }
                                            'l13: {
                                                nameHash = (nameHash).wrapping_add(
                                                    (((((((&raw mut gSpeciesNames).cast::<u8>())
                                                        .wrapping_offset(
                                                            (((((partyData).wrapping_offset(
                                                                (i) as isize * 8,
                                                            ))
                                                            .wrapping_add(4)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 11,
                                                        ))
                                                    .cast::<u8>())
                                                    .wrapping_offset((j) as isize))
                                                    .read())
                                                        as u32),
                                                );
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                    personalityValue =
                                        (personalityValue).wrapping_add((nameHash << 8));
                                    fixedIV = ((crate::c::div_i32(
                                        (((((partyData).wrapping_offset((i) as isize * 8))
                                            .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_mul(31i32),
                                        255i32,
                                    )) as u8);
                                    CreateMon(
                                        (party).wrapping_offset((i) as isize * 100),
                                        (((partyData).wrapping_offset((i) as isize * 8))
                                            .wrapping_add(4)
                                            .cast::<u16>())
                                        .read(),
                                        (((partyData).wrapping_offset((i) as isize * 8))
                                            .wrapping_add(2))
                                        .read(),
                                        fixedIV,
                                        1u8,
                                        personalityValue,
                                        2u8,
                                        0u32,
                                    );
                                    SetMonData(
                                        (party).wrapping_offset((i) as isize * 100),
                                        12i32,
                                        (((partyData).wrapping_offset((i) as isize * 8))
                                            .wrapping_add(6)
                                            .cast::<u16>())
                                        .cast::<u8>(),
                                    );
                                    break 'l5;
                                }
                            }
                            if __sw1 == 3i32 {
                                {
                                    let mut partyData: *mut u8 = (((((&raw mut gTrainers)
                                        .cast::<u8>())
                                    .wrapping_offset(((trainerNum) as i32) as isize * 40))
                                    .wrapping_add(36))
                                    .cast::<*mut u8>())
                                    .read();
                                    {
                                        j = 0i32;
                                        'l14: loop {
                                            if !((((((((&raw mut gSpeciesNames).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((partyData)
                                                        .wrapping_offset((i) as isize * 16))
                                                    .wrapping_add(4)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 11,
                                                ))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                                != 255i32)
                                            {
                                                break 'l14;
                                            }
                                            'l15: {
                                                nameHash = (nameHash).wrapping_add(
                                                    (((((((&raw mut gSpeciesNames).cast::<u8>())
                                                        .wrapping_offset(
                                                            (((((partyData).wrapping_offset(
                                                                (i) as isize * 16,
                                                            ))
                                                            .wrapping_add(4)
                                                            .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 11,
                                                        ))
                                                    .cast::<u8>())
                                                    .wrapping_offset((j) as isize))
                                                    .read())
                                                        as u32),
                                                );
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                    personalityValue =
                                        (personalityValue).wrapping_add((nameHash << 8));
                                    fixedIV = ((crate::c::div_i32(
                                        (((((partyData).wrapping_offset((i) as isize * 16))
                                            .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_mul(31i32),
                                        255i32,
                                    )) as u8);
                                    CreateMon(
                                        (party).wrapping_offset((i) as isize * 100),
                                        (((partyData).wrapping_offset((i) as isize * 16))
                                            .wrapping_add(4)
                                            .cast::<u16>())
                                        .read(),
                                        (((partyData).wrapping_offset((i) as isize * 16))
                                            .wrapping_add(2))
                                        .read(),
                                        fixedIV,
                                        1u8,
                                        personalityValue,
                                        2u8,
                                        0u32,
                                    );
                                    SetMonData(
                                        (party).wrapping_offset((i) as isize * 100),
                                        12i32,
                                        (((partyData).wrapping_offset((i) as isize * 16))
                                            .wrapping_add(6)
                                            .cast::<u16>())
                                        .cast::<u8>(),
                                    );
                                    {
                                        j = 0i32;
                                        'l16: loop {
                                            if !(j < 4i32) {
                                                break 'l16;
                                            }
                                            'l17: {
                                                SetMonData(
                                                    (party).wrapping_offset((i) as isize * 100),
                                                    (13i32).wrapping_add(j),
                                                    (((((partyData)
                                                        .wrapping_offset((i) as isize * 16))
                                                    .wrapping_add(8))
                                                    .cast::<u16>())
                                                    .wrapping_offset((j) as isize))
                                                    .cast::<u8>(),
                                                );
                                                SetMonData(
                                                    (party).wrapping_offset((i) as isize * 100),
                                                    (17i32).wrapping_add(j),
                                                    (((&raw mut gBattleMoves).cast::<u8>())
                                                        .wrapping_offset(
                                                            (((((((partyData).wrapping_offset(
                                                                (i) as isize * 16,
                                                            ))
                                                            .wrapping_add(8))
                                                            .cast::<u16>())
                                                            .wrapping_offset((j) as isize))
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 12,
                                                        ))
                                                    .wrapping_add(4),
                                                );
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                    break 'l5;
                                }
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            let __p2 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
            (__p2).write(
                ((__p2).read()
                    | ((((((&raw mut gTrainers).cast::<u8>())
                        .wrapping_offset(((trainerNum) as i32) as isize * 40))
                    .wrapping_add(24))
                    .read()) as u32)),
            );
        }
        return ((((&raw mut gTrainers).cast::<u8>())
            .wrapping_offset(((trainerNum) as i32) as isize * 40))
        .wrapping_add(32))
        .read();
    }
}
pub(crate) unsafe extern "C" fn HBlankCB_Battle() {
    unsafe {
        if (((((67108870i32) as usize as *mut u16).read_volatile()) as i32) < 160i32)
            && (((((67108870i32) as usize as *mut u16).read_volatile()) as i32) >= 111i32)
        {
            SetGpuReg(8u8, 38912u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn VBlankCB_Battle() {
    unsafe {
        if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 20906242u32) != 0) {
            Random();
        }
        SetGpuReg(
            16u8,
            ((&raw mut gBattle_BG0_X).cast::<u8>().cast::<u16>()).read(),
        );
        SetGpuReg(
            18u8,
            ((&raw mut gBattle_BG0_Y).cast::<u8>().cast::<u16>()).read(),
        );
        SetGpuReg(
            20u8,
            ((&raw mut gBattle_BG1_X).cast::<u8>().cast::<u16>()).read(),
        );
        SetGpuReg(
            22u8,
            ((&raw mut gBattle_BG1_Y).cast::<u8>().cast::<u16>()).read(),
        );
        SetGpuReg(
            24u8,
            ((&raw mut gBattle_BG2_X).cast::<u8>().cast::<u16>()).read(),
        );
        SetGpuReg(
            26u8,
            ((&raw mut gBattle_BG2_Y).cast::<u8>().cast::<u16>()).read(),
        );
        SetGpuReg(
            28u8,
            ((&raw mut gBattle_BG3_X).cast::<u8>().cast::<u16>()).read(),
        );
        SetGpuReg(
            30u8,
            ((&raw mut gBattle_BG3_Y).cast::<u8>().cast::<u16>()).read(),
        );
        SetGpuReg(
            64u8,
            ((&raw mut gBattle_WIN0H).cast::<u8>().cast::<u16>()).read(),
        );
        SetGpuReg(
            68u8,
            ((&raw mut gBattle_WIN0V).cast::<u8>().cast::<u16>()).read(),
        );
        SetGpuReg(
            66u8,
            ((&raw mut gBattle_WIN1H).cast::<u8>().cast::<u16>()).read(),
        );
        SetGpuReg(
            70u8,
            ((&raw mut gBattle_WIN1V).cast::<u8>().cast::<u16>()).read(),
        );
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        ScanlineEffect_InitHBlankDmaTransfer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_VsLetterDummy(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_VsLetter(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_add(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            & 65280i32)
                            >> 8),
                    )) as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_sub(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            & 65280i32)
                            >> 8),
                    )) as i16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(384i32)) as i16));
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            FreeSpriteTilesByTag(10000u16);
            FreeSpritePaletteByTag(10000u16);
            FreeSpriteOamMatrix(sprite);
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_VsLetterInit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        StartSpriteAffineAnim(sprite, 1u8);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_VsLetter));
        PlaySE(104u16);
    }
}
pub(crate) unsafe extern "C" fn BufferPartyVsScreenHealth_AtEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut party1: *mut u8 = core::ptr::null_mut();
        let mut party2: *mut u8 = core::ptr::null_mut();
        let mut multiplayerId: u8 =
            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37)).read();
        let mut flags: u32 = 0u32;
        let mut i: i32 = 0i32;
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 64u32) != 0 {
            'l1: {
                let __sw1 = ((((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_offset(((multiplayerId) as i32) as isize * 28))
                .wrapping_add(24)
                .cast::<u16>())
                .read()) as i32);
                if __sw1 == 0i32 || __sw1 == 2i32 {
                    party1 = (&raw mut gPlayerParty).cast::<u8>();
                    party2 = (&raw mut gEnemyParty).cast::<u8>();
                    break 'l1;
                }
                if __sw1 == 1i32 || __sw1 == 3i32 {
                    party1 = (&raw mut gEnemyParty).cast::<u8>();
                    party2 = (&raw mut gPlayerParty).cast::<u8>();
                    break 'l1;
                }
            }
        } else {
            party1 = (&raw mut gPlayerParty).cast::<u8>();
            party2 = (&raw mut gEnemyParty).cast::<u8>();
        }
        flags = 0u32;
        {
            i = 0i32;
            'l2: loop {
                if !(i < 6i32) {
                    break 'l2;
                }
                'l3: {
                    let mut species: u16 =
                        ((GetMonData2((party1).wrapping_offset((i) as isize * 100), 65i32)) as u16);
                    let mut hp: u16 =
                        ((GetMonData2((party1).wrapping_offset((i) as isize * 100), 57i32)) as u16);
                    let mut status: u32 =
                        GetMonData2((party1).wrapping_offset((i) as isize * 100), 55i32);
                    if ((species) as i32) == 0i32 {
                        break 'l3;
                    }
                    if ((((species) as i32) != 412i32) && (((hp) as i32) != 0i32))
                        && (status == 0u32)
                    {
                        flags = (flags
                            | ((crate::c::shl_i32(1i32, (((i).wrapping_mul(2i32)) as u32)))
                                as u32));
                    }
                    if ((species) as i32) == 0i32 {
                        break 'l3;
                    }
                    if (((hp) as i32) != 0i32)
                        && ((((species) as i32) == 412i32) || (status != 0u32))
                    {
                        flags = (flags
                            | ((crate::c::shl_i32(2i32, (((i).wrapping_mul(2i32)) as u32)))
                                as u32));
                    }
                    if ((species) as i32) == 0i32 {
                        break 'l3;
                    }
                    if (((species) as i32) != 412i32) && (((hp) as i32) == 0i32) {
                        flags = (flags
                            | ((crate::c::shl_i32(3i32, (((i).wrapping_mul(2i32)) as u32)))
                                as u32));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((flags) as i16));
        flags = 0u32;
        {
            i = 0i32;
            'l4: loop {
                if !(i < 6i32) {
                    break 'l4;
                }
                'l5: {
                    let mut species: u16 =
                        ((GetMonData2((party2).wrapping_offset((i) as isize * 100), 65i32)) as u16);
                    let mut hp: u16 =
                        ((GetMonData2((party2).wrapping_offset((i) as isize * 100), 57i32)) as u16);
                    let mut status: u32 =
                        GetMonData2((party2).wrapping_offset((i) as isize * 100), 55i32);
                    if ((species) as i32) == 0i32 {
                        break 'l5;
                    }
                    if ((((species) as i32) != 412i32) && (((hp) as i32) != 0i32))
                        && (status == 0u32)
                    {
                        flags = (flags
                            | ((crate::c::shl_i32(1i32, (((i).wrapping_mul(2i32)) as u32)))
                                as u32));
                    }
                    if ((species) as i32) == 0i32 {
                        break 'l5;
                    }
                    if (((hp) as i32) != 0i32)
                        && ((((species) as i32) == 412i32) || (status != 0u32))
                    {
                        flags = (flags
                            | ((crate::c::shl_i32(2i32, (((i).wrapping_mul(2i32)) as u32)))
                                as u32));
                    }
                    if ((species) as i32) == 0i32 {
                        break 'l5;
                    }
                    if (((species) as i32) != 412i32) && (((hp) as i32) == 0i32) {
                        flags = (flags
                            | ((crate::c::shl_i32(3i32, (((i).wrapping_mul(2i32)) as u32)))
                                as u32));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((flags) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitEndLinkBattle() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut taskId: u8 = 0u8;
        SetHBlankCallback(None);
        SetVBlankCallback(None);
        let __p1 = (&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>();
        (__p1).write(((__p1).read() & 4294967263u32));
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 4129024u32) != 0 {
            SetMainCallback2(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
            FreeBattleResources();
            FreeBattleSpritesData();
            FreeMonSpritesGfx();
        } else {
            'l1: loop {
                'l2: {
                    {
                        let mut tmp: u32 = 0u32;
                        (&raw mut tmp).write_volatile(0u32);
                        'l3: loop {
                            'l4: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    ((100663296i32) as usize as *mut u8),
                                    ((83886080i32
                                        | (crate::c::div_i32(
                                            98304i32,
                                            crate::c::div_i32(32i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l3;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
            SetGpuReg(76u8, 0u16);
            SetGpuReg(64u8, 240u16);
            SetGpuReg(
                68u8,
                (((crate::c::div_i32(160i32, 2i32) << 8)
                    | (crate::c::div_i32(160i32, 2i32)).wrapping_add(1i32))
                    as u16),
            );
            SetGpuReg(72u8, 0u16);
            SetGpuReg(74u8, 0u16);
            ((&raw mut gBattle_WIN0H).cast::<u8>().cast::<u16>()).write(240u16);
            ((&raw mut gBattle_WIN0V).cast::<u8>().cast::<u16>()).write(
                (((crate::c::div_i32(160i32, 2i32) << 8)
                    | (crate::c::div_i32(160i32, 2i32)).wrapping_add(1i32))
                    as u16),
            );
            ScanlineEffect_Clear();
            i = 0i32;
            'l5: loop {
                if !(i < 80i32) {
                    break 'l5;
                }
                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                    .wrapping_offset((i) as isize))
                .write(240u16);
                (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                    .cast::<u16>())
                .wrapping_offset((i) as isize))
                .write(240u16);
                i = (i).wrapping_add(1);
            }
            'l6: loop {
                if !(i < 160i32) {
                    break 'l6;
                }
                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                    .wrapping_offset((i) as isize))
                .write(65296u16);
                (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                    .cast::<u16>())
                .wrapping_offset((i) as isize))
                .write(65296u16);
                i = (i).wrapping_add(1);
            }
            ResetPaletteFade();
            ((&raw mut gBattle_BG0_X).cast::<u8>().cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG0_Y).cast::<u8>().cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG1_X).cast::<u8>().cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG1_Y).cast::<u8>().cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG2_X).cast::<u8>().cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG2_Y).cast::<u8>().cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG3_X).cast::<u8>().cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG3_Y).cast::<u8>().cast::<u16>()).write(0u16);
            InitBattleBgsVideo();
            LoadCompressedPalette(
                ((&raw mut gBattleTextboxPalette).cast::<u32>()).cast::<u32>(),
                0u16,
                64u16,
            );
            LoadBattleMenuWindowGfx();
            ResetSpriteData();
            ResetTasks();
            DrawBattleEntryBackground();
            SetGpuReg(74u8, 55u16);
            FreeAllSpritePalettes();
            ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(4u8);
            SetVBlankCallback(Some(VBlankCB_Battle));
            taskId = CreateTask(Some(InitLinkBattleVsScreen), 0u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(270i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(90i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(1i16);
            BufferPartyVsScreenHealth_AtEnd(taskId);
            SetMainCallback2(Some(CB2_EndLinkBattle));
            (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_EndLinkBattle() {
    unsafe {
        EndLinkBattleInSteps();
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
        RunTasks();
    }
}
pub(crate) unsafe extern "C" fn EndLinkBattleInSteps() {
    unsafe {
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 =
                (((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).read()) as i32);
            if __sw1 == 0i32 {
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                    .write(255u8);
                let __p2 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1);
                    let __t4 = ((__p3).read()).wrapping_sub(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == 0i32
                {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    let __p5 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let mut battlerCount: u8 = 0u8;
                    crate::c::bf_write(
                        ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                        2,
                        1,
                        (RecordedBattle_GetFrontierPassFlag()) as i32,
                    );
                    if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 64u32)
                        != 0
                    {
                        battlerCount = 4u8;
                    } else {
                        battlerCount = 2u8;
                    }
                    {
                        i = 0i32;
                        'l2: loop {
                            if !((i < ((battlerCount) as i32))
                                && ((((((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset((i) as isize * 28))
                                .cast::<u16>())
                                .read()) as i32)
                                    & 255i32)
                                    == 3i32))
                            {
                                break 'l2;
                            }
                            'l3: {}
                            i = (i).wrapping_add(1);
                        }
                    }
                    if (!((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1629),
                        3,
                        1,
                        false,
                    ) as u8)
                        != 0))
                        && (i == ((battlerCount) as i32))
                    {
                        if (FlagGet(2258u16)) != 0 {
                            FreeAllWindowBuffers();
                            SetMainCallback2(Some(CB2_InitAskRecordBattle));
                        } else {
                            if !((crate::c::bf_read(
                                ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                                2,
                                1,
                                false,
                            ) as u8)
                                != 0)
                            {
                                SetMainCallback2(
                                    (((&raw mut gMain).cast::<u8>())
                                        .wrapping_add(8)
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                    .read(),
                                );
                                FreeBattleResources();
                                FreeBattleSpritesData();
                                FreeMonSpritesGfx();
                            } else {
                                if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read())
                                    as i32)
                                    == 0i32
                                {
                                    CreateTask(Some(Task_ReconnectWithLinkPlayers), 5u8);
                                    let __p6 =
                                        ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                                    (__p6).write(((__p6).read()).wrapping_add(1));
                                } else {
                                    let __p7 =
                                        ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                                    (__p7).write(((__p7).read()).wrapping_add(1));
                                }
                            }
                        }
                    } else {
                        SetMainCallback2(
                            (((&raw mut gMain).cast::<u8>())
                                .wrapping_add(8)
                                .cast::<Option<unsafe extern "C" fn()>>())
                            .read(),
                        );
                        FreeBattleResources();
                        FreeBattleSpritesData();
                        FreeMonSpritesGfx();
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                'l4: loop {
                    'l5: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
                            'l6: loop {
                                'l7: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((100663296i32) as usize as *mut u8),
                                        ((83886080i32
                                            | (crate::c::div_i32(
                                                98304i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l6;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l4;
                    }
                }
                {
                    i = 0i32;
                    'l8: loop {
                        if !(i < 2i32) {
                            break 'l8;
                        }
                        'l9: {
                            LoadChosenBattleElement(((i) as u8));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p8 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p9 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((FuncIsActiveTask(Some(Task_ReconnectWithLinkPlayers))) != 0) {
                    let __p10 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((IsLinkTaskFinished()) as i32) == 1i32 {
                    SetLinkStandbyCallback();
                    BattlePutTextOnWindow((&raw mut gText_LinkStandby3).cast::<u8>(), 0u8);
                    let __p11 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((IsTextPrinterActive(0u8)) != 0) {
                    if ((IsLinkTaskFinished()) as i32) == 1i32 {
                        let __p12 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                        (__p12).write(((__p12).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if !((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0) {
                    SetCloseLinkCallback();
                }
                let __p13 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                if ((!((crate::c::bf_read(
                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                    2,
                    1,
                    false,
                ) as u8)
                    != 0))
                    || ((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0))
                    || (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32)
                        != 1i32)
                {
                    crate::c::bf_write(
                        ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                        2,
                        1,
                        (0u8) as i32,
                    );
                    SetMainCallback2(
                        (((&raw mut gMain).cast::<u8>())
                            .wrapping_add(8)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                    FreeBattleResources();
                    FreeBattleSpritesData();
                    FreeMonSpritesGfx();
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleBgTemplateData(arrayId: u8, caseId: u8) -> u32 {
    unsafe {
        let mut arrayId = arrayId;
        let mut caseId = caseId;
        let mut ret: u32 = 0u32;
        'l1: {
            let __sw1 = ((caseId) as i32);
            if __sw1 == 0i32 {
                ret = ((crate::c::bf_read(
                    (((&raw mut gBattleBgTemplates).cast::<u8>())
                        .wrapping_offset(((arrayId) as i32) as isize * 4))
                    .wrapping_add(0),
                    0,
                    2,
                    false,
                ) as u16) as u32);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ret = ((crate::c::bf_read(
                    (((&raw mut gBattleBgTemplates).cast::<u8>())
                        .wrapping_offset(((arrayId) as i32) as isize * 4))
                    .wrapping_add(0),
                    2,
                    2,
                    false,
                ) as u16) as u32);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ret = ((crate::c::bf_read(
                    (((&raw mut gBattleBgTemplates).cast::<u8>())
                        .wrapping_offset(((arrayId) as i32) as isize * 4))
                    .wrapping_add(0),
                    4,
                    5,
                    false,
                ) as u16) as u32);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ret = ((crate::c::bf_read(
                    (((&raw mut gBattleBgTemplates).cast::<u8>())
                        .wrapping_offset(((arrayId) as i32) as isize * 4))
                    .wrapping_add(1),
                    1,
                    2,
                    false,
                ) as u16) as u32);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ret = ((crate::c::bf_read(
                    (((&raw mut gBattleBgTemplates).cast::<u8>())
                        .wrapping_offset(((arrayId) as i32) as isize * 4))
                    .wrapping_add(1),
                    3,
                    1,
                    false,
                ) as u16) as u32);
                break 'l1;
            }
            if __sw1 == 5i32 {
                ret = ((crate::c::bf_read(
                    (((&raw mut gBattleBgTemplates).cast::<u8>())
                        .wrapping_offset(((arrayId) as i32) as isize * 4))
                    .wrapping_add(1),
                    4,
                    2,
                    false,
                ) as u16) as u32);
                break 'l1;
            }
            if __sw1 == 6i32 {
                ret = ((crate::c::bf_read(
                    (((&raw mut gBattleBgTemplates).cast::<u8>())
                        .wrapping_offset(((arrayId) as i32) as isize * 4))
                    .wrapping_add(2),
                    0,
                    10,
                    false,
                ) as u16) as u32);
                break 'l1;
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn CB2_InitAskRecordBattle() {
    unsafe {
        let mut i: i32 = 0i32;
        SetHBlankCallback(None);
        SetVBlankCallback(None);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((100663296i32) as usize as *mut u8),
                                ((83886080i32
                                    | (crate::c::div_i32(98304i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        ResetPaletteFade();
        ((&raw mut gBattle_BG0_X).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_X).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_X).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG2_Y).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG3_X).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG3_Y).cast::<u8>().cast::<u16>()).write(0u16);
        InitBattleBgsVideo();
        SetGpuReg(0u8, 4160u16);
        LoadBattleMenuWindowGfx();
        {
            i = 0i32;
            'l5: loop {
                if !(i < 2i32) {
                    break 'l5;
                }
                'l6: {
                    LoadChosenBattleElement(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
        ResetSpriteData();
        ResetTasks();
        FreeAllSpritePalettes();
        ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(4u8);
        SetVBlankCallback(Some(VBlankCB_Battle));
        SetMainCallback2(Some(CB2_AskRecordBattle));
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn CB2_AskRecordBattle() {
    unsafe {
        AskRecordBattle();
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
        RunTasks();
    }
}
pub(crate) unsafe extern "C" fn AskRecordBattle() {
    unsafe {
        'l1: {
            let __sw1 =
                (((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).read()) as i32);
            if __sw1 == 0i32 {
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                let __p2 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((crate::c::bf_read(
                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                    2,
                    1,
                    false,
                ) as u8)
                    != 0)
                    && (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32)
                        == 0i32)
                {
                    CreateTask(Some(Task_ReconnectWithLinkPlayers), 5u8);
                }
                let __p3 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((FuncIsActiveTask(Some(Task_ReconnectWithLinkPlayers))) != 0) {
                    let __p4 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    BattlePutTextOnWindow((&raw mut gText_RecordBattleToPass).cast::<u8>(), 0u8);
                    let __p5 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsTextPrinterActive(0u8)) != 0) {
                    HandleBattleWindow(24u8, 8u8, 29u8, 13u8, 0u8);
                    BattlePutTextOnWindow((&raw mut gText_BattleYesNoChoice).cast::<u8>(), 12u8);
                    ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1))
                    .write(1u8);
                    BattleCreateYesNoCursorAt(1u8);
                    let __p6 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    if ((((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        != 0i32
                    {
                        PlaySE(5u16);
                        BattleDestroyYesNoCursorAt(
                            ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(1))
                            .read(),
                        );
                        ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(1))
                        .write(0u8);
                        BattleCreateYesNoCursorAt(0u8);
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        if ((((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(1))
                        .read()) as i32)
                            == 0i32
                        {
                            PlaySE(5u16);
                            BattleDestroyYesNoCursorAt(
                                ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(1))
                                .read(),
                            );
                            ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(1))
                            .write(1u8);
                            BattleCreateYesNoCursorAt(1u8);
                        }
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 1i32)
                            != 0
                        {
                            PlaySE(5u16);
                            if ((((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(1))
                            .read()) as i32)
                                == 0i32
                            {
                                HandleBattleWindow(24u8, 8u8, 29u8, 13u8, 1u8);
                                ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(1))
                                .write(((MoveRecordedBattleToSaveData()) as u8));
                                (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                    .write(10u8);
                            } else {
                                let __p7 =
                                    ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                                (__p7).write(((__p7).read()).wrapping_add(1));
                            }
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 2i32)
                                != 0
                            {
                                PlaySE(5u16);
                                let __p8 =
                                    ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                                (__p8).write(((__p8).read()).wrapping_add(1));
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((IsLinkTaskFinished()) as i32) == 1i32 {
                    HandleBattleWindow(24u8, 8u8, 29u8, 13u8, 1u8);
                    if (crate::c::bf_read(
                        ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                        2,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        SetLinkStandbyCallback();
                        BattlePutTextOnWindow((&raw mut gText_LinkStandby3).cast::<u8>(), 0u8);
                    }
                    let __p9 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if (({
                    let __p10 = (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1);
                    let __t11 = ((__p10).read()).wrapping_sub(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    == 0i32
                {
                    if ((crate::c::bf_read(
                        ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                        2,
                        1,
                        false,
                    ) as u8)
                        != 0)
                        && (!((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0))
                    {
                        SetCloseLinkCallback();
                    }
                    let __p12 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if ((!((crate::c::bf_read(
                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                    2,
                    1,
                    false,
                ) as u8)
                    != 0))
                    || ((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0))
                    || (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32)
                        != 1i32)
                {
                    crate::c::bf_write(
                        ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                        2,
                        1,
                        (0u8) as i32,
                    );
                    if !((crate::c::bf_read(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                        7,
                        1,
                        false,
                    ) as u16)
                        != 0)
                    {
                        SetMainCallback2(
                            (((&raw mut gMain).cast::<u8>())
                                .wrapping_add(8)
                                .cast::<Option<unsafe extern "C" fn()>>())
                            .read(),
                        );
                        FreeBattleResources();
                        FreeBattleSpritesData();
                        FreeMonSpritesGfx();
                    }
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                if ((((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(1))
                .read()) as i32)
                    == 1i32
                {
                    PlaySE(55u16);
                    BattleStringExpandPlaceholdersToDisplayedString(
                        (&raw mut gText_BattleRecordedOnPass).cast::<u8>(),
                    );
                    BattlePutTextOnWindow(
                        ((&raw mut gDisplayedStringBattle).cast::<u8>()).cast::<u8>(),
                        0u8,
                    );
                    ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1))
                    .write(128u8);
                    let __p13 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p13).write(((__p13).read()).wrapping_add(1));
                } else {
                    BattleStringExpandPlaceholdersToDisplayedString(
                        (&raw mut BattleFrontier_BattleTowerBattleRoom_Text_RecordCouldntBeSaved)
                            .cast::<u8>(),
                    );
                    BattlePutTextOnWindow(
                        ((&raw mut gDisplayedStringBattle).cast::<u8>()).cast::<u8>(),
                        0u8,
                    );
                    ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1))
                    .write(128u8);
                    let __p14 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if ((((IsLinkTaskFinished()) as i32) == 1i32)
                    && (!((IsTextPrinterActive(0u8)) != 0)))
                    && ((({
                        let __p15 = (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(1);
                        let __t16 = ((__p15).read()).wrapping_sub(1);
                        (__p15).write(__t16);
                        __t16
                    }) as i32)
                        == 0i32)
                {
                    if (crate::c::bf_read(
                        ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                        2,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        SetLinkStandbyCallback();
                        BattlePutTextOnWindow((&raw mut gText_LinkStandby3).cast::<u8>(), 0u8);
                    }
                    let __p17 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                    (__p17).write(((__p17).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 12i32 || __sw1 == 7i32 {
                if !((IsTextPrinterActive(0u8)) != 0) {
                    if (crate::c::bf_read(
                        ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                        2,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        if ((IsLinkTaskFinished()) as i32) == 1i32 {
                            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                            ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(1))
                            .write(32u8);
                            (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                .write(8u8);
                        }
                    } else {
                        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                        ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(1))
                        .write(32u8);
                        (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(8u8);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryCorrectShedinjaLanguage(mon: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut nickname = crate::ffi::Align4([0u8; 11]);
        let mut language: u8 = 1u8;
        if (GetMonData2(mon, 11i32) == 303u32) && (GetMonData2(mon, 3i32) != ((language) as u32)) {
            GetMonData3(mon, 2i32, (&raw mut nickname).cast::<u8>());
            if StringCompareWithoutExtCtrlCodes(
                (&raw mut nickname).cast::<u8>(),
                ((&raw const sText_ShedinjaJpnName).cast::<u8>().cast_mut()).cast::<u8>(),
            ) == 0i32
            {
                SetMonData(mon, 3i32, &raw mut language);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleWindowTemplatePixelWidth(windowsType: u32, tableId: u32) -> u32 {
    unsafe {
        let mut windowsType = windowsType;
        let mut tableId = tableId;
        return (((((((((((&raw mut gBattleWindowTemplates).cast::<*mut u8>())
            .cast::<*mut u8>())
        .wrapping_offset(((windowsType) as i32) as isize))
        .read())
        .wrapping_offset(((tableId) as i32) as isize * 8))
        .wrapping_add(3))
        .read()) as i32)
            .wrapping_mul(8i32)) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_WildMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_MoveWildMonToRight));
        StartSpriteAnimIfDifferent(sprite, 0u8);
        BeginNormalPaletteFade(131072u32, 0i8, 10u8, 10u8, 8456u16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MoveWildMonToRight(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((&raw mut gIntroSlideFlags).cast::<u8>().cast::<u16>()).read()) as i32) & 1i32)
            == 0i32
        {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
            if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) == 0i32 {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_WildMonShowHealthbox));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WildMonShowHealthbox(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            StartHealthboxSlideIn((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8));
            SetHealthboxSpriteVisible(
                ((((&raw mut gHealthboxSpriteIds).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                ))
                .read(),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_WildMonAnimate));
            StartSpriteAnimIfDifferent(sprite, 0u8);
            BeginNormalPaletteFade(131072u32, 0i8, 10u8, 0u8, 8456u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WildMonAnimate(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            BattleAnimateFrontSprite(
                sprite,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
                0u8,
                1u8,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCallbackDummy_2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_InitFlicker(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(6i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Flicker));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Flicker(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p1).write(((__p1).read()).wrapping_sub(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(8i16);
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p2).write(((__p2).read()).wrapping_sub(1));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                == 0i32
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy_2));
                (((&raw mut sFlickerArray).cast::<u8>().cast::<u32>()).cast::<u32>()).write(0u32);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_FaintOpponentMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
        let mut species: u16 = 0u16;
        let mut yOffset: u8 = 0u8;
        if (((((((((&raw mut gBattleSpritesDataPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            species = (((((((&raw mut gBattleSpritesDataPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read();
        } else {
            species =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16);
        }
        GetMonData2(
            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 100,
            ),
            0i32,
        );
        if ((species) as i32) == 201i32 {
            let mut personalityValue: u32 = GetMonData2(
                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u8>().cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 100,
                ),
                0i32,
            );
            let mut unownForm: u16 = ((crate::c::rem_u32(
                (((((personalityValue & 50331648u32) >> 18)
                    | ((personalityValue & 196608u32) >> 12))
                    | ((personalityValue & 768u32) >> 6))
                    | ((personalityValue & 3u32) >> 0)),
                28u32,
            )) as u16);
            let mut unownSpecies: u16 = 0u16;
            if ((unownForm) as i32) == 0i32 {
                unownSpecies = 201u16;
            } else {
                unownSpecies = (((412i32).wrapping_add(((unownForm) as i32))) as u16);
            }
            yOffset = ((((&raw mut gMonFrontPicCoords).cast::<u8>())
                .wrapping_offset(((unownSpecies) as i32) as isize * 4))
            .wrapping_add(1))
            .read();
        } else {
            if ((species) as i32) == 385i32 {
                yOffset = ((((&raw mut gCastformFrontSpriteCoords).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattleMonForms).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 4,
                ))
                .wrapping_add(1))
                .read();
            } else {
                if ((species) as i32) > 412i32 {
                    yOffset = (((&raw mut gMonFrontPicCoords).cast::<u8>()).wrapping_add(1)).read();
                } else {
                    yOffset = ((((&raw mut gMonFrontPicCoords).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 4))
                    .wrapping_add(1))
                    .read();
                }
            }
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write((((8i32).wrapping_sub(crate::c::div_i32(((yOffset) as i32), 8i32))) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_AnimFaintOpponent));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_AnimFaintOpponent(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i32 = 0i32;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(2i16);
            let __p3 = (sprite).wrapping_add(38).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_add(8i32)) as i16));
            if (({
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                let __t5 = ((__p4).read()).wrapping_sub(1);
                (__p4).write(__t5);
                __t5
            }) as i32)
                < 0i32
            {
                FreeSpriteOamMatrix(sprite);
                DestroySprite(sprite);
            } else {
                let mut dst: *mut u8 = ((((((((&raw mut gMonSpritesGfxPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4))
                .cast::<*mut u8>())
                .wrapping_offset(
                    ((GetBattlerPosition(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
                    )) as i32) as isize,
                ))
                .read())
                .wrapping_offset(
                    (((((((&raw mut gBattleMonForms).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        << 11) as isize,
                ))
                .wrapping_offset(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        << 8) as isize,
                );
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i < 256i32) {
                            break 'l1;
                        }
                        'l2: {
                            ({
                                let __t6 = dst;
                                dst = (dst).wrapping_offset(1);
                                __t6
                            })
                            .write(0u8);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                StartSpriteAnim(
                    sprite,
                    ((((&raw mut gBattleMonForms).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                    ))
                    .read(),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_ShowAsMoveTarget(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(8i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_BlinkVisible));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BlinkVisible(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(8i16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_HideAsMoveTarget(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u16)
                as i32,
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy_2));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_OpponentMonFromBall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            if (!((((&raw mut gHitMarker).cast::<u8>().cast::<u32>()).read() & 128u32) != 0))
                || ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 33554434u32)
                    != 0)
            {
                if (HasTwoFramesAnimation(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as u16),
                )) != 0
                {
                    StartSpriteAnim(sprite, 1u8);
                }
            }
            BattleAnimateFrontSprite(
                sprite,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
                1u8,
                1u8,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_BattleSpriteStartSlideLeft(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_BattleSpriteSlideLeft));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BattleSpriteSlideLeft(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((&raw mut gIntroSlideFlags).cast::<u8>().cast::<u16>()).read()) as i32) & 1i32)
            != 0)
        {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
            if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) == 0i32 {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_Idle));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetIdleSpriteCallback(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Idle));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Idle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_FaintSlideAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((&raw mut gIntroSlideFlags).cast::<u8>().cast::<u16>()).read()) as i32) & 1i32)
            != 0)
        {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(38).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBounceEffect(battler: u8, which: u8, delta: i8, amplitude: i8) {
    unsafe {
        let mut battler = battler;
        let mut which = which;
        let mut delta = delta;
        let mut amplitude = amplitude;
        let mut invisibleSpriteId: u8 = 0u8;
        let mut bouncerSpriteId: u8 = 0u8;
        'l1: {
            let __sw1 = ((which) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 0i32;
            if __sw1 == 1i32 || !__matched {
                if (crate::c::bf_read(
                    ((((((&raw mut gBattleSpritesDataPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 12))
                    .wrapping_add(0),
                    1,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    return;
                }
                break 'l1;
            }
            if __sw1 == 0i32 {
                if (crate::c::bf_read(
                    ((((((&raw mut gBattleSpritesDataPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 12))
                    .wrapping_add(0),
                    2,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    return;
                }
                break 'l1;
            }
        }
        invisibleSpriteId = CreateInvisibleSpriteWithCallback(Some(SpriteCB_BounceEffect));
        if ((which) as i32) == 1i32 {
            bouncerSpriteId = ((((&raw mut gHealthboxSpriteIds).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read();
            (((((((&raw mut gBattleSpritesDataPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4)
            .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 12))
            .wrapping_add(2))
            .write(invisibleSpriteId);
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(0),
                1,
                1,
                (1u8) as i32,
            );
            (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((invisibleSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .write(128i16);
        } else {
            bouncerSpriteId = ((((&raw mut gBattlerSpriteIds).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read();
            (((((((&raw mut gBattleSpritesDataPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4)
            .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 12))
            .wrapping_add(3))
            .write(invisibleSpriteId);
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(0),
                2,
                1,
                (1u8) as i32,
            );
            (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((invisibleSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .write(192i16);
        }
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((invisibleSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((delta) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((invisibleSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((amplitude) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((invisibleSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((bouncerSpriteId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((invisibleSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((which) as i16));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((bouncerSpriteId) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>())
        .write(0i16);
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((bouncerSpriteId) as i32) as isize * 68))
        .wrapping_add(38)
        .cast::<i16>())
        .write(0i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EndBounceEffect(battler: u8, which: u8) {
    unsafe {
        let mut battler = battler;
        let mut which = which;
        let mut bouncerSpriteId: u8 = 0u8;
        if ((which) as i32) == 1i32 {
            if !((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(0),
                1,
                1,
                false,
            ) as u8)
                != 0)
            {
                return;
            }
            bouncerSpriteId = ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut gBattleSpritesDataPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as u8);
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut gBattleSpritesDataPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 12))
                    .wrapping_add(2))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(0),
                1,
                1,
                (0u8) as i32,
            );
        } else {
            if !((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(0),
                2,
                1,
                false,
            ) as u8)
                != 0)
            {
                return;
            }
            bouncerSpriteId = ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut gBattleSpritesDataPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(3))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as u8);
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut gBattleSpritesDataPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 12))
                    .wrapping_add(3))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(0),
                2,
                1,
                (0u8) as i32,
            );
        }
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((bouncerSpriteId) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>())
        .write(0i16);
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((bouncerSpriteId) as i32) as isize * 68))
        .wrapping_add(38)
        .cast::<i16>())
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BounceEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut bouncerSpriteId: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8);
        let mut index: i32 = 0i32;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            == 1i32
        {
            index = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
        } else {
            index = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
        }
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((bouncerSpriteId) as i32) as isize * 68))
        .wrapping_add(38)
        .cast::<i16>())
        .write(
            ((((Sin(
                ((index) as i16),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            )) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            ) & 255i32) as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_PlayerMonFromBall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            BattleAnimateBackSprite(
                sprite,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerThrowObject_Main(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        AnimSetCenterToCornerVecX(sprite);
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Idle));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_TrainerThrowObject(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        StartSpriteAnim(sprite, 1u8);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_TrainerThrowObject_Main));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimSetCenterToCornerVecX(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((crate::c::bf_read((sprite).wrapping_add(44), 0, 6, false) as u8) as i32) == 0i32 {
            ((sprite).wrapping_add(40).cast::<i8>()).write(
                ((((&raw const sCenterToCornerVecXs)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<i8>())
                .cast::<i8>())
                .wrapping_offset(((((sprite).wrapping_add(43)).read()) as i32) as isize))
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BeginBattleIntroDummy() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BeginBattleIntro() {
    unsafe {
        BattleStartClearSetData();
        ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .write(0u8);
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(BattleIntroGetMonsData));
    }
}
pub(crate) unsafe extern "C" fn BattleMainCB1() {
    unsafe {
        (((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .read())
        .unwrap_unchecked()();
        {
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
            'l1: loop {
                if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gBattlerControllerFuncs)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    ))
                    .read())
                    .unwrap_unchecked()();
                }
                let __p1 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BattleStartClearSetData() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: u32 = 0u32;
        let mut dataPtr: *mut u8 = core::ptr::null_mut();
        TurnValuesCleanUp(0u8);
        SpecialStatusesClear();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gStatuses3).cast::<u8>().cast::<u32>()).cast::<u32>())
                        .wrapping_offset((i) as isize))
                    .write(0u32);
                    dataPtr = (((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 28);
                    {
                        j = 0u32;
                        'l3: loop {
                            if !(j < 28u32) {
                                break 'l3;
                            }
                            'l4: {
                                ((dataPtr).wrapping_offset(((j) as i32) as isize)).write(0u8);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    (((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 28))
                    .wrapping_add(22))
                    .write(2u8);
                    ((((&raw mut sUnusedBattlersArray).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(0u8);
                    ((((&raw mut gLastMoves).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(0u16);
                    ((((&raw mut gLastLandedMoves).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(0u16);
                    ((((&raw mut gLastHitByType).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(0u16);
                    ((((&raw mut gLastResultingMoves).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(0u16);
                    ((((&raw mut gLastHitBy).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(255u8);
                    ((((&raw mut gLockedMoves).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(0u16);
                    ((((&raw mut gLastPrintedMoves).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(0u16);
                    (((((((&raw mut gBattleResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u32>())
                    .wrapping_offset((i) as isize))
                    .write(0u32);
                    ((((&raw mut gPalaceSelectionBattleScripts)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset((i) as isize))
                    .write(core::ptr::null_mut());
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 2i32) {
                    break 'l5;
                }
                'l6: {
                    ((((&raw mut gSideStatuses).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(0u16);
                    dataPtr = (((&raw mut gSideTimers).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 12);
                    {
                        j = 0u32;
                        'l7: loop {
                            if !(j < 12u32) {
                                break 'l7;
                            }
                            'l8: {
                                ((dataPtr).wrapping_offset(((j) as i32) as isize)).write(0u8);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gBattlerAttacker).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gBattlerTarget).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gBattleWeather).cast::<u8>().cast::<u16>()).write(0u16);
        dataPtr = (&raw mut gWishFutureKnock).cast::<u8>();
        {
            i = 0i32;
            'l9: loop {
                if !(((i) as u32) < 44u32) {
                    break 'l9;
                }
                'l10: {
                    ((dataPtr).wrapping_offset((i) as isize)).write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gHitMarker).cast::<u8>().cast::<u32>()).write(0u32);
        if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 16777216u32) != 0) {
            if (!((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0))
                && (((crate::c::bf_read(
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
                    2,
                    1,
                    false,
                ) as u16) as i32)
                    == 1i32)
            {
                let __p1 = (&raw mut gHitMarker).cast::<u8>().cast::<u32>();
                (__p1).write(((__p1).read() | 128u32));
            }
        } else {
            if (!((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 33554434u32)
                != 0))
                && ((GetBattleSceneInRecordedBattle()) != 0)
            {
                let __p2 = (&raw mut gHitMarker).cast::<u8>().cast::<u32>();
                (__p2).write(((__p2).read() | 128u32));
            }
        }
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(29)).write(
            ((crate::c::bf_read(
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
                1,
                1,
                false,
            ) as u16) as u8),
        );
        ((&raw mut gMultiHitCounter).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .write(0u32);
        ((&raw mut gPaydayMoney).cast::<u8>().cast::<u16>()).write(0u16);
        ((((((&raw mut gBattleResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(32))
        .write(0u8);
        ((((((&raw mut gBattleResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(32))
        .write(0u8);
        {
            i = 0i32;
            'l11: loop {
                if !(i < 8i32) {
                    break 'l11;
                }
                'l12: {
                    ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gPauseCounterBattle).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gBattleMoveDamage).cast::<u8>().cast::<i32>()).write(0i32);
        ((&raw mut gIntroSlideFlags).cast::<u8>().cast::<u16>()).write(0u16);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(24)).write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(25)).write(0u8);
        ((&raw mut gLeveledUpInBattle).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gAbsentBattlerFlags).cast::<u8>().cast::<u8>()).write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(108))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(121))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(122))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(124))
            .write(
                ((crate::c::div_i32(
                    ((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                        ((GetMonData2((&raw mut gEnemyParty).cast::<u8>(), 11i32)) as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(8))
                    .read()) as i32)
                        .wrapping_mul(100i32),
                    1275i32,
                )) as u8),
            );
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(123))
            .write(3u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(74))
            .write(1u8);
        {
            i = 0i32;
            'l13: loop {
                if !(i < 8i32) {
                    break 'l13;
                }
                'l14: {
                    ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(152))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(0u8);
                    (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(184))
                    .cast::<u16>())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(0u8);
                    (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(200))
                    .cast::<u16>())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(0u8);
                    (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(208))
                    .cast::<u16>())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(0u8);
                    ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224))
                    .cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(0i32)) as isize))
                    .write(0u8);
                    ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224))
                    .cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(8i32)) as isize))
                    .write(0u8);
                    ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224))
                    .cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(16i32)) as isize))
                    .write(0u8);
                    ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224))
                    .cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(24i32)) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l15: loop {
                if !(i < 4i32) {
                    break 'l15;
                }
                'l16: {
                    ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(660))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(6u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(223))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(146))
            .write(0u8);
        ((&raw mut gRandomTurnNumber).cast::<u8>().cast::<u16>()).write(Random());
        dataPtr = (&raw mut gBattleResults).cast::<u8>();
        {
            i = 0i32;
            'l17: loop {
                if !(((i) as u32) < 68u32) {
                    break 'l17;
                }
                'l18: {
                    ((dataPtr).wrapping_offset((i) as isize)).write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::bf_write(
            ((&raw mut gBattleResults).cast::<u8>()).wrapping_add(5),
            6,
            1,
            (IsMonShiny((&raw mut gEnemyParty).cast::<u8>())) as i32,
        );
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(672))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(673))
            .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwitchInClearSetData() {
    unsafe {
        let mut disableStructCopy = crate::ffi::Align4([0u8; 28]);
        (&raw mut disableStructCopy)
            .cast::<u8>()
            .cast::<crate::c::Rec4<28>>()
            .write_unaligned(
                (((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize
                            * 28,
                    )
                    .cast::<crate::c::Rec4<28>>()
                    .read_unaligned(),
            );
        let mut i: i32 = 0i32;
        let mut ptr: *mut u8 = core::ptr::null_mut();
        if (((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
            ((((&raw mut gCurrentMove).cast::<u8>().cast::<u16>()).read()) as i32) as isize * 12,
        ))
        .read()) as i32)
            != 127i32
        {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 8i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize
                                    * 88,
                            ))
                        .wrapping_add(24))
                        .cast::<i8>())
                        .wrapping_offset((i) as isize))
                        .write(6i8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l3: loop {
                    if !(i
                        < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                    {
                        break 'l3;
                    }
                    'l4: {
                        if (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .read()
                            & 67108864u32)
                            != 0)
                            && ((((((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(20))
                            .read()) as i32)
                                == ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                        {
                            let __p1 = ((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 88))
                            .wrapping_add(80)
                            .cast::<u32>();
                            (__p1).write(((__p1).read() & 4227858431u32));
                        }
                        if ((((((&raw mut gStatuses3).cast::<u8>().cast::<u32>()).cast::<u32>())
                            .wrapping_offset((i) as isize))
                        .read()
                            & 24u32)
                            != 0)
                            && ((((((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(21))
                            .read()) as i32)
                                == ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                        {
                            let __p2 = (((&raw mut gStatuses3).cast::<u8>().cast::<u32>())
                                .cast::<u32>())
                            .wrapping_offset((i) as isize);
                            (__p2).write(((__p2).read() & 4294967271u32));
                            (((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(21))
                            .write(0u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if (((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
            ((((&raw mut gCurrentMove).cast::<u8>().cast::<u16>()).read()) as i32) as isize * 12,
        ))
        .read()) as i32)
            == 127i32
        {
            let __p3 = ((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 88,
            ))
            .wrapping_add(80)
            .cast::<u32>();
            (__p3).write(((__p3).read() & 353370119u32));
            let __p4 = (((&raw mut gStatuses3).cast::<u8>().cast::<u32>()).cast::<u32>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize,
                );
            (__p4).write(((__p4).read() & 197695u32));
            {
                i = 0i32;
                'l5: loop {
                    if !(i
                        < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                    {
                        break 'l5;
                    }
                    'l6: {
                        if ((((GetBattlerSide(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        )) as i32)
                            != ((GetBattlerSide(((i) as u8))) as i32))
                            && ((((((&raw mut gStatuses3).cast::<u8>().cast::<u32>())
                                .cast::<u32>())
                            .wrapping_offset((i) as isize))
                            .read()
                                & 24u32)
                                != 0u32))
                            && ((((((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(21))
                            .read()) as i32)
                                == ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32))
                        {
                            let __p5 = (((&raw mut gStatuses3).cast::<u8>().cast::<u32>())
                                .cast::<u32>())
                            .wrapping_offset((i) as isize);
                            (__p5).write(((__p5).read() & 4294967271u32));
                            let __p6 = (((&raw mut gStatuses3).cast::<u8>().cast::<u32>())
                                .cast::<u32>())
                            .wrapping_offset((i) as isize);
                            (__p6).write(((__p6).read() | 16u32));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 88,
            ))
            .wrapping_add(80)
            .cast::<u32>())
            .write(0u32);
            ((((&raw mut gStatuses3).cast::<u8>().cast::<u32>()).cast::<u32>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
            ))
            .write(0u32);
        }
        {
            i = 0i32;
            'l7: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)) {
                    break 'l7;
                }
                'l8: {
                    if ((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 88))
                    .wrapping_add(80)
                    .cast::<u32>())
                    .read()
                        & (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                        .read()
                            << 16))
                        != 0
                    {
                        let __p7 = ((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(80)
                        .cast::<u32>();
                        (__p7).write(
                            ((__p7).read()
                                & !(((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()
                                    << 16)),
                        );
                    }
                    if (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 88))
                    .wrapping_add(80)
                    .cast::<u32>())
                    .read()
                        & 57344u32)
                        != 0)
                        && (((((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(20))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            == ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                as i32))
                    {
                        let __p8 = ((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(80)
                        .cast::<u32>();
                        (__p8).write(((__p8).read() & 4294909951u32));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gActionSelectionCursor).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u8);
        ((((&raw mut gMoveSelectionCursor).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u8);
        ptr = (((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 28,
        );
        {
            i = 0i32;
            'l9: loop {
                if !(((i) as u32) < 28u32) {
                    break 'l9;
                }
                'l10: {
                    ((ptr).wrapping_offset((i) as isize)).write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
            ((((&raw mut gCurrentMove).cast::<u8>().cast::<u16>()).read()) as i32) as isize * 12,
        ))
        .read()) as i32)
            == 127i32
        {
            (((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(10))
            .write((((&raw mut disableStructCopy).cast::<u8>()).wrapping_add(10)).read());
            (((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(21))
            .write((((&raw mut disableStructCopy).cast::<u8>()).wrapping_add(21)).read());
            crate::c::bf_write(
                ((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 28,
                ))
                .wrapping_add(15),
                0,
                4,
                (crate::c::bf_read(
                    ((&raw mut disableStructCopy).cast::<u8>()).wrapping_add(15),
                    0,
                    4,
                    false,
                ) as u8) as i32,
            );
            crate::c::bf_write(
                ((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 28,
                ))
                .wrapping_add(15),
                4,
                4,
                (crate::c::bf_read(
                    ((&raw mut disableStructCopy).cast::<u8>()).wrapping_add(15),
                    4,
                    4,
                    false,
                ) as u8) as i32,
            );
            (((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(20))
            .write((((&raw mut disableStructCopy).cast::<u8>()).wrapping_add(20)).read());
        }
        ((&raw mut gMoveResultFlags).cast::<u8>().cast::<u8>()).write(0u8);
        (((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 28,
        ))
        .wrapping_add(22))
        .write(2u8);
        crate::c::bf_write(
            ((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(24),
            1,
            1,
            (crate::c::bf_read(
                ((&raw mut disableStructCopy).cast::<u8>()).wrapping_add(24),
                1,
                1,
                false,
            ) as u8) as i32,
        );
        ((((&raw mut gLastMoves).cast::<u8>().cast::<u16>()).cast::<u16>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u16);
        ((((&raw mut gLastLandedMoves).cast::<u8>().cast::<u16>()).cast::<u16>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u16);
        ((((&raw mut gLastHitByType).cast::<u8>().cast::<u16>()).cast::<u16>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u16);
        ((((&raw mut gLastResultingMoves).cast::<u8>().cast::<u16>()).cast::<u16>())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
            ))
        .write(0u16);
        ((((&raw mut gLastPrintedMoves).cast::<u8>().cast::<u16>()).cast::<u16>())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
            ))
        .write(0u16);
        ((((&raw mut gLastHitBy).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(255u8);
        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(152))
            .cast::<u8>())
        .wrapping_offset(
            (((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                .wrapping_mul(2i32)) as isize,
        ))
        .write(0u8);
        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(152))
        .cast::<u8>())
        .wrapping_offset(
            (((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                .wrapping_mul(2i32)) as isize,
        ))
        .wrapping_offset(1))
        .write(0u8);
        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(224))
            .cast::<u8>())
        .wrapping_offset(
            ((0i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .write(0u8);
        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(224))
        .cast::<u8>())
        .wrapping_offset(
            ((0i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .wrapping_offset(1))
        .write(0u8);
        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(224))
            .cast::<u8>())
        .wrapping_offset(
            ((2i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .write(0u8);
        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(224))
        .cast::<u8>())
        .wrapping_offset(
            ((2i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .wrapping_offset(1))
        .write(0u8);
        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(224))
            .cast::<u8>())
        .wrapping_offset(
            ((4i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .write(0u8);
        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(224))
        .cast::<u8>())
        .wrapping_offset(
            ((4i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .wrapping_offset(1))
        .write(0u8);
        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(224))
            .cast::<u8>())
        .wrapping_offset(
            ((6i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .write(0u8);
        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(224))
        .cast::<u8>())
        .wrapping_offset(
            ((6i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .wrapping_offset(1))
        .write(0u8);
        let __p9 =
            (((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(146);
        (__p9).write(
            (((((__p9).read()) as u32)
                & !(((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize,
                ))
                .read())) as u8),
        );
        {
            i = 0i32;
            'l11: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)) {
                    break 'l11;
                }
                'l12: {
                    if (i
                        != ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32))
                        && (((GetBattlerSide(((i) as u8))) as i32)
                            != ((GetBattlerSide(
                                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                            )) as i32))
                    {
                        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(152))
                        .cast::<u8>())
                        .wrapping_offset(((i).wrapping_mul(2i32)) as isize))
                        .write(0u8);
                        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(152))
                        .cast::<u8>())
                        .wrapping_offset(((i).wrapping_mul(2i32)) as isize))
                        .wrapping_offset(1))
                        .write(0u8);
                    }
                    ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((i).wrapping_mul(8i32)).wrapping_add(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                .wrapping_mul(2i32),
                        )) as isize,
                    ))
                    .write(0u8);
                    (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((i).wrapping_mul(8i32)).wrapping_add(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                .wrapping_mul(2i32),
                        )) as isize,
                    ))
                    .wrapping_offset(1))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(200))
        .cast::<u16>())
        .wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .cast::<u8>())
        .write(0u8);
        ((((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(200))
        .cast::<u16>())
        .wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .cast::<u8>())
        .wrapping_offset(1))
        .write(0u8);
        (((((((&raw mut gBattleResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .cast::<u32>())
        .wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u32);
        ((&raw mut gCurrentMove).cast::<u8>().cast::<u16>()).write(0u16);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(218))
            .write(255u8);
        ClearBattlerMoveHistory(((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read());
        ClearBattlerAbilityHistory(((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FaintClearSetData() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut ptr: *mut u8 = core::ptr::null_mut();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize
                            * 88,
                    ))
                    .wrapping_add(24))
                    .cast::<i8>())
                    .wrapping_offset((i) as isize))
                    .write(6i8);
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(80)
        .cast::<u32>())
        .write(0u32);
        ((((&raw mut gStatuses3).cast::<u8>().cast::<u32>()).cast::<u32>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u32);
        {
            i = 0i32;
            'l3: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)) {
                    break 'l3;
                }
                'l4: {
                    if (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 88))
                    .wrapping_add(80)
                    .cast::<u32>())
                    .read()
                        & 67108864u32)
                        != 0)
                        && ((((((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 28))
                        .wrapping_add(20))
                        .read()) as i32)
                            == ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                as i32))
                    {
                        let __p1 = ((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(80)
                        .cast::<u32>();
                        (__p1).write(((__p1).read() & 4227858431u32));
                    }
                    if ((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 88))
                    .wrapping_add(80)
                    .cast::<u32>())
                    .read()
                        & (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                        .read()
                            << 16))
                        != 0
                    {
                        let __p2 = ((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(80)
                        .cast::<u32>();
                        (__p2).write(
                            ((__p2).read()
                                & !(((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()
                                    << 16)),
                        );
                    }
                    if (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 88))
                    .wrapping_add(80)
                    .cast::<u32>())
                    .read()
                        & 57344u32)
                        != 0)
                        && (((((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(20))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            == ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                as i32))
                    {
                        let __p3 = ((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(80)
                        .cast::<u32>();
                        (__p3).write(((__p3).read() & 4294909951u32));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gActionSelectionCursor).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u8);
        ((((&raw mut gMoveSelectionCursor).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u8);
        ptr = (((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 28,
        );
        {
            i = 0i32;
            'l5: loop {
                if !(((i) as u32) < 28u32) {
                    break 'l5;
                }
                'l6: {
                    ((ptr).wrapping_offset((i) as isize)).write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(0),
            0,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(0),
            1,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(0),
            2,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(0),
            3,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(0),
            4,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(0),
            5,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(0),
            6,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(0),
            7,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(1),
            0,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(1),
            1,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(1),
            2,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(1),
            3,
            2,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(1),
            5,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(1),
            6,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(1),
            7,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(2),
            0,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(2),
            1,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(2),
            2,
            1,
            (0u32) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 16,
            ))
            .wrapping_add(2),
            3,
            1,
            (0u32) as i32,
        );
        (((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 28,
        ))
        .wrapping_add(22))
        .write(2u8);
        ((((&raw mut gLastMoves).cast::<u8>().cast::<u16>()).cast::<u16>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u16);
        ((((&raw mut gLastLandedMoves).cast::<u8>().cast::<u16>()).cast::<u16>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u16);
        ((((&raw mut gLastHitByType).cast::<u8>().cast::<u16>()).cast::<u16>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u16);
        ((((&raw mut gLastResultingMoves).cast::<u8>().cast::<u16>()).cast::<u16>())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
            ))
        .write(0u16);
        ((((&raw mut gLastPrintedMoves).cast::<u8>().cast::<u16>()).cast::<u16>())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
            ))
        .write(0u16);
        ((((&raw mut gLastHitBy).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(255u8);
        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(200))
        .cast::<u16>())
        .wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .cast::<u8>())
        .write(0u8);
        ((((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(200))
        .cast::<u16>())
        .wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .cast::<u8>())
        .wrapping_offset(1))
        .write(0u8);
        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(152))
            .cast::<u8>())
        .wrapping_offset(
            (((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                .wrapping_mul(2i32)) as isize,
        ))
        .write(0u8);
        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(152))
        .cast::<u8>())
        .wrapping_offset(
            (((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                .wrapping_mul(2i32)) as isize,
        ))
        .wrapping_offset(1))
        .write(0u8);
        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(224))
            .cast::<u8>())
        .wrapping_offset(
            ((0i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .write(0u8);
        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(224))
        .cast::<u8>())
        .wrapping_offset(
            ((0i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .wrapping_offset(1))
        .write(0u8);
        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(224))
            .cast::<u8>())
        .wrapping_offset(
            ((2i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .write(0u8);
        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(224))
        .cast::<u8>())
        .wrapping_offset(
            ((2i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .wrapping_offset(1))
        .write(0u8);
        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(224))
            .cast::<u8>())
        .wrapping_offset(
            ((4i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .write(0u8);
        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(224))
        .cast::<u8>())
        .wrapping_offset(
            ((4i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .wrapping_offset(1))
        .write(0u8);
        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(224))
            .cast::<u8>())
        .wrapping_offset(
            ((6i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .write(0u8);
        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(224))
        .cast::<u8>())
        .wrapping_offset(
            ((6i32).wrapping_add(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(8i32),
            )) as isize,
        ))
        .wrapping_offset(1))
        .write(0u8);
        let __p4 =
            (((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(146);
        (__p4).write(
            (((((__p4).read()) as u32)
                & !(((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize,
                ))
                .read())) as u8),
        );
        {
            i = 0i32;
            'l7: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)) {
                    break 'l7;
                }
                'l8: {
                    if (i
                        != ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32))
                        && (((GetBattlerSide(((i) as u8))) as i32)
                            != ((GetBattlerSide(
                                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                            )) as i32))
                    {
                        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(152))
                        .cast::<u8>())
                        .wrapping_offset(((i).wrapping_mul(2i32)) as isize))
                        .write(0u8);
                        (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(152))
                        .cast::<u8>())
                        .wrapping_offset(((i).wrapping_mul(2i32)) as isize))
                        .wrapping_offset(1))
                        .write(0u8);
                    }
                    ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((i).wrapping_mul(8i32)).wrapping_add(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                .wrapping_mul(2i32),
                        )) as isize,
                    ))
                    .write(0u8);
                    (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((i).wrapping_mul(8i32)).wrapping_add(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                .wrapping_mul(2i32),
                        )) as isize,
                    ))
                    .wrapping_offset(1))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((((&raw mut gBattleResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .cast::<u32>())
        .wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(0u32);
        ((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(33))
        .cast::<u8>())
        .write(
            (((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 88,
                ))
                .cast::<u16>())
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(6))
            .cast::<u8>())
            .read(),
        );
        (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(33))
        .cast::<u8>())
        .wrapping_offset(1))
        .write(
            ((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 88,
                ))
                .cast::<u16>())
                .read()) as i32) as isize
                    * 28,
            ))
            .wrapping_add(6))
            .cast::<u8>())
            .wrapping_offset(1))
            .read(),
        );
        ClearBattlerMoveHistory(((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read());
        ClearBattlerAbilityHistory(((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn BattleIntroGetMonsData() {
    unsafe {
        'l1: {
            let __sw1 =
                (((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(
                    ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1))
                    .read(),
                );
                BtlController_EmitGetMonData(0u8, 0u8, 0u8);
                MarkBattlerForControllerExec(
                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                );
                let __p2 = ((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((&raw mut gBattleControllerExecFlags)
                    .cast::<u8>()
                    .cast::<u32>())
                .read()
                    == 0u32
                {
                    let __p3 = (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    if ((((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        == ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)
                    {
                        ((&raw mut gBattleMainFunc)
                            .cast::<u8>()
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(BattleIntroPrepareBackgroundSlide));
                    } else {
                        (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(0u8);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BattleIntroPrepareBackgroundSlide() {
    unsafe {
        if ((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read()
            == 0u32
        {
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(GetBattlerAtPosition(0u8));
            BtlController_EmitIntroSlide(
                0u8,
                ((&raw mut gBattleEnvironment).cast::<u8>().cast::<u8>()).read(),
            );
            MarkBattlerForControllerExec(
                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
            );
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(BattleIntroDrawTrainersOrMonsSprites));
            (((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).write(0u8);
            ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                .write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn BattleIntroDrawTrainersOrMonsSprites() {
    unsafe {
        let mut ptr: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        if (((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read())
            != 0
        {
            return;
        }
        {
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
            'l1: loop {
                if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 128u32)
                        != 0)
                        && (((GetBattlerSide(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        )) as i32)
                            == 0i32)
                    {
                        ptr = (((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 88,
                        );
                        {
                            i = 0i32;
                            'l3: loop {
                                if !(((i) as u32) < 88u32) {
                                    break 'l3;
                                }
                                'l4: {
                                    ((ptr).wrapping_offset((i) as isize)).write(0u8);
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    } else {
                        let mut hpOnSwitchout: *mut u16 = core::ptr::null_mut();
                        ptr = (((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 88,
                        );
                        {
                            i = 0i32;
                            'l5: loop {
                                if !(((i) as u32) < 88u32) {
                                    break 'l5;
                                }
                                'l6: {
                                    ((ptr).wrapping_offset((i) as isize)).write(
                                        ((((((&raw mut gBattleBufferB).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 512,
                                        ))
                                        .cast::<u8>())
                                        .wrapping_offset(((4i32).wrapping_add(i)) as isize))
                                        .read(),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        ((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 88,
                        ))
                        .wrapping_add(33))
                        .cast::<u8>())
                        .write(
                            (((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 88,
                                    ))
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(6))
                            .cast::<u8>())
                            .read(),
                        );
                        (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize
                                    * 88,
                            ))
                        .wrapping_add(33))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .write(
                            ((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 88,
                                    ))
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(6))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read(),
                        );
                        (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 88,
                        ))
                        .wrapping_add(32))
                        .write(GetAbilityBySpecies(
                            (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize
                                        * 88,
                                ))
                            .cast::<u16>())
                            .read(),
                            ((crate::c::bf_read(
                                ((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(23),
                                7,
                                1,
                                false,
                            ) as u32) as u8),
                        ));
                        hpOnSwitchout =
                            (((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(168))
                            .cast::<u16>())
                            .wrapping_offset(
                                ((GetBattlerSide(
                                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                                )) as i32) as isize,
                            );
                        (hpOnSwitchout).write(
                            (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize
                                        * 88,
                                ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read(),
                        );
                        {
                            i = 0i32;
                            'l7: loop {
                                if !(i < 8i32) {
                                    break 'l7;
                                }
                                'l8: {
                                    (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(24))
                                    .cast::<i8>())
                                    .wrapping_offset((i) as isize))
                                    .write(6i8);
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 88,
                        ))
                        .wrapping_add(80)
                        .cast::<u32>())
                        .write(0u32);
                    }
                    if ((GetBattlerPosition(
                        ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                    )) as i32)
                        == 0i32
                    {
                        BtlController_EmitDrawTrainerPic(0u8);
                        MarkBattlerForControllerExec(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        );
                    }
                    if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 8u32) != 0
                    {
                        if ((GetBattlerPosition(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        )) as i32)
                            == 1i32
                        {
                            BtlController_EmitDrawTrainerPic(0u8);
                            MarkBattlerForControllerExec(
                                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                            );
                        }
                        if (((GetBattlerSide(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        )) as i32)
                            == 1i32)
                            && (!((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>())
                                .read()
                                & 104794370u32)
                                != 0))
                        {
                            HandleSetPokedexFlag(
                                SpeciesToNationalPokedexNum(
                                    (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .cast::<u16>())
                                    .read(),
                                ),
                                2u8,
                                (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(72)
                                .cast::<u32>())
                                .read(),
                            );
                        }
                    } else {
                        if ((GetBattlerSide(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        )) as i32)
                            == 1i32
                        {
                            if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read()
                                & 104794370u32)
                                != 0)
                            {
                                HandleSetPokedexFlag(
                                    SpeciesToNationalPokedexNum(
                                        (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 88,
                                            ))
                                        .cast::<u16>())
                                        .read(),
                                    ),
                                    2u8,
                                    (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(72)
                                    .cast::<u32>())
                                    .read(),
                                );
                            }
                            BtlController_EmitLoadMonSprite(0u8);
                            MarkBattlerForControllerExec(
                                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                            );
                            (((&raw mut gBattleResults).cast::<u8>())
                                .wrapping_add(32)
                                .cast::<u16>())
                            .write(
                                ((GetMonData3(
                                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                        ((((((&raw mut gBattlerPartyIndexes)
                                            .cast::<u8>()
                                            .cast::<u16>())
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read()) as i32)
                                            as isize
                                            * 100,
                                    ),
                                    11i32,
                                    core::ptr::null_mut(),
                                )) as u16),
                            );
                        }
                    }
                    if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 64u32)
                        != 0
                    {
                        if (((GetBattlerPosition(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        )) as i32)
                            == 2i32)
                            || (((GetBattlerPosition(
                                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                            )) as i32)
                                == 3i32)
                        {
                            BtlController_EmitDrawTrainerPic(0u8);
                            MarkBattlerForControllerExec(
                                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                            );
                        }
                    }
                    if ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read()
                        & 32768u32)
                        != 0)
                        && (((GetBattlerPosition(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        )) as i32)
                            == 3i32)
                    {
                        BtlController_EmitDrawTrainerPic(0u8);
                        MarkBattlerForControllerExec(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        );
                    }
                    if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 262144u32)
                        != 0
                    {
                        BattleArena_InitPoints();
                    }
                }
                let __p1 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(BattleIntroDrawPartySummaryScreens));
    }
}
pub(crate) unsafe extern "C" fn BattleIntroDrawPartySummaryScreens() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut hpStatus = crate::ffi::Align4([0u8; 48]);
        if (((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read())
            != 0
        {
            return;
        }
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 8u32) != 0 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (GetMonData2(
                            ((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            65i32,
                        ) == 0u32)
                            || (GetMonData2(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                65i32,
                            ) == 412u32)
                        {
                            ((((&raw mut hpStatus).cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                            .cast::<u16>())
                            .write(65535u16);
                            ((((&raw mut hpStatus).cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                            .wrapping_add(4)
                            .cast::<u32>())
                            .write(0u32);
                        } else {
                            ((((&raw mut hpStatus).cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                            .cast::<u16>())
                            .write(
                                ((GetMonData2(
                                    ((&raw mut gEnemyParty).cast::<u8>())
                                        .wrapping_offset((i) as isize * 100),
                                    57i32,
                                )) as u16),
                            );
                            ((((&raw mut hpStatus).cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                            .wrapping_add(4)
                            .cast::<u32>())
                            .write(GetMonData2(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                55i32,
                            ));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(GetBattlerAtPosition(1u8));
            BtlController_EmitDrawPartyStatusSummary(0u8, (&raw mut hpStatus).cast::<u8>(), 128u8);
            MarkBattlerForControllerExec(
                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
            );
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 6i32) {
                        break 'l3;
                    }
                    'l4: {
                        if (GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            65i32,
                        ) == 0u32)
                            || (GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                65i32,
                            ) == 412u32)
                        {
                            ((((&raw mut hpStatus).cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                            .cast::<u16>())
                            .write(65535u16);
                            ((((&raw mut hpStatus).cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                            .wrapping_add(4)
                            .cast::<u32>())
                            .write(0u32);
                        } else {
                            ((((&raw mut hpStatus).cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                            .cast::<u16>())
                            .write(
                                ((GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset((i) as isize * 100),
                                    57i32,
                                )) as u16),
                            );
                            ((((&raw mut hpStatus).cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                            .wrapping_add(4)
                            .cast::<u32>())
                            .write(GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                55i32,
                            ));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(GetBattlerAtPosition(0u8));
            BtlController_EmitDrawPartyStatusSummary(0u8, (&raw mut hpStatus).cast::<u8>(), 128u8);
            MarkBattlerForControllerExec(
                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
            );
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(BattleIntroPrintTrainerWantsToBattle));
        } else {
            {
                i = 0i32;
                'l5: loop {
                    if !(i < 6i32) {
                        break 'l5;
                    }
                    'l6: {
                        if (GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            65i32,
                        ) == 0u32)
                            || (GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                65i32,
                            ) == 412u32)
                        {
                            ((((&raw mut hpStatus).cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                            .cast::<u16>())
                            .write(65535u16);
                            ((((&raw mut hpStatus).cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                            .wrapping_add(4)
                            .cast::<u32>())
                            .write(0u32);
                        } else {
                            ((((&raw mut hpStatus).cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                            .cast::<u16>())
                            .write(
                                ((GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset((i) as isize * 100),
                                    57i32,
                                )) as u16),
                            );
                            ((((&raw mut hpStatus).cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                            .wrapping_add(4)
                            .cast::<u32>())
                            .write(GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                55i32,
                            ));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(BattleIntroPrintWildMonAttacked));
        }
    }
}
pub(crate) unsafe extern "C" fn BattleIntroPrintTrainerWantsToBattle() {
    unsafe {
        if ((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read()
            == 0u32
        {
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(GetBattlerAtPosition(1u8));
            PrepareStringBattle(
                0u16,
                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
            );
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(BattleIntroPrintOpponentSendsOut));
        }
    }
}
pub(crate) unsafe extern "C" fn BattleIntroPrintWildMonAttacked() {
    unsafe {
        if ((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read()
            == 0u32
        {
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(BattleIntroPrintPlayerSendsOut));
            PrepareStringBattle(0u16, 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn BattleIntroPrintOpponentSendsOut() {
    unsafe {
        let mut position: u32 = 0u32;
        if (((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read())
            != 0
        {
            return;
        }
        if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 16777216u32) != 0) {
            position = 1u32;
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 33554432u32) != 0
            {
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2147483648u32)
                    != 0
                {
                    position = 1u32;
                } else {
                    position = 0u32;
                }
            } else {
                position = 1u32;
            }
        }
        PrepareStringBattle(1u16, GetBattlerAtPosition(((position) as u8)));
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(BattleIntroOpponent1SendsOutMonAnimation));
    }
}
pub(crate) unsafe extern "C" fn BattleIntroOpponent2SendsOutMonAnimation() {
    unsafe {
        let mut position: u32 = 0u32;
        if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 16777216u32) != 0) {
            position = 3u32;
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 33554432u32) != 0
            {
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2147483648u32)
                    != 0
                {
                    position = 3u32;
                } else {
                    position = 2u32;
                }
            } else {
                position = 3u32;
            }
        }
        {
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
            'l1: loop {
                if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((GetBattlerPosition(
                        ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                    )) as u32)
                        == position
                    {
                        BtlController_EmitIntroTrainerBallThrow(0u8);
                        MarkBattlerForControllerExec(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        );
                    }
                }
                let __p1 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(BattleIntroRecordMonsToDex));
    }
}
pub(crate) unsafe extern "C" fn BattleIntroOpponent1SendsOutMonAnimation() {
    unsafe {
        let mut position: u32 = 0u32;
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 16777216u32) != 0 {
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 33554432u32) != 0
            {
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2147483648u32)
                    != 0
                {
                    position = 1u32;
                } else {
                    position = 0u32;
                }
            } else {
                position = 1u32;
            }
        } else {
            position = 1u32;
        }
        if (((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read())
            != 0
        {
            return;
        }
        {
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
            'l1: loop {
                if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((GetBattlerPosition(
                        ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                    )) as u32)
                        == position
                    {
                        BtlController_EmitIntroTrainerBallThrow(0u8);
                        MarkBattlerForControllerExec(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        );
                        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read()
                            & 32832u32)
                            != 0
                        {
                            ((&raw mut gBattleMainFunc)
                                .cast::<u8>()
                                .cast::<Option<unsafe extern "C" fn()>>())
                            .write(Some(BattleIntroOpponent2SendsOutMonAnimation));
                            return;
                        }
                    }
                }
                let __p1 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(BattleIntroRecordMonsToDex));
    }
}
pub(crate) unsafe extern "C" fn BattleIntroRecordMonsToDex() {
    unsafe {
        if ((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read()
            == 0u32
        {
            {
                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
                'l1: loop {
                    if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        if (((GetBattlerSide(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        )) as i32)
                            == 1i32)
                            && (!((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>())
                                .read()
                                & 104794370u32)
                                != 0))
                        {
                            HandleSetPokedexFlag(
                                SpeciesToNationalPokedexNum(
                                    (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .cast::<u16>())
                                    .read(),
                                ),
                                2u8,
                                (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(72)
                                .cast::<u32>())
                                .read(),
                            );
                        }
                    }
                    let __p1 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                }
            }
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(BattleIntroPrintPlayerSendsOut));
        }
    }
}
pub(crate) unsafe extern "C" fn BattleIntroSkipRecordMonsToDex() {
    unsafe {
        if ((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read()
            == 0u32
        {
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(BattleIntroPrintPlayerSendsOut));
        }
    }
}
pub(crate) unsafe extern "C" fn BattleIntroPrintPlayerSendsOut() {
    unsafe {
        if ((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read()
            == 0u32
        {
            let mut position: u8 = 0u8;
            if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 16777216u32)
                != 0)
            {
                position = 0u8;
            } else {
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 33554432u32)
                    != 0
                {
                    if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read()
                        & 2147483648u32)
                        != 0
                    {
                        position = 0u8;
                    } else {
                        position = 1u8;
                    }
                } else {
                    position = 0u8;
                }
            }
            if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 128u32) != 0) {
                PrepareStringBattle(1u16, GetBattlerAtPosition(position));
            }
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(BattleIntroPlayer1SendsOutMonAnimation));
        }
    }
}
pub(crate) unsafe extern "C" fn BattleIntroPlayer2SendsOutMonAnimation() {
    unsafe {
        let mut position: u32 = 0u32;
        if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 16777216u32) != 0) {
            position = 2u32;
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 33554432u32) != 0
            {
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2147483648u32)
                    != 0
                {
                    position = 2u32;
                } else {
                    position = 3u32;
                }
            } else {
                position = 2u32;
            }
        }
        {
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
            'l1: loop {
                if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((GetBattlerPosition(
                        ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                    )) as u32)
                        == position
                    {
                        BtlController_EmitIntroTrainerBallThrow(0u8);
                        MarkBattlerForControllerExec(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        );
                    }
                }
                let __p1 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(76))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(217))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(418))
            .write(0u8);
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(TryDoEventsBeforeFirstTurn));
    }
}
pub(crate) unsafe extern "C" fn BattleIntroPlayer1SendsOutMonAnimation() {
    unsafe {
        let mut position: u32 = 0u32;
        if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 16777216u32) != 0) {
            position = 0u32;
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 33554432u32) != 0
            {
                if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2147483648u32)
                    != 0
                {
                    position = 0u32;
                } else {
                    position = 1u32;
                }
            } else {
                position = 0u32;
            }
        }
        if (((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read())
            != 0
        {
            return;
        }
        {
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
            'l1: loop {
                if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((GetBattlerPosition(
                        ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                    )) as u32)
                        == position
                    {
                        BtlController_EmitIntroTrainerBallThrow(0u8);
                        MarkBattlerForControllerExec(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        );
                        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 64u32)
                            != 0
                        {
                            ((&raw mut gBattleMainFunc)
                                .cast::<u8>()
                                .cast::<Option<unsafe extern "C" fn()>>())
                            .write(Some(BattleIntroPlayer2SendsOutMonAnimation));
                            return;
                        }
                    }
                }
                let __p1 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(76))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(217))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(418))
            .write(0u8);
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(TryDoEventsBeforeFirstTurn));
    }
}
pub(crate) unsafe extern "C" fn BattleIntroSwitchInPlayerMons() {
    unsafe {
        if ((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read()
            == 0u32
        {
            {
                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
                'l1: loop {
                    if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        if ((GetBattlerSide(
                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                        )) as i32)
                            == 0i32
                        {
                            BtlController_EmitSwitchInAnim(
                                0u8,
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize,
                                ))
                                .read()) as u8),
                                0u8,
                            );
                            MarkBattlerForControllerExec(
                                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                            );
                        }
                    }
                    let __p1 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                }
            }
            ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(76))
                .write(0u8);
            ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(217))
                .write(0u8);
            ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(418))
                .write(0u8);
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(TryDoEventsBeforeFirstTurn));
        }
    }
}
pub(crate) unsafe extern "C" fn TryDoEventsBeforeFirstTurn() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut effect: u8 = 0u8;
        if (((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read())
            != 0
        {
            return;
        }
        if ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(76))
            .read()) as i32)
            == 0i32
        {
            {
                i = 0i32;
                'l1: loop {
                    if !(i
                        < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut gBattlerByTurnOrder).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .write(((i) as u8));
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l3: loop {
                    if !(i
                        < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)
                            .wrapping_sub(1i32))
                    {
                        break 'l3;
                    }
                    'l4: {
                        {
                            j = (i).wrapping_add(1i32);
                            'l5: loop {
                                if !(j
                                    < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>())
                                        .read()) as i32))
                                {
                                    break 'l5;
                                }
                                'l6: {
                                    if ((GetWhoStrikesFirst(
                                        ((((&raw mut gBattlerByTurnOrder).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read(),
                                        ((((&raw mut gBattlerByTurnOrder).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .read(),
                                        1u8,
                                    )) as i32)
                                        != 0i32
                                    {
                                        SwapTurnOrder(((i) as u8), ((j) as u8));
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if (!((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(418))
        .read())
            != 0))
            && (((AbilityBattleEffects(0u8, 0u8, 0u8, 255u8, 0u16)) as i32) != 0i32)
        {
            ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(418))
                .write(1u8);
            return;
        }
        'l7: loop {
            if !(((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(76))
            .read()) as i32)
                < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
            {
                break 'l7;
            }
            if ((AbilityBattleEffects(
                0u8,
                ((((&raw mut gBattlerByTurnOrder).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76))
                    .read()) as i32) as isize,
                ))
                .read(),
                0u8,
                0u8,
                0u16,
            )) as i32)
                != 0i32
            {
                effect = (effect).wrapping_add(1);
            }
            let __p1 =
                (((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(76);
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((effect) as i32) != 0i32 {
                return;
            }
        }
        if ((AbilityBattleEffects(9u8, 0u8, 0u8, 0u8, 0u16)) as i32) != 0i32 {
            return;
        }
        if ((AbilityBattleEffects(11u8, 0u8, 0u8, 0u8, 0u16)) as i32) != 0i32 {
            return;
        }
        'l8: loop {
            if !(((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(217))
            .read()) as i32)
                < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
            {
                break 'l8;
            }
            if (ItemBattleEffects(
                0u8,
                ((((&raw mut gBattlerByTurnOrder).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(217))
                    .read()) as i32) as isize,
                ))
                .read(),
                0u8,
            )) != 0
            {
                effect = (effect).wrapping_add(1);
            }
            let __p2 = (((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(217);
            (__p2).write(((__p2).read()).wrapping_add(1));
            if ((effect) as i32) != 0i32 {
                return;
            }
        }
        {
            i = 0i32;
            'l9: loop {
                if !(i < 4i32) {
                    break 'l9;
                }
                'l10: {
                    ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(92))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(6u8);
                    ((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(255u8);
                    ((((&raw mut gChosenMoveByBattler).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        TurnValuesCleanUp(0u8);
        SpecialStatusesClear();
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(145))
            .write(((&raw mut gAbsentBattlerFlags).cast::<u8>().cast::<u8>()).read());
        BattlePutTextOnWindow((&raw mut gText_EmptyString3).cast::<u8>(), 0u8);
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(HandleTurnActionSelectionState));
        ResetSentPokesToOpponentValue();
        {
            i = 0i32;
            'l11: loop {
                if !(i < 8i32) {
                    break 'l11;
                }
                'l12: {
                    ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l13: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)) {
                    break 'l13;
                }
                'l14: {
                    let __p3 = ((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 88))
                    .wrapping_add(80)
                    .cast::<u32>();
                    (__p3).write(((__p3).read() & 4294967287u32));
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(416))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(417))
            .write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(20)).write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(77))
            .write(0u8);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
            .write(0u8);
        ((&raw mut gMoveResultFlags).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gRandomTurnNumber).cast::<u8>().cast::<u16>()).write(Random());
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 262144u32) != 0 {
            StopCryAndClearCrySongs();
            BattleScriptExecute((&raw mut BattleScript_ArenaTurnBeginning).cast::<u8>());
        }
    }
}
pub(crate) unsafe extern "C" fn HandleEndTurn_ContinueBattle() {
    unsafe {
        let mut i: i32 = 0i32;
        if ((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read()
            == 0u32
        {
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(BattleTurnPassed));
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 8i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l3: loop {
                    if !(i
                        < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                    {
                        break 'l3;
                    }
                    'l4: {
                        let __p1 = ((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(80)
                        .cast::<u32>();
                        (__p1).write(((__p1).read() & 4294967287u32));
                        if (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(76)
                        .cast::<u32>())
                        .read()
                            & 7u32)
                            != 0)
                            && (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 88))
                            .wrapping_add(80)
                            .cast::<u32>())
                            .read()
                                & 4096u32)
                                != 0)
                        {
                            CancelMultiTurnMoves(((i) as u8));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
            ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                .write(0u8);
            ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(416))
                .write(0u8);
            ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(417))
                .write(0u8);
            ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                .write(0u8);
            ((&raw mut gMoveResultFlags).cast::<u8>().cast::<u8>()).write(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleTurnPassed() {
    unsafe {
        let mut i: i32 = 0i32;
        TurnValuesCleanUp(1u8);
        if ((((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).read()) as i32) == 0i32 {
            if (DoFieldEndTurnEffects()) != 0 {
                return;
            }
            if (DoBattlerEndTurnEffects()) != 0 {
                return;
            }
        }
        if (HandleFaintedMonActions()) != 0 {
            return;
        }
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(77))
            .write(0u8);
        if (HandleWishPerishSongOnTurnEnd()) != 0 {
            return;
        }
        TurnValuesCleanUp(0u8);
        let __p1 = (&raw mut gHitMarker).cast::<u8>().cast::<u32>();
        (__p1).write(((__p1).read() & 4294966783u32));
        let __p2 = (&raw mut gHitMarker).cast::<u8>().cast::<u32>();
        (__p2).write(((__p2).read() & 4294443007u32));
        let __p3 = (&raw mut gHitMarker).cast::<u8>().cast::<u32>();
        (__p3).write(((__p3).read() & 4290772991u32));
        let __p4 = (&raw mut gHitMarker).cast::<u8>().cast::<u32>();
        (__p4).write(((__p4).read() & 4293918719u32));
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(24)).write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(25)).write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(20)).write(0u8);
        ((&raw mut gBattleMoveDamage).cast::<u8>().cast::<i32>()).write(0i32);
        ((&raw mut gMoveResultFlags).cast::<u8>().cast::<u8>()).write(0u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).read()) as i32) != 0i32 {
            ((&raw mut gCurrentActionFuncId).cast::<u8>().cast::<u8>()).write(12u8);
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(RunTurnActionsFunctions));
            return;
        }
        if (((((&raw mut gBattleResults).cast::<u8>()).wrapping_add(19)).read()) as i32) < 255i32 {
            let __p5 = ((&raw mut gBattleResults).cast::<u8>()).wrapping_add(19);
            (__p5).write(((__p5).read()).wrapping_add(1));
            let __p6 = (((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(218);
            (__p6).write(((__p6).read()).wrapping_add(1));
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)) {
                    break 'l3;
                }
                'l4: {
                    ((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(255u8);
                    ((((&raw mut gChosenMoveByBattler).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 4i32) {
                    break 'l5;
                }
                'l6: {
                    ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(92))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(6u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(145))
            .write(((&raw mut gAbsentBattlerFlags).cast::<u8>().cast::<u8>()).read());
        BattlePutTextOnWindow((&raw mut gText_EmptyString3).cast::<u8>(), 0u8);
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(HandleTurnActionSelectionState));
        ((&raw mut gRandomTurnNumber).cast::<u8>().cast::<u16>()).write(Random());
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 131072u32) != 0 {
            BattleScriptExecute((&raw mut BattleScript_PalacePrintFlavorText).cast::<u8>());
        } else {
            if ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 262144u32) != 0)
                && (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(218))
                .read()) as i32)
                    == 0i32)
            {
                BattleScriptExecute((&raw mut BattleScript_ArenaTurnBeginning).cast::<u8>());
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRunningFromBattleImpossible() -> u8 {
    unsafe {
        let mut holdEffect: u8 = 0u8;
        let mut side: u8 = 0u8;
        let mut i: i32 = 0i32;
        if (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(46)
        .cast::<u16>())
        .read()) as i32)
            == 175i32
        {
            holdEffect = (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 28,
                ))
            .wrapping_add(7))
            .read();
        } else {
            holdEffect = GetItemHoldEffect(
                (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 88,
                ))
                .wrapping_add(46)
                .cast::<u16>())
                .read(),
            );
        }
        ((&raw mut gPotentialItemEffectBattler)
            .cast::<u8>()
            .cast::<u8>())
        .write(((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read());
        if ((holdEffect) as i32) == 37i32 {
            return 0u8;
        }
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0 {
            return 0u8;
        }
        if (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(32))
        .read()) as i32)
            == 50i32
        {
            return 0u8;
        }
        side = GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read());
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((side) as i32) != ((GetBattlerSide(((i) as u8))) as i32))
                        && ((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(32))
                        .read()) as i32)
                            == 23i32)
                    {
                        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                            .write(((i) as u8));
                        ((&raw mut gLastUsedAbility).cast::<u8>().cast::<u8>()).write(
                            (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 88))
                            .wrapping_add(32))
                            .read(),
                        );
                        ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(5))
                        .write(2u8);
                        return 2u8;
                    }
                    if (((((side) as i32) != ((GetBattlerSide(((i) as u8))) as i32))
                        && ((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize
                                    * 88,
                            ))
                        .wrapping_add(32))
                        .read()) as i32)
                            != 26i32))
                        && (!((((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize
                                    * 88,
                            ))
                        .wrapping_add(33))
                        .cast::<u8>())
                        .read()) as i32)
                            == 2i32)
                            || ((((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize
                                        * 88,
                                ))
                            .wrapping_add(33))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                == 2i32))))
                        && ((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 88))
                        .wrapping_add(32))
                        .read()) as i32)
                            == 71i32)
                    {
                        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                            .write(((i) as u8));
                        ((&raw mut gLastUsedAbility).cast::<u8>().cast::<u8>()).write(
                            (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 88))
                            .wrapping_add(32))
                            .read(),
                        );
                        ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(5))
                        .write(2u8);
                        return 2u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        i = ((AbilityBattleEffects(
            15u8,
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
            42u8,
            0u8,
            0u16,
        )) as i32);
        if (i != 0i32)
            && ((((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 88,
            ))
            .wrapping_add(33))
            .cast::<u8>())
            .read()) as i32)
                == 8i32)
                || ((((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 88,
                ))
                .wrapping_add(33))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 8i32))
        {
            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                .write((((i).wrapping_sub(1i32)) as u8));
            ((&raw mut gLastUsedAbility).cast::<u8>().cast::<u8>()).write(
                (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((i).wrapping_sub(1i32)) as isize * 88))
                .wrapping_add(32))
                .read(),
            );
            ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(5))
                .write(2u8);
            return 2u8;
        }
        if (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(80)
        .cast::<u32>())
        .read()
            & 67166208u32)
            != 0)
            || ((((((&raw mut gStatuses3).cast::<u8>().cast::<u32>()).cast::<u32>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize,
                ))
            .read()
                & 1024u32)
                != 0)
        {
            ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(5))
                .write(0u8);
            return 1u8;
        }
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 16u32) != 0 {
            ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(5))
                .write(1u8);
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwitchPartyOrder(battler: u8) {
    unsafe {
        let mut battler = battler;
        let mut i: i32 = 0i32;
        let mut partyId1: u8 = 0u8;
        let mut partyId2: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(3u32, 1u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut gBattlePartyCurrentOrder).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(96))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((battler) as i32).wrapping_mul(3i32)).wrapping_add(i)) as isize,
                        ))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        partyId1 = GetPartyIdFromBattlePartyId(
            ((((((&raw mut gBattlerPartyIndexes).cast::<u8>().cast::<u16>()).cast::<u16>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as u8),
        );
        partyId2 = GetPartyIdFromBattlePartyId(
            ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(92))
            .cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize))
            .read(),
        );
        SwitchPartyMonSlots(partyId1, partyId2);
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 1u32) != 0 {
            {
                i = 0i32;
                'l3: loop {
                    if !(i < ((crate::c::div_u32(3u32, 1u32)) as i32)) {
                        break 'l3;
                    }
                    'l4: {
                        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(96))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((battler) as i32).wrapping_mul(3i32)).wrapping_add(i)) as isize,
                        ))
                        .write(
                            (((&raw mut gBattlePartyCurrentOrder).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read(),
                        );
                        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(96))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((battler) as i32) ^ 2i32).wrapping_mul(3i32)).wrapping_add(i))
                                as isize,
                        ))
                        .write(
                            (((&raw mut gBattlePartyCurrentOrder).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0i32;
                'l5: loop {
                    if !(i < ((crate::c::div_u32(3u32, 1u32)) as i32)) {
                        break 'l5;
                    }
                    'l6: {
                        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(96))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((battler) as i32).wrapping_mul(3i32)).wrapping_add(i)) as isize,
                        ))
                        .write(
                            (((&raw mut gBattlePartyCurrentOrder).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandleTurnActionSelectionState() {
    unsafe {
        let mut i: i32 = 0i32;
        ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(4))
            .write(0u8);
        {
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
            'l1: loop {
                if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut position: u8 = GetBattlerPosition(
                        ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                    );
                    'l3: {
                        let __sw1 = ((((((&raw mut gBattleCommunication).cast::<u8>())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                        .read()) as i32);
                        if __sw1 == 0i32 {
                            RecordedBattle_CopyBattlerMoves();
                            ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize,
                                ))
                            .write(1u8);
                            break 'l3;
                        }
                        if __sw1 == 1i32 {
                            ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(92))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize,
                            ))
                            .write(6u8);
                            if ((((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>())
                                .read()
                                & 64u32)
                                != 0)
                                || ((((position) as i32) & 2i32) == 0i32))
                                || ((((((((&raw mut gBattleStruct)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(145))
                                .read()) as u32)
                                    & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((GetBattlerAtPosition(
                                                ((((position) as i32) ^ 2i32) as u8),
                                            )) as i32)
                                                as isize,
                                        ))
                                    .read())
                                    != 0))
                                || (((((((&raw mut gBattleCommunication).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((GetBattlerAtPosition(((((position) as i32) ^ 2i32) as u8)))
                                        as i32) as isize,
                                ))
                                .read()) as i32)
                                    == 5i32)
                            {
                                if (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(145))
                                .read()) as u32)
                                    & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read())
                                    != 0
                                {
                                    ((((&raw mut gChosenActionByBattler).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                    .write(13u8);
                                    if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>())
                                        .read()
                                        & 64u32)
                                        != 0)
                                    {
                                        ((((&raw mut gBattleCommunication).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .write(5u8);
                                    } else {
                                        ((((&raw mut gBattleCommunication).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .write(4u8);
                                    }
                                } else {
                                    if (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(80)
                                    .cast::<u32>())
                                    .read()
                                        & 4096u32)
                                        != 0)
                                        || (((((((&raw mut gBattleMons).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                        .wrapping_add(80)
                                        .cast::<u32>())
                                        .read()
                                            & 4194304u32)
                                            != 0)
                                    {
                                        ((((&raw mut gChosenActionByBattler).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .write(0u8);
                                        ((((&raw mut gBattleCommunication).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .write(4u8);
                                    } else {
                                        BtlController_EmitChooseAction(
                                            0u8,
                                            (((&raw mut gChosenActionByBattler).cast::<u8>())
                                                .cast::<u8>())
                                            .read(),
                                            (((((((((&raw mut gBattleBufferB).cast::<u8>())
                                                .cast::<u8>())
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                | ((((((((&raw mut gBattleBufferB).cast::<u8>())
                                                    .cast::<u8>())
                                                .cast::<u8>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    << 8))
                                                as u16),
                                        );
                                        MarkBattlerForControllerExec(
                                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read(),
                                        );
                                        let __p2 = (((&raw mut gBattleCommunication).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        );
                                        (__p2).write(((__p2).read()).wrapping_add(1));
                                    }
                                }
                            }
                            break 'l3;
                        }
                        if __sw1 == 2i32 {
                            if !((((&raw mut gBattleControllerExecFlags)
                                .cast::<u8>()
                                .cast::<u32>())
                            .read()
                                & ((((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()
                                    | 4026531840u32)
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 4))
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 8))
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 12)))
                                != 0)
                            {
                                RecordedBattle_SetBattlerAction(
                                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                                    ((((((&raw mut gBattleBufferB).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 512,
                                        ))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read(),
                                );
                                ((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                .write(
                                    ((((((&raw mut gBattleBufferB).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 512,
                                        ))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read(),
                                );
                                'l4: {
                                    let __sw3 = ((((((((&raw mut gBattleBufferB).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 512,
                                    ))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read())
                                        as i32);
                                    if __sw3 == 0i32 {
                                        if (AreAllMovesUnusable()) != 0 {
                                            ((((&raw mut gBattleCommunication).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(6u8);
                                            ((((((&raw mut gBattleStruct)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(84))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(0u8);
                                            ((((((&raw mut gBattleStruct)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(132))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(4u8);
                                            ((((((&raw mut gBattleStruct)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(12))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(
                                                ((((((&raw mut gBattleBufferB).cast::<u8>())
                                                    .cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 512,
                                                ))
                                                .cast::<u8>())
                                                .wrapping_offset(3))
                                                .read(),
                                            );
                                            return;
                                        } else {
                                            if (((((((&raw mut gDisableStructs).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 28,
                                            ))
                                            .wrapping_add(6)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                != 0i32
                                            {
                                                ((((&raw mut gChosenMoveByBattler)
                                                    .cast::<u8>()
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .write(
                                                    (((((&raw mut gDisableStructs).cast::<u8>())
                                                        .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 28,
                                                    ))
                                                    .wrapping_add(6)
                                                    .cast::<u16>())
                                                    .read(),
                                                );
                                                ((((((&raw mut gBattleStruct)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(128))
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .write(
                                                    (((((&raw mut gDisableStructs).cast::<u8>())
                                                        .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 28,
                                                    ))
                                                    .wrapping_add(12))
                                                    .read(),
                                                );
                                                ((((&raw mut gBattleCommunication).cast::<u8>())
                                                    .cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .write(4u8);
                                                return;
                                            } else {
                                                let mut moveInfo = crate::ffi::Align4([0u8; 20]);
                                                (((&raw mut moveInfo).cast::<u8>())
                                                    .wrapping_add(16)
                                                    .cast::<u16>())
                                                .write(
                                                    (((((&raw mut gBattleMons).cast::<u8>())
                                                        .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 88,
                                                    ))
                                                    .cast::<u16>())
                                                    .read(),
                                                );
                                                ((((&raw mut moveInfo).cast::<u8>())
                                                    .wrapping_add(18))
                                                .cast::<u8>())
                                                .write(
                                                    ((((((&raw mut gBattleMons).cast::<u8>())
                                                        .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 88,
                                                    ))
                                                    .wrapping_add(33))
                                                    .cast::<u8>())
                                                    .read(),
                                                );
                                                (((((&raw mut moveInfo).cast::<u8>())
                                                    .wrapping_add(18))
                                                .cast::<u8>())
                                                .wrapping_offset(1))
                                                .write(
                                                    (((((((&raw mut gBattleMons).cast::<u8>())
                                                        .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 88,
                                                    ))
                                                    .wrapping_add(33))
                                                    .cast::<u8>())
                                                    .wrapping_offset(1))
                                                    .read(),
                                                );
                                                {
                                                    i = 0i32;
                                                    'l5: loop {
                                                        if !(i < 4i32) {
                                                            break 'l5;
                                                        }
                                                        'l6: {
                                                            ((((&raw mut moveInfo).cast::<u8>())
                                                                .cast::<u16>())
                                                            .wrapping_offset((i) as isize))
                                                            .write(
                                                                (((((((&raw mut gBattleMons)
                                                                    .cast::<u8>())
                                                                .cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((((&raw mut gActiveBattler)
                                                                        .cast::<u8>()
                                                                        .cast::<u8>())
                                                                    .read())
                                                                        as i32)
                                                                        as isize
                                                                        * 88,
                                                                ))
                                                                .wrapping_add(12))
                                                                .cast::<u16>())
                                                                .wrapping_offset((i) as isize))
                                                                .read(),
                                                            );
                                                            (((((&raw mut moveInfo)
                                                                .cast::<u8>())
                                                            .wrapping_add(8))
                                                            .cast::<u8>())
                                                            .wrapping_offset((i) as isize))
                                                            .write(
                                                                (((((((&raw mut gBattleMons)
                                                                    .cast::<u8>())
                                                                .cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((((&raw mut gActiveBattler)
                                                                        .cast::<u8>()
                                                                        .cast::<u8>())
                                                                    .read())
                                                                        as i32)
                                                                        as isize
                                                                        * 88,
                                                                ))
                                                                .wrapping_add(36))
                                                                .cast::<u8>())
                                                                .wrapping_offset((i) as isize))
                                                                .read(),
                                                            );
                                                            (((((&raw mut moveInfo)
                                                                .cast::<u8>())
                                                            .wrapping_add(12))
                                                            .cast::<u8>())
                                                            .wrapping_offset((i) as isize))
                                                            .write(CalculatePPWithBonus(
                                                                (((((((&raw mut gBattleMons)
                                                                    .cast::<u8>())
                                                                .cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((((&raw mut gActiveBattler)
                                                                        .cast::<u8>()
                                                                        .cast::<u8>())
                                                                    .read())
                                                                        as i32)
                                                                        as isize
                                                                        * 88,
                                                                ))
                                                                .wrapping_add(12))
                                                                .cast::<u16>())
                                                                .wrapping_offset((i) as isize))
                                                                .read(),
                                                                (((((&raw mut gBattleMons)
                                                                    .cast::<u8>())
                                                                .cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((((&raw mut gActiveBattler)
                                                                        .cast::<u8>()
                                                                        .cast::<u8>())
                                                                    .read())
                                                                        as i32)
                                                                        as isize
                                                                        * 88,
                                                                ))
                                                                .wrapping_add(59))
                                                                .read(),
                                                                ((i) as u8),
                                                            ));
                                                        }
                                                        i = (i).wrapping_add(1);
                                                    }
                                                }
                                                BtlController_EmitChooseMove(
                                                    0u8,
                                                    (((((&raw mut gBattleTypeFlags)
                                                        .cast::<u8>()
                                                        .cast::<u32>())
                                                    .read()
                                                        & 1u32)
                                                        != 0u32)
                                                        as u8),
                                                    0u8,
                                                    (&raw mut moveInfo).cast::<u8>(),
                                                );
                                                MarkBattlerForControllerExec(
                                                    ((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read(),
                                                );
                                            }
                                        }
                                        break 'l4;
                                    }
                                    if __sw3 == 1i32 {
                                        if (((&raw mut gBattleTypeFlags)
                                            .cast::<u8>()
                                            .cast::<u32>())
                                        .read()
                                            & 35588354u32)
                                            != 0
                                        {
                                            RecordedBattle_ClearBattlerAction(
                                                ((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read(),
                                                1u8,
                                            );
                                            ((((&raw mut gSelectionBattleScripts).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>()).wrapping_offset((((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)) as isize)).write(((&raw mut BattleScript_ActionSelectionItemsCantBeUsed)).cast::<u8>());
                                            ((((&raw mut gBattleCommunication).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(6u8);
                                            ((((((&raw mut gBattleStruct)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(84))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(0u8);
                                            ((((((&raw mut gBattleStruct)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(132))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(1u8);
                                            return;
                                        } else {
                                            BtlController_EmitChooseItem(
                                                0u8,
                                                ((((((&raw mut gBattleStruct)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(96))
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 3,
                                                ))
                                                .cast::<u8>(),
                                            );
                                            MarkBattlerForControllerExec(
                                                ((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read(),
                                            );
                                        }
                                        break 'l4;
                                    }
                                    if __sw3 == 2i32 {
                                        ((((((&raw mut gBattleStruct)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(88))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .write(
                                            ((((((&raw mut gBattlerPartyIndexes)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .read())
                                                as u8),
                                        );
                                        if ((((((((&raw mut gBattleMons).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                        .wrapping_add(80)
                                        .cast::<u32>())
                                        .read()
                                            & 67166208u32)
                                            != 0)
                                            || ((((&raw mut gBattleTypeFlags)
                                                .cast::<u8>()
                                                .cast::<u32>())
                                            .read()
                                                & 262144u32)
                                                != 0))
                                            || ((((((&raw mut gStatuses3)
                                                .cast::<u8>()
                                                .cast::<u32>())
                                            .cast::<u32>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .read()
                                                & 1024u32)
                                                != 0)
                                        {
                                            BtlController_EmitChoosePokemon(
                                                0u8,
                                                2u8,
                                                6u8,
                                                0u8,
                                                ((((((&raw mut gBattleStruct)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(96))
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 3,
                                                ))
                                                .cast::<u8>(),
                                            );
                                        } else {
                                            if ((({
                                                let __v4 = ((AbilityBattleEffects(
                                                    12u8,
                                                    ((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read(),
                                                    23u8,
                                                    0u8,
                                                    0u16,
                                                ))
                                                    as i32);
                                                i = __v4;
                                                __v4
                                            }) != 0)
                                                || (((({
                                                    let __v5 = ((AbilityBattleEffects(
                                                        12u8,
                                                        ((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read(),
                                                        71u8,
                                                        0u8,
                                                        0u16,
                                                    ))
                                                        as i32);
                                                    i = __v5;
                                                    __v5
                                                }) != 0)
                                                    && (!((((((((((&raw mut gBattleMons)
                                                        .cast::<u8>())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 88,
                                                    ))
                                                    .wrapping_add(33))
                                                    .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        == 2i32)
                                                        || ((((((((((&raw mut gBattleMons)
                                                            .cast::<u8>())
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 88,
                                                        ))
                                                        .wrapping_add(33))
                                                        .cast::<u8>())
                                                        .wrapping_offset(1))
                                                        .read())
                                                            as i32)
                                                            == 2i32))))
                                                    && ((((((((&raw mut gBattleMons)
                                                        .cast::<u8>())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 88,
                                                    ))
                                                    .wrapping_add(32))
                                                    .read())
                                                        as i32)
                                                        != 26i32)))
                                                || ((({
                                                    let __v6 = ((AbilityBattleEffects(
                                                        15u8,
                                                        ((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read(),
                                                        42u8,
                                                        0u8,
                                                        0u16,
                                                    ))
                                                        as i32);
                                                    i = __v6;
                                                    __v6
                                                }) != 0)
                                                    && ((((((((((&raw mut gBattleMons)
                                                        .cast::<u8>())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 88,
                                                    ))
                                                    .wrapping_add(33))
                                                    .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        == 8i32)
                                                        || ((((((((((&raw mut gBattleMons)
                                                            .cast::<u8>())
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 88,
                                                        ))
                                                        .wrapping_add(33))
                                                        .cast::<u8>())
                                                        .wrapping_offset(1))
                                                        .read())
                                                            as i32)
                                                            == 8i32)))
                                            {
                                                BtlController_EmitChoosePokemon(
                                                    0u8,
                                                    ((((i).wrapping_sub(1i32) << 4) | 4i32) as u8),
                                                    6u8,
                                                    ((&raw mut gLastUsedAbility)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read(),
                                                    ((((((&raw mut gBattleStruct)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(96))
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 3,
                                                    ))
                                                    .cast::<u8>(),
                                                );
                                            } else {
                                                if (((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    == 2i32)
                                                    && ((((((&raw mut gChosenActionByBattler)
                                                        .cast::<u8>())
                                                    .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        == 2i32)
                                                {
                                                    BtlController_EmitChoosePokemon(
                                                        0u8,
                                                        0u8,
                                                        (((((&raw mut gBattleStruct)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(92))
                                                        .cast::<u8>())
                                                        .read(),
                                                        0u8,
                                                        ((((((&raw mut gBattleStruct)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(96))
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 3,
                                                        ))
                                                        .cast::<u8>(),
                                                    );
                                                } else {
                                                    if ((((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)) == 3i32) && ((((((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>()).wrapping_offset(1)).read()) as i32)) == 2i32) {
BtlController_EmitChoosePokemon(0u8, 0u8, ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(92)).cast::<u8>()).wrapping_offset(1)).read(), 0u8, ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(96)).cast::<u8>()).wrapping_offset((((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)) as isize * 3)).cast::<u8>());
} else {
BtlController_EmitChoosePokemon(0u8, 0u8, 6u8, 0u8, ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(96)).cast::<u8>()).wrapping_offset((((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)) as isize * 3)).cast::<u8>());
}
                                                }
                                            }
                                        }
                                        MarkBattlerForControllerExec(
                                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read(),
                                        );
                                        break 'l4;
                                    }
                                    if __sw3 == 5i32 {
                                        if (IsPlayerPartyAndPokemonStorageFull()) != 0 {
                                            ((((&raw mut gSelectionBattleScripts)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .cast::<*mut u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(
                                                (&raw mut BattleScript_PrintFullBox).cast::<u8>(),
                                            );
                                            ((((&raw mut gBattleCommunication).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(6u8);
                                            ((((((&raw mut gBattleStruct)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(84))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(0u8);
                                            ((((((&raw mut gBattleStruct)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(132))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(1u8);
                                            return;
                                        }
                                        break 'l4;
                                    }
                                    if __sw3 == 6i32 {
                                        BtlController_EmitChooseItem(
                                            0u8,
                                            ((((((&raw mut gBattleStruct)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(96))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 3,
                                            ))
                                            .cast::<u8>(),
                                        );
                                        MarkBattlerForControllerExec(
                                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read(),
                                        );
                                        break 'l4;
                                    }
                                    if __sw3 == 12i32 {
                                        ((((&raw mut gBattleCommunication).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .write(7u8);
                                        ((((&raw mut gBattleCommunication).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((GetBattlerAtPosition(
                                                ((((GetBattlerPosition(
                                                    ((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read(),
                                                ))
                                                    as i32)
                                                    ^ 2i32)
                                                    as u8),
                                            )) as i32)
                                                as isize,
                                        ))
                                        .write(1u8);
                                        RecordedBattle_ClearBattlerAction(
                                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read(),
                                            1u8,
                                        );
                                        if (((((((&raw mut gBattleMons).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((GetBattlerAtPosition(
                                                ((((GetBattlerPosition(
                                                    ((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read(),
                                                ))
                                                    as i32)
                                                    ^ 2i32)
                                                    as u8),
                                            )) as i32)
                                                as isize
                                                * 88,
                                        ))
                                        .wrapping_add(80)
                                        .cast::<u32>())
                                        .read()
                                            & 4096u32)
                                            != 0)
                                            || (((((((&raw mut gBattleMons).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                ((GetBattlerAtPosition(
                                                    ((((GetBattlerPosition(
                                                        ((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read(),
                                                    ))
                                                        as i32)
                                                        ^ 2i32)
                                                        as u8),
                                                ))
                                                    as i32)
                                                    as isize
                                                    * 88,
                                            ))
                                            .wrapping_add(80)
                                            .cast::<u32>())
                                            .read()
                                                & 4194304u32)
                                                != 0)
                                        {
                                            BtlController_EmitEndBounceEffect(0u8);
                                            MarkBattlerForControllerExec(
                                                ((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read(),
                                            );
                                            return;
                                        } else {
                                            if ((((((&raw mut gChosenActionByBattler)
                                                .cast::<u8>())
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((GetBattlerAtPosition(
                                                    ((((GetBattlerPosition(
                                                        ((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read(),
                                                    ))
                                                        as i32)
                                                        ^ 2i32)
                                                        as u8),
                                                ))
                                                    as i32)
                                                    as isize,
                                            ))
                                            .read())
                                                as i32)
                                                == 2i32
                                            {
                                                RecordedBattle_ClearBattlerAction(
                                                    GetBattlerAtPosition(
                                                        ((((GetBattlerPosition(
                                                            ((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read(),
                                                        ))
                                                            as i32)
                                                            ^ 2i32)
                                                            as u8),
                                                    ),
                                                    2u8,
                                                );
                                            } else {
                                                if ((((((&raw mut gChosenActionByBattler)
                                                    .cast::<u8>())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((GetBattlerAtPosition(
                                                        ((((GetBattlerPosition(
                                                            ((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read(),
                                                        ))
                                                            as i32)
                                                            ^ 2i32)
                                                            as u8),
                                                    ))
                                                        as i32)
                                                        as isize,
                                                ))
                                                .read())
                                                    as i32)
                                                    == 3i32
                                                {
                                                    RecordedBattle_ClearBattlerAction(
                                                        GetBattlerAtPosition(
                                                            ((((GetBattlerPosition(
                                                                ((&raw mut gActiveBattler)
                                                                    .cast::<u8>()
                                                                    .cast::<u8>())
                                                                .read(),
                                                            ))
                                                                as i32)
                                                                ^ 2i32)
                                                                as u8),
                                                        ),
                                                        1u8,
                                                    );
                                                } else {
                                                    if (((((((&raw mut gChosenActionByBattler)
                                                        .cast::<u8>())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((GetBattlerAtPosition(
                                                            ((((GetBattlerPosition(
                                                                ((&raw mut gActiveBattler)
                                                                    .cast::<u8>()
                                                                    .cast::<u8>())
                                                                .read(),
                                                            ))
                                                                as i32)
                                                                ^ 2i32)
                                                                as u8),
                                                        ))
                                                            as i32)
                                                            as isize,
                                                    ))
                                                    .read())
                                                        as i32)
                                                        == 0i32)
                                                        && (((crate::c::bf_read(
                                                            ((((&raw mut gProtectStructs)
                                                                .cast::<u8>())
                                                            .cast::<u8>())
                                                            .wrapping_offset(
                                                                ((GetBattlerAtPosition(
                                                                    ((((GetBattlerPosition(
                                                                        ((&raw mut gActiveBattler)
                                                                            .cast::<u8>()
                                                                            .cast::<u8>())
                                                                        .read(),
                                                                    ))
                                                                        as i32)
                                                                        ^ 2i32)
                                                                        as u8),
                                                                ))
                                                                    as i32)
                                                                    as isize
                                                                    * 16,
                                                            ))
                                                            .wrapping_add(0),
                                                            2,
                                                            1,
                                                            false,
                                                        )
                                                            as u32)
                                                            != 0)
                                                            || (((((((&raw mut gDisableStructs)
                                                                .cast::<u8>())
                                                            .cast::<u8>())
                                                            .wrapping_offset(
                                                                ((GetBattlerAtPosition(
                                                                    ((((GetBattlerPosition(
                                                                        ((&raw mut gActiveBattler)
                                                                            .cast::<u8>()
                                                                            .cast::<u8>())
                                                                        .read(),
                                                                    ))
                                                                        as i32)
                                                                        ^ 2i32)
                                                                        as u8),
                                                                ))
                                                                    as i32)
                                                                    as isize
                                                                    * 28,
                                                            ))
                                                            .wrapping_add(6)
                                                            .cast::<u16>())
                                                            .read())
                                                                != 0))
                                                    {
                                                        RecordedBattle_ClearBattlerAction(
                                                            GetBattlerAtPosition(
                                                                ((((GetBattlerPosition(
                                                                    ((&raw mut gActiveBattler)
                                                                        .cast::<u8>()
                                                                        .cast::<u8>())
                                                                    .read(),
                                                                ))
                                                                    as i32)
                                                                    ^ 2i32)
                                                                    as u8),
                                                            ),
                                                            1u8,
                                                        );
                                                    } else {
                                                        if (((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 131072u32)) != 0) && ((((((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>()).wrapping_offset((((GetBattlerAtPosition(((((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())) as i32)) ^ 2i32)) as u8))) as i32)) as isize)).read()) as i32)) == 0i32) {
((&raw mut gRngValue).cast::<u32>()).write(((&raw mut gBattlePalaceMoveSelectionRngValue).cast::<u32>()).read());
RecordedBattle_ClearBattlerAction(GetBattlerAtPosition(((((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())) as i32)) ^ 2i32)) as u8)), 1u8);
} else {
RecordedBattle_ClearBattlerAction(GetBattlerAtPosition(((((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())) as i32)) ^ 2i32)) as u8)), 3u8);
}
                                                    }
                                                }
                                            }
                                        }
                                        BtlController_EmitEndBounceEffect(0u8);
                                        MarkBattlerForControllerExec(
                                            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read(),
                                        );
                                        return;
                                    }
                                }
                                if (((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>())
                                    .read()
                                    & 8u32)
                                    != 0)
                                    && ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>())
                                        .read()
                                        & 71237888u32)
                                        != 0))
                                    && (((((((((&raw mut gBattleBufferB).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 512,
                                    ))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        == 3i32)
                                {
                                    ((((&raw mut gSelectionBattleScripts)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                    .write(
                                        (&raw mut BattleScript_AskIfWantsToForfeitMatch)
                                            .cast::<u8>(),
                                    );
                                    ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .write(8u8);
                                    ((((((&raw mut gBattleStruct)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(84))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                    .write(0u8);
                                    ((((((&raw mut gBattleStruct)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(132))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                    .write(1u8);
                                    return;
                                } else {
                                    if (((((&raw mut gBattleTypeFlags)
                                        .cast::<u8>()
                                        .cast::<u32>())
                                    .read()
                                        & 8u32)
                                        != 0)
                                        && (!((((&raw mut gBattleTypeFlags)
                                            .cast::<u8>()
                                            .cast::<u32>())
                                        .read()
                                            & 33554434u32)
                                            != 0)))
                                        && (((((((((&raw mut gBattleBufferB).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 512,
                                        ))
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read())
                                            as i32)
                                            == 3i32)
                                    {
                                        BattleScriptExecute(
                                            (&raw mut BattleScript_PrintCantRunFromTrainer)
                                                .cast::<u8>(),
                                        );
                                        ((((&raw mut gBattleCommunication).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .write(1u8);
                                    } else {
                                        if (((IsRunningFromBattleImpossible()) as i32) != 0i32)
                                            && (((((((((&raw mut gBattleBufferB).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 512,
                                            ))
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                == 3i32)
                                        {
                                            ((((&raw mut gSelectionBattleScripts)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .cast::<*mut u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(
                                                (&raw mut BattleScript_PrintCantEscapeFromBattle)
                                                    .cast::<u8>(),
                                            );
                                            ((((&raw mut gBattleCommunication).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(6u8);
                                            ((((((&raw mut gBattleStruct)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(84))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(0u8);
                                            ((((((&raw mut gBattleStruct)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(132))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(1u8);
                                            return;
                                        } else {
                                            let __p7 = (((&raw mut gBattleCommunication)
                                                .cast::<u8>())
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            );
                                            (__p7).write(((__p7).read()).wrapping_add(1));
                                        }
                                    }
                                }
                            }
                            break 'l3;
                        }
                        if __sw1 == 3i32 {
                            if !((((&raw mut gBattleControllerExecFlags)
                                .cast::<u8>()
                                .cast::<u32>())
                            .read()
                                & ((((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()
                                    | 4026531840u32)
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 4))
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 8))
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 12)))
                                != 0)
                            {
                                'l7: {
                                    let __sw8 =
                                        ((((((&raw mut gChosenActionByBattler).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read()) as i32);
                                    if __sw8 == 0i32 {
                                        'l8: {
                                            let __sw9 = ((((((((&raw mut gBattleBufferB)
                                                .cast::<u8>())
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 512,
                                            ))
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32);
                                            let __matched = __sw9 == 3i32
                                                || __sw9 == 4i32
                                                || __sw9 == 5i32
                                                || __sw9 == 6i32
                                                || __sw9 == 7i32
                                                || __sw9 == 8i32
                                                || __sw9 == 9i32
                                                || __sw9 == 15i32;
                                            if __sw9 == 3i32
                                                || __sw9 == 4i32
                                                || __sw9 == 5i32
                                                || __sw9 == 6i32
                                                || __sw9 == 7i32
                                                || __sw9 == 8i32
                                                || __sw9 == 9i32
                                            {
                                                ((((&raw mut gChosenActionByBattler)
                                                    .cast::<u8>())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .write(
                                                    ((((((&raw mut gBattleBufferB)
                                                        .cast::<u8>())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 512,
                                                    ))
                                                    .cast::<u8>())
                                                    .wrapping_offset(1))
                                                    .read(),
                                                );
                                                return;
                                            }
                                            if __sw9 == 15i32 {
                                                ((((&raw mut gChosenActionByBattler)
                                                    .cast::<u8>())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .write(2u8);
                                                UpdateBattlerPartyOrdersOnSwitch();
                                                return;
                                            }
                                            if !__matched {
                                                RecordedBattle_CheckMovesetChanges(2u8);
                                                if (((((((((&raw mut gBattleBufferB)
                                                    .cast::<u8>())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 512,
                                                ))
                                                .cast::<u8>())
                                                .wrapping_offset(2))
                                                .read())
                                                    as i32)
                                                    | (((((((((&raw mut gBattleBufferB)
                                                        .cast::<u8>())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 512,
                                                    ))
                                                    .cast::<u8>())
                                                    .wrapping_offset(3))
                                                    .read())
                                                        as i32)
                                                        << 8))
                                                    == 65535i32
                                                {
                                                    ((((&raw mut gBattleCommunication)
                                                        .cast::<u8>())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize,
                                                    ))
                                                    .write(1u8);
                                                    RecordedBattle_ClearBattlerAction(
                                                        ((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read(),
                                                        1u8,
                                                    );
                                                } else {
                                                    if (TrySetCantSelectMoveBattleScript()) != 0 {
                                                        RecordedBattle_ClearBattlerAction(
                                                            ((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read(),
                                                            1u8,
                                                        );
                                                        ((((&raw mut gBattleCommunication)
                                                            .cast::<u8>())
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(6u8);
                                                        ((((((&raw mut gBattleStruct)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(84))
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(0u8);
                                                        ((((((&raw mut gBattleBufferB)
                                                            .cast::<u8>())
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 512,
                                                        ))
                                                        .cast::<u8>())
                                                        .wrapping_offset(1))
                                                        .write(0u8);
                                                        ((((((&raw mut gBattleStruct)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(132))
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(2u8);
                                                        return;
                                                    } else {
                                                        if !((((&raw mut gBattleTypeFlags)
                                                            .cast::<u8>()
                                                            .cast::<u32>())
                                                        .read()
                                                            & 131072u32)
                                                            != 0)
                                                        {
                                                            RecordedBattle_SetBattlerAction(
                                                                ((&raw mut gActiveBattler)
                                                                    .cast::<u8>()
                                                                    .cast::<u8>())
                                                                .read(),
                                                                ((((((&raw mut gBattleBufferB)
                                                                    .cast::<u8>())
                                                                .cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((((&raw mut gActiveBattler)
                                                                        .cast::<u8>()
                                                                        .cast::<u8>())
                                                                    .read())
                                                                        as i32)
                                                                        as isize
                                                                        * 512,
                                                                ))
                                                                .cast::<u8>())
                                                                .wrapping_offset(2))
                                                                .read(),
                                                            );
                                                            RecordedBattle_SetBattlerAction(
                                                                ((&raw mut gActiveBattler)
                                                                    .cast::<u8>()
                                                                    .cast::<u8>())
                                                                .read(),
                                                                ((((((&raw mut gBattleBufferB)
                                                                    .cast::<u8>())
                                                                .cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((((&raw mut gActiveBattler)
                                                                        .cast::<u8>()
                                                                        .cast::<u8>())
                                                                    .read())
                                                                        as i32)
                                                                        as isize
                                                                        * 512,
                                                                ))
                                                                .cast::<u8>())
                                                                .wrapping_offset(3))
                                                                .read(),
                                                            );
                                                        }
                                                        ((((((&raw mut gBattleStruct)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(128))
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(
                                                            ((((((&raw mut gBattleBufferB)
                                                                .cast::<u8>())
                                                            .cast::<u8>())
                                                            .wrapping_offset(
                                                                ((((&raw mut gActiveBattler)
                                                                    .cast::<u8>()
                                                                    .cast::<u8>())
                                                                .read())
                                                                    as i32)
                                                                    as isize
                                                                    * 512,
                                                            ))
                                                            .cast::<u8>())
                                                            .wrapping_offset(2))
                                                            .read(),
                                                        );
                                                        ((((&raw mut gChosenMoveByBattler).cast::<u8>().cast::<u16>()).cast::<u16>()).wrapping_offset((((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)) as isize)).write((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>()).wrapping_offset((((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)) as isize * 88)).wrapping_add(12)).cast::<u16>()).wrapping_offset((((((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(128)).cast::<u8>()).wrapping_offset((((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)) as isize)).read()) as i32)) as isize)).read());
                                                        ((((((&raw mut gBattleStruct)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(12))
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .write(
                                                            ((((((&raw mut gBattleBufferB)
                                                                .cast::<u8>())
                                                            .cast::<u8>())
                                                            .wrapping_offset(
                                                                ((((&raw mut gActiveBattler)
                                                                    .cast::<u8>()
                                                                    .cast::<u8>())
                                                                .read())
                                                                    as i32)
                                                                    as isize
                                                                    * 512,
                                                            ))
                                                            .cast::<u8>())
                                                            .wrapping_offset(3))
                                                            .read(),
                                                        );
                                                        let __p10 =
                                                            (((&raw mut gBattleCommunication)
                                                                .cast::<u8>())
                                                            .cast::<u8>())
                                                            .wrapping_offset(
                                                                ((((&raw mut gActiveBattler)
                                                                    .cast::<u8>()
                                                                    .cast::<u8>())
                                                                .read())
                                                                    as i32)
                                                                    as isize,
                                                            );
                                                        (__p10).write(
                                                            ((__p10).read()).wrapping_add(1),
                                                        );
                                                    }
                                                }
                                                break 'l8;
                                            }
                                        }
                                        break 'l7;
                                    }
                                    if __sw8 == 1i32 {
                                        if (((((((((&raw mut gBattleBufferB).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 512,
                                        ))
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read())
                                            as i32)
                                            | (((((((((&raw mut gBattleBufferB).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 512,
                                            ))
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read())
                                                as i32)
                                                << 8))
                                            == 0i32
                                        {
                                            ((((&raw mut gBattleCommunication).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(1u8);
                                        } else {
                                            ((&raw mut gLastUsedItem).cast::<u8>().cast::<u16>())
                                                .write(
                                                    ((((((((((&raw mut gBattleBufferB)
                                                        .cast::<u8>())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gActiveBattler)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 512,
                                                    ))
                                                    .cast::<u8>())
                                                    .wrapping_offset(1))
                                                    .read())
                                                        as i32)
                                                        | (((((((((&raw mut gBattleBufferB)
                                                            .cast::<u8>())
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gActiveBattler)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 512,
                                                        ))
                                                        .cast::<u8>())
                                                        .wrapping_offset(2))
                                                        .read())
                                                            as i32)
                                                            << 8))
                                                        as u16),
                                                );
                                            let __p11 = (((&raw mut gBattleCommunication)
                                                .cast::<u8>())
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            );
                                            (__p11).write(((__p11).read()).wrapping_add(1));
                                        }
                                        break 'l7;
                                    }
                                    if __sw8 == 2i32 {
                                        if ((((((((&raw mut gBattleBufferB).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 512,
                                        ))
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read()) as i32)
                                            == 6i32
                                        {
                                            ((((&raw mut gBattleCommunication).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(1u8);
                                            RecordedBattle_ClearBattlerAction(
                                                ((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read(),
                                                1u8,
                                            );
                                        } else {
                                            UpdateBattlerPartyOrdersOnSwitch();
                                            let __p12 = (((&raw mut gBattleCommunication)
                                                .cast::<u8>())
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            );
                                            (__p12).write(((__p12).read()).wrapping_add(1));
                                        }
                                        break 'l7;
                                    }
                                    if __sw8 == 3i32 {
                                        let __p13 =
                                            (&raw mut gHitMarker).cast::<u8>().cast::<u32>();
                                        (__p13).write(((__p13).read() | 32768u32));
                                        let __p14 = (((&raw mut gBattleCommunication)
                                            .cast::<u8>())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        );
                                        (__p14).write(((__p14).read()).wrapping_add(1));
                                        break 'l7;
                                    }
                                    if __sw8 == 4i32 {
                                        let __p15 = (((&raw mut gBattleCommunication)
                                            .cast::<u8>())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        );
                                        (__p15).write(((__p15).read()).wrapping_add(1));
                                        break 'l7;
                                    }
                                    if __sw8 == 5i32 {
                                        let __p16 = (((&raw mut gBattleCommunication)
                                            .cast::<u8>())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        );
                                        (__p16).write(((__p16).read()).wrapping_add(1));
                                        break 'l7;
                                    }
                                    if __sw8 == 6i32 {
                                        if (((((((((&raw mut gBattleBufferB).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 512,
                                        ))
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read())
                                            as i32)
                                            | (((((((((&raw mut gBattleBufferB).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 512,
                                            ))
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read())
                                                as i32)
                                                << 8))
                                            != 0i32
                                        {
                                            let __p17 = (((&raw mut gBattleCommunication)
                                                .cast::<u8>())
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            );
                                            (__p17).write(((__p17).read()).wrapping_add(1));
                                        } else {
                                            ((((&raw mut gBattleCommunication).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .write(1u8);
                                        }
                                        break 'l7;
                                    }
                                    if __sw8 == 7i32 {
                                        let __p18 = (((&raw mut gBattleCommunication)
                                            .cast::<u8>())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        );
                                        (__p18).write(((__p18).read()).wrapping_add(1));
                                        break 'l7;
                                    }
                                    if __sw8 == 8i32 {
                                        let __p19 =
                                            (&raw mut gHitMarker).cast::<u8>().cast::<u32>();
                                        (__p19).write(((__p19).read() | 32768u32));
                                        let __p20 = (((&raw mut gBattleCommunication)
                                            .cast::<u8>())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        );
                                        (__p20).write(((__p20).read()).wrapping_add(1));
                                        break 'l7;
                                    }
                                    if __sw8 == 9i32 {
                                        let __p21 = (((&raw mut gBattleCommunication)
                                            .cast::<u8>())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        );
                                        (__p21).write(((__p21).read()).wrapping_add(1));
                                        break 'l7;
                                    }
                                }
                            }
                            break 'l3;
                        }
                        if __sw1 == 4i32 {
                            if !((((&raw mut gBattleControllerExecFlags)
                                .cast::<u8>()
                                .cast::<u32>())
                            .read()
                                & ((((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()
                                    | 4026531840u32)
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 4))
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 8))
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 12)))
                                != 0)
                            {
                                if (AllAtActionConfirmed()) != 0 {
                                    i = 1i32;
                                } else {
                                    i = 0i32;
                                }
                                if ((((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>())
                                    .read()
                                    & 64u32)
                                    != 0)
                                    || (!((((&raw mut gBattleTypeFlags)
                                        .cast::<u8>()
                                        .cast::<u32>())
                                    .read()
                                        & 1u32)
                                        != 0)))
                                    || ((((position) as i32) & 2i32) != 0i32))
                                    || ((((((((&raw mut gBattleStruct)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(145))
                                    .read()) as u32)
                                        & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                            .wrapping_offset(
                                                ((GetBattlerAtPosition(
                                                    ((((position) as i32) ^ 2i32) as u8),
                                                ))
                                                    as i32)
                                                    as isize,
                                            ))
                                        .read())
                                        != 0)
                                {
                                    BtlController_EmitLinkStandbyMsg(0u8, 0u8, ((i) as u32));
                                } else {
                                    BtlController_EmitLinkStandbyMsg(0u8, 1u8, ((i) as u32));
                                }
                                MarkBattlerForControllerExec(
                                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                                );
                                let __p22 = (((&raw mut gBattleCommunication).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize,
                                );
                                (__p22).write(((__p22).read()).wrapping_add(1));
                            }
                            break 'l3;
                        }
                        if __sw1 == 5i32 {
                            if !((((&raw mut gBattleControllerExecFlags)
                                .cast::<u8>()
                                .cast::<u32>())
                            .read()
                                & ((((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()
                                    | 4026531840u32)
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 4))
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 8))
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 12)))
                                != 0)
                            {
                                let __p23 = (((&raw mut gBattleCommunication).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(4);
                                (__p23).write(((__p23).read()).wrapping_add(1));
                            }
                            break 'l3;
                        }
                        if __sw1 == 6i32 {
                            if (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(84))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize,
                            ))
                            .read())
                                != 0
                            {
                                ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                .write(
                                    ((((((&raw mut gBattleStruct)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(132))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                    .read(),
                                );
                            } else {
                                ((&raw mut gBattlerAttacker).cast::<u8>().cast::<u8>()).write(
                                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                                );
                                ((&raw mut gBattlescriptCurrInstr)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .write(
                                    ((((&raw mut gSelectionBattleScripts)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                    .read(),
                                );
                                if !((((&raw mut gBattleControllerExecFlags)
                                    .cast::<u8>()
                                    .cast::<u32>())
                                .read()
                                    & ((((((((&raw mut gBitTable).cast::<u32>())
                                        .cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                    .read()
                                        | 4026531840u32)
                                        | (((((&raw mut gBitTable).cast::<u32>())
                                            .cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read()
                                            << 4))
                                        | (((((&raw mut gBitTable).cast::<u32>())
                                            .cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read()
                                            << 8))
                                        | (((((&raw mut gBitTable).cast::<u32>())
                                            .cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read()
                                            << 12)))
                                    != 0)
                                {
                                    (((((&raw mut gBattleScriptingCommandsTable)
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                    .wrapping_offset(
                                        (((((&raw mut gBattlescriptCurrInstr)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read())
                                    .unwrap_unchecked()();
                                }
                                ((((&raw mut gSelectionBattleScripts)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize,
                                ))
                                .write(
                                    ((&raw mut gBattlescriptCurrInstr)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read(),
                                );
                            }
                            break 'l3;
                        }
                        if __sw1 == 7i32 {
                            if !((((&raw mut gBattleControllerExecFlags)
                                .cast::<u8>()
                                .cast::<u32>())
                            .read()
                                & ((((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()
                                    | 4026531840u32)
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 4))
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 8))
                                    | (((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .read()
                                        << 12)))
                                != 0)
                            {
                                ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                .write(1u8);
                            }
                            break 'l3;
                        }
                        if __sw1 == 8i32 {
                            if (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(84))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize,
                            ))
                            .read())
                                != 0
                            {
                                if ((((((((&raw mut gBattleBufferB).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 512,
                                    ))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    == 13i32
                                {
                                    let __p24 = (&raw mut gHitMarker).cast::<u8>().cast::<u32>();
                                    (__p24).write(((__p24).read() | 32768u32));
                                    ((((&raw mut gChosenActionByBattler).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                    .write(3u8);
                                    ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .write(4u8);
                                } else {
                                    RecordedBattle_ClearBattlerAction(
                                        ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read(),
                                        1u8,
                                    );
                                    ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .write(
                                        ((((((&raw mut gBattleStruct)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(132))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read(),
                                    );
                                }
                            } else {
                                ((&raw mut gBattlerAttacker).cast::<u8>().cast::<u8>()).write(
                                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                                );
                                ((&raw mut gBattlescriptCurrInstr)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .write(
                                    ((((&raw mut gSelectionBattleScripts)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                    .read(),
                                );
                                if !((((&raw mut gBattleControllerExecFlags)
                                    .cast::<u8>()
                                    .cast::<u32>())
                                .read()
                                    & ((((((((&raw mut gBitTable).cast::<u32>())
                                        .cast::<u32>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                    .read()
                                        | 4026531840u32)
                                        | (((((&raw mut gBitTable).cast::<u32>())
                                            .cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read()
                                            << 4))
                                        | (((((&raw mut gBitTable).cast::<u32>())
                                            .cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read()
                                            << 8))
                                        | (((((&raw mut gBitTable).cast::<u32>())
                                            .cast::<u32>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                                .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read()
                                            << 12)))
                                    != 0)
                                {
                                    (((((&raw mut gBattleScriptingCommandsTable)
                                        .cast::<Option<unsafe extern "C" fn()>>())
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                    .wrapping_offset(
                                        (((((&raw mut gBattlescriptCurrInstr)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read())
                                    .unwrap_unchecked()();
                                }
                                ((((&raw mut gSelectionBattleScripts)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize,
                                ))
                                .write(
                                    ((&raw mut gBattlescriptCurrInstr)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read(),
                                );
                            }
                            break 'l3;
                        }
                    }
                }
                let __p25 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                (__p25).write(((__p25).read()).wrapping_add(1));
            }
        }
        if ((((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(4))
            .read()) as i32)
            == ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)
        {
            RecordedBattle_CheckMovesetChanges(1u8);
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(SetActionsAndBattlersTurnOrder));
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 4194304u32) != 0 {
                {
                    i = 0i32;
                    'l9: loop {
                        if !(i
                            < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read())
                                as i32))
                        {
                            break 'l9;
                        }
                        'l10: {
                            if ((((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == 2i32
                            {
                                SwitchPartyOrderInGameMulti(
                                    ((i) as u8),
                                    ((((((&raw mut gBattleStruct)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(92))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read(),
                                );
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AllAtActionConfirmed() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut count: i32 = 0i32;
        {
            count = 0i32;
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 5i32
                    {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (count).wrapping_add(1i32)
            == ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateBattlerPartyOrdersOnSwitch() {
    unsafe {
        ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(92))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .write(
            ((((((&raw mut gBattleBufferB).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read(),
        );
        RecordedBattle_SetBattlerAction(
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
            ((((((&raw mut gBattleBufferB).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read(),
        );
        if ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0)
            && ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 64u32) != 0)
        {
            let __p1 = (((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(96))
            .cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(3i32)) as isize,
            );
            (__p1).write((((((__p1).read()) as i32) & 15i32) as u8));
            let __p2 = (((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(96))
            .cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(3i32)) as isize,
            );
            (__p2).write(
                (((((__p2).read()) as i32)
                    | (((((((((&raw mut gBattleBufferB).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 512,
                        ))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        & 240i32)) as u8),
            );
            (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(96))
            .cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(3i32)) as isize,
            ))
            .wrapping_offset(1))
            .write(
                ((((((&raw mut gBattleBufferB).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(3))
                .read(),
            );
            let __p3 = (((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(96))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) ^ 2i32)
                    .wrapping_mul(3i32)) as isize,
            );
            (__p3).write((((((__p3).read()) as i32) & 240i32) as u8));
            let __p4 = (((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(96))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) ^ 2i32)
                    .wrapping_mul(3i32)) as isize,
            );
            (__p4).write(
                (((((__p4).read()) as i32)
                    | ((((((((((&raw mut gBattleBufferB).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 512,
                        ))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        & 240i32)
                        >> 4)) as u8),
            );
            (((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(96))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32) ^ 2i32)
                    .wrapping_mul(3i32)) as isize,
            ))
            .wrapping_offset(2))
            .write(
                ((((((&raw mut gBattleBufferB).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 512,
                ))
                .cast::<u8>())
                .wrapping_offset(3))
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwapTurnOrder(id1: u8, id2: u8) {
    unsafe {
        let mut id1 = id1;
        let mut id2 = id2;
        let mut temp: u32 = 0u32;
        {
            temp = ((((((&raw mut gActionsByTurnOrder).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id1) as i32) as isize))
            .read()) as u32);
            ((((&raw mut gActionsByTurnOrder).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id1) as i32) as isize))
            .write(
                ((((&raw mut gActionsByTurnOrder).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((id2) as i32) as isize))
                .read(),
            );
            ((((&raw mut gActionsByTurnOrder).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id2) as i32) as isize))
            .write(((temp) as u8));
        }
        {
            temp = ((((((&raw mut gBattlerByTurnOrder).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id1) as i32) as isize))
            .read()) as u32);
            ((((&raw mut gBattlerByTurnOrder).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id1) as i32) as isize))
            .write(
                ((((&raw mut gBattlerByTurnOrder).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((id2) as i32) as isize))
                .read(),
            );
            ((((&raw mut gBattlerByTurnOrder).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id2) as i32) as isize))
            .write(((temp) as u8));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWhoStrikesFirst(
    battler1: u8,
    battler2: u8,
    ignoreChosenMoves: u8,
) -> u8 {
    unsafe {
        let mut battler1 = battler1;
        let mut battler2 = battler2;
        let mut ignoreChosenMoves = ignoreChosenMoves;
        let mut strikesFirst: u8 = 0u8;
        let mut speedMultiplierBattler1: u8 = 0u8;
        let mut speedMultiplierBattler2: u8 = 0u8;
        let mut speedBattler1: u32 = 0u32;
        let mut speedBattler2: u32 = 0u32;
        let mut holdEffect: u8 = 0u8;
        let mut holdEffectParam: u8 = 0u8;
        let mut moveBattler1: u16 = 0u16;
        let mut moveBattler2: u16 = 0u16;
        if (!((AbilityBattleEffects(19u8, 0u8, 13u8, 0u8, 0u16)) != 0))
            && (!((AbilityBattleEffects(19u8, 0u8, 77u8, 0u8, 0u16)) != 0))
        {
            if (((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler1) as i32) as isize * 88))
            .wrapping_add(32))
            .read()) as i32)
                == 33i32)
                && ((((((&raw mut gBattleWeather).cast::<u8>().cast::<u16>()).read()) as i32)
                    & 7i32)
                    != 0))
                || (((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((battler1) as i32) as isize * 88))
                .wrapping_add(32))
                .read()) as i32)
                    == 34i32)
                    && ((((((&raw mut gBattleWeather).cast::<u8>().cast::<u16>()).read()) as i32)
                        & 96i32)
                        != 0))
            {
                speedMultiplierBattler1 = 2u8;
            } else {
                speedMultiplierBattler1 = 1u8;
            }
            if (((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler2) as i32) as isize * 88))
            .wrapping_add(32))
            .read()) as i32)
                == 33i32)
                && ((((((&raw mut gBattleWeather).cast::<u8>().cast::<u16>()).read()) as i32)
                    & 7i32)
                    != 0))
                || (((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((battler2) as i32) as isize * 88))
                .wrapping_add(32))
                .read()) as i32)
                    == 34i32)
                    && ((((((&raw mut gBattleWeather).cast::<u8>().cast::<u16>()).read()) as i32)
                        & 96i32)
                        != 0))
            {
                speedMultiplierBattler2 = 2u8;
            } else {
                speedMultiplierBattler2 = 1u8;
            }
        } else {
            speedMultiplierBattler1 = 1u8;
            speedMultiplierBattler2 = 1u8;
        }
        speedBattler1 = ((crate::c::div_i32(
            ((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler1) as i32) as isize * 88))
            .wrapping_add(6)
            .cast::<u16>())
            .read()) as i32)
                .wrapping_mul(((speedMultiplierBattler1) as i32)))
            .wrapping_mul(
                ((((((&raw mut gStatStageRatios).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((battler1) as i32) as isize * 88))
                    .wrapping_add(24))
                    .cast::<i8>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize
                        * 2,
                ))
                .cast::<u8>())
                .read()) as i32),
            ),
            (((((((&raw mut gStatStageRatios).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((battler1) as i32) as isize * 88))
                .wrapping_add(24))
                .cast::<i8>())
                .wrapping_offset(3))
                .read()) as i32) as isize
                    * 2,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32),
        )) as u32);
        if (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((battler1) as i32) as isize * 88))
        .wrapping_add(46)
        .cast::<u16>())
        .read()) as i32)
            == 175i32
        {
            holdEffect = (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler1) as i32) as isize * 28))
            .wrapping_add(7))
            .read();
            holdEffectParam = (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler1) as i32) as isize * 28))
            .wrapping_add(26))
            .read();
        } else {
            holdEffect = GetItemHoldEffect(
                (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((battler1) as i32) as isize * 88))
                .wrapping_add(46)
                .cast::<u16>())
                .read(),
            );
            holdEffectParam = GetItemHoldEffectParam(
                (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((battler1) as i32) as isize * 88))
                .wrapping_add(46)
                .cast::<u16>())
                .read(),
            );
        }
        if ((!((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 37683458u32)
            != 0))
            && ((FlagGet(2153u16)) != 0))
            && (((GetBattlerSide(battler1)) as i32) == 0i32)
        {
            speedBattler1 = crate::c::div_u32((speedBattler1).wrapping_mul(110u32), 100u32);
        }
        if ((holdEffect) as i32) == 24i32 {
            speedBattler1 = crate::c::div_u32(speedBattler1, 2u32);
        }
        if ((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((battler1) as i32) as isize * 88))
        .wrapping_add(76)
        .cast::<u32>())
        .read()
            & 64u32)
            != 0
        {
            speedBattler1 = crate::c::div_u32(speedBattler1, 4u32);
        }
        if (((holdEffect) as i32) == 26i32)
            && (((((&raw mut gRandomTurnNumber).cast::<u8>().cast::<u16>()).read()) as i32)
                < crate::c::div_i32((65535i32).wrapping_mul(((holdEffectParam) as i32)), 100i32))
        {
            speedBattler1 = 4294967295u32;
        }
        speedBattler2 = ((crate::c::div_i32(
            ((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler2) as i32) as isize * 88))
            .wrapping_add(6)
            .cast::<u16>())
            .read()) as i32)
                .wrapping_mul(((speedMultiplierBattler2) as i32)))
            .wrapping_mul(
                ((((((&raw mut gStatStageRatios).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((battler2) as i32) as isize * 88))
                    .wrapping_add(24))
                    .cast::<i8>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize
                        * 2,
                ))
                .cast::<u8>())
                .read()) as i32),
            ),
            (((((((&raw mut gStatStageRatios).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((battler2) as i32) as isize * 88))
                .wrapping_add(24))
                .cast::<i8>())
                .wrapping_offset(3))
                .read()) as i32) as isize
                    * 2,
            ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32),
        )) as u32);
        if (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((battler2) as i32) as isize * 88))
        .wrapping_add(46)
        .cast::<u16>())
        .read()) as i32)
            == 175i32
        {
            holdEffect = (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler2) as i32) as isize * 28))
            .wrapping_add(7))
            .read();
            holdEffectParam = (((((&raw mut gEnigmaBerries).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler2) as i32) as isize * 28))
            .wrapping_add(26))
            .read();
        } else {
            holdEffect = GetItemHoldEffect(
                (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((battler2) as i32) as isize * 88))
                .wrapping_add(46)
                .cast::<u16>())
                .read(),
            );
            holdEffectParam = GetItemHoldEffectParam(
                (((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((battler2) as i32) as isize * 88))
                .wrapping_add(46)
                .cast::<u16>())
                .read(),
            );
        }
        if ((!((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 37683458u32)
            != 0))
            && ((FlagGet(2153u16)) != 0))
            && (((GetBattlerSide(battler2)) as i32) == 0i32)
        {
            speedBattler2 = crate::c::div_u32((speedBattler2).wrapping_mul(110u32), 100u32);
        }
        if ((holdEffect) as i32) == 24i32 {
            speedBattler2 = crate::c::div_u32(speedBattler2, 2u32);
        }
        if ((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((battler2) as i32) as isize * 88))
        .wrapping_add(76)
        .cast::<u32>())
        .read()
            & 64u32)
            != 0
        {
            speedBattler2 = crate::c::div_u32(speedBattler2, 4u32);
        }
        if (((holdEffect) as i32) == 26i32)
            && (((((&raw mut gRandomTurnNumber).cast::<u8>().cast::<u16>()).read()) as i32)
                < crate::c::div_i32((65535i32).wrapping_mul(((holdEffectParam) as i32)), 100i32))
        {
            speedBattler2 = 4294967295u32;
        }
        if (ignoreChosenMoves) != 0 {
            moveBattler1 = 0u16;
            moveBattler2 = 0u16;
        } else {
            if ((((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler1) as i32) as isize))
            .read()) as i32)
                == 0i32
            {
                if (crate::c::bf_read(
                    ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((battler1) as i32) as isize * 16))
                    .wrapping_add(0),
                    2,
                    1,
                    false,
                ) as u32)
                    != 0
                {
                    moveBattler1 = 165u16;
                } else {
                    moveBattler1 = (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((battler1) as i32) as isize * 88))
                    .wrapping_add(12))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(128))
                        .cast::<u8>())
                        .wrapping_offset(((battler1) as i32) as isize))
                        .read()) as i32) as isize,
                    ))
                    .read();
                }
            } else {
                moveBattler1 = 0u16;
            }
            if ((((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((battler2) as i32) as isize))
            .read()) as i32)
                == 0i32
            {
                if (crate::c::bf_read(
                    ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((battler2) as i32) as isize * 16))
                    .wrapping_add(0),
                    2,
                    1,
                    false,
                ) as u32)
                    != 0
                {
                    moveBattler2 = 165u16;
                } else {
                    moveBattler2 = (((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((battler2) as i32) as isize * 88))
                    .wrapping_add(12))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(128))
                        .cast::<u8>())
                        .wrapping_offset(((battler2) as i32) as isize))
                        .read()) as i32) as isize,
                    ))
                    .read();
                }
            } else {
                moveBattler2 = 0u16;
            }
        }
        if (((((((&raw mut gBattleMoves).cast::<u8>())
            .wrapping_offset(((moveBattler1) as i32) as isize * 12))
        .wrapping_add(7)
        .cast::<i8>())
        .read()) as i32)
            != 0i32)
            || (((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((moveBattler2) as i32) as isize * 12))
            .wrapping_add(7)
            .cast::<i8>())
            .read()) as i32)
                != 0i32)
        {
            if ((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((moveBattler1) as i32) as isize * 12))
            .wrapping_add(7)
            .cast::<i8>())
            .read()) as i32)
                == ((((((&raw mut gBattleMoves).cast::<u8>())
                    .wrapping_offset(((moveBattler2) as i32) as isize * 12))
                .wrapping_add(7)
                .cast::<i8>())
                .read()) as i32)
            {
                if (speedBattler1 == speedBattler2) && ((((Random()) as i32) & 1i32) != 0) {
                    strikesFirst = 2u8;
                } else {
                    if speedBattler1 < speedBattler2 {
                        strikesFirst = 1u8;
                    }
                }
            } else {
                if ((((((&raw mut gBattleMoves).cast::<u8>())
                    .wrapping_offset(((moveBattler1) as i32) as isize * 12))
                .wrapping_add(7)
                .cast::<i8>())
                .read()) as i32)
                    < ((((((&raw mut gBattleMoves).cast::<u8>())
                        .wrapping_offset(((moveBattler2) as i32) as isize * 12))
                    .wrapping_add(7)
                    .cast::<i8>())
                    .read()) as i32)
                {
                    strikesFirst = 1u8;
                }
            }
        } else {
            if (speedBattler1 == speedBattler2) && ((((Random()) as i32) & 1i32) != 0) {
                strikesFirst = 2u8;
            } else {
                if speedBattler1 < speedBattler2 {
                    strikesFirst = 1u8;
                }
            }
        }
        return strikesFirst;
    }
}
pub(crate) unsafe extern "C" fn SetActionsAndBattlersTurnOrder() {
    unsafe {
        let mut turnOrderId: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 128u32) != 0 {
            {
                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
                'l1: loop {
                    if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut gActionsByTurnOrder).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((turnOrderId) as isize))
                        .write(
                            ((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize,
                                ))
                            .read(),
                        );
                        ((((&raw mut gBattlerByTurnOrder).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((turnOrderId) as isize))
                        .write(((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read());
                        turnOrderId = (turnOrderId).wrapping_add(1);
                    }
                    let __p1 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                }
            }
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0 {
                {
                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
                    'l3: loop {
                        if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                            as i32)
                            < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read())
                                as i32))
                        {
                            break 'l3;
                        }
                        'l4: {
                            if ((((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize,
                                ))
                            .read()) as i32)
                                == 3i32
                            {
                                turnOrderId = 5i32;
                                break 'l3;
                            }
                        }
                        let __p2 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                }
            } else {
                if (((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>()).read()) as i32)
                    == 3i32
                {
                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
                    turnOrderId = 5i32;
                }
                if ((((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(2))
                .read()) as i32)
                    == 3i32
                {
                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(2u8);
                    turnOrderId = 5i32;
                }
            }
            if turnOrderId == 5i32 {
                (((&raw mut gActionsByTurnOrder).cast::<u8>()).cast::<u8>()).write(
                    ((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                    .read(),
                );
                (((&raw mut gBattlerByTurnOrder).cast::<u8>()).cast::<u8>())
                    .write(((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read());
                turnOrderId = 1i32;
                {
                    i = 0i32;
                    'l5: loop {
                        if !(i
                            < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read())
                                as i32))
                        {
                            break 'l5;
                        }
                        'l6: {
                            if i != ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                as i32)
                            {
                                ((((&raw mut gActionsByTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((turnOrderId) as isize))
                                .write(
                                    ((((&raw mut gChosenActionByBattler).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read(),
                                );
                                ((((&raw mut gBattlerByTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((turnOrderId) as isize))
                                .write(((i) as u8));
                                turnOrderId = (turnOrderId).wrapping_add(1);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((&raw mut gBattleMainFunc)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(CheckFocusPunch_ClearVarsBeforeTurnStarts));
                ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(72))
                .write(0u8);
                return;
            } else {
                {
                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
                    'l7: loop {
                        if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                            as i32)
                            < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read())
                                as i32))
                        {
                            break 'l7;
                        }
                        'l8: {
                            if (((((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize,
                                ))
                            .read()) as i32)
                                == 1i32)
                                || (((((((&raw mut gChosenActionByBattler).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize,
                                ))
                                .read()) as i32)
                                    == 2i32)
                            {
                                ((((&raw mut gActionsByTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((turnOrderId) as isize))
                                .write(
                                    ((((&raw mut gChosenActionByBattler).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                    .read(),
                                );
                                ((((&raw mut gBattlerByTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((turnOrderId) as isize))
                                .write(
                                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                                );
                                turnOrderId = (turnOrderId).wrapping_add(1);
                            }
                        }
                        let __p3 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    }
                }
                {
                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
                    'l9: loop {
                        if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                            as i32)
                            < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read())
                                as i32))
                        {
                            break 'l9;
                        }
                        'l10: {
                            if (((((((&raw mut gChosenActionByBattler).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize,
                                ))
                            .read()) as i32)
                                != 1i32)
                                && (((((((&raw mut gChosenActionByBattler).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize,
                                ))
                                .read()) as i32)
                                    != 2i32)
                            {
                                ((((&raw mut gActionsByTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((turnOrderId) as isize))
                                .write(
                                    ((((&raw mut gChosenActionByBattler).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize,
                                    ))
                                    .read(),
                                );
                                ((((&raw mut gBattlerByTurnOrder).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((turnOrderId) as isize))
                                .write(
                                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                                );
                                turnOrderId = (turnOrderId).wrapping_add(1);
                            }
                        }
                        let __p4 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    }
                }
                {
                    i = 0i32;
                    'l11: loop {
                        if !(i
                            < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read())
                                as i32)
                                .wrapping_sub(1i32))
                        {
                            break 'l11;
                        }
                        'l12: {
                            {
                                j = (i).wrapping_add(1i32);
                                'l13: loop {
                                    if !(j
                                        < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32))
                                    {
                                        break 'l13;
                                    }
                                    'l14: {
                                        let mut battler1: u8 = ((((&raw mut gBattlerByTurnOrder)
                                            .cast::<u8>())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read();
                                        let mut battler2: u8 = ((((&raw mut gBattlerByTurnOrder)
                                            .cast::<u8>())
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .read();
                                        if (((((((((&raw mut gActionsByTurnOrder).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read())
                                            as i32)
                                            != 1i32)
                                            && (((((((&raw mut gActionsByTurnOrder).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                                != 1i32))
                                            && (((((((&raw mut gActionsByTurnOrder).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                != 2i32))
                                            && (((((((&raw mut gActionsByTurnOrder).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                                != 2i32)
                                        {
                                            if (GetWhoStrikesFirst(battler1, battler2, 0u8)) != 0 {
                                                SwapTurnOrder(((i) as u8), ((j) as u8));
                                            }
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CheckFocusPunch_ClearVarsBeforeTurnStarts));
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(72))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn TurnValuesCleanUp(var0: u8) {
    unsafe {
        let mut var0 = var0;
        let mut i: i32 = 0i32;
        let mut dataPtr: *mut u8 = core::ptr::null_mut();
        {
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
            'l1: loop {
                if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (var0) != 0 {
                        crate::c::bf_write(
                            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize
                                        * 16,
                                ))
                            .wrapping_add(0),
                            0,
                            1,
                            (0u32) as i32,
                        );
                        crate::c::bf_write(
                            ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize
                                        * 16,
                                ))
                            .wrapping_add(0),
                            1,
                            1,
                            (0u32) as i32,
                        );
                    } else {
                        dataPtr = (((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize
                                    * 16,
                            );
                        {
                            i = 0i32;
                            'l3: loop {
                                if !(((i) as u32) < 16u32) {
                                    break 'l3;
                                }
                                'l4: {
                                    ((dataPtr).wrapping_offset((i) as isize)).write(0u8);
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        if ((((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize
                                    * 28,
                            ))
                        .wrapping_add(22))
                        .read())
                            != 0
                        {
                            let __p1 = ((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize
                                        * 28,
                                ))
                            .wrapping_add(22);
                            (__p1).write(((__p1).read()).wrapping_sub(1));
                        }
                        if ((((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize
                                    * 28,
                            ))
                        .wrapping_add(25))
                        .read())
                            != 0
                        {
                            let __p2 = ((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize
                                        * 28,
                                ))
                            .wrapping_add(25);
                            (__p2).write(((__p2).read()).wrapping_sub(1));
                            if (((((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                        as i32) as isize
                                        * 28,
                                ))
                            .wrapping_add(25))
                            .read()) as i32)
                                == 0i32
                            {
                                let __p3 = ((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p3).write(((__p3).read() & 4290772991u32));
                            }
                        }
                    }
                    if (((((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 28,
                        ))
                    .wrapping_add(10))
                    .read()) as i32)
                        == 0i32
                    {
                        let __p4 = ((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize
                                    * 88,
                            ))
                        .wrapping_add(80)
                        .cast::<u32>();
                        (__p4).write(((__p4).read() & 4278190079u32));
                    }
                }
                let __p5 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                (__p5).write(((__p5).read()).wrapping_add(1));
            }
        }
        ((((&raw mut gSideTimers).cast::<u8>()).cast::<u8>()).wrapping_add(8)).write(0u8);
        (((((&raw mut gSideTimers).cast::<u8>()).cast::<u8>()).wrapping_offset(12))
            .wrapping_add(8))
        .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpecialStatusesClear() {
    unsafe {
        {
            ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
            'l1: loop {
                if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                    < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut i: i32 = 0i32;
                    let mut dataPtr: *mut u8 =
                        (((&raw mut gSpecialStatuses).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 20,
                        );
                    {
                        i = 0i32;
                        'l3: loop {
                            if !(((i) as u32) < 20u32) {
                                break 'l3;
                            }
                            'l4: {
                                ((dataPtr).wrapping_offset((i) as isize)).write(0u8);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                let __p1 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CheckFocusPunch_ClearVarsBeforeTurnStarts() {
    unsafe {
        if !((((&raw mut gHitMarker).cast::<u8>().cast::<u32>()).read() & 32768u32) != 0) {
            'l1: loop {
                if !(((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(72))
                .read()) as i32)
                    < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write({
                    let __v1 = ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(72))
                    .read();
                    ((&raw mut gBattlerAttacker).cast::<u8>().cast::<u8>()).write(__v1);
                    __v1
                });
                let __p2 = (((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(72);
                (__p2).write(((__p2).read()).wrapping_add(1));
                if (((((((((&raw mut gChosenMoveByBattler).cast::<u8>().cast::<u16>())
                    .cast::<u16>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize,
                ))
                .read()) as i32)
                    == 264i32)
                    && (!(((((((&raw mut gBattleMons).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 88,
                        ))
                    .wrapping_add(76)
                    .cast::<u32>())
                    .read()
                        & 7u32)
                        != 0)))
                    && (!((crate::c::bf_read(
                        ((((&raw mut gDisableStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>().cast::<u8>()).read())
                                as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(24),
                        0,
                        1,
                        false,
                    ) as u8)
                        != 0)))
                    && (!((crate::c::bf_read(
                        ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read()) as i32)
                                as isize
                                * 16,
                        ))
                        .wrapping_add(0),
                        2,
                        1,
                        false,
                    ) as u32)
                        != 0))
                {
                    BattleScriptExecute((&raw mut BattleScript_FocusPunchSetUp).cast::<u8>());
                    return;
                }
            }
        }
        TryClearRageStatuses();
        ((&raw mut gCurrentTurnActionNumber)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        ((&raw mut gCurrentActionFuncId).cast::<u8>().cast::<u8>()).write(
            ((((&raw mut gActionsByTurnOrder).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gCurrentTurnActionNumber)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize,
            ))
            .read(),
        );
        ((&raw mut gDynamicBasePower).cast::<u8>().cast::<u16>()).write(0u16);
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(19))
            .write(0u8);
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(RunTurnActionsFunctions));
        ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .write(0u8);
        ((((&raw mut gBattleCommunication).cast::<u8>()).cast::<u8>()).wrapping_offset(4))
            .write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(22)).write(0u8);
        ((((((&raw mut gBattleResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(32))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn RunTurnActionsFunctions() {
    unsafe {
        if ((((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).read()) as i32) != 0i32 {
            ((&raw mut gCurrentActionFuncId).cast::<u8>().cast::<u8>()).write(12u8);
        }
        ((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(75))
            .write(
                ((&raw mut gCurrentTurnActionNumber)
                    .cast::<u8>()
                    .cast::<u8>())
                .read(),
            );
        (((((&raw const sTurnActionsFuncsTable)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((&raw mut gCurrentActionFuncId).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
        if ((((&raw mut gCurrentTurnActionNumber)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            >= ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read()) as i32)
        {
            let __p1 = (&raw mut gHitMarker).cast::<u8>().cast::<u32>();
            (__p1).write(((__p1).read() & 4293918719u32));
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(
                ((((&raw const sEndTurnFuncsTable)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(
                    (((((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).read()) as i32)
                        & 127i32) as isize,
                ))
                .read(),
            );
        } else {
            if ((((((&raw mut gBattleStruct).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(75))
            .read()) as i32)
                != ((((&raw mut gCurrentTurnActionNumber)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32)
            {
                let __p2 = (&raw mut gHitMarker).cast::<u8>().cast::<u32>();
                (__p2).write(((__p2).read() & 4294966783u32));
                let __p3 = (&raw mut gHitMarker).cast::<u8>().cast::<u32>();
                (__p3).write(((__p3).read() & 4294443007u32));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandleEndTurn_BattleWon() {
    unsafe {
        ((&raw mut gCurrentActionFuncId).cast::<u8>().cast::<u8>()).write(0u8);
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 33554434u32) != 0 {
            ((&raw mut gSpecialVar_Result).cast::<u16>())
                .write(((((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).read()) as u16));
            (((&raw mut gBattleTextBuff1).cast::<u8>()).cast::<u8>())
                .write(((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).read());
            ((&raw mut gBattlerAttacker).cast::<u8>().cast::<u8>())
                .write(GetBattlerAtPosition(0u8));
            ((&raw mut gBattlescriptCurrInstr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write((&raw mut BattleScript_LinkBattleWonOrLost).cast::<u8>());
            let __p1 = (&raw mut gBattleOutcome).cast::<u8>().cast::<u8>();
            (__p1).write((((((__p1).read()) as i32) & (-129i32)) as u8));
        } else {
            if ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 8u32) != 0)
                && ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 71239936u32)
                    != 0)
            {
                BattleStopLowHpSound();
                ((&raw mut gBattlescriptCurrInstr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write((&raw mut BattleScript_FrontierTrainerBattleWon).cast::<u8>());
                if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) == 1022i32
                {
                    PlayBGM(354u16);
                } else {
                    PlayBGM(412u16);
                }
            } else {
                if ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 8u32) != 0)
                    && (!((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32)
                        != 0))
                {
                    BattleStopLowHpSound();
                    ((&raw mut gBattlescriptCurrInstr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .write((&raw mut BattleScript_LocalTrainerBattleWon).cast::<u8>());
                    'l1: {
                        let __sw2 = ((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                                as isize
                                * 40,
                        ))
                        .wrapping_add(1))
                        .read()) as i32);
                        let __matched = __sw2 == 31i32
                            || __sw2 == 38i32
                            || __sw2 == 3i32
                            || __sw2 == 9i32
                            || __sw2 == 11i32
                            || __sw2 == 13i32
                            || __sw2 == 49i32
                            || __sw2 == 53i32
                            || __sw2 == 32i32;
                        if __sw2 == 31i32 || __sw2 == 38i32 {
                            PlayBGM(355u16);
                            break 'l1;
                        }
                        if __sw2 == 3i32
                            || __sw2 == 9i32
                            || __sw2 == 11i32
                            || __sw2 == 13i32
                            || __sw2 == 49i32
                            || __sw2 == 53i32
                        {
                            PlayBGM(424u16);
                            break 'l1;
                        }
                        if __sw2 == 32i32 {
                            PlayBGM(354u16);
                            break 'l1;
                        }
                        if !__matched {
                            PlayBGM(412u16);
                            break 'l1;
                        }
                    }
                } else {
                    ((&raw mut gBattlescriptCurrInstr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .write((&raw mut BattleScript_PayDayMoneyAndPickUpItems).cast::<u8>());
                }
            }
        }
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(HandleEndTurn_FinishBattle));
    }
}
pub(crate) unsafe extern "C" fn HandleEndTurn_BattleLost() {
    unsafe {
        ((&raw mut gCurrentActionFuncId).cast::<u8>().cast::<u8>()).write(0u8);
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 33554434u32) != 0 {
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 4129024u32) != 0 {
                if (((((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).read()) as i32)
                    & 128i32)
                    != 0
                {
                    ((&raw mut gBattlescriptCurrInstr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .write((&raw mut BattleScript_PrintPlayerForfeitedLinkBattle).cast::<u8>());
                    let __p1 = (&raw mut gBattleOutcome).cast::<u8>().cast::<u8>();
                    (__p1).write((((((__p1).read()) as i32) & (-129i32)) as u8));
                    crate::c::bf_write(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1629),
                        3,
                        1,
                        (1u8) as i32,
                    );
                } else {
                    ((&raw mut gBattlescriptCurrInstr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .write((&raw mut BattleScript_FrontierLinkBattleLost).cast::<u8>());
                    let __p2 = (&raw mut gBattleOutcome).cast::<u8>().cast::<u8>();
                    (__p2).write((((((__p2).read()) as i32) & (-129i32)) as u8));
                }
            } else {
                (((&raw mut gBattleTextBuff1).cast::<u8>()).cast::<u8>())
                    .write(((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).read());
                ((&raw mut gBattlerAttacker).cast::<u8>().cast::<u8>())
                    .write(GetBattlerAtPosition(0u8));
                ((&raw mut gBattlescriptCurrInstr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write((&raw mut BattleScript_LinkBattleWonOrLost).cast::<u8>());
                let __p3 = (&raw mut gBattleOutcome).cast::<u8>().cast::<u8>();
                (__p3).write((((((__p3).read()) as i32) & (-129i32)) as u8));
            }
        } else {
            ((&raw mut gBattlescriptCurrInstr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write((&raw mut BattleScript_LocalBattleLost).cast::<u8>());
        }
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(HandleEndTurn_FinishBattle));
    }
}
pub(crate) unsafe extern "C" fn HandleEndTurn_RanFromBattle() {
    unsafe {
        ((&raw mut gCurrentActionFuncId).cast::<u8>().cast::<u8>()).write(0u8);
        if ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 4129024u32) != 0)
            && ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 8u32) != 0)
        {
            ((&raw mut gBattlescriptCurrInstr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write((&raw mut BattleScript_PrintPlayerForfeited).cast::<u8>());
            ((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).write(9u8);
            crate::c::bf_write(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                3,
                1,
                (1u8) as i32,
            );
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 67108864u32) != 0
            {
                ((&raw mut gBattlescriptCurrInstr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write((&raw mut BattleScript_PrintPlayerForfeited).cast::<u8>());
                ((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).write(9u8);
            } else {
                'l1: {
                    let __sw1 = (crate::c::bf_read(
                        ((((&raw mut gProtectStructs).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattlerAttacker).cast::<u8>().cast::<u8>()).read())
                                as i32) as isize
                                * 16,
                        ))
                        .wrapping_add(1),
                        3,
                        2,
                        false,
                    ) as u32);
                    let __matched = __sw1 == 1u32 || __sw1 == 2u32;
                    if !__matched {
                        ((&raw mut gBattlescriptCurrInstr)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write((&raw mut BattleScript_GotAwaySafely).cast::<u8>());
                        break 'l1;
                    }
                    if __sw1 == 1u32 {
                        ((&raw mut gBattlescriptCurrInstr)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write((&raw mut BattleScript_SmokeBallEscape).cast::<u8>());
                        break 'l1;
                    }
                    if __sw1 == 2u32 {
                        ((&raw mut gBattlescriptCurrInstr)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write((&raw mut BattleScript_RanAwayUsingMonAbility).cast::<u8>());
                        break 'l1;
                    }
                }
            }
        }
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(HandleEndTurn_FinishBattle));
    }
}
pub(crate) unsafe extern "C" fn HandleEndTurn_MonFled() {
    unsafe {
        ((&raw mut gCurrentActionFuncId).cast::<u8>().cast::<u8>()).write(0u8);
        {
            (((&raw mut gBattleTextBuff1).cast::<u8>()).cast::<u8>()).write(253u8);
            ((((&raw mut gBattleTextBuff1).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                .write(7u8);
            ((((&raw mut gBattleTextBuff1).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
                .write(((&raw mut gBattlerAttacker).cast::<u8>().cast::<u8>()).read());
            ((((&raw mut gBattleTextBuff1).cast::<u8>()).cast::<u8>()).wrapping_offset(3)).write(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((&raw mut gBattlerAttacker).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    ))
                .read()) as u8),
            );
            ((((&raw mut gBattleTextBuff1).cast::<u8>()).cast::<u8>()).wrapping_offset(4))
                .write(255u8);
        }
        ((&raw mut gBattlescriptCurrInstr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write((&raw mut BattleScript_WildMonFled).cast::<u8>());
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(HandleEndTurn_FinishBattle));
    }
}
pub(crate) unsafe extern "C" fn HandleEndTurn_FinishBattle() {
    unsafe {
        if (((((&raw mut gCurrentActionFuncId).cast::<u8>().cast::<u8>()).read()) as i32) == 11i32)
            || (((((&raw mut gCurrentActionFuncId).cast::<u8>().cast::<u8>()).read()) as i32)
                == 12i32)
        {
            if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 37686162u32)
                != 0)
            {
                {
                    ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).write(0u8);
                    'l1: loop {
                        if !(((((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read())
                            as i32)
                            < ((((&raw mut gBattlersCount).cast::<u8>().cast::<u8>()).read())
                                as i32))
                        {
                            break 'l1;
                        }
                        'l2: {
                            if ((GetBattlerSide(
                                ((&raw mut gActiveBattler).cast::<u8>().cast::<u8>()).read(),
                            )) as i32)
                                == 0i32
                            {
                                if (((((&raw mut gBattleResults).cast::<u8>())
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                .read()) as i32)
                                    == 0i32
                                {
                                    (((&raw mut gBattleResults).cast::<u8>())
                                        .wrapping_add(6)
                                        .cast::<u16>())
                                    .write(
                                        ((GetMonData3(
                                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                                ((((((&raw mut gBattlerPartyIndexes)
                                                    .cast::<u8>()
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 100,
                                            ),
                                            11i32,
                                            core::ptr::null_mut(),
                                        )) as u16),
                                    );
                                    GetMonData3(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((((((&raw mut gBattlerPartyIndexes)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .read())
                                                as i32)
                                                as isize
                                                * 100,
                                        ),
                                        2i32,
                                        (((&raw mut gBattleResults).cast::<u8>()).wrapping_add(8))
                                            .cast::<u8>(),
                                    );
                                } else {
                                    (((&raw mut gBattleResults).cast::<u8>())
                                        .wrapping_add(38)
                                        .cast::<u16>())
                                    .write(
                                        ((GetMonData3(
                                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                                ((((((&raw mut gBattlerPartyIndexes)
                                                    .cast::<u8>()
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    ((((&raw mut gActiveBattler)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize,
                                                ))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 100,
                                            ),
                                            11i32,
                                            core::ptr::null_mut(),
                                        )) as u16),
                                    );
                                    GetMonData3(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((((((&raw mut gBattlerPartyIndexes)
                                                .cast::<u8>()
                                                .cast::<u16>())
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    as isize,
                                            ))
                                            .read())
                                                as i32)
                                                as isize
                                                * 100,
                                        ),
                                        2i32,
                                        (((&raw mut gBattleResults).cast::<u8>()).wrapping_add(20))
                                            .cast::<u8>(),
                                    );
                                }
                            }
                        }
                        let __p1 = (&raw mut gActiveBattler).cast::<u8>().cast::<u8>();
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                }
                TryPutPokemonTodayOnAir();
            }
            if (!((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 37686170u32)
                != 0))
                && ((crate::c::bf_read(
                    ((&raw mut gBattleResults).cast::<u8>()).wrapping_add(5),
                    6,
                    1,
                    false,
                ) as u8)
                    != 0)
            {
                TryPutBreakingNewsOnAir();
            }
            RecordedBattle_SetPlaybackFinished();
            BeginFastPaletteFade(3u8);
            FadeOutMapMusic(5u8);
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FreeResetData_ReturnToOvOrDoEvolutions));
            ((&raw mut gCB2_AfterEvolution).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(BattleMainCB2));
        } else {
            if ((&raw mut gBattleControllerExecFlags)
                .cast::<u8>()
                .cast::<u32>())
            .read()
                == 0u32
            {
                (((((&raw mut gBattleScriptingCommandsTable)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(
                    (((((&raw mut gBattlescriptCurrInstr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .read()) as i32) as isize,
                ))
                .read())
                .unwrap_unchecked()();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FreeResetData_ReturnToOvOrDoEvolutions() {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            ResetSpriteData();
            if (((((&raw mut gLeveledUpInBattle).cast::<u8>().cast::<u8>()).read()) as i32) == 0i32)
                || (((((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).read()) as i32) != 1i32)
            {
                ((&raw mut gBattleMainFunc)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(ReturnFromBattleToOverworld));
                return;
            } else {
                ((&raw mut gBattleMainFunc)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(TryEvolvePokemon));
            }
        }
        FreeAllWindowBuffers();
        if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0) {
            FreeMonSpritesGfx();
            FreeBattleResources();
            FreeBattleSpritesData();
        }
    }
}
pub(crate) unsafe extern "C" fn TryEvolvePokemon() {
    unsafe {
        let mut i: i32 = 0i32;
        'l1: loop {
            if !(((((&raw mut gLeveledUpInBattle).cast::<u8>().cast::<u8>()).read()) as i32)
                != 0i32)
            {
                break 'l1;
            }
            {
                i = 0i32;
                'l2: loop {
                    if !(i < 6i32) {
                        break 'l2;
                    }
                    'l3: {
                        if (((((&raw mut gLeveledUpInBattle).cast::<u8>().cast::<u8>()).read())
                            as u32)
                            & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read())
                            != 0
                        {
                            let mut species: u16 = 0u16;
                            let mut levelUpBits: u8 =
                                ((&raw mut gLeveledUpInBattle).cast::<u8>().cast::<u8>()).read();
                            levelUpBits = ((((levelUpBits) as u32)
                                & !(((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset((i) as isize))
                                .read())) as u8);
                            ((&raw mut gLeveledUpInBattle).cast::<u8>().cast::<u8>())
                                .write(levelUpBits);
                            species = GetEvolutionTargetSpecies(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                0u8,
                                ((levelUpBits) as u16),
                            );
                            if ((species) as i32) != 0i32 {
                                FreeAllWindowBuffers();
                                ((&raw mut gBattleMainFunc)
                                    .cast::<u8>()
                                    .cast::<Option<unsafe extern "C" fn()>>())
                                .write(Some(WaitForEvoSceneToFinish));
                                EvolutionScene(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset((i) as isize * 100),
                                    species,
                                    1u8,
                                    ((i) as u8),
                                );
                                return;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        ((&raw mut gBattleMainFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(ReturnFromBattleToOverworld));
    }
}
pub(crate) unsafe extern "C" fn WaitForEvoSceneToFinish() {
    unsafe {
        if core::mem::transmute::<_, usize>(
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        ) == (BattleMainCB2 as *const () as usize)
        {
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(TryEvolvePokemon));
        }
    }
}
pub(crate) unsafe extern "C" fn ReturnFromBattleToOverworld() {
    unsafe {
        if !((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0) {
            RandomlyGivePartyPokerus((&raw mut gPlayerParty).cast::<u8>());
            PartySpreadPokerus((&raw mut gPlayerParty).cast::<u8>());
        }
        if ((((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 2u32) != 0)
            && ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
        {
            return;
        }
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).read()) as u16));
        crate::c::bf_write(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            (0u8) as i32,
        );
        (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(
            ((&raw mut gPreBattleCallback1)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
        if (((&raw mut gBattleTypeFlags).cast::<u8>().cast::<u32>()).read() & 1024u32) != 0 {
            UpdateRoamerHPStatus((&raw mut gEnemyParty).cast::<u8>());
            if ((((((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32)
                != 0)
                || (((((&raw mut gBattleOutcome).cast::<u8>().cast::<u8>()).read()) as i32) == 7i32)
            {
                SetRoamerInactive();
            }
        }
        m4aSongNumStop(90u16);
        SetMainCallback2(
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(8)
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunBattleScriptCommands_PopCallbacksStack() {
    unsafe {
        if (((((&raw mut gCurrentActionFuncId).cast::<u8>().cast::<u8>()).read()) as i32) == 11i32)
            || (((((&raw mut gCurrentActionFuncId).cast::<u8>().cast::<u8>()).read()) as i32)
                == 12i32)
        {
            if ((((((((&raw mut gBattleResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32))
            .read()) as i32)
                != 0i32
            {
                let __p1 = (((((&raw mut gBattleResources).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(12)
                .cast::<*mut u8>())
                .read())
                .wrapping_add(32);
                (__p1).write(((__p1).read()).wrapping_sub(1));
            }
            ((&raw mut gBattleMainFunc)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(
                (((((((&raw mut gBattleResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(
                    ((((((((&raw mut gBattleResources).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(32))
                    .read()) as i32) as isize,
                ))
                .read(),
            );
        } else {
            if ((&raw mut gBattleControllerExecFlags)
                .cast::<u8>()
                .cast::<u32>())
            .read()
                == 0u32
            {
                (((((&raw mut gBattleScriptingCommandsTable)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(
                    (((((&raw mut gBattlescriptCurrInstr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .read()) as i32) as isize,
                ))
                .read())
                .unwrap_unchecked()();
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunBattleScriptCommands() {
    unsafe {
        if ((&raw mut gBattleControllerExecFlags)
            .cast::<u8>()
            .cast::<u32>())
        .read()
            == 0u32
        {
            (((((&raw mut gBattleScriptingCommandsTable)
                .cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(
                (((((&raw mut gBattlescriptCurrInstr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()();
        }
    }
}
