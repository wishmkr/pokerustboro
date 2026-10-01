//! Translated from `src/battle_message.c` by tools/rustport/c2rs.py.
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
    clippy::eq_op,
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    unused_assignments,
    unused_variables
)]

use crate::battle_anim_mons::{GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide};
use crate::battle_main::{
    GetBattleWindowTemplatePixelWidth, gActiveBattler, gBattleResources, gBattleScripting,
    gBattleStruct, gBattleTypeFlags, gBattlerAttacker, gBattlerTarget, gEffectBattler,
    gEnigmaBerries, gLastUsedAbility, gLastUsedItem, gPotentialItemEffectBattler,
};
use crate::battle_main::{
    gBattleBufferA, gBattleTextBuff1, gBattleTextBuff2, gBattleTextBuff3, gBattlerPartyIndexes,
    gDisplayedStringBattle, gMoveSelectionCursor,
};
use crate::battle_setup::{
    GetTrainerALoseText, GetTrainerBLoseText, gPartnerTrainerId, gTrainerBattleOpponent_A,
    gTrainerBattleOpponent_B,
};
use crate::battle_tower::{
    GetEreaderTrainerClassId, GetEreaderTrainerName, GetFrontierOpponentClass,
    GetFrontierTrainerName,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::FlagGet;
use crate::frontier_util::{
    CopyFrontierBrainTrainerName, CopyFrontierTrainerText, GetFrontierBrainTrainerClass,
};
use crate::international_string_util::GetStringCenterAlignXOffsetWithLetterSpacing;
use crate::item::CopyItemName;
use crate::link::{GetMultiplayerId, gLinkPlayers};
use crate::load_save::gSaveBlock2Ptr;
use crate::menu::GetPlayerTextSpeedDelay;
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::pokemon::{
    GetBattlerMultiplayerId, GetMonData3, GetSecretBaseTrainerClass, GetSpeciesName,
    GetUnionRoomTrainerClass, gEnemyParty, gPlayerParty,
};
use crate::recorded_battle::{GetTextSpeedInRecordedBattle, gRecordedBattleMultiplayerId};
use crate::string_util::{ConvertIntToDecimalStringN, StringAppend, StringCopy};
use crate::string_util::{ConvertInternationalString, StringGet_Nickname};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::trainer_hill::{
    CopyTrainerHillTrainerText, GetTrainerHillOpponentClass, GetTrainerHillTrainerName,
};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sText_Trainer1LoseText sText_PkmnGainedEXP sText_EmptyString4 sText_ABoosted sText_PkmnGrewToLv sText_PkmnLearnedMove sText_TryToLearnMove1 sText_TryToLearnMove2 sText_TryToLearnMove3 sText_PkmnForgotMove sText_StopLearningMove sText_DidNotLearnMove sText_UseNextPkmn sText_AttackMissed sText_PkmnProtectedItself sText_AvoidedDamage sText_PkmnMakesGroundMiss sText_PkmnAvoidedAttack sText_ItDoesntAffect sText_AttackerFainted sText_TargetFainted sText_PlayerGotMoney sText_PlayerWhiteout sText_PlayerWhiteout2 sText_PreventsEscape sText_CantEscape2 sText_AttackerCantEscape sText_HitXTimes sText_PkmnFellAsleep sText_PkmnMadeSleep sText_PkmnAlreadyAsleep sText_PkmnAlreadyAsleep2 sText_PkmnWasntAffected sText_PkmnWasPoisoned sText_PkmnPoisonedBy sText_PkmnHurtByPoison sText_PkmnAlreadyPoisoned sText_PkmnBadlyPoisoned sText_PkmnEnergyDrained sText_PkmnWasBurned sText_PkmnBurnedBy sText_PkmnHurtByBurn sText_PkmnAlreadyHasBurn sText_PkmnWasFrozen sText_PkmnFrozenBy sText_PkmnIsFrozen sText_PkmnWasDefrosted sText_PkmnWasDefrosted2 sText_PkmnWasDefrostedBy sText_PkmnWasParalyzed sText_PkmnWasParalyzedBy sText_PkmnIsParalyzed sText_PkmnIsAlreadyParalyzed sText_PkmnHealedParalysis sText_PkmnDreamEaten sText_StatsWontIncrease sText_StatsWontDecrease sText_TeamStoppedWorking sText_FoeStoppedWorking sText_PkmnIsConfused sText_PkmnHealedConfusion sText_PkmnWasConfused sText_PkmnAlreadyConfused sText_PkmnFellInLove sText_PkmnInLove sText_PkmnImmobilizedByLove sText_PkmnBlownAway sText_PkmnChangedType sText_PkmnFlinched sText_PkmnRegainedHealth sText_PkmnHPFull sText_PkmnRaisedSpDef sText_PkmnRaisedSpDefALittle sText_PkmnRaisedDef sText_PkmnRaisedDefALittle sText_PkmnCoveredByVeil sText_PkmnUsedSafeguard sText_PkmnSafeguardExpired sText_PkmnWentToSleep sText_PkmnSleptHealthy sText_PkmnWhippedWhirlwind sText_PkmnTookSunlight sText_PkmnLoweredHead sText_PkmnIsGlowing sText_PkmnFlewHigh sText_PkmnDugHole sText_PkmnHidUnderwater sText_PkmnSprangUp sText_PkmnSqueezedByBind sText_PkmnTrappedInVortex sText_PkmnTrappedBySandTomb sText_PkmnWrappedBy sText_PkmnClamped sText_PkmnHurtBy sText_PkmnFreedFrom sText_PkmnCrashed gText_PkmnShroudedInMist sText_PkmnProtectedByMist gText_PkmnGettingPumped sText_PkmnHitWithRecoil sText_PkmnProtectedItself2 sText_PkmnBuffetedBySandstorm sText_PkmnPeltedByHail sText_PkmnsXWoreOff sText_PkmnSeeded sText_PkmnEvadedAttack sText_PkmnSappedByLeechSeed sText_PkmnFastAsleep sText_PkmnWokeUp sText_PkmnUproarKeptAwake sText_PkmnWokeUpInUproar sText_PkmnCausedUproar sText_PkmnMakingUproar sText_PkmnCalmedDown sText_PkmnCantSleepInUproar sText_PkmnStockpiled sText_PkmnCantStockpile sText_PkmnCantSleepInUproar2 sText_UproarKeptPkmnAwake sText_PkmnStayedAwakeUsing sText_PkmnStoringEnergy sText_PkmnUnleashedEnergy sText_PkmnFatigueConfusion sText_PlayerPickedUpMoney sText_PkmnUnaffected sText_PkmnTransformedInto sText_PkmnMadeSubstitute sText_PkmnHasSubstitute sText_SubstituteDamaged sText_PkmnSubstituteFaded sText_PkmnMustRecharge sText_PkmnRageBuilding sText_PkmnMoveWasDisabled sText_PkmnMoveDisabledNoMore sText_PkmnGotEncore sText_PkmnEncoreEnded sText_PkmnTookAim sText_PkmnSketchedMove sText_PkmnTryingToTakeFoe sText_PkmnTookFoe sText_PkmnReducedPP sText_PkmnStoleItem sText_TargetCantEscapeNow sText_PkmnFellIntoNightmare sText_PkmnLockedInNightmare sText_PkmnLaidCurse sText_PkmnAfflictedByCurse sText_SpikesScattered sText_PkmnHurtBySpikes sText_PkmnIdentified sText_PkmnPerishCountFell sText_PkmnBracedItself sText_PkmnEnduredHit sText_MagnitudeStrength sText_PkmnCutHPMaxedAttack sText_PkmnCopiedStatChanges sText_PkmnGotFree sText_PkmnShedLeechSeed sText_PkmnBlewAwaySpikes sText_PkmnFledFromBattle sText_PkmnForesawAttack sText_PkmnTookAttack sText_PkmnChoseXAsDestiny sText_PkmnAttack sText_PkmnCenterAttention sText_PkmnChargingPower sText_NaturePowerTurnedInto sText_PkmnStatusNormal sText_PkmnSubjectedToTorment sText_PkmnTighteningFocus sText_PkmnFellForTaunt sText_PkmnReadyToHelp sText_PkmnSwitchedItems sText_PkmnObtainedX sText_PkmnObtainedX2 sText_PkmnObtainedXYObtainedZ sText_PkmnCopiedFoe sText_PkmnMadeWish sText_PkmnWishCameTrue sText_PkmnPlantedRoots sText_PkmnAbsorbedNutrients sText_PkmnAnchoredItself sText_PkmnWasMadeDrowsy sText_PkmnKnockedOff sText_PkmnSwappedAbilities sText_PkmnSealedOpponentMove sText_PkmnWantsGrudge sText_PkmnLostPPGrudge sText_PkmnShroudedItself sText_PkmnMoveBounced sText_PkmnWaitsForTarget sText_PkmnSnatchedMove sText_ElectricityWeakened sText_FireWeakened sText_XFoundOneY sText_SoothingAroma sText_ItemsCantBeUsedNow sText_ForXCommaYZ sText_PkmnUsedXToGetPumped sText_PkmnLostFocus sText_PkmnWasDraggedOut sText_TheWallShattered sText_ButNoEffect sText_PkmnHasNoMovesLeft sText_PkmnMoveIsDisabled sText_PkmnCantUseMoveTorment sText_PkmnCantUseMoveTaunt sText_PkmnCantUseMoveSealed sText_PkmnMadeItRain sText_PkmnRaisedSpeed sText_PkmnProtectedBy sText_PkmnPreventsUsage sText_PkmnRestoredHPUsing sText_PkmnsXMadeYUseless sText_PkmnChangedTypeWith sText_PkmnPreventsParalysisWith sText_PkmnPreventsRomanceWith sText_PkmnPreventsPoisoningWith sText_PkmnPreventsConfusionWith sText_PkmnRaisedFirePowerWith sText_PkmnAnchorsItselfWith sText_PkmnCutsAttackWith sText_PkmnPreventsStatLossWith sText_PkmnHurtsWith sText_PkmnTraced sText_PkmnsXPreventsBurns sText_PkmnsXBlocksY sText_PkmnsXBlocksY2 sText_PkmnsXRestoredHPALittle2 sText_PkmnsXWhippedUpSandstorm sText_PkmnsXIntensifiedSun sText_PkmnsXPreventsYLoss sText_PkmnsXInfatuatedY sText_PkmnsXMadeYIneffective sText_PkmnsXCuredYProblem sText_ItSuckedLiquidOoze sText_PkmnTransformed sText_PkmnsXTookAttack gText_PkmnsXPreventsSwitching sText_PreventedFromWorking sText_PkmnsXMadeItIneffective sText_PkmnsXPreventsFlinching sText_PkmnsXPreventsYsZ sText_PkmnsXCuredItsYProblem sText_PkmnsXHadNoEffectOnY sText_StatSharply gText_StatRose sText_StatHarshly sText_StatFell sText_AttackersStatRose gText_DefendersStatRose sText_UsingItemTheStatOfPkmnRose sText_AttackersStatFell sText_DefendersStatFell sText_StatsWontIncrease2 sText_StatsWontDecrease2 sText_CriticalHit sText_OneHitKO sText_123Poof sText_AndEllipsis sText_HMMovesCantBeForgotten sText_NotVeryEffective sText_SuperEffective sText_GotAwaySafely sText_PkmnFledUsingIts sText_PkmnFledUsing sText_WildPkmnFled sText_PlayerDefeatedLinkTrainer sText_TwoLinkTrainersDefeated sText_PlayerLostAgainstLinkTrainer sText_PlayerLostToTwo sText_PlayerBattledToDrawLinkTrainer sText_PlayerBattledToDrawVsTwo sText_WildFled sText_TwoWildFled sText_NoRunningFromTrainers sText_CantEscape sText_DontLeaveBirch sText_ButNothingHappened sText_ButItFailed sText_ItHurtConfusion sText_MirrorMoveFailed sText_StartedToRain sText_DownpourStarted sText_RainContinues sText_DownpourContinues sText_RainStopped sText_SandstormBrewed sText_SandstormRages sText_SandstormSubsided sText_SunlightGotBright sText_SunlightStrong sText_SunlightFaded sText_StartedHail sText_HailContinues sText_HailStopped sText_FailedToSpitUp sText_FailedToSwallow sText_WindBecameHeatWave sText_StatChangesGone sText_CoinsScattered sText_TooWeakForSubstitute sText_SharedPain sText_BellChimed sText_FaintInThree sText_NoPPLeft sText_ButNoPPLeft sText_PkmnIgnoresAsleep sText_PkmnIgnoredOrders sText_PkmnBeganToNap sText_PkmnLoafing sText_PkmnWontObey sText_PkmnTurnedAway sText_PkmnPretendNotNotice sText_EnemyAboutToSwitchPkmn sText_PkmnLearnedMove2 sText_PlayerDefeatedLinkTrainerTrainer1 sText_CreptCloser sText_CantGetCloser sText_PkmnWatchingCarefully sText_PkmnCuriousAboutX sText_PkmnEnthralledByX sText_PkmnIgnoredX sText_ThrewPokeblockAtPkmn sText_OutOfSafariBalls sText_OpponentMon1Appeared sText_WildPkmnAppeared sText_LegendaryPkmnAppeared sText_WildPkmnAppearedPause sText_TwoWildPkmnAppeared sText_Trainer1WantsToBattle sText_LinkTrainerWantsToBattle sText_TwoLinkTrainersWantToBattle sText_Trainer1SentOutPkmn sText_Trainer1SentOutTwoPkmn sText_Trainer1SentOutPkmn2 sText_LinkTrainerSentOutPkmn sText_LinkTrainerSentOutTwoPkmn sText_TwoLinkTrainersSentOutPkmn sText_LinkTrainerSentOutPkmn2 sText_LinkTrainerMultiSentOutPkmn sText_GoPkmn sText_GoTwoPkmn sText_GoPkmn2 sText_DoItPkmn sText_GoForItPkmn sText_YourFoesWeakGetEmPkmn sText_LinkPartnerSentOutPkmnGoPkmn sText_PkmnThatsEnough sText_PkmnComeBack sText_PkmnOkComeBack sText_PkmnGoodComeBack sText_Trainer1WithdrewPkmn sText_LinkTrainer1WithdrewPkmn sText_LinkTrainer2WithdrewPkmn sText_WildPkmnPrefix sText_FoePkmnPrefix sText_EmptyString8 sText_FoePkmnPrefix2 sText_AllyPkmnPrefix sText_FoePkmnPrefix3 sText_AllyPkmnPrefix2 sText_FoePkmnPrefix4 sText_AllyPkmnPrefix3 sText_AttackerUsedX sText_ExclamationMark sText_ExclamationMark2 sText_ExclamationMark3 sText_ExclamationMark4 sText_ExclamationMark5 sText_HP2 sText_Attack2 sText_Defense2 sText_Speed sText_SpAtk2 sText_SpDef2 sText_Accuracy sText_Evasiveness gStatNamesTable sText_PokeblockWasTooSpicy sText_PokeblockWasTooDry sText_PokeblockWasTooSweet sText_PokeblockWasTooBitter sText_PokeblockWasTooSour gPokeblockWasTooXStringTable sText_PlayerUsedItem sText_WallyUsedItem sText_Trainer1UsedItem sText_TrainerBlockedBall sText_DontBeAThief sText_ItDodgedBall sText_YouMissedPkmn sText_PkmnBrokeFree sText_ItAppearedCaught sText_AarghAlmostHadIt sText_ShootSoClose sText_GotchaPkmnCaughtPlayer sText_GotchaPkmnCaughtWally sText_GiveNicknameCaptured sText_PkmnSentToPC sText_Someones sText_Lanettes sText_PkmnDataAddedToDex sText_ItIsRaining sText_SandstormIsRaging sText_BoxIsFull sText_EnigmaBerry sText_BerrySuffix sText_PkmnsItemCuredParalysis sText_PkmnsItemCuredPoison sText_PkmnsItemHealedBurn sText_PkmnsItemDefrostedIt sText_PkmnsItemWokeIt sText_PkmnsItemSnappedOut sText_PkmnsItemCuredProblem sText_PkmnsItemNormalizedStatus sText_PkmnsItemRestoredHealth sText_PkmnsItemRestoredPP sText_PkmnsItemRestoredStatus sText_PkmnsItemRestoredHPALittle sText_ItemAllowsOnlyYMove sText_PkmnHungOnWithX gText_EmptyString3 sText_YouThrowABallNowRight sText_PkmnIncapableOfPower sText_GlintAppearsInEye sText_PkmnGettingIntoPosition sText_PkmnBeganGrowlingDeeply sText_PkmnEagerForMore sText_DefeatedOpponentByReferee sText_LostToOpponentByReferee sText_TiedOpponentByReferee sText_QuestionForfeitMatch sText_ForfeitedMatch sText_Trainer1WinText sText_Trainer2WinText sText_TwoInGameTrainersDefeated sText_Trainer2LoseText gBattleStringsTable gMissStringIds gNoEscapeStringIds gMoveWeatherChangeStringIds gSandStormHailContinuesStringIds gSandStormHailDmgStringIds gSandStormHailEndStringIds gRainContinuesStringIds gProtectLikeUsedStringIds gReflectLightScreenSafeguardStringIds gLeechSeedStringIds gRestUsedStringIds gUproarOverTurnStringIds gStockpileUsedStringIds gWokeUpStringIds gSwallowFailStringIds gUproarAwakeStringIds gStatUpStringIds gStatDownStringIds gFirstTurnOfTwoStringIds gWrappedStringIds gMistUsedStringIds gFocusEnergyUsedStringIds gTransformUsedStringIds gSubstituteUsedStringIds gGotPoisonedStringIds gGotParalyzedStringIds gFellAsleepStringIds gGotBurnedStringIds gGotFrozenStringIds gGotDefrostedStringIds gKOFailedStringIds gAttractUsedStringIds gAbsorbDrainStringIds gSportsUsedStringIds gPartyStatusHealStringIds gFutureMoveUsedStringIds gBallEscapeStringIds gWeatherStartsStringIds gInobedientStringIds gSafariGetNearStringIds gSafariPokeblockResultStringIds gTrainerItemCuredStatusStringIds gBerryEffectStringIds gBRNPreventionStringIds gPRLZPreventionStringIds gPSNPreventionStringIds gItemSwapStringIds gFlashFireStringIds gCaughtMonStringIds gTrappingMoves gText_PkmnIsEvolving gText_CongratsPkmnEvolved gText_PkmnStoppedEvolving gText_EllipsisQuestionMark gText_WhatWillPkmnDo gText_WhatWillPkmnDo2 gText_WhatWillWallyDo gText_LinkStandby gText_BattleMenu gText_SafariZoneMenu gText_MoveInterfacePP gText_MoveInterfaceType gText_MoveInterfacePPType gText_MoveInterfaceDynamicColors gText_WhichMoveToForget4 gText_BattleYesNoChoice gText_BattleSwitchWhich gText_BattleSwitchWhich2 gText_BattleSwitchWhich3 gText_BattleSwitchWhich4 gText_BattleSwitchWhich5 sText_HP sText_Attack sText_Defense sText_SpAtk sText_SpDef sStatNamesTable2 gText_SafariBalls gText_SafariBallLeft gText_Sleep gText_Poison gText_Burn gText_Paralysis gText_Ice gText_Confusion gText_Love gText_SpaceAndSpace gText_CommaSpace gText_Space2 gText_LineBreak gText_NewLine gText_Are gText_Are2 gText_BadEgg gText_BattleWallyName gText_Win gText_Loss gText_Draw sText_SpaceIs sText_ApostropheS sATypeMove_Table gText_BattleTourney sText_Round1 sText_Round2 sText_Semifinal sText_Final gRoundsStringTable gText_TheGreatNewHope gText_WillChampionshipDreamComeTrue gText_AFormerChampion gText_ThePreviousChampion gText_TheUnbeatenChampion gText_PlayerMon1Name gText_Vs gText_OpponentMon1Name gText_Mind gText_Skill gText_Body gText_Judgment sText_TwoTrainersSentPkmn sText_Trainer2SentOutPkmn sText_TwoTrainersWantToBattle sText_InGamePartnerSentOutZGoN sText_TwoInGameTrainersDefeated sText_Trainer2LoseText sText_PkmnIncapableOfPower sText_GlintAppearsInEye sText_PkmnGettingIntoPosition sText_PkmnBeganGrowlingDeeply sText_PkmnEagerForMore gBattlePalaceFlavorTextTable sText_RefIfNothingIsDecided sText_RefThatsIt sText_RefJudgeMind sText_RefJudgeSkill sText_RefJudgeBody sText_RefPlayerWon sText_RefOpponentWon sText_RefDraw sText_DefeatedOpponentByReferee sText_LostToOpponentByReferee sText_TiedOpponentByReferee sText_RefCommenceBattle gRefereeStringsTable sText_QuestionForfeitMatch sText_ForfeitedMatch sText_Trainer1WinText sText_Trainer2WinText sText_Trainer1Fled sText_PlayerLostAgainstTrainer1 sText_PlayerBattledToDrawTrainer1 gText_RecordBattleToPass gText_BattleRecordedOnPass sText_LinkTrainerWantsToBattlePause sText_TwoLinkTrainersWantToBattlePause sGrammarMoveUsedTable sText_EmptyStatus sTextOnWindowsInfo_Normal sTextOnWindowsInfo_Arena sBattleTextOnWindowsInfo sRecordedBattleTextSpeeds

/// `struct BattleWindowText`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct BattleWindowText {
    pub fillValue: u8,
    pub fontId: u8,
    pub x: u8,
    pub y: u8,
    pub letterSpacing: u8,
    pub lineSpacing: u8,
    pub speed: u8,
    pub fgColor: u8,
    pub bgColor: u8,
    pub shadowColor: u8,
}

unsafe impl Sync for BattleWindowText {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<BattleWindowText>() == 12);
    assert!(offset_of!(BattleWindowText, fillValue) == 0);
    assert!(offset_of!(BattleWindowText, fontId) == 1);
    assert!(offset_of!(BattleWindowText, x) == 2);
    assert!(offset_of!(BattleWindowText, y) == 3);
    assert!(offset_of!(BattleWindowText, letterSpacing) == 4);
    assert!(offset_of!(BattleWindowText, lineSpacing) == 5);
    assert!(offset_of!(BattleWindowText, speed) == 6);
    assert!(offset_of!(BattleWindowText, fgColor) == 7);
    assert!(offset_of!(BattleWindowText, bgColor) == 8);
    assert!(offset_of!(BattleWindowText, shadowColor) == 9);
};

static gBattleStringsTable: Table<CArray<*mut u8, 369>> =
    Table((&raw const crate::data::battle_message::gBattleStringsTable).cast());
static gPokeblockWasTooXStringTable: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::battle_message::gPokeblockWasTooXStringTable).cast());
static gStatNamesTable: Table<CArray<*mut u8, 8>> =
    Table((&raw const crate::data::battle_message::gStatNamesTable).cast());
static sATypeMove_Table: Table<CArray<CArray<u8, 17>, 18>> =
    Table((&raw const crate::data::battle_message::sATypeMove_Table).cast());
static sBattleTextOnWindowsInfo: Table<CArray<*mut BattleWindowText, 2>> =
    Table((&raw const crate::data::battle_message::sBattleTextOnWindowsInfo).cast());
static sGrammarMoveUsedTable: Table<CArray<u16, 118>> =
    Table((&raw const crate::data::battle_message::sGrammarMoveUsedTable).cast());
static sRecordedBattleTextSpeeds: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_message::sRecordedBattleTextSpeeds).cast());
static sText_AllyPkmnPrefix: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::battle_message::sText_AllyPkmnPrefix).cast());
static sText_AllyPkmnPrefix2: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::battle_message::sText_AllyPkmnPrefix2).cast());
static sText_AllyPkmnPrefix3: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::battle_message::sText_AllyPkmnPrefix3).cast());
static sText_ApostropheS: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_message::sText_ApostropheS).cast());
static sText_AttackerUsedX: Table<CArray<u8, 11>> =
    Table((&raw const crate::data::battle_message::sText_AttackerUsedX).cast());
static sText_BerrySuffix: Table<CArray<u8, 7>> =
    Table((&raw const crate::data::battle_message::sText_BerrySuffix).cast());
static sText_DoItPkmn: Table<CArray<u8, 11>> =
    Table((&raw const crate::data::battle_message::sText_DoItPkmn).cast());
static sText_EmptyStatus: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::battle_message::sText_EmptyStatus).cast());
static sText_EnigmaBerry: Table<CArray<u8, 13>> =
    Table((&raw const crate::data::battle_message::sText_EnigmaBerry).cast());
static sText_ExclamationMark: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::battle_message::sText_ExclamationMark).cast());
static sText_ExclamationMark2: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::battle_message::sText_ExclamationMark2).cast());
static sText_ExclamationMark3: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::battle_message::sText_ExclamationMark3).cast());
static sText_ExclamationMark4: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::battle_message::sText_ExclamationMark4).cast());
static sText_ExclamationMark5: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::battle_message::sText_ExclamationMark5).cast());
static sText_FoePkmnPrefix: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::battle_message::sText_FoePkmnPrefix).cast());
static sText_FoePkmnPrefix2: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_message::sText_FoePkmnPrefix2).cast());
static sText_FoePkmnPrefix3: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_message::sText_FoePkmnPrefix3).cast());
static sText_FoePkmnPrefix4: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_message::sText_FoePkmnPrefix4).cast());
static sText_GoForItPkmn: Table<CArray<u8, 15>> =
    Table((&raw const crate::data::battle_message::sText_GoForItPkmn).cast());
static sText_GoPkmn: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::battle_message::sText_GoPkmn).cast());
static sText_GoPkmn2: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::battle_message::sText_GoPkmn2).cast());
static sText_GoTwoPkmn: Table<CArray<u8, 15>> =
    Table((&raw const crate::data::battle_message::sText_GoTwoPkmn).cast());
static sText_GotAwaySafely: Table<CArray<u8, 22>> =
    Table((&raw const crate::data::battle_message::sText_GotAwaySafely).cast());
static sText_InGamePartnerSentOutZGoN: Table<CArray<u8, 27>> =
    Table((&raw const crate::data::battle_message::sText_InGamePartnerSentOutZGoN).cast());
static sText_Lanettes: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::battle_message::sText_Lanettes).cast());
static sText_LegendaryPkmnAppeared: Table<CArray<u8, 19>> =
    Table((&raw const crate::data::battle_message::sText_LegendaryPkmnAppeared).cast());
static sText_LinkPartnerSentOutPkmnGoPkmn: Table<CArray<u8, 24>> =
    Table((&raw const crate::data::battle_message::sText_LinkPartnerSentOutPkmnGoPkmn).cast());
static sText_LinkTrainer1WithdrewPkmn: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::battle_message::sText_LinkTrainer1WithdrewPkmn).cast());
static sText_LinkTrainer2WithdrewPkmn: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::battle_message::sText_LinkTrainer2WithdrewPkmn).cast());
static sText_LinkTrainerMultiSentOutPkmn: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::battle_message::sText_LinkTrainerMultiSentOutPkmn).cast());
static sText_LinkTrainerSentOutPkmn: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::battle_message::sText_LinkTrainerSentOutPkmn).cast());
static sText_LinkTrainerSentOutPkmn2: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::battle_message::sText_LinkTrainerSentOutPkmn2).cast());
static sText_LinkTrainerSentOutTwoPkmn: Table<CArray<u8, 23>> =
    Table((&raw const crate::data::battle_message::sText_LinkTrainerSentOutTwoPkmn).cast());
static sText_LinkTrainerWantsToBattle: Table<CArray<u8, 20>> =
    Table((&raw const crate::data::battle_message::sText_LinkTrainerWantsToBattle).cast());
static sText_LinkTrainerWantsToBattlePause: Table<CArray<u8, 23>> =
    Table((&raw const crate::data::battle_message::sText_LinkTrainerWantsToBattlePause).cast());
static sText_PkmnComeBack: Table<CArray<u8, 15>> =
    Table((&raw const crate::data::battle_message::sText_PkmnComeBack).cast());
static sText_PkmnGoodComeBack: Table<CArray<u8, 21>> =
    Table((&raw const crate::data::battle_message::sText_PkmnGoodComeBack).cast());
static sText_PkmnOkComeBack: Table<CArray<u8, 19>> =
    Table((&raw const crate::data::battle_message::sText_PkmnOkComeBack).cast());
static sText_PkmnThatsEnough: Table<CArray<u8, 30>> =
    Table((&raw const crate::data::battle_message::sText_PkmnThatsEnough).cast());
static sText_PlayerBattledToDrawLinkTrainer: Table<CArray<u8, 37>> =
    Table((&raw const crate::data::battle_message::sText_PlayerBattledToDrawLinkTrainer).cast());
static sText_PlayerBattledToDrawTrainer1: Table<CArray<u8, 40>> =
    Table((&raw const crate::data::battle_message::sText_PlayerBattledToDrawTrainer1).cast());
static sText_PlayerBattledToDrawVsTwo: Table<CArray<u8, 44>> =
    Table((&raw const crate::data::battle_message::sText_PlayerBattledToDrawVsTwo).cast());
static sText_PlayerDefeatedLinkTrainer: Table<CArray<u8, 20>> =
    Table((&raw const crate::data::battle_message::sText_PlayerDefeatedLinkTrainer).cast());
static sText_PlayerDefeatedLinkTrainerTrainer1: Table<CArray<u8, 24>> =
    Table((&raw const crate::data::battle_message::sText_PlayerDefeatedLinkTrainerTrainer1).cast());
static sText_PlayerLostAgainstLinkTrainer: Table<CArray<u8, 24>> =
    Table((&raw const crate::data::battle_message::sText_PlayerLostAgainstLinkTrainer).cast());
static sText_PlayerLostAgainstTrainer1: Table<CArray<u8, 27>> =
    Table((&raw const crate::data::battle_message::sText_PlayerLostAgainstTrainer1).cast());
static sText_PlayerLostToTwo: Table<CArray<u8, 26>> =
    Table((&raw const crate::data::battle_message::sText_PlayerLostToTwo).cast());
static sText_Someones: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::battle_message::sText_Someones).cast());
static sText_SpaceIs: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_message::sText_SpaceIs).cast());
static sText_Trainer1SentOutPkmn: Table<CArray<u8, 19>> =
    Table((&raw const crate::data::battle_message::sText_Trainer1SentOutPkmn).cast());
static sText_Trainer1SentOutPkmn2: Table<CArray<u8, 19>> =
    Table((&raw const crate::data::battle_message::sText_Trainer1SentOutPkmn2).cast());
static sText_Trainer1SentOutTwoPkmn: Table<CArray<u8, 26>> =
    Table((&raw const crate::data::battle_message::sText_Trainer1SentOutTwoPkmn).cast());
static sText_Trainer1WantsToBattle: Table<CArray<u8, 29>> =
    Table((&raw const crate::data::battle_message::sText_Trainer1WantsToBattle).cast());
static sText_Trainer1WithdrewPkmn: Table<CArray<u8, 19>> =
    Table((&raw const crate::data::battle_message::sText_Trainer1WithdrewPkmn).cast());
static sText_Trainer2SentOutPkmn: Table<CArray<u8, 19>> =
    Table((&raw const crate::data::battle_message::sText_Trainer2SentOutPkmn).cast());
static sText_TwoInGameTrainersDefeated: Table<CArray<u8, 32>> =
    Table((&raw const crate::data::battle_message::sText_TwoInGameTrainersDefeated).cast());
static sText_TwoLinkTrainersDefeated: Table<CArray<u8, 23>> =
    Table((&raw const crate::data::battle_message::sText_TwoLinkTrainersDefeated).cast());
static sText_TwoLinkTrainersSentOutPkmn: Table<CArray<u8, 32>> =
    Table((&raw const crate::data::battle_message::sText_TwoLinkTrainersSentOutPkmn).cast());
static sText_TwoLinkTrainersWantToBattle: Table<CArray<u8, 26>> =
    Table((&raw const crate::data::battle_message::sText_TwoLinkTrainersWantToBattle).cast());
static sText_TwoLinkTrainersWantToBattlePause: Table<CArray<u8, 29>> =
    Table((&raw const crate::data::battle_message::sText_TwoLinkTrainersWantToBattlePause).cast());
static sText_TwoTrainersSentPkmn: Table<CArray<u8, 38>> =
    Table((&raw const crate::data::battle_message::sText_TwoTrainersSentPkmn).cast());
static sText_TwoTrainersWantToBattle: Table<CArray<u8, 33>> =
    Table((&raw const crate::data::battle_message::sText_TwoTrainersWantToBattle).cast());
static sText_TwoWildFled: Table<CArray<u8, 20>> =
    Table((&raw const crate::data::battle_message::sText_TwoWildFled).cast());
static sText_TwoWildPkmnAppeared: Table<CArray<u8, 26>> =
    Table((&raw const crate::data::battle_message::sText_TwoWildPkmnAppeared).cast());
static sText_WildFled: Table<CArray<u8, 13>> =
    Table((&raw const crate::data::battle_message::sText_WildFled).cast());
static sText_WildPkmnAppeared: Table<CArray<u8, 19>> =
    Table((&raw const crate::data::battle_message::sText_WildPkmnAppeared).cast());
static sText_WildPkmnAppearedPause: Table<CArray<u8, 21>> =
    Table((&raw const crate::data::battle_message::sText_WildPkmnAppearedPause).cast());
static sText_WildPkmnPrefix: Table<CArray<u8, 6>> =
    Table((&raw const crate::data::battle_message::sText_WildPkmnPrefix).cast());
static sText_YourFoesWeakGetEmPkmn: Table<CArray<u8, 30>> =
    Table((&raw const crate::data::battle_message::sText_YourFoesWeakGetEmPkmn).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattlerAbilities: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleMsgDataPtr: *mut BattleMsgData = null_mut();

/// `AddTextPrinter` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinter(
    a0: *mut TextPrinterTemplate,
    a1: u8,
    a2: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe { crate::text::AddTextPrinter(a0 as _, a1, core::mem::transmute(a2)) }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

pub unsafe fn BufferStringBattle(stringID: u16) {
    let mut stringPtr: *mut u8 = null_mut();
    gBattleMsgDataPtr = &raw mut gBattleBufferA[gActiveBattler][4] as *mut BattleMsgData;
    gLastUsedItem = (*gBattleMsgDataPtr).lastItem;
    gLastUsedAbility = (*gBattleMsgDataPtr).lastAbility;
    gBattleScripting.battler = (*gBattleMsgDataPtr).scrActive;
    (*gBattleStruct).scriptPartyIdx = (*gBattleMsgDataPtr).bakScriptPartyIdx;
    (*gBattleStruct).hpScale = (*gBattleMsgDataPtr).hpScale;
    gPotentialItemEffectBattler = (*gBattleMsgDataPtr).itemEffectBattler;
    (*gBattleStruct).stringMoveType = (*gBattleMsgDataPtr).moveType;
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        sBattlerAbilities[i] = (*gBattleMsgDataPtr).abilities[i];
    }
    let mut i: i32 = 0;
    while i
        < (if 16 >= (if 14 >= 11 { 14 } else { 11 }) {
            16
        } else {
            if 14 >= 11 { 14 } else { 11 }
        })
    {
        gBattleTextBuff1[i] = (*gBattleMsgDataPtr).textBuffs[0][i];
        gBattleTextBuff2[i] = (*gBattleMsgDataPtr).textBuffs[1][i];
        gBattleTextBuff3[i] = (*gBattleMsgDataPtr).textBuffs[2][i];
        i += 1;
    }
    match stringID {
        STRINGID_INTROMSG => {
            if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
                if gBattleTypeFlags & 0x2000002 != 0 {
                    if gBattleTypeFlags & BATTLE_TYPE_TOWER_LINK_MULTI != 0 {
                        stringPtr = sText_TwoTrainersWantToBattle.as_ptr().cast_mut();
                    } else if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
                            stringPtr = sText_TwoLinkTrainersWantToBattlePause.as_ptr().cast_mut();
                        } else {
                            stringPtr = sText_TwoLinkTrainersWantToBattle.as_ptr().cast_mut();
                        }
                    } else {
                        if gTrainerBattleOpponent_A == TRAINER_UNION_ROOM {
                            stringPtr = sText_Trainer1WantsToBattle.as_ptr().cast_mut();
                        } else if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
                            stringPtr = sText_LinkTrainerWantsToBattlePause.as_ptr().cast_mut();
                        } else {
                            stringPtr = sText_LinkTrainerWantsToBattle.as_ptr().cast_mut();
                        }
                    }
                } else {
                    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 {
                        stringPtr = sText_TwoTrainersWantToBattle.as_ptr().cast_mut();
                    } else if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
                        stringPtr = sText_TwoTrainersWantToBattle.as_ptr().cast_mut();
                    } else {
                        stringPtr = sText_Trainer1WantsToBattle.as_ptr().cast_mut();
                    }
                }
            } else {
                if gBattleTypeFlags & BATTLE_TYPE_LEGENDARY != 0 {
                    stringPtr = sText_LegendaryPkmnAppeared.as_ptr().cast_mut();
                } else if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
                    stringPtr = sText_TwoWildPkmnAppeared.as_ptr().cast_mut();
                } else if gBattleTypeFlags & BATTLE_TYPE_WALLY_TUTORIAL != 0 {
                    stringPtr = sText_WildPkmnAppearedPause.as_ptr().cast_mut();
                } else {
                    stringPtr = sText_WildPkmnAppeared.as_ptr().cast_mut();
                }
            }
        }
        STRINGID_INTROSENDOUT => {
            if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
                if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
                    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 {
                        stringPtr = sText_InGamePartnerSentOutZGoN.as_ptr().cast_mut();
                    } else if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
                        stringPtr = sText_GoTwoPkmn.as_ptr().cast_mut();
                    } else if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                        stringPtr = sText_LinkPartnerSentOutPkmnGoPkmn.as_ptr().cast_mut();
                    } else {
                        stringPtr = sText_GoTwoPkmn.as_ptr().cast_mut();
                    }
                } else {
                    stringPtr = sText_GoPkmn.as_ptr().cast_mut();
                }
            } else {
                if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
                    if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
                        stringPtr = sText_TwoTrainersSentPkmn.as_ptr().cast_mut();
                    } else if gBattleTypeFlags & BATTLE_TYPE_TOWER_LINK_MULTI != 0 {
                        stringPtr = sText_TwoTrainersSentPkmn.as_ptr().cast_mut();
                    } else if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                        stringPtr = sText_TwoLinkTrainersSentOutPkmn.as_ptr().cast_mut();
                    } else if gBattleTypeFlags & 0x2000002 != 0 {
                        stringPtr = sText_LinkTrainerSentOutTwoPkmn.as_ptr().cast_mut();
                    } else {
                        stringPtr = sText_Trainer1SentOutTwoPkmn.as_ptr().cast_mut();
                    }
                } else {
                    if gBattleTypeFlags & 0x2000002 == 0 {
                        stringPtr = sText_Trainer1SentOutPkmn.as_ptr().cast_mut();
                    } else if gTrainerBattleOpponent_A == TRAINER_UNION_ROOM {
                        stringPtr = sText_Trainer1SentOutPkmn.as_ptr().cast_mut();
                    } else {
                        stringPtr = sText_LinkTrainerSentOutPkmn.as_ptr().cast_mut();
                    }
                }
            }
        }
        STRINGID_RETURNMON => {
            if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER {
                if (*gBattleStruct).hpScale == 0 {
                    stringPtr = sText_PkmnThatsEnough.as_ptr().cast_mut();
                } else if (*gBattleStruct).hpScale == 1 || gBattleTypeFlags & 1 != 0 {
                    stringPtr = sText_PkmnComeBack.as_ptr().cast_mut();
                } else if (*gBattleStruct).hpScale == 2 {
                    stringPtr = sText_PkmnOkComeBack.as_ptr().cast_mut();
                } else {
                    stringPtr = sText_PkmnGoodComeBack.as_ptr().cast_mut();
                }
            } else {
                if gTrainerBattleOpponent_A == TRAINER_LINK_OPPONENT
                    || gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0
                {
                    if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                        stringPtr = sText_LinkTrainer2WithdrewPkmn.as_ptr().cast_mut();
                    } else {
                        stringPtr = sText_LinkTrainer1WithdrewPkmn.as_ptr().cast_mut();
                    }
                } else {
                    stringPtr = sText_Trainer1WithdrewPkmn.as_ptr().cast_mut();
                }
            }
        }
        STRINGID_SWITCHINMON => {
            if GetBattlerSide(gBattleScripting.battler) == B_SIDE_PLAYER {
                if (*gBattleStruct).hpScale == 0 || gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
                    stringPtr = sText_GoPkmn2.as_ptr().cast_mut();
                } else if (*gBattleStruct).hpScale == 1 {
                    stringPtr = sText_DoItPkmn.as_ptr().cast_mut();
                } else if (*gBattleStruct).hpScale == 2 {
                    stringPtr = sText_GoForItPkmn.as_ptr().cast_mut();
                } else {
                    stringPtr = sText_YourFoesWeakGetEmPkmn.as_ptr().cast_mut();
                }
            } else {
                if gBattleTypeFlags & 0x2000002 != 0 {
                    if gBattleTypeFlags & BATTLE_TYPE_TOWER_LINK_MULTI != 0 {
                        if gBattleScripting.battler == 1 {
                            stringPtr = sText_Trainer1SentOutPkmn2.as_ptr().cast_mut();
                        } else {
                            stringPtr = sText_Trainer2SentOutPkmn.as_ptr().cast_mut();
                        }
                    } else {
                        if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                            stringPtr = sText_LinkTrainerMultiSentOutPkmn.as_ptr().cast_mut();
                        } else if gTrainerBattleOpponent_A == TRAINER_UNION_ROOM {
                            stringPtr = sText_Trainer1SentOutPkmn2.as_ptr().cast_mut();
                        } else {
                            stringPtr = sText_LinkTrainerSentOutPkmn2.as_ptr().cast_mut();
                        }
                    }
                } else {
                    if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
                        if gBattleScripting.battler == 1 {
                            stringPtr = sText_Trainer1SentOutPkmn2.as_ptr().cast_mut();
                        } else {
                            stringPtr = sText_Trainer2SentOutPkmn.as_ptr().cast_mut();
                        }
                    } else {
                        stringPtr = sText_Trainer1SentOutPkmn2.as_ptr().cast_mut();
                    }
                }
            }
        }
        STRINGID_USEDMOVE => {
            ChooseMoveUsedParticle(gBattleTextBuff1.as_mut_ptr());
            if (*gBattleMsgDataPtr).currentMove >= MOVES_COUNT {
                StringCopy(
                    gBattleTextBuff2.as_mut_ptr(),
                    sATypeMove_Table[(*gBattleStruct).stringMoveType]
                        .as_ptr()
                        .cast_mut(),
                );
            } else {
                StringCopy(
                    gBattleTextBuff2.as_mut_ptr(),
                    (*(&raw const crate::data::data_tables::gMoveNames)
                        .cast::<CArray<CArray<u8, 13>, 355>>())[(*gBattleMsgDataPtr).currentMove]
                        .as_ptr()
                        .cast_mut(),
                );
            }
            ChooseTypeOfMoveUsedString(gBattleTextBuff2.as_mut_ptr());
            stringPtr = sText_AttackerUsedX.as_ptr().cast_mut();
        }
        STRINGID_BATTLEEND => {
            if gBattleTextBuff1[0] as i32 & B_OUTCOME_LINK_BATTLE_RAN != 0 {
                gBattleTextBuff1[0] &= 127;
                if GetBattlerSide(gActiveBattler) == B_SIDE_OPPONENT
                    && gBattleTextBuff1[0] != B_OUTCOME_DREW
                {
                    gBattleTextBuff1[0] ^= 3;
                }
                if gBattleTextBuff1[0] == B_OUTCOME_LOST || gBattleTextBuff1[0] == B_OUTCOME_DREW {
                    stringPtr = sText_GotAwaySafely.as_ptr().cast_mut();
                } else if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                    stringPtr = sText_TwoWildFled.as_ptr().cast_mut();
                } else {
                    stringPtr = sText_WildFled.as_ptr().cast_mut();
                }
            } else {
                if GetBattlerSide(gActiveBattler) == B_SIDE_OPPONENT
                    && gBattleTextBuff1[0] != B_OUTCOME_DREW
                {
                    gBattleTextBuff1[0] ^= 3;
                }
                if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                    match gBattleTextBuff1[0] {
                        B_OUTCOME_WON => {
                            if gBattleTypeFlags & BATTLE_TYPE_TOWER_LINK_MULTI != 0 {
                                stringPtr = sText_TwoInGameTrainersDefeated.as_ptr().cast_mut();
                            } else {
                                stringPtr = sText_TwoLinkTrainersDefeated.as_ptr().cast_mut();
                            }
                        }
                        B_OUTCOME_LOST => {
                            stringPtr = sText_PlayerLostToTwo.as_ptr().cast_mut();
                        }
                        B_OUTCOME_DREW => {
                            stringPtr = sText_PlayerBattledToDrawVsTwo.as_ptr().cast_mut();
                        }
                        _ => {}
                    }
                } else if gTrainerBattleOpponent_A == TRAINER_UNION_ROOM {
                    match gBattleTextBuff1[0] {
                        B_OUTCOME_WON => {
                            stringPtr = sText_PlayerDefeatedLinkTrainerTrainer1.as_ptr().cast_mut();
                        }
                        B_OUTCOME_LOST => {
                            stringPtr = sText_PlayerLostAgainstTrainer1.as_ptr().cast_mut();
                        }
                        B_OUTCOME_DREW => {
                            stringPtr = sText_PlayerBattledToDrawTrainer1.as_ptr().cast_mut();
                        }
                        _ => {}
                    }
                } else {
                    match gBattleTextBuff1[0] {
                        B_OUTCOME_WON => {
                            stringPtr = sText_PlayerDefeatedLinkTrainer.as_ptr().cast_mut();
                        }
                        B_OUTCOME_LOST => {
                            stringPtr = sText_PlayerLostAgainstLinkTrainer.as_ptr().cast_mut();
                        }
                        B_OUTCOME_DREW => {
                            stringPtr = sText_PlayerBattledToDrawLinkTrainer.as_ptr().cast_mut();
                        }
                        _ => {}
                    }
                }
            }
        }
        _ => {
            if stringID >= BATTLESTRINGS_COUNT {
                gDisplayedStringBattle[0] = EOS;
                return;
            } else {
                stringPtr = gBattleStringsTable[stringID as i32 - BATTLESTRINGS_TABLE_START];
            }
        }
    }
    BattleStringExpandPlaceholdersToDisplayedString(stringPtr);
}
pub unsafe fn BattleStringExpandPlaceholdersToDisplayedString(src: *mut u8) -> u32 {
    BattleStringExpandPlaceholders(src, gDisplayedStringBattle.as_mut_ptr())
}
unsafe fn TryGetStatusString(mut src: *mut u8) -> *mut u8 {
    let mut status: CArray<u8, 8> = zeroed();
    memcpy(
        status.as_mut_ptr(),
        sText_EmptyStatus.as_ptr().cast_mut(),
        if 8 < 8 { 8 } else { 8 },
    );
    let mut statusPtr: *mut u8 = status.as_mut_ptr();
    for i in 0..8u32 {
        if *src == EOS {
            break;
        }
        *statusPtr = *src;
        src = src.at(1);
        statusPtr = statusPtr.at(1);
    }
    let chars1: u32 = *(&raw mut status[0] as *mut u32);
    let chars2: u32 = *(&raw mut status[4] as *mut u32);
    for i in 0..7u32 {
        if chars1
            == *((*(&raw const crate::data::battle_main::gStatusConditionStringsTable)
                .cast::<CArray<CArray<*mut u8, 2>, 7>>())[i][0] as *mut u32)
            && chars2
                == *((*(&raw const crate::data::battle_main::gStatusConditionStringsTable)
                    .cast::<CArray<CArray<*mut u8, 2>, 7>>())[i][0]
                    .at(4) as *mut u32)
        {
            return (*(&raw const crate::data::battle_main::gStatusConditionStringsTable)
                .cast::<CArray<CArray<*mut u8, 2>, 7>>())[i][1];
        }
    }
    null_mut()
}
pub unsafe fn BattleStringExpandPlaceholders(mut src: *mut u8, dst: *mut u8) -> u32 {
    let mut dstID: u32 = 0;
    let mut toCpy: *mut u8 = null_mut();
    let mut text: CArray<u8, 32> = zeroed();
    let mut multiplayerId: u8 = 0;
    let mut i: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
        multiplayerId = gRecordedBattleMultiplayerId;
    } else {
        multiplayerId = GetMultiplayerId();
    }
    while *src != EOS {
        if *src == PLACEHOLDER_BEGIN {
            src = src.at(1);
            match *src {
                B_TXT_BUFF1 => {
                    if gBattleTextBuff1[0] == B_BUFF_PLACEHOLDER_BEGIN {
                        ExpandBattleTextBuffPlaceholders(
                            gBattleTextBuff1.as_mut_ptr(),
                            gStringVar1.as_mut_ptr(),
                        );
                        toCpy = gStringVar1.as_mut_ptr();
                    } else {
                        toCpy = TryGetStatusString(gBattleTextBuff1.as_mut_ptr());
                        if toCpy.is_null() {
                            toCpy = gBattleTextBuff1.as_mut_ptr();
                        }
                    }
                }
                B_TXT_BUFF2 => {
                    if gBattleTextBuff2[0] == B_BUFF_PLACEHOLDER_BEGIN {
                        ExpandBattleTextBuffPlaceholders(
                            gBattleTextBuff2.as_mut_ptr(),
                            gStringVar2.as_mut_ptr(),
                        );
                        toCpy = gStringVar2.as_mut_ptr();
                    } else {
                        toCpy = gBattleTextBuff2.as_mut_ptr();
                    }
                }
                B_TXT_BUFF3 => {
                    if gBattleTextBuff3[0] == B_BUFF_PLACEHOLDER_BEGIN {
                        ExpandBattleTextBuffPlaceholders(
                            gBattleTextBuff3.as_mut_ptr(),
                            gStringVar3.as_mut_ptr(),
                        );
                        toCpy = gStringVar3.as_mut_ptr();
                    } else {
                        toCpy = gBattleTextBuff3.as_mut_ptr();
                    }
                }
                B_TXT_COPY_VAR_1 => {
                    toCpy = gStringVar1.as_mut_ptr();
                }
                B_TXT_COPY_VAR_2 => {
                    toCpy = gStringVar2.as_mut_ptr();
                }
                B_TXT_COPY_VAR_3 => {
                    toCpy = gStringVar3.as_mut_ptr();
                }
                B_TXT_PLAYER_MON1_NAME => {
                    GetMonData3(
                        &raw mut gPlayerParty
                            [gBattlerPartyIndexes[GetBattlerAtPosition(B_POSITION_PLAYER_LEFT)]],
                        MON_DATA_NICKNAME,
                        text.as_mut_ptr(),
                    );
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_OPPONENT_MON1_NAME => {
                    GetMonData3(
                        &raw mut gEnemyParty
                            [gBattlerPartyIndexes[GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT)]],
                        MON_DATA_NICKNAME,
                        text.as_mut_ptr(),
                    );
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_PLAYER_MON2_NAME => {
                    GetMonData3(
                        &raw mut gPlayerParty
                            [gBattlerPartyIndexes[GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT)]],
                        MON_DATA_NICKNAME,
                        text.as_mut_ptr(),
                    );
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_OPPONENT_MON2_NAME => {
                    GetMonData3(
                        &raw mut gEnemyParty
                            [gBattlerPartyIndexes[GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT)]],
                        MON_DATA_NICKNAME,
                        text.as_mut_ptr(),
                    );
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_LINK_PLAYER_MON1_NAME => {
                    GetMonData3(
                        &raw mut gPlayerParty[gBattlerPartyIndexes[gLinkPlayers[multiplayerId].id]],
                        MON_DATA_NICKNAME,
                        text.as_mut_ptr(),
                    );
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_LINK_OPPONENT_MON1_NAME => {
                    GetMonData3(
                        &raw mut gEnemyParty
                            [gBattlerPartyIndexes[gLinkPlayers[multiplayerId].id as i32 ^ 1]],
                        MON_DATA_NICKNAME,
                        text.as_mut_ptr(),
                    );
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_LINK_PLAYER_MON2_NAME => {
                    GetMonData3(
                        &raw mut gPlayerParty
                            [gBattlerPartyIndexes[gLinkPlayers[multiplayerId].id as i32 ^ 2]],
                        MON_DATA_NICKNAME,
                        text.as_mut_ptr(),
                    );
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_LINK_OPPONENT_MON2_NAME => {
                    GetMonData3(
                        &raw mut gEnemyParty
                            [gBattlerPartyIndexes[gLinkPlayers[multiplayerId].id as i32 ^ 3]],
                        MON_DATA_NICKNAME,
                        text.as_mut_ptr(),
                    );
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_ATK_NAME_WITH_PREFIX_MON1 => {
                    if GetBattlerSide(gBattlerAttacker) != 0 {
                        if gBattleTypeFlags & 8 != 0 {
                            toCpy = sText_FoePkmnPrefix.as_ptr().cast_mut();
                        } else {
                            toCpy = sText_WildPkmnPrefix.as_ptr().cast_mut();
                        }
                        while *toCpy != 0xFF {
                            *dst.at(dstID) = *toCpy;
                            dstID += 1;
                            toCpy = toCpy.at(1);
                        }
                        GetMonData3(
                            &raw mut gEnemyParty[gBattlerPartyIndexes
                                [GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker) & 1)]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    } else {
                        GetMonData3(
                            &raw mut gPlayerParty[gBattlerPartyIndexes
                                [GetBattlerAtPosition(GetBattlerPosition(gBattlerAttacker) & 1)]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    }
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_ATK_PARTNER_NAME => {
                    if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
                        GetMonData3(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[GetBattlerAtPosition(
                                GetBattlerPosition(gBattlerAttacker) & 1,
                            )
                                as i32
                                + 2]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    } else {
                        GetMonData3(
                            &raw mut gEnemyParty[gBattlerPartyIndexes[GetBattlerAtPosition(
                                GetBattlerPosition(gBattlerAttacker) & 1,
                            )
                                as i32
                                + 2]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    }
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_ATK_NAME_WITH_PREFIX => {
                    if GetBattlerSide(gBattlerAttacker) != 0 {
                        if gBattleTypeFlags & 8 != 0 {
                            toCpy = sText_FoePkmnPrefix.as_ptr().cast_mut();
                        } else {
                            toCpy = sText_WildPkmnPrefix.as_ptr().cast_mut();
                        }
                        while *toCpy != 0xFF {
                            *dst.at(dstID) = *toCpy;
                            dstID += 1;
                            toCpy = toCpy.at(1);
                        }
                        GetMonData3(
                            &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerAttacker]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    } else {
                        GetMonData3(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[gBattlerAttacker]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    }
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_DEF_NAME_WITH_PREFIX => {
                    if GetBattlerSide(gBattlerTarget) != 0 {
                        if gBattleTypeFlags & 8 != 0 {
                            toCpy = sText_FoePkmnPrefix.as_ptr().cast_mut();
                        } else {
                            toCpy = sText_WildPkmnPrefix.as_ptr().cast_mut();
                        }
                        while *toCpy != 0xFF {
                            *dst.at(dstID) = *toCpy;
                            dstID += 1;
                            toCpy = toCpy.at(1);
                        }
                        GetMonData3(
                            &raw mut gEnemyParty[gBattlerPartyIndexes[gBattlerTarget]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    } else {
                        GetMonData3(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[gBattlerTarget]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    }
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_EFF_NAME_WITH_PREFIX => {
                    if GetBattlerSide(gEffectBattler) != 0 {
                        if gBattleTypeFlags & 8 != 0 {
                            toCpy = sText_FoePkmnPrefix.as_ptr().cast_mut();
                        } else {
                            toCpy = sText_WildPkmnPrefix.as_ptr().cast_mut();
                        }
                        while *toCpy != 0xFF {
                            *dst.at(dstID) = *toCpy;
                            dstID += 1;
                            toCpy = toCpy.at(1);
                        }
                        GetMonData3(
                            &raw mut gEnemyParty[gBattlerPartyIndexes[gEffectBattler]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    } else {
                        GetMonData3(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[gEffectBattler]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    }
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_ACTIVE_NAME_WITH_PREFIX => {
                    if GetBattlerSide(gActiveBattler) != 0 {
                        if gBattleTypeFlags & 8 != 0 {
                            toCpy = sText_FoePkmnPrefix.as_ptr().cast_mut();
                        } else {
                            toCpy = sText_WildPkmnPrefix.as_ptr().cast_mut();
                        }
                        while *toCpy != 0xFF {
                            *dst.at(dstID) = *toCpy;
                            dstID += 1;
                            toCpy = toCpy.at(1);
                        }
                        GetMonData3(
                            &raw mut gEnemyParty[gBattlerPartyIndexes[gActiveBattler]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    } else {
                        GetMonData3(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[gActiveBattler]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    }
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_SCR_ACTIVE_NAME_WITH_PREFIX => {
                    if GetBattlerSide(gBattleScripting.battler) != 0 {
                        if gBattleTypeFlags & 8 != 0 {
                            toCpy = sText_FoePkmnPrefix.as_ptr().cast_mut();
                        } else {
                            toCpy = sText_WildPkmnPrefix.as_ptr().cast_mut();
                        }
                        while *toCpy != 0xFF {
                            *dst.at(dstID) = *toCpy;
                            dstID += 1;
                            toCpy = toCpy.at(1);
                        }
                        GetMonData3(
                            &raw mut gEnemyParty[gBattlerPartyIndexes[gBattleScripting.battler]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    } else {
                        GetMonData3(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[gBattleScripting.battler]],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    }
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_CURRENT_MOVE => {
                    if (*gBattleMsgDataPtr).currentMove >= MOVES_COUNT {
                        toCpy = sATypeMove_Table[(*gBattleStruct).stringMoveType]
                            .as_ptr()
                            .cast_mut();
                    } else {
                        toCpy = (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<
                            CArray<u8, 13>,
                            355,
                        >>(
                        ))[(*gBattleMsgDataPtr).currentMove]
                            .as_ptr()
                            .cast_mut();
                    }
                }
                B_TXT_LAST_MOVE => {
                    if (*gBattleMsgDataPtr).originallyUsedMove >= MOVES_COUNT {
                        toCpy = sATypeMove_Table[(*gBattleStruct).stringMoveType]
                            .as_ptr()
                            .cast_mut();
                    } else {
                        toCpy = (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<
                            CArray<u8, 13>,
                            355,
                        >>(
                        ))[(*gBattleMsgDataPtr).originallyUsedMove]
                            .as_ptr()
                            .cast_mut();
                    }
                }
                B_TXT_LAST_ITEM => {
                    if gBattleTypeFlags & 0x2000002 != 0 {
                        if gLastUsedItem == ITEM_ENIGMA_BERRY {
                            if gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
                                if gBattleScripting.multiplayerId != 0
                                    && gPotentialItemEffectBattler as i32 & BIT_SIDE as i32 != 0
                                    || gBattleScripting.multiplayerId == 0
                                        && gPotentialItemEffectBattler as i32 & BIT_SIDE as i32 == 0
                                {
                                    StringCopy(
                                        text.as_mut_ptr(),
                                        gEnigmaBerries[gPotentialItemEffectBattler]
                                            .name
                                            .as_mut_ptr(),
                                    );
                                    StringAppend(
                                        text.as_mut_ptr(),
                                        sText_BerrySuffix.as_ptr().cast_mut(),
                                    );
                                    toCpy = text.as_mut_ptr();
                                } else {
                                    toCpy = sText_EnigmaBerry.as_ptr().cast_mut();
                                }
                            } else {
                                if gLinkPlayers[gBattleScripting.multiplayerId].id
                                    == gPotentialItemEffectBattler as u16
                                {
                                    StringCopy(
                                        text.as_mut_ptr(),
                                        gEnigmaBerries[gPotentialItemEffectBattler]
                                            .name
                                            .as_mut_ptr(),
                                    );
                                    StringAppend(
                                        text.as_mut_ptr(),
                                        sText_BerrySuffix.as_ptr().cast_mut(),
                                    );
                                    toCpy = text.as_mut_ptr();
                                } else {
                                    toCpy = sText_EnigmaBerry.as_ptr().cast_mut();
                                }
                            }
                        } else {
                            CopyItemName(gLastUsedItem, text.as_mut_ptr());
                            toCpy = text.as_mut_ptr();
                        }
                    } else {
                        CopyItemName(gLastUsedItem, text.as_mut_ptr());
                        toCpy = text.as_mut_ptr();
                    }
                }
                B_TXT_LAST_ABILITY => {
                    toCpy = (*(&raw const crate::data::battle_main::gAbilityNames).cast::<CArray<
                        CArray<u8, 13>,
                        0,
                    >>(
                    ))[gLastUsedAbility]
                        .as_ptr()
                        .cast_mut();
                }
                B_TXT_ATK_ABILITY => {
                    toCpy = (*(&raw const crate::data::battle_main::gAbilityNames).cast::<CArray<
                        CArray<u8, 13>,
                        0,
                    >>(
                    ))[sBattlerAbilities[gBattlerAttacker]]
                        .as_ptr()
                        .cast_mut();
                }
                B_TXT_DEF_ABILITY => {
                    toCpy = (*(&raw const crate::data::battle_main::gAbilityNames).cast::<CArray<
                        CArray<u8, 13>,
                        0,
                    >>(
                    ))[sBattlerAbilities[gBattlerTarget]]
                        .as_ptr()
                        .cast_mut();
                }
                B_TXT_SCR_ACTIVE_ABILITY => {
                    toCpy = (*(&raw const crate::data::battle_main::gAbilityNames).cast::<CArray<
                        CArray<u8, 13>,
                        0,
                    >>(
                    ))[sBattlerAbilities[gBattleScripting.battler]]
                        .as_ptr()
                        .cast_mut();
                }
                B_TXT_EFF_ABILITY => {
                    toCpy = (*(&raw const crate::data::battle_main::gAbilityNames).cast::<CArray<
                        CArray<u8, 13>,
                        0,
                    >>(
                    ))[sBattlerAbilities[gEffectBattler]]
                        .as_ptr()
                        .cast_mut();
                }
                B_TXT_TRAINER1_CLASS => {
                    if gBattleTypeFlags & BATTLE_TYPE_SECRET_BASE != 0 {
                        toCpy = (*(&raw const crate::data::data_tables::gTrainerClassNames)
                            .cast::<CArray<CArray<u8, 13>, 0>>())[GetSecretBaseTrainerClass()]
                        .as_ptr()
                        .cast_mut();
                    } else if gTrainerBattleOpponent_A == TRAINER_UNION_ROOM {
                        toCpy = (*(&raw const crate::data::data_tables::gTrainerClassNames)
                            .cast::<CArray<CArray<u8, 13>, 0>>())[GetUnionRoomTrainerClass()]
                        .as_ptr()
                        .cast_mut();
                    } else if gTrainerBattleOpponent_A == TRAINER_FRONTIER_BRAIN {
                        toCpy = (*(&raw const crate::data::data_tables::gTrainerClassNames)
                            .cast::<CArray<CArray<u8, 13>, 0>>())[GetFrontierBrainTrainerClass()]
                        .as_ptr()
                        .cast_mut();
                    } else if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
                        toCpy = (*(&raw const crate::data::data_tables::gTrainerClassNames)
                            .cast::<CArray<CArray<u8, 13>, 0>>())
                            [GetFrontierOpponentClass(gTrainerBattleOpponent_A)]
                        .as_ptr()
                        .cast_mut();
                    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER_HILL != 0 {
                        toCpy = (*(&raw const crate::data::data_tables::gTrainerClassNames)
                            .cast::<CArray<CArray<u8, 13>, 0>>())
                            [GetTrainerHillOpponentClass(gTrainerBattleOpponent_A)]
                        .as_ptr()
                        .cast_mut();
                    } else if gBattleTypeFlags & BATTLE_TYPE_EREADER_TRAINER != 0 {
                        toCpy = (*(&raw const crate::data::data_tables::gTrainerClassNames)
                            .cast::<CArray<CArray<u8, 13>, 0>>())[GetEreaderTrainerClassId()]
                        .as_ptr()
                        .cast_mut();
                    } else {
                        toCpy = (*(&raw const crate::data::data_tables::gTrainerClassNames)
                            .cast::<CArray<CArray<u8, 13>, 0>>())
                            [(*(&raw const crate::data::data_tables::gTrainers)
                                .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A]
                                .trainerClass]
                            .as_ptr()
                            .cast_mut();
                    }
                }
                B_TXT_TRAINER1_NAME => {
                    if gBattleTypeFlags & BATTLE_TYPE_SECRET_BASE != 0 {
                        i = 0;
                        while i < 7 {
                            text[i] = (*(*gBattleResources).secretBase).trainerName[i];
                            i += 1;
                        }
                        text[i] = EOS;
                        ConvertInternationalString(
                            text.as_mut_ptr(),
                            (*(*gBattleResources).secretBase).language,
                        );
                        toCpy = text.as_mut_ptr();
                    } else if gTrainerBattleOpponent_A == TRAINER_UNION_ROOM {
                        toCpy = gLinkPlayers[multiplayerId as i32 ^ BIT_SIDE as i32]
                            .name
                            .as_mut_ptr();
                    } else if gTrainerBattleOpponent_A == TRAINER_FRONTIER_BRAIN {
                        CopyFrontierBrainTrainerName(text.as_mut_ptr());
                        toCpy = text.as_mut_ptr();
                    } else if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
                        GetFrontierTrainerName(text.as_mut_ptr(), gTrainerBattleOpponent_A);
                        toCpy = text.as_mut_ptr();
                    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER_HILL != 0 {
                        GetTrainerHillTrainerName(text.as_mut_ptr(), gTrainerBattleOpponent_A);
                        toCpy = text.as_mut_ptr();
                    } else if gBattleTypeFlags & BATTLE_TYPE_EREADER_TRAINER != 0 {
                        GetEreaderTrainerName(text.as_mut_ptr());
                        toCpy = text.as_mut_ptr();
                    } else {
                        toCpy =
                            (*(&raw const crate::data::data_tables::gTrainers)
                                .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A]
                                .trainerName
                                .as_ptr()
                                .cast_mut();
                    }
                }
                B_TXT_LINK_PLAYER_NAME => {
                    toCpy = gLinkPlayers[multiplayerId].name.as_mut_ptr();
                }
                B_TXT_LINK_PARTNER_NAME => {
                    toCpy = gLinkPlayers
                        [GetBattlerMultiplayerId(gLinkPlayers[multiplayerId].id ^ 2)]
                    .name
                    .as_mut_ptr();
                }
                B_TXT_LINK_OPPONENT1_NAME => {
                    toCpy = gLinkPlayers
                        [GetBattlerMultiplayerId(gLinkPlayers[multiplayerId].id ^ 1)]
                    .name
                    .as_mut_ptr();
                }
                B_TXT_LINK_OPPONENT2_NAME => {
                    toCpy = gLinkPlayers
                        [GetBattlerMultiplayerId(gLinkPlayers[multiplayerId].id ^ 1 ^ 2)]
                    .name
                    .as_mut_ptr();
                }
                B_TXT_LINK_SCR_TRAINER_NAME => {
                    toCpy = gLinkPlayers[GetBattlerMultiplayerId(gBattleScripting.battler as u16)]
                        .name
                        .as_mut_ptr();
                }
                B_TXT_PLAYER_NAME => {
                    if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
                        toCpy = gLinkPlayers[0].name.as_mut_ptr();
                    } else {
                        toCpy = (*gSaveBlock2Ptr).playerName.as_mut_ptr();
                    }
                }
                B_TXT_TRAINER1_LOSE_TEXT => {
                    if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
                        CopyFrontierTrainerText(FRONTIER_PLAYER_WON_TEXT, gTrainerBattleOpponent_A);
                        toCpy = gStringVar4.as_mut_ptr();
                    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER_HILL != 0 {
                        CopyTrainerHillTrainerText(
                            TRAINER_HILL_TEXT_PLAYER_WON,
                            gTrainerBattleOpponent_A,
                        );
                        toCpy = gStringVar4.as_mut_ptr();
                    } else {
                        toCpy = GetTrainerALoseText();
                    }
                }
                B_TXT_TRAINER1_WIN_TEXT => {
                    if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
                        CopyFrontierTrainerText(
                            FRONTIER_PLAYER_LOST_TEXT,
                            gTrainerBattleOpponent_A,
                        );
                        toCpy = gStringVar4.as_mut_ptr();
                    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER_HILL != 0 {
                        CopyTrainerHillTrainerText(
                            TRAINER_HILL_TEXT_PLAYER_LOST,
                            gTrainerBattleOpponent_A,
                        );
                        toCpy = gStringVar4.as_mut_ptr();
                    }
                }
                B_TXT_26 => {
                    if GetBattlerSide(gBattleScripting.battler) != 0 {
                        if gBattleTypeFlags & 8 != 0 {
                            toCpy = sText_FoePkmnPrefix.as_ptr().cast_mut();
                        } else {
                            toCpy = sText_WildPkmnPrefix.as_ptr().cast_mut();
                        }
                        while *toCpy != 0xFF {
                            *dst.at(dstID) = *toCpy;
                            dstID += 1;
                            toCpy = toCpy.at(1);
                        }
                        GetMonData3(
                            &raw mut gEnemyParty[(*gBattleStruct).scriptPartyIdx],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    } else {
                        GetMonData3(
                            &raw mut gPlayerParty[(*gBattleStruct).scriptPartyIdx],
                            MON_DATA_NICKNAME,
                            text.as_mut_ptr(),
                        );
                    }
                    StringGet_Nickname(text.as_mut_ptr());
                    toCpy = text.as_mut_ptr();
                }
                B_TXT_PC_CREATOR_NAME => {
                    if FlagGet(FLAG_SYS_PC_LANETTE) != 0 {
                        toCpy = sText_Lanettes.as_ptr().cast_mut();
                    } else {
                        toCpy = sText_Someones.as_ptr().cast_mut();
                    }
                }
                B_TXT_ATK_PREFIX2 => {
                    if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
                        toCpy = sText_AllyPkmnPrefix2.as_ptr().cast_mut();
                    } else {
                        toCpy = sText_FoePkmnPrefix3.as_ptr().cast_mut();
                    }
                }
                B_TXT_DEF_PREFIX2 => {
                    if GetBattlerSide(gBattlerTarget) == B_SIDE_PLAYER {
                        toCpy = sText_AllyPkmnPrefix2.as_ptr().cast_mut();
                    } else {
                        toCpy = sText_FoePkmnPrefix3.as_ptr().cast_mut();
                    }
                }
                B_TXT_ATK_PREFIX1 => {
                    if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
                        toCpy = sText_AllyPkmnPrefix.as_ptr().cast_mut();
                    } else {
                        toCpy = sText_FoePkmnPrefix2.as_ptr().cast_mut();
                    }
                }
                B_TXT_DEF_PREFIX1 => {
                    if GetBattlerSide(gBattlerTarget) == B_SIDE_PLAYER {
                        toCpy = sText_AllyPkmnPrefix.as_ptr().cast_mut();
                    } else {
                        toCpy = sText_FoePkmnPrefix2.as_ptr().cast_mut();
                    }
                }
                B_TXT_ATK_PREFIX3 => {
                    if GetBattlerSide(gBattlerAttacker) == B_SIDE_PLAYER {
                        toCpy = sText_AllyPkmnPrefix3.as_ptr().cast_mut();
                    } else {
                        toCpy = sText_FoePkmnPrefix4.as_ptr().cast_mut();
                    }
                }
                B_TXT_DEF_PREFIX3 => {
                    if GetBattlerSide(gBattlerTarget) == B_SIDE_PLAYER {
                        toCpy = sText_AllyPkmnPrefix3.as_ptr().cast_mut();
                    } else {
                        toCpy = sText_FoePkmnPrefix4.as_ptr().cast_mut();
                    }
                }
                B_TXT_TRAINER2_CLASS => {
                    if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
                        toCpy = (*(&raw const crate::data::data_tables::gTrainerClassNames)
                            .cast::<CArray<CArray<u8, 13>, 0>>())
                            [GetFrontierOpponentClass(gTrainerBattleOpponent_B)]
                        .as_ptr()
                        .cast_mut();
                    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER_HILL != 0 {
                        toCpy = (*(&raw const crate::data::data_tables::gTrainerClassNames)
                            .cast::<CArray<CArray<u8, 13>, 0>>())
                            [GetTrainerHillOpponentClass(gTrainerBattleOpponent_B)]
                        .as_ptr()
                        .cast_mut();
                    } else {
                        toCpy = (*(&raw const crate::data::data_tables::gTrainerClassNames)
                            .cast::<CArray<CArray<u8, 13>, 0>>())
                            [(*(&raw const crate::data::data_tables::gTrainers)
                                .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_B]
                                .trainerClass]
                            .as_ptr()
                            .cast_mut();
                    }
                }
                B_TXT_TRAINER2_NAME => {
                    if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
                        GetFrontierTrainerName(text.as_mut_ptr(), gTrainerBattleOpponent_B);
                        toCpy = text.as_mut_ptr();
                    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER_HILL != 0 {
                        GetTrainerHillTrainerName(text.as_mut_ptr(), gTrainerBattleOpponent_B);
                        toCpy = text.as_mut_ptr();
                    } else {
                        toCpy =
                            (*(&raw const crate::data::data_tables::gTrainers)
                                .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_B]
                                .trainerName
                                .as_ptr()
                                .cast_mut();
                    }
                }
                B_TXT_TRAINER2_LOSE_TEXT => {
                    if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
                        CopyFrontierTrainerText(FRONTIER_PLAYER_WON_TEXT, gTrainerBattleOpponent_B);
                        toCpy = gStringVar4.as_mut_ptr();
                    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER_HILL != 0 {
                        CopyTrainerHillTrainerText(
                            TRAINER_HILL_TEXT_PLAYER_WON,
                            gTrainerBattleOpponent_B,
                        );
                        toCpy = gStringVar4.as_mut_ptr();
                    } else {
                        toCpy = GetTrainerBLoseText();
                    }
                }
                B_TXT_TRAINER2_WIN_TEXT => {
                    if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
                        CopyFrontierTrainerText(
                            FRONTIER_PLAYER_LOST_TEXT,
                            gTrainerBattleOpponent_B,
                        );
                        toCpy = gStringVar4.as_mut_ptr();
                    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER_HILL != 0 {
                        CopyTrainerHillTrainerText(
                            TRAINER_HILL_TEXT_PLAYER_LOST,
                            gTrainerBattleOpponent_B,
                        );
                        toCpy = gStringVar4.as_mut_ptr();
                    }
                }
                B_TXT_PARTNER_CLASS => {
                    toCpy = (*(&raw const crate::data::data_tables::gTrainerClassNames)
                        .cast::<CArray<CArray<u8, 13>, 0>>())
                        [GetFrontierOpponentClass(gPartnerTrainerId)]
                    .as_ptr()
                    .cast_mut();
                }
                B_TXT_PARTNER_NAME => {
                    GetFrontierTrainerName(text.as_mut_ptr(), gPartnerTrainerId);
                    toCpy = text.as_mut_ptr();
                }
                _ => {}
            }
            while *toCpy != EOS {
                *dst.at(dstID) = *toCpy;
                dstID += 1;
                toCpy = toCpy.at(1);
            }
            if *src == B_TXT_TRAINER1_LOSE_TEXT
                || *src == B_TXT_TRAINER2_LOSE_TEXT
                || *src == B_TXT_TRAINER1_WIN_TEXT
                || *src == B_TXT_TRAINER2_WIN_TEXT
            {
                *dst.at(dstID) = EXT_CTRL_CODE_BEGIN;
                dstID += 1;
                *dst.at(dstID) = EXT_CTRL_CODE_PAUSE_UNTIL_PRESS;
                dstID += 1;
            }
        } else {
            *dst.at(dstID) = *src;
            dstID += 1;
        }
        src = src.at(1);
    }
    *dst.at(dstID) = *src;
    dstID += 1;
    dstID
}
unsafe fn ExpandBattleTextBuffPlaceholders(src: *mut u8, dst: *mut u8) {
    let mut srcID: u32 = 1;
    let mut value: u32 = 0;
    let mut nickname: CArray<u8, 11> = zeroed();
    let mut hword: u16 = 0;
    *dst = EOS;
    while *src.at(srcID) != B_BUFF_EOS {
        match *src.at(srcID) {
            B_BUFF_STRING => {
                hword = *src.at(srcID + 1) as u16 | (*src.at(srcID + 1).at(1) as u16) << 8;
                StringAppend(
                    dst,
                    gBattleStringsTable[hword as i32 - BATTLESTRINGS_TABLE_START],
                );
                srcID += 3;
            }
            B_BUFF_NUMBER => {
                match *src.at(srcID + 1) {
                    1 => {
                        value = *src.at(srcID + 3) as u32;
                    }
                    2 => {
                        value = *src.at(srcID + 3) as u32 | (*src.at(srcID + 3).at(1) as u32) << 8;
                    }
                    4 => {
                        value = *src.at(srcID + 3) as u32
                            | (*src.at(srcID + 3).at(1) as u32) << 8
                            | (*src.at(srcID + 3).at(2) as u32) << 16
                            | (*src.at(srcID + 3).at(3) as u32) << 24;
                    }
                    _ => {}
                }
                ConvertIntToDecimalStringN(
                    dst,
                    value as i32,
                    STR_CONV_MODE_LEFT_ALIGN,
                    *src.at(srcID + 2),
                );
                srcID += *src.at(srcID + 1) as u32 + 3;
            }
            B_BUFF_MOVE => {
                StringAppend(
                    dst,
                    (*(&raw const crate::data::data_tables::gMoveNames)
                        .cast::<CArray<CArray<u8, 13>, 355>>())
                        [*src.at(srcID + 1) as i32 | (*src.at(srcID + 1).at(1) as i32) << 8]
                        .as_ptr()
                        .cast_mut(),
                );
                srcID += 3;
            }
            B_BUFF_TYPE => {
                StringAppend(
                    dst,
                    (*(&raw const crate::data::battle_main::gTypeNames)
                        .cast::<CArray<CArray<u8, 7>, 18>>())[*src.at(srcID + 1)]
                    .as_ptr()
                    .cast_mut(),
                );
                srcID += 2;
            }
            B_BUFF_MON_NICK_WITH_PREFIX => {
                if GetBattlerSide(*src.at(srcID + 1)) == B_SIDE_PLAYER {
                    GetMonData3(
                        &raw mut gPlayerParty[*src.at(srcID + 2)],
                        MON_DATA_NICKNAME,
                        nickname.as_mut_ptr(),
                    );
                } else {
                    if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
                        StringAppend(dst, sText_FoePkmnPrefix.as_ptr().cast_mut());
                    } else {
                        StringAppend(dst, sText_WildPkmnPrefix.as_ptr().cast_mut());
                    }
                    GetMonData3(
                        &raw mut gEnemyParty[*src.at(srcID + 2)],
                        MON_DATA_NICKNAME,
                        nickname.as_mut_ptr(),
                    );
                }
                StringGet_Nickname(nickname.as_mut_ptr());
                StringAppend(dst, nickname.as_mut_ptr());
                srcID += 3;
            }
            B_BUFF_STAT => {
                StringAppend(dst, gStatNamesTable[*src.at(srcID + 1)]);
                srcID += 2;
            }
            B_BUFF_SPECIES => {
                GetSpeciesName(
                    dst,
                    *src.at(srcID + 1) as u16 | (*src.at(srcID + 1).at(1) as u16) << 8,
                );
                srcID += 3;
            }
            B_BUFF_MON_NICK => {
                if GetBattlerSide(*src.at(srcID + 1)) == B_SIDE_PLAYER {
                    GetMonData3(
                        &raw mut gPlayerParty[*src.at(srcID + 2)],
                        MON_DATA_NICKNAME,
                        dst,
                    );
                } else {
                    GetMonData3(
                        &raw mut gEnemyParty[*src.at(srcID + 2)],
                        MON_DATA_NICKNAME,
                        dst,
                    );
                }
                StringGet_Nickname(dst);
                srcID += 3;
            }
            B_BUFF_NEGATIVE_FLAVOR => {
                StringAppend(dst, gPokeblockWasTooXStringTable[*src.at(srcID + 1)]);
                srcID += 2;
            }
            B_BUFF_ABILITY => {
                StringAppend(
                    dst,
                    (*(&raw const crate::data::battle_main::gAbilityNames)
                        .cast::<CArray<CArray<u8, 13>, 0>>())[*src.at(srcID + 1)]
                    .as_ptr()
                    .cast_mut(),
                );
                srcID += 2;
            }
            B_BUFF_ITEM => {
                hword = *src.at(srcID + 1) as u16 | (*src.at(srcID + 1).at(1) as u16) << 8;
                if gBattleTypeFlags & 0x2000002 != 0 {
                    if hword == ITEM_ENIGMA_BERRY {
                        if gLinkPlayers[gBattleScripting.multiplayerId].id
                            == gPotentialItemEffectBattler as u16
                        {
                            StringCopy(
                                dst,
                                gEnigmaBerries[gPotentialItemEffectBattler]
                                    .name
                                    .as_mut_ptr(),
                            );
                            StringAppend(dst, sText_BerrySuffix.as_ptr().cast_mut());
                        } else {
                            StringAppend(dst, sText_EnigmaBerry.as_ptr().cast_mut());
                        }
                    } else {
                        CopyItemName(hword, dst);
                    }
                } else {
                    CopyItemName(hword, dst);
                }
                srcID += 3;
            }
            _ => {}
        }
    }
}
unsafe fn ChooseMoveUsedParticle(textBuff: *mut u8) {
    let mut counter: i32 = 0;
    let mut i: u32 = 0;
    while counter != MAX_MON_MOVES {
        if sGrammarMoveUsedTable[i] == 0 {
            counter += 1;
        }
        if sGrammarMoveUsedTable[{
            let t1 = i;
            i += 1;
            t1
        }] == (*gBattleMsgDataPtr).currentMove
        {
            break;
        }
    }
    if counter >= 0 {
        if counter <= 2 {
            StringCopy(textBuff, sText_SpaceIs.as_ptr().cast_mut());
        } else if counter <= MAX_MON_MOVES {
            StringCopy(textBuff, sText_ApostropheS.as_ptr().cast_mut());
        }
    }
}
unsafe fn ChooseTypeOfMoveUsedString(mut dst: *mut u8) {
    let mut counter: i32 = 0;
    let mut i: i32 = 0;
    while *dst != EOS {
        dst = dst.at(1);
    }
    while counter != MAX_MON_MOVES {
        if sGrammarMoveUsedTable[i] == MOVE_NONE {
            counter += 1;
        }
        if sGrammarMoveUsedTable[{
            let t1 = i;
            i += 1;
            t1
        }] == (*gBattleMsgDataPtr).currentMove
        {
            break;
        }
    }
    match counter {
        0 => {
            StringCopy(dst, sText_ExclamationMark.as_ptr().cast_mut());
        }
        1 => {
            StringCopy(dst, sText_ExclamationMark2.as_ptr().cast_mut());
        }
        2 => {
            StringCopy(dst, sText_ExclamationMark3.as_ptr().cast_mut());
        }
        3 => {
            StringCopy(dst, sText_ExclamationMark4.as_ptr().cast_mut());
        }
        4 => {
            StringCopy(dst, sText_ExclamationMark5.as_ptr().cast_mut());
        }
        _ => {}
    }
}
pub unsafe fn BattlePutTextOnWindow(text: *mut u8, mut windowId: u8) {
    let textInfo: *mut BattleWindowText = sBattleTextOnWindowsInfo[gBattleScripting.windowsType];
    let mut copyToVram: u32 = 0;
    let mut printerTemplate: TextPrinterTemplate = zeroed();
    let mut speed: u8 = 0;
    if windowId as i32 & B_WIN_COPYTOVRAM != 0 {
        windowId &= 127;
        copyToVram = FALSE as u32;
    } else {
        FillWindowPixelBuffer(windowId, (*textInfo.at(windowId)).fillValue);
        copyToVram = TRUE as u32;
    }
    printerTemplate.currentChar = text;
    printerTemplate.windowId = windowId;
    printerTemplate.fontId = (*textInfo.at(windowId)).fontId;
    printerTemplate.x = (*textInfo.at(windowId)).x;
    printerTemplate.y = (*textInfo.at(windowId)).y;
    printerTemplate.currentX = printerTemplate.x;
    printerTemplate.currentY = printerTemplate.y;
    printerTemplate.letterSpacing = (*textInfo.at(windowId)).letterSpacing;
    printerTemplate.lineSpacing = (*textInfo.at(windowId)).lineSpacing;
    printerTemplate.set_unk(0);
    printerTemplate.set_fgColor((*textInfo.at(windowId)).fgColor);
    printerTemplate.set_bgColor((*textInfo.at(windowId)).bgColor);
    printerTemplate.set_shadowColor((*textInfo.at(windowId)).shadowColor);
    if printerTemplate.x == 0xFF {
        let width: u32 =
            GetBattleWindowTemplatePixelWidth(gBattleScripting.windowsType as u32, windowId as u32);
        let alignX: i32 = GetStringCenterAlignXOffsetWithLetterSpacing(
            printerTemplate.fontId as i32,
            printerTemplate.currentChar,
            width as i32,
            printerTemplate.letterSpacing as i32,
        );
        printerTemplate.x = {
            printerTemplate.currentX = alignX as u8;
            printerTemplate.currentX
        };
    }
    if windowId == ARENA_WIN_JUDGMENT_TEXT {
        (*(&raw const crate::text::gTextFlags)
            .cast::<TextFlags>()
            .cast_mut())
        .set_useAlternateDownArrow(FALSE);
    } else {
        (*(&raw const crate::text::gTextFlags)
            .cast::<TextFlags>()
            .cast_mut())
        .set_useAlternateDownArrow(TRUE);
    }
    if gBattleTypeFlags & 0x1000002 != 0 {
        (*(&raw const crate::text::gTextFlags)
            .cast::<TextFlags>()
            .cast_mut())
        .set_autoScroll(TRUE);
    } else {
        (*(&raw const crate::text::gTextFlags)
            .cast::<TextFlags>()
            .cast_mut())
        .set_autoScroll(FALSE);
    }
    if windowId == B_WIN_MSG || windowId == ARENA_WIN_JUDGMENT_TEXT {
        if gBattleTypeFlags & 0x2000002 != 0 {
            speed = 1;
        } else if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            speed = sRecordedBattleTextSpeeds[GetTextSpeedInRecordedBattle()];
        } else {
            speed = GetPlayerTextSpeedDelay();
        }
        (*(&raw const crate::text::gTextFlags)
            .cast::<TextFlags>()
            .cast_mut())
        .set_canABSpeedUpPrint(1);
    } else {
        speed = (*textInfo.at(windowId)).speed;
        (*(&raw const crate::text::gTextFlags)
            .cast::<TextFlags>()
            .cast_mut())
        .set_canABSpeedUpPrint(0);
    }
    AddTextPrinter(&raw mut printerTemplate, speed, None);
    if copyToVram != 0 {
        PutWindowTilemap(windowId);
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
pub unsafe fn SetPPNumbersPaletteInMoveSelection() {
    let chooseMoveStruct: *mut ChooseMoveStruct =
        &raw mut gBattleBufferA[gActiveBattler][4] as *mut ChooseMoveStruct;
    let palPtr: *mut u16 = (*(&raw const crate::data::graphics::gPPTextPalette)
        .cast::<CArray<u16, 0>>())
    .as_ptr()
    .cast_mut();
    let var: u8 = GetCurrentPPToMaxPPState(
        (*chooseMoveStruct).currentPP[gMoveSelectionCursor[gActiveBattler]],
        (*chooseMoveStruct).maxPP[gMoveSelectionCursor[gActiveBattler]],
    );
    gPlttBufferUnfaded[92] = *palPtr.at(var as i32 * 2);
    gPlttBufferUnfaded[91] = *palPtr.at(var as i32 * 2 + 1);
    CpuSet(
        &raw mut gPlttBufferUnfaded[92] as *mut c_void,
        &raw mut gPlttBufferFaded[92] as *mut c_void,
        1,
    );
    CpuSet(
        &raw mut gPlttBufferUnfaded[91] as *mut c_void,
        &raw mut gPlttBufferFaded[91] as *mut c_void,
        1,
    );
}
pub unsafe fn GetCurrentPPToMaxPPState(currentPP: u8, maxPP: u8) -> u8 {
    if maxPP == currentPP {
        return 3;
    } else if maxPP <= 2 {
        if currentPP > 1 {
            return 3;
        } else {
            return 2 - currentPP;
        }
    } else if maxPP <= 7 {
        if currentPP > 2 {
            return 3;
        } else {
            return 2 - currentPP;
        }
    } else {
        if currentPP == 0 {
            return 2;
        }
        if currentPP as i32 <= maxPP as i32 / 4 {
            return 1;
        }
        if currentPP as i32 > maxPP as i32 / 2 {
            return 3;
        }
    }
    0
}
