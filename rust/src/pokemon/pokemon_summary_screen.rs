//! Translated from `src/pokemon_summary_screen.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sNullDescription sPoundDescription sKarateChopDescription sDoubleSlapDescription sCometPunchDescription sMegaPunchDescription sPayDayDescription sFirePunchDescription sIcePunchDescription sThunderPunchDescription sScratchDescription sViceGripDescription sGuillotineDescription sRazorWindDescription sSwordsDanceDescription sCutDescription sGustDescription sWingAttackDescription sWhirlwindDescription sFlyDescription sBindDescription sSlamDescription sVineWhipDescription sStompDescription sDoubleKickDescription sMegaKickDescription sJumpKickDescription sRollingKickDescription sSandAttackDescription sHeadbuttDescription sHornAttackDescription sFuryAttackDescription sHornDrillDescription sTackleDescription sBodySlamDescription sWrapDescription sTakeDownDescription sThrashDescription sDoubleEdgeDescription sTailWhipDescription sPoisonStingDescription sTwineedleDescription sPinMissileDescription sLeerDescription sBiteDescription sGrowlDescription sRoarDescription sSingDescription sSupersonicDescription sSonicBoomDescription sDisableDescription sAcidDescription sEmberDescription sFlamethrowerDescription sMistDescription sWaterGunDescription sHydroPumpDescription sSurfDescription sIceBeamDescription sBlizzardDescription sPsybeamDescription sBubbleBeamDescription sAuroraBeamDescription sHyperBeamDescription sPeckDescription sDrillPeckDescription sSubmissionDescription sLowKickDescription sCounterDescription sSeismicTossDescription sStrengthDescription sAbsorbDescription sMegaDrainDescription sLeechSeedDescription sGrowthDescription sRazorLeafDescription sSolarBeamDescription sPoisonPowderDescription sStunSporeDescription sSleepPowderDescription sPetalDanceDescription sStringShotDescription sDragonRageDescription sFireSpinDescription sThunderShockDescription sThunderboltDescription sThunderWaveDescription sThunderDescription sRockThrowDescription sEarthquakeDescription sFissureDescription sDigDescription sToxicDescription sConfusionDescription sPsychicDescription sHypnosisDescription sMeditateDescription sAgilityDescription sQuickAttackDescription sRageDescription sTeleportDescription sNightShadeDescription sMimicDescription sScreechDescription sDoubleTeamDescription sRecoverDescription sHardenDescription sMinimizeDescription sSmokescreenDescription sConfuseRayDescription sWithdrawDescription sDefenseCurlDescription sBarrierDescription sLightScreenDescription sHazeDescription sReflectDescription sFocusEnergyDescription sBideDescription sMetronomeDescription sMirrorMoveDescription sSelfDestructDescription sEggBombDescription sLickDescription sSmogDescription sSludgeDescription sBoneClubDescription sFireBlastDescription sWaterfallDescription sClampDescription sSwiftDescription sSkullBashDescription sSpikeCannonDescription sConstrictDescription sAmnesiaDescription sKinesisDescription sSoftBoiledDescription sHiJumpKickDescription sGlareDescription sDreamEaterDescription sPoisonGasDescription sBarrageDescription sLeechLifeDescription sLovelyKissDescription sSkyAttackDescription sTransformDescription sBubbleDescription sDizzyPunchDescription sSporeDescription sFlashDescription sPsywaveDescription sSplashDescription sAcidArmorDescription sCrabhammerDescription sExplosionDescription sFurySwipesDescription sBonemerangDescription sRestDescription sRockSlideDescription sHyperFangDescription sSharpenDescription sConversionDescription sTriAttackDescription sSuperFangDescription sSlashDescription sSubstituteDescription sStruggleDescription sSketchDescription sTripleKickDescription sThiefDescription sSpiderWebDescription sMindReaderDescription sNightmareDescription sFlameWheelDescription sSnoreDescription sCurseDescription sFlailDescription sConversion2Description sAeroblastDescription sCottonSporeDescription sReversalDescription sSpiteDescription sPowderSnowDescription sProtectDescription sMachPunchDescription sScaryFaceDescription sFaintAttackDescription sSweetKissDescription sBellyDrumDescription sSludgeBombDescription sMudSlapDescription sOctazookaDescription sSpikesDescription sZapCannonDescription sForesightDescription sDestinyBondDescription sPerishSongDescription sIcyWindDescription sDetectDescription sBoneRushDescription sLockOnDescription sOutrageDescription sSandstormDescription sGigaDrainDescription sEndureDescription sCharmDescription sRolloutDescription sFalseSwipeDescription sSwaggerDescription sMilkDrinkDescription sSparkDescription sFuryCutterDescription sSteelWingDescription sMeanLookDescription sAttractDescription sSleepTalkDescription sHealBellDescription sReturnDescription sPresentDescription sFrustrationDescription sSafeguardDescription sPainSplitDescription sSacredFireDescription sMagnitudeDescription sDynamicPunchDescription sMegahornDescription sDragonBreathDescription sBatonPassDescription sEncoreDescription sPursuitDescription sRapidSpinDescription sSweetScentDescription sIronTailDescription sMetalClawDescription sVitalThrowDescription sMorningSunDescription sSynthesisDescription sMoonlightDescription sHiddenPowerDescription sCrossChopDescription sTwisterDescription sRainDanceDescription sSunnyDayDescription sCrunchDescription sMirrorCoatDescription sPsychUpDescription sExtremeSpeedDescription sAncientPowerDescription sShadowBallDescription sFutureSightDescription sRockSmashDescription sWhirlpoolDescription sBeatUpDescription sFakeOutDescription sUproarDescription sStockpileDescription sSpitUpDescription sSwallowDescription sHeatWaveDescription sHailDescription sTormentDescription sFlatterDescription sWillOWispDescription sMementoDescription sFacadeDescription sFocusPunchDescription sSmellingSaltDescription sFollowMeDescription sNaturePowerDescription sChargeDescription sTauntDescription sHelpingHandDescription sTrickDescription sRolePlayDescription sWishDescription sAssistDescription sIngrainDescription sSuperpowerDescription sMagicCoatDescription sRecycleDescription sRevengeDescription sBrickBreakDescription sYawnDescription sKnockOffDescription sEndeavorDescription sEruptionDescription sSkillSwapDescription sImprisonDescription sRefreshDescription sGrudgeDescription sSnatchDescription sSecretPowerDescription sDiveDescription sArmThrustDescription sCamouflageDescription sTailGlowDescription sLusterPurgeDescription sMistBallDescription sFeatherDanceDescription sTeeterDanceDescription sBlazeKickDescription sMudSportDescription sIceBallDescription sNeedleArmDescription sSlackOffDescription sHyperVoiceDescription sPoisonFangDescription sCrushClawDescription sBlastBurnDescription sHydroCannonDescription sMeteorMashDescription sAstonishDescription sWeatherBallDescription sAromatherapyDescription sFakeTearsDescription sAirCutterDescription sOverheatDescription sOdorSleuthDescription sRockTombDescription sSilverWindDescription sMetalSoundDescription sGrassWhistleDescription sTickleDescription sCosmicPowerDescription sWaterSpoutDescription sSignalBeamDescription sShadowPunchDescription sExtrasensoryDescription sSkyUppercutDescription sSandTombDescription sSheerColdDescription sMuddyWaterDescription sBulletSeedDescription sAerialAceDescription sIcicleSpearDescription sIronDefenseDescription sBlockDescription sHowlDescription sDragonClawDescription sFrenzyPlantDescription sBulkUpDescription sBounceDescription sMudShotDescription sPoisonTailDescription sCovetDescription sVoltTackleDescription sMagicalLeafDescription sWaterSportDescription sCalmMindDescription sLeafBladeDescription sDragonDanceDescription sRockBlastDescription sShockWaveDescription sWaterPulseDescription sDoomDesireDescription sPsychoBoostDescription gMoveDescriptionPointers sHardyNatureName sLonelyNatureName sBraveNatureName sAdamantNatureName sNaughtyNatureName sBoldNatureName sDocileNatureName sRelaxedNatureName sImpishNatureName sLaxNatureName sTimidNatureName sHastyNatureName sSeriousNatureName sJollyNatureName sNaiveNatureName sModestNatureName sMildNatureName sQuietNatureName sBashfulNatureName sRashNatureName sCalmNatureName sGentleNatureName sSassyNatureName sCarefulNatureName sQuirkyNatureName gNatureNamePointers sBgTemplates sStatusTilemap sStatusSlidingWindow1 sStatusSlidingWindow2 sPowerAccSlidingWindow sAppealJamSlidingWindow sMultiBattleOrder sSummaryTemplate sPageInfoTemplate sPageSkillsTemplate sPageMovesTemplate sTextColors sButtons_Gfx sTextPrinterFunctions sTextPrinterTasks sMemoNatureTextColor sMemoMiscTextColor sStatsLeftColumnLayout sStatsRightColumnLayout sMovesPPLayout sOamData_MoveTypes sSpriteAnim_TypeNormal sSpriteAnim_TypeFighting sSpriteAnim_TypeFlying sSpriteAnim_TypePoison sSpriteAnim_TypeGround sSpriteAnim_TypeRock sSpriteAnim_TypeBug sSpriteAnim_TypeGhost sSpriteAnim_TypeSteel sSpriteAnim_TypeMystery sSpriteAnim_TypeFire sSpriteAnim_TypeWater sSpriteAnim_TypeGrass sSpriteAnim_TypeElectric sSpriteAnim_TypePsychic sSpriteAnim_TypeIce sSpriteAnim_TypeDragon sSpriteAnim_TypeDark sSpriteAnim_CategoryCool sSpriteAnim_CategoryBeauty sSpriteAnim_CategoryCute sSpriteAnim_CategorySmart sSpriteAnim_CategoryTough sSpriteAnimTable_MoveTypes sSpriteSheet_MoveTypes sSpriteTemplate_MoveTypes sMoveTypeToOamPaletteNum sOamData_MoveSelector sSpriteAnim_MoveSelector0 sSpriteAnim_MoveSelector1 sSpriteAnim_MoveSelector2 sSpriteAnim_MoveSelector3 sSpriteAnim_MoveSelectorLeft sSpriteAnim_MoveSelectorRight sSpriteAnim_MoveSelectorMiddle sSpriteAnim_MoveSelector7 sSpriteAnim_MoveSelector8 sSpriteAnim_MoveSelector9 sSpriteAnimTable_MoveSelector sMoveSelectorSpriteSheet sMoveSelectorSpritePal sMoveSelectorSpriteTemplate sOamData_StatusCondition sSpriteAnim_StatusPoison sSpriteAnim_StatusParalyzed sSpriteAnim_StatusSleep sSpriteAnim_StatusFrozen sSpriteAnim_StatusBurn sSpriteAnim_StatusPokerus sSpriteAnim_StatusFaint sSpriteAnimTable_StatusCondition sStatusIconsSpriteSheet sStatusIconsSpritePalette sSpriteTemplate_StatusCondition sMarkings_Pal
#[allow(unused_imports)]
use crate::data::pokemon_summary_screen::*;

pub(crate) static mut sMonSummaryScreen: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastViewedMonIndex: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMoveSlotToReplace: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimDelayTaskId: u8 = 0u8;

unsafe extern "C" {
    static mut gAbilityDescriptionPointers: u8;
    static mut gAbilityNames: u8;
    static mut gBallSpriteTemplates: u8;
    static mut gBattleMoves: u8;
    static mut gBattleTypeFlags: u8;
    static mut gContestEffectDescriptionPointers: u8;
    static mut gContestEffects: u8;
    static mut gContestMoves: u8;
    static mut gEnemyParty: u8;
    static mut gExperienceTables: u8;
    static mut gLinkPlayers: u8;
    static mut gMPlayInfo_BGM: u8;
    static mut gMain: u8;
    static mut gMonFrontPicTable: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gMoveNames: u8;
    static mut gMoveTypes_Pal: u8;
    static mut gMultiuseSpriteTemplate: u8;
    static mut gPPTextPalette: u8;
    static mut gPPUpGetMask: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpeciesInfo: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gSummaryPage_BattleMoves_Tilemap: u8;
    static mut gSummaryPage_ContestMoves_Tilemap: u8;
    static mut gSummaryPage_InfoEgg_Tilemap: u8;
    static mut gSummaryPage_Info_Tilemap: u8;
    static mut gSummaryPage_Skills_Tilemap: u8;
    static mut gSummaryScreen_Gfx: u8;
    static mut gSummaryScreen_MoveEffect_Cancel_Tilemap: u8;
    static mut gSummaryScreen_Pal: u8;
    static mut gTasks: u8;
    static mut gText_Accuracy2: u8;
    static mut gText_Appeal: u8;
    static mut gText_Attack3: u8;
    static mut gText_BattleMoves: u8;
    static mut gText_Cancel: u8;
    static mut gText_Cancel2: u8;
    static mut gText_ContestMoves: u8;
    static mut gText_Defense3: u8;
    static mut gText_EggAboutToHatch: u8;
    static mut gText_EggFromHotSprings: u8;
    static mut gText_EggFromTraveler: u8;
    static mut gText_EggWillHatchSoon: u8;
    static mut gText_EggWillTakeALongTime: u8;
    static mut gText_EggWillTakeSomeTime: u8;
    static mut gText_EmptyString5: u8;
    static mut gText_ExpPoints: u8;
    static mut gText_FemaleSymbol: u8;
    static mut gText_FiveMarks: u8;
    static mut gText_HMMovesCantBeForgotten2: u8;
    static mut gText_HP4: u8;
    static mut gText_IDNumber2: u8;
    static mut gText_Info: u8;
    static mut gText_Jam: u8;
    static mut gText_LevelSymbol: u8;
    static mut gText_MaleSymbol: u8;
    static mut gText_NextLv: u8;
    static mut gText_None: u8;
    static mut gText_NumberClear01: u8;
    static mut gText_OTSlash: u8;
    static mut gText_OddEggFoundByCouple: u8;
    static mut gText_OneDash: u8;
    static mut gText_PeculiarEggNicePlace: u8;
    static mut gText_PeculiarEggTrade: u8;
    static mut gText_PkmnInfo: u8;
    static mut gText_PkmnSkills: u8;
    static mut gText_Power: u8;
    static mut gText_RentalPkmn: u8;
    static mut gText_RibbonsVar1: u8;
    static mut gText_SpAtk4: u8;
    static mut gText_SpDef4: u8;
    static mut gText_Speed2: u8;
    static mut gText_Status: u8;
    static mut gText_Switch: u8;
    static mut gText_ThreeDashes: u8;
    static mut gText_TwoDashes: u8;
    static mut gText_TypeSlash: u8;
    static mut gText_XNature: u8;
    static mut gText_XNatureFatefulEncounter: u8;
    static mut gText_XNatureHatchedAtYZ: u8;
    static mut gText_XNatureHatchedSomewhereAt: u8;
    static mut gText_XNatureMetAtYZ: u8;
    static mut gText_XNatureMetSomewhereAt: u8;
    static mut gText_XNatureObtainedInTrade: u8;
    static mut gText_XNatureProbablyMetAt: u8;
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
    fn AddWindow(a0: *mut u8) -> u16;
    fn AdvanceStorageMonIndex(a0: *mut u8, a1: u8, a2: u8, a3: u8) -> i16;
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BlitBitmapToWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn BoxMonToMon(a0: *mut u8, a1: *mut u8);
    fn BuildOamBuffer();
    fn CalculatePPWithBonus(a0: u16, a1: u8, a2: u8) -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckPartyHasHadPokerus(a0: *mut u8, a1: u8) -> u8;
    fn CheckPartyPokerus(a0: *mut u8, a1: u8) -> u8;
    fn ClearScheduledBgCopiesToVram();
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateMonMarkingAllCombosSprite(a0: u16, a1: u16, a2: *mut u16) -> *mut u8;
    fn CreateMonSpritesGfxManager(a0: u8, a1: u8) -> *mut u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroyMonSpritesGfxManager(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndFreeResources(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetAbilityBySpecies(a0: u16, a1: u8) -> u8;
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetCurrentPPToMaxPPState(a0: u8, a1: u8) -> u8;
    fn GetItemName(a0: u16) -> *mut u8;
    fn GetLRKeysPressed() -> u8;
    fn GetMapNameHandleAquaHideout(a0: *mut u8, a1: u16) -> *mut u8;
    fn GetMonAilment(a0: *mut u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut u8) -> u8;
    fn GetMonNickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn GetMonSpritePalStructFromOtIdPersonality(a0: u16, a1: u32, a2: u32) -> *mut u8;
    fn GetMultiplayerId() -> u8;
    fn GetNature(a0: *mut u8) -> u8;
    fn GetPlayerIDAsU32() -> u32;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn HandleLoadSpecialPokePic_2(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn InBattleFactory() -> u8;
    fn InSlateportBattleTent() -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsMonShiny(a0: *mut u8) -> u8;
    fn IsMonSpriteNotFlipped(a0: u16) -> u8;
    fn IsMoveHm(a0: u16) -> u8;
    fn IsMultiBattle() -> u8;
    fn ItemIdToBallId(a0: u16) -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LoadBallGfx(a0: u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn MenuHelpers_IsLinkActive() -> u8;
    fn MenuHelpers_ShouldWaitForLinkRecv() -> u8;
    fn MonSpritesGfxManager_GetSpritePtr(a0: u8, a1: u8) -> *mut u8;
    fn PlayCry_ByMode(a0: u16, a1: i8, a2: u8);
    fn PlaySE(a0: u16);
    fn PokemonSummaryDoMonAnimation(a0: *mut u8, a1: u16, a2: u8);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTempTileDataBuffers();
    fn ResetVramOamAndBgCntRegs();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetBgTilemapPalette(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn SetBoxMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShouldIgnoreDeoxysForm(a0: u8, a1: u8) -> u8;
    fn ShouldPlayNormalMonCry(a0: *mut u8) -> u32;
    fn ShowBg(a0: u8);
    fn SpeciesToPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StopCryAndClearCrySongs();
    fn StopPokemonAnimationDelayTask();
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCompareWithoutExtCtrlCodes(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayVolumeControl(a0: *mut u8, a1: u16, a2: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPokemonSummaryScreen(
    mode: u8,
    mons: *mut u8,
    monIndex: u8,
    maxMonIndex: u8,
    callback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut mode = mode;
        let mut mons = mons;
        let mut monIndex = monIndex;
        let mut maxMonIndex = maxMonIndex;
        let mut callback = callback;
        ((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(16632u32));
        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16572))
        .write(mode);
        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
            .write(mons);
        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16574))
        .write(monIndex);
        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16575))
        .write(maxMonIndex);
        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(callback);
        if ((mode) as i32) == 2i32 {
            ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16573))
            .write(1u8);
        } else {
            ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16573))
            .write(0u8);
        }
        'l1: {
            let __sw1 = ((mode) as i32);
            if __sw1 == 0i32 || __sw1 == 2i32 {
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16577))
                .write(0u8);
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16578))
                .write(3u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16577))
                .write(0u8);
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16578))
                .write(3u8);
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16584))
                .write(1u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16577))
                .write(2u8);
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16578))
                .write(3u8);
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16579))
                .write(1u8);
                break 'l1;
            }
        }
        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16576))
        .write(
            ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16577))
            .read(),
        );
        SummaryScreen_SetAnimDelayTaskId(255u8);
        if ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()) as usize) == 0usize {
            CreateMonSpritesGfxManager(0u8, 0u8);
        }
        SetMainCallback2(Some(CB2_InitSummaryScreen));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowSelectMovePokemonSummaryScreen(
    mons: *mut u8,
    monIndex: u8,
    maxMonIndex: u8,
    callback: Option<unsafe extern "C" fn()>,
    newMove: u16,
) {
    unsafe {
        let mut mons = mons;
        let mut monIndex = monIndex;
        let mut maxMonIndex = maxMonIndex;
        let mut callback = callback;
        let mut newMove = newMove;
        ShowPokemonSummaryScreen(3u8, mons, monIndex, maxMonIndex, callback);
        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16580)
            .cast::<u16>())
        .write(newMove);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPokemonSummaryScreenHandleDeoxys(
    mode: u8,
    mons: *mut u8,
    monIndex: u8,
    maxMonIndex: u8,
    callback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut mode = mode;
        let mut mons = mons;
        let mut monIndex = monIndex;
        let mut maxMonIndex = maxMonIndex;
        let mut callback = callback;
        ShowPokemonSummaryScreen(mode, mons, monIndex, maxMonIndex, callback);
        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16623))
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn MainCB2() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlank() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_InitSummaryScreen() {
    unsafe {
        'l1: loop {
            if !(((((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32)
                && (((LoadGraphics()) as i32) != 1i32))
                && (((MenuHelpers_IsLinkActive()) as i32) != 1i32))
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadGraphics() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
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
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 17i32
                || __sw1 == 18i32
                || __sw1 == 19i32
                || __sw1 == 20i32
                || __sw1 == 21i32
                || __sw1 == 22i32
                || __sw1 == 23i32
                || __sw1 == 24i32;
            if __sw1 == 0i32 {
                SetVBlankHBlankCallbacksToNull();
                ResetVramOamAndBgCntRegs();
                ClearScheduledBgCopiesToVram();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ScanlineEffect_Stop();
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetPaletteFade();
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (1u16) as i32,
                );
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetSpriteData();
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                FreeAllSpritePalettes();
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                InitBGs();
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>())
                .write(0i16);
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((DecompressGraphics()) as i32) != 0i32 {
                    let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                ResetWindows();
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                DrawPagination();
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                CopyMonToSummaryStruct(
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12),
                );
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>())
                .write(0i16);
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                if ((ExtractMonDataToSummaryStruct(
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12),
                )) as i32)
                    != 0i32
                {
                    let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                PrintMonInfo();
                let __p13 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                PrintPageNamesAndStats();
                let __p14 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p14).write(((__p14).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                PrintPageSpecificText(
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16576))
                    .read(),
                );
                let __p15 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 14i32 {
                SetDefaultTilemaps();
                let __p16 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 15i32 {
                PutPageWindowTilemaps(
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16576))
                    .read(),
                );
                let __p17 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p17).write(((__p17).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 16i32 {
                ResetSpriteIds();
                CreateMoveTypeIcons();
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>())
                .write(0i16);
                let __p18 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p18).write(((__p18).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 17i32 {
                (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16595))
                .cast::<u8>())
                .write(LoadMonGfxAndSprite(
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12),
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16624)
                        .cast::<i16>(),
                ));
                if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16595))
                .cast::<u8>())
                .read()) as i32)
                    != 255i32
                {
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16624)
                        .cast::<i16>())
                    .write(0i16);
                    let __p19 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p19).write(((__p19).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 18i32 {
                CreateMonMarkingsSprite(
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12),
                );
                let __p20 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p20).write(((__p20).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 19i32 {
                CreateCaughtBallSprite(
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12),
                );
                let __p21 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p21).write(((__p21).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 20i32 {
                CreateSetStatusSprite();
                let __p22 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p22).write(((__p22).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 21i32 {
                SetTypeIcons();
                let __p23 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p23).write(((__p23).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 22i32 {
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16572))
                .read()) as i32)
                    != 3i32
                {
                    CreateTask(Some(Task_HandleInput), 0u8);
                } else {
                    CreateTask(Some(Task_SetHandleReplaceMoveInput), 0u8);
                }
                let __p24 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p24).write(((__p24).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 23i32 {
                BlendPalettes(4294967295u32, 16u8, 0u16);
                let __p25 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p25).write(((__p25).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 24i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                let __p26 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p26).write(((__p26).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                SetVBlankCallback(Some(VBlank));
                SetMainCallback2(Some(MainCB2));
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn InitBGs() {
    unsafe {
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            1u8,
            ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .wrapping_offset(8192))
            .cast::<u8>())
            .cast::<u16>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u8>())
            .cast::<u16>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            3u8,
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .cast::<u8>())
            .cast::<u16>())
            .cast::<u8>(),
        );
        ResetAllBgsCoordinates();
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ScheduleBgCopyTilemapToVram(3u8);
        SetGpuReg(0u8, 4160u16);
        SetGpuReg(80u8, 0u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        ShowBg(3u8);
    }
}
pub(crate) unsafe extern "C" fn DecompressGraphics() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16624)
                .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ResetTempTileDataBuffers();
                DecompressAndCopyTileDataToVram(
                    1u8,
                    (((&raw mut gSummaryScreen_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                let __p2 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((FreeTempTileDataBuffersIfPossible()) as i32) != 1i32 {
                    LZDecompressWram(
                        ((&raw mut gSummaryPage_Info_Tilemap).cast::<u32>()).cast::<u32>(),
                        (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(188))
                        .cast::<u8>())
                        .cast::<u8>())
                        .cast::<u16>())
                        .cast::<u8>(),
                    );
                    let __p3 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(16624)
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                LZDecompressWram(
                    ((&raw mut gSummaryPage_InfoEgg_Tilemap).cast::<u32>()).cast::<u32>(),
                    ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(188))
                    .cast::<u8>())
                    .cast::<u8>())
                    .wrapping_offset(2048))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                let __p4 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                LZDecompressWram(
                    ((&raw mut gSummaryPage_Skills_Tilemap).cast::<u32>()).cast::<u32>(),
                    (((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(188))
                    .cast::<u8>())
                    .wrapping_offset(4096))
                    .cast::<u8>())
                    .wrapping_offset(2048))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                let __p5 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                LZDecompressWram(
                    ((&raw mut gSummaryPage_BattleMoves_Tilemap).cast::<u32>()).cast::<u32>(),
                    (((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(188))
                    .cast::<u8>())
                    .wrapping_offset(8192))
                    .cast::<u8>())
                    .wrapping_offset(2048))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                let __p6 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                LZDecompressWram(
                    ((&raw mut gSummaryPage_ContestMoves_Tilemap).cast::<u32>()).cast::<u32>(),
                    (((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(188))
                    .cast::<u8>())
                    .wrapping_offset(12288))
                    .cast::<u8>())
                    .wrapping_offset(2048))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                let __p7 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                LoadCompressedPalette(
                    ((&raw mut gSummaryScreen_Pal).cast::<u32>()).cast::<u32>(),
                    0u16,
                    256u16,
                );
                LoadPalette(
                    (((&raw mut gPPTextPalette).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                    129u16,
                    30u16,
                );
                let __p8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>();
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheet_MoveTypes).cast::<u8>().cast_mut(),
                );
                let __p9 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>();
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                LoadCompressedSpriteSheet(
                    (&raw const sMoveSelectorSpriteSheet)
                        .cast::<u8>()
                        .cast_mut(),
                );
                let __p10 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>();
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                LoadCompressedSpriteSheet(
                    (&raw const sStatusIconsSpriteSheet).cast::<u8>().cast_mut(),
                );
                let __p11 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>();
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                LoadCompressedSpritePalette(
                    (&raw const sStatusIconsSpritePalette)
                        .cast::<u8>()
                        .cast_mut(),
                );
                let __p12 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>();
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                LoadCompressedSpritePalette(
                    (&raw const sMoveSelectorSpritePal).cast::<u8>().cast_mut(),
                );
                let __p13 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>();
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                LoadCompressedPalette(
                    ((&raw mut gMoveTypes_Pal).cast::<u32>()).cast::<u32>(),
                    464u16,
                    96u16,
                );
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>())
                .write(0i16);
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CopyMonToSummaryStruct(mon: *mut u8) {
    unsafe {
        let mut mon = mon;
        if !((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16573))
        .read())
            != 0)
        {
            let mut partyMon: *mut u8 =
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read();
            mon.cast::<crate::c::Rec4<100>>().write_unaligned(
                (partyMon)
                    .wrapping_offset(
                        ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16574))
                        .read()) as i32) as isize
                            * 100,
                    )
                    .cast::<crate::c::Rec4<100>>()
                    .read_unaligned(),
            );
        } else {
            let mut boxMon: *mut u8 =
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read();
            BoxMonToMon(
                (boxMon).wrapping_offset(
                    ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16574))
                    .read()) as i32) as isize
                        * 80,
                ),
                mon,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ExtractMonDataToSummaryStruct(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut i: u32 = 0u32;
        let mut sum: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        'l1: {
            let __sw1 = ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16624)
                .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                ((sum).cast::<u16>()).write(((GetMonData2(mon, 11i32)) as u16));
                ((sum).wrapping_add(2).cast::<u16>()).write(((GetMonData2(mon, 65i32)) as u16));
                ((sum).wrapping_add(16).cast::<u32>()).write(GetMonData2(mon, 25i32));
                ((sum).wrapping_add(5)).write(((GetMonData2(mon, 56i32)) as u8));
                ((sum).wrapping_add(8)).write(((GetMonData2(mon, 46i32)) as u8));
                ((sum).wrapping_add(46).cast::<u16>()).write(((GetMonData2(mon, 12i32)) as u16));
                ((sum).wrapping_add(12).cast::<u32>()).write(GetMonData2(mon, 0i32));
                ((sum).wrapping_add(53)).write(((GetMonData2(mon, 4i32)) as u8));
                if (((sum).wrapping_add(53)).read()) != 0 {
                    ((sum).wrapping_add(4)).write(1u8);
                } else {
                    ((sum).wrapping_add(4)).write(((GetMonData2(mon, 45i32)) as u8));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = 0u32;
                    'l2: loop {
                        if !(i < 4u32) {
                            break 'l2;
                        }
                        'l3: {
                            ((((sum).wrapping_add(20)).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(((GetMonData2(mon, (((13u32).wrapping_add(i)) as i32))) as u16));
                            ((((sum).wrapping_add(28)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(((GetMonData2(mon, (((17u32).wrapping_add(i)) as i32))) as u8));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((sum).wrapping_add(52)).write(((GetMonData2(mon, 21i32)) as u8));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read()) as usize)
                    == (((&raw mut gPlayerParty).cast::<u8>()) as usize))
                    || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16572))
                    .read()) as i32)
                        == 2i32))
                    || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16623))
                    .read()) as i32)
                        == 1i32)
                {
                    ((sum).wrapping_add(51)).write(GetNature(mon));
                    ((sum).wrapping_add(32).cast::<u16>())
                        .write(((GetMonData2(mon, 57i32)) as u16));
                    ((sum).wrapping_add(34).cast::<u16>())
                        .write(((GetMonData2(mon, 58i32)) as u16));
                    ((sum).wrapping_add(36).cast::<u16>())
                        .write(((GetMonData2(mon, 59i32)) as u16));
                    ((sum).wrapping_add(38).cast::<u16>())
                        .write(((GetMonData2(mon, 60i32)) as u16));
                    ((sum).wrapping_add(40).cast::<u16>())
                        .write(((GetMonData2(mon, 62i32)) as u16));
                    ((sum).wrapping_add(42).cast::<u16>())
                        .write(((GetMonData2(mon, 63i32)) as u16));
                    ((sum).wrapping_add(44).cast::<u16>())
                        .write(((GetMonData2(mon, 61i32)) as u16));
                } else {
                    ((sum).wrapping_add(51)).write(GetNature(mon));
                    ((sum).wrapping_add(32).cast::<u16>())
                        .write(((GetMonData2(mon, 57i32)) as u16));
                    ((sum).wrapping_add(34).cast::<u16>())
                        .write(((GetMonData2(mon, 58i32)) as u16));
                    ((sum).wrapping_add(36).cast::<u16>())
                        .write(((GetMonData2(mon, 84i32)) as u16));
                    ((sum).wrapping_add(38).cast::<u16>())
                        .write(((GetMonData2(mon, 85i32)) as u16));
                    ((sum).wrapping_add(40).cast::<u16>())
                        .write(((GetMonData2(mon, 87i32)) as u16));
                    ((sum).wrapping_add(42).cast::<u16>())
                        .write(((GetMonData2(mon, 88i32)) as u16));
                    ((sum).wrapping_add(44).cast::<u16>())
                        .write(((GetMonData2(mon, 86i32)) as u16));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                GetMonData3(mon, 7i32, ((sum).wrapping_add(54)).cast::<u8>());
                ConvertInternationalString(
                    ((sum).wrapping_add(54)).cast::<u8>(),
                    ((GetMonData2(mon, 3i32)) as u8),
                );
                ((sum).wrapping_add(7)).write(GetMonAilment(mon));
                ((sum).wrapping_add(50)).write(((GetMonData2(mon, 49i32)) as u8));
                ((sum).wrapping_add(72).cast::<u32>()).write(GetMonData2(mon, 1i32));
                ((sum).wrapping_add(9)).write(((GetMonData2(mon, 35i32)) as u8));
                ((sum).wrapping_add(10)).write(((GetMonData2(mon, 36i32)) as u8));
                ((sum).wrapping_add(11)).write(((GetMonData2(mon, 37i32)) as u8));
                ((sum).wrapping_add(48).cast::<u16>()).write(((GetMonData2(mon, 32i32)) as u16));
                break 'l1;
            }
            if !__matched {
                ((sum).wrapping_add(6)).write(((GetMonData2(mon, 82i32)) as u8));
                return 1u8;
            }
        }
        let __p2 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16624)
            .cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetDefaultTilemaps() {
    unsafe {
        if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16576))
        .read()) as i32)
            != 2i32)
            && (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16576))
            .read()) as i32)
                != 3i32)
        {
            PositionPowerAccSlidingWindow(0u16, 255i16);
            PositionAppealJamSlidingWindow(0u16, 255i16, 0u16);
        } else {
            DrawContestMoveHearts(
                (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(20))
                .cast::<u16>())
                .wrapping_offset(
                    ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16582))
                    .read()) as i32) as isize,
                ))
                .read(),
            );
            TilemapFiveMovesDisplay(
                (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(188))
                .cast::<u8>())
                .wrapping_offset(8192))
                .cast::<u8>())
                .cast::<u16>(),
                3u16,
                0u8,
            );
            TilemapFiveMovesDisplay(
                (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(188))
                .cast::<u8>())
                .wrapping_offset(12288))
                .cast::<u8>())
                .cast::<u16>(),
                1u16,
                0u8,
            );
            SetBgTilemapBuffer(
                1u8,
                ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(188))
                .cast::<u8>())
                .wrapping_offset(12288))
                .cast::<u8>())
                .cast::<u16>())
                .cast::<u8>(),
            );
            SetBgTilemapBuffer(
                2u8,
                ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(188))
                .cast::<u8>())
                .wrapping_offset(8192))
                .cast::<u8>())
                .cast::<u16>())
                .cast::<u8>(),
            );
            ChangeBgX(2u8, 65536i32, 1u8);
            ClearWindowTilemap(19u8);
            ClearWindowTilemap(13u8);
        }
        if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(112))
        .wrapping_add(7))
        .read()) as i32)
            == 0i32
        {
            PositionStatusSlidingWindow(0u16, 255i16);
        } else {
            if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16576))
            .read()) as i32)
                != 2i32)
                && (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16576))
                .read()) as i32)
                    != 3i32)
            {
                PutWindowTilemap(13u8);
            }
        }
        LimitEggSummaryPageDisplay();
        DrawPokerusCuredSymbol(
            (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12),
        );
    }
}
pub(crate) unsafe extern "C" fn FreeSummaryScreen() {
    unsafe {
        FreeAllWindowBuffers();
        Free(((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn BeginCloseSummaryScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(CloseSummaryScreen));
    }
}
pub(crate) unsafe extern "C" fn CloseSummaryScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32)
            && (!((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0))
        {
            SetMainCallback2(
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
            ((&raw mut gLastViewedMonIndex).cast::<u8>().cast::<u8>()).write(
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16574))
                .read(),
            );
            SummaryScreen_DestroyAnimDelayTask();
            ResetSpriteData();
            FreeAllSpritePalettes();
            StopCryAndClearCrySongs();
            m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 256u16);
            if ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()) as usize) == 0usize {
                DestroyMonSpritesGfxManager(0u8);
            }
            FreeSummaryScreen();
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32)
            && (!((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0))
        {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 64i32)
                != 0
            {
                ChangeSummaryPokemon(taskId, (-1i8));
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 128i32)
                    != 0
                {
                    ChangeSummaryPokemon(taskId, 1i8);
                } else {
                    if (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 32i32)
                        != 0)
                        || (((GetLRKeysPressed()) as i32) == 1i32)
                    {
                        ChangePage(taskId, (-1i8));
                    } else {
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 16i32)
                            != 0)
                            || (((GetLRKeysPressed()) as i32) == 2i32)
                        {
                            ChangePage(taskId, 1i8);
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 1i32)
                                != 0
                            {
                                if ((((((&raw mut sMonSummaryScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(16576))
                                .read()) as i32)
                                    != 1i32
                                {
                                    if ((((((&raw mut sMonSummaryScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(16576))
                                    .read()) as i32)
                                        == 0i32
                                    {
                                        StopPokemonAnimations();
                                        PlaySE(5u16);
                                        BeginCloseSummaryScreen(taskId);
                                    } else {
                                        PlaySE(5u16);
                                        SwitchToMoveSelection(taskId);
                                    }
                                }
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(46)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 2i32)
                                    != 0
                                {
                                    StopPokemonAnimations();
                                    PlaySE(5u16);
                                    BeginCloseSummaryScreen(taskId);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ChangeSummaryPokemon(taskId: u8, delta: i8) {
    unsafe {
        let mut taskId = taskId;
        let mut delta = delta;
        let mut monId: i8 = 0i8;
        if !((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16579))
        .read())
            != 0)
        {
            if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16573))
            .read()) as i32)
                == 1i32
            {
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16576))
                .read()) as i32)
                    != 0i32
                {
                    if ((delta) as i32) == 1i32 {
                        delta = 0i8;
                    } else {
                        delta = 2i8;
                    }
                } else {
                    if ((delta) as i32) == 1i32 {
                        delta = 1i8;
                    } else {
                        delta = 3i8;
                    }
                }
                monId = ((AdvanceStorageMonIndex(
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read(),
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16574))
                    .read(),
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16575))
                    .read(),
                    ((delta) as u8),
                )) as i8);
            } else {
                if ((IsMultiBattle()) as i32) == 1i32 {
                    monId = AdvanceMultiBattleMonIndex(delta);
                } else {
                    monId = AdvanceMonIndex(delta);
                }
            }
            if ((monId) as i32) != (-1i32) {
                PlaySE(5u16);
                if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(7))
                .read()) as i32)
                    != 0i32
                {
                    SetSpriteInvisibility(2u8, 1u8);
                    ClearWindowTilemap(13u8);
                    ScheduleBgCopyTilemapToVram(0u8);
                    PositionStatusSlidingWindow(0u16, 2i16);
                }
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16574))
                .write(((monId) as u8));
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ChangeSummaryMon));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ChangeSummaryMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
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
                || __sw1 == 12i32;
            if __sw1 == 0i32 {
                StopCryAndClearCrySongs();
                break 'l1;
            }
            if __sw1 == 1i32 {
                SummaryScreen_DestroyAnimDelayTask();
                DestroySpriteAndFreeResources(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16595))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                DestroySpriteAndFreeResources(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16595))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                CopyMonToSummaryStruct(
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12),
                );
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16624)
                    .cast::<i16>())
                .write(0i16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((ExtractMonDataToSummaryStruct(
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12),
                )) as i32)
                    == 0i32
                {
                    return;
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                RemoveAndCreateMonMarkingsSprite(
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12),
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                CreateCaughtBallSprite(
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12),
                );
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(7))
                .read()) as i32)
                    != 0i32
                {
                    PositionStatusSlidingWindow(10u16, (-2i16));
                }
                DrawPokerusCuredSymbol(
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12),
                );
                ((data).wrapping_offset(1)).write(0i16);
                break 'l1;
            }
            if __sw1 == 8i32 {
                (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16595))
                .cast::<u8>())
                .write(LoadMonGfxAndSprite(
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12),
                    (data).wrapping_offset(1),
                ));
                if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16595))
                .cast::<u8>())
                .read()) as i32)
                    == 255i32
                {
                    return;
                }
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16595))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(1i16);
                TryDrawExperienceProgressBar();
                ((data).wrapping_offset(1)).write(0i16);
                break 'l1;
            }
            if __sw1 == 9i32 {
                SetTypeIcons();
                break 'l1;
            }
            if __sw1 == 10i32 {
                PrintMonInfo();
                break 'l1;
            }
            if __sw1 == 11i32 {
                PrintPageSpecificText(
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16576))
                    .read(),
                );
                LimitEggSummaryPageDisplay();
                break 'l1;
            }
            if __sw1 == 12i32 {
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16595))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(0i16);
                break 'l1;
            }
            if !__matched {
                if (!((MenuHelpers_ShouldWaitForLinkRecv()) != 0))
                    && (!((FuncIsActiveTask(Some(Task_SlideStatusWindow))) != 0))
                {
                    (data).write(0i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_HandleInput));
                }
                return;
            }
        }
        (data).write(((data).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn AdvanceMonIndex(delta: i8) -> i8 {
    unsafe {
        let mut delta = delta;
        let mut mon: *mut u8 = ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .cast::<*mut u8>())
        .read();
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16576))
        .read()) as i32)
            == 0i32
        {
            if (((delta) as i32) == (-1i32))
                && (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16574))
                .read()) as i32)
                    == 0i32)
            {
                return (-1i8);
            } else {
                if (((delta) as i32) == 1i32)
                    && (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16574))
                    .read()) as i32)
                        >= ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16575))
                        .read()) as i32))
                {
                    return (-1i8);
                } else {
                    return ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(16574))
                    .read()) as i32)
                        .wrapping_add(((delta) as i32))) as i8);
                }
            }
        } else {
            let mut index: i8 = ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(16574))
            .read()) as i8);
            'l1: loop {
                'l2: {
                    index = ((((index) as i32).wrapping_add(((delta) as i32))) as i8);
                    if (((index) as i32) < 0i32)
                        || (((index) as i32)
                            > ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16575))
                            .read()) as i32))
                    {
                        return (-1i8);
                    }
                }
                if !((GetMonData2(
                    (mon).wrapping_offset(((index) as i32) as isize * 100),
                    45i32,
                )) != 0)
                {
                    break 'l1;
                }
            }
            return index;
        }
        #[allow(unreachable_code)]
        {
            return 0i8;
        }
    }
}
pub(crate) unsafe extern "C" fn AdvanceMultiBattleMonIndex(delta: i8) -> i8 {
    unsafe {
        let mut delta = delta;
        let mut mons: *mut u8 = ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .cast::<*mut u8>())
        .read();
        let mut index: i8 = 0i8;
        let mut arrId: i8 = 0i8;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sMultiBattleOrder)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<i8>())
                    .cast::<i8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16574))
                        .read()) as i32)
                    {
                        arrId = ((i) as i8);
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        'l3: loop {
            if !((1i32) != 0) {
                break 'l3;
            }
            let mut order: *mut i8 = ((&raw const sMultiBattleOrder)
                .cast::<u8>()
                .cast_mut()
                .cast::<i8>())
            .cast::<i8>();
            arrId = ((((arrId) as i32).wrapping_add(((delta) as i32))) as i8);
            if (((arrId) as i32) < 0i32) || (((arrId) as i32) >= 6i32) {
                return (-1i8);
            }
            index = ((order).wrapping_offset(((arrId) as i32) as isize)).read();
            if ((IsValidToViewInMulti((mons).wrapping_offset(((index) as i32) as isize * 100)))
                as i32)
                == 1i32
            {
                return index;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i8;
        }
    }
}
pub(crate) unsafe extern "C" fn IsValidToViewInMulti(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        if GetMonData2(mon, 11i32) == 0u32 {
            return 0u8;
        } else {
            if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16574))
            .read()) as i32)
                != 0i32)
                || (!((GetMonData2(mon, 45i32)) != 0))
            {
                return 1u8;
            } else {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn ChangePage(taskId: u8, delta: i8) {
    unsafe {
        let mut taskId = taskId;
        let mut delta = delta;
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((summary).wrapping_add(4)).read()) != 0 {
            return;
        } else {
            if (((delta) as i32) == (-1i32))
                && (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16576))
                .read()) as i32)
                    == ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16577))
                    .read()) as i32))
            {
                return;
            } else {
                if (((delta) as i32) == 1i32)
                    && (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16576))
                    .read()) as i32)
                        == ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16578))
                        .read()) as i32))
                {
                    return;
                }
            }
        }
        PlaySE(5u16);
        ClearPageWindowTilemaps(
            ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16576))
            .read(),
        );
        let __p1 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16576);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(((delta) as i32))) as u8));
        (data).write(0i16);
        if ((delta) as i32) == 1i32 {
            SetTaskFuncWithFollowupFunc(
                taskId,
                Some(PssScrollRight),
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read(),
            );
        } else {
            SetTaskFuncWithFollowupFunc(
                taskId,
                Some(PssScrollLeft),
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read(),
            );
        }
        CreateTextPrinterTask(
            ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16576))
            .read(),
        );
        HidePageSpecificSprites();
    }
}
pub(crate) unsafe extern "C" fn PssScrollRight(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((data).read()) as i32) == 0i32 {
            if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16585))
            .read()) as i32)
                == 0i32
            {
                ((data).wrapping_offset(1)).write(1i16);
                SetBgAttribute(1u8, 7u8, 1u8);
                SetBgAttribute(2u8, 7u8, 2u8);
                ScheduleBgCopyTilemapToVram(1u8);
            } else {
                ((data).wrapping_offset(1)).write(2i16);
                SetBgAttribute(2u8, 7u8, 1u8);
                SetBgAttribute(1u8, 7u8, 2u8);
                ScheduleBgCopyTilemapToVram(2u8);
            }
            ChangeBgX(((((data).wrapping_offset(1)).read()) as u8), 0i32, 0u8);
            SetBgTilemapBuffer(
                ((((data).wrapping_offset(1)).read()) as u8),
                ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(188))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16576))
                    .read()) as i32) as isize
                        * 4096,
                ))
                .cast::<u8>())
                .cast::<u16>())
                .cast::<u8>(),
            );
            ShowBg(1u8);
            ShowBg(2u8);
        }
        ChangeBgX(((((data).wrapping_offset(1)).read()) as u8), 8192i32, 1u8);
        (data).write((((((data).read()) as i32).wrapping_add(32i32)) as i16));
        if (((data).read()) as i32) > 255i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(PssScrollRightEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn PssScrollRightEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let __p1 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16585);
        (__p1).write((((((__p1).read()) as i32) ^ 1i32) as u8));
        ((data).wrapping_offset(1)).write(0i16);
        (data).write(0i16);
        DrawPagination();
        PutPageWindowTilemaps(
            ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16576))
            .read(),
        );
        SetTypeIcons();
        TryDrawExperienceProgressBar();
        SwitchTaskToFollowupFunc(taskId);
    }
}
pub(crate) unsafe extern "C" fn PssScrollLeft(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((data).read()) as i32) == 0i32 {
            if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16585))
            .read()) as i32)
                == 0i32
            {
                ((data).wrapping_offset(1)).write(2i16);
            } else {
                ((data).wrapping_offset(1)).write(1i16);
            }
            ChangeBgX(((((data).wrapping_offset(1)).read()) as u8), 65536i32, 0u8);
        }
        ChangeBgX(((((data).wrapping_offset(1)).read()) as u8), 8192i32, 2u8);
        (data).write((((((data).read()) as i32).wrapping_add(32i32)) as i16));
        if (((data).read()) as i32) > 255i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(PssScrollLeftEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn PssScrollLeftEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16585))
        .read()) as i32)
            == 0i32
        {
            SetBgAttribute(1u8, 7u8, 1u8);
            SetBgAttribute(2u8, 7u8, 2u8);
            ScheduleBgCopyTilemapToVram(2u8);
        } else {
            SetBgAttribute(2u8, 7u8, 1u8);
            SetBgAttribute(1u8, 7u8, 2u8);
            ScheduleBgCopyTilemapToVram(1u8);
        }
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16576))
        .read()) as i32)
            > 1i32
        {
            SetBgTilemapBuffer(
                ((((data).wrapping_offset(1)).read()) as u8),
                ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(188))
                .cast::<u8>())
                .wrapping_offset(
                    (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16576))
                    .read()) as i32)
                        .wrapping_sub(1i32)) as isize
                        * 4096,
                ))
                .cast::<u8>())
                .cast::<u16>())
                .cast::<u8>(),
            );
            ChangeBgX(((((data).wrapping_offset(1)).read()) as u8), 65536i32, 0u8);
        }
        ShowBg(1u8);
        ShowBg(2u8);
        let __p1 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16585);
        (__p1).write((((((__p1).read()) as i32) ^ 1i32) as u8));
        ((data).wrapping_offset(1)).write(0i16);
        (data).write(0i16);
        DrawPagination();
        PutPageWindowTilemaps(
            ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16576))
            .read(),
        );
        SetTypeIcons();
        TryDrawExperienceProgressBar();
        SwitchTaskToFollowupFunc(taskId);
    }
}
pub(crate) unsafe extern "C" fn TryDrawExperienceProgressBar() {
    unsafe {
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16576))
        .read()) as i32)
            == 1i32
        {
            DrawExperienceProgressBar(
                (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SwitchToMoveSelection(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut r#move: u16 = 0u16;
        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16582))
        .write(0u8);
        r#move = (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(112))
        .wrapping_add(20))
        .cast::<u16>())
        .wrapping_offset(
            ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16582))
            .read()) as i32) as isize,
        ))
        .read();
        ClearWindowTilemap(19u8);
        if !((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16595))
                .cast::<u8>())
                .wrapping_offset(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            false,
        ) as u16)
            != 0)
        {
            ClearWindowTilemap(13u8);
        }
        PositionPowerAccSlidingWindow(9u16, (-3i16));
        PositionAppealJamSlidingWindow(9u16, (-3i16), r#move);
        if !((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16584))
        .read())
            != 0)
        {
            ClearWindowTilemap(5u8);
            PutWindowTilemap(6u8);
        }
        TilemapFiveMovesDisplay(
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .wrapping_offset(8192))
            .cast::<u8>())
            .cast::<u16>(),
            3u16,
            0u8,
        );
        TilemapFiveMovesDisplay(
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .wrapping_offset(12288))
            .cast::<u8>())
            .cast::<u16>(),
            1u16,
            0u8,
        );
        PrintMoveDetails(r#move);
        PrintNewMoveDetailsOrCancelText();
        SetNewMoveTypeIcon();
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        CreateMoveSelectorSprites(8u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleInput_MoveSelect));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleInput_MoveSelect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32 {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 64i32)
                != 0
            {
                (data).write(4i16);
                ChangeSelectedMove(
                    data,
                    (-1i8),
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16582),
                );
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 128i32)
                    != 0
                {
                    (data).write(4i16);
                    ChangeSelectedMove(
                        data,
                        1i8,
                        (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16582),
                    );
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16584))
                        .read()) as i32)
                            == 1i32)
                            || ((((((((&raw mut sMonSummaryScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(16580)
                            .cast::<u16>())
                            .read()) as i32)
                                == 0i32)
                                && (((((((&raw mut sMonSummaryScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(16582))
                                .read()) as i32)
                                    == 4i32))
                        {
                            PlaySE(5u16);
                            CloseMoveSelectMode(taskId);
                        } else {
                            if ((HasMoreThanOneMove()) as i32) == 1i32 {
                                PlaySE(5u16);
                                SwitchToMovePositionSwitchMode(taskId);
                            } else {
                                PlaySE(32u16);
                            }
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
                            CloseMoveSelectMode(taskId);
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HasMoreThanOneMove() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(112))
                    .wrapping_add(20))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ChangeSelectedMove(
    taskData: *mut i16,
    direction: i8,
    moveIndexPtr: *mut u8,
) {
    unsafe {
        let mut taskData = taskData;
        let mut direction = direction;
        let mut moveIndexPtr = moveIndexPtr;
        let mut i: i8 = 0i8;
        let mut newMoveIndex: i8 = 0i8;
        let mut r#move: u16 = 0u16;
        PlaySE(5u16);
        newMoveIndex = (((moveIndexPtr).read()) as i8);
        {
            i = 0i8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    newMoveIndex =
                        ((((newMoveIndex) as i32).wrapping_add(((direction) as i32))) as i8);
                    if ((newMoveIndex) as i32) > (((taskData).read()) as i32) {
                        newMoveIndex = 0i8;
                    } else {
                        if ((newMoveIndex) as i32) < 0i32 {
                            newMoveIndex = (((taskData).read()) as i8);
                        }
                    }
                    if ((newMoveIndex) as i32) == 4i32 {
                        r#move = ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16580)
                        .cast::<u16>())
                        .read();
                        break 'l1;
                    }
                    r#move =
                        (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(112))
                        .wrapping_add(20))
                        .cast::<u16>())
                        .wrapping_offset(((newMoveIndex) as i32) as isize))
                        .read();
                    if ((r#move) as i32) != 0i32 {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        DrawContestMoveHearts(r#move);
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        PrintMoveDetails(r#move);
        if (((((moveIndexPtr).read()) as i32) == 4i32)
            && (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16580)
                .cast::<u16>())
            .read()) as i32)
                == 0i32))
            || (((((taskData).wrapping_offset(1)).read()) as i32) == 1i32)
        {
            ClearWindowTilemap(19u8);
            if !((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16595))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                false,
            ) as u16)
                != 0)
            {
                ClearWindowTilemap(13u8);
            }
            ScheduleBgCopyTilemapToVram(0u8);
            PositionPowerAccSlidingWindow(9u16, (-3i16));
            PositionAppealJamSlidingWindow(9u16, (-3i16), r#move);
        }
        if (((((moveIndexPtr).read()) as i32) != 4i32) && (((newMoveIndex) as i32) == 4i32))
            && (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16580)
                .cast::<u16>())
            .read()) as i32)
                == 0i32)
        {
            ClearWindowTilemap(14u8);
            ClearWindowTilemap(15u8);
            ScheduleBgCopyTilemapToVram(0u8);
            PositionPowerAccSlidingWindow(0u16, 3i16);
            PositionAppealJamSlidingWindow(0u16, 3i16, 0u16);
        }
        (moveIndexPtr).write(((newMoveIndex) as u8));
        if ((moveIndexPtr) as usize)
            == (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16582)) as usize)
        {
            KeepMoveSelectorVisible(8u8);
        } else {
            KeepMoveSelectorVisible(18u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CloseMoveSelectMode(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyMoveSelectorSprites(8u8);
        ClearWindowTilemap(6u8);
        PutWindowTilemap(5u8);
        PrintMoveDetails(0u16);
        TilemapFiveMovesDisplay(
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .wrapping_offset(8192))
            .cast::<u8>())
            .cast::<u16>(),
            3u16,
            1u8,
        );
        TilemapFiveMovesDisplay(
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .wrapping_offset(12288))
            .cast::<u8>())
            .cast::<u16>(),
            1u16,
            1u8,
        );
        AddAndFillMoveNamesWindow();
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16582))
        .read()) as i32)
            != 4i32
        {
            ClearWindowTilemap(14u8);
            ClearWindowTilemap(15u8);
            PositionPowerAccSlidingWindow(0u16, 3i16);
            PositionAppealJamSlidingWindow(0u16, 3i16, 0u16);
        }
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleInput));
    }
}
pub(crate) unsafe extern "C" fn SwitchToMovePositionSwitchMode(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16583))
        .write(
            ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16582))
            .read(),
        );
        SetMainMoveSelectorColor(1u8);
        CreateMoveSelectorSprites(18u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleInput_MovePositionSwitch));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleInput_MovePositionSwitch(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32 {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 64i32)
                != 0
            {
                (data).write(3i16);
                ChangeSelectedMove(
                    data,
                    (-1i8),
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16583),
                );
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 128i32)
                    != 0
                {
                    (data).write(3i16);
                    ChangeSelectedMove(
                        data,
                        1i8,
                        (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16583),
                    );
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16582))
                        .read()) as i32)
                            == ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16583))
                            .read()) as i32)
                        {
                            ExitMovePositionSwitchMode(taskId, 0u8);
                        } else {
                            ExitMovePositionSwitchMode(taskId, 1u8);
                        }
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 2i32)
                            != 0
                        {
                            ExitMovePositionSwitchMode(taskId, 0u8);
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ExitMovePositionSwitchMode(taskId: u8, swapMoves: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut swapMoves = swapMoves;
        let mut r#move: u16 = 0u16;
        PlaySE(5u16);
        SetMainMoveSelectorColor(0u8);
        DestroyMoveSelectorSprites(18u8);
        if ((swapMoves) as i32) == 1i32 {
            if !((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16573))
            .read())
                != 0)
            {
                let mut mon: *mut u8 =
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read();
                SwapMonMoves(
                    (mon).wrapping_offset(
                        ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16574))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16582))
                    .read(),
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16583))
                    .read(),
                );
            } else {
                let mut boxMon: *mut u8 =
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read();
                SwapBoxMonMoves(
                    (boxMon).wrapping_offset(
                        ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16574))
                        .read()) as i32) as isize
                            * 80,
                    ),
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16582))
                    .read(),
                    ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16583))
                    .read(),
                );
            }
            CopyMonToSummaryStruct(
                (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12),
            );
            SwapMovesNamesPP(
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16582))
                .read(),
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16583))
                .read(),
            );
            SwapMovesTypeSprites(
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16582))
                .read(),
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16583))
                .read(),
            );
            ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16582))
            .write(
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16583))
                .read(),
            );
        }
        r#move = (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(112))
        .wrapping_add(20))
        .cast::<u16>())
        .wrapping_offset(
            ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16582))
            .read()) as i32) as isize,
        ))
        .read();
        PrintMoveDetails(r#move);
        DrawContestMoveHearts(r#move);
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleInput_MoveSelect));
    }
}
pub(crate) unsafe extern "C" fn SwapMonMoves(mon: *mut u8, moveIndex1: u8, moveIndex2: u8) {
    unsafe {
        let mut mon = mon;
        let mut moveIndex1 = moveIndex1;
        let mut moveIndex2 = moveIndex2;
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        let mut move1: u16 = ((((summary).wrapping_add(20)).cast::<u16>())
            .wrapping_offset(((moveIndex1) as i32) as isize))
        .read();
        let mut move2: u16 = ((((summary).wrapping_add(20)).cast::<u16>())
            .wrapping_offset(((moveIndex2) as i32) as isize))
        .read();
        let mut move1pp: u8 = ((((summary).wrapping_add(28)).cast::<u8>())
            .wrapping_offset(((moveIndex1) as i32) as isize))
        .read();
        let mut move2pp: u8 = ((((summary).wrapping_add(28)).cast::<u8>())
            .wrapping_offset(((moveIndex2) as i32) as isize))
        .read();
        let mut ppBonuses: u8 = ((summary).wrapping_add(52)).read();
        let mut ppUpMask1: u8 = (((&raw mut gPPUpGetMask).cast::<u8>())
            .wrapping_offset(((moveIndex1) as i32) as isize))
        .read();
        let mut ppBonusMove1: u8 = ((crate::c::shr_i32(
            (((ppBonuses) as i32) & ((ppUpMask1) as i32)),
            ((((moveIndex1) as i32).wrapping_mul(2i32)) as u32),
        )) as u8);
        let mut ppUpMask2: u8 = (((&raw mut gPPUpGetMask).cast::<u8>())
            .wrapping_offset(((moveIndex2) as i32) as isize))
        .read();
        let mut ppBonusMove2: u8 = ((crate::c::shr_i32(
            (((ppBonuses) as i32) & ((ppUpMask2) as i32)),
            ((((moveIndex2) as i32).wrapping_mul(2i32)) as u32),
        )) as u8);
        ppBonuses = ((((ppBonuses) as i32) & !((ppUpMask1) as i32)) as u8);
        ppBonuses = ((((ppBonuses) as i32) & !((ppUpMask2) as i32)) as u8);
        ppBonuses = ((((ppBonuses) as i32)
            | (crate::c::shl_i32(
                ((ppBonusMove1) as i32),
                ((((moveIndex2) as i32).wrapping_mul(2i32)) as u32),
            ))
            .wrapping_add(crate::c::shl_i32(
                ((ppBonusMove2) as i32),
                ((((moveIndex1) as i32).wrapping_mul(2i32)) as u32),
            ))) as u8);
        SetMonData(
            mon,
            (13i32).wrapping_add(((moveIndex1) as i32)),
            (&raw mut move2).cast::<u8>(),
        );
        SetMonData(
            mon,
            (13i32).wrapping_add(((moveIndex2) as i32)),
            (&raw mut move1).cast::<u8>(),
        );
        SetMonData(
            mon,
            (17i32).wrapping_add(((moveIndex1) as i32)),
            &raw mut move2pp,
        );
        SetMonData(
            mon,
            (17i32).wrapping_add(((moveIndex2) as i32)),
            &raw mut move1pp,
        );
        SetMonData(mon, 21i32, &raw mut ppBonuses);
        ((((summary).wrapping_add(20)).cast::<u16>())
            .wrapping_offset(((moveIndex1) as i32) as isize))
        .write(move2);
        ((((summary).wrapping_add(20)).cast::<u16>())
            .wrapping_offset(((moveIndex2) as i32) as isize))
        .write(move1);
        ((((summary).wrapping_add(28)).cast::<u8>())
            .wrapping_offset(((moveIndex1) as i32) as isize))
        .write(move2pp);
        ((((summary).wrapping_add(28)).cast::<u8>())
            .wrapping_offset(((moveIndex2) as i32) as isize))
        .write(move1pp);
        ((summary).wrapping_add(52)).write(ppBonuses);
    }
}
pub(crate) unsafe extern "C" fn SwapBoxMonMoves(mon: *mut u8, moveIndex1: u8, moveIndex2: u8) {
    unsafe {
        let mut mon = mon;
        let mut moveIndex1 = moveIndex1;
        let mut moveIndex2 = moveIndex2;
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        let mut move1: u16 = ((((summary).wrapping_add(20)).cast::<u16>())
            .wrapping_offset(((moveIndex1) as i32) as isize))
        .read();
        let mut move2: u16 = ((((summary).wrapping_add(20)).cast::<u16>())
            .wrapping_offset(((moveIndex2) as i32) as isize))
        .read();
        let mut move1pp: u8 = ((((summary).wrapping_add(28)).cast::<u8>())
            .wrapping_offset(((moveIndex1) as i32) as isize))
        .read();
        let mut move2pp: u8 = ((((summary).wrapping_add(28)).cast::<u8>())
            .wrapping_offset(((moveIndex2) as i32) as isize))
        .read();
        let mut ppBonuses: u8 = ((summary).wrapping_add(52)).read();
        let mut ppUpMask1: u8 = (((&raw mut gPPUpGetMask).cast::<u8>())
            .wrapping_offset(((moveIndex1) as i32) as isize))
        .read();
        let mut ppBonusMove1: u8 = ((crate::c::shr_i32(
            (((ppBonuses) as i32) & ((ppUpMask1) as i32)),
            ((((moveIndex1) as i32).wrapping_mul(2i32)) as u32),
        )) as u8);
        let mut ppUpMask2: u8 = (((&raw mut gPPUpGetMask).cast::<u8>())
            .wrapping_offset(((moveIndex2) as i32) as isize))
        .read();
        let mut ppBonusMove2: u8 = ((crate::c::shr_i32(
            (((ppBonuses) as i32) & ((ppUpMask2) as i32)),
            ((((moveIndex2) as i32).wrapping_mul(2i32)) as u32),
        )) as u8);
        ppBonuses = ((((ppBonuses) as i32) & !((ppUpMask1) as i32)) as u8);
        ppBonuses = ((((ppBonuses) as i32) & !((ppUpMask2) as i32)) as u8);
        ppBonuses = ((((ppBonuses) as i32)
            | (crate::c::shl_i32(
                ((ppBonusMove1) as i32),
                ((((moveIndex2) as i32).wrapping_mul(2i32)) as u32),
            ))
            .wrapping_add(crate::c::shl_i32(
                ((ppBonusMove2) as i32),
                ((((moveIndex1) as i32).wrapping_mul(2i32)) as u32),
            ))) as u8);
        SetBoxMonData(
            mon,
            (13i32).wrapping_add(((moveIndex1) as i32)),
            (&raw mut move2).cast::<u8>(),
        );
        SetBoxMonData(
            mon,
            (13i32).wrapping_add(((moveIndex2) as i32)),
            (&raw mut move1).cast::<u8>(),
        );
        SetBoxMonData(
            mon,
            (17i32).wrapping_add(((moveIndex1) as i32)),
            &raw mut move2pp,
        );
        SetBoxMonData(
            mon,
            (17i32).wrapping_add(((moveIndex2) as i32)),
            &raw mut move1pp,
        );
        SetBoxMonData(mon, 21i32, &raw mut ppBonuses);
        ((((summary).wrapping_add(20)).cast::<u16>())
            .wrapping_offset(((moveIndex1) as i32) as isize))
        .write(move2);
        ((((summary).wrapping_add(20)).cast::<u16>())
            .wrapping_offset(((moveIndex2) as i32) as isize))
        .write(move1);
        ((((summary).wrapping_add(28)).cast::<u8>())
            .wrapping_offset(((moveIndex1) as i32) as isize))
        .write(move2pp);
        ((((summary).wrapping_add(28)).cast::<u8>())
            .wrapping_offset(((moveIndex2) as i32) as isize))
        .write(move1pp);
        ((summary).wrapping_add(52)).write(ppBonuses);
    }
}
pub(crate) unsafe extern "C" fn Task_SetHandleReplaceMoveInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetNewMoveTypeIcon();
        CreateMoveSelectorSprites(8u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleReplaceMoveInput));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleReplaceMoveInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32 {
            if ((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16) as i32)
                != 1i32
            {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    (data).write(4i16);
                    ChangeSelectedMove(
                        data,
                        (-1i8),
                        (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16582),
                    );
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        (data).write(4i16);
                        ChangeSelectedMove(
                            data,
                            1i8,
                            (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(16582),
                        );
                    } else {
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 32i32)
                            != 0)
                            || (((GetLRKeysPressed()) as i32) == 1i32)
                        {
                            ChangePage(taskId, (-1i8));
                        } else {
                            if (((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 16i32)
                                != 0)
                                || (((GetLRKeysPressed()) as i32) == 2i32)
                            {
                                ChangePage(taskId, 1i8);
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(46)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 1i32)
                                    != 0
                                {
                                    if ((CanReplaceMove()) as i32) == 1i32 {
                                        StopPokemonAnimations();
                                        PlaySE(5u16);
                                        ((&raw mut sMoveSlotToReplace).cast::<u8>().cast::<u8>())
                                            .write(
                                                ((((&raw mut sMonSummaryScreen)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(16582))
                                                .read(),
                                            );
                                        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(
                                            ((((&raw mut sMoveSlotToReplace)
                                                .cast::<u8>()
                                                .cast::<u8>())
                                            .read())
                                                as u16),
                                        );
                                        BeginCloseSummaryScreen(taskId);
                                    } else {
                                        PlaySE(32u16);
                                        ShowCantForgetHMsWindow(taskId);
                                    }
                                } else {
                                    if ((((((&raw mut gMain).cast::<u8>())
                                        .wrapping_add(46)
                                        .cast::<u16>())
                                    .read()) as i32)
                                        & 2i32)
                                        != 0
                                    {
                                        StopPokemonAnimations();
                                        PlaySE(5u16);
                                        ((&raw mut sMoveSlotToReplace).cast::<u8>().cast::<u8>())
                                            .write(4u8);
                                        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(4u16);
                                        BeginCloseSummaryScreen(taskId);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CanReplaceMove() -> u8 {
    unsafe {
        if ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16582))
        .read()) as i32)
            == 4i32)
            || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16580)
                .cast::<u16>())
            .read()) as i32)
                == 0i32))
            || (((IsMoveHm(
                (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(20))
                .cast::<u16>())
                .wrapping_offset(
                    ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16582))
                    .read()) as i32) as isize,
                ))
                .read(),
            )) as i32)
                != 1i32)
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
pub(crate) unsafe extern "C" fn ShowCantForgetHMsWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearWindowTilemap(14u8);
        ClearWindowTilemap(15u8);
        ScheduleBgCopyTilemapToVram(0u8);
        PositionPowerAccSlidingWindow(0u16, 3i16);
        PositionAppealJamSlidingWindow(0u16, 3i16, 0u16);
        PrintHMMovesCantBeForgotten();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleInputCantForgetHMsMoves));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleInputCantForgetHMsMoves(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut r#move: u16 = 0u16;
        if ((FuncIsActiveTask(Some(Task_SlidePowerAccWindow))) as i32) != 1i32 {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 64i32)
                != 0
            {
                ((data).wrapping_offset(1)).write(1i16);
                (data).write(4i16);
                ChangeSelectedMove(
                    data,
                    (-1i8),
                    (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16582),
                );
                ((data).wrapping_offset(1)).write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_HandleReplaceMoveInput));
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 128i32)
                    != 0
                {
                    ((data).wrapping_offset(1)).write(1i16);
                    (data).write(4i16);
                    ChangeSelectedMove(
                        data,
                        1i8,
                        (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16582),
                    );
                    ((data).wrapping_offset(1)).write(0i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_HandleReplaceMoveInput));
                } else {
                    if (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 32i32)
                        != 0)
                        || (((GetLRKeysPressed()) as i32) == 1i32)
                    {
                        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16576))
                        .read()) as i32)
                            != 2i32
                        {
                            ClearWindowTilemap(19u8);
                            if !((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sMonSummaryScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(16595))
                                    .cast::<u8>())
                                    .wrapping_offset(2))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(62),
                                2,
                                1,
                                false,
                            ) as u16)
                                != 0)
                            {
                                ClearWindowTilemap(13u8);
                            }
                            r#move = (((((((&raw mut sMonSummaryScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(112))
                            .wrapping_add(20))
                            .cast::<u16>())
                            .wrapping_offset(
                                ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(16582))
                                .read()) as i32) as isize,
                            ))
                            .read();
                            ((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .write(Some(Task_HandleReplaceMoveInput));
                            ChangePage(taskId, (-1i8));
                            PositionPowerAccSlidingWindow(9u16, (-2i16));
                            PositionAppealJamSlidingWindow(9u16, (-2i16), r#move);
                        }
                    } else {
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 16i32)
                            != 0)
                            || (((GetLRKeysPressed()) as i32) == 2i32)
                        {
                            if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16576))
                            .read()) as i32)
                                != 3i32
                            {
                                ClearWindowTilemap(19u8);
                                if !((crate::c::bf_read(
                                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut sMonSummaryScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(16595))
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(62),
                                    2,
                                    1,
                                    false,
                                ) as u16)
                                    != 0)
                                {
                                    ClearWindowTilemap(13u8);
                                }
                                r#move = (((((((&raw mut sMonSummaryScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(112))
                                .wrapping_add(20))
                                .cast::<u16>())
                                .wrapping_offset(
                                    ((((((&raw mut sMonSummaryScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(16582))
                                    .read()) as i32) as isize,
                                ))
                                .read();
                                ((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                                .write(Some(Task_HandleReplaceMoveInput));
                                ChangePage(taskId, 1i8);
                                PositionPowerAccSlidingWindow(9u16, (-2i16));
                                PositionAppealJamSlidingWindow(9u16, (-2i16), r#move);
                            }
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 3i32)
                                != 0
                            {
                                ClearWindowTilemap(19u8);
                                if !((crate::c::bf_read(
                                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut sMonSummaryScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(16595))
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(62),
                                    2,
                                    1,
                                    false,
                                ) as u16)
                                    != 0)
                                {
                                    ClearWindowTilemap(13u8);
                                }
                                r#move = (((((((&raw mut sMonSummaryScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(112))
                                .wrapping_add(20))
                                .cast::<u16>())
                                .wrapping_offset(
                                    ((((((&raw mut sMonSummaryScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(16582))
                                    .read()) as i32) as isize,
                                ))
                                .read();
                                PrintMoveDetails(r#move);
                                ScheduleBgCopyTilemapToVram(0u8);
                                PositionPowerAccSlidingWindow(9u16, (-3i16));
                                PositionAppealJamSlidingWindow(9u16, (-3i16), r#move);
                                ((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                                .write(Some(Task_HandleReplaceMoveInput));
                            }
                        }
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMoveSlotToReplace() -> u8 {
    unsafe {
        return ((&raw mut sMoveSlotToReplace).cast::<u8>().cast::<u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn DrawPagination() {
    unsafe {
        let mut tilemap: *mut u16 = (Alloc(32u32)).cast::<u16>();
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut j: u8 = ((((i) as i32).wrapping_mul(2i32)) as u8);
                    if ((i) as i32)
                        < ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16577))
                        .read()) as i32)
                    {
                        ((tilemap).wrapping_offset((((j) as i32).wrapping_add(0i32)) as isize))
                            .write(64u16);
                        ((tilemap).wrapping_offset((((j) as i32).wrapping_add(1i32)) as isize))
                            .write(64u16);
                        ((tilemap).wrapping_offset((((j) as i32).wrapping_add(8i32)) as isize))
                            .write(80u16);
                        ((tilemap).wrapping_offset(
                            ((((j) as i32).wrapping_add(8i32)).wrapping_add(1i32)) as isize,
                        ))
                        .write(80u16);
                    } else {
                        if ((i) as i32)
                            > ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16578))
                            .read()) as i32)
                        {
                            ((tilemap).wrapping_offset((((j) as i32).wrapping_add(0i32)) as isize))
                                .write(74u16);
                            ((tilemap).wrapping_offset((((j) as i32).wrapping_add(1i32)) as isize))
                                .write(74u16);
                            ((tilemap).wrapping_offset((((j) as i32).wrapping_add(8i32)) as isize))
                                .write(90u16);
                            ((tilemap).wrapping_offset(
                                ((((j) as i32).wrapping_add(8i32)).wrapping_add(1i32)) as isize,
                            ))
                            .write(90u16);
                        } else {
                            if ((i) as i32)
                                < ((((((&raw mut sMonSummaryScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(16576))
                                .read()) as i32)
                            {
                                ((tilemap)
                                    .wrapping_offset((((j) as i32).wrapping_add(0i32)) as isize))
                                .write(70u16);
                                ((tilemap)
                                    .wrapping_offset((((j) as i32).wrapping_add(1i32)) as isize))
                                .write(71u16);
                                ((tilemap)
                                    .wrapping_offset((((j) as i32).wrapping_add(8i32)) as isize))
                                .write(86u16);
                                ((tilemap).wrapping_offset(
                                    ((((j) as i32).wrapping_add(8i32)).wrapping_add(1i32)) as isize,
                                ))
                                .write(87u16);
                            } else {
                                if ((i) as i32)
                                    == ((((((&raw mut sMonSummaryScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(16576))
                                    .read()) as i32)
                                {
                                    if ((i) as i32)
                                        != ((((((&raw mut sMonSummaryScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(16578))
                                        .read()) as i32)
                                    {
                                        ((tilemap).wrapping_offset(
                                            (((j) as i32).wrapping_add(0i32)) as isize,
                                        ))
                                        .write(65u16);
                                        ((tilemap).wrapping_offset(
                                            (((j) as i32).wrapping_add(1i32)) as isize,
                                        ))
                                        .write(66u16);
                                        ((tilemap).wrapping_offset(
                                            (((j) as i32).wrapping_add(8i32)) as isize,
                                        ))
                                        .write(81u16);
                                        ((tilemap).wrapping_offset(
                                            ((((j) as i32).wrapping_add(8i32)).wrapping_add(1i32))
                                                as isize,
                                        ))
                                        .write(82u16);
                                    } else {
                                        ((tilemap).wrapping_offset(
                                            (((j) as i32).wrapping_add(0i32)) as isize,
                                        ))
                                        .write(75u16);
                                        ((tilemap).wrapping_offset(
                                            (((j) as i32).wrapping_add(1i32)) as isize,
                                        ))
                                        .write(76u16);
                                        ((tilemap).wrapping_offset(
                                            (((j) as i32).wrapping_add(8i32)) as isize,
                                        ))
                                        .write(91u16);
                                        ((tilemap).wrapping_offset(
                                            ((((j) as i32).wrapping_add(8i32)).wrapping_add(1i32))
                                                as isize,
                                        ))
                                        .write(92u16);
                                    }
                                } else {
                                    if ((i) as i32)
                                        != ((((((&raw mut sMonSummaryScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(16578))
                                        .read()) as i32)
                                    {
                                        ((tilemap).wrapping_offset(
                                            (((j) as i32).wrapping_add(0i32)) as isize,
                                        ))
                                        .write(67u16);
                                        ((tilemap).wrapping_offset(
                                            (((j) as i32).wrapping_add(1i32)) as isize,
                                        ))
                                        .write(68u16);
                                        ((tilemap).wrapping_offset(
                                            (((j) as i32).wrapping_add(8i32)) as isize,
                                        ))
                                        .write(83u16);
                                        ((tilemap).wrapping_offset(
                                            ((((j) as i32).wrapping_add(8i32)).wrapping_add(1i32))
                                                as isize,
                                        ))
                                        .write(84u16);
                                    } else {
                                        ((tilemap).wrapping_offset(
                                            (((j) as i32).wrapping_add(0i32)) as isize,
                                        ))
                                        .write(72u16);
                                        ((tilemap).wrapping_offset(
                                            (((j) as i32).wrapping_add(1i32)) as isize,
                                        ))
                                        .write(73u16);
                                        ((tilemap).wrapping_offset(
                                            (((j) as i32).wrapping_add(8i32)) as isize,
                                        ))
                                        .write(88u16);
                                        ((tilemap).wrapping_offset(
                                            ((((j) as i32).wrapping_add(8i32)).wrapping_add(1i32))
                                                as isize,
                                        ))
                                        .write(89u16);
                                    }
                                }
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyToBgTilemapBufferRect_ChangePalette(
            3u8,
            (tilemap).cast::<u8>(),
            11u8,
            0u8,
            8u8,
            2u8,
            16u8,
        );
        ScheduleBgCopyTilemapToVram(3u8);
        Free((tilemap).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn CopyNColumnsToTilemap(
    slidingWindow: *mut u8,
    tilemapDest: *mut u16,
    visibleColumns: u8,
    isOpeningToTheLeft: u8,
) {
    unsafe {
        let mut slidingWindow = slidingWindow;
        let mut tilemapDest = tilemapDest;
        let mut visibleColumns = visibleColumns;
        let mut isOpeningToTheLeft = isOpeningToTheLeft;
        let mut i: u16 = 0u16;
        let mut alloced: *mut u16 = (Alloc(
            (((((((slidingWindow).wrapping_add(6)).read()) as i32).wrapping_mul(2i32))
                .wrapping_mul(((((slidingWindow).wrapping_add(7)).read()) as i32)))
                as u32),
        ))
        .cast::<u16>();
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp)
                        .write_volatile(((slidingWindow).wrapping_add(4).cast::<u16>()).read());
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (alloced).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(
                                        (((((slidingWindow).wrapping_add(6)).read()) as i32)
                                            .wrapping_mul(2i32))
                                        .wrapping_mul(
                                            ((((slidingWindow).wrapping_add(7)).read()) as i32),
                                        ),
                                        crate::c::div_i32(16i32, 8i32),
                                    ) & 2097151i32)) as u32),
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
        if ((((slidingWindow).wrapping_add(6)).read()) as i32) != ((visibleColumns) as i32) {
            if !((isOpeningToTheLeft) != 0) {
                {
                    i = 0u16;
                    'l5: loop {
                        if !(((i) as i32) < ((((slidingWindow).wrapping_add(7)).read()) as i32)) {
                            break 'l5;
                        }
                        'l6: {
                            'l7: loop {
                                'l8: {
                                    'l9: loop {
                                        'l10: {
                                            CpuSet(
                                                ((((slidingWindow).cast::<*mut u16>()).read())
                                                    .wrapping_offset(
                                                        (((visibleColumns) as i32).wrapping_add(
                                                            ((((slidingWindow).wrapping_add(6))
                                                                .read())
                                                                as i32)
                                                                .wrapping_mul(((i) as i32)),
                                                        ))
                                                            as isize,
                                                    ))
                                                .cast::<u8>(),
                                                ((alloced).wrapping_offset(
                                                    (((((slidingWindow).wrapping_add(6)).read())
                                                        as i32)
                                                        .wrapping_mul(((i) as i32)))
                                                        as isize,
                                                ))
                                                .cast::<u8>(),
                                                ((0i32 | (crate::c::div_i32(
                                                    (((((slidingWindow).wrapping_add(6)).read())
                                                        as i32)
                                                        .wrapping_sub(((visibleColumns) as i32)))
                                                    .wrapping_mul(2i32),
                                                    crate::c::div_i32(16i32, 8i32),
                                                ) & 2097151i32))
                                                    as u32),
                                            );
                                        }
                                        if !((0i32) != 0) {
                                            break 'l9;
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l7;
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                {
                    i = 0u16;
                    'l11: loop {
                        if !(((i) as i32) < ((((slidingWindow).wrapping_add(7)).read()) as i32)) {
                            break 'l11;
                        }
                        'l12: {
                            'l13: loop {
                                'l14: {
                                    'l15: loop {
                                        'l16: {
                                            CpuSet(
                                                ((((slidingWindow).cast::<*mut u16>()).read())
                                                    .wrapping_offset(
                                                    (((((slidingWindow).wrapping_add(6)).read())
                                                        as i32)
                                                        .wrapping_mul(((i) as i32)))
                                                        as isize,
                                                ))
                                                .cast::<u8>(),
                                                ((alloced).wrapping_offset(
                                                    (((visibleColumns) as i32).wrapping_add(
                                                        ((((slidingWindow).wrapping_add(6)).read())
                                                            as i32)
                                                            .wrapping_mul(((i) as i32)),
                                                    ))
                                                        as isize,
                                                ))
                                                .cast::<u8>(),
                                                ((0i32 | (crate::c::div_i32(
                                                    (((((slidingWindow).wrapping_add(6)).read())
                                                        as i32)
                                                        .wrapping_sub(((visibleColumns) as i32)))
                                                    .wrapping_mul(2i32),
                                                    crate::c::div_i32(16i32, 8i32),
                                                ) & 2097151i32))
                                                    as u32),
                                            );
                                        }
                                        if !((0i32) != 0) {
                                            break 'l15;
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l13;
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
        {
            i = 0u16;
            'l17: loop {
                if !(((i) as i32) < ((((slidingWindow).wrapping_add(7)).read()) as i32)) {
                    break 'l17;
                }
                'l18: {
                    'l19: loop {
                        'l20: {
                            'l21: loop {
                                'l22: {
                                    CpuSet(
                                        ((alloced).wrapping_offset(
                                            (((((slidingWindow).wrapping_add(6)).read()) as i32)
                                                .wrapping_mul(((i) as i32)))
                                                as isize,
                                        ))
                                        .cast::<u8>(),
                                        ((tilemapDest).wrapping_offset(
                                            (((((((slidingWindow).wrapping_add(9)).read())
                                                as i32)
                                                .wrapping_add(((i) as i32)))
                                            .wrapping_mul(32i32))
                                            .wrapping_add(
                                                ((((slidingWindow).wrapping_add(8)).read()) as i32),
                                            )) as isize,
                                        ))
                                        .cast::<u8>(),
                                        ((0i32
                                            | (crate::c::div_i32(
                                                ((((slidingWindow).wrapping_add(6)).read()) as i32)
                                                    .wrapping_mul(2i32),
                                                crate::c::div_i32(16i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l21;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l19;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        Free((alloced).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn PositionPowerAccSlidingWindow(visibleColumns: u16, speed: i16) {
    unsafe {
        let mut visibleColumns = visibleColumns;
        let mut speed = speed;
        if ((speed) as i32)
            > (((((&raw const sPowerAccSlidingWindow).cast::<u8>().cast_mut()).wrapping_add(6))
                .read()) as i32)
        {
            speed = (((((&raw const sPowerAccSlidingWindow).cast::<u8>().cast_mut())
                .wrapping_add(6))
            .read()) as i16);
        }
        if (((speed) as i32) == 0i32)
            || (((speed) as i32)
                == (((((&raw const sPowerAccSlidingWindow).cast::<u8>().cast_mut())
                    .wrapping_add(6))
                .read()) as i32))
        {
            CopyNColumnsToTilemap(
                (&raw const sPowerAccSlidingWindow).cast::<u8>().cast_mut(),
                (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(188))
                .cast::<u8>())
                .wrapping_offset(8192))
                .cast::<u8>())
                .cast::<u16>(),
                ((speed) as u8),
                1u8,
            );
        } else {
            let mut taskId: u8 = FindTaskIdByFunc(Some(Task_SlidePowerAccWindow));
            if ((taskId) as i32) == 255i32 {
                taskId = CreateTask(Some(Task_SlidePowerAccWindow), 8u8);
            }
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(speed);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((visibleColumns) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SlidePowerAccWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let __p1 = (data).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add((((data).read()) as i32))) as i16));
        if ((((data).wrapping_offset(1)).read()) as i32) < 0i32 {
            ((data).wrapping_offset(1)).write(0i16);
        } else {
            if ((((data).wrapping_offset(1)).read()) as i32)
                > (((((&raw const sPowerAccSlidingWindow).cast::<u8>().cast_mut()).wrapping_add(6))
                    .read()) as i32)
            {
                ((data).wrapping_offset(1)).write(
                    (((((&raw const sPowerAccSlidingWindow).cast::<u8>().cast_mut())
                        .wrapping_add(6))
                    .read()) as i16),
                );
            }
        }
        CopyNColumnsToTilemap(
            (&raw const sPowerAccSlidingWindow).cast::<u8>().cast_mut(),
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .wrapping_offset(8192))
            .cast::<u8>())
            .cast::<u16>(),
            ((((data).wrapping_offset(1)).read()) as u8),
            1u8,
        );
        if (((((data).wrapping_offset(1)).read()) as i32) <= 0i32)
            || (((((data).wrapping_offset(1)).read()) as i32)
                >= (((((&raw const sPowerAccSlidingWindow).cast::<u8>().cast_mut())
                    .wrapping_add(6))
                .read()) as i32))
        {
            if (((data).read()) as i32) < 0i32 {
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16576))
                .read()) as i32)
                    == 2i32
                {
                    PutWindowTilemap(14u8);
                }
            } else {
                if !((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16595))
                        .cast::<u8>())
                        .wrapping_offset(2))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    PutWindowTilemap(13u8);
                }
                PutWindowTilemap(19u8);
            }
            ScheduleBgCopyTilemapToVram(0u8);
            DestroyTask(taskId);
        }
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn PositionAppealJamSlidingWindow(
    visibleColumns: u16,
    speed: i16,
    r#move: u16,
) {
    unsafe {
        let mut visibleColumns = visibleColumns;
        let mut speed = speed;
        let mut r#move = r#move;
        if ((speed) as i32)
            > (((((&raw const sAppealJamSlidingWindow).cast::<u8>().cast_mut()).wrapping_add(6))
                .read()) as i32)
        {
            speed = (((((&raw const sAppealJamSlidingWindow).cast::<u8>().cast_mut())
                .wrapping_add(6))
            .read()) as i16);
        }
        if (((speed) as i32) == 0i32)
            || (((speed) as i32)
                == (((((&raw const sAppealJamSlidingWindow).cast::<u8>().cast_mut())
                    .wrapping_add(6))
                .read()) as i32))
        {
            CopyNColumnsToTilemap(
                (&raw const sAppealJamSlidingWindow).cast::<u8>().cast_mut(),
                (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(188))
                .cast::<u8>())
                .wrapping_offset(12288))
                .cast::<u8>())
                .cast::<u16>(),
                ((speed) as u8),
                1u8,
            );
        } else {
            let mut taskId: u8 = FindTaskIdByFunc(Some(Task_SlideAppealJamWindow));
            if ((taskId) as i32) == 255i32 {
                taskId = CreateTask(Some(Task_SlideAppealJamWindow), 8u8);
            }
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(speed);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((visibleColumns) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((r#move) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SlideAppealJamWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let __p1 = (data).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add((((data).read()) as i32))) as i16));
        if ((((data).wrapping_offset(1)).read()) as i32) < 0i32 {
            ((data).wrapping_offset(1)).write(0i16);
        } else {
            if ((((data).wrapping_offset(1)).read()) as i32)
                > (((((&raw const sAppealJamSlidingWindow).cast::<u8>().cast_mut())
                    .wrapping_add(6))
                .read()) as i32)
            {
                ((data).wrapping_offset(1)).write(
                    (((((&raw const sAppealJamSlidingWindow).cast::<u8>().cast_mut())
                        .wrapping_add(6))
                    .read()) as i16),
                );
            }
        }
        CopyNColumnsToTilemap(
            (&raw const sAppealJamSlidingWindow).cast::<u8>().cast_mut(),
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .wrapping_offset(12288))
            .cast::<u8>())
            .cast::<u16>(),
            ((((data).wrapping_offset(1)).read()) as u8),
            1u8,
        );
        if (((((data).wrapping_offset(1)).read()) as i32) <= 0i32)
            || (((((data).wrapping_offset(1)).read()) as i32)
                >= (((((&raw const sAppealJamSlidingWindow).cast::<u8>().cast_mut())
                    .wrapping_add(6))
                .read()) as i32))
        {
            if (((data).read()) as i32) < 0i32 {
                if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16576))
                .read()) as i32)
                    == 3i32)
                    && (((FuncIsActiveTask(Some(PssScrollRight))) as i32) == 0i32)
                {
                    PutWindowTilemap(15u8);
                }
                DrawContestMoveHearts(((((data).wrapping_offset(2)).read()) as u16));
            } else {
                if !((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16595))
                        .cast::<u8>())
                        .wrapping_offset(2))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    PutWindowTilemap(13u8);
                }
                PutWindowTilemap(19u8);
            }
            ScheduleBgCopyTilemapToVram(0u8);
            DestroyTask(taskId);
        }
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn PositionStatusSlidingWindow(visibleColumns: u16, speed: i16) {
    unsafe {
        let mut visibleColumns = visibleColumns;
        let mut speed = speed;
        if ((speed) as i32)
            > (((((&raw const sStatusSlidingWindow1).cast::<u8>().cast_mut()).wrapping_add(6))
                .read()) as i32)
        {
            speed = (((((&raw const sStatusSlidingWindow1).cast::<u8>().cast_mut())
                .wrapping_add(6))
            .read()) as i16);
        }
        if (((speed) as i32) == 0i32)
            || (((speed) as i32)
                == (((((&raw const sStatusSlidingWindow1).cast::<u8>().cast_mut()).wrapping_add(6))
                    .read()) as i32))
        {
            CopyNColumnsToTilemap(
                (&raw const sStatusSlidingWindow1).cast::<u8>().cast_mut(),
                ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(188))
                .cast::<u8>())
                .cast::<u8>())
                .cast::<u16>(),
                ((speed) as u8),
                0u8,
            );
            CopyNColumnsToTilemap(
                (&raw const sStatusSlidingWindow2).cast::<u8>().cast_mut(),
                ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(188))
                .cast::<u8>())
                .cast::<u8>())
                .cast::<u16>(),
                ((speed) as u8),
                0u8,
            );
        } else {
            let mut taskId: u8 = CreateTask(Some(Task_SlideStatusWindow), 8u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(speed);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((visibleColumns) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SlideStatusWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let __p1 = (data).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add((((data).read()) as i32))) as i16));
        if ((((data).wrapping_offset(1)).read()) as i32) < 0i32 {
            ((data).wrapping_offset(1)).write(0i16);
        } else {
            if ((((data).wrapping_offset(1)).read()) as i32)
                > (((((&raw const sStatusSlidingWindow1).cast::<u8>().cast_mut()).wrapping_add(6))
                    .read()) as i32)
            {
                ((data).wrapping_offset(1)).write(
                    (((((&raw const sStatusSlidingWindow1).cast::<u8>().cast_mut())
                        .wrapping_add(6))
                    .read()) as i16),
                );
            }
        }
        CopyNColumnsToTilemap(
            (&raw const sStatusSlidingWindow1).cast::<u8>().cast_mut(),
            ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .cast::<u8>())
            .cast::<u16>(),
            ((((data).wrapping_offset(1)).read()) as u8),
            0u8,
        );
        CopyNColumnsToTilemap(
            (&raw const sStatusSlidingWindow2).cast::<u8>().cast_mut(),
            ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .cast::<u8>())
            .cast::<u16>(),
            ((((data).wrapping_offset(1)).read()) as u8),
            0u8,
        );
        ScheduleBgCopyTilemapToVram(3u8);
        if (((((data).wrapping_offset(1)).read()) as i32) <= 0i32)
            || (((((data).wrapping_offset(1)).read()) as i32)
                >= (((((&raw const sStatusSlidingWindow1).cast::<u8>().cast_mut()).wrapping_add(6))
                    .read()) as i32))
        {
            if (((data).read()) as i32) < 0i32 {
                CreateSetStatusSprite();
                PutWindowTilemap(13u8);
                ScheduleBgCopyTilemapToVram(0u8);
            }
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn TilemapFiveMovesDisplay(dst: *mut u16, palette: u16, remove: u8) {
    unsafe {
        let mut dst = dst;
        let mut palette = palette;
        let mut remove = remove;
        let mut i: u16 = 0u16;
        let mut id: u16 = 0u16;
        palette = ((((palette) as i32).wrapping_mul(4096i32)) as u16);
        id = 1386u16;
        if !((remove) != 0) {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((dst)
                            .wrapping_offset((((id) as i32).wrapping_add(((i) as i32))) as isize))
                        .write(
                            ((((((((&raw mut gSummaryScreen_MoveEffect_Cancel_Tilemap)
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                .wrapping_add(((palette) as i32)))
                                as u16),
                        );
                        ((dst).wrapping_offset(
                            ((((id) as i32).wrapping_add(((i) as i32))).wrapping_add(32i32))
                                as isize,
                        ))
                        .write(
                            ((((((((&raw mut gSummaryScreen_MoveEffect_Cancel_Tilemap)
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                .wrapping_add(((palette) as i32)))
                                as u16),
                        );
                        ((dst).wrapping_offset(
                            ((((id) as i32).wrapping_add(((i) as i32))).wrapping_add(64i32))
                                as isize,
                        ))
                        .write(
                            ((((((((&raw mut gSummaryScreen_MoveEffect_Cancel_Tilemap)
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset((((i) as i32).wrapping_add(20i32)) as isize))
                            .read()) as i32)
                                .wrapping_add(((palette) as i32)))
                                as u16),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((dst)
                            .wrapping_offset((((id) as i32).wrapping_add(((i) as i32))) as isize))
                        .write(
                            ((((((((&raw mut gSummaryScreen_MoveEffect_Cancel_Tilemap)
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset((((i) as i32).wrapping_add(20i32)) as isize))
                            .read()) as i32)
                                .wrapping_add(((palette) as i32)))
                                as u16),
                        );
                        ((dst).wrapping_offset(
                            ((((id) as i32).wrapping_add(((i) as i32))).wrapping_add(32i32))
                                as isize,
                        ))
                        .write(
                            ((((((((&raw mut gSummaryScreen_MoveEffect_Cancel_Tilemap)
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset((((i) as i32).wrapping_add(40i32)) as isize))
                            .read()) as i32)
                                .wrapping_add(((palette) as i32)))
                                as u16),
                        );
                        ((dst).wrapping_offset(
                            ((((id) as i32).wrapping_add(((i) as i32))).wrapping_add(64i32))
                                as isize,
                        ))
                        .write(
                            ((((((((&raw mut gSummaryScreen_MoveEffect_Cancel_Tilemap)
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset((((i) as i32).wrapping_add(40i32)) as isize))
                            .read()) as i32)
                                .wrapping_add(((palette) as i32)))
                                as u16),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawPokerusCuredSymbol(mon: *mut u8) {
    unsafe {
        let mut mon = mon;
        if (!((CheckPartyPokerus(mon, 0u8)) != 0)) && ((CheckPartyHasHadPokerus(mon, 0u8)) != 0) {
            ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .cast::<u8>())
            .cast::<u16>())
            .wrapping_offset(547))
            .write(44u16);
            (((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u16>())
            .wrapping_offset(547))
            .write(44u16);
        } else {
            ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .cast::<u8>())
            .cast::<u16>())
            .wrapping_offset(547))
            .write(2074u16);
            (((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u16>())
            .wrapping_offset(547))
            .write(2074u16);
        }
        ScheduleBgCopyTilemapToVram(3u8);
    }
}
pub(crate) unsafe extern "C" fn SetMonPicBackgroundPalette(isMonShiny: u8) {
    unsafe {
        let mut isMonShiny = isMonShiny;
        if !((isMonShiny) != 0) {
            SetBgTilemapPalette(3u8, 1u8, 4u8, 8u8, 8u8, 0u8);
        } else {
            SetBgTilemapPalette(3u8, 1u8, 4u8, 8u8, 8u8, 5u8);
        }
        ScheduleBgCopyTilemapToVram(3u8);
    }
}
pub(crate) unsafe extern "C" fn DrawExperienceProgressBar(unused: *mut u8) {
    unsafe {
        let mut unused = unused;
        let mut numExpProgressBarTicks: i64 = 0i64;
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        let mut dst: *mut u16 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        if ((((summary).wrapping_add(5)).read()) as i32) < 100i32 {
            let mut expBetweenLevels: u32 = ((((((&raw mut gExperienceTables).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                        ((((summary).cast::<u16>()).read()) as i32) as isize * 28,
                    ))
                    .wrapping_add(19))
                    .read()) as i32) as isize
                        * 404,
                ))
            .cast::<u32>())
            .wrapping_offset(
                (((((summary).wrapping_add(5)).read()) as i32).wrapping_add(1i32)) as isize,
            ))
            .read())
            .wrapping_sub(
                (((((&raw mut gExperienceTables).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                        ((((summary).cast::<u16>()).read()) as i32) as isize * 28,
                    ))
                    .wrapping_add(19))
                    .read()) as i32) as isize
                        * 404,
                ))
                .cast::<u32>())
                .wrapping_offset(((((summary).wrapping_add(5)).read()) as i32) as isize))
                .read(),
            );
            let mut expSinceLastLevel: u32 = (((summary).wrapping_add(16).cast::<u32>()).read())
                .wrapping_sub(
                    (((((&raw mut gExperienceTables).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                            ((((summary).cast::<u16>()).read()) as i32) as isize * 28,
                        ))
                        .wrapping_add(19))
                        .read()) as i32) as isize
                            * 404,
                    ))
                    .cast::<u32>())
                    .wrapping_offset(((((summary).wrapping_add(5)).read()) as i32) as isize))
                    .read(),
                );
            numExpProgressBarTicks =
                ((crate::c::div_u32((expSinceLastLevel).wrapping_mul(64u32), expBetweenLevels))
                    as i64);
            if (numExpProgressBarTicks == 0i64) && (expSinceLastLevel != 0u32) {
                numExpProgressBarTicks = 1i64;
            }
        } else {
            numExpProgressBarTicks = 0i64;
        }
        dst = (((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(188))
        .cast::<u8>())
        .wrapping_offset(4096))
        .cast::<u8>())
        .wrapping_offset(2048))
        .cast::<u16>())
        .wrapping_offset(597);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if numExpProgressBarTicks > 7i64 {
                        ((dst).wrapping_offset(((i) as i32) as isize)).write(8298u16);
                    } else {
                        ((dst).wrapping_offset(((i) as i32) as isize)).write(
                            (((8290i64)
                                .wrapping_add(crate::c::rem_i64(numExpProgressBarTicks, 8i64)))
                                as u16),
                        );
                    }
                    numExpProgressBarTicks = (numExpProgressBarTicks).wrapping_sub(8i64);
                    if numExpProgressBarTicks < 0i64 {
                        numExpProgressBarTicks = 0i64;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((GetBgTilemapBuffer(1u8)) as usize)
            == (((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u8>())
            .cast::<u16>()) as usize)
        {
            ScheduleBgCopyTilemapToVram(1u8);
        } else {
            ScheduleBgCopyTilemapToVram(2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DrawContestMoveHearts(r#move: u16) {
    unsafe {
        let mut r#move = r#move;
        let mut tilemap: *mut u16 =
            ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(188))
            .cast::<u8>())
            .wrapping_offset(12288))
            .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u16>();
        let mut i: u8 = 0u8;
        if ((r#move) as i32) != 0i32 {
            let mut effectValue: u8 = ((((&raw mut gContestEffects).cast::<u8>())
                .wrapping_offset(
                    (((((&raw mut gContestMoves).cast::<u8>())
                        .wrapping_offset(((r#move) as i32) as isize * 8))
                    .read()) as i32) as isize
                        * 4,
                ))
            .wrapping_add(1))
            .read();
            if ((effectValue) as i32) != 255i32 {
                effectValue = ((crate::c::div_i32(((effectValue) as i32), 10i32)) as u8);
            }
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 8i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((effectValue) as i32) != 255i32)
                            && (((i) as i32) < ((effectValue) as i32))
                        {
                            ((tilemap).wrapping_offset(
                                ((((crate::c::div_i32(((i) as i32), 4i32)).wrapping_mul(32i32))
                                    .wrapping_add((((i) as i32) & 3i32)))
                                .wrapping_add(486i32)) as isize,
                            ))
                            .write(4154u16);
                        } else {
                            ((tilemap).wrapping_offset(
                                ((((crate::c::div_i32(((i) as i32), 4i32)).wrapping_mul(32i32))
                                    .wrapping_add((((i) as i32) & 3i32)))
                                .wrapping_add(486i32)) as isize,
                            ))
                            .write(4153u16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            effectValue = ((((&raw mut gContestEffects).cast::<u8>()).wrapping_offset(
                (((((&raw mut gContestMoves).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 8))
                .read()) as i32) as isize
                    * 4,
            ))
            .wrapping_add(2))
            .read();
            if ((effectValue) as i32) != 255i32 {
                effectValue = ((crate::c::div_i32(((effectValue) as i32), 10i32)) as u8);
            }
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 8i32) {
                        break 'l3;
                    }
                    'l4: {
                        if (((effectValue) as i32) != 255i32)
                            && (((i) as i32) < ((effectValue) as i32))
                        {
                            ((tilemap).wrapping_offset(
                                ((((crate::c::div_i32(((i) as i32), 4i32)).wrapping_mul(32i32))
                                    .wrapping_add((((i) as i32) & 3i32)))
                                .wrapping_add(550i32)) as isize,
                            ))
                            .write(4156u16);
                        } else {
                            ((tilemap).wrapping_offset(
                                ((((crate::c::div_i32(((i) as i32), 4i32)).wrapping_mul(32i32))
                                    .wrapping_add((((i) as i32) & 3i32)))
                                .wrapping_add(550i32)) as isize,
                            ))
                            .write(4157u16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LimitEggSummaryPageDisplay() {
    unsafe {
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(112))
        .wrapping_add(4))
        .read())
            != 0
        {
            ChangeBgX(3u8, 65536i32, 0u8);
        } else {
            ChangeBgX(3u8, 0i32, 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn ResetWindows() {
    unsafe {
        let mut i: u8 = 0u8;
        InitWindows(((&raw const sSummaryTemplate).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    FillWindowPixelBuffer(i, 0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l3;
                }
                'l4: {
                    ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16587))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintTextOnWindow(
    windowId: u8,
    string: *mut u8,
    x: u8,
    y: u8,
    lineSpacing: u8,
    colorId: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut string = string;
        let mut x = x;
        let mut y = y;
        let mut lineSpacing = lineSpacing;
        let mut colorId = colorId;
        AddTextPrinterParameterized4(
            windowId,
            1u8,
            x,
            y,
            0u8,
            lineSpacing,
            ((((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((colorId) as i32) as isize * 3))
            .cast::<u8>(),
            0i8,
            string,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMonInfo() {
    unsafe {
        FillWindowPixelBuffer(17u8, 0u8);
        FillWindowPixelBuffer(18u8, 0u8);
        FillWindowPixelBuffer(19u8, 0u8);
        if !(((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(112))
        .wrapping_add(4))
        .read())
            != 0)
        {
            PrintNotEggInfo();
        } else {
            PrintEggInfo();
        }
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintNotEggInfo() {
    unsafe {
        let mut strArray = crate::ffi::Align4([0u8; 16]);
        let mut mon: *mut u8 =
            (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12);
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        let mut dexNum: u16 = SpeciesToPokedexNum(((summary).cast::<u16>()).read());
        if ((dexNum) as i32) != 65535i32 {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (&raw mut gText_NumberClear01).cast::<u8>(),
            );
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar2).cast::<u8>(),
                ((dexNum) as i32),
                2i32,
                3u8,
            );
            StringAppend(
                (&raw mut gStringVar1).cast::<u8>(),
                (&raw mut gStringVar2).cast::<u8>(),
            );
            if !((IsMonShiny(mon)) != 0) {
                PrintTextOnWindow(
                    17u8,
                    (&raw mut gStringVar1).cast::<u8>(),
                    0u8,
                    1u8,
                    0u8,
                    1u8,
                );
                SetMonPicBackgroundPalette(0u8);
            } else {
                PrintTextOnWindow(
                    17u8,
                    (&raw mut gStringVar1).cast::<u8>(),
                    0u8,
                    1u8,
                    0u8,
                    7u8,
                );
                SetMonPicBackgroundPalette(1u8);
            }
            PutWindowTilemap(17u8);
        } else {
            ClearWindowTilemap(17u8);
            if !((IsMonShiny(mon)) != 0) {
                SetMonPicBackgroundPalette(0u8);
            } else {
                SetMonPicBackgroundPalette(1u8);
            }
        }
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut gText_LevelSymbol).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            ((((summary).wrapping_add(5)).read()) as i32),
            0i32,
            3u8,
        );
        StringAppend(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut gStringVar2).cast::<u8>(),
        );
        PrintTextOnWindow(
            19u8,
            (&raw mut gStringVar1).cast::<u8>(),
            24u8,
            17u8,
            0u8,
            1u8,
        );
        GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
        PrintTextOnWindow(
            18u8,
            (&raw mut gStringVar1).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            1u8,
        );
        ((&raw mut strArray).cast::<u8>()).write(186u8);
        StringCopy(
            ((&raw mut strArray).cast::<u8>()).wrapping_offset(1),
            (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
            ))
            .cast::<u8>(),
        );
        PrintTextOnWindow(19u8, (&raw mut strArray).cast::<u8>(), 0u8, 1u8, 0u8, 1u8);
        PrintGenderSymbol(mon, ((summary).wrapping_add(2).cast::<u16>()).read());
        PutWindowTilemap(18u8);
        PutWindowTilemap(19u8);
    }
}
pub(crate) unsafe extern "C" fn PrintEggInfo() {
    unsafe {
        GetMonNickname(
            (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        PrintTextOnWindow(
            18u8,
            (&raw mut gStringVar1).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            1u8,
        );
        PutWindowTilemap(18u8);
        ClearWindowTilemap(17u8);
        ClearWindowTilemap(19u8);
    }
}
pub(crate) unsafe extern "C" fn PrintGenderSymbol(mon: *mut u8, species: u16) {
    unsafe {
        let mut mon = mon;
        let mut species = species;
        if (((species) as i32) != 32i32) && (((species) as i32) != 29i32) {
            'l1: {
                let __sw1 = ((GetMonGender(mon)) as i32);
                if __sw1 == 0i32 {
                    PrintTextOnWindow(
                        19u8,
                        (&raw mut gText_MaleSymbol).cast::<u8>(),
                        57u8,
                        17u8,
                        0u8,
                        3u8,
                    );
                    break 'l1;
                }
                if __sw1 == 254i32 {
                    PrintTextOnWindow(
                        19u8,
                        (&raw mut gText_FemaleSymbol).cast::<u8>(),
                        57u8,
                        17u8,
                        0u8,
                        4u8,
                    );
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintAOrBButtonIcon(windowId: u8, bButton: u8, x: u32) {
    unsafe {
        let mut windowId = windowId;
        let mut bButton = bButton;
        let mut x = x;
        let mut button: *mut u8 = core::ptr::null_mut();
        if !((bButton) != 0) {
            button =
                (((&raw const sButtons_Gfx).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>();
        } else {
            button = ((((&raw const sButtons_Gfx).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(128))
            .cast::<u8>();
        }
        BlitBitmapToWindow(windowId, button, ((x) as u16), 0u16, 16u16, 16u16);
    }
}
pub(crate) unsafe extern "C" fn PrintPageNamesAndStats() {
    unsafe {
        let mut stringXPos: i32 = 0i32;
        let mut iconXPos: i32 = 0i32;
        let mut statsXPos: i32 = 0i32;
        PrintTextOnWindow(
            0u8,
            (&raw mut gText_PkmnInfo).cast::<u8>(),
            2u8,
            1u8,
            0u8,
            1u8,
        );
        PrintTextOnWindow(
            1u8,
            (&raw mut gText_PkmnSkills).cast::<u8>(),
            2u8,
            1u8,
            0u8,
            1u8,
        );
        PrintTextOnWindow(
            2u8,
            (&raw mut gText_BattleMoves).cast::<u8>(),
            2u8,
            1u8,
            0u8,
            1u8,
        );
        PrintTextOnWindow(
            3u8,
            (&raw mut gText_ContestMoves).cast::<u8>(),
            2u8,
            1u8,
            0u8,
            1u8,
        );
        stringXPos = GetStringRightAlignXOffset(1i32, (&raw mut gText_Cancel2).cast::<u8>(), 62i32);
        iconXPos = (stringXPos).wrapping_sub(16i32);
        if iconXPos < 0i32 {
            iconXPos = 0i32;
        }
        PrintAOrBButtonIcon(4u8, 0u8, ((iconXPos) as u32));
        PrintTextOnWindow(
            4u8,
            (&raw mut gText_Cancel2).cast::<u8>(),
            ((stringXPos) as u8),
            1u8,
            0u8,
            0u8,
        );
        stringXPos = GetStringRightAlignXOffset(1i32, (&raw mut gText_Info).cast::<u8>(), 62i32);
        iconXPos = (stringXPos).wrapping_sub(16i32);
        if iconXPos < 0i32 {
            iconXPos = 0i32;
        }
        PrintAOrBButtonIcon(5u8, 0u8, ((iconXPos) as u32));
        PrintTextOnWindow(
            5u8,
            (&raw mut gText_Info).cast::<u8>(),
            ((stringXPos) as u8),
            1u8,
            0u8,
            0u8,
        );
        stringXPos = GetStringRightAlignXOffset(1i32, (&raw mut gText_Switch).cast::<u8>(), 62i32);
        iconXPos = (stringXPos).wrapping_sub(16i32);
        if iconXPos < 0i32 {
            iconXPos = 0i32;
        }
        PrintAOrBButtonIcon(6u8, 0u8, ((iconXPos) as u32));
        PrintTextOnWindow(
            6u8,
            (&raw mut gText_Switch).cast::<u8>(),
            ((stringXPos) as u8),
            1u8,
            0u8,
            0u8,
        );
        PrintTextOnWindow(
            8u8,
            (&raw mut gText_RentalPkmn).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            1u8,
        );
        PrintTextOnWindow(
            9u8,
            (&raw mut gText_TypeSlash).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            0u8,
        );
        statsXPos = (6i32).wrapping_add(GetStringCenterAlignXOffset(
            1i32,
            (&raw mut gText_HP4).cast::<u8>(),
            42i32,
        ));
        PrintTextOnWindow(
            10u8,
            (&raw mut gText_HP4).cast::<u8>(),
            ((statsXPos) as u8),
            1u8,
            0u8,
            1u8,
        );
        statsXPos = (6i32).wrapping_add(GetStringCenterAlignXOffset(
            1i32,
            (&raw mut gText_Attack3).cast::<u8>(),
            42i32,
        ));
        PrintTextOnWindow(
            10u8,
            (&raw mut gText_Attack3).cast::<u8>(),
            ((statsXPos) as u8),
            17u8,
            0u8,
            1u8,
        );
        statsXPos = (6i32).wrapping_add(GetStringCenterAlignXOffset(
            1i32,
            (&raw mut gText_Defense3).cast::<u8>(),
            42i32,
        ));
        PrintTextOnWindow(
            10u8,
            (&raw mut gText_Defense3).cast::<u8>(),
            ((statsXPos) as u8),
            33u8,
            0u8,
            1u8,
        );
        statsXPos = (2i32).wrapping_add(GetStringCenterAlignXOffset(
            1i32,
            (&raw mut gText_SpAtk4).cast::<u8>(),
            36i32,
        ));
        PrintTextOnWindow(
            11u8,
            (&raw mut gText_SpAtk4).cast::<u8>(),
            ((statsXPos) as u8),
            1u8,
            0u8,
            1u8,
        );
        statsXPos = (2i32).wrapping_add(GetStringCenterAlignXOffset(
            1i32,
            (&raw mut gText_SpDef4).cast::<u8>(),
            36i32,
        ));
        PrintTextOnWindow(
            11u8,
            (&raw mut gText_SpDef4).cast::<u8>(),
            ((statsXPos) as u8),
            17u8,
            0u8,
            1u8,
        );
        statsXPos = (2i32).wrapping_add(GetStringCenterAlignXOffset(
            1i32,
            (&raw mut gText_Speed2).cast::<u8>(),
            36i32,
        ));
        PrintTextOnWindow(
            11u8,
            (&raw mut gText_Speed2).cast::<u8>(),
            ((statsXPos) as u8),
            33u8,
            0u8,
            1u8,
        );
        PrintTextOnWindow(
            12u8,
            (&raw mut gText_ExpPoints).cast::<u8>(),
            6u8,
            1u8,
            0u8,
            1u8,
        );
        PrintTextOnWindow(
            12u8,
            (&raw mut gText_NextLv).cast::<u8>(),
            6u8,
            17u8,
            0u8,
            1u8,
        );
        PrintTextOnWindow(
            13u8,
            (&raw mut gText_Status).cast::<u8>(),
            2u8,
            1u8,
            0u8,
            1u8,
        );
        PrintTextOnWindow(
            14u8,
            (&raw mut gText_Power).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            1u8,
        );
        PrintTextOnWindow(
            14u8,
            (&raw mut gText_Accuracy2).cast::<u8>(),
            0u8,
            17u8,
            0u8,
            1u8,
        );
        PrintTextOnWindow(
            15u8,
            (&raw mut gText_Appeal).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            1u8,
        );
        PrintTextOnWindow(15u8, (&raw mut gText_Jam).cast::<u8>(), 0u8, 17u8, 0u8, 1u8);
    }
}
pub(crate) unsafe extern "C" fn PutPageWindowTilemaps(page: u8) {
    unsafe {
        let mut page = page;
        let mut i: u8 = 0u8;
        ClearWindowTilemap(0u8);
        ClearWindowTilemap(1u8);
        ClearWindowTilemap(2u8);
        ClearWindowTilemap(3u8);
        'l1: {
            let __sw1 = ((page) as i32);
            if __sw1 == 0i32 {
                PutWindowTilemap(0u8);
                PutWindowTilemap(4u8);
                if (((InBattleFactory()) as i32) == 1i32)
                    || (((InSlateportBattleTent()) as i32) == 1i32)
                {
                    PutWindowTilemap(8u8);
                }
                PutWindowTilemap(9u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                PutWindowTilemap(1u8);
                PutWindowTilemap(10u8);
                PutWindowTilemap(11u8);
                PutWindowTilemap(12u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                PutWindowTilemap(2u8);
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16572))
                .read()) as i32)
                    == 3i32
                {
                    if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16580)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32)
                        || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16582))
                        .read()) as i32)
                            != 4i32)
                    {
                        PutWindowTilemap(14u8);
                    }
                } else {
                    PutWindowTilemap(5u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                PutWindowTilemap(3u8);
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16572))
                .read()) as i32)
                    == 3i32
                {
                    if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16580)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32)
                        || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16582))
                        .read()) as i32)
                            != 4i32)
                    {
                        PutWindowTilemap(15u8);
                    }
                } else {
                    PutWindowTilemap(5u8);
                }
                break 'l1;
            }
        }
        {
            i = 0u8;
            'l2: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l2;
                }
                'l3: {
                    PutWindowTilemap(
                        ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16587))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn ClearPageWindowTilemaps(page: u8) {
    unsafe {
        let mut page = page;
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((page) as i32);
            if __sw1 == 0i32 {
                ClearWindowTilemap(4u8);
                if (((InBattleFactory()) as i32) == 1i32)
                    || (((InSlateportBattleTent()) as i32) == 1i32)
                {
                    ClearWindowTilemap(8u8);
                }
                ClearWindowTilemap(9u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ClearWindowTilemap(10u8);
                ClearWindowTilemap(11u8);
                ClearWindowTilemap(12u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16572))
                .read()) as i32)
                    == 3i32
                {
                    if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16580)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32)
                        || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16582))
                        .read()) as i32)
                            != 4i32)
                    {
                        ClearWindowTilemap(14u8);
                    }
                } else {
                    ClearWindowTilemap(5u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16572))
                .read()) as i32)
                    == 3i32
                {
                    if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16580)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32)
                        || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16582))
                        .read()) as i32)
                            != 4i32)
                    {
                        ClearWindowTilemap(15u8);
                    }
                } else {
                    ClearWindowTilemap(5u8);
                }
                break 'l1;
            }
        }
        {
            i = 0u8;
            'l2: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l2;
                }
                'l3: {
                    RemoveWindowByIndex(i);
                }
                i = (i).wrapping_add(1);
            }
        }
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn AddWindowFromTemplateList(template: *mut u8, templateId: u8) -> u8 {
    unsafe {
        let mut template = template;
        let mut templateId = templateId;
        let mut windowIdPtr: *mut u8 =
            (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16587))
            .cast::<u8>())
            .wrapping_offset(((templateId) as i32) as isize);
        if (((windowIdPtr).read()) as i32) == 255i32 {
            (windowIdPtr).write(
                ((AddWindow((template).wrapping_offset(((templateId) as i32) as isize * 8))) as u8),
            );
            FillWindowPixelBuffer((windowIdPtr).read(), 0u8);
        }
        return (windowIdPtr).read();
    }
}
pub(crate) unsafe extern "C" fn RemoveWindowByIndex(windowIndex: u8) {
    unsafe {
        let mut windowIndex = windowIndex;
        let mut windowIdPtr: *mut u8 =
            (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16587))
            .cast::<u8>())
            .wrapping_offset(((windowIndex) as i32) as isize);
        if (((windowIdPtr).read()) as i32) != 255i32 {
            ClearWindowTilemap((windowIdPtr).read());
            RemoveWindow((windowIdPtr).read());
            (windowIdPtr).write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn PrintPageSpecificText(pageIndex: u8) {
    unsafe {
        let mut pageIndex = pageIndex;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(16587))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 255i32
                    {
                        FillWindowPixelBuffer(
                            ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16587))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                            0u8,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((&raw const sTextPrinterFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(((pageIndex) as i32) as isize))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn CreateTextPrinterTask(pageIndex: u8) {
    unsafe {
        let mut pageIndex = pageIndex;
        CreateTask(
            ((((&raw const sTextPrinterTasks)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .wrapping_offset(((pageIndex) as i32) as isize))
            .read(),
            16u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintInfoPageText() {
    unsafe {
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(112))
        .wrapping_add(4))
        .read())
            != 0
        {
            PrintEggOTName();
            PrintEggOTID();
            PrintEggState();
            PrintEggMemo();
        } else {
            PrintMonOTName();
            PrintMonOTID();
            PrintMonAbilityName();
            PrintMonAbilityDescription();
            BufferMonTrainerMemo();
            PrintMonTrainerMemo();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintInfoPage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 1i32 {
                PrintMonOTName();
                break 'l1;
            }
            if __sw1 == 2i32 {
                PrintMonOTID();
                break 'l1;
            }
            if __sw1 == 3i32 {
                PrintMonAbilityName();
                break 'l1;
            }
            if __sw1 == 4i32 {
                PrintMonAbilityDescription();
                break 'l1;
            }
            if __sw1 == 5i32 {
                BufferMonTrainerMemo();
                break 'l1;
            }
            if __sw1 == 6i32 {
                PrintMonTrainerMemo();
                break 'l1;
            }
            if __sw1 == 7i32 {
                DestroyTask(taskId);
                return;
            }
        }
        (data).write(((data).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn PrintMonOTName() {
    unsafe {
        let mut x: i32 = 0i32;
        let mut windowId: i32 = 0i32;
        if (((InBattleFactory()) as i32) != 1i32) && (((InSlateportBattleTent()) as i32) != 1i32) {
            windowId = ((AddWindowFromTemplateList(
                ((&raw const sPageInfoTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                0u8,
            )) as i32);
            PrintTextOnWindow(
                ((windowId) as u8),
                (&raw mut gText_OTSlash).cast::<u8>(),
                0u8,
                1u8,
                0u8,
                1u8,
            );
            x = GetStringWidth(1u8, (&raw mut gText_OTSlash).cast::<u8>(), 0i16);
            if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(50))
            .read()) as i32)
                == 0i32
            {
                PrintTextOnWindow(
                    ((windowId) as u8),
                    (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(54))
                    .cast::<u8>(),
                    ((x) as u8),
                    1u8,
                    0u8,
                    5u8,
                );
            } else {
                PrintTextOnWindow(
                    ((windowId) as u8),
                    (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(54))
                    .cast::<u8>(),
                    ((x) as u8),
                    1u8,
                    0u8,
                    6u8,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintMonOTID() {
    unsafe {
        let mut xPos: i32 = 0i32;
        if (((InBattleFactory()) as i32) != 1i32) && (((InSlateportBattleTent()) as i32) != 1i32) {
            ConvertIntToDecimalStringN(
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (&raw mut gText_IDNumber2).cast::<u8>(),
                ),
                ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(72)
                .cast::<u32>())
                .read()) as u16) as i32),
                2i32,
                5u8,
            );
            xPos = GetStringRightAlignXOffset(1i32, (&raw mut gStringVar1).cast::<u8>(), 56i32);
            PrintTextOnWindow(
                AddWindowFromTemplateList(
                    ((&raw const sPageInfoTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                    1u8,
                ),
                (&raw mut gStringVar1).cast::<u8>(),
                ((xPos) as u8),
                1u8,
                0u8,
                1u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintMonAbilityName() {
    unsafe {
        let mut ability: u8 = GetAbilityBySpecies(
            (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .cast::<u16>())
            .read(),
            (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(8))
            .read(),
        );
        PrintTextOnWindow(
            AddWindowFromTemplateList(
                ((&raw const sPageInfoTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                2u8,
            ),
            (((&raw mut gAbilityNames).cast::<u8>())
                .wrapping_offset(((ability) as i32) as isize * 13))
            .cast::<u8>(),
            0u8,
            1u8,
            0u8,
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMonAbilityDescription() {
    unsafe {
        let mut ability: u8 = GetAbilityBySpecies(
            (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .cast::<u16>())
            .read(),
            (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(8))
            .read(),
        );
        PrintTextOnWindow(
            AddWindowFromTemplateList(
                ((&raw const sPageInfoTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                2u8,
            ),
            ((((&raw mut gAbilityDescriptionPointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(((ability) as i32) as isize))
            .read(),
            0u8,
            17u8,
            0u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn BufferMonTrainerMemo() {
    unsafe {
        let mut sum: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        let mut text: *mut u8 = core::ptr::null_mut();
        DynamicPlaceholderTextUtil_Reset();
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            0u8,
            ((&raw const sMemoNatureTextColor).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            1u8,
            ((&raw const sMemoMiscTextColor).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        BufferNatureString();
        if ((((InBattleFactory()) as i32) == 1i32) || (((InSlateportBattleTent()) as i32) == 1i32))
            || (((IsInGamePartnerMon()) as i32) == 1i32)
        {
            DynamicPlaceholderTextUtil_ExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_XNature).cast::<u8>(),
            );
        } else {
            let mut metLevelString: *mut u8 = Alloc(32u32);
            let mut metLocationString: *mut u8 = Alloc(32u32);
            GetMetLevelString(metLevelString);
            if ((((sum).wrapping_add(9)).read()) as i32) < 213i32 {
                GetMapNameHandleAquaHideout(
                    metLocationString,
                    ((((sum).wrapping_add(9)).read()) as u16),
                );
                DynamicPlaceholderTextUtil_SetPlaceholderPtr(4u8, metLocationString);
            }
            if ((DoesMonOTMatchOwner()) as i32) == 1i32 {
                if ((((sum).wrapping_add(10)).read()) as i32) == 0i32 {
                    text = (if ((((sum).wrapping_add(9)).read()) as i32) >= 213i32 {
                        (&raw mut gText_XNatureHatchedSomewhereAt).cast::<u8>()
                    } else {
                        (&raw mut gText_XNatureHatchedAtYZ).cast::<u8>()
                    });
                } else {
                    text = (if ((((sum).wrapping_add(9)).read()) as i32) >= 213i32 {
                        (&raw mut gText_XNatureMetSomewhereAt).cast::<u8>()
                    } else {
                        (&raw mut gText_XNatureMetAtYZ).cast::<u8>()
                    });
                }
            } else {
                if ((((sum).wrapping_add(9)).read()) as i32) == 255i32 {
                    text = (&raw mut gText_XNatureFatefulEncounter).cast::<u8>();
                } else {
                    if (((((sum).wrapping_add(9)).read()) as i32) != 254i32)
                        && ((DidMonComeFromGBAGames()) != 0)
                    {
                        text = (if ((((sum).wrapping_add(9)).read()) as i32) >= 213i32 {
                            (&raw mut gText_XNatureObtainedInTrade).cast::<u8>()
                        } else {
                            (&raw mut gText_XNatureProbablyMetAt).cast::<u8>()
                        });
                    } else {
                        text = (&raw mut gText_XNatureObtainedInTrade).cast::<u8>();
                    }
                }
            }
            DynamicPlaceholderTextUtil_ExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                text,
            );
            Free(metLevelString);
            Free(metLocationString);
        }
    }
}
pub(crate) unsafe extern "C" fn PrintMonTrainerMemo() {
    unsafe {
        PrintTextOnWindow(
            AddWindowFromTemplateList(
                ((&raw const sPageInfoTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                3u8,
            ),
            (&raw mut gStringVar4).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn BufferNatureString() {
    unsafe {
        let mut sumStruct: *mut u8 =
            ((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read();
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            2u8,
            ((((&raw const gNatureNamePointers)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                (((((sumStruct).wrapping_add(112)).wrapping_add(51)).read()) as i32) as isize,
            ))
            .read(),
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            5u8,
            (&raw mut gText_EmptyString5).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetMetLevelString(output: *mut u8) {
    unsafe {
        let mut output = output;
        let mut level: u8 = (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112))
        .wrapping_add(10))
        .read();
        if ((level) as i32) == 0i32 {
            level = 5u8;
        }
        ConvertIntToDecimalStringN(output, ((level) as i32), 0i32, 3u8);
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(3u8, output);
    }
}
pub(crate) unsafe extern "C" fn DoesMonOTMatchOwner() -> u8 {
    unsafe {
        let mut sum: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        let mut trainerId: u32 = 0u32;
        let mut gender: u8 = 0u8;
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read()) as usize)
            == (((&raw mut gEnemyParty).cast::<u8>()) as usize)
        {
            let mut multiID: u8 = ((((GetMultiplayerId()) as i32) ^ 1i32) as u8);
            trainerId = (((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((multiID) as i32) as isize * 28))
            .wrapping_add(4)
            .cast::<u32>())
            .read()
                & 65535u32);
            gender = ((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((multiID) as i32) as isize * 28))
            .wrapping_add(19))
            .read();
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_offset(((multiID) as i32) as isize * 28))
                .wrapping_add(8))
                .cast::<u8>(),
            );
        } else {
            trainerId = (GetPlayerIDAsU32() & 65535u32);
            gender =
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read();
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
        }
        if ((((gender) as i32) != ((((sum).wrapping_add(50)).read()) as i32))
            || (trainerId != (((sum).wrapping_add(72).cast::<u32>()).read() & 65535u32)))
            || ((StringCompareWithoutExtCtrlCodes(
                (&raw mut gStringVar1).cast::<u8>(),
                ((sum).wrapping_add(54)).cast::<u8>(),
            )) != 0)
        {
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
pub(crate) unsafe extern "C" fn DidMonComeFromGBAGames() -> u8 {
    unsafe {
        let mut sum: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        if (((((sum).wrapping_add(11)).read()) as i32) > 0i32)
            && (((((sum).wrapping_add(11)).read()) as i32) <= 5i32)
        {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DidMonComeFromRSE() -> u8 {
    unsafe {
        let mut sum: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        if (((((sum).wrapping_add(11)).read()) as i32) > 0i32)
            && (((((sum).wrapping_add(11)).read()) as i32) <= 3i32)
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IsInGamePartnerMon() -> u8 {
    unsafe {
        if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4194304u32) != 0)
            && ((crate::c::bf_read(
                ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                1,
                1,
                false,
            ) as u8)
                != 0)
        {
            if ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16574))
            .read()) as i32)
                == 1i32)
                || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16574))
                .read()) as i32)
                    == 4i32))
                || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16574))
                .read()) as i32)
                    == 5i32)
            {
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PrintEggOTName() {
    unsafe {
        let mut windowId: u32 = ((AddWindowFromTemplateList(
            ((&raw const sPageInfoTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
            0u8,
        )) as u32);
        let mut width: u32 =
            ((GetStringWidth(1u8, (&raw mut gText_OTSlash).cast::<u8>(), 0i16)) as u32);
        PrintTextOnWindow(
            ((windowId) as u8),
            (&raw mut gText_OTSlash).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            1u8,
        );
        PrintTextOnWindow(
            ((windowId) as u8),
            (&raw mut gText_FiveMarks).cast::<u8>(),
            ((width) as u8),
            1u8,
            0u8,
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintEggOTID() {
    unsafe {
        let mut x: i32 = 0i32;
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut gText_IDNumber2).cast::<u8>(),
        );
        StringAppend(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut gText_FiveMarks).cast::<u8>(),
        );
        x = GetStringRightAlignXOffset(1i32, (&raw mut gStringVar1).cast::<u8>(), 56i32);
        PrintTextOnWindow(
            AddWindowFromTemplateList(
                ((&raw const sPageInfoTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                1u8,
            ),
            (&raw mut gStringVar1).cast::<u8>(),
            ((x) as u8),
            1u8,
            0u8,
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintEggState() {
    unsafe {
        let mut text: *mut u8 = core::ptr::null_mut();
        let mut sum: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(112))
        .wrapping_add(53))
        .read()) as i32)
            == 1i32
        {
            text = (&raw mut gText_EggWillTakeALongTime).cast::<u8>();
        } else {
            if ((((sum).wrapping_add(48).cast::<u16>()).read()) as i32) <= 5i32 {
                text = (&raw mut gText_EggAboutToHatch).cast::<u8>();
            } else {
                if ((((sum).wrapping_add(48).cast::<u16>()).read()) as i32) <= 10i32 {
                    text = (&raw mut gText_EggWillHatchSoon).cast::<u8>();
                } else {
                    if ((((sum).wrapping_add(48).cast::<u16>()).read()) as i32) <= 40i32 {
                        text = (&raw mut gText_EggWillTakeSomeTime).cast::<u8>();
                    } else {
                        text = (&raw mut gText_EggWillTakeALongTime).cast::<u8>();
                    }
                }
            }
        }
        PrintTextOnWindow(
            AddWindowFromTemplateList(
                ((&raw const sPageInfoTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                2u8,
            ),
            text,
            0u8,
            1u8,
            0u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintEggMemo() {
    unsafe {
        let mut text: *mut u8 = core::ptr::null_mut();
        let mut sum: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(112))
        .wrapping_add(53))
        .read()) as i32)
            != 1i32
        {
            if ((((sum).wrapping_add(9)).read()) as i32) == 255i32 {
                text = (&raw mut gText_PeculiarEggNicePlace).cast::<u8>();
            } else {
                if (((DidMonComeFromGBAGames()) as i32) == 0i32)
                    || (((DoesMonOTMatchOwner()) as i32) == 0i32)
                {
                    text = (&raw mut gText_PeculiarEggTrade).cast::<u8>();
                } else {
                    if ((((sum).wrapping_add(9)).read()) as i32) == 253i32 {
                        text = (if ((DidMonComeFromRSE()) as i32) == 1i32 {
                            (&raw mut gText_EggFromHotSprings).cast::<u8>()
                        } else {
                            (&raw mut gText_EggFromTraveler).cast::<u8>()
                        });
                    } else {
                        text = (&raw mut gText_OddEggFoundByCouple).cast::<u8>();
                    }
                }
            }
        } else {
            text = (&raw mut gText_OddEggFoundByCouple).cast::<u8>();
        }
        PrintTextOnWindow(
            AddWindowFromTemplateList(
                ((&raw const sPageInfoTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                3u8,
            ),
            text,
            0u8,
            1u8,
            0u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintSkillsPageText() {
    unsafe {
        PrintHeldItemName();
        PrintRibbonCount();
        BufferLeftColumnStats();
        PrintLeftColumnStats();
        BufferRightColumnStats();
        PrintRightColumnStats();
        PrintExpPointsNextLevel();
    }
}
pub(crate) unsafe extern "C" fn Task_PrintSkillsPage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 1i32 {
                PrintHeldItemName();
                break 'l1;
            }
            if __sw1 == 2i32 {
                PrintRibbonCount();
                break 'l1;
            }
            if __sw1 == 3i32 {
                BufferLeftColumnStats();
                break 'l1;
            }
            if __sw1 == 4i32 {
                PrintLeftColumnStats();
                break 'l1;
            }
            if __sw1 == 5i32 {
                BufferRightColumnStats();
                break 'l1;
            }
            if __sw1 == 6i32 {
                PrintRightColumnStats();
                break 'l1;
            }
            if __sw1 == 7i32 {
                PrintExpPointsNextLevel();
                break 'l1;
            }
            if __sw1 == 8i32 {
                DestroyTask(taskId);
                return;
            }
        }
        (data).write(((data).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn PrintHeldItemName() {
    unsafe {
        let mut text: *mut u8 = core::ptr::null_mut();
        let mut x: i32 = 0i32;
        if (((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(112))
        .wrapping_add(46)
        .cast::<u16>())
        .read()) as i32)
            == 175i32)
            && (((IsMultiBattle()) as i32) == 1i32))
            && (((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16574))
            .read()) as i32)
                == 1i32)
                || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16574))
                .read()) as i32)
                    == 4i32))
                || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16574))
                .read()) as i32)
                    == 5i32))
        {
            text = GetItemName(175u16);
        } else {
            if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(46)
            .cast::<u16>())
            .read()) as i32)
                == 0i32
            {
                text = (&raw mut gText_None).cast::<u8>();
            } else {
                CopyItemName(
                    (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(46)
                    .cast::<u16>())
                    .read(),
                    (&raw mut gStringVar1).cast::<u8>(),
                );
                text = (&raw mut gStringVar1).cast::<u8>();
            }
        }
        x = (GetStringCenterAlignXOffset(1i32, text, 72i32)).wrapping_add(6i32);
        PrintTextOnWindow(
            AddWindowFromTemplateList(
                ((&raw const sPageSkillsTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                0u8,
            ),
            text,
            ((x) as u8),
            1u8,
            0u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintRibbonCount() {
    unsafe {
        let mut text: *mut u8 = core::ptr::null_mut();
        let mut x: i32 = 0i32;
        if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(112))
        .wrapping_add(6))
        .read()) as i32)
            == 0i32
        {
            text = (&raw mut gText_None).cast::<u8>();
        } else {
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(6))
                .read()) as i32),
                1i32,
                2u8,
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_RibbonsVar1).cast::<u8>(),
            );
            text = (&raw mut gStringVar4).cast::<u8>();
        }
        x = (GetStringCenterAlignXOffset(1i32, text, 70i32)).wrapping_add(6i32);
        PrintTextOnWindow(
            AddWindowFromTemplateList(
                ((&raw const sPageSkillsTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                1u8,
            ),
            text,
            ((x) as u8),
            1u8,
            0u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn BufferLeftColumnStats() {
    unsafe {
        let mut currentHPString: *mut u8 = Alloc(8u32);
        let mut maxHPString: *mut u8 = Alloc(8u32);
        let mut attackString: *mut u8 = Alloc(8u32);
        let mut defenseString: *mut u8 = Alloc(8u32);
        ConvertIntToDecimalStringN(
            currentHPString,
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(32)
            .cast::<u16>())
            .read()) as i32),
            1i32,
            3u8,
        );
        ConvertIntToDecimalStringN(
            maxHPString,
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(34)
            .cast::<u16>())
            .read()) as i32),
            1i32,
            3u8,
        );
        ConvertIntToDecimalStringN(
            attackString,
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(36)
            .cast::<u16>())
            .read()) as i32),
            1i32,
            7u8,
        );
        ConvertIntToDecimalStringN(
            defenseString,
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(38)
            .cast::<u16>())
            .read()) as i32),
            1i32,
            7u8,
        );
        DynamicPlaceholderTextUtil_Reset();
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(0u8, currentHPString);
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(1u8, maxHPString);
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(2u8, attackString);
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(3u8, defenseString);
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            ((&raw const sStatsLeftColumnLayout).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        Free(currentHPString);
        Free(maxHPString);
        Free(attackString);
        Free(defenseString);
    }
}
pub(crate) unsafe extern "C" fn PrintLeftColumnStats() {
    unsafe {
        PrintTextOnWindow(
            AddWindowFromTemplateList(
                ((&raw const sPageSkillsTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                2u8,
            ),
            (&raw mut gStringVar4).cast::<u8>(),
            4u8,
            1u8,
            0u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn BufferRightColumnStats() {
    unsafe {
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(40)
            .cast::<u16>())
            .read()) as i32),
            1i32,
            3u8,
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(42)
            .cast::<u16>())
            .read()) as i32),
            1i32,
            3u8,
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar3).cast::<u8>(),
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(44)
            .cast::<u16>())
            .read()) as i32),
            1i32,
            3u8,
        );
        DynamicPlaceholderTextUtil_Reset();
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(0u8, (&raw mut gStringVar1).cast::<u8>());
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(1u8, (&raw mut gStringVar2).cast::<u8>());
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(2u8, (&raw mut gStringVar3).cast::<u8>());
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            ((&raw const sStatsRightColumnLayout).cast::<u8>().cast_mut()).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintRightColumnStats() {
    unsafe {
        PrintTextOnWindow(
            AddWindowFromTemplateList(
                ((&raw const sPageSkillsTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                3u8,
            ),
            (&raw mut gStringVar4).cast::<u8>(),
            2u8,
            1u8,
            0u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintExpPointsNextLevel() {
    unsafe {
        let mut sum: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        let mut windowId: u8 = AddWindowFromTemplateList(
            ((&raw const sPageSkillsTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
            4u8,
        );
        let mut x: i32 = 0i32;
        let mut expToNextLevel: u32 = 0u32;
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((sum).wrapping_add(16).cast::<u32>()).read()) as i32),
            1i32,
            7u8,
        );
        x = (GetStringRightAlignXOffset(1i32, (&raw mut gStringVar1).cast::<u8>(), 42i32))
            .wrapping_add(2i32);
        PrintTextOnWindow(
            windowId,
            (&raw mut gStringVar1).cast::<u8>(),
            ((x) as u8),
            1u8,
            0u8,
            0u8,
        );
        if ((((sum).wrapping_add(5)).read()) as i32) < 100i32 {
            expToNextLevel = ((((((&raw mut gExperienceTables).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gSpeciesInfo).cast::<u8>())
                    .wrapping_offset(((((sum).cast::<u16>()).read()) as i32) as isize * 28))
                .wrapping_add(19))
                .read()) as i32) as isize
                    * 404,
            ))
            .cast::<u32>())
            .wrapping_offset(
                (((((sum).wrapping_add(5)).read()) as i32).wrapping_add(1i32)) as isize,
            ))
            .read())
            .wrapping_sub(((sum).wrapping_add(16).cast::<u32>()).read());
        } else {
            expToNextLevel = 0u32;
        }
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((expToNextLevel) as i32),
            1i32,
            6u8,
        );
        x = (GetStringRightAlignXOffset(1i32, (&raw mut gStringVar1).cast::<u8>(), 42i32))
            .wrapping_add(2i32);
        PrintTextOnWindow(
            windowId,
            (&raw mut gStringVar1).cast::<u8>(),
            ((x) as u8),
            17u8,
            0u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintBattleMoves() {
    unsafe {
        PrintMoveNameAndPP(0u8);
        PrintMoveNameAndPP(1u8);
        PrintMoveNameAndPP(2u8);
        PrintMoveNameAndPP(3u8);
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16572))
        .read()) as i32)
            == 3i32
        {
            PrintNewMoveDetailsOrCancelText();
            if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16582))
            .read()) as i32)
                == 4i32
            {
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16580)
                    .cast::<u16>())
                .read()) as i32)
                    != 0i32
                {
                    PrintMoveDetails(
                        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16580)
                            .cast::<u16>())
                        .read(),
                    );
                }
            } else {
                PrintMoveDetails(
                    (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(20))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16582))
                        .read()) as i32) as isize,
                    ))
                    .read(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintBattleMoves(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 1i32 {
                PrintMoveNameAndPP(0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                PrintMoveNameAndPP(1u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                PrintMoveNameAndPP(2u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                PrintMoveNameAndPP(3u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16572))
                .read()) as i32)
                    == 3i32
                {
                    PrintNewMoveDetailsOrCancelText();
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16572))
                .read()) as i32)
                    == 3i32
                {
                    if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16582))
                    .read()) as i32)
                        == 4i32
                    {
                        ((data).wrapping_offset(1)).write(
                            ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16580)
                            .cast::<u16>())
                            .read()) as i16),
                        );
                    } else {
                        ((data).wrapping_offset(1)).write(
                            (((((((((&raw mut sMonSummaryScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(112))
                            .wrapping_add(20))
                            .cast::<u16>())
                            .wrapping_offset(
                                ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(16582))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i16),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16572))
                .read()) as i32)
                    == 3i32
                {
                    if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16580)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32)
                        || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16582))
                        .read()) as i32)
                            != 4i32)
                    {
                        PrintMoveDetails(((((data).wrapping_offset(1)).read()) as u16));
                    }
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                DestroyTask(taskId);
                return;
            }
        }
        (data).write(((data).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn PrintMoveNameAndPP(moveIndex: u8) {
    unsafe {
        let mut moveIndex = moveIndex;
        let mut pp: u8 = 0u8;
        let mut ppState: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut text: *mut u8 = core::ptr::null_mut();
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        let mut moveNameWindowId: u8 = AddWindowFromTemplateList(
            ((&raw const sPageMovesTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
            0u8,
        );
        let mut ppValueWindowId: u8 = AddWindowFromTemplateList(
            ((&raw const sPageMovesTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
            1u8,
        );
        let mut r#move: u16 = ((((summary).wrapping_add(20)).cast::<u16>())
            .wrapping_offset(((moveIndex) as i32) as isize))
        .read();
        if ((r#move) as i32) != 0i32 {
            pp = CalculatePPWithBonus(r#move, ((summary).wrapping_add(52)).read(), moveIndex);
            PrintTextOnWindow(
                moveNameWindowId,
                (((&raw mut gMoveNames).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 13))
                .cast::<u8>(),
                0u8,
                (((((moveIndex) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
                0u8,
                1u8,
            );
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((((summary).wrapping_add(28)).cast::<u8>())
                    .wrapping_offset(((moveIndex) as i32) as isize))
                .read()) as i32),
                1i32,
                2u8,
            );
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar2).cast::<u8>(),
                ((pp) as i32),
                1i32,
                2u8,
            );
            DynamicPlaceholderTextUtil_Reset();
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(0u8, (&raw mut gStringVar1).cast::<u8>());
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(1u8, (&raw mut gStringVar2).cast::<u8>());
            DynamicPlaceholderTextUtil_ExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                ((&raw const sMovesPPLayout).cast::<u8>().cast_mut()).cast::<u8>(),
            );
            text = (&raw mut gStringVar4).cast::<u8>();
            ppState = ((GetCurrentPPToMaxPPState(
                ((((summary).wrapping_add(28)).cast::<u8>())
                    .wrapping_offset(((moveIndex) as i32) as isize))
                .read(),
                pp,
            )) as i32)
                .wrapping_add(9i32);
            x = GetStringRightAlignXOffset(1i32, text, 44i32);
        } else {
            PrintTextOnWindow(
                moveNameWindowId,
                (&raw mut gText_OneDash).cast::<u8>(),
                0u8,
                (((((moveIndex) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
                0u8,
                1u8,
            );
            text = (&raw mut gText_TwoDashes).cast::<u8>();
            ppState = 12i32;
            x = GetStringCenterAlignXOffset(1i32, text, 44i32);
        }
        PrintTextOnWindow(
            ppValueWindowId,
            text,
            ((x) as u8),
            (((((moveIndex) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
            0u8,
            ((ppState) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMovePowerAndAccuracy(moveIndex: u16) {
    unsafe {
        let mut moveIndex = moveIndex;
        let mut text: *mut u8 = core::ptr::null_mut();
        if ((moveIndex) as i32) != 0i32 {
            FillWindowPixelRect(14u8, 0u8, 53u16, 0u16, 19u16, 32u16);
            if ((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((moveIndex) as i32) as isize * 12))
            .wrapping_add(1))
            .read()) as i32)
                < 2i32
            {
                text = (&raw mut gText_ThreeDashes).cast::<u8>();
            } else {
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((((&raw mut gBattleMoves).cast::<u8>())
                        .wrapping_offset(((moveIndex) as i32) as isize * 12))
                    .wrapping_add(1))
                    .read()) as i32),
                    1i32,
                    3u8,
                );
                text = (&raw mut gStringVar1).cast::<u8>();
            }
            PrintTextOnWindow(14u8, text, 53u8, 1u8, 0u8, 0u8);
            if ((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((moveIndex) as i32) as isize * 12))
            .wrapping_add(3))
            .read()) as i32)
                == 0i32
            {
                text = (&raw mut gText_ThreeDashes).cast::<u8>();
            } else {
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((((&raw mut gBattleMoves).cast::<u8>())
                        .wrapping_offset(((moveIndex) as i32) as isize * 12))
                    .wrapping_add(3))
                    .read()) as i32),
                    1i32,
                    3u8,
                );
                text = (&raw mut gStringVar1).cast::<u8>();
            }
            PrintTextOnWindow(14u8, text, 53u8, 17u8, 0u8, 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn PrintContestMoves() {
    unsafe {
        PrintMoveNameAndPP(0u8);
        PrintMoveNameAndPP(1u8);
        PrintMoveNameAndPP(2u8);
        PrintMoveNameAndPP(3u8);
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16572))
        .read()) as i32)
            == 3i32
        {
            PrintNewMoveDetailsOrCancelText();
            PrintContestMoveDescription(
                ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16582))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintContestMoves(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 1i32 {
                PrintMoveNameAndPP(0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                PrintMoveNameAndPP(1u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                PrintMoveNameAndPP(2u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                PrintMoveNameAndPP(3u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16572))
                .read()) as i32)
                    == 3i32
                {
                    PrintNewMoveDetailsOrCancelText();
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16572))
                .read()) as i32)
                    == 3i32
                {
                    if (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16580)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32)
                        || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16582))
                        .read()) as i32)
                            != 4i32)
                    {
                        PrintContestMoveDescription(
                            ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16582))
                            .read(),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                DestroyTask(taskId);
                return;
            }
        }
        (data).write(((data).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn PrintContestMoveDescription(moveSlot: u8) {
    unsafe {
        let mut moveSlot = moveSlot;
        let mut r#move: u16 = 0u16;
        if ((moveSlot) as i32) == 4i32 {
            r#move = ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16580)
                .cast::<u16>())
            .read();
        } else {
            r#move = (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(20))
            .cast::<u16>())
            .wrapping_offset(((moveSlot) as i32) as isize))
            .read();
        }
        if ((r#move) as i32) != 0i32 {
            let mut windowId: u8 = AddWindowFromTemplateList(
                ((&raw const sPageMovesTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
                2u8,
            );
            PrintTextOnWindow(
                windowId,
                ((((&raw mut gContestEffectDescriptionPointers).cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(
                    (((((&raw mut gContestMoves).cast::<u8>())
                        .wrapping_offset(((r#move) as i32) as isize * 8))
                    .read()) as i32) as isize,
                ))
                .read(),
                6u8,
                1u8,
                0u8,
                0u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintMoveDetails(r#move: u16) {
    unsafe {
        let mut r#move = r#move;
        let mut windowId: u8 = AddWindowFromTemplateList(
            ((&raw const sPageMovesTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
            2u8,
        );
        FillWindowPixelBuffer(windowId, 0u8);
        if ((r#move) as i32) != 0i32 {
            if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16576))
            .read()) as i32)
                == 2i32
            {
                PrintMovePowerAndAccuracy(r#move);
                PrintTextOnWindow(
                    windowId,
                    ((((&raw const gMoveDescriptionPointers)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset((((r#move) as i32).wrapping_sub(1i32)) as isize))
                    .read(),
                    6u8,
                    1u8,
                    0u8,
                    0u8,
                );
            } else {
                PrintTextOnWindow(
                    windowId,
                    ((((&raw mut gContestEffectDescriptionPointers).cast::<*mut u8>())
                        .cast::<*mut u8>())
                    .wrapping_offset(
                        (((((&raw mut gContestMoves).cast::<u8>())
                            .wrapping_offset(((r#move) as i32) as isize * 8))
                        .read()) as i32) as isize,
                    ))
                    .read(),
                    6u8,
                    1u8,
                    0u8,
                    0u8,
                );
            }
            PutWindowTilemap(windowId);
        } else {
            ClearWindowTilemap(windowId);
        }
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintNewMoveDetailsOrCancelText() {
    unsafe {
        let mut windowId1: u8 = AddWindowFromTemplateList(
            ((&raw const sPageMovesTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
            0u8,
        );
        let mut windowId2: u8 = AddWindowFromTemplateList(
            ((&raw const sPageMovesTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
            1u8,
        );
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16580)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            PrintTextOnWindow(
                windowId1,
                (&raw mut gText_Cancel).cast::<u8>(),
                0u8,
                65u8,
                0u8,
                1u8,
            );
        } else {
            let mut r#move: u16 = ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(16580)
            .cast::<u16>())
            .read();
            if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16576))
            .read()) as i32)
                == 2i32
            {
                PrintTextOnWindow(
                    windowId1,
                    (((&raw mut gMoveNames).cast::<u8>())
                        .wrapping_offset(((r#move) as i32) as isize * 13))
                    .cast::<u8>(),
                    0u8,
                    65u8,
                    0u8,
                    6u8,
                );
            } else {
                PrintTextOnWindow(
                    windowId1,
                    (((&raw mut gMoveNames).cast::<u8>())
                        .wrapping_offset(((r#move) as i32) as isize * 13))
                    .cast::<u8>(),
                    0u8,
                    65u8,
                    0u8,
                    5u8,
                );
            }
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((((&raw mut gBattleMoves).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 12))
                .wrapping_add(4))
                .read()) as i32),
                1i32,
                2u8,
            );
            DynamicPlaceholderTextUtil_Reset();
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(0u8, (&raw mut gStringVar1).cast::<u8>());
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(1u8, (&raw mut gStringVar1).cast::<u8>());
            DynamicPlaceholderTextUtil_ExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                ((&raw const sMovesPPLayout).cast::<u8>().cast_mut()).cast::<u8>(),
            );
            PrintTextOnWindow(
                windowId2,
                (&raw mut gStringVar4).cast::<u8>(),
                ((GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 44i32))
                    as u8),
                65u8,
                0u8,
                12u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AddAndFillMoveNamesWindow() {
    unsafe {
        let mut windowId: u8 = AddWindowFromTemplateList(
            ((&raw const sPageMovesTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
            0u8,
        );
        FillWindowPixelRect(windowId, 0u8, 0u16, 66u16, 72u16, 16u16);
        CopyWindowToVram(windowId, 2u8);
    }
}
pub(crate) unsafe extern "C" fn SwapMovesNamesPP(moveIndex1: u8, moveIndex2: u8) {
    unsafe {
        let mut moveIndex1 = moveIndex1;
        let mut moveIndex2 = moveIndex2;
        let mut windowId1: u8 = AddWindowFromTemplateList(
            ((&raw const sPageMovesTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
            0u8,
        );
        let mut windowId2: u8 = AddWindowFromTemplateList(
            ((&raw const sPageMovesTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
            1u8,
        );
        FillWindowPixelRect(
            windowId1,
            0u8,
            0u16,
            ((((moveIndex1) as i32).wrapping_mul(16i32)) as u16),
            72u16,
            16u16,
        );
        FillWindowPixelRect(
            windowId1,
            0u8,
            0u16,
            ((((moveIndex2) as i32).wrapping_mul(16i32)) as u16),
            72u16,
            16u16,
        );
        FillWindowPixelRect(
            windowId2,
            0u8,
            0u16,
            ((((moveIndex1) as i32).wrapping_mul(16i32)) as u16),
            48u16,
            16u16,
        );
        FillWindowPixelRect(
            windowId2,
            0u8,
            0u16,
            ((((moveIndex2) as i32).wrapping_mul(16i32)) as u16),
            48u16,
            16u16,
        );
        PrintMoveNameAndPP(moveIndex1);
        PrintMoveNameAndPP(moveIndex2);
    }
}
pub(crate) unsafe extern "C" fn PrintHMMovesCantBeForgotten() {
    unsafe {
        let mut windowId: u8 = AddWindowFromTemplateList(
            ((&raw const sPageMovesTemplate).cast::<u8>().cast_mut()).cast::<u8>(),
            2u8,
        );
        FillWindowPixelBuffer(windowId, 0u8);
        PrintTextOnWindow(
            windowId,
            (&raw mut gText_HMMovesCantBeForgotten2).cast::<u8>(),
            6u8,
            1u8,
            0u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ResetSpriteIds() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(28u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16595))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DestroySpriteInArray(spriteArrayId: u8) {
    unsafe {
        let mut spriteArrayId = spriteArrayId;
        if ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16595))
        .cast::<u8>())
        .wrapping_offset(((spriteArrayId) as i32) as isize))
        .read()) as i32)
            != 255i32
        {
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16595))
                    .cast::<u8>())
                    .wrapping_offset(((spriteArrayId) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16595))
            .cast::<u8>())
            .wrapping_offset(((spriteArrayId) as i32) as isize))
            .write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn SetSpriteInvisibility(spriteArrayId: u8, invisible: u8) {
    unsafe {
        let mut spriteArrayId = spriteArrayId;
        let mut invisible = invisible;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16595))
                .cast::<u8>())
                .wrapping_offset(((spriteArrayId) as i32) as isize))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            ((invisible) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn HidePageSpecificSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 3u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(28u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(16595))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 255i32
                    {
                        SetSpriteInvisibility(i, 1u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetTypeIcons() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16576))
            .read()) as i32);
            if __sw1 == 0i32 {
                SetMonTypeIcons();
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetMoveTypeIcons();
                SetNewMoveTypeIcon();
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetContestMoveTypeIcons();
                SetNewMoveTypeIcon();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMoveTypeIcons() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 3u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(16595))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 255i32
                    {
                        ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16595))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(CreateSprite(
                            (&raw const sSpriteTemplate_MoveTypes)
                                .cast::<u8>()
                                .cast_mut(),
                            0i16,
                            0i16,
                            2u8,
                        ));
                    }
                    SetSpriteInvisibility(i, 1u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetTypeSpritePosAndPal(
    typeId: u8,
    x: u8,
    y: u8,
    spriteArrayId: u8,
) {
    unsafe {
        let mut typeId = typeId;
        let mut x = x;
        let mut y = y;
        let mut spriteArrayId = spriteArrayId;
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16595))
            .cast::<u8>())
            .wrapping_offset(((spriteArrayId) as i32) as isize))
            .read()) as i32) as isize
                * 68,
        );
        StartSpriteAnim(sprite, typeId);
        crate::c::bf_write(
            (sprite).wrapping_add(5),
            4,
            4,
            ((((((&raw const sMoveTypeToOamPaletteNum)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((typeId) as i32) as isize))
            .read()) as u16) as i32,
        );
        ((sprite).wrapping_add(32).cast::<i16>())
            .write(((((x) as i32).wrapping_add(16i32)) as i16));
        ((sprite).wrapping_add(34).cast::<i16>()).write(((((y) as i32).wrapping_add(8i32)) as i16));
        SetSpriteInvisibility(spriteArrayId, 0u8);
    }
}
pub(crate) unsafe extern "C" fn SetMonTypeIcons() {
    unsafe {
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        if (((summary).wrapping_add(4)).read()) != 0 {
            SetTypeSpritePosAndPal(9u8, 120u8, 48u8, 3u8);
            SetSpriteInvisibility(4u8, 1u8);
        } else {
            SetTypeSpritePosAndPal(
                (((((&raw mut gSpeciesInfo).cast::<u8>())
                    .wrapping_offset(((((summary).cast::<u16>()).read()) as i32) as isize * 28))
                .wrapping_add(6))
                .cast::<u8>())
                .read(),
                120u8,
                48u8,
                3u8,
            );
            if (((((((&raw mut gSpeciesInfo).cast::<u8>())
                .wrapping_offset(((((summary).cast::<u16>()).read()) as i32) as isize * 28))
            .wrapping_add(6))
            .cast::<u8>())
            .read()) as i32)
                != ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                    .wrapping_offset(((((summary).cast::<u16>()).read()) as i32) as isize * 28))
                .wrapping_add(6))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
            {
                SetTypeSpritePosAndPal(
                    ((((((&raw mut gSpeciesInfo).cast::<u8>()).wrapping_offset(
                        ((((summary).cast::<u16>()).read()) as i32) as isize * 28,
                    ))
                    .wrapping_add(6))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    160u8,
                    48u8,
                    4u8,
                );
                SetSpriteInvisibility(4u8, 0u8);
            } else {
                SetSpriteInvisibility(4u8, 1u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetMoveTypeIcons() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((summary).wrapping_add(20)).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        SetTypeSpritePosAndPal(
                            ((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                                ((((((summary).wrapping_add(20)).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 12,
                            ))
                            .wrapping_add(2))
                            .read(),
                            85u8,
                            (((32i32).wrapping_add(((i) as i32).wrapping_mul(16i32))) as u8),
                            ((((i) as i32).wrapping_add(3i32)) as u8),
                        );
                    } else {
                        SetSpriteInvisibility(((((i) as i32).wrapping_add(3i32)) as u8), 1u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetContestMoveTypeIcons() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((summary).wrapping_add(20)).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        SetTypeSpritePosAndPal(
                            (((18i32).wrapping_add(
                                ((crate::c::bf_read(
                                    (((&raw mut gContestMoves).cast::<u8>()).wrapping_offset(
                                        ((((((summary).wrapping_add(20)).cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 8,
                                    ))
                                    .wrapping_add(1),
                                    0,
                                    3,
                                    false,
                                ) as u8) as i32),
                            )) as u8),
                            85u8,
                            (((32i32).wrapping_add(((i) as i32).wrapping_mul(16i32))) as u8),
                            ((((i) as i32).wrapping_add(3i32)) as u8),
                        );
                    } else {
                        SetSpriteInvisibility(((((i) as i32).wrapping_add(3i32)) as u8), 1u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetNewMoveTypeIcon() {
    unsafe {
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16580)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            SetSpriteInvisibility(7u8, 1u8);
        } else {
            if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16576))
            .read()) as i32)
                == 2i32
            {
                SetTypeSpritePosAndPal(
                    ((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16580)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 12,
                    ))
                    .wrapping_add(2))
                    .read(),
                    85u8,
                    96u8,
                    7u8,
                );
            } else {
                SetTypeSpritePosAndPal(
                    (((18i32).wrapping_add(
                        ((crate::c::bf_read(
                            (((&raw mut gContestMoves).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(16580)
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 8,
                            ))
                            .wrapping_add(1),
                            0,
                            3,
                            false,
                        ) as u8) as i32),
                    )) as u8),
                    85u8,
                    96u8,
                    7u8,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SwapMovesTypeSprites(moveIndex1: u8, moveIndex2: u8) {
    unsafe {
        let mut moveIndex1 = moveIndex1;
        let mut moveIndex2 = moveIndex2;
        let mut sprite1: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16595))
            .cast::<u8>())
            .wrapping_offset((((moveIndex1) as i32).wrapping_add(3i32)) as isize))
            .read()) as i32) as isize
                * 68,
        );
        let mut sprite2: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16595))
            .cast::<u8>())
            .wrapping_offset((((moveIndex2) as i32).wrapping_add(3i32)) as isize))
            .read()) as i32) as isize
                * 68,
        );
        let mut temp: u8 = ((sprite1).wrapping_add(42)).read();
        ((sprite1).wrapping_add(42)).write(((sprite2).wrapping_add(42)).read());
        ((sprite2).wrapping_add(42)).write(temp);
        temp = ((crate::c::bf_read((sprite1).wrapping_add(5), 4, 4, false) as u16) as u8);
        crate::c::bf_write(
            (sprite1).wrapping_add(5),
            4,
            4,
            (crate::c::bf_read((sprite2).wrapping_add(5), 4, 4, false) as u16) as i32,
        );
        crate::c::bf_write((sprite2).wrapping_add(5), 4, 4, ((temp) as u16) as i32);
        crate::c::bf_write((sprite1).wrapping_add(63), 2, 1, (1u16) as i32);
        crate::c::bf_write((sprite1).wrapping_add(63), 4, 1, (0u16) as i32);
        crate::c::bf_write((sprite2).wrapping_add(63), 2, 1, (1u16) as i32);
        crate::c::bf_write((sprite2).wrapping_add(63), 4, 1, (0u16) as i32);
    }
}
pub(crate) unsafe extern "C" fn LoadMonGfxAndSprite(mon: *mut u8, state: *mut i16) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut state = state;
        let mut pal: *mut u8 = core::ptr::null_mut();
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        'l1: {
            let __sw1 = (((state).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if !__matched {
                return CreateMonSprite(mon);
            }
            if __sw1 == 0i32 {
                if (crate::c::bf_read(
                    ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                    1,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    if (ShouldIgnoreDeoxysForm(
                        3u8,
                        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16574))
                        .read(),
                    )) != 0
                    {
                        HandleLoadSpecialPokePic_DontHandleDeoxys(
                            ((&raw mut gMonFrontPicTable).cast::<u8>()).wrapping_offset(
                                ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32)
                                    as isize
                                    * 8,
                            ),
                            ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .cast::<*mut u8>())
                            .wrapping_offset(1))
                            .read(),
                            ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32),
                            ((summary).wrapping_add(12).cast::<u32>()).read(),
                        );
                    } else {
                        HandleLoadSpecialPokePic_2(
                            ((&raw mut gMonFrontPicTable).cast::<u8>()).wrapping_offset(
                                ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32)
                                    as isize
                                    * 8,
                            ),
                            ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .cast::<*mut u8>())
                            .wrapping_offset(1))
                            .read(),
                            ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32),
                            ((summary).wrapping_add(12).cast::<u32>()).read(),
                        );
                    }
                } else {
                    if ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()) as usize)
                        != 0usize
                    {
                        if ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read()) as usize)
                            == (((&raw mut gPlayerParty).cast::<u8>()) as usize))
                            || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16572))
                            .read()) as i32)
                                == 2i32))
                            || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16623))
                            .read()) as i32)
                                == 1i32)
                        {
                            HandleLoadSpecialPokePic_2(
                                ((&raw mut gMonFrontPicTable).cast::<u8>()).wrapping_offset(
                                    ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32)
                                        as isize
                                        * 8,
                                ),
                                ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(4))
                                .cast::<*mut u8>())
                                .wrapping_offset(1))
                                .read(),
                                ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32),
                                ((summary).wrapping_add(12).cast::<u32>()).read(),
                            );
                        } else {
                            HandleLoadSpecialPokePic_DontHandleDeoxys(
                                ((&raw mut gMonFrontPicTable).cast::<u8>()).wrapping_offset(
                                    ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32)
                                        as isize
                                        * 8,
                                ),
                                ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(4))
                                .cast::<*mut u8>())
                                .wrapping_offset(1))
                                .read(),
                                ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32),
                                ((summary).wrapping_add(12).cast::<u32>()).read(),
                            );
                        }
                    } else {
                        if ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read()) as usize)
                            == (((&raw mut gPlayerParty).cast::<u8>()) as usize))
                            || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16572))
                            .read()) as i32)
                                == 2i32))
                            || (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(16623))
                            .read()) as i32)
                                == 1i32)
                        {
                            HandleLoadSpecialPokePic_2(
                                ((&raw mut gMonFrontPicTable).cast::<u8>()).wrapping_offset(
                                    ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32)
                                        as isize
                                        * 8,
                                ),
                                MonSpritesGfxManager_GetSpritePtr(0u8, 1u8),
                                ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32),
                                ((summary).wrapping_add(12).cast::<u32>()).read(),
                            );
                        } else {
                            HandleLoadSpecialPokePic_DontHandleDeoxys(
                                ((&raw mut gMonFrontPicTable).cast::<u8>()).wrapping_offset(
                                    ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32)
                                        as isize
                                        * 8,
                                ),
                                MonSpritesGfxManager_GetSpritePtr(0u8, 1u8),
                                ((((summary).wrapping_add(2).cast::<u16>()).read()) as i32),
                                ((summary).wrapping_add(12).cast::<u32>()).read(),
                            );
                        }
                    }
                }
                (state).write(((state).read()).wrapping_add(1));
                return 255u8;
            }
            if __sw1 == 1i32 {
                pal = GetMonSpritePalStructFromOtIdPersonality(
                    ((summary).wrapping_add(2).cast::<u16>()).read(),
                    ((summary).wrapping_add(72).cast::<u32>()).read(),
                    ((summary).wrapping_add(12).cast::<u32>()).read(),
                );
                LoadCompressedSpritePalette(pal);
                SetMultiuseSpriteTemplateToPokemon(
                    ((pal).wrapping_add(4).cast::<u16>()).read(),
                    1u8,
                );
                (state).write(((state).read()).wrapping_add(1));
                return 255u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn PlayMonCry() {
    unsafe {
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        if !((((summary).wrapping_add(4)).read()) != 0) {
            if ShouldPlayNormalMonCry(
                (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12),
            ) == 1u32
            {
                PlayCry_ByMode(((summary).wrapping_add(2).cast::<u16>()).read(), 0i8, 0u8);
            } else {
                PlayCry_ByMode(((summary).wrapping_add(2).cast::<u16>()).read(), 0i8, 11u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMonSprite(unused: *mut u8) -> u8 {
    unsafe {
        let mut unused = unused;
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        let mut spriteId: u8 = CreateSprite(
            (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
            40i16,
            64i16,
            5u8,
        );
        FreeSpriteOamMatrix(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((((summary).wrapping_add(2).cast::<u16>()).read()) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Pokemon));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        if !((IsMonSpriteNotFlipped(((summary).wrapping_add(2).cast::<u16>()).read())) != 0) {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(63),
                0,
                1,
                (1u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(63),
                0,
                1,
                (0u16) as i32,
            );
        }
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Pokemon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut summary: *mut u8 = (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(112);
        if (!((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0))
            && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                != 1i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((IsMonSpriteNotFlipped(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u16),
                )) as i16),
            );
            PlayMonCry();
            PokemonSummaryDoMonAnimation(
                sprite,
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u16),
                ((summary).wrapping_add(4)).read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SummaryScreen_SetAnimDelayTaskId(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut sAnimDelayTaskId).cast::<u8>().cast::<u8>()).write(taskId);
    }
}
pub(crate) unsafe extern "C" fn SummaryScreen_DestroyAnimDelayTask() {
    unsafe {
        if ((((&raw mut sAnimDelayTaskId).cast::<u8>().cast::<u8>()).read()) as i32) != 255i32 {
            DestroyTask(((&raw mut sAnimDelayTaskId).cast::<u8>().cast::<u8>()).read());
            ((&raw mut sAnimDelayTaskId).cast::<u8>().cast::<u8>()).write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn IsMonAnimationFinished() -> u32 {
    unsafe {
        if core::mem::transmute::<_, usize>(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16595))
                .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) == (SpriteCallbackDummy as *const () as usize)
        {
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
pub(crate) unsafe extern "C" fn StopPokemonAnimations() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut paletteIndex: u16 = 0u16;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16595))
                .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(44),
            6,
            1,
            (1u8) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16595))
            .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        StopPokemonAnimationDelayTask();
        paletteIndex = (((256i32).wrapping_add(
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16595))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                4,
                4,
                false,
            ) as u16) as i32)
                .wrapping_mul(16i32),
        )) as u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    let mut id: u16 = ((((i) as i32).wrapping_add(((paletteIndex) as i32))) as u16);
                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((id) as i32) as isize))
                    .write(
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((id) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMonMarkingsSprite(mon: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut sprite: *mut u8 = CreateMonMarkingAllCombosSprite(
            30003u16,
            30003u16,
            ((&raw const sMarkings_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>(),
        );
        ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .write(sprite);
        if ((sprite) as usize) != 0usize {
            StartSpriteAnim(sprite, ((GetMonData2(mon, 8i32)) as u8));
            ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(60i16);
            ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write(26i16);
            crate::c::bf_write(
                (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(5),
                2,
                2,
                (1u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn RemoveAndCreateMonMarkingsSprite(mon: *mut u8) {
    unsafe {
        let mut mon = mon;
        DestroySprite(
            ((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read(),
        );
        FreeSpriteTilesByTag(30003u16);
        CreateMonMarkingsSprite(mon);
    }
}
pub(crate) unsafe extern "C" fn CreateCaughtBallSprite(mon: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut ball: u8 = ItemIdToBallId(((GetMonData2(mon, 38i32)) as u16));
        LoadBallGfx(ball);
        ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16595))
        .cast::<u8>())
        .wrapping_offset(1))
        .write(CreateSprite(
            ((&raw mut gBallSpriteTemplates).cast::<u8>())
                .wrapping_offset(((ball) as i32) as isize * 24),
            16i16,
            136i16,
            0u8,
        ));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16595))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16595))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            2,
            2,
            (3u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateSetStatusSprite() {
    unsafe {
        let mut spriteId: *mut u8 =
            (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16595))
            .cast::<u8>())
            .wrapping_offset(2);
        let mut statusAnim: u8 = 0u8;
        if (((spriteId).read()) as i32) == 255i32 {
            (spriteId).write(CreateSprite(
                (&raw const sSpriteTemplate_StatusCondition)
                    .cast::<u8>()
                    .cast_mut(),
                64i16,
                152i16,
                0u8,
            ));
        }
        statusAnim = GetMonAilment(
            (((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12),
        );
        if ((statusAnim) as i32) != 0i32 {
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset((((spriteId).read()) as i32) as isize * 68),
                ((((statusAnim) as i32).wrapping_sub(1i32)) as u8),
            );
            SetSpriteInvisibility(2u8, 0u8);
        } else {
            SetSpriteInvisibility(2u8, 1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMoveSelectorSprites(idArrayStart: u8) {
    unsafe {
        let mut idArrayStart = idArrayStart;
        let mut i: u8 = 0u8;
        let mut spriteIds: *mut u8 =
            (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16595))
            .cast::<u8>())
            .wrapping_offset(((idArrayStart) as i32) as isize);
        if ((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16576))
        .read()) as i32)
            >= 2i32
        {
            let mut subpriority: u8 = 0u8;
            if ((idArrayStart) as i32) == 8i32 {
                subpriority = 1u8;
            }
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 10i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((spriteIds).wrapping_offset(((i) as i32) as isize)).write(CreateSprite(
                            (&raw const sMoveSelectorSpriteTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                            (((((i) as i32).wrapping_mul(16i32)).wrapping_add(89i32)) as i16),
                            40i16,
                            subpriority,
                        ));
                        if ((i) as i32) == 0i32 {
                            StartSpriteAnim(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read())
                                        as i32) as isize
                                        * 68,
                                ),
                                4u8,
                            );
                        } else {
                            if ((i) as i32) == 9i32 {
                                StartSpriteAnim(
                                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((spriteIds).wrapping_offset(((i) as i32) as isize))
                                            .read())
                                            as i32)
                                            as isize
                                            * 68,
                                    ),
                                    5u8,
                                );
                            } else {
                                StartSpriteAnim(
                                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((spriteIds).wrapping_offset(((i) as i32) as isize))
                                            .read())
                                            as i32)
                                            as isize
                                            * 68,
                                    ),
                                    6u8,
                                );
                            }
                        }
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                as isize
                                * 68,
                        ))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_MoveSelector));
                        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                as isize
                                * 68,
                        ))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((idArrayStart) as i16));
                        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                as isize
                                * 68,
                        ))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(0i16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MoveSelector(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(42)).read()) as i32) > 3i32)
            && (((((sprite).wrapping_add(42)).read()) as i32) < 7i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_add(1i32)
                    & 31i32) as i16),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 24i32
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 8i32 {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16582))
                .read()) as i32)
                    .wrapping_mul(16i32)) as i16),
            );
        } else {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16583))
                .read()) as i32)
                    .wrapping_mul(16i32)) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyMoveSelectorSprites(firstArrayId: u8) {
    unsafe {
        let mut firstArrayId = firstArrayId;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    DestroySpriteInArray(
                        ((((firstArrayId) as i32).wrapping_add(((i) as i32))) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetMainMoveSelectorColor(which: u8) {
    unsafe {
        let mut which = which;
        let mut i: u8 = 0u8;
        let mut spriteIds: *mut u8 =
            (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16595))
            .cast::<u8>())
            .wrapping_offset(8);
        which = ((((which) as i32).wrapping_mul(3i32)) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    if ((i) as i32) == 0i32 {
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read())
                                    as i32) as isize
                                    * 68,
                            ),
                            ((((which) as i32).wrapping_add(4i32)) as u8),
                        );
                    } else {
                        if ((i) as i32) == 9i32 {
                            StartSpriteAnim(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read())
                                        as i32) as isize
                                        * 68,
                                ),
                                ((((which) as i32).wrapping_add(5i32)) as u8),
                            );
                        } else {
                            StartSpriteAnim(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read())
                                        as i32) as isize
                                        * 68,
                                ),
                                ((((which) as i32).wrapping_add(6i32)) as u8),
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn KeepMoveSelectorVisible(firstSpriteId: u8) {
    unsafe {
        let mut firstSpriteId = firstSpriteId;
        let mut i: u8 = 0u8;
        let mut spriteIds: *mut u8 =
            (((((&raw mut sMonSummaryScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16595))
            .cast::<u8>())
            .wrapping_offset(((firstSpriteId) as i32) as isize);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                            as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((spriteIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
