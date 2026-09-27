//! Translated from `src/battle_message.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sText_Trainer1LoseText sText_PkmnGainedEXP sText_EmptyString4 sText_ABoosted sText_PkmnGrewToLv sText_PkmnLearnedMove sText_TryToLearnMove1 sText_TryToLearnMove2 sText_TryToLearnMove3 sText_PkmnForgotMove sText_StopLearningMove sText_DidNotLearnMove sText_UseNextPkmn sText_AttackMissed sText_PkmnProtectedItself sText_AvoidedDamage sText_PkmnMakesGroundMiss sText_PkmnAvoidedAttack sText_ItDoesntAffect sText_AttackerFainted sText_TargetFainted sText_PlayerGotMoney sText_PlayerWhiteout sText_PlayerWhiteout2 sText_PreventsEscape sText_CantEscape2 sText_AttackerCantEscape sText_HitXTimes sText_PkmnFellAsleep sText_PkmnMadeSleep sText_PkmnAlreadyAsleep sText_PkmnAlreadyAsleep2 sText_PkmnWasntAffected sText_PkmnWasPoisoned sText_PkmnPoisonedBy sText_PkmnHurtByPoison sText_PkmnAlreadyPoisoned sText_PkmnBadlyPoisoned sText_PkmnEnergyDrained sText_PkmnWasBurned sText_PkmnBurnedBy sText_PkmnHurtByBurn sText_PkmnAlreadyHasBurn sText_PkmnWasFrozen sText_PkmnFrozenBy sText_PkmnIsFrozen sText_PkmnWasDefrosted sText_PkmnWasDefrosted2 sText_PkmnWasDefrostedBy sText_PkmnWasParalyzed sText_PkmnWasParalyzedBy sText_PkmnIsParalyzed sText_PkmnIsAlreadyParalyzed sText_PkmnHealedParalysis sText_PkmnDreamEaten sText_StatsWontIncrease sText_StatsWontDecrease sText_TeamStoppedWorking sText_FoeStoppedWorking sText_PkmnIsConfused sText_PkmnHealedConfusion sText_PkmnWasConfused sText_PkmnAlreadyConfused sText_PkmnFellInLove sText_PkmnInLove sText_PkmnImmobilizedByLove sText_PkmnBlownAway sText_PkmnChangedType sText_PkmnFlinched sText_PkmnRegainedHealth sText_PkmnHPFull sText_PkmnRaisedSpDef sText_PkmnRaisedSpDefALittle sText_PkmnRaisedDef sText_PkmnRaisedDefALittle sText_PkmnCoveredByVeil sText_PkmnUsedSafeguard sText_PkmnSafeguardExpired sText_PkmnWentToSleep sText_PkmnSleptHealthy sText_PkmnWhippedWhirlwind sText_PkmnTookSunlight sText_PkmnLoweredHead sText_PkmnIsGlowing sText_PkmnFlewHigh sText_PkmnDugHole sText_PkmnHidUnderwater sText_PkmnSprangUp sText_PkmnSqueezedByBind sText_PkmnTrappedInVortex sText_PkmnTrappedBySandTomb sText_PkmnWrappedBy sText_PkmnClamped sText_PkmnHurtBy sText_PkmnFreedFrom sText_PkmnCrashed gText_PkmnShroudedInMist sText_PkmnProtectedByMist gText_PkmnGettingPumped sText_PkmnHitWithRecoil sText_PkmnProtectedItself2 sText_PkmnBuffetedBySandstorm sText_PkmnPeltedByHail sText_PkmnsXWoreOff sText_PkmnSeeded sText_PkmnEvadedAttack sText_PkmnSappedByLeechSeed sText_PkmnFastAsleep sText_PkmnWokeUp sText_PkmnUproarKeptAwake sText_PkmnWokeUpInUproar sText_PkmnCausedUproar sText_PkmnMakingUproar sText_PkmnCalmedDown sText_PkmnCantSleepInUproar sText_PkmnStockpiled sText_PkmnCantStockpile sText_PkmnCantSleepInUproar2 sText_UproarKeptPkmnAwake sText_PkmnStayedAwakeUsing sText_PkmnStoringEnergy sText_PkmnUnleashedEnergy sText_PkmnFatigueConfusion sText_PlayerPickedUpMoney sText_PkmnUnaffected sText_PkmnTransformedInto sText_PkmnMadeSubstitute sText_PkmnHasSubstitute sText_SubstituteDamaged sText_PkmnSubstituteFaded sText_PkmnMustRecharge sText_PkmnRageBuilding sText_PkmnMoveWasDisabled sText_PkmnMoveDisabledNoMore sText_PkmnGotEncore sText_PkmnEncoreEnded sText_PkmnTookAim sText_PkmnSketchedMove sText_PkmnTryingToTakeFoe sText_PkmnTookFoe sText_PkmnReducedPP sText_PkmnStoleItem sText_TargetCantEscapeNow sText_PkmnFellIntoNightmare sText_PkmnLockedInNightmare sText_PkmnLaidCurse sText_PkmnAfflictedByCurse sText_SpikesScattered sText_PkmnHurtBySpikes sText_PkmnIdentified sText_PkmnPerishCountFell sText_PkmnBracedItself sText_PkmnEnduredHit sText_MagnitudeStrength sText_PkmnCutHPMaxedAttack sText_PkmnCopiedStatChanges sText_PkmnGotFree sText_PkmnShedLeechSeed sText_PkmnBlewAwaySpikes sText_PkmnFledFromBattle sText_PkmnForesawAttack sText_PkmnTookAttack sText_PkmnChoseXAsDestiny sText_PkmnAttack sText_PkmnCenterAttention sText_PkmnChargingPower sText_NaturePowerTurnedInto sText_PkmnStatusNormal sText_PkmnSubjectedToTorment sText_PkmnTighteningFocus sText_PkmnFellForTaunt sText_PkmnReadyToHelp sText_PkmnSwitchedItems sText_PkmnObtainedX sText_PkmnObtainedX2 sText_PkmnObtainedXYObtainedZ sText_PkmnCopiedFoe sText_PkmnMadeWish sText_PkmnWishCameTrue sText_PkmnPlantedRoots sText_PkmnAbsorbedNutrients sText_PkmnAnchoredItself sText_PkmnWasMadeDrowsy sText_PkmnKnockedOff sText_PkmnSwappedAbilities sText_PkmnSealedOpponentMove sText_PkmnWantsGrudge sText_PkmnLostPPGrudge sText_PkmnShroudedItself sText_PkmnMoveBounced sText_PkmnWaitsForTarget sText_PkmnSnatchedMove sText_ElectricityWeakened sText_FireWeakened sText_XFoundOneY sText_SoothingAroma sText_ItemsCantBeUsedNow sText_ForXCommaYZ sText_PkmnUsedXToGetPumped sText_PkmnLostFocus sText_PkmnWasDraggedOut sText_TheWallShattered sText_ButNoEffect sText_PkmnHasNoMovesLeft sText_PkmnMoveIsDisabled sText_PkmnCantUseMoveTorment sText_PkmnCantUseMoveTaunt sText_PkmnCantUseMoveSealed sText_PkmnMadeItRain sText_PkmnRaisedSpeed sText_PkmnProtectedBy sText_PkmnPreventsUsage sText_PkmnRestoredHPUsing sText_PkmnsXMadeYUseless sText_PkmnChangedTypeWith sText_PkmnPreventsParalysisWith sText_PkmnPreventsRomanceWith sText_PkmnPreventsPoisoningWith sText_PkmnPreventsConfusionWith sText_PkmnRaisedFirePowerWith sText_PkmnAnchorsItselfWith sText_PkmnCutsAttackWith sText_PkmnPreventsStatLossWith sText_PkmnHurtsWith sText_PkmnTraced sText_PkmnsXPreventsBurns sText_PkmnsXBlocksY sText_PkmnsXBlocksY2 sText_PkmnsXRestoredHPALittle2 sText_PkmnsXWhippedUpSandstorm sText_PkmnsXIntensifiedSun sText_PkmnsXPreventsYLoss sText_PkmnsXInfatuatedY sText_PkmnsXMadeYIneffective sText_PkmnsXCuredYProblem sText_ItSuckedLiquidOoze sText_PkmnTransformed sText_PkmnsXTookAttack gText_PkmnsXPreventsSwitching sText_PreventedFromWorking sText_PkmnsXMadeItIneffective sText_PkmnsXPreventsFlinching sText_PkmnsXPreventsYsZ sText_PkmnsXCuredItsYProblem sText_PkmnsXHadNoEffectOnY sText_StatSharply gText_StatRose sText_StatHarshly sText_StatFell sText_AttackersStatRose gText_DefendersStatRose sText_UsingItemTheStatOfPkmnRose sText_AttackersStatFell sText_DefendersStatFell sText_StatsWontIncrease2 sText_StatsWontDecrease2 sText_CriticalHit sText_OneHitKO sText_123Poof sText_AndEllipsis sText_HMMovesCantBeForgotten sText_NotVeryEffective sText_SuperEffective sText_GotAwaySafely sText_PkmnFledUsingIts sText_PkmnFledUsing sText_WildPkmnFled sText_PlayerDefeatedLinkTrainer sText_TwoLinkTrainersDefeated sText_PlayerLostAgainstLinkTrainer sText_PlayerLostToTwo sText_PlayerBattledToDrawLinkTrainer sText_PlayerBattledToDrawVsTwo sText_WildFled sText_TwoWildFled sText_NoRunningFromTrainers sText_CantEscape sText_DontLeaveBirch sText_ButNothingHappened sText_ButItFailed sText_ItHurtConfusion sText_MirrorMoveFailed sText_StartedToRain sText_DownpourStarted sText_RainContinues sText_DownpourContinues sText_RainStopped sText_SandstormBrewed sText_SandstormRages sText_SandstormSubsided sText_SunlightGotBright sText_SunlightStrong sText_SunlightFaded sText_StartedHail sText_HailContinues sText_HailStopped sText_FailedToSpitUp sText_FailedToSwallow sText_WindBecameHeatWave sText_StatChangesGone sText_CoinsScattered sText_TooWeakForSubstitute sText_SharedPain sText_BellChimed sText_FaintInThree sText_NoPPLeft sText_ButNoPPLeft sText_PkmnIgnoresAsleep sText_PkmnIgnoredOrders sText_PkmnBeganToNap sText_PkmnLoafing sText_PkmnWontObey sText_PkmnTurnedAway sText_PkmnPretendNotNotice sText_EnemyAboutToSwitchPkmn sText_PkmnLearnedMove2 sText_PlayerDefeatedLinkTrainerTrainer1 sText_CreptCloser sText_CantGetCloser sText_PkmnWatchingCarefully sText_PkmnCuriousAboutX sText_PkmnEnthralledByX sText_PkmnIgnoredX sText_ThrewPokeblockAtPkmn sText_OutOfSafariBalls sText_OpponentMon1Appeared sText_WildPkmnAppeared sText_LegendaryPkmnAppeared sText_WildPkmnAppearedPause sText_TwoWildPkmnAppeared sText_Trainer1WantsToBattle sText_LinkTrainerWantsToBattle sText_TwoLinkTrainersWantToBattle sText_Trainer1SentOutPkmn sText_Trainer1SentOutTwoPkmn sText_Trainer1SentOutPkmn2 sText_LinkTrainerSentOutPkmn sText_LinkTrainerSentOutTwoPkmn sText_TwoLinkTrainersSentOutPkmn sText_LinkTrainerSentOutPkmn2 sText_LinkTrainerMultiSentOutPkmn sText_GoPkmn sText_GoTwoPkmn sText_GoPkmn2 sText_DoItPkmn sText_GoForItPkmn sText_YourFoesWeakGetEmPkmn sText_LinkPartnerSentOutPkmnGoPkmn sText_PkmnThatsEnough sText_PkmnComeBack sText_PkmnOkComeBack sText_PkmnGoodComeBack sText_Trainer1WithdrewPkmn sText_LinkTrainer1WithdrewPkmn sText_LinkTrainer2WithdrewPkmn sText_WildPkmnPrefix sText_FoePkmnPrefix sText_EmptyString8 sText_FoePkmnPrefix2 sText_AllyPkmnPrefix sText_FoePkmnPrefix3 sText_AllyPkmnPrefix2 sText_FoePkmnPrefix4 sText_AllyPkmnPrefix3 sText_AttackerUsedX sText_ExclamationMark sText_ExclamationMark2 sText_ExclamationMark3 sText_ExclamationMark4 sText_ExclamationMark5 sText_HP2 sText_Attack2 sText_Defense2 sText_Speed sText_SpAtk2 sText_SpDef2 sText_Accuracy sText_Evasiveness gStatNamesTable sText_PokeblockWasTooSpicy sText_PokeblockWasTooDry sText_PokeblockWasTooSweet sText_PokeblockWasTooBitter sText_PokeblockWasTooSour gPokeblockWasTooXStringTable sText_PlayerUsedItem sText_WallyUsedItem sText_Trainer1UsedItem sText_TrainerBlockedBall sText_DontBeAThief sText_ItDodgedBall sText_YouMissedPkmn sText_PkmnBrokeFree sText_ItAppearedCaught sText_AarghAlmostHadIt sText_ShootSoClose sText_GotchaPkmnCaughtPlayer sText_GotchaPkmnCaughtWally sText_GiveNicknameCaptured sText_PkmnSentToPC sText_Someones sText_Lanettes sText_PkmnDataAddedToDex sText_ItIsRaining sText_SandstormIsRaging sText_BoxIsFull sText_EnigmaBerry sText_BerrySuffix sText_PkmnsItemCuredParalysis sText_PkmnsItemCuredPoison sText_PkmnsItemHealedBurn sText_PkmnsItemDefrostedIt sText_PkmnsItemWokeIt sText_PkmnsItemSnappedOut sText_PkmnsItemCuredProblem sText_PkmnsItemNormalizedStatus sText_PkmnsItemRestoredHealth sText_PkmnsItemRestoredPP sText_PkmnsItemRestoredStatus sText_PkmnsItemRestoredHPALittle sText_ItemAllowsOnlyYMove sText_PkmnHungOnWithX gText_EmptyString3 sText_YouThrowABallNowRight sText_PkmnIncapableOfPower sText_GlintAppearsInEye sText_PkmnGettingIntoPosition sText_PkmnBeganGrowlingDeeply sText_PkmnEagerForMore sText_DefeatedOpponentByReferee sText_LostToOpponentByReferee sText_TiedOpponentByReferee sText_QuestionForfeitMatch sText_ForfeitedMatch sText_Trainer1WinText sText_Trainer2WinText sText_TwoInGameTrainersDefeated sText_Trainer2LoseText gBattleStringsTable gMissStringIds gNoEscapeStringIds gMoveWeatherChangeStringIds gSandStormHailContinuesStringIds gSandStormHailDmgStringIds gSandStormHailEndStringIds gRainContinuesStringIds gProtectLikeUsedStringIds gReflectLightScreenSafeguardStringIds gLeechSeedStringIds gRestUsedStringIds gUproarOverTurnStringIds gStockpileUsedStringIds gWokeUpStringIds gSwallowFailStringIds gUproarAwakeStringIds gStatUpStringIds gStatDownStringIds gFirstTurnOfTwoStringIds gWrappedStringIds gMistUsedStringIds gFocusEnergyUsedStringIds gTransformUsedStringIds gSubstituteUsedStringIds gGotPoisonedStringIds gGotParalyzedStringIds gFellAsleepStringIds gGotBurnedStringIds gGotFrozenStringIds gGotDefrostedStringIds gKOFailedStringIds gAttractUsedStringIds gAbsorbDrainStringIds gSportsUsedStringIds gPartyStatusHealStringIds gFutureMoveUsedStringIds gBallEscapeStringIds gWeatherStartsStringIds gInobedientStringIds gSafariGetNearStringIds gSafariPokeblockResultStringIds gTrainerItemCuredStatusStringIds gBerryEffectStringIds gBRNPreventionStringIds gPRLZPreventionStringIds gPSNPreventionStringIds gItemSwapStringIds gFlashFireStringIds gCaughtMonStringIds gTrappingMoves gText_PkmnIsEvolving gText_CongratsPkmnEvolved gText_PkmnStoppedEvolving gText_EllipsisQuestionMark gText_WhatWillPkmnDo gText_WhatWillPkmnDo2 gText_WhatWillWallyDo gText_LinkStandby gText_BattleMenu gText_SafariZoneMenu gText_MoveInterfacePP gText_MoveInterfaceType gText_MoveInterfacePPType gText_MoveInterfaceDynamicColors gText_WhichMoveToForget4 gText_BattleYesNoChoice gText_BattleSwitchWhich gText_BattleSwitchWhich2 gText_BattleSwitchWhich3 gText_BattleSwitchWhich4 gText_BattleSwitchWhich5 sText_HP sText_Attack sText_Defense sText_SpAtk sText_SpDef sStatNamesTable2 gText_SafariBalls gText_SafariBallLeft gText_Sleep gText_Poison gText_Burn gText_Paralysis gText_Ice gText_Confusion gText_Love gText_SpaceAndSpace gText_CommaSpace gText_Space2 gText_LineBreak gText_NewLine gText_Are gText_Are2 gText_BadEgg gText_BattleWallyName gText_Win gText_Loss gText_Draw sText_SpaceIs sText_ApostropheS sATypeMove_Table gText_BattleTourney sText_Round1 sText_Round2 sText_Semifinal sText_Final gRoundsStringTable gText_TheGreatNewHope gText_WillChampionshipDreamComeTrue gText_AFormerChampion gText_ThePreviousChampion gText_TheUnbeatenChampion gText_PlayerMon1Name gText_Vs gText_OpponentMon1Name gText_Mind gText_Skill gText_Body gText_Judgment sText_TwoTrainersSentPkmn sText_Trainer2SentOutPkmn sText_TwoTrainersWantToBattle sText_InGamePartnerSentOutZGoN sText_TwoInGameTrainersDefeated sText_Trainer2LoseText sText_PkmnIncapableOfPower sText_GlintAppearsInEye sText_PkmnGettingIntoPosition sText_PkmnBeganGrowlingDeeply sText_PkmnEagerForMore gBattlePalaceFlavorTextTable sText_RefIfNothingIsDecided sText_RefThatsIt sText_RefJudgeMind sText_RefJudgeSkill sText_RefJudgeBody sText_RefPlayerWon sText_RefOpponentWon sText_RefDraw sText_DefeatedOpponentByReferee sText_LostToOpponentByReferee sText_TiedOpponentByReferee sText_RefCommenceBattle gRefereeStringsTable sText_QuestionForfeitMatch sText_ForfeitedMatch sText_Trainer1WinText sText_Trainer2WinText sText_Trainer1Fled sText_PlayerLostAgainstTrainer1 sText_PlayerBattledToDrawTrainer1 gText_RecordBattleToPass gText_BattleRecordedOnPass sText_LinkTrainerWantsToBattlePause sText_TwoLinkTrainersWantToBattlePause sGrammarMoveUsedTable sText_EmptyStatus sTextOnWindowsInfo_Normal sTextOnWindowsInfo_Arena sBattleTextOnWindowsInfo sRecordedBattleTextSpeeds
#[allow(unused_imports)]
use crate::data::battle_message::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattlerAbilities: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattleMsgDataPtr: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gAbilityNames: u8;
    static mut gActiveBattler: u8;
    static mut gBattleBufferA: u8;
    static mut gBattleResources: u8;
    static mut gBattleScripting: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTextBuff1: u8;
    static mut gBattleTextBuff2: u8;
    static mut gBattleTextBuff3: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattlerAttacker: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerTarget: u8;
    static mut gDisplayedStringBattle: u8;
    static mut gEffectBattler: u8;
    static mut gEnemyParty: u8;
    static mut gEnigmaBerries: u8;
    static mut gLastUsedAbility: u8;
    static mut gLastUsedItem: u8;
    static mut gLinkPlayers: u8;
    static mut gMoveNames: u8;
    static mut gMoveSelectionCursor: u8;
    static mut gPPTextPalette: u8;
    static mut gPartnerTrainerId: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gPotentialItemEffectBattler: u8;
    static mut gRecordedBattleMultiplayerId: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gStatusConditionStringsTable: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTextFlags: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainerBattleOpponent_B: u8;
    static mut gTrainerClassNames: u8;
    static mut gTrainers: u8;
    static mut gTypeNames: u8;
    fn AddTextPrinter(a0: *mut u8, a1: u8, a2: Option<unsafe extern "C" fn(*mut u8, u16)>) -> u16;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyFrontierBrainTrainerName(a0: *mut u8);
    fn CopyFrontierTrainerText(a0: u8, a1: u16);
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CopyTrainerHillTrainerText(a0: u8, a1: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn GetBattleWindowTemplatePixelWidth(a0: u32, a1: u32) -> u32;
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerMultiplayerId(a0: u16) -> i32;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetEreaderTrainerClassId() -> u8;
    fn GetEreaderTrainerName(a0: *mut u8);
    fn GetFrontierBrainTrainerClass() -> u8;
    fn GetFrontierOpponentClass(a0: u16) -> u8;
    fn GetFrontierTrainerName(a0: *mut u8, a1: u16);
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetSecretBaseTrainerClass() -> u8;
    fn GetSpeciesName(a0: *mut u8, a1: u16);
    fn GetStringCenterAlignXOffsetWithLetterSpacing(a0: i32, a1: *mut u8, a2: i32, a3: i32) -> i32;
    fn GetTextSpeedInRecordedBattle() -> u8;
    fn GetTrainerALoseText() -> *mut u8;
    fn GetTrainerBLoseText() -> *mut u8;
    fn GetTrainerHillOpponentClass(a0: u16) -> u8;
    fn GetTrainerHillTrainerName(a0: *mut u8, a1: u16);
    fn GetUnionRoomTrainerClass() -> u16;
    fn PutWindowTilemap(a0: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferStringBattle(stringID: u16) {
    unsafe {
        let mut stringID = stringID;
        let mut i: i32 = 0i32;
        let mut stringPtr: *mut u8 = core::ptr::null_mut();
        ((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>()).write(
            ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
            .cast::<u8>())
            .wrapping_offset(4),
        );
        ((&raw mut gLastUsedItem).cast::<u16>()).write(
            ((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        ((&raw mut gLastUsedAbility).cast::<u8>()).write(
            ((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6))
            .read(),
        );
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23)).write(
            ((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7))
            .read(),
        );
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(82)).write(
            ((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8))
            .read(),
        );
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(177)).write(
            ((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(9))
            .read(),
        );
        ((&raw mut gPotentialItemEffectBattler).cast::<u8>()).write(
            ((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10))
            .read(),
        );
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(142)).write(
            ((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(11))
            .read(),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sBattlerAbilities).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        ((((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i
                    < (if 16i32 >= (if 14i32 >= 11i32 { 14i32 } else { 11i32 }) {
                        16i32
                    } else {
                        (if 14i32 >= 11i32 { 14i32 } else { 11i32 })
                    }))
                {
                    break 'l3;
                }
                'l4: {
                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(
                            (((((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16))
                            .cast::<u8>())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(
                            ((((((((&raw mut gBattleMsgDataPtr)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(16))
                            .cast::<u8>())
                            .wrapping_offset(16))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                    (((&raw mut gBattleTextBuff3).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(
                            ((((((((&raw mut gBattleMsgDataPtr)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(16))
                            .cast::<u8>())
                            .wrapping_offset(32))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        'l5: {
            let __sw1 = ((stringID) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32;
            if __sw1 == 0i32 {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8388608u32) != 0 {
                            stringPtr = ((&raw const sText_TwoTrainersWantToBattle)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>();
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                    & 16777216u32)
                                    != 0
                                {
                                    stringPtr =
                                        ((&raw const sText_TwoLinkTrainersWantToBattlePause)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>();
                                } else {
                                    stringPtr = ((&raw const sText_TwoLinkTrainersWantToBattle)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                }
                            } else {
                                if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                    as i32)
                                    == 3072i32
                                {
                                    stringPtr = ((&raw const sText_Trainer1WantsToBattle)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                } else {
                                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                        & 16777216u32)
                                        != 0
                                    {
                                        stringPtr =
                                            ((&raw const sText_LinkTrainerWantsToBattlePause)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>();
                                    } else {
                                        stringPtr = ((&raw const sText_LinkTrainerWantsToBattle)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>();
                                    }
                                }
                            }
                        }
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4194304u32) != 0 {
                            stringPtr = ((&raw const sText_TwoTrainersWantToBattle)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>();
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 32768u32) != 0
                            {
                                stringPtr = ((&raw const sText_TwoTrainersWantToBattle)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                            } else {
                                stringPtr = ((&raw const sText_Trainer1WantsToBattle)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                            }
                        }
                    }
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8192u32) != 0 {
                        stringPtr = ((&raw const sText_LegendaryPkmnAppeared)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>();
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
                            stringPtr = ((&raw const sText_TwoWildPkmnAppeared)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>();
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 512u32) != 0 {
                                stringPtr = ((&raw const sText_WildPkmnAppearedPause)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                            } else {
                                stringPtr =
                                    ((&raw const sText_WildPkmnAppeared).cast::<u8>().cast_mut())
                                        .cast::<u8>();
                            }
                        }
                    }
                }
                break 'l5;
            }
            if __sw1 == 1i32 {
                if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4194304u32) != 0 {
                            stringPtr = ((&raw const sText_InGamePartnerSentOutZGoN)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>();
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 32768u32) != 0
                            {
                                stringPtr = ((&raw const sText_GoTwoPkmn).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            } else {
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0
                                {
                                    stringPtr = ((&raw const sText_LinkPartnerSentOutPkmnGoPkmn)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                } else {
                                    stringPtr =
                                        ((&raw const sText_GoTwoPkmn).cast::<u8>().cast_mut())
                                            .cast::<u8>();
                                }
                            }
                        }
                    } else {
                        stringPtr =
                            ((&raw const sText_GoPkmn).cast::<u8>().cast_mut()).cast::<u8>();
                    }
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 32768u32) != 0 {
                            stringPtr = ((&raw const sText_TwoTrainersSentPkmn)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>();
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8388608u32)
                                != 0
                            {
                                stringPtr = ((&raw const sText_TwoTrainersSentPkmn)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                            } else {
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0
                                {
                                    stringPtr = ((&raw const sText_TwoLinkTrainersSentOutPkmn)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                } else {
                                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                        & 33554434u32)
                                        != 0
                                    {
                                        stringPtr = ((&raw const sText_LinkTrainerSentOutTwoPkmn)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>();
                                    } else {
                                        stringPtr = ((&raw const sText_Trainer1SentOutTwoPkmn)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>();
                                    }
                                }
                            }
                        }
                    } else {
                        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32)
                            != 0)
                        {
                            stringPtr = ((&raw const sText_Trainer1SentOutPkmn)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>();
                        } else {
                            if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                                == 3072i32
                            {
                                stringPtr = ((&raw const sText_Trainer1SentOutPkmn)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                            } else {
                                stringPtr = ((&raw const sText_LinkTrainerSentOutPkmn)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                            }
                        }
                    }
                }
                break 'l5;
            }
            if __sw1 == 2i32 {
                if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    if ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(177))
                        .read()) as i32)
                        == 0i32
                    {
                        stringPtr = ((&raw const sText_PkmnThatsEnough).cast::<u8>().cast_mut())
                            .cast::<u8>();
                    } else {
                        if (((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                            .wrapping_add(177))
                        .read()) as i32)
                            == 1i32)
                            || ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0)
                        {
                            stringPtr = ((&raw const sText_PkmnComeBack).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        } else {
                            if ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(177))
                            .read()) as i32)
                                == 2i32
                            {
                                stringPtr =
                                    ((&raw const sText_PkmnOkComeBack).cast::<u8>().cast_mut())
                                        .cast::<u8>();
                            } else {
                                stringPtr =
                                    ((&raw const sText_PkmnGoodComeBack).cast::<u8>().cast_mut())
                                        .cast::<u8>();
                            }
                        }
                    }
                } else {
                    if (((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                        == 2048i32)
                        || ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554432u32) != 0)
                    {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
                            stringPtr = ((&raw const sText_LinkTrainer2WithdrewPkmn)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>();
                        } else {
                            stringPtr = ((&raw const sText_LinkTrainer1WithdrewPkmn)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>();
                        }
                    } else {
                        stringPtr = ((&raw const sText_Trainer1WithdrewPkmn)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>();
                    }
                }
                break 'l5;
            }
            if __sw1 == 3i32 {
                if ((GetBattlerSide(
                    (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23)).read(),
                )) as i32)
                    == 0i32
                {
                    if (((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(177))
                        .read()) as i32)
                        == 0i32)
                        || ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0)
                    {
                        stringPtr =
                            ((&raw const sText_GoPkmn2).cast::<u8>().cast_mut()).cast::<u8>();
                    } else {
                        if ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                            .wrapping_add(177))
                        .read()) as i32)
                            == 1i32
                        {
                            stringPtr =
                                ((&raw const sText_DoItPkmn).cast::<u8>().cast_mut()).cast::<u8>();
                        } else {
                            if ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(177))
                            .read()) as i32)
                                == 2i32
                            {
                                stringPtr =
                                    ((&raw const sText_GoForItPkmn).cast::<u8>().cast_mut())
                                        .cast::<u8>();
                            } else {
                                stringPtr = ((&raw const sText_YourFoesWeakGetEmPkmn)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                            }
                        }
                    }
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8388608u32) != 0 {
                            if (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                .read()) as i32)
                                == 1i32
                            {
                                stringPtr = ((&raw const sText_Trainer1SentOutPkmn2)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                            } else {
                                stringPtr = ((&raw const sText_Trainer2SentOutPkmn)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                            }
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
                                stringPtr = ((&raw const sText_LinkTrainerMultiSentOutPkmn)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                            } else {
                                if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                    as i32)
                                    == 3072i32
                                {
                                    stringPtr = ((&raw const sText_Trainer1SentOutPkmn2)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                } else {
                                    stringPtr = ((&raw const sText_LinkTrainerSentOutPkmn2)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                }
                            }
                        }
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 32768u32) != 0 {
                            if (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                .read()) as i32)
                                == 1i32
                            {
                                stringPtr = ((&raw const sText_Trainer1SentOutPkmn2)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                            } else {
                                stringPtr = ((&raw const sText_Trainer2SentOutPkmn)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                            }
                        } else {
                            stringPtr = ((&raw const sText_Trainer1SentOutPkmn2)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>();
                        }
                    }
                }
                break 'l5;
            }
            if __sw1 == 4i32 {
                ChooseMoveUsedParticle((&raw mut gBattleTextBuff1).cast::<u8>());
                if ((((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>())
                .read()) as i32)
                    >= 355i32
                {
                    StringCopy(
                        (&raw mut gBattleTextBuff2).cast::<u8>(),
                        ((((&raw const sATypeMove_Table).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(142))
                                .read()) as i32) as isize
                                    * 17,
                            ))
                        .cast::<u8>(),
                    );
                } else {
                    StringCopy(
                        (&raw mut gBattleTextBuff2).cast::<u8>(),
                        (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 13,
                        ))
                        .cast::<u8>(),
                    );
                }
                ChooseTypeOfMoveUsedString((&raw mut gBattleTextBuff2).cast::<u8>());
                stringPtr = ((&raw const sText_AttackerUsedX).cast::<u8>().cast_mut()).cast::<u8>();
                break 'l5;
            }
            if __sw1 == 5i32 {
                if (((((&raw mut gBattleTextBuff1).cast::<u8>()).read()) as i32) & 128i32) != 0 {
                    let __p2 = (&raw mut gBattleTextBuff1).cast::<u8>();
                    (__p2).write((((((__p2).read()) as i32) & (-129i32)) as u8));
                    if (((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32)
                        == 1i32)
                        && (((((&raw mut gBattleTextBuff1).cast::<u8>()).read()) as i32) != 3i32)
                    {
                        let __p3 = (&raw mut gBattleTextBuff1).cast::<u8>();
                        (__p3).write((((((__p3).read()) as i32) ^ 3i32) as u8));
                    }
                    if (((((&raw mut gBattleTextBuff1).cast::<u8>()).read()) as i32) == 2i32)
                        || (((((&raw mut gBattleTextBuff1).cast::<u8>()).read()) as i32) == 3i32)
                    {
                        stringPtr =
                            ((&raw const sText_GotAwaySafely).cast::<u8>().cast_mut()).cast::<u8>();
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
                            stringPtr = ((&raw const sText_TwoWildFled).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        } else {
                            stringPtr =
                                ((&raw const sText_WildFled).cast::<u8>().cast_mut()).cast::<u8>();
                        }
                    }
                } else {
                    if (((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32)
                        == 1i32)
                        && (((((&raw mut gBattleTextBuff1).cast::<u8>()).read()) as i32) != 3i32)
                    {
                        let __p4 = (&raw mut gBattleTextBuff1).cast::<u8>();
                        (__p4).write((((((__p4).read()) as i32) ^ 3i32) as u8));
                    }
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
                        'l6: {
                            let __sw5 =
                                ((((&raw mut gBattleTextBuff1).cast::<u8>()).read()) as i32);
                            if __sw5 == 1i32 {
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8388608u32)
                                    != 0
                                {
                                    stringPtr = ((&raw const sText_TwoInGameTrainersDefeated)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                } else {
                                    stringPtr = ((&raw const sText_TwoLinkTrainersDefeated)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                }
                                break 'l6;
                            }
                            if __sw5 == 2i32 {
                                stringPtr =
                                    ((&raw const sText_PlayerLostToTwo).cast::<u8>().cast_mut())
                                        .cast::<u8>();
                                break 'l6;
                            }
                            if __sw5 == 3i32 {
                                stringPtr = ((&raw const sText_PlayerBattledToDrawVsTwo)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>();
                                break 'l6;
                            }
                        }
                    } else {
                        if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                            == 3072i32
                        {
                            'l7: {
                                let __sw6 =
                                    ((((&raw mut gBattleTextBuff1).cast::<u8>()).read()) as i32);
                                if __sw6 == 1i32 {
                                    stringPtr =
                                        ((&raw const sText_PlayerDefeatedLinkTrainerTrainer1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>();
                                    break 'l7;
                                }
                                if __sw6 == 2i32 {
                                    stringPtr = ((&raw const sText_PlayerLostAgainstTrainer1)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                    break 'l7;
                                }
                                if __sw6 == 3i32 {
                                    stringPtr = ((&raw const sText_PlayerBattledToDrawTrainer1)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                    break 'l7;
                                }
                            }
                        } else {
                            'l8: {
                                let __sw7 =
                                    ((((&raw mut gBattleTextBuff1).cast::<u8>()).read()) as i32);
                                if __sw7 == 1i32 {
                                    stringPtr = ((&raw const sText_PlayerDefeatedLinkTrainer)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                    break 'l8;
                                }
                                if __sw7 == 2i32 {
                                    stringPtr = ((&raw const sText_PlayerLostAgainstLinkTrainer)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                    break 'l8;
                                }
                                if __sw7 == 3i32 {
                                    stringPtr = ((&raw const sText_PlayerBattledToDrawLinkTrainer)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>();
                                    break 'l8;
                                }
                            }
                        }
                    }
                }
                break 'l5;
            }
            if !__matched {
                if ((stringID) as i32) >= 381i32 {
                    ((&raw mut gDisplayedStringBattle).cast::<u8>()).write(255u8);
                    return;
                } else {
                    stringPtr = ((((&raw const gBattleStringsTable)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset((((stringID) as i32).wrapping_sub(12i32)) as isize))
                    .read();
                }
                break 'l5;
            }
        }
        BattleStringExpandPlaceholdersToDisplayedString(stringPtr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleStringExpandPlaceholdersToDisplayedString(src: *mut u8) -> u32 {
    unsafe {
        let mut src = src;
        return BattleStringExpandPlaceholders(src, (&raw mut gDisplayedStringBattle).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn TryGetStatusString(src: *mut u8) -> *mut u8 {
    unsafe {
        let mut src = src;
        let mut i: u32 = 0u32;
        let mut status = crate::ffi::Align4([0u8; 8]);
        let mut chars1: u32 = 0u32;
        let mut chars2: u32 = 0u32;
        let mut statusPtr: *mut u8 = core::ptr::null_mut();
        crate::c::memcpy(
            (&raw mut status).cast::<u8>(),
            ((&raw const sText_EmptyStatus).cast::<u8>().cast_mut()).cast::<u8>(),
            (if crate::c::div_u32(8u32, 1u32) < crate::c::div_u32(8u32, 1u32) {
                crate::c::div_u32(8u32, 1u32)
            } else {
                crate::c::div_u32(8u32, 1u32)
            }),
        );
        statusPtr = (&raw mut status).cast::<u8>();
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(8u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((src).read()) as i32) == 255i32 {
                        break 'l1;
                    }
                    (statusPtr).write((src).read());
                    src = (src).wrapping_offset(1);
                    statusPtr = (statusPtr).wrapping_offset(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        chars1 = (((&raw mut status).cast::<u8>()).cast::<u32>()).read();
        chars2 = ((((&raw mut status).cast::<u8>()).wrapping_offset(4)).cast::<u32>()).read();
        {
            i = 0u32;
            'l3: loop {
                if !(i < crate::c::div_u32(56u32, 8u32)) {
                    break 'l3;
                }
                'l4: {
                    if (chars1
                        == ((((((&raw mut gStatusConditionStringsTable).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                        .cast::<*mut u8>())
                        .read())
                        .cast::<u32>())
                        .read())
                        && (chars2
                            == (((((((&raw mut gStatusConditionStringsTable).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 8))
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(4))
                            .cast::<u32>())
                            .read())
                    {
                        return (((((&raw mut gStatusConditionStringsTable).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                        .cast::<*mut u8>())
                        .wrapping_offset(1))
                        .read();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return core::ptr::null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleStringExpandPlaceholders(src: *mut u8, dst: *mut u8) -> u32 {
    unsafe {
        let mut src = src;
        let mut dst = dst;
        let mut dstID: u32 = 0u32;
        let mut toCpy: *mut u8 = core::ptr::null_mut();
        let mut text = crate::ffi::Align4([0u8; 32]);
        let mut multiplayerId: u8 = 0u8;
        let mut i: i32 = 0i32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554432u32) != 0 {
            multiplayerId = ((&raw mut gRecordedBattleMultiplayerId).cast::<u8>()).read();
        } else {
            multiplayerId = GetMultiplayerId();
        }
        'l1: loop {
            if !((((src).read()) as i32) != 255i32) {
                break 'l1;
            }
            if (((src).read()) as i32) == 253i32 {
                src = (src).wrapping_offset(1);
                'l2: {
                    let __sw1 = (((src).read()) as i32);
                    if __sw1 == 0i32 {
                        if ((((&raw mut gBattleTextBuff1).cast::<u8>()).read()) as i32) == 253i32 {
                            ExpandBattleTextBuffPlaceholders(
                                (&raw mut gBattleTextBuff1).cast::<u8>(),
                                (&raw mut gStringVar1).cast::<u8>(),
                            );
                            toCpy = (&raw mut gStringVar1).cast::<u8>();
                        } else {
                            toCpy = TryGetStatusString((&raw mut gBattleTextBuff1).cast::<u8>());
                            if ((toCpy) as usize) == 0usize {
                                toCpy = (&raw mut gBattleTextBuff1).cast::<u8>();
                            }
                        }
                        break 'l2;
                    }
                    if __sw1 == 1i32 {
                        if ((((&raw mut gBattleTextBuff2).cast::<u8>()).read()) as i32) == 253i32 {
                            ExpandBattleTextBuffPlaceholders(
                                (&raw mut gBattleTextBuff2).cast::<u8>(),
                                (&raw mut gStringVar2).cast::<u8>(),
                            );
                            toCpy = (&raw mut gStringVar2).cast::<u8>();
                        } else {
                            toCpy = (&raw mut gBattleTextBuff2).cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 52i32 {
                        if ((((&raw mut gBattleTextBuff3).cast::<u8>()).read()) as i32) == 253i32 {
                            ExpandBattleTextBuffPlaceholders(
                                (&raw mut gBattleTextBuff3).cast::<u8>(),
                                (&raw mut gStringVar3).cast::<u8>(),
                            );
                            toCpy = (&raw mut gStringVar3).cast::<u8>();
                        } else {
                            toCpy = (&raw mut gBattleTextBuff3).cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 2i32 {
                        toCpy = (&raw mut gStringVar1).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 3i32 {
                        toCpy = (&raw mut gStringVar2).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 4i32 {
                        toCpy = (&raw mut gStringVar3).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 5i32 {
                        GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((GetBattlerAtPosition(0u8)) as i32) as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            2i32,
                            (&raw mut text).cast::<u8>(),
                        );
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 6i32 {
                        GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((GetBattlerAtPosition(1u8)) as i32) as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            2i32,
                            (&raw mut text).cast::<u8>(),
                        );
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 7i32 {
                        GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((GetBattlerAtPosition(2u8)) as i32) as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            2i32,
                            (&raw mut text).cast::<u8>(),
                        );
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 8i32 {
                        GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((GetBattlerAtPosition(3u8)) as i32) as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            2i32,
                            (&raw mut text).cast::<u8>(),
                        );
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 9i32 {
                        GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset(
                                                ((multiplayerId) as i32) as isize * 28,
                                            ))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            2i32,
                            (&raw mut text).cast::<u8>(),
                        );
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 10i32 {
                        GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        (((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset(
                                                ((multiplayerId) as i32) as isize * 28,
                                            ))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            ^ 1i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            2i32,
                            (&raw mut text).cast::<u8>(),
                        );
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 11i32 {
                        GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        (((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset(
                                                ((multiplayerId) as i32) as isize * 28,
                                            ))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            ^ 2i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            2i32,
                            (&raw mut text).cast::<u8>(),
                        );
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 12i32 {
                        GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        (((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset(
                                                ((multiplayerId) as i32) as isize * 28,
                                            ))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            ^ 3i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            2i32,
                            (&raw mut text).cast::<u8>(),
                        );
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 13i32 {
                        if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                            as i32)
                            != 0i32
                        {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                                toCpy = ((&raw const sText_FoePkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            } else {
                                toCpy = ((&raw const sText_WildPkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            }
                            'l3: loop {
                                if !((((toCpy).read()) as i32) != 255i32) {
                                    break 'l3;
                                }
                                ((dst).wrapping_offset(((dstID) as i32) as isize))
                                    .write((toCpy).read());
                                dstID = (dstID).wrapping_add(1);
                                toCpy = (toCpy).wrapping_offset(1);
                            }
                            GetMonData3(
                                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((GetBattlerAtPosition(
                                            ((((GetBattlerPosition(
                                                ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                                            ))
                                                as i32)
                                                & 1i32)
                                                as u8),
                                        )) as i32) as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        } else {
                            GetMonData3(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((GetBattlerAtPosition(
                                            ((((GetBattlerPosition(
                                                ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                                            ))
                                                as i32)
                                                & 1i32)
                                                as u8),
                                        )) as i32) as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        }
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 14i32 {
                        if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                            as i32)
                            == 0i32
                        {
                            GetMonData3(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        (((GetBattlerAtPosition(
                                            ((((GetBattlerPosition(
                                                ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                                            ))
                                                as i32)
                                                & 1i32)
                                                as u8),
                                        )) as i32)
                                            .wrapping_add(2i32))
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        } else {
                            GetMonData3(
                                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        (((GetBattlerAtPosition(
                                            ((((GetBattlerPosition(
                                                ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                                            ))
                                                as i32)
                                                & 1i32)
                                                as u8),
                                        )) as i32)
                                            .wrapping_add(2i32))
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        }
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 15i32 {
                        if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                            as i32)
                            != 0i32
                        {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                                toCpy = ((&raw const sText_FoePkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            } else {
                                toCpy = ((&raw const sText_WildPkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            }
                            'l4: loop {
                                if !((((toCpy).read()) as i32) != 255i32) {
                                    break 'l4;
                                }
                                ((dst).wrapping_offset(((dstID) as i32) as isize))
                                    .write((toCpy).read());
                                dstID = (dstID).wrapping_add(1);
                                toCpy = (toCpy).wrapping_offset(1);
                            }
                            GetMonData3(
                                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        } else {
                            GetMonData3(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        }
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 16i32 {
                        if ((GetBattlerSide(((&raw mut gBattlerTarget).cast::<u8>()).read()))
                            as i32)
                            != 0i32
                        {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                                toCpy = ((&raw const sText_FoePkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            } else {
                                toCpy = ((&raw const sText_WildPkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            }
                            'l5: loop {
                                if !((((toCpy).read()) as i32) != 255i32) {
                                    break 'l5;
                                }
                                ((dst).wrapping_offset(((dstID) as i32) as isize))
                                    .write((toCpy).read());
                                dstID = (dstID).wrapping_add(1);
                                toCpy = (toCpy).wrapping_offset(1);
                            }
                            GetMonData3(
                                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        } else {
                            GetMonData3(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        }
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 17i32 {
                        if ((GetBattlerSide(((&raw mut gEffectBattler).cast::<u8>()).read()))
                            as i32)
                            != 0i32
                        {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                                toCpy = ((&raw const sText_FoePkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            } else {
                                toCpy = ((&raw const sText_WildPkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            }
                            'l6: loop {
                                if !((((toCpy).read()) as i32) != 255i32) {
                                    break 'l6;
                                }
                                ((dst).wrapping_offset(((dstID) as i32) as isize))
                                    .write((toCpy).read());
                                dstID = (dstID).wrapping_add(1);
                                toCpy = (toCpy).wrapping_offset(1);
                            }
                            GetMonData3(
                                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        } else {
                            GetMonData3(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        }
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 18i32 {
                        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read()))
                            as i32)
                            != 0i32
                        {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                                toCpy = ((&raw const sText_FoePkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            } else {
                                toCpy = ((&raw const sText_WildPkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            }
                            'l7: loop {
                                if !((((toCpy).read()) as i32) != 255i32) {
                                    break 'l7;
                                }
                                ((dst).wrapping_offset(((dstID) as i32) as isize))
                                    .write((toCpy).read());
                                dstID = (dstID).wrapping_add(1);
                                toCpy = (toCpy).wrapping_offset(1);
                            }
                            GetMonData3(
                                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        } else {
                            GetMonData3(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        }
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 19i32 {
                        if ((GetBattlerSide(
                            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23)).read(),
                        )) as i32)
                            != 0i32
                        {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                                toCpy = ((&raw const sText_FoePkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            } else {
                                toCpy = ((&raw const sText_WildPkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            }
                            'l8: loop {
                                if !((((toCpy).read()) as i32) != 255i32) {
                                    break 'l8;
                                }
                                ((dst).wrapping_offset(((dstID) as i32) as isize))
                                    .write((toCpy).read());
                                dstID = (dstID).wrapping_add(1);
                                toCpy = (toCpy).wrapping_offset(1);
                            }
                            GetMonData3(
                                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        (((((&raw mut gBattleScripting).cast::<u8>())
                                            .wrapping_add(23))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        } else {
                            GetMonData3(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        (((((&raw mut gBattleScripting).cast::<u8>())
                                            .wrapping_add(23))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        }
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 20i32 {
                        if ((((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<u16>())
                        .read()) as i32)
                            >= 355i32
                        {
                            toCpy = ((((&raw const sATypeMove_Table).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(142))
                                .read()) as i32) as isize
                                    * 17,
                            ))
                            .cast::<u8>();
                        } else {
                            toCpy = (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 13,
                            ))
                            .cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 21i32 {
                        if ((((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                            >= 355i32
                        {
                            toCpy = ((((&raw const sATypeMove_Table).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(142))
                                .read()) as i32) as isize
                                    * 17,
                            ))
                            .cast::<u8>();
                        } else {
                            toCpy = (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2)
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 13,
                            ))
                            .cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 22i32 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0 {
                            if ((((&raw mut gLastUsedItem).cast::<u16>()).read()) as i32) == 175i32
                            {
                                if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32)
                                    != 0)
                                {
                                    if (((((((&raw mut gBattleScripting).cast::<u8>())
                                        .wrapping_add(37))
                                    .read()) as i32)
                                        != 0i32)
                                        && ((((((&raw mut gPotentialItemEffectBattler)
                                            .cast::<u8>())
                                        .read())
                                            as i32)
                                            & 1i32)
                                            != 0))
                                        || (((((((&raw mut gBattleScripting).cast::<u8>())
                                            .wrapping_add(37))
                                        .read())
                                            as i32)
                                            == 0i32)
                                            && (!((((((&raw mut gPotentialItemEffectBattler)
                                                .cast::<u8>())
                                            .read())
                                                as i32)
                                                & 1i32)
                                                != 0)))
                                    {
                                        StringCopy(
                                            (&raw mut text).cast::<u8>(),
                                            (((&raw mut gEnigmaBerries).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gPotentialItemEffectBattler)
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 28,
                                                ))
                                            .cast::<u8>(),
                                        );
                                        StringAppend(
                                            (&raw mut text).cast::<u8>(),
                                            ((&raw const sText_BerrySuffix)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>(),
                                        );
                                        toCpy = (&raw mut text).cast::<u8>();
                                    } else {
                                        toCpy = ((&raw const sText_EnigmaBerry)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>();
                                    }
                                } else {
                                    if ((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset(
                                            (((((&raw mut gBattleScripting).cast::<u8>())
                                                .wrapping_add(37))
                                            .read())
                                                as i32)
                                                as isize
                                                * 28,
                                        ))
                                    .wrapping_add(24)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        == ((((&raw mut gPotentialItemEffectBattler).cast::<u8>())
                                            .read())
                                            as i32)
                                    {
                                        StringCopy(
                                            (&raw mut text).cast::<u8>(),
                                            (((&raw mut gEnigmaBerries).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((&raw mut gPotentialItemEffectBattler)
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 28,
                                                ))
                                            .cast::<u8>(),
                                        );
                                        StringAppend(
                                            (&raw mut text).cast::<u8>(),
                                            ((&raw const sText_BerrySuffix)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>(),
                                        );
                                        toCpy = (&raw mut text).cast::<u8>();
                                    } else {
                                        toCpy = ((&raw const sText_EnigmaBerry)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>();
                                    }
                                }
                            } else {
                                CopyItemName(
                                    ((&raw mut gLastUsedItem).cast::<u16>()).read(),
                                    (&raw mut text).cast::<u8>(),
                                );
                                toCpy = (&raw mut text).cast::<u8>();
                            }
                        } else {
                            CopyItemName(
                                ((&raw mut gLastUsedItem).cast::<u16>()).read(),
                                (&raw mut text).cast::<u8>(),
                            );
                            toCpy = (&raw mut text).cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 23i32 {
                        toCpy = (((&raw mut gAbilityNames).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gLastUsedAbility).cast::<u8>()).read()) as i32) as isize
                                * 13,
                        ))
                        .cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 24i32 {
                        toCpy = (((&raw mut gAbilityNames).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sBattlerAbilities).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerAttacker).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32) as isize
                                * 13,
                        ))
                        .cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 25i32 {
                        toCpy = (((&raw mut gAbilityNames).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sBattlerAbilities).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32) as isize
                                * 13,
                        ))
                        .cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 26i32 {
                        toCpy = (((&raw mut gAbilityNames).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sBattlerAbilities).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                        .read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32) as isize
                                * 13,
                        ))
                        .cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 27i32 {
                        toCpy = (((&raw mut gAbilityNames).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sBattlerAbilities).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    ((((&raw mut gEffectBattler).cast::<u8>()).read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32) as isize
                                * 13,
                        ))
                        .cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 28i32 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 134217728u32) != 0
                        {
                            toCpy = (((&raw mut gTrainerClassNames).cast::<u8>()).wrapping_offset(
                                ((GetSecretBaseTrainerClass()) as i32) as isize * 13,
                            ))
                            .cast::<u8>();
                        } else {
                            if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                                == 3072i32
                            {
                                toCpy = (((&raw mut gTrainerClassNames).cast::<u8>())
                                    .wrapping_offset(
                                        ((GetUnionRoomTrainerClass()) as i32) as isize * 13,
                                    ))
                                .cast::<u8>();
                            } else {
                                if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                    as i32)
                                    == 1022i32
                                {
                                    toCpy = (((&raw mut gTrainerClassNames).cast::<u8>())
                                        .wrapping_offset(
                                            ((GetFrontierBrainTrainerClass()) as i32) as isize * 13,
                                        ))
                                    .cast::<u8>();
                                } else {
                                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                        & 4129024u32)
                                        != 0
                                    {
                                        toCpy = (((&raw mut gTrainerClassNames).cast::<u8>())
                                            .wrapping_offset(
                                                ((GetFrontierOpponentClass(
                                                    ((&raw mut gTrainerBattleOpponent_A)
                                                        .cast::<u16>())
                                                    .read(),
                                                ))
                                                    as i32)
                                                    as isize
                                                    * 13,
                                            ))
                                        .cast::<u8>();
                                    } else {
                                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                            & 67108864u32)
                                            != 0
                                        {
                                            toCpy = (((&raw mut gTrainerClassNames).cast::<u8>())
                                                .wrapping_offset(
                                                    ((GetTrainerHillOpponentClass(
                                                        ((&raw mut gTrainerBattleOpponent_A)
                                                            .cast::<u16>())
                                                        .read(),
                                                    ))
                                                        as i32)
                                                        as isize
                                                        * 13,
                                                ))
                                            .cast::<u8>();
                                        } else {
                                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                                & 2048u32)
                                                != 0
                                            {
                                                toCpy = (((&raw mut gTrainerClassNames)
                                                    .cast::<u8>())
                                                .wrapping_offset(
                                                    ((GetEreaderTrainerClassId()) as i32) as isize
                                                        * 13,
                                                ))
                                                .cast::<u8>();
                                            } else {
                                                toCpy = (((&raw mut gTrainerClassNames)
                                                    .cast::<u8>())
                                                .wrapping_offset(
                                                    ((((((&raw mut gTrainers).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((&raw mut gTrainerBattleOpponent_A)
                                                                .cast::<u16>())
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 40,
                                                        ))
                                                    .wrapping_add(1))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 13,
                                                ))
                                                .cast::<u8>();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        break 'l2;
                    }
                    if __sw1 == 29i32 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 134217728u32) != 0
                        {
                            {
                                i = 0i32;
                                'l9: loop {
                                    if !(i < ((crate::c::div_u32(7u32, 1u32)) as i32)) {
                                        break 'l9;
                                    }
                                    'l10: {
                                        (((&raw mut text).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .write(
                                            ((((((((&raw mut gBattleResources)
                                                .cast::<*mut u8>())
                                            .read())
                                            .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(2))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                            .read(),
                                        );
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                            (((&raw mut text).cast::<u8>()).wrapping_offset((i) as isize))
                                .write(255u8);
                            ConvertInternationalString(
                                (&raw mut text).cast::<u8>(),
                                ((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(13))
                                .read(),
                            );
                            toCpy = (&raw mut text).cast::<u8>();
                        } else {
                            if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                                == 3072i32
                            {
                                toCpy = ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                                    (((multiplayerId) as i32) ^ 1i32) as isize * 28,
                                ))
                                .wrapping_add(8))
                                .cast::<u8>();
                            } else {
                                if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read())
                                    as i32)
                                    == 1022i32
                                {
                                    CopyFrontierBrainTrainerName((&raw mut text).cast::<u8>());
                                    toCpy = (&raw mut text).cast::<u8>();
                                } else {
                                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                        & 4129024u32)
                                        != 0
                                    {
                                        GetFrontierTrainerName(
                                            (&raw mut text).cast::<u8>(),
                                            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>())
                                                .read(),
                                        );
                                        toCpy = (&raw mut text).cast::<u8>();
                                    } else {
                                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                            & 67108864u32)
                                            != 0
                                        {
                                            GetTrainerHillTrainerName(
                                                (&raw mut text).cast::<u8>(),
                                                ((&raw mut gTrainerBattleOpponent_A).cast::<u16>())
                                                    .read(),
                                            );
                                            toCpy = (&raw mut text).cast::<u8>();
                                        } else {
                                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                                & 2048u32)
                                                != 0
                                            {
                                                GetEreaderTrainerName((&raw mut text).cast::<u8>());
                                                toCpy = (&raw mut text).cast::<u8>();
                                            } else {
                                                toCpy = ((((&raw mut gTrainers).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((&raw mut gTrainerBattleOpponent_A)
                                                            .cast::<u16>())
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 40,
                                                    ))
                                                .wrapping_add(4))
                                                .cast::<u8>();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        break 'l2;
                    }
                    if __sw1 == 30i32 {
                        toCpy = ((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset(((multiplayerId) as i32) as isize * 28))
                        .wrapping_add(8))
                        .cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 31i32 {
                        toCpy = ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                            (GetBattlerMultiplayerId(
                                ((((((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset(((multiplayerId) as i32) as isize * 28))
                                .wrapping_add(24)
                                .cast::<u16>())
                                .read()) as i32)
                                    ^ 2i32) as u16),
                            )) as isize
                                * 28,
                        ))
                        .wrapping_add(8))
                        .cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 32i32 {
                        toCpy = ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                            (GetBattlerMultiplayerId(
                                ((((((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset(((multiplayerId) as i32) as isize * 28))
                                .wrapping_add(24)
                                .cast::<u16>())
                                .read()) as i32)
                                    ^ 1i32) as u16),
                            )) as isize
                                * 28,
                        ))
                        .wrapping_add(8))
                        .cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 33i32 {
                        toCpy = ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                            (GetBattlerMultiplayerId(
                                (((((((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset(((multiplayerId) as i32) as isize * 28))
                                .wrapping_add(24)
                                .cast::<u16>())
                                .read()) as i32)
                                    ^ 1i32)
                                    ^ 2i32) as u16),
                            )) as isize
                                * 28,
                        ))
                        .wrapping_add(8))
                        .cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 34i32 {
                        toCpy = ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                            (GetBattlerMultiplayerId(
                                (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23))
                                    .read()) as u16),
                            )) as isize
                                * 28,
                        ))
                        .wrapping_add(8))
                        .cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 35i32 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
                            toCpy = (((&raw mut gLinkPlayers).cast::<u8>()).wrapping_add(8))
                                .cast::<u8>();
                        } else {
                            toCpy =
                                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 36i32 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32) != 0 {
                            CopyFrontierTrainerText(
                                2u8,
                                ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                            );
                            toCpy = (&raw mut gStringVar4).cast::<u8>();
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 67108864u32)
                                != 0
                            {
                                CopyTrainerHillTrainerText(
                                    4u8,
                                    ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                                );
                                toCpy = (&raw mut gStringVar4).cast::<u8>();
                            } else {
                                toCpy = GetTrainerALoseText();
                            }
                        }
                        break 'l2;
                    }
                    if __sw1 == 37i32 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32) != 0 {
                            CopyFrontierTrainerText(
                                1u8,
                                ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                            );
                            toCpy = (&raw mut gStringVar4).cast::<u8>();
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 67108864u32)
                                != 0
                            {
                                CopyTrainerHillTrainerText(
                                    3u8,
                                    ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                                );
                                toCpy = (&raw mut gStringVar4).cast::<u8>();
                            }
                        }
                        break 'l2;
                    }
                    if __sw1 == 38i32 {
                        if ((GetBattlerSide(
                            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23)).read(),
                        )) as i32)
                            != 0i32
                        {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                                toCpy = ((&raw const sText_FoePkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            } else {
                                toCpy = ((&raw const sText_WildPkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>();
                            }
                            'l11: loop {
                                if !((((toCpy).read()) as i32) != 255i32) {
                                    break 'l11;
                                }
                                ((dst).wrapping_offset(((dstID) as i32) as isize))
                                    .write((toCpy).read());
                                dstID = (dstID).wrapping_add(1);
                                toCpy = (toCpy).wrapping_offset(1);
                            }
                            GetMonData3(
                                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                        .wrapping_add(82))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        } else {
                            GetMonData3(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                        .wrapping_add(82))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                2i32,
                                (&raw mut text).cast::<u8>(),
                            );
                        }
                        StringGet_Nickname((&raw mut text).cast::<u8>());
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 39i32 {
                        if (FlagGet(2219u16)) != 0 {
                            toCpy =
                                ((&raw const sText_Lanettes).cast::<u8>().cast_mut()).cast::<u8>();
                        } else {
                            toCpy =
                                ((&raw const sText_Someones).cast::<u8>().cast_mut()).cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 42i32 {
                        if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                            as i32)
                            == 0i32
                        {
                            toCpy = ((&raw const sText_AllyPkmnPrefix2).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        } else {
                            toCpy = ((&raw const sText_FoePkmnPrefix3).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 43i32 {
                        if ((GetBattlerSide(((&raw mut gBattlerTarget).cast::<u8>()).read()))
                            as i32)
                            == 0i32
                        {
                            toCpy = ((&raw const sText_AllyPkmnPrefix2).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        } else {
                            toCpy = ((&raw const sText_FoePkmnPrefix3).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 40i32 {
                        if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                            as i32)
                            == 0i32
                        {
                            toCpy = ((&raw const sText_AllyPkmnPrefix).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        } else {
                            toCpy = ((&raw const sText_FoePkmnPrefix2).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 41i32 {
                        if ((GetBattlerSide(((&raw mut gBattlerTarget).cast::<u8>()).read()))
                            as i32)
                            == 0i32
                        {
                            toCpy = ((&raw const sText_AllyPkmnPrefix).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        } else {
                            toCpy = ((&raw const sText_FoePkmnPrefix2).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 44i32 {
                        if ((GetBattlerSide(((&raw mut gBattlerAttacker).cast::<u8>()).read()))
                            as i32)
                            == 0i32
                        {
                            toCpy = ((&raw const sText_AllyPkmnPrefix3).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        } else {
                            toCpy = ((&raw const sText_FoePkmnPrefix4).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 45i32 {
                        if ((GetBattlerSide(((&raw mut gBattlerTarget).cast::<u8>()).read()))
                            as i32)
                            == 0i32
                        {
                            toCpy = ((&raw const sText_AllyPkmnPrefix3).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        } else {
                            toCpy = ((&raw const sText_FoePkmnPrefix4).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        }
                        break 'l2;
                    }
                    if __sw1 == 46i32 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32) != 0 {
                            toCpy = (((&raw mut gTrainerClassNames).cast::<u8>()).wrapping_offset(
                                ((GetFrontierOpponentClass(
                                    ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
                                )) as i32) as isize
                                    * 13,
                            ))
                            .cast::<u8>();
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 67108864u32)
                                != 0
                            {
                                toCpy = (((&raw mut gTrainerClassNames).cast::<u8>())
                                    .wrapping_offset(
                                        ((GetTrainerHillOpponentClass(
                                            ((&raw mut gTrainerBattleOpponent_B).cast::<u16>())
                                                .read(),
                                        )) as i32) as isize
                                            * 13,
                                    ))
                                .cast::<u8>();
                            } else {
                                toCpy = (((&raw mut gTrainerClassNames).cast::<u8>())
                                    .wrapping_offset(
                                        ((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                                            ((((&raw mut gTrainerBattleOpponent_B).cast::<u16>())
                                                .read())
                                                as i32)
                                                as isize
                                                * 40,
                                        ))
                                        .wrapping_add(1))
                                        .read()) as i32)
                                            as isize
                                            * 13,
                                    ))
                                .cast::<u8>();
                            }
                        }
                        break 'l2;
                    }
                    if __sw1 == 47i32 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32) != 0 {
                            GetFrontierTrainerName(
                                (&raw mut text).cast::<u8>(),
                                ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
                            );
                            toCpy = (&raw mut text).cast::<u8>();
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 67108864u32)
                                != 0
                            {
                                GetTrainerHillTrainerName(
                                    (&raw mut text).cast::<u8>(),
                                    ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
                                );
                                toCpy = (&raw mut text).cast::<u8>();
                            } else {
                                toCpy = ((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read())
                                        as i32) as isize
                                        * 40,
                                ))
                                .wrapping_add(4))
                                .cast::<u8>();
                            }
                        }
                        break 'l2;
                    }
                    if __sw1 == 48i32 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32) != 0 {
                            CopyFrontierTrainerText(
                                2u8,
                                ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
                            );
                            toCpy = (&raw mut gStringVar4).cast::<u8>();
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 67108864u32)
                                != 0
                            {
                                CopyTrainerHillTrainerText(
                                    4u8,
                                    ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
                                );
                                toCpy = (&raw mut gStringVar4).cast::<u8>();
                            } else {
                                toCpy = GetTrainerBLoseText();
                            }
                        }
                        break 'l2;
                    }
                    if __sw1 == 49i32 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32) != 0 {
                            CopyFrontierTrainerText(
                                1u8,
                                ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
                            );
                            toCpy = (&raw mut gStringVar4).cast::<u8>();
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 67108864u32)
                                != 0
                            {
                                CopyTrainerHillTrainerText(
                                    3u8,
                                    ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
                                );
                                toCpy = (&raw mut gStringVar4).cast::<u8>();
                            }
                        }
                        break 'l2;
                    }
                    if __sw1 == 50i32 {
                        toCpy = (((&raw mut gTrainerClassNames).cast::<u8>()).wrapping_offset(
                            ((GetFrontierOpponentClass(
                                ((&raw mut gPartnerTrainerId).cast::<u16>()).read(),
                            )) as i32) as isize
                                * 13,
                        ))
                        .cast::<u8>();
                        break 'l2;
                    }
                    if __sw1 == 51i32 {
                        GetFrontierTrainerName(
                            (&raw mut text).cast::<u8>(),
                            ((&raw mut gPartnerTrainerId).cast::<u16>()).read(),
                        );
                        toCpy = (&raw mut text).cast::<u8>();
                        break 'l2;
                    }
                }
                'l12: loop {
                    if !((((toCpy).read()) as i32) != 255i32) {
                        break 'l12;
                    }
                    ((dst).wrapping_offset(((dstID) as i32) as isize)).write((toCpy).read());
                    dstID = (dstID).wrapping_add(1);
                    toCpy = (toCpy).wrapping_offset(1);
                }
                if ((((((src).read()) as i32) == 36i32) || ((((src).read()) as i32) == 48i32))
                    || ((((src).read()) as i32) == 37i32))
                    || ((((src).read()) as i32) == 49i32)
                {
                    ((dst).wrapping_offset(((dstID) as i32) as isize)).write(252u8);
                    dstID = (dstID).wrapping_add(1);
                    ((dst).wrapping_offset(((dstID) as i32) as isize)).write(9u8);
                    dstID = (dstID).wrapping_add(1);
                }
            } else {
                ((dst).wrapping_offset(((dstID) as i32) as isize)).write((src).read());
                dstID = (dstID).wrapping_add(1);
            }
            src = (src).wrapping_offset(1);
        }
        ((dst).wrapping_offset(((dstID) as i32) as isize)).write((src).read());
        dstID = (dstID).wrapping_add(1);
        return dstID;
    }
}
pub(crate) unsafe extern "C" fn ExpandBattleTextBuffPlaceholders(src: *mut u8, dst: *mut u8) {
    unsafe {
        let mut src = src;
        let mut dst = dst;
        let mut srcID: u32 = 1u32;
        let mut value: u32 = 0u32;
        let mut nickname = crate::ffi::Align4([0u8; 11]);
        let mut hword: u16 = 0u16;
        (dst).write(255u8);
        'l1: loop {
            if !(((((src).wrapping_offset(((srcID) as i32) as isize)).read()) as i32) != 255i32) {
                break 'l1;
            }
            'l2: {
                let __sw1 = ((((src).wrapping_offset(((srcID) as i32) as isize)).read()) as i32);
                if __sw1 == 0i32 {
                    hword = ((((((src)
                        .wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                    .read()) as i32)
                        | ((((((src)
                            .wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                        .wrapping_offset(1))
                        .read()) as i32)
                            << 8)) as u16);
                    StringAppend(
                        dst,
                        ((((&raw const gBattleStringsTable)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset((((hword) as i32).wrapping_sub(12i32)) as isize))
                        .read(),
                    );
                    srcID = (srcID).wrapping_add(3u32);
                    break 'l2;
                }
                if __sw1 == 1i32 {
                    'l3: {
                        let __sw2 = ((((src)
                            .wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                        .read()) as i32);
                        if __sw2 == 1i32 {
                            value = ((((src)
                                .wrapping_offset((((srcID).wrapping_add(3u32)) as i32) as isize))
                            .read()) as u32);
                            break 'l3;
                        }
                        if __sw2 == 2i32 {
                            value = ((((((src)
                                .wrapping_offset((((srcID).wrapping_add(3u32)) as i32) as isize))
                            .read()) as i32)
                                | ((((((src).wrapping_offset(
                                    (((srcID).wrapping_add(3u32)) as i32) as isize,
                                ))
                                .wrapping_offset(1))
                                .read()) as i32)
                                    << 8)) as u32);
                            break 'l3;
                        }
                        if __sw2 == 4i32 {
                            value = ((((((((src)
                                .wrapping_offset((((srcID).wrapping_add(3u32)) as i32) as isize))
                            .read()) as i32)
                                | ((((((src).wrapping_offset(
                                    (((srcID).wrapping_add(3u32)) as i32) as isize,
                                ))
                                .wrapping_offset(1))
                                .read()) as i32)
                                    << 8))
                                | ((((((src).wrapping_offset(
                                    (((srcID).wrapping_add(3u32)) as i32) as isize,
                                ))
                                .wrapping_offset(2))
                                .read()) as i32)
                                    << 16))
                                | ((((((src).wrapping_offset(
                                    (((srcID).wrapping_add(3u32)) as i32) as isize,
                                ))
                                .wrapping_offset(3))
                                .read()) as i32)
                                    << 24)) as u32);
                            break 'l3;
                        }
                    }
                    ConvertIntToDecimalStringN(
                        dst,
                        ((value) as i32),
                        0i32,
                        ((src).wrapping_offset((((srcID).wrapping_add(2u32)) as i32) as isize))
                            .read(),
                    );
                    srcID = (srcID).wrapping_add(
                        ((((((src).wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                            .read()) as i32)
                            .wrapping_add(3i32)) as u32),
                    );
                    break 'l2;
                }
                if __sw1 == 2i32 {
                    StringAppend(
                        dst,
                        (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                            (((((src)
                                .wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                            .read()) as i32)
                                | ((((((src).wrapping_offset(
                                    (((srcID).wrapping_add(1u32)) as i32) as isize,
                                ))
                                .wrapping_offset(1))
                                .read()) as i32)
                                    << 8)) as isize
                                * 13,
                        ))
                        .cast::<u8>(),
                    );
                    srcID = (srcID).wrapping_add(3u32);
                    break 'l2;
                }
                if __sw1 == 3i32 {
                    StringAppend(
                        dst,
                        (((&raw mut gTypeNames).cast::<u8>()).wrapping_offset(
                            ((((src)
                                .wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                            .read()) as i32) as isize
                                * 7,
                        ))
                        .cast::<u8>(),
                    );
                    srcID = (srcID).wrapping_add(2u32);
                    break 'l2;
                }
                if __sw1 == 4i32 {
                    if ((GetBattlerSide(
                        ((src).wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                            .read(),
                    )) as i32)
                        == 0i32
                    {
                        GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((src).wrapping_offset(
                                    (((srcID).wrapping_add(2u32)) as i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            2i32,
                            (&raw mut nickname).cast::<u8>(),
                        );
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                            StringAppend(
                                dst,
                                ((&raw const sText_FoePkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>(),
                            );
                        } else {
                            StringAppend(
                                dst,
                                ((&raw const sText_WildPkmnPrefix).cast::<u8>().cast_mut())
                                    .cast::<u8>(),
                            );
                        }
                        GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((((src).wrapping_offset(
                                    (((srcID).wrapping_add(2u32)) as i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            2i32,
                            (&raw mut nickname).cast::<u8>(),
                        );
                    }
                    StringGet_Nickname((&raw mut nickname).cast::<u8>());
                    StringAppend(dst, (&raw mut nickname).cast::<u8>());
                    srcID = (srcID).wrapping_add(3u32);
                    break 'l2;
                }
                if __sw1 == 5i32 {
                    StringAppend(
                        dst,
                        ((((&raw const gStatNamesTable)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(
                            ((((src)
                                .wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                            .read()) as i32) as isize,
                        ))
                        .read(),
                    );
                    srcID = (srcID).wrapping_add(2u32);
                    break 'l2;
                }
                if __sw1 == 6i32 {
                    GetSpeciesName(
                        dst,
                        ((((((src).wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                            .read()) as i32)
                            | ((((((src)
                                .wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                            .wrapping_offset(1))
                            .read()) as i32)
                                << 8)) as u16),
                    );
                    srcID = (srcID).wrapping_add(3u32);
                    break 'l2;
                }
                if __sw1 == 7i32 {
                    if ((GetBattlerSide(
                        ((src).wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                            .read(),
                    )) as i32)
                        == 0i32
                    {
                        GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((src).wrapping_offset(
                                    (((srcID).wrapping_add(2u32)) as i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            2i32,
                            dst,
                        );
                    } else {
                        GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((((src).wrapping_offset(
                                    (((srcID).wrapping_add(2u32)) as i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            2i32,
                            dst,
                        );
                    }
                    StringGet_Nickname(dst);
                    srcID = (srcID).wrapping_add(3u32);
                    break 'l2;
                }
                if __sw1 == 8i32 {
                    StringAppend(
                        dst,
                        ((((&raw const gPokeblockWasTooXStringTable)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(
                            ((((src)
                                .wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                            .read()) as i32) as isize,
                        ))
                        .read(),
                    );
                    srcID = (srcID).wrapping_add(2u32);
                    break 'l2;
                }
                if __sw1 == 9i32 {
                    StringAppend(
                        dst,
                        (((&raw mut gAbilityNames).cast::<u8>()).wrapping_offset(
                            ((((src)
                                .wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                            .read()) as i32) as isize
                                * 13,
                        ))
                        .cast::<u8>(),
                    );
                    srcID = (srcID).wrapping_add(2u32);
                    break 'l2;
                }
                if __sw1 == 10i32 {
                    hword = ((((((src)
                        .wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                    .read()) as i32)
                        | ((((((src)
                            .wrapping_offset((((srcID).wrapping_add(1u32)) as i32) as isize))
                        .wrapping_offset(1))
                        .read()) as i32)
                            << 8)) as u16);
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0 {
                        if ((hword) as i32) == 175i32 {
                            if ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37))
                                    .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(24)
                            .cast::<u16>())
                            .read()) as i32)
                                == ((((&raw mut gPotentialItemEffectBattler).cast::<u8>()).read())
                                    as i32)
                            {
                                StringCopy(
                                    dst,
                                    (((&raw mut gEnigmaBerries).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gPotentialItemEffectBattler).cast::<u8>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .cast::<u8>(),
                                );
                                StringAppend(
                                    dst,
                                    ((&raw const sText_BerrySuffix).cast::<u8>().cast_mut())
                                        .cast::<u8>(),
                                );
                            } else {
                                StringAppend(
                                    dst,
                                    ((&raw const sText_EnigmaBerry).cast::<u8>().cast_mut())
                                        .cast::<u8>(),
                                );
                            }
                        } else {
                            CopyItemName(hword, dst);
                        }
                    } else {
                        CopyItemName(hword, dst);
                    }
                    srcID = (srcID).wrapping_add(3u32);
                    break 'l2;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ChooseMoveUsedParticle(textBuff: *mut u8) {
    unsafe {
        let mut textBuff = textBuff;
        let mut counter: i32 = 0i32;
        let mut i: u32 = 0u32;
        'l1: loop {
            if !(counter != 4i32) {
                break 'l1;
            }
            if ((((((&raw const sGrammarMoveUsedTable)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((i) as i32) as isize))
            .read()) as i32)
                == 0i32
            {
                counter = (counter).wrapping_add(1);
            }
            if ((((((&raw const sGrammarMoveUsedTable)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(
                (({
                    let __t1 = i;
                    i = (i).wrapping_add(1);
                    __t1
                }) as i32) as isize,
            ))
            .read()) as i32)
                == ((((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>())
                .read()) as i32)
            {
                break 'l1;
            }
        }
        if counter >= 0i32 {
            if counter <= 2i32 {
                StringCopy(
                    textBuff,
                    ((&raw const sText_SpaceIs).cast::<u8>().cast_mut()).cast::<u8>(),
                );
            } else {
                if counter <= 4i32 {
                    StringCopy(
                        textBuff,
                        ((&raw const sText_ApostropheS).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ChooseTypeOfMoveUsedString(dst: *mut u8) {
    unsafe {
        let mut dst = dst;
        let mut counter: i32 = 0i32;
        let mut i: i32 = 0i32;
        'l1: loop {
            if !((((dst).read()) as i32) != 255i32) {
                break 'l1;
            }
            dst = (dst).wrapping_offset(1);
        }
        'l2: loop {
            if !(counter != 4i32) {
                break 'l2;
            }
            if ((((((&raw const sGrammarMoveUsedTable)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset((i) as isize))
            .read()) as i32)
                == 0i32
            {
                counter = (counter).wrapping_add(1);
            }
            if ((((((&raw const sGrammarMoveUsedTable)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(
                ({
                    let __t1 = i;
                    i = (i).wrapping_add(1);
                    __t1
                }) as isize,
            ))
            .read()) as i32)
                == ((((((&raw mut gBattleMsgDataPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>())
                .read()) as i32)
            {
                break 'l2;
            }
        }
        'l3: {
            let __sw2 = counter;
            if __sw2 == 0i32 {
                StringCopy(
                    dst,
                    ((&raw const sText_ExclamationMark).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                break 'l3;
            }
            if __sw2 == 1i32 {
                StringCopy(
                    dst,
                    ((&raw const sText_ExclamationMark2).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                break 'l3;
            }
            if __sw2 == 2i32 {
                StringCopy(
                    dst,
                    ((&raw const sText_ExclamationMark3).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                break 'l3;
            }
            if __sw2 == 3i32 {
                StringCopy(
                    dst,
                    ((&raw const sText_ExclamationMark4).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                break 'l3;
            }
            if __sw2 == 4i32 {
                StringCopy(
                    dst,
                    ((&raw const sText_ExclamationMark5).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                break 'l3;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattlePutTextOnWindow(text: *mut u8, windowId: u8) {
    unsafe {
        let mut text = text;
        let mut windowId = windowId;
        let mut textInfo: *mut u8 = ((((&raw const sBattleTextOnWindowsInfo)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(
            (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(36)).read()) as i32)
                as isize,
        ))
        .read();
        let mut copyToVram: u32 = 0u32;
        let mut printerTemplate = crate::ffi::Align4([0u8; 16]);
        let mut speed: u8 = 0u8;
        if (((windowId) as i32) & 128i32) != 0 {
            windowId = ((((windowId) as i32) & (-129i32)) as u8);
            copyToVram = 0u32;
        } else {
            FillWindowPixelBuffer(
                windowId,
                ((textInfo).wrapping_offset(((windowId) as i32) as isize * 12)).read(),
            );
            copyToVram = 1u32;
        }
        (((&raw mut printerTemplate).cast::<u8>()).cast::<*mut u8>()).write(text);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(4)).write(windowId);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(5)).write(
            (((textInfo).wrapping_offset(((windowId) as i32) as isize * 12)).wrapping_add(1))
                .read(),
        );
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(6)).write(
            (((textInfo).wrapping_offset(((windowId) as i32) as isize * 12)).wrapping_add(2))
                .read(),
        );
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(7)).write(
            (((textInfo).wrapping_offset(((windowId) as i32) as isize * 12)).wrapping_add(3))
                .read(),
        );
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(8))
            .write((((&raw mut printerTemplate).cast::<u8>()).wrapping_add(6)).read());
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(9))
            .write((((&raw mut printerTemplate).cast::<u8>()).wrapping_add(7)).read());
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).write(
            (((textInfo).wrapping_offset(((windowId) as i32) as isize * 12)).wrapping_add(4))
                .read(),
        );
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(11)).write(
            (((textInfo).wrapping_offset(((windowId) as i32) as isize * 12)).wrapping_add(5))
                .read(),
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
            4,
            4,
            ((((textInfo).wrapping_offset(((windowId) as i32) as isize * 12)).wrapping_add(7))
                .read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
            0,
            4,
            ((((textInfo).wrapping_offset(((windowId) as i32) as isize * 12)).wrapping_add(8))
                .read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
            4,
            4,
            ((((textInfo).wrapping_offset(((windowId) as i32) as isize * 12)).wrapping_add(9))
                .read()) as i32,
        );
        if (((((&raw mut printerTemplate).cast::<u8>()).wrapping_add(6)).read()) as i32) == 255i32 {
            let mut width: u32 = GetBattleWindowTemplatePixelWidth(
                (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(36)).read()) as u32),
                ((windowId) as u32),
            );
            let mut alignX: i32 = GetStringCenterAlignXOffsetWithLetterSpacing(
                (((((&raw mut printerTemplate).cast::<u8>()).wrapping_add(5)).read()) as i32),
                (((&raw mut printerTemplate).cast::<u8>()).cast::<*mut u8>()).read(),
                ((width) as i32),
                (((((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).read()) as i32),
            );
            (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(6)).write({
                let __v1 = ((alignX) as u8);
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(8)).write(__v1);
                __v1
            });
        }
        if ((windowId) as i32) == 22i32 {
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                1,
                1,
                (0u8) as i32,
            );
        } else {
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                1,
                1,
                (1u8) as i32,
            );
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777218u32) != 0 {
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                2,
                1,
                (1u8) as i32,
            );
        } else {
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                2,
                1,
                (0u8) as i32,
            );
        }
        if (((windowId) as i32) == 0i32) || (((windowId) as i32) == 22i32) {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0 {
                speed = 1u8;
            } else {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
                    speed = ((((&raw const sRecordedBattleTextSpeeds)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((GetTextSpeedInRecordedBattle()) as i32) as isize))
                    .read();
                } else {
                    speed = GetPlayerTextSpeedDelay();
                }
            }
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                0,
                1,
                (1u8) as i32,
            );
        } else {
            speed = (((textInfo).wrapping_offset(((windowId) as i32) as isize * 12))
                .wrapping_add(6))
            .read();
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                0,
                1,
                (0u8) as i32,
            );
        }
        AddTextPrinter((&raw mut printerTemplate).cast::<u8>(), speed, None);
        if (copyToVram) != 0 {
            PutWindowTilemap(windowId);
            CopyWindowToVram(windowId, 3u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPPNumbersPaletteInMoveSelection() {
    unsafe {
        let mut chooseMoveStruct: *mut u8 = ((((&raw mut gBattleBufferA).cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
            ))
        .cast::<u8>())
        .wrapping_offset(4);
        let mut palPtr: *mut u16 = ((&raw mut gPPTextPalette).cast::<u16>()).cast::<u16>();
        let mut var: u8 = GetCurrentPPToMaxPPState(
            ((((chooseMoveStruct).wrapping_add(8)).cast::<u8>()).wrapping_offset(
                (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize,
            ))
            .read(),
            ((((chooseMoveStruct).wrapping_add(12)).cast::<u8>()).wrapping_offset(
                (((((&raw mut gMoveSelectionCursor).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize,
            ))
            .read(),
        );
        ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(92)).write(
            ((palPtr).wrapping_offset(
                ((((var) as i32).wrapping_mul(2i32)).wrapping_add(0i32)) as isize,
            ))
            .read(),
        );
        ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(91)).write(
            ((palPtr).wrapping_offset(
                ((((var) as i32).wrapping_mul(2i32)).wrapping_add(1i32)) as isize,
            ))
            .read(),
        );
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(92))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(92))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    2u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(91))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(91))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    2u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l7;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentPPToMaxPPState(currentPP: u8, maxPP: u8) -> u8 {
    unsafe {
        let mut currentPP = currentPP;
        let mut maxPP = maxPP;
        if ((maxPP) as i32) == ((currentPP) as i32) {
            return 3u8;
        } else {
            if ((maxPP) as i32) <= 2i32 {
                if ((currentPP) as i32) > 1i32 {
                    return 3u8;
                } else {
                    return (((2i32).wrapping_sub(((currentPP) as i32))) as u8);
                }
            } else {
                if ((maxPP) as i32) <= 7i32 {
                    if ((currentPP) as i32) > 2i32 {
                        return 3u8;
                    } else {
                        return (((2i32).wrapping_sub(((currentPP) as i32))) as u8);
                    }
                } else {
                    if ((currentPP) as i32) == 0i32 {
                        return 2u8;
                    }
                    if ((currentPP) as i32) <= crate::c::div_i32(((maxPP) as i32), 4i32) {
                        return 1u8;
                    }
                    if ((currentPP) as i32) > crate::c::div_i32(((maxPP) as i32), 2i32) {
                        return 3u8;
                    }
                }
            }
        }
        return 0u8;
    }
}
