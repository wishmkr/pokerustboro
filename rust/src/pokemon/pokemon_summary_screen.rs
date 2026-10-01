//! Translated from `src/pokemon_summary_screen.c` by tools/rustport/c2rs.py.
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
    clippy::manual_swap,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::battle_anim_throw::ItemIdToBallId;
use crate::battle_factory::InBattleFactory;
use crate::battle_gfx_sfx_util::ShouldPlayNormalMonCry;
use crate::battle_main::{gBattleTypeFlags, gMonSpritesGfxPtr};
use crate::battle_message::GetCurrentPPToMaxPPState;
use crate::battle_tent::InSlateportBattleTent;
use crate::bg::{ChangeBgX, ResetBgsAndClearDma3BusyFlags, SetBgAttribute, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::dynamic_placeholder_text_util::DynamicPlaceholderTextUtil_Reset;
use crate::ffi::gSpecialVar_0x8005;
use crate::gpu_regs::SetGpuReg;
use crate::item::CopyItemName;
use crate::link::{GetMultiplayerId, gLinkPlayers};
use crate::load_save::gSaveBlock2Ptr;
use crate::m4a::{gMPlayInfo_BGM, m4aMPlayVolumeControl};
use crate::menu::{
    AddTextPrinterParameterized4, ClearScheduledBgCopiesToVram, DecompressAndCopyTileDataToVram,
    DoScheduledBgTilemapCopiesToVram, FreeTempTileDataBuffersIfPossible, ResetTempTileDataBuffers,
    ScheduleBgCopyTilemapToVram, SetBgTilemapPalette,
};
use crate::menu_helpers::{
    GetLRKeysPressed, MenuHelpers_IsLinkActive, MenuHelpers_ShouldWaitForLinkRecv,
    ResetAllBgsCoordinates, ResetVramOamAndBgCntRegs, SetVBlankHBlankCallbacksToNull,
};
use crate::mon_markings::CreateMonMarkingAllCombosSprite;
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadCompressedPalette, LoadPalette, ResetPaletteFade,
    TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::party_menu::{GetMonAilment, GetMonNickname, IsMoveHm, IsMultiBattle};
use crate::pokeball::LoadBallGfx;
use crate::pokemon::{
    BoxMonToMon, CalculatePPWithBonus, CheckPartyHasHadPokerus, CheckPartyPokerus,
    CreateMonSpritesGfxManager, DestroyMonSpritesGfxManager, GetAbilityBySpecies, GetMonData2,
    GetMonData3, GetMonGender, GetMonSpritePalStructFromOtIdPersonality, GetNature, IsMonShiny,
    IsMonSpriteNotFlipped, MonSpritesGfxManager_GetSpritePtr, PokemonSummaryDoMonAnimation,
    SetMonData, SetMultiuseSpriteTemplateToPokemon, ShouldIgnoreDeoxysForm, SpeciesToPokedexNum,
    StopPokemonAnimationDelayTask, gEnemyParty, gMultiuseSpriteTemplate, gPlayerParty,
};
use crate::pokemon_storage_system::AdvanceStorageMonIndex;
use crate::region_map::GetMapNameHandleAquaHideout;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sound::{PlayCry_ByMode, PlaySE, StopCryAndClearCrySongs};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeSpriteTilesByTag, LoadOam,
    ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::string_util::ConvertInternationalString;
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::{DestroyTask, RunTasks, SwitchTaskToFollowupFunc};
use crate::task::{gTasks, task_func, task_set, task_set_func};
use crate::text::DeactivateAllTextPrinters;
use crate::tv::GetPlayerIDAsU32;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, FillWindowPixelRect,
    FreeAllWindowBuffers, PutWindowTilemap, RemoveWindow,
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
/// `BlitBitmapToWindow` with this module's view of its types.
#[inline]
unsafe fn BlitBitmapToWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16) {
    unsafe {
        crate::window::BlitBitmapToWindow(a0, a1 as _, a2, a3, a4, a5);
    }
}
/// `ConvertIntToDecimalStringN` with this module's view of its types.
#[inline]
unsafe fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8 {
    unsafe { crate::string_util::ConvertIntToDecimalStringN(a0 as _, a1, a2, a3) as *mut u8 }
}
/// `CopyToBgTilemapBufferRect_ChangePalette` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBufferRect_ChangePalette(
    a0: u8,
    a1: *mut c_void,
    a2: u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: u8,
) {
    unsafe {
        crate::bg::CopyToBgTilemapBufferRect_ChangePalette(a0, a1 as _, a2, a3, a4, a5, a6);
    }
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
/// `DestroySpriteAndFreeResources` with this module's view of its types.
#[inline]
unsafe fn DestroySpriteAndFreeResources(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySpriteAndFreeResources(a0 as _);
    }
}
/// `DynamicPlaceholderTextUtil_ExpandPlaceholders` with this module's view of its types.
#[inline]
unsafe fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe {
        crate::dynamic_placeholder_text_util::DynamicPlaceholderTextUtil_ExpandPlaceholders(
            a0 as _, a1 as _,
        ) as *mut u8
    }
}
/// `DynamicPlaceholderTextUtil_SetPlaceholderPtr` with this module's view of its types.
#[inline]
unsafe fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8) {
    unsafe {
        crate::dynamic_placeholder_text_util::DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            a0, a1 as _,
        );
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
/// `GetStringCenterAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringCenterAlignXOffset(a0, a1 as _, a2) }
}
/// `GetStringRightAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringRightAlignXOffset(a0, a1 as _, a2) }
}
/// `GetStringWidth` with this module's view of its types.
#[inline]
unsafe fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32 {
    unsafe { crate::text::GetStringWidth(a0, a1 as _, a2) }
}
/// `HandleLoadSpecialPokePic_2` with this module's view of its types.
#[inline]
unsafe fn HandleLoadSpecialPokePic_2(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
) {
    unsafe {
        crate::decompress::HandleLoadSpecialPokePic_2(a0 as _, a1 as _, a2, a3);
    }
}
/// `HandleLoadSpecialPokePic_DontHandleDeoxys` with this module's view of its types.
#[inline]
unsafe fn HandleLoadSpecialPokePic_DontHandleDeoxys(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
) {
    unsafe {
        crate::decompress::HandleLoadSpecialPokePic_DontHandleDeoxys(a0 as _, a1 as _, a2, a3);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
unsafe fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `SetBoxMonData` with this module's view of its types.
#[inline]
unsafe fn SetBoxMonData(a0: *mut BoxPokemon, a1: i32, a2: *mut c_void) {
    unsafe {
        crate::box_mon::SetBoxMonData(a0 as _, a1, a2 as _);
    }
}
/// `SetTaskFuncWithFollowupFunc` with this module's view of its types.
#[inline]
unsafe fn SetTaskFuncWithFollowupFunc(
    a0: u8,
    a1: Option<unsafe fn(u8)>,
    a2: Option<unsafe fn(u8)>,
) {
    unsafe {
        crate::task::SetTaskFuncWithFollowupFunc(
            a0,
            core::mem::transmute(a1),
            core::mem::transmute(a2),
        );
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
/// `StringAppend` with this module's view of its types.
#[inline]
unsafe fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringAppend(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCompareWithoutExtCtrlCodes` with this module's view of its types.
#[inline]
unsafe fn StringCompareWithoutExtCtrlCodes(a0: *mut u8, a1: *mut u8) -> i32 {
    unsafe { crate::string_util::StringCompareWithoutExtCtrlCodes(a0 as _, a1 as _) }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringExpandPlaceholders` with this module's view of its types.
#[inline]
unsafe fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringExpandPlaceholders(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const tScrollingSpeed: usize = 0;
const tVisibleColumns: usize = 1;
const tMove: usize = 2;
// Data tables (translate with cdata.py): sNullDescription sPoundDescription sKarateChopDescription sDoubleSlapDescription sCometPunchDescription sMegaPunchDescription sPayDayDescription sFirePunchDescription sIcePunchDescription sThunderPunchDescription sScratchDescription sViceGripDescription sGuillotineDescription sRazorWindDescription sSwordsDanceDescription sCutDescription sGustDescription sWingAttackDescription sWhirlwindDescription sFlyDescription sBindDescription sSlamDescription sVineWhipDescription sStompDescription sDoubleKickDescription sMegaKickDescription sJumpKickDescription sRollingKickDescription sSandAttackDescription sHeadbuttDescription sHornAttackDescription sFuryAttackDescription sHornDrillDescription sTackleDescription sBodySlamDescription sWrapDescription sTakeDownDescription sThrashDescription sDoubleEdgeDescription sTailWhipDescription sPoisonStingDescription sTwineedleDescription sPinMissileDescription sLeerDescription sBiteDescription sGrowlDescription sRoarDescription sSingDescription sSupersonicDescription sSonicBoomDescription sDisableDescription sAcidDescription sEmberDescription sFlamethrowerDescription sMistDescription sWaterGunDescription sHydroPumpDescription sSurfDescription sIceBeamDescription sBlizzardDescription sPsybeamDescription sBubbleBeamDescription sAuroraBeamDescription sHyperBeamDescription sPeckDescription sDrillPeckDescription sSubmissionDescription sLowKickDescription sCounterDescription sSeismicTossDescription sStrengthDescription sAbsorbDescription sMegaDrainDescription sLeechSeedDescription sGrowthDescription sRazorLeafDescription sSolarBeamDescription sPoisonPowderDescription sStunSporeDescription sSleepPowderDescription sPetalDanceDescription sStringShotDescription sDragonRageDescription sFireSpinDescription sThunderShockDescription sThunderboltDescription sThunderWaveDescription sThunderDescription sRockThrowDescription sEarthquakeDescription sFissureDescription sDigDescription sToxicDescription sConfusionDescription sPsychicDescription sHypnosisDescription sMeditateDescription sAgilityDescription sQuickAttackDescription sRageDescription sTeleportDescription sNightShadeDescription sMimicDescription sScreechDescription sDoubleTeamDescription sRecoverDescription sHardenDescription sMinimizeDescription sSmokescreenDescription sConfuseRayDescription sWithdrawDescription sDefenseCurlDescription sBarrierDescription sLightScreenDescription sHazeDescription sReflectDescription sFocusEnergyDescription sBideDescription sMetronomeDescription sMirrorMoveDescription sSelfDestructDescription sEggBombDescription sLickDescription sSmogDescription sSludgeDescription sBoneClubDescription sFireBlastDescription sWaterfallDescription sClampDescription sSwiftDescription sSkullBashDescription sSpikeCannonDescription sConstrictDescription sAmnesiaDescription sKinesisDescription sSoftBoiledDescription sHiJumpKickDescription sGlareDescription sDreamEaterDescription sPoisonGasDescription sBarrageDescription sLeechLifeDescription sLovelyKissDescription sSkyAttackDescription sTransformDescription sBubbleDescription sDizzyPunchDescription sSporeDescription sFlashDescription sPsywaveDescription sSplashDescription sAcidArmorDescription sCrabhammerDescription sExplosionDescription sFurySwipesDescription sBonemerangDescription sRestDescription sRockSlideDescription sHyperFangDescription sSharpenDescription sConversionDescription sTriAttackDescription sSuperFangDescription sSlashDescription sSubstituteDescription sStruggleDescription sSketchDescription sTripleKickDescription sThiefDescription sSpiderWebDescription sMindReaderDescription sNightmareDescription sFlameWheelDescription sSnoreDescription sCurseDescription sFlailDescription sConversion2Description sAeroblastDescription sCottonSporeDescription sReversalDescription sSpiteDescription sPowderSnowDescription sProtectDescription sMachPunchDescription sScaryFaceDescription sFaintAttackDescription sSweetKissDescription sBellyDrumDescription sSludgeBombDescription sMudSlapDescription sOctazookaDescription sSpikesDescription sZapCannonDescription sForesightDescription sDestinyBondDescription sPerishSongDescription sIcyWindDescription sDetectDescription sBoneRushDescription sLockOnDescription sOutrageDescription sSandstormDescription sGigaDrainDescription sEndureDescription sCharmDescription sRolloutDescription sFalseSwipeDescription sSwaggerDescription sMilkDrinkDescription sSparkDescription sFuryCutterDescription sSteelWingDescription sMeanLookDescription sAttractDescription sSleepTalkDescription sHealBellDescription sReturnDescription sPresentDescription sFrustrationDescription sSafeguardDescription sPainSplitDescription sSacredFireDescription sMagnitudeDescription sDynamicPunchDescription sMegahornDescription sDragonBreathDescription sBatonPassDescription sEncoreDescription sPursuitDescription sRapidSpinDescription sSweetScentDescription sIronTailDescription sMetalClawDescription sVitalThrowDescription sMorningSunDescription sSynthesisDescription sMoonlightDescription sHiddenPowerDescription sCrossChopDescription sTwisterDescription sRainDanceDescription sSunnyDayDescription sCrunchDescription sMirrorCoatDescription sPsychUpDescription sExtremeSpeedDescription sAncientPowerDescription sShadowBallDescription sFutureSightDescription sRockSmashDescription sWhirlpoolDescription sBeatUpDescription sFakeOutDescription sUproarDescription sStockpileDescription sSpitUpDescription sSwallowDescription sHeatWaveDescription sHailDescription sTormentDescription sFlatterDescription sWillOWispDescription sMementoDescription sFacadeDescription sFocusPunchDescription sSmellingSaltDescription sFollowMeDescription sNaturePowerDescription sChargeDescription sTauntDescription sHelpingHandDescription sTrickDescription sRolePlayDescription sWishDescription sAssistDescription sIngrainDescription sSuperpowerDescription sMagicCoatDescription sRecycleDescription sRevengeDescription sBrickBreakDescription sYawnDescription sKnockOffDescription sEndeavorDescription sEruptionDescription sSkillSwapDescription sImprisonDescription sRefreshDescription sGrudgeDescription sSnatchDescription sSecretPowerDescription sDiveDescription sArmThrustDescription sCamouflageDescription sTailGlowDescription sLusterPurgeDescription sMistBallDescription sFeatherDanceDescription sTeeterDanceDescription sBlazeKickDescription sMudSportDescription sIceBallDescription sNeedleArmDescription sSlackOffDescription sHyperVoiceDescription sPoisonFangDescription sCrushClawDescription sBlastBurnDescription sHydroCannonDescription sMeteorMashDescription sAstonishDescription sWeatherBallDescription sAromatherapyDescription sFakeTearsDescription sAirCutterDescription sOverheatDescription sOdorSleuthDescription sRockTombDescription sSilverWindDescription sMetalSoundDescription sGrassWhistleDescription sTickleDescription sCosmicPowerDescription sWaterSpoutDescription sSignalBeamDescription sShadowPunchDescription sExtrasensoryDescription sSkyUppercutDescription sSandTombDescription sSheerColdDescription sMuddyWaterDescription sBulletSeedDescription sAerialAceDescription sIcicleSpearDescription sIronDefenseDescription sBlockDescription sHowlDescription sDragonClawDescription sFrenzyPlantDescription sBulkUpDescription sBounceDescription sMudShotDescription sPoisonTailDescription sCovetDescription sVoltTackleDescription sMagicalLeafDescription sWaterSportDescription sCalmMindDescription sLeafBladeDescription sDragonDanceDescription sRockBlastDescription sShockWaveDescription sWaterPulseDescription sDoomDesireDescription sPsychoBoostDescription gMoveDescriptionPointers sHardyNatureName sLonelyNatureName sBraveNatureName sAdamantNatureName sNaughtyNatureName sBoldNatureName sDocileNatureName sRelaxedNatureName sImpishNatureName sLaxNatureName sTimidNatureName sHastyNatureName sSeriousNatureName sJollyNatureName sNaiveNatureName sModestNatureName sMildNatureName sQuietNatureName sBashfulNatureName sRashNatureName sCalmNatureName sGentleNatureName sSassyNatureName sCarefulNatureName sQuirkyNatureName gNatureNamePointers sBgTemplates sStatusTilemap sStatusSlidingWindow1 sStatusSlidingWindow2 sPowerAccSlidingWindow sAppealJamSlidingWindow sMultiBattleOrder sSummaryTemplate sPageInfoTemplate sPageSkillsTemplate sPageMovesTemplate sTextColors sButtons_Gfx sTextPrinterFunctions sTextPrinterTasks sMemoNatureTextColor sMemoMiscTextColor sStatsLeftColumnLayout sStatsRightColumnLayout sMovesPPLayout sOamData_MoveTypes sSpriteAnim_TypeNormal sSpriteAnim_TypeFighting sSpriteAnim_TypeFlying sSpriteAnim_TypePoison sSpriteAnim_TypeGround sSpriteAnim_TypeRock sSpriteAnim_TypeBug sSpriteAnim_TypeGhost sSpriteAnim_TypeSteel sSpriteAnim_TypeMystery sSpriteAnim_TypeFire sSpriteAnim_TypeWater sSpriteAnim_TypeGrass sSpriteAnim_TypeElectric sSpriteAnim_TypePsychic sSpriteAnim_TypeIce sSpriteAnim_TypeDragon sSpriteAnim_TypeDark sSpriteAnim_CategoryCool sSpriteAnim_CategoryBeauty sSpriteAnim_CategoryCute sSpriteAnim_CategorySmart sSpriteAnim_CategoryTough sSpriteAnimTable_MoveTypes sSpriteSheet_MoveTypes sSpriteTemplate_MoveTypes sMoveTypeToOamPaletteNum sOamData_MoveSelector sSpriteAnim_MoveSelector0 sSpriteAnim_MoveSelector1 sSpriteAnim_MoveSelector2 sSpriteAnim_MoveSelector3 sSpriteAnim_MoveSelectorLeft sSpriteAnim_MoveSelectorRight sSpriteAnim_MoveSelectorMiddle sSpriteAnim_MoveSelector7 sSpriteAnim_MoveSelector8 sSpriteAnim_MoveSelector9 sSpriteAnimTable_MoveSelector sMoveSelectorSpriteSheet sMoveSelectorSpritePal sMoveSelectorSpriteTemplate sOamData_StatusCondition sSpriteAnim_StatusPoison sSpriteAnim_StatusParalyzed sSpriteAnim_StatusSleep sSpriteAnim_StatusFrozen sSpriteAnim_StatusBurn sSpriteAnim_StatusPokerus sSpriteAnim_StatusFaint sSpriteAnimTable_StatusCondition sStatusIconsSpriteSheet sStatusIconsSpritePalette sSpriteTemplate_StatusCondition sMarkings_Pal

/// `struct PokemonSummaryScreenData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonSummaryScreenData {
    pub monList: PokemonSummaryScreenData_monList,
    pub callback: Option<unsafe fn()>,
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
static sTextPrinterFunctions: Table<CArray<Option<unsafe fn()>, 4>> =
    Table((&raw const crate::data::pokemon_summary_screen::sTextPrinterFunctions).cast());
static sTextPrinterTasks: Table<CArray<Option<unsafe fn(u8)>, 4>> =
    Table((&raw const crate::data::pokemon_summary_screen::sTextPrinterTasks).cast());

pub(crate) static mut sMonSummaryScreen: *mut PokemonSummaryScreenData = null_mut();
#[unsafe(link_section = "ewram_data")]
pub static mut gLastViewedMonIndex: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static sMoveSlotToReplace: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sAnimDelayTaskId: crate::global::Global<u8> = crate::global::Global::new(0);

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
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `GetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn GetBgTilemapBuffer(a0: u8) -> *mut c_void {
    unsafe { crate::bg::GetBgTilemapBuffer(a0) as *mut c_void }
}
/// `GetItemName` with this module's view of its types.
#[inline]
unsafe fn GetItemName(a0: u16) -> *mut u8 {
    crate::item::GetItemName(a0) as *mut u8
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub unsafe fn ShowPokemonSummaryScreen(
    mode: u8,
    mons: *mut c_void,
    monIndex: u8,
    maxMonIndex: u8,
    callback: Option<unsafe fn()>,
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
pub unsafe fn ShowSelectMovePokemonSummaryScreen(
    mons: *mut Pokemon,
    monIndex: u8,
    maxMonIndex: u8,
    callback: Option<unsafe fn()>,
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
pub unsafe fn ShowPokemonSummaryScreenHandleDeoxys(
    mode: u8,
    mons: *mut BoxPokemon,
    monIndex: u8,
    maxMonIndex: u8,
    callback: Option<unsafe fn()>,
) {
    ShowPokemonSummaryScreen(mode, mons as *mut c_void, monIndex, maxMonIndex, callback);
    (*sMonSummaryScreen).handleDeoxys = TRUE;
}
pub(crate) unsafe fn MainCB2() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlank() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn CB2_InitSummaryScreen() {
    while MenuHelpers_ShouldWaitForLinkRecv() != TRUE
        && LoadGraphics() != TRUE
        && MenuHelpers_IsLinkActive() != TRUE
    {}
}
unsafe fn LoadGraphics() -> u8 {
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
    FALSE
}
unsafe fn InitBGs() {
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
unsafe fn DecompressGraphics() -> u8 {
    match (*sMonSummaryScreen).switchCounter {
        0 => {
            ResetTempTileDataBuffers();
            DecompressAndCopyTileDataToVram(
                1,
                (&raw const (*(&raw const crate::data::graphics::gSummaryScreen_Gfx)
                    .cast::<CArray<u32, 0>>()))
                    .cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            (*sMonSummaryScreen).switchCounter += 1;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != 1 {
                LZDecompressWram(
                    (*(&raw const crate::data::graphics::gSummaryPage_Info_Tilemap)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    (*sMonSummaryScreen).bgTilemapBuffers[0][0].as_mut_ptr() as *mut c_void,
                );
                (*sMonSummaryScreen).switchCounter += 1;
            }
        }
        2 => {
            LZDecompressWram(
                (*(&raw const crate::data::graphics::gSummaryPage_InfoEgg_Tilemap)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                (*sMonSummaryScreen).bgTilemapBuffers[0][1].as_mut_ptr() as *mut c_void,
            );
            (*sMonSummaryScreen).switchCounter += 1;
        }
        3 => {
            LZDecompressWram(
                (*(&raw const crate::data::graphics::gSummaryPage_Skills_Tilemap)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                (*sMonSummaryScreen).bgTilemapBuffers[1][1].as_mut_ptr() as *mut c_void,
            );
            (*sMonSummaryScreen).switchCounter += 1;
        }
        4 => {
            LZDecompressWram(
                (*(&raw const crate::data::graphics::gSummaryPage_BattleMoves_Tilemap)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                (*sMonSummaryScreen).bgTilemapBuffers[2][1].as_mut_ptr() as *mut c_void,
            );
            (*sMonSummaryScreen).switchCounter += 1;
        }
        5 => {
            LZDecompressWram(
                (*(&raw const crate::data::graphics::gSummaryPage_ContestMoves_Tilemap)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                (*sMonSummaryScreen).bgTilemapBuffers[3][1].as_mut_ptr() as *mut c_void,
            );
            (*sMonSummaryScreen).switchCounter += 1;
        }
        6 => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gSummaryScreen_Pal).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                0,
                256,
            );
            LoadPalette(
                (&raw const (*(&raw const crate::data::graphics::gPPTextPalette)
                    .cast::<CArray<u16, 0>>()))
                    .cast_mut() as *mut c_void,
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
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gMoveTypes_Pal).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                464,
                96,
            );
            (*sMonSummaryScreen).switchCounter = 0;
            return TRUE;
        }
        _ => {}
    }
    FALSE
}
unsafe fn CopyMonToSummaryStruct(mon: *mut Pokemon) {
    if (*sMonSummaryScreen).isBoxMon == 0 {
        let partyMon: *mut Pokemon = (*sMonSummaryScreen).monList.mons;
        *mon = *partyMon.at((*sMonSummaryScreen).curMonIndex);
    } else {
        let boxMon: *mut BoxPokemon = (*sMonSummaryScreen).monList.boxMons;
        BoxMonToMon(boxMon.at((*sMonSummaryScreen).curMonIndex), mon);
    }
}
unsafe fn ExtractMonDataToSummaryStruct(mon: *mut Pokemon) -> u8 {
    let sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
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
            for i in 0..(MAX_MON_MOVES as u32) {
                (*sum).moves[i] = GetMonData2(mon, MON_DATA_MOVE1 + i as i32) as u16;
                (*sum).pp[i] = GetMonData2(mon, MON_DATA_PP1 + i as i32) as u8;
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
    FALSE
}
unsafe fn SetDefaultTilemaps() {
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
unsafe fn FreeSummaryScreen() {
    FreeAllWindowBuffers();
    Free(sMonSummaryScreen as *mut c_void);
}
unsafe fn BeginCloseSummaryScreen(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    task_set_func(taskId, Some(CloseSummaryScreen));
}
pub(crate) unsafe fn CloseSummaryScreen(taskId: u8) {
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
pub(crate) unsafe fn Task_HandleInput(taskId: u8) {
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
unsafe fn ChangeSummaryPokemon(taskId: u8, mut delta: i8) {
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
            task_set(taskId, 0, 0);
            task_set_func(taskId, Some(Task_ChangeSummaryMon));
        }
    }
}
pub(crate) unsafe fn Task_ChangeSummaryMon(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
                task_set_func(taskId, Some(Task_HandleInput));
            }
            return;
        }
    }
    *data += 1;
}
unsafe fn AdvanceMonIndex(delta: i8) -> i8 {
    let mon: *mut Pokemon = (*sMonSummaryScreen).monList.mons;
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
        0
    }
}
unsafe fn AdvanceMultiBattleMonIndex(delta: i8) -> i8 {
    let mons: *mut Pokemon = (*sMonSummaryScreen).monList.mons;
    let mut index: i8 = 0;
    let mut arrId: i8 = 0;
    for i in 0..(PARTY_SIZE as u8) {
        if sMultiBattleOrder[i] as i32 == (*sMonSummaryScreen).curMonIndex as i32 {
            arrId = i as i8;
            break;
        }
    }
    loop {
        let order: *mut i8 = sMultiBattleOrder.as_ptr().cast_mut();
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
        0
    }
}
unsafe fn IsValidToViewInMulti(mon: *mut Pokemon) -> u8 {
    if GetMonData2(mon, MON_DATA_SPECIES) == SPECIES_NONE as u32 {
        return FALSE;
    } else if (*sMonSummaryScreen).curMonIndex != 0 || GetMonData2(mon, MON_DATA_IS_EGG) == 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn ChangePage(taskId: u8, delta: i8) {
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
        SetTaskFuncWithFollowupFunc(taskId, Some(PssScrollRight), task_func(taskId));
    } else {
        SetTaskFuncWithFollowupFunc(taskId, Some(PssScrollLeft), task_func(taskId));
    }
    CreateTextPrinterTask((*sMonSummaryScreen).currPageIndex);
    HidePageSpecificSprites();
}
pub(crate) unsafe fn PssScrollRight(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
        task_set_func(taskId, Some(PssScrollRightEnd));
    }
}
pub(crate) unsafe fn PssScrollRightEnd(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    (*sMonSummaryScreen).bgDisplayOrder ^= 1;
    *data.at(1) = 0;
    *data = 0;
    DrawPagination();
    PutPageWindowTilemaps((*sMonSummaryScreen).currPageIndex);
    SetTypeIcons();
    TryDrawExperienceProgressBar();
    SwitchTaskToFollowupFunc(taskId);
}
pub(crate) unsafe fn PssScrollLeft(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
        task_set_func(taskId, Some(PssScrollLeftEnd));
    }
}
pub(crate) unsafe fn PssScrollLeftEnd(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
unsafe fn TryDrawExperienceProgressBar() {
    if (*sMonSummaryScreen).currPageIndex == PSS_PAGE_SKILLS {
        DrawExperienceProgressBar(&raw mut (*sMonSummaryScreen).currentMon);
    }
}
unsafe fn SwitchToMoveSelection(taskId: u8) {
    (*sMonSummaryScreen).firstMoveIndex = 0;
    let r#move: u16 = (*sMonSummaryScreen).summary.moves[(*sMonSummaryScreen).firstMoveIndex];
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
    task_set_func(taskId, Some(Task_HandleInput_MoveSelect));
}
pub(crate) unsafe fn Task_HandleInput_MoveSelect(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
unsafe fn HasMoreThanOneMove() -> u8 {
    for i in 1..(MAX_MON_MOVES as u8) {
        if (*sMonSummaryScreen).summary.moves[i] != 0 {
            return TRUE;
        }
    }
    FALSE
}
unsafe fn ChangeSelectedMove(taskData: *mut i16, direction: i8, moveIndexPtr: *mut u8) {
    let mut r#move: u16 = 0;
    PlaySE(SE_SELECT);
    let mut newMoveIndex: i8 = *moveIndexPtr as i8;
    for i in 0..(MAX_MON_MOVES as i8) {
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
unsafe fn CloseMoveSelectMode(taskId: u8) {
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
    task_set_func(taskId, Some(Task_HandleInput));
}
unsafe fn SwitchToMovePositionSwitchMode(taskId: u8) {
    (*sMonSummaryScreen).secondMoveIndex = (*sMonSummaryScreen).firstMoveIndex;
    SetMainMoveSelectorColor(1);
    CreateMoveSelectorSprites(SPRITE_ARR_ID_MOVE_SELECTOR2);
    task_set_func(taskId, Some(Task_HandleInput_MovePositionSwitch));
}
pub(crate) unsafe fn Task_HandleInput_MovePositionSwitch(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
unsafe fn ExitMovePositionSwitchMode(taskId: u8, swapMoves: u8) {
    PlaySE(SE_SELECT);
    SetMainMoveSelectorColor(0);
    DestroyMoveSelectorSprites(SPRITE_ARR_ID_MOVE_SELECTOR2);
    if swapMoves == TRUE {
        if (*sMonSummaryScreen).isBoxMon == 0 {
            let mon: *mut Pokemon = (*sMonSummaryScreen).monList.mons;
            SwapMonMoves(
                mon.at((*sMonSummaryScreen).curMonIndex),
                (*sMonSummaryScreen).firstMoveIndex,
                (*sMonSummaryScreen).secondMoveIndex,
            );
        } else {
            let boxMon: *mut BoxPokemon = (*sMonSummaryScreen).monList.boxMons;
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
    let r#move: u16 = (*sMonSummaryScreen).summary.moves[(*sMonSummaryScreen).firstMoveIndex];
    PrintMoveDetails(r#move);
    DrawContestMoveHearts(r#move);
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    task_set_func(taskId, Some(Task_HandleInput_MoveSelect));
}
unsafe fn SwapMonMoves(mon: *mut Pokemon, moveIndex1: u8, moveIndex2: u8) {
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut move1: u16 = (*summary).moves[moveIndex1];
    let mut move2: u16 = (*summary).moves[moveIndex2];
    let mut move1pp: u8 = (*summary).pp[moveIndex1];
    let mut move2pp: u8 = (*summary).pp[moveIndex2];
    let mut ppBonuses: u8 = (*summary).ppBonuses;
    let ppUpMask1: u8 =
        (*(&raw const crate::data::pokemon::gPPUpGetMask).cast::<CArray<u8, 0>>())[moveIndex1];
    let ppBonusMove1: u8 =
        shr_i32(ppBonuses as i32 & ppUpMask1 as i32, moveIndex1 as u32 * 2) as u8;
    let ppUpMask2: u8 =
        (*(&raw const crate::data::pokemon::gPPUpGetMask).cast::<CArray<u8, 0>>())[moveIndex2];
    let ppBonusMove2: u8 =
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
unsafe fn SwapBoxMonMoves(mon: *mut BoxPokemon, moveIndex1: u8, moveIndex2: u8) {
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut move1: u16 = (*summary).moves[moveIndex1];
    let mut move2: u16 = (*summary).moves[moveIndex2];
    let mut move1pp: u8 = (*summary).pp[moveIndex1];
    let mut move2pp: u8 = (*summary).pp[moveIndex2];
    let mut ppBonuses: u8 = (*summary).ppBonuses;
    let ppUpMask1: u8 =
        (*(&raw const crate::data::pokemon::gPPUpGetMask).cast::<CArray<u8, 0>>())[moveIndex1];
    let ppBonusMove1: u8 =
        shr_i32(ppBonuses as i32 & ppUpMask1 as i32, moveIndex1 as u32 * 2) as u8;
    let ppUpMask2: u8 =
        (*(&raw const crate::data::pokemon::gPPUpGetMask).cast::<CArray<u8, 0>>())[moveIndex2];
    let ppBonusMove2: u8 =
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
pub(crate) unsafe fn Task_SetHandleReplaceMoveInput(taskId: u8) {
    SetNewMoveTypeIcon();
    CreateMoveSelectorSprites(SPRITE_ARR_ID_MOVE_SELECTOR1);
    task_set_func(taskId, Some(Task_HandleReplaceMoveInput));
}
pub(crate) unsafe fn Task_HandleReplaceMoveInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE && gPaletteFade.active() != TRUE as u16 {
        if gMain.newKeys as i32 & DPAD_UP != 0 {
            *data = 4;
            ChangeSelectedMove(data, -1, &raw mut (*sMonSummaryScreen).firstMoveIndex);
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
            *data = 4;
            ChangeSelectedMove(data, 1, &raw mut (*sMonSummaryScreen).firstMoveIndex);
        } else if gMain.newKeys as i32 & DPAD_LEFT != 0 || GetLRKeysPressed() == MENU_L_PRESSED {
            ChangePage(taskId, -1);
        } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 || GetLRKeysPressed() == MENU_R_PRESSED {
            ChangePage(taskId, 1);
        } else if gMain.newKeys as i32 & A_BUTTON != 0 {
            if CanReplaceMove() == TRUE {
                StopPokemonAnimations();
                PlaySE(SE_SELECT);
                sMoveSlotToReplace.set((*sMonSummaryScreen).firstMoveIndex);
                gSpecialVar_0x8005 = sMoveSlotToReplace.get() as u16;
                BeginCloseSummaryScreen(taskId);
            } else {
                PlaySE(SE_FAILURE);
                ShowCantForgetHMsWindow(taskId);
            }
        } else if gMain.newKeys as i32 & B_BUTTON != 0 {
            StopPokemonAnimations();
            PlaySE(SE_SELECT);
            sMoveSlotToReplace.set(MAX_MON_MOVES as u8);
            gSpecialVar_0x8005 = MAX_MON_MOVES as u16;
            BeginCloseSummaryScreen(taskId);
        }
    }
}
unsafe fn CanReplaceMove() -> u8 {
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
        0
    }
}
unsafe fn ShowCantForgetHMsWindow(taskId: u8) {
    ClearWindowTilemap(PSS_LABEL_WINDOW_MOVES_POWER_ACC);
    ClearWindowTilemap(PSS_LABEL_WINDOW_MOVES_APPEAL_JAM);
    ScheduleBgCopyTilemapToVram(0);
    PositionPowerAccSlidingWindow(0, 3);
    PositionAppealJamSlidingWindow(0, 3, 0);
    PrintHMMovesCantBeForgotten();
    task_set_func(taskId, Some(Task_HandleInputCantForgetHMsMoves));
}
pub(crate) unsafe fn Task_HandleInputCantForgetHMsMoves(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let mut r#move: u16 = 0;
    if FuncIsActiveTask(Some(Task_SlidePowerAccWindow)) != 1 {
        if gMain.newKeys as i32 & DPAD_UP != 0 {
            *data.at(1) = 1;
            *data = 4;
            ChangeSelectedMove(data, -1, &raw mut (*sMonSummaryScreen).firstMoveIndex);
            *data.at(1) = 0;
            task_set_func(taskId, Some(Task_HandleReplaceMoveInput));
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
            *data.at(1) = 1;
            *data = 4;
            ChangeSelectedMove(data, 1, &raw mut (*sMonSummaryScreen).firstMoveIndex);
            *data.at(1) = 0;
            task_set_func(taskId, Some(Task_HandleReplaceMoveInput));
        } else if gMain.newKeys as i32 & DPAD_LEFT != 0 || GetLRKeysPressed() == MENU_L_PRESSED {
            if (*sMonSummaryScreen).currPageIndex != PSS_PAGE_BATTLE_MOVES {
                ClearWindowTilemap(PSS_LABEL_WINDOW_PORTRAIT_SPECIES);
                if gSprites[(*sMonSummaryScreen).spriteIds[2]].invisible() == 0 {
                    ClearWindowTilemap(PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS);
                }
                r#move = (*sMonSummaryScreen).summary.moves[(*sMonSummaryScreen).firstMoveIndex];
                task_set_func(taskId, Some(Task_HandleReplaceMoveInput));
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
                task_set_func(taskId, Some(Task_HandleReplaceMoveInput));
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
            task_set_func(taskId, Some(Task_HandleReplaceMoveInput));
        }
    }
}
pub fn GetMoveSlotToReplace() -> u8 {
    sMoveSlotToReplace.get()
}
unsafe fn DrawPagination() {
    let tilemap: *mut u16 = Alloc(32) as *mut u16;
    for i in 0..PSS_PAGE_COUNT {
        let j: u8 = i * 2;
        if i < (*sMonSummaryScreen).minPageIndex {
            *tilemap.at(j as i32) = 0x40;
            *tilemap.at(j as i32 + 1) = 0x40;
            *tilemap.at(j as i32 + 8) = 0x50;
            *tilemap.at(j as i32 + 8 + 1) = 0x50;
        } else if i > (*sMonSummaryScreen).maxPageIndex {
            *tilemap.at(j as i32) = 0x4A;
            *tilemap.at(j as i32 + 1) = 0x4A;
            *tilemap.at(j as i32 + 8) = 0x5A;
            *tilemap.at(j as i32 + 8 + 1) = 0x5A;
        } else if i < (*sMonSummaryScreen).currPageIndex {
            *tilemap.at(j as i32) = 0x46;
            *tilemap.at(j as i32 + 1) = 0x47;
            *tilemap.at(j as i32 + 8) = 0x56;
            *tilemap.at(j as i32 + 8 + 1) = 0x57;
        } else if i == (*sMonSummaryScreen).currPageIndex {
            if i != (*sMonSummaryScreen).maxPageIndex {
                *tilemap.at(j as i32) = 0x41;
                *tilemap.at(j as i32 + 1) = 0x42;
                *tilemap.at(j as i32 + 8) = 0x51;
                *tilemap.at(j as i32 + 8 + 1) = 0x52;
            } else {
                *tilemap.at(j as i32) = 0x4B;
                *tilemap.at(j as i32 + 1) = 0x4C;
                *tilemap.at(j as i32 + 8) = 0x5B;
                *tilemap.at(j as i32 + 8 + 1) = 0x5C;
            }
        } else if i != (*sMonSummaryScreen).maxPageIndex {
            *tilemap.at(j as i32) = 0x43;
            *tilemap.at(j as i32 + 1) = 0x44;
            *tilemap.at(j as i32 + 8) = 0x53;
            *tilemap.at(j as i32 + 8 + 1) = 0x54;
        } else {
            *tilemap.at(j as i32) = 0x48;
            *tilemap.at(j as i32 + 1) = 0x49;
            *tilemap.at(j as i32 + 8) = 0x58;
            *tilemap.at(j as i32 + 8 + 1) = 0x59;
        }
    }
    CopyToBgTilemapBufferRect_ChangePalette(3, tilemap as *mut c_void, 11, 0, 8, 2, 16);
    ScheduleBgCopyTilemapToVram(3);
    Free(tilemap as *mut c_void);
}
unsafe fn CopyNColumnsToTilemap(
    slidingWindow: *mut SlidingWindow,
    tilemapDest: *mut u16,
    visibleColumns: u8,
    isOpeningToTheLeft: u8,
) {
    let mut i: u16 = 0;
    let alloced: *mut u16 =
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
                    (((*slidingWindow).width as i32 - visibleColumns as i32) * 2 / 2) as u32
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
                    (((*slidingWindow).width as i32 - visibleColumns as i32) * 2 / 2) as u32
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
            ((*slidingWindow).width as i32 * 2 / 2) as u32 & 0x1FFFFF,
        );
        i += 1;
    }
    Free(alloced as *mut c_void);
}
unsafe fn PositionPowerAccSlidingWindow(visibleColumns: u16, mut speed: i16) {
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
        task_set(taskId, tScrollingSpeed, speed);
        task_set(taskId, tVisibleColumns, visibleColumns as i16);
    }
}
pub(crate) unsafe fn Task_SlidePowerAccWindow(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
unsafe fn PositionAppealJamSlidingWindow(visibleColumns: u16, mut speed: i16, r#move: u16) {
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
        task_set(taskId, tScrollingSpeed, speed);
        task_set(taskId, tVisibleColumns, visibleColumns as i16);
        task_set(taskId, tMove, r#move as i16);
    }
}
pub(crate) unsafe fn Task_SlideAppealJamWindow(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
unsafe fn PositionStatusSlidingWindow(visibleColumns: u16, mut speed: i16) {
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
        let taskId: u8 = CreateTask(Some(Task_SlideStatusWindow), 8);
        task_set(taskId, tScrollingSpeed, speed);
        task_set(taskId, tVisibleColumns, visibleColumns as i16);
    }
}
pub(crate) unsafe fn Task_SlideStatusWindow(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
unsafe fn TilemapFiveMovesDisplay(dst: *mut u16, mut palette: u16, remove: u8) {
    palette *= 0x1000;
    let id: u16 = 0x56A;
    if remove == 0 {
        for i in 0..20u16 {
            *dst.at(id as i32 + i as i32) =
                (*(&raw const crate::data::graphics::gSummaryScreen_MoveEffect_Cancel_Tilemap)
                    .cast::<CArray<u16, 0>>())[i]
                    + palette;
            *dst.at(id as i32 + i as i32 + 0x20) =
                (*(&raw const crate::data::graphics::gSummaryScreen_MoveEffect_Cancel_Tilemap)
                    .cast::<CArray<u16, 0>>())[i]
                    + palette;
            *dst.at(id as i32 + i as i32 + 0x40) =
                (*(&raw const crate::data::graphics::gSummaryScreen_MoveEffect_Cancel_Tilemap)
                    .cast::<CArray<u16, 0>>())[i as i32 + 20]
                    + palette;
        }
    } else {
        for i in 0..20u16 {
            *dst.at(id as i32 + i as i32) =
                (*(&raw const crate::data::graphics::gSummaryScreen_MoveEffect_Cancel_Tilemap)
                    .cast::<CArray<u16, 0>>())[i as i32 + 20]
                    + palette;
            *dst.at(id as i32 + i as i32 + 0x20) =
                (*(&raw const crate::data::graphics::gSummaryScreen_MoveEffect_Cancel_Tilemap)
                    .cast::<CArray<u16, 0>>())[i as i32 + 40]
                    + palette;
            *dst.at(id as i32 + i as i32 + 0x40) =
                (*(&raw const crate::data::graphics::gSummaryScreen_MoveEffect_Cancel_Tilemap)
                    .cast::<CArray<u16, 0>>())[i as i32 + 40]
                    + palette;
        }
    }
}
unsafe fn DrawPokerusCuredSymbol(mon: *mut Pokemon) {
    if CheckPartyPokerus(mon, 0) == 0 && CheckPartyHasHadPokerus(mon, 0) != 0 {
        (*sMonSummaryScreen).bgTilemapBuffers[0][0][547] = 0x2C;
        (*sMonSummaryScreen).bgTilemapBuffers[0][1][547] = 0x2C;
    } else {
        (*sMonSummaryScreen).bgTilemapBuffers[0][0][547] = 0x81A;
        (*sMonSummaryScreen).bgTilemapBuffers[0][1][547] = 0x81A;
    }
    ScheduleBgCopyTilemapToVram(3);
}
unsafe fn SetMonPicBackgroundPalette(isMonShiny: u8) {
    if isMonShiny == 0 {
        SetBgTilemapPalette(3, 1, 4, 8, 8, 0);
    } else {
        SetBgTilemapPalette(3, 1, 4, 8, 8, 5);
    }
    ScheduleBgCopyTilemapToVram(3);
}
unsafe fn DrawExperienceProgressBar(unused: *mut Pokemon) {
    let mut numExpProgressBarTicks: i64 = 0;
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*summary).level < MAX_LEVEL as u8 {
        let expBetweenLevels: u32 = (*(&raw const crate::data::pokemon::gExperienceTables)
            .cast::<CArray<CArray<u32, 101>, 0>>())
            [(*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [(*summary).species]
                .growthRate][(*summary).level as i32 + 1]
            - (*(&raw const crate::data::pokemon::gExperienceTables)
                .cast::<CArray<CArray<u32, 101>, 0>>())
                [(*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[(*summary).species]
                    .growthRate][(*summary).level];
        let expSinceLastLevel: u32 = (*summary).exp
            - (*(&raw const crate::data::pokemon::gExperienceTables)
                .cast::<CArray<CArray<u32, 101>, 0>>())
                [(*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[(*summary).species]
                    .growthRate][(*summary).level];
        numExpProgressBarTicks = div_u32(expSinceLastLevel * 64, expBetweenLevels) as i64;
        if numExpProgressBarTicks == 0 && expSinceLastLevel != 0 {
            numExpProgressBarTicks = 1;
        }
    } else {
        numExpProgressBarTicks = 0;
    }
    let dst: *mut u16 = &raw mut (*sMonSummaryScreen).bgTilemapBuffers[1][1][597];
    for i in 0..8u8 {
        if numExpProgressBarTicks > 7 {
            *dst.at(i) = 0x206A;
        } else {
            *dst.at(i) = 0x2062 + (numExpProgressBarTicks % 8) as u16;
        }
        numExpProgressBarTicks -= 8;
        if numExpProgressBarTicks < 0 {
            numExpProgressBarTicks = 0;
        }
    }
    if core::ptr::addr_eq(
        GetBgTilemapBuffer(1),
        (*sMonSummaryScreen).bgTilemapBuffers[1][0].as_mut_ptr(),
    ) {
        ScheduleBgCopyTilemapToVram(1);
    } else {
        ScheduleBgCopyTilemapToVram(2);
    }
}
unsafe fn DrawContestMoveHearts(r#move: u16) {
    let tilemap: *mut u16 = (*sMonSummaryScreen).bgTilemapBuffers[3][1].as_mut_ptr();
    let mut i: u8 = 0;
    if r#move != MOVE_NONE {
        let mut effectValue: u8 = (*(&raw const crate::data::contest_effect::gContestEffects)
            .cast::<CArray<ContestEffect, 0>>())
            [(*(&raw const crate::data::contest_effect::gContestMoves)
                .cast::<CArray<ContestMove, 0>>())[r#move]
                .effect]
            .appeal;
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
        effectValue = (*(&raw const crate::data::contest_effect::gContestEffects)
            .cast::<CArray<ContestEffect, 0>>())
            [(*(&raw const crate::data::contest_effect::gContestMoves)
                .cast::<CArray<ContestMove, 0>>())[r#move]
                .effect]
            .jam;
        if effectValue != 0xFF {
            effectValue = (effectValue as i32 / 10) as u8;
        }
        for i in 0..MAX_CONTEST_MOVE_HEARTS {
            if effectValue != 0xFF && i < effectValue {
                *tilemap.at(i as i32 / 4 * 32 + (i as i32 & 3) + 0x226) = TILE_FILLED_JAM_HEART;
            } else {
                *tilemap.at(i as i32 / 4 * 32 + (i as i32 & 3) + 0x226) = TILE_EMPTY_JAM_HEART;
            }
        }
    }
}
unsafe fn LimitEggSummaryPageDisplay() {
    if (*sMonSummaryScreen).summary.isEgg != 0 {
        ChangeBgX(3, 0x10000, BG_COORD_SET);
    } else {
        ChangeBgX(3, 0, BG_COORD_SET);
    }
}
unsafe fn ResetWindows() {
    InitWindows(sSummaryTemplate.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    for i in 0..PSS_LABEL_WINDOW_END {
        FillWindowPixelBuffer(i, 0);
    }
    for i in 0..8u8 {
        (*sMonSummaryScreen).windowIds[i] = WINDOW_NONE;
    }
}
unsafe fn PrintTextOnWindow(
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
pub(crate) unsafe fn PrintMonInfo() {
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
unsafe fn PrintNotEggInfo() {
    let mut strArray: CArray<u8, 16> = zeroed();
    let mon: *mut Pokemon = &raw mut (*sMonSummaryScreen).currentMon;
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let dexNum: u16 = SpeciesToPokedexNum((*summary).species);
    if dexNum != 0xFFFF {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (&raw const (*(&raw const crate::data::strings::gText_NumberClear01)
                .cast::<CArray<u8, 0>>())[0])
                .cast_mut(),
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
        (*(&raw const crate::data::strings::gText_LevelSymbol).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
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
        (&raw const (*(&raw const crate::data::data_tables::gSpeciesNames)
            .cast::<CArray<CArray<u8, 11>, 0>>())[(*summary).species2][0])
            .cast_mut(),
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
unsafe fn PrintEggInfo() {
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
unsafe fn PrintGenderSymbol(mon: *mut Pokemon, species: u16) {
    if species != SPECIES_NIDORAN_M && species != SPECIES_NIDORAN_F {
        match GetMonGender(mon) {
            MON_MALE => {
                PrintTextOnWindow(
                    PSS_LABEL_WINDOW_PORTRAIT_SPECIES,
                    (*(&raw const crate::data::strings::gText_MaleSymbol).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    57,
                    17,
                    0,
                    3,
                );
            }
            MON_FEMALE => {
                PrintTextOnWindow(
                    PSS_LABEL_WINDOW_PORTRAIT_SPECIES,
                    (*(&raw const crate::data::strings::gText_FemaleSymbol)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
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
unsafe fn PrintAOrBButtonIcon(windowId: u8, bButton: u8, x: u32) {
    let mut button: *mut u8 = null_mut();
    if bButton == 0 {
        button = sButtons_Gfx[0].as_ptr().cast_mut();
    } else {
        button = sButtons_Gfx[1].as_ptr().cast_mut();
    }
    BlitBitmapToWindow(windowId, button, x as u16, 0, 16, 16);
}
unsafe fn PrintPageNamesAndStats() {
    PrintTextOnWindow(
        0,
        (*(&raw const crate::data::strings::gText_PkmnInfo).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        2,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        1,
        (*(&raw const crate::data::strings::gText_PkmnSkills).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        2,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        2,
        (*(&raw const crate::data::strings::gText_BattleMoves).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        2,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_CONTEST_MOVES_TITLE,
        (*(&raw const crate::data::strings::gText_ContestMoves).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        2,
        1,
        0,
        1,
    );
    let mut stringXPos: i32 = GetStringRightAlignXOffset(
        FONT_NORMAL as i32,
        (*(&raw const crate::data::strings::gText_Cancel2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        62,
    );
    let mut iconXPos: i32 = stringXPos - 16;
    if iconXPos < 0 {
        iconXPos = 0;
    }
    PrintAOrBButtonIcon(PSS_LABEL_WINDOW_PROMPT_CANCEL, FALSE, iconXPos as u32);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_PROMPT_CANCEL,
        (*(&raw const crate::data::strings::gText_Cancel2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        stringXPos as u8,
        1,
        0,
        0,
    );
    stringXPos = GetStringRightAlignXOffset(
        FONT_NORMAL as i32,
        (*(&raw const crate::data::strings::gText_Info).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        62,
    );
    iconXPos = stringXPos - 16;
    if iconXPos < 0 {
        iconXPos = 0;
    }
    PrintAOrBButtonIcon(PSS_LABEL_WINDOW_PROMPT_INFO, FALSE, iconXPos as u32);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_PROMPT_INFO,
        (*(&raw const crate::data::strings::gText_Info).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        stringXPos as u8,
        1,
        0,
        0,
    );
    stringXPos = GetStringRightAlignXOffset(
        FONT_NORMAL as i32,
        (*(&raw const crate::data::strings::gText_Switch).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        62,
    );
    iconXPos = stringXPos - 16;
    if iconXPos < 0 {
        iconXPos = 0;
    }
    PrintAOrBButtonIcon(PSS_LABEL_WINDOW_PROMPT_SWITCH, FALSE, iconXPos as u32);
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_PROMPT_SWITCH,
        (*(&raw const crate::data::strings::gText_Switch).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        stringXPos as u8,
        1,
        0,
        0,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_INFO_RENTAL,
        (*(&raw const crate::data::strings::gText_RentalPkmn).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_INFO_TYPE,
        (*(&raw const crate::data::strings::gText_TypeSlash).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        0,
    );
    let mut statsXPos: i32 = 6 + GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        (*(&raw const crate::data::strings::gText_HP4).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        42,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_LEFT,
        (*(&raw const crate::data::strings::gText_HP4).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        statsXPos as u8,
        1,
        0,
        1,
    );
    statsXPos = 6 + GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        (*(&raw const crate::data::strings::gText_Attack3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        42,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_LEFT,
        (*(&raw const crate::data::strings::gText_Attack3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        statsXPos as u8,
        17,
        0,
        1,
    );
    statsXPos = 6 + GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        (*(&raw const crate::data::strings::gText_Defense3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        42,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_LEFT,
        (*(&raw const crate::data::strings::gText_Defense3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        statsXPos as u8,
        33,
        0,
        1,
    );
    statsXPos = 2 + GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        (*(&raw const crate::data::strings::gText_SpAtk4).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        36,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_RIGHT,
        (*(&raw const crate::data::strings::gText_SpAtk4).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        statsXPos as u8,
        1,
        0,
        1,
    );
    statsXPos = 2 + GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        (*(&raw const crate::data::strings::gText_SpDef4).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        36,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_RIGHT,
        (*(&raw const crate::data::strings::gText_SpDef4).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        statsXPos as u8,
        17,
        0,
        1,
    );
    statsXPos = 2 + GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        (*(&raw const crate::data::strings::gText_Speed2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        36,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATS_RIGHT,
        (*(&raw const crate::data::strings::gText_Speed2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        statsXPos as u8,
        33,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_EXP,
        (*(&raw const crate::data::strings::gText_ExpPoints).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        6,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_EXP,
        (*(&raw const crate::data::strings::gText_NextLv).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        6,
        17,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_POKEMON_SKILLS_STATUS,
        (*(&raw const crate::data::strings::gText_Status).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        2,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_MOVES_POWER_ACC,
        (*(&raw const crate::data::strings::gText_Power).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_MOVES_POWER_ACC,
        (*(&raw const crate::data::strings::gText_Accuracy2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        17,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_MOVES_APPEAL_JAM,
        (*(&raw const crate::data::strings::gText_Appeal).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        PSS_LABEL_WINDOW_MOVES_APPEAL_JAM,
        (*(&raw const crate::data::strings::gText_Jam).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        17,
        0,
        1,
    );
}
unsafe fn PutPageWindowTilemaps(page: u8) {
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
    for i in 0..8u8 {
        PutWindowTilemap((*sMonSummaryScreen).windowIds[i]);
    }
    ScheduleBgCopyTilemapToVram(0);
}
unsafe fn ClearPageWindowTilemaps(page: u8) {
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
    for i in 0..8u8 {
        RemoveWindowByIndex(i);
    }
    ScheduleBgCopyTilemapToVram(0);
}
unsafe fn AddWindowFromTemplateList(template: *mut WindowTemplate, templateId: u8) -> u8 {
    let windowIdPtr: *mut u8 = &raw mut (*sMonSummaryScreen).windowIds[templateId];
    if *windowIdPtr == WINDOW_NONE {
        *windowIdPtr = AddWindow(template.at(templateId)) as u8;
        FillWindowPixelBuffer(*windowIdPtr, 0);
    }
    *windowIdPtr
}
unsafe fn RemoveWindowByIndex(windowIndex: u8) {
    let windowIdPtr: *mut u8 = &raw mut (*sMonSummaryScreen).windowIds[windowIndex];
    if *windowIdPtr != WINDOW_NONE {
        ClearWindowTilemap(*windowIdPtr);
        RemoveWindow(*windowIdPtr);
        *windowIdPtr = WINDOW_NONE;
    }
}
unsafe fn PrintPageSpecificText(pageIndex: u8) {
    for i in 0..8u16 {
        if (*sMonSummaryScreen).windowIds[i] != WINDOW_NONE {
            FillWindowPixelBuffer((*sMonSummaryScreen).windowIds[i], 0);
        }
    }
    sTextPrinterFunctions[pageIndex].unwrap_unchecked()();
}
unsafe fn CreateTextPrinterTask(pageIndex: u8) {
    CreateTask(sTextPrinterTasks[pageIndex], 16);
}
pub(crate) unsafe fn PrintInfoPageText() {
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
pub(crate) unsafe fn Task_PrintInfoPage(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
unsafe fn PrintMonOTName() {
    let mut x: i32 = 0;
    let mut windowId: i32 = 0;
    if InBattleFactory() != TRUE && InSlateportBattleTent() != TRUE {
        windowId = AddWindowFromTemplateList(
            sPageInfoTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_INFO_ORIGINAL_TRAINER,
        ) as i32;
        PrintTextOnWindow(
            windowId as u8,
            (*(&raw const crate::data::strings::gText_OTSlash).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
            1,
            0,
            1,
        );
        x = GetStringWidth(
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_OTSlash).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
        );
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
unsafe fn PrintMonOTID() {
    let mut xPos: i32 = 0;
    if InBattleFactory() != TRUE && InSlateportBattleTent() != TRUE {
        ConvertIntToDecimalStringN(
            StringCopy(
                gStringVar1.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_IDNumber2).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
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
unsafe fn PrintMonAbilityName() {
    let ability: u8 = GetAbilityBySpecies(
        (*sMonSummaryScreen).summary.species,
        (*sMonSummaryScreen).summary.abilityNum,
    );
    PrintTextOnWindow(
        AddWindowFromTemplateList(
            sPageInfoTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_INFO_ABILITY,
        ),
        (*(&raw const crate::data::battle_main::gAbilityNames).cast::<CArray<CArray<u8, 13>, 0>>())
            [ability]
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        1,
    );
}
unsafe fn PrintMonAbilityDescription() {
    let ability: u8 = GetAbilityBySpecies(
        (*sMonSummaryScreen).summary.species,
        (*sMonSummaryScreen).summary.abilityNum,
    );
    PrintTextOnWindow(
        AddWindowFromTemplateList(
            sPageInfoTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_INFO_ABILITY,
        ),
        (*(&raw const crate::data::battle_main::gAbilityDescriptionPointers)
            .cast::<CArray<*mut u8, 0>>())[ability],
        0,
        17,
        0,
        0,
    );
}
unsafe fn BufferMonTrainerMemo() {
    let sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut text: *mut u8 = null_mut();
    DynamicPlaceholderTextUtil_Reset();
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, sMemoNatureTextColor.as_ptr().cast_mut());
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(1, sMemoMiscTextColor.as_ptr().cast_mut());
    BufferNatureString();
    if InBattleFactory() == TRUE || InSlateportBattleTent() == TRUE || IsInGamePartnerMon() == TRUE
    {
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_XNature).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        let metLevelString: *mut u8 = Alloc(32) as *mut u8;
        let metLocationString: *mut u8 = Alloc(32) as *mut u8;
        GetMetLevelString(metLevelString);
        if (*sum).metLocation < MAPSEC_NONE as u8 {
            GetMapNameHandleAquaHideout(metLocationString, (*sum).metLocation as u16);
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(4, metLocationString);
        }
        if DoesMonOTMatchOwner() == TRUE {
            if (*sum).metLevel == 0 {
                text = if (*sum).metLocation >= MAPSEC_NONE as u8 {
                    (*(&raw const crate::data::strings::gText_XNatureHatchedSomewhereAt)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                } else {
                    (*(&raw const crate::data::strings::gText_XNatureHatchedAtYZ)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                };
            } else {
                text = if (*sum).metLocation >= MAPSEC_NONE as u8 {
                    (*(&raw const crate::data::strings::gText_XNatureMetSomewhereAt)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                } else {
                    (*(&raw const crate::data::strings::gText_XNatureMetAtYZ)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
                };
            }
        } else if (*sum).metLocation == METLOC_FATEFUL_ENCOUNTER {
            text = (*(&raw const crate::data::strings::gText_XNatureFatefulEncounter)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        } else if (*sum).metLocation != METLOC_IN_GAME_TRADE && DidMonComeFromGBAGames() != 0 {
            text = if (*sum).metLocation >= MAPSEC_NONE as u8 {
                (*(&raw const crate::data::strings::gText_XNatureObtainedInTrade)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            } else {
                (*(&raw const crate::data::strings::gText_XNatureProbablyMetAt)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            };
        } else {
            text = (*(&raw const crate::data::strings::gText_XNatureObtainedInTrade)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        DynamicPlaceholderTextUtil_ExpandPlaceholders(gStringVar4.as_mut_ptr(), text);
        Free(metLevelString as *mut c_void);
        Free(metLocationString as *mut c_void);
    }
}
unsafe fn PrintMonTrainerMemo() {
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
unsafe fn BufferNatureString() {
    let sumStruct: *mut PokemonSummaryScreenData = sMonSummaryScreen;
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(
        2,
        gNatureNamePointers[(*sumStruct).summary.nature],
    );
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(
        5,
        (*(&raw const crate::data::strings::gText_EmptyString5).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
unsafe fn GetMetLevelString(output: *mut u8) {
    let mut level: u8 = (*sMonSummaryScreen).summary.metLevel;
    if level == 0 {
        level = EGG_HATCH_LEVEL;
    }
    ConvertIntToDecimalStringN(output, level as i32, STR_CONV_MODE_LEFT_ALIGN, 3);
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(3, output);
}
unsafe fn DoesMonOTMatchOwner() -> u8 {
    let sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let mut trainerId: u32 = 0;
    let mut gender: u8 = 0;
    if (*sMonSummaryScreen).monList.mons == gEnemyParty.as_mut_ptr() {
        let multiID: u8 = GetMultiplayerId() ^ 1;
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
        0
    }
}
unsafe fn DidMonComeFromGBAGames() -> u8 {
    let sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*sum).metGame > 0 && (*sum).metGame <= VERSION_LEAF_GREEN as u8 {
        return TRUE;
    }
    FALSE
}
pub unsafe fn DidMonComeFromRSE() -> u8 {
    let sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*sum).metGame > 0 && (*sum).metGame <= VERSION_EMERALD {
        return TRUE;
    }
    FALSE
}
unsafe fn IsInGamePartnerMon() -> u8 {
    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0
        && gMain.inBattle() != 0
        && ((*sMonSummaryScreen).curMonIndex == 1
            || (*sMonSummaryScreen).curMonIndex == 4
            || (*sMonSummaryScreen).curMonIndex == 5)
    {
        return TRUE;
    }
    FALSE
}
unsafe fn PrintEggOTName() {
    let windowId: u32 = AddWindowFromTemplateList(
        sPageInfoTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_INFO_ORIGINAL_TRAINER,
    ) as u32;
    let width: u32 = GetStringWidth(
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_OTSlash).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    ) as u32;
    PrintTextOnWindow(
        windowId as u8,
        (*(&raw const crate::data::strings::gText_OTSlash).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        1,
    );
    PrintTextOnWindow(
        windowId as u8,
        (*(&raw const crate::data::strings::gText_FiveMarks).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        width as u8,
        1,
        0,
        1,
    );
}
unsafe fn PrintEggOTID() {
    StringCopy(
        gStringVar1.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_IDNumber2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    StringAppend(
        gStringVar1.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_FiveMarks).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    let x: i32 = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar1.as_mut_ptr(), 56);
    PrintTextOnWindow(
        AddWindowFromTemplateList(sPageInfoTemplate.as_ptr().cast_mut(), 1),
        gStringVar1.as_mut_ptr(),
        x as u8,
        1,
        0,
        1,
    );
}
unsafe fn PrintEggState() {
    let mut text: *mut u8 = null_mut();
    let sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*sMonSummaryScreen).summary.sanity == TRUE {
        text = (*(&raw const crate::data::strings::gText_EggWillTakeALongTime)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    } else if (*sum).friendship <= 5 {
        text = (*(&raw const crate::data::strings::gText_EggAboutToHatch).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if (*sum).friendship <= 10 {
        text = (*(&raw const crate::data::strings::gText_EggWillHatchSoon).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if (*sum).friendship <= 40 {
        text = (*(&raw const crate::data::strings::gText_EggWillTakeSomeTime)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    } else {
        text = (*(&raw const crate::data::strings::gText_EggWillTakeALongTime)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
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
unsafe fn PrintEggMemo() {
    let mut text: *mut u8 = null_mut();
    let sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*sMonSummaryScreen).summary.sanity != 1 {
        if (*sum).metLocation == METLOC_FATEFUL_ENCOUNTER {
            text = (*(&raw const crate::data::strings::gText_PeculiarEggNicePlace)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        } else if DidMonComeFromGBAGames() == FALSE || DoesMonOTMatchOwner() == FALSE {
            text = (*(&raw const crate::data::strings::gText_PeculiarEggTrade)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        } else if (*sum).metLocation == METLOC_SPECIAL_EGG {
            text = if DidMonComeFromRSE() == TRUE {
                (*(&raw const crate::data::strings::gText_EggFromHotSprings)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
            } else {
                (*(&raw const crate::data::strings::gText_EggFromTraveler).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut()
            };
        } else {
            text = (*(&raw const crate::data::strings::gText_OddEggFoundByCouple)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
    } else {
        text = (*(&raw const crate::data::strings::gText_OddEggFoundByCouple)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
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
pub(crate) unsafe fn PrintSkillsPageText() {
    PrintHeldItemName();
    PrintRibbonCount();
    BufferLeftColumnStats();
    PrintLeftColumnStats();
    BufferRightColumnStats();
    PrintRightColumnStats();
    PrintExpPointsNextLevel();
}
pub(crate) unsafe fn Task_PrintSkillsPage(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
unsafe fn PrintHeldItemName() {
    let mut text: *mut u8 = null_mut();
    if (*sMonSummaryScreen).summary.item == ITEM_ENIGMA_BERRY
        && IsMultiBattle() == TRUE
        && ((*sMonSummaryScreen).curMonIndex == 1
            || (*sMonSummaryScreen).curMonIndex == 4
            || (*sMonSummaryScreen).curMonIndex == 5)
    {
        text = GetItemName(ITEM_ENIGMA_BERRY);
    } else if (*sMonSummaryScreen).summary.item == ITEM_NONE {
        text = (*(&raw const crate::data::strings::gText_None).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else {
        CopyItemName((*sMonSummaryScreen).summary.item, gStringVar1.as_mut_ptr());
        text = gStringVar1.as_mut_ptr();
    }
    let x: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, text, 72) + 6;
    PrintTextOnWindow(
        AddWindowFromTemplateList(sPageSkillsTemplate.as_ptr().cast_mut(), 0),
        text,
        x as u8,
        1,
        0,
        0,
    );
}
unsafe fn PrintRibbonCount() {
    let mut text: *mut u8 = null_mut();
    if (*sMonSummaryScreen).summary.ribbonCount == 0 {
        text = (*(&raw const crate::data::strings::gText_None).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else {
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*sMonSummaryScreen).summary.ribbonCount as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            2,
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_RibbonsVar1).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        text = gStringVar4.as_mut_ptr();
    }
    let x: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, text, 70) + 6;
    PrintTextOnWindow(
        AddWindowFromTemplateList(sPageSkillsTemplate.as_ptr().cast_mut(), 1),
        text,
        x as u8,
        1,
        0,
        0,
    );
}
unsafe fn BufferLeftColumnStats() {
    let currentHPString: *mut u8 = Alloc(8) as *mut u8;
    let maxHPString: *mut u8 = Alloc(8) as *mut u8;
    let attackString: *mut u8 = Alloc(8) as *mut u8;
    let defenseString: *mut u8 = Alloc(8) as *mut u8;
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
unsafe fn PrintLeftColumnStats() {
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
unsafe fn BufferRightColumnStats() {
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
unsafe fn PrintRightColumnStats() {
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
unsafe fn PrintExpPointsNextLevel() {
    let sum: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let windowId: u8 =
        AddWindowFromTemplateList(sPageSkillsTemplate.as_ptr().cast_mut(), PSS_DATA_WINDOW_EXP);
    let mut expToNextLevel: u32 = 0;
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        (*sum).exp as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        7,
    );
    let mut x: i32 =
        GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar1.as_mut_ptr(), 42) + 2;
    PrintTextOnWindow(windowId, gStringVar1.as_mut_ptr(), x as u8, 1, 0, 0);
    if (*sum).level < MAX_LEVEL as u8 {
        expToNextLevel = (*(&raw const crate::data::pokemon::gExperienceTables)
            .cast::<CArray<CArray<u32, 101>, 0>>())
            [(*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [(*sum).species]
                .growthRate][(*sum).level as i32 + 1]
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
pub(crate) unsafe fn PrintBattleMoves() {
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
pub(crate) unsafe fn Task_PrintBattleMoves(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
            if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE
                && ((*sMonSummaryScreen).newMove != MOVE_NONE
                    || (*sMonSummaryScreen).firstMoveIndex != MAX_MON_MOVES as u8)
            {
                PrintMoveDetails(*data.at(1) as u16);
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
unsafe fn PrintMoveNameAndPP(moveIndex: u8) {
    let mut pp: u8 = 0;
    let mut ppState: i32 = 0;
    let mut x: i32 = 0;
    let mut text: *mut u8 = null_mut();
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let moveNameWindowId: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_NAMES,
    );
    let ppValueWindowId: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_PP,
    );
    let r#move: u16 = (*summary).moves[moveIndex];
    if r#move != 0 {
        pp = CalculatePPWithBonus(r#move, (*summary).ppBonuses, moveIndex);
        PrintTextOnWindow(
            moveNameWindowId,
            (*(&raw const crate::data::data_tables::gMoveNames)
                .cast::<CArray<CArray<u8, 13>, 355>>())[r#move]
                .as_ptr()
                .cast_mut(),
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
            (*(&raw const crate::data::strings::gText_OneDash).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
            moveIndex * 16 + 1,
            0,
            1,
        );
        text = (*(&raw const crate::data::strings::gText_TwoDashes).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
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
unsafe fn PrintMovePowerAndAccuracy(moveIndex: u16) {
    let mut text: *mut u8 = null_mut();
    if moveIndex != 0 {
        FillWindowPixelRect(PSS_LABEL_WINDOW_MOVES_POWER_ACC, 0, 53, 0, 19, 32);
        if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [moveIndex]
            .power
            < 2
        {
            text = (*(&raw const crate::data::strings::gText_ThreeDashes).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        } else {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                    [moveIndex]
                    .power as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                3,
            );
            text = gStringVar1.as_mut_ptr();
        }
        PrintTextOnWindow(PSS_LABEL_WINDOW_MOVES_POWER_ACC, text, 53, 1, 0, 0);
        if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [moveIndex]
            .accuracy
            == 0
        {
            text = (*(&raw const crate::data::strings::gText_ThreeDashes).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        } else {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                    [moveIndex]
                    .accuracy as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                3,
            );
            text = gStringVar1.as_mut_ptr();
        }
        PrintTextOnWindow(PSS_LABEL_WINDOW_MOVES_POWER_ACC, text, 53, 17, 0, 0);
    }
}
pub(crate) unsafe fn PrintContestMoves() {
    PrintMoveNameAndPP(0);
    PrintMoveNameAndPP(1);
    PrintMoveNameAndPP(2);
    PrintMoveNameAndPP(3);
    if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE {
        PrintNewMoveDetailsOrCancelText();
        PrintContestMoveDescription((*sMonSummaryScreen).firstMoveIndex);
    }
}
pub(crate) unsafe fn Task_PrintContestMoves(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
            if (*sMonSummaryScreen).mode == SUMMARY_MODE_SELECT_MOVE
                && ((*sMonSummaryScreen).newMove != MOVE_NONE
                    || (*sMonSummaryScreen).firstMoveIndex != MAX_MON_MOVES as u8)
            {
                PrintContestMoveDescription((*sMonSummaryScreen).firstMoveIndex);
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
pub(crate) unsafe fn PrintContestMoveDescription(moveSlot: u8) {
    let mut r#move: u16 = 0;
    if moveSlot == MAX_MON_MOVES as u8 {
        r#move = (*sMonSummaryScreen).newMove;
    } else {
        r#move = (*sMonSummaryScreen).summary.moves[moveSlot];
    }
    if r#move != MOVE_NONE {
        let windowId: u8 = AddWindowFromTemplateList(
            sPageMovesTemplate.as_ptr().cast_mut(),
            PSS_DATA_WINDOW_MOVE_DESCRIPTION,
        );
        PrintTextOnWindow(
            windowId,
            (*(&raw const crate::data::contest::gContestEffectDescriptionPointers)
                .cast::<CArray<*mut u8, 0>>())
                [(*(&raw const crate::data::contest_effect::gContestMoves)
                    .cast::<CArray<ContestMove, 0>>())[r#move]
                    .effect],
            6,
            1,
            0,
            0,
        );
    }
}
unsafe fn PrintMoveDetails(r#move: u16) {
    let windowId: u8 = AddWindowFromTemplateList(
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
                (*(&raw const crate::data::contest::gContestEffectDescriptionPointers)
                    .cast::<CArray<*mut u8, 0>>())
                    [(*(&raw const crate::data::contest_effect::gContestMoves).cast::<CArray<
                        ContestMove,
                        0,
                    >>(
                    ))[r#move]
                        .effect],
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
unsafe fn PrintNewMoveDetailsOrCancelText() {
    let windowId1: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_NAMES,
    );
    let windowId2: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_PP,
    );
    if (*sMonSummaryScreen).newMove == MOVE_NONE {
        PrintTextOnWindow(
            windowId1,
            (*(&raw const crate::data::strings::gText_Cancel).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
            65,
            0,
            1,
        );
    } else {
        let r#move: u16 = (*sMonSummaryScreen).newMove;
        if (*sMonSummaryScreen).currPageIndex == PSS_PAGE_BATTLE_MOVES {
            PrintTextOnWindow(
                windowId1,
                (*(&raw const crate::data::data_tables::gMoveNames)
                    .cast::<CArray<CArray<u8, 13>, 355>>())[r#move]
                    .as_ptr()
                    .cast_mut(),
                0,
                65,
                0,
                6,
            );
        } else {
            PrintTextOnWindow(
                windowId1,
                (*(&raw const crate::data::data_tables::gMoveNames)
                    .cast::<CArray<CArray<u8, 13>, 355>>())[r#move]
                    .as_ptr()
                    .cast_mut(),
                0,
                65,
                0,
                5,
            );
        }
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [r#move]
                .pp as i32,
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
unsafe fn AddAndFillMoveNamesWindow() {
    let windowId: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_NAMES,
    );
    FillWindowPixelRect(windowId, 0, 0, 66, 72, 16);
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
unsafe fn SwapMovesNamesPP(moveIndex1: u8, moveIndex2: u8) {
    let windowId1: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_NAMES,
    );
    let windowId2: u8 = AddWindowFromTemplateList(
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
unsafe fn PrintHMMovesCantBeForgotten() {
    let windowId: u8 = AddWindowFromTemplateList(
        sPageMovesTemplate.as_ptr().cast_mut(),
        PSS_DATA_WINDOW_MOVE_DESCRIPTION,
    );
    FillWindowPixelBuffer(windowId, 0);
    PrintTextOnWindow(
        windowId,
        (*(&raw const crate::data::strings::gText_HMMovesCantBeForgotten2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        6,
        1,
        0,
        0,
    );
}
unsafe fn ResetSpriteIds() {
    for i in 0..28u8 {
        (*sMonSummaryScreen).spriteIds[i] = SPRITE_NONE;
    }
}
unsafe fn DestroySpriteInArray(spriteArrayId: u8) {
    if (*sMonSummaryScreen).spriteIds[spriteArrayId] != SPRITE_NONE {
        DestroySprite(&raw mut gSprites[(*sMonSummaryScreen).spriteIds[spriteArrayId]]);
        (*sMonSummaryScreen).spriteIds[spriteArrayId] = SPRITE_NONE;
    }
}
unsafe fn SetSpriteInvisibility(spriteArrayId: u8, invisible: u8) {
    gSprites[(*sMonSummaryScreen).spriteIds[spriteArrayId]].set_invisible(invisible as u16);
}
unsafe fn HidePageSpecificSprites() {
    for i in SPRITE_ARR_ID_TYPE..28 {
        if (*sMonSummaryScreen).spriteIds[i] != SPRITE_NONE {
            SetSpriteInvisibility(i, TRUE);
        }
    }
}
unsafe fn SetTypeIcons() {
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
unsafe fn CreateMoveTypeIcons() {
    for i in SPRITE_ARR_ID_TYPE..8 {
        if (*sMonSummaryScreen).spriteIds[i] == SPRITE_NONE {
            (*sMonSummaryScreen).spriteIds[i] =
                CreateSprite((&raw const *sSpriteTemplate_MoveTypes).cast_mut(), 0, 0, 2);
        }
        SetSpriteInvisibility(i, TRUE);
    }
}
unsafe fn SetTypeSpritePosAndPal(typeId: u8, x: u8, y: u8, spriteArrayId: u8) {
    let sprite: *mut Sprite = &raw mut gSprites[(*sMonSummaryScreen).spriteIds[spriteArrayId]];
    StartSpriteAnim(sprite, typeId);
    (*sprite)
        .oam
        .set_paletteNum(sMoveTypeToOamPaletteNum[typeId] as u16);
    (*sprite).x = x as i16 + 16;
    (*sprite).y = y as i16 + 8;
    SetSpriteInvisibility(spriteArrayId, FALSE);
}
unsafe fn SetMonTypeIcons() {
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*summary).isEgg != 0 {
        SetTypeSpritePosAndPal(TYPE_MYSTERY, 120, 48, SPRITE_ARR_ID_TYPE);
        SetSpriteInvisibility(4, 1);
    } else {
        SetTypeSpritePosAndPal(
            (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [(*summary).species]
                .types[0],
            120,
            48,
            SPRITE_ARR_ID_TYPE,
        );
        if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
            [(*summary).species]
            .types[0]
            != (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [(*summary).species]
                .types[1]
        {
            SetTypeSpritePosAndPal(
                (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                    [(*summary).species]
                    .types[1],
                160,
                48,
                4,
            );
            SetSpriteInvisibility(4, FALSE);
        } else {
            SetSpriteInvisibility(4, 1);
        }
    }
}
unsafe fn SetMoveTypeIcons() {
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    for i in 0..(MAX_MON_MOVES as u8) {
        if (*summary).moves[i] != MOVE_NONE {
            SetTypeSpritePosAndPal(
                (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                    [(*summary).moves[i]]
                    .r#type,
                85,
                32 + i * 16,
                i + SPRITE_ARR_ID_TYPE,
            );
        } else {
            SetSpriteInvisibility(i + SPRITE_ARR_ID_TYPE, TRUE);
        }
    }
}
unsafe fn SetContestMoveTypeIcons() {
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    for i in 0..(MAX_MON_MOVES as u8) {
        if (*summary).moves[i] != MOVE_NONE {
            SetTypeSpritePosAndPal(
                NUMBER_OF_MON_TYPES
                    + (*(&raw const crate::data::contest_effect::gContestMoves).cast::<CArray<
                        ContestMove,
                        0,
                    >>(
                    ))[(*summary).moves[i]]
                        .contestCategory(),
                85,
                32 + i * 16,
                i + SPRITE_ARR_ID_TYPE,
            );
        } else {
            SetSpriteInvisibility(i + SPRITE_ARR_ID_TYPE, TRUE);
        }
    }
}
unsafe fn SetNewMoveTypeIcon() {
    if (*sMonSummaryScreen).newMove == MOVE_NONE {
        SetSpriteInvisibility(7, TRUE);
    } else {
        if (*sMonSummaryScreen).currPageIndex == PSS_PAGE_BATTLE_MOVES {
            SetTypeSpritePosAndPal(
                (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                    [(*sMonSummaryScreen).newMove]
                    .r#type,
                85,
                96,
                7,
            );
        } else {
            SetTypeSpritePosAndPal(
                NUMBER_OF_MON_TYPES
                    + (*(&raw const crate::data::contest_effect::gContestMoves).cast::<CArray<
                        ContestMove,
                        0,
                    >>(
                    ))[(*sMonSummaryScreen).newMove]
                        .contestCategory(),
                85,
                96,
                7,
            );
        }
    }
}
unsafe fn SwapMovesTypeSprites(moveIndex1: u8, moveIndex2: u8) {
    let sprite1: *mut Sprite = &raw mut gSprites
        [(*sMonSummaryScreen).spriteIds[moveIndex1 as i32 + SPRITE_ARR_ID_TYPE as i32]];
    let sprite2: *mut Sprite = &raw mut gSprites
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
unsafe fn LoadMonGfxAndSprite(mon: *mut Pokemon, state: *mut i16) -> u8 {
    let mut pal: *mut CompressedSpritePalette = null_mut();
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    match *state {
        0 => {
            if gMain.inBattle() != 0 {
                if ShouldIgnoreDeoxysForm(3, (*sMonSummaryScreen).curMonIndex) != 0 {
                    HandleLoadSpecialPokePic_DontHandleDeoxys(
                        (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                            .cast::<CArray<CompressedSpriteSheet, 0>>())[(*summary).species2])
                            .cast_mut(),
                        (*gMonSpritesGfxPtr).sprites.ptr[1],
                        (*summary).species2 as i32,
                        (*summary).pid,
                    );
                } else {
                    HandleLoadSpecialPokePic_2(
                        (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                            .cast::<CArray<CompressedSpriteSheet, 0>>())[(*summary).species2])
                            .cast_mut(),
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
                            (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable).cast::<CArray<CompressedSpriteSheet, 0>>())[(*summary).species2]).cast_mut(),
                            (*gMonSpritesGfxPtr).sprites.ptr[1],
                            (*summary).species2 as i32,
                            (*summary).pid,
                        );
                    } else {
                        HandleLoadSpecialPokePic_DontHandleDeoxys(
                            (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable).cast::<CArray<CompressedSpriteSheet, 0>>())[(*summary).species2]).cast_mut(),
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
                            (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable).cast::<CArray<CompressedSpriteSheet, 0>>())[(*summary).species2]).cast_mut(),
                            MonSpritesGfxManager_GetSpritePtr(
                                MON_SPR_GFX_MANAGER_A,
                                B_POSITION_OPPONENT_LEFT,
                            ) as *mut c_void,
                            (*summary).species2 as i32,
                            (*summary).pid,
                        );
                    } else {
                        HandleLoadSpecialPokePic_DontHandleDeoxys(
                            (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable).cast::<CArray<CompressedSpriteSheet, 0>>())[(*summary).species2]).cast_mut(),
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
        0
    }
}
unsafe fn PlayMonCry() {
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if (*summary).isEgg == 0 {
        if ShouldPlayNormalMonCry(&raw mut (*sMonSummaryScreen).currentMon) == TRUE as u32 {
            PlayCry_ByMode((*summary).species2, 0, 0);
        } else {
            PlayCry_ByMode((*summary).species2, 0, CRY_MODE_WEAK);
        }
    }
}
pub(crate) unsafe fn CreateMonSprite(unused: *mut Pokemon) -> u8 {
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    let spriteId: u8 = CreateSprite(&raw mut gMultiuseSpriteTemplate, 40, 64, 5);
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
    spriteId
}
pub(crate) unsafe fn SpriteCB_Pokemon(sprite: *mut Sprite) {
    let summary: *mut PokeSummary = &raw mut (*sMonSummaryScreen).summary;
    if gPaletteFade.active() == 0 && (*sprite).data[2] != 1 {
        (*sprite).data[1] = IsMonSpriteNotFlipped((*sprite).data[0] as u16) as i16;
        PlayMonCry();
        PokemonSummaryDoMonAnimation(sprite, (*sprite).data[0] as u16, (*summary).isEgg);
    }
}
pub fn SummaryScreen_SetAnimDelayTaskId(taskId: u8) {
    sAnimDelayTaskId.set(taskId);
}
fn SummaryScreen_DestroyAnimDelayTask() {
    if sAnimDelayTaskId.get() != TASK_NONE {
        DestroyTask(sAnimDelayTaskId.get());
        sAnimDelayTaskId.set(TASK_NONE);
    }
}
unsafe fn IsMonAnimationFinished() -> u32 {
    if gSprites[(*sMonSummaryScreen).spriteIds[0]].callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn StopPokemonAnimations() {
    gSprites[(*sMonSummaryScreen).spriteIds[0]].set_animPaused(TRUE);
    gSprites[(*sMonSummaryScreen).spriteIds[0]].callback = Some(SpriteCallbackDummy);
    StopPokemonAnimationDelayTask();
    let paletteIndex: u16 =
        0x100 + gSprites[(*sMonSummaryScreen).spriteIds[0]].oam.paletteNum() * 16;
    for i in 0..16u16 {
        let id: u16 = i + paletteIndex;
        gPlttBufferUnfaded[id] = gPlttBufferFaded[id];
    }
}
unsafe fn CreateMonMarkingsSprite(mon: *mut Pokemon) {
    let sprite: *mut Sprite = CreateMonMarkingAllCombosSprite(
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
unsafe fn RemoveAndCreateMonMarkingsSprite(mon: *mut Pokemon) {
    DestroySprite((*sMonSummaryScreen).markingsSprite);
    FreeSpriteTilesByTag(TAG_MON_MARKINGS);
    CreateMonMarkingsSprite(mon);
}
unsafe fn CreateCaughtBallSprite(mon: *mut Pokemon) {
    let ball: u8 = ItemIdToBallId(GetMonData2(mon, MON_DATA_POKEBALL) as u16);
    LoadBallGfx(ball);
    (*sMonSummaryScreen).spriteIds[1] = CreateSprite(
        (&raw const (*(&raw const crate::data::pokeball::gBallSpriteTemplates)
            .cast::<CArray<SpriteTemplate, 0>>())[ball])
            .cast_mut(),
        16,
        136,
        0,
    );
    gSprites[(*sMonSummaryScreen).spriteIds[1]].callback = Some(SpriteCallbackDummy);
    gSprites[(*sMonSummaryScreen).spriteIds[1]]
        .oam
        .set_priority(3);
}
unsafe fn CreateSetStatusSprite() {
    let spriteId: *mut u8 = &raw mut (*sMonSummaryScreen).spriteIds[2];
    if *spriteId == SPRITE_NONE {
        *spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_StatusCondition).cast_mut(),
            64,
            152,
            0,
        );
    }
    let statusAnim: u8 = GetMonAilment(&raw mut (*sMonSummaryScreen).currentMon);
    if statusAnim != 0 {
        StartSpriteAnim(&raw mut gSprites[*spriteId], statusAnim - 1);
        SetSpriteInvisibility(SPRITE_ARR_ID_STATUS, FALSE);
    } else {
        SetSpriteInvisibility(SPRITE_ARR_ID_STATUS, TRUE);
    }
}
unsafe fn CreateMoveSelectorSprites(idArrayStart: u8) {
    let spriteIds: *mut u8 = &raw mut (*sMonSummaryScreen).spriteIds[idArrayStart];
    if (*sMonSummaryScreen).currPageIndex >= PSS_PAGE_BATTLE_MOVES {
        let mut subpriority: u8 = 0;
        if idArrayStart == SPRITE_ARR_ID_MOVE_SELECTOR1 {
            subpriority = 1;
        }
        for i in 0..MOVE_SELECTOR_SPRITES_COUNT {
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
        }
    }
}
pub(crate) unsafe fn SpriteCB_MoveSelector(sprite: *mut Sprite) {
    if (*sprite).animNum > 3 && (*sprite).animNum < 7 {
        (*sprite).data[1] = ((*sprite).data[1] + 1) & 0x1F;
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
unsafe fn DestroyMoveSelectorSprites(firstArrayId: u8) {
    for i in 0..MOVE_SELECTOR_SPRITES_COUNT {
        DestroySpriteInArray(firstArrayId + i);
    }
}
unsafe fn SetMainMoveSelectorColor(mut which: u8) {
    let spriteIds: *mut u8 = &raw mut (*sMonSummaryScreen).spriteIds[8];
    which *= 3;
    for i in 0..MOVE_SELECTOR_SPRITES_COUNT {
        if i == 0 {
            StartSpriteAnim(&raw mut gSprites[*spriteIds.at(i)], which + 4);
        } else if i == 9 {
            StartSpriteAnim(&raw mut gSprites[*spriteIds.at(i)], which + 5);
        } else {
            StartSpriteAnim(&raw mut gSprites[*spriteIds.at(i)], which + 6);
        }
    }
}
unsafe fn KeepMoveSelectorVisible(firstSpriteId: u8) {
    let spriteIds: *mut u8 = &raw mut (*sMonSummaryScreen).spriteIds[firstSpriteId];
    for i in 0..MOVE_SELECTOR_SPRITES_COUNT {
        gSprites[*spriteIds.at(i)].data[1] = 0;
        gSprites[*spriteIds.at(i)].set_invisible(FALSE as u16);
    }
}
