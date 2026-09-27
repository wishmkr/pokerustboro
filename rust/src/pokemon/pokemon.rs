//! Translated from `src/pokemon.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gBattleMoves sCombinedMoves sSpeciesToHoennPokedexNum sSpeciesToNationalPokedexNum sHoennToNationalOrder gSpindaSpotGraphics gItemEffect_Potion gItemEffect_Antidote gItemEffect_BurnHeal gItemEffect_IceHeal gItemEffect_Awakening gItemEffect_ParalyzeHeal gItemEffect_FullRestore gItemEffect_MaxPotion gItemEffect_HyperPotion gItemEffect_SuperPotion gItemEffect_FullHeal gItemEffect_Revive gItemEffect_MaxRevive gItemEffect_FreshWater gItemEffect_SodaPop gItemEffect_Lemonade gItemEffect_MoomooMilk gItemEffect_EnergyPowder gItemEffect_EnergyRoot gItemEffect_HealPowder gItemEffect_RevivalHerb gItemEffect_Ether gItemEffect_MaxEther gItemEffect_Elixir gItemEffect_MaxElixir gItemEffect_LavaCookie gItemEffect_BlueFlute gItemEffect_YellowFlute gItemEffect_RedFlute gItemEffect_BerryJuice gItemEffect_SacredAsh gItemEffect_HPUp gItemEffect_Protein gItemEffect_Iron gItemEffect_Carbos gItemEffect_Calcium gItemEffect_RareCandy gItemEffect_PPUp gItemEffect_Zinc gItemEffect_PPMax gItemEffect_GuardSpec gItemEffect_DireHit gItemEffect_XAttack gItemEffect_XDefend gItemEffect_XSpeed gItemEffect_XAccuracy gItemEffect_XSpecial gItemEffect_SunStone gItemEffect_MoonStone gItemEffect_FireStone gItemEffect_ThunderStone gItemEffect_WaterStone gItemEffect_LeafStone gItemEffect_CheriBerry gItemEffect_ChestoBerry gItemEffect_PechaBerry gItemEffect_RawstBerry gItemEffect_AspearBerry gItemEffect_LeppaBerry gItemEffect_OranBerry gItemEffect_PersimBerry gItemEffect_LumBerry gItemEffect_SitrusBerry gItemEffect_PomegBerry gItemEffect_KelpsyBerry gItemEffect_QualotBerry gItemEffect_HondewBerry gItemEffect_GrepaBerry gItemEffect_TamatoBerry gItemEffectTable gNatureStatTable gTMHMLearnsets gFacilityClassToPicIndex gFacilityClassToTrainerClass gSpeciesIdToCryId gExperienceTables gSpeciesInfo sBulbasaurLevelUpLearnset sIvysaurLevelUpLearnset sVenusaurLevelUpLearnset sCharmanderLevelUpLearnset sCharmeleonLevelUpLearnset sCharizardLevelUpLearnset sSquirtleLevelUpLearnset sWartortleLevelUpLearnset sBlastoiseLevelUpLearnset sCaterpieLevelUpLearnset sMetapodLevelUpLearnset sButterfreeLevelUpLearnset sWeedleLevelUpLearnset sKakunaLevelUpLearnset sBeedrillLevelUpLearnset sPidgeyLevelUpLearnset sPidgeottoLevelUpLearnset sPidgeotLevelUpLearnset sRattataLevelUpLearnset sRaticateLevelUpLearnset sSpearowLevelUpLearnset sFearowLevelUpLearnset sEkansLevelUpLearnset sArbokLevelUpLearnset sPikachuLevelUpLearnset sRaichuLevelUpLearnset sSandshrewLevelUpLearnset sSandslashLevelUpLearnset sNidoranFLevelUpLearnset sNidorinaLevelUpLearnset sNidoqueenLevelUpLearnset sNidoranMLevelUpLearnset sNidorinoLevelUpLearnset sNidokingLevelUpLearnset sClefairyLevelUpLearnset sClefableLevelUpLearnset sVulpixLevelUpLearnset sNinetalesLevelUpLearnset sJigglypuffLevelUpLearnset sWigglytuffLevelUpLearnset sZubatLevelUpLearnset sGolbatLevelUpLearnset sOddishLevelUpLearnset sGloomLevelUpLearnset sVileplumeLevelUpLearnset sParasLevelUpLearnset sParasectLevelUpLearnset sVenonatLevelUpLearnset sVenomothLevelUpLearnset sDiglettLevelUpLearnset sDugtrioLevelUpLearnset sMeowthLevelUpLearnset sPersianLevelUpLearnset sPsyduckLevelUpLearnset sGolduckLevelUpLearnset sMankeyLevelUpLearnset sPrimeapeLevelUpLearnset sGrowlitheLevelUpLearnset sArcanineLevelUpLearnset sPoliwagLevelUpLearnset sPoliwhirlLevelUpLearnset sPoliwrathLevelUpLearnset sAbraLevelUpLearnset sKadabraLevelUpLearnset sAlakazamLevelUpLearnset sMachopLevelUpLearnset sMachokeLevelUpLearnset sMachampLevelUpLearnset sBellsproutLevelUpLearnset sWeepinbellLevelUpLearnset sVictreebelLevelUpLearnset sTentacoolLevelUpLearnset sTentacruelLevelUpLearnset sGeodudeLevelUpLearnset sGravelerLevelUpLearnset sGolemLevelUpLearnset sPonytaLevelUpLearnset sRapidashLevelUpLearnset sSlowpokeLevelUpLearnset sSlowbroLevelUpLearnset sMagnemiteLevelUpLearnset sMagnetonLevelUpLearnset sFarfetchdLevelUpLearnset sDoduoLevelUpLearnset sDodrioLevelUpLearnset sSeelLevelUpLearnset sDewgongLevelUpLearnset sGrimerLevelUpLearnset sMukLevelUpLearnset sShellderLevelUpLearnset sCloysterLevelUpLearnset sGastlyLevelUpLearnset sHaunterLevelUpLearnset sGengarLevelUpLearnset sOnixLevelUpLearnset sDrowzeeLevelUpLearnset sHypnoLevelUpLearnset sKrabbyLevelUpLearnset sKinglerLevelUpLearnset sVoltorbLevelUpLearnset sElectrodeLevelUpLearnset sExeggcuteLevelUpLearnset sExeggutorLevelUpLearnset sCuboneLevelUpLearnset sMarowakLevelUpLearnset sHitmonleeLevelUpLearnset sHitmonchanLevelUpLearnset sLickitungLevelUpLearnset sKoffingLevelUpLearnset sWeezingLevelUpLearnset sRhyhornLevelUpLearnset sRhydonLevelUpLearnset sChanseyLevelUpLearnset sTangelaLevelUpLearnset sKangaskhanLevelUpLearnset sHorseaLevelUpLearnset sSeadraLevelUpLearnset sGoldeenLevelUpLearnset sSeakingLevelUpLearnset sStaryuLevelUpLearnset sStarmieLevelUpLearnset sMrMimeLevelUpLearnset sScytherLevelUpLearnset sJynxLevelUpLearnset sElectabuzzLevelUpLearnset sMagmarLevelUpLearnset sPinsirLevelUpLearnset sTaurosLevelUpLearnset sMagikarpLevelUpLearnset sGyaradosLevelUpLearnset sLaprasLevelUpLearnset sDittoLevelUpLearnset sEeveeLevelUpLearnset sVaporeonLevelUpLearnset sJolteonLevelUpLearnset sFlareonLevelUpLearnset sPorygonLevelUpLearnset sOmanyteLevelUpLearnset sOmastarLevelUpLearnset sKabutoLevelUpLearnset sKabutopsLevelUpLearnset sAerodactylLevelUpLearnset sSnorlaxLevelUpLearnset sArticunoLevelUpLearnset sZapdosLevelUpLearnset sMoltresLevelUpLearnset sDratiniLevelUpLearnset sDragonairLevelUpLearnset sDragoniteLevelUpLearnset sMewtwoLevelUpLearnset sMewLevelUpLearnset sChikoritaLevelUpLearnset sBayleefLevelUpLearnset sMeganiumLevelUpLearnset sCyndaquilLevelUpLearnset sQuilavaLevelUpLearnset sTyphlosionLevelUpLearnset sTotodileLevelUpLearnset sCroconawLevelUpLearnset sFeraligatrLevelUpLearnset sSentretLevelUpLearnset sFurretLevelUpLearnset sHoothootLevelUpLearnset sNoctowlLevelUpLearnset sLedybaLevelUpLearnset sLedianLevelUpLearnset sSpinarakLevelUpLearnset sAriadosLevelUpLearnset sCrobatLevelUpLearnset sChinchouLevelUpLearnset sLanturnLevelUpLearnset sPichuLevelUpLearnset sCleffaLevelUpLearnset sIgglybuffLevelUpLearnset sTogepiLevelUpLearnset sTogeticLevelUpLearnset sNatuLevelUpLearnset sXatuLevelUpLearnset sMareepLevelUpLearnset sFlaaffyLevelUpLearnset sAmpharosLevelUpLearnset sBellossomLevelUpLearnset sMarillLevelUpLearnset sAzumarillLevelUpLearnset sSudowoodoLevelUpLearnset sPolitoedLevelUpLearnset sHoppipLevelUpLearnset sSkiploomLevelUpLearnset sJumpluffLevelUpLearnset sAipomLevelUpLearnset sSunkernLevelUpLearnset sSunfloraLevelUpLearnset sYanmaLevelUpLearnset sWooperLevelUpLearnset sQuagsireLevelUpLearnset sEspeonLevelUpLearnset sUmbreonLevelUpLearnset sMurkrowLevelUpLearnset sSlowkingLevelUpLearnset sMisdreavusLevelUpLearnset sUnownLevelUpLearnset sWobbuffetLevelUpLearnset sGirafarigLevelUpLearnset sPinecoLevelUpLearnset sForretressLevelUpLearnset sDunsparceLevelUpLearnset sGligarLevelUpLearnset sSteelixLevelUpLearnset sSnubbullLevelUpLearnset sGranbullLevelUpLearnset sQwilfishLevelUpLearnset sScizorLevelUpLearnset sShuckleLevelUpLearnset sHeracrossLevelUpLearnset sSneaselLevelUpLearnset sTeddiursaLevelUpLearnset sUrsaringLevelUpLearnset sSlugmaLevelUpLearnset sMagcargoLevelUpLearnset sSwinubLevelUpLearnset sPiloswineLevelUpLearnset sCorsolaLevelUpLearnset sRemoraidLevelUpLearnset sOctilleryLevelUpLearnset sDelibirdLevelUpLearnset sMantineLevelUpLearnset sSkarmoryLevelUpLearnset sHoundourLevelUpLearnset sHoundoomLevelUpLearnset sKingdraLevelUpLearnset sPhanpyLevelUpLearnset sDonphanLevelUpLearnset sPorygon2LevelUpLearnset sStantlerLevelUpLearnset sSmeargleLevelUpLearnset sTyrogueLevelUpLearnset sHitmontopLevelUpLearnset sSmoochumLevelUpLearnset sElekidLevelUpLearnset sMagbyLevelUpLearnset sMiltankLevelUpLearnset sBlisseyLevelUpLearnset sRaikouLevelUpLearnset sEnteiLevelUpLearnset sSuicuneLevelUpLearnset sLarvitarLevelUpLearnset sPupitarLevelUpLearnset sTyranitarLevelUpLearnset sLugiaLevelUpLearnset sHoOhLevelUpLearnset sCelebiLevelUpLearnset sSpecies252LevelUpLearnset sSpecies253LevelUpLearnset sSpecies254LevelUpLearnset sSpecies255LevelUpLearnset sSpecies256LevelUpLearnset sSpecies257LevelUpLearnset sSpecies258LevelUpLearnset sSpecies259LevelUpLearnset sSpecies260LevelUpLearnset sSpecies261LevelUpLearnset sSpecies262LevelUpLearnset sSpecies263LevelUpLearnset sSpecies264LevelUpLearnset sSpecies265LevelUpLearnset sSpecies266LevelUpLearnset sSpecies267LevelUpLearnset sSpecies268LevelUpLearnset sSpecies269LevelUpLearnset sSpecies270LevelUpLearnset sSpecies271LevelUpLearnset sSpecies272LevelUpLearnset sSpecies273LevelUpLearnset sSpecies274LevelUpLearnset sSpecies275LevelUpLearnset sSpecies276LevelUpLearnset sTreeckoLevelUpLearnset sGrovyleLevelUpLearnset sSceptileLevelUpLearnset sTorchicLevelUpLearnset sCombuskenLevelUpLearnset sBlazikenLevelUpLearnset sMudkipLevelUpLearnset sMarshtompLevelUpLearnset sSwampertLevelUpLearnset sPoochyenaLevelUpLearnset sMightyenaLevelUpLearnset sZigzagoonLevelUpLearnset sLinooneLevelUpLearnset sWurmpleLevelUpLearnset sSilcoonLevelUpLearnset sBeautiflyLevelUpLearnset sCascoonLevelUpLearnset sDustoxLevelUpLearnset sLotadLevelUpLearnset sLombreLevelUpLearnset sLudicoloLevelUpLearnset sSeedotLevelUpLearnset sNuzleafLevelUpLearnset sShiftryLevelUpLearnset sNincadaLevelUpLearnset sNinjaskLevelUpLearnset sShedinjaLevelUpLearnset sTaillowLevelUpLearnset sSwellowLevelUpLearnset sShroomishLevelUpLearnset sBreloomLevelUpLearnset sSpindaLevelUpLearnset sWingullLevelUpLearnset sPelipperLevelUpLearnset sSurskitLevelUpLearnset sMasquerainLevelUpLearnset sWailmerLevelUpLearnset sWailordLevelUpLearnset sSkittyLevelUpLearnset sDelcattyLevelUpLearnset sKecleonLevelUpLearnset sBaltoyLevelUpLearnset sClaydolLevelUpLearnset sNosepassLevelUpLearnset sTorkoalLevelUpLearnset sSableyeLevelUpLearnset sBarboachLevelUpLearnset sWhiscashLevelUpLearnset sLuvdiscLevelUpLearnset sCorphishLevelUpLearnset sCrawdauntLevelUpLearnset sFeebasLevelUpLearnset sMiloticLevelUpLearnset sCarvanhaLevelUpLearnset sSharpedoLevelUpLearnset sTrapinchLevelUpLearnset sVibravaLevelUpLearnset sFlygonLevelUpLearnset sMakuhitaLevelUpLearnset sHariyamaLevelUpLearnset sElectrikeLevelUpLearnset sManectricLevelUpLearnset sNumelLevelUpLearnset sCameruptLevelUpLearnset sSphealLevelUpLearnset sSealeoLevelUpLearnset sWalreinLevelUpLearnset sCacneaLevelUpLearnset sCacturneLevelUpLearnset sSnoruntLevelUpLearnset sGlalieLevelUpLearnset sLunatoneLevelUpLearnset sSolrockLevelUpLearnset sAzurillLevelUpLearnset sSpoinkLevelUpLearnset sGrumpigLevelUpLearnset sPlusleLevelUpLearnset sMinunLevelUpLearnset sMawileLevelUpLearnset sMedititeLevelUpLearnset sMedichamLevelUpLearnset sSwabluLevelUpLearnset sAltariaLevelUpLearnset sWynautLevelUpLearnset sDuskullLevelUpLearnset sDusclopsLevelUpLearnset sRoseliaLevelUpLearnset sSlakothLevelUpLearnset sVigorothLevelUpLearnset sSlakingLevelUpLearnset sGulpinLevelUpLearnset sSwalotLevelUpLearnset sTropiusLevelUpLearnset sWhismurLevelUpLearnset sLoudredLevelUpLearnset sExploudLevelUpLearnset sClamperlLevelUpLearnset sHuntailLevelUpLearnset sGorebyssLevelUpLearnset sAbsolLevelUpLearnset sShuppetLevelUpLearnset sBanetteLevelUpLearnset sSeviperLevelUpLearnset sZangooseLevelUpLearnset sRelicanthLevelUpLearnset sAronLevelUpLearnset sLaironLevelUpLearnset sAggronLevelUpLearnset sCastformLevelUpLearnset sVolbeatLevelUpLearnset sIllumiseLevelUpLearnset sLileepLevelUpLearnset sCradilyLevelUpLearnset sAnorithLevelUpLearnset sArmaldoLevelUpLearnset sRaltsLevelUpLearnset sKirliaLevelUpLearnset sGardevoirLevelUpLearnset sBagonLevelUpLearnset sShelgonLevelUpLearnset sSalamenceLevelUpLearnset sBeldumLevelUpLearnset sMetangLevelUpLearnset sMetagrossLevelUpLearnset sRegirockLevelUpLearnset sRegiceLevelUpLearnset sRegisteelLevelUpLearnset sKyogreLevelUpLearnset sGroudonLevelUpLearnset sRayquazaLevelUpLearnset sLatiasLevelUpLearnset sLatiosLevelUpLearnset sJirachiLevelUpLearnset sDeoxysLevelUpLearnset sChimechoLevelUpLearnset gEvolutionTable gLevelUpLearnsets sMonFrontAnimIdsTable sMonAnimationDelayTable gPPUpGetMask gPPUpClearMask gPPUpAddValues gStatStageRatios sDeoxysBaseStats gUnionRoomFacilityClasses sHoldEffectToType gBattlerSpriteTemplates sTrainerBackSpriteTemplates sSecretBaseFacilityClasses sGetMonDataEVConstants sStatsToRaise sFriendshipEventModifiers sHMMoves sAlteringCaveWildMonHeldItems sOamData_64x64 sSpriteTemplate_64x64
#[allow(unused_imports)]
use crate::data::pokemon::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLearningMoveTableID: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerPartyCount: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gEnemyPartyCount: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerParty: crate::ffi::Align4<[u8; 600]> = crate::ffi::Align4([0; 600]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gEnemyParty: crate::ffi::Align4<[u8; 600]> = crate::ffi::Align4([0; 600]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMultiuseSpriteTemplate: crate::ffi::Align4<[u8; 24]> = crate::ffi::Align4([0; 24]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMonSpritesGfxManagers: crate::ffi::Align4<[u8; 8]> =
    crate::ffi::Align4([0; 8]);

unsafe extern "C" {
    static mut gAbsentBattlerFlags: u8;
    static mut gActiveBattler: u8;
    static mut gAnims_MonPic: u8;
    static mut gApprentices: u8;
    static mut gBattleMonForms: u8;
    static mut gBattleMons: u8;
    static mut gBattleMoveDamage: u8;
    static mut gBattleMovePower: u8;
    static mut gBattleResources: u8;
    static mut gBattleResults: u8;
    static mut gBattleScripting: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTextBuff1: u8;
    static mut gBattleTextBuff2: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattleWeather: u8;
    static mut gBattlerAttacker: u8;
    static mut gBattlerInMenuId: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerTarget: u8;
    static mut gBattlersCount: u8;
    static mut gBitTable: u8;
    static mut gCritMultiplier: u8;
    static mut gCurrentMove: u8;
    static mut gDisableStructs: u8;
    static mut gDisplayedStringBattle: u8;
    static mut gEnigmaBerries: u8;
    static mut gGameLanguage: u8;
    static mut gGameVersion: u8;
    static mut gHitMarker: u8;
    static mut gLastUsedAbility: u8;
    static mut gLinkPlayers: u8;
    static mut gLocalTime: u8;
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gMonFrontAnimsPtrTable: u8;
    static mut gMonPaletteTable: u8;
    static mut gMonShinyPaletteTable: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gMoveToLearn: u8;
    static mut gPartnerTrainerId: u8;
    static mut gPokeblockFlavorCompatibilityTable: u8;
    static mut gPotentialItemEffectBattler: u8;
    static mut gRecordedBattleMultiplayerId: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSideTimers: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_MonBoxId: u8;
    static mut gSpecialVar_MonBoxPos: u8;
    static mut gSpeciesNames: u8;
    static mut gStatNamesTable: u8;
    static mut gStringVar1: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_BadEgg: u8;
    static mut gText_BattleWallyName: u8;
    static mut gText_DefendersStatRose: u8;
    static mut gText_EggNickname: u8;
    static mut gText_PkmnGettingPumped: u8;
    static mut gText_PkmnShroudedInMist: u8;
    static mut gText_PkmnsXPreventsSwitching: u8;
    static mut gText_StatRose: u8;
    static mut gTrainerBackAnimsPtrTable: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainerClassNames: u8;
    static mut gTrainerFrontAnimsPtrTable: u8;
    static mut gTrainers: u8;
    fn AbilityBattleEffects(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn BattleStringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> u32;
    fn BattleStringExpandPlaceholdersToDisplayedString(a0: *mut u8) -> u32;
    fn BeginEvolutionScene(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn BtlController_EmitGetMonData(a0: u8, a1: u8, a2: u8);
    fn ClearTemporarySpeciesSpriteData(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn DestroyTask(a0: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn GetApprenticeNameInLanguage(a0: u32, a1: i32) -> *mut u8;
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBoxMonDataAt(a0: u8, a1: u8, a2: i32) -> u32;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut u8;
    fn GetCurrentRegionMapSectionId() -> u8;
    fn GetFrontierEnemyMonLevel(a0: u8) -> u8;
    fn GetFrontierOpponentClass(a0: u16) -> u8;
    fn GetFrontierTrainerName(a0: *mut u8, a1: u16);
    fn GetItemHoldEffect(a0: u16) -> u8;
    fn GetItemHoldEffectParam(a0: u16) -> u8;
    fn GetMultiplayerId() -> u8;
    fn GetPCBoxToSendMon() -> u16;
    fn GetPartyIdFromBattlePartyId(a0: u8) -> u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetSpeciesBackAnimSet(a0: u16) -> u8;
    fn GetTrainerEncounterMusicIdInBattlePyramid(a0: u16) -> u8;
    fn GetTrainerEncounterMusicIdInTrainerHill(a0: u16) -> u8;
    fn InBattlePike() -> u8;
    fn InTrainerHillChallenge() -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn LaunchAnimationTaskForBackSprite(a0: *mut u8, a1: u8);
    fn LaunchAnimationTaskForFrontSprite(a0: *mut u8, a1: u8);
    fn MarkBattlerForControllerExec(a0: u8);
    fn PlayBGM(a0: u16);
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayNewMapMusic(a0: u16);
    fn Random() -> u16;
    fn ResetMapMusic();
    fn RtcCalcLocalTime();
    fn SetPCBoxToSendMon(a0: u8);
    fn SetSpriteCB_MonAnimDummy(a0: *mut u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn SpriteCallbackDummy_2(a0: *mut u8);
    fn StartMonSummaryAnimation(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StorageGetCurrentBox() -> u8;
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn StripExtCtrlCodes(a0: *mut u8);
    fn SummaryScreen_SetAnimDelayTaskId(a0: u8);
    fn UpdateSentPokesToOpponentValue(a0: u8);
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn m4aMPlayAllStop();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ZeroBoxMonData(boxMon: *mut u8) {
    unsafe {
        let mut boxMon = boxMon;
        let mut raw: *mut u8 = boxMon;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < 80u32) {
                    break 'l1;
                }
                'l2: {
                    ((raw).wrapping_offset(((i) as i32) as isize)).write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ZeroMonData(mon: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut arg: u32 = 0u32;
        ZeroBoxMonData((mon));
        arg = 0u32;
        SetMonData(mon, 55i32, (&raw mut arg).cast::<u8>());
        SetMonData(mon, 56i32, (&raw mut arg).cast::<u8>());
        SetMonData(mon, 57i32, (&raw mut arg).cast::<u8>());
        SetMonData(mon, 58i32, (&raw mut arg).cast::<u8>());
        SetMonData(mon, 59i32, (&raw mut arg).cast::<u8>());
        SetMonData(mon, 60i32, (&raw mut arg).cast::<u8>());
        SetMonData(mon, 61i32, (&raw mut arg).cast::<u8>());
        SetMonData(mon, 62i32, (&raw mut arg).cast::<u8>());
        SetMonData(mon, 63i32, (&raw mut arg).cast::<u8>());
        arg = 255u32;
        SetMonData(mon, 64i32, (&raw mut arg).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ZeroPlayerPartyMons() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ZeroMonData(
                        (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 100),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ZeroEnemyPartyMons() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ZeroMonData(
                        (((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 100),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMon(
    mon: *mut u8,
    species: u16,
    level: u8,
    fixedIV: u8,
    hasFixedPersonality: u8,
    fixedPersonality: u32,
    otIdType: u8,
    fixedOtId: u32,
) {
    unsafe {
        let mut mon = mon;
        let mut species = species;
        let mut level = level;
        let mut fixedIV = fixedIV;
        let mut hasFixedPersonality = hasFixedPersonality;
        let mut fixedPersonality = fixedPersonality;
        let mut otIdType = otIdType;
        let mut fixedOtId = fixedOtId;
        let mut mail: u32 = 0u32;
        ZeroMonData(mon);
        CreateBoxMon(
            (mon),
            species,
            level,
            fixedIV,
            hasFixedPersonality,
            fixedPersonality,
            otIdType,
            fixedOtId,
        );
        SetMonData(mon, 56i32, &raw mut level);
        mail = 255u32;
        SetMonData(mon, 64i32, (&raw mut mail).cast::<u8>());
        CalculateMonStats(mon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBoxMon(
    boxMon: *mut u8,
    species: u16,
    level: u8,
    fixedIV: u8,
    hasFixedPersonality: u8,
    fixedPersonality: u32,
    otIdType: u8,
    fixedOtId: u32,
) {
    unsafe {
        let mut boxMon = boxMon;
        let mut species = species;
        let mut level = level;
        let mut fixedIV = fixedIV;
        let mut hasFixedPersonality = hasFixedPersonality;
        let mut fixedPersonality = fixedPersonality;
        let mut otIdType = otIdType;
        let mut fixedOtId = fixedOtId;
        let mut speciesName = crate::ffi::Align4([0u8; 11]);
        let mut personality: u32 = 0u32;
        let mut value: u32 = 0u32;
        let mut checksum: u16 = 0u16;
        ZeroBoxMonData(boxMon);
        if (hasFixedPersonality) != 0 {
            personality = fixedPersonality;
        } else {
            personality = ((((Random()) as i32) | (((Random()) as i32) << 16)) as u32);
        }
        SetBoxMonData(boxMon, 0i32, (&raw mut personality).cast::<u8>());
        if ((otIdType) as i32) == 2i32 {
            let mut shinyValue: u32 = 0u32;
            'l1: loop {
                'l2: {
                    value = ((((Random()) as i32) | (((Random()) as i32) << 16)) as u32);
                    shinyValue = (((((value & 4294901760u32) >> 16) ^ (value & 65535u32))
                        ^ ((personality & 4294901760u32) >> 16))
                        ^ (personality & 65535u32));
                }
                if !(shinyValue < 8u32) {
                    break 'l1;
                }
            }
        } else {
            if ((otIdType) as i32) == 1i32 {
                value = fixedOtId;
            } else {
                value = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(10))
                .cast::<u8>())
                .read()) as i32)
                    | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8))
                    | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 16))
                    | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .cast::<u8>())
                    .wrapping_offset(3))
                    .read()) as i32)
                        << 24)) as u32);
            }
        }
        SetBoxMonData(boxMon, 1i32, (&raw mut value).cast::<u8>());
        checksum = CalculateBoxMonChecksum(boxMon);
        SetBoxMonData(boxMon, 9i32, (&raw mut checksum).cast::<u8>());
        EncryptBoxMon(boxMon);
        GetSpeciesName((&raw mut speciesName).cast::<u8>(), species);
        SetBoxMonData(boxMon, 2i32, (&raw mut speciesName).cast::<u8>());
        SetBoxMonData(boxMon, 3i32, (&raw mut gGameLanguage).cast::<u8>());
        SetBoxMonData(
            boxMon,
            7i32,
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        SetBoxMonData(boxMon, 11i32, (&raw mut species).cast::<u8>());
        SetBoxMonData(
            boxMon,
            25i32,
            ((((((&raw const gExperienceTables).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(19))
                    .read()) as i32) as isize
                        * 404,
                ))
            .cast::<u32>())
            .wrapping_offset(((level) as i32) as isize))
            .cast::<u8>(),
        );
        SetBoxMonData(
            boxMon,
            32i32,
            ((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(18),
        );
        value = ((GetCurrentRegionMapSectionId()) as u32);
        SetBoxMonData(boxMon, 35i32, (&raw mut value).cast::<u8>());
        SetBoxMonData(boxMon, 36i32, &raw mut level);
        SetBoxMonData(boxMon, 37i32, (&raw mut gGameVersion).cast::<u8>());
        value = 4u32;
        SetBoxMonData(boxMon, 38i32, (&raw mut value).cast::<u8>());
        SetBoxMonData(
            boxMon,
            49i32,
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8),
        );
        if ((fixedIV) as i32) < 32i32 {
            SetBoxMonData(boxMon, 39i32, &raw mut fixedIV);
            SetBoxMonData(boxMon, 40i32, &raw mut fixedIV);
            SetBoxMonData(boxMon, 41i32, &raw mut fixedIV);
            SetBoxMonData(boxMon, 42i32, &raw mut fixedIV);
            SetBoxMonData(boxMon, 43i32, &raw mut fixedIV);
            SetBoxMonData(boxMon, 44i32, &raw mut fixedIV);
        } else {
            let mut iv: u32 = 0u32;
            value = ((Random()) as u32);
            iv = (value & 31u32);
            SetBoxMonData(boxMon, 39i32, (&raw mut iv).cast::<u8>());
            iv = ((value & 992u32) >> 5);
            SetBoxMonData(boxMon, 40i32, (&raw mut iv).cast::<u8>());
            iv = ((value & 31744u32) >> 10);
            SetBoxMonData(boxMon, 41i32, (&raw mut iv).cast::<u8>());
            value = ((Random()) as u32);
            iv = (value & 31u32);
            SetBoxMonData(boxMon, 42i32, (&raw mut iv).cast::<u8>());
            iv = ((value & 992u32) >> 5);
            SetBoxMonData(boxMon, 43i32, (&raw mut iv).cast::<u8>());
            iv = ((value & 31744u32) >> 10);
            SetBoxMonData(boxMon, 44i32, (&raw mut iv).cast::<u8>());
        }
        if ((((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((species) as i32) as isize * 28))
        .wrapping_add(22))
        .cast::<u8>())
        .wrapping_offset(1))
        .read())
            != 0
        {
            value = (personality & 1u32);
            SetBoxMonData(boxMon, 46i32, (&raw mut value).cast::<u8>());
        }
        GiveBoxMonInitialMoveset(boxMon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonWithNature(
    mon: *mut u8,
    species: u16,
    level: u8,
    fixedIV: u8,
    nature: u8,
) {
    unsafe {
        let mut mon = mon;
        let mut species = species;
        let mut level = level;
        let mut fixedIV = fixedIV;
        let mut nature = nature;
        let mut personality: u32 = 0u32;
        'l1: loop {
            'l2: {
                personality = ((((Random()) as i32) | (((Random()) as i32) << 16)) as u32);
            }
            if !(((nature) as i32) != ((GetNatureFromPersonality(personality)) as i32)) {
                break 'l1;
            }
        }
        CreateMon(mon, species, level, fixedIV, 1u8, personality, 0u8, 0u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonWithGenderNatureLetter(
    mon: *mut u8,
    species: u16,
    level: u8,
    fixedIV: u8,
    gender: u8,
    nature: u8,
    unownLetter: u8,
) {
    unsafe {
        let mut mon = mon;
        let mut species = species;
        let mut level = level;
        let mut fixedIV = fixedIV;
        let mut gender = gender;
        let mut nature = nature;
        let mut unownLetter = unownLetter;
        let mut personality: u32 = 0u32;
        if (((((unownLetter) as i32).wrapping_sub(1i32)) as u8) as i32) < 28i32 {
            let mut actualLetter: u16 = 0u16;
            'l1: loop {
                'l2: {
                    personality = ((((Random()) as i32) | (((Random()) as i32) << 16)) as u32);
                    actualLetter = ((crate::c::rem_u32(
                        (((((personality & 50331648u32) >> 18)
                            | ((personality & 196608u32) >> 12))
                            | ((personality & 768u32) >> 6))
                            | ((personality & 3u32) >> 0)),
                        28u32,
                    )) as u16);
                }
                if !(((((nature) as i32) != ((GetNatureFromPersonality(personality)) as i32))
                    || (((gender) as i32)
                        != ((GetGenderFromSpeciesAndPersonality(species, personality)) as i32)))
                    || (((actualLetter) as i32) != ((unownLetter) as i32).wrapping_sub(1i32)))
                {
                    break 'l1;
                }
            }
        } else {
            'l3: loop {
                'l4: {
                    personality = ((((Random()) as i32) | (((Random()) as i32) << 16)) as u32);
                }
                if !((((nature) as i32) != ((GetNatureFromPersonality(personality)) as i32))
                    || (((gender) as i32)
                        != ((GetGenderFromSpeciesAndPersonality(species, personality)) as i32)))
                {
                    break 'l3;
                }
            }
        }
        CreateMon(mon, species, level, fixedIV, 1u8, personality, 0u8, 0u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMaleMon(mon: *mut u8, species: u16, level: u8) {
    unsafe {
        let mut mon = mon;
        let mut species = species;
        let mut level = level;
        let mut personality: u32 = 0u32;
        let mut otId: u32 = 0u32;
        'l1: loop {
            'l2: {
                otId = ((((Random()) as i32) | (((Random()) as i32) << 16)) as u32);
                personality = ((((Random()) as i32) | (((Random()) as i32) << 16)) as u32);
            }
            if !(((GetGenderFromSpeciesAndPersonality(species, personality)) as i32) != 0i32) {
                break 'l1;
            }
        }
        CreateMon(mon, species, level, 32u8, 1u8, personality, 1u8, otId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonWithIVsPersonality(
    mon: *mut u8,
    species: u16,
    level: u8,
    ivs: u32,
    personality: u32,
) {
    unsafe {
        let mut mon = mon;
        let mut species = species;
        let mut level = level;
        let mut ivs = ivs;
        let mut personality = personality;
        CreateMon(mon, species, level, 0u8, 1u8, personality, 0u8, 0u32);
        SetMonData(mon, 66i32, (&raw mut ivs).cast::<u8>());
        CalculateMonStats(mon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonWithIVsOTID(
    mon: *mut u8,
    species: u16,
    level: u8,
    ivs: *mut u8,
    otId: u32,
) {
    unsafe {
        let mut mon = mon;
        let mut species = species;
        let mut level = level;
        let mut ivs = ivs;
        let mut otId = otId;
        CreateMon(mon, species, level, 0u8, 0u8, 0u32, 1u8, otId);
        SetMonData(mon, 39i32, ivs);
        SetMonData(mon, 40i32, (ivs).wrapping_offset(1));
        SetMonData(mon, 41i32, (ivs).wrapping_offset(2));
        SetMonData(mon, 42i32, (ivs).wrapping_offset(3));
        SetMonData(mon, 43i32, (ivs).wrapping_offset(4));
        SetMonData(mon, 44i32, (ivs).wrapping_offset(5));
        CalculateMonStats(mon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonWithEVSpread(
    mon: *mut u8,
    species: u16,
    level: u8,
    fixedIV: u8,
    evSpread: u8,
) {
    unsafe {
        let mut mon = mon;
        let mut species = species;
        let mut level = level;
        let mut fixedIV = fixedIV;
        let mut evSpread = evSpread;
        let mut i: i32 = 0i32;
        let mut statCount: i32 = 0i32;
        let mut evAmount: u16 = 0u16;
        let mut evsBits: u8 = 0u8;
        CreateMon(mon, species, level, fixedIV, 0u8, 0u32, 0u8, 0u32);
        evsBits = evSpread;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (((evsBits) as i32) & 1i32) != 0 {
                        statCount = (statCount).wrapping_add(1);
                    }
                    evsBits = ((((evsBits) as i32) >> 1) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        evAmount = ((crate::c::div_i32(510i32, statCount)) as u16);
        evsBits = 1u8;
        {
            i = 0i32;
            'l3: loop {
                if !(i < 6i32) {
                    break 'l3;
                }
                'l4: {
                    if (((evSpread) as i32) & ((evsBits) as i32)) != 0 {
                        SetMonData(
                            mon,
                            (26i32).wrapping_add(i),
                            (&raw mut evAmount).cast::<u8>(),
                        );
                    }
                    evsBits = ((((evsBits) as i32) << 1) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        CalculateMonStats(mon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBattleTowerMon(mon: *mut u8, src: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut src = src;
        let mut i: i32 = 0i32;
        let mut nickname = crate::ffi::Align4([0u8; 32]);
        let mut language: u8 = 0u8;
        let mut value: u8 = 0u8;
        CreateMon(
            mon,
            ((src).cast::<u16>()).read(),
            ((src).wrapping_add(12)).read(),
            0u8,
            1u8,
            ((src).wrapping_add(28).cast::<u32>()).read(),
            1u8,
            ((src).wrapping_add(20).cast::<u32>()).read(),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    SetMonMoveSlot(
                        mon,
                        ((((src).wrapping_add(4)).cast::<u16>()).wrapping_offset((i) as isize))
                            .read(),
                        ((i) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        SetMonData(mon, 21i32, (src).wrapping_add(13));
        SetMonData(
            mon,
            12i32,
            ((src).wrapping_add(2).cast::<u16>()).cast::<u8>(),
        );
        SetMonData(mon, 32i32, (src).wrapping_add(43));
        StringCopy(
            (&raw mut nickname).cast::<u8>(),
            ((src).wrapping_add(32)).cast::<u8>(),
        );
        if (((((&raw mut nickname).cast::<u8>()).read()) as i32) == 252i32)
            && ((((((&raw mut nickname).cast::<u8>()).wrapping_offset(1)).read()) as i32) == 21i32)
        {
            language = 1u8;
            StripExtCtrlCodes((&raw mut nickname).cast::<u8>());
        } else {
            language = 2u8;
        }
        SetMonData(mon, 3i32, &raw mut language);
        SetMonData(mon, 2i32, (&raw mut nickname).cast::<u8>());
        SetMonData(mon, 26i32, (src).wrapping_add(14));
        SetMonData(mon, 27i32, (src).wrapping_add(15));
        SetMonData(mon, 28i32, (src).wrapping_add(16));
        SetMonData(mon, 29i32, (src).wrapping_add(17));
        SetMonData(mon, 30i32, (src).wrapping_add(18));
        SetMonData(mon, 31i32, (src).wrapping_add(19));
        value = ((crate::c::bf_read((src).wrapping_add(27), 7, 1, false) as u32) as u8);
        SetMonData(mon, 46i32, &raw mut value);
        value = ((crate::c::bf_read((src).wrapping_add(24), 0, 5, false) as u32) as u8);
        SetMonData(mon, 39i32, &raw mut value);
        value = ((crate::c::bf_read((src).wrapping_add(24), 5, 5, false) as u32) as u8);
        SetMonData(mon, 40i32, &raw mut value);
        value = ((crate::c::bf_read((src).wrapping_add(25), 2, 5, false) as u32) as u8);
        SetMonData(mon, 41i32, &raw mut value);
        value = ((crate::c::bf_read((src).wrapping_add(25), 7, 5, false) as u32) as u8);
        SetMonData(mon, 42i32, &raw mut value);
        value = ((crate::c::bf_read((src).wrapping_add(26), 4, 5, false) as u32) as u8);
        SetMonData(mon, 43i32, &raw mut value);
        value = ((crate::c::bf_read((src).wrapping_add(27), 1, 5, false) as u32) as u8);
        SetMonData(mon, 44i32, &raw mut value);
        MonRestorePP(mon);
        CalculateMonStats(mon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBattleTowerMon_HandleLevel(mon: *mut u8, src: *mut u8, lvl50: u8) {
    unsafe {
        let mut mon = mon;
        let mut src = src;
        let mut lvl50 = lvl50;
        let mut i: i32 = 0i32;
        let mut nickname = crate::ffi::Align4([0u8; 32]);
        let mut level: u8 = 0u8;
        let mut language: u8 = 0u8;
        let mut value: u8 = 0u8;
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32)
            != 0i32
        {
            level = GetFrontierEnemyMonLevel(
                (crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8),
            );
        } else {
            if (lvl50) != 0 {
                level = 50u8;
            } else {
                level = ((src).wrapping_add(12)).read();
            }
        }
        CreateMon(
            mon,
            ((src).cast::<u16>()).read(),
            level,
            0u8,
            1u8,
            ((src).wrapping_add(28).cast::<u32>()).read(),
            1u8,
            ((src).wrapping_add(20).cast::<u32>()).read(),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    SetMonMoveSlot(
                        mon,
                        ((((src).wrapping_add(4)).cast::<u16>()).wrapping_offset((i) as isize))
                            .read(),
                        ((i) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        SetMonData(mon, 21i32, (src).wrapping_add(13));
        SetMonData(
            mon,
            12i32,
            ((src).wrapping_add(2).cast::<u16>()).cast::<u8>(),
        );
        SetMonData(mon, 32i32, (src).wrapping_add(43));
        StringCopy(
            (&raw mut nickname).cast::<u8>(),
            ((src).wrapping_add(32)).cast::<u8>(),
        );
        if (((((&raw mut nickname).cast::<u8>()).read()) as i32) == 252i32)
            && ((((((&raw mut nickname).cast::<u8>()).wrapping_offset(1)).read()) as i32) == 21i32)
        {
            language = 1u8;
            StripExtCtrlCodes((&raw mut nickname).cast::<u8>());
        } else {
            language = 2u8;
        }
        SetMonData(mon, 3i32, &raw mut language);
        SetMonData(mon, 2i32, (&raw mut nickname).cast::<u8>());
        SetMonData(mon, 26i32, (src).wrapping_add(14));
        SetMonData(mon, 27i32, (src).wrapping_add(15));
        SetMonData(mon, 28i32, (src).wrapping_add(16));
        SetMonData(mon, 29i32, (src).wrapping_add(17));
        SetMonData(mon, 30i32, (src).wrapping_add(18));
        SetMonData(mon, 31i32, (src).wrapping_add(19));
        value = ((crate::c::bf_read((src).wrapping_add(27), 7, 1, false) as u32) as u8);
        SetMonData(mon, 46i32, &raw mut value);
        value = ((crate::c::bf_read((src).wrapping_add(24), 0, 5, false) as u32) as u8);
        SetMonData(mon, 39i32, &raw mut value);
        value = ((crate::c::bf_read((src).wrapping_add(24), 5, 5, false) as u32) as u8);
        SetMonData(mon, 40i32, &raw mut value);
        value = ((crate::c::bf_read((src).wrapping_add(25), 2, 5, false) as u32) as u8);
        SetMonData(mon, 41i32, &raw mut value);
        value = ((crate::c::bf_read((src).wrapping_add(25), 7, 5, false) as u32) as u8);
        SetMonData(mon, 42i32, &raw mut value);
        value = ((crate::c::bf_read((src).wrapping_add(26), 4, 5, false) as u32) as u8);
        SetMonData(mon, 43i32, &raw mut value);
        value = ((crate::c::bf_read((src).wrapping_add(27), 1, 5, false) as u32) as u8);
        SetMonData(mon, 44i32, &raw mut value);
        MonRestorePP(mon);
        CalculateMonStats(mon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateApprenticeMon(mon: *mut u8, src: *mut u8, monId: u8) {
    unsafe {
        let mut mon = mon;
        let mut src = src;
        let mut monId = monId;
        let mut i: i32 = 0i32;
        let mut evAmount: u16 = 0u16;
        let mut language: u8 = 0u8;
        let mut otId: u32 = ((((((&raw mut gApprentices).cast::<u8>()).wrapping_offset(
            ((crate::c::bf_read((src).wrapping_add(0), 0, 5, false) as u8) as i32) as isize * 88,
        ))
        .wrapping_add(48)
        .cast::<u16>())
        .read()) as u32);
        let mut personality: u32 =
            (((((((((((&raw mut gApprentices).cast::<u8>()).wrapping_offset(
                ((crate::c::bf_read((src).wrapping_add(0), 0, 5, false) as u8) as i32) as isize
                    * 88,
            ))
            .wrapping_add(48)
            .cast::<u16>())
            .read()) as i32)
                >> 8)
                | ((((((((&raw mut gApprentices).cast::<u8>()).wrapping_offset(
                    ((crate::c::bf_read((src).wrapping_add(0), 0, 5, false) as u8) as i32) as isize
                        * 88,
                ))
                .wrapping_add(48)
                .cast::<u16>())
                .read()) as i32)
                    & 255i32)
                    << 8))
                .wrapping_add(
                    (((((((src).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 12))
                    .cast::<u16>())
                    .read()) as i32),
                ))
            .wrapping_add(((((src).wrapping_add(2)).read()) as i32))) as u32);
        CreateMon(
            mon,
            (((((src).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(((monId) as i32) as isize * 12))
            .cast::<u16>())
            .read(),
            GetFrontierEnemyMonLevel(
                ((((crate::c::bf_read((src).wrapping_add(0), 5, 2, false) as u8) as i32)
                    .wrapping_sub(1i32)) as u8),
            ),
            31u8,
            1u8,
            personality,
            1u8,
            otId,
        );
        SetMonData(
            mon,
            12i32,
            (((((src).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(((monId) as i32) as isize * 12))
            .wrapping_add(10)
            .cast::<u16>())
            .cast::<u8>(),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    SetMonMoveSlot(
                        mon,
                        (((((((src).wrapping_add(4)).cast::<u8>())
                            .wrapping_offset(((monId) as i32) as isize * 12))
                        .wrapping_add(2))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                        ((i) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        evAmount = ((crate::c::div_i32(510i32, 6i32)) as u16);
        {
            i = 0i32;
            'l3: loop {
                if !(i < 6i32) {
                    break 'l3;
                }
                'l4: {
                    SetMonData(
                        mon,
                        (26i32).wrapping_add(i),
                        (&raw mut evAmount).cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        language = ((src).wrapping_add(63)).read();
        SetMonData(mon, 3i32, &raw mut language);
        SetMonData(
            mon,
            7i32,
            GetApprenticeNameInLanguage(
                ((crate::c::bf_read((src).wrapping_add(0), 0, 5, false) as u8) as u32),
                ((language) as i32),
            ),
        );
        CalculateMonStats(mon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonWithEVSpreadNatureOTID(
    mon: *mut u8,
    species: u16,
    level: u8,
    nature: u8,
    fixedIV: u8,
    evSpread: u8,
    otId: u32,
) {
    unsafe {
        let mut mon = mon;
        let mut species = species;
        let mut level = level;
        let mut nature = nature;
        let mut fixedIV = fixedIV;
        let mut evSpread = evSpread;
        let mut otId = otId;
        let mut i: i32 = 0i32;
        let mut statCount: i32 = 0i32;
        let mut evsBits: u8 = 0u8;
        let mut evAmount: u16 = 0u16;
        'l1: loop {
            'l2: {
                i = (((Random()) as i32) | (((Random()) as i32) << 16));
            }
            if !(((nature) as i32) != ((GetNatureFromPersonality(((i) as u32))) as i32)) {
                break 'l1;
            }
        }
        CreateMon(mon, species, level, fixedIV, 1u8, ((i) as u32), 1u8, otId);
        evsBits = evSpread;
        {
            i = 0i32;
            'l3: loop {
                if !(i < 6i32) {
                    break 'l3;
                }
                'l4: {
                    if (((evsBits) as i32) & 1i32) != 0 {
                        statCount = (statCount).wrapping_add(1);
                    }
                    evsBits = ((((evsBits) as i32) >> 1) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        evAmount = ((crate::c::div_i32(510i32, statCount)) as u16);
        evsBits = 1u8;
        {
            i = 0i32;
            'l5: loop {
                if !(i < 6i32) {
                    break 'l5;
                }
                'l6: {
                    if (((evSpread) as i32) & ((evsBits) as i32)) != 0 {
                        SetMonData(
                            mon,
                            (26i32).wrapping_add(i),
                            (&raw mut evAmount).cast::<u8>(),
                        );
                    }
                    evsBits = ((((evsBits) as i32) << 1) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        CalculateMonStats(mon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertPokemonToBattleTowerPokemon(mon: *mut u8, dest: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut dest = dest;
        let mut i: i32 = 0i32;
        let mut heldItem: u16 = 0u16;
        ((dest).cast::<u16>()).write(((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16));
        heldItem = ((GetMonData3(mon, 12i32, core::ptr::null_mut())) as u16);
        if ((heldItem) as i32) == 175i32 {
            heldItem = 0u16;
        }
        ((dest).wrapping_add(2).cast::<u16>()).write(heldItem);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((dest).wrapping_add(4)).cast::<u16>()).wrapping_offset((i) as isize)).write(
                        ((GetMonData3(mon, (13i32).wrapping_add(i), core::ptr::null_mut())) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((dest).wrapping_add(12)).write(((GetMonData3(mon, 56i32, core::ptr::null_mut())) as u8));
        ((dest).wrapping_add(13)).write(((GetMonData3(mon, 21i32, core::ptr::null_mut())) as u8));
        ((dest).wrapping_add(20).cast::<u32>()).write(GetMonData3(
            mon,
            1i32,
            core::ptr::null_mut(),
        ));
        ((dest).wrapping_add(14)).write(((GetMonData3(mon, 26i32, core::ptr::null_mut())) as u8));
        ((dest).wrapping_add(15)).write(((GetMonData3(mon, 27i32, core::ptr::null_mut())) as u8));
        ((dest).wrapping_add(16)).write(((GetMonData3(mon, 28i32, core::ptr::null_mut())) as u8));
        ((dest).wrapping_add(17)).write(((GetMonData3(mon, 29i32, core::ptr::null_mut())) as u8));
        ((dest).wrapping_add(18)).write(((GetMonData3(mon, 30i32, core::ptr::null_mut())) as u8));
        ((dest).wrapping_add(19)).write(((GetMonData3(mon, 31i32, core::ptr::null_mut())) as u8));
        ((dest).wrapping_add(43)).write(((GetMonData3(mon, 32i32, core::ptr::null_mut())) as u8));
        crate::c::bf_write(
            (dest).wrapping_add(24),
            0,
            5,
            (GetMonData3(mon, 39i32, core::ptr::null_mut())) as i32,
        );
        crate::c::bf_write(
            (dest).wrapping_add(24),
            5,
            5,
            (GetMonData3(mon, 40i32, core::ptr::null_mut())) as i32,
        );
        crate::c::bf_write(
            (dest).wrapping_add(25),
            2,
            5,
            (GetMonData3(mon, 41i32, core::ptr::null_mut())) as i32,
        );
        crate::c::bf_write(
            (dest).wrapping_add(25),
            7,
            5,
            (GetMonData3(mon, 42i32, core::ptr::null_mut())) as i32,
        );
        crate::c::bf_write(
            (dest).wrapping_add(26),
            4,
            5,
            (GetMonData3(mon, 43i32, core::ptr::null_mut())) as i32,
        );
        crate::c::bf_write(
            (dest).wrapping_add(27),
            1,
            5,
            (GetMonData3(mon, 44i32, core::ptr::null_mut())) as i32,
        );
        crate::c::bf_write(
            (dest).wrapping_add(27),
            7,
            1,
            (GetMonData3(mon, 46i32, core::ptr::null_mut())) as i32,
        );
        ((dest).wrapping_add(28).cast::<u32>()).write(GetMonData3(
            mon,
            0i32,
            core::ptr::null_mut(),
        ));
        GetMonData3(mon, 2i32, ((dest).wrapping_add(32)).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn CreateEventMon(
    mon: *mut u8,
    species: u16,
    level: u8,
    fixedIV: u8,
    hasFixedPersonality: u8,
    fixedPersonality: u32,
    otIdType: u8,
    fixedOtId: u32,
) {
    unsafe {
        let mut mon = mon;
        let mut species = species;
        let mut level = level;
        let mut fixedIV = fixedIV;
        let mut hasFixedPersonality = hasFixedPersonality;
        let mut fixedPersonality = fixedPersonality;
        let mut otIdType = otIdType;
        let mut fixedOtId = fixedOtId;
        let mut isModernFatefulEncounter: u32 = 1u32;
        CreateMon(
            mon,
            species,
            level,
            fixedIV,
            hasFixedPersonality,
            fixedPersonality,
            otIdType,
            fixedOtId,
        );
        SetMonData(mon, 80i32, (&raw mut isModernFatefulEncounter).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldIgnoreDeoxysForm(caseId: u8, battler: u8) -> u8 {
    unsafe {
        let mut caseId = caseId;
        let mut battler = battler;
        'l1: {
            let __sw1 = ((caseId) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32;
            if __sw1 == 0i32 || !__matched {
                return 0u8;
            }
            if __sw1 == 1i32 {
                if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0) {
                    return 0u8;
                }
                if !((crate::c::bf_read(
                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                    1,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    return 0u8;
                }
                if ((((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                .wrapping_add(24)
                .cast::<u16>())
                .read()) as i32)
                    == ((battler) as i32)
                {
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0) {
                    return 0u8;
                }
                if !((crate::c::bf_read(
                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                    1,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    return 0u8;
                }
                if ((((battler) as i32) == 1i32) || (((battler) as i32) == 4i32))
                    || (((battler) as i32) == 5i32)
                {
                    return 1u8;
                }
                return 0u8;
            }
            if __sw1 == 4i32 {
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0 {
                    if !((crate::c::bf_read(
                        ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                        1,
                        1,
                        false,
                    ) as u8)
                        != 0)
                    {
                        return 0u8;
                    }
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
                        if ((((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                        .wrapping_add(24)
                        .cast::<u16>())
                        .read()) as i32)
                            == ((battler) as i32)
                        {
                            return 0u8;
                        }
                    } else {
                        if ((GetBattlerSide(battler)) as i32) == 0i32 {
                            return 0u8;
                        }
                    }
                } else {
                    if !((crate::c::bf_read(
                        ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                        1,
                        1,
                        false,
                    ) as u8)
                        != 0)
                    {
                        return 0u8;
                    }
                    if ((GetBattlerSide(battler)) as i32) == 0i32 {
                        return 0u8;
                    }
                }
                break 'l1;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn GetDeoxysStat(mon: *mut u8, statId: i32) -> u16 {
    unsafe {
        let mut mon = mon;
        let mut statId = statId;
        let mut ivVal: i32 = 0i32;
        let mut evVal: i32 = 0i32;
        let mut statValue: u16 = 0u16;
        let mut nature: u8 = 0u8;
        if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 32u32) != 0)
            || (GetMonData3(mon, 11i32, core::ptr::null_mut()) != 410u32)
        {
            return 0u16;
        }
        ivVal = ((GetMonData3(mon, (39i32).wrapping_add(statId), core::ptr::null_mut())) as i32);
        evVal = ((GetMonData3(mon, (26i32).wrapping_add(statId), core::ptr::null_mut())) as i32);
        statValue = (((crate::c::div_i32(
            (((((((((&raw const sDeoxysBaseStats)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset((statId) as isize))
            .read()) as i32)
                .wrapping_mul(2i32))
            .wrapping_add(ivVal))
            .wrapping_add(crate::c::div_i32(evVal, 4i32)))
            .wrapping_mul(((((mon).wrapping_add(84)).read()) as i32)),
            100i32,
        ))
        .wrapping_add(5i32)) as u16);
        nature = GetNature(mon);
        statValue = ModifyStatByNature(nature, statValue, ((statId) as u8));
        return statValue;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDeoxysStats() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut value: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut mon: *mut u8 = (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 100);
                    if GetMonData3(mon, 11i32, core::ptr::null_mut()) != 410u32 {
                        break 'l2;
                    }
                    value = ((GetMonData3(mon, 59i32, core::ptr::null_mut())) as i32);
                    SetMonData(mon, 59i32, (&raw mut value).cast::<u8>());
                    value = ((GetMonData3(mon, 60i32, core::ptr::null_mut())) as i32);
                    SetMonData(mon, 60i32, (&raw mut value).cast::<u8>());
                    value = ((GetMonData3(mon, 61i32, core::ptr::null_mut())) as i32);
                    SetMonData(mon, 61i32, (&raw mut value).cast::<u8>());
                    value = ((GetMonData3(mon, 62i32, core::ptr::null_mut())) as i32);
                    SetMonData(mon, 62i32, (&raw mut value).cast::<u8>());
                    value = ((GetMonData3(mon, 63i32, core::ptr::null_mut())) as i32);
                    SetMonData(mon, 63i32, (&raw mut value).cast::<u8>());
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUnionRoomTrainerPic() -> u16 {
    unsafe {
        let mut linkId: u8 = 0u8;
        let mut arrId: u32 = 0u32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554432u32) != 0 {
            linkId = ((((((&raw mut gRecordedBattleMultiplayerId).cast::<u8>()).read()) as i32)
                ^ 1i32) as u8);
        } else {
            linkId = ((((GetMultiplayerId()) as i32) ^ 1i32) as u8);
        }
        arrId = crate::c::rem_u32(
            ((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((linkId) as i32) as isize * 28))
            .wrapping_add(4)
            .cast::<u32>())
            .read(),
            8u32,
        );
        arrId = (arrId
            | ((((((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((linkId) as i32) as isize * 28))
            .wrapping_add(19))
            .read()) as i32)
                .wrapping_mul(8i32)) as u32));
        return FacilityClassToPicIndex(
            ((((&raw const gUnionRoomFacilityClasses)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((arrId) as i32) as isize))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUnionRoomTrainerClass() -> u16 {
    unsafe {
        let mut linkId: u8 = 0u8;
        let mut arrId: u32 = 0u32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554432u32) != 0 {
            linkId = ((((((&raw mut gRecordedBattleMultiplayerId).cast::<u8>()).read()) as i32)
                ^ 1i32) as u8);
        } else {
            linkId = ((((GetMultiplayerId()) as i32) ^ 1i32) as u8);
        }
        arrId = crate::c::rem_u32(
            ((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((linkId) as i32) as isize * 28))
            .wrapping_add(4)
            .cast::<u32>())
            .read(),
            8u32,
        );
        arrId = (arrId
            | ((((((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((linkId) as i32) as isize * 28))
            .wrapping_add(19))
            .read()) as i32)
                .wrapping_mul(8i32)) as u32));
        return ((((((&raw const gFacilityClassToTrainerClass)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw const gUnionRoomFacilityClasses)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((arrId) as i32) as isize))
            .read()) as i32) as isize,
        ))
        .read()) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateEnemyEventMon() {
    unsafe {
        let mut species: i32 = ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32);
        let mut level: i32 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
        let mut itemId: i32 = ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32);
        ZeroEnemyPartyMons();
        CreateEventMon(
            ((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>(),
            ((species) as u16),
            ((level) as u8),
            32u8,
            0u8,
            0u32,
            0u8,
            0u32,
        );
        if (itemId) != 0 {
            let mut heldItem = crate::ffi::Align4([0u8; 2]);
            ((&raw mut heldItem).cast::<u8>()).write(((itemId) as u8));
            (((&raw mut heldItem).cast::<u8>()).wrapping_offset(1)).write(((itemId >> 8) as u8));
            SetMonData(
                ((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>(),
                12i32,
                (&raw mut heldItem).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CalculateBoxMonChecksum(boxMon: *mut u8) -> u16 {
    unsafe {
        let mut boxMon = boxMon;
        let mut checksum: u16 = 0u16;
        let mut substruct0: *mut u8 = GetSubstruct(boxMon, ((boxMon).cast::<u32>()).read(), 0u8);
        let mut substruct1: *mut u8 = GetSubstruct(boxMon, ((boxMon).cast::<u32>()).read(), 1u8);
        let mut substruct2: *mut u8 = GetSubstruct(boxMon, ((boxMon).cast::<u32>()).read(), 2u8);
        let mut substruct3: *mut u8 = GetSubstruct(boxMon, ((boxMon).cast::<u32>()).read(), 3u8);
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(12u32, 2u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    checksum = ((((checksum) as i32).wrapping_add(
                        (((((substruct0).cast::<u16>()).wrapping_offset((i) as isize)).read())
                            as i32),
                    )) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < ((crate::c::div_u32(12u32, 2u32)) as i32)) {
                    break 'l3;
                }
                'l4: {
                    checksum = ((((checksum) as i32).wrapping_add(
                        (((((substruct1).cast::<u16>()).wrapping_offset((i) as isize)).read())
                            as i32),
                    )) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < ((crate::c::div_u32(12u32, 2u32)) as i32)) {
                    break 'l5;
                }
                'l6: {
                    checksum = ((((checksum) as i32).wrapping_add(
                        (((((substruct2).cast::<u16>()).wrapping_offset((i) as isize)).read())
                            as i32),
                    )) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l7: loop {
                if !(i < ((crate::c::div_u32(12u32, 2u32)) as i32)) {
                    break 'l7;
                }
                'l8: {
                    checksum = ((((checksum) as i32).wrapping_add(
                        (((((substruct3).cast::<u16>()).wrapping_offset((i) as isize)).read())
                            as i32),
                    )) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        return checksum;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculateMonStats(mon: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut oldMaxHP: i32 = ((GetMonData3(mon, 58i32, core::ptr::null_mut())) as i32);
        let mut currentHP: i32 = ((GetMonData3(mon, 57i32, core::ptr::null_mut())) as i32);
        let mut hpIV: i32 = ((GetMonData3(mon, 39i32, core::ptr::null_mut())) as i32);
        let mut hpEV: i32 = ((GetMonData3(mon, 26i32, core::ptr::null_mut())) as i32);
        let mut attackIV: i32 = ((GetMonData3(mon, 40i32, core::ptr::null_mut())) as i32);
        let mut attackEV: i32 = ((GetMonData3(mon, 27i32, core::ptr::null_mut())) as i32);
        let mut defenseIV: i32 = ((GetMonData3(mon, 41i32, core::ptr::null_mut())) as i32);
        let mut defenseEV: i32 = ((GetMonData3(mon, 28i32, core::ptr::null_mut())) as i32);
        let mut speedIV: i32 = ((GetMonData3(mon, 42i32, core::ptr::null_mut())) as i32);
        let mut speedEV: i32 = ((GetMonData3(mon, 29i32, core::ptr::null_mut())) as i32);
        let mut spAttackIV: i32 = ((GetMonData3(mon, 43i32, core::ptr::null_mut())) as i32);
        let mut spAttackEV: i32 = ((GetMonData3(mon, 30i32, core::ptr::null_mut())) as i32);
        let mut spDefenseIV: i32 = ((GetMonData3(mon, 44i32, core::ptr::null_mut())) as i32);
        let mut spDefenseEV: i32 = ((GetMonData3(mon, 31i32, core::ptr::null_mut())) as i32);
        let mut species: u16 = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
        let mut level: i32 = ((GetLevelFromMonExp(mon)) as i32);
        let mut newMaxHP: i32 = 0i32;
        SetMonData(mon, 56i32, (&raw mut level).cast::<u8>());
        if ((species) as i32) == 303i32 {
            newMaxHP = 1i32;
        } else {
            let mut n: i32 = ((2i32).wrapping_mul(
                ((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .read()) as i32),
            ))
            .wrapping_add(hpIV);
            newMaxHP = ((crate::c::div_i32(
                ((n).wrapping_add(crate::c::div_i32(hpEV, 4i32))).wrapping_mul(level),
                100i32,
            ))
            .wrapping_add(level))
            .wrapping_add(10i32);
        }
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(35))
            .write((((newMaxHP).wrapping_sub(oldMaxHP)) as u8));
        if (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(35)).read()) as i32) == 0i32 {
            (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(35)).write(1u8);
        }
        SetMonData(mon, 58i32, (&raw mut newMaxHP).cast::<u8>());
        {
            let mut baseStat: u8 = (((((&raw const gSpeciesInfo).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(1))
            .read();
            let mut n: i32 = (crate::c::div_i32(
                ((((2i32).wrapping_mul(((baseStat) as i32))).wrapping_add(attackIV))
                    .wrapping_add(crate::c::div_i32(attackEV, 4i32)))
                .wrapping_mul(level),
                100i32,
            ))
            .wrapping_add(5i32);
            let mut nature: u8 = GetNature(mon);
            n = ((ModifyStatByNature(nature, ((n) as u16), 1u8)) as i32);
            SetMonData(mon, 59i32, (&raw mut n).cast::<u8>());
        }
        {
            let mut baseStat: u8 = (((((&raw const gSpeciesInfo).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(2))
            .read();
            let mut n: i32 = (crate::c::div_i32(
                ((((2i32).wrapping_mul(((baseStat) as i32))).wrapping_add(defenseIV))
                    .wrapping_add(crate::c::div_i32(defenseEV, 4i32)))
                .wrapping_mul(level),
                100i32,
            ))
            .wrapping_add(5i32);
            let mut nature: u8 = GetNature(mon);
            n = ((ModifyStatByNature(nature, ((n) as u16), 2u8)) as i32);
            SetMonData(mon, 60i32, (&raw mut n).cast::<u8>());
        }
        {
            let mut baseStat: u8 = (((((&raw const gSpeciesInfo).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(3))
            .read();
            let mut n: i32 = (crate::c::div_i32(
                ((((2i32).wrapping_mul(((baseStat) as i32))).wrapping_add(speedIV))
                    .wrapping_add(crate::c::div_i32(speedEV, 4i32)))
                .wrapping_mul(level),
                100i32,
            ))
            .wrapping_add(5i32);
            let mut nature: u8 = GetNature(mon);
            n = ((ModifyStatByNature(nature, ((n) as u16), 3u8)) as i32);
            SetMonData(mon, 61i32, (&raw mut n).cast::<u8>());
        }
        {
            let mut baseStat: u8 = (((((&raw const gSpeciesInfo).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(4))
            .read();
            let mut n: i32 = (crate::c::div_i32(
                ((((2i32).wrapping_mul(((baseStat) as i32))).wrapping_add(spAttackIV))
                    .wrapping_add(crate::c::div_i32(spAttackEV, 4i32)))
                .wrapping_mul(level),
                100i32,
            ))
            .wrapping_add(5i32);
            let mut nature: u8 = GetNature(mon);
            n = ((ModifyStatByNature(nature, ((n) as u16), 4u8)) as i32);
            SetMonData(mon, 62i32, (&raw mut n).cast::<u8>());
        }
        {
            let mut baseStat: u8 = (((((&raw const gSpeciesInfo).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(5))
            .read();
            let mut n: i32 = (crate::c::div_i32(
                ((((2i32).wrapping_mul(((baseStat) as i32))).wrapping_add(spDefenseIV))
                    .wrapping_add(crate::c::div_i32(spDefenseEV, 4i32)))
                .wrapping_mul(level),
                100i32,
            ))
            .wrapping_add(5i32);
            let mut nature: u8 = GetNature(mon);
            n = ((ModifyStatByNature(nature, ((n) as u16), 5u8)) as i32);
            SetMonData(mon, 63i32, (&raw mut n).cast::<u8>());
        }
        if ((species) as i32) == 303i32 {
            if (currentHP != 0i32) || (oldMaxHP == 0i32) {
                currentHP = 1i32;
            } else {
                return;
            }
        } else {
            if (currentHP == 0i32) && (oldMaxHP == 0i32) {
                currentHP = newMaxHP;
            } else {
                if currentHP != 0i32 {
                    currentHP = (currentHP).wrapping_add((newMaxHP).wrapping_sub(oldMaxHP));
                } else {
                    return;
                }
            }
        }
        SetMonData(mon, 57i32, (&raw mut currentHP).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BoxMonToMon(src: *mut u8, dest: *mut u8) {
    unsafe {
        let mut src = src;
        let mut dest = dest;
        let mut value: u32 = 0u32;
        (dest)
            .cast::<crate::c::Rec4<80>>()
            .write_unaligned(src.cast::<crate::c::Rec4<80>>().read_unaligned());
        SetMonData(dest, 55i32, (&raw mut value).cast::<u8>());
        SetMonData(dest, 57i32, (&raw mut value).cast::<u8>());
        SetMonData(dest, 58i32, (&raw mut value).cast::<u8>());
        value = 255u32;
        SetMonData(dest, 64i32, (&raw mut value).cast::<u8>());
        CalculateMonStats(dest);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLevelFromMonExp(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut species: u16 = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
        let mut exp: u32 = GetMonData3(mon, 25i32, core::ptr::null_mut());
        let mut level: i32 = 1i32;
        'l1: loop {
            if !((level <= 100i32)
                && (((((((&raw const gExperienceTables).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(19))
                        .read()) as i32) as isize
                            * 404,
                    ))
                .cast::<u32>())
                .wrapping_offset((level) as isize))
                .read()
                    <= exp))
            {
                break 'l1;
            }
            level = (level).wrapping_add(1);
        }
        return (((level).wrapping_sub(1i32)) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLevelFromBoxMonExp(boxMon: *mut u8) -> u8 {
    unsafe {
        let mut boxMon = boxMon;
        let mut species: u16 = ((GetBoxMonData3(boxMon, 11i32, core::ptr::null_mut())) as u16);
        let mut exp: u32 = GetBoxMonData3(boxMon, 25i32, core::ptr::null_mut());
        let mut level: i32 = 1i32;
        'l1: loop {
            if !((level <= 100i32)
                && (((((((&raw const gExperienceTables).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(19))
                        .read()) as i32) as isize
                            * 404,
                    ))
                .cast::<u32>())
                .wrapping_offset((level) as isize))
                .read()
                    <= exp))
            {
                break 'l1;
            }
            level = (level).wrapping_add(1);
        }
        return (((level).wrapping_sub(1i32)) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveMoveToMon(mon: *mut u8, r#move: u16) -> u16 {
    unsafe {
        let mut mon = mon;
        let mut r#move = r#move;
        return GiveMoveToBoxMon((mon), r#move);
    }
}
pub(crate) unsafe extern "C" fn GiveMoveToBoxMon(boxMon: *mut u8, r#move: u16) -> u16 {
    unsafe {
        let mut boxMon = boxMon;
        let mut r#move = r#move;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut existingMove: u16 =
                        ((GetBoxMonData3(boxMon, (13i32).wrapping_add(i), core::ptr::null_mut()))
                            as u16);
                    if ((existingMove) as i32) == 0i32 {
                        SetBoxMonData(
                            boxMon,
                            (13i32).wrapping_add(i),
                            (&raw mut r#move).cast::<u8>(),
                        );
                        SetBoxMonData(
                            boxMon,
                            (17i32).wrapping_add(i),
                            ((((&raw const gBattleMoves).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(((r#move) as i32) as isize * 12))
                            .wrapping_add(4),
                        );
                        return r#move;
                    }
                    if ((existingMove) as i32) == ((r#move) as i32) {
                        return 65534u16;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 65535u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveMoveToBattleMon(mon: *mut u8, r#move: u16) -> u16 {
    unsafe {
        let mut mon = mon;
        let mut r#move = r#move;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((mon).wrapping_add(12)).cast::<u16>()).wrapping_offset((i) as isize))
                        .read()) as i32)
                        == 0i32
                    {
                        ((((mon).wrapping_add(12)).cast::<u16>()).wrapping_offset((i) as isize))
                            .write(r#move);
                        ((((mon).wrapping_add(36)).cast::<u8>()).wrapping_offset((i) as isize))
                            .write(
                                (((((&raw const gBattleMoves).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((r#move) as i32) as isize * 12))
                                .wrapping_add(4))
                                .read(),
                            );
                        return r#move;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 65535u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMonMoveSlot(mon: *mut u8, r#move: u16, slot: u8) {
    unsafe {
        let mut mon = mon;
        let mut r#move = r#move;
        let mut slot = slot;
        SetMonData(
            mon,
            (13i32).wrapping_add(((slot) as i32)),
            (&raw mut r#move).cast::<u8>(),
        );
        SetMonData(
            mon,
            (17i32).wrapping_add(((slot) as i32)),
            ((((&raw const gBattleMoves).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(4),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattleMonMoveSlot(mon: *mut u8, r#move: u16, slot: u8) {
    unsafe {
        let mut mon = mon;
        let mut r#move = r#move;
        let mut slot = slot;
        ((((mon).wrapping_add(12)).cast::<u16>()).wrapping_offset(((slot) as i32) as isize))
            .write(r#move);
        ((((mon).wrapping_add(36)).cast::<u8>()).wrapping_offset(((slot) as i32) as isize)).write(
            (((((&raw const gBattleMoves).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(4))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveMonInitialMoveset(mon: *mut u8) {
    unsafe {
        let mut mon = mon;
        GiveBoxMonInitialMoveset((mon));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveBoxMonInitialMoveset(boxMon: *mut u8) {
    unsafe {
        let mut boxMon = boxMon;
        let mut species: u16 = ((GetBoxMonData3(boxMon, 11i32, core::ptr::null_mut())) as u16);
        let mut level: i32 = ((GetLevelFromBoxMonExp(boxMon)) as i32);
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((((((((&raw const gLevelUpLearnsets)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((species) as i32) as isize))
                .read())
                .wrapping_offset((i) as isize))
                .read()) as i32)
                    != 65535i32)
                {
                    break 'l1;
                }
                'l2: {
                    let mut moveLevel: u16 = 0u16;
                    let mut r#move: u16 = 0u16;
                    moveLevel = ((((((((((&raw const gLevelUpLearnsets)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u16>())
                    .cast::<*mut u16>())
                    .wrapping_offset(((species) as i32) as isize))
                    .read())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        & 65024i32) as u16);
                    if ((moveLevel) as i32) > (level << 9) {
                        break 'l1;
                    }
                    r#move = ((((((((((&raw const gLevelUpLearnsets)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u16>())
                    .cast::<*mut u16>())
                    .wrapping_offset(((species) as i32) as isize))
                    .read())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        & 511i32) as u16);
                    if ((GiveMoveToBoxMon(boxMon, r#move)) as i32) == 65535i32 {
                        DeleteFirstMoveAndGiveMoveToBoxMon(boxMon, r#move);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonTryLearningNewMove(mon: *mut u8, firstMove: u8) -> u16 {
    unsafe {
        let mut mon = mon;
        let mut firstMove = firstMove;
        let mut retVal: u32 = 0u32;
        let mut species: u16 = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
        let mut level: u8 = ((GetMonData3(mon, 56i32, core::ptr::null_mut())) as u8);
        if (firstMove) != 0 {
            ((&raw mut sLearningMoveTableID).cast::<u8>().cast::<u8>()).write(0u8);
            'l1: loop {
                if !((((((((((&raw const gLevelUpLearnsets)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((species) as i32) as isize))
                .read())
                .wrapping_offset(
                    ((((&raw mut sLearningMoveTableID).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize,
                ))
                .read()) as i32)
                    & 65024i32)
                    != (((level) as i32) << 9))
                {
                    break 'l1;
                }
                let __p1 = (&raw mut sLearningMoveTableID).cast::<u8>().cast::<u8>();
                (__p1).write(((__p1).read()).wrapping_add(1));
                if ((((((((&raw const gLevelUpLearnsets)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((species) as i32) as isize))
                .read())
                .wrapping_offset(
                    ((((&raw mut sLearningMoveTableID).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize,
                ))
                .read()) as i32)
                    == 65535i32
                {
                    return 0u16;
                }
            }
        }
        if (((((((((&raw const gLevelUpLearnsets)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u16>())
        .cast::<*mut u16>())
        .wrapping_offset(((species) as i32) as isize))
        .read())
        .wrapping_offset(
            ((((&raw mut sLearningMoveTableID).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .read()) as i32)
            & 65024i32)
            == (((level) as i32) << 9)
        {
            ((&raw mut gMoveToLearn).cast::<u16>()).write(
                ((((((((((&raw const gLevelUpLearnsets)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u16>())
                .cast::<*mut u16>())
                .wrapping_offset(((species) as i32) as isize))
                .read())
                .wrapping_offset(
                    ((((&raw mut sLearningMoveTableID).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize,
                ))
                .read()) as i32)
                    & 511i32) as u16),
            );
            let __p2 = (&raw mut sLearningMoveTableID).cast::<u8>().cast::<u8>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            retVal = ((GiveMoveToMon(mon, ((&raw mut gMoveToLearn).cast::<u16>()).read())) as u32);
        }
        return ((retVal) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DeleteFirstMoveAndGiveMoveToMon(mon: *mut u8, r#move: u16) {
    unsafe {
        let mut mon = mon;
        let mut r#move = r#move;
        let mut i: i32 = 0i32;
        let mut moves = crate::ffi::Align4([0u8; 8]);
        let mut pp = crate::ffi::Align4([0u8; 4]);
        let mut ppBonuses: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut moves).cast::<u16>()).wrapping_offset((i) as isize)).write(
                        ((GetMonData3(mon, (14i32).wrapping_add(i), core::ptr::null_mut())) as u16),
                    );
                    (((&raw mut pp).cast::<u8>()).wrapping_offset((i) as isize)).write(
                        ((GetMonData3(mon, (18i32).wrapping_add(i), core::ptr::null_mut())) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ppBonuses = ((GetMonData3(mon, 21i32, core::ptr::null_mut())) as u8);
        ppBonuses = ((((ppBonuses) as i32) >> 2) as u8);
        (((&raw mut moves).cast::<u16>()).wrapping_offset(3)).write(r#move);
        (((&raw mut pp).cast::<u8>()).wrapping_offset(3)).write(
            (((((&raw const gBattleMoves).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(4))
            .read(),
        );
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    SetMonData(
                        mon,
                        (13i32).wrapping_add(i),
                        (((&raw mut moves).cast::<u16>()).wrapping_offset((i) as isize))
                            .cast::<u8>(),
                    );
                    SetMonData(
                        mon,
                        (17i32).wrapping_add(i),
                        ((&raw mut pp).cast::<u8>()).wrapping_offset((i) as isize),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        SetMonData(mon, 21i32, &raw mut ppBonuses);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DeleteFirstMoveAndGiveMoveToBoxMon(boxMon: *mut u8, r#move: u16) {
    unsafe {
        let mut boxMon = boxMon;
        let mut r#move = r#move;
        let mut i: i32 = 0i32;
        let mut moves = crate::ffi::Align4([0u8; 8]);
        let mut pp = crate::ffi::Align4([0u8; 4]);
        let mut ppBonuses: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut moves).cast::<u16>()).wrapping_offset((i) as isize)).write(
                        ((GetBoxMonData3(boxMon, (14i32).wrapping_add(i), core::ptr::null_mut()))
                            as u16),
                    );
                    (((&raw mut pp).cast::<u8>()).wrapping_offset((i) as isize)).write(
                        ((GetBoxMonData3(boxMon, (18i32).wrapping_add(i), core::ptr::null_mut()))
                            as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ppBonuses = ((GetBoxMonData3(boxMon, 21i32, core::ptr::null_mut())) as u8);
        ppBonuses = ((((ppBonuses) as i32) >> 2) as u8);
        (((&raw mut moves).cast::<u16>()).wrapping_offset(3)).write(r#move);
        (((&raw mut pp).cast::<u8>()).wrapping_offset(3)).write(
            (((((&raw const gBattleMoves).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(4))
            .read(),
        );
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    SetBoxMonData(
                        boxMon,
                        (13i32).wrapping_add(i),
                        (((&raw mut moves).cast::<u16>()).wrapping_offset((i) as isize))
                            .cast::<u8>(),
                    );
                    SetBoxMonData(
                        boxMon,
                        (17i32).wrapping_add(i),
                        ((&raw mut pp).cast::<u8>()).wrapping_offset((i) as isize),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        SetBoxMonData(boxMon, 21i32, &raw mut ppBonuses);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculateBaseDamage(
    attacker: *mut u8,
    defender: *mut u8,
    r#move: u32,
    sideStatus: u16,
    powerOverride: u16,
    typeOverride: u8,
    battlerIdAtk: u8,
    battlerIdDef: u8,
) -> i32 {
    unsafe {
        let mut attacker = attacker;
        let mut defender = defender;
        let mut r#move = r#move;
        let mut sideStatus = sideStatus;
        let mut powerOverride = powerOverride;
        let mut typeOverride = typeOverride;
        let mut battlerIdAtk = battlerIdAtk;
        let mut battlerIdDef = battlerIdDef;
        let mut i: u32 = 0u32;
        let mut damage: i32 = 0i32;
        let mut damageHelper: i32 = 0i32;
        let mut r#type: u8 = 0u8;
        let mut attack: u16 = 0u16;
        let mut defense: u16 = 0u16;
        let mut spAttack: u16 = 0u16;
        let mut spDefense: u16 = 0u16;
        let mut defenderHoldEffect: u8 = 0u8;
        let mut defenderHoldEffectParam: u8 = 0u8;
        let mut attackerHoldEffect: u8 = 0u8;
        let mut attackerHoldEffectParam: u8 = 0u8;
        if !((powerOverride) != 0) {
            ((&raw mut gBattleMovePower).cast::<u16>()).write(
                (((((((&raw const gBattleMoves).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 12))
                .wrapping_add(1))
                .read()) as u16),
            );
        } else {
            ((&raw mut gBattleMovePower).cast::<u16>()).write(powerOverride);
        }
        if !((typeOverride) != 0) {
            r#type = (((((&raw const gBattleMoves).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(2))
            .read();
        } else {
            r#type = ((((typeOverride) as i32) & 63i32) as u8);
        }
        attack = ((attacker).wrapping_add(2).cast::<u16>()).read();
        defense = ((defender).wrapping_add(4).cast::<u16>()).read();
        spAttack = ((attacker).wrapping_add(8).cast::<u16>()).read();
        spDefense = ((defender).wrapping_add(10).cast::<u16>()).read();
        if ((((attacker).wrapping_add(46).cast::<u16>()).read()) as i32) == 175i32 {
            attackerHoldEffect = ((((&raw mut gEnigmaBerries).cast::<u8>())
                .wrapping_offset(((battlerIdAtk) as i32) as isize * 28))
            .wrapping_add(7))
            .read();
            attackerHoldEffectParam = ((((&raw mut gEnigmaBerries).cast::<u8>())
                .wrapping_offset(((battlerIdAtk) as i32) as isize * 28))
            .wrapping_add(26))
            .read();
        } else {
            attackerHoldEffect =
                GetItemHoldEffect(((attacker).wrapping_add(46).cast::<u16>()).read());
            attackerHoldEffectParam =
                GetItemHoldEffectParam(((attacker).wrapping_add(46).cast::<u16>()).read());
        }
        if ((((defender).wrapping_add(46).cast::<u16>()).read()) as i32) == 175i32 {
            defenderHoldEffect = ((((&raw mut gEnigmaBerries).cast::<u8>())
                .wrapping_offset(((battlerIdDef) as i32) as isize * 28))
            .wrapping_add(7))
            .read();
            defenderHoldEffectParam = ((((&raw mut gEnigmaBerries).cast::<u8>())
                .wrapping_offset(((battlerIdDef) as i32) as isize * 28))
            .wrapping_add(26))
            .read();
        } else {
            defenderHoldEffect =
                GetItemHoldEffect(((defender).wrapping_add(46).cast::<u16>()).read());
            defenderHoldEffectParam =
                GetItemHoldEffectParam(((defender).wrapping_add(46).cast::<u16>()).read());
        }
        if (((((attacker).wrapping_add(32)).read()) as i32) == 37i32)
            || (((((attacker).wrapping_add(32)).read()) as i32) == 74i32)
        {
            attack = ((((attack) as i32).wrapping_mul(2i32)) as u16);
        }
        if (ShouldGetStatBadgeBoost(2151u16, battlerIdAtk)) != 0 {
            attack = ((crate::c::div_i32((110i32).wrapping_mul(((attack) as i32)), 100i32)) as u16);
        }
        if (ShouldGetStatBadgeBoost(2155u16, battlerIdDef)) != 0 {
            defense =
                ((crate::c::div_i32((110i32).wrapping_mul(((defense) as i32)), 100i32)) as u16);
        }
        if (ShouldGetStatBadgeBoost(2157u16, battlerIdAtk)) != 0 {
            spAttack =
                ((crate::c::div_i32((110i32).wrapping_mul(((spAttack) as i32)), 100i32)) as u16);
        }
        if (ShouldGetStatBadgeBoost(2157u16, battlerIdDef)) != 0 {
            spDefense =
                ((crate::c::div_i32((110i32).wrapping_mul(((spDefense) as i32)), 100i32)) as u16);
        }
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(34u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((attackerHoldEffect) as i32)
                        == (((((((&raw const sHoldEffectToType).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .read()) as i32))
                        && (((r#type) as i32)
                            == ((((((((&raw const sHoldEffectToType).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 2))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32))
                    {
                        if ((r#type) as i32) < 9i32 {
                            attack = ((crate::c::div_i32(
                                ((attack) as i32).wrapping_mul(
                                    ((attackerHoldEffectParam) as i32).wrapping_add(100i32),
                                ),
                                100i32,
                            )) as u16);
                        } else {
                            spAttack = ((crate::c::div_i32(
                                ((spAttack) as i32).wrapping_mul(
                                    ((attackerHoldEffectParam) as i32).wrapping_add(100i32),
                                ),
                                100i32,
                            )) as u16);
                        }
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((attackerHoldEffect) as i32) == 29i32 {
            attack = ((crate::c::div_i32((150i32).wrapping_mul(((attack) as i32)), 100i32)) as u16);
        }
        if ((((attackerHoldEffect) as i32) == 34i32)
            && (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32) != 0)))
            && ((((((attacker).cast::<u16>()).read()) as i32) == 407i32)
                || (((((attacker).cast::<u16>()).read()) as i32) == 408i32))
        {
            spAttack =
                ((crate::c::div_i32((150i32).wrapping_mul(((spAttack) as i32)), 100i32)) as u16);
        }
        if ((((defenderHoldEffect) as i32) == 34i32)
            && (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32) != 0)))
            && ((((((defender).cast::<u16>()).read()) as i32) == 407i32)
                || (((((defender).cast::<u16>()).read()) as i32) == 408i32))
        {
            spDefense =
                ((crate::c::div_i32((150i32).wrapping_mul(((spDefense) as i32)), 100i32)) as u16);
        }
        if (((attackerHoldEffect) as i32) == 35i32)
            && (((((attacker).cast::<u16>()).read()) as i32) == 373i32)
        {
            spAttack = ((((spAttack) as i32).wrapping_mul(2i32)) as u16);
        }
        if (((defenderHoldEffect) as i32) == 36i32)
            && (((((defender).cast::<u16>()).read()) as i32) == 373i32)
        {
            spDefense = ((((spDefense) as i32).wrapping_mul(2i32)) as u16);
        }
        if (((attackerHoldEffect) as i32) == 45i32)
            && (((((attacker).cast::<u16>()).read()) as i32) == 25i32)
        {
            spAttack = ((((spAttack) as i32).wrapping_mul(2i32)) as u16);
        }
        if (((defenderHoldEffect) as i32) == 64i32)
            && (((((defender).cast::<u16>()).read()) as i32) == 132i32)
        {
            defense = ((((defense) as i32).wrapping_mul(2i32)) as u16);
        }
        if (((attackerHoldEffect) as i32) == 65i32)
            && ((((((attacker).cast::<u16>()).read()) as i32) == 104i32)
                || (((((attacker).cast::<u16>()).read()) as i32) == 105i32))
        {
            attack = ((((attack) as i32).wrapping_mul(2i32)) as u16);
        }
        if (((((defender).wrapping_add(32)).read()) as i32) == 47i32)
            && ((((r#type) as i32) == 10i32) || (((r#type) as i32) == 15i32))
        {
            spAttack = ((crate::c::div_i32(((spAttack) as i32), 2i32)) as u16);
        }
        if ((((attacker).wrapping_add(32)).read()) as i32) == 55i32 {
            attack = ((crate::c::div_i32((150i32).wrapping_mul(((attack) as i32)), 100i32)) as u16);
        }
        if (((((attacker).wrapping_add(32)).read()) as i32) == 57i32)
            && ((AbilityBattleEffects(14u8, 0u8, 58u8, 0u8, 0u16)) != 0)
        {
            spAttack =
                ((crate::c::div_i32((150i32).wrapping_mul(((spAttack) as i32)), 100i32)) as u16);
        }
        if (((((attacker).wrapping_add(32)).read()) as i32) == 58i32)
            && ((AbilityBattleEffects(14u8, 0u8, 57u8, 0u8, 0u16)) != 0)
        {
            spAttack =
                ((crate::c::div_i32((150i32).wrapping_mul(((spAttack) as i32)), 100i32)) as u16);
        }
        if (((((attacker).wrapping_add(32)).read()) as i32) == 62i32)
            && ((((attacker).wrapping_add(76).cast::<u32>()).read()) != 0)
        {
            attack = ((crate::c::div_i32((150i32).wrapping_mul(((attack) as i32)), 100i32)) as u16);
        }
        if (((((defender).wrapping_add(32)).read()) as i32) == 63i32)
            && ((((defender).wrapping_add(76).cast::<u32>()).read()) != 0)
        {
            defense =
                ((crate::c::div_i32((150i32).wrapping_mul(((defense) as i32)), 100i32)) as u16);
        }
        if (((r#type) as i32) == 13i32)
            && ((AbilityBattleEffects(14u8, 0u8, 0u8, 253u8, 0u16)) != 0)
        {
            let __p1 = (&raw mut gBattleMovePower).cast::<u16>();
            (__p1).write(((crate::c::div_i32((((__p1).read()) as i32), 2i32)) as u16));
        }
        if (((r#type) as i32) == 10i32)
            && ((AbilityBattleEffects(14u8, 0u8, 0u8, 254u8, 0u16)) != 0)
        {
            let __p2 = (&raw mut gBattleMovePower).cast::<u16>();
            (__p2).write(((crate::c::div_i32((((__p2).read()) as i32), 2i32)) as u16));
        }
        if ((((r#type) as i32) == 12i32)
            && (((((attacker).wrapping_add(32)).read()) as i32) == 65i32))
            && (((((attacker).wrapping_add(40).cast::<u16>()).read()) as i32)
                <= crate::c::div_i32(
                    ((((attacker).wrapping_add(44).cast::<u16>()).read()) as i32),
                    3i32,
                ))
        {
            ((&raw mut gBattleMovePower).cast::<u16>()).write(
                ((crate::c::div_i32(
                    (150i32).wrapping_mul(
                        ((((&raw mut gBattleMovePower).cast::<u16>()).read()) as i32),
                    ),
                    100i32,
                )) as u16),
            );
        }
        if ((((r#type) as i32) == 10i32)
            && (((((attacker).wrapping_add(32)).read()) as i32) == 66i32))
            && (((((attacker).wrapping_add(40).cast::<u16>()).read()) as i32)
                <= crate::c::div_i32(
                    ((((attacker).wrapping_add(44).cast::<u16>()).read()) as i32),
                    3i32,
                ))
        {
            ((&raw mut gBattleMovePower).cast::<u16>()).write(
                ((crate::c::div_i32(
                    (150i32).wrapping_mul(
                        ((((&raw mut gBattleMovePower).cast::<u16>()).read()) as i32),
                    ),
                    100i32,
                )) as u16),
            );
        }
        if ((((r#type) as i32) == 11i32)
            && (((((attacker).wrapping_add(32)).read()) as i32) == 67i32))
            && (((((attacker).wrapping_add(40).cast::<u16>()).read()) as i32)
                <= crate::c::div_i32(
                    ((((attacker).wrapping_add(44).cast::<u16>()).read()) as i32),
                    3i32,
                ))
        {
            ((&raw mut gBattleMovePower).cast::<u16>()).write(
                ((crate::c::div_i32(
                    (150i32).wrapping_mul(
                        ((((&raw mut gBattleMovePower).cast::<u16>()).read()) as i32),
                    ),
                    100i32,
                )) as u16),
            );
        }
        if ((((r#type) as i32) == 6i32)
            && (((((attacker).wrapping_add(32)).read()) as i32) == 68i32))
            && (((((attacker).wrapping_add(40).cast::<u16>()).read()) as i32)
                <= crate::c::div_i32(
                    ((((attacker).wrapping_add(44).cast::<u16>()).read()) as i32),
                    3i32,
                ))
        {
            ((&raw mut gBattleMovePower).cast::<u16>()).write(
                ((crate::c::div_i32(
                    (150i32).wrapping_mul(
                        ((((&raw mut gBattleMovePower).cast::<u16>()).read()) as i32),
                    ),
                    100i32,
                )) as u16),
            );
        }
        if ((((((&raw const gBattleMoves).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) as isize * 12,
        ))
        .read()) as i32)
            == 7i32
        {
            defense = ((crate::c::div_i32(((defense) as i32), 2i32)) as u16);
        }
        if ((r#type) as i32) < 9i32 {
            if ((((&raw mut gCritMultiplier).cast::<u8>()).read()) as i32) == 2i32 {
                if ((((((attacker).wrapping_add(24)).cast::<i8>()).wrapping_offset(1)).read())
                    as i32)
                    > 6i32
                {
                    damage = ((attack) as i32).wrapping_mul(
                        (((((((&raw const gStatStageRatios).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((attacker).wrapping_add(24)).cast::<i8>()).wrapping_offset(1))
                                .read()) as i32) as isize
                                * 2,
                        ))
                        .cast::<u8>())
                        .read()) as i32),
                    );
                    damage = crate::c::div_i32(
                        damage,
                        ((((((((&raw const gStatStageRatios).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((attacker).wrapping_add(24)).cast::<i8>()).wrapping_offset(1))
                                .read()) as i32) as isize
                                * 2,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    );
                } else {
                    damage = ((attack) as i32);
                }
            } else {
                damage = ((attack) as i32).wrapping_mul(
                    (((((((&raw const gStatStageRatios).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((attacker).wrapping_add(24)).cast::<i8>()).wrapping_offset(1))
                                .read()) as i32) as isize
                                * 2,
                        ))
                    .cast::<u8>())
                    .read()) as i32),
                );
                damage = crate::c::div_i32(
                    damage,
                    ((((((((&raw const gStatStageRatios).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((attacker).wrapping_add(24)).cast::<i8>()).wrapping_offset(1))
                                .read()) as i32) as isize
                                * 2,
                        ))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32),
                );
            }
            damage = (damage)
                .wrapping_mul(((((&raw mut gBattleMovePower).cast::<u16>()).read()) as i32));
            damage = (damage).wrapping_mul(
                (crate::c::div_i32(
                    (2i32).wrapping_mul(((((attacker).wrapping_add(42)).read()) as i32)),
                    5i32,
                ))
                .wrapping_add(2i32),
            );
            if ((((&raw mut gCritMultiplier).cast::<u8>()).read()) as i32) == 2i32 {
                if ((((((defender).wrapping_add(24)).cast::<i8>()).wrapping_offset(2)).read())
                    as i32)
                    < 6i32
                {
                    damageHelper = ((defense) as i32).wrapping_mul(
                        (((((((&raw const gStatStageRatios).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((defender).wrapping_add(24)).cast::<i8>()).wrapping_offset(2))
                                .read()) as i32) as isize
                                * 2,
                        ))
                        .cast::<u8>())
                        .read()) as i32),
                    );
                    damageHelper = crate::c::div_i32(
                        damageHelper,
                        ((((((((&raw const gStatStageRatios).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((defender).wrapping_add(24)).cast::<i8>()).wrapping_offset(2))
                                .read()) as i32) as isize
                                * 2,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    );
                } else {
                    damageHelper = ((defense) as i32);
                }
            } else {
                damageHelper = ((defense) as i32).wrapping_mul(
                    (((((((&raw const gStatStageRatios).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((defender).wrapping_add(24)).cast::<i8>()).wrapping_offset(2))
                                .read()) as i32) as isize
                                * 2,
                        ))
                    .cast::<u8>())
                    .read()) as i32),
                );
                damageHelper = crate::c::div_i32(
                    damageHelper,
                    ((((((((&raw const gStatStageRatios).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((defender).wrapping_add(24)).cast::<i8>()).wrapping_offset(2))
                                .read()) as i32) as isize
                                * 2,
                        ))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32),
                );
            }
            damage = crate::c::div_i32(damage, damageHelper);
            damage = crate::c::div_i32(damage, 50i32);
            if ((((attacker).wrapping_add(76).cast::<u32>()).read() & 16u32) != 0)
                && (((((attacker).wrapping_add(32)).read()) as i32) != 62i32)
            {
                damage = crate::c::div_i32(damage, 2i32);
            }
            if ((((sideStatus) as i32) & 1i32) != 0)
                && (((((&raw mut gCritMultiplier).cast::<u8>()).read()) as i32) == 1i32)
            {
                if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0)
                    && (((CountAliveMonsInBattle(2u8)) as i32) == 2i32)
                {
                    damage = (2i32).wrapping_mul(crate::c::div_i32(damage, 3i32));
                } else {
                    damage = crate::c::div_i32(damage, 2i32);
                }
            }
            if (((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0)
                && ((((((((&raw const gBattleMoves).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 12))
                .wrapping_add(6))
                .read()) as i32)
                    == 8i32))
                && (((CountAliveMonsInBattle(2u8)) as i32) == 2i32)
            {
                damage = crate::c::div_i32(damage, 2i32);
            }
            if damage == 0i32 {
                damage = 1i32;
            }
        }
        if ((r#type) as i32) == 9i32 {
            damage = 0i32;
        }
        if ((r#type) as i32) > 9i32 {
            if ((((&raw mut gCritMultiplier).cast::<u8>()).read()) as i32) == 2i32 {
                if ((((((attacker).wrapping_add(24)).cast::<i8>()).wrapping_offset(4)).read())
                    as i32)
                    > 6i32
                {
                    damage = ((spAttack) as i32).wrapping_mul(
                        (((((((&raw const gStatStageRatios).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((attacker).wrapping_add(24)).cast::<i8>()).wrapping_offset(4))
                                .read()) as i32) as isize
                                * 2,
                        ))
                        .cast::<u8>())
                        .read()) as i32),
                    );
                    damage = crate::c::div_i32(
                        damage,
                        ((((((((&raw const gStatStageRatios).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((attacker).wrapping_add(24)).cast::<i8>()).wrapping_offset(4))
                                .read()) as i32) as isize
                                * 2,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    );
                } else {
                    damage = ((spAttack) as i32);
                }
            } else {
                damage = ((spAttack) as i32).wrapping_mul(
                    (((((((&raw const gStatStageRatios).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((attacker).wrapping_add(24)).cast::<i8>()).wrapping_offset(4))
                                .read()) as i32) as isize
                                * 2,
                        ))
                    .cast::<u8>())
                    .read()) as i32),
                );
                damage = crate::c::div_i32(
                    damage,
                    ((((((((&raw const gStatStageRatios).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((attacker).wrapping_add(24)).cast::<i8>()).wrapping_offset(4))
                                .read()) as i32) as isize
                                * 2,
                        ))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32),
                );
            }
            damage = (damage)
                .wrapping_mul(((((&raw mut gBattleMovePower).cast::<u16>()).read()) as i32));
            damage = (damage).wrapping_mul(
                (crate::c::div_i32(
                    (2i32).wrapping_mul(((((attacker).wrapping_add(42)).read()) as i32)),
                    5i32,
                ))
                .wrapping_add(2i32),
            );
            if ((((&raw mut gCritMultiplier).cast::<u8>()).read()) as i32) == 2i32 {
                if ((((((defender).wrapping_add(24)).cast::<i8>()).wrapping_offset(5)).read())
                    as i32)
                    < 6i32
                {
                    damageHelper = ((spDefense) as i32).wrapping_mul(
                        (((((((&raw const gStatStageRatios).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((defender).wrapping_add(24)).cast::<i8>()).wrapping_offset(5))
                                .read()) as i32) as isize
                                * 2,
                        ))
                        .cast::<u8>())
                        .read()) as i32),
                    );
                    damageHelper = crate::c::div_i32(
                        damageHelper,
                        ((((((((&raw const gStatStageRatios).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((defender).wrapping_add(24)).cast::<i8>()).wrapping_offset(5))
                                .read()) as i32) as isize
                                * 2,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    );
                } else {
                    damageHelper = ((spDefense) as i32);
                }
            } else {
                damageHelper = ((spDefense) as i32).wrapping_mul(
                    (((((((&raw const gStatStageRatios).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((defender).wrapping_add(24)).cast::<i8>()).wrapping_offset(5))
                                .read()) as i32) as isize
                                * 2,
                        ))
                    .cast::<u8>())
                    .read()) as i32),
                );
                damageHelper = crate::c::div_i32(
                    damageHelper,
                    ((((((((&raw const gStatStageRatios).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((defender).wrapping_add(24)).cast::<i8>()).wrapping_offset(5))
                                .read()) as i32) as isize
                                * 2,
                        ))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32),
                );
            }
            damage = crate::c::div_i32(damage, damageHelper);
            damage = crate::c::div_i32(damage, 50i32);
            if ((((sideStatus) as i32) & 2i32) != 0)
                && (((((&raw mut gCritMultiplier).cast::<u8>()).read()) as i32) == 1i32)
            {
                if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0)
                    && (((CountAliveMonsInBattle(2u8)) as i32) == 2i32)
                {
                    damage = (2i32).wrapping_mul(crate::c::div_i32(damage, 3i32));
                } else {
                    damage = crate::c::div_i32(damage, 2i32);
                }
            }
            if (((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0)
                && ((((((((&raw const gBattleMoves).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 12))
                .wrapping_add(6))
                .read()) as i32)
                    == 8i32))
                && (((CountAliveMonsInBattle(2u8)) as i32) == 2i32)
            {
                damage = crate::c::div_i32(damage, 2i32);
            }
            if (!((AbilityBattleEffects(14u8, 0u8, 13u8, 0u8, 0u16)) != 0))
                && (!((AbilityBattleEffects(14u8, 0u8, 77u8, 0u8, 0u16)) != 0))
            {
                if (((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 1i32) != 0 {
                    'l3: {
                        let __sw3 = ((r#type) as i32);
                        if __sw3 == 10i32 {
                            damage = crate::c::div_i32(damage, 2i32);
                            break 'l3;
                        }
                        if __sw3 == 11i32 {
                            damage = crate::c::div_i32((15i32).wrapping_mul(damage), 10i32);
                            break 'l3;
                        }
                    }
                }
                if ((((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 159i32) != 0)
                    && (((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) == 76i32)
                {
                    damage = crate::c::div_i32(damage, 2i32);
                }
                if (((((&raw mut gBattleWeather).cast::<u16>()).read()) as i32) & 96i32) != 0 {
                    'l4: {
                        let __sw4 = ((r#type) as i32);
                        if __sw4 == 10i32 {
                            damage = crate::c::div_i32((15i32).wrapping_mul(damage), 10i32);
                            break 'l4;
                        }
                        if __sw4 == 11i32 {
                            damage = crate::c::div_i32(damage, 2i32);
                            break 'l4;
                        }
                    }
                }
            }
            if (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .cast::<u32>())
            .wrapping_offset(((battlerIdAtk) as i32) as isize))
            .read()
                & 1u32)
                != 0)
                && (((r#type) as i32) == 10i32)
            {
                damage = crate::c::div_i32((15i32).wrapping_mul(damage), 10i32);
            }
        }
        return (damage).wrapping_add(2i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountAliveMonsInBattle(caseId: u8) -> u8 {
    unsafe {
        let mut caseId = caseId;
        let mut i: i32 = 0i32;
        let mut retVal: u8 = 0u8;
        'l1: {
            let __sw1 = ((caseId) as i32);
            if __sw1 == 0i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            if (i != ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32))
                                && (!((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read())
                                    as u32)
                                    & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset((i) as isize))
                                    .read())
                                    != 0))
                            {
                                retVal = (retVal).wrapping_add(1);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i < 4i32) {
                            break 'l4;
                        }
                        'l5: {
                            if (((GetBattlerSide(((i) as u8))) as i32)
                                == ((GetBattlerSide(
                                    ((&raw mut gBattlerAttacker).cast::<u8>()).read(),
                                )) as i32))
                                && (!((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read())
                                    as u32)
                                    & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset((i) as isize))
                                    .read())
                                    != 0))
                            {
                                retVal = (retVal).wrapping_add(1);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l6: loop {
                        if !(i < 4i32) {
                            break 'l6;
                        }
                        'l7: {
                            if (((GetBattlerSide(((i) as u8))) as i32)
                                == ((GetBattlerSide(
                                    ((&raw mut gBattlerTarget).cast::<u8>()).read(),
                                )) as i32))
                                && (!((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read())
                                    as u32)
                                    & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                        .wrapping_offset((i) as isize))
                                    .read())
                                    != 0))
                            {
                                retVal = (retVal).wrapping_add(1);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn ShouldGetStatBadgeBoost(badgeFlag: u16, battler: u8) -> u8 {
    unsafe {
        let mut badgeFlag = badgeFlag;
        let mut battler = battler;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 37685506u32) != 0 {
            return 0u8;
        } else {
            if ((GetBattlerSide(battler)) as i32) != 0i32 {
                return 0u8;
            } else {
                if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0)
                    && (((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                        == 1024i32)
                {
                    return 0u8;
                } else {
                    if (FlagGet(badgeFlag)) != 0 {
                        return 1u8;
                    } else {
                        return 0u8;
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDefaultMoveTarget(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut opposing: u8 = (((((GetBattlerPosition(battler)) as i32) & 1i32) ^ 1i32) as u8);
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0) {
            return GetBattlerAtPosition(opposing);
        }
        if ((CountAliveMonsInBattle(0u8)) as i32) > 1i32 {
            let mut position: u8 = 0u8;
            if (((Random()) as i32) & 1i32) == 0i32 {
                position = ((((opposing) as i32) ^ 2i32) as u8);
            } else {
                position = opposing;
            }
            return GetBattlerAtPosition(position);
        } else {
            if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                    .wrapping_offset(((opposing) as i32) as isize))
                .read())
                != 0
            {
                return GetBattlerAtPosition(((((opposing) as i32) ^ 2i32) as u8));
            } else {
                return GetBattlerAtPosition(opposing);
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonGender(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        return GetBoxMonGender((mon));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonGender(boxMon: *mut u8) -> u8 {
    unsafe {
        let mut boxMon = boxMon;
        let mut species: u16 = ((GetBoxMonData3(boxMon, 11i32, core::ptr::null_mut())) as u16);
        let mut personality: u32 = GetBoxMonData3(boxMon, 0i32, core::ptr::null_mut());
        'l1: {
            let __sw1 = (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(16))
            .read()) as i32);
            if __sw1 == 0i32 || __sw1 == 254i32 || __sw1 == 255i32 {
                return (((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .wrapping_add(16))
                .read();
            }
        }
        if (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((species) as i32) as isize * 28))
        .wrapping_add(16))
        .read()) as u32)
            > (personality & 255u32)
        {
            return 254u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGenderFromSpeciesAndPersonality(species: u16, personality: u32) -> u8 {
    unsafe {
        let mut species = species;
        let mut personality = personality;
        'l1: {
            let __sw1 = (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(16))
            .read()) as i32);
            if __sw1 == 0i32 || __sw1 == 254i32 || __sw1 == 255i32 {
                return (((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .wrapping_add(16))
                .read();
            }
        }
        if (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((species) as i32) as isize * 28))
        .wrapping_add(16))
        .read()) as u32)
            > (personality & 255u32)
        {
            return 254u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMultiuseSpriteTemplateToPokemon(speciesTag: u16, battlerPosition: u8) {
    unsafe {
        let mut speciesTag = speciesTag;
        let mut battlerPosition = battlerPosition;
        if ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()) as usize) != 0usize {
            (&raw mut gMultiuseSpriteTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<24>>()
                .write_unaligned(
                    (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(20))
                        .cast::<u8>())
                    .wrapping_offset(((battlerPosition) as i32) as isize * 24)
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
                );
        } else {
            if !((((&raw mut sMonSpritesGfxManagers)
                .cast::<u8>()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .read())
            .is_null()
            {
                (&raw mut gMultiuseSpriteTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        ((((((&raw mut sMonSpritesGfxManagers)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((battlerPosition) as i32) as isize * 24)
                        .cast::<crate::c::Rec4<24>>()
                        .read_unaligned(),
                    );
            } else {
                if !(((((&raw mut sMonSpritesGfxManagers)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(1))
                .read())
                .is_null()
                {
                    (&raw mut gMultiuseSpriteTemplate)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<24>>()
                        .write_unaligned(
                            (((((((&raw mut sMonSpritesGfxManagers)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset(1))
                            .read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((battlerPosition) as i32) as isize * 24)
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                        );
                } else {
                    (&raw mut gMultiuseSpriteTemplate)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<24>>()
                        .write_unaligned(
                            (((&raw const gBattlerSpriteTemplates).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((battlerPosition) as i32) as isize * 24)
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                        );
                }
            }
        }
        (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(speciesTag);
        if (((battlerPosition) as i32) == 0i32) || (((battlerPosition) as i32) == 2i32) {
            (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut *mut u8>())
            .write(((&raw mut gAnims_MonPic).cast::<*mut u8>()).cast::<*mut u8>());
        } else {
            if ((speciesTag) as i32) > 500i32 {
                (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<*mut *mut u8>())
                .write(
                    ((((&raw mut gMonFrontAnimsPtrTable).cast::<*mut *mut u8>())
                        .cast::<*mut *mut u8>())
                    .wrapping_offset((((speciesTag) as i32).wrapping_sub(500i32)) as isize))
                    .read(),
                );
            } else {
                (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<*mut *mut u8>())
                .write(
                    ((((&raw mut gMonFrontAnimsPtrTable).cast::<*mut *mut u8>())
                        .cast::<*mut *mut u8>())
                    .wrapping_offset(((speciesTag) as i32) as isize))
                    .read(),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMultiuseSpriteTemplateToTrainerBack(
    trainerPicId: u16,
    battlerPosition: u8,
) {
    unsafe {
        let mut trainerPicId = trainerPicId;
        let mut battlerPosition = battlerPosition;
        (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(trainerPicId);
        if (((battlerPosition) as i32) == 0i32) || (((battlerPosition) as i32) == 2i32) {
            (&raw mut gMultiuseSpriteTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<24>>()
                .write_unaligned(
                    (((&raw const sTrainerBackSpriteTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((trainerPicId) as i32) as isize * 24)
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
                );
            (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut *mut u8>())
            .write(
                ((((&raw mut gTrainerBackAnimsPtrTable).cast::<*mut *mut u8>())
                    .cast::<*mut *mut u8>())
                .wrapping_offset(((trainerPicId) as i32) as isize))
                .read(),
            );
        } else {
            if ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()) as usize) != 0usize {
                (&raw mut gMultiuseSpriteTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(20))
                        .cast::<u8>())
                        .wrapping_offset(((battlerPosition) as i32) as isize * 24)
                        .cast::<crate::c::Rec4<24>>()
                        .read_unaligned(),
                    );
            } else {
                (&raw mut gMultiuseSpriteTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        (((&raw const gBattlerSpriteTemplates).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((battlerPosition) as i32) as isize * 24)
                        .cast::<crate::c::Rec4<24>>()
                        .read_unaligned(),
                    );
            }
            (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut *mut u8>())
            .write(
                ((((&raw mut gTrainerFrontAnimsPtrTable).cast::<*mut *mut u8>())
                    .cast::<*mut *mut u8>())
                .wrapping_offset(((trainerPicId) as i32) as isize))
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMultiuseSpriteTemplateToTrainerFront(
    trainerPicId: u16,
    battlerPosition: u8,
) {
    unsafe {
        let mut trainerPicId = trainerPicId;
        let mut battlerPosition = battlerPosition;
        if ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()) as usize) != 0usize {
            (&raw mut gMultiuseSpriteTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<24>>()
                .write_unaligned(
                    (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(20))
                        .cast::<u8>())
                    .wrapping_offset(((battlerPosition) as i32) as isize * 24)
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
                );
        } else {
            (&raw mut gMultiuseSpriteTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<24>>()
                .write_unaligned(
                    (((&raw const gBattlerSpriteTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((battlerPosition) as i32) as isize * 24)
                        .cast::<crate::c::Rec4<24>>()
                        .read_unaligned(),
                );
        }
        (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(trainerPicId);
        (((&raw mut gMultiuseSpriteTemplate).cast::<u8>())
            .wrapping_add(8)
            .cast::<*mut *mut u8>())
        .write(
            ((((&raw mut gTrainerFrontAnimsPtrTable).cast::<*mut *mut u8>())
                .cast::<*mut *mut u8>())
            .wrapping_offset(((trainerPicId) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn EncryptBoxMon(boxMon: *mut u8) {
    unsafe {
        let mut boxMon = boxMon;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(48u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    let __p1 = (((boxMon).wrapping_add(32)).cast::<u32>())
                        .wrapping_offset(((i) as i32) as isize);
                    (__p1).write(((__p1).read() ^ ((boxMon).cast::<u32>()).read()));
                    let __p2 = (((boxMon).wrapping_add(32)).cast::<u32>())
                        .wrapping_offset(((i) as i32) as isize);
                    (__p2).write(((__p2).read() ^ ((boxMon).wrapping_add(4).cast::<u32>()).read()));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DecryptBoxMon(boxMon: *mut u8) {
    unsafe {
        let mut boxMon = boxMon;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(48u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    let __p1 = (((boxMon).wrapping_add(32)).cast::<u32>())
                        .wrapping_offset(((i) as i32) as isize);
                    (__p1).write(((__p1).read() ^ ((boxMon).wrapping_add(4).cast::<u32>()).read()));
                    let __p2 = (((boxMon).wrapping_add(32)).cast::<u32>())
                        .wrapping_offset(((i) as i32) as isize);
                    (__p2).write(((__p2).read() ^ ((boxMon).cast::<u32>()).read()));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetSubstruct(
    boxMon: *mut u8,
    personality: u32,
    substructType: u8,
) -> *mut u8 {
    unsafe {
        let mut boxMon = boxMon;
        let mut personality = personality;
        let mut substructType = substructType;
        let mut substruct: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = crate::c::rem_u32(personality, 24u32);
            if __sw1 == 0u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l2: {
                        let __sw2 = ((substructType) as i32);
                        if __sw2 == 0i32 {
                            substruct = substructs0;
                            break 'l2;
                        }
                        if __sw2 == 1i32 {
                            substruct = (substructs0).wrapping_offset(12);
                            break 'l2;
                        }
                        if __sw2 == 2i32 {
                            substruct = (substructs0).wrapping_offset(24);
                            break 'l2;
                        }
                        if __sw2 == 3i32 {
                            substruct = (substructs0).wrapping_offset(36);
                            break 'l2;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 1u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l3: {
                        let __sw3 = ((substructType) as i32);
                        if __sw3 == 0i32 {
                            substruct = substructs1;
                            break 'l3;
                        }
                        if __sw3 == 1i32 {
                            substruct = (substructs1).wrapping_offset(12);
                            break 'l3;
                        }
                        if __sw3 == 2i32 {
                            substruct = (substructs1).wrapping_offset(36);
                            break 'l3;
                        }
                        if __sw3 == 3i32 {
                            substruct = (substructs1).wrapping_offset(24);
                            break 'l3;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 2u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l4: {
                        let __sw4 = ((substructType) as i32);
                        if __sw4 == 0i32 {
                            substruct = substructs2;
                            break 'l4;
                        }
                        if __sw4 == 1i32 {
                            substruct = (substructs2).wrapping_offset(24);
                            break 'l4;
                        }
                        if __sw4 == 2i32 {
                            substruct = (substructs2).wrapping_offset(12);
                            break 'l4;
                        }
                        if __sw4 == 3i32 {
                            substruct = (substructs2).wrapping_offset(36);
                            break 'l4;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 3u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l5: {
                        let __sw5 = ((substructType) as i32);
                        if __sw5 == 0i32 {
                            substruct = substructs3;
                            break 'l5;
                        }
                        if __sw5 == 1i32 {
                            substruct = (substructs3).wrapping_offset(36);
                            break 'l5;
                        }
                        if __sw5 == 2i32 {
                            substruct = (substructs3).wrapping_offset(12);
                            break 'l5;
                        }
                        if __sw5 == 3i32 {
                            substruct = (substructs3).wrapping_offset(24);
                            break 'l5;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 4u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l6: {
                        let __sw6 = ((substructType) as i32);
                        if __sw6 == 0i32 {
                            substruct = substructs4;
                            break 'l6;
                        }
                        if __sw6 == 1i32 {
                            substruct = (substructs4).wrapping_offset(24);
                            break 'l6;
                        }
                        if __sw6 == 2i32 {
                            substruct = (substructs4).wrapping_offset(36);
                            break 'l6;
                        }
                        if __sw6 == 3i32 {
                            substruct = (substructs4).wrapping_offset(12);
                            break 'l6;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 5u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l7: {
                        let __sw7 = ((substructType) as i32);
                        if __sw7 == 0i32 {
                            substruct = substructs5;
                            break 'l7;
                        }
                        if __sw7 == 1i32 {
                            substruct = (substructs5).wrapping_offset(36);
                            break 'l7;
                        }
                        if __sw7 == 2i32 {
                            substruct = (substructs5).wrapping_offset(24);
                            break 'l7;
                        }
                        if __sw7 == 3i32 {
                            substruct = (substructs5).wrapping_offset(12);
                            break 'l7;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 6u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l8: {
                        let __sw8 = ((substructType) as i32);
                        if __sw8 == 0i32 {
                            substruct = (substructs6).wrapping_offset(12);
                            break 'l8;
                        }
                        if __sw8 == 1i32 {
                            substruct = substructs6;
                            break 'l8;
                        }
                        if __sw8 == 2i32 {
                            substruct = (substructs6).wrapping_offset(24);
                            break 'l8;
                        }
                        if __sw8 == 3i32 {
                            substruct = (substructs6).wrapping_offset(36);
                            break 'l8;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 7u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l9: {
                        let __sw9 = ((substructType) as i32);
                        if __sw9 == 0i32 {
                            substruct = (substructs7).wrapping_offset(12);
                            break 'l9;
                        }
                        if __sw9 == 1i32 {
                            substruct = substructs7;
                            break 'l9;
                        }
                        if __sw9 == 2i32 {
                            substruct = (substructs7).wrapping_offset(36);
                            break 'l9;
                        }
                        if __sw9 == 3i32 {
                            substruct = (substructs7).wrapping_offset(24);
                            break 'l9;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 8u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l10: {
                        let __sw10 = ((substructType) as i32);
                        if __sw10 == 0i32 {
                            substruct = (substructs8).wrapping_offset(24);
                            break 'l10;
                        }
                        if __sw10 == 1i32 {
                            substruct = substructs8;
                            break 'l10;
                        }
                        if __sw10 == 2i32 {
                            substruct = (substructs8).wrapping_offset(12);
                            break 'l10;
                        }
                        if __sw10 == 3i32 {
                            substruct = (substructs8).wrapping_offset(36);
                            break 'l10;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 9u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l11: {
                        let __sw11 = ((substructType) as i32);
                        if __sw11 == 0i32 {
                            substruct = (substructs9).wrapping_offset(36);
                            break 'l11;
                        }
                        if __sw11 == 1i32 {
                            substruct = substructs9;
                            break 'l11;
                        }
                        if __sw11 == 2i32 {
                            substruct = (substructs9).wrapping_offset(12);
                            break 'l11;
                        }
                        if __sw11 == 3i32 {
                            substruct = (substructs9).wrapping_offset(24);
                            break 'l11;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 10u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l12: {
                        let __sw12 = ((substructType) as i32);
                        if __sw12 == 0i32 {
                            substruct = (substructs10).wrapping_offset(24);
                            break 'l12;
                        }
                        if __sw12 == 1i32 {
                            substruct = substructs10;
                            break 'l12;
                        }
                        if __sw12 == 2i32 {
                            substruct = (substructs10).wrapping_offset(36);
                            break 'l12;
                        }
                        if __sw12 == 3i32 {
                            substruct = (substructs10).wrapping_offset(12);
                            break 'l12;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 11u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l13: {
                        let __sw13 = ((substructType) as i32);
                        if __sw13 == 0i32 {
                            substruct = (substructs11).wrapping_offset(36);
                            break 'l13;
                        }
                        if __sw13 == 1i32 {
                            substruct = substructs11;
                            break 'l13;
                        }
                        if __sw13 == 2i32 {
                            substruct = (substructs11).wrapping_offset(24);
                            break 'l13;
                        }
                        if __sw13 == 3i32 {
                            substruct = (substructs11).wrapping_offset(12);
                            break 'l13;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 12u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l14: {
                        let __sw14 = ((substructType) as i32);
                        if __sw14 == 0i32 {
                            substruct = (substructs12).wrapping_offset(12);
                            break 'l14;
                        }
                        if __sw14 == 1i32 {
                            substruct = (substructs12).wrapping_offset(24);
                            break 'l14;
                        }
                        if __sw14 == 2i32 {
                            substruct = substructs12;
                            break 'l14;
                        }
                        if __sw14 == 3i32 {
                            substruct = (substructs12).wrapping_offset(36);
                            break 'l14;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 13u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l15: {
                        let __sw15 = ((substructType) as i32);
                        if __sw15 == 0i32 {
                            substruct = (substructs13).wrapping_offset(12);
                            break 'l15;
                        }
                        if __sw15 == 1i32 {
                            substruct = (substructs13).wrapping_offset(36);
                            break 'l15;
                        }
                        if __sw15 == 2i32 {
                            substruct = substructs13;
                            break 'l15;
                        }
                        if __sw15 == 3i32 {
                            substruct = (substructs13).wrapping_offset(24);
                            break 'l15;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 14u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l16: {
                        let __sw16 = ((substructType) as i32);
                        if __sw16 == 0i32 {
                            substruct = (substructs14).wrapping_offset(24);
                            break 'l16;
                        }
                        if __sw16 == 1i32 {
                            substruct = (substructs14).wrapping_offset(12);
                            break 'l16;
                        }
                        if __sw16 == 2i32 {
                            substruct = substructs14;
                            break 'l16;
                        }
                        if __sw16 == 3i32 {
                            substruct = (substructs14).wrapping_offset(36);
                            break 'l16;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 15u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l17: {
                        let __sw17 = ((substructType) as i32);
                        if __sw17 == 0i32 {
                            substruct = (substructs15).wrapping_offset(36);
                            break 'l17;
                        }
                        if __sw17 == 1i32 {
                            substruct = (substructs15).wrapping_offset(12);
                            break 'l17;
                        }
                        if __sw17 == 2i32 {
                            substruct = substructs15;
                            break 'l17;
                        }
                        if __sw17 == 3i32 {
                            substruct = (substructs15).wrapping_offset(24);
                            break 'l17;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 16u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l18: {
                        let __sw18 = ((substructType) as i32);
                        if __sw18 == 0i32 {
                            substruct = (substructs16).wrapping_offset(24);
                            break 'l18;
                        }
                        if __sw18 == 1i32 {
                            substruct = (substructs16).wrapping_offset(36);
                            break 'l18;
                        }
                        if __sw18 == 2i32 {
                            substruct = substructs16;
                            break 'l18;
                        }
                        if __sw18 == 3i32 {
                            substruct = (substructs16).wrapping_offset(12);
                            break 'l18;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 17u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l19: {
                        let __sw19 = ((substructType) as i32);
                        if __sw19 == 0i32 {
                            substruct = (substructs17).wrapping_offset(36);
                            break 'l19;
                        }
                        if __sw19 == 1i32 {
                            substruct = (substructs17).wrapping_offset(24);
                            break 'l19;
                        }
                        if __sw19 == 2i32 {
                            substruct = substructs17;
                            break 'l19;
                        }
                        if __sw19 == 3i32 {
                            substruct = (substructs17).wrapping_offset(12);
                            break 'l19;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 18u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l20: {
                        let __sw20 = ((substructType) as i32);
                        if __sw20 == 0i32 {
                            substruct = (substructs18).wrapping_offset(12);
                            break 'l20;
                        }
                        if __sw20 == 1i32 {
                            substruct = (substructs18).wrapping_offset(24);
                            break 'l20;
                        }
                        if __sw20 == 2i32 {
                            substruct = (substructs18).wrapping_offset(36);
                            break 'l20;
                        }
                        if __sw20 == 3i32 {
                            substruct = substructs18;
                            break 'l20;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 19u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l21: {
                        let __sw21 = ((substructType) as i32);
                        if __sw21 == 0i32 {
                            substruct = (substructs19).wrapping_offset(12);
                            break 'l21;
                        }
                        if __sw21 == 1i32 {
                            substruct = (substructs19).wrapping_offset(36);
                            break 'l21;
                        }
                        if __sw21 == 2i32 {
                            substruct = (substructs19).wrapping_offset(24);
                            break 'l21;
                        }
                        if __sw21 == 3i32 {
                            substruct = substructs19;
                            break 'l21;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 20u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l22: {
                        let __sw22 = ((substructType) as i32);
                        if __sw22 == 0i32 {
                            substruct = (substructs20).wrapping_offset(24);
                            break 'l22;
                        }
                        if __sw22 == 1i32 {
                            substruct = (substructs20).wrapping_offset(12);
                            break 'l22;
                        }
                        if __sw22 == 2i32 {
                            substruct = (substructs20).wrapping_offset(36);
                            break 'l22;
                        }
                        if __sw22 == 3i32 {
                            substruct = substructs20;
                            break 'l22;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 21u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l23: {
                        let __sw23 = ((substructType) as i32);
                        if __sw23 == 0i32 {
                            substruct = (substructs21).wrapping_offset(36);
                            break 'l23;
                        }
                        if __sw23 == 1i32 {
                            substruct = (substructs21).wrapping_offset(12);
                            break 'l23;
                        }
                        if __sw23 == 2i32 {
                            substruct = (substructs21).wrapping_offset(24);
                            break 'l23;
                        }
                        if __sw23 == 3i32 {
                            substruct = substructs21;
                            break 'l23;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 22u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l24: {
                        let __sw24 = ((substructType) as i32);
                        if __sw24 == 0i32 {
                            substruct = (substructs22).wrapping_offset(24);
                            break 'l24;
                        }
                        if __sw24 == 1i32 {
                            substruct = (substructs22).wrapping_offset(36);
                            break 'l24;
                        }
                        if __sw24 == 2i32 {
                            substruct = (substructs22).wrapping_offset(12);
                            break 'l24;
                        }
                        if __sw24 == 3i32 {
                            substruct = substructs22;
                            break 'l24;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 23u32 {
                {
                    let mut substructs0: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs1: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs2: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs3: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs4: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs5: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs6: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs7: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs8: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs9: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs10: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs11: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs12: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs13: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs14: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs15: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs16: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs17: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs18: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs19: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs20: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs21: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs22: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    let mut substructs23: *mut u8 = ((boxMon).wrapping_add(32)).cast::<u8>();
                    'l25: {
                        let __sw25 = ((substructType) as i32);
                        if __sw25 == 0i32 {
                            substruct = (substructs23).wrapping_offset(36);
                            break 'l25;
                        }
                        if __sw25 == 1i32 {
                            substruct = (substructs23).wrapping_offset(24);
                            break 'l25;
                        }
                        if __sw25 == 2i32 {
                            substruct = (substructs23).wrapping_offset(12);
                            break 'l25;
                        }
                        if __sw25 == 3i32 {
                            substruct = substructs23;
                            break 'l25;
                        }
                    }
                    break 'l1;
                }
            }
        }
        return substruct;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonData3(mon: *mut u8, field: i32, data: *mut u8) -> u32 {
    unsafe {
        let mut mon = mon;
        let mut field = field;
        let mut data = data;
        let mut ret: u32 = 0u32;
        'l1: {
            let __sw1 = field;
            let __matched = __sw1 == 55i32
                || __sw1 == 56i32
                || __sw1 == 57i32
                || __sw1 == 58i32
                || __sw1 == 59i32
                || __sw1 == 60i32
                || __sw1 == 61i32
                || __sw1 == 62i32
                || __sw1 == 63i32
                || __sw1 == 84i32
                || __sw1 == 85i32
                || __sw1 == 86i32
                || __sw1 == 87i32
                || __sw1 == 88i32
                || __sw1 == 64i32;
            if __sw1 == 55i32 {
                ret = ((mon).wrapping_add(80).cast::<u32>()).read();
                break 'l1;
            }
            if __sw1 == 56i32 {
                ret = ((((mon).wrapping_add(84)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 57i32 {
                ret = ((((mon).wrapping_add(86).cast::<u16>()).read()) as u32);
                break 'l1;
            }
            if __sw1 == 58i32 {
                ret = ((((mon).wrapping_add(88).cast::<u16>()).read()) as u32);
                break 'l1;
            }
            if __sw1 == 59i32 {
                ret = ((GetDeoxysStat(mon, 1i32)) as u32);
                if !((ret) != 0) {
                    ret = ((((mon).wrapping_add(90).cast::<u16>()).read()) as u32);
                }
                break 'l1;
            }
            if __sw1 == 60i32 {
                ret = ((GetDeoxysStat(mon, 2i32)) as u32);
                if !((ret) != 0) {
                    ret = ((((mon).wrapping_add(92).cast::<u16>()).read()) as u32);
                }
                break 'l1;
            }
            if __sw1 == 61i32 {
                ret = ((GetDeoxysStat(mon, 3i32)) as u32);
                if !((ret) != 0) {
                    ret = ((((mon).wrapping_add(94).cast::<u16>()).read()) as u32);
                }
                break 'l1;
            }
            if __sw1 == 62i32 {
                ret = ((GetDeoxysStat(mon, 4i32)) as u32);
                if !((ret) != 0) {
                    ret = ((((mon).wrapping_add(96).cast::<u16>()).read()) as u32);
                }
                break 'l1;
            }
            if __sw1 == 63i32 {
                ret = ((GetDeoxysStat(mon, 5i32)) as u32);
                if !((ret) != 0) {
                    ret = ((((mon).wrapping_add(98).cast::<u16>()).read()) as u32);
                }
                break 'l1;
            }
            if __sw1 == 84i32 {
                ret = ((((mon).wrapping_add(90).cast::<u16>()).read()) as u32);
                break 'l1;
            }
            if __sw1 == 85i32 {
                ret = ((((mon).wrapping_add(92).cast::<u16>()).read()) as u32);
                break 'l1;
            }
            if __sw1 == 86i32 {
                ret = ((((mon).wrapping_add(94).cast::<u16>()).read()) as u32);
                break 'l1;
            }
            if __sw1 == 87i32 {
                ret = ((((mon).wrapping_add(96).cast::<u16>()).read()) as u32);
                break 'l1;
            }
            if __sw1 == 88i32 {
                ret = ((((mon).wrapping_add(98).cast::<u16>()).read()) as u32);
                break 'l1;
            }
            if __sw1 == 64i32 {
                ret = ((((mon).wrapping_add(85)).read()) as u32);
                break 'l1;
            }
            if !__matched {
                ret = GetBoxMonData3((mon), field, data);
                break 'l1;
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonData2(mon: *mut u8, field: i32) -> u32 {
    unsafe {
        let mut mon = mon;
        let mut field = field;
        return GetMonData3(mon, field, core::ptr::null_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonData3(boxMon: *mut u8, field: i32, data: *mut u8) -> u32 {
    unsafe {
        let mut boxMon = boxMon;
        let mut field = field;
        let mut data = data;
        let mut i: i32 = 0i32;
        let mut retVal: u32 = 0u32;
        let mut substruct0: *mut u8 = core::ptr::null_mut();
        let mut substruct1: *mut u8 = core::ptr::null_mut();
        let mut substruct2: *mut u8 = core::ptr::null_mut();
        let mut substruct3: *mut u8 = core::ptr::null_mut();
        if field > 10i32 {
            substruct0 = (GetSubstruct(boxMon, ((boxMon).cast::<u32>()).read(), 0u8));
            substruct1 = (GetSubstruct(boxMon, ((boxMon).cast::<u32>()).read(), 1u8));
            substruct2 = (GetSubstruct(boxMon, ((boxMon).cast::<u32>()).read(), 2u8));
            substruct3 = (GetSubstruct(boxMon, ((boxMon).cast::<u32>()).read(), 3u8));
            DecryptBoxMon(boxMon);
            if ((CalculateBoxMonChecksum(boxMon)) as i32)
                != ((((boxMon).wrapping_add(28).cast::<u16>()).read()) as i32)
            {
                crate::c::bf_write((boxMon).wrapping_add(19), 0, 1, (1u8) as i32);
                crate::c::bf_write((boxMon).wrapping_add(19), 2, 1, (1u8) as i32);
                crate::c::bf_write((substruct3).wrapping_add(7), 6, 1, (1u32) as i32);
            }
        }
        'l1: {
            let __sw1 = field;
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 25i32
                || __sw1 == 21i32
                || __sw1 == 32i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 17i32
                || __sw1 == 18i32
                || __sw1 == 19i32
                || __sw1 == 20i32
                || __sw1 == 26i32
                || __sw1 == 27i32
                || __sw1 == 28i32
                || __sw1 == 29i32
                || __sw1 == 30i32
                || __sw1 == 31i32
                || __sw1 == 22i32
                || __sw1 == 23i32
                || __sw1 == 24i32
                || __sw1 == 33i32
                || __sw1 == 47i32
                || __sw1 == 48i32
                || __sw1 == 34i32
                || __sw1 == 35i32
                || __sw1 == 36i32
                || __sw1 == 37i32
                || __sw1 == 38i32
                || __sw1 == 49i32
                || __sw1 == 39i32
                || __sw1 == 40i32
                || __sw1 == 41i32
                || __sw1 == 42i32
                || __sw1 == 43i32
                || __sw1 == 44i32
                || __sw1 == 45i32
                || __sw1 == 46i32
                || __sw1 == 50i32
                || __sw1 == 51i32
                || __sw1 == 52i32
                || __sw1 == 53i32
                || __sw1 == 54i32
                || __sw1 == 67i32
                || __sw1 == 68i32
                || __sw1 == 69i32
                || __sw1 == 70i32
                || __sw1 == 71i32
                || __sw1 == 72i32
                || __sw1 == 73i32
                || __sw1 == 74i32
                || __sw1 == 75i32
                || __sw1 == 76i32
                || __sw1 == 77i32
                || __sw1 == 78i32
                || __sw1 == 79i32
                || __sw1 == 80i32
                || __sw1 == 65i32
                || __sw1 == 66i32
                || __sw1 == 81i32
                || __sw1 == 82i32
                || __sw1 == 83i32;
            if __sw1 == 0i32 {
                retVal = ((boxMon).cast::<u32>()).read();
                break 'l1;
            }
            if __sw1 == 1i32 {
                retVal = ((boxMon).wrapping_add(4).cast::<u32>()).read();
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    if (crate::c::bf_read((boxMon).wrapping_add(19), 0, 1, false) as u8) != 0 {
                        {
                            retVal = 0u32;
                            'l2: loop {
                                if !((retVal < 10u32)
                                    && ((((((&raw mut gText_BadEgg).cast::<u8>())
                                        .wrapping_offset(((retVal) as i32) as isize))
                                    .read()) as i32)
                                        != 255i32))
                                {
                                    break 'l2;
                                }
                                'l3: {}
                                ((data).wrapping_offset(((retVal) as i32) as isize)).write(
                                    (((&raw mut gText_BadEgg).cast::<u8>())
                                        .wrapping_offset(((retVal) as i32) as isize))
                                    .read(),
                                );
                                retVal = (retVal).wrapping_add(1);
                            }
                        }
                        ((data).wrapping_offset(((retVal) as i32) as isize)).write(255u8);
                    } else {
                        if (crate::c::bf_read((boxMon).wrapping_add(19), 2, 1, false) as u8) != 0 {
                            StringCopy(data, (&raw mut gText_EggNickname).cast::<u8>());
                            retVal = ((StringLength(data)) as u32);
                        } else {
                            if ((((boxMon).wrapping_add(18)).read()) as i32) == 1i32 {
                                (data).write(252u8);
                                ((data).wrapping_offset(1)).write(21u8);
                                {
                                    retVal = 2u32;
                                    i = 0i32;
                                    'l4: loop {
                                        if !((i < 5i32)
                                            && (((((((boxMon).wrapping_add(8)).cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                != 255i32))
                                        {
                                            break 'l4;
                                        }
                                        'l5: {}
                                        ((data).wrapping_offset(((retVal) as i32) as isize)).write(
                                            ((((boxMon).wrapping_add(8)).cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                            .read(),
                                        );
                                        retVal = (retVal).wrapping_add(1);
                                        i = (i).wrapping_add(1);
                                    }
                                }
                                ((data).wrapping_offset(
                                    (({
                                        let __t2 = retVal;
                                        retVal = (retVal).wrapping_add(1);
                                        __t2
                                    }) as i32) as isize,
                                ))
                                .write(252u8);
                                ((data).wrapping_offset(
                                    (({
                                        let __t3 = retVal;
                                        retVal = (retVal).wrapping_add(1);
                                        __t3
                                    }) as i32) as isize,
                                ))
                                .write(22u8);
                                ((data).wrapping_offset(((retVal) as i32) as isize)).write(255u8);
                            } else {
                                {
                                    retVal = 0u32;
                                    'l6: loop {
                                        if !(retVal < 10u32) {
                                            break 'l6;
                                        }
                                        'l7: {}
                                        ((data).wrapping_offset(((retVal) as i32) as isize)).write(
                                            ((((boxMon).wrapping_add(8)).cast::<u8>())
                                                .wrapping_offset(((retVal) as i32) as isize))
                                            .read(),
                                        );
                                        retVal = (retVal).wrapping_add(1);
                                    }
                                }
                                ((data).wrapping_offset(((retVal) as i32) as isize)).write(255u8);
                            }
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 3i32 {
                retVal = ((((boxMon).wrapping_add(18)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 4i32 {
                retVal = ((crate::c::bf_read((boxMon).wrapping_add(19), 0, 1, false) as u8) as u32);
                break 'l1;
            }
            if __sw1 == 5i32 {
                retVal = ((crate::c::bf_read((boxMon).wrapping_add(19), 1, 1, false) as u8) as u32);
                break 'l1;
            }
            if __sw1 == 6i32 {
                retVal = ((crate::c::bf_read((boxMon).wrapping_add(19), 2, 1, false) as u8) as u32);
                break 'l1;
            }
            if __sw1 == 7i32 {
                {
                    retVal = 0u32;
                    'l8: loop {
                        if !(retVal < 7u32) {
                            break 'l8;
                        }
                        ((data).wrapping_offset(((retVal) as i32) as isize)).write(
                            ((((boxMon).wrapping_add(20)).cast::<u8>())
                                .wrapping_offset(((retVal) as i32) as isize))
                            .read(),
                        );
                        retVal = (retVal).wrapping_add(1);
                    }
                    ((data).wrapping_offset(((retVal) as i32) as isize)).write(255u8);
                    break 'l1;
                }
            }
            if __sw1 == 8i32 {
                retVal = ((((boxMon).wrapping_add(27)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 9i32 {
                retVal = ((((boxMon).wrapping_add(28).cast::<u16>()).read()) as u32);
                break 'l1;
            }
            if __sw1 == 10i32 {
                retVal = ((((boxMon).wrapping_add(30).cast::<u16>()).read()) as u32);
                break 'l1;
            }
            if __sw1 == 11i32 {
                retVal = ((if (crate::c::bf_read((boxMon).wrapping_add(19), 0, 1, false) as u8) != 0
                {
                    412i32
                } else {
                    ((((substruct0).cast::<u16>()).read()) as i32)
                }) as u32);
                break 'l1;
            }
            if __sw1 == 12i32 {
                retVal = ((((substruct0).wrapping_add(2).cast::<u16>()).read()) as u32);
                break 'l1;
            }
            if __sw1 == 25i32 {
                retVal = ((substruct0).wrapping_add(4).cast::<u32>()).read();
                break 'l1;
            }
            if __sw1 == 21i32 {
                retVal = ((((substruct0).wrapping_add(8)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 32i32 {
                retVal = ((((substruct0).wrapping_add(9)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 13i32 || __sw1 == 14i32 || __sw1 == 15i32 || __sw1 == 16i32 {
                retVal = (((((substruct1).cast::<u16>())
                    .wrapping_offset(((field).wrapping_sub(13i32)) as isize))
                .read()) as u32);
                break 'l1;
            }
            if __sw1 == 17i32 || __sw1 == 18i32 || __sw1 == 19i32 || __sw1 == 20i32 {
                retVal = ((((((substruct1).wrapping_add(8)).cast::<u8>())
                    .wrapping_offset(((field).wrapping_sub(17i32)) as isize))
                .read()) as u32);
                break 'l1;
            }
            if __sw1 == 26i32 {
                retVal = (((substruct2).read()) as u32);
                break 'l1;
            }
            if __sw1 == 27i32 {
                retVal = ((((substruct2).wrapping_add(1)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 28i32 {
                retVal = ((((substruct2).wrapping_add(2)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 29i32 {
                retVal = ((((substruct2).wrapping_add(3)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 30i32 {
                retVal = ((((substruct2).wrapping_add(4)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 31i32 {
                retVal = ((((substruct2).wrapping_add(5)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 22i32 {
                retVal = ((((substruct2).wrapping_add(6)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 23i32 {
                retVal = ((((substruct2).wrapping_add(7)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 24i32 {
                retVal = ((((substruct2).wrapping_add(8)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 33i32 {
                retVal = ((((substruct2).wrapping_add(9)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 47i32 {
                retVal = ((((substruct2).wrapping_add(10)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 48i32 {
                retVal = ((((substruct2).wrapping_add(11)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 34i32 {
                retVal = (((substruct3).read()) as u32);
                break 'l1;
            }
            if __sw1 == 35i32 {
                retVal = ((((substruct3).wrapping_add(1)).read()) as u32);
                break 'l1;
            }
            if __sw1 == 36i32 {
                retVal =
                    ((crate::c::bf_read((substruct3).wrapping_add(2), 0, 7, false) as u16) as u32);
                break 'l1;
            }
            if __sw1 == 37i32 {
                retVal =
                    ((crate::c::bf_read((substruct3).wrapping_add(2), 7, 4, false) as u16) as u32);
                break 'l1;
            }
            if __sw1 == 38i32 {
                retVal =
                    ((crate::c::bf_read((substruct3).wrapping_add(3), 3, 4, false) as u16) as u32);
                break 'l1;
            }
            if __sw1 == 49i32 {
                retVal =
                    ((crate::c::bf_read((substruct3).wrapping_add(3), 7, 1, false) as u16) as u32);
                break 'l1;
            }
            if __sw1 == 39i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(4), 0, 5, false) as u32);
                break 'l1;
            }
            if __sw1 == 40i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(4), 5, 5, false) as u32);
                break 'l1;
            }
            if __sw1 == 41i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(5), 2, 5, false) as u32);
                break 'l1;
            }
            if __sw1 == 42i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(5), 7, 5, false) as u32);
                break 'l1;
            }
            if __sw1 == 43i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(6), 4, 5, false) as u32);
                break 'l1;
            }
            if __sw1 == 44i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(7), 1, 5, false) as u32);
                break 'l1;
            }
            if __sw1 == 45i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(7), 6, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 46i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(7), 7, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 50i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(8), 0, 3, false) as u32);
                break 'l1;
            }
            if __sw1 == 51i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(8), 3, 3, false) as u32);
                break 'l1;
            }
            if __sw1 == 52i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(8), 6, 3, false) as u32);
                break 'l1;
            }
            if __sw1 == 53i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(9), 1, 3, false) as u32);
                break 'l1;
            }
            if __sw1 == 54i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(9), 4, 3, false) as u32);
                break 'l1;
            }
            if __sw1 == 67i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(9), 7, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 68i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(10), 0, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 69i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(10), 1, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 70i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(10), 2, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 71i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(10), 3, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 72i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(10), 4, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 73i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(10), 5, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 74i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(10), 6, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 75i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(10), 7, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 76i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(11), 0, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 77i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(11), 1, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 78i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(11), 2, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 79i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(11), 3, 4, false) as u32);
                break 'l1;
            }
            if __sw1 == 80i32 {
                retVal = (crate::c::bf_read((substruct3).wrapping_add(11), 7, 1, false) as u32);
                break 'l1;
            }
            if __sw1 == 65i32 {
                retVal = ((((substruct0).cast::<u16>()).read()) as u32);
                if ((((substruct0).cast::<u16>()).read()) != 0)
                    && (((crate::c::bf_read((substruct3).wrapping_add(7), 6, 1, false) as u32)
                        != 0)
                        || ((crate::c::bf_read((boxMon).wrapping_add(19), 0, 1, false) as u8) != 0))
                {
                    retVal = 412u32;
                }
                break 'l1;
            }
            if __sw1 == 66i32 {
                retVal = ((((((crate::c::bf_read((substruct3).wrapping_add(4), 0, 5, false)
                    as u32)
                    | ((crate::c::bf_read((substruct3).wrapping_add(4), 5, 5, false) as u32)
                        << 5))
                    | ((crate::c::bf_read((substruct3).wrapping_add(5), 2, 5, false) as u32)
                        << 10))
                    | ((crate::c::bf_read((substruct3).wrapping_add(5), 7, 5, false) as u32)
                        << 15))
                    | ((crate::c::bf_read((substruct3).wrapping_add(6), 4, 5, false) as u32)
                        << 20))
                    | ((crate::c::bf_read((substruct3).wrapping_add(7), 1, 5, false) as u32)
                        << 25));
                break 'l1;
            }
            if __sw1 == 81i32 {
                if ((((substruct0).cast::<u16>()).read()) != 0)
                    && (!((crate::c::bf_read((substruct3).wrapping_add(7), 6, 1, false) as u32)
                        != 0))
                {
                    let mut moves: *mut u16 = (data).cast::<u16>();
                    let mut i: i32 = 0i32;
                    'l9: loop {
                        if !(((((moves).wrapping_offset((i) as isize)).read()) as i32) != 355i32) {
                            break 'l9;
                        }
                        let mut r#move: u16 = ((moves).wrapping_offset((i) as isize)).read();
                        if (((((((substruct1).cast::<u16>()).read()) as i32) == ((r#move) as i32))
                            || ((((((substruct1).cast::<u16>()).wrapping_offset(1)).read())
                                as i32)
                                == ((r#move) as i32)))
                            || ((((((substruct1).cast::<u16>()).wrapping_offset(2)).read())
                                as i32)
                                == ((r#move) as i32)))
                            || ((((((substruct1).cast::<u16>()).wrapping_offset(3)).read()) as i32)
                                == ((r#move) as i32))
                        {
                            retVal = (retVal
                                | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset((i) as isize))
                                .read());
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 82i32 {
                retVal = 0u32;
                if ((((substruct0).cast::<u16>()).read()) != 0)
                    && (!((crate::c::bf_read((substruct3).wrapping_add(7), 6, 1, false) as u32)
                        != 0))
                {
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(8), 0, 3, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(8), 3, 3, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(8), 6, 3, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(9), 1, 3, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(9), 4, 3, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(9), 7, 1, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(10), 0, 1, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(10), 1, 1, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(10), 2, 1, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(10), 3, 1, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(10), 4, 1, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(10), 5, 1, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(10), 6, 1, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(10), 7, 1, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(11), 0, 1, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(11), 1, 1, false) as u32),
                    );
                    retVal = (retVal).wrapping_add(
                        (crate::c::bf_read((substruct3).wrapping_add(11), 2, 1, false) as u32),
                    );
                }
                break 'l1;
            }
            if __sw1 == 83i32 {
                retVal = 0u32;
                if ((((substruct0).cast::<u16>()).read()) != 0)
                    && (!((crate::c::bf_read((substruct3).wrapping_add(7), 6, 1, false) as u32)
                        != 0))
                {
                    retVal = (((((((((((((((((crate::c::bf_read(
                        (substruct3).wrapping_add(9),
                        7,
                        1,
                        false,
                    ) as u32)
                        | ((crate::c::bf_read((substruct3).wrapping_add(8), 0, 3, false)
                            as u32)
                            << 1))
                        | ((crate::c::bf_read((substruct3).wrapping_add(8), 3, 3, false)
                            as u32)
                            << 4))
                        | ((crate::c::bf_read((substruct3).wrapping_add(8), 6, 3, false)
                            as u32)
                            << 7))
                        | ((crate::c::bf_read((substruct3).wrapping_add(9), 1, 3, false)
                            as u32)
                            << 10))
                        | ((crate::c::bf_read((substruct3).wrapping_add(9), 4, 3, false)
                            as u32)
                            << 13))
                        | ((crate::c::bf_read((substruct3).wrapping_add(10), 0, 1, false)
                            as u32)
                            << 16))
                        | ((crate::c::bf_read((substruct3).wrapping_add(10), 1, 1, false)
                            as u32)
                            << 17))
                        | ((crate::c::bf_read((substruct3).wrapping_add(10), 2, 1, false)
                            as u32)
                            << 18))
                        | ((crate::c::bf_read((substruct3).wrapping_add(10), 3, 1, false)
                            as u32)
                            << 19))
                        | ((crate::c::bf_read((substruct3).wrapping_add(10), 4, 1, false)
                            as u32)
                            << 20))
                        | ((crate::c::bf_read((substruct3).wrapping_add(10), 5, 1, false)
                            as u32)
                            << 21))
                        | ((crate::c::bf_read((substruct3).wrapping_add(10), 6, 1, false)
                            as u32)
                            << 22))
                        | ((crate::c::bf_read((substruct3).wrapping_add(10), 7, 1, false)
                            as u32)
                            << 23))
                        | ((crate::c::bf_read((substruct3).wrapping_add(11), 0, 1, false)
                            as u32)
                            << 24))
                        | ((crate::c::bf_read((substruct3).wrapping_add(11), 1, 1, false)
                            as u32)
                            << 25))
                        | ((crate::c::bf_read((substruct3).wrapping_add(11), 2, 1, false) as u32)
                            << 26));
                }
                break 'l1;
            }
            if !__matched {
                break 'l1;
            }
        }
        if field > 10i32 {
            EncryptBoxMon(boxMon);
        }
        return retVal;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonData2(boxMon: *mut u8, field: i32) -> u32 {
    unsafe {
        let mut boxMon = boxMon;
        let mut field = field;
        return GetBoxMonData3(boxMon, field, core::ptr::null_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMonData(mon: *mut u8, field: i32, dataArg: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut field = field;
        let mut dataArg = dataArg;
        let mut data: *mut u8 = dataArg;
        'l1: {
            let __sw1 = field;
            let __matched = __sw1 == 55i32
                || __sw1 == 56i32
                || __sw1 == 57i32
                || __sw1 == 58i32
                || __sw1 == 59i32
                || __sw1 == 60i32
                || __sw1 == 61i32
                || __sw1 == 62i32
                || __sw1 == 63i32
                || __sw1 == 64i32
                || __sw1 == 65i32;
            if __sw1 == 55i32 {
                ((mon).wrapping_add(80).cast::<u32>()).write(
                    (((((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                    .wrapping_add((((((data).wrapping_offset(2)).read()) as i32) << 16)))
                    .wrapping_add((((((data).wrapping_offset(3)).read()) as i32) << 24)))
                        as u32),
                );
                break 'l1;
            }
            if __sw1 == 56i32 {
                ((mon).wrapping_add(84)).write((data).read());
                break 'l1;
            }
            if __sw1 == 57i32 {
                ((mon).wrapping_add(86).cast::<u16>()).write(
                    (((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                        as u16),
                );
                break 'l1;
            }
            if __sw1 == 58i32 {
                ((mon).wrapping_add(88).cast::<u16>()).write(
                    (((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                        as u16),
                );
                break 'l1;
            }
            if __sw1 == 59i32 {
                ((mon).wrapping_add(90).cast::<u16>()).write(
                    (((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                        as u16),
                );
                break 'l1;
            }
            if __sw1 == 60i32 {
                ((mon).wrapping_add(92).cast::<u16>()).write(
                    (((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                        as u16),
                );
                break 'l1;
            }
            if __sw1 == 61i32 {
                ((mon).wrapping_add(94).cast::<u16>()).write(
                    (((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                        as u16),
                );
                break 'l1;
            }
            if __sw1 == 62i32 {
                ((mon).wrapping_add(96).cast::<u16>()).write(
                    (((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                        as u16),
                );
                break 'l1;
            }
            if __sw1 == 63i32 {
                ((mon).wrapping_add(98).cast::<u16>()).write(
                    (((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                        as u16),
                );
                break 'l1;
            }
            if __sw1 == 64i32 {
                ((mon).wrapping_add(85)).write((data).read());
                break 'l1;
            }
            if __sw1 == 65i32 {
                break 'l1;
            }
            if !__matched {
                SetBoxMonData((mon), field, data);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBoxMonData(boxMon: *mut u8, field: i32, dataArg: *mut u8) {
    unsafe {
        let mut boxMon = boxMon;
        let mut field = field;
        let mut dataArg = dataArg;
        let mut data: *mut u8 = dataArg;
        let mut substruct0: *mut u8 = core::ptr::null_mut();
        let mut substruct1: *mut u8 = core::ptr::null_mut();
        let mut substruct2: *mut u8 = core::ptr::null_mut();
        let mut substruct3: *mut u8 = core::ptr::null_mut();
        if field > 10i32 {
            substruct0 = (GetSubstruct(boxMon, ((boxMon).cast::<u32>()).read(), 0u8));
            substruct1 = (GetSubstruct(boxMon, ((boxMon).cast::<u32>()).read(), 1u8));
            substruct2 = (GetSubstruct(boxMon, ((boxMon).cast::<u32>()).read(), 2u8));
            substruct3 = (GetSubstruct(boxMon, ((boxMon).cast::<u32>()).read(), 3u8));
            DecryptBoxMon(boxMon);
            if ((CalculateBoxMonChecksum(boxMon)) as i32)
                != ((((boxMon).wrapping_add(28).cast::<u16>()).read()) as i32)
            {
                crate::c::bf_write((boxMon).wrapping_add(19), 0, 1, (1u8) as i32);
                crate::c::bf_write((boxMon).wrapping_add(19), 2, 1, (1u8) as i32);
                crate::c::bf_write((substruct3).wrapping_add(7), 6, 1, (1u32) as i32);
                EncryptBoxMon(boxMon);
                return;
            }
        }
        'l1: {
            let __sw1 = field;
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 25i32
                || __sw1 == 21i32
                || __sw1 == 32i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 17i32
                || __sw1 == 18i32
                || __sw1 == 19i32
                || __sw1 == 20i32
                || __sw1 == 26i32
                || __sw1 == 27i32
                || __sw1 == 28i32
                || __sw1 == 29i32
                || __sw1 == 30i32
                || __sw1 == 31i32
                || __sw1 == 22i32
                || __sw1 == 23i32
                || __sw1 == 24i32
                || __sw1 == 33i32
                || __sw1 == 47i32
                || __sw1 == 48i32
                || __sw1 == 34i32
                || __sw1 == 35i32
                || __sw1 == 36i32
                || __sw1 == 37i32
                || __sw1 == 38i32
                || __sw1 == 49i32
                || __sw1 == 39i32
                || __sw1 == 40i32
                || __sw1 == 41i32
                || __sw1 == 42i32
                || __sw1 == 43i32
                || __sw1 == 44i32
                || __sw1 == 45i32
                || __sw1 == 46i32
                || __sw1 == 50i32
                || __sw1 == 51i32
                || __sw1 == 52i32
                || __sw1 == 53i32
                || __sw1 == 54i32
                || __sw1 == 67i32
                || __sw1 == 68i32
                || __sw1 == 69i32
                || __sw1 == 70i32
                || __sw1 == 71i32
                || __sw1 == 72i32
                || __sw1 == 73i32
                || __sw1 == 74i32
                || __sw1 == 75i32
                || __sw1 == 76i32
                || __sw1 == 77i32
                || __sw1 == 78i32
                || __sw1 == 79i32
                || __sw1 == 80i32
                || __sw1 == 66i32;
            if __sw1 == 0i32 {
                ((boxMon).cast::<u32>()).write(
                    (((((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                    .wrapping_add((((((data).wrapping_offset(2)).read()) as i32) << 16)))
                    .wrapping_add((((((data).wrapping_offset(3)).read()) as i32) << 24)))
                        as u32),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((boxMon).wrapping_add(4).cast::<u32>()).write(
                    (((((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                    .wrapping_add((((((data).wrapping_offset(2)).read()) as i32) << 16)))
                    .wrapping_add((((((data).wrapping_offset(3)).read()) as i32) << 24)))
                        as u32),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    let mut i: i32 = 0i32;
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 10i32) {
                                break 'l2;
                            }
                            'l3: {
                                ((((boxMon).wrapping_add(8)).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .write(((data).wrapping_offset((i) as isize)).read());
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 3i32 {
                ((boxMon).wrapping_add(18)).write((data).read());
                break 'l1;
            }
            if __sw1 == 4i32 {
                crate::c::bf_write((boxMon).wrapping_add(19), 0, 1, ((data).read()) as i32);
                break 'l1;
            }
            if __sw1 == 5i32 {
                crate::c::bf_write((boxMon).wrapping_add(19), 1, 1, ((data).read()) as i32);
                break 'l1;
            }
            if __sw1 == 6i32 {
                crate::c::bf_write((boxMon).wrapping_add(19), 2, 1, ((data).read()) as i32);
                break 'l1;
            }
            if __sw1 == 7i32 {
                {
                    let mut i: i32 = 0i32;
                    {
                        i = 0i32;
                        'l4: loop {
                            if !(i < 7i32) {
                                break 'l4;
                            }
                            'l5: {
                                ((((boxMon).wrapping_add(20)).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .write(((data).wrapping_offset((i) as isize)).read());
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 8i32 {
                ((boxMon).wrapping_add(27)).write((data).read());
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((boxMon).wrapping_add(28).cast::<u16>()).write(
                    (((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                        as u16),
                );
                break 'l1;
            }
            if __sw1 == 10i32 {
                ((boxMon).wrapping_add(30).cast::<u16>()).write(
                    (((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                        as u16),
                );
                break 'l1;
            }
            if __sw1 == 11i32 {
                {
                    ((substruct0).cast::<u16>()).write(
                        (((((data).read()) as i32)
                            .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                            as u16),
                    );
                    if (((substruct0).cast::<u16>()).read()) != 0 {
                        crate::c::bf_write((boxMon).wrapping_add(19), 1, 1, (1u8) as i32);
                    } else {
                        crate::c::bf_write((boxMon).wrapping_add(19), 1, 1, (0u8) as i32);
                    }
                    break 'l1;
                }
            }
            if __sw1 == 12i32 {
                ((substruct0).wrapping_add(2).cast::<u16>()).write(
                    (((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                        as u16),
                );
                break 'l1;
            }
            if __sw1 == 25i32 {
                ((substruct0).wrapping_add(4).cast::<u32>()).write(
                    (((((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                    .wrapping_add((((((data).wrapping_offset(2)).read()) as i32) << 16)))
                    .wrapping_add((((((data).wrapping_offset(3)).read()) as i32) << 24)))
                        as u32),
                );
                break 'l1;
            }
            if __sw1 == 21i32 {
                ((substruct0).wrapping_add(8)).write((data).read());
                break 'l1;
            }
            if __sw1 == 32i32 {
                ((substruct0).wrapping_add(9)).write((data).read());
                break 'l1;
            }
            if __sw1 == 13i32 || __sw1 == 14i32 || __sw1 == 15i32 || __sw1 == 16i32 {
                (((substruct1).cast::<u16>())
                    .wrapping_offset(((field).wrapping_sub(13i32)) as isize))
                .write(
                    (((((data).read()) as i32)
                        .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) << 8)))
                        as u16),
                );
                break 'l1;
            }
            if __sw1 == 17i32 || __sw1 == 18i32 || __sw1 == 19i32 || __sw1 == 20i32 {
                ((((substruct1).wrapping_add(8)).cast::<u8>())
                    .wrapping_offset(((field).wrapping_sub(17i32)) as isize))
                .write((data).read());
                break 'l1;
            }
            if __sw1 == 26i32 {
                (substruct2).write((data).read());
                break 'l1;
            }
            if __sw1 == 27i32 {
                ((substruct2).wrapping_add(1)).write((data).read());
                break 'l1;
            }
            if __sw1 == 28i32 {
                ((substruct2).wrapping_add(2)).write((data).read());
                break 'l1;
            }
            if __sw1 == 29i32 {
                ((substruct2).wrapping_add(3)).write((data).read());
                break 'l1;
            }
            if __sw1 == 30i32 {
                ((substruct2).wrapping_add(4)).write((data).read());
                break 'l1;
            }
            if __sw1 == 31i32 {
                ((substruct2).wrapping_add(5)).write((data).read());
                break 'l1;
            }
            if __sw1 == 22i32 {
                ((substruct2).wrapping_add(6)).write((data).read());
                break 'l1;
            }
            if __sw1 == 23i32 {
                ((substruct2).wrapping_add(7)).write((data).read());
                break 'l1;
            }
            if __sw1 == 24i32 {
                ((substruct2).wrapping_add(8)).write((data).read());
                break 'l1;
            }
            if __sw1 == 33i32 {
                ((substruct2).wrapping_add(9)).write((data).read());
                break 'l1;
            }
            if __sw1 == 47i32 {
                ((substruct2).wrapping_add(10)).write((data).read());
                break 'l1;
            }
            if __sw1 == 48i32 {
                ((substruct2).wrapping_add(11)).write((data).read());
                break 'l1;
            }
            if __sw1 == 34i32 {
                (substruct3).write((data).read());
                break 'l1;
            }
            if __sw1 == 35i32 {
                'l6: loop {
                    'l7: {
                        if 1u32 == 1u32 {
                            ((substruct3).wrapping_add(1)).write((data).read());
                        } else {
                            if 1u32 == 2u32 {
                                ((substruct3).wrapping_add(1)).write(
                                    (((((data).read()) as i32).wrapping_add(
                                        (((((data).wrapping_offset(1)).read()) as i32) << 8),
                                    )) as u8),
                                );
                            } else {
                                if 1u32 == 4u32 {
                                    ((substruct3).wrapping_add(1)).write(
                                        (((((((data).read()) as i32).wrapping_add(
                                            (((((data).wrapping_offset(1)).read()) as i32) << 8),
                                        ))
                                        .wrapping_add(
                                            (((((data).wrapping_offset(2)).read()) as i32) << 16),
                                        ))
                                        .wrapping_add(
                                            (((((data).wrapping_offset(3)).read()) as i32) << 24),
                                        )) as u8),
                                    );
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l6;
                    }
                }
                break 'l1;
            }
            if __sw1 == 36i32 {
                {
                    let mut metLevel: u8 = (data).read();
                    crate::c::bf_write(
                        (substruct3).wrapping_add(2),
                        0,
                        7,
                        ((metLevel) as u16) as i32,
                    );
                    break 'l1;
                }
            }
            if __sw1 == 37i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(2),
                    7,
                    4,
                    (((data).read()) as u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 38i32 {
                {
                    let mut pokeball: u8 = (data).read();
                    crate::c::bf_write(
                        (substruct3).wrapping_add(3),
                        3,
                        4,
                        ((pokeball) as u16) as i32,
                    );
                    break 'l1;
                }
            }
            if __sw1 == 49i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(3),
                    7,
                    1,
                    (((data).read()) as u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 39i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(4),
                    0,
                    5,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 40i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(4),
                    5,
                    5,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 41i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(5),
                    2,
                    5,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 42i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(5),
                    7,
                    5,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 43i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(6),
                    4,
                    5,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 44i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(7),
                    1,
                    5,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 45i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(7),
                    6,
                    1,
                    (((data).read()) as u32) as i32,
                );
                if (crate::c::bf_read((substruct3).wrapping_add(7), 6, 1, false) as u32) != 0 {
                    crate::c::bf_write((boxMon).wrapping_add(19), 2, 1, (1u8) as i32);
                } else {
                    crate::c::bf_write((boxMon).wrapping_add(19), 2, 1, (0u8) as i32);
                }
                break 'l1;
            }
            if __sw1 == 46i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(7),
                    7,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 50i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(8),
                    0,
                    3,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 51i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(8),
                    3,
                    3,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 52i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(8),
                    6,
                    3,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 53i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(9),
                    1,
                    3,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 54i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(9),
                    4,
                    3,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 67i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(9),
                    7,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 68i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(10),
                    0,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 69i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(10),
                    1,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 70i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(10),
                    2,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 71i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(10),
                    3,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 72i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(10),
                    4,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 73i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(10),
                    5,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 74i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(10),
                    6,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 75i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(10),
                    7,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 76i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(11),
                    0,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 77i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(11),
                    1,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 78i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(11),
                    2,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 79i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(11),
                    3,
                    4,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 80i32 {
                crate::c::bf_write(
                    (substruct3).wrapping_add(11),
                    7,
                    1,
                    (((data).read()) as u32) as i32,
                );
                break 'l1;
            }
            if __sw1 == 66i32 {
                {
                    let mut ivs: u32 = (((((((data).read()) as i32)
                        | (((((data).wrapping_offset(1)).read()) as i32) << 8))
                        | (((((data).wrapping_offset(2)).read()) as i32) << 16))
                        | (((((data).wrapping_offset(3)).read()) as i32) << 24))
                        as u32);
                    crate::c::bf_write((substruct3).wrapping_add(4), 0, 5, (ivs & 31u32) as i32);
                    crate::c::bf_write(
                        (substruct3).wrapping_add(4),
                        5,
                        5,
                        ((ivs >> 5) & 31u32) as i32,
                    );
                    crate::c::bf_write(
                        (substruct3).wrapping_add(5),
                        2,
                        5,
                        ((ivs >> 10) & 31u32) as i32,
                    );
                    crate::c::bf_write(
                        (substruct3).wrapping_add(5),
                        7,
                        5,
                        ((ivs >> 15) & 31u32) as i32,
                    );
                    crate::c::bf_write(
                        (substruct3).wrapping_add(6),
                        4,
                        5,
                        ((ivs >> 20) & 31u32) as i32,
                    );
                    crate::c::bf_write(
                        (substruct3).wrapping_add(7),
                        1,
                        5,
                        ((ivs >> 25) & 31u32) as i32,
                    );
                    break 'l1;
                }
            }
            if !__matched {
                break 'l1;
            }
        }
        if field > 10i32 {
            ((boxMon).wrapping_add(28).cast::<u16>()).write(CalculateBoxMonChecksum(boxMon));
            EncryptBoxMon(boxMon);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyMon(dest: *mut u8, src: *mut u8, size: u32) {
    unsafe {
        let mut dest = dest;
        let mut src = src;
        let mut size = size;
        crate::c::memcpy(dest, src, size);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveMonToPlayer(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut i: i32 = 0i32;
        SetMonData(
            mon,
            7i32,
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        SetMonData(
            mon,
            49i32,
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8),
        );
        SetMonData(
            mon,
            1i32,
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10)).cast::<u8>(),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if GetMonData3(
                        (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 100),
                        11i32,
                        core::ptr::null_mut(),
                    ) == 0u32
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i >= 6i32 {
            return CopyMonToPC(mon);
        }
        CopyMon(
            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                .wrapping_offset((i) as isize * 100),
            mon,
            100u32,
        );
        ((&raw mut gPlayerPartyCount).cast::<u8>().cast::<u8>())
            .write((((i).wrapping_add(1i32)) as u8));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CopyMonToPC(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut boxNo: i32 = 0i32;
        let mut boxPos: i32 = 0i32;
        SetPCBoxToSendMon(((VarGet(16438u16)) as u8));
        boxNo = ((StorageGetCurrentBox()) as i32);
        'l1: loop {
            'l2: {
                {
                    boxPos = 0i32;
                    'l3: loop {
                        if !(boxPos < 30i32) {
                            break 'l3;
                        }
                        'l4: {
                            let mut checkingMon: *mut u8 =
                                GetBoxedMonPtr(((boxNo) as u8), ((boxPos) as u8));
                            if GetBoxMonData3(checkingMon, 11i32, core::ptr::null_mut()) == 0u32 {
                                MonRestorePP(mon);
                                CopyMon(checkingMon, (mon), 80u32);
                                ((&raw mut gSpecialVar_MonBoxId).cast::<u16>())
                                    .write(((boxNo) as u16));
                                ((&raw mut gSpecialVar_MonBoxPos).cast::<u16>())
                                    .write(((boxPos) as u16));
                                if ((GetPCBoxToSendMon()) as i32) != boxNo {
                                    FlagClear(2263u16);
                                }
                                VarSet(16438u16, ((boxNo) as u16));
                                return 1u8;
                            }
                        }
                        boxPos = (boxPos).wrapping_add(1);
                    }
                }
                boxNo = (boxNo).wrapping_add(1);
                if boxNo == 14i32 {
                    boxNo = 0i32;
                }
            }
            if !(boxNo != ((StorageGetCurrentBox()) as i32)) {
                break 'l1;
            }
        }
        return 2u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculatePlayerPartyCount() -> u8 {
    unsafe {
        ((&raw mut gPlayerPartyCount).cast::<u8>().cast::<u8>()).write(0u8);
        'l1: loop {
            if !((((((&raw mut gPlayerPartyCount).cast::<u8>().cast::<u8>()).read()) as i32)
                < 6i32)
                && (GetMonData3(
                    (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gPlayerPartyCount).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize
                            * 100,
                    ),
                    11i32,
                    core::ptr::null_mut(),
                ) != 0u32))
            {
                break 'l1;
            }
            let __p1 = (&raw mut gPlayerPartyCount).cast::<u8>().cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return ((&raw mut gPlayerPartyCount).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculateEnemyPartyCount() -> u8 {
    unsafe {
        ((&raw mut gEnemyPartyCount).cast::<u8>().cast::<u8>()).write(0u8);
        'l1: loop {
            if !((((((&raw mut gEnemyPartyCount).cast::<u8>().cast::<u8>()).read()) as i32) < 6i32)
                && (GetMonData3(
                    (((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gEnemyPartyCount).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize
                            * 100,
                    ),
                    11i32,
                    core::ptr::null_mut(),
                ) != 0u32))
            {
                break 'l1;
            }
            let __p1 = (&raw mut gEnemyPartyCount).cast::<u8>().cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return ((&raw mut gEnemyPartyCount).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonsStateToDoubles() -> u8 {
    unsafe {
        let mut aliveCount: i32 = 0i32;
        let mut i: i32 = 0i32;
        CalculatePlayerPartyCount();
        if ((((&raw mut gPlayerPartyCount).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return ((&raw mut gPlayerPartyCount).cast::<u8>().cast::<u8>()).read();
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gPlayerPartyCount).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((GetMonData3(
                        (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 100),
                        65i32,
                        core::ptr::null_mut(),
                    ) != 412u32)
                        && (GetMonData3(
                            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            57i32,
                            core::ptr::null_mut(),
                        ) != 0u32))
                        && (GetMonData3(
                            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            65i32,
                            core::ptr::null_mut(),
                        ) != 0u32)
                    {
                        aliveCount = (aliveCount).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((if aliveCount > 1i32 { 0i32 } else { 2i32 }) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonsStateToDoubles_2() -> u8 {
    unsafe {
        let mut aliveCount: i32 = 0i32;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut species: u32 = GetMonData3(
                        (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 100),
                        65i32,
                        core::ptr::null_mut(),
                    );
                    if ((species != 412u32) && (species != 0u32))
                        && (GetMonData3(
                            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            57i32,
                            core::ptr::null_mut(),
                        ) != 0u32)
                    {
                        aliveCount = (aliveCount).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if aliveCount == 1i32 {
            return 1u8;
        }
        return ((if aliveCount > 1i32 { 0i32 } else { 2i32 }) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAbilityBySpecies(species: u16, abilityNum: u8) -> u8 {
    unsafe {
        let mut species = species;
        let mut abilityNum = abilityNum;
        if (abilityNum) != 0 {
            ((&raw mut gLastUsedAbility).cast::<u8>()).write(
                (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .wrapping_add(22))
                .cast::<u8>())
                .wrapping_offset(1))
                .read(),
            );
        } else {
            ((&raw mut gLastUsedAbility).cast::<u8>()).write(
                ((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .wrapping_add(22))
                .cast::<u8>())
                .read(),
            );
        }
        return ((&raw mut gLastUsedAbility).cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonAbility(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut species: u16 = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
        let mut abilityNum: u8 = ((GetMonData3(mon, 46i32, core::ptr::null_mut())) as u8);
        return GetAbilityBySpecies(species, abilityNum);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateSecretBaseEnemyParty(secretBaseRecord: *mut u8) {
    unsafe {
        let mut secretBaseRecord = secretBaseRecord;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        ZeroEnemyPartyMons();
        ((((&raw mut gBattleResources).cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read()
            .cast::<crate::c::Rec4<160>>()
            .write_unaligned(
                secretBaseRecord
                    .cast::<crate::c::Rec4<160>>()
                    .read_unaligned(),
            );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(52))
                    .wrapping_add(72))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read())
                        != 0
                    {
                        CreateMon(
                            (((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(52))
                            .wrapping_add(72))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read(),
                            (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(52))
                            .wrapping_add(96))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                            15u8,
                            1u8,
                            ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(52))
                            .cast::<u32>())
                            .wrapping_offset((i) as isize))
                            .read(),
                            2u8,
                            0u32,
                        );
                        SetMonData(
                            (((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            12i32,
                            (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(52))
                            .wrapping_add(84))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .cast::<u8>(),
                        );
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < 6i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    SetMonData(
                                        (((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>())
                                            .wrapping_offset((i) as isize * 100),
                                        (26i32).wrapping_add(j),
                                        ((((((((&raw mut gBattleResources).cast::<*mut u8>())
                                            .read())
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(52))
                                        .wrapping_add(102))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        {
                            j = 0i32;
                            'l5: loop {
                                if !(j < 4i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    SetMonData(
                                        (((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>())
                                            .wrapping_offset((i) as isize * 100),
                                        (13i32).wrapping_add(j),
                                        (((((((((&raw mut gBattleResources)
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(52))
                                        .wrapping_add(24))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                        ))
                                        .cast::<u8>(),
                                    );
                                    SetMonData(
                                        (((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>())
                                            .wrapping_offset((i) as isize * 100),
                                        (17i32).wrapping_add(j),
                                        ((((&raw const gBattleMoves).cast::<u8>().cast_mut())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            (((((((((((&raw mut gBattleResources)
                                                .cast::<*mut u8>())
                                            .read())
                                            .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(52))
                                            .wrapping_add(24))
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                (((i).wrapping_mul(4i32)).wrapping_add(j)) as isize,
                                            ))
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
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseTrainerPicIndex() -> u8 {
    unsafe {
        let mut facilityClass: u8 = ((((((&raw const sSecretBaseFacilityClasses)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                (((((&raw mut gBattleResources).cast::<*mut u8>()).read()).cast::<*mut u8>())
                    .read())
                .wrapping_add(1),
                4,
                1,
                false,
            ) as u8) as i32) as isize
                * 5,
        ))
        .cast::<u8>())
        .wrapping_offset(
            (crate::c::rem_i32(
                (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(9))
                .cast::<u8>())
                .read()) as i32),
                5i32,
            )) as isize,
        ))
        .read();
        return ((((&raw const gFacilityClassToPicIndex)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(((facilityClass) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseTrainerClass() -> u8 {
    unsafe {
        let mut facilityClass: u8 = ((((((&raw const sSecretBaseFacilityClasses)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                (((((&raw mut gBattleResources).cast::<*mut u8>()).read()).cast::<*mut u8>())
                    .read())
                .wrapping_add(1),
                4,
                1,
                false,
            ) as u8) as i32) as isize
                * 5,
        ))
        .cast::<u8>())
        .wrapping_offset(
            (crate::c::rem_i32(
                (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(9))
                .cast::<u8>())
                .read()) as i32),
                5i32,
            )) as isize,
        ))
        .read();
        return ((((&raw const gFacilityClassToTrainerClass)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(((facilityClass) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPlayerPartyAndPokemonStorageFull() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if GetMonData3(
                        (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 100),
                        11i32,
                        core::ptr::null_mut(),
                    ) == 0u32
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return IsPokemonStorageFull();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPokemonStorageFull() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 14i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 30i32) {
                                break 'l3;
                            }
                            'l4: {
                                if GetBoxMonDataAt(((i) as u8), ((j) as u8), 11i32) == 0u32 {
                                    return 0u8;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSpeciesName(name: *mut u8, species: u16) {
    unsafe {
        let mut name = name;
        let mut species = species;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i <= 10i32) {
                    break 'l1;
                }
                'l2: {
                    if ((species) as i32) > 412i32 {
                        ((name).wrapping_offset((i) as isize)).write(
                            ((((&raw mut gSpeciesNames).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read(),
                        );
                    } else {
                        ((name).wrapping_offset((i) as isize)).write(
                            (((((&raw mut gSpeciesNames).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 11))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    if ((((name).wrapping_offset((i) as isize)).read()) as i32) == 255i32 {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((name).wrapping_offset((i) as isize)).write(255u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculatePPWithBonus(r#move: u16, ppBonuses: u8, moveIndex: u8) -> u8 {
    unsafe {
        let mut r#move = r#move;
        let mut ppBonuses = ppBonuses;
        let mut moveIndex = moveIndex;
        let mut basePP: u8 = (((((&raw const gBattleMoves).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((r#move) as i32) as isize * 12))
        .wrapping_add(4))
        .read();
        return ((((basePP) as i32).wrapping_add(crate::c::div_i32(
            (((basePP) as i32).wrapping_mul(20i32)).wrapping_mul(crate::c::shr_i32(
                (((((((&raw const gPPUpGetMask).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((moveIndex) as i32) as isize))
                .read()) as i32)
                    & ((ppBonuses) as i32)),
                (((2i32).wrapping_mul(((moveIndex) as i32))) as u32),
            )),
            100i32,
        ))) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveMonPPBonus(mon: *mut u8, moveIndex: u8) {
    unsafe {
        let mut mon = mon;
        let mut moveIndex = moveIndex;
        let mut ppBonuses: u8 = ((GetMonData3(mon, 21i32, core::ptr::null_mut())) as u8);
        ppBonuses = ((((ppBonuses) as i32)
            & ((((((&raw const gPPUpClearMask).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((moveIndex) as i32) as isize))
            .read()) as i32)) as u8);
        SetMonData(mon, 21i32, &raw mut ppBonuses);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveBattleMonPPBonus(mon: *mut u8, moveIndex: u8) {
    unsafe {
        let mut mon = mon;
        let mut moveIndex = moveIndex;
        let __p1 = (mon).wrapping_add(59);
        (__p1).write(
            (((((__p1).read()) as i32)
                & ((((((&raw const gPPUpClearMask).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((moveIndex) as i32) as isize))
                .read()) as i32)) as u8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyPlayerPartyMonToBattleData(battler: u8, partyIndex: u8) {
    unsafe {
        let mut battler = battler;
        let mut partyIndex = partyIndex;
        let mut hpSwitchout: *mut u16 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut nickname = crate::ffi::Align4([0u8; 20]);
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .cast::<u16>())
        .write(
            ((GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                11i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(46)
            .cast::<u16>())
        .write(
            ((GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                12i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(12))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((GetMonData3(
                            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((partyIndex) as i32) as isize * 100),
                            (13i32).wrapping_add(i),
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(36))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((GetMonData3(
                            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((partyIndex) as i32) as isize * 100),
                            (17i32).wrapping_add(i),
                            core::ptr::null_mut(),
                        )) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(59))
        .write(
            ((GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                21i32,
                core::ptr::null_mut(),
            )) as u8),
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(43))
        .write(
            ((GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                32i32,
                core::ptr::null_mut(),
            )) as u8),
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(68)
            .cast::<u32>())
        .write(GetMonData3(
            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            25i32,
            core::ptr::null_mut(),
        ));
        crate::c::bf_write(
            (((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(20),
            0,
            5,
            (GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                39i32,
                core::ptr::null_mut(),
            )) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(20),
            5,
            5,
            (GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                40i32,
                core::ptr::null_mut(),
            )) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(21),
            2,
            5,
            (GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                41i32,
                core::ptr::null_mut(),
            )) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(21),
            7,
            5,
            (GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                42i32,
                core::ptr::null_mut(),
            )) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(22),
            4,
            5,
            (GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                43i32,
                core::ptr::null_mut(),
            )) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(23),
            1,
            5,
            (GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                44i32,
                core::ptr::null_mut(),
            )) as i32,
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(72)
            .cast::<u32>())
        .write(GetMonData3(
            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            0i32,
            core::ptr::null_mut(),
        ));
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(76)
            .cast::<u32>())
        .write(GetMonData3(
            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            55i32,
            core::ptr::null_mut(),
        ));
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(42))
        .write(
            ((GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                56i32,
                core::ptr::null_mut(),
            )) as u8),
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(40)
            .cast::<u16>())
        .write(
            ((GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                57i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(44)
            .cast::<u16>())
        .write(
            ((GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                58i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(2)
            .cast::<u16>())
        .write(
            ((GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                59i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(4)
            .cast::<u16>())
        .write(
            ((GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                60i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(6)
            .cast::<u16>())
        .write(
            ((GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                61i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(8)
            .cast::<u16>())
        .write(
            ((GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                62i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(10)
            .cast::<u16>())
        .write(
            ((GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                63i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        crate::c::bf_write(
            (((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(23),
            6,
            1,
            (GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                45i32,
                core::ptr::null_mut(),
            )) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(23),
            7,
            1,
            (GetMonData3(
                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
                46i32,
                core::ptr::null_mut(),
            )) as i32,
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(84)
            .cast::<u32>())
        .write(GetMonData3(
            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            1i32,
            core::ptr::null_mut(),
        ));
        (((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(33))
        .cast::<u8>())
        .write(
            ((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 28,
                ))
            .wrapping_add(6))
            .cast::<u8>())
            .read(),
        );
        ((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 88))
        .wrapping_add(33))
        .cast::<u8>())
        .wrapping_offset(1))
        .write(
            (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 28,
                ))
            .wrapping_add(6))
            .cast::<u8>())
            .wrapping_offset(1))
            .read(),
        );
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(32))
        .write(GetAbilityBySpecies(
            ((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .cast::<u16>())
            .read(),
            ((crate::c::bf_read(
                (((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(23),
                7,
                1,
                false,
            ) as u32) as u8),
        ));
        GetMonData3(
            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            2i32,
            (&raw mut nickname).cast::<u8>(),
        );
        StringCopy_Nickname(
            ((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(48))
            .cast::<u8>(),
            (&raw mut nickname).cast::<u8>(),
        );
        GetMonData3(
            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100),
            7i32,
            ((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(60))
            .cast::<u8>(),
        );
        hpSwitchout = (((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(168))
            .cast::<u16>())
        .wrapping_offset(((GetBattlerSide(battler)) as i32) as isize);
        (hpSwitchout).write(
            ((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(40)
            .cast::<u16>())
            .read(),
        );
        {
            i = 0i32;
            'l3: loop {
                if !(i < 8i32) {
                    break 'l3;
                }
                'l4: {
                    ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(24))
                    .cast::<i8>())
                    .wrapping_offset((i) as isize))
                    .write(6i8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(80)
            .cast::<u32>())
        .write(0u32);
        UpdateSentPokesToOpponentValue(battler);
        ClearTemporarySpeciesSpriteData(battler, 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ExecuteTableBasedItemEffect(
    mon: *mut u8,
    item: u16,
    partyIndex: u8,
    moveIndex: u8,
) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut item = item;
        let mut partyIndex = partyIndex;
        let mut moveIndex = moveIndex;
        return PokemonUseItemEffects(mon, item, partyIndex, moveIndex, 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokemonUseItemEffects(
    mon: *mut u8,
    item: u16,
    partyIndex: u8,
    moveIndex: u8,
    usedByAI: u8,
) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut item = item;
        let mut partyIndex = partyIndex;
        let mut moveIndex = moveIndex;
        let mut usedByAI = usedByAI;
        let mut dataUnsigned: u32 = 0u32;
        let mut dataSigned: i32 = 0i32;
        let mut friendship: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut retVal: u8 = 1u8;
        let mut itemEffect: *mut u8 = core::ptr::null_mut();
        let mut itemEffectParam: u8 = 6u8;
        let mut temp1: u32 = 0u32;
        let mut temp2: u32 = 0u32;
        let mut friendshipChange: i8 = 0i8;
        let mut holdEffect: u8 = 0u8;
        let mut battler: u8 = 4u8;
        let mut friendshipOnly: u32 = 0u32;
        let mut heldItem: u16 = 0u16;
        let mut effectFlags: u8 = 0u8;
        let mut evChange: i8 = 0i8;
        let mut evCount: u16 = 0u16;
        heldItem = ((GetMonData3(mon, 12i32, core::ptr::null_mut())) as u16);
        if ((heldItem) as i32) == 175i32 {
            if (crate::c::bf_read(
                ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                1,
                1,
                false,
            ) as u8)
                != 0
            {
                holdEffect = ((((&raw mut gEnigmaBerries).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerInMenuId).cast::<u8>()).read()) as i32) as isize * 28,
                ))
                .wrapping_add(7))
                .read();
            } else {
                holdEffect = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(12792))
                .wrapping_add(46))
                .read();
            }
        } else {
            holdEffect = GetItemHoldEffect(heldItem);
        }
        ((&raw mut gPotentialItemEffectBattler).cast::<u8>())
            .write(((&raw mut gBattlerInMenuId).cast::<u8>()).read());
        if (crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0
        {
            ((&raw mut gActiveBattler).cast::<u8>())
                .write(((&raw mut gBattlerInMenuId).cast::<u8>()).read());
            i = ((((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32)
                != 0i32) as i32);
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                if ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset((i) as isize))
                .read()) as i32)
                    == ((partyIndex) as i32)
                {
                    battler = ((i) as u8);
                    break 'l1;
                }
                i = (i).wrapping_add(2i32);
            }
        } else {
            ((&raw mut gActiveBattler).cast::<u8>()).write(0u8);
            battler = 4u8;
        }
        if !((((item) as i32) >= 13i32) && (((item) as i32) <= 178i32)) {
            return 1u8;
        }
        if (((((((&raw const gItemEffectTable)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset((((item) as i32).wrapping_sub(13i32)) as isize))
        .read()) as usize)
            == 0usize)
            && (((item) as i32) != 175i32)
        {
            return 1u8;
        }
        if ((item) as i32) == 175i32 {
            if (crate::c::bf_read(
                ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                1,
                1,
                false,
            ) as u8)
                != 0
            {
                itemEffect = ((((&raw mut gEnigmaBerries).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 28,
                ))
                .wrapping_add(8))
                .cast::<u8>();
            } else {
                itemEffect = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(12792))
                .wrapping_add(28))
                .cast::<u8>();
            }
        } else {
            itemEffect = ((((&raw const gItemEffectTable)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset((((item) as i32).wrapping_sub(13i32)) as isize))
            .read();
        }
        {
            i = 0i32;
            'l2: loop {
                if !(i < 6i32) {
                    break 'l2;
                }
                'l3: {
                    'l4: {
                        let __sw1 = i;
                        if __sw1 == 0i32 {
                            if ((((((((itemEffect).wrapping_offset((i) as isize)).read())
                                as i32)
                                & 128i32)
                                != 0)
                                && ((crate::c::bf_read(
                                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                                    1,
                                    1,
                                    false,
                                ) as u8)
                                    != 0))
                                && (((battler) as i32) != 4i32))
                                && ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>())
                                .read()
                                    & 983040u32)
                                    != 0)
                            {
                                let __p2 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p2).write(((__p2).read() & 4293984255u32));
                                retVal = 0u8;
                            }
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 48i32)
                                != 0)
                                && (!((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(80)
                                .cast::<u32>())
                                .read()
                                    & 1048576u32)
                                    != 0))
                            {
                                let __p3 = (((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p3).write(((__p3).read() | 1048576u32));
                                retVal = 0u8;
                            }
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 15i32)
                                != 0)
                                && (((((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    < 12i32)
                            {
                                let __p4 = (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(1);
                                (__p4).write(
                                    (((((__p4).read()) as i32).wrapping_add(
                                        (((((itemEffect).wrapping_offset((i) as isize)).read())
                                            as i32)
                                            & 15i32),
                                    )) as i8),
                                );
                                if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    > 12i32
                                {
                                    ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(24))
                                    .cast::<i8>())
                                    .wrapping_offset(1))
                                    .write(12i8);
                                }
                                retVal = 0u8;
                            }
                            break 'l4;
                        }
                        if __sw1 == 1i32 {
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 240i32)
                                != 0)
                                && (((((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    < 12i32)
                            {
                                let __p5 = (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(2);
                                (__p5).write(
                                    (((((__p5).read()) as i32).wrapping_add(
                                        ((((((itemEffect).wrapping_offset((i) as isize)).read())
                                            as i32)
                                            & 240i32)
                                            >> 4),
                                    )) as i8),
                                );
                                if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    > 12i32
                                {
                                    ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(24))
                                    .cast::<i8>())
                                    .wrapping_offset(2))
                                    .write(12i8);
                                }
                                retVal = 0u8;
                            }
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 15i32)
                                != 0)
                                && (((((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(3))
                                .read()) as i32)
                                    < 12i32)
                            {
                                let __p6 = (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(3);
                                (__p6).write(
                                    (((((__p6).read()) as i32).wrapping_add(
                                        (((((itemEffect).wrapping_offset((i) as isize)).read())
                                            as i32)
                                            & 15i32),
                                    )) as i8),
                                );
                                if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(3))
                                .read()) as i32)
                                    > 12i32
                                {
                                    ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(24))
                                    .cast::<i8>())
                                    .wrapping_offset(3))
                                    .write(12i8);
                                }
                                retVal = 0u8;
                            }
                            break 'l4;
                        }
                        if __sw1 == 2i32 {
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 240i32)
                                != 0)
                                && (((((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(6))
                                .read()) as i32)
                                    < 12i32)
                            {
                                let __p7 = (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(6);
                                (__p7).write(
                                    (((((__p7).read()) as i32).wrapping_add(
                                        ((((((itemEffect).wrapping_offset((i) as isize)).read())
                                            as i32)
                                            & 240i32)
                                            >> 4),
                                    )) as i8),
                                );
                                if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(6))
                                .read()) as i32)
                                    > 12i32
                                {
                                    ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(24))
                                    .cast::<i8>())
                                    .wrapping_offset(6))
                                    .write(12i8);
                                }
                                retVal = 0u8;
                            }
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 15i32)
                                != 0)
                                && (((((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(4))
                                .read()) as i32)
                                    < 12i32)
                            {
                                let __p8 = (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(4);
                                (__p8).write(
                                    (((((__p8).read()) as i32).wrapping_add(
                                        (((((itemEffect).wrapping_offset((i) as isize)).read())
                                            as i32)
                                            & 15i32),
                                    )) as i8),
                                );
                                if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(24))
                                .cast::<i8>())
                                .wrapping_offset(4))
                                .read()) as i32)
                                    > 12i32
                                {
                                    ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(24))
                                    .cast::<i8>())
                                    .wrapping_offset(4))
                                    .write(12i8);
                                }
                                retVal = 0u8;
                            }
                            break 'l4;
                        }
                        if __sw1 == 3i32 {
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 128i32)
                                != 0)
                                && (((((((&raw mut gSideTimers).cast::<u8>()).wrapping_offset(
                                    ((GetBattlerSide(
                                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                    )) as i32) as isize
                                        * 12,
                                ))
                                .wrapping_add(4))
                                .read()) as i32)
                                    == 0i32)
                            {
                                ((((&raw mut gSideTimers).cast::<u8>()).wrapping_offset(
                                    ((GetBattlerSide(
                                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                    )) as i32) as isize
                                        * 12,
                                ))
                                .wrapping_add(4))
                                .write(5u8);
                                retVal = 0u8;
                            }
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 64i32)
                                != 0)
                                && (GetMonData3(mon, 56i32, core::ptr::null_mut()) != 100u32)
                            {
                                dataUnsigned = ((((((&raw const gExperienceTables)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as i32)
                                            as isize
                                            * 28,
                                    ))
                                    .wrapping_add(19))
                                    .read()) as i32) as isize
                                        * 404,
                                ))
                                .cast::<u32>())
                                .wrapping_offset(
                                    (((GetMonData3(mon, 56i32, core::ptr::null_mut()))
                                        .wrapping_add(1u32))
                                        as i32) as isize,
                                ))
                                .read();
                                SetMonData(mon, 25i32, (&raw mut dataUnsigned).cast::<u8>());
                                CalculateMonStats(mon);
                                retVal = 0u8;
                            }
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 32i32)
                                != 0)
                                && (((HealStatusConditions(
                                    mon,
                                    ((partyIndex) as u32),
                                    7u32,
                                    battler,
                                )) as i32)
                                    == 0i32)
                            {
                                if ((battler) as i32) != 4i32 {
                                    let __p9 = (((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 88))
                                    .wrapping_add(80)
                                    .cast::<u32>();
                                    (__p9).write(((__p9).read() & 4160749567u32));
                                }
                                retVal = 0u8;
                            }
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 16i32)
                                != 0)
                                && (((HealStatusConditions(
                                    mon,
                                    ((partyIndex) as u32),
                                    3976u32,
                                    battler,
                                )) as i32)
                                    == 0i32)
                            {
                                retVal = 0u8;
                            }
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 8i32)
                                != 0)
                                && (((HealStatusConditions(
                                    mon,
                                    ((partyIndex) as u32),
                                    16u32,
                                    battler,
                                )) as i32)
                                    == 0i32)
                            {
                                retVal = 0u8;
                            }
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 4i32)
                                != 0)
                                && (((HealStatusConditions(
                                    mon,
                                    ((partyIndex) as u32),
                                    32u32,
                                    battler,
                                )) as i32)
                                    == 0i32)
                            {
                                retVal = 0u8;
                            }
                            if ((((((itemEffect).wrapping_offset((i) as isize)).read()) as i32)
                                & 2i32)
                                != 0)
                                && (((HealStatusConditions(
                                    mon,
                                    ((partyIndex) as u32),
                                    64u32,
                                    battler,
                                )) as i32)
                                    == 0i32)
                            {
                                retVal = 0u8;
                            }
                            if ((((((((itemEffect).wrapping_offset((i) as isize)).read())
                                as i32)
                                & 1i32)
                                != 0)
                                && ((crate::c::bf_read(
                                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                                    1,
                                    1,
                                    false,
                                ) as u8)
                                    != 0))
                                && (((battler) as i32) != 4i32))
                                && ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>())
                                .read()
                                    & 7u32)
                                    != 0)
                            {
                                let __p10 = (((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((battler) as i32) as isize * 88))
                                .wrapping_add(80)
                                .cast::<u32>();
                                (__p10).write(((__p10).read() & 4294967288u32));
                                retVal = 0u8;
                            }
                            break 'l4;
                        }
                        if __sw1 == 4i32 {
                            effectFlags = ((itemEffect).wrapping_offset((i) as isize)).read();
                            if (((effectFlags) as i32) & 32i32) != 0 {
                                effectFlags = ((((effectFlags) as i32) & (-33i32)) as u8);
                                dataUnsigned = crate::c::shr_u32(
                                    (GetMonData3(mon, 21i32, core::ptr::null_mut())
                                        & ((((((&raw const gPPUpGetMask).cast::<u8>().cast_mut())
                                            .cast::<u8>())
                                        .wrapping_offset(((moveIndex) as i32) as isize))
                                        .read())
                                            as u32)),
                                    ((((moveIndex) as i32).wrapping_mul(2i32)) as u32),
                                );
                                temp1 = ((CalculatePPWithBonus(
                                    ((GetMonData3(
                                        mon,
                                        (13i32).wrapping_add(((moveIndex) as i32)),
                                        core::ptr::null_mut(),
                                    )) as u16),
                                    ((GetMonData3(mon, 21i32, core::ptr::null_mut())) as u8),
                                    moveIndex,
                                )) as u32);
                                if (dataUnsigned <= 2u32) && (temp1 > 4u32) {
                                    dataUnsigned = (GetMonData3(mon, 21i32, core::ptr::null_mut()))
                                        .wrapping_add(
                                            ((((((&raw const gPPUpAddValues)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(((moveIndex) as i32) as isize))
                                            .read())
                                                as u32),
                                        );
                                    SetMonData(mon, 21i32, (&raw mut dataUnsigned).cast::<u8>());
                                    dataUnsigned = ((CalculatePPWithBonus(
                                        ((GetMonData3(
                                            mon,
                                            (13i32).wrapping_add(((moveIndex) as i32)),
                                            core::ptr::null_mut(),
                                        )) as u16),
                                        ((dataUnsigned) as u8),
                                        moveIndex,
                                    )) as u32)
                                        .wrapping_sub(temp1);
                                    dataUnsigned = (GetMonData3(
                                        mon,
                                        (17i32).wrapping_add(((moveIndex) as i32)),
                                        core::ptr::null_mut(),
                                    ))
                                    .wrapping_add(dataUnsigned);
                                    SetMonData(
                                        mon,
                                        (17i32).wrapping_add(((moveIndex) as i32)),
                                        (&raw mut dataUnsigned).cast::<u8>(),
                                    );
                                    retVal = 0u8;
                                }
                            }
                            temp1 = 0u32;
                            'l5: loop {
                                if !(((effectFlags) as i32) != 0i32) {
                                    break 'l5;
                                }
                                if (((effectFlags) as i32) & 1i32) != 0 {
                                    'l6: {
                                        let __sw11 = temp1;
                                        if __sw11 == 0u32 || __sw11 == 1u32 {
                                            evCount = GetMonEVCount(mon);
                                            temp2 = ((((itemEffect).wrapping_offset(
                                                ((itemEffectParam) as i32) as isize,
                                            ))
                                            .read())
                                                as u32);
                                            dataSigned = ((GetMonData3(
                                                mon,
                                                ((((((&raw const sGetMonDataEVConstants)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(((temp1) as i32) as isize))
                                                .read())
                                                    as i32),
                                                core::ptr::null_mut(),
                                            ))
                                                as i32);
                                            evChange = ((temp2) as i8);
                                            if ((evChange) as i32) > 0i32 {
                                                if ((evCount) as i32) >= 510i32 {
                                                    return 1u8;
                                                }
                                                if dataSigned >= 100i32 {
                                                    break 'l6;
                                                }
                                                if (dataSigned).wrapping_add(((evChange) as i32))
                                                    > 100i32
                                                {
                                                    temp2 = ((((100i32).wrapping_sub(
                                                        (dataSigned)
                                                            .wrapping_add(((evChange) as i32)),
                                                    ))
                                                    .wrapping_add(((evChange) as i32)))
                                                        as u32);
                                                } else {
                                                    temp2 = ((evChange) as u32);
                                                }
                                                if ((evCount) as u32).wrapping_add(temp2) > 510u32 {
                                                    temp2 = (temp2).wrapping_add(
                                                        (510u32).wrapping_sub(
                                                            ((evCount) as u32).wrapping_add(temp2),
                                                        ),
                                                    );
                                                }
                                                dataSigned = ((((dataSigned) as u32)
                                                    .wrapping_add(temp2))
                                                    as i32);
                                            } else {
                                                if dataSigned == 0i32 {
                                                    friendshipOnly = 1u32;
                                                    itemEffectParam =
                                                        (itemEffectParam).wrapping_add(1);
                                                    break 'l6;
                                                }
                                                dataSigned =
                                                    (dataSigned).wrapping_add(((evChange) as i32));
                                                if dataSigned < 0i32 {
                                                    dataSigned = 0i32;
                                                }
                                            }
                                            SetMonData(
                                                mon,
                                                ((((((&raw const sGetMonDataEVConstants)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(((temp1) as i32) as isize))
                                                .read())
                                                    as i32),
                                                (&raw mut dataSigned).cast::<u8>(),
                                            );
                                            CalculateMonStats(mon);
                                            itemEffectParam = (itemEffectParam).wrapping_add(1);
                                            retVal = 0u8;
                                            break 'l6;
                                        }
                                        if __sw11 == 2u32 {
                                            if (((effectFlags) as i32) & 16i32) != 0 {
                                                if GetMonData3(mon, 57i32, core::ptr::null_mut())
                                                    != 0u32
                                                {
                                                    itemEffectParam =
                                                        (itemEffectParam).wrapping_add(1);
                                                    break 'l6;
                                                }
                                                if (crate::c::bf_read(
                                                    ((&raw mut gMain).cast::<u8>())
                                                        .wrapping_add(1081),
                                                    1,
                                                    1,
                                                    false,
                                                )
                                                    as u8)
                                                    != 0
                                                {
                                                    if ((battler) as i32) != 4i32 {
                                                        let __p12 = (&raw mut gAbsentBattlerFlags)
                                                            .cast::<u8>();
                                                        (__p12).write(
                                                            (((((__p12).read()) as u32)
                                                                & !(((((&raw mut gBitTable)
                                                                    .cast::<u32>())
                                                                .cast::<u32>())
                                                                .wrapping_offset(
                                                                    ((battler) as i32) as isize,
                                                                ))
                                                                .read()))
                                                                as u8),
                                                        );
                                                        CopyPlayerPartyMonToBattleData(battler, GetPartyIdFromBattlePartyId(((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).wrapping_offset((((battler) as i32)) as isize)).read()) as u8)));
                                                        if (((GetBattlerSide(
                                                            ((&raw mut gActiveBattler)
                                                                .cast::<u8>())
                                                            .read(),
                                                        ))
                                                            as i32)
                                                            == 0i32)
                                                            && ((((((&raw mut gBattleResults)
                                                                .cast::<u8>())
                                                            .wrapping_add(4))
                                                            .read())
                                                                as i32)
                                                                < 255i32)
                                                        {
                                                            let __p13 = ((&raw mut gBattleResults)
                                                                .cast::<u8>())
                                                            .wrapping_add(4);
                                                            (__p13).write(
                                                                ((__p13).read()).wrapping_add(1),
                                                            );
                                                        }
                                                    } else {
                                                        let __p14 = (&raw mut gAbsentBattlerFlags)
                                                            .cast::<u8>();
                                                        (__p14).write(
                                                            (((((__p14).read()) as u32)
                                                                & !(((((&raw mut gBitTable)
                                                                    .cast::<u32>())
                                                                .cast::<u32>())
                                                                .wrapping_offset(
                                                                    (((((&raw mut gActiveBattler)
                                                                        .cast::<u8>())
                                                                    .read())
                                                                        as i32)
                                                                        ^ 2i32)
                                                                        as isize,
                                                                ))
                                                                .read()))
                                                                as u8),
                                                        );
                                                        if (((GetBattlerSide(
                                                            ((&raw mut gActiveBattler)
                                                                .cast::<u8>())
                                                            .read(),
                                                        ))
                                                            as i32)
                                                            == 0i32)
                                                            && ((((((&raw mut gBattleResults)
                                                                .cast::<u8>())
                                                            .wrapping_add(4))
                                                            .read())
                                                                as i32)
                                                                < 255i32)
                                                        {
                                                            let __p15 = ((&raw mut gBattleResults)
                                                                .cast::<u8>())
                                                            .wrapping_add(4);
                                                            (__p15).write(
                                                                ((__p15).read()).wrapping_add(1),
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                if GetMonData3(mon, 57i32, core::ptr::null_mut())
                                                    == 0u32
                                                {
                                                    itemEffectParam =
                                                        (itemEffectParam).wrapping_add(1);
                                                    break 'l6;
                                                }
                                            }
                                            dataUnsigned = ((((itemEffect).wrapping_offset(
                                                (({
                                                    let __t16 = itemEffectParam;
                                                    itemEffectParam =
                                                        (itemEffectParam).wrapping_add(1);
                                                    __t16
                                                })
                                                    as i32)
                                                    as isize,
                                            ))
                                            .read())
                                                as u32);
                                            'l7: {
                                                let __sw17 = dataUnsigned;
                                                if __sw17 == 255u32 {
                                                    dataUnsigned = (GetMonData3(
                                                        mon,
                                                        58i32,
                                                        core::ptr::null_mut(),
                                                    ))
                                                    .wrapping_sub(GetMonData3(
                                                        mon,
                                                        57i32,
                                                        core::ptr::null_mut(),
                                                    ));
                                                    break 'l7;
                                                }
                                                if __sw17 == 254u32 {
                                                    dataUnsigned = crate::c::div_u32(
                                                        GetMonData3(
                                                            mon,
                                                            58i32,
                                                            core::ptr::null_mut(),
                                                        ),
                                                        2u32,
                                                    );
                                                    if dataUnsigned == 0u32 {
                                                        dataUnsigned = 1u32;
                                                    }
                                                    break 'l7;
                                                }
                                                if __sw17 == 253u32 {
                                                    dataUnsigned =
                                                        (((((&raw mut gBattleScripting)
                                                            .cast::<u8>())
                                                        .wrapping_add(35))
                                                        .read())
                                                            as u32);
                                                    break 'l7;
                                                }
                                            }
                                            if GetMonData3(mon, 58i32, core::ptr::null_mut())
                                                != GetMonData3(mon, 57i32, core::ptr::null_mut())
                                            {
                                                if !((usedByAI) != 0) {
                                                    dataUnsigned = (GetMonData3(
                                                        mon,
                                                        57i32,
                                                        core::ptr::null_mut(),
                                                    ))
                                                    .wrapping_add(dataUnsigned);
                                                    if dataUnsigned
                                                        > GetMonData3(
                                                            mon,
                                                            58i32,
                                                            core::ptr::null_mut(),
                                                        )
                                                    {
                                                        dataUnsigned = GetMonData3(
                                                            mon,
                                                            58i32,
                                                            core::ptr::null_mut(),
                                                        );
                                                    }
                                                    SetMonData(
                                                        mon,
                                                        57i32,
                                                        (&raw mut dataUnsigned).cast::<u8>(),
                                                    );
                                                    if ((crate::c::bf_read(
                                                        ((&raw mut gMain).cast::<u8>())
                                                            .wrapping_add(1081),
                                                        1,
                                                        1,
                                                        false,
                                                    )
                                                        as u8)
                                                        != 0)
                                                        && (((battler) as i32) != 4i32)
                                                    {
                                                        ((((&raw mut gBattleMons).cast::<u8>())
                                                            .wrapping_offset(
                                                                ((battler) as i32) as isize * 88,
                                                            ))
                                                        .wrapping_add(40)
                                                        .cast::<u16>())
                                                        .write(((dataUnsigned) as u16));
                                                        if (!((((effectFlags) as i32) & 16i32)
                                                            != 0))
                                                            && (((GetBattlerSide(
                                                                ((&raw mut gActiveBattler)
                                                                    .cast::<u8>())
                                                                .read(),
                                                            ))
                                                                as i32)
                                                                == 0i32)
                                                        {
                                                            if (((((&raw mut gBattleResults)
                                                                .cast::<u8>())
                                                            .wrapping_add(3))
                                                            .read())
                                                                as i32)
                                                                < 255i32
                                                            {
                                                                let __p18 =
                                                                    ((&raw mut gBattleResults)
                                                                        .cast::<u8>())
                                                                    .wrapping_add(3);
                                                                (__p18).write(
                                                                    ((__p18).read())
                                                                        .wrapping_add(1),
                                                                );
                                                            }
                                                            temp2 = ((((&raw mut gActiveBattler)
                                                                .cast::<u8>())
                                                            .read())
                                                                as u32);
                                                            ((&raw mut gActiveBattler)
                                                                .cast::<u8>())
                                                            .write(battler);
                                                            BtlController_EmitGetMonData(
                                                                0u8, 0u8, 0u8,
                                                            );
                                                            MarkBattlerForControllerExec(
                                                                ((&raw mut gActiveBattler)
                                                                    .cast::<u8>())
                                                                .read(),
                                                            );
                                                            ((&raw mut gActiveBattler)
                                                                .cast::<u8>())
                                                            .write(((temp2) as u8));
                                                        }
                                                    }
                                                } else {
                                                    ((&raw mut gBattleMoveDamage).cast::<i32>())
                                                        .write(
                                                            (((dataUnsigned).wrapping_neg())
                                                                as i32),
                                                        );
                                                }
                                                retVal = 0u8;
                                            }
                                            effectFlags =
                                                ((((effectFlags) as i32) & (-17i32)) as u8);
                                            break 'l6;
                                        }
                                        if __sw11 == 3u32 {
                                            if !((((effectFlags) as i32) & 2i32) != 0) {
                                                {
                                                    temp2 = 0u32;
                                                    'l8: loop {
                                                        if !(((temp2) as i32) < 4i32) {
                                                            break 'l8;
                                                        }
                                                        'l9: {
                                                            let mut r#move: u16 = 0u16;
                                                            dataUnsigned = GetMonData3(
                                                                mon,
                                                                (((17u32).wrapping_add(temp2))
                                                                    as i32),
                                                                core::ptr::null_mut(),
                                                            );
                                                            r#move = ((GetMonData3(
                                                                mon,
                                                                (((13u32).wrapping_add(temp2))
                                                                    as i32),
                                                                core::ptr::null_mut(),
                                                            ))
                                                                as u16);
                                                            if dataUnsigned
                                                                != ((CalculatePPWithBonus(
                                                                    r#move,
                                                                    ((GetMonData3(
                                                                        mon,
                                                                        21i32,
                                                                        core::ptr::null_mut(),
                                                                    ))
                                                                        as u8),
                                                                    ((temp2) as u8),
                                                                ))
                                                                    as u32)
                                                            {
                                                                dataUnsigned = (dataUnsigned)
                                                                    .wrapping_add(
                                                                        ((((itemEffect)
                                                                            .wrapping_offset(
                                                                                ((itemEffectParam)
                                                                                    as i32)
                                                                                    as isize,
                                                                            ))
                                                                        .read())
                                                                            as u32),
                                                                    );
                                                                r#move = ((GetMonData3(
                                                                    mon,
                                                                    (((13u32).wrapping_add(temp2))
                                                                        as i32),
                                                                    core::ptr::null_mut(),
                                                                ))
                                                                    as u16);
                                                                if dataUnsigned
                                                                    > ((CalculatePPWithBonus(
                                                                        r#move,
                                                                        ((GetMonData3(
                                                                            mon,
                                                                            21i32,
                                                                            core::ptr::null_mut(),
                                                                        ))
                                                                            as u8),
                                                                        ((temp2) as u8),
                                                                    ))
                                                                        as u32)
                                                                {
                                                                    r#move = ((GetMonData3(
                                                                        mon,
                                                                        (((13u32)
                                                                            .wrapping_add(temp2))
                                                                            as i32),
                                                                        core::ptr::null_mut(),
                                                                    ))
                                                                        as u16);
                                                                    dataUnsigned =
                                                                        ((CalculatePPWithBonus(
                                                                            r#move,
                                                                            ((GetMonData3(
                                                                                mon,
                                                                                21i32,
                                                                                core::ptr::null_mut(
                                                                                ),
                                                                            ))
                                                                                as u8),
                                                                            ((temp2) as u8),
                                                                        ))
                                                                            as u32);
                                                                }
                                                                SetMonData(
                                                                    mon,
                                                                    (((17u32).wrapping_add(temp2))
                                                                        as i32),
                                                                    (&raw mut dataUnsigned)
                                                                        .cast::<u8>(),
                                                                );
                                                                if ((((crate::c::bf_read(((&raw mut gMain).cast::<u8>()).wrapping_add(1081), 1, 1, false) as u8)) != 0) && ((((battler) as i32)) != 4i32)) && ((!((((((((&raw mut gBattleMons)).cast::<u8>()).wrapping_offset((((battler) as i32)) as isize * 88)).wrapping_add(80).cast::<u32>()).read() & 2097152u32)) != 0)) && (!(((((((crate::c::bf_read(((((&raw mut gDisableStructs)).cast::<u8>()).wrapping_offset((((battler) as i32)) as isize * 28)).wrapping_add(24), 4, 4, false) as u8)) as u32)) & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset((((temp2) as i32)) as isize)).read())) != 0))) {
(((((((&raw mut gBattleMons)).cast::<u8>()).wrapping_offset((((battler) as i32)) as isize * 88)).wrapping_add(36)).cast::<u8>()).wrapping_offset((((temp2) as i32)) as isize)).write(((dataUnsigned) as u8));
}
                                                                retVal = 0u8;
                                                            }
                                                        }
                                                        temp2 = (temp2).wrapping_add(1);
                                                    }
                                                }
                                                itemEffectParam = (itemEffectParam).wrapping_add(1);
                                            } else {
                                                let mut r#move: u16 = 0u16;
                                                dataUnsigned = GetMonData3(
                                                    mon,
                                                    (17i32).wrapping_add(((moveIndex) as i32)),
                                                    core::ptr::null_mut(),
                                                );
                                                r#move = ((GetMonData3(
                                                    mon,
                                                    (13i32).wrapping_add(((moveIndex) as i32)),
                                                    core::ptr::null_mut(),
                                                ))
                                                    as u16);
                                                if dataUnsigned
                                                    != ((CalculatePPWithBonus(
                                                        r#move,
                                                        ((GetMonData3(
                                                            mon,
                                                            21i32,
                                                            core::ptr::null_mut(),
                                                        ))
                                                            as u8),
                                                        moveIndex,
                                                    ))
                                                        as u32)
                                                {
                                                    dataUnsigned = (dataUnsigned).wrapping_add(
                                                        ((((itemEffect).wrapping_offset(
                                                            (({
                                                                let __t19 = itemEffectParam;
                                                                itemEffectParam = (itemEffectParam)
                                                                    .wrapping_add(1);
                                                                __t19
                                                            })
                                                                as i32)
                                                                as isize,
                                                        ))
                                                        .read())
                                                            as u32),
                                                    );
                                                    r#move = ((GetMonData3(
                                                        mon,
                                                        (13i32).wrapping_add(((moveIndex) as i32)),
                                                        core::ptr::null_mut(),
                                                    ))
                                                        as u16);
                                                    if dataUnsigned
                                                        > ((CalculatePPWithBonus(
                                                            r#move,
                                                            ((GetMonData3(
                                                                mon,
                                                                21i32,
                                                                core::ptr::null_mut(),
                                                            ))
                                                                as u8),
                                                            moveIndex,
                                                        ))
                                                            as u32)
                                                    {
                                                        r#move = ((GetMonData3(
                                                            mon,
                                                            (13i32)
                                                                .wrapping_add(((moveIndex) as i32)),
                                                            core::ptr::null_mut(),
                                                        ))
                                                            as u16);
                                                        dataUnsigned = ((CalculatePPWithBonus(
                                                            r#move,
                                                            ((GetMonData3(
                                                                mon,
                                                                21i32,
                                                                core::ptr::null_mut(),
                                                            ))
                                                                as u8),
                                                            moveIndex,
                                                        ))
                                                            as u32);
                                                    }
                                                    SetMonData(
                                                        mon,
                                                        (17i32).wrapping_add(((moveIndex) as i32)),
                                                        (&raw mut dataUnsigned).cast::<u8>(),
                                                    );
                                                    if (((crate::c::bf_read(
                                                        ((&raw mut gMain).cast::<u8>())
                                                            .wrapping_add(1081),
                                                        1,
                                                        1,
                                                        false,
                                                    )
                                                        as u8)
                                                        != 0)
                                                        && (((battler) as i32) != 4i32))
                                                        && ((!((((((&raw mut gBattleMons)
                                                            .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((battler) as i32) as isize * 88,
                                                        ))
                                                        .wrapping_add(80)
                                                        .cast::<u32>())
                                                        .read()
                                                            & 2097152u32)
                                                            != 0))
                                                            && (!((((crate::c::bf_read(
                                                                (((&raw mut gDisableStructs)
                                                                    .cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((battler) as i32) as isize
                                                                        * 28,
                                                                ))
                                                                .wrapping_add(24),
                                                                4,
                                                                4,
                                                                false,
                                                            )
                                                                as u8)
                                                                as u32)
                                                                & ((((&raw mut gBitTable)
                                                                    .cast::<u32>())
                                                                .cast::<u32>())
                                                                .wrapping_offset(
                                                                    ((moveIndex) as i32) as isize,
                                                                ))
                                                                .read())
                                                                != 0)))
                                                    {
                                                        ((((((&raw mut gBattleMons)
                                                            .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((battler) as i32) as isize * 88,
                                                        ))
                                                        .wrapping_add(36))
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((moveIndex) as i32) as isize,
                                                        ))
                                                        .write(((dataUnsigned) as u8));
                                                    }
                                                    retVal = 0u8;
                                                }
                                            }
                                            break 'l6;
                                        }
                                        if __sw11 == 7u32 {
                                            {
                                                let mut targetSpecies: u16 =
                                                    GetEvolutionTargetSpecies(mon, 2u8, item);
                                                if ((targetSpecies) as i32) != 0i32 {
                                                    BeginEvolutionScene(
                                                        mon,
                                                        targetSpecies,
                                                        0u8,
                                                        partyIndex,
                                                    );
                                                    return 0u8;
                                                }
                                            }
                                            break 'l6;
                                        }
                                    }
                                }
                                temp1 = (temp1).wrapping_add(1);
                                effectFlags = ((((effectFlags) as i32) >> 1) as u8);
                            }
                            break 'l4;
                        }
                        if __sw1 == 5i32 {
                            effectFlags = ((itemEffect).wrapping_offset((i) as isize)).read();
                            temp1 = 0u32;
                            'l10: loop {
                                if !(((effectFlags) as i32) != 0i32) {
                                    break 'l10;
                                }
                                if (((effectFlags) as i32) & 1i32) != 0 {
                                    'l11: {
                                        let __sw20 = temp1;
                                        if __sw20 == 0u32
                                            || __sw20 == 1u32
                                            || __sw20 == 2u32
                                            || __sw20 == 3u32
                                        {
                                            evCount = GetMonEVCount(mon);
                                            temp2 = ((((itemEffect).wrapping_offset(
                                                ((itemEffectParam) as i32) as isize,
                                            ))
                                            .read())
                                                as u32);
                                            dataSigned = ((GetMonData3(
                                                mon,
                                                ((((((&raw const sGetMonDataEVConstants)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    (((temp1).wrapping_add(2u32)) as i32) as isize,
                                                ))
                                                .read())
                                                    as i32),
                                                core::ptr::null_mut(),
                                            ))
                                                as i32);
                                            evChange = ((temp2) as i8);
                                            if ((evChange) as i32) > 0i32 {
                                                if ((evCount) as i32) >= 510i32 {
                                                    return 1u8;
                                                }
                                                if dataSigned >= 100i32 {
                                                    break 'l11;
                                                }
                                                if (dataSigned).wrapping_add(((evChange) as i32))
                                                    > 100i32
                                                {
                                                    temp2 = ((((100i32).wrapping_sub(
                                                        (dataSigned)
                                                            .wrapping_add(((evChange) as i32)),
                                                    ))
                                                    .wrapping_add(((evChange) as i32)))
                                                        as u32);
                                                } else {
                                                    temp2 = ((evChange) as u32);
                                                }
                                                if ((evCount) as u32).wrapping_add(temp2) > 510u32 {
                                                    temp2 = (temp2).wrapping_add(
                                                        (510u32).wrapping_sub(
                                                            ((evCount) as u32).wrapping_add(temp2),
                                                        ),
                                                    );
                                                }
                                                dataSigned = ((((dataSigned) as u32)
                                                    .wrapping_add(temp2))
                                                    as i32);
                                            } else {
                                                if dataSigned == 0i32 {
                                                    friendshipOnly = 1u32;
                                                    itemEffectParam =
                                                        (itemEffectParam).wrapping_add(1);
                                                    break 'l11;
                                                }
                                                dataSigned =
                                                    (dataSigned).wrapping_add(((evChange) as i32));
                                                if dataSigned < 0i32 {
                                                    dataSigned = 0i32;
                                                }
                                            }
                                            SetMonData(
                                                mon,
                                                ((((((&raw const sGetMonDataEVConstants)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    (((temp1).wrapping_add(2u32)) as i32) as isize,
                                                ))
                                                .read())
                                                    as i32),
                                                (&raw mut dataSigned).cast::<u8>(),
                                            );
                                            CalculateMonStats(mon);
                                            retVal = 0u8;
                                            itemEffectParam = (itemEffectParam).wrapping_add(1);
                                            break 'l11;
                                        }
                                        if __sw20 == 4u32 {
                                            dataUnsigned = crate::c::shr_u32(
                                                (GetMonData3(mon, 21i32, core::ptr::null_mut())
                                                    & ((((((&raw const gPPUpGetMask)
                                                        .cast::<u8>()
                                                        .cast_mut())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((moveIndex) as i32) as isize,
                                                    ))
                                                    .read())
                                                        as u32)),
                                                ((((moveIndex) as i32).wrapping_mul(2i32)) as u32),
                                            );
                                            temp2 = ((CalculatePPWithBonus(
                                                ((GetMonData3(
                                                    mon,
                                                    (13i32).wrapping_add(((moveIndex) as i32)),
                                                    core::ptr::null_mut(),
                                                ))
                                                    as u16),
                                                ((GetMonData3(mon, 21i32, core::ptr::null_mut()))
                                                    as u8),
                                                moveIndex,
                                            ))
                                                as u32);
                                            if (dataUnsigned < 3u32) && (temp2 >= 5u32) {
                                                dataUnsigned =
                                                    GetMonData3(mon, 21i32, core::ptr::null_mut());
                                                dataUnsigned = (dataUnsigned
                                                    & ((((((&raw const gPPUpClearMask)
                                                        .cast::<u8>()
                                                        .cast_mut())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((moveIndex) as i32) as isize,
                                                    ))
                                                    .read())
                                                        as u32));
                                                dataUnsigned = (dataUnsigned).wrapping_add(
                                                    ((((((((&raw const gPPUpAddValues)
                                                        .cast::<u8>()
                                                        .cast_mut())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((moveIndex) as i32) as isize,
                                                    ))
                                                    .read())
                                                        as i32)
                                                        .wrapping_mul(3i32))
                                                        as u32),
                                                );
                                                SetMonData(
                                                    mon,
                                                    21i32,
                                                    (&raw mut dataUnsigned).cast::<u8>(),
                                                );
                                                dataUnsigned = ((CalculatePPWithBonus(
                                                    ((GetMonData3(
                                                        mon,
                                                        (13i32).wrapping_add(((moveIndex) as i32)),
                                                        core::ptr::null_mut(),
                                                    ))
                                                        as u16),
                                                    ((dataUnsigned) as u8),
                                                    moveIndex,
                                                ))
                                                    as u32)
                                                    .wrapping_sub(temp2);
                                                dataUnsigned = (GetMonData3(
                                                    mon,
                                                    (17i32).wrapping_add(((moveIndex) as i32)),
                                                    core::ptr::null_mut(),
                                                ))
                                                .wrapping_add(dataUnsigned);
                                                SetMonData(
                                                    mon,
                                                    (17i32).wrapping_add(((moveIndex) as i32)),
                                                    (&raw mut dataUnsigned).cast::<u8>(),
                                                );
                                                retVal = 0u8;
                                            }
                                            break 'l11;
                                        }
                                        if __sw20 == 5u32 {
                                            if GetMonData3(mon, 32i32, core::ptr::null_mut())
                                                < 100u32
                                            {
                                                if (((((retVal) as i32) == 0i32)
                                                    || ((friendshipOnly) != 0))
                                                    && (!((ShouldSkipFriendshipChange()) != 0)))
                                                    && (((friendshipChange) as i32) == 0i32)
                                                {
                                                    friendshipChange = ((((itemEffect)
                                                        .wrapping_offset(
                                                            ((itemEffectParam) as i32) as isize,
                                                        ))
                                                    .read())
                                                        as i8);
                                                    friendship = ((GetMonData3(
                                                        mon,
                                                        32i32,
                                                        core::ptr::null_mut(),
                                                    ))
                                                        as i32);
                                                    if (((friendshipChange) as i32) > 0i32)
                                                        && (((holdEffect) as i32) == 27i32)
                                                    {
                                                        friendship = (friendship).wrapping_add(
                                                            crate::c::div_i32(
                                                                (150i32).wrapping_mul(
                                                                    ((friendshipChange) as i32),
                                                                ),
                                                                100i32,
                                                            ),
                                                        );
                                                    } else {
                                                        friendship = (friendship).wrapping_add(
                                                            ((friendshipChange) as i32),
                                                        );
                                                    }
                                                    if ((friendshipChange) as i32) > 0i32 {
                                                        if GetMonData3(
                                                            mon,
                                                            38i32,
                                                            core::ptr::null_mut(),
                                                        ) == 11u32
                                                        {
                                                            friendship =
                                                                (friendship).wrapping_add(1);
                                                        }
                                                        if GetMonData3(
                                                            mon,
                                                            35i32,
                                                            core::ptr::null_mut(),
                                                        ) == ((GetCurrentRegionMapSectionId())
                                                            as u32)
                                                        {
                                                            friendship =
                                                                (friendship).wrapping_add(1);
                                                        }
                                                    }
                                                    if friendship < 0i32 {
                                                        friendship = 0i32;
                                                    }
                                                    if friendship > 255i32 {
                                                        friendship = 255i32;
                                                    }
                                                    SetMonData(
                                                        mon,
                                                        32i32,
                                                        (&raw mut friendship).cast::<u8>(),
                                                    );
                                                    retVal = 0u8;
                                                }
                                            }
                                            itemEffectParam = (itemEffectParam).wrapping_add(1);
                                            break 'l11;
                                        }
                                        if __sw20 == 6u32 {
                                            if (GetMonData3(mon, 32i32, core::ptr::null_mut())
                                                >= 100u32)
                                                && (GetMonData3(mon, 32i32, core::ptr::null_mut())
                                                    < 200u32)
                                            {
                                                if (((((retVal) as i32) == 0i32)
                                                    || ((friendshipOnly) != 0))
                                                    && (!((ShouldSkipFriendshipChange()) != 0)))
                                                    && (((friendshipChange) as i32) == 0i32)
                                                {
                                                    friendshipChange = ((((itemEffect)
                                                        .wrapping_offset(
                                                            ((itemEffectParam) as i32) as isize,
                                                        ))
                                                    .read())
                                                        as i8);
                                                    friendship = ((GetMonData3(
                                                        mon,
                                                        32i32,
                                                        core::ptr::null_mut(),
                                                    ))
                                                        as i32);
                                                    if (((friendshipChange) as i32) > 0i32)
                                                        && (((holdEffect) as i32) == 27i32)
                                                    {
                                                        friendship = (friendship).wrapping_add(
                                                            crate::c::div_i32(
                                                                (150i32).wrapping_mul(
                                                                    ((friendshipChange) as i32),
                                                                ),
                                                                100i32,
                                                            ),
                                                        );
                                                    } else {
                                                        friendship = (friendship).wrapping_add(
                                                            ((friendshipChange) as i32),
                                                        );
                                                    }
                                                    if ((friendshipChange) as i32) > 0i32 {
                                                        if GetMonData3(
                                                            mon,
                                                            38i32,
                                                            core::ptr::null_mut(),
                                                        ) == 11u32
                                                        {
                                                            friendship =
                                                                (friendship).wrapping_add(1);
                                                        }
                                                        if GetMonData3(
                                                            mon,
                                                            35i32,
                                                            core::ptr::null_mut(),
                                                        ) == ((GetCurrentRegionMapSectionId())
                                                            as u32)
                                                        {
                                                            friendship =
                                                                (friendship).wrapping_add(1);
                                                        }
                                                    }
                                                    if friendship < 0i32 {
                                                        friendship = 0i32;
                                                    }
                                                    if friendship > 255i32 {
                                                        friendship = 255i32;
                                                    }
                                                    SetMonData(
                                                        mon,
                                                        32i32,
                                                        (&raw mut friendship).cast::<u8>(),
                                                    );
                                                    retVal = 0u8;
                                                }
                                            }
                                            itemEffectParam = (itemEffectParam).wrapping_add(1);
                                            break 'l11;
                                        }
                                        if __sw20 == 7u32 {
                                            if GetMonData3(mon, 32i32, core::ptr::null_mut())
                                                >= 200u32
                                            {
                                                if (((((retVal) as i32) == 0i32)
                                                    || ((friendshipOnly) != 0))
                                                    && (!((ShouldSkipFriendshipChange()) != 0)))
                                                    && (((friendshipChange) as i32) == 0i32)
                                                {
                                                    friendshipChange = ((((itemEffect)
                                                        .wrapping_offset(
                                                            ((itemEffectParam) as i32) as isize,
                                                        ))
                                                    .read())
                                                        as i8);
                                                    friendship = ((GetMonData3(
                                                        mon,
                                                        32i32,
                                                        core::ptr::null_mut(),
                                                    ))
                                                        as i32);
                                                    if (((friendshipChange) as i32) > 0i32)
                                                        && (((holdEffect) as i32) == 27i32)
                                                    {
                                                        friendship = (friendship).wrapping_add(
                                                            crate::c::div_i32(
                                                                (150i32).wrapping_mul(
                                                                    ((friendshipChange) as i32),
                                                                ),
                                                                100i32,
                                                            ),
                                                        );
                                                    } else {
                                                        friendship = (friendship).wrapping_add(
                                                            ((friendshipChange) as i32),
                                                        );
                                                    }
                                                    if ((friendshipChange) as i32) > 0i32 {
                                                        if GetMonData3(
                                                            mon,
                                                            38i32,
                                                            core::ptr::null_mut(),
                                                        ) == 11u32
                                                        {
                                                            friendship =
                                                                (friendship).wrapping_add(1);
                                                        }
                                                        if GetMonData3(
                                                            mon,
                                                            35i32,
                                                            core::ptr::null_mut(),
                                                        ) == ((GetCurrentRegionMapSectionId())
                                                            as u32)
                                                        {
                                                            friendship =
                                                                (friendship).wrapping_add(1);
                                                        }
                                                    }
                                                    if friendship < 0i32 {
                                                        friendship = 0i32;
                                                    }
                                                    if friendship > 255i32 {
                                                        friendship = 255i32;
                                                    }
                                                    SetMonData(
                                                        mon,
                                                        32i32,
                                                        (&raw mut friendship).cast::<u8>(),
                                                    );
                                                    retVal = 0u8;
                                                }
                                            }
                                            itemEffectParam = (itemEffectParam).wrapping_add(1);
                                            break 'l11;
                                        }
                                    }
                                }
                                temp1 = (temp1).wrapping_add(1);
                                effectFlags = ((((effectFlags) as i32) >> 1) as u8);
                            }
                            break 'l4;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return retVal;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HealStatusConditions(
    mon: *mut u8,
    battlePartyId: u32,
    healMask: u32,
    battler: u8,
) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut battlePartyId = battlePartyId;
        let mut healMask = healMask;
        let mut battler = battler;
        let mut status: u32 = GetMonData3(mon, 55i32, core::ptr::null_mut());
        if (status & healMask) != 0 {
            status = (status & !(healMask));
            SetMonData(mon, 55i32, (&raw mut status).cast::<u8>());
            if ((crate::c::bf_read(
                ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                1,
                1,
                false,
            ) as u8)
                != 0)
                && (((battler) as i32) != 4i32)
            {
                let __p1 = (((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize * 88))
                .wrapping_add(76)
                .cast::<u32>();
                (__p1).write(((__p1).read() & !(healMask)));
            }
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemEffectParamOffset(
    itemId: u16,
    effectByte: u8,
    effectBit: u8,
) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut effectByte = effectByte;
        let mut effectBit = effectBit;
        let mut temp: *mut u8 = core::ptr::null_mut();
        let mut itemEffect: *mut u8 = core::ptr::null_mut();
        let mut offset: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut j: u8 = 0u8;
        let mut effectFlags: u8 = 0u8;
        offset = 6u8;
        temp = ((((&raw const gItemEffectTable)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset((((itemId) as i32).wrapping_sub(13i32)) as isize))
        .read();
        if (!(!(temp).is_null())) && (((itemId) as i32) != 175i32) {
            return 0u8;
        }
        if ((itemId) as i32) == 175i32 {
            temp = ((((&raw mut gEnigmaBerries).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 28,
            ))
            .wrapping_add(8))
            .cast::<u8>();
        }
        itemEffect = temp;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    'l3: {
                        let __sw1 = i;
                        if __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 {
                            if i == ((effectByte) as i32) {
                                return 0u8;
                            }
                            break 'l3;
                        }
                        if __sw1 == 4i32 {
                            effectFlags = ((itemEffect).wrapping_offset(4)).read();
                            if (((effectFlags) as i32) & 32i32) != 0 {
                                effectFlags = ((((effectFlags) as i32) & (-33i32)) as u8);
                            }
                            j = 0u8;
                            'l4: loop {
                                if !((effectFlags) != 0) {
                                    break 'l4;
                                }
                                if (((effectFlags) as i32) & 1i32) != 0 {
                                    'l5: {
                                        let __sw2 = ((j) as i32);
                                        let mut __fall = false;
                                        if __sw2 == 2i32 {
                                            __fall = true;
                                            if (((effectFlags) as i32) & 16i32) != 0 {
                                                effectFlags =
                                                    ((((effectFlags) as i32) & (-17i32)) as u8);
                                            }
                                        }
                                        if __fall || __sw2 == 0i32 {
                                            __fall = true;
                                            if (i == ((effectByte) as i32))
                                                && ((((effectFlags) as i32) & ((effectBit) as i32))
                                                    != 0)
                                            {
                                                return offset;
                                            }
                                            offset = (offset).wrapping_add(1);
                                            break 'l5;
                                        }
                                        if __sw2 == 1i32 {
                                            __fall = true;
                                            if (i == ((effectByte) as i32))
                                                && ((((effectFlags) as i32) & ((effectBit) as i32))
                                                    != 0)
                                            {
                                                return offset;
                                            }
                                            offset = (offset).wrapping_add(1);
                                            break 'l5;
                                        }
                                        if __sw2 == 3i32 {
                                            __fall = true;
                                            if (i == ((effectByte) as i32))
                                                && ((((effectFlags) as i32) & ((effectBit) as i32))
                                                    != 0)
                                            {
                                                return offset;
                                            }
                                            offset = (offset).wrapping_add(1);
                                            break 'l5;
                                        }
                                        if __sw2 == 7i32 {
                                            __fall = true;
                                            if i == ((effectByte) as i32) {
                                                return 0u8;
                                            }
                                            break 'l5;
                                        }
                                    }
                                }
                                j = (j).wrapping_add(1);
                                effectFlags = ((((effectFlags) as i32) >> 1) as u8);
                                if i == ((effectByte) as i32) {
                                    effectBit = ((((effectBit) as i32) >> 1) as u8);
                                }
                            }
                            break 'l3;
                        }
                        if __sw1 == 5i32 {
                            effectFlags = ((itemEffect).wrapping_offset(5)).read();
                            j = 0u8;
                            'l6: loop {
                                if !((effectFlags) != 0) {
                                    break 'l6;
                                }
                                if (((effectFlags) as i32) & 1i32) != 0 {
                                    'l7: {
                                        let __sw3 = ((j) as i32);
                                        if __sw3 == 0i32
                                            || __sw3 == 1i32
                                            || __sw3 == 2i32
                                            || __sw3 == 3i32
                                            || __sw3 == 4i32
                                            || __sw3 == 5i32
                                            || __sw3 == 6i32
                                        {
                                            if (i == ((effectByte) as i32))
                                                && ((((effectFlags) as i32) & ((effectBit) as i32))
                                                    != 0)
                                            {
                                                return offset;
                                            }
                                            offset = (offset).wrapping_add(1);
                                            break 'l7;
                                        }
                                        if __sw3 == 7i32 {
                                            if i == ((effectByte) as i32) {
                                                return 0u8;
                                            }
                                            break 'l7;
                                        }
                                    }
                                }
                                j = (j).wrapping_add(1);
                                effectFlags = ((((effectFlags) as i32) >> 1) as u8);
                                if i == ((effectByte) as i32) {
                                    effectBit = ((((effectBit) as i32) >> 1) as u8);
                                }
                            }
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return offset;
    }
}
pub(crate) unsafe extern "C" fn BufferStatRoseMessage(statIdx: i32) {
    unsafe {
        let mut statIdx = statIdx;
        ((&raw mut gBattlerTarget).cast::<u8>())
            .write(((&raw mut gBattlerInMenuId).cast::<u8>()).read());
        StringCopy(
            (&raw mut gBattleTextBuff1).cast::<u8>(),
            ((((&raw mut gStatNamesTable).cast::<*mut u8>()).cast::<*mut u8>()).wrapping_offset(
                ((((((&raw const sStatsToRaise).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((statIdx) as isize))
                .read()) as i32) as isize,
            ))
            .read(),
        );
        StringCopy(
            (&raw mut gBattleTextBuff2).cast::<u8>(),
            (&raw mut gText_StatRose).cast::<u8>(),
        );
        BattleStringExpandPlaceholdersToDisplayedString(
            (&raw mut gText_DefendersStatRose).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UseStatIncreaseItem(itemId: u16) -> *mut u8 {
    unsafe {
        let mut itemId = itemId;
        let mut i: i32 = 0i32;
        let mut itemEffect: *mut u8 = core::ptr::null_mut();
        if ((itemId) as i32) == 175i32 {
            if (crate::c::bf_read(
                ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                1,
                1,
                false,
            ) as u8)
                != 0
            {
                itemEffect = ((((&raw mut gEnigmaBerries).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattlerInMenuId).cast::<u8>()).read()) as i32) as isize * 28,
                ))
                .wrapping_add(8))
                .cast::<u8>();
            } else {
                itemEffect = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(12792))
                .wrapping_add(28))
                .cast::<u8>();
            }
        } else {
            itemEffect = ((((&raw const gItemEffectTable)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset((((itemId) as i32).wrapping_sub(13i32)) as isize))
            .read();
        }
        ((&raw mut gPotentialItemEffectBattler).cast::<u8>())
            .write(((&raw mut gBattlerInMenuId).cast::<u8>()).read());
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((itemEffect).wrapping_offset((i) as isize)).read()) as i32) & 15i32) != 0
                    {
                        BufferStatRoseMessage((i).wrapping_mul(2i32));
                    }
                    if (((((itemEffect).wrapping_offset((i) as isize)).read()) as i32) & 240i32)
                        != 0
                    {
                        if i != 0i32 {
                            BufferStatRoseMessage(((i).wrapping_mul(2i32)).wrapping_add(1i32));
                        } else {
                            ((&raw mut gBattlerAttacker).cast::<u8>())
                                .write(((&raw mut gBattlerInMenuId).cast::<u8>()).read());
                            BattleStringExpandPlaceholdersToDisplayedString(
                                (&raw mut gText_PkmnGettingPumped).cast::<u8>(),
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((((itemEffect).wrapping_offset(3)).read()) as i32) & 128i32) != 0 {
            ((&raw mut gBattlerAttacker).cast::<u8>())
                .write(((&raw mut gBattlerInMenuId).cast::<u8>()).read());
            BattleStringExpandPlaceholdersToDisplayedString(
                (&raw mut gText_PkmnShroudedInMist).cast::<u8>(),
            );
        }
        return (&raw mut gDisplayedStringBattle).cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNature(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        return ((crate::c::rem_u32(GetMonData3(mon, 0i32, core::ptr::null_mut()), 25u32)) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNatureFromPersonality(personality: u32) -> u8 {
    unsafe {
        let mut personality = personality;
        return ((crate::c::rem_u32(personality, 25u32)) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetEvolutionTargetSpecies(
    mon: *mut u8,
    mode: u8,
    evolutionItem: u16,
) -> u16 {
    unsafe {
        let mut mon = mon;
        let mut mode = mode;
        let mut evolutionItem = evolutionItem;
        let mut i: i32 = 0i32;
        let mut targetSpecies: u16 = 0u16;
        let mut species: u16 = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
        let mut heldItem: u16 = ((GetMonData3(mon, 12i32, core::ptr::null_mut())) as u16);
        let mut personality: u32 = GetMonData3(mon, 0i32, core::ptr::null_mut());
        let mut level: u8 = 0u8;
        let mut friendship: u16 = 0u16;
        let mut beauty: u8 = ((GetMonData3(mon, 23i32, core::ptr::null_mut())) as u8);
        let mut upperPersonality: u16 = ((personality >> 16) as u16);
        let mut holdEffect: u8 = 0u8;
        if ((heldItem) as i32) == 175i32 {
            holdEffect = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(12792))
            .wrapping_add(46))
            .read();
        } else {
            holdEffect = GetItemHoldEffect(heldItem);
        }
        if (((holdEffect) as i32) == 38i32) && (((mode) as i32) != 3i32) {
            return 0u16;
        }
        'l1: {
            let __sw1 = ((mode) as i32);
            if __sw1 == 0i32 {
                level = ((GetMonData3(mon, 56i32, core::ptr::null_mut())) as u8);
                friendship = ((GetMonData3(mon, 32i32, core::ptr::null_mut())) as u16);
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 5i32) {
                            break 'l2;
                        }
                        'l3: {
                            'l4: {
                                let __sw2 = (((((((((&raw const gEvolutionTable)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 40))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                                .cast::<u16>())
                                .read()) as i32);
                                if __sw2 == 1i32 {
                                    if ((friendship) as i32) >= 220i32 {
                                        targetSpecies = (((((((&raw const gEvolutionTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 40))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 8))
                                        .wrapping_add(4)
                                        .cast::<u16>())
                                        .read();
                                    }
                                    break 'l4;
                                }
                                if __sw2 == 2i32 {
                                    RtcCalcLocalTime();
                                    if (((((((&raw mut gLocalTime).cast::<u8>())
                                        .wrapping_add(2)
                                        .cast::<i8>())
                                    .read()) as i32)
                                        >= 12i32)
                                        && ((((((&raw mut gLocalTime).cast::<u8>())
                                            .wrapping_add(2)
                                            .cast::<i8>())
                                        .read())
                                            as i32)
                                            < 24i32))
                                        && (((friendship) as i32) >= 220i32)
                                    {
                                        targetSpecies = (((((((&raw const gEvolutionTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 40))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 8))
                                        .wrapping_add(4)
                                        .cast::<u16>())
                                        .read();
                                    }
                                    break 'l4;
                                }
                                if __sw2 == 3i32 {
                                    RtcCalcLocalTime();
                                    if (((((((&raw mut gLocalTime).cast::<u8>())
                                        .wrapping_add(2)
                                        .cast::<i8>())
                                    .read()) as i32)
                                        >= 0i32)
                                        && ((((((&raw mut gLocalTime).cast::<u8>())
                                            .wrapping_add(2)
                                            .cast::<i8>())
                                        .read())
                                            as i32)
                                            < 12i32))
                                        && (((friendship) as i32) >= 220i32)
                                    {
                                        targetSpecies = (((((((&raw const gEvolutionTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 40))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 8))
                                        .wrapping_add(4)
                                        .cast::<u16>())
                                        .read();
                                    }
                                    break 'l4;
                                }
                                if __sw2 == 4i32 {
                                    if (((((((((&raw const gEvolutionTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 40))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        <= ((level) as i32)
                                    {
                                        targetSpecies = (((((((&raw const gEvolutionTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 40))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 8))
                                        .wrapping_add(4)
                                        .cast::<u16>())
                                        .read();
                                    }
                                    break 'l4;
                                }
                                if __sw2 == 8i32 {
                                    if (((((((((&raw const gEvolutionTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 40))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        <= ((level) as i32)
                                    {
                                        if GetMonData3(mon, 59i32, core::ptr::null_mut())
                                            > GetMonData3(mon, 60i32, core::ptr::null_mut())
                                        {
                                            targetSpecies =
                                                (((((((&raw const gEvolutionTable)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((species) as i32) as isize * 40,
                                                ))
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize * 8))
                                                .wrapping_add(4)
                                                .cast::<u16>())
                                                .read();
                                        }
                                    }
                                    break 'l4;
                                }
                                if __sw2 == 9i32 {
                                    if (((((((((&raw const gEvolutionTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 40))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        <= ((level) as i32)
                                    {
                                        if GetMonData3(mon, 59i32, core::ptr::null_mut())
                                            == GetMonData3(mon, 60i32, core::ptr::null_mut())
                                        {
                                            targetSpecies =
                                                (((((((&raw const gEvolutionTable)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((species) as i32) as isize * 40,
                                                ))
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize * 8))
                                                .wrapping_add(4)
                                                .cast::<u16>())
                                                .read();
                                        }
                                    }
                                    break 'l4;
                                }
                                if __sw2 == 10i32 {
                                    if (((((((((&raw const gEvolutionTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 40))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        <= ((level) as i32)
                                    {
                                        if GetMonData3(mon, 59i32, core::ptr::null_mut())
                                            < GetMonData3(mon, 60i32, core::ptr::null_mut())
                                        {
                                            targetSpecies =
                                                (((((((&raw const gEvolutionTable)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    ((species) as i32) as isize * 40,
                                                ))
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize * 8))
                                                .wrapping_add(4)
                                                .cast::<u16>())
                                                .read();
                                        }
                                    }
                                    break 'l4;
                                }
                                if __sw2 == 11i32 {
                                    if ((((((((((&raw const gEvolutionTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 40))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        <= ((level) as i32))
                                        && (crate::c::rem_i32(((upperPersonality) as i32), 10i32)
                                            <= 4i32)
                                    {
                                        targetSpecies = (((((((&raw const gEvolutionTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 40))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 8))
                                        .wrapping_add(4)
                                        .cast::<u16>())
                                        .read();
                                    }
                                    break 'l4;
                                }
                                if __sw2 == 12i32 {
                                    if ((((((((((&raw const gEvolutionTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 40))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        <= ((level) as i32))
                                        && (crate::c::rem_i32(((upperPersonality) as i32), 10i32)
                                            > 4i32)
                                    {
                                        targetSpecies = (((((((&raw const gEvolutionTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 40))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 8))
                                        .wrapping_add(4)
                                        .cast::<u16>())
                                        .read();
                                    }
                                    break 'l4;
                                }
                                if __sw2 == 13i32 {
                                    if (((((((((&raw const gEvolutionTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 40))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        <= ((level) as i32)
                                    {
                                        targetSpecies = (((((((&raw const gEvolutionTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 40))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 8))
                                        .wrapping_add(4)
                                        .cast::<u16>())
                                        .read();
                                    }
                                    break 'l4;
                                }
                                if __sw2 == 15i32 {
                                    if (((((((((&raw const gEvolutionTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 40))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        <= ((beauty) as i32)
                                    {
                                        targetSpecies = (((((((&raw const gEvolutionTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 40))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 8))
                                        .wrapping_add(4)
                                        .cast::<u16>())
                                        .read();
                                    }
                                    break 'l4;
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = 0i32;
                    'l5: loop {
                        if !(i < 5i32) {
                            break 'l5;
                        }
                        'l6: {
                            'l7: {
                                let __sw3 = (((((((((&raw const gEvolutionTable)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 40))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                                .cast::<u16>())
                                .read()) as i32);
                                if __sw3 == 5i32 {
                                    targetSpecies = (((((((&raw const gEvolutionTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 40))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                    .wrapping_add(4)
                                    .cast::<u16>())
                                    .read();
                                    break 'l7;
                                }
                                if __sw3 == 6i32 {
                                    if (((((((((&raw const gEvolutionTable)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 40))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        == ((heldItem) as i32)
                                    {
                                        heldItem = 0u16;
                                        SetMonData(mon, 12i32, (&raw mut heldItem).cast::<u8>());
                                        targetSpecies = (((((((&raw const gEvolutionTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 40))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 8))
                                        .wrapping_add(4)
                                        .cast::<u16>())
                                        .read();
                                    }
                                    break 'l7;
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 3i32 {
                {
                    i = 0i32;
                    'l8: loop {
                        if !(i < 5i32) {
                            break 'l8;
                        }
                        'l9: {
                            if ((((((((((&raw const gEvolutionTable).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 40))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 8))
                            .cast::<u16>())
                            .read()) as i32)
                                == 7i32)
                                && ((((((((((&raw const gEvolutionTable)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 40))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .read()) as i32)
                                    == ((evolutionItem) as i32))
                            {
                                targetSpecies = (((((((&raw const gEvolutionTable)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 40))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                                .wrapping_add(4)
                                .cast::<u16>())
                                .read();
                                break 'l8;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
        }
        return targetSpecies;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HoennPokedexNumToSpecies(hoennNum: u16) -> u16 {
    unsafe {
        let mut hoennNum = hoennNum;
        let mut species: u16 = 0u16;
        if !((hoennNum) != 0) {
            return 0u16;
        }
        species = 0u16;
        'l1: loop {
            if !((((species) as i32) < 411i32)
                && (((((((&raw const sSpeciesToHoennPokedexNum)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((species) as i32) as isize))
                .read()) as i32)
                    != ((hoennNum) as i32)))
            {
                break 'l1;
            }
            species = (species).wrapping_add(1);
        }
        if ((species) as i32) == 411i32 {
            return 0u16;
        }
        return ((((species) as i32).wrapping_add(1i32)) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn NationalPokedexNumToSpecies(nationalNum: u16) -> u16 {
    unsafe {
        let mut nationalNum = nationalNum;
        let mut species: u16 = 0u16;
        if !((nationalNum) != 0) {
            return 0u16;
        }
        species = 0u16;
        'l1: loop {
            if !((((species) as i32) < 411i32)
                && (((((((&raw const sSpeciesToNationalPokedexNum)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((species) as i32) as isize))
                .read()) as i32)
                    != ((nationalNum) as i32)))
            {
                break 'l1;
            }
            species = (species).wrapping_add(1);
        }
        if ((species) as i32) == 411i32 {
            return 0u16;
        }
        return ((((species) as i32).wrapping_add(1i32)) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn NationalToHoennOrder(nationalNum: u16) -> u16 {
    unsafe {
        let mut nationalNum = nationalNum;
        let mut hoennNum: u16 = 0u16;
        if !((nationalNum) != 0) {
            return 0u16;
        }
        hoennNum = 0u16;
        'l1: loop {
            if !((((hoennNum) as i32) < 411i32)
                && (((((((&raw const sHoennToNationalOrder)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((hoennNum) as i32) as isize))
                .read()) as i32)
                    != ((nationalNum) as i32)))
            {
                break 'l1;
            }
            hoennNum = (hoennNum).wrapping_add(1);
        }
        if ((hoennNum) as i32) == 411i32 {
            return 0u16;
        }
        return ((((hoennNum) as i32).wrapping_add(1i32)) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpeciesToNationalPokedexNum(species: u16) -> u16 {
    unsafe {
        let mut species = species;
        if !((species) != 0) {
            return 0u16;
        }
        return ((((&raw const sSpeciesToNationalPokedexNum)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset((((species) as i32).wrapping_sub(1i32)) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpeciesToHoennPokedexNum(species: u16) -> u16 {
    unsafe {
        let mut species = species;
        if !((species) != 0) {
            return 0u16;
        }
        return ((((&raw const sSpeciesToHoennPokedexNum)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset((((species) as i32).wrapping_sub(1i32)) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HoennToNationalOrder(hoennNum: u16) -> u16 {
    unsafe {
        let mut hoennNum = hoennNum;
        if !((hoennNum) != 0) {
            return 0u16;
        }
        return ((((&raw const sHoennToNationalOrder)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset((((hoennNum) as i32).wrapping_sub(1i32)) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpeciesToCryId(species: u16) -> u16 {
    unsafe {
        let mut species = species;
        if ((species) as i32) <= 250i32 {
            return species;
        }
        if ((species) as i32) < 276i32 {
            return 200u16;
        }
        return ((((&raw const gSpeciesIdToCryId)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset((((species) as i32).wrapping_sub(276i32)) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn DrawSpindaSpotsUnused(
    species: u16,
    personality: u32,
    dest: *mut u8,
) {
    unsafe {
        let mut species = species;
        let mut personality = personality;
        let mut dest = dest;
        if ((((species) as i32) == 308i32)
            && (((dest) as usize)
                != (((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<*mut u8>())
                .read()) as usize)))
            && (((dest) as usize)
                != ((((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .cast::<*mut u8>())
                .wrapping_offset(2))
                .read()) as usize))
        {
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((crate::c::div_u32(144u32, 36u32)) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        let mut row: i32 = 0i32;
                        let mut x: u8 =
                            ((((((((&raw const gSpindaSpotGraphics).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 36))
                            .read()) as u32)
                                .wrapping_add((personality & 15u32).wrapping_sub(8u32)))
                                as u8);
                        let mut y: u8 =
                            (((((((((&raw const gSpindaSpotGraphics).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 36))
                            .wrapping_add(1))
                            .read()) as u32)
                                .wrapping_add(((personality & 240u32) >> 4).wrapping_sub(8u32)))
                                as u8);
                        {
                            row = 0i32;
                            'l3: loop {
                                if !(row < 16i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    let mut column: i32 = 0i32;
                                    let mut spotPixelRow: i32 =
                                        (((((((((&raw const gSpindaSpotGraphics)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 36))
                                        .wrapping_add(2))
                                        .cast::<u16>())
                                        .wrapping_offset((row) as isize))
                                        .read()) as i32);
                                    {
                                        column = ((x) as i32);
                                        'l5: loop {
                                            if !(column < ((x) as i32).wrapping_add(16i32)) {
                                                break 'l5;
                                            }
                                            'l6: {
                                                let mut destPixels: *mut u8 = ((((dest)
                                                    .wrapping_offset(
                                                        ((crate::c::div_i32(column, 8i32))
                                                            .wrapping_mul(crate::c::div_i32(
                                                                256i32, 8i32,
                                                            )))
                                                            as isize,
                                                    ))
                                                .wrapping_offset(
                                                    (crate::c::div_i32(
                                                        crate::c::rem_i32(column, 8i32),
                                                        2i32,
                                                    ))
                                                        as isize,
                                                ))
                                                .wrapping_offset(
                                                    (((crate::c::div_i32(((y) as i32), 8i32))
                                                        .wrapping_mul(crate::c::div_i32(
                                                            256i32, 8i32,
                                                        )))
                                                    .wrapping_mul(8i32))
                                                        as isize,
                                                ))
                                                .wrapping_offset(
                                                    ((crate::c::rem_i32(((y) as i32), 8i32))
                                                        .wrapping_mul(4i32))
                                                        as isize,
                                                );
                                                if (spotPixelRow & 1i32) != 0 {
                                                    if (column & 1i32) != 0 {
                                                        if (((((destPixels).read()) as i32)
                                                            & 240i32)
                                                            >= 16i32)
                                                            && (((((destPixels).read()) as i32)
                                                                & 240i32)
                                                                <= 48i32)
                                                        {
                                                            (destPixels).write(
                                                                (((((destPixels).read()) as i32)
                                                                    .wrapping_add(64i32))
                                                                    as u8),
                                                            );
                                                        }
                                                    } else {
                                                        if (((((destPixels).read()) as i32)
                                                            & 15i32)
                                                            >= 1i32)
                                                            && (((((destPixels).read()) as i32)
                                                                & 15i32)
                                                                <= 3i32)
                                                        {
                                                            (destPixels).write(
                                                                (((((destPixels).read()) as i32)
                                                                    .wrapping_add(4i32))
                                                                    as u8),
                                                            );
                                                        }
                                                    }
                                                }
                                                spotPixelRow = (spotPixelRow >> 1);
                                            }
                                            column = (column).wrapping_add(1);
                                        }
                                    }
                                    y = (y).wrapping_add(1);
                                }
                                row = (row).wrapping_add(1);
                            }
                        }
                        personality = (personality >> 8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawSpindaSpots(
    species: u16,
    personality: u32,
    dest: *mut u8,
    isFrontPic: u8,
) {
    unsafe {
        let mut species = species;
        let mut personality = personality;
        let mut dest = dest;
        let mut isFrontPic = isFrontPic;
        if (((species) as i32) == 308i32) && ((isFrontPic) != 0) {
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((crate::c::div_u32(144u32, 36u32)) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        let mut row: i32 = 0i32;
                        let mut x: u8 =
                            ((((((((&raw const gSpindaSpotGraphics).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 36))
                            .read()) as u32)
                                .wrapping_add((personality & 15u32).wrapping_sub(8u32)))
                                as u8);
                        let mut y: u8 =
                            (((((((((&raw const gSpindaSpotGraphics).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 36))
                            .wrapping_add(1))
                            .read()) as u32)
                                .wrapping_add(((personality & 240u32) >> 4).wrapping_sub(8u32)))
                                as u8);
                        {
                            row = 0i32;
                            'l3: loop {
                                if !(row < 16i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    let mut column: i32 = 0i32;
                                    let mut spotPixelRow: i32 =
                                        (((((((((&raw const gSpindaSpotGraphics)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 36))
                                        .wrapping_add(2))
                                        .cast::<u16>())
                                        .wrapping_offset((row) as isize))
                                        .read()) as i32);
                                    {
                                        column = ((x) as i32);
                                        'l5: loop {
                                            if !(column < ((x) as i32).wrapping_add(16i32)) {
                                                break 'l5;
                                            }
                                            'l6: {
                                                let mut destPixels: *mut u8 = ((((dest)
                                                    .wrapping_offset(
                                                        ((crate::c::div_i32(column, 8i32))
                                                            .wrapping_mul(crate::c::div_i32(
                                                                256i32, 8i32,
                                                            )))
                                                            as isize,
                                                    ))
                                                .wrapping_offset(
                                                    (crate::c::div_i32(
                                                        crate::c::rem_i32(column, 8i32),
                                                        2i32,
                                                    ))
                                                        as isize,
                                                ))
                                                .wrapping_offset(
                                                    (((crate::c::div_i32(((y) as i32), 8i32))
                                                        .wrapping_mul(crate::c::div_i32(
                                                            256i32, 8i32,
                                                        )))
                                                    .wrapping_mul(8i32))
                                                        as isize,
                                                ))
                                                .wrapping_offset(
                                                    ((crate::c::rem_i32(((y) as i32), 8i32))
                                                        .wrapping_mul(4i32))
                                                        as isize,
                                                );
                                                if (spotPixelRow & 1i32) != 0 {
                                                    if (column & 1i32) != 0 {
                                                        if (((((destPixels).read()) as i32)
                                                            & 240i32)
                                                            >= 16i32)
                                                            && (((((destPixels).read()) as i32)
                                                                & 240i32)
                                                                <= 48i32)
                                                        {
                                                            (destPixels).write(
                                                                (((((destPixels).read()) as i32)
                                                                    .wrapping_add(64i32))
                                                                    as u8),
                                                            );
                                                        }
                                                    } else {
                                                        if (((((destPixels).read()) as i32)
                                                            & 15i32)
                                                            >= 1i32)
                                                            && (((((destPixels).read()) as i32)
                                                                & 15i32)
                                                                <= 3i32)
                                                        {
                                                            (destPixels).write(
                                                                (((((destPixels).read()) as i32)
                                                                    .wrapping_add(4i32))
                                                                    as u8),
                                                            );
                                                        }
                                                    }
                                                }
                                                spotPixelRow = (spotPixelRow >> 1);
                                            }
                                            column = (column).wrapping_add(1);
                                        }
                                    }
                                    y = (y).wrapping_add(1);
                                }
                                row = (row).wrapping_add(1);
                            }
                        }
                        personality = (personality >> 8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionRenameMon(mon: *mut u8, oldSpecies: u16, newSpecies: u16) {
    unsafe {
        let mut mon = mon;
        let mut oldSpecies = oldSpecies;
        let mut newSpecies = newSpecies;
        let mut language: u8 = 0u8;
        GetMonData3(mon, 2i32, (&raw mut gStringVar1).cast::<u8>());
        language = ((GetMonData3(mon, 3i32, &raw mut language)) as u8);
        if (((language) as i32) == 2i32)
            && (!((StringCompare(
                (((&raw mut gSpeciesNames).cast::<u8>())
                    .wrapping_offset(((oldSpecies) as i32) as isize * 11))
                .cast::<u8>(),
                (&raw mut gStringVar1).cast::<u8>(),
            )) != 0))
        {
            SetMonData(
                mon,
                2i32,
                (((&raw mut gSpeciesNames).cast::<u8>())
                    .wrapping_offset(((newSpecies) as i32) as isize * 11))
                .cast::<u8>(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerFlankId() -> u8 {
    unsafe {
        let mut flankId: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
            .wrapping_add(24)
            .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 || __sw1 == 3i32 {
                flankId = 0u8;
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 2i32 {
                flankId = 1u8;
                break 'l1;
            }
        }
        return flankId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkTrainerFlankId(linkPlayerId: u8) -> u16 {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        let mut flankId: u16 = 0u16;
        'l1: {
            let __sw1 = ((((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((linkPlayerId) as i32) as isize * 28))
            .wrapping_add(24)
            .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 || __sw1 == 3i32 {
                flankId = 0u16;
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 2i32 {
                flankId = 1u16;
                break 'l1;
            }
        }
        return flankId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerMultiplayerId(id: u16) -> i32 {
    unsafe {
        let mut id = id;
        let mut multiplayerId: i32 = 0i32;
        {
            multiplayerId = 0i32;
            'l1: loop {
                if !(multiplayerId < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset((multiplayerId) as isize * 28))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .read()) as i32)
                        == ((id) as i32)
                    {
                        break 'l1;
                    }
                }
                multiplayerId = (multiplayerId).wrapping_add(1);
            }
        }
        return multiplayerId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerEncounterMusicId(trainerOpponentId: u16) -> u8 {
    unsafe {
        let mut trainerOpponentId = trainerOpponentId;
        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
            return GetTrainerEncounterMusicIdInBattlePyramid(trainerOpponentId);
        } else {
            if (InTrainerHillChallenge()) != 0 {
                return GetTrainerEncounterMusicIdInTrainerHill(trainerOpponentId);
            } else {
                return ((((((((&raw mut gTrainers).cast::<u8>())
                    .wrapping_offset(((trainerOpponentId) as i32) as isize * 40))
                .wrapping_add(2))
                .read()) as i32)
                    & 127i32) as u8);
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ModifyStatByNature(nature: u8, stat: u16, statIndex: u8) -> u16 {
    unsafe {
        let mut nature = nature;
        let mut stat = stat;
        let mut statIndex = statIndex;
        let mut retVal: u16 = 0u16;
        if (((statIndex) as i32) <= 0i32) || (((statIndex) as i32) > 5i32) {
            return stat;
        }
        'l1: {
            let __sw1 = ((((((((&raw const gNatureStatTable).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((nature) as i32) as isize * 5))
            .cast::<i8>())
            .wrapping_offset((((statIndex) as i32).wrapping_sub(1i32)) as isize))
            .read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == (-1i32);
            if __sw1 == 1i32 {
                retVal = ((((stat) as i32).wrapping_mul(110i32)) as u16);
                retVal = ((crate::c::div_i32(((retVal) as i32), 100i32)) as u16);
                break 'l1;
            }
            if __sw1 == (-1i32) {
                retVal = ((((stat) as i32).wrapping_mul(90i32)) as u16);
                retVal = ((crate::c::div_i32(((retVal) as i32), 100i32)) as u16);
                break 'l1;
            }
            if !__matched {
                retVal = stat;
                break 'l1;
            }
        }
        return retVal;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AdjustFriendship(mon: *mut u8, event: u8) {
    unsafe {
        let mut mon = mon;
        let mut event = event;
        let mut species: u16 = 0u16;
        let mut heldItem: u16 = 0u16;
        let mut holdEffect: u8 = 0u8;
        let mut r#mod: i8 = 0i8;
        if (ShouldSkipFriendshipChange()) != 0 {
            return;
        }
        species = ((GetMonData3(mon, 65i32, core::ptr::null_mut())) as u16);
        heldItem = ((GetMonData3(mon, 12i32, core::ptr::null_mut())) as u16);
        if ((heldItem) as i32) == 175i32 {
            if (crate::c::bf_read(
                ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                1,
                1,
                false,
            ) as u8)
                != 0
            {
                holdEffect = (((&raw mut gEnigmaBerries).cast::<u8>()).wrapping_add(7)).read();
            } else {
                holdEffect = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(12792))
                .wrapping_add(46))
                .read();
            }
        } else {
            holdEffect = GetItemHoldEffect(heldItem);
        }
        if ((species) != 0) && (((species) as i32) != 412i32) {
            let mut friendshipLevel: u8 = 0u8;
            let mut friendship: i16 = ((GetMonData3(mon, 32i32, core::ptr::null_mut())) as i16);
            if ((friendship) as i32) > 99i32 {
                friendshipLevel = (friendshipLevel).wrapping_add(1);
            }
            if ((friendship) as i32) > 199i32 {
                friendshipLevel = (friendshipLevel).wrapping_add(1);
            }
            if ((event) as i32) == 5i32 {
                if (((Random()) as i32) & 1i32) != 0 {
                    return;
                }
            }
            if ((event) as i32) == 3i32 {
                if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0) {
                    return;
                }
                if !(((((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(1))
                .read()) as i32)
                    == 32i32)
                    || (((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                            as isize
                            * 40,
                    ))
                    .wrapping_add(1))
                    .read()) as i32)
                        == 31i32))
                    || (((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32)
                            as isize
                            * 40,
                    ))
                    .wrapping_add(1))
                    .read()) as i32)
                        == 38i32))
                {
                    return;
                }
            }
            r#mod = ((((((&raw const sFriendshipEventModifiers)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((event) as i32) as isize * 3))
            .cast::<i8>())
            .wrapping_offset(((friendshipLevel) as i32) as isize))
            .read();
            if (((r#mod) as i32) > 0i32) && (((holdEffect) as i32) == 27i32) {
                r#mod =
                    ((crate::c::div_i32((150i32).wrapping_mul(((r#mod) as i32)), 100i32)) as i8);
            }
            friendship = ((((friendship) as i32).wrapping_add(((r#mod) as i32))) as i16);
            if ((r#mod) as i32) > 0i32 {
                if GetMonData3(mon, 38i32, core::ptr::null_mut()) == 11u32 {
                    friendship = (friendship).wrapping_add(1);
                }
                if GetMonData3(mon, 35i32, core::ptr::null_mut())
                    == ((GetCurrentRegionMapSectionId()) as u32)
                {
                    friendship = (friendship).wrapping_add(1);
                }
            }
            if ((friendship) as i32) < 0i32 {
                friendship = 0i16;
            }
            if ((friendship) as i32) > 255i32 {
                friendship = 255i16;
            }
            SetMonData(mon, 32i32, (&raw mut friendship).cast::<u8>());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonGainEVs(mon: *mut u8, defeatedSpecies: u16) {
    unsafe {
        let mut mon = mon;
        let mut defeatedSpecies = defeatedSpecies;
        let mut evs = crate::ffi::Align4([0u8; 6]);
        let mut evIncrease: u16 = 0u16;
        let mut totalEVs: u16 = 0u16;
        let mut heldItem: u16 = 0u16;
        let mut holdEffect: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut multiplier: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut evs).cast::<u8>()).wrapping_offset((i) as isize)).write(
                        ((GetMonData3(mon, (26i32).wrapping_add(i), core::ptr::null_mut())) as u8),
                    );
                    totalEVs = ((((totalEVs) as i32).wrapping_add(
                        (((((&raw mut evs).cast::<u8>()).wrapping_offset((i) as isize)).read())
                            as i32),
                    )) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 6i32) {
                    break 'l3;
                }
                'l4: {
                    if ((totalEVs) as i32) >= 510i32 {
                        break 'l3;
                    }
                    if (CheckPartyHasHadPokerus(mon, 0u8)) != 0 {
                        multiplier = 2i32;
                    } else {
                        multiplier = 1i32;
                    }
                    'l5: {
                        let __sw1 = i;
                        if __sw1 == 0i32 {
                            evIncrease = ((((crate::c::bf_read(
                                ((((&raw const gSpeciesInfo).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((defeatedSpecies) as i32) as isize * 28))
                                .wrapping_add(10),
                                0,
                                2,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(multiplier))
                                as u16);
                            break 'l5;
                        }
                        if __sw1 == 1i32 {
                            evIncrease = ((((crate::c::bf_read(
                                ((((&raw const gSpeciesInfo).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((defeatedSpecies) as i32) as isize * 28))
                                .wrapping_add(10),
                                2,
                                2,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(multiplier))
                                as u16);
                            break 'l5;
                        }
                        if __sw1 == 2i32 {
                            evIncrease = ((((crate::c::bf_read(
                                ((((&raw const gSpeciesInfo).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((defeatedSpecies) as i32) as isize * 28))
                                .wrapping_add(10),
                                4,
                                2,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(multiplier))
                                as u16);
                            break 'l5;
                        }
                        if __sw1 == 3i32 {
                            evIncrease = ((((crate::c::bf_read(
                                ((((&raw const gSpeciesInfo).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((defeatedSpecies) as i32) as isize * 28))
                                .wrapping_add(10),
                                6,
                                2,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(multiplier))
                                as u16);
                            break 'l5;
                        }
                        if __sw1 == 4i32 {
                            evIncrease = ((((crate::c::bf_read(
                                ((((&raw const gSpeciesInfo).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((defeatedSpecies) as i32) as isize * 28))
                                .wrapping_add(11),
                                0,
                                2,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(multiplier))
                                as u16);
                            break 'l5;
                        }
                        if __sw1 == 5i32 {
                            evIncrease = ((((crate::c::bf_read(
                                ((((&raw const gSpeciesInfo).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((defeatedSpecies) as i32) as isize * 28))
                                .wrapping_add(11),
                                2,
                                2,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(multiplier))
                                as u16);
                            break 'l5;
                        }
                    }
                    heldItem = ((GetMonData3(mon, 12i32, core::ptr::null_mut())) as u16);
                    if ((heldItem) as i32) == 175i32 {
                        if (crate::c::bf_read(
                            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                            1,
                            1,
                            false,
                        ) as u8)
                            != 0
                        {
                            holdEffect =
                                (((&raw mut gEnigmaBerries).cast::<u8>()).wrapping_add(7)).read();
                        } else {
                            holdEffect = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(12792))
                            .wrapping_add(46))
                            .read();
                        }
                    } else {
                        holdEffect = GetItemHoldEffect(heldItem);
                    }
                    if ((holdEffect) as i32) == 24i32 {
                        evIncrease = ((((evIncrease) as i32).wrapping_mul(2i32)) as u16);
                    }
                    if ((totalEVs) as i32).wrapping_add((((evIncrease) as i16) as i32)) > 510i32 {
                        evIncrease = ((((((evIncrease) as i16) as i32).wrapping_add(510i32))
                            .wrapping_sub(((totalEVs) as i32).wrapping_add(((evIncrease) as i32))))
                            as u16);
                    }
                    if (((((&raw mut evs).cast::<u8>()).wrapping_offset((i) as isize)).read())
                        as i32)
                        .wrapping_add((((evIncrease) as i16) as i32))
                        > 255i32
                    {
                        let mut val1: i32 = (((evIncrease) as i16) as i32).wrapping_add(255i32);
                        let mut val2: i32 = (((((&raw mut evs).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            .wrapping_add(((evIncrease) as i32));
                        evIncrease = (((val1).wrapping_sub(val2)) as u16);
                    }
                    let __p2 = ((&raw mut evs).cast::<u8>()).wrapping_offset((i) as isize);
                    (__p2).write(
                        (((((__p2).read()) as i32).wrapping_add(((evIncrease) as i32))) as u8),
                    );
                    totalEVs = ((((totalEVs) as i32).wrapping_add(((evIncrease) as i32))) as u16);
                    SetMonData(
                        mon,
                        (26i32).wrapping_add(i),
                        ((&raw mut evs).cast::<u8>()).wrapping_offset((i) as isize),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonEVCount(mon: *mut u8) -> u16 {
    unsafe {
        let mut mon = mon;
        let mut i: i32 = 0i32;
        let mut count: u16 = 0u16;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    count = ((((count) as u32).wrapping_add(GetMonData3(
                        mon,
                        (26i32).wrapping_add(i),
                        core::ptr::null_mut(),
                    ))) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        return count;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RandomlyGivePartyPokerus(party: *mut u8) {
    unsafe {
        let mut party = party;
        let mut rnd: u16 = Random();
        if ((((rnd) as i32) == 16384i32) || (((rnd) as i32) == 32768i32))
            || (((rnd) as i32) == 49152i32)
        {
            let mut mon: *mut u8 = core::ptr::null_mut();
            'l1: loop {
                'l2: {
                    rnd = ((crate::c::rem_i32(((Random()) as i32), 6i32)) as u16);
                    mon = (party).wrapping_offset(((rnd) as i32) as isize * 100);
                }
                if !((!((GetMonData3(mon, 11i32, core::ptr::null_mut())) != 0))
                    || ((GetMonData3(mon, 45i32, core::ptr::null_mut())) != 0))
                {
                    break 'l1;
                }
            }
            if !((CheckPartyHasHadPokerus(
                party,
                ((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                    .wrapping_offset(((rnd) as i32) as isize))
                .read()) as u8),
            )) != 0)
            {
                let mut rnd2: u8 = 0u8;
                'l3: loop {
                    'l4: {
                        rnd2 = ((Random()) as u8);
                    }
                    if !((((rnd2) as i32) & 7i32) == 0i32) {
                        break 'l3;
                    }
                }
                if (((rnd2) as i32) & 240i32) != 0 {
                    rnd2 = ((((rnd2) as i32) & 7i32) as u8);
                }
                rnd2 = ((((rnd2) as i32) | (((rnd2) as i32) << 4)) as u8);
                rnd2 = ((((rnd2) as i32) & 243i32) as u8);
                rnd2 = (rnd2).wrapping_add(1);
                SetMonData(
                    (party).wrapping_offset(((rnd) as i32) as isize * 100),
                    34i32,
                    &raw mut rnd2,
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckPartyPokerus(party: *mut u8, selection: u8) -> u8 {
    unsafe {
        let mut party = party;
        let mut selection = selection;
        let mut retVal: u8 = 0u8;
        let mut partyIndex: i32 = 0i32;
        let mut curBit: u32 = 1u32;
        retVal = 0u8;
        if (selection) != 0 {
            'l1: loop {
                'l2: {
                    if ((((selection) as i32) & 1i32) != 0)
                        && ((GetMonData3(
                            (party).wrapping_offset((partyIndex) as isize * 100),
                            34i32,
                            core::ptr::null_mut(),
                        ) & 15u32)
                            != 0)
                    {
                        retVal = ((((retVal) as u32) | curBit) as u8);
                    }
                    partyIndex = (partyIndex).wrapping_add(1);
                    curBit = (curBit << 1);
                    selection = ((((selection) as i32) >> 1) as u8);
                }
                if !((selection) != 0) {
                    break 'l1;
                }
            }
        } else {
            if (GetMonData3(party, 34i32, core::ptr::null_mut()) & 15u32) != 0 {
                retVal = 1u8;
            }
        }
        return retVal;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckPartyHasHadPokerus(party: *mut u8, selection: u8) -> u8 {
    unsafe {
        let mut party = party;
        let mut selection = selection;
        let mut retVal: u8 = 0u8;
        let mut partyIndex: i32 = 0i32;
        let mut curBit: u32 = 1u32;
        retVal = 0u8;
        if (selection) != 0 {
            'l1: loop {
                'l2: {
                    if ((((selection) as i32) & 1i32) != 0)
                        && ((GetMonData3(
                            (party).wrapping_offset((partyIndex) as isize * 100),
                            34i32,
                            core::ptr::null_mut(),
                        )) != 0)
                    {
                        retVal = ((((retVal) as u32) | curBit) as u8);
                    }
                    partyIndex = (partyIndex).wrapping_add(1);
                    curBit = (curBit << 1);
                    selection = ((((selection) as i32) >> 1) as u8);
                }
                if !((selection) != 0) {
                    break 'l1;
                }
            }
        } else {
            if (GetMonData3(party, 34i32, core::ptr::null_mut())) != 0 {
                retVal = 1u8;
            }
        }
        return retVal;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdatePartyPokerusTime(days: u16) {
    unsafe {
        let mut days = days;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (GetMonData3(
                        (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 100),
                        11i32,
                        core::ptr::null_mut(),
                    )) != 0
                    {
                        let mut pokerus: u8 = ((GetMonData3(
                            (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            34i32,
                            core::ptr::null_mut(),
                        )) as u8);
                        if (((pokerus) as i32) & 15i32) != 0 {
                            if ((((pokerus) as i32) & 15i32) < ((days) as i32))
                                || (((days) as i32) > 4i32)
                            {
                                pokerus = ((((pokerus) as i32) & 240i32) as u8);
                            } else {
                                pokerus =
                                    ((((pokerus) as i32).wrapping_sub(((days) as i32))) as u8);
                            }
                            if ((pokerus) as i32) == 0i32 {
                                pokerus = 16u8;
                            }
                            SetMonData(
                                (((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                34i32,
                                &raw mut pokerus,
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PartySpreadPokerus(party: *mut u8) {
    unsafe {
        let mut party = party;
        if crate::c::rem_i32(((Random()) as i32), 3i32) == 0i32 {
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (GetMonData3(
                            (party).wrapping_offset((i) as isize * 100),
                            11i32,
                            core::ptr::null_mut(),
                        )) != 0
                        {
                            let mut pokerus: u8 = ((GetMonData3(
                                (party).wrapping_offset((i) as isize * 100),
                                34i32,
                                core::ptr::null_mut(),
                            )) as u8);
                            let mut curPokerus: u8 = pokerus;
                            if (pokerus) != 0 {
                                if (((pokerus) as i32) & 15i32) != 0 {
                                    if (i != 0i32)
                                        && (!((GetMonData3(
                                            (party).wrapping_offset(
                                                ((i).wrapping_sub(1i32)) as isize * 100,
                                            ),
                                            34i32,
                                            core::ptr::null_mut(),
                                        ) & 240u32)
                                            != 0))
                                    {
                                        SetMonData(
                                            (party).wrapping_offset(
                                                ((i).wrapping_sub(1i32)) as isize * 100,
                                            ),
                                            34i32,
                                            &raw mut curPokerus,
                                        );
                                    }
                                    if (i != 5i32)
                                        && (!((GetMonData3(
                                            (party).wrapping_offset(
                                                ((i).wrapping_add(1i32)) as isize * 100,
                                            ),
                                            34i32,
                                            core::ptr::null_mut(),
                                        ) & 240u32)
                                            != 0))
                                    {
                                        SetMonData(
                                            (party).wrapping_offset(
                                                ((i).wrapping_add(1i32)) as isize * 100,
                                            ),
                                            34i32,
                                            &raw mut curPokerus,
                                        );
                                        i = (i).wrapping_add(1);
                                    }
                                }
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryIncrementMonLevel(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut species: u16 = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
        let mut nextLevel: u8 =
            (((GetMonData3(mon, 56i32, core::ptr::null_mut())).wrapping_add(1u32)) as u8);
        let mut expPoints: u32 = GetMonData3(mon, 25i32, core::ptr::null_mut());
        if expPoints
            > ((((((&raw const gExperienceTables).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(19))
                    .read()) as i32) as isize
                        * 404,
                ))
            .cast::<u32>())
            .wrapping_offset(100))
            .read()
        {
            expPoints = ((((((&raw const gExperienceTables).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(
                (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .wrapping_add(19))
                .read()) as i32) as isize
                    * 404,
            ))
            .cast::<u32>())
            .wrapping_offset(100))
            .read();
            SetMonData(mon, 25i32, (&raw mut expPoints).cast::<u8>());
        }
        if (((nextLevel) as i32) > 100i32)
            || (expPoints
                < ((((((&raw const gExperienceTables).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(19))
                        .read()) as i32) as isize
                            * 404,
                    ))
                .cast::<u32>())
                .wrapping_offset(((nextLevel) as i32) as isize))
                .read())
        {
            return 0u8;
        } else {
            SetMonData(mon, 56i32, &raw mut nextLevel);
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanMonLearnTMHM(mon: *mut u8, tm: u8) -> u32 {
    unsafe {
        let mut mon = mon;
        let mut tm = tm;
        let mut species: u16 = ((GetMonData3(mon, 65i32, core::ptr::null_mut())) as u16);
        if ((species) as i32) == 412i32 {
            return 0u32;
        }
        if 8u32 <= 8u32 {
            if ((tm) as i32) < 32i32 {
                let mut mask: u32 = ((crate::c::shl_i32(1i32, ((tm) as u32))) as u32);
                return ((((((&raw const gTMHMLearnsets).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 8))
                .cast::<u32>())
                .read()
                    & mask);
            } else {
                let mut mask: u32 =
                    ((crate::c::shl_i32(1i32, ((((tm) as i32).wrapping_sub(32i32)) as u32)))
                        as u32);
                return (((((((&raw const gTMHMLearnsets).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 8))
                .cast::<u32>())
                .wrapping_offset(1))
                .read()
                    & mask);
            }
        } else {
            let mut index: u32 = ((crate::c::div_i32(((tm) as i32), 32i32)) as u32);
            let mut mask: u32 =
                ((crate::c::shl_i32(1i32, ((crate::c::rem_i32(((tm) as i32), 32i32)) as u32)))
                    as u32);
            return (((((((&raw const gTMHMLearnsets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 8))
            .cast::<u32>())
            .wrapping_offset(((index) as i32) as isize))
            .read()
                & mask);
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanSpeciesLearnTMHM(species: u16, tm: u8) -> u32 {
    unsafe {
        let mut species = species;
        let mut tm = tm;
        if ((species) as i32) == 412i32 {
            return 0u32;
        }
        if 8u32 <= 8u32 {
            if ((tm) as i32) < 32i32 {
                let mut mask: u32 = ((crate::c::shl_i32(1i32, ((tm) as u32))) as u32);
                return ((((((&raw const gTMHMLearnsets).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 8))
                .cast::<u32>())
                .read()
                    & mask);
            } else {
                let mut mask: u32 =
                    ((crate::c::shl_i32(1i32, ((((tm) as i32).wrapping_sub(32i32)) as u32)))
                        as u32);
                return (((((((&raw const gTMHMLearnsets).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 8))
                .cast::<u32>())
                .wrapping_offset(1))
                .read()
                    & mask);
            }
        } else {
            let mut index: u32 = ((crate::c::div_i32(((tm) as i32), 32i32)) as u32);
            let mut mask: u32 =
                ((crate::c::shl_i32(1i32, ((crate::c::rem_i32(((tm) as i32), 32i32)) as u32)))
                    as u32);
            return (((((((&raw const gTMHMLearnsets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 8))
            .cast::<u32>())
            .wrapping_offset(((index) as i32) as isize))
            .read()
                & mask);
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMoveRelearnerMoves(mon: *mut u8, moves: *mut u16) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut moves = moves;
        let mut learnedMoves = crate::ffi::Align4([0u8; 8]);
        let mut numMoves: u8 = 0u8;
        let mut species: u16 = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
        let mut level: u8 = ((GetMonData3(mon, 56i32, core::ptr::null_mut())) as u8);
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut learnedMoves).cast::<u16>()).wrapping_offset((i) as isize)).write(
                        ((GetMonData3(mon, (13i32).wrapping_add(i), core::ptr::null_mut())) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 20i32) {
                    break 'l3;
                }
                'l4: {
                    let mut moveLevel: u16 = 0u16;
                    if ((((((((&raw const gLevelUpLearnsets)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u16>())
                    .cast::<*mut u16>())
                    .wrapping_offset(((species) as i32) as isize))
                    .read())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 65535i32
                    {
                        break 'l3;
                    }
                    moveLevel = ((((((((((&raw const gLevelUpLearnsets)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u16>())
                    .cast::<*mut u16>())
                    .wrapping_offset(((species) as i32) as isize))
                    .read())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        & 65024i32) as u16);
                    if ((moveLevel) as i32) <= (((level) as i32) << 9) {
                        {
                            j = 0i32;
                            'l5: loop {
                                if !((j < 4i32)
                                    && ((((((&raw mut learnedMoves).cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                        != (((((((((&raw const gLevelUpLearnsets)
                                            .cast::<u8>()
                                            .cast_mut()
                                            .cast::<*mut u16>())
                                        .cast::<*mut u16>())
                                        .wrapping_offset(((species) as i32) as isize))
                                        .read())
                                        .wrapping_offset((i) as isize))
                                        .read())
                                            as i32)
                                            & 511i32)))
                                {
                                    break 'l5;
                                }
                                'l6: {}
                                j = (j).wrapping_add(1);
                            }
                        }
                        if j == 4i32 {
                            {
                                k = 0i32;
                                'l7: loop {
                                    if !((k < ((numMoves) as i32))
                                        && (((((moves).wrapping_offset((k) as isize)).read())
                                            as i32)
                                            != (((((((((&raw const gLevelUpLearnsets)
                                                .cast::<u8>()
                                                .cast_mut()
                                                .cast::<*mut u16>())
                                            .cast::<*mut u16>())
                                            .wrapping_offset(((species) as i32) as isize))
                                            .read())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                & 511i32)))
                                    {
                                        break 'l7;
                                    }
                                    'l8: {}
                                    k = (k).wrapping_add(1);
                                }
                            }
                            if k == ((numMoves) as i32) {
                                ((moves).wrapping_offset(
                                    (({
                                        let __t1 = numMoves;
                                        numMoves = (numMoves).wrapping_add(1);
                                        __t1
                                    }) as i32) as isize,
                                ))
                                .write(
                                    ((((((((((&raw const gLevelUpLearnsets)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<*mut u16>())
                                    .cast::<*mut u16>())
                                    .wrapping_offset(((species) as i32) as isize))
                                    .read())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        & 511i32) as u16),
                                );
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return numMoves;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLevelUpMovesBySpecies(species: u16, moves: *mut u16) -> u8 {
    unsafe {
        let mut species = species;
        let mut moves = moves;
        let mut numMoves: u8 = 0u8;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !((i < 20i32)
                    && (((((((((&raw const gLevelUpLearnsets)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u16>())
                    .cast::<*mut u16>())
                    .wrapping_offset(((species) as i32) as isize))
                    .read())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 65535i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((moves).wrapping_offset(
                        (({
                            let __t1 = numMoves;
                            numMoves = (numMoves).wrapping_add(1);
                            __t1
                        }) as i32) as isize,
                    ))
                    .write(
                        ((((((((((&raw const gLevelUpLearnsets)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((species) as i32) as isize))
                        .read())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            & 511i32) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        return numMoves;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumberOfRelearnableMoves(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut learnedMoves = crate::ffi::Align4([0u8; 8]);
        let mut moves = crate::ffi::Align4([0u8; 40]);
        let mut numMoves: u8 = 0u8;
        let mut species: u16 = ((GetMonData3(mon, 65i32, core::ptr::null_mut())) as u16);
        let mut level: u8 = ((GetMonData3(mon, 56i32, core::ptr::null_mut())) as u8);
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        if ((species) as i32) == 412i32 {
            return 0u8;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut learnedMoves).cast::<u16>()).wrapping_offset((i) as isize)).write(
                        ((GetMonData3(mon, (13i32).wrapping_add(i), core::ptr::null_mut())) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 20i32) {
                    break 'l3;
                }
                'l4: {
                    let mut moveLevel: u16 = 0u16;
                    if ((((((((&raw const gLevelUpLearnsets)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u16>())
                    .cast::<*mut u16>())
                    .wrapping_offset(((species) as i32) as isize))
                    .read())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 65535i32
                    {
                        break 'l3;
                    }
                    moveLevel = ((((((((((&raw const gLevelUpLearnsets)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u16>())
                    .cast::<*mut u16>())
                    .wrapping_offset(((species) as i32) as isize))
                    .read())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        & 65024i32) as u16);
                    if ((moveLevel) as i32) <= (((level) as i32) << 9) {
                        {
                            j = 0i32;
                            'l5: loop {
                                if !((j < 4i32)
                                    && ((((((&raw mut learnedMoves).cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                        != (((((((((&raw const gLevelUpLearnsets)
                                            .cast::<u8>()
                                            .cast_mut()
                                            .cast::<*mut u16>())
                                        .cast::<*mut u16>())
                                        .wrapping_offset(((species) as i32) as isize))
                                        .read())
                                        .wrapping_offset((i) as isize))
                                        .read())
                                            as i32)
                                            & 511i32)))
                                {
                                    break 'l5;
                                }
                                'l6: {}
                                j = (j).wrapping_add(1);
                            }
                        }
                        if j == 4i32 {
                            {
                                k = 0i32;
                                'l7: loop {
                                    if !((k < ((numMoves) as i32))
                                        && ((((((&raw mut moves).cast::<u16>())
                                            .wrapping_offset((k) as isize))
                                        .read())
                                            as i32)
                                            != (((((((((&raw const gLevelUpLearnsets)
                                                .cast::<u8>()
                                                .cast_mut()
                                                .cast::<*mut u16>())
                                            .cast::<*mut u16>())
                                            .wrapping_offset(((species) as i32) as isize))
                                            .read())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                & 511i32)))
                                    {
                                        break 'l7;
                                    }
                                    'l8: {}
                                    k = (k).wrapping_add(1);
                                }
                            }
                            if k == ((numMoves) as i32) {
                                (((&raw mut moves).cast::<u16>()).wrapping_offset(
                                    (({
                                        let __t1 = numMoves;
                                        numMoves = (numMoves).wrapping_add(1);
                                        __t1
                                    }) as i32) as isize,
                                ))
                                .write(
                                    ((((((((((&raw const gLevelUpLearnsets)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<*mut u16>())
                                    .cast::<*mut u16>())
                                    .wrapping_offset(((species) as i32) as isize))
                                    .read())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        & 511i32) as u16),
                                );
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return numMoves;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpeciesToPokedexNum(species: u16) -> u16 {
    unsafe {
        let mut species = species;
        if (IsNationalPokedexEnabled()) != 0 {
            return SpeciesToNationalPokedexNum(species);
        } else {
            species = SpeciesToHoennPokedexNum(species);
            if ((species) as i32) <= 202i32 {
                return species;
            }
            return 65535u16;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSpeciesInHoennDex(species: u16) -> u32 {
    unsafe {
        let mut species = species;
        if ((SpeciesToHoennPokedexNum(species)) as i32) > 202i32 {
            return 0u32;
        } else {
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBattleMonForms() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut gBattleMonForms).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleBGM() -> u16 {
    unsafe {
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4096u32) != 0 {
            return 480u16;
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16384u32) != 0 {
                return 479u16;
            } else {
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0 {
                    return 476u16;
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
                        let mut trainerClass: u8 = 0u8;
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32) != 0 {
                            trainerClass = GetFrontierOpponentClass(
                                ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                            );
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 67108864u32)
                                != 0
                            {
                                trainerClass = 10u8;
                            } else {
                                trainerClass = ((((&raw mut gTrainers).cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 40,
                                    ))
                                .wrapping_add(1))
                                .read();
                            }
                        }
                        'l1: {
                            let __sw1 = ((trainerClass) as i32);
                            let __matched = __sw1 == 13i32
                                || __sw1 == 53i32
                                || __sw1 == 3i32
                                || __sw1 == 9i32
                                || __sw1 == 11i32
                                || __sw1 == 49i32
                                || __sw1 == 32i32
                                || __sw1 == 38i32
                                || __sw1 == 50i32
                                || __sw1 == 31i32
                                || __sw1 == 58i32
                                || __sw1 == 59i32
                                || __sw1 == 60i32
                                || __sw1 == 61i32
                                || __sw1 == 62i32
                                || __sw1 == 63i32
                                || __sw1 == 64i32;
                            if __sw1 == 13i32 || __sw1 == 53i32 {
                                return 483u16;
                            }
                            if __sw1 == 3i32 || __sw1 == 9i32 || __sw1 == 11i32 || __sw1 == 49i32 {
                                return 475u16;
                            }
                            if __sw1 == 32i32 {
                                return 477u16;
                            }
                            if __sw1 == 38i32 {
                                return 478u16;
                            }
                            if __sw1 == 50i32 {
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32)
                                    != 0
                                {
                                    return 481u16;
                                }
                                if !((StringCompare(
                                    ((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>())
                                            .read())
                                            as i32)
                                            as isize
                                            * 40,
                                    ))
                                    .wrapping_add(4))
                                    .cast::<u8>(),
                                    (&raw mut gText_BattleWallyName).cast::<u8>(),
                                )) != 0)
                                {
                                    return 476u16;
                                }
                                return 481u16;
                            }
                            if __sw1 == 31i32 {
                                return 482u16;
                            }
                            if __sw1 == 58i32
                                || __sw1 == 59i32
                                || __sw1 == 60i32
                                || __sw1 == 61i32
                                || __sw1 == 62i32
                                || __sw1 == 63i32
                                || __sw1 == 64i32
                            {
                                return 471u16;
                            }
                            if !__matched {
                                return 476u16;
                            }
                        }
                    } else {
                        return 474u16;
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayBattleBGM() {
    unsafe {
        ResetMapMusic();
        m4aMPlayAllStop();
        PlayBGM(GetBattleBGM());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayMapChosenOrBattleBGM(songId: u16) {
    unsafe {
        let mut songId = songId;
        ResetMapMusic();
        m4aMPlayAllStop();
        if (songId) != 0 {
            PlayNewMapMusic(songId);
        } else {
            PlayNewMapMusic(GetBattleBGM());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_PlayMapChosenOrBattleBGM(songId: u16) {
    unsafe {
        let mut songId = songId;
        let mut taskId: u8 = 0u8;
        ResetMapMusic();
        m4aMPlayAllStop();
        taskId = CreateTask(Some(Task_PlayMapChosenOrBattleBGM), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((songId) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_PlayMapChosenOrBattleBGM(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .read())
            != 0
        {
            PlayNewMapMusic(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u16),
            );
        } else {
            PlayNewMapMusic(GetBattleBGM());
        }
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonFrontSpritePal(mon: *mut u8) -> *mut u32 {
    unsafe {
        let mut mon = mon;
        let mut species: u16 = ((GetMonData3(mon, 65i32, core::ptr::null_mut())) as u16);
        let mut otId: u32 = GetMonData3(mon, 1i32, core::ptr::null_mut());
        let mut personality: u32 = GetMonData3(mon, 0i32, core::ptr::null_mut());
        return GetMonSpritePalFromSpeciesAndPersonality(species, otId, personality);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonSpritePalFromSpeciesAndPersonality(
    species: u16,
    otId: u32,
    personality: u32,
) -> *mut u32 {
    unsafe {
        let mut species = species;
        let mut otId = otId;
        let mut personality = personality;
        let mut shinyValue: u32 = 0u32;
        if ((species) as i32) > 412i32 {
            return (((&raw mut gMonPaletteTable).cast::<u8>()).cast::<*mut u32>()).read();
        }
        shinyValue = (((((otId & 4294901760u32) >> 16) ^ (otId & 65535u32))
            ^ ((personality & 4294901760u32) >> 16))
            ^ (personality & 65535u32));
        if shinyValue < 8u32 {
            return ((((&raw mut gMonShinyPaletteTable).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 8))
            .cast::<*mut u32>())
            .read();
        } else {
            return ((((&raw mut gMonPaletteTable).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 8))
            .cast::<*mut u32>())
            .read();
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonSpritePalStruct(mon: *mut u8) -> *mut u8 {
    unsafe {
        let mut mon = mon;
        let mut species: u16 = ((GetMonData3(mon, 65i32, core::ptr::null_mut())) as u16);
        let mut otId: u32 = GetMonData3(mon, 1i32, core::ptr::null_mut());
        let mut personality: u32 = GetMonData3(mon, 0i32, core::ptr::null_mut());
        return GetMonSpritePalStructFromOtIdPersonality(species, otId, personality);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonSpritePalStructFromOtIdPersonality(
    species: u16,
    otId: u32,
    personality: u32,
) -> *mut u8 {
    unsafe {
        let mut species = species;
        let mut otId = otId;
        let mut personality = personality;
        let mut shinyValue: u32 = 0u32;
        shinyValue = (((((otId & 4294901760u32) >> 16) ^ (otId & 65535u32))
            ^ ((personality & 4294901760u32) >> 16))
            ^ (personality & 65535u32));
        if shinyValue < 8u32 {
            return ((&raw mut gMonShinyPaletteTable).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 8);
        } else {
            return ((&raw mut gMonPaletteTable).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 8);
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsHMMove2(r#move: u16) -> u32 {
    unsafe {
        let mut r#move = r#move;
        let mut i: i32 = 0i32;
        'l1: loop {
            if !(((((((&raw const sHMMoves).cast::<u8>().cast_mut().cast::<u16>()).cast::<u16>())
                .wrapping_offset((i) as isize))
            .read()) as i32)
                != 65535i32)
            {
                break 'l1;
            }
            if ((((((&raw const sHMMoves).cast::<u8>().cast_mut().cast::<u16>()).cast::<u16>())
                .wrapping_offset(
                    ({
                        let __t1 = i;
                        i = (i).wrapping_add(1);
                        __t1
                    }) as isize,
                ))
            .read()) as i32)
                == ((r#move) as i32)
            {
                return 1u32;
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMonSpriteNotFlipped(species: u16) -> u8 {
    unsafe {
        let mut species = species;
        return (crate::c::bf_read(
            ((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 28))
            .wrapping_add(25),
            7,
            1,
            false,
        ) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonFlavorRelation(mon: *mut u8, flavor: u8) -> i8 {
    unsafe {
        let mut mon = mon;
        let mut flavor = flavor;
        let mut nature: u8 = GetNature(mon);
        return ((((&raw mut gPokeblockFlavorCompatibilityTable).cast::<i8>()).cast::<i8>())
            .wrapping_offset(
                ((((nature) as i32).wrapping_mul(5i32)).wrapping_add(((flavor) as i32))) as isize,
            ))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFlavorRelationByPersonality(personality: u32, flavor: u8) -> i8 {
    unsafe {
        let mut personality = personality;
        let mut flavor = flavor;
        let mut nature: u8 = GetNatureFromPersonality(personality);
        return ((((&raw mut gPokeblockFlavorCompatibilityTable).cast::<i8>()).cast::<i8>())
            .wrapping_offset(
                ((((nature) as i32).wrapping_mul(5i32)).wrapping_add(((flavor) as i32))) as isize,
            ))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsTradedMon(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut otName = crate::ffi::Align4([0u8; 8]);
        let mut otId: u32 = 0u32;
        GetMonData3(mon, 7i32, (&raw mut otName).cast::<u8>());
        otId = GetMonData3(mon, 1i32, core::ptr::null_mut());
        return IsOtherTrainer(otId, (&raw mut otName).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsOtherTrainer(otId: u32, otName: *mut u8) -> u8 {
    unsafe {
        let mut otId = otId;
        let mut otName = otName;
        if otId
            == (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .read()) as i32)
                | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                    .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    << 8))
                | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                    .cast::<u8>())
                .wrapping_offset(2))
                .read()) as i32)
                    << 16))
                | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                    .cast::<u8>())
                .wrapping_offset(3))
                .read()) as i32)
                    << 24)) as u32)
        {
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(((((otName).wrapping_offset((i) as isize)).read()) as i32) != 255i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((otName).wrapping_offset((i) as isize)).read()) as i32)
                            != (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                        {
                            return 1u8;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonRestorePP(mon: *mut u8) {
    unsafe {
        let mut mon = mon;
        BoxMonRestorePP((mon));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BoxMonRestorePP(boxMon: *mut u8) {
    unsafe {
        let mut boxMon = boxMon;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (GetBoxMonData3(boxMon, (13i32).wrapping_add(i), core::ptr::null_mut())) != 0
                    {
                        let mut r#move: u16 = ((GetBoxMonData3(
                            boxMon,
                            (13i32).wrapping_add(i),
                            core::ptr::null_mut(),
                        )) as u16);
                        let mut bonus: u16 =
                            ((GetBoxMonData3(boxMon, 21i32, core::ptr::null_mut())) as u16);
                        let mut pp: u8 = CalculatePPWithBonus(r#move, ((bonus) as u8), ((i) as u8));
                        SetBoxMonData(boxMon, (17i32).wrapping_add(i), &raw mut pp);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMonPreventsSwitchingString() {
    unsafe {
        ((&raw mut gLastUsedAbility).cast::<u8>()).write(
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(176)).read(),
        );
        ((&raw mut gBattleTextBuff1).cast::<u8>()).write(253u8);
        (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1)).write(4u8);
        (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(2))
            .write(((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(73)).read());
        (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(4)).write(255u8);
        if ((GetBattlerSide(
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(73)).read(),
        )) as i32)
            == 0i32
        {
            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3)).write(
                GetPartyIdFromBattlePartyId(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(73))
                            .read()) as i32) as isize,
                        ))
                    .read()) as u8),
                ),
            );
        } else {
            (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(3)).write(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(73))
                            .read()) as i32) as isize,
                    ))
                .read()) as u8),
            );
        }
        {
            ((&raw mut gBattleTextBuff2).cast::<u8>()).write(253u8);
            (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(1)).write(4u8);
            (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(2))
                .write(((&raw mut gBattlerInMenuId).cast::<u8>()).read());
            (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(3)).write(
                GetPartyIdFromBattlePartyId(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gBattlerInMenuId).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as u8),
                ),
            );
            (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(4)).write(255u8);
        }
        BattleStringExpandPlaceholders(
            (&raw mut gText_PkmnsXPreventsSwitching).cast::<u8>(),
            (&raw mut gStringVar4).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetWildMonTableIdInAlteringCave(species: u16) -> i32 {
    unsafe {
        let mut species = species;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(36u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sAlteringCaveWildMonHeldItems)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((species) as i32)
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWildMonHeldItem() {
    unsafe {
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 3153928u32) != 0) {
            let mut rnd: u16 = ((crate::c::rem_i32(((Random()) as i32), 100i32)) as u16);
            let mut species: u16 = ((GetMonData3(
                ((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>(),
                11i32,
                core::ptr::null_mut(),
            )) as u16);
            let mut chanceNoItem: u16 = 45u16;
            let mut chanceNotRare: u16 = 95u16;
            if (!((GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>(),
                6i32,
                core::ptr::null_mut(),
            )) != 0))
                && (((GetMonAbility(((&raw mut gPlayerParty).cast::<u8>()).cast::<u8>())) as i32)
                    == 14i32)
            {
                chanceNoItem = 20u16;
                chanceNotRare = 80u16;
            }
            if (((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 420i32
            {
                let mut alteringCaveId: i32 = GetWildMonTableIdInAlteringCave(species);
                if alteringCaveId != 0i32 {
                    if ((rnd) as i32) < ((chanceNotRare) as i32) {
                        return;
                    }
                    SetMonData(
                        ((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>(),
                        12i32,
                        (((((&raw const sAlteringCaveWildMonHeldItems)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((alteringCaveId) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .cast::<u8>(),
                    );
                } else {
                    if ((rnd) as i32) < ((chanceNoItem) as i32) {
                        return;
                    }
                    if ((rnd) as i32) < ((chanceNotRare) as i32) {
                        SetMonData(
                            ((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>(),
                            12i32,
                            (((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 28))
                            .wrapping_add(12)
                            .cast::<u16>())
                            .cast::<u8>(),
                        );
                    } else {
                        SetMonData(
                            ((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>(),
                            12i32,
                            (((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 28))
                            .wrapping_add(14)
                            .cast::<u16>())
                            .cast::<u8>(),
                        );
                    }
                }
            } else {
                if ((((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 28))
                .wrapping_add(12)
                .cast::<u16>())
                .read()) as i32)
                    == (((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(14)
                    .cast::<u16>())
                    .read()) as i32))
                    && ((((((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(12)
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32)
                {
                    SetMonData(
                        ((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>(),
                        12i32,
                        (((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(12)
                        .cast::<u16>())
                        .cast::<u8>(),
                    );
                } else {
                    if ((rnd) as i32) < ((chanceNoItem) as i32) {
                        return;
                    }
                    if ((rnd) as i32) < ((chanceNotRare) as i32) {
                        SetMonData(
                            ((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>(),
                            12i32,
                            (((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 28))
                            .wrapping_add(12)
                            .cast::<u16>())
                            .cast::<u8>(),
                        );
                    } else {
                        SetMonData(
                            ((&raw mut gEnemyParty).cast::<u8>()).cast::<u8>(),
                            12i32,
                            (((((&raw const gSpeciesInfo).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 28))
                            .wrapping_add(14)
                            .cast::<u16>())
                            .cast::<u8>(),
                        );
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMonShiny(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut otId: u32 = GetMonData3(mon, 1i32, core::ptr::null_mut());
        let mut personality: u32 = GetMonData3(mon, 0i32, core::ptr::null_mut());
        return IsShinyOtIdPersonality(otId, personality);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsShinyOtIdPersonality(otId: u32, personality: u32) -> u8 {
    unsafe {
        let mut otId = otId;
        let mut personality = personality;
        let mut retVal: u8 = 0u8;
        let mut shinyValue: u32 = (((((otId & 4294901760u32) >> 16) ^ (otId & 65535u32))
            ^ ((personality & 4294901760u32) >> 16))
            ^ (personality & 65535u32));
        if shinyValue < 8u32 {
            retVal = 1u8;
        }
        return retVal;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerPartnerName() -> *mut u8 {
    unsafe {
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4194304u32) != 0 {
            if ((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32) == 3075i32 {
                return ((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(32160))
                    .wrapping_add(4))
                .cast::<u8>();
            } else {
                GetFrontierTrainerName(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((&raw mut gPartnerTrainerId).cast::<u16>()).read(),
                );
                return (&raw mut gStringVar1).cast::<u8>();
            }
        } else {
            let mut id: u8 = GetMultiplayerId();
            return ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                (GetBattlerMultiplayerId(
                    ((((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 28))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .read()) as i32)
                        ^ 2i32) as u16),
                )) as isize
                    * 28,
            ))
            .wrapping_add(8))
            .cast::<u8>();
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_AnimateAfterDelay(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            LaunchAnimationTaskForFrontSprite(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u16) as i32)
                    | ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as u16) as i32)
                        << 16)) as usize as *mut u8),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u8),
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PokemonSummaryAnimateAfterDelay(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            StartMonSummaryAnimation(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u16) as i32)
                    | ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as u16) as i32)
                        << 16)) as usize as *mut u8),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u8),
            );
            SummaryScreen_SetAnimDelayTaskId(255u8);
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAnimateFrontSprite(
    sprite: *mut u8,
    species: u16,
    noCry: u8,
    panMode: u8,
) {
    unsafe {
        let mut sprite = sprite;
        let mut species = species;
        let mut noCry = noCry;
        let mut panMode = panMode;
        if ((((&raw mut gHitMarker).cast::<u32>()).read() & 128u32) != 0)
            && (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0))
        {
            DoMonFrontSpriteAnimation(
                sprite,
                species,
                noCry,
                ((((panMode) as i32) | 128i32) as u8),
            );
        } else {
            DoMonFrontSpriteAnimation(sprite, species, noCry, panMode);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoMonFrontSpriteAnimation(
    sprite: *mut u8,
    species: u16,
    noCry: u8,
    panModeAnimFlag: u8,
) {
    unsafe {
        let mut sprite = sprite;
        let mut species = species;
        let mut noCry = noCry;
        let mut panModeAnimFlag = panModeAnimFlag;
        let mut pan: i8 = 0i8;
        'l1: {
            let __sw1 = (((panModeAnimFlag) as i32) & 127i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                pan = (-25i8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                pan = 25i8;
                break 'l1;
            }
            if !__matched {
                pan = 0i8;
                break 'l1;
            }
        }
        if (((panModeAnimFlag) as i32) & 128i32) != 0 {
            if !((noCry) != 0) {
                PlayCry_Normal(species, pan);
            }
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        } else {
            if !((noCry) != 0) {
                PlayCry_Normal(species, pan);
                if (HasTwoFramesAnimation(species)) != 0 {
                    StartSpriteAnim(sprite, 1u8);
                }
            }
            if ((((((&raw const sMonAnimationDelayTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((((species) as i32).wrapping_sub(1i32)) as isize))
            .read()) as i32)
                != 0i32
            {
                let mut taskId: u8 = CreateTask(Some(Task_AnimateAfterDelay), 0u8);
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write((((sprite) as usize as u32) as i16));
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(((((sprite) as usize as u32) >> 16) as i16));
                }
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(
                    ((((((&raw const sMonFrontAnimIdsTable).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((((species) as i32).wrapping_sub(1i32)) as isize))
                    .read()) as i16),
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(
                    ((((((&raw const sMonAnimationDelayTable).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset((((species) as i32).wrapping_sub(1i32)) as isize))
                    .read()) as i16),
                );
            } else {
                LaunchAnimationTaskForFrontSprite(
                    sprite,
                    ((((&raw const sMonFrontAnimIdsTable).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((((species) as i32).wrapping_sub(1i32)) as isize))
                    .read(),
                );
            }
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy_2));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokemonSummaryDoMonAnimation(sprite: *mut u8, species: u16, oneFrame: u8) {
    unsafe {
        let mut sprite = sprite;
        let mut species = species;
        let mut oneFrame = oneFrame;
        if (!((oneFrame) != 0)) && ((HasTwoFramesAnimation(species)) != 0) {
            StartSpriteAnim(sprite, 1u8);
        }
        if ((((((&raw const sMonAnimationDelayTable).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((((species) as i32).wrapping_sub(1i32)) as isize))
        .read()) as i32)
            != 0i32
        {
            let mut taskId: u8 = CreateTask(Some(Task_PokemonSummaryAnimateAfterDelay), 0u8);
            {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write((((sprite) as usize as u32) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(((((sprite) as usize as u32) >> 16) as i16));
            }
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(
                ((((((&raw const sMonFrontAnimIdsTable).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((((species) as i32).wrapping_sub(1i32)) as isize))
                .read()) as i16),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(
                ((((((&raw const sMonAnimationDelayTable).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((((species) as i32).wrapping_sub(1i32)) as isize))
                .read()) as i16),
            );
            SummaryScreen_SetAnimDelayTaskId(taskId);
            SetSpriteCB_MonAnimDummy(sprite);
        } else {
            StartMonSummaryAnimation(
                sprite,
                ((((&raw const sMonFrontAnimIdsTable).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((((species) as i32).wrapping_sub(1i32)) as isize))
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopPokemonAnimationDelayTask() {
    unsafe {
        let mut delayTaskId: u8 = FindTaskIdByFunc(Some(Task_PokemonSummaryAnimateAfterDelay));
        if ((delayTaskId) as i32) != 255i32 {
            DestroyTask(delayTaskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAnimateBackSprite(sprite: *mut u8, species: u16) {
    unsafe {
        let mut sprite = sprite;
        let mut species = species;
        if ((((&raw mut gHitMarker).cast::<u32>()).read() & 128u32) != 0)
            && (!((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0))
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        } else {
            LaunchAnimationTaskForBackSprite(sprite, GetSpeciesBackAnimSet(species));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy_2));
        }
    }
}
pub(crate) unsafe extern "C" fn GetOwnOpposingLinkMultiBattlerId(rightSide: u8) -> u8 {
    unsafe {
        let mut rightSide = rightSide;
        let mut i: i32 = 0i32;
        let mut battler: i32 = 0i32;
        let mut multiplayerId: u8 = GetMultiplayerId();
        'l1: {
            let __sw1 = ((((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((multiplayerId) as i32) as isize * 28))
            .wrapping_add(24)
            .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 || __sw1 == 2i32 {
                battler = (if (rightSide) != 0 { 1i32 } else { 3i32 });
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 3i32 {
                battler = (if (rightSide) != 0 { 2i32 } else { 0i32 });
                break 'l1;
            }
        }
        {
            i = 0i32;
            'l2: loop {
                if !(i < 4i32) {
                    break 'l2;
                }
                'l3: {
                    if ((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset((i) as isize * 28))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .read()) as i32)
                        == (((battler) as i16) as i32)
                    {
                        break 'l2;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((i) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetOpposingLinkMultiBattlerId(rightSide: u8, multiplayerId: u8) -> u8 {
    unsafe {
        let mut rightSide = rightSide;
        let mut multiplayerId = multiplayerId;
        let mut i: i32 = 0i32;
        let mut battler: i32 = 0i32;
        'l1: {
            let __sw1 = ((((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((multiplayerId) as i32) as isize * 28))
            .wrapping_add(24)
            .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 || __sw1 == 2i32 {
                battler = (if (rightSide) != 0 { 1i32 } else { 3i32 });
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 3i32 {
                battler = (if (rightSide) != 0 { 2i32 } else { 0i32 });
                break 'l1;
            }
        }
        {
            i = 0i32;
            'l2: loop {
                if !(i < 4i32) {
                    break 'l2;
                }
                'l3: {
                    if ((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset((i) as isize * 28))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .read()) as i32)
                        == (((battler) as i16) as i32)
                    {
                        break 'l2;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((i) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FacilityClassToPicIndex(facilityClass: u16) -> u16 {
    unsafe {
        let mut facilityClass = facilityClass;
        return ((((((&raw const gFacilityClassToPicIndex)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(((facilityClass) as i32) as isize))
        .read()) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerGenderToFrontTrainerPicId(playerGender: u8) -> u16 {
    unsafe {
        let mut playerGender = playerGender;
        if ((playerGender) as i32) != 0i32 {
            return FacilityClassToPicIndex(63u16);
        } else {
            return FacilityClassToPicIndex(60u16);
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleSetPokedexFlag(nationalNum: u16, caseId: u8, personality: u32) {
    unsafe {
        let mut nationalNum = nationalNum;
        let mut caseId = caseId;
        let mut personality = personality;
        let mut getFlagCaseId: u8 = ((if ((caseId) as i32) == 2i32 {
            0i32
        } else {
            1i32
        }) as u8);
        if !((GetSetPokedexFlag(nationalNum, getFlagCaseId)) != 0) {
            GetSetPokedexFlag(nationalNum, caseId);
            if ((NationalPokedexNumToSpecies(nationalNum)) as i32) == 201i32 {
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                    .wrapping_add(4)
                    .cast::<u32>())
                .write(personality);
            }
            if ((NationalPokedexNumToSpecies(nationalNum)) as i32) == 308i32 {
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(24))
                    .wrapping_add(8)
                    .cast::<u32>())
                .write(personality);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerClassNameFromId(trainerId: u16) -> *mut u8 {
    unsafe {
        let mut trainerId = trainerId;
        if ((trainerId) as i32) >= 855i32 {
            trainerId = 0u16;
        }
        return (((&raw mut gTrainerClassNames).cast::<u8>()).wrapping_offset(
            ((((((&raw mut gTrainers).cast::<u8>())
                .wrapping_offset(((trainerId) as i32) as isize * 40))
            .wrapping_add(1))
            .read()) as i32) as isize
                * 13,
        ))
        .cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerNameFromId(trainerId: u16) -> *mut u8 {
    unsafe {
        let mut trainerId = trainerId;
        if ((trainerId) as i32) >= 855i32 {
            trainerId = 0u16;
        }
        return ((((&raw mut gTrainers).cast::<u8>())
            .wrapping_offset(((trainerId) as i32) as isize * 40))
        .wrapping_add(4))
        .cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasTwoFramesAnimation(species: u16) -> u8 {
    unsafe {
        let mut species = species;
        return (((((((species) as i32) != 385i32) && (((species) as i32) != 410i32))
            && (((species) as i32) != 308i32))
            && (((species) as i32) != 201i32)) as u8);
    }
}
pub(crate) unsafe extern "C" fn ShouldSkipFriendshipChange() -> u8 {
    unsafe {
        if ((crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0)
            && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4129024u32) != 0)
        {
            return 1u8;
        }
        if (!((crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0))
            && (((InBattlePike()) != 0) || (((CurrentBattlePyramidLocation()) as i32) != 0i32))
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn InitMonSpritesGfx_Battle(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < (crate::c::bf_read((gfx).wrapping_add(0), 0, 4, false) as u32))
                {
                    break 'l1;
                }
                'l2: {
                    (((gfx).wrapping_add(12).cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 24)
                        .cast::<crate::c::Rec4<24>>()
                        .write_unaligned(
                            (((&raw const gBattlerSpriteTemplates).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 24)
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                        );
                    {
                        j = 0u16;
                        'l3: loop {
                            if !(((j) as u32)
                                < (crate::c::bf_read((gfx).wrapping_add(1), 0, 8, false) as u32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                (((((gfx).wrapping_add(16).cast::<*mut u8>()).read())
                                    .wrapping_offset(
                                        (((((i) as u32).wrapping_mul(
                                            (crate::c::bf_read((gfx).wrapping_add(1), 0, 8, false)
                                                as u32),
                                        ))
                                        .wrapping_add(((j) as u32)))
                                            as i32)
                                            as isize
                                            * 8,
                                    ))
                                .cast::<*mut u8>())
                                .write(
                                    (((((gfx).wrapping_add(8).cast::<*mut *mut u8>()).read())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read())
                                    .wrapping_offset(
                                        (((j) as i32)
                                            .wrapping_mul(crate::c::div_i32(4096i32, 2i32)))
                                            as isize,
                                    ),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    (((((gfx).wrapping_add(12).cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 24))
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                    .write(
                        (((gfx).wrapping_add(16).cast::<*mut u8>()).read()).wrapping_offset(
                            ((((i) as u32).wrapping_mul(
                                (crate::c::bf_read((gfx).wrapping_add(1), 0, 8, false) as u32),
                            )) as i32) as isize
                                * 8,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitMonSpritesGfx_FullParty(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < (crate::c::bf_read((gfx).wrapping_add(0), 0, 4, false) as u32))
                {
                    break 'l1;
                }
                'l2: {
                    (((gfx).wrapping_add(12).cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 24)
                        .cast::<crate::c::Rec4<24>>()
                        .write_unaligned(
                            (&raw const sSpriteTemplate_64x64)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<crate::c::Rec4<24>>()
                                .read_unaligned(),
                        );
                    {
                        j = 0u16;
                        'l3: loop {
                            if !(((j) as u32)
                                < (crate::c::bf_read((gfx).wrapping_add(1), 0, 8, false) as u32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                (((((gfx).wrapping_add(16).cast::<*mut u8>()).read())
                                    .wrapping_offset(
                                        (((((i) as u32).wrapping_mul(
                                            (crate::c::bf_read((gfx).wrapping_add(0), 0, 4, false)
                                                as u32),
                                        ))
                                        .wrapping_add(((j) as u32)))
                                            as i32)
                                            as isize
                                            * 8,
                                    ))
                                .cast::<*mut u8>())
                                .write(
                                    (((((gfx).wrapping_add(8).cast::<*mut *mut u8>()).read())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read())
                                    .wrapping_offset(
                                        (((j) as i32)
                                            .wrapping_mul(crate::c::div_i32(4096i32, 2i32)))
                                            as isize,
                                    ),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    (((((gfx).wrapping_add(12).cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 24))
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                    .write(
                        (((gfx).wrapping_add(16).cast::<*mut u8>()).read()).wrapping_offset(
                            ((((i) as u32).wrapping_mul(
                                (crate::c::bf_read((gfx).wrapping_add(0), 0, 4, false) as u32),
                            )) as i32) as isize
                                * 8,
                        ),
                    );
                    (((((gfx).wrapping_add(12).cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 24))
                    .wrapping_add(8)
                    .cast::<*mut *mut u8>())
                    .write(((&raw mut gAnims_MonPic).cast::<*mut u8>()).cast::<*mut u8>());
                    (((((gfx).wrapping_add(12).cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 24))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .write(i);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonSpritesGfxManager(managerId: u8, mode: u8) -> *mut u8 {
    unsafe {
        let mut managerId = managerId;
        let mut mode = mode;
        let mut i: u8 = 0u8;
        let mut failureFlags: u8 = 0u8;
        let mut gfx: *mut u8 = core::ptr::null_mut();
        failureFlags = 0u8;
        managerId = ((crate::c::rem_i32(((managerId) as i32), 2i32)) as u8);
        gfx = AllocZeroed(20u32);
        if ((gfx) as usize) == 0usize {
            return core::ptr::null_mut();
        }
        'l1: {
            let __sw1 = ((mode) as i32);
            let __matched = __sw1 == 2i32 || __sw1 == 0i32;
            if __sw1 == 2i32 {
                crate::c::bf_write((gfx).wrapping_add(0), 0, 4, (7u32) as i32);
                crate::c::bf_write((gfx).wrapping_add(0), 4, 4, (7u32) as i32);
                crate::c::bf_write((gfx).wrapping_add(1), 0, 8, (4u32) as i32);
                crate::c::bf_write((gfx).wrapping_add(3), 0, 4, (1u32) as i32);
                crate::c::bf_write((gfx).wrapping_add(3), 4, 4, (2u32) as i32);
                break 'l1;
            }
            if __sw1 == 0i32 || !__matched {
                crate::c::bf_write((gfx).wrapping_add(0), 0, 4, (4u32) as i32);
                crate::c::bf_write((gfx).wrapping_add(0), 4, 4, (4u32) as i32);
                crate::c::bf_write((gfx).wrapping_add(1), 0, 8, (4u32) as i32);
                crate::c::bf_write((gfx).wrapping_add(3), 0, 4, (1u32) as i32);
                crate::c::bf_write((gfx).wrapping_add(3), 4, 4, (0u32) as i32);
                break 'l1;
            }
        }
        ((gfx).wrapping_add(4).cast::<*mut u8>()).write(AllocZeroed(
            (((crate::c::bf_read((gfx).wrapping_add(3), 0, 4, false) as u32)
                .wrapping_mul(((crate::c::div_i32(4096i32, 2i32)) as u32)))
            .wrapping_mul(4u32))
            .wrapping_mul((crate::c::bf_read((gfx).wrapping_add(0), 0, 4, false) as u32)),
        ));
        ((gfx).wrapping_add(8).cast::<*mut *mut u8>()).write(
            (AllocZeroed(
                (crate::c::bf_read((gfx).wrapping_add(0), 0, 4, false) as u32).wrapping_mul(32u32),
            ))
            .cast::<*mut u8>(),
        );
        if (((((gfx).wrapping_add(4).cast::<*mut u8>()).read()) as usize) == 0usize)
            || (((((gfx).wrapping_add(8).cast::<*mut *mut u8>()).read()) as usize) == 0usize)
        {
            failureFlags = ((((failureFlags) as i32) | 1i32) as u8);
        } else {
            {
                i = 0u8;
                'l2: loop {
                    if !(((i) as u32)
                        < (crate::c::bf_read((gfx).wrapping_add(0), 0, 4, false) as u32))
                    {
                        break 'l2;
                    }
                    'l3: {
                        ((((gfx).wrapping_add(8).cast::<*mut *mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(
                            (((gfx).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(
                                (((((crate::c::bf_read((gfx).wrapping_add(3), 0, 4, false)
                                    as u32)
                                    .wrapping_mul(((crate::c::div_i32(4096i32, 2i32)) as u32)))
                                .wrapping_mul(4u32))
                                .wrapping_mul(((i) as u32)))
                                    as i32) as isize
                                    * 1,
                            ),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        ((gfx).wrapping_add(12).cast::<*mut u8>()).write(AllocZeroed(
            (24u32).wrapping_mul((crate::c::bf_read((gfx).wrapping_add(0), 0, 4, false) as u32)),
        ));
        ((gfx).wrapping_add(16).cast::<*mut u8>()).write(AllocZeroed(
            ((8u32).wrapping_mul((crate::c::bf_read((gfx).wrapping_add(0), 0, 4, false) as u32)))
                .wrapping_mul((crate::c::bf_read((gfx).wrapping_add(1), 0, 8, false) as u32)),
        ));
        if (((((gfx).wrapping_add(12).cast::<*mut u8>()).read()) as usize) == 0usize)
            || (((((gfx).wrapping_add(16).cast::<*mut u8>()).read()) as usize) == 0usize)
        {
            failureFlags = ((((failureFlags) as i32) | 2i32) as u8);
        } else {
            {
                i = 0u8;
                'l4: loop {
                    if !(((i) as u32)
                        < (crate::c::bf_read((gfx).wrapping_add(1), 0, 8, false) as u32)
                            .wrapping_mul(
                                (crate::c::bf_read((gfx).wrapping_add(0), 0, 4, false) as u32),
                            ))
                    {
                        break 'l4;
                    }
                    'l5: {
                        (((((gfx).wrapping_add(16).cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .write(((crate::c::div_i32(4096i32, 2i32)) as u16));
                    }
                    i = (i).wrapping_add(1);
                }
            }
            'l6: {
                let __sw2 = (crate::c::bf_read((gfx).wrapping_add(3), 4, 4, false) as u32);
                let __matched = __sw2 == 2u32 || __sw2 == 0u32 || __sw2 == 1u32;
                if __sw2 == 2u32 {
                    InitMonSpritesGfx_FullParty(gfx);
                    break 'l6;
                }
                if __sw2 == 0u32 || __sw2 == 1u32 || !__matched {
                    InitMonSpritesGfx_Battle(gfx);
                    break 'l6;
                }
            }
        }
        if (((failureFlags) as i32) & 2i32) != 0 {
            if ((((gfx).wrapping_add(16).cast::<*mut u8>()).read()) as usize) != 0usize {
                Free(((gfx).wrapping_add(16).cast::<*mut u8>()).read());
                ((gfx).wrapping_add(16).cast::<*mut u8>()).write(core::ptr::null_mut());
            }
            if ((((gfx).wrapping_add(12).cast::<*mut u8>()).read()) as usize) != 0usize {
                Free(((gfx).wrapping_add(12).cast::<*mut u8>()).read());
                ((gfx).wrapping_add(12).cast::<*mut u8>()).write(core::ptr::null_mut());
            }
        }
        if (((failureFlags) as i32) & 1i32) != 0 {
            if ((((gfx).wrapping_add(8).cast::<*mut *mut u8>()).read()) as usize) != 0usize {
                Free((((gfx).wrapping_add(8).cast::<*mut *mut u8>()).read()).cast::<u8>());
                ((gfx).wrapping_add(8).cast::<*mut *mut u8>()).write(core::ptr::null_mut());
            }
            if ((((gfx).wrapping_add(4).cast::<*mut u8>()).read()) as usize) != 0usize {
                Free(((gfx).wrapping_add(4).cast::<*mut u8>()).read());
                ((gfx).wrapping_add(4).cast::<*mut u8>()).write(core::ptr::null_mut());
            }
        }
        if (failureFlags) != 0 {
            crate::c::memset(gfx, 0i32, 20u32);
            Free(gfx);
        } else {
            crate::c::bf_write((gfx).wrapping_add(2), 0, 8, (163u32) as i32);
            ((((&raw mut sMonSpritesGfxManagers)
                .cast::<u8>()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((managerId) as i32) as isize))
            .write(gfx);
        }
        return ((((&raw mut sMonSpritesGfxManagers)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((managerId) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyMonSpritesGfxManager(managerId: u8) {
    unsafe {
        let mut managerId = managerId;
        let mut gfx: *mut u8 = core::ptr::null_mut();
        managerId = ((crate::c::rem_i32(((managerId) as i32), 2i32)) as u8);
        gfx = ((((&raw mut sMonSpritesGfxManagers)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((managerId) as i32) as isize))
        .read();
        if ((gfx) as usize) == 0usize {
            return;
        }
        if (crate::c::bf_read((gfx).wrapping_add(2), 0, 8, false) as u32) != 163u32 {
            crate::c::memset(gfx, 0i32, 20u32);
        } else {
            if ((((gfx).wrapping_add(16).cast::<*mut u8>()).read()) as usize) != 0usize {
                Free(((gfx).wrapping_add(16).cast::<*mut u8>()).read());
                ((gfx).wrapping_add(16).cast::<*mut u8>()).write(core::ptr::null_mut());
            }
            if ((((gfx).wrapping_add(12).cast::<*mut u8>()).read()) as usize) != 0usize {
                Free(((gfx).wrapping_add(12).cast::<*mut u8>()).read());
                ((gfx).wrapping_add(12).cast::<*mut u8>()).write(core::ptr::null_mut());
            }
            if ((((gfx).wrapping_add(8).cast::<*mut *mut u8>()).read()) as usize) != 0usize {
                Free((((gfx).wrapping_add(8).cast::<*mut *mut u8>()).read()).cast::<u8>());
                ((gfx).wrapping_add(8).cast::<*mut *mut u8>()).write(core::ptr::null_mut());
            }
            if ((((gfx).wrapping_add(4).cast::<*mut u8>()).read()) as usize) != 0usize {
                Free(((gfx).wrapping_add(4).cast::<*mut u8>()).read());
                ((gfx).wrapping_add(4).cast::<*mut u8>()).write(core::ptr::null_mut());
            }
            crate::c::memset(gfx, 0i32, 20u32);
            Free(gfx);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonSpritesGfxManager_GetSpritePtr(
    managerId: u8,
    spriteNum: u8,
) -> *mut u8 {
    unsafe {
        let mut managerId = managerId;
        let mut spriteNum = spriteNum;
        let mut gfx: *mut u8 = ((((&raw mut sMonSpritesGfxManagers)
            .cast::<u8>()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset((crate::c::rem_i32(((managerId) as i32), 2i32)) as isize))
        .read();
        if (crate::c::bf_read((gfx).wrapping_add(2), 0, 8, false) as u32) != 163u32 {
            return core::ptr::null_mut();
        } else {
            if ((spriteNum) as u32)
                >= (crate::c::bf_read((gfx).wrapping_add(0), 0, 4, false) as u32)
            {
                spriteNum = 0u8;
            }
            return ((((gfx).wrapping_add(8).cast::<*mut *mut u8>()).read())
                .wrapping_offset(((spriteNum) as i32) as isize))
            .read();
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
