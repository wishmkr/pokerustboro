//! Translated from `src/pokemon_summary_screen.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sNullDescription sPoundDescription sKarateChopDescription sDoubleSlapDescription sCometPunchDescription sMegaPunchDescription sPayDayDescription sFirePunchDescription sIcePunchDescription sThunderPunchDescription sScratchDescription sViceGripDescription sGuillotineDescription sRazorWindDescription sSwordsDanceDescription sCutDescription sGustDescription sWingAttackDescription sWhirlwindDescription sFlyDescription sBindDescription sSlamDescription sVineWhipDescription sStompDescription sDoubleKickDescription sMegaKickDescription sJumpKickDescription sRollingKickDescription sSandAttackDescription sHeadbuttDescription sHornAttackDescription sFuryAttackDescription sHornDrillDescription sTackleDescription sBodySlamDescription sWrapDescription sTakeDownDescription sThrashDescription sDoubleEdgeDescription sTailWhipDescription sPoisonStingDescription sTwineedleDescription sPinMissileDescription sLeerDescription sBiteDescription sGrowlDescription sRoarDescription sSingDescription sSupersonicDescription sSonicBoomDescription sDisableDescription sAcidDescription sEmberDescription sFlamethrowerDescription sMistDescription sWaterGunDescription sHydroPumpDescription sSurfDescription sIceBeamDescription sBlizzardDescription sPsybeamDescription sBubbleBeamDescription sAuroraBeamDescription sHyperBeamDescription sPeckDescription sDrillPeckDescription sSubmissionDescription sLowKickDescription sCounterDescription sSeismicTossDescription sStrengthDescription sAbsorbDescription sMegaDrainDescription sLeechSeedDescription sGrowthDescription sRazorLeafDescription sSolarBeamDescription sPoisonPowderDescription sStunSporeDescription sSleepPowderDescription sPetalDanceDescription sStringShotDescription sDragonRageDescription sFireSpinDescription sThunderShockDescription sThunderboltDescription sThunderWaveDescription sThunderDescription sRockThrowDescription sEarthquakeDescription sFissureDescription sDigDescription sToxicDescription sConfusionDescription sPsychicDescription sHypnosisDescription sMeditateDescription sAgilityDescription sQuickAttackDescription sRageDescription sTeleportDescription sNightShadeDescription sMimicDescription sScreechDescription sDoubleTeamDescription sRecoverDescription sHardenDescription sMinimizeDescription sSmokescreenDescription sConfuseRayDescription sWithdrawDescription sDefenseCurlDescription sBarrierDescription sLightScreenDescription sHazeDescription sReflectDescription sFocusEnergyDescription sBideDescription sMetronomeDescription sMirrorMoveDescription sSelfDestructDescription sEggBombDescription sLickDescription sSmogDescription sSludgeDescription sBoneClubDescription sFireBlastDescription sWaterfallDescription sClampDescription sSwiftDescription sSkullBashDescription sSpikeCannonDescription sConstrictDescription sAmnesiaDescription sKinesisDescription sSoftBoiledDescription sHiJumpKickDescription sGlareDescription sDreamEaterDescription sPoisonGasDescription sBarrageDescription sLeechLifeDescription sLovelyKissDescription sSkyAttackDescription sTransformDescription sBubbleDescription sDizzyPunchDescription sSporeDescription sFlashDescription sPsywaveDescription sSplashDescription sAcidArmorDescription sCrabhammerDescription sExplosionDescription sFurySwipesDescription sBonemerangDescription sRestDescription sRockSlideDescription sHyperFangDescription sSharpenDescription sConversionDescription sTriAttackDescription sSuperFangDescription sSlashDescription sSubstituteDescription sStruggleDescription sSketchDescription sTripleKickDescription sThiefDescription sSpiderWebDescription sMindReaderDescription sNightmareDescription sFlameWheelDescription sSnoreDescription sCurseDescription sFlailDescription sConversion2Description sAeroblastDescription sCottonSporeDescription sReversalDescription sSpiteDescription sPowderSnowDescription sProtectDescription sMachPunchDescription sScaryFaceDescription sFaintAttackDescription sSweetKissDescription sBellyDrumDescription sSludgeBombDescription sMudSlapDescription sOctazookaDescription sSpikesDescription sZapCannonDescription sForesightDescription sDestinyBondDescription sPerishSongDescription sIcyWindDescription sDetectDescription sBoneRushDescription sLockOnDescription sOutrageDescription sSandstormDescription sGigaDrainDescription sEndureDescription sCharmDescription sRolloutDescription sFalseSwipeDescription sSwaggerDescription sMilkDrinkDescription sSparkDescription sFuryCutterDescription sSteelWingDescription sMeanLookDescription sAttractDescription sSleepTalkDescription sHealBellDescription sReturnDescription sPresentDescription sFrustrationDescription sSafeguardDescription sPainSplitDescription sSacredFireDescription sMagnitudeDescription sDynamicPunchDescription sMegahornDescription sDragonBreathDescription sBatonPassDescription sEncoreDescription sPursuitDescription sRapidSpinDescription sSweetScentDescription sIronTailDescription sMetalClawDescription sVitalThrowDescription sMorningSunDescription sSynthesisDescription sMoonlightDescription sHiddenPowerDescription sCrossChopDescription sTwisterDescription sRainDanceDescription sSunnyDayDescription sCrunchDescription sMirrorCoatDescription sPsychUpDescription sExtremeSpeedDescription sAncientPowerDescription sShadowBallDescription sFutureSightDescription sRockSmashDescription sWhirlpoolDescription sBeatUpDescription sFakeOutDescription sUproarDescription sStockpileDescription sSpitUpDescription sSwallowDescription sHeatWaveDescription sHailDescription sTormentDescription sFlatterDescription sWillOWispDescription sMementoDescription sFacadeDescription sFocusPunchDescription sSmellingSaltDescription sFollowMeDescription sNaturePowerDescription sChargeDescription sTauntDescription sHelpingHandDescription sTrickDescription sRolePlayDescription sWishDescription sAssistDescription sIngrainDescription sSuperpowerDescription sMagicCoatDescription sRecycleDescription sRevengeDescription sBrickBreakDescription sYawnDescription sKnockOffDescription sEndeavorDescription sEruptionDescription sSkillSwapDescription sImprisonDescription sRefreshDescription sGrudgeDescription sSnatchDescription sSecretPowerDescription sDiveDescription sArmThrustDescription sCamouflageDescription sTailGlowDescription sLusterPurgeDescription sMistBallDescription sFeatherDanceDescription sTeeterDanceDescription sBlazeKickDescription sMudSportDescription sIceBallDescription sNeedleArmDescription sSlackOffDescription sHyperVoiceDescription sPoisonFangDescription sCrushClawDescription sBlastBurnDescription sHydroCannonDescription sMeteorMashDescription sAstonishDescription sWeatherBallDescription sAromatherapyDescription sFakeTearsDescription sAirCutterDescription sOverheatDescription sOdorSleuthDescription sRockTombDescription sSilverWindDescription sMetalSoundDescription sGrassWhistleDescription sTickleDescription sCosmicPowerDescription sWaterSpoutDescription sSignalBeamDescription sShadowPunchDescription sExtrasensoryDescription sSkyUppercutDescription sSandTombDescription sSheerColdDescription sMuddyWaterDescription sBulletSeedDescription sAerialAceDescription sIcicleSpearDescription sIronDefenseDescription sBlockDescription sHowlDescription sDragonClawDescription sFrenzyPlantDescription sBulkUpDescription sBounceDescription sMudShotDescription sPoisonTailDescription sCovetDescription sVoltTackleDescription sMagicalLeafDescription sWaterSportDescription sCalmMindDescription sLeafBladeDescription sDragonDanceDescription sRockBlastDescription sShockWaveDescription sWaterPulseDescription sDoomDesireDescription sPsychoBoostDescription gMoveDescriptionPointers sHardyNatureName sLonelyNatureName sBraveNatureName sAdamantNatureName sNaughtyNatureName sBoldNatureName sDocileNatureName sRelaxedNatureName sImpishNatureName sLaxNatureName sTimidNatureName sHastyNatureName sSeriousNatureName sJollyNatureName sNaiveNatureName sModestNatureName sMildNatureName sQuietNatureName sBashfulNatureName sRashNatureName sCalmNatureName sGentleNatureName sSassyNatureName sCarefulNatureName sQuirkyNatureName gNatureNamePointers sBgTemplates sStatusTilemap sStatusSlidingWindow1 sStatusSlidingWindow2 sPowerAccSlidingWindow sAppealJamSlidingWindow sMultiBattleOrder sSummaryTemplate sPageInfoTemplate sPageSkillsTemplate sPageMovesTemplate sTextColors sButtons_Gfx sTextPrinterFunctions sTextPrinterTasks sMemoNatureTextColor sMemoMiscTextColor sStatsLeftColumnLayout sStatsRightColumnLayout sMovesPPLayout sOamData_MoveTypes sSpriteAnim_TypeNormal sSpriteAnim_TypeFighting sSpriteAnim_TypeFlying sSpriteAnim_TypePoison sSpriteAnim_TypeGround sSpriteAnim_TypeRock sSpriteAnim_TypeBug sSpriteAnim_TypeGhost sSpriteAnim_TypeSteel sSpriteAnim_TypeMystery sSpriteAnim_TypeFire sSpriteAnim_TypeWater sSpriteAnim_TypeGrass sSpriteAnim_TypeElectric sSpriteAnim_TypePsychic sSpriteAnim_TypeIce sSpriteAnim_TypeDragon sSpriteAnim_TypeDark sSpriteAnim_CategoryCool sSpriteAnim_CategoryBeauty sSpriteAnim_CategoryCute sSpriteAnim_CategorySmart sSpriteAnim_CategoryTough sSpriteAnimTable_MoveTypes sSpriteSheet_MoveTypes sSpriteTemplate_MoveTypes sMoveTypeToOamPaletteNum sOamData_MoveSelector sSpriteAnim_MoveSelector0 sSpriteAnim_MoveSelector1 sSpriteAnim_MoveSelector2 sSpriteAnim_MoveSelector3 sSpriteAnim_MoveSelectorLeft sSpriteAnim_MoveSelectorRight sSpriteAnim_MoveSelectorMiddle sSpriteAnim_MoveSelector7 sSpriteAnim_MoveSelector8 sSpriteAnim_MoveSelector9 sSpriteAnimTable_MoveSelector sMoveSelectorSpriteSheet sMoveSelectorSpritePal sMoveSelectorSpriteTemplate sOamData_StatusCondition sSpriteAnim_StatusPoison sSpriteAnim_StatusParalyzed sSpriteAnim_StatusSleep sSpriteAnim_StatusFrozen sSpriteAnim_StatusBurn sSpriteAnim_StatusPokerus sSpriteAnim_StatusFaint sSpriteAnimTable_StatusCondition sStatusIconsSpriteSheet sStatusIconsSpritePalette sSpriteTemplate_StatusCondition sMarkings_Pal

/// `struct PokemonSummaryScreenData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonSummaryScreenData {
    pub monList: PokemonSummaryScreenData_monList,
    pub callback: Option<unsafe extern "C" fn()>,
    pub markingsSprite: *mut Sprite,
    pub currentMon: Pokemon,
    pub summary: PokeSummary,
    pub bgTilemapBuffers: CArray<CArray<CArray<u16, 1024>, 2>, 4>,
    pub mode: u8,
    pub isBoxMon: u8,
    pub curMonIndex: u8,
    pub maxMonIndex: u8,
    pub currPageIndex: u8,
    pub minPageIndex: u8,
    pub maxPageIndex: u8,
    pub lockMonFlag: u8,
    pub newMove: u16,
    pub firstMoveIndex: u8,
    pub secondMoveIndex: u8,
    pub lockMovesFlag: u8,
    pub bgDisplayOrder: u8,
    pub filler40CA: u8,
    pub windowIds: CArray<u8, 8>,
    pub spriteIds: CArray<u8, 28>,
    pub handleDeoxys: u8,
    pub switchCounter: i16,
    pub unk_filler4: CArray<u8, 6>,
}

unsafe impl Sync for PokemonSummaryScreenData {}

/// `struct PokeSummary`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokeSummary {
    pub species: u16,
    pub species2: u16,
    pub isEgg: u8,
    pub level: u8,
    pub ribbonCount: u8,
    pub ailment: u8,
    pub abilityNum: u8,
    pub metLocation: u8,
    pub metLevel: u8,
    pub metGame: u8,
    pub pid: u32,
    pub exp: u32,
    pub moves: CArray<u16, 4>,
    pub pp: CArray<u8, 4>,
    pub currentHP: u16,
    pub maxHP: u16,
    pub atk: u16,
    pub def: u16,
    pub spatk: u16,
    pub spdef: u16,
    pub speed: u16,
    pub item: u16,
    pub friendship: u16,
    pub OTGender: u8,
    pub nature: u8,
    pub ppBonuses: u8,
    pub sanity: u8,
    pub OTName: CArray<u8, 17>,
    pub OTID: u32,
}

unsafe impl Sync for PokeSummary {}

/// `struct SlidingWindow`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SlidingWindow {
    pub gfx: *mut u16,
    pub defaultTile: u16,
    pub width: u8,
    pub height: u8,
    pub left: u8,
    pub top: u8,
}

unsafe impl Sync for SlidingWindow {}

/// The anonymous type of `PokemonSummaryScreenData::monList`.
#[repr(C)]
#[derive(Clone, Copy)]
pub union PokemonSummaryScreenData_monList {
    pub mons: *mut Pokemon,
    pub boxMons: *mut BoxPokemon,
}

unsafe impl Sync for PokemonSummaryScreenData_monList {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PokemonSummaryScreenData>() == 16632);
    assert!(offset_of!(PokemonSummaryScreenData, monList) == 0);
    assert!(offset_of!(PokemonSummaryScreenData, callback) == 4);
    assert!(offset_of!(PokemonSummaryScreenData, markingsSprite) == 8);
    assert!(offset_of!(PokemonSummaryScreenData, currentMon) == 12);
    assert!(offset_of!(PokemonSummaryScreenData, summary) == 112);
    assert!(offset_of!(PokemonSummaryScreenData, bgTilemapBuffers) == 188);
    assert!(offset_of!(PokemonSummaryScreenData, mode) == 16572);
    assert!(offset_of!(PokemonSummaryScreenData, isBoxMon) == 16573);
    assert!(offset_of!(PokemonSummaryScreenData, curMonIndex) == 16574);
    assert!(offset_of!(PokemonSummaryScreenData, maxMonIndex) == 16575);
    assert!(offset_of!(PokemonSummaryScreenData, currPageIndex) == 16576);
    assert!(offset_of!(PokemonSummaryScreenData, minPageIndex) == 16577);
    assert!(offset_of!(PokemonSummaryScreenData, maxPageIndex) == 16578);
    assert!(offset_of!(PokemonSummaryScreenData, lockMonFlag) == 16579);
    assert!(offset_of!(PokemonSummaryScreenData, newMove) == 16580);
    assert!(offset_of!(PokemonSummaryScreenData, firstMoveIndex) == 16582);
    assert!(offset_of!(PokemonSummaryScreenData, secondMoveIndex) == 16583);
    assert!(offset_of!(PokemonSummaryScreenData, lockMovesFlag) == 16584);
    assert!(offset_of!(PokemonSummaryScreenData, bgDisplayOrder) == 16585);
    assert!(offset_of!(PokemonSummaryScreenData, filler40CA) == 16586);
    assert!(offset_of!(PokemonSummaryScreenData, windowIds) == 16587);
    assert!(offset_of!(PokemonSummaryScreenData, spriteIds) == 16595);
    assert!(offset_of!(PokemonSummaryScreenData, handleDeoxys) == 16623);
    assert!(offset_of!(PokemonSummaryScreenData, switchCounter) == 16624);
    assert!(offset_of!(PokemonSummaryScreenData, unk_filler4) == 16626);
    assert!(size_of::<PokeSummary>() == 76);
    assert!(offset_of!(PokeSummary, species) == 0);
    assert!(offset_of!(PokeSummary, species2) == 2);
    assert!(offset_of!(PokeSummary, isEgg) == 4);
    assert!(offset_of!(PokeSummary, level) == 5);
    assert!(offset_of!(PokeSummary, ribbonCount) == 6);
    assert!(offset_of!(PokeSummary, ailment) == 7);
    assert!(offset_of!(PokeSummary, abilityNum) == 8);
    assert!(offset_of!(PokeSummary, metLocation) == 9);
    assert!(offset_of!(PokeSummary, metLevel) == 10);
    assert!(offset_of!(PokeSummary, metGame) == 11);
    assert!(offset_of!(PokeSummary, pid) == 12);
    assert!(offset_of!(PokeSummary, exp) == 16);
    assert!(offset_of!(PokeSummary, moves) == 20);
    assert!(offset_of!(PokeSummary, pp) == 28);
    assert!(offset_of!(PokeSummary, currentHP) == 32);
    assert!(offset_of!(PokeSummary, maxHP) == 34);
    assert!(offset_of!(PokeSummary, atk) == 36);
    assert!(offset_of!(PokeSummary, def) == 38);
    assert!(offset_of!(PokeSummary, spatk) == 40);
    assert!(offset_of!(PokeSummary, spdef) == 42);
    assert!(offset_of!(PokeSummary, speed) == 44);
    assert!(offset_of!(PokeSummary, item) == 46);
    assert!(offset_of!(PokeSummary, friendship) == 48);
    assert!(offset_of!(PokeSummary, OTGender) == 50);
    assert!(offset_of!(PokeSummary, nature) == 51);
    assert!(offset_of!(PokeSummary, ppBonuses) == 52);
    assert!(offset_of!(PokeSummary, sanity) == 53);
    assert!(offset_of!(PokeSummary, OTName) == 54);
    assert!(offset_of!(PokeSummary, OTID) == 72);
    assert!(size_of::<SlidingWindow>() == 12);
    assert!(offset_of!(SlidingWindow, gfx) == 0);
    assert!(offset_of!(SlidingWindow, defaultTile) == 4);
    assert!(offset_of!(SlidingWindow, width) == 6);
    assert!(offset_of!(SlidingWindow, height) == 7);
    assert!(offset_of!(SlidingWindow, left) == 8);
    assert!(offset_of!(SlidingWindow, top) == 9);
    assert!(size_of::<PokemonSummaryScreenData_monList>() == 4);
};

const MOVE_SELECTOR_SPRITES_COUNT: u8 = 10;
const PSS_DATA_WINDOW_EXP: u8 = 4;
const PSS_DATA_WINDOW_INFO_ABILITY: u8 = 2;
const PSS_DATA_WINDOW_INFO_MEMO: u8 = 3;
const PSS_DATA_WINDOW_INFO_ORIGINAL_TRAINER: u8 = 0;
const PSS_DATA_WINDOW_MOVE_DESCRIPTION: u8 = 2;
const PSS_DATA_WINDOW_MOVE_NAMES: u8 = 0;
const PSS_DATA_WINDOW_MOVE_PP: u8 = 1;
const PSS_DATA_WINDOW_SKILLS_STATS_LEFT: u8 = 2;
const PSS_DATA_WINDOW_SKILLS_STATS_RIGHT: u8 = 3;
const PSS_LABEL_WINDOW_BATTLE_MOVES_TITLE: u8 = 2;
const PSS_LABEL_WINDOW_CONTEST_MOVES_TITLE: u8 = 3;
const PSS_LABEL_WINDOW_END: u8 = 20;
const PSS_LABEL_WINDOW_MOVES_APPEAL_JAM: u8 = 15;
const PSS_LABEL_WINDOW_MOVES_POWER_ACC: u8 = 14;
const PSS_LABEL_WINDOW_POKEMON_INFO_RENTAL: u8 = 8;
const PSS_LABEL_WINDOW_POKEMON_INFO_TITLE: u8 = 0;
const PSS_LABEL_WINDOW_POKEMON_INFO_TYPE: u8 = 9;
const PSS_LABEL_WINDOW_POKEMON_SKILLS_EXP: u8 = 12;
const PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_LEFT: u8 = 10;
const PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_RIGHT: u8 = 11;
const PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS: u8 = 13;
const PSS_LABEL_WINDOW_POKEMON_SKILLS_TITLE: u8 = 1;
const PSS_LABEL_WINDOW_PORTRAIT_DEX_NUMBER: u8 = 17;
const PSS_LABEL_WINDOW_PORTRAIT_NICKNAME: u8 = 18;
const PSS_LABEL_WINDOW_PORTRAIT_SPECIES: u8 = 19;
const PSS_LABEL_WINDOW_PROMPT_CANCEL: u8 = 4;
const PSS_LABEL_WINDOW_PROMPT_INFO: u8 = 5;
const PSS_LABEL_WINDOW_PROMPT_SWITCH: u8 = 6;
const PSS_PAGE_BATTLE_MOVES: u8 = 2;
const PSS_PAGE_CONTEST_MOVES: u8 = 3;
const PSS_PAGE_COUNT: u8 = 4;
const PSS_PAGE_INFO: u8 = 0;
const PSS_PAGE_SKILLS: u8 = 1;
const SPRITE_ARR_ID_BALL: i32 = 1;
const SPRITE_ARR_ID_MON: i32 = 0;
const SPRITE_ARR_ID_MOVE_SELECTOR1: u8 = 8;
const SPRITE_ARR_ID_MOVE_SELECTOR2: u8 = 18;
const SPRITE_ARR_ID_STATUS: u8 = 2;
const SPRITE_ARR_ID_TYPE: u8 = 3;
const TAG_MON_MARKINGS: u16 = 30003;
const TILE_EMPTY_APPEAL_HEART: u16 = 4153;
const TILE_EMPTY_JAM_HEART: u16 = 4157;
const TILE_FILLED_APPEAL_HEART: u16 = 4154;
const TILE_FILLED_JAM_HEART: u16 = 4156;
const TYPE_ICON_SPRITE_COUNT: i32 = 5;

static gMoveDescriptionPointers: Table<CArray<*mut u8, 354>> =
    Table((&raw const crate::data::pokemon_summary_screen::gMoveDescriptionPointers).cast());
static gNatureNamePointers: Table<CArray<*mut u8, 25>> =
    Table((&raw const crate::data::pokemon_summary_screen::gNatureNamePointers).cast());
static sAppealJamSlidingWindow: Table<SlidingWindow> =
    Table((&raw const crate::data::pokemon_summary_screen::sAppealJamSlidingWindow).cast());
static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::pokemon_summary_screen::sBgTemplates).cast());
static sButtons_Gfx: Table<CArray<CArray<u8, 128>, 2>> =
    Table((&raw const crate::data::pokemon_summary_screen::sButtons_Gfx).cast());
static sMarkings_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_summary_screen::sMarkings_Pal).cast());
static sMemoMiscTextColor: Table<CArray<u8, 7>> =
    Table((&raw const crate::data::pokemon_summary_screen::sMemoMiscTextColor).cast());
static sMemoNatureTextColor: Table<CArray<u8, 7>> =
    Table((&raw const crate::data::pokemon_summary_screen::sMemoNatureTextColor).cast());
static sMoveSelectorSpritePal: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::pokemon_summary_screen::sMoveSelectorSpritePal).cast());
static sMoveSelectorSpriteSheet: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::pokemon_summary_screen::sMoveSelectorSpriteSheet).cast());
static sMoveSelectorSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_summary_screen::sMoveSelectorSpriteTemplate).cast());
static sMoveTypeToOamPaletteNum: Table<CArray<u8, 23>> =
    Table((&raw const crate::data::pokemon_summary_screen::sMoveTypeToOamPaletteNum).cast());
static sMovesPPLayout: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::pokemon_summary_screen::sMovesPPLayout).cast());
static sMultiBattleOrder: Table<CArray<i8, 6>> =
    Table((&raw const crate::data::pokemon_summary_screen::sMultiBattleOrder).cast());
static sPageInfoTemplate: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::pokemon_summary_screen::sPageInfoTemplate).cast());
static sPageMovesTemplate: Table<CArray<WindowTemplate, 3>> =
    Table((&raw const crate::data::pokemon_summary_screen::sPageMovesTemplate).cast());
static sPageSkillsTemplate: Table<CArray<WindowTemplate, 5>> =
    Table((&raw const crate::data::pokemon_summary_screen::sPageSkillsTemplate).cast());
static sPowerAccSlidingWindow: Table<SlidingWindow> =
    Table((&raw const crate::data::pokemon_summary_screen::sPowerAccSlidingWindow).cast());
static sSpriteSheet_MoveTypes: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::pokemon_summary_screen::sSpriteSheet_MoveTypes).cast());
static sSpriteTemplate_MoveTypes: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_summary_screen::sSpriteTemplate_MoveTypes).cast());
static sSpriteTemplate_StatusCondition: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_summary_screen::sSpriteTemplate_StatusCondition).cast());
static sStatsLeftColumnLayout: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::pokemon_summary_screen::sStatsLeftColumnLayout).cast());
static sStatsRightColumnLayout: Table<CArray<u8, 9>> =
    Table((&raw const crate::data::pokemon_summary_screen::sStatsRightColumnLayout).cast());
static sStatusIconsSpritePalette: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::pokemon_summary_screen::sStatusIconsSpritePalette).cast());
static sStatusIconsSpriteSheet: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::pokemon_summary_screen::sStatusIconsSpriteSheet).cast());
static sStatusSlidingWindow1: Table<SlidingWindow> =
    Table((&raw const crate::data::pokemon_summary_screen::sStatusSlidingWindow1).cast());
static sStatusSlidingWindow2: Table<SlidingWindow> =
    Table((&raw const crate::data::pokemon_summary_screen::sStatusSlidingWindow2).cast());
static sSummaryTemplate: Table<CArray<WindowTemplate, 21>> =
    Table((&raw const crate::data::pokemon_summary_screen::sSummaryTemplate).cast());
static sTextColors: Table<CArray<CArray<u8, 3>, 13>> =
    Table((&raw const crate::data::pokemon_summary_screen::sTextColors).cast());
static sTextPrinterFunctions: Table<CArray<Option<unsafe extern "C" fn()>, 4>> =
    Table((&raw const crate::data::pokemon_summary_screen::sTextPrinterFunctions).cast());
static sTextPrinterTasks: Table<CArray<Option<unsafe extern "C" fn(u8)>, 4>> =
    Table((&raw const crate::data::pokemon_summary_screen::sTextPrinterTasks).cast());

pub(crate) static mut sMonSummaryScreen: *mut PokemonSummaryScreenData = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastViewedMonIndex: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMoveSlotToReplace: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimDelayTaskId: u8 = 0;

unsafe extern "C" {
    static gAbilityDescriptionPointers: CArray<*mut u8, 0>;
    static gAbilityNames: CArray<CArray<u8, 13>, 0>;
    static gBallSpriteTemplates: CArray<SpriteTemplate, 0>;
    static gBattleMoves: CArray<BattleMove, 0>;
    static mut gBattleTypeFlags: u32;
    static gContestEffectDescriptionPointers: CArray<*mut u8, 0>;
    static gContestEffects: CArray<ContestEffect, 0>;
    static gContestMoves: CArray<ContestMove, 0>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static gExperienceTables: CArray<CArray<u32, 101>, 0>;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gMPlayInfo_BGM: MusicPlayerInfo;
    static mut gMain: Main;
    static gMonFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    static mut gMonSpritesGfxPtr: *mut MonSpritesGfx;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static gMoveTypes_Pal: CArray<u32, 0>;
    static mut gMultiuseSpriteTemplate: SpriteTemplate;
    static gPPTextPalette: CArray<u16, 0>;
    static gPPUpGetMask: CArray<u8, 0>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_0x8005: u16;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static gSummaryPage_BattleMoves_Tilemap: CArray<u32, 0>;
    static gSummaryPage_ContestMoves_Tilemap: CArray<u32, 0>;
    static gSummaryPage_InfoEgg_Tilemap: CArray<u32, 0>;
    static gSummaryPage_Info_Tilemap: CArray<u32, 0>;
    static gSummaryPage_Skills_Tilemap: CArray<u32, 0>;
    static gSummaryScreen_Gfx: CArray<u32, 0>;
    static gSummaryScreen_MoveEffect_Cancel_Tilemap: CArray<u16, 0>;
    static gSummaryScreen_Pal: CArray<u32, 0>;
    static mut gTasks: CArray<Task, 0>;
    static gText_Accuracy2: CArray<u8, 0>;
    static gText_Appeal: CArray<u8, 0>;
    static gText_Attack3: CArray<u8, 0>;
    static gText_BattleMoves: CArray<u8, 0>;
    static gText_Cancel: CArray<u8, 0>;
    static gText_Cancel2: CArray<u8, 0>;
    static gText_ContestMoves: CArray<u8, 0>;
    static gText_Defense3: CArray<u8, 0>;
    static gText_EggAboutToHatch: CArray<u8, 0>;
    static gText_EggFromHotSprings: CArray<u8, 0>;
    static gText_EggFromTraveler: CArray<u8, 0>;
    static gText_EggWillHatchSoon: CArray<u8, 0>;
    static gText_EggWillTakeALongTime: CArray<u8, 0>;
    static gText_EggWillTakeSomeTime: CArray<u8, 0>;
    static gText_EmptyString5: CArray<u8, 0>;
    static gText_ExpPoints: CArray<u8, 0>;
    static gText_FemaleSymbol: CArray<u8, 0>;
    static gText_FiveMarks: CArray<u8, 0>;
    static gText_HMMovesCantBeForgotten2: CArray<u8, 0>;
    static gText_HP4: CArray<u8, 0>;
    static gText_IDNumber2: CArray<u8, 0>;
    static gText_Info: CArray<u8, 0>;
    static gText_Jam: CArray<u8, 0>;
    static gText_LevelSymbol: CArray<u8, 0>;
    static gText_MaleSymbol: CArray<u8, 0>;
    static gText_NextLv: CArray<u8, 0>;
    static gText_None: CArray<u8, 0>;
    static gText_NumberClear01: CArray<u8, 0>;
    static gText_OTSlash: CArray<u8, 0>;
    static gText_OddEggFoundByCouple: CArray<u8, 0>;
    static gText_OneDash: CArray<u8, 0>;
    static gText_PeculiarEggNicePlace: CArray<u8, 0>;
    static gText_PeculiarEggTrade: CArray<u8, 0>;
    static gText_PkmnInfo: CArray<u8, 0>;
    static gText_PkmnSkills: CArray<u8, 0>;
    static gText_Power: CArray<u8, 0>;
    static gText_RentalPkmn: CArray<u8, 0>;
    static gText_RibbonsVar1: CArray<u8, 0>;
    static gText_SpAtk4: CArray<u8, 0>;
    static gText_SpDef4: CArray<u8, 0>;
    static gText_Speed2: CArray<u8, 0>;
    static gText_Status: CArray<u8, 0>;
    static gText_Switch: CArray<u8, 0>;
    static gText_ThreeDashes: CArray<u8, 0>;
    static gText_TwoDashes: CArray<u8, 0>;
    static gText_TypeSlash: CArray<u8, 0>;
    static gText_XNature: CArray<u8, 0>;
    static gText_XNatureFatefulEncounter: CArray<u8, 0>;
    static gText_XNatureHatchedAtYZ: CArray<u8, 0>;
    static gText_XNatureHatchedSomewhereAt: CArray<u8, 0>;
    static gText_XNatureMetAtYZ: CArray<u8, 0>;
    static gText_XNatureMetSomewhereAt: CArray<u8, 0>;
    static gText_XNatureObtainedInTrade: CArray<u8, 0>;
    static gText_XNatureProbablyMetAt: CArray<u8, 0>;
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
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AdvanceStorageMonIndex(a0: *mut BoxPokemon, a1: u8, a2: u8, a3: u8) -> i16;
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BlitBitmapToWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn BoxMonToMon(a0: *mut BoxPokemon, a1: *mut Pokemon);
    fn BuildOamBuffer();
    fn CalculatePPWithBonus(a0: u16, a1: u8, a2: u8) -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckPartyHasHadPokerus(a0: *mut Pokemon, a1: u8) -> u8;
    fn CheckPartyPokerus(a0: *mut Pokemon, a1: u8) -> u8;
    fn ClearScheduledBgCopiesToVram();
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateMonMarkingAllCombosSprite(a0: u16, a1: u16, a2: *mut u16) -> *mut Sprite;
    fn CreateMonSpritesGfxManager(a0: u8, a1: u8) -> *mut MonSpritesGfxManager;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroyMonSpritesGfxManager(a0: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroySpriteAndFreeResources(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetAbilityBySpecies(a0: u16, a1: u8) -> u8;
    fn GetBgTilemapBuffer(a0: u8) -> *mut c_void;
    fn GetCurrentPPToMaxPPState(a0: u8, a1: u8) -> u8;
    fn GetItemName(a0: u16) -> *mut u8;
    fn GetLRKeysPressed() -> u8;
    fn GetMapNameHandleAquaHideout(a0: *mut u8, a1: u16) -> *mut u8;
    fn GetMonAilment(a0: *mut Pokemon) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut Pokemon) -> u8;
    fn GetMonNickname(a0: *mut Pokemon, a1: *mut u8) -> *mut u8;
    fn GetMonSpritePalStructFromOtIdPersonality(
        a0: u16,
        a1: u32,
        a2: u32,
    ) -> *mut CompressedSpritePalette;
    fn GetMultiplayerId() -> u8;
    fn GetNature(a0: *mut Pokemon) -> u8;
    fn GetPlayerIDAsU32() -> u32;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn HandleLoadSpecialPokePic_2(
        a0: *mut CompressedSpriteSheet,
        a1: *mut c_void,
        a2: i32,
        a3: u32,
    );
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(
        a0: *mut CompressedSpriteSheet,
        a1: *mut c_void,
        a2: i32,
        a3: u32,
    );
    fn InBattleFactory() -> u8;
    fn InSlateportBattleTent() -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsMonShiny(a0: *mut Pokemon) -> u8;
    fn IsMonSpriteNotFlipped(a0: u16) -> u8;
    fn IsMoveHm(a0: u16) -> u8;
    fn IsMultiBattle() -> u8;
    fn ItemIdToBallId(a0: u16) -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn LoadBallGfx(a0: u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn MenuHelpers_IsLinkActive() -> u8;
    fn MenuHelpers_ShouldWaitForLinkRecv() -> u8;
    fn MonSpritesGfxManager_GetSpritePtr(a0: u8, a1: u8) -> *mut u8;
    fn PlayCry_ByMode(a0: u16, a1: i8, a2: u8);
    fn PlaySE(a0: u16);
    fn PokemonSummaryDoMonAnimation(a0: *mut Sprite, a1: u16, a2: u8);
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
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetBgTilemapPalette(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn SetBoxMonData(a0: *mut BoxPokemon, a1: i32, a2: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShouldIgnoreDeoxysForm(a0: u8, a1: u8) -> u8;
    fn ShouldPlayNormalMonCry(a0: *mut Pokemon) -> u32;
    fn ShowBg(a0: u8);
    fn SpeciesToPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StopCryAndClearCrySongs();
    fn StopPokemonAnimationDelayTask();
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCompareWithoutExtCtrlCodes(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayVolumeControl(a0: *mut MusicPlayerInfo, a1: u16, a2: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPokemonSummaryScreen(
    mode: u8,
    mons: *mut c_void,
    monIndex: u8,
    maxMonIndex: u8,
    callback: Option<unsafe extern "C" fn()>,
) {
    sMonSummaryScreen = AllocZeroed(16632) as *mut PokemonSummaryScreenData;
    (*sMonSummaryScreen).mode = mode;
    (*sMonSummaryScreen).monList.mons = mons as *mut Pokemon;
    (*sMonSummaryScreen).curMonIndex = monIndex;
    (*sMonSummaryScreen).maxMonIndex = maxMonIndex;
    (*sMonSummaryScreen).callback = callback;
    if mode == SUMMARY_MODE_BOX {
        (*sMonSummaryScreen).isBoxMon = TRUE;
    } else {
        (*sMonSummaryScreen).isBoxMon = FALSE;
    }
    match mode {
        SUMMARY_MODE_NORMAL | SUMMARY_MODE_BOX => {
            (*sMonSummaryScreen).minPageIndex = 0;
            (*sMonSummaryScreen).maxPageIndex = 3;
        }
        SUMMARY_MODE_LOCK_MOVES => {
            (*sMonSummaryScreen).minPageIndex = 0;
            (*sMonSummaryScreen).maxPageIndex = 3;
            (*sMonSummaryScreen).lockMovesFlag = TRUE;
        }
        SUMMARY_MODE_SELECT_MOVE => {
            (*sMonSummaryScreen).minPageIndex = PSS_PAGE_BATTLE_MOVES;
            (*sMonSummaryScreen).maxPageIndex = 3;
            (*sMonSummaryScreen).lockMonFlag = TRUE;
        }
        _ => {}
    }
    (*sMonSummaryScreen).currPageIndex = (*sMonSummaryScreen).minPageIndex;
    SummaryScreen_SetAnimDelayTaskId(TASK_NONE);
    if gMonSpritesGfxPtr.is_null() {
        CreateMonSpritesGfxManager(MON_SPR_GFX_MANAGER_A, MON_SPR_GFX_MODE_NORMAL as u8);
    }
    SetMainCallback2(Some(CB2_InitSummaryScreen));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowSelectMovePokemonSummaryScreen(
    mons: *mut Pokemon,
    monIndex: u8,
    maxMonIndex: u8,
    callback: Option<unsafe extern "C" fn()>,
    newMove: u16,
) {
    ShowPokemonSummaryScreen(
        SUMMARY_MODE_SELECT_MOVE,
        mons as *mut c_void,
        monIndex,
        maxMonIndex,
        callback,
    );
    (*sMonSummaryScreen).newMove = newMove;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPokemonSummaryScreenHandleDeoxys(
    mode: u8,
    mons: *mut BoxPokemon,
    monIndex: u8,
    maxMonIndex: u8,
    callback: Option<unsafe extern "C" fn()>,
) {
    ShowPokemonSummaryScreen(mode, mons as *mut c_void, monIndex, maxMonIndex, callback);
    (*sMonSummaryScreen).handleDeoxys = TRUE;
}
pub(crate) unsafe extern "C" fn MainCB2() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VBlank() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn CB2_InitSummaryScreen() {
    while MenuHelpers_ShouldWaitForLinkRecv() != TRUE
        && LoadGraphics() != TRUE
        && MenuHelpers_IsLinkActive() != TRUE
    {}
}
pub(crate) unsafe extern "C" fn LoadGraphics() -> u8 {
    match gMain.state {
        0 => {
            SetVBlankHBlankCallbacksToNull();
            ResetVramOamAndBgCntRegs();
            ClearScheduledBgCopiesToVram();
            gMain.state += 1;
        }
        1 => {
            ScanlineEffect_Stop();
            gMain.state += 1;
        }
        2 => {
            ResetPaletteFade();
            gPaletteFade.set_bufferTransferDisabled(1);
            gMain.state += 1;
        }
        3 => {
            ResetSpriteData();
            gMain.state += 1;
        }
        4 => {
            FreeAllSpritePalettes();
            gMain.state += 1;
        }
        5 => {
            InitBGs();
            (*sMonSummaryScreen).switchCounter = 0;
            gMain.state += 1;
        }
        6 => {
            if DecompressGraphics() != FALSE {
                gMain.state += 1;
            }
        }
        7 => {
            ResetWindows();
            gMain.state += 1;
        }
        8 => {
            DrawPagination();
            gMain.state += 1;
        }
        9 => {
            CopyMonToSummaryStruct(&raw mut (*sMonSummaryScreen).currentMon);
            (*sMonSummaryScreen).switchCounter = 0;
            gMain.state += 1;
        }
        10 => {
            if ExtractMonDataToSummaryStruct(&raw mut (*sMonSummaryScreen).currentMon) != 0 {
                gMain.state += 1;
            }
        }
        11 => {
            PrintMonInfo();
            gMain.state += 1;
        }
        12 => {
            PrintPageNamesAndStats();
            gMain.state += 1;
        }
        13 => {
            PrintPageSpecificText((*sMonSummaryScreen).currPageIndex);
            gMain.state += 1;
        }
        14 => {
            SetDefaultTilemaps();
            gMain.state += 1;
        }
        15 => {
            PutPageWindowTilemaps((*sMonSummaryScreen).currPageIndex);
            gMain.state += 1;
        }
        16 => {
            ResetSpriteIds();
            CreateMoveTypeIcons();
            (*sMonSummaryScreen).switchCounter = 0;
            gMain.state += 1;
        }
        17 => {
            (*sMonSummaryScreen).spriteIds[0] = LoadMonGfxAndSprite(
                &raw mut (*sMonSummaryScreen).currentMon,
                &raw mut (*sMonSummaryScreen).switchCounter,
            );
            if (*sMonSummaryScreen).spriteIds[0] != SPRITE_NONE {
                (*sMonSummaryScreen).switchCounter = 0;
                gMain.state += 1;
            }
        }
        18 => {
            CreateMonMarkingsSprite(&raw mut (*sMonSummaryScreen).currentMon);
            gMain.state += 1;
        }
        19 => {
            CreateCaughtBallSprite(&raw mut (*sMonSummaryScreen).currentMon);
            gMain.state += 1;
        }
        20 => {
            CreateSetStatusSprite();
            gMain.state += 1;
        }
        21 => {
            SetTypeIcons();
            gMain.state += 1;
        }
        22 => {
            if (*sMonSummaryScreen).mode != SUMMARY_MODE_SELECT_MOVE {
                CreateTask(Some(Task_HandleInput), 0);
            } else {
                CreateTask(Some(Task_SetHandleReplaceMoveInput), 0);
            }
            gMain.state += 1;
        }
        23 => {
            BlendPalettes(PALETTES_ALL, 16, 0);
            gMain.state += 1;
        }
        24 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            gPaletteFade.set_bufferTransferDisabled(0);
            gMain.state += 1;
        }
        _ => {
            SetVBlankCallback(Some(VBlank));
            SetMainCallback2(Some(MainCB2));
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn InitBGs() {
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
    SetBgTilemapBuffer(
        1,
        (*sMonSummaryScreen).bgTilemapBuffers[2][0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*sMonSummaryScreen).bgTilemapBuffers[1][0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        3,
        (*sMonSummaryScreen).bgTilemapBuffers[0][0].as_mut_ptr() as *mut c_void,
    );
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    ScheduleBgCopyTilemapToVram(3);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
}
pub(crate) unsafe extern "C" fn DecompressGraphics() -> u8 {
    match (*sMonSummaryScreen).switchCounter {
        0 => {
            ResetTempTileDataBuffers();
            DecompressAndCopyTileDataToVram(
                1,
                (&raw const gSummaryScreen_Gfx).cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            (*sMonSummaryScreen).switchCounter += 1;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != 1 {
                LZDecompressWram(
                    gSummaryPage_Info_Tilemap.as_ptr().cast_mut(),
                    (*sMonSummaryScreen).bgTilemapBuffers[0][0].as_mut_ptr() as *mut c_void,
                );
                (*sMonSummaryScreen).switchCounter += 1;
            }
        }
        2 => {
            LZDecompressWram(
                gSummaryPage_InfoEgg_Tilemap.as_ptr().cast_mut(),
                (*sMonSummaryScreen).bgTilemapBuffers[0][1].as_mut_ptr() as *mut c_void,
            );
            (*sMonSummaryScreen).switchCounter += 1;
        }
        3 => {
            LZDecompressWram(
                gSummaryPage_Skills_Tilemap.as_ptr().cast_mut(),
                (*sMonSummaryScreen).bgTilemapBuffers[1][1].as_mut_ptr() as *mut c_void,
            );
            (*sMonSummaryScreen).switchCounter += 1;
        }
        4 => {
            LZDecompressWram(
                gSummaryPage_BattleMoves_Tilemap.as_ptr().cast_mut(),
                (*sMonSummaryScreen).bgTilemapBuffers[2][1].as_mut_ptr() as *mut c_void,
            );
            (*sMonSummaryScreen).switchCounter += 1;
        }
        5 => {
            LZDecompressWram(
                gSummaryPage_ContestMoves_Tilemap.as_ptr().cast_mut(),
                (*sMonSummaryScreen).bgTilemapBuffers[3][1].as_mut_ptr() as *mut c_void,
            );
            (*sMonSummaryScreen).switchCounter += 1;
        }
        6 => {
            LoadCompressedPalette(gSummaryScreen_Pal.as_ptr().cast_mut(), 0, 256);
            LoadPalette(
                (&raw const gPPTextPalette).cast_mut() as *mut c_void,
                129,
                30,
            );
            (*sMonSummaryScreen).switchCounter += 1;
        }
        7 => {
            LoadCompressedSpriteSheet((&raw const *sSpriteSheet_MoveTypes).cast_mut());
            (*sMonSummaryScreen).switchCounter += 1;
        }
        8 => {
            LoadCompressedSpriteSheet((&raw const *sMoveSelectorSpriteSheet).cast_mut());
            (*sMonSummaryScreen).switchCounter += 1;
        }
        9 => {
            LoadCompressedSpriteSheet((&raw const *sStatusIconsSpriteSheet).cast_mut());
            (*sMonSummaryScreen).switchCounter += 1;
        }
        10 => {
            LoadCompressedSpritePalette((&raw const *sStatusIconsSpritePalette).cast_mut());
            (*sMonSummaryScreen).switchCounter += 1;
        }
        11 => {
            LoadCompressedSpritePalette((&raw const *sMoveSelectorSpritePal).cast_mut());
            (*sMonSummaryScreen).switchCounter += 1;
        }
        12 => {
            LoadCompressedPalette(gMoveTypes_Pal.as_ptr().cast_mut(), 464, 96);
            (*sMonSummaryScreen).switchCounter = 0;
            return TRUE;
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn CopyMonToSummaryStruct(mon: *mut Pokemon) {
    if (*sMonSummaryScreen).isBoxMon == 0 {
        let mut partyMon: *mut Pokemon = (*sMonSummaryScreen).monList.mons;
        *mon = *partyMon.at((*sMonSummaryScreen).curMonIndex);
    } else {
        let mut boxMon: *mut BoxPokemon = (*sMonSummaryScreen).monList.boxMons;
        BoxMonToMon(boxMon.at((*sMonSummaryScreen).curMonIndex), mon);
    }
}
pub(crate) unsafe extern "C" fn ExtractMonDataToSummaryStruct(mon: *mut Pokemon) -> u8 {
    let mut i: u32 = 0;
    let mut sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    match (*sMonSummaryScreen).switchCounter {
        0 => {
            (*sum).species = GetMonData2(mon, MON_DATA_SPECIES) as u16;
            (*sum).species2 = GetMonData2(mon, MON_DATA_SPECIES_OR_EGG) as u16;
            (*sum).exp = GetMonData2(mon, MON_DATA_EXP);
            (*sum).level = GetMonData2(mon, MON_DATA_LEVEL) as u8;
            (*sum).abilityNum = GetMonData2(mon, MON_DATA_ABILITY_NUM) as u8;
            (*sum).item = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
            (*sum).pid = GetMonData2(mon, MON_DATA_PERSONALITY);
            (*sum).sanity = GetMonData2(mon, MON_DATA_SANITY_IS_BAD_EGG) as u8;
            if (*sum).sanity != 0 {
                (*sum).isEgg = TRUE;
            } else {
                (*sum).isEgg = GetMonData2(mon, MON_DATA_IS_EGG) as u8;
            }
        }
        1 => {
            i = 0;
            while i < MAX_MON_MOVES as u32 {
                (*sum).moves[i] = GetMonData2(mon, MON_DATA_MOVE1 + i as i32) as u16;
                (*sum).pp[i] = GetMonData2(mon, MON_DATA_PP1 + i as i32) as u8;
                i += 1;
            }
            (*sum).ppBonuses = GetMonData2(mon, MON_DATA_PP_BONUSES) as u8;
        }
        2 => {
            if (*sMonSummaryScreen).monList.mons == gPlayerParty.as_mut_ptr()
                || (*sMonSummaryScreen).mode == SUMMARY_MODE_BOX
                || (*sMonSummaryScreen).handleDeoxys == TRUE
            {
                (*sum).nature = GetNature(mon);
                (*sum).currentHP = GetMonData2(mon, MON_DATA_HP) as u16;
                (*sum).maxHP = GetMonData2(mon, MON_DATA_MAX_HP) as u16;
                (*sum).atk = GetMonData2(mon, MON_DATA_ATK) as u16;
                (*sum).def = GetMonData2(mon, MON_DATA_DEF) as u16;
                (*sum).spatk = GetMonData2(mon, MON_DATA_SPATK) as u16;
                (*sum).spdef = GetMonData2(mon, MON_DATA_SPDEF) as u16;
                (*sum).speed = GetMonData2(mon, MON_DATA_SPEED) as u16;
            } else {
                (*sum).nature = GetNature(mon);
                (*sum).currentHP = GetMonData2(mon, MON_DATA_HP) as u16;
                (*sum).maxHP = GetMonData2(mon, MON_DATA_MAX_HP) as u16;
                (*sum).atk = GetMonData2(mon, MON_DATA_ATK2) as u16;
                (*sum).def = GetMonData2(mon, MON_DATA_DEF2) as u16;
                (*sum).spatk = GetMonData2(mon, MON_DATA_SPATK2) as u16;
                (*sum).spdef = GetMonData2(mon, MON_DATA_SPDEF2) as u16;
                (*sum).speed = GetMonData2(mon, MON_DATA_SPEED2) as u16;
            }
        }
        3 => {
            GetMonData3(mon, MON_DATA_OT_NAME, (*sum).OTName.as_mut_ptr());
            ConvertInternationalString(
                (*sum).OTName.as_mut_ptr(),
                GetMonData2(mon, MON_DATA_LANGUAGE) as u8,
            );
            (*sum).ailment = GetMonAilment(mon);
            (*sum).OTGender = GetMonData2(mon, MON_DATA_OT_GENDER) as u8;
            (*sum).OTID = GetMonData2(mon, MON_DATA_OT_ID);
            (*sum).metLocation = GetMonData2(mon, MON_DATA_MET_LOCATION) as u8;
            (*sum).metLevel = GetMonData2(mon, MON_DATA_MET_LEVEL) as u8;
            (*sum).metGame = GetMonData2(mon, MON_DATA_MET_GAME) as u8;
            (*sum).friendship = GetMonData2(mon, MON_DATA_FRIENDSHIP) as u16;
        }
        _ => {
            (*sum).ribbonCount = GetMonData2(mon, MON_DATA_RIBBON_COUNT) as u8;
            return TRUE;
        }
    }
    (*sMonSummaryScreen).switchCounter += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn SetDefaultTilemaps() {
    if (*sMonSummaryScreen).currPageIndex != PSS_PAGE_BATTLE_MOVES
        && (*sMonSummaryScreen).currPageIndex != PSS_PAGE_CONTEST_MOVES
    {
        PositionPowerAccSlidingWindow(0, 0xFF);
        PositionAppealJamSlidingWindow(0, 0xFF, 0);
    } else {
        DrawContestMoveHearts(
            (*sMonSummaryScreen).summary.moves[(*sMonSummaryScreen).firstMoveIndex],
        );
        TilemapFiveMovesDisplay(
            (*sMonSummaryScreen).bgTilemapBuffers[2][0].as_mut_ptr(),
            3,
            0,
        );
        TilemapFiveMovesDisplay(
            (*sMonSummaryScreen).bgTilemapBuffers[3][0].as_mut_ptr(),
            1,
            0,
        );
        SetBgTilemapBuffer(
            1,
            (*sMonSummaryScreen).bgTilemapBuffers[3][0].as_mut_ptr() as *mut c_void,
        );
        SetBgTilemapBuffer(
            2,
            (*sMonSummaryScreen).bgTilemapBuffers[2][0].as_mut_ptr() as *mut c_void,
        );
        ChangeBgX(2, 0x10000, BG_COORD_ADD);
        ClearWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_SPECIES);
        ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS);
    }
    if (*sMonSummaryScreen).summary.ailment == AILMENT_NONE {
        PositionStatusSlidingWindow(0, 0xFF);
    } else if (*sMonSummaryScreen).currPageIndex != PSS_PAGE_BATTLE_MOVES
        && (*sMonSummaryScreen).currPageIndex != PSS_PAGE_CONTEST_MOVES
    {
        PutWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS);
    }
    LimitEggSummaryPageDisplay();
    DrawPokerusCuredSymbol(&raw mut (*sMonSummaryScreen).currentMon);
}
pub(crate) unsafe extern "C" fn FreeSummaryScreen() {
    FreeAllWindowBuffers();
    Free(sMonSummaryScreen as *mut c_void);
}
pub(crate) unsafe extern "C" fn BeginCloseSummaryScreen(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    gTasks[taskId].func = Some(CloseSummaryScreen);
}
pub(crate) unsafe extern "C" fn CloseSummaryScreen(taskId: u8) {
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE && gPaletteFade.active() == 0 {
        SetMainCallback2((*sMonSummaryScreen).callback);
        gLastViewedMonIndex = (*sMonSummaryScreen).curMonIndex;
        SummaryScreen_DestroyAnimDelayTask();
        ResetSpriteData();
        FreeAllSpritePalettes();
        StopCryAndClearCrySongs();
        m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
        if gMonSpritesGfxPtr.is_null() {
            DestroyMonSpritesGfxManager(MON_SPR_GFX_MANAGER_A);
        }
        FreeSummaryScreen();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleInput(taskId: u8) {
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE && gPaletteFade.active() == 0 {
        if gMain.newKeys as i32 & DPAD_UP != 0 {
            ChangeSummaryPokemon(taskId, -1);
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
            ChangeSummaryPokemon(taskId, 1);
        } else if gMain.newKeys as i32 & DPAD_LEFT != 0 || GetLRKeysPressed() == MENU_L_PRESSED {
            ChangePage(taskId, -1);
        } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 || GetLRKeysPressed() == MENU_R_PRESSED {
            ChangePage(taskId, 1);
        } else if gMain.newKeys as i32 & A_BUTTON != 0 {
            if (*sMonSummaryScreen).currPageIndex != PSS_PAGE_SKILLS {
                if (*sMonSummaryScreen).currPageIndex == PSS_PAGE_INFO {
                    StopPokemonAnimations();
                    PlaySE(SE_SELECT);
                    BeginCloseSummaryScreen(taskId);
                } else {
                    PlaySE(SE_SELECT);
                    SwitchToMoveSelection(taskId);
                }
            }
        } else if gMain.newKeys as i32 & B_BUTTON != 0 {
            StopPokemonAnimations();
            PlaySE(SE_SELECT);
            BeginCloseSummaryScreen(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ChangeSummaryPokemon(taskId: u8, mut delta: i8) {
    let mut monId: i8 = 0;
    if (*sMonSummaryScreen).lockMonFlag == 0 {
        if (*sMonSummaryScreen).isBoxMon == TRUE {
            if (*sMonSummaryScreen).currPageIndex != PSS_PAGE_INFO {
                if delta == 1 {
                    delta = 0;
                } else {
                    delta = 2;
                }
            } else {
                if delta == 1 {
                    delta = 1;
                } else {
                    delta = 3;
                }
            }
            monId = AdvanceStorageMonIndex(
                (*sMonSummaryScreen).monList.boxMons,
                (*sMonSummaryScreen).curMonIndex,
                (*sMonSummaryScreen).maxMonIndex,
                delta as u8,
            ) as i8;
        } else if IsMultiBattle() == TRUE {
            monId = AdvanceMultiBattleMonIndex(delta);
        } else {
            monId = AdvanceMonIndex(delta);
        }
        if monId != -1 {
            PlaySE(SE_SELECT);
            if (*sMonSummaryScreen).summary.ailment != AILMENT_NONE {
                SetSpriteInvisibility(SPRITE_ARR_ID_STATUS, TRUE);
                ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS);
                ScheduleBgCopyTilemapToVram(0);
                PositionStatusSlidingWindow(0, 2);
            }
            (*sMonSummaryScreen).curMonIndex = monId as u8;
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].func = Some(Task_ChangeSummaryMon);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ChangeSummaryMon(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            StopCryAndClearCrySongs();
        }
        1 => {
            SummaryScreen_DestroyAnimDelayTask();
            DestroySpriteAndFreeResources(&raw mut gSprites[(*sMonSummaryScreen).spriteIds[0]]);
        }
        2 => {
            DestroySpriteAndFreeResources(&raw mut gSprites[(*sMonSummaryScreen).spriteIds[1]]);
        }
        3 => {
            CopyMonToSummaryStruct(&raw mut (*sMonSummaryScreen).currentMon);
            (*sMonSummaryScreen).switchCounter = 0;
        }
        4 => {
            if ExtractMonDataToSummaryStruct(&raw mut (*sMonSummaryScreen).currentMon) == FALSE {
                return;
            }
        }
        5 => {
            RemoveAndCreateMonMarkingsSprite(&raw mut (*sMonSummaryScreen).currentMon);
        }
        6 => {
            CreateCaughtBallSprite(&raw mut (*sMonSummaryScreen).currentMon);
        }
        7 => {
            if (*sMonSummaryScreen).summary.ailment != AILMENT_NONE {
                PositionStatusSlidingWindow(10, -2);
            }
            DrawPokerusCuredSymbol(&raw mut (*sMonSummaryScreen).currentMon);
            *data.at(1) = 0;
        }
        8 => {
            (*sMonSummaryScreen).spriteIds[0] =
                LoadMonGfxAndSprite(&raw mut (*sMonSummaryScreen).currentMon, data.at(1));
            if (*sMonSummaryScreen).spriteIds[0] == SPRITE_NONE {
                return;
            }
            gSprites[(*sMonSummaryScreen).spriteIds[0]].data[2] = 1;
            TryDrawExperienceProgressBar();
            *data.at(1) = 0;
        }
        9 => {
            SetTypeIcons();
        }
        10 => {
            PrintMonInfo();
        }
        11 => {
            PrintPageSpecificText((*sMonSummaryScreen).currPageIndex);
            LimitEggSummaryPageDisplay();
        }
        12 => {
            gSprites[(*sMonSummaryScreen).spriteIds[0]].data[2] = 0;
        }
        _ => {
            if MenuHelpers_ShouldWaitForLinkRecv() == 0
                && FuncIsActiveTask(Some(Task_SlideStatusWindow)) == 0
            {
                *data = 0;
                gTasks[taskId].func = Some(Task_HandleInput);
            }
            return;
        }
    }
    *data += 1;
}
pub(crate) unsafe extern "C" fn AdvanceMonIndex(delta: i8) -> i8 {
    let mut mon: *mut Pokemon = (*sMonSummaryScreen).monList.mons;
    if (*sMonSummaryScreen).currPageIndex == PSS_PAGE_INFO {
        if delta == -1 && (*sMonSummaryScreen).curMonIndex == 0 {
            return -1;
        } else if delta == 1 && (*sMonSummaryScreen).curMonIndex >= (*sMonSummaryScreen).maxMonIndex
        {
            return -1;
        } else {
            return (*sMonSummaryScreen).curMonIndex as i8 + delta;
        }
    } else {
        let mut index: i8 = (*sMonSummaryScreen).curMonIndex as i8;
        loop {
            index += delta;
            if index < 0 || index as i32 > (*sMonSummaryScreen).maxMonIndex as i32 {
                return -1;
            }
            if GetMonData2(mon.at(index), MON_DATA_IS_EGG) == 0 {
                break;
            }
        }
        return index;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn AdvanceMultiBattleMonIndex(delta: i8) -> i8 {
    let mut mons: *mut Pokemon = (*sMonSummaryScreen).monList.mons;
    let mut index: i8 = 0;
    let mut arrId: i8 = 0;
    let mut i: u8 = 0;
    i = 0;
    while i < PARTY_SIZE as u8 {
        if sMultiBattleOrder[i] as i32 == (*sMonSummaryScreen).curMonIndex as i32 {
            arrId = i as i8;
            break;
        }
        i += 1;
    }
    loop {
        let mut order: *mut i8 = sMultiBattleOrder.as_ptr().cast_mut();
        arrId += delta;
        if arrId < 0 || arrId >= PARTY_SIZE as i8 {
            return -1;
        }
        index = *order.at(arrId);
        if IsValidToViewInMulti(mons.at(index)) == TRUE {
            return index;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn IsValidToViewInMulti(mon: *mut Pokemon) -> u8 {
    if GetMonData2(mon, MON_DATA_SPECIES) == SPECIES_NONE as u32 {
        return FALSE;
    } else if (*sMonSummaryScreen).curMonIndex != 0 || GetMonData2(mon, MON_DATA_IS_EGG) == 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ChangePage(taskId: u8, delta: i8) {
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if (*summary).isEgg != 0 {
        return;
    } else if delta == -1 && (*sMonSummaryScreen).currPageIndex == (*sMonSummaryScreen).minPageIndex
    {
        return;
    } else if delta == 1 && (*sMonSummaryScreen).currPageIndex == (*sMonSummaryScreen).maxPageIndex
    {
        return;
    }
    PlaySE(SE_SELECT);
    ClearPageWindowTilemaps((*sMonSummaryScreen).currPageIndex);
    (*sMonSummaryScreen).currPageIndex += delta as u8;
    *data = 0;
    if delta == 1 {
        SetTaskFuncWithFollowupFunc(taskId, Some(PssScrollRight), gTasks[taskId].func);
    } else {
        SetTaskFuncWithFollowupFunc(taskId, Some(PssScrollLeft), gTasks[taskId].func);
    }
    CreateTextPrinterTask((*sMonSummaryScreen).currPageIndex);
    HidePageSpecificSprites();
}
pub(crate) unsafe extern "C" fn PssScrollRight(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data == 0 {
        if (*sMonSummaryScreen).bgDisplayOrder == 0 {
            *data.at(1) = 1;
            SetBgAttribute(1, BG_ATTR_PRIORITY, 1);
            SetBgAttribute(2, BG_ATTR_PRIORITY, 2);
            ScheduleBgCopyTilemapToVram(1);
        } else {
            *data.at(1) = 2;
            SetBgAttribute(2, BG_ATTR_PRIORITY, 1);
            SetBgAttribute(1, BG_ATTR_PRIORITY, 2);
            ScheduleBgCopyTilemapToVram(2);
        }
        ChangeBgX(*data.at(1) as u8, 0, BG_COORD_SET);
        SetBgTilemapBuffer(
            *data.at(1) as u8,
            (*sMonSummaryScreen).bgTilemapBuffers[(*sMonSummaryScreen).currPageIndex][0]
                .as_mut_ptr() as *mut c_void,
        );
        ShowBg(1);
        ShowBg(2);
    }
    ChangeBgX(*data.at(1) as u8, 0x2000, BG_COORD_ADD);
    *data += 32;
    if *data > 0xFF {
        gTasks[taskId].func = Some(PssScrollRightEnd);
    }
}
pub(crate) unsafe extern "C" fn PssScrollRightEnd(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    (*sMonSummaryScreen).bgDisplayOrder ^= 1;
    *data.at(1) = 0;
    *data = 0;
    DrawPagination();
    PutPageWindowTilemaps((*sMonSummaryScreen).currPageIndex);
    SetTypeIcons();
    TryDrawExperienceProgressBar();
    SwitchTaskToFollowupFunc(taskId);
}
pub(crate) unsafe extern "C" fn PssScrollLeft(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data == 0 {
        if (*sMonSummaryScreen).bgDisplayOrder == 0 {
            *data.at(1) = 2;
        } else {
            *data.at(1) = 1;
        }
        ChangeBgX(*data.at(1) as u8, 0x10000, BG_COORD_SET);
    }
    ChangeBgX(*data.at(1) as u8, 0x2000, BG_COORD_SUB);
    *data += 32;
    if *data > 0xFF {
        gTasks[taskId].func = Some(PssScrollLeftEnd);
    }
}
pub(crate) unsafe extern "C" fn PssScrollLeftEnd(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if (*sMonSummaryScreen).bgDisplayOrder == 0 {
        SetBgAttribute(1, BG_ATTR_PRIORITY, 1);
        SetBgAttribute(2, BG_ATTR_PRIORITY, 2);
        ScheduleBgCopyTilemapToVram(2);
    } else {
        SetBgAttribute(2, BG_ATTR_PRIORITY, 1);
        SetBgAttribute(1, BG_ATTR_PRIORITY, 2);
        ScheduleBgCopyTilemapToVram(1);
    }
    if (*sMonSummaryScreen).currPageIndex > 1 {
        SetBgTilemapBuffer(
            *data.at(1) as u8,
            (*sMonSummaryScreen).bgTilemapBuffers[(*sMonSummaryScreen).currPageIndex as i32 - 1][0]
                .as_mut_ptr() as *mut c_void,
        );
        ChangeBgX(*data.at(1) as u8, 0x10000, BG_COORD_SET);
    }
    ShowBg(1);
    ShowBg(2);
    (*sMonSummaryScreen).bgDisplayOrder ^= 1;
    *data.at(1) = 0;
    *data = 0;
    DrawPagination();
    PutPageWindowTilemaps((*sMonSummaryScreen).currPageIndex);
    SetTypeIcons();
    TryDrawExperienceProgressBar();
    SwitchTaskToFollowupFunc(taskId);
}
pub(crate) unsafe extern "C" fn TryDrawExperienceProgressBar() {
    if (*sMonSummaryScreen).currPageIndex == PSS_PAGE_SKILLS {
        DrawExperienceProgressBar(&raw mut (*sMonSummaryScreen).currentMon);
    }
}
pub(crate) unsafe extern "C" fn SwitchToMoveSelection(taskId: u8) {
    let mut r#move: u16 = 0;
    (*sMonSummaryScreen).firstMoveIndex = 0;
    r#move = (*sMonSummaryScreen).summary.moves[(*sMonSummaryScreen).firstMoveIndex];
    ClearWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_SPECIES);
    if gSprites[(*sMonSummaryScreen).spriteIds[2]].invisible() == 0 {
        ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS);
    }
    PositionPowerAccSlidingWindow(9, -3);
    PositionAppealJamSlidingWindow(9, -3, r#move);
    if (*sMonSummaryScreen).lockMovesFlag == 0 {
        ClearWindowTilemap(PSS_LABEL_WINDOW_PROMPT_INFO);
        PutWindowTilemap(PSS_LABEL_WINDOW_PROMPT_SWITCH);
    }
    TilemapFiveMovesDisplay(
        (*sMonSummaryScreen).bgTilemapBuffers[2][0].as_mut_ptr(),
        3,
        0,
    );
    TilemapFiveMovesDisplay(
        (*sMonSummaryScreen).bgTilemapBuffers[3][0].as_mut_ptr(),
        1,
        0,
    );
    PrintMoveDetails(r#move);
    PrintNewMoveDetailsOrCancelText();
    SetNewMoveTypeIcon();
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    CreateMoveSelectorSprites(SPRITE_ARR_ID_MOVE_SELECTOR1);
    gTasks[taskId].func = Some(Task_HandleInput_MoveSelect);
}
pub(crate) unsafe extern "C" fn Task_HandleInput_MoveSelect(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        if gMain.newKeys as i32 & DPAD_UP != 0 {
            *data = 4;
            ChangeSelectedMove(data, -1, &raw mut (*sMonSummaryScreen).firstMoveIndex);
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
            *data = 4;
            ChangeSelectedMove(data, 1, &raw mut (*sMonSummaryScreen).firstMoveIndex);
        } else if gMain.newKeys as i32 & A_BUTTON != 0 {
            if (*sMonSummaryScreen).lockMovesFlag == TRUE
                || (*sMonSummaryScreen).newMove == MOVE_NONE
                    && (*sMonSummaryScreen).firstMoveIndex == MAX_MON_MOVES as u8
            {
                PlaySE(SE_SELECT);
                CloseMoveSelectMode(taskId);
            } else if HasMoreThanOneMove() == TRUE {
                PlaySE(SE_SELECT);
                SwitchToMovePositionSwitchMode(taskId);
            } else {
                PlaySE(SE_FAILURE);
            }
        } else if gMain.newKeys as i32 & B_BUTTON != 0 {
            PlaySE(SE_SELECT);
            CloseMoveSelectMode(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn HasMoreThanOneMove() -> u8 {
    let mut i: u8 = 0;
    i = 1;
    while i < MAX_MON_MOVES as u8 {
        if (*sMonSummaryScreen).summary.moves[i] != 0 {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ChangeSelectedMove(
    taskData: *mut i16,
    direction: i8,
    moveIndexPtr: *mut u8,
) {
    let mut i: i8 = 0;
    let mut newMoveIndex: i8 = 0;
    let mut r#move: u16 = 0;
    PlaySE(SE_SELECT);
    newMoveIndex = *moveIndexPtr as i8;
    i = 0;
    while i < MAX_MON_MOVES as i8 {
        newMoveIndex += direction;
        if newMoveIndex as i16 > *taskData {
            newMoveIndex = 0;
        } else if newMoveIndex < 0 {
            newMoveIndex = *taskData as i8;
        }
        if newMoveIndex == MAX_MON_MOVES as i8 {
            r#move = (*sMonSummaryScreen).newMove;
            break;
        }
        r#move = (*sMonSummaryScreen).summary.moves[newMoveIndex];
        if r#move != 0 {
            break;
        }
        i += 1;
    }
    DrawContestMoveHearts(r#move);
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    PrintMoveDetails(r#move);
    if *moveIndexPtr == MAX_MON_MOVES as u8 && (*sMonSummaryScreen).newMove == MOVE_NONE
        || *taskData.at(1) == 1
    {
        ClearWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_SPECIES);
        if gSprites[(*sMonSummaryScreen).spriteIds[2]].invisible() == 0 {
            ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS);
        }
        ScheduleBgCopyTilemapToVram(0);
        PositionPowerAccSlidingWindow(9, -3);
        PositionAppealJamSlidingWindow(9, -3, r#move);
    }
    if *moveIndexPtr != MAX_MON_MOVES as u8
        && newMoveIndex == MAX_MON_MOVES as i8
        && (*sMonSummaryScreen).newMove == MOVE_NONE
    {
        ClearWindowTilemap(PSS_LABEL_WINDOW_MOVES_POWER_ACC);
        ClearWindowTilemap(PSS_LABEL_WINDOW_MOVES_APPEAL_JAM);
        ScheduleBgCopyTilemapToVram(0);
        PositionPowerAccSlidingWindow(0, 3);
        PositionAppealJamSlidingWindow(0, 3, 0);
    }
    *moveIndexPtr = newMoveIndex as u8;
    if moveIndexPtr == &raw mut (*sMonSummaryScreen).firstMoveIndex {
        KeepMoveSelectorVisible(SPRITE_ARR_ID_MOVE_SELECTOR1);
    } else {
        KeepMoveSelectorVisible(SPRITE_ARR_ID_MOVE_SELECTOR2);
    }
}
pub(crate) unsafe extern "C" fn CloseMoveSelectMode(taskId: u8) {
    DestroyMoveSelectorSprites(SPRITE_ARR_ID_MOVE_SELECTOR1);
    ClearWindowTilemap(PSS_LABEL_WINDOW_PROMPT_SWITCH);
    PutWindowTilemap(PSS_LABEL_WINDOW_PROMPT_INFO);
    PrintMoveDetails(0);
    TilemapFiveMovesDisplay(
        (*sMonSummaryScreen).bgTilemapBuffers[2][0].as_mut_ptr(),
        3,
        TRUE,
    );
    TilemapFiveMovesDisplay(
        (*sMonSummaryScreen).bgTilemapBuffers[3][0].as_mut_ptr(),
        1,
        1,
    );
    AddAndFillMoveNamesWindow();
    if (*sMonSummaryScreen).firstMoveIndex != MAX_MON_MOVES as u8 {
        ClearWindowTilemap(PSS_LABEL_WINDOW_MOVES_POWER_ACC);
        ClearWindowTilemap(PSS_LABEL_WINDOW_MOVES_APPEAL_JAM);
        PositionPowerAccSlidingWindow(0, 3);
        PositionAppealJamSlidingWindow(0, 3, 0);
    }
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    gTasks[taskId].func = Some(Task_HandleInput);
}
pub(crate) unsafe extern "C" fn SwitchToMovePositionSwitchMode(taskId: u8) {
    (*sMonSummaryScreen).secondMoveIndex = (*sMonSummaryScreen).firstMoveIndex;
    SetMainMoveSelectorColor(1);
    CreateMoveSelectorSprites(SPRITE_ARR_ID_MOVE_SELECTOR2);
    gTasks[taskId].func = Some(Task_HandleInput_MovePositionSwitch);
}
pub(crate) unsafe extern "C" fn Task_HandleInput_MovePositionSwitch(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        if gMain.newKeys as i32 & DPAD_UP != 0 {
            *data = 3;
            ChangeSelectedMove(data, -1, &raw mut (*sMonSummaryScreen).secondMoveIndex);
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
            *data = 3;
            ChangeSelectedMove(data, 1, &raw mut (*sMonSummaryScreen).secondMoveIndex);
        } else if gMain.newKeys as i32 & A_BUTTON != 0 {
            if (*sMonSummaryScreen).firstMoveIndex == (*sMonSummaryScreen).secondMoveIndex {
                ExitMovePositionSwitchMode(taskId, FALSE);
            } else {
                ExitMovePositionSwitchMode(taskId, TRUE);
            }
        } else if gMain.newKeys as i32 & B_BUTTON != 0 {
            ExitMovePositionSwitchMode(taskId, FALSE);
        }
    }
}
pub(crate) unsafe extern "C" fn ExitMovePositionSwitchMode(taskId: u8, swapMoves: u8) {
    let mut r#move: u16 = 0;
    PlaySE(SE_SELECT);
    SetMainMoveSelectorColor(0);
    DestroyMoveSelectorSprites(SPRITE_ARR_ID_MOVE_SELECTOR2);
    if swapMoves == TRUE {
        if (*sMonSummaryScreen).isBoxMon == 0 {
            let mut mon: *mut Pokemon = (*sMonSummaryScreen).monList.mons;
            SwapMonMoves(
                mon.at((*sMonSummaryScreen).curMonIndex),
                (*sMonSummaryScreen).firstMoveIndex,
                (*sMonSummaryScreen).secondMoveIndex,
            );
        } else {
            let mut boxMon: *mut BoxPokemon = (*sMonSummaryScreen).monList.boxMons;
            SwapBoxMonMoves(
                boxMon.at((*sMonSummaryScreen).curMonIndex),
                (*sMonSummaryScreen).firstMoveIndex,
                (*sMonSummaryScreen).secondMoveIndex,
            );
        }
        CopyMonToSummaryStruct(&raw mut (*sMonSummaryScreen).currentMon);
        SwapMovesNamesPP(
            (*sMonSummaryScreen).firstMoveIndex,
            (*sMonSummaryScreen).secondMoveIndex,
        );
        SwapMovesTypeSprites(
            (*sMonSummaryScreen).firstMoveIndex,
            (*sMonSummaryScreen).secondMoveIndex,
        );
        (*sMonSummaryScreen).firstMoveIndex = (*sMonSummaryScreen).secondMoveIndex;
    }
    r#move = (*sMonSummaryScreen).summary.moves[(*sMonSummaryScreen).firstMoveIndex];
    PrintMoveDetails(r#move);
    DrawContestMoveHearts(r#move);
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    gTasks[taskId].func = Some(Task_HandleInput_MoveSelect);
}
pub(crate) unsafe extern "C" fn SwapMonMoves(mon: *mut Pokemon, moveIndex1: u8, moveIndex2: u8) {
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut move1: u16 = (*summary).moves[moveIndex1];
    let mut move2: u16 = (*summary).moves[moveIndex2];
    let mut move1pp: u8 = (*summary).pp[moveIndex1];
    let mut move2pp: u8 = (*summary).pp[moveIndex2];
    let mut ppBonuses: u8 = (*summary).ppBonuses;
    let mut ppUpMask1: u8 = gPPUpGetMask[moveIndex1];
    let mut ppBonusMove1: u8 =
        shr_i32(ppBonuses as i32 & ppUpMask1 as i32, moveIndex1 as u32 * 2) as u8;
    let mut ppUpMask2: u8 = gPPUpGetMask[moveIndex2];
    let mut ppBonusMove2: u8 =
        shr_i32(ppBonuses as i32 & ppUpMask2 as i32, moveIndex2 as u32 * 2) as u8;
    ppBonuses &= !ppUpMask1;
    ppBonuses &= !ppUpMask2;
    ppBonuses |= shl_i32(ppBonusMove1 as i32, moveIndex2 as u32 * 2) as u8
        + shl_i32(ppBonusMove2 as i32, moveIndex1 as u32 * 2) as u8;
    SetMonData(
        mon,
        MON_DATA_MOVE1 + moveIndex1 as i32,
        &raw mut move2 as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_MOVE1 + moveIndex2 as i32,
        &raw mut move1 as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_PP1 + moveIndex1 as i32,
        &raw mut move2pp as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_PP1 + moveIndex2 as i32,
        &raw mut move1pp as *mut c_void,
    );
    SetMonData(mon, MON_DATA_PP_BONUSES, &raw mut ppBonuses as *mut c_void);
    (*summary).moves[moveIndex1] = move2;
    (*summary).moves[moveIndex2] = move1;
    (*summary).pp[moveIndex1] = move2pp;
    (*summary).pp[moveIndex2] = move1pp;
    (*summary).ppBonuses = ppBonuses;
}
pub(crate) unsafe extern "C" fn SwapBoxMonMoves(
    mon: *mut BoxPokemon,
    moveIndex1: u8,
    moveIndex2: u8,
) {
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut move1: u16 = (*summary).moves[moveIndex1];
    let mut move2: u16 = (*summary).moves[moveIndex2];
    let mut move1pp: u8 = (*summary).pp[moveIndex1];
    let mut move2pp: u8 = (*summary).pp[moveIndex2];
    let mut ppBonuses: u8 = (*summary).ppBonuses;
    let mut ppUpMask1: u8 = gPPUpGetMask[moveIndex1];
    let mut ppBonusMove1: u8 =
        shr_i32(ppBonuses as i32 & ppUpMask1 as i32, moveIndex1 as u32 * 2) as u8;
    let mut ppUpMask2: u8 = gPPUpGetMask[moveIndex2];
    let mut ppBonusMove2: u8 =
        shr_i32(ppBonuses as i32 & ppUpMask2 as i32, moveIndex2 as u32 * 2) as u8;
    ppBonuses &= !ppUpMask1;
    ppBonuses &= !ppUpMask2;
    ppBonuses |= shl_i32(ppBonusMove1 as i32, moveIndex2 as u32 * 2) as u8
        + shl_i32(ppBonusMove2 as i32, moveIndex1 as u32 * 2) as u8;
    SetBoxMonData(
        mon,
        MON_DATA_MOVE1 + moveIndex1 as i32,
        &raw mut move2 as *mut c_void,
    );
    SetBoxMonData(
        mon,
        MON_DATA_MOVE1 + moveIndex2 as i32,
        &raw mut move1 as *mut c_void,
    );
    SetBoxMonData(
        mon,
        MON_DATA_PP1 + moveIndex1 as i32,
        &raw mut move2pp as *mut c_void,
    );
    SetBoxMonData(
        mon,
        MON_DATA_PP1 + moveIndex2 as i32,
        &raw mut move1pp as *mut c_void,
    );
    SetBoxMonData(mon, MON_DATA_PP_BONUSES, &raw mut ppBonuses as *mut c_void);
    (*summary).moves[moveIndex1] = move2;
    (*summary).moves[moveIndex2] = move1;
    (*summary).pp[moveIndex1] = move2pp;
    (*summary).pp[moveIndex2] = move1pp;
    (*summary).ppBonuses = ppBonuses;
}
pub(crate) unsafe extern "C" fn Task_SetHandleReplaceMoveInput(taskId: u8) {
    SetNewMoveTypeIcon();
    CreateMoveSelectorSprites(SPRITE_ARR_ID_MOVE_SELECTOR1);
    gTasks[taskId].func = Some(Task_HandleReplaceMoveInput);
}
pub(crate) unsafe extern "C" fn Task_HandleReplaceMoveInput(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        if gPaletteFade.active() != TRUE as u16 {
            if gMain.newKeys as i32 & DPAD_UP != 0 {
                *data = 4;
                ChangeSelectedMove(data, -1, &raw mut (*sMonSummaryScreen).firstMoveIndex);
            } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
                *data = 4;
                ChangeSelectedMove(data, 1, &raw mut (*sMonSummaryScreen).firstMoveIndex);
            } else if gMain.newKeys as i32 & DPAD_LEFT != 0 || GetLRKeysPressed() == MENU_L_PRESSED
            {
                ChangePage(taskId, -1);
            } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 || GetLRKeysPressed() == MENU_R_PRESSED
            {
                ChangePage(taskId, 1);
            } else if gMain.newKeys as i32 & A_BUTTON != 0 {
                if CanReplaceMove() == TRUE {
                    StopPokemonAnimations();
                    PlaySE(SE_SELECT);
                    sMoveSlotToReplace = (*sMonSummaryScreen).firstMoveIndex;
                    gSpecialVar_0x8005 = sMoveSlotToReplace as u16;
                    BeginCloseSummaryScreen(taskId);
                } else {
                    PlaySE(SE_FAILURE);
                    ShowCantForgetHMsWindow(taskId);
                }
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                StopPokemonAnimations();
                PlaySE(SE_SELECT);
                sMoveSlotToReplace = MAX_MON_MOVES as u8;
                gSpecialVar_0x8005 = MAX_MON_MOVES as u16;
                BeginCloseSummaryScreen(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CanReplaceMove() -> u8 {
    if (*sMonSummaryScreen).firstMoveIndex == MAX_MON_MOVES as u8
        || (*sMonSummaryScreen).newMove == MOVE_NONE
        || IsMoveHm((*sMonSummaryScreen).summary.moves[(*sMonSummaryScreen).firstMoveIndex]) != TRUE
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
pub(crate) unsafe extern "C" fn ShowCantForgetHMsWindow(taskId: u8) {
    ClearWindowTilemap(PSS_LABEL_WINDOW_MOVES_POWER_ACC);
    ClearWindowTilemap(PSS_LABEL_WINDOW_MOVES_APPEAL_JAM);
    ScheduleBgCopyTilemapToVram(0);
    PositionPowerAccSlidingWindow(0, 3);
    PositionAppealJamSlidingWindow(0, 3, 0);
    PrintHMMovesCantBeForgotten();
    gTasks[taskId].func = Some(Task_HandleInputCantForgetHMsMoves);
}
pub(crate) unsafe extern "C" fn Task_HandleInputCantForgetHMsMoves(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut r#move: u16 = 0;
    if FuncIsActiveTask(Some(Task_SlidePowerAccWindow)) != 1 {
        if gMain.newKeys as i32 & DPAD_UP != 0 {
            *data.at(1) = 1;
            *data = 4;
            ChangeSelectedMove(data, -1, &raw mut (*sMonSummaryScreen).firstMoveIndex);
            *data.at(1) = 0;
            gTasks[taskId].func = Some(Task_HandleReplaceMoveInput);
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
            *data.at(1) = 1;
            *data = 4;
            ChangeSelectedMove(data, 1, &raw mut (*sMonSummaryScreen).firstMoveIndex);
            *data.at(1) = 0;
            gTasks[taskId].func = Some(Task_HandleReplaceMoveInput);
        } else if gMain.newKeys as i32 & DPAD_LEFT != 0 || GetLRKeysPressed() == MENU_L_PRESSED {
            if (*sMonSummaryScreen).currPageIndex != PSS_PAGE_BATTLE_MOVES {
                ClearWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_SPECIES);
                if gSprites[(*sMonSummaryScreen).spriteIds[2]].invisible() == 0 {
                    ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS);
                }
                r#move = (*sMonSummaryScreen).summary.moves[(*sMonSummaryScreen).firstMoveIndex];
                gTasks[taskId].func = Some(Task_HandleReplaceMoveInput);
                ChangePage(taskId, -1);
                PositionPowerAccSlidingWindow(9, -2);
                PositionAppealJamSlidingWindow(9, -2, r#move);
            }
        } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 || GetLRKeysPressed() == MENU_R_PRESSED {
            if (*sMonSummaryScreen).currPageIndex != PSS_PAGE_CONTEST_MOVES {
                ClearWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_SPECIES);
                if gSprites[(*sMonSummaryScreen).spriteIds[2]].invisible() == 0 {
                    ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS);
                }
                r#move = (*sMonSummaryScreen).summary.moves[(*sMonSummaryScreen).firstMoveIndex];
                gTasks[taskId].func = Some(Task_HandleReplaceMoveInput);
                ChangePage(taskId, 1);
                PositionPowerAccSlidingWindow(9, -2);
                PositionAppealJamSlidingWindow(9, -2, r#move);
            }
        } else if gMain.newKeys as i32 & 3 != 0 {
            ClearWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_SPECIES);
            if gSprites[(*sMonSummaryScreen).spriteIds[2]].invisible() == 0 {
                ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS);
            }
            r#move = (*sMonSummaryScreen).summary.moves[(*sMonSummaryScreen).firstMoveIndex];
            PrintMoveDetails(r#move);
            ScheduleBgCopyTilemapToVram(0);
            PositionPowerAccSlidingWindow(9, -3);
            PositionAppealJamSlidingWindow(9, -3, r#move);
            gTasks[taskId].func = Some(Task_HandleReplaceMoveInput);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMoveSlotToReplace() -> u8 {
    return sMoveSlotToReplace;
}
pub(crate) unsafe extern "C" fn DrawPagination() {
    let mut tilemap: *mut u16 = Alloc(32) as *mut u16;
    let mut i: u8 = 0;
    i = 0;
    while i < PSS_PAGE_COUNT {
        let mut j: u8 = i * 2;
        if i < (*sMonSummaryScreen).minPageIndex {
            *tilemap.at(j as i32 + 0) = 0x40;
            *tilemap.at(j as i32 + 1) = 0x40;
            *tilemap.at(j as i32 + 8) = 0x50;
            *tilemap.at(j as i32 + 8 + 1) = 0x50;
        } else if i > (*sMonSummaryScreen).maxPageIndex {
            *tilemap.at(j as i32 + 0) = 0x4A;
            *tilemap.at(j as i32 + 1) = 0x4A;
            *tilemap.at(j as i32 + 8) = 0x5A;
            *tilemap.at(j as i32 + 8 + 1) = 0x5A;
        } else if i < (*sMonSummaryScreen).currPageIndex {
            *tilemap.at(j as i32 + 0) = 0x46;
            *tilemap.at(j as i32 + 1) = 0x47;
            *tilemap.at(j as i32 + 8) = 0x56;
            *tilemap.at(j as i32 + 8 + 1) = 0x57;
        } else if i == (*sMonSummaryScreen).currPageIndex {
            if i != (*sMonSummaryScreen).maxPageIndex {
                *tilemap.at(j as i32 + 0) = 0x41;
                *tilemap.at(j as i32 + 1) = 0x42;
                *tilemap.at(j as i32 + 8) = 0x51;
                *tilemap.at(j as i32 + 8 + 1) = 0x52;
            } else {
                *tilemap.at(j as i32 + 0) = 0x4B;
                *tilemap.at(j as i32 + 1) = 0x4C;
                *tilemap.at(j as i32 + 8) = 0x5B;
                *tilemap.at(j as i32 + 8 + 1) = 0x5C;
            }
        } else if i != (*sMonSummaryScreen).maxPageIndex {
            *tilemap.at(j as i32 + 0) = 0x43;
            *tilemap.at(j as i32 + 1) = 0x44;
            *tilemap.at(j as i32 + 8) = 0x53;
            *tilemap.at(j as i32 + 8 + 1) = 0x54;
        } else {
            *tilemap.at(j as i32 + 0) = 0x48;
            *tilemap.at(j as i32 + 1) = 0x49;
            *tilemap.at(j as i32 + 8) = 0x58;
            *tilemap.at(j as i32 + 8 + 1) = 0x59;
        }
        i += 1;
    }
    CopyToBgTilemapBufferRect_ChangePalette(3, tilemap as *mut c_void, 11, 0, 8, 2, 16);
    ScheduleBgCopyTilemapToVram(3);
    Free(tilemap as *mut c_void);
}
pub(crate) unsafe extern "C" fn CopyNColumnsToTilemap(
    slidingWindow: *mut SlidingWindow,
    mut tilemapDest: *mut u16,
    visibleColumns: u8,
    isOpeningToTheLeft: u8,
) {
    let mut i: u16 = 0;
    let mut alloced: *mut u16 =
        Alloc((*slidingWindow).width as u32 * 2 * (*slidingWindow).height as u32) as *mut u16;
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, (*slidingWindow).defaultTile);
            CpuSet(
                &raw mut tmp as *mut c_void,
                alloced as *mut c_void,
                0x1000000
                    | ((*slidingWindow).width as i32 * 2 * (*slidingWindow).height as i32 / 2)
                        as u32
                        & 0x1FFFFF,
            );
        }
    }
    if (*slidingWindow).width != visibleColumns {
        if isOpeningToTheLeft == 0 {
            i = 0;
            while i < (*slidingWindow).height as u16 {
                CpuSet(
                    (*slidingWindow)
                        .gfx
                        .at(visibleColumns as i32 + (*slidingWindow).width as i32 * i as i32)
                        as *mut c_void,
                    alloced.at((*slidingWindow).width as i32 * i as i32) as *mut c_void,
                    0x00000000
                        | (((*slidingWindow).width as i32 - visibleColumns as i32) * 2 / 2) as u32
                            & 0x1FFFFF,
                );
                i += 1;
            }
        } else {
            i = 0;
            while i < (*slidingWindow).height as u16 {
                CpuSet(
                    (*slidingWindow)
                        .gfx
                        .at((*slidingWindow).width as i32 * i as i32)
                        as *mut c_void,
                    alloced.at(visibleColumns as i32 + (*slidingWindow).width as i32 * i as i32)
                        as *mut c_void,
                    0x00000000
                        | (((*slidingWindow).width as i32 - visibleColumns as i32) * 2 / 2) as u32
                            & 0x1FFFFF,
                );
                i += 1;
            }
        }
    }
    i = 0;
    while i < (*slidingWindow).height as u16 {
        CpuSet(
            alloced.at((*slidingWindow).width as i32 * i as i32) as *mut c_void,
            tilemapDest
                .at(((*slidingWindow).top as i32 + i as i32) * 32 + (*slidingWindow).left as i32)
                as *mut c_void,
            0x00000000 | ((*slidingWindow).width as i32 * 2 / 2) as u32 & 0x1FFFFF,
        );
        i += 1;
    }
    Free(alloced as *mut c_void);
}
pub(crate) unsafe extern "C" fn PositionPowerAccSlidingWindow(visibleColumns: u16, mut speed: i16) {
    if speed > sPowerAccSlidingWindow.width as i16 {
        speed = sPowerAccSlidingWindow.width as i16;
    }
    if speed == 0 || speed == sPowerAccSlidingWindow.width as i16 {
        CopyNColumnsToTilemap(
            (&raw const *sPowerAccSlidingWindow).cast_mut(),
            (*sMonSummaryScreen).bgTilemapBuffers[2][0].as_mut_ptr(),
            speed as u8,
            TRUE,
        );
    } else {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_SlidePowerAccWindow));
        if taskId == TASK_NONE {
            taskId = CreateTask(Some(Task_SlidePowerAccWindow), 8);
        }
        gTasks[taskId].data[0] = speed;
        gTasks[taskId].data[1] = visibleColumns as i16;
    }
}
pub(crate) unsafe extern "C" fn Task_SlidePowerAccWindow(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data.at(1) += *data;
    if *data.at(1) < 0 {
        *data.at(1) = 0;
    } else if *data.at(1) > sPowerAccSlidingWindow.width as i16 {
        *data.at(1) = sPowerAccSlidingWindow.width as i16;
    }
    CopyNColumnsToTilemap(
        (&raw const *sPowerAccSlidingWindow).cast_mut(),
        (*sMonSummaryScreen).bgTilemapBuffers[2][0].as_mut_ptr(),
        *data.at(1) as u8,
        TRUE,
    );
    if *data.at(1) <= 0 || *data.at(1) >= sPowerAccSlidingWindow.width as i16 {
        if *data < 0 {
            if (*sMonSummaryScreen).currPageIndex == PSS_PAGE_BATTLE_MOVES {
                PutWindowTilemap(PSS_LABEL_WINDOW_MOVES_POWER_ACC);
            }
        } else {
            if gSprites[(*sMonSummaryScreen).spriteIds[2]].invisible() == 0 {
                PutWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS);
            }
            PutWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_SPECIES);
        }
        ScheduleBgCopyTilemapToVram(0);
        DestroyTask(taskId);
    }
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe extern "C" fn PositionAppealJamSlidingWindow(
    visibleColumns: u16,
    mut speed: i16,
    r#move: u16,
) {
    if speed > sAppealJamSlidingWindow.width as i16 {
        speed = sAppealJamSlidingWindow.width as i16;
    }
    if speed == 0 || speed == sAppealJamSlidingWindow.width as i16 {
        CopyNColumnsToTilemap(
            (&raw const *sAppealJamSlidingWindow).cast_mut(),
            (*sMonSummaryScreen).bgTilemapBuffers[3][0].as_mut_ptr(),
            speed as u8,
            TRUE,
        );
    } else {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_SlideAppealJamWindow));
        if taskId == TASK_NONE {
            taskId = CreateTask(Some(Task_SlideAppealJamWindow), 8);
        }
        gTasks[taskId].data[0] = speed;
        gTasks[taskId].data[1] = visibleColumns as i16;
        gTasks[taskId].data[2] = r#move as i16;
    }
}
pub(crate) unsafe extern "C" fn Task_SlideAppealJamWindow(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data.at(1) += *data;
    if *data.at(1) < 0 {
        *data.at(1) = 0;
    } else if *data.at(1) > sAppealJamSlidingWindow.width as i16 {
        *data.at(1) = sAppealJamSlidingWindow.width as i16;
    }
    CopyNColumnsToTilemap(
        (&raw const *sAppealJamSlidingWindow).cast_mut(),
        (*sMonSummaryScreen).bgTilemapBuffers[3][0].as_mut_ptr(),
        *data.at(1) as u8,
        TRUE,
    );
    if *data.at(1) <= 0 || *data.at(1) >= sAppealJamSlidingWindow.width as i16 {
        if *data < 0 {
            if (*sMonSummaryScreen).currPageIndex == PSS_PAGE_CONTEST_MOVES
                && FuncIsActiveTask(Some(PssScrollRight)) == 0
            {
                PutWindowTilemap(PSS_LABEL_WINDOW_MOVES_APPEAL_JAM);
            }
            DrawContestMoveHearts(*data.at(2) as u16);
        } else {
            if gSprites[(*sMonSummaryScreen).spriteIds[2]].invisible() == 0 {
                PutWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS);
            }
            PutWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_SPECIES);
        }
        ScheduleBgCopyTilemapToVram(0);
        DestroyTask(taskId);
    }
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe extern "C" fn PositionStatusSlidingWindow(visibleColumns: u16, mut speed: i16) {
    if speed > sStatusSlidingWindow1.width as i16 {
        speed = sStatusSlidingWindow1.width as i16;
    }
    if speed == 0 || speed == sStatusSlidingWindow1.width as i16 {
        CopyNColumnsToTilemap(
            (&raw const *sStatusSlidingWindow1).cast_mut(),
            (*sMonSummaryScreen).bgTilemapBuffers[0][0].as_mut_ptr(),
            speed as u8,
            0,
        );
        CopyNColumnsToTilemap(
            (&raw const *sStatusSlidingWindow2).cast_mut(),
            (*sMonSummaryScreen).bgTilemapBuffers[0][0].as_mut_ptr(),
            speed as u8,
            0,
        );
    } else {
        let mut taskId: u8 = CreateTask(Some(Task_SlideStatusWindow), 8);
        gTasks[taskId].data[0] = speed;
        gTasks[taskId].data[1] = visibleColumns as i16;
    }
}
pub(crate) unsafe extern "C" fn Task_SlideStatusWindow(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data.at(1) += *data;
    if *data.at(1) < 0 {
        *data.at(1) = 0;
    } else if *data.at(1) > sStatusSlidingWindow1.width as i16 {
        *data.at(1) = sStatusSlidingWindow1.width as i16;
    }
    CopyNColumnsToTilemap(
        (&raw const *sStatusSlidingWindow1).cast_mut(),
        (*sMonSummaryScreen).bgTilemapBuffers[0][0].as_mut_ptr(),
        *data.at(1) as u8,
        0,
    );
    CopyNColumnsToTilemap(
        (&raw const *sStatusSlidingWindow2).cast_mut(),
        (*sMonSummaryScreen).bgTilemapBuffers[0][0].as_mut_ptr(),
        *data.at(1) as u8,
        0,
    );
    ScheduleBgCopyTilemapToVram(3);
    if *data.at(1) <= 0 || *data.at(1) >= sStatusSlidingWindow1.width as i16 {
        if *data < 0 {
            CreateSetStatusSprite();
            PutWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS);
            ScheduleBgCopyTilemapToVram(0);
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn TilemapFiveMovesDisplay(
    mut dst: *mut u16,
    mut palette: u16,
    remove: u8,
) {
    let mut i: u16 = 0;
    let mut id: u16 = 0;
    palette *= 0x1000;
    id = 0x56A;
    if remove == 0 {
        i = 0;
        while i < 20 {
            *dst.at(id as i32 + i as i32) = gSummaryScreen_MoveEffect_Cancel_Tilemap[i] + palette;
            *dst.at(id as i32 + i as i32 + 0x20) =
                gSummaryScreen_MoveEffect_Cancel_Tilemap[i] + palette;
            *dst.at(id as i32 + i as i32 + 0x40) =
                gSummaryScreen_MoveEffect_Cancel_Tilemap[i as i32 + 20] + palette;
            i += 1;
        }
    } else {
        i = 0;
        while i < 20 {
            *dst.at(id as i32 + i as i32) =
                gSummaryScreen_MoveEffect_Cancel_Tilemap[i as i32 + 20] + palette;
            *dst.at(id as i32 + i as i32 + 0x20) =
                gSummaryScreen_MoveEffect_Cancel_Tilemap[i as i32 + 40] + palette;
            *dst.at(id as i32 + i as i32 + 0x40) =
                gSummaryScreen_MoveEffect_Cancel_Tilemap[i as i32 + 40] + palette;
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn DrawPokerusCuredSymbol(mon: *mut Pokemon) {
    if CheckPartyPokerus(mon, 0) == 0 && CheckPartyHasHadPokerus(mon, 0) != 0 {
        (*sMonSummaryScreen).bgTilemapBuffers[0][0][547] = 0x2C;
        (*sMonSummaryScreen).bgTilemapBuffers[0][1][547] = 0x2C;
    } else {
        (*sMonSummaryScreen).bgTilemapBuffers[0][0][547] = 0x81A;
        (*sMonSummaryScreen).bgTilemapBuffers[0][1][547] = 0x81A;
    }
    ScheduleBgCopyTilemapToVram(3);
}
pub(crate) unsafe extern "C" fn SetMonPicBackgroundPalette(isMonShiny: u8) {
    if isMonShiny == 0 {
        SetBgTilemapPalette(3, 1, 4, 8, 8, 0);
    } else {
        SetBgTilemapPalette(3, 1, 4, 8, 8, 5);
    }
    ScheduleBgCopyTilemapToVram(3);
}
pub(crate) unsafe extern "C" fn DrawExperienceProgressBar(unused: *mut Pokemon) {
    let mut numExpProgressBarTicks: i64 = 0;
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut dst: *mut u16 = null_mut();
    let mut i: u8 = 0;
    if (*summary).level < MAX_LEVEL as u8 {
        let mut expBetweenLevels: u32 = gExperienceTables
            [gSpeciesInfo[(*summary).species].growthRate][(*summary).level as i32 + 1]
            - gExperienceTables[gSpeciesInfo[(*summary).species].growthRate][(*summary).level];
        let mut expSinceLastLevel: u32 = (*summary).exp
            - gExperienceTables[gSpeciesInfo[(*summary).species].growthRate][(*summary).level];
        numExpProgressBarTicks = div_u32(expSinceLastLevel * 64, expBetweenLevels) as i64;
        if numExpProgressBarTicks == 0 && expSinceLastLevel != 0 {
            numExpProgressBarTicks = 1;
        }
    } else {
        numExpProgressBarTicks = 0;
    }
    dst = &raw mut (*sMonSummaryScreen).bgTilemapBuffers[1][1][597];
    i = 0;
    while i < 8 {
        if numExpProgressBarTicks > 7 {
            *dst.at(i) = 0x206A;
        } else {
            *dst.at(i) = 0x2062 + (numExpProgressBarTicks % 8) as u16;
        }
        numExpProgressBarTicks -= 8;
        if numExpProgressBarTicks < 0 {
            numExpProgressBarTicks = 0;
        }
        i += 1;
    }
    if (GetBgTilemapBuffer(1) as usize)
        == ((*sMonSummaryScreen).bgTilemapBuffers[1][0].as_mut_ptr() as usize)
    {
        ScheduleBgCopyTilemapToVram(1);
    } else {
        ScheduleBgCopyTilemapToVram(2);
    }
}
pub(crate) unsafe extern "C" fn DrawContestMoveHearts(r#move: u16) {
    let mut tilemap: *mut u16 = (*sMonSummaryScreen).bgTilemapBuffers[3][1].as_mut_ptr();
    let mut i: u8 = 0;
    if r#move != MOVE_NONE {
        let mut effectValue: u8 = gContestEffects[gContestMoves[r#move].effect].appeal;
        if effectValue != 0xFF {
            effectValue = (effectValue as i32 / 10) as u8;
        }
        i = 0;
        while i < MAX_CONTEST_MOVE_HEARTS {
            if effectValue != 0xFF && i < effectValue {
                *tilemap.at(i as i32 / 4 * 32 + (i as i32 & 3) + 0x1E6) = TILE_FILLED_APPEAL_HEART;
            } else {
                *tilemap.at(i as i32 / 4 * 32 + (i as i32 & 3) + 0x1E6) = TILE_EMPTY_APPEAL_HEART;
            }
            i += 1;
        }
        effectValue = gContestEffects[gContestMoves[r#move].effect].jam;
        if effectValue != 0xFF {
            effectValue = (effectValue as i32 / 10) as u8;
        }
        i = 0;
        while i < MAX_CONTEST_MOVE_HEARTS {
            if effectValue != 0xFF && i < effectValue {
                *tilemap.at(i as i32 / 4 * 32 + (i as i32 & 3) + 0x226) = TILE_FILLED_JAM_HEART;
            } else {
                *tilemap.at(i as i32 / 4 * 32 + (i as i32 & 3) + 0x226) = TILE_EMPTY_JAM_HEART;
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn LimitEggSummaryPageDisplay() {
    if (*sMonSummaryScreen).summary.isEgg != 0 {
        ChangeBgX(3, 0x10000, BG_COORD_SET);
    } else {
        ChangeBgX(3, 0, BG_COORD_SET);
    }
}
pub(crate) unsafe extern "C" fn ResetWindows() {
    let mut i: u8 = 0;
    InitWindows(sSummaryTemplate.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    i = 0;
    while i < PSS_LABEL_WINDOW_END {
        FillWindowPixelBuffer(i, 0);
        i += 1;
    }
    i = 0;
    while i < 8 {
        (*sMonSummaryScreen).windowIds[i] = WINDOW_NONE;
        i += 1;
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
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        x,
        y,
        0,
        lineSpacing,
        sTextColors[colorId].as_ptr().cast_mut(),
        0,
        string,
    );
}
pub(crate) unsafe extern "C" fn PrintMonInfo() {
    FillWindowPixelBuffer(PSS_LABEL_WINDOW_PORTRAIT_DEX_NUMBER, 0);
    FillWindowPixelBuffer(PSS_LABEL_WINDOW_PORTRAIT_NICKNAME, 0);
    FillWindowPixelBuffer(PSS_LABEL_WINDOW_PORTRAIT_SPECIES, 0);
    if (*sMonSummaryScreen).summary.isEgg == 0 {
        PrintNotEggInfo();
    } else {
        PrintEggInfo();
    }
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn PrintNotEggInfo() {
    let mut strArray: CArray<u8, 16> = zeroed();
    let mut mon: *mut Pokemon = &raw mut (*sMonSummaryScreen).currentMon;
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut dexNum: u16 = SpeciesToPokedexNum((*summary).species);
    if dexNum != 0xFFFF {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (&raw const gText_NumberClear01[0]).cast_mut(),
        );
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            dexNum as i32,
            STR_CONV_MODE_LEADING_ZEROS,
            3,
        );
        StringAppend(gStringVar1.as_mut_ptr(), gStringVar2.as_mut_ptr());
        if IsMonShiny(mon) == 0 {
            PrintTextOnWindow(
                PSS_LABEL_WINDOW_PORTRAIT_DEX_NUMBER,
                gStringVar1.as_mut_ptr(),
                0,
                1,
                0,
                1,
            );
            SetMonPicBackgroundPalette(FALSE);
        } else {
            PrintTextOnWindow(
                PSS_LABEL_WINDOW_PORTRAIT_DEX_NUMBER,
                gStringVar1.as_mut_ptr(),
                0,
                1,
                0,
                7,
            );
            SetMonPicBackgroundPalette(TRUE);
        }
        PutWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_DEX_NUMBER);
    } else {
        ClearWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_DEX_NUMBER);
        if IsMonShiny(mon) == 0 {
            SetMonPicBackgroundPalette(FALSE);
        } else {
            SetMonPicBackgroundPalette(TRUE);
        }
    }
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gText_LevelSymbol.as_ptr().cast_mut(),
    );
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        (*summary).level as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    StringAppend(gStringVar1.as_mut_ptr(), gStringVar2.as_mut_ptr());
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_PORTRAIT_SPECIES,
        gStringVar1.as_mut_ptr(),
        24,
        17,
        0,
        1,
    );
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_PORTRAIT_NICKNAME,
        gStringVar1.as_mut_ptr(),
        0,
        1,
        0,
        1,
    );
    strArray[0] = CHAR_SLASH;
    StringCopy(
        &raw mut strArray[1],
        (&raw const gSpeciesNames[(*summary).species2][0]).cast_mut(),
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_PORTRAIT_SPECIES,
        strArray.as_mut_ptr(),
        0,
        1,
        0,
        1,
    );
    PrintGenderSymbol(mon, (*summary).species2);
    PutWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_NICKNAME);
    PutWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_SPECIES);
}
pub(crate) unsafe extern "C" fn PrintEggInfo() {
    GetMonNickname(
        &raw mut (*sMonSummaryScreen).currentMon,
        gStringVar1.as_mut_ptr(),
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_PORTRAIT_NICKNAME,
        gStringVar1.as_mut_ptr(),
        0,
        1,
        0,
        1,
    );
    PutWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_NICKNAME);
    ClearWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_DEX_NUMBER);
    ClearWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_SPECIES);
}
pub(crate) unsafe extern "C" fn PrintGenderSymbol(mon: *mut Pokemon, species: u16) {
    if species != SPECIES_NIDORAN_M && species != SPECIES_NIDORAN_F {
        match GetMonGender(mon) {
            MON_MALE => {
                PrintTextOnWindow(
                    PSS_LABEL_WINDOW_PORTRAIT_SPECIES,
                    gText_MaleSymbol.as_ptr().cast_mut(),
                    57,
                    17,
                    0,
                    3,
                );
            }
            MON_FEMALE => {
                PrintTextOnWindow(
                    PSS_LABEL_WINDOW_PORTRAIT_SPECIES,
                    gText_FemaleSymbol.as_ptr().cast_mut(),
                    57,
                    17,
                    0,
                    4,
                );
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn PrintAOrBButtonIcon(windowId: u8, bButton: u8, x: u32) {
    let mut button: *mut u8 = null_mut();
    if bButton == 0 {
        button = sButtons_Gfx[0].as_ptr().cast_mut();
    } else {
        button = sButtons_Gfx[1].as_ptr().cast_mut();
    }
    BlitBitmapToWindow(windowId, button, x as u16, 0, 16, 16);
}
pub(crate) unsafe extern "C" fn PrintPageNamesAndStats() {
    let mut stringXPos: i32 = 0;
    let mut iconXPos: i32 = 0;
    let mut statsXPos: i32 = 0;
    PrintTextOnWindow(0, gText_PkmnInfo.as_ptr().cast_mut(), 2, 1, 0, 1);
    PrintTextOnWindow(1, gText_PkmnSkills.as_ptr().cast_mut(), 2, 1, 0, 1);
    PrintTextOnWindow(2, gText_BattleMoves.as_ptr().cast_mut(), 2, 1, 0, 1);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_CONTEST_MOVES_TITLE,
        gText_ContestMoves.as_ptr().cast_mut(),
        2,
        1,
        0,
        1,
    );
    stringXPos =
        GetStringRightAlignXOffset(FONT_NORMAL as i32, gText_Cancel2.as_ptr().cast_mut(), 62);
    iconXPos = stringXPos - 16;
    if iconXPos < 0 {
        iconXPos = 0;
    }
    PrintAOrBButtonIcon(PSS_LABEL_WINDOW_PROMPT_CANCEL, FALSE, iconXPos as u32);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_PROMPT_CANCEL,
        gText_Cancel2.as_ptr().cast_mut(),
        stringXPos as u8,
        1,
        0,
        0,
    );
    stringXPos = GetStringRightAlignXOffset(FONT_NORMAL as i32, gText_Info.as_ptr().cast_mut(), 62);
    iconXPos = stringXPos - 16;
    if iconXPos < 0 {
        iconXPos = 0;
    }
    PrintAOrBButtonIcon(PSS_LABEL_WINDOW_PROMPT_INFO, FALSE, iconXPos as u32);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_PROMPT_INFO,
        gText_Info.as_ptr().cast_mut(),
        stringXPos as u8,
        1,
        0,
        0,
    );
    stringXPos =
        GetStringRightAlignXOffset(FONT_NORMAL as i32, gText_Switch.as_ptr().cast_mut(), 62);
    iconXPos = stringXPos - 16;
    if iconXPos < 0 {
        iconXPos = 0;
    }
    PrintAOrBButtonIcon(PSS_LABEL_WINDOW_PROMPT_SWITCH, FALSE, iconXPos as u32);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_PROMPT_SWITCH,
        gText_Switch.as_ptr().cast_mut(),
        stringXPos as u8,
        1,
        0,
        0,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_INFO_RENTAL,
        gText_RentalPkmn.as_ptr().cast_mut(),
        0,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_INFO_TYPE,
        gText_TypeSlash.as_ptr().cast_mut(),
        0,
        1,
        0,
        0,
    );
    statsXPos =
        6 + GetStringCenterAlignXOffset(FONT_NORMAL as i32, gText_HP4.as_ptr().cast_mut(), 42);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_LEFT,
        gText_HP4.as_ptr().cast_mut(),
        statsXPos as u8,
        1,
        0,
        1,
    );
    statsXPos =
        6 + GetStringCenterAlignXOffset(FONT_NORMAL as i32, gText_Attack3.as_ptr().cast_mut(), 42);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_LEFT,
        gText_Attack3.as_ptr().cast_mut(),
        statsXPos as u8,
        17,
        0,
        1,
    );
    statsXPos =
        6 + GetStringCenterAlignXOffset(FONT_NORMAL as i32, gText_Defense3.as_ptr().cast_mut(), 42);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_LEFT,
        gText_Defense3.as_ptr().cast_mut(),
        statsXPos as u8,
        33,
        0,
        1,
    );
    statsXPos =
        2 + GetStringCenterAlignXOffset(FONT_NORMAL as i32, gText_SpAtk4.as_ptr().cast_mut(), 36);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_RIGHT,
        gText_SpAtk4.as_ptr().cast_mut(),
        statsXPos as u8,
        1,
        0,
        1,
    );
    statsXPos =
        2 + GetStringCenterAlignXOffset(FONT_NORMAL as i32, gText_SpDef4.as_ptr().cast_mut(), 36);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_RIGHT,
        gText_SpDef4.as_ptr().cast_mut(),
        statsXPos as u8,
        17,
        0,
        1,
    );
    statsXPos =
        2 + GetStringCenterAlignXOffset(FONT_NORMAL as i32, gText_Speed2.as_ptr().cast_mut(), 36);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_RIGHT,
        gText_Speed2.as_ptr().cast_mut(),
        statsXPos as u8,
        33,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_EXP,
        gText_ExpPoints.as_ptr().cast_mut(),
        6,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_EXP,
        gText_NextLv.as_ptr().cast_mut(),
        6,
        17,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS,
        gText_Status.as_ptr().cast_mut(),
        2,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_MOVES_POWER_ACC,
        gText_Power.as_ptr().cast_mut(),
        0,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_MOVES_POWER_ACC,
        gText_Accuracy2.as_ptr().cast_mut(),
        0,
        17,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_MOVES_APPEAL_JAM,
        gText_Appeal.as_ptr().cast_mut(),
        0,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_MOVES_APPEAL_JAM,
        gText_Jam.as_ptr().cast_mut(),
        0,
        17,
        0,
        1,
    );
}
pub(crate) unsafe extern "C" fn PutPageWindowTilemaps(page: u8) {
    let mut i: u8 = 0;
    ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_INFO_TITLE);
    ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_TITLE);
    ClearWindowTilemap(PSS_LABEL_WINDOW_BATTLE_MOVES_TITLE);
    ClearWindowTilemap(PSS_LABEL_WINDOW_CONTEST_MOVES_TITLE);
    match page {
        PSS_PAGE_INFO => {
            PutWindowTilemap(PSS_LABEL_WINDOW_POKEMON_INFO_TITLE);
            PutWindowTilemap(PSS_LABEL_WINDOW_PROMPT_CANCEL);
            if InBattleFactory() == TRUE || InSlateportBattleTent() == TRUE {
                PutWindowTilemap(PSS_LABEL_WINDOW_POKEMON_INFO_RENTAL);
            }
            PutWindowTilemap(PSS_LABEL_WINDOW_POKEMON_INFO_TYPE);
        }
        PSS_PAGE_SKILLS => {
            PutWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_TITLE);
            PutWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_LEFT);
            PutWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_RIGHT);
            PutWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_EXP);
        }
        PSS_PAGE_BATTLE_MOVES => {
            PutWindowTilemap(PSS_LABEL_WINDOW_BATTLE_MOVES_TITLE);
            if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE {
                if (*sMonSummaryScreen).newMove != MOVE_NONE
                    || (*sMonSummaryScreen).firstMoveIndex != MAX_MON_MOVES as u8
                {
                    PutWindowTilemap(PSS_LABEL_WINDOW_MOVES_POWER_ACC);
                }
            } else {
                PutWindowTilemap(PSS_LABEL_WINDOW_PROMPT_INFO);
            }
        }
        PSS_PAGE_CONTEST_MOVES => {
            PutWindowTilemap(PSS_LABEL_WINDOW_CONTEST_MOVES_TITLE);
            if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE {
                if (*sMonSummaryScreen).newMove != MOVE_NONE
                    || (*sMonSummaryScreen).firstMoveIndex != MAX_MON_MOVES as u8
                {
                    PutWindowTilemap(PSS_LABEL_WINDOW_MOVES_APPEAL_JAM);
                }
            } else {
                PutWindowTilemap(PSS_LABEL_WINDOW_PROMPT_INFO);
            }
        }
        _ => {}
    }
    i = 0;
    while i < 8 {
        PutWindowTilemap((*sMonSummaryScreen).windowIds[i]);
        i += 1;
    }
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn ClearPageWindowTilemaps(page: u8) {
    let mut i: u8 = 0;
    match page {
        PSS_PAGE_INFO => {
            ClearWindowTilemap(PSS_LABEL_WINDOW_PROMPT_CANCEL);
            if InBattleFactory() == TRUE || InSlateportBattleTent() == TRUE {
                ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_INFO_RENTAL);
            }
            ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_INFO_TYPE);
        }
        PSS_PAGE_SKILLS => {
            ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_LEFT);
            ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_RIGHT);
            ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_EXP);
        }
        PSS_PAGE_BATTLE_MOVES => {
            if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE {
                if (*sMonSummaryScreen).newMove != MOVE_NONE
                    || (*sMonSummaryScreen).firstMoveIndex != MAX_MON_MOVES as u8
                {
                    ClearWindowTilemap(PSS_LABEL_WINDOW_MOVES_POWER_ACC);
                }
            } else {
                ClearWindowTilemap(PSS_LABEL_WINDOW_PROMPT_INFO);
            }
        }
        PSS_PAGE_CONTEST_MOVES => {
            if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE {
                if (*sMonSummaryScreen).newMove != MOVE_NONE
                    || (*sMonSummaryScreen).firstMoveIndex != MAX_MON_MOVES as u8
                {
                    ClearWindowTilemap(PSS_LABEL_WINDOW_MOVES_APPEAL_JAM);
                }
            } else {
                ClearWindowTilemap(PSS_LABEL_WINDOW_PROMPT_INFO);
            }
        }
        _ => {}
    }
    i = 0;
    while i < 8 {
        RemoveWindowByIndex(i);
        i += 1;
    }
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn AddWindowFromTemplateList(
    mut template: *mut WindowTemplate,
    templateId: u8,
) -> u8 {
    let mut windowIdPtr: *mut u8 = &raw mut (*sMonSummaryScreen).windowIds[templateId];
    if *windowIdPtr == WINDOW_NONE {
        *windowIdPtr = AddWindow(template.at(templateId)) as u8;
        FillWindowPixelBuffer(*windowIdPtr, 0);
    }
    return *windowIdPtr;
}
pub(crate) unsafe extern "C" fn RemoveWindowByIndex(windowIndex: u8) {
    let mut windowIdPtr: *mut u8 = &raw mut (*sMonSummaryScreen).windowIds[windowIndex];
    if *windowIdPtr != WINDOW_NONE {
        ClearWindowTilemap(*windowIdPtr);
        RemoveWindow(*windowIdPtr);
        *windowIdPtr = WINDOW_NONE;
    }
}
pub(crate) unsafe extern "C" fn PrintPageSpecificText(pageIndex: u8) {
    let mut i: u16 = 0;
    i = 0;
    while i < 8 {
        if (*sMonSummaryScreen).windowIds[i] != WINDOW_NONE {
            FillWindowPixelBuffer((*sMonSummaryScreen).windowIds[i], 0);
        }
        i += 1;
    }
    sTextPrinterFunctions[pageIndex].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn CreateTextPrinterTask(pageIndex: u8) {
    CreateTask(sTextPrinterTasks[pageIndex], 16);
}
pub(crate) unsafe extern "C" fn PrintInfoPageText() {
    if (*sMonSummaryScreen).summary.isEgg != 0 {
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
pub(crate) unsafe extern "C" fn Task_PrintInfoPage(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        1 => {
            PrintMonOTName();
        }
        2 => {
            PrintMonOTID();
        }
        3 => {
            PrintMonAbilityName();
        }
        4 => {
            PrintMonAbilityDescription();
        }
        5 => {
            BufferMonTrainerMemo();
        }
        6 => {
            PrintMonTrainerMemo();
        }
        7 => {
            DestroyTask(taskId);
            return;
        }
        _ => {}
    }
    *data += 1;
}
pub(crate) unsafe extern "C" fn PrintMonOTName() {
    let mut x: i32 = 0;
    let mut windowId: i32 = 0;
    if InBattleFactory() != TRUE && InSlateportBattleTent() != TRUE {
        windowId = AddWindowFromTemplateList(
            sPageInfoTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_INFO_ORIGINAL_TRAINER,
        ) as i32;
        PrintTextOnWindow(
            windowId as u8,
            gText_OTSlash.as_ptr().cast_mut(),
            0,
            1,
            0,
            1,
        );
        x = GetStringWidth(FONT_NORMAL, gText_OTSlash.as_ptr().cast_mut(), 0);
        if (*sMonSummaryScreen).summary.OTGender == 0 {
            PrintTextOnWindow(
                windowId as u8,
                (*sMonSummaryScreen).summary.OTName.as_mut_ptr(),
                x as u8,
                1,
                0,
                5,
            );
        } else {
            PrintTextOnWindow(
                windowId as u8,
                (*sMonSummaryScreen).summary.OTName.as_mut_ptr(),
                x as u8,
                1,
                0,
                6,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintMonOTID() {
    let mut xPos: i32 = 0;
    if InBattleFactory() != TRUE && InSlateportBattleTent() != TRUE {
        ConvertIntToDecimalStringN(
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gText_IDNumber2.as_ptr().cast_mut(),
            ),
            (*sMonSummaryScreen).summary.OTID as u16 as i32,
            STR_CONV_MODE_LEADING_ZEROS,
            5,
        );
        xPos = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar1.as_mut_ptr(), 56);
        PrintTextOnWindow(
            AddWindowFromTemplateList(sPageInfoTemplate.as_ptr().cast_mut(), 1),
            gStringVar1.as_mut_ptr(),
            xPos as u8,
            1,
            0,
            1,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMonAbilityName() {
    let mut ability: u8 = GetAbilityBySpecies(
        (*sMonSummaryScreen).summary.species,
        (*sMonSummaryScreen).summary.abilityNum,
    );
    PrintTextOnWindow(
        AddWindowFromTemplateList(
            sPageInfoTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_INFO_ABILITY,
        ),
        gAbilityNames[ability].as_ptr().cast_mut(),
        0,
        1,
        0,
        1,
    );
}
pub(crate) unsafe extern "C" fn PrintMonAbilityDescription() {
    let mut ability: u8 = GetAbilityBySpecies(
        (*sMonSummaryScreen).summary.species,
        (*sMonSummaryScreen).summary.abilityNum,
    );
    PrintTextOnWindow(
        AddWindowFromTemplateList(
            sPageInfoTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_INFO_ABILITY,
        ),
        gAbilityDescriptionPointers[ability],
        0,
        17,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn BufferMonTrainerMemo() {
    let mut sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut text: *mut u8 = null_mut();
    DynamicPlaceholderTextUtil_Reset();
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, sMemoNatureTextColor.as_ptr().cast_mut());
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(1, sMemoMiscTextColor.as_ptr().cast_mut());
    BufferNatureString();
    if InBattleFactory() == TRUE || InSlateportBattleTent() == TRUE || IsInGamePartnerMon() == TRUE
    {
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_XNature.as_ptr().cast_mut(),
        );
    } else {
        let mut metLevelString: *mut u8 = Alloc(32) as *mut u8;
        let mut metLocationString: *mut u8 = Alloc(32) as *mut u8;
        GetMetLevelString(metLevelString);
        if (*sum).metLocation < MAPSEC_NONE as u8 {
            GetMapNameHandleAquaHideout(metLocationString, (*sum).metLocation as u16);
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(4, metLocationString);
        }
        if DoesMonOTMatchOwner() == TRUE {
            if (*sum).metLevel == 0 {
                text = if (*sum).metLocation >= MAPSEC_NONE as u8 {
                    gText_XNatureHatchedSomewhereAt.as_ptr().cast_mut()
                } else {
                    gText_XNatureHatchedAtYZ.as_ptr().cast_mut()
                };
            } else {
                text = if (*sum).metLocation >= MAPSEC_NONE as u8 {
                    gText_XNatureMetSomewhereAt.as_ptr().cast_mut()
                } else {
                    gText_XNatureMetAtYZ.as_ptr().cast_mut()
                };
            }
        } else if (*sum).metLocation == METLOC_FATEFUL_ENCOUNTER {
            text = gText_XNatureFatefulEncounter.as_ptr().cast_mut();
        } else if (*sum).metLocation != METLOC_IN_GAME_TRADE && DidMonComeFromGBAGames() != 0 {
            text = if (*sum).metLocation >= MAPSEC_NONE as u8 {
                gText_XNatureObtainedInTrade.as_ptr().cast_mut()
            } else {
                gText_XNatureProbablyMetAt.as_ptr().cast_mut()
            };
        } else {
            text = gText_XNatureObtainedInTrade.as_ptr().cast_mut();
        }
        DynamicPlaceholderTextUtil_ExpandPlaceholders(gStringVar4.as_mut_ptr(), text);
        Free(metLevelString as *mut c_void);
        Free(metLocationString as *mut c_void);
    }
}
pub(crate) unsafe extern "C" fn PrintMonTrainerMemo() {
    PrintTextOnWindow(
        AddWindowFromTemplateList(
            sPageInfoTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_INFO_MEMO,
        ),
        gStringVar4.as_mut_ptr(),
        0,
        1,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn BufferNatureString() {
    let mut sumStruct: *mut PokemonSummaryScreenData = sMonSummaryScreen;
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(
        2,
        gNatureNamePointers[(*sumStruct).summary.nature],
    );
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(5, gText_EmptyString5.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn GetMetLevelString(output: *mut u8) {
    let mut level: u8 = (*sMonSummaryScreen).summary.metLevel;
    if level == 0 {
        level = EGG_HATCH_LEVEL;
    }
    ConvertIntToDecimalStringN(output, level as i32, STR_CONV_MODE_LEFT_ALIGN, 3);
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(3, output);
}
pub(crate) unsafe extern "C" fn DoesMonOTMatchOwner() -> u8 {
    let mut sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut trainerId: u32 = 0;
    let mut gender: u8 = 0;
    if (*sMonSummaryScreen).monList.mons == gEnemyParty.as_mut_ptr() {
        let mut multiID: u8 = GetMultiplayerId() ^ 1;
        trainerId = gLinkPlayers[multiID].trainerId & 0xFFFF;
        gender = gLinkPlayers[multiID].gender;
        StringCopy(
            gStringVar1.as_mut_ptr(),
            gLinkPlayers[multiID].name.as_mut_ptr(),
        );
    } else {
        trainerId = GetPlayerIDAsU32() & 0xFFFF;
        gender = (*gSaveBlock2Ptr).playerGender;
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
    }
    if gender != (*sum).OTGender
        || trainerId != (*sum).OTID & 0xFFFF
        || StringCompareWithoutExtCtrlCodes(gStringVar1.as_mut_ptr(), (*sum).OTName.as_mut_ptr())
            != 0
    {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn DidMonComeFromGBAGames() -> u8 {
    let mut sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*sum).metGame > 0 && (*sum).metGame <= VERSION_LEAF_GREEN as u8 {
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DidMonComeFromRSE() -> u8 {
    let mut sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*sum).metGame > 0 && (*sum).metGame <= VERSION_EMERALD {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsInGamePartnerMon() -> u8 {
    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 && gMain.inBattle() != 0 {
        if (*sMonSummaryScreen).curMonIndex == 1
            || (*sMonSummaryScreen).curMonIndex == 4
            || (*sMonSummaryScreen).curMonIndex == 5
        {
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn PrintEggOTName() {
    let mut windowId: u32 = AddWindowFromTemplateList(
        sPageInfoTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_INFO_ORIGINAL_TRAINER,
    ) as u32;
    let mut width: u32 = GetStringWidth(FONT_NORMAL, gText_OTSlash.as_ptr().cast_mut(), 0) as u32;
    PrintTextOnWindow(
        windowId as u8,
        gText_OTSlash.as_ptr().cast_mut(),
        0,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        windowId as u8,
        gText_FiveMarks.as_ptr().cast_mut(),
        width as u8,
        1,
        0,
        1,
    );
}
pub(crate) unsafe extern "C" fn PrintEggOTID() {
    let mut x: i32 = 0;
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gText_IDNumber2.as_ptr().cast_mut(),
    );
    StringAppend(
        gStringVar1.as_mut_ptr(),
        gText_FiveMarks.as_ptr().cast_mut(),
    );
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar1.as_mut_ptr(), 56);
    PrintTextOnWindow(
        AddWindowFromTemplateList(sPageInfoTemplate.as_ptr().cast_mut(), 1),
        gStringVar1.as_mut_ptr(),
        x as u8,
        1,
        0,
        1,
    );
}
pub(crate) unsafe extern "C" fn PrintEggState() {
    let mut text: *mut u8 = null_mut();
    let mut sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*sMonSummaryScreen).summary.sanity == TRUE {
        text = gText_EggWillTakeALongTime.as_ptr().cast_mut();
    } else if (*sum).friendship <= 5 {
        text = gText_EggAboutToHatch.as_ptr().cast_mut();
    } else if (*sum).friendship <= 10 {
        text = gText_EggWillHatchSoon.as_ptr().cast_mut();
    } else if (*sum).friendship <= 40 {
        text = gText_EggWillTakeSomeTime.as_ptr().cast_mut();
    } else {
        text = gText_EggWillTakeALongTime.as_ptr().cast_mut();
    }
    PrintTextOnWindow(
        AddWindowFromTemplateList(
            sPageInfoTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_INFO_ABILITY,
        ),
        text,
        0,
        1,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn PrintEggMemo() {
    let mut text: *mut u8 = null_mut();
    let mut sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*sMonSummaryScreen).summary.sanity != 1 {
        if (*sum).metLocation == METLOC_FATEFUL_ENCOUNTER {
            text = gText_PeculiarEggNicePlace.as_ptr().cast_mut();
        } else if DidMonComeFromGBAGames() == FALSE || DoesMonOTMatchOwner() == FALSE {
            text = gText_PeculiarEggTrade.as_ptr().cast_mut();
        } else if (*sum).metLocation == METLOC_SPECIAL_EGG {
            text = if DidMonComeFromRSE() == TRUE {
                gText_EggFromHotSprings.as_ptr().cast_mut()
            } else {
                gText_EggFromTraveler.as_ptr().cast_mut()
            };
        } else {
            text = gText_OddEggFoundByCouple.as_ptr().cast_mut();
        }
    } else {
        text = gText_OddEggFoundByCouple.as_ptr().cast_mut();
    }
    PrintTextOnWindow(
        AddWindowFromTemplateList(
            sPageInfoTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_INFO_MEMO,
        ),
        text,
        0,
        1,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn PrintSkillsPageText() {
    PrintHeldItemName();
    PrintRibbonCount();
    BufferLeftColumnStats();
    PrintLeftColumnStats();
    BufferRightColumnStats();
    PrintRightColumnStats();
    PrintExpPointsNextLevel();
}
pub(crate) unsafe extern "C" fn Task_PrintSkillsPage(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        1 => {
            PrintHeldItemName();
        }
        2 => {
            PrintRibbonCount();
        }
        3 => {
            BufferLeftColumnStats();
        }
        4 => {
            PrintLeftColumnStats();
        }
        5 => {
            BufferRightColumnStats();
        }
        6 => {
            PrintRightColumnStats();
        }
        7 => {
            PrintExpPointsNextLevel();
        }
        8 => {
            DestroyTask(taskId);
            return;
        }
        _ => {}
    }
    *data += 1;
}
pub(crate) unsafe extern "C" fn PrintHeldItemName() {
    let mut text: *mut u8 = null_mut();
    let mut x: i32 = 0;
    if (*sMonSummaryScreen).summary.item == ITEM_ENIGMA_BERRY
        && IsMultiBattle() == TRUE
        && ((*sMonSummaryScreen).curMonIndex == 1
            || (*sMonSummaryScreen).curMonIndex == 4
            || (*sMonSummaryScreen).curMonIndex == 5)
    {
        text = GetItemName(ITEM_ENIGMA_BERRY);
    } else if (*sMonSummaryScreen).summary.item == ITEM_NONE {
        text = gText_None.as_ptr().cast_mut();
    } else {
        CopyItemName((*sMonSummaryScreen).summary.item, gStringVar1.as_mut_ptr());
        text = gStringVar1.as_mut_ptr();
    }
    x = GetStringCenterAlignXOffset(FONT_NORMAL as i32, text, 72) + 6;
    PrintTextOnWindow(
        AddWindowFromTemplateList(sPageSkillsTemplate.as_ptr().cast_mut(), 0),
        text,
        x as u8,
        1,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn PrintRibbonCount() {
    let mut text: *mut u8 = null_mut();
    let mut x: i32 = 0;
    if (*sMonSummaryScreen).summary.ribbonCount == 0 {
        text = gText_None.as_ptr().cast_mut();
    } else {
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*sMonSummaryScreen).summary.ribbonCount as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            2,
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_RibbonsVar1.as_ptr().cast_mut(),
        );
        text = gStringVar4.as_mut_ptr();
    }
    x = GetStringCenterAlignXOffset(FONT_NORMAL as i32, text, 70) + 6;
    PrintTextOnWindow(
        AddWindowFromTemplateList(sPageSkillsTemplate.as_ptr().cast_mut(), 1),
        text,
        x as u8,
        1,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn BufferLeftColumnStats() {
    let mut currentHPString: *mut u8 = Alloc(8) as *mut u8;
    let mut maxHPString: *mut u8 = Alloc(8) as *mut u8;
    let mut attackString: *mut u8 = Alloc(8) as *mut u8;
    let mut defenseString: *mut u8 = Alloc(8) as *mut u8;
    ConvertIntToDecimalStringN(
        currentHPString,
        (*sMonSummaryScreen).summary.currentHP as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    ConvertIntToDecimalStringN(
        maxHPString,
        (*sMonSummaryScreen).summary.maxHP as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    ConvertIntToDecimalStringN(
        attackString,
        (*sMonSummaryScreen).summary.atk as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        7,
    );
    ConvertIntToDecimalStringN(
        defenseString,
        (*sMonSummaryScreen).summary.def as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        7,
    );
    DynamicPlaceholderTextUtil_Reset();
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, currentHPString);
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(1, maxHPString);
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(2, attackString);
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(3, defenseString);
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        sStatsLeftColumnLayout.as_ptr().cast_mut(),
    );
    Free(currentHPString as *mut c_void);
    Free(maxHPString as *mut c_void);
    Free(attackString as *mut c_void);
    Free(defenseString as *mut c_void);
}
pub(crate) unsafe extern "C" fn PrintLeftColumnStats() {
    PrintTextOnWindow(
        AddWindowFromTemplateList(
            sPageSkillsTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_SKILLS_STATS_LEFT,
        ),
        gStringVar4.as_mut_ptr(),
        4,
        1,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn BufferRightColumnStats() {
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        (*sMonSummaryScreen).summary.spatk as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        (*sMonSummaryScreen).summary.spdef as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    ConvertIntToDecimalStringN(
        gStringVar3.as_mut_ptr(),
        (*sMonSummaryScreen).summary.speed as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    DynamicPlaceholderTextUtil_Reset();
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, gStringVar1.as_mut_ptr());
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(1, gStringVar2.as_mut_ptr());
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(2, gStringVar3.as_mut_ptr());
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        sStatsRightColumnLayout.as_ptr().cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn PrintRightColumnStats() {
    PrintTextOnWindow(
        AddWindowFromTemplateList(
            sPageSkillsTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_SKILLS_STATS_RIGHT,
        ),
        gStringVar4.as_mut_ptr(),
        2,
        1,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn PrintExpPointsNextLevel() {
    let mut sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut windowId: u8 =
        AddWindowFromTemplateList(sPageSkillsTemplate.as_ptr().cast_mut(), PSS_DATA_WINDOW_EXP);
    let mut x: i32 = 0;
    let mut expToNextLevel: u32 = 0;
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        (*sum).exp as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        7,
    );
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar1.as_mut_ptr(), 42) + 2;
    PrintTextOnWindow(windowId, gStringVar1.as_mut_ptr(), x as u8, 1, 0, 0);
    if (*sum).level < MAX_LEVEL as u8 {
        expToNextLevel = gExperienceTables[gSpeciesInfo[(*sum).species].growthRate]
            [(*sum).level as i32 + 1]
            - (*sum).exp;
    } else {
        expToNextLevel = 0;
    }
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        expToNextLevel as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        6,
    );
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar1.as_mut_ptr(), 42) + 2;
    PrintTextOnWindow(windowId, gStringVar1.as_mut_ptr(), x as u8, 17, 0, 0);
}
pub(crate) unsafe extern "C" fn PrintBattleMoves() {
    PrintMoveNameAndPP(0);
    PrintMoveNameAndPP(1);
    PrintMoveNameAndPP(2);
    PrintMoveNameAndPP(3);
    if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE {
        PrintNewMoveDetailsOrCancelText();
        if (*sMonSummaryScreen).firstMoveIndex == MAX_MON_MOVES as u8 {
            if (*sMonSummaryScreen).newMove != MOVE_NONE {
                PrintMoveDetails((*sMonSummaryScreen).newMove);
            }
        } else {
            PrintMoveDetails(
                (*sMonSummaryScreen).summary.moves[(*sMonSummaryScreen).firstMoveIndex],
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PrintBattleMoves(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        1 => {
            PrintMoveNameAndPP(0);
        }
        2 => {
            PrintMoveNameAndPP(1);
        }
        3 => {
            PrintMoveNameAndPP(2);
        }
        4 => {
            PrintMoveNameAndPP(3);
        }
        5 => {
            if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE {
                PrintNewMoveDetailsOrCancelText();
            }
        }
        6 => {
            if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE {
                if (*sMonSummaryScreen).firstMoveIndex == MAX_MON_MOVES as u8 {
                    *data.at(1) = (*sMonSummaryScreen).newMove as i16;
                } else {
                    *data.at(1) = (*sMonSummaryScreen).summary.moves
                        [(*sMonSummaryScreen).firstMoveIndex]
                        as i16;
                }
            }
        }
        7 => {
            if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE {
                if (*sMonSummaryScreen).newMove != MOVE_NONE
                    || (*sMonSummaryScreen).firstMoveIndex != MAX_MON_MOVES as u8
                {
                    PrintMoveDetails(*data.at(1) as u16);
                }
            }
        }
        8 => {
            DestroyTask(taskId);
            return;
        }
        _ => {}
    }
    *data += 1;
}
pub(crate) unsafe extern "C" fn PrintMoveNameAndPP(moveIndex: u8) {
    let mut pp: u8 = 0;
    let mut ppState: i32 = 0;
    let mut x: i32 = 0;
    let mut text: *mut u8 = null_mut();
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut moveNameWindowId: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_NAMES,
    );
    let mut ppValueWindowId: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_PP,
    );
    let mut r#move: u16 = (*summary).moves[moveIndex];
    if r#move != 0 {
        pp = CalculatePPWithBonus(r#move, (*summary).ppBonuses, moveIndex);
        PrintTextOnWindow(
            moveNameWindowId,
            gMoveNames[r#move].as_ptr().cast_mut(),
            0,
            moveIndex * 16 + 1,
            0,
            1,
        );
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*summary).pp[moveIndex] as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            2,
        );
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            pp as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            2,
        );
        DynamicPlaceholderTextUtil_Reset();
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, gStringVar1.as_mut_ptr());
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(1, gStringVar2.as_mut_ptr());
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            sMovesPPLayout.as_ptr().cast_mut(),
        );
        text = gStringVar4.as_mut_ptr();
        ppState = GetCurrentPPToMaxPPState((*summary).pp[moveIndex], pp) as i32 + 9;
        x = GetStringRightAlignXOffset(FONT_NORMAL as i32, text, 44);
    } else {
        PrintTextOnWindow(
            moveNameWindowId,
            gText_OneDash.as_ptr().cast_mut(),
            0,
            moveIndex * 16 + 1,
            0,
            1,
        );
        text = gText_TwoDashes.as_ptr().cast_mut();
        ppState = 12;
        x = GetStringCenterAlignXOffset(FONT_NORMAL as i32, text, 44);
    }
    PrintTextOnWindow(
        ppValueWindowId,
        text,
        x as u8,
        moveIndex * 16 + 1,
        0,
        ppState as u8,
    );
}
pub(crate) unsafe extern "C" fn PrintMovePowerAndAccuracy(moveIndex: u16) {
    let mut text: *mut u8 = null_mut();
    if moveIndex != 0 {
        FillWindowPixelRect(PSS_LABEL_WINDOW_MOVES_POWER_ACC, 0, 53, 0, 19, 32);
        if gBattleMoves[moveIndex].power < 2 {
            text = gText_ThreeDashes.as_ptr().cast_mut();
        } else {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                gBattleMoves[moveIndex].power as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                3,
            );
            text = gStringVar1.as_mut_ptr();
        }
        PrintTextOnWindow(PSS_LABEL_WINDOW_MOVES_POWER_ACC, text, 53, 1, 0, 0);
        if gBattleMoves[moveIndex].accuracy == 0 {
            text = gText_ThreeDashes.as_ptr().cast_mut();
        } else {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                gBattleMoves[moveIndex].accuracy as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                3,
            );
            text = gStringVar1.as_mut_ptr();
        }
        PrintTextOnWindow(PSS_LABEL_WINDOW_MOVES_POWER_ACC, text, 53, 17, 0, 0);
    }
}
pub(crate) unsafe extern "C" fn PrintContestMoves() {
    PrintMoveNameAndPP(0);
    PrintMoveNameAndPP(1);
    PrintMoveNameAndPP(2);
    PrintMoveNameAndPP(3);
    if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE {
        PrintNewMoveDetailsOrCancelText();
        PrintContestMoveDescription((*sMonSummaryScreen).firstMoveIndex);
    }
}
pub(crate) unsafe extern "C" fn Task_PrintContestMoves(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        1 => {
            PrintMoveNameAndPP(0);
        }
        2 => {
            PrintMoveNameAndPP(1);
        }
        3 => {
            PrintMoveNameAndPP(2);
        }
        4 => {
            PrintMoveNameAndPP(3);
        }
        5 => {
            if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE {
                PrintNewMoveDetailsOrCancelText();
            }
        }
        6 => {
            if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE {
                if (*sMonSummaryScreen).newMove != MOVE_NONE
                    || (*sMonSummaryScreen).firstMoveIndex != MAX_MON_MOVES as u8
                {
                    PrintContestMoveDescription((*sMonSummaryScreen).firstMoveIndex);
                }
            }
        }
        7 => {
            DestroyTask(taskId);
            return;
        }
        _ => {}
    }
    *data += 1;
}
pub(crate) unsafe extern "C" fn PrintContestMoveDescription(moveSlot: u8) {
    let mut r#move: u16 = 0;
    if moveSlot == MAX_MON_MOVES as u8 {
        r#move = (*sMonSummaryScreen).newMove;
    } else {
        r#move = (*sMonSummaryScreen).summary.moves[moveSlot];
    }
    if r#move != MOVE_NONE {
        let mut windowId: u8 = AddWindowFromTemplateList(
            sPageMovesTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_MOVE_DESCRIPTION,
        );
        PrintTextOnWindow(
            windowId,
            gContestEffectDescriptionPointers[gContestMoves[r#move].effect],
            6,
            1,
            0,
            0,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMoveDetails(r#move: u16) {
    let mut windowId: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_DESCRIPTION,
    );
    FillWindowPixelBuffer(windowId, 0);
    if r#move != MOVE_NONE {
        if (*sMonSummaryScreen).currPageIndex == PSS_PAGE_BATTLE_MOVES {
            PrintMovePowerAndAccuracy(r#move);
            PrintTextOnWindow(
                windowId,
                gMoveDescriptionPointers[r#move as i32 - 1],
                6,
                1,
                0,
                0,
            );
        } else {
            PrintTextOnWindow(
                windowId,
                gContestEffectDescriptionPointers[gContestMoves[r#move].effect],
                6,
                1,
                0,
                0,
            );
        }
        PutWindowTilemap(windowId);
    } else {
        ClearWindowTilemap(windowId);
    }
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn PrintNewMoveDetailsOrCancelText() {
    let mut windowId1: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_NAMES,
    );
    let mut windowId2: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_PP,
    );
    if (*sMonSummaryScreen).newMove == MOVE_NONE {
        PrintTextOnWindow(windowId1, gText_Cancel.as_ptr().cast_mut(), 0, 65, 0, 1);
    } else {
        let mut r#move: u16 = (*sMonSummaryScreen).newMove;
        if (*sMonSummaryScreen).currPageIndex == PSS_PAGE_BATTLE_MOVES {
            PrintTextOnWindow(
                windowId1,
                gMoveNames[r#move].as_ptr().cast_mut(),
                0,
                65,
                0,
                6,
            );
        } else {
            PrintTextOnWindow(
                windowId1,
                gMoveNames[r#move].as_ptr().cast_mut(),
                0,
                65,
                0,
                5,
            );
        }
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            gBattleMoves[r#move].pp as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            2,
        );
        DynamicPlaceholderTextUtil_Reset();
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, gStringVar1.as_mut_ptr());
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(1, gStringVar1.as_mut_ptr());
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            sMovesPPLayout.as_ptr().cast_mut(),
        );
        PrintTextOnWindow(
            windowId2,
            gStringVar4.as_mut_ptr(),
            GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 44) as u8,
            65,
            0,
            12,
        );
    }
}
pub(crate) unsafe extern "C" fn AddAndFillMoveNamesWindow() {
    let mut windowId: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_NAMES,
    );
    FillWindowPixelRect(windowId, 0, 0, 66, 72, 16);
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn SwapMovesNamesPP(moveIndex1: u8, moveIndex2: u8) {
    let mut windowId1: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_NAMES,
    );
    let mut windowId2: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_PP,
    );
    FillWindowPixelRect(windowId1, 0, 0, moveIndex1 as u16 * 16, 72, 16);
    FillWindowPixelRect(windowId1, 0, 0, moveIndex2 as u16 * 16, 72, 16);
    FillWindowPixelRect(windowId2, 0, 0, moveIndex1 as u16 * 16, 48, 16);
    FillWindowPixelRect(windowId2, 0, 0, moveIndex2 as u16 * 16, 48, 16);
    PrintMoveNameAndPP(moveIndex1);
    PrintMoveNameAndPP(moveIndex2);
}
pub(crate) unsafe extern "C" fn PrintHMMovesCantBeForgotten() {
    let mut windowId: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_DESCRIPTION,
    );
    FillWindowPixelBuffer(windowId, 0);
    PrintTextOnWindow(
        windowId,
        gText_HMMovesCantBeForgotten2.as_ptr().cast_mut(),
        6,
        1,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn ResetSpriteIds() {
    let mut i: u8 = 0;
    i = 0;
    while i < 28 {
        (*sMonSummaryScreen).spriteIds[i] = SPRITE_NONE;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DestroySpriteInArray(spriteArrayId: u8) {
    if (*sMonSummaryScreen).spriteIds[spriteArrayId] != SPRITE_NONE {
        DestroySprite(&raw mut gSprites[(*sMonSummaryScreen).spriteIds[spriteArrayId]]);
        (*sMonSummaryScreen).spriteIds[spriteArrayId] = SPRITE_NONE;
    }
}
pub(crate) unsafe extern "C" fn SetSpriteInvisibility(spriteArrayId: u8, invisible: u8) {
    gSprites[(*sMonSummaryScreen).spriteIds[spriteArrayId]].set_invisible(invisible as u16);
}
pub(crate) unsafe extern "C" fn HidePageSpecificSprites() {
    let mut i: u8 = 0;
    i = SPRITE_ARR_ID_TYPE;
    while i < 28 {
        if (*sMonSummaryScreen).spriteIds[i] != SPRITE_NONE {
            SetSpriteInvisibility(i, TRUE);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetTypeIcons() {
    match (*sMonSummaryScreen).currPageIndex {
        PSS_PAGE_INFO => {
            SetMonTypeIcons();
        }
        PSS_PAGE_BATTLE_MOVES => {
            SetMoveTypeIcons();
            SetNewMoveTypeIcon();
        }
        PSS_PAGE_CONTEST_MOVES => {
            SetContestMoveTypeIcons();
            SetNewMoveTypeIcon();
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateMoveTypeIcons() {
    let mut i: u8 = 0;
    i = SPRITE_ARR_ID_TYPE;
    while i < 8 {
        if (*sMonSummaryScreen).spriteIds[i] == SPRITE_NONE {
            (*sMonSummaryScreen).spriteIds[i] =
                CreateSprite((&raw const *sSpriteTemplate_MoveTypes).cast_mut(), 0, 0, 2);
        }
        SetSpriteInvisibility(i, TRUE);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetTypeSpritePosAndPal(
    typeId: u8,
    x: u8,
    y: u8,
    spriteArrayId: u8,
) {
    let mut sprite: *mut Sprite = &raw mut gSprites[(*sMonSummaryScreen).spriteIds[spriteArrayId]];
    StartSpriteAnim(sprite, typeId);
    (*sprite)
        .oam
        .set_paletteNum(sMoveTypeToOamPaletteNum[typeId] as u16);
    (*sprite).x = x as i16 + 16;
    (*sprite).y = y as i16 + 8;
    SetSpriteInvisibility(spriteArrayId, FALSE);
}
pub(crate) unsafe extern "C" fn SetMonTypeIcons() {
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*summary).isEgg != 0 {
        SetTypeSpritePosAndPal(TYPE_MYSTERY, 120, 48, SPRITE_ARR_ID_TYPE);
        SetSpriteInvisibility(4, 1);
    } else {
        SetTypeSpritePosAndPal(
            gSpeciesInfo[(*summary).species].types[0],
            120,
            48,
            SPRITE_ARR_ID_TYPE,
        );
        if gSpeciesInfo[(*summary).species].types[0] != gSpeciesInfo[(*summary).species].types[1] {
            SetTypeSpritePosAndPal(gSpeciesInfo[(*summary).species].types[1], 160, 48, 4);
            SetSpriteInvisibility(4, FALSE);
        } else {
            SetSpriteInvisibility(4, 1);
        }
    }
}
pub(crate) unsafe extern "C" fn SetMoveTypeIcons() {
    let mut i: u8 = 0;
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    i = 0;
    while i < MAX_MON_MOVES as u8 {
        if (*summary).moves[i] != MOVE_NONE {
            SetTypeSpritePosAndPal(
                gBattleMoves[(*summary).moves[i]].r#type,
                85,
                32 + i * 16,
                i + SPRITE_ARR_ID_TYPE,
            );
        } else {
            SetSpriteInvisibility(i + SPRITE_ARR_ID_TYPE, TRUE);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetContestMoveTypeIcons() {
    let mut i: u8 = 0;
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    i = 0;
    while i < MAX_MON_MOVES as u8 {
        if (*summary).moves[i] != MOVE_NONE {
            SetTypeSpritePosAndPal(
                NUMBER_OF_MON_TYPES + gContestMoves[(*summary).moves[i]].contestCategory(),
                85,
                32 + i * 16,
                i + SPRITE_ARR_ID_TYPE,
            );
        } else {
            SetSpriteInvisibility(i + SPRITE_ARR_ID_TYPE, TRUE);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetNewMoveTypeIcon() {
    if (*sMonSummaryScreen).newMove == MOVE_NONE {
        SetSpriteInvisibility(7, TRUE);
    } else {
        if (*sMonSummaryScreen).currPageIndex == PSS_PAGE_BATTLE_MOVES {
            SetTypeSpritePosAndPal(gBattleMoves[(*sMonSummaryScreen).newMove].r#type, 85, 96, 7);
        } else {
            SetTypeSpritePosAndPal(
                NUMBER_OF_MON_TYPES + gContestMoves[(*sMonSummaryScreen).newMove].contestCategory(),
                85,
                96,
                7,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SwapMovesTypeSprites(moveIndex1: u8, moveIndex2: u8) {
    let mut sprite1: *mut Sprite = &raw mut gSprites
        [(*sMonSummaryScreen).spriteIds[moveIndex1 as i32 + SPRITE_ARR_ID_TYPE as i32]];
    let mut sprite2: *mut Sprite = &raw mut gSprites
        [(*sMonSummaryScreen).spriteIds[moveIndex2 as i32 + SPRITE_ARR_ID_TYPE as i32]];
    let mut temp: u8 = (*sprite1).animNum;
    (*sprite1).animNum = (*sprite2).animNum;
    (*sprite2).animNum = temp;
    temp = (*sprite1).oam.paletteNum() as u8;
    (*sprite1).oam.set_paletteNum((*sprite2).oam.paletteNum());
    (*sprite2).oam.set_paletteNum(temp as u16);
    (*sprite1).set_animBeginning(TRUE as u16);
    (*sprite1).set_animEnded(FALSE as u16);
    (*sprite2).set_animBeginning(TRUE as u16);
    (*sprite2).set_animEnded(FALSE as u16);
}
pub(crate) unsafe extern "C" fn LoadMonGfxAndSprite(mon: *mut Pokemon, state: *mut i16) -> u8 {
    let mut pal: *mut CompressedSpritePalette = null_mut();
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    match *state {
        0 => {
            if gMain.inBattle() != 0 {
                if ShouldIgnoreDeoxysForm(3, (*sMonSummaryScreen).curMonIndex) != 0 {
                    HandleLoadSpecialPokePic_DontHandleDeoxys(
                        (&raw const gMonFrontPicTable[(*summary).species2]).cast_mut(),
                        (*gMonSpritesGfxPtr).sprites.ptr[1],
                        (*summary).species2 as i32,
                        (*summary).pid,
                    );
                } else {
                    HandleLoadSpecialPokePic_2(
                        (&raw const gMonFrontPicTable[(*summary).species2]).cast_mut(),
                        (*gMonSpritesGfxPtr).sprites.ptr[1],
                        (*summary).species2 as i32,
                        (*summary).pid,
                    );
                }
            } else {
                if !gMonSpritesGfxPtr.is_null() {
                    if (*sMonSummaryScreen).monList.mons == gPlayerParty.as_mut_ptr()
                        || (*sMonSummaryScreen).mode == SUMMARY_MODE_BOX
                        || (*sMonSummaryScreen).handleDeoxys == TRUE
                    {
                        HandleLoadSpecialPokePic_2(
                            (&raw const gMonFrontPicTable[(*summary).species2]).cast_mut(),
                            (*gMonSpritesGfxPtr).sprites.ptr[1],
                            (*summary).species2 as i32,
                            (*summary).pid,
                        );
                    } else {
                        HandleLoadSpecialPokePic_DontHandleDeoxys(
                            (&raw const gMonFrontPicTable[(*summary).species2]).cast_mut(),
                            (*gMonSpritesGfxPtr).sprites.ptr[1],
                            (*summary).species2 as i32,
                            (*summary).pid,
                        );
                    }
                } else {
                    if (*sMonSummaryScreen).monList.mons == gPlayerParty.as_mut_ptr()
                        || (*sMonSummaryScreen).mode == SUMMARY_MODE_BOX
                        || (*sMonSummaryScreen).handleDeoxys == TRUE
                    {
                        HandleLoadSpecialPokePic_2(
                            (&raw const gMonFrontPicTable[(*summary).species2]).cast_mut(),
                            MonSpritesGfxManager_GetSpritePtr(
                                MON_SPR_GFX_MANAGER_A,
                                B_POSITION_OPPONENT_LEFT,
                            ) as *mut c_void,
                            (*summary).species2 as i32,
                            (*summary).pid,
                        );
                    } else {
                        HandleLoadSpecialPokePic_DontHandleDeoxys(
                            (&raw const gMonFrontPicTable[(*summary).species2]).cast_mut(),
                            MonSpritesGfxManager_GetSpritePtr(
                                MON_SPR_GFX_MANAGER_A,
                                B_POSITION_OPPONENT_LEFT,
                            ) as *mut c_void,
                            (*summary).species2 as i32,
                            (*summary).pid,
                        );
                    }
                }
            }
            *state += 1;
            return 0xFF;
        }
        1 => {
            pal = GetMonSpritePalStructFromOtIdPersonality(
                (*summary).species2,
                (*summary).OTID,
                (*summary).pid,
            );
            LoadCompressedSpritePalette(pal);
            SetMultiuseSpriteTemplateToPokemon((*pal).tag, B_POSITION_OPPONENT_LEFT);
            *state += 1;
            return 0xFF;
        }
        _ => {
            return CreateMonSprite(mon);
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn PlayMonCry() {
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*summary).isEgg == 0 {
        if ShouldPlayNormalMonCry(&raw mut (*sMonSummaryScreen).currentMon) == TRUE as u32 {
            PlayCry_ByMode((*summary).species2, 0, 0);
        } else {
            PlayCry_ByMode((*summary).species2, 0, CRY_MODE_WEAK);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMonSprite(unused: *mut Pokemon) -> u8 {
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut spriteId: u8 = CreateSprite(&raw mut gMultiuseSpriteTemplate, 40, 64, 5);
    FreeSpriteOamMatrix(&raw mut gSprites[spriteId]);
    gSprites[spriteId].data[0] = (*summary).species2 as i16;
    gSprites[spriteId].data[2] = 0;
    gSprites[spriteId].callback = Some(SpriteCB_Pokemon);
    gSprites[spriteId].oam.set_priority(0);
    if IsMonSpriteNotFlipped((*summary).species2) == 0 {
        gSprites[spriteId].set_hFlip(TRUE as u16);
    } else {
        gSprites[spriteId].set_hFlip(FALSE as u16);
    }
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_Pokemon(sprite: *mut Sprite) {
    let mut summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if gPaletteFade.active() == 0 && (*sprite).data[2] != 1 {
        (*sprite).data[1] = IsMonSpriteNotFlipped((*sprite).data[0] as u16) as i16;
        PlayMonCry();
        PokemonSummaryDoMonAnimation(sprite, (*sprite).data[0] as u16, (*summary).isEgg);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SummaryScreen_SetAnimDelayTaskId(taskId: u8) {
    sAnimDelayTaskId = taskId;
}
pub(crate) unsafe extern "C" fn SummaryScreen_DestroyAnimDelayTask() {
    if sAnimDelayTaskId != TASK_NONE {
        DestroyTask(sAnimDelayTaskId);
        sAnimDelayTaskId = TASK_NONE;
    }
}
pub(crate) unsafe extern "C" fn IsMonAnimationFinished() -> u32 {
    if gSprites[(*sMonSummaryScreen).spriteIds[0]].callback
        == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
    {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn StopPokemonAnimations() {
    let mut i: u16 = 0;
    let mut paletteIndex: u16 = 0;
    gSprites[(*sMonSummaryScreen).spriteIds[0]].set_animPaused(TRUE);
    gSprites[(*sMonSummaryScreen).spriteIds[0]].callback = Some(SpriteCallbackDummy);
    StopPokemonAnimationDelayTask();
    paletteIndex = 0x100 + gSprites[(*sMonSummaryScreen).spriteIds[0]].oam.paletteNum() * 16;
    i = 0;
    while i < 16 {
        let mut id: u16 = i + paletteIndex;
        gPlttBufferUnfaded[id] = gPlttBufferFaded[id];
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateMonMarkingsSprite(mon: *mut Pokemon) {
    let mut sprite: *mut Sprite = CreateMonMarkingAllCombosSprite(
        TAG_MON_MARKINGS,
        TAG_MON_MARKINGS,
        sMarkings_Pal.as_ptr().cast_mut(),
    );
    (*sMonSummaryScreen).markingsSprite = sprite;
    if !sprite.is_null() {
        StartSpriteAnim(sprite, GetMonData2(mon, MON_DATA_MARKINGS) as u8);
        (*(*sMonSummaryScreen).markingsSprite).x = 60;
        (*(*sMonSummaryScreen).markingsSprite).y = 26;
        (*(*sMonSummaryScreen).markingsSprite).oam.set_priority(1);
    }
}
pub(crate) unsafe extern "C" fn RemoveAndCreateMonMarkingsSprite(mon: *mut Pokemon) {
    DestroySprite((*sMonSummaryScreen).markingsSprite);
    FreeSpriteTilesByTag(TAG_MON_MARKINGS);
    CreateMonMarkingsSprite(mon);
}
pub(crate) unsafe extern "C" fn CreateCaughtBallSprite(mon: *mut Pokemon) {
    let mut ball: u8 = ItemIdToBallId(GetMonData2(mon, MON_DATA_POKEBALL) as u16);
    LoadBallGfx(ball);
    (*sMonSummaryScreen).spriteIds[1] = CreateSprite(
        (&raw const gBallSpriteTemplates[ball]).cast_mut(),
        16,
        136,
        0,
    );
    gSprites[(*sMonSummaryScreen).spriteIds[1]].callback = Some(SpriteCallbackDummy);
    gSprites[(*sMonSummaryScreen).spriteIds[1]]
        .oam
        .set_priority(3);
}
pub(crate) unsafe extern "C" fn CreateSetStatusSprite() {
    let mut spriteId: *mut u8 = &raw mut (*sMonSummaryScreen).spriteIds[2];
    let mut statusAnim: u8 = 0;
    if *spriteId == SPRITE_NONE {
        *spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_StatusCondition).cast_mut(),
            64,
            152,
            0,
        );
    }
    statusAnim = GetMonAilment(&raw mut (*sMonSummaryScreen).currentMon);
    if statusAnim != 0 {
        StartSpriteAnim(&raw mut gSprites[*spriteId], statusAnim - 1);
        SetSpriteInvisibility(SPRITE_ARR_ID_STATUS, FALSE);
    } else {
        SetSpriteInvisibility(SPRITE_ARR_ID_STATUS, TRUE);
    }
}
pub(crate) unsafe extern "C" fn CreateMoveSelectorSprites(idArrayStart: u8) {
    let mut i: u8 = 0;
    let mut spriteIds: *mut u8 = &raw mut (*sMonSummaryScreen).spriteIds[idArrayStart];
    if (*sMonSummaryScreen).currPageIndex >= PSS_PAGE_BATTLE_MOVES {
        let mut subpriority: u8 = 0;
        if idArrayStart == SPRITE_ARR_ID_MOVE_SELECTOR1 {
            subpriority = 1;
        }
        i = 0;
        while i < MOVE_SELECTOR_SPRITES_COUNT {
            *spriteIds.at(i) = CreateSprite(
                (&raw const *sMoveSelectorSpriteTemplate).cast_mut(),
                i as i16 * 16 + 89,
                40,
                subpriority,
            );
            if i == 0 {
                StartSpriteAnim(&raw mut gSprites[*spriteIds.at(i)], 4);
            } else if i == 9 {
                StartSpriteAnim(&raw mut gSprites[*spriteIds.at(i)], 5);
            } else {
                StartSpriteAnim(&raw mut gSprites[*spriteIds.at(i)], 6);
            }
            gSprites[*spriteIds.at(i)].callback = Some(SpriteCB_MoveSelector);
            gSprites[*spriteIds.at(i)].data[0] = idArrayStart as i16;
            gSprites[*spriteIds.at(i)].data[1] = 0;
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MoveSelector(sprite: *mut Sprite) {
    if (*sprite).animNum > 3 && (*sprite).animNum < 7 {
        (*sprite).data[1] = (*sprite).data[1] + 1 & 0x1F;
        if (*sprite).data[1] > 24 {
            (*sprite).set_invisible(TRUE as u16);
        } else {
            (*sprite).set_invisible(FALSE as u16);
        }
    } else {
        (*sprite).data[1] = 0;
        (*sprite).set_invisible(FALSE as u16);
    }
    if (*sprite).data[0] == SPRITE_ARR_ID_MOVE_SELECTOR1 as i16 {
        (*sprite).y2 = (*sMonSummaryScreen).firstMoveIndex as i16 * 16;
    } else {
        (*sprite).y2 = (*sMonSummaryScreen).secondMoveIndex as i16 * 16;
    }
}
pub(crate) unsafe extern "C" fn DestroyMoveSelectorSprites(firstArrayId: u8) {
    let mut i: u8 = 0;
    i = 0;
    while i < MOVE_SELECTOR_SPRITES_COUNT {
        DestroySpriteInArray(firstArrayId + i);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetMainMoveSelectorColor(mut which: u8) {
    let mut i: u8 = 0;
    let mut spriteIds: *mut u8 = &raw mut (*sMonSummaryScreen).spriteIds[8];
    which *= 3;
    i = 0;
    while i < MOVE_SELECTOR_SPRITES_COUNT {
        if i == 0 {
            StartSpriteAnim(&raw mut gSprites[*spriteIds.at(i)], which + 4);
        } else if i == 9 {
            StartSpriteAnim(&raw mut gSprites[*spriteIds.at(i)], which + 5);
        } else {
            StartSpriteAnim(&raw mut gSprites[*spriteIds.at(i)], which + 6);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn KeepMoveSelectorVisible(firstSpriteId: u8) {
    let mut i: u8 = 0;
    let mut spriteIds: *mut u8 = &raw mut (*sMonSummaryScreen).spriteIds[firstSpriteId];
    i = 0;
    while i < MOVE_SELECTOR_SPRITES_COUNT {
        gSprites[*spriteIds.at(i)].data[1] = 0;
        gSprites[*spriteIds.at(i)].set_invisible(FALSE as u16);
        i += 1;
    }
}
