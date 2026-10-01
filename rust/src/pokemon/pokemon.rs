//! Translated from `src/pokemon.c` by tools/rustport/c2rs.py.
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
    clippy::absurd_extreme_comparisons,
    clippy::eq_op,
    clippy::explicit_counter_loop,
    clippy::if_same_then_else,
    clippy::manual_clamp,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::agb_main::{gGameLanguage, gGameVersion};
use crate::apprentice::GetApprenticeNameInLanguage;
use crate::battle_anim_mons::{GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide};
use crate::battle_controllers::BtlController_EmitGetMonData;
use crate::battle_gfx_sfx_util::ClearTemporarySpeciesSpriteData;
use crate::battle_main::{
    SpriteCallbackDummy_2, gAbsentBattlerFlags, gActiveBattler, gBattleMons, gBattleMoveDamage,
    gBattleMovePower, gBattleResources, gBattleResults, gBattleScripting, gBattleStruct,
    gBattleTypeFlags, gBattleWeather, gBattlerAttacker, gBattlerInMenuId, gBattlerTarget,
    gBattlersCount, gCritMultiplier, gCurrentMove, gDisableStructs, gEnigmaBerries, gHitMarker,
    gLastUsedAbility, gMonSpritesGfxPtr, gMoveToLearn, gPotentialItemEffectBattler, gSideTimers,
};
use crate::battle_main::{
    gBattleMonForms, gBattleTextBuff1, gBattleTextBuff2, gBattlerPartyIndexes,
    gDisplayedStringBattle,
};
use crate::battle_message::{
    BattleStringExpandPlaceholders, BattleStringExpandPlaceholdersToDisplayedString,
};
use crate::battle_pike::InBattlePike;
use crate::battle_pyramid::{
    CurrentBattlePyramidLocation, GetTrainerEncounterMusicIdInBattlePyramid,
};
use crate::battle_setup::{gPartnerTrainerId, gTrainerBattleOpponent_A};
use crate::battle_tower::{
    GetFrontierEnemyMonLevel, GetFrontierOpponentClass, GetFrontierTrainerName,
};
use crate::battle_util::{
    AbilityBattleEffects, MarkBattlerForControllerExec, UpdateSentPokesToOpponentValue,
};
use crate::box_mon::GetBoxMonData3;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagClear, FlagGet, IsNationalPokedexEnabled, VarGet, VarSet};
use crate::evolution_scene::BeginEvolutionScene;
use crate::ffi::{
    gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_0x8006, gSpecialVar_MonBoxId,
    gSpecialVar_MonBoxPos,
};
use crate::field_specials::{GetPCBoxToSendMon, SetPCBoxToSendMon};
use crate::fieldmap::gMapHeader;
use crate::item::{GetItemHoldEffect, GetItemHoldEffectParam};
use crate::link::{GetMultiplayerId, gLinkPlayers};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::m4a::m4aMPlayAllStop;
use crate::overworld::GetCurrentRegionMapSectionId;
use crate::party_menu::GetPartyIdFromBattlePartyId;
use crate::pokedex::GetSetPokedexFlag;
use crate::pokemon_animation::{
    GetSpeciesBackAnimSet, LaunchAnimationTaskForBackSprite, LaunchAnimationTaskForFrontSprite,
    SetSpriteCB_MonAnimDummy, StartMonSummaryAnimation,
};
use crate::pokemon_storage_system::{GetBoxMonDataAt, GetBoxedMonPtr, StorageGetCurrentBox};
use crate::pokemon_summary_screen::SummaryScreen_SetAnimDelayTaskId;
use crate::random::Random;
use crate::recorded_battle::gRecordedBattleMultiplayerId;
use crate::rtc::{RtcCalcLocalTime, gLocalTime};
use crate::sound::{PlayBGM, PlayCry_Normal, PlayNewMapMusic, ResetMapMusic};
use crate::string_util::StripExtCtrlCodes;
use crate::string_util::{StringCompare, StringCopy, StringCopy_Nickname};
use crate::string_util::{gStringVar1, gStringVar4};
use crate::task::DestroyTask;
use crate::task::{task_get, task_set};
use crate::trainer_hill::{GetTrainerEncounterMusicIdInTrainerHill, InTrainerHillChallenge};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
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
/// `SetBoxMonData` with this module's view of its types.
#[inline]
unsafe fn SetBoxMonData(a0: *mut BoxPokemon, a1: i32, a2: *mut c_void) {
    unsafe {
        crate::box_mon::SetBoxMonData(a0 as _, a1, a2 as _);
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
// The C's names for task and sprite data slots.
const tSongId: usize = 0;
const sAnimId: usize = 2;
const sAnimDelay: usize = 3;
// Data tables (translate with cdata.py): gBattleMoves sCombinedMoves sSpeciesToHoennPokedexNum sSpeciesToNationalPokedexNum sHoennToNationalOrder gSpindaSpotGraphics gItemEffect_Potion gItemEffect_Antidote gItemEffect_BurnHeal gItemEffect_IceHeal gItemEffect_Awakening gItemEffect_ParalyzeHeal gItemEffect_FullRestore gItemEffect_MaxPotion gItemEffect_HyperPotion gItemEffect_SuperPotion gItemEffect_FullHeal gItemEffect_Revive gItemEffect_MaxRevive gItemEffect_FreshWater gItemEffect_SodaPop gItemEffect_Lemonade gItemEffect_MoomooMilk gItemEffect_EnergyPowder gItemEffect_EnergyRoot gItemEffect_HealPowder gItemEffect_RevivalHerb gItemEffect_Ether gItemEffect_MaxEther gItemEffect_Elixir gItemEffect_MaxElixir gItemEffect_LavaCookie gItemEffect_BlueFlute gItemEffect_YellowFlute gItemEffect_RedFlute gItemEffect_BerryJuice gItemEffect_SacredAsh gItemEffect_HPUp gItemEffect_Protein gItemEffect_Iron gItemEffect_Carbos gItemEffect_Calcium gItemEffect_RareCandy gItemEffect_PPUp gItemEffect_Zinc gItemEffect_PPMax gItemEffect_GuardSpec gItemEffect_DireHit gItemEffect_XAttack gItemEffect_XDefend gItemEffect_XSpeed gItemEffect_XAccuracy gItemEffect_XSpecial gItemEffect_SunStone gItemEffect_MoonStone gItemEffect_FireStone gItemEffect_ThunderStone gItemEffect_WaterStone gItemEffect_LeafStone gItemEffect_CheriBerry gItemEffect_ChestoBerry gItemEffect_PechaBerry gItemEffect_RawstBerry gItemEffect_AspearBerry gItemEffect_LeppaBerry gItemEffect_OranBerry gItemEffect_PersimBerry gItemEffect_LumBerry gItemEffect_SitrusBerry gItemEffect_PomegBerry gItemEffect_KelpsyBerry gItemEffect_QualotBerry gItemEffect_HondewBerry gItemEffect_GrepaBerry gItemEffect_TamatoBerry gItemEffectTable gNatureStatTable gTMHMLearnsets gFacilityClassToPicIndex gFacilityClassToTrainerClass gSpeciesIdToCryId gExperienceTables gSpeciesInfo sBulbasaurLevelUpLearnset sIvysaurLevelUpLearnset sVenusaurLevelUpLearnset sCharmanderLevelUpLearnset sCharmeleonLevelUpLearnset sCharizardLevelUpLearnset sSquirtleLevelUpLearnset sWartortleLevelUpLearnset sBlastoiseLevelUpLearnset sCaterpieLevelUpLearnset sMetapodLevelUpLearnset sButterfreeLevelUpLearnset sWeedleLevelUpLearnset sKakunaLevelUpLearnset sBeedrillLevelUpLearnset sPidgeyLevelUpLearnset sPidgeottoLevelUpLearnset sPidgeotLevelUpLearnset sRattataLevelUpLearnset sRaticateLevelUpLearnset sSpearowLevelUpLearnset sFearowLevelUpLearnset sEkansLevelUpLearnset sArbokLevelUpLearnset sPikachuLevelUpLearnset sRaichuLevelUpLearnset sSandshrewLevelUpLearnset sSandslashLevelUpLearnset sNidoranFLevelUpLearnset sNidorinaLevelUpLearnset sNidoqueenLevelUpLearnset sNidoranMLevelUpLearnset sNidorinoLevelUpLearnset sNidokingLevelUpLearnset sClefairyLevelUpLearnset sClefableLevelUpLearnset sVulpixLevelUpLearnset sNinetalesLevelUpLearnset sJigglypuffLevelUpLearnset sWigglytuffLevelUpLearnset sZubatLevelUpLearnset sGolbatLevelUpLearnset sOddishLevelUpLearnset sGloomLevelUpLearnset sVileplumeLevelUpLearnset sParasLevelUpLearnset sParasectLevelUpLearnset sVenonatLevelUpLearnset sVenomothLevelUpLearnset sDiglettLevelUpLearnset sDugtrioLevelUpLearnset sMeowthLevelUpLearnset sPersianLevelUpLearnset sPsyduckLevelUpLearnset sGolduckLevelUpLearnset sMankeyLevelUpLearnset sPrimeapeLevelUpLearnset sGrowlitheLevelUpLearnset sArcanineLevelUpLearnset sPoliwagLevelUpLearnset sPoliwhirlLevelUpLearnset sPoliwrathLevelUpLearnset sAbraLevelUpLearnset sKadabraLevelUpLearnset sAlakazamLevelUpLearnset sMachopLevelUpLearnset sMachokeLevelUpLearnset sMachampLevelUpLearnset sBellsproutLevelUpLearnset sWeepinbellLevelUpLearnset sVictreebelLevelUpLearnset sTentacoolLevelUpLearnset sTentacruelLevelUpLearnset sGeodudeLevelUpLearnset sGravelerLevelUpLearnset sGolemLevelUpLearnset sPonytaLevelUpLearnset sRapidashLevelUpLearnset sSlowpokeLevelUpLearnset sSlowbroLevelUpLearnset sMagnemiteLevelUpLearnset sMagnetonLevelUpLearnset sFarfetchdLevelUpLearnset sDoduoLevelUpLearnset sDodrioLevelUpLearnset sSeelLevelUpLearnset sDewgongLevelUpLearnset sGrimerLevelUpLearnset sMukLevelUpLearnset sShellderLevelUpLearnset sCloysterLevelUpLearnset sGastlyLevelUpLearnset sHaunterLevelUpLearnset sGengarLevelUpLearnset sOnixLevelUpLearnset sDrowzeeLevelUpLearnset sHypnoLevelUpLearnset sKrabbyLevelUpLearnset sKinglerLevelUpLearnset sVoltorbLevelUpLearnset sElectrodeLevelUpLearnset sExeggcuteLevelUpLearnset sExeggutorLevelUpLearnset sCuboneLevelUpLearnset sMarowakLevelUpLearnset sHitmonleeLevelUpLearnset sHitmonchanLevelUpLearnset sLickitungLevelUpLearnset sKoffingLevelUpLearnset sWeezingLevelUpLearnset sRhyhornLevelUpLearnset sRhydonLevelUpLearnset sChanseyLevelUpLearnset sTangelaLevelUpLearnset sKangaskhanLevelUpLearnset sHorseaLevelUpLearnset sSeadraLevelUpLearnset sGoldeenLevelUpLearnset sSeakingLevelUpLearnset sStaryuLevelUpLearnset sStarmieLevelUpLearnset sMrMimeLevelUpLearnset sScytherLevelUpLearnset sJynxLevelUpLearnset sElectabuzzLevelUpLearnset sMagmarLevelUpLearnset sPinsirLevelUpLearnset sTaurosLevelUpLearnset sMagikarpLevelUpLearnset sGyaradosLevelUpLearnset sLaprasLevelUpLearnset sDittoLevelUpLearnset sEeveeLevelUpLearnset sVaporeonLevelUpLearnset sJolteonLevelUpLearnset sFlareonLevelUpLearnset sPorygonLevelUpLearnset sOmanyteLevelUpLearnset sOmastarLevelUpLearnset sKabutoLevelUpLearnset sKabutopsLevelUpLearnset sAerodactylLevelUpLearnset sSnorlaxLevelUpLearnset sArticunoLevelUpLearnset sZapdosLevelUpLearnset sMoltresLevelUpLearnset sDratiniLevelUpLearnset sDragonairLevelUpLearnset sDragoniteLevelUpLearnset sMewtwoLevelUpLearnset sMewLevelUpLearnset sChikoritaLevelUpLearnset sBayleefLevelUpLearnset sMeganiumLevelUpLearnset sCyndaquilLevelUpLearnset sQuilavaLevelUpLearnset sTyphlosionLevelUpLearnset sTotodileLevelUpLearnset sCroconawLevelUpLearnset sFeraligatrLevelUpLearnset sSentretLevelUpLearnset sFurretLevelUpLearnset sHoothootLevelUpLearnset sNoctowlLevelUpLearnset sLedybaLevelUpLearnset sLedianLevelUpLearnset sSpinarakLevelUpLearnset sAriadosLevelUpLearnset sCrobatLevelUpLearnset sChinchouLevelUpLearnset sLanturnLevelUpLearnset sPichuLevelUpLearnset sCleffaLevelUpLearnset sIgglybuffLevelUpLearnset sTogepiLevelUpLearnset sTogeticLevelUpLearnset sNatuLevelUpLearnset sXatuLevelUpLearnset sMareepLevelUpLearnset sFlaaffyLevelUpLearnset sAmpharosLevelUpLearnset sBellossomLevelUpLearnset sMarillLevelUpLearnset sAzumarillLevelUpLearnset sSudowoodoLevelUpLearnset sPolitoedLevelUpLearnset sHoppipLevelUpLearnset sSkiploomLevelUpLearnset sJumpluffLevelUpLearnset sAipomLevelUpLearnset sSunkernLevelUpLearnset sSunfloraLevelUpLearnset sYanmaLevelUpLearnset sWooperLevelUpLearnset sQuagsireLevelUpLearnset sEspeonLevelUpLearnset sUmbreonLevelUpLearnset sMurkrowLevelUpLearnset sSlowkingLevelUpLearnset sMisdreavusLevelUpLearnset sUnownLevelUpLearnset sWobbuffetLevelUpLearnset sGirafarigLevelUpLearnset sPinecoLevelUpLearnset sForretressLevelUpLearnset sDunsparceLevelUpLearnset sGligarLevelUpLearnset sSteelixLevelUpLearnset sSnubbullLevelUpLearnset sGranbullLevelUpLearnset sQwilfishLevelUpLearnset sScizorLevelUpLearnset sShuckleLevelUpLearnset sHeracrossLevelUpLearnset sSneaselLevelUpLearnset sTeddiursaLevelUpLearnset sUrsaringLevelUpLearnset sSlugmaLevelUpLearnset sMagcargoLevelUpLearnset sSwinubLevelUpLearnset sPiloswineLevelUpLearnset sCorsolaLevelUpLearnset sRemoraidLevelUpLearnset sOctilleryLevelUpLearnset sDelibirdLevelUpLearnset sMantineLevelUpLearnset sSkarmoryLevelUpLearnset sHoundourLevelUpLearnset sHoundoomLevelUpLearnset sKingdraLevelUpLearnset sPhanpyLevelUpLearnset sDonphanLevelUpLearnset sPorygon2LevelUpLearnset sStantlerLevelUpLearnset sSmeargleLevelUpLearnset sTyrogueLevelUpLearnset sHitmontopLevelUpLearnset sSmoochumLevelUpLearnset sElekidLevelUpLearnset sMagbyLevelUpLearnset sMiltankLevelUpLearnset sBlisseyLevelUpLearnset sRaikouLevelUpLearnset sEnteiLevelUpLearnset sSuicuneLevelUpLearnset sLarvitarLevelUpLearnset sPupitarLevelUpLearnset sTyranitarLevelUpLearnset sLugiaLevelUpLearnset sHoOhLevelUpLearnset sCelebiLevelUpLearnset sSpecies252LevelUpLearnset sSpecies253LevelUpLearnset sSpecies254LevelUpLearnset sSpecies255LevelUpLearnset sSpecies256LevelUpLearnset sSpecies257LevelUpLearnset sSpecies258LevelUpLearnset sSpecies259LevelUpLearnset sSpecies260LevelUpLearnset sSpecies261LevelUpLearnset sSpecies262LevelUpLearnset sSpecies263LevelUpLearnset sSpecies264LevelUpLearnset sSpecies265LevelUpLearnset sSpecies266LevelUpLearnset sSpecies267LevelUpLearnset sSpecies268LevelUpLearnset sSpecies269LevelUpLearnset sSpecies270LevelUpLearnset sSpecies271LevelUpLearnset sSpecies272LevelUpLearnset sSpecies273LevelUpLearnset sSpecies274LevelUpLearnset sSpecies275LevelUpLearnset sSpecies276LevelUpLearnset sTreeckoLevelUpLearnset sGrovyleLevelUpLearnset sSceptileLevelUpLearnset sTorchicLevelUpLearnset sCombuskenLevelUpLearnset sBlazikenLevelUpLearnset sMudkipLevelUpLearnset sMarshtompLevelUpLearnset sSwampertLevelUpLearnset sPoochyenaLevelUpLearnset sMightyenaLevelUpLearnset sZigzagoonLevelUpLearnset sLinooneLevelUpLearnset sWurmpleLevelUpLearnset sSilcoonLevelUpLearnset sBeautiflyLevelUpLearnset sCascoonLevelUpLearnset sDustoxLevelUpLearnset sLotadLevelUpLearnset sLombreLevelUpLearnset sLudicoloLevelUpLearnset sSeedotLevelUpLearnset sNuzleafLevelUpLearnset sShiftryLevelUpLearnset sNincadaLevelUpLearnset sNinjaskLevelUpLearnset sShedinjaLevelUpLearnset sTaillowLevelUpLearnset sSwellowLevelUpLearnset sShroomishLevelUpLearnset sBreloomLevelUpLearnset sSpindaLevelUpLearnset sWingullLevelUpLearnset sPelipperLevelUpLearnset sSurskitLevelUpLearnset sMasquerainLevelUpLearnset sWailmerLevelUpLearnset sWailordLevelUpLearnset sSkittyLevelUpLearnset sDelcattyLevelUpLearnset sKecleonLevelUpLearnset sBaltoyLevelUpLearnset sClaydolLevelUpLearnset sNosepassLevelUpLearnset sTorkoalLevelUpLearnset sSableyeLevelUpLearnset sBarboachLevelUpLearnset sWhiscashLevelUpLearnset sLuvdiscLevelUpLearnset sCorphishLevelUpLearnset sCrawdauntLevelUpLearnset sFeebasLevelUpLearnset sMiloticLevelUpLearnset sCarvanhaLevelUpLearnset sSharpedoLevelUpLearnset sTrapinchLevelUpLearnset sVibravaLevelUpLearnset sFlygonLevelUpLearnset sMakuhitaLevelUpLearnset sHariyamaLevelUpLearnset sElectrikeLevelUpLearnset sManectricLevelUpLearnset sNumelLevelUpLearnset sCameruptLevelUpLearnset sSphealLevelUpLearnset sSealeoLevelUpLearnset sWalreinLevelUpLearnset sCacneaLevelUpLearnset sCacturneLevelUpLearnset sSnoruntLevelUpLearnset sGlalieLevelUpLearnset sLunatoneLevelUpLearnset sSolrockLevelUpLearnset sAzurillLevelUpLearnset sSpoinkLevelUpLearnset sGrumpigLevelUpLearnset sPlusleLevelUpLearnset sMinunLevelUpLearnset sMawileLevelUpLearnset sMedititeLevelUpLearnset sMedichamLevelUpLearnset sSwabluLevelUpLearnset sAltariaLevelUpLearnset sWynautLevelUpLearnset sDuskullLevelUpLearnset sDusclopsLevelUpLearnset sRoseliaLevelUpLearnset sSlakothLevelUpLearnset sVigorothLevelUpLearnset sSlakingLevelUpLearnset sGulpinLevelUpLearnset sSwalotLevelUpLearnset sTropiusLevelUpLearnset sWhismurLevelUpLearnset sLoudredLevelUpLearnset sExploudLevelUpLearnset sClamperlLevelUpLearnset sHuntailLevelUpLearnset sGorebyssLevelUpLearnset sAbsolLevelUpLearnset sShuppetLevelUpLearnset sBanetteLevelUpLearnset sSeviperLevelUpLearnset sZangooseLevelUpLearnset sRelicanthLevelUpLearnset sAronLevelUpLearnset sLaironLevelUpLearnset sAggronLevelUpLearnset sCastformLevelUpLearnset sVolbeatLevelUpLearnset sIllumiseLevelUpLearnset sLileepLevelUpLearnset sCradilyLevelUpLearnset sAnorithLevelUpLearnset sArmaldoLevelUpLearnset sRaltsLevelUpLearnset sKirliaLevelUpLearnset sGardevoirLevelUpLearnset sBagonLevelUpLearnset sShelgonLevelUpLearnset sSalamenceLevelUpLearnset sBeldumLevelUpLearnset sMetangLevelUpLearnset sMetagrossLevelUpLearnset sRegirockLevelUpLearnset sRegiceLevelUpLearnset sRegisteelLevelUpLearnset sKyogreLevelUpLearnset sGroudonLevelUpLearnset sRayquazaLevelUpLearnset sLatiasLevelUpLearnset sLatiosLevelUpLearnset sJirachiLevelUpLearnset sDeoxysLevelUpLearnset sChimechoLevelUpLearnset gEvolutionTable gLevelUpLearnsets sMonFrontAnimIdsTable sMonAnimationDelayTable gPPUpGetMask gPPUpClearMask gPPUpAddValues gStatStageRatios sDeoxysBaseStats gUnionRoomFacilityClasses sHoldEffectToType gBattlerSpriteTemplates sTrainerBackSpriteTemplates sSecretBaseFacilityClasses sGetMonDataEVConstants sStatsToRaise sFriendshipEventModifiers sHMMoves sAlteringCaveWildMonHeldItems sOamData_64x64 sSpriteTemplate_64x64

/// `__typeof__(gTMHMLearnsets[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub union gTMHMLearnsets_0_t {
    pub learnset: TMHMLearnset,
    pub as_u32s: CArray<u32, 2>,
}

unsafe impl Sync for gTMHMLearnsets_0_t {}

/// `struct SpeciesItem`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct SpeciesItem {
    pub species: u16,
    pub item: u16,
}

unsafe impl Sync for SpeciesItem {}

/// `struct TMHMLearnset`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct TMHMLearnset {
    bits_0: u8,
    bits_1: u8,
    bits_2: u8,
    bits_3: u8,
    bits_4: u8,
    bits_5: u8,
    bits_6: u8,
    bits_7: u8,
}

impl TMHMLearnset {
    #[inline(always)]
    pub fn FOCUS_PUNCH(&self) -> u32 {
        (self.bits_0 as u32) & 0x1
    }
    #[inline(always)]
    pub fn set_FOCUS_PUNCH(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 0)) | (v as u8 & 0x1);
    }
    #[inline(always)]
    pub fn DRAGON_CLAW(&self) -> u32 {
        (self.bits_0 as u32 >> 1) & 0x1
    }
    #[inline(always)]
    pub fn set_DRAGON_CLAW(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn WATER_PULSE(&self) -> u32 {
        (self.bits_0 as u32 >> 2) & 0x1
    }
    #[inline(always)]
    pub fn set_WATER_PULSE(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn CALM_MIND(&self) -> u32 {
        (self.bits_0 as u32 >> 3) & 0x1
    }
    #[inline(always)]
    pub fn set_CALM_MIND(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn ROAR(&self) -> u32 {
        (self.bits_0 as u32 >> 4) & 0x1
    }
    #[inline(always)]
    pub fn set_ROAR(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn TOXIC(&self) -> u32 {
        (self.bits_0 as u32 >> 5) & 0x1
    }
    #[inline(always)]
    pub fn set_TOXIC(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn HAIL(&self) -> u32 {
        (self.bits_0 as u32 >> 6) & 0x1
    }
    #[inline(always)]
    pub fn set_HAIL(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn BULK_UP(&self) -> u32 {
        (self.bits_0 as u32 >> 7) & 0x1
    }
    #[inline(always)]
    pub fn set_BULK_UP(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn BULLET_SEED(&self) -> u32 {
        (self.bits_1 as u32) & 0x1
    }
    #[inline(always)]
    pub fn set_BULLET_SEED(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 0)) | (v as u8 & 0x1);
    }
    #[inline(always)]
    pub fn HIDDEN_POWER(&self) -> u32 {
        (self.bits_1 as u32 >> 1) & 0x1
    }
    #[inline(always)]
    pub fn set_HIDDEN_POWER(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn SUNNY_DAY(&self) -> u32 {
        (self.bits_1 as u32 >> 2) & 0x1
    }
    #[inline(always)]
    pub fn set_SUNNY_DAY(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn TAUNT(&self) -> u32 {
        (self.bits_1 as u32 >> 3) & 0x1
    }
    #[inline(always)]
    pub fn set_TAUNT(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn ICE_BEAM(&self) -> u32 {
        (self.bits_1 as u32 >> 4) & 0x1
    }
    #[inline(always)]
    pub fn set_ICE_BEAM(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn BLIZZARD(&self) -> u32 {
        (self.bits_1 as u32 >> 5) & 0x1
    }
    #[inline(always)]
    pub fn set_BLIZZARD(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn HYPER_BEAM(&self) -> u32 {
        (self.bits_1 as u32 >> 6) & 0x1
    }
    #[inline(always)]
    pub fn set_HYPER_BEAM(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn LIGHT_SCREEN(&self) -> u32 {
        (self.bits_1 as u32 >> 7) & 0x1
    }
    #[inline(always)]
    pub fn set_LIGHT_SCREEN(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn PROTECT(&self) -> u32 {
        (self.bits_2 as u32) & 0x1
    }
    #[inline(always)]
    pub fn set_PROTECT(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 0)) | (v as u8 & 0x1);
    }
    #[inline(always)]
    pub fn RAIN_DANCE(&self) -> u32 {
        (self.bits_2 as u32 >> 1) & 0x1
    }
    #[inline(always)]
    pub fn set_RAIN_DANCE(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn GIGA_DRAIN(&self) -> u32 {
        (self.bits_2 as u32 >> 2) & 0x1
    }
    #[inline(always)]
    pub fn set_GIGA_DRAIN(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn SAFEGUARD(&self) -> u32 {
        (self.bits_2 as u32 >> 3) & 0x1
    }
    #[inline(always)]
    pub fn set_SAFEGUARD(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn FRUSTRATION(&self) -> u32 {
        (self.bits_2 as u32 >> 4) & 0x1
    }
    #[inline(always)]
    pub fn set_FRUSTRATION(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn SOLAR_BEAM(&self) -> u32 {
        (self.bits_2 as u32 >> 5) & 0x1
    }
    #[inline(always)]
    pub fn set_SOLAR_BEAM(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn IRON_TAIL(&self) -> u32 {
        (self.bits_2 as u32 >> 6) & 0x1
    }
    #[inline(always)]
    pub fn set_IRON_TAIL(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn THUNDERBOLT(&self) -> u32 {
        (self.bits_2 as u32 >> 7) & 0x1
    }
    #[inline(always)]
    pub fn set_THUNDERBOLT(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn THUNDER(&self) -> u32 {
        (self.bits_3 as u32) & 0x1
    }
    #[inline(always)]
    pub fn set_THUNDER(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 0)) | (v as u8 & 0x1);
    }
    #[inline(always)]
    pub fn EARTHQUAKE(&self) -> u32 {
        (self.bits_3 as u32 >> 1) & 0x1
    }
    #[inline(always)]
    pub fn set_EARTHQUAKE(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn RETURN(&self) -> u32 {
        (self.bits_3 as u32 >> 2) & 0x1
    }
    #[inline(always)]
    pub fn set_RETURN(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn DIG(&self) -> u32 {
        (self.bits_3 as u32 >> 3) & 0x1
    }
    #[inline(always)]
    pub fn set_DIG(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn PSYCHIC(&self) -> u32 {
        (self.bits_3 as u32 >> 4) & 0x1
    }
    #[inline(always)]
    pub fn set_PSYCHIC(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn SHADOW_BALL(&self) -> u32 {
        (self.bits_3 as u32 >> 5) & 0x1
    }
    #[inline(always)]
    pub fn set_SHADOW_BALL(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn BRICK_BREAK(&self) -> u32 {
        (self.bits_3 as u32 >> 6) & 0x1
    }
    #[inline(always)]
    pub fn set_BRICK_BREAK(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn DOUBLE_TEAM(&self) -> u32 {
        (self.bits_3 as u32 >> 7) & 0x1
    }
    #[inline(always)]
    pub fn set_DOUBLE_TEAM(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn REFLECT(&self) -> u32 {
        (self.bits_4 as u32) & 0x1
    }
    #[inline(always)]
    pub fn set_REFLECT(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 0)) | (v as u8 & 0x1);
    }
    #[inline(always)]
    pub fn SHOCK_WAVE(&self) -> u32 {
        (self.bits_4 as u32 >> 1) & 0x1
    }
    #[inline(always)]
    pub fn set_SHOCK_WAVE(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn FLAMETHROWER(&self) -> u32 {
        (self.bits_4 as u32 >> 2) & 0x1
    }
    #[inline(always)]
    pub fn set_FLAMETHROWER(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn SLUDGE_BOMB(&self) -> u32 {
        (self.bits_4 as u32 >> 3) & 0x1
    }
    #[inline(always)]
    pub fn set_SLUDGE_BOMB(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn SANDSTORM(&self) -> u32 {
        (self.bits_4 as u32 >> 4) & 0x1
    }
    #[inline(always)]
    pub fn set_SANDSTORM(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn FIRE_BLAST(&self) -> u32 {
        (self.bits_4 as u32 >> 5) & 0x1
    }
    #[inline(always)]
    pub fn set_FIRE_BLAST(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn ROCK_TOMB(&self) -> u32 {
        (self.bits_4 as u32 >> 6) & 0x1
    }
    #[inline(always)]
    pub fn set_ROCK_TOMB(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn AERIAL_ACE(&self) -> u32 {
        (self.bits_4 as u32 >> 7) & 0x1
    }
    #[inline(always)]
    pub fn set_AERIAL_ACE(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn TORMENT(&self) -> u32 {
        (self.bits_5 as u32) & 0x1
    }
    #[inline(always)]
    pub fn set_TORMENT(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 0)) | (v as u8 & 0x1);
    }
    #[inline(always)]
    pub fn FACADE(&self) -> u32 {
        (self.bits_5 as u32 >> 1) & 0x1
    }
    #[inline(always)]
    pub fn set_FACADE(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn SECRET_POWER(&self) -> u32 {
        (self.bits_5 as u32 >> 2) & 0x1
    }
    #[inline(always)]
    pub fn set_SECRET_POWER(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn REST(&self) -> u32 {
        (self.bits_5 as u32 >> 3) & 0x1
    }
    #[inline(always)]
    pub fn set_REST(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn ATTRACT(&self) -> u32 {
        (self.bits_5 as u32 >> 4) & 0x1
    }
    #[inline(always)]
    pub fn set_ATTRACT(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn THIEF(&self) -> u32 {
        (self.bits_5 as u32 >> 5) & 0x1
    }
    #[inline(always)]
    pub fn set_THIEF(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn STEEL_WING(&self) -> u32 {
        (self.bits_5 as u32 >> 6) & 0x1
    }
    #[inline(always)]
    pub fn set_STEEL_WING(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn SKILL_SWAP(&self) -> u32 {
        (self.bits_5 as u32 >> 7) & 0x1
    }
    #[inline(always)]
    pub fn set_SKILL_SWAP(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn SNATCH(&self) -> u32 {
        (self.bits_6 as u32) & 0x1
    }
    #[inline(always)]
    pub fn set_SNATCH(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 0)) | (v as u8 & 0x1);
    }
    #[inline(always)]
    pub fn OVERHEAT(&self) -> u32 {
        (self.bits_6 as u32 >> 1) & 0x1
    }
    #[inline(always)]
    pub fn set_OVERHEAT(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn CUT(&self) -> u32 {
        (self.bits_6 as u32 >> 2) & 0x1
    }
    #[inline(always)]
    pub fn set_CUT(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn FLY(&self) -> u32 {
        (self.bits_6 as u32 >> 3) & 0x1
    }
    #[inline(always)]
    pub fn set_FLY(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn SURF(&self) -> u32 {
        (self.bits_6 as u32 >> 4) & 0x1
    }
    #[inline(always)]
    pub fn set_SURF(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn STRENGTH(&self) -> u32 {
        (self.bits_6 as u32 >> 5) & 0x1
    }
    #[inline(always)]
    pub fn set_STRENGTH(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn FLASH(&self) -> u32 {
        (self.bits_6 as u32 >> 6) & 0x1
    }
    #[inline(always)]
    pub fn set_FLASH(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn ROCK_SMASH(&self) -> u32 {
        (self.bits_6 as u32 >> 7) & 0x1
    }
    #[inline(always)]
    pub fn set_ROCK_SMASH(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn WATERFALL(&self) -> u32 {
        (self.bits_7 as u32) & 0x1
    }
    #[inline(always)]
    pub fn set_WATERFALL(&mut self, v: u32) {
        self.bits_7 = (self.bits_7 & !(0x1 << 0)) | (v as u8 & 0x1);
    }
    #[inline(always)]
    pub fn DIVE(&self) -> u32 {
        (self.bits_7 as u32 >> 1) & 0x1
    }
    #[inline(always)]
    pub fn set_DIVE(&mut self, v: u32) {
        self.bits_7 = (self.bits_7 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
}

unsafe impl Sync for TMHMLearnset {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<gTMHMLearnsets_0_t>() == 8);
    assert!(size_of::<SpeciesItem>() == 4);
    assert!(offset_of!(SpeciesItem, species) == 0);
    assert!(offset_of!(SpeciesItem, item) == 2);
    assert!(size_of::<TMHMLearnset>() == 8);
    assert!(offset_of!(TMHMLearnset, bits_0) == 0);
    assert!(offset_of!(TMHMLearnset, bits_1) == 1);
    assert!(offset_of!(TMHMLearnset, bits_2) == 2);
    assert!(offset_of!(TMHMLearnset, bits_3) == 3);
    assert!(offset_of!(TMHMLearnset, bits_4) == 4);
    assert!(offset_of!(TMHMLearnset, bits_5) == 5);
    assert!(offset_of!(TMHMLearnset, bits_6) == 6);
    assert!(offset_of!(TMHMLearnset, bits_7) == 7);
};

const ALLOC_FAIL_BUFFER: u8 = 1;
const ALLOC_FAIL_STRUCT: u8 = 2;
const DAY_EVO_HOUR_BEGIN: i8 = 12;
const DAY_EVO_HOUR_END: i8 = 24;
const FRIENDSHIP_EVO_THRESHOLD: u16 = 220;
const GFX_MANAGER_ACTIVE: u32 = 163;
const HM_MOVES_END: u16 = 65535;
const NIGHT_EVO_HOUR_BEGIN: i8 = 0;
const NIGHT_EVO_HOUR_END: i8 = 12;
const NUM_SECRET_BASE_CLASSES: i32 = 5;

static gBattleMoves: Table<CArray<BattleMove, 355>> =
    Table((&raw const crate::data::pokemon::gBattleMoves).cast());
static gBattlerSpriteTemplates: Table<CArray<SpriteTemplate, 4>> =
    Table((&raw const crate::data::pokemon::gBattlerSpriteTemplates).cast());
static gEvolutionTable: Table<CArray<CArray<Evolution, 5>, 412>> =
    Table((&raw const crate::data::pokemon::gEvolutionTable).cast());
static gExperienceTables: Table<CArray<CArray<u32, 101>, 8>> =
    Table((&raw const crate::data::pokemon::gExperienceTables).cast());
static gFacilityClassToPicIndex: Table<CArray<u8, 82>> =
    Table((&raw const crate::data::pokemon::gFacilityClassToPicIndex).cast());
static gFacilityClassToTrainerClass: Table<CArray<u8, 82>> =
    Table((&raw const crate::data::pokemon::gFacilityClassToTrainerClass).cast());
static gItemEffectTable: Table<CArray<*mut u8, 163>> =
    Table((&raw const crate::data::pokemon::gItemEffectTable).cast());
static gLevelUpLearnsets: Table<CArray<*mut u16, 412>> =
    Table((&raw const crate::data::pokemon::gLevelUpLearnsets).cast());
static gNatureStatTable: Table<CArray<CArray<i8, 5>, 25>> =
    Table((&raw const crate::data::pokemon::gNatureStatTable).cast());
static gPPUpAddValues: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::pokemon::gPPUpAddValues).cast());
static gPPUpClearMask: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::pokemon::gPPUpClearMask).cast());
static gPPUpGetMask: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::pokemon::gPPUpGetMask).cast());
static gSpeciesIdToCryId: Table<CArray<u16, 135>> =
    Table((&raw const crate::data::pokemon::gSpeciesIdToCryId).cast());
static gSpeciesInfo: Table<CArray<SpeciesInfo, 412>> =
    Table((&raw const crate::data::pokemon::gSpeciesInfo).cast());
static gSpindaSpotGraphics: Table<CArray<SpindaSpot, 4>> =
    Table((&raw const crate::data::pokemon::gSpindaSpotGraphics).cast());
static gStatStageRatios: Table<CArray<CArray<u8, 2>, 13>> =
    Table((&raw const crate::data::pokemon::gStatStageRatios).cast());
static gTMHMLearnsets: Table<CArray<gTMHMLearnsets_0_t, 412>> =
    Table((&raw const crate::data::pokemon::gTMHMLearnsets).cast());
static gUnionRoomFacilityClasses: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon::gUnionRoomFacilityClasses).cast());
static sAlteringCaveWildMonHeldItems: Table<CArray<SpeciesItem, 9>> =
    Table((&raw const crate::data::pokemon::sAlteringCaveWildMonHeldItems).cast());
static sDeoxysBaseStats: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::pokemon::sDeoxysBaseStats).cast());
static sFriendshipEventModifiers: Table<CArray<CArray<i8, 3>, 9>> =
    Table((&raw const crate::data::pokemon::sFriendshipEventModifiers).cast());
static sGetMonDataEVConstants: Table<CArray<u8, 6>> =
    Table((&raw const crate::data::pokemon::sGetMonDataEVConstants).cast());
static sHMMoves: Table<CArray<u16, 9>> = Table((&raw const crate::data::pokemon::sHMMoves).cast());
static sHoennToNationalOrder: Table<CArray<u16, 411>> =
    Table((&raw const crate::data::pokemon::sHoennToNationalOrder).cast());
static sHoldEffectToType: Table<CArray<CArray<u8, 2>, 17>> =
    Table((&raw const crate::data::pokemon::sHoldEffectToType).cast());
static sMonAnimationDelayTable: Table<CArray<u8, 411>> =
    Table((&raw const crate::data::pokemon::sMonAnimationDelayTable).cast());
static sMonFrontAnimIdsTable: Table<CArray<u8, 411>> =
    Table((&raw const crate::data::pokemon::sMonFrontAnimIdsTable).cast());
static sSecretBaseFacilityClasses: Table<CArray<CArray<u8, 5>, 2>> =
    Table((&raw const crate::data::pokemon::sSecretBaseFacilityClasses).cast());
static sSpeciesToHoennPokedexNum: Table<CArray<u16, 411>> =
    Table((&raw const crate::data::pokemon::sSpeciesToHoennPokedexNum).cast());
static sSpeciesToNationalPokedexNum: Table<CArray<u16, 411>> =
    Table((&raw const crate::data::pokemon::sSpeciesToNationalPokedexNum).cast());
static sSpriteTemplate_64x64: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon::sSpriteTemplate_64x64).cast());
static sStatsToRaise: Table<CArray<u8, 6>> =
    Table((&raw const crate::data::pokemon::sStatsToRaise).cast());
static sTrainerBackSpriteTemplates: Table<CArray<SpriteTemplate, 8>> =
    Table((&raw const crate::data::pokemon::sTrainerBackSpriteTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLearningMoveTableID: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerPartyCount: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gEnemyPartyCount: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerParty: CArray<Pokemon, 6> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gEnemyParty: CArray<Pokemon, 6> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMultiuseSpriteTemplate: SpriteTemplate = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMonSpritesGfxManagers: CArray<*mut MonSpritesGfxManager, 2> =
    unsafe { zeroed() };

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}

pub unsafe fn ZeroBoxMonData(boxMon: *mut BoxPokemon) {
    let raw: *mut u8 = boxMon as *mut u8;
    for i in 0..80u32 {
        *raw.at(i) = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ZeroMonData(mon: *mut Pokemon) {
    ZeroBoxMonData(&raw mut (*mon).r#box);
    let mut arg: u32 = 0;
    SetMonData(mon, MON_DATA_STATUS, &raw mut arg as *mut c_void);
    SetMonData(mon, MON_DATA_LEVEL, &raw mut arg as *mut c_void);
    SetMonData(mon, MON_DATA_HP, &raw mut arg as *mut c_void);
    SetMonData(mon, MON_DATA_MAX_HP, &raw mut arg as *mut c_void);
    SetMonData(mon, MON_DATA_ATK, &raw mut arg as *mut c_void);
    SetMonData(mon, MON_DATA_DEF, &raw mut arg as *mut c_void);
    SetMonData(mon, MON_DATA_SPEED, &raw mut arg as *mut c_void);
    SetMonData(mon, MON_DATA_SPATK, &raw mut arg as *mut c_void);
    SetMonData(mon, MON_DATA_SPDEF, &raw mut arg as *mut c_void);
    arg = MAIL_NONE;
    SetMonData(mon, MON_DATA_MAIL, &raw mut arg as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe fn ZeroPlayerPartyMons() {
    for i in 0..PARTY_SIZE {
        ZeroMonData(&raw mut gPlayerParty[i]);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ZeroEnemyPartyMons() {
    for i in 0..PARTY_SIZE {
        ZeroMonData(&raw mut gEnemyParty[i]);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CreateMon(
    mon: *mut Pokemon,
    species: u16,
    mut level: u8,
    fixedIV: u8,
    hasFixedPersonality: u8,
    fixedPersonality: u32,
    otIdType: u8,
    fixedOtId: u32,
) {
    ZeroMonData(mon);
    CreateBoxMon(
        &raw mut (*mon).r#box,
        species,
        level,
        fixedIV,
        hasFixedPersonality,
        fixedPersonality,
        otIdType,
        fixedOtId,
    );
    SetMonData(mon, MON_DATA_LEVEL, &raw mut level as *mut c_void);
    let mut mail: u32 = MAIL_NONE;
    SetMonData(mon, MON_DATA_MAIL, &raw mut mail as *mut c_void);
    CalculateMonStats(mon);
}
pub unsafe fn CreateBoxMon(
    boxMon: *mut BoxPokemon,
    mut species: u16,
    mut level: u8,
    mut fixedIV: u8,
    hasFixedPersonality: u8,
    fixedPersonality: u32,
    otIdType: u8,
    fixedOtId: u32,
) {
    let mut speciesName: CArray<u8, 11> = zeroed();
    let mut personality: u32 = 0;
    let mut value: u32 = 0;
    ZeroBoxMonData(boxMon);
    if hasFixedPersonality != 0 {
        personality = fixedPersonality;
    } else {
        personality = Random() as u32 | (Random() as u32) << 16;
    }
    SetBoxMonData(
        boxMon,
        MON_DATA_PERSONALITY,
        &raw mut personality as *mut c_void,
    );
    if otIdType == OT_ID_RANDOM_NO_SHINY {
        let mut shinyValue: u32 = 0;
        loop {
            value = Random() as u32 | (Random() as u32) << 16;
            shinyValue = (value & 0xFFFF0000) >> 16
                ^ value & 0xFFFF
                ^ (personality & 0xFFFF0000) >> 16
                ^ personality & 0xFFFF;
            if shinyValue >= SHINY_ODDS {
                break;
            }
        }
    } else if otIdType == OT_ID_PRESET {
        value = fixedOtId;
    } else {
        value = (*gSaveBlock2Ptr).playerTrainerId[0] as u32
            | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
            | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
            | ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24;
    }
    SetBoxMonData(boxMon, MON_DATA_OT_ID, &raw mut value as *mut c_void);
    let mut checksum: u16 = CalculateBoxMonChecksum(boxMon);
    SetBoxMonData(boxMon, MON_DATA_CHECKSUM, &raw mut checksum as *mut c_void);
    EncryptBoxMon(boxMon);
    GetSpeciesName(speciesName.as_mut_ptr(), species);
    SetBoxMonData(
        boxMon,
        MON_DATA_NICKNAME,
        speciesName.as_mut_ptr() as *mut c_void,
    );
    SetBoxMonData(
        boxMon,
        MON_DATA_LANGUAGE,
        (&raw const gGameLanguage).cast_mut() as *mut c_void,
    );
    SetBoxMonData(
        boxMon,
        MON_DATA_OT_NAME,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr() as *mut c_void,
    );
    SetBoxMonData(boxMon, MON_DATA_SPECIES, &raw mut species as *mut c_void);
    SetBoxMonData(
        boxMon,
        MON_DATA_EXP,
        (&raw const gExperienceTables[gSpeciesInfo[species].growthRate][level]).cast_mut()
            as *mut c_void,
    );
    SetBoxMonData(
        boxMon,
        MON_DATA_FRIENDSHIP,
        (&raw const gSpeciesInfo[species].friendship).cast_mut() as *mut c_void,
    );
    value = GetCurrentRegionMapSectionId() as u32;
    SetBoxMonData(boxMon, MON_DATA_MET_LOCATION, &raw mut value as *mut c_void);
    SetBoxMonData(boxMon, MON_DATA_MET_LEVEL, &raw mut level as *mut c_void);
    SetBoxMonData(
        boxMon,
        MON_DATA_MET_GAME,
        (&raw const gGameVersion).cast_mut() as *mut c_void,
    );
    value = ITEM_POKE_BALL as u32;
    SetBoxMonData(boxMon, MON_DATA_POKEBALL, &raw mut value as *mut c_void);
    SetBoxMonData(
        boxMon,
        MON_DATA_OT_GENDER,
        &raw mut (*gSaveBlock2Ptr).playerGender as *mut c_void,
    );
    if fixedIV < USE_RANDOM_IVS {
        SetBoxMonData(boxMon, MON_DATA_HP_IV, &raw mut fixedIV as *mut c_void);
        SetBoxMonData(boxMon, MON_DATA_ATK_IV, &raw mut fixedIV as *mut c_void);
        SetBoxMonData(boxMon, MON_DATA_DEF_IV, &raw mut fixedIV as *mut c_void);
        SetBoxMonData(boxMon, MON_DATA_SPEED_IV, &raw mut fixedIV as *mut c_void);
        SetBoxMonData(boxMon, MON_DATA_SPATK_IV, &raw mut fixedIV as *mut c_void);
        SetBoxMonData(boxMon, MON_DATA_SPDEF_IV, &raw mut fixedIV as *mut c_void);
    } else {
        value = Random() as u32;
        let mut iv: u32 = value & MAX_IV_MASK;
        SetBoxMonData(boxMon, MON_DATA_HP_IV, &raw mut iv as *mut c_void);
        iv = (value & 992) >> 5;
        SetBoxMonData(boxMon, MON_DATA_ATK_IV, &raw mut iv as *mut c_void);
        iv = (value & 31744) >> 10;
        SetBoxMonData(boxMon, MON_DATA_DEF_IV, &raw mut iv as *mut c_void);
        value = Random() as u32;
        iv = value & MAX_IV_MASK;
        SetBoxMonData(boxMon, MON_DATA_SPEED_IV, &raw mut iv as *mut c_void);
        iv = (value & 992) >> 5;
        SetBoxMonData(boxMon, MON_DATA_SPATK_IV, &raw mut iv as *mut c_void);
        iv = (value & 31744) >> 10;
        SetBoxMonData(boxMon, MON_DATA_SPDEF_IV, &raw mut iv as *mut c_void);
    }
    if gSpeciesInfo[species].abilities[1] != 0 {
        value = personality & 1;
        SetBoxMonData(boxMon, MON_DATA_ABILITY_NUM, &raw mut value as *mut c_void);
    }
    GiveBoxMonInitialMoveset(boxMon);
}
pub unsafe fn CreateMonWithNature(
    mon: *mut Pokemon,
    species: u16,
    level: u8,
    fixedIV: u8,
    nature: u8,
) {
    let mut personality: u32 = 0;
    loop {
        personality = Random() as u32 | (Random() as u32) << 16;
        if nature == GetNatureFromPersonality(personality) {
            break;
        }
    }
    CreateMon(mon, species, level, fixedIV, TRUE, personality, 0, 0);
}
pub unsafe fn CreateMonWithGenderNatureLetter(
    mon: *mut Pokemon,
    species: u16,
    level: u8,
    fixedIV: u8,
    gender: u8,
    nature: u8,
    unownLetter: u8,
) {
    let mut personality: u32 = 0;
    if unownLetter as i32 - 1 < NUM_UNOWN_FORMS {
        let mut actualLetter: u16 = 0;
        loop {
            personality = Random() as u32 | (Random() as u32) << 16;
            actualLetter = (((personality & 0x03000000) >> 18
                | (personality & 0x00030000) >> 12
                | (personality & 0x00000300) >> 6
                | (personality & 0x00000003))
                % 28) as u16;
            if !(nature != GetNatureFromPersonality(personality)
                || gender != GetGenderFromSpeciesAndPersonality(species, personality)
                || actualLetter as i32 != unownLetter as i32 - 1)
            {
                break;
            }
        }
    } else {
        loop {
            personality = Random() as u32 | (Random() as u32) << 16;
            if !(nature != GetNatureFromPersonality(personality)
                || gender != GetGenderFromSpeciesAndPersonality(species, personality))
            {
                break;
            }
        }
    }
    CreateMon(mon, species, level, fixedIV, TRUE, personality, 0, 0);
}
pub unsafe fn CreateMaleMon(mon: *mut Pokemon, species: u16, level: u8) {
    let mut personality: u32 = 0;
    let mut otId: u32 = 0;
    loop {
        otId = Random() as u32 | (Random() as u32) << 16;
        personality = Random() as u32 | (Random() as u32) << 16;
        if GetGenderFromSpeciesAndPersonality(species, personality) == MON_MALE {
            break;
        }
    }
    CreateMon(mon, species, level, USE_RANDOM_IVS, 1, personality, 1, otId);
}
#[unsafe(no_mangle)]
pub unsafe fn CreateMonWithIVsPersonality(
    mon: *mut Pokemon,
    species: u16,
    level: u8,
    mut ivs: u32,
    personality: u32,
) {
    CreateMon(mon, species, level, 0, TRUE, personality, 0, 0);
    SetMonData(mon, MON_DATA_IVS, &raw mut ivs as *mut c_void);
    CalculateMonStats(mon);
}
pub unsafe fn CreateMonWithIVsOTID(
    mon: *mut Pokemon,
    species: u16,
    level: u8,
    ivs: *mut u8,
    otId: u32,
) {
    CreateMon(mon, species, level, 0, 0, 0, OT_ID_PRESET, otId);
    SetMonData(mon, MON_DATA_HP_IV, ivs as *mut c_void);
    SetMonData(mon, MON_DATA_ATK_IV, ivs.at(1) as *mut c_void);
    SetMonData(mon, MON_DATA_DEF_IV, ivs.at(2) as *mut c_void);
    SetMonData(mon, MON_DATA_SPEED_IV, ivs.at(3) as *mut c_void);
    SetMonData(mon, MON_DATA_SPATK_IV, ivs.at(4) as *mut c_void);
    SetMonData(mon, MON_DATA_SPDEF_IV, ivs.at(5) as *mut c_void);
    CalculateMonStats(mon);
}
pub unsafe fn CreateMonWithEVSpread(
    mon: *mut Pokemon,
    species: u16,
    level: u8,
    fixedIV: u8,
    evSpread: u8,
) {
    let mut statCount: i32 = 0;
    CreateMon(mon, species, level, fixedIV, 0, 0, 0, 0);
    let mut evsBits: u8 = evSpread;
    let mut i: i32 = 0;
    while i < NUM_STATS {
        if evsBits as i32 & 1 != 0 {
            statCount += 1;
        }
        evsBits >>= 1;
        i += 1;
    }
    let mut evAmount: u16 = div_i32(MAX_TOTAL_EVS, statCount) as u16;
    evsBits = 1;
    for i in 0..NUM_STATS {
        if evSpread as i32 & evsBits as i32 != 0 {
            SetMonData(mon, MON_DATA_HP_EV + i, &raw mut evAmount as *mut c_void);
        }
        evsBits <<= 1;
    }
    CalculateMonStats(mon);
}
pub unsafe fn CreateBattleTowerMon(mon: *mut Pokemon, src: *mut BattleTowerPokemon) {
    let mut nickname: CArray<u8, 32> = zeroed();
    let mut language: u8 = 0;
    CreateMon(
        mon,
        (*src).species,
        (*src).level,
        0,
        1,
        (*src).personality,
        1,
        (*src).otId,
    );
    for i in 0..MAX_MON_MOVES {
        SetMonMoveSlot(mon, (*src).moves[i], i as u8);
    }
    SetMonData(
        mon,
        MON_DATA_PP_BONUSES,
        &raw mut (*src).ppBonuses as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_HELD_ITEM,
        &raw mut (*src).heldItem as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_FRIENDSHIP,
        &raw mut (*src).friendship as *mut c_void,
    );
    StringCopy(nickname.as_mut_ptr(), (*src).nickname.as_mut_ptr());
    if nickname[0] == EXT_CTRL_CODE_BEGIN && nickname[1] == EXT_CTRL_CODE_JPN {
        language = LANGUAGE_JAPANESE;
        StripExtCtrlCodes(nickname.as_mut_ptr());
    } else {
        language = GAME_LANGUAGE;
    }
    SetMonData(mon, MON_DATA_LANGUAGE, &raw mut language as *mut c_void);
    SetMonData(mon, MON_DATA_NICKNAME, nickname.as_mut_ptr() as *mut c_void);
    SetMonData(mon, MON_DATA_HP_EV, &raw mut (*src).hpEV as *mut c_void);
    SetMonData(
        mon,
        MON_DATA_ATK_EV,
        &raw mut (*src).attackEV as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_DEF_EV,
        &raw mut (*src).defenseEV as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_SPEED_EV,
        &raw mut (*src).speedEV as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_SPATK_EV,
        &raw mut (*src).spAttackEV as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_SPDEF_EV,
        &raw mut (*src).spDefenseEV as *mut c_void,
    );
    let mut value: u8 = (*src).abilityNum() as u8;
    SetMonData(mon, MON_DATA_ABILITY_NUM, &raw mut value as *mut c_void);
    value = (*src).hpIV() as u8;
    SetMonData(mon, MON_DATA_HP_IV, &raw mut value as *mut c_void);
    value = (*src).attackIV() as u8;
    SetMonData(mon, MON_DATA_ATK_IV, &raw mut value as *mut c_void);
    value = (*src).defenseIV() as u8;
    SetMonData(mon, MON_DATA_DEF_IV, &raw mut value as *mut c_void);
    value = (*src).speedIV() as u8;
    SetMonData(mon, MON_DATA_SPEED_IV, &raw mut value as *mut c_void);
    value = (*src).spAttackIV() as u8;
    SetMonData(mon, MON_DATA_SPATK_IV, &raw mut value as *mut c_void);
    value = (*src).spDefenseIV() as u8;
    SetMonData(mon, MON_DATA_SPDEF_IV, &raw mut value as *mut c_void);
    MonRestorePP(mon);
    CalculateMonStats(mon);
}
pub unsafe fn CreateBattleTowerMon_HandleLevel(
    mon: *mut Pokemon,
    src: *mut BattleTowerPokemon,
    lvl50: u8,
) {
    let mut nickname: CArray<u8, 32> = zeroed();
    let mut level: u8 = 0;
    let mut language: u8 = 0;
    if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_50 {
        level = GetFrontierEnemyMonLevel((*gSaveBlock2Ptr).frontier.lvlMode());
    } else if lvl50 != 0 {
        level = FRONTIER_MAX_LEVEL_50;
    } else {
        level = (*src).level;
    }
    CreateMon(
        mon,
        (*src).species,
        level,
        0,
        1,
        (*src).personality,
        1,
        (*src).otId,
    );
    for i in 0..MAX_MON_MOVES {
        SetMonMoveSlot(mon, (*src).moves[i], i as u8);
    }
    SetMonData(
        mon,
        MON_DATA_PP_BONUSES,
        &raw mut (*src).ppBonuses as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_HELD_ITEM,
        &raw mut (*src).heldItem as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_FRIENDSHIP,
        &raw mut (*src).friendship as *mut c_void,
    );
    StringCopy(nickname.as_mut_ptr(), (*src).nickname.as_mut_ptr());
    if nickname[0] == EXT_CTRL_CODE_BEGIN && nickname[1] == EXT_CTRL_CODE_JPN {
        language = LANGUAGE_JAPANESE;
        StripExtCtrlCodes(nickname.as_mut_ptr());
    } else {
        language = GAME_LANGUAGE;
    }
    SetMonData(mon, MON_DATA_LANGUAGE, &raw mut language as *mut c_void);
    SetMonData(mon, MON_DATA_NICKNAME, nickname.as_mut_ptr() as *mut c_void);
    SetMonData(mon, MON_DATA_HP_EV, &raw mut (*src).hpEV as *mut c_void);
    SetMonData(
        mon,
        MON_DATA_ATK_EV,
        &raw mut (*src).attackEV as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_DEF_EV,
        &raw mut (*src).defenseEV as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_SPEED_EV,
        &raw mut (*src).speedEV as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_SPATK_EV,
        &raw mut (*src).spAttackEV as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_SPDEF_EV,
        &raw mut (*src).spDefenseEV as *mut c_void,
    );
    let mut value: u8 = (*src).abilityNum() as u8;
    SetMonData(mon, MON_DATA_ABILITY_NUM, &raw mut value as *mut c_void);
    value = (*src).hpIV() as u8;
    SetMonData(mon, MON_DATA_HP_IV, &raw mut value as *mut c_void);
    value = (*src).attackIV() as u8;
    SetMonData(mon, MON_DATA_ATK_IV, &raw mut value as *mut c_void);
    value = (*src).defenseIV() as u8;
    SetMonData(mon, MON_DATA_DEF_IV, &raw mut value as *mut c_void);
    value = (*src).speedIV() as u8;
    SetMonData(mon, MON_DATA_SPEED_IV, &raw mut value as *mut c_void);
    value = (*src).spAttackIV() as u8;
    SetMonData(mon, MON_DATA_SPATK_IV, &raw mut value as *mut c_void);
    value = (*src).spDefenseIV() as u8;
    SetMonData(mon, MON_DATA_SPDEF_IV, &raw mut value as *mut c_void);
    MonRestorePP(mon);
    CalculateMonStats(mon);
}
pub unsafe fn CreateApprenticeMon(mon: *mut Pokemon, src: *mut Apprentice, monId: u8) {
    let mut language: u8 = 0;
    let otId: u32 = (*(&raw const crate::data::apprentice::gApprentices)
        .cast::<CArray<ApprenticeTrainer, 0>>())[(*src).id()]
    .otId as u32;
    let personality: u32 = (((*(&raw const crate::data::apprentice::gApprentices)
        .cast::<CArray<ApprenticeTrainer, 0>>())[(*src).id()]
    .otId
        >> 8) as u32
        | ((*(&raw const crate::data::apprentice::gApprentices)
            .cast::<CArray<ApprenticeTrainer, 0>>())[(*src).id()]
        .otId as u32
            & 0xFF)
            << 8)
        + (*src).party[monId].species as u32
        + (*src).number as u32;
    CreateMon(
        mon,
        (*src).party[monId].species,
        GetFrontierEnemyMonLevel((*src).lvlMode() - 1),
        MAX_PER_STAT_IVS,
        TRUE,
        personality,
        OT_ID_PRESET,
        otId,
    );
    SetMonData(
        mon,
        MON_DATA_HELD_ITEM,
        &raw mut (*src).party[monId].item as *mut c_void,
    );
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        SetMonMoveSlot(mon, (*src).party[monId].moves[i], i as u8);
        i += 1;
    }
    let mut evAmount: u16 = 85;
    for i in 0..NUM_STATS {
        SetMonData(mon, MON_DATA_HP_EV + i, &raw mut evAmount as *mut c_void);
    }
    language = (*src).language;
    SetMonData(mon, MON_DATA_LANGUAGE, &raw mut language as *mut c_void);
    SetMonData(
        mon,
        MON_DATA_OT_NAME,
        GetApprenticeNameInLanguage((*src).id() as u32, language as i32) as *mut c_void,
    );
    CalculateMonStats(mon);
}
pub unsafe fn CreateMonWithEVSpreadNatureOTID(
    mon: *mut Pokemon,
    species: u16,
    level: u8,
    nature: u8,
    fixedIV: u8,
    evSpread: u8,
    otId: u32,
) {
    let mut i: i32 = 0;
    let mut statCount: i32 = 0;
    loop {
        i = Random() as i32 | (Random() as i32) << 16;
        if nature == GetNatureFromPersonality(i as u32) {
            break;
        }
    }
    CreateMon(mon, species, level, fixedIV, 1, i as u32, 1, otId);
    let mut evsBits: u8 = evSpread;
    i = 0;
    while i < NUM_STATS {
        if evsBits as i32 & 1 != 0 {
            statCount += 1;
        }
        evsBits >>= 1;
        i += 1;
    }
    let mut evAmount: u16 = div_i32(MAX_TOTAL_EVS, statCount) as u16;
    evsBits = 1;
    for i in 0..NUM_STATS {
        if evSpread as i32 & evsBits as i32 != 0 {
            SetMonData(mon, MON_DATA_HP_EV + i, &raw mut evAmount as *mut c_void);
        }
        evsBits <<= 1;
    }
    CalculateMonStats(mon);
}
pub unsafe fn ConvertPokemonToBattleTowerPokemon(mon: *mut Pokemon, dest: *mut BattleTowerPokemon) {
    (*dest).species = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut heldItem: u16 = GetMonData3(mon, MON_DATA_HELD_ITEM, null_mut()) as u16;
    if heldItem == ITEM_ENIGMA_BERRY {
        heldItem = ITEM_NONE;
    }
    (*dest).heldItem = heldItem;
    for i in 0..MAX_MON_MOVES {
        (*dest).moves[i] = GetMonData3(mon, MON_DATA_MOVE1 + i, null_mut()) as u16;
    }
    (*dest).level = GetMonData3(mon, MON_DATA_LEVEL, null_mut()) as u8;
    (*dest).ppBonuses = GetMonData3(mon, MON_DATA_PP_BONUSES, null_mut()) as u8;
    (*dest).otId = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    (*dest).hpEV = GetMonData3(mon, MON_DATA_HP_EV, null_mut()) as u8;
    (*dest).attackEV = GetMonData3(mon, MON_DATA_ATK_EV, null_mut()) as u8;
    (*dest).defenseEV = GetMonData3(mon, MON_DATA_DEF_EV, null_mut()) as u8;
    (*dest).speedEV = GetMonData3(mon, MON_DATA_SPEED_EV, null_mut()) as u8;
    (*dest).spAttackEV = GetMonData3(mon, MON_DATA_SPATK_EV, null_mut()) as u8;
    (*dest).spDefenseEV = GetMonData3(mon, MON_DATA_SPDEF_EV, null_mut()) as u8;
    (*dest).friendship = GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut()) as u8;
    (*dest).set_hpIV(GetMonData3(mon, MON_DATA_HP_IV, null_mut()));
    (*dest).set_attackIV(GetMonData3(mon, MON_DATA_ATK_IV, null_mut()));
    (*dest).set_defenseIV(GetMonData3(mon, MON_DATA_DEF_IV, null_mut()));
    (*dest).set_speedIV(GetMonData3(mon, MON_DATA_SPEED_IV, null_mut()));
    (*dest).set_spAttackIV(GetMonData3(mon, MON_DATA_SPATK_IV, null_mut()));
    (*dest).set_spDefenseIV(GetMonData3(mon, MON_DATA_SPDEF_IV, null_mut()));
    (*dest).set_abilityNum(GetMonData3(mon, MON_DATA_ABILITY_NUM, null_mut()));
    (*dest).personality = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    GetMonData3(mon, MON_DATA_NICKNAME, (*dest).nickname.as_mut_ptr());
}
unsafe fn CreateEventMon(
    mon: *mut Pokemon,
    species: u16,
    level: u8,
    fixedIV: u8,
    hasFixedPersonality: u8,
    fixedPersonality: u32,
    otIdType: u8,
    fixedOtId: u32,
) {
    let mut isModernFatefulEncounter: u32 = TRUE as u32;
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
    SetMonData(
        mon,
        MON_DATA_MODERN_FATEFUL_ENCOUNTER,
        &raw mut isModernFatefulEncounter as *mut c_void,
    );
}
pub unsafe fn ShouldIgnoreDeoxysForm(caseId: u8, battler: u8) -> u8 {
    match caseId {
        1 => {
            if gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
                return FALSE;
            }
            if gMain.inBattle() == 0 {
                return FALSE;
            }
            if gLinkPlayers[GetMultiplayerId()].id == battler as u16 {
                return FALSE;
            }
        }
        2 => {}
        3 => {
            if gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
                return FALSE;
            }
            if gMain.inBattle() == 0 {
                return FALSE;
            }
            if battler == 1 || battler == 4 || battler == 5 {
                return TRUE;
            }
            return FALSE;
        }
        4 => {}
        5 => {
            if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
                if gMain.inBattle() == 0 {
                    return FALSE;
                }
                if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                    if gLinkPlayers[GetMultiplayerId()].id == battler as u16 {
                        return FALSE;
                    }
                } else {
                    if GetBattlerSide(battler) == B_SIDE_PLAYER {
                        return FALSE;
                    }
                }
            } else {
                if gMain.inBattle() == 0 {
                    return FALSE;
                }
                if GetBattlerSide(battler) == B_SIDE_PLAYER {
                    return FALSE;
                }
            }
        }
        _ => {
            return FALSE;
        }
    }
    TRUE
}
unsafe fn GetDeoxysStat(mon: *mut Pokemon, statId: i32) -> u16 {
    if gBattleTypeFlags & BATTLE_TYPE_LINK_IN_BATTLE != 0
        || GetMonData3(mon, MON_DATA_SPECIES, null_mut()) != SPECIES_DEOXYS
    {
        return 0;
    }
    let ivVal: i32 = GetMonData3(mon, MON_DATA_HP_IV + statId, null_mut()) as i32;
    let evVal: i32 = GetMonData3(mon, MON_DATA_HP_EV + statId, null_mut()) as i32;
    let mut statValue: u16 = ((sDeoxysBaseStats[statId] as i32 * 2 + ivVal + evVal / 4)
        * (*mon).level as i32
        / 100) as u16
        + 5;
    let nature: u8 = GetNature(mon);
    statValue = ModifyStatByNature(nature, statValue, statId as u8);
    statValue
}
pub unsafe fn SetDeoxysStats() {
    let mut value: i32 = 0;
    for i in 0..PARTY_SIZE {
        'l1: {
            let mon: *mut Pokemon = &raw mut gPlayerParty[i];
            if GetMonData3(mon, MON_DATA_SPECIES, null_mut()) != SPECIES_DEOXYS {
                break 'l1;
            }
            value = GetMonData3(mon, MON_DATA_ATK, null_mut()) as i32;
            SetMonData(mon, MON_DATA_ATK, &raw mut value as *mut c_void);
            value = GetMonData3(mon, MON_DATA_DEF, null_mut()) as i32;
            SetMonData(mon, MON_DATA_DEF, &raw mut value as *mut c_void);
            value = GetMonData3(mon, MON_DATA_SPEED, null_mut()) as i32;
            SetMonData(mon, MON_DATA_SPEED, &raw mut value as *mut c_void);
            value = GetMonData3(mon, MON_DATA_SPATK, null_mut()) as i32;
            SetMonData(mon, MON_DATA_SPATK, &raw mut value as *mut c_void);
            value = GetMonData3(mon, MON_DATA_SPDEF, null_mut()) as i32;
            SetMonData(mon, MON_DATA_SPDEF, &raw mut value as *mut c_void);
        }
    }
}
pub unsafe fn GetUnionRoomTrainerPic() -> u16 {
    let mut linkId: u8 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
        linkId = gRecordedBattleMultiplayerId ^ 1;
    } else {
        linkId = GetMultiplayerId() ^ 1;
    }
    let mut arrId: u32 = gLinkPlayers[linkId].trainerId % 8;
    arrId |= gLinkPlayers[linkId].gender as u32 * NUM_UNION_ROOM_CLASSES;
    FacilityClassToPicIndex(gUnionRoomFacilityClasses[arrId])
}
pub unsafe fn GetUnionRoomTrainerClass() -> u16 {
    let mut linkId: u8 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
        linkId = gRecordedBattleMultiplayerId ^ 1;
    } else {
        linkId = GetMultiplayerId() ^ 1;
    }
    let mut arrId: u32 = gLinkPlayers[linkId].trainerId % 8;
    arrId |= gLinkPlayers[linkId].gender as u32 * NUM_UNION_ROOM_CLASSES;
    gFacilityClassToTrainerClass[gUnionRoomFacilityClasses[arrId]] as u16
}
#[unsafe(no_mangle)]
pub unsafe fn CreateEnemyEventMon() {
    let species: i32 = gSpecialVar_0x8004 as i32;
    let level: i32 = gSpecialVar_0x8005 as i32;
    let itemId: i32 = gSpecialVar_0x8006 as i32;
    ZeroEnemyPartyMons();
    CreateEventMon(
        &raw mut gEnemyParty[0],
        species as u16,
        level as u8,
        USE_RANDOM_IVS,
        0,
        0,
        0,
        0,
    );
    if itemId != 0 {
        let mut heldItem: CArray<u8, 2> = zeroed();
        heldItem[0] = itemId as u8;
        heldItem[1] = (itemId >> 8) as u8;
        SetMonData(
            &raw mut gEnemyParty[0],
            MON_DATA_HELD_ITEM,
            heldItem.as_mut_ptr() as *mut c_void,
        );
    }
}
unsafe fn CalculateBoxMonChecksum(boxMon: *mut BoxPokemon) -> u16 {
    let mut checksum: u16 = 0;
    let substruct0: *mut PokemonSubstruct = GetSubstruct(boxMon, (*boxMon).personality, 0);
    let substruct1: *mut PokemonSubstruct = GetSubstruct(boxMon, (*boxMon).personality, 1);
    let substruct2: *mut PokemonSubstruct = GetSubstruct(boxMon, (*boxMon).personality, 2);
    let substruct3: *mut PokemonSubstruct = GetSubstruct(boxMon, (*boxMon).personality, 3);
    for i in 0..6i32 {
        checksum += (*substruct0).raw[i];
    }
    let mut i: i32 = 0;
    while i < 6 {
        checksum += (*substruct1).raw[i];
        i += 1;
    }
    for i in 0..6i32 {
        checksum += (*substruct2).raw[i];
    }
    for i in 0..6i32 {
        checksum += (*substruct3).raw[i];
    }
    checksum
}
pub unsafe fn CalculateMonStats(mon: *mut Pokemon) {
    let oldMaxHP: i32 = GetMonData3(mon, MON_DATA_MAX_HP, null_mut()) as i32;
    let mut currentHP: i32 = GetMonData3(mon, MON_DATA_HP, null_mut()) as i32;
    let hpIV: i32 = GetMonData3(mon, MON_DATA_HP_IV, null_mut()) as i32;
    let hpEV: i32 = GetMonData3(mon, MON_DATA_HP_EV, null_mut()) as i32;
    let attackIV: i32 = GetMonData3(mon, MON_DATA_ATK_IV, null_mut()) as i32;
    let attackEV: i32 = GetMonData3(mon, MON_DATA_ATK_EV, null_mut()) as i32;
    let defenseIV: i32 = GetMonData3(mon, MON_DATA_DEF_IV, null_mut()) as i32;
    let defenseEV: i32 = GetMonData3(mon, MON_DATA_DEF_EV, null_mut()) as i32;
    let speedIV: i32 = GetMonData3(mon, MON_DATA_SPEED_IV, null_mut()) as i32;
    let speedEV: i32 = GetMonData3(mon, MON_DATA_SPEED_EV, null_mut()) as i32;
    let spAttackIV: i32 = GetMonData3(mon, MON_DATA_SPATK_IV, null_mut()) as i32;
    let spAttackEV: i32 = GetMonData3(mon, MON_DATA_SPATK_EV, null_mut()) as i32;
    let spDefenseIV: i32 = GetMonData3(mon, MON_DATA_SPDEF_IV, null_mut()) as i32;
    let spDefenseEV: i32 = GetMonData3(mon, MON_DATA_SPDEF_EV, null_mut()) as i32;
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut level: i32 = GetLevelFromMonExp(mon) as i32;
    let mut newMaxHP: i32 = 0;
    SetMonData(mon, MON_DATA_LEVEL, &raw mut level as *mut c_void);
    if species == SPECIES_SHEDINJA {
        newMaxHP = 1;
    } else {
        let n: i32 = 2 * gSpeciesInfo[species].baseHP as i32 + hpIV;
        newMaxHP = (n + hpEV / 4) * level / 100 + level + 10;
    }
    gBattleScripting.levelUpHP = newMaxHP as u8 - oldMaxHP as u8;
    if gBattleScripting.levelUpHP == 0 {
        gBattleScripting.levelUpHP = 1;
    }
    SetMonData(mon, MON_DATA_MAX_HP, &raw mut newMaxHP as *mut c_void);
    {
        let baseStat: u8 = gSpeciesInfo[species].baseAttack;
        let mut n: i32 = (2 * baseStat as i32 + attackIV + attackEV / 4) * level / 100 + 5;
        let nature: u8 = GetNature(mon);
        n = ModifyStatByNature(nature, n as u16, STAT_ATK) as i32;
        SetMonData(mon, MON_DATA_ATK, &raw mut n as *mut c_void);
    }
    {
        let baseStat: u8 = gSpeciesInfo[species].baseDefense;
        let mut n: i32 = (STAT_DEF * baseStat as i32 + defenseIV + defenseEV / 4) * level / 100 + 5;
        let nature: u8 = GetNature(mon);
        n = ModifyStatByNature(nature, n as u16, STAT_DEF as u8) as i32;
        SetMonData(mon, MON_DATA_DEF, &raw mut n as *mut c_void);
    }
    {
        let baseStat: u8 = gSpeciesInfo[species].baseSpeed;
        let mut n: i32 = (2 * baseStat as i32 + speedIV + speedEV / 4) * level / 100 + 5;
        let nature: u8 = GetNature(mon);
        n = ModifyStatByNature(nature, n as u16, STAT_SPEED) as i32;
        SetMonData(mon, MON_DATA_SPEED, &raw mut n as *mut c_void);
    }
    {
        let baseStat: u8 = gSpeciesInfo[species].baseSpAttack;
        let mut n: i32 = (2 * baseStat as i32 + spAttackIV + spAttackEV / 4) * level / 100 + 5;
        let nature: u8 = GetNature(mon);
        n = ModifyStatByNature(nature, n as u16, STAT_SPATK) as i32;
        SetMonData(mon, MON_DATA_SPATK, &raw mut n as *mut c_void);
    }
    {
        let baseStat: u8 = gSpeciesInfo[species].baseSpDefense;
        let mut n: i32 =
            (2 * baseStat as i32 + spDefenseIV + spDefenseEV / 4) * level / 100 + STAT_SPDEF;
        let nature: u8 = GetNature(mon);
        n = ModifyStatByNature(nature, n as u16, STAT_SPDEF as u8) as i32;
        SetMonData(mon, MON_DATA_SPDEF, &raw mut n as *mut c_void);
    }
    if species == SPECIES_SHEDINJA {
        if currentHP != 0 || oldMaxHP == 0 {
            currentHP = 1;
        } else {
            return;
        }
    } else {
        if currentHP == 0 && oldMaxHP == 0 {
            currentHP = newMaxHP;
        } else if currentHP != 0 {
            currentHP += newMaxHP - oldMaxHP;
        } else {
            return;
        }
    }
    SetMonData(mon, MON_DATA_HP, &raw mut currentHP as *mut c_void);
}
pub unsafe fn BoxMonToMon(src: *mut BoxPokemon, dest: *mut Pokemon) {
    let mut value: u32 = 0;
    (*dest).r#box = *src;
    SetMonData(dest, MON_DATA_STATUS, &raw mut value as *mut c_void);
    SetMonData(dest, MON_DATA_HP, &raw mut value as *mut c_void);
    SetMonData(dest, MON_DATA_MAX_HP, &raw mut value as *mut c_void);
    value = MAIL_NONE;
    SetMonData(dest, MON_DATA_MAIL, &raw mut value as *mut c_void);
    CalculateMonStats(dest);
}
pub unsafe fn GetLevelFromMonExp(mon: *mut Pokemon) -> u8 {
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let exp: u32 = GetMonData3(mon, MON_DATA_EXP, null_mut());
    let mut level: i32 = 1;
    while level <= MAX_LEVEL as i32
        && gExperienceTables[gSpeciesInfo[species].growthRate][level] <= exp
    {
        level += 1;
    }
    level as u8 - 1
}
pub unsafe fn GetLevelFromBoxMonExp(boxMon: *mut BoxPokemon) -> u8 {
    let species: u16 = GetBoxMonData3(boxMon, MON_DATA_SPECIES, null_mut()) as u16;
    let exp: u32 = GetBoxMonData3(boxMon, MON_DATA_EXP, null_mut());
    let mut level: i32 = 1;
    while level <= MAX_LEVEL as i32
        && gExperienceTables[gSpeciesInfo[species].growthRate][level] <= exp
    {
        level += 1;
    }
    level as u8 - 1
}
pub unsafe fn GiveMoveToMon(mon: *mut Pokemon, r#move: u16) -> u16 {
    GiveMoveToBoxMon(&raw mut (*mon).r#box, r#move)
}
unsafe fn GiveMoveToBoxMon(boxMon: *mut BoxPokemon, mut r#move: u16) -> u16 {
    for i in 0..MAX_MON_MOVES {
        let existingMove: u16 = GetBoxMonData3(boxMon, MON_DATA_MOVE1 + i, null_mut()) as u16;
        if existingMove == MOVE_NONE {
            SetBoxMonData(boxMon, MON_DATA_MOVE1 + i, &raw mut r#move as *mut c_void);
            SetBoxMonData(
                boxMon,
                MON_DATA_PP1 + i,
                (&raw const gBattleMoves[r#move].pp).cast_mut() as *mut c_void,
            );
            return r#move;
        }
        if existingMove == r#move {
            return MON_ALREADY_KNOWS_MOVE;
        }
    }
    MON_HAS_MAX_MOVES
}
pub unsafe fn GiveMoveToBattleMon(mon: *mut BattlePokemon, r#move: u16) -> u16 {
    for i in 0..MAX_MON_MOVES {
        if (*mon).moves[i] == MOVE_NONE {
            (*mon).moves[i] = r#move;
            (*mon).pp[i] = gBattleMoves[r#move].pp;
            return r#move;
        }
    }
    MON_HAS_MAX_MOVES
}
#[unsafe(no_mangle)]
pub unsafe fn SetMonMoveSlot(mon: *mut Pokemon, mut r#move: u16, slot: u8) {
    SetMonData(
        mon,
        MON_DATA_MOVE1 + slot as i32,
        &raw mut r#move as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_PP1 + slot as i32,
        (&raw const gBattleMoves[r#move].pp).cast_mut() as *mut c_void,
    );
}
pub unsafe fn SetBattleMonMoveSlot(mon: *mut BattlePokemon, r#move: u16, slot: u8) {
    (*mon).moves[slot] = r#move;
    (*mon).pp[slot] = gBattleMoves[r#move].pp;
}
pub unsafe fn GiveMonInitialMoveset(mon: *mut Pokemon) {
    GiveBoxMonInitialMoveset(&raw mut (*mon).r#box);
}
pub unsafe fn GiveBoxMonInitialMoveset(boxMon: *mut BoxPokemon) {
    let species: u16 = GetBoxMonData3(boxMon, MON_DATA_SPECIES, null_mut()) as u16;
    let level: i32 = GetLevelFromBoxMonExp(boxMon) as i32;
    let mut i: i32 = 0;
    while *gLevelUpLearnsets[species].at(i) != LEVEL_UP_END {
        let moveLevel: u16 = *gLevelUpLearnsets[species].at(i) & LEVEL_UP_MOVE_LV as u16;
        if moveLevel as i32 > level << 9 {
            break;
        }
        let r#move: u16 = *gLevelUpLearnsets[species].at(i) & LEVEL_UP_MOVE_ID;
        if GiveMoveToBoxMon(boxMon, r#move) == MON_HAS_MAX_MOVES {
            DeleteFirstMoveAndGiveMoveToBoxMon(boxMon, r#move);
        }
        i += 1;
    }
}
pub unsafe fn MonTryLearningNewMove(mon: *mut Pokemon, firstMove: u8) -> u16 {
    let mut retVal: u32 = MOVE_NONE as u32;
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let level: u8 = GetMonData3(mon, MON_DATA_LEVEL, null_mut()) as u8;
    if firstMove != 0 {
        sLearningMoveTableID = 0;
        while *gLevelUpLearnsets[species].at(sLearningMoveTableID) as i32 & LEVEL_UP_MOVE_LV
            != (level as i32) << 9
        {
            sLearningMoveTableID += 1;
            if *gLevelUpLearnsets[species].at(sLearningMoveTableID) == LEVEL_UP_END {
                return MOVE_NONE;
            }
        }
    }
    if *gLevelUpLearnsets[species].at(sLearningMoveTableID) as i32 & LEVEL_UP_MOVE_LV
        == (level as i32) << 9
    {
        gMoveToLearn = *gLevelUpLearnsets[species].at(sLearningMoveTableID) & LEVEL_UP_MOVE_ID;
        sLearningMoveTableID += 1;
        retVal = GiveMoveToMon(mon, gMoveToLearn) as u32;
    }
    retVal as u16
}
pub unsafe fn DeleteFirstMoveAndGiveMoveToMon(mon: *mut Pokemon, r#move: u16) {
    let mut moves: CArray<u16, 4> = zeroed();
    let mut pp: CArray<u8, 4> = zeroed();
    let mut i: i32 = 0;
    while i < 3 {
        moves[i] = GetMonData3(mon, MON_DATA_MOVE2 + i, null_mut()) as u16;
        pp[i] = GetMonData3(mon, MON_DATA_PP2 + i, null_mut()) as u8;
        i += 1;
    }
    let mut ppBonuses: u8 = GetMonData3(mon, MON_DATA_PP_BONUSES, null_mut()) as u8;
    ppBonuses >>= 2;
    moves[3] = r#move;
    pp[3] = gBattleMoves[r#move].pp;
    for i in 0..MAX_MON_MOVES {
        SetMonData(mon, MON_DATA_MOVE1 + i, &raw mut moves[i] as *mut c_void);
        SetMonData(mon, MON_DATA_PP1 + i, &raw mut pp[i] as *mut c_void);
    }
    SetMonData(mon, MON_DATA_PP_BONUSES, &raw mut ppBonuses as *mut c_void);
}
pub unsafe fn DeleteFirstMoveAndGiveMoveToBoxMon(boxMon: *mut BoxPokemon, r#move: u16) {
    let mut moves: CArray<u16, 4> = zeroed();
    let mut pp: CArray<u8, 4> = zeroed();
    let mut i: i32 = 0;
    while i < 3 {
        moves[i] = GetBoxMonData3(boxMon, MON_DATA_MOVE2 + i, null_mut()) as u16;
        pp[i] = GetBoxMonData3(boxMon, MON_DATA_PP2 + i, null_mut()) as u8;
        i += 1;
    }
    let mut ppBonuses: u8 = GetBoxMonData3(boxMon, MON_DATA_PP_BONUSES, null_mut()) as u8;
    ppBonuses >>= 2;
    moves[3] = r#move;
    pp[3] = gBattleMoves[r#move].pp;
    for i in 0..MAX_MON_MOVES {
        SetBoxMonData(boxMon, MON_DATA_MOVE1 + i, &raw mut moves[i] as *mut c_void);
        SetBoxMonData(boxMon, MON_DATA_PP1 + i, &raw mut pp[i] as *mut c_void);
    }
    SetBoxMonData(
        boxMon,
        MON_DATA_PP_BONUSES,
        &raw mut ppBonuses as *mut c_void,
    );
}
pub unsafe fn CalculateBaseDamage(
    attacker: *mut BattlePokemon,
    defender: *mut BattlePokemon,
    r#move: u32,
    sideStatus: u16,
    powerOverride: u16,
    typeOverride: u8,
    battlerIdAtk: u8,
    battlerIdDef: u8,
) -> i32 {
    let mut damage: i32 = 0;
    let mut damageHelper: i32 = 0;
    let mut r#type: u8 = 0;
    let mut attack: u16 = 0;
    let mut defense: u16 = 0;
    let mut spAttack: u16 = 0;
    let mut spDefense: u16 = 0;
    let mut defenderHoldEffect: u8 = 0;
    let mut defenderHoldEffectParam: u8 = 0;
    let mut attackerHoldEffect: u8 = 0;
    let mut attackerHoldEffectParam: u8 = 0;
    if powerOverride == 0 {
        gBattleMovePower = gBattleMoves[r#move].power as u16;
    } else {
        gBattleMovePower = powerOverride;
    }
    if typeOverride == 0 {
        r#type = gBattleMoves[r#move].r#type;
    } else {
        r#type = typeOverride & DYNAMIC_TYPE_MASK as u8;
    }
    attack = (*attacker).attack;
    defense = (*defender).defense;
    spAttack = (*attacker).spAttack;
    spDefense = (*defender).spDefense;
    if (*attacker).item == ITEM_ENIGMA_BERRY {
        attackerHoldEffect = gEnigmaBerries[battlerIdAtk].holdEffect;
        attackerHoldEffectParam = gEnigmaBerries[battlerIdAtk].holdEffectParam;
    } else {
        attackerHoldEffect = GetItemHoldEffect((*attacker).item);
        attackerHoldEffectParam = GetItemHoldEffectParam((*attacker).item);
    }
    if (*defender).item == ITEM_ENIGMA_BERRY {
        defenderHoldEffect = gEnigmaBerries[battlerIdDef].holdEffect;
        defenderHoldEffectParam = gEnigmaBerries[battlerIdDef].holdEffectParam;
    } else {
        defenderHoldEffect = GetItemHoldEffect((*defender).item);
        defenderHoldEffectParam = GetItemHoldEffectParam((*defender).item);
    }
    if (*attacker).ability == ABILITY_HUGE_POWER || (*attacker).ability == ABILITY_PURE_POWER {
        attack *= 2;
    }
    if ShouldGetStatBadgeBoost(FLAG_BADGE01_GET as u16, battlerIdAtk) != 0 {
        attack = (110 * attack as i32 / 100) as u16;
    }
    if ShouldGetStatBadgeBoost(FLAG_BADGE05_GET, battlerIdDef) != 0 {
        defense = (110 * defense as i32 / 100) as u16;
    }
    if ShouldGetStatBadgeBoost(FLAG_BADGE07_GET, battlerIdAtk) != 0 {
        spAttack = (110 * spAttack as i32 / 100) as u16;
    }
    if ShouldGetStatBadgeBoost(FLAG_BADGE07_GET, battlerIdDef) != 0 {
        spDefense = (110 * spDefense as i32 / 100) as u16;
    }
    for i in 0..17u32 {
        if attackerHoldEffect == sHoldEffectToType[i][0] && r#type == sHoldEffectToType[i][1] {
            if r#type < 9 {
                attack = (attack as i32 * (attackerHoldEffectParam as i32 + 100) / 100) as u16;
            } else {
                spAttack = (spAttack as i32 * (attackerHoldEffectParam as i32 + 100) / 100) as u16;
            }
            break;
        }
    }
    if attackerHoldEffect == HOLD_EFFECT_CHOICE_BAND {
        attack = (150 * attack as i32 / 100) as u16;
    }
    if attackerHoldEffect == HOLD_EFFECT_SOUL_DEW
        && gBattleTypeFlags & BATTLE_TYPE_FRONTIER == 0
        && ((*attacker).species == SPECIES_LATIAS || (*attacker).species == SPECIES_LATIOS)
    {
        spAttack = (150 * spAttack as i32 / 100) as u16;
    }
    if defenderHoldEffect == HOLD_EFFECT_SOUL_DEW
        && gBattleTypeFlags & BATTLE_TYPE_FRONTIER == 0
        && ((*defender).species == SPECIES_LATIAS || (*defender).species == SPECIES_LATIOS)
    {
        spDefense = (150 * spDefense as i32 / 100) as u16;
    }
    if attackerHoldEffect == HOLD_EFFECT_DEEP_SEA_TOOTH && (*attacker).species == SPECIES_CLAMPERL {
        spAttack *= 2;
    }
    if defenderHoldEffect == HOLD_EFFECT_DEEP_SEA_SCALE && (*defender).species == SPECIES_CLAMPERL {
        spDefense *= 2;
    }
    if attackerHoldEffect == HOLD_EFFECT_LIGHT_BALL && (*attacker).species == SPECIES_PIKACHU {
        spAttack *= 2;
    }
    if defenderHoldEffect == HOLD_EFFECT_METAL_POWDER && (*defender).species == SPECIES_DITTO {
        defense *= 2;
    }
    if attackerHoldEffect == HOLD_EFFECT_THICK_CLUB
        && ((*attacker).species == SPECIES_CUBONE || (*attacker).species == SPECIES_MAROWAK)
    {
        attack *= 2;
    }
    if (*defender).ability == ABILITY_THICK_FAT && (r#type == TYPE_FIRE || r#type == TYPE_ICE) {
        spAttack = (spAttack as i32 / 2) as u16;
    }
    if (*attacker).ability == ABILITY_HUSTLE {
        attack = (150 * attack as i32 / 100) as u16;
    }
    if (*attacker).ability == ABILITY_PLUS && AbilityBattleEffects(14, 0, ABILITY_MINUS, 0, 0) != 0
    {
        spAttack = (150 * spAttack as i32 / 100) as u16;
    }
    if (*attacker).ability == ABILITY_MINUS && AbilityBattleEffects(14, 0, ABILITY_PLUS, 0, 0) != 0
    {
        spAttack = (150 * spAttack as i32 / 100) as u16;
    }
    if (*attacker).ability == ABILITY_GUTS && (*attacker).status1 != 0 {
        attack = (150 * attack as i32 / 100) as u16;
    }
    if (*defender).ability == ABILITY_MARVEL_SCALE && (*defender).status1 != 0 {
        defense = (150 * defense as i32 / 100) as u16;
    }
    if r#type == TYPE_ELECTRIC
        && AbilityBattleEffects(ABILITYEFFECT_FIELD_SPORT, 0, 0, ABILITYEFFECT_MUD_SPORT, 0) != 0
    {
        gBattleMovePower = (gBattleMovePower as i32 / 2) as u16;
    }
    if r#type == TYPE_FIRE
        && AbilityBattleEffects(
            ABILITYEFFECT_FIELD_SPORT,
            0,
            0,
            ABILITYEFFECT_WATER_SPORT,
            0,
        ) != 0
    {
        gBattleMovePower = (gBattleMovePower as i32 / 2) as u16;
    }
    if r#type == TYPE_GRASS
        && (*attacker).ability == ABILITY_OVERGROW
        && (*attacker).hp as i32 <= (*attacker).maxHP as i32 / 3
    {
        gBattleMovePower = (150 * gBattleMovePower as i32 / 100) as u16;
    }
    if r#type == TYPE_FIRE
        && (*attacker).ability == ABILITY_BLAZE
        && (*attacker).hp as i32 <= (*attacker).maxHP as i32 / 3
    {
        gBattleMovePower = (150 * gBattleMovePower as i32 / 100) as u16;
    }
    if r#type == TYPE_WATER
        && (*attacker).ability == ABILITY_TORRENT
        && (*attacker).hp as i32 <= (*attacker).maxHP as i32 / 3
    {
        gBattleMovePower = (150 * gBattleMovePower as i32 / 100) as u16;
    }
    if r#type == TYPE_BUG
        && (*attacker).ability == ABILITY_SWARM
        && (*attacker).hp as i32 <= (*attacker).maxHP as i32 / 3
    {
        gBattleMovePower = (150 * gBattleMovePower as i32 / 100) as u16;
    }
    if gBattleMoves[gCurrentMove].effect == EFFECT_EXPLOSION {
        defense = (defense as i32 / 2) as u16;
    }
    if r#type < 9 {
        if gCritMultiplier == 2 {
            if (*attacker).statStages[1] > DEFAULT_STAT_STAGE {
                damage = attack as i32 * gStatStageRatios[(*attacker).statStages[1]][0] as i32;
                damage = div_i32(
                    damage,
                    gStatStageRatios[(*attacker).statStages[1]][1] as i32,
                );
            } else {
                damage = attack as i32;
            }
        } else {
            damage = attack as i32 * gStatStageRatios[(*attacker).statStages[1]][0] as i32;
            damage = div_i32(
                damage,
                gStatStageRatios[(*attacker).statStages[1]][1] as i32,
            );
        }
        damage *= gBattleMovePower as i32;
        damage *= 2 * (*attacker).level as i32 / 5 + 2;
        if gCritMultiplier == 2 {
            if (*defender).statStages[2] < DEFAULT_STAT_STAGE {
                damageHelper =
                    defense as i32 * gStatStageRatios[(*defender).statStages[2]][0] as i32;
                damageHelper = div_i32(
                    damageHelper,
                    gStatStageRatios[(*defender).statStages[2]][1] as i32,
                );
            } else {
                damageHelper = defense as i32;
            }
        } else {
            damageHelper = defense as i32 * gStatStageRatios[(*defender).statStages[2]][0] as i32;
            damageHelper = div_i32(
                damageHelper,
                gStatStageRatios[(*defender).statStages[2]][1] as i32,
            );
        }
        damage = div_i32(damage, damageHelper);
        damage /= 50;
        if (*attacker).status1 & STATUS1_BURN != 0 && (*attacker).ability != ABILITY_GUTS {
            damage /= 2;
        }
        if sideStatus as i32 & 1 != 0 && gCritMultiplier == 1 {
            if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 && CountAliveMonsInBattle(2) == 2 {
                damage = 2 * (damage / 3);
            } else {
                damage /= 2;
            }
        }
        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
            && gBattleMoves[r#move].target == MOVE_TARGET_BOTH
            && CountAliveMonsInBattle(2) == 2
        {
            damage /= 2;
        }
        if damage == 0 {
            damage = 1;
        }
    }
    if r#type == TYPE_MYSTERY {
        damage = 0;
    }
    if r#type > 9 {
        if gCritMultiplier == 2 {
            if (*attacker).statStages[4] > DEFAULT_STAT_STAGE {
                damage = spAttack as i32 * gStatStageRatios[(*attacker).statStages[4]][0] as i32;
                damage = div_i32(
                    damage,
                    gStatStageRatios[(*attacker).statStages[4]][1] as i32,
                );
            } else {
                damage = spAttack as i32;
            }
        } else {
            damage = spAttack as i32 * gStatStageRatios[(*attacker).statStages[4]][0] as i32;
            damage = div_i32(
                damage,
                gStatStageRatios[(*attacker).statStages[4]][1] as i32,
            );
        }
        damage *= gBattleMovePower as i32;
        damage *= 2 * (*attacker).level as i32 / 5 + 2;
        if gCritMultiplier == 2 {
            if (*defender).statStages[5] < DEFAULT_STAT_STAGE {
                damageHelper =
                    spDefense as i32 * gStatStageRatios[(*defender).statStages[5]][0] as i32;
                damageHelper = div_i32(
                    damageHelper,
                    gStatStageRatios[(*defender).statStages[5]][1] as i32,
                );
            } else {
                damageHelper = spDefense as i32;
            }
        } else {
            damageHelper = spDefense as i32 * gStatStageRatios[(*defender).statStages[5]][0] as i32;
            damageHelper = div_i32(
                damageHelper,
                gStatStageRatios[(*defender).statStages[5]][1] as i32,
            );
        }
        damage = div_i32(damage, damageHelper);
        damage /= 50;
        if sideStatus as i32 & SIDE_STATUS_LIGHTSCREEN != 0 && gCritMultiplier == 1 {
            if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 && CountAliveMonsInBattle(2) == 2 {
                damage = 2 * (damage / 3);
            } else {
                damage /= 2;
            }
        }
        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
            && gBattleMoves[r#move].target == MOVE_TARGET_BOTH
            && CountAliveMonsInBattle(2) == 2
        {
            damage /= 2;
        }
        if AbilityBattleEffects(14, 0, 13, 0, 0) == 0 && AbilityBattleEffects(14, 0, 77, 0, 0) == 0
        {
            if gBattleWeather as i32 & B_WEATHER_RAIN_TEMPORARY as i32 != 0 {
                match r#type {
                    TYPE_FIRE => {
                        damage /= 2;
                    }
                    TYPE_WATER => {
                        damage = 15 * damage / 10;
                    }
                    _ => {}
                }
            }
            if gBattleWeather as i32 & 159 != 0 && gCurrentMove == MOVE_SOLAR_BEAM {
                damage /= 2;
            }
            if gBattleWeather as i32 & B_WEATHER_SUN != 0 {
                match r#type {
                    TYPE_FIRE => {
                        damage = 15 * damage / 10;
                    }
                    TYPE_WATER => {
                        damage /= 2;
                    }
                    _ => {}
                }
            }
        }
        if (*(*gBattleResources).flags).flags[battlerIdAtk] & RESOURCE_FLAG_FLASH_FIRE != 0
            && r#type == TYPE_FIRE
        {
            damage = 15 * damage / 10;
        }
    }
    damage + 2
}
pub unsafe fn CountAliveMonsInBattle(caseId: u8) -> u8 {
    let mut retVal: u8 = 0;
    match caseId {
        BATTLE_ALIVE_EXCEPT_ACTIVE => {
            for i in 0..(MAX_BATTLERS_COUNT as i32) {
                if i != gActiveBattler as i32
                    && gAbsentBattlerFlags as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                        == 0
                {
                    retVal += 1;
                }
            }
        }
        BATTLE_ALIVE_ATK_SIDE => {
            for i in 0..(MAX_BATTLERS_COUNT as i32) {
                if GetBattlerSide(i as u8) == GetBattlerSide(gBattlerAttacker)
                    && gAbsentBattlerFlags as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                        == 0
                {
                    retVal += 1;
                }
            }
        }
        BATTLE_ALIVE_DEF_SIDE => {
            for i in 0..(MAX_BATTLERS_COUNT as i32) {
                if GetBattlerSide(i as u8) == GetBattlerSide(gBattlerTarget)
                    && gAbsentBattlerFlags as u32
                        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                        == 0
                {
                    retVal += 1;
                }
            }
        }
        _ => {}
    }
    retVal
}
unsafe fn ShouldGetStatBadgeBoost(badgeFlag: u16, battler: u8) -> u8 {
    if gBattleTypeFlags & 0x23f0902 != 0 {
        return FALSE;
    } else if GetBattlerSide(battler) != B_SIDE_PLAYER {
        return FALSE;
    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0
        && gTrainerBattleOpponent_A == TRAINER_SECRET_BASE
    {
        return FALSE;
    } else if FlagGet(badgeFlag) != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetDefaultMoveTarget(battler: u8) -> u8 {
    let opposing: u8 = GetBattlerPosition(battler) & 1 ^ 1;
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE == 0 {
        return GetBattlerAtPosition(opposing);
    }
    if CountAliveMonsInBattle(BATTLE_ALIVE_EXCEPT_ACTIVE) > 1 {
        let mut position: u8 = 0;
        if Random() as i32 & 1 == 0 {
            position = opposing ^ 2;
        } else {
            position = opposing;
        }
        return GetBattlerAtPosition(position);
    } else {
        if gAbsentBattlerFlags as u32
            & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[opposing]
            != 0
        {
            return GetBattlerAtPosition(opposing ^ 2);
        } else {
            return GetBattlerAtPosition(opposing);
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetMonGender(mon: *mut Pokemon) -> u8 {
    GetBoxMonGender(&raw mut (*mon).r#box)
}
pub unsafe fn GetBoxMonGender(boxMon: *mut BoxPokemon) -> u8 {
    let species: u16 = GetBoxMonData3(boxMon, MON_DATA_SPECIES, null_mut()) as u16;
    let personality: u32 = GetBoxMonData3(boxMon, MON_DATA_PERSONALITY, null_mut());
    match gSpeciesInfo[species].genderRatio {
        MON_MALE | MON_FEMALE | MON_GENDERLESS => {
            return gSpeciesInfo[species].genderRatio;
        }
        _ => {}
    }
    if gSpeciesInfo[species].genderRatio as u32 > personality & 0xFF {
        return MON_FEMALE;
    } else {
        return MON_MALE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetGenderFromSpeciesAndPersonality(species: u16, personality: u32) -> u8 {
    match gSpeciesInfo[species].genderRatio {
        MON_MALE | MON_FEMALE | MON_GENDERLESS => {
            return gSpeciesInfo[species].genderRatio;
        }
        _ => {}
    }
    if gSpeciesInfo[species].genderRatio as u32 > personality & 0xFF {
        return MON_FEMALE;
    } else {
        return MON_MALE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetMultiuseSpriteTemplateToPokemon(speciesTag: u16, battlerPosition: u8) {
    if !gMonSpritesGfxPtr.is_null() {
        gMultiuseSpriteTemplate = (*gMonSpritesGfxPtr).templates[battlerPosition];
    } else if !sMonSpritesGfxManagers[0].is_null() {
        gMultiuseSpriteTemplate = *(*sMonSpritesGfxManagers[0]).templates.at(battlerPosition);
    } else if !sMonSpritesGfxManagers[1].is_null() {
        gMultiuseSpriteTemplate = *(*sMonSpritesGfxManagers[1]).templates.at(battlerPosition);
    } else {
        gMultiuseSpriteTemplate = gBattlerSpriteTemplates[battlerPosition];
    }
    gMultiuseSpriteTemplate.paletteTag = speciesTag;
    if battlerPosition == B_POSITION_PLAYER_LEFT || battlerPosition == B_POSITION_PLAYER_RIGHT {
        gMultiuseSpriteTemplate.anims = (*(&raw const crate::data::data_tables::gAnims_MonPic)
            .cast::<CArray<*mut AnimCmd, 0>>())
        .as_ptr()
        .cast_mut();
    } else if speciesTag > SPECIES_SHINY_TAG {
        gMultiuseSpriteTemplate.anims =
            (*(&raw const crate::data::data_tables::gMonFrontAnimsPtrTable)
                .cast::<CArray<*mut *mut AnimCmd, 0>>())
                [speciesTag as i32 - SPECIES_SHINY_TAG as i32];
    } else {
        gMultiuseSpriteTemplate.anims =
            (*(&raw const crate::data::data_tables::gMonFrontAnimsPtrTable)
                .cast::<CArray<*mut *mut AnimCmd, 0>>())[speciesTag];
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetMultiuseSpriteTemplateToTrainerBack(trainerPicId: u16, battlerPosition: u8) {
    gMultiuseSpriteTemplate.paletteTag = trainerPicId;
    if battlerPosition == B_POSITION_PLAYER_LEFT || battlerPosition == B_POSITION_PLAYER_RIGHT {
        gMultiuseSpriteTemplate = sTrainerBackSpriteTemplates[trainerPicId];
        gMultiuseSpriteTemplate.anims =
            (*(&raw const crate::data::data_tables::gTrainerBackAnimsPtrTable)
                .cast::<CArray<*mut *mut AnimCmd, 0>>())[trainerPicId];
    } else {
        if !gMonSpritesGfxPtr.is_null() {
            gMultiuseSpriteTemplate = (*gMonSpritesGfxPtr).templates[battlerPosition];
        } else {
            gMultiuseSpriteTemplate = gBattlerSpriteTemplates[battlerPosition];
        }
        gMultiuseSpriteTemplate.anims =
            (*(&raw const crate::data::data_tables::gTrainerFrontAnimsPtrTable)
                .cast::<CArray<*mut *mut AnimCmd, 0>>())[trainerPicId];
    }
}
pub unsafe fn SetMultiuseSpriteTemplateToTrainerFront(trainerPicId: u16, battlerPosition: u8) {
    if !gMonSpritesGfxPtr.is_null() {
        gMultiuseSpriteTemplate = (*gMonSpritesGfxPtr).templates[battlerPosition];
    } else {
        gMultiuseSpriteTemplate = gBattlerSpriteTemplates[battlerPosition];
    }
    gMultiuseSpriteTemplate.paletteTag = trainerPicId;
    gMultiuseSpriteTemplate.anims =
        (*(&raw const crate::data::data_tables::gTrainerFrontAnimsPtrTable)
            .cast::<CArray<*mut *mut AnimCmd, 0>>())[trainerPicId];
}
unsafe fn EncryptBoxMon(boxMon: *mut BoxPokemon) {
    for i in 0..12u32 {
        (*boxMon).secure.raw[i] ^= (*boxMon).personality;
        (*boxMon).secure.raw[i] ^= (*boxMon).otId;
    }
}
unsafe fn DecryptBoxMon(boxMon: *mut BoxPokemon) {
    for i in 0..12u32 {
        (*boxMon).secure.raw[i] ^= (*boxMon).otId;
        (*boxMon).secure.raw[i] ^= (*boxMon).personality;
    }
}
unsafe fn GetSubstruct(
    boxMon: *mut BoxPokemon,
    personality: u32,
    substructType: u8,
) -> *mut PokemonSubstruct {
    let mut substruct: *mut PokemonSubstruct = null_mut();
    'l1: {
        match personality % 24 {
            0 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs0;
                    }
                    1 => {
                        substruct = substructs0.at(1);
                    }
                    2 => {
                        substruct = substructs0.at(2);
                    }
                    3 => {
                        substruct = substructs0.at(3);
                    }
                    _ => {}
                }
                break 'l1;
            }
            1 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs1;
                    }
                    1 => {
                        substruct = substructs1.at(1);
                    }
                    2 => {
                        substruct = substructs1.at(3);
                    }
                    3 => {
                        substruct = substructs1.at(2);
                    }
                    _ => {}
                }
                break 'l1;
            }
            2 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs2;
                    }
                    1 => {
                        substruct = substructs2.at(2);
                    }
                    2 => {
                        substruct = substructs2.at(1);
                    }
                    3 => {
                        substruct = substructs2.at(3);
                    }
                    _ => {}
                }
                break 'l1;
            }
            3 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs3;
                    }
                    1 => {
                        substruct = substructs3.at(3);
                    }
                    2 => {
                        substruct = substructs3.at(1);
                    }
                    3 => {
                        substruct = substructs3.at(2);
                    }
                    _ => {}
                }
                break 'l1;
            }
            4 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs4;
                    }
                    1 => {
                        substruct = substructs4.at(2);
                    }
                    2 => {
                        substruct = substructs4.at(3);
                    }
                    3 => {
                        substruct = substructs4.at(1);
                    }
                    _ => {}
                }
                break 'l1;
            }
            5 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs5;
                    }
                    1 => {
                        substruct = substructs5.at(3);
                    }
                    2 => {
                        substruct = substructs5.at(2);
                    }
                    3 => {
                        substruct = substructs5.at(1);
                    }
                    _ => {}
                }
                break 'l1;
            }
            6 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs6.at(1);
                    }
                    1 => {
                        substruct = substructs6;
                    }
                    2 => {
                        substruct = substructs6.at(2);
                    }
                    3 => {
                        substruct = substructs6.at(3);
                    }
                    _ => {}
                }
                break 'l1;
            }
            7 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs7.at(1);
                    }
                    1 => {
                        substruct = substructs7;
                    }
                    2 => {
                        substruct = substructs7.at(3);
                    }
                    3 => {
                        substruct = substructs7.at(2);
                    }
                    _ => {}
                }
                break 'l1;
            }
            8 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs8.at(2);
                    }
                    1 => {
                        substruct = substructs8;
                    }
                    2 => {
                        substruct = substructs8.at(1);
                    }
                    3 => {
                        substruct = substructs8.at(3);
                    }
                    _ => {}
                }
                break 'l1;
            }
            9 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs9.at(3);
                    }
                    1 => {
                        substruct = substructs9;
                    }
                    2 => {
                        substruct = substructs9.at(1);
                    }
                    3 => {
                        substruct = substructs9.at(2);
                    }
                    _ => {}
                }
                break 'l1;
            }
            10 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs10.at(2);
                    }
                    1 => {
                        substruct = substructs10;
                    }
                    2 => {
                        substruct = substructs10.at(3);
                    }
                    3 => {
                        substruct = substructs10.at(1);
                    }
                    _ => {}
                }
                break 'l1;
            }
            11 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs11.at(3);
                    }
                    1 => {
                        substruct = substructs11;
                    }
                    2 => {
                        substruct = substructs11.at(2);
                    }
                    3 => {
                        substruct = substructs11.at(1);
                    }
                    _ => {}
                }
                break 'l1;
            }
            12 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs12.at(1);
                    }
                    1 => {
                        substruct = substructs12.at(2);
                    }
                    2 => {
                        substruct = substructs12;
                    }
                    3 => {
                        substruct = substructs12.at(3);
                    }
                    _ => {}
                }
                break 'l1;
            }
            13 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs13.at(1);
                    }
                    1 => {
                        substruct = substructs13.at(3);
                    }
                    2 => {
                        substruct = substructs13;
                    }
                    3 => {
                        substruct = substructs13.at(2);
                    }
                    _ => {}
                }
                break 'l1;
            }
            14 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs14.at(2);
                    }
                    1 => {
                        substruct = substructs14.at(1);
                    }
                    2 => {
                        substruct = substructs14;
                    }
                    3 => {
                        substruct = substructs14.at(3);
                    }
                    _ => {}
                }
                break 'l1;
            }
            15 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs15.at(3);
                    }
                    1 => {
                        substruct = substructs15.at(1);
                    }
                    2 => {
                        substruct = substructs15;
                    }
                    3 => {
                        substruct = substructs15.at(2);
                    }
                    _ => {}
                }
                break 'l1;
            }
            16 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs16.at(2);
                    }
                    1 => {
                        substruct = substructs16.at(3);
                    }
                    2 => {
                        substruct = substructs16;
                    }
                    3 => {
                        substruct = substructs16.at(1);
                    }
                    _ => {}
                }
                break 'l1;
            }
            17 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs17.at(3);
                    }
                    1 => {
                        substruct = substructs17.at(2);
                    }
                    2 => {
                        substruct = substructs17;
                    }
                    3 => {
                        substruct = substructs17.at(1);
                    }
                    _ => {}
                }
                break 'l1;
            }
            18 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs18.at(1);
                    }
                    1 => {
                        substruct = substructs18.at(2);
                    }
                    2 => {
                        substruct = substructs18.at(3);
                    }
                    3 => {
                        substruct = substructs18;
                    }
                    _ => {}
                }
                break 'l1;
            }
            19 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs19.at(1);
                    }
                    1 => {
                        substruct = substructs19.at(3);
                    }
                    2 => {
                        substruct = substructs19.at(2);
                    }
                    3 => {
                        substruct = substructs19;
                    }
                    _ => {}
                }
                break 'l1;
            }
            20 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs20.at(2);
                    }
                    1 => {
                        substruct = substructs20.at(1);
                    }
                    2 => {
                        substruct = substructs20.at(3);
                    }
                    3 => {
                        substruct = substructs20;
                    }
                    _ => {}
                }
                break 'l1;
            }
            21 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs21.at(3);
                    }
                    1 => {
                        substruct = substructs21.at(1);
                    }
                    2 => {
                        substruct = substructs21.at(2);
                    }
                    3 => {
                        substruct = substructs21;
                    }
                    _ => {}
                }
                break 'l1;
            }
            22 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs22.at(2);
                    }
                    1 => {
                        substruct = substructs22.at(3);
                    }
                    2 => {
                        substruct = substructs22.at(1);
                    }
                    3 => {
                        substruct = substructs22;
                    }
                    _ => {}
                }
                break 'l1;
            }
            23 => {
                let substructs0: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs1: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs2: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs3: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs4: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs5: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs6: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs7: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs8: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs9: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs10: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs11: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs12: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs13: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs14: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs15: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs16: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs17: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs18: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs19: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs20: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs21: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs22: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                let substructs23: *mut PokemonSubstruct = (*boxMon).secure.substructs.as_mut_ptr();
                match substructType {
                    0 => {
                        substruct = substructs23.at(3);
                    }
                    1 => {
                        substruct = substructs23.at(2);
                    }
                    2 => {
                        substruct = substructs23.at(1);
                    }
                    3 => {
                        substruct = substructs23;
                    }
                    _ => {}
                }
                break 'l1;
            }
            _ => {}
        }
    }
    substruct
}
#[unsafe(no_mangle)]
pub unsafe fn GetMonData3(mon: *mut Pokemon, field: i32, data: *mut u8) -> u32 {
    let mut ret: u32 = 0;
    match field {
        MON_DATA_STATUS => {
            ret = (*mon).status;
        }
        MON_DATA_LEVEL => {
            ret = (*mon).level as u32;
        }
        MON_DATA_HP => {
            ret = (*mon).hp as u32;
        }
        MON_DATA_MAX_HP => {
            ret = (*mon).maxHP as u32;
        }
        MON_DATA_ATK => {
            ret = GetDeoxysStat(mon, STAT_ATK as i32) as u32;
            if ret == 0 {
                ret = (*mon).attack as u32;
            }
        }
        MON_DATA_DEF => {
            ret = GetDeoxysStat(mon, STAT_DEF) as u32;
            if ret == 0 {
                ret = (*mon).defense as u32;
            }
        }
        MON_DATA_SPEED => {
            ret = GetDeoxysStat(mon, STAT_SPEED as i32) as u32;
            if ret == 0 {
                ret = (*mon).speed as u32;
            }
        }
        MON_DATA_SPATK => {
            ret = GetDeoxysStat(mon, STAT_SPATK as i32) as u32;
            if ret == 0 {
                ret = (*mon).spAttack as u32;
            }
        }
        MON_DATA_SPDEF => {
            ret = GetDeoxysStat(mon, STAT_SPDEF) as u32;
            if ret == 0 {
                ret = (*mon).spDefense as u32;
            }
        }
        MON_DATA_ATK2 => {
            ret = (*mon).attack as u32;
        }
        MON_DATA_DEF2 => {
            ret = (*mon).defense as u32;
        }
        MON_DATA_SPEED2 => {
            ret = (*mon).speed as u32;
        }
        MON_DATA_SPATK2 => {
            ret = (*mon).spAttack as u32;
        }
        MON_DATA_SPDEF2 => {
            ret = (*mon).spDefense as u32;
        }
        MON_DATA_MAIL => {
            ret = (*mon).mail as u32;
        }
        _ => {
            ret = GetBoxMonData3(&raw mut (*mon).r#box, field, data);
        }
    }
    ret
}
#[unsafe(no_mangle)]
pub unsafe fn GetMonData2(mon: *mut Pokemon, field: i32) -> u32 {
    GetMonData3(mon, field, null_mut())
}
#[unsafe(no_mangle)]
pub unsafe fn SetMonData(mon: *mut Pokemon, field: i32, dataArg: *mut c_void) {
    let data: *mut u8 = dataArg as *mut u8;
    match field {
        MON_DATA_STATUS => {
            (*mon).status = *data as u32
                + ((*data.at(1) as u32) << 8)
                + ((*data.at(2) as u32) << 16)
                + ((*data.at(3) as u32) << 24);
        }
        MON_DATA_LEVEL => {
            (*mon).level = *data;
        }
        MON_DATA_HP => {
            (*mon).hp = *data as u16 + ((*data.at(1) as u16) << 8);
        }
        MON_DATA_MAX_HP => {
            (*mon).maxHP = *data as u16 + ((*data.at(1) as u16) << 8);
        }
        MON_DATA_ATK => {
            (*mon).attack = *data as u16 + ((*data.at(1) as u16) << 8);
        }
        MON_DATA_DEF => {
            (*mon).defense = *data as u16 + ((*data.at(1) as u16) << 8);
        }
        MON_DATA_SPEED => {
            (*mon).speed = *data as u16 + ((*data.at(1) as u16) << 8);
        }
        MON_DATA_SPATK => {
            (*mon).spAttack = *data as u16 + ((*data.at(1) as u16) << 8);
        }
        MON_DATA_SPDEF => {
            (*mon).spDefense = *data as u16 + ((*data.at(1) as u16) << 8);
        }
        MON_DATA_MAIL => {
            (*mon).mail = *data;
        }
        MON_DATA_SPECIES_OR_EGG => {}
        _ => {
            SetBoxMonData(&raw mut (*mon).r#box, field, data as *mut c_void);
        }
    }
}
pub unsafe fn CopyMon(dest: *mut c_void, src: *mut c_void, size: u32) {
    memcpy(dest as *mut u8, src as *mut u8, size);
}
#[unsafe(no_mangle)]
pub unsafe fn GiveMonToPlayer(mon: *mut Pokemon) -> u8 {
    SetMonData(
        mon,
        MON_DATA_OT_NAME,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr() as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_OT_GENDER,
        &raw mut (*gSaveBlock2Ptr).playerGender as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_OT_ID,
        (*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr() as *mut c_void,
    );
    let mut i: i32 = 0;
    while i < PARTY_SIZE {
        if GetMonData3(&raw mut gPlayerParty[i], MON_DATA_SPECIES, null_mut())
            == SPECIES_NONE as u32
        {
            break;
        }
        i += 1;
    }
    if i >= PARTY_SIZE {
        return CopyMonToPC(mon);
    }
    CopyMon(
        &raw mut gPlayerParty[i] as *mut c_void,
        mon as *mut c_void,
        100,
    );
    gPlayerPartyCount = i as u8 + 1;
    MON_GIVEN_TO_PARTY
}
unsafe fn CopyMonToPC(mon: *mut Pokemon) -> u8 {
    SetPCBoxToSendMon(VarGet(VAR_PC_BOX_TO_SEND_MON) as u8);
    let mut boxNo: i32 = StorageGetCurrentBox() as i32;
    loop {
        for boxPos in 0..IN_BOX_COUNT {
            let checkingMon: *mut BoxPokemon = GetBoxedMonPtr(boxNo as u8, boxPos as u8);
            if GetBoxMonData3(checkingMon, MON_DATA_SPECIES, null_mut()) == SPECIES_NONE as u32 {
                MonRestorePP(mon);
                CopyMon(
                    checkingMon as *mut c_void,
                    &raw mut (*mon).r#box as *mut c_void,
                    80,
                );
                gSpecialVar_MonBoxId = boxNo as u16;
                gSpecialVar_MonBoxPos = boxPos as u16;
                if GetPCBoxToSendMon() as i32 != boxNo {
                    FlagClear(FLAG_SHOWN_BOX_WAS_FULL_MESSAGE);
                }
                VarSet(VAR_PC_BOX_TO_SEND_MON, boxNo as u16);
                return MON_GIVEN_TO_PC;
            }
        }
        boxNo += 1;
        if boxNo == TOTAL_BOXES_COUNT as i32 {
            boxNo = 0;
        }
        if boxNo == StorageGetCurrentBox() as i32 {
            break;
        }
    }
    MON_CANT_GIVE
}
#[unsafe(no_mangle)]
pub unsafe fn CalculatePlayerPartyCount() -> u8 {
    gPlayerPartyCount = 0;
    while gPlayerPartyCount < PARTY_SIZE as u8
        && GetMonData3(
            &raw mut gPlayerParty[gPlayerPartyCount],
            MON_DATA_SPECIES,
            null_mut(),
        ) != SPECIES_NONE as u32
    {
        gPlayerPartyCount += 1;
    }
    gPlayerPartyCount
}
pub unsafe fn CalculateEnemyPartyCount() -> u8 {
    gEnemyPartyCount = 0;
    while gEnemyPartyCount < PARTY_SIZE as u8
        && GetMonData3(
            &raw mut gEnemyParty[gEnemyPartyCount],
            MON_DATA_SPECIES,
            null_mut(),
        ) != SPECIES_NONE as u32
    {
        gEnemyPartyCount += 1;
    }
    gEnemyPartyCount
}
#[unsafe(no_mangle)]
pub unsafe fn GetMonsStateToDoubles() -> u8 {
    let mut aliveCount: i32 = 0;
    CalculatePlayerPartyCount();
    if gPlayerPartyCount == 1 {
        return gPlayerPartyCount;
    }
    let mut i: i32 = 0;
    while i < gPlayerPartyCount as i32 {
        if GetMonData3(
            &raw mut gPlayerParty[i],
            MON_DATA_SPECIES_OR_EGG,
            null_mut(),
        ) != SPECIES_EGG
            && GetMonData3(&raw mut gPlayerParty[i], MON_DATA_HP, null_mut()) != 0
            && GetMonData3(
                &raw mut gPlayerParty[i],
                MON_DATA_SPECIES_OR_EGG,
                null_mut(),
            ) != SPECIES_NONE as u32
        {
            aliveCount += 1;
        }
        i += 1;
    }
    (if aliveCount > 1 {
        PLAYER_HAS_TWO_USABLE_MONS
    } else {
        PLAYER_HAS_ONE_USABLE_MON
    }) as u8
}
pub unsafe fn GetMonsStateToDoubles_2() -> u8 {
    let mut aliveCount: i32 = 0;
    for i in 0..PARTY_SIZE {
        let species: u32 = GetMonData3(
            &raw mut gPlayerParty[i],
            MON_DATA_SPECIES_OR_EGG,
            null_mut(),
        );
        if species != SPECIES_EGG
            && species != SPECIES_NONE as u32
            && GetMonData3(&raw mut gPlayerParty[i], MON_DATA_HP, null_mut()) != 0
        {
            aliveCount += 1;
        }
    }
    if aliveCount == 1 {
        return PLAYER_HAS_ONE_MON;
    }
    (if aliveCount > 1 {
        PLAYER_HAS_TWO_USABLE_MONS
    } else {
        PLAYER_HAS_ONE_USABLE_MON
    }) as u8
}
pub unsafe fn GetAbilityBySpecies(species: u16, abilityNum: u8) -> u8 {
    if abilityNum != 0 {
        gLastUsedAbility = gSpeciesInfo[species].abilities[1];
    } else {
        gLastUsedAbility = gSpeciesInfo[species].abilities[0];
    }
    gLastUsedAbility
}
pub unsafe fn GetMonAbility(mon: *mut Pokemon) -> u8 {
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let abilityNum: u8 = GetMonData3(mon, MON_DATA_ABILITY_NUM, null_mut()) as u8;
    GetAbilityBySpecies(species, abilityNum)
}
#[unsafe(no_mangle)]
pub unsafe fn CreateSecretBaseEnemyParty(secretBaseRecord: *mut SecretBase) {
    let mut j: i32 = 0;
    ZeroEnemyPartyMons();
    *(*gBattleResources).secretBase = *secretBaseRecord;
    for i in 0..PARTY_SIZE {
        if (*(*gBattleResources).secretBase).party.species[i] != 0 {
            CreateMon(
                &raw mut gEnemyParty[i],
                (*(*gBattleResources).secretBase).party.species[i],
                (*(*gBattleResources).secretBase).party.levels[i],
                15,
                TRUE,
                (*(*gBattleResources).secretBase).party.personality[i],
                OT_ID_RANDOM_NO_SHINY,
                0,
            );
            SetMonData(
                &raw mut gEnemyParty[i],
                MON_DATA_HELD_ITEM,
                &raw mut (*(*gBattleResources).secretBase).party.heldItems[i] as *mut c_void,
            );
            j = 0;
            while j < NUM_STATS {
                SetMonData(
                    &raw mut gEnemyParty[i],
                    MON_DATA_HP_EV + j,
                    &raw mut (*(*gBattleResources).secretBase).party.EVs[i] as *mut c_void,
                );
                j += 1;
            }
            for j in 0..MAX_MON_MOVES {
                SetMonData(
                    &raw mut gEnemyParty[i],
                    MON_DATA_MOVE1 + j,
                    &raw mut (*(*gBattleResources).secretBase).party.moves[i * MAX_MON_MOVES + j]
                        as *mut c_void,
                );
                SetMonData(
                    &raw mut gEnemyParty[i],
                    MON_DATA_PP1 + j,
                    (&raw const gBattleMoves
                        [(*(*gBattleResources).secretBase).party.moves[i * MAX_MON_MOVES + j]]
                        .pp)
                        .cast_mut() as *mut c_void,
                );
            }
        }
    }
}
pub unsafe fn GetSecretBaseTrainerPicIndex() -> u8 {
    let facilityClass: u8 = sSecretBaseFacilityClasses[(*(*gBattleResources).secretBase).gender()]
        [(*(*gBattleResources).secretBase).trainerId[0] as i32 % 5];
    gFacilityClassToPicIndex[facilityClass]
}
pub unsafe fn GetSecretBaseTrainerClass() -> u8 {
    let facilityClass: u8 = sSecretBaseFacilityClasses[(*(*gBattleResources).secretBase).gender()]
        [(*(*gBattleResources).secretBase).trainerId[0] as i32 % 5];
    gFacilityClassToTrainerClass[facilityClass]
}
pub unsafe fn IsPlayerPartyAndPokemonStorageFull() -> u8 {
    for i in 0..PARTY_SIZE {
        if GetMonData3(&raw mut gPlayerParty[i], MON_DATA_SPECIES, null_mut())
            == SPECIES_NONE as u32
        {
            return FALSE;
        }
    }
    IsPokemonStorageFull()
}
pub unsafe fn IsPokemonStorageFull() -> u8 {
    for i in 0..(TOTAL_BOXES_COUNT as i32) {
        for j in 0..IN_BOX_COUNT {
            if GetBoxMonDataAt(i as u8, j as u8, MON_DATA_SPECIES) == SPECIES_NONE as u32 {
                return FALSE;
            }
        }
    }
    TRUE
}
pub unsafe fn GetSpeciesName(name: *mut u8, species: u16) {
    let mut i: i32 = 0;
    while i <= POKEMON_NAME_LENGTH as i32 {
        if species > NUM_SPECIES {
            *name.at(i) = (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[0][i];
        } else {
            *name.at(i) = (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[species][i];
        }
        if *name.at(i) == EOS {
            break;
        }
        i += 1;
    }
    *name.at(i) = EOS;
}
#[unsafe(no_mangle)]
pub unsafe fn CalculatePPWithBonus(r#move: u16, ppBonuses: u8, moveIndex: u8) -> u8 {
    let basePP: u8 = gBattleMoves[r#move].pp;
    basePP
        + (basePP as i32
            * 20
            * shr_i32(
                gPPUpGetMask[moveIndex] as i32 & ppBonuses as i32,
                2 * moveIndex as u32,
            )
            / 100) as u8
}
pub unsafe fn RemoveMonPPBonus(mon: *mut Pokemon, moveIndex: u8) {
    let mut ppBonuses: u8 = GetMonData3(mon, MON_DATA_PP_BONUSES, null_mut()) as u8;
    ppBonuses &= gPPUpClearMask[moveIndex];
    SetMonData(mon, MON_DATA_PP_BONUSES, &raw mut ppBonuses as *mut c_void);
}
pub unsafe fn RemoveBattleMonPPBonus(mon: *mut BattlePokemon, moveIndex: u8) {
    (*mon).ppBonuses &= gPPUpClearMask[moveIndex];
}
pub unsafe fn CopyPlayerPartyMonToBattleData(battler: u8, partyIndex: u8) {
    let mut nickname: CArray<u8, 20> = zeroed();
    gBattleMons[battler].species = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_SPECIES,
        null_mut(),
    ) as u16;
    gBattleMons[battler].item = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_HELD_ITEM,
        null_mut(),
    ) as u16;
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        gBattleMons[battler].moves[i] = GetMonData3(
            &raw mut gPlayerParty[partyIndex],
            MON_DATA_MOVE1 + i,
            null_mut(),
        ) as u16;
        gBattleMons[battler].pp[i] = GetMonData3(
            &raw mut gPlayerParty[partyIndex],
            MON_DATA_PP1 + i,
            null_mut(),
        ) as u8;
        i += 1;
    }
    gBattleMons[battler].ppBonuses = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_PP_BONUSES,
        null_mut(),
    ) as u8;
    gBattleMons[battler].friendship = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_FRIENDSHIP,
        null_mut(),
    ) as u8;
    gBattleMons[battler].experience =
        GetMonData3(&raw mut gPlayerParty[partyIndex], MON_DATA_EXP, null_mut());
    gBattleMons[battler].set_hpIV(GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_HP_IV,
        null_mut(),
    ));
    gBattleMons[battler].set_attackIV(GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_ATK_IV,
        null_mut(),
    ));
    gBattleMons[battler].set_defenseIV(GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_DEF_IV,
        null_mut(),
    ));
    gBattleMons[battler].set_speedIV(GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_SPEED_IV,
        null_mut(),
    ));
    gBattleMons[battler].set_spAttackIV(GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_SPATK_IV,
        null_mut(),
    ));
    gBattleMons[battler].set_spDefenseIV(GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_SPDEF_IV,
        null_mut(),
    ));
    gBattleMons[battler].personality = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_PERSONALITY,
        null_mut(),
    );
    gBattleMons[battler].status1 = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_STATUS,
        null_mut(),
    );
    gBattleMons[battler].level = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_LEVEL,
        null_mut(),
    ) as u8;
    gBattleMons[battler].hp =
        GetMonData3(&raw mut gPlayerParty[partyIndex], MON_DATA_HP, null_mut()) as u16;
    gBattleMons[battler].maxHP = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_MAX_HP,
        null_mut(),
    ) as u16;
    gBattleMons[battler].attack =
        GetMonData3(&raw mut gPlayerParty[partyIndex], MON_DATA_ATK, null_mut()) as u16;
    gBattleMons[battler].defense =
        GetMonData3(&raw mut gPlayerParty[partyIndex], MON_DATA_DEF, null_mut()) as u16;
    gBattleMons[battler].speed = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_SPEED,
        null_mut(),
    ) as u16;
    gBattleMons[battler].spAttack = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_SPATK,
        null_mut(),
    ) as u16;
    gBattleMons[battler].spDefense = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_SPDEF,
        null_mut(),
    ) as u16;
    gBattleMons[battler].set_isEgg(GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_IS_EGG,
        null_mut(),
    ));
    gBattleMons[battler].set_abilityNum(GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_ABILITY_NUM,
        null_mut(),
    ));
    gBattleMons[battler].otId = GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_OT_ID,
        null_mut(),
    );
    gBattleMons[battler].types[0] = gSpeciesInfo[gBattleMons[battler].species].types[0];
    gBattleMons[battler].types[1] = gSpeciesInfo[gBattleMons[battler].species].types[1];
    gBattleMons[battler].ability = GetAbilityBySpecies(
        gBattleMons[battler].species,
        gBattleMons[battler].abilityNum() as u8,
    );
    GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_NICKNAME,
        nickname.as_mut_ptr(),
    );
    StringCopy_Nickname(
        gBattleMons[battler].nickname.as_mut_ptr(),
        nickname.as_mut_ptr(),
    );
    GetMonData3(
        &raw mut gPlayerParty[partyIndex],
        MON_DATA_OT_NAME,
        gBattleMons[battler].otName.as_mut_ptr(),
    );
    let hpSwitchout: *mut u16 = &raw mut (*gBattleStruct).hpOnSwitchout[GetBattlerSide(battler)];
    *hpSwitchout = gBattleMons[battler].hp;
    for i in 0..NUM_BATTLE_STATS {
        gBattleMons[battler].statStages[i] = DEFAULT_STAT_STAGE;
    }
    gBattleMons[battler].status2 = 0;
    UpdateSentPokesToOpponentValue(battler);
    ClearTemporarySpeciesSpriteData(battler, FALSE);
}
pub unsafe fn ExecuteTableBasedItemEffect(
    mon: *mut Pokemon,
    item: u16,
    partyIndex: u8,
    moveIndex: u8,
) -> u8 {
    PokemonUseItemEffects(mon, item, partyIndex, moveIndex, FALSE)
}
pub unsafe fn PokemonUseItemEffects(
    mon: *mut Pokemon,
    item: u16,
    partyIndex: u8,
    moveIndex: u8,
    usedByAI: u8,
) -> u8 {
    let mut dataUnsigned: u32 = 0;
    let mut dataSigned: i32 = 0;
    let mut friendship: i32 = 0;
    let mut i: i32 = 0;
    let mut retVal: u8 = TRUE;
    let mut itemEffect: *mut u8 = null_mut();
    let mut itemEffectParam: u8 = ITEM_EFFECT_ARG_START;
    let mut temp1: u32 = 0;
    let mut temp2: u32 = 0;
    let mut friendshipChange: i8 = 0;
    let mut holdEffect: u8 = 0;
    let mut battler: u8 = MAX_BATTLERS_COUNT;
    let mut friendshipOnly: u32 = FALSE as u32;
    let mut effectFlags: u8 = 0;
    let mut evChange: i8 = 0;
    let mut evCount: u16 = 0;
    let heldItem: u16 = GetMonData3(mon, MON_DATA_HELD_ITEM, null_mut()) as u16;
    if heldItem == ITEM_ENIGMA_BERRY {
        if gMain.inBattle() != 0 {
            holdEffect = gEnigmaBerries[gBattlerInMenuId].holdEffect;
        } else {
            holdEffect = (*gSaveBlock1Ptr).enigmaBerry.holdEffect;
        }
    } else {
        holdEffect = GetItemHoldEffect(heldItem);
    }
    gPotentialItemEffectBattler = gBattlerInMenuId;
    if gMain.inBattle() != 0 {
        gActiveBattler = gBattlerInMenuId;
        i = (GetBattlerSide(gActiveBattler) != B_SIDE_PLAYER) as i32;
        while i < gBattlersCount as i32 {
            if gBattlerPartyIndexes[i] == partyIndex as u16 {
                battler = i as u8;
                break;
            }
            i += 2;
        }
    } else {
        gActiveBattler = 0;
        battler = MAX_BATTLERS_COUNT;
    }
    if !(item >= ITEM_POTION as u16 && item <= ITEM_UNUSED_BERRY_3) {
        return TRUE;
    }
    if gItemEffectTable[item as i32 - ITEM_POTION].is_null() && item != ITEM_ENIGMA_BERRY {
        return TRUE;
    }
    if item == ITEM_ENIGMA_BERRY {
        if gMain.inBattle() != 0 {
            itemEffect = gEnigmaBerries[gActiveBattler].itemEffect.as_mut_ptr();
        } else {
            itemEffect = (*gSaveBlock1Ptr).enigmaBerry.itemEffect.as_mut_ptr();
        }
    } else {
        itemEffect = gItemEffectTable[item as i32 - ITEM_POTION];
    }
    for i in 0..(ITEM_EFFECT_ARG_START as i32) {
        match i {
            0 => {
                if *itemEffect.at(i) as i32 & ITEM0_INFATUATION != 0
                    && gMain.inBattle() != 0
                    && battler != MAX_BATTLERS_COUNT
                    && gBattleMons[battler].status2 & STATUS2_INFATUATION != 0
                {
                    gBattleMons[battler].status2 &= 0xfff0ffff;
                    retVal = FALSE;
                }
                if *itemEffect.at(i) as i32 & ITEM0_DIRE_HIT != 0
                    && gBattleMons[gActiveBattler].status2 & STATUS2_FOCUS_ENERGY == 0
                {
                    gBattleMons[gActiveBattler].status2 |= STATUS2_FOCUS_ENERGY;
                    retVal = FALSE;
                }
                if *itemEffect.at(i) as i32 & ITEM0_X_ATTACK != 0
                    && gBattleMons[gActiveBattler].statStages[1] < MAX_STAT_STAGE
                {
                    gBattleMons[gActiveBattler].statStages[1] +=
                        *itemEffect.at(i) as i8 & ITEM0_X_ATTACK as i8;
                    if gBattleMons[gActiveBattler].statStages[1] > MAX_STAT_STAGE {
                        gBattleMons[gActiveBattler].statStages[1] = MAX_STAT_STAGE;
                    }
                    retVal = FALSE;
                }
            }
            1 => {
                if *itemEffect.at(i) as i32 & ITEM1_X_DEFEND != 0
                    && gBattleMons[gActiveBattler].statStages[2] < MAX_STAT_STAGE
                {
                    gBattleMons[gActiveBattler].statStages[2] +=
                        ((*itemEffect.at(i) as i32 & ITEM1_X_DEFEND) >> 4) as i8;
                    if gBattleMons[gActiveBattler].statStages[2] > MAX_STAT_STAGE {
                        gBattleMons[gActiveBattler].statStages[2] = MAX_STAT_STAGE;
                    }
                    retVal = FALSE;
                }
                if *itemEffect.at(i) as i32 & ITEM1_X_SPEED != 0
                    && gBattleMons[gActiveBattler].statStages[3] < MAX_STAT_STAGE
                {
                    gBattleMons[gActiveBattler].statStages[3] +=
                        *itemEffect.at(i) as i8 & ITEM1_X_SPEED as i8;
                    if gBattleMons[gActiveBattler].statStages[3] > MAX_STAT_STAGE {
                        gBattleMons[gActiveBattler].statStages[3] = MAX_STAT_STAGE;
                    }
                    retVal = FALSE;
                }
            }
            2 => {
                if *itemEffect.at(i) as i32 & ITEM2_X_ACCURACY != 0
                    && gBattleMons[gActiveBattler].statStages[6] < MAX_STAT_STAGE
                {
                    gBattleMons[gActiveBattler].statStages[6] +=
                        ((*itemEffect.at(i) as i32 & ITEM2_X_ACCURACY) >> 4) as i8;
                    if gBattleMons[gActiveBattler].statStages[6] > MAX_STAT_STAGE {
                        gBattleMons[gActiveBattler].statStages[6] = MAX_STAT_STAGE;
                    }
                    retVal = FALSE;
                }
                if *itemEffect.at(i) as i32 & ITEM2_X_SPATK != 0
                    && gBattleMons[gActiveBattler].statStages[4] < MAX_STAT_STAGE
                {
                    gBattleMons[gActiveBattler].statStages[4] +=
                        *itemEffect.at(i) as i8 & ITEM2_X_SPATK as i8;
                    if gBattleMons[gActiveBattler].statStages[4] > MAX_STAT_STAGE {
                        gBattleMons[gActiveBattler].statStages[4] = MAX_STAT_STAGE;
                    }
                    retVal = FALSE;
                }
            }
            3 => {
                if *itemEffect.at(i) as i32 & ITEM3_GUARD_SPEC != 0
                    && gSideTimers[GetBattlerSide(gActiveBattler)].mistTimer == 0
                {
                    gSideTimers[GetBattlerSide(gActiveBattler)].mistTimer = 5;
                    retVal = FALSE;
                }
                if *itemEffect.at(i) as i32 & ITEM3_LEVEL_UP != 0
                    && GetMonData3(mon, MON_DATA_LEVEL, null_mut()) != MAX_LEVEL
                {
                    dataUnsigned = gExperienceTables
                        [gSpeciesInfo[GetMonData3(mon, MON_DATA_SPECIES, null_mut())].growthRate]
                        [GetMonData3(mon, MON_DATA_LEVEL, null_mut()) + 1];
                    SetMonData(mon, MON_DATA_EXP, &raw mut dataUnsigned as *mut c_void);
                    CalculateMonStats(mon);
                    retVal = FALSE;
                }
                if *itemEffect.at(i) as i32 & ITEM3_SLEEP != 0
                    && HealStatusConditions(mon, partyIndex as u32, STATUS1_SLEEP, battler) == 0
                {
                    if battler != MAX_BATTLERS_COUNT {
                        gBattleMons[battler].status2 &= 0xf7ffffff;
                    }
                    retVal = FALSE;
                }
                if *itemEffect.at(i) as i32 & ITEM3_POISON != 0
                    && HealStatusConditions(mon, partyIndex as u32, 3976, battler) == 0
                {
                    retVal = FALSE;
                }
                if *itemEffect.at(i) as i32 & ITEM3_BURN != 0
                    && HealStatusConditions(mon, partyIndex as u32, STATUS1_BURN, battler) == 0
                {
                    retVal = FALSE;
                }
                if *itemEffect.at(i) as i32 & ITEM3_FREEZE != 0
                    && HealStatusConditions(mon, partyIndex as u32, STATUS1_FREEZE, battler) == 0
                {
                    retVal = FALSE;
                }
                if *itemEffect.at(i) as i32 & ITEM3_PARALYSIS != 0
                    && HealStatusConditions(mon, partyIndex as u32, STATUS1_PARALYSIS, battler) == 0
                {
                    retVal = FALSE;
                }
                if *itemEffect.at(i) as i32 & ITEM3_CONFUSION != 0
                    && gMain.inBattle() != 0
                    && battler != MAX_BATTLERS_COUNT
                    && gBattleMons[battler].status2 & STATUS2_CONFUSION != 0
                {
                    gBattleMons[battler].status2 &= 0xfffffff8;
                    retVal = FALSE;
                }
            }
            4 => {
                effectFlags = *itemEffect.at(i);
                if effectFlags as i32 & ITEM4_PP_UP != 0 {
                    effectFlags &= 223;
                    dataUnsigned = shr_u32(
                        GetMonData3(mon, MON_DATA_PP_BONUSES, null_mut())
                            & gPPUpGetMask[moveIndex] as u32,
                        moveIndex as u32 * 2,
                    );
                    temp1 = CalculatePPWithBonus(
                        GetMonData3(mon, MON_DATA_MOVE1 + moveIndex as i32, null_mut()) as u16,
                        GetMonData3(mon, MON_DATA_PP_BONUSES, null_mut()) as u8,
                        moveIndex,
                    ) as u32;
                    if dataUnsigned <= 2 && temp1 > 4 {
                        dataUnsigned = GetMonData3(mon, MON_DATA_PP_BONUSES, null_mut())
                            + gPPUpAddValues[moveIndex] as u32;
                        SetMonData(
                            mon,
                            MON_DATA_PP_BONUSES,
                            &raw mut dataUnsigned as *mut c_void,
                        );
                        dataUnsigned = CalculatePPWithBonus(
                            GetMonData3(mon, MON_DATA_MOVE1 + moveIndex as i32, null_mut()) as u16,
                            dataUnsigned as u8,
                            moveIndex,
                        ) as u32
                            - temp1;
                        dataUnsigned +=
                            GetMonData3(mon, MON_DATA_PP1 + moveIndex as i32, null_mut());
                        SetMonData(
                            mon,
                            MON_DATA_PP1 + moveIndex as i32,
                            &raw mut dataUnsigned as *mut c_void,
                        );
                        retVal = FALSE;
                    }
                }
                temp1 = 0;
                while effectFlags != 0 {
                    if effectFlags as i32 & 1 != 0 {
                        'l5: {
                            match temp1 {
                                0 | 1 => {
                                    evCount = GetMonEVCount(mon);
                                    temp2 = *itemEffect.at(itemEffectParam) as u32;
                                    dataSigned = GetMonData3(
                                        mon,
                                        sGetMonDataEVConstants[temp1] as i32,
                                        null_mut(),
                                    ) as i32;
                                    evChange = temp2 as i8;
                                    if evChange > 0 {
                                        if evCount >= MAX_TOTAL_EVS as u16 {
                                            return TRUE;
                                        }
                                        if dataSigned >= EV_ITEM_RAISE_LIMIT {
                                            break 'l5;
                                        }
                                        if dataSigned + evChange as i32 > EV_ITEM_RAISE_LIMIT {
                                            temp2 = EV_ITEM_RAISE_LIMIT as u32
                                                - (dataSigned as u32 + evChange as u32)
                                                + evChange as u32;
                                        } else {
                                            temp2 = evChange as u32;
                                        }
                                        if evCount as u32 + temp2 > MAX_TOTAL_EVS as u32 {
                                            temp2 +=
                                                MAX_TOTAL_EVS as u32 - (evCount as u32 + temp2);
                                        }
                                        dataSigned += temp2 as i32;
                                    } else {
                                        if dataSigned == 0 {
                                            friendshipOnly = TRUE as u32;
                                            itemEffectParam += 1;
                                            break 'l5;
                                        }
                                        dataSigned += evChange as i32;
                                        if dataSigned < 0 {
                                            dataSigned = 0;
                                        }
                                    }
                                    SetMonData(
                                        mon,
                                        sGetMonDataEVConstants[temp1] as i32,
                                        &raw mut dataSigned as *mut c_void,
                                    );
                                    CalculateMonStats(mon);
                                    itemEffectParam += 1;
                                    retVal = FALSE;
                                }
                                2 => {
                                    if effectFlags as i32 & 16 != 0 {
                                        if GetMonData3(mon, MON_DATA_HP, null_mut()) != 0 {
                                            itemEffectParam += 1;
                                            break 'l5;
                                        }
                                        if gMain.inBattle() != 0 {
                                            if battler != MAX_BATTLERS_COUNT {
                                                gAbsentBattlerFlags &= !(gBitTable[battler] as u8);
                                                CopyPlayerPartyMonToBattleData(
                                                    battler,
                                                    GetPartyIdFromBattlePartyId(
                                                        gBattlerPartyIndexes[battler] as u8,
                                                    ),
                                                );
                                                if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER
                                                    && gBattleResults.numRevivesUsed < 255
                                                {
                                                    gBattleResults.numRevivesUsed += 1;
                                                }
                                            } else {
                                                gAbsentBattlerFlags &=
                                                    !(gBitTable[gActiveBattler as i32 ^ 2] as u8);
                                                if GetBattlerSide(gActiveBattler) == B_SIDE_PLAYER
                                                    && gBattleResults.numRevivesUsed < 255
                                                {
                                                    gBattleResults.numRevivesUsed += 1;
                                                }
                                            }
                                        }
                                    } else {
                                        if GetMonData3(mon, MON_DATA_HP, null_mut()) == 0 {
                                            itemEffectParam += 1;
                                            break 'l5;
                                        }
                                    }
                                    dataUnsigned = *itemEffect.at({
                                        let t1 = itemEffectParam;
                                        itemEffectParam += 1;
                                        t1
                                    }) as u32;
                                    match dataUnsigned {
                                        255 => {
                                            dataUnsigned =
                                                GetMonData3(mon, MON_DATA_MAX_HP, null_mut())
                                                    - GetMonData3(mon, MON_DATA_HP, null_mut());
                                        }
                                        254 => {
                                            dataUnsigned =
                                                GetMonData3(mon, MON_DATA_MAX_HP, null_mut()) / 2;
                                            if dataUnsigned == 0 {
                                                dataUnsigned = 1;
                                            }
                                        }
                                        253 => {
                                            dataUnsigned = gBattleScripting.levelUpHP as u32;
                                        }
                                        _ => {}
                                    }
                                    if GetMonData3(mon, MON_DATA_MAX_HP, null_mut())
                                        != GetMonData3(mon, MON_DATA_HP, null_mut())
                                    {
                                        if usedByAI == 0 {
                                            dataUnsigned +=
                                                GetMonData3(mon, MON_DATA_HP, null_mut());
                                            if dataUnsigned
                                                > GetMonData3(mon, MON_DATA_MAX_HP, null_mut())
                                            {
                                                dataUnsigned =
                                                    GetMonData3(mon, MON_DATA_MAX_HP, null_mut());
                                            }
                                            SetMonData(
                                                mon,
                                                MON_DATA_HP,
                                                &raw mut dataUnsigned as *mut c_void,
                                            );
                                            if gMain.inBattle() != 0
                                                && battler != MAX_BATTLERS_COUNT
                                            {
                                                gBattleMons[battler].hp = dataUnsigned as u16;
                                                if effectFlags as i32 & 16 == 0
                                                    && GetBattlerSide(gActiveBattler)
                                                        == B_SIDE_PLAYER
                                                {
                                                    gBattleResults.numHealingItemsUsed =
                                                        gBattleResults
                                                            .numHealingItemsUsed
                                                            .saturating_add(1);
                                                    temp2 = gActiveBattler as u32;
                                                    gActiveBattler = battler;
                                                    BtlController_EmitGetMonData(
                                                        B_COMM_TO_CONTROLLER,
                                                        REQUEST_ALL_BATTLE,
                                                        0,
                                                    );
                                                    MarkBattlerForControllerExec(gActiveBattler);
                                                    gActiveBattler = temp2 as u8;
                                                }
                                            }
                                        } else {
                                            gBattleMoveDamage = -(dataUnsigned as i32);
                                        }
                                        retVal = FALSE;
                                    }
                                    effectFlags &= 239;
                                }
                                3 => {
                                    if effectFlags as i32 & 2 == 0 {
                                        temp2 = 0;
                                        while (temp2 as i32) < MAX_MON_MOVES {
                                            dataUnsigned = GetMonData3(
                                                mon,
                                                MON_DATA_PP1 + temp2 as i32,
                                                null_mut(),
                                            );
                                            let mut r#move: u16 = GetMonData3(
                                                mon,
                                                MON_DATA_MOVE1 + temp2 as i32,
                                                null_mut(),
                                            )
                                                as u16;
                                            if dataUnsigned
                                                != CalculatePPWithBonus(
                                                    r#move,
                                                    GetMonData3(
                                                        mon,
                                                        MON_DATA_PP_BONUSES,
                                                        null_mut(),
                                                    )
                                                        as u8,
                                                    temp2 as u8,
                                                )
                                                    as u32
                                            {
                                                dataUnsigned +=
                                                    *itemEffect.at(itemEffectParam) as u32;
                                                r#move = GetMonData3(
                                                    mon,
                                                    MON_DATA_MOVE1 + temp2 as i32,
                                                    null_mut(),
                                                )
                                                    as u16;
                                                if dataUnsigned
                                                    > CalculatePPWithBonus(
                                                        r#move,
                                                        GetMonData3(
                                                            mon,
                                                            MON_DATA_PP_BONUSES,
                                                            null_mut(),
                                                        )
                                                            as u8,
                                                        temp2 as u8,
                                                    )
                                                        as u32
                                                {
                                                    r#move = GetMonData3(
                                                        mon,
                                                        MON_DATA_MOVE1 + temp2 as i32,
                                                        null_mut(),
                                                    )
                                                        as u16;
                                                    dataUnsigned = CalculatePPWithBonus(
                                                        r#move,
                                                        GetMonData3(
                                                            mon,
                                                            MON_DATA_PP_BONUSES,
                                                            null_mut(),
                                                        )
                                                            as u8,
                                                        temp2 as u8,
                                                    )
                                                        as u32;
                                                }
                                                SetMonData(
                                                    mon,
                                                    MON_DATA_PP1 + temp2 as i32,
                                                    &raw mut dataUnsigned as *mut c_void,
                                                );
                                                if gMain.inBattle() != 0
                                                    && battler != MAX_BATTLERS_COUNT
                                                    && (gBattleMons[battler].status2 & 0x200000
                                                        == 0
                                                        && gDisableStructs[battler].mimickedMoves()
                                                            as u32
                                                            & (*(&raw const crate::util::gBitTable)
                                                                .cast::<CArray<u32, 0>>())[temp2]
                                                            == 0)
                                                {
                                                    gBattleMons[battler].pp[temp2] =
                                                        dataUnsigned as u8;
                                                }
                                                retVal = FALSE;
                                            }
                                            temp2 += 1;
                                        }
                                        itemEffectParam += 1;
                                    } else {
                                        dataUnsigned = GetMonData3(
                                            mon,
                                            MON_DATA_PP1 + moveIndex as i32,
                                            null_mut(),
                                        );
                                        let mut r#move: u16 = GetMonData3(
                                            mon,
                                            MON_DATA_MOVE1 + moveIndex as i32,
                                            null_mut(),
                                        )
                                            as u16;
                                        if dataUnsigned
                                            != CalculatePPWithBonus(
                                                r#move,
                                                GetMonData3(mon, MON_DATA_PP_BONUSES, null_mut())
                                                    as u8,
                                                moveIndex,
                                            ) as u32
                                        {
                                            dataUnsigned += *itemEffect.at({
                                                let t2 = itemEffectParam;
                                                itemEffectParam += 1;
                                                t2
                                            })
                                                as u32;
                                            r#move = GetMonData3(
                                                mon,
                                                MON_DATA_MOVE1 + moveIndex as i32,
                                                null_mut(),
                                            )
                                                as u16;
                                            if dataUnsigned
                                                > CalculatePPWithBonus(
                                                    r#move,
                                                    GetMonData3(
                                                        mon,
                                                        MON_DATA_PP_BONUSES,
                                                        null_mut(),
                                                    )
                                                        as u8,
                                                    moveIndex,
                                                )
                                                    as u32
                                            {
                                                r#move = GetMonData3(
                                                    mon,
                                                    MON_DATA_MOVE1 + moveIndex as i32,
                                                    null_mut(),
                                                )
                                                    as u16;
                                                dataUnsigned = CalculatePPWithBonus(
                                                    r#move,
                                                    GetMonData3(
                                                        mon,
                                                        MON_DATA_PP_BONUSES,
                                                        null_mut(),
                                                    )
                                                        as u8,
                                                    moveIndex,
                                                )
                                                    as u32;
                                            }
                                            SetMonData(
                                                mon,
                                                MON_DATA_PP1 + moveIndex as i32,
                                                &raw mut dataUnsigned as *mut c_void,
                                            );
                                            if gMain.inBattle() != 0
                                                && battler != MAX_BATTLERS_COUNT
                                                && (gBattleMons[battler].status2 & 0x200000 == 0
                                                    && gDisableStructs[battler].mimickedMoves()
                                                        as u32
                                                        & (*(&raw const crate::util::gBitTable)
                                                            .cast::<CArray<u32, 0>>())[moveIndex]
                                                        == 0)
                                            {
                                                gBattleMons[battler].pp[moveIndex] =
                                                    dataUnsigned as u8;
                                            }
                                            retVal = FALSE;
                                        }
                                    }
                                }
                                7 => {
                                    let targetSpecies: u16 =
                                        GetEvolutionTargetSpecies(mon, EVO_MODE_ITEM_USE, item);
                                    if targetSpecies != SPECIES_NONE {
                                        BeginEvolutionScene(mon, targetSpecies, FALSE, partyIndex);
                                        return FALSE;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    temp1 += 1;
                    effectFlags >>= 1;
                }
            }
            5 => {
                effectFlags = *itemEffect.at(i);
                temp1 = 0;
                while effectFlags != 0 {
                    if effectFlags as i32 & 1 != 0 {
                        'l9: {
                            match temp1 {
                                0..=3 => {
                                    evCount = GetMonEVCount(mon);
                                    temp2 = *itemEffect.at(itemEffectParam) as u32;
                                    dataSigned = GetMonData3(
                                        mon,
                                        sGetMonDataEVConstants[temp1 + 2] as i32,
                                        null_mut(),
                                    ) as i32;
                                    evChange = temp2 as i8;
                                    if evChange > 0 {
                                        if evCount >= MAX_TOTAL_EVS as u16 {
                                            return TRUE;
                                        }
                                        if dataSigned >= EV_ITEM_RAISE_LIMIT {
                                            break 'l9;
                                        }
                                        if dataSigned + evChange as i32 > EV_ITEM_RAISE_LIMIT {
                                            temp2 = EV_ITEM_RAISE_LIMIT as u32
                                                - (dataSigned as u32 + evChange as u32)
                                                + evChange as u32;
                                        } else {
                                            temp2 = evChange as u32;
                                        }
                                        if evCount as u32 + temp2 > MAX_TOTAL_EVS as u32 {
                                            temp2 +=
                                                MAX_TOTAL_EVS as u32 - (evCount as u32 + temp2);
                                        }
                                        dataSigned += temp2 as i32;
                                    } else {
                                        if dataSigned == 0 {
                                            friendshipOnly = TRUE as u32;
                                            itemEffectParam += 1;
                                            break 'l9;
                                        }
                                        dataSigned += evChange as i32;
                                        if dataSigned < 0 {
                                            dataSigned = 0;
                                        }
                                    }
                                    SetMonData(
                                        mon,
                                        sGetMonDataEVConstants[temp1 + 2] as i32,
                                        &raw mut dataSigned as *mut c_void,
                                    );
                                    CalculateMonStats(mon);
                                    retVal = FALSE;
                                    itemEffectParam += 1;
                                }
                                4 => {
                                    dataUnsigned = shr_u32(
                                        GetMonData3(mon, MON_DATA_PP_BONUSES, null_mut())
                                            & gPPUpGetMask[moveIndex] as u32,
                                        moveIndex as u32 * 2,
                                    );
                                    temp2 = CalculatePPWithBonus(
                                        GetMonData3(
                                            mon,
                                            MON_DATA_MOVE1 + moveIndex as i32,
                                            null_mut(),
                                        ) as u16,
                                        GetMonData3(mon, MON_DATA_PP_BONUSES, null_mut()) as u8,
                                        moveIndex,
                                    ) as u32;
                                    if dataUnsigned < 3 && temp2 >= 5 {
                                        dataUnsigned =
                                            GetMonData3(mon, MON_DATA_PP_BONUSES, null_mut());
                                        dataUnsigned &= gPPUpClearMask[moveIndex] as u32;
                                        dataUnsigned += gPPUpAddValues[moveIndex] as u32 * 3;
                                        SetMonData(
                                            mon,
                                            MON_DATA_PP_BONUSES,
                                            &raw mut dataUnsigned as *mut c_void,
                                        );
                                        dataUnsigned = CalculatePPWithBonus(
                                            GetMonData3(
                                                mon,
                                                MON_DATA_MOVE1 + moveIndex as i32,
                                                null_mut(),
                                            ) as u16,
                                            dataUnsigned as u8,
                                            moveIndex,
                                        )
                                            as u32
                                            - temp2;
                                        dataUnsigned += GetMonData3(
                                            mon,
                                            MON_DATA_PP1 + moveIndex as i32,
                                            null_mut(),
                                        );
                                        SetMonData(
                                            mon,
                                            MON_DATA_PP1 + moveIndex as i32,
                                            &raw mut dataUnsigned as *mut c_void,
                                        );
                                        retVal = FALSE;
                                    }
                                }
                                5 => {
                                    if GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut()) < 100
                                        && (retVal == 0 || friendshipOnly != 0)
                                        && ShouldSkipFriendshipChange() == 0
                                        && friendshipChange == 0
                                    {
                                        friendshipChange = *itemEffect.at(itemEffectParam) as i8;
                                        friendship =
                                            GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut())
                                                as i32;
                                        if friendshipChange > 0 && holdEffect == 27 {
                                            friendship += 150 * friendshipChange as i32 / 100;
                                        } else {
                                            friendship += friendshipChange as i32;
                                        }
                                        if friendshipChange > 0 {
                                            if GetMonData3(mon, MON_DATA_POKEBALL, null_mut())
                                                == ITEM_LUXURY_BALL
                                            {
                                                friendship += 1;
                                            }
                                            if GetMonData3(mon, MON_DATA_MET_LOCATION, null_mut())
                                                == GetCurrentRegionMapSectionId() as u32
                                            {
                                                friendship += 1;
                                            }
                                        }
                                        if friendship < 0 {
                                            friendship = 0;
                                        }
                                        if friendship > 255 {
                                            friendship = 255;
                                        }
                                        SetMonData(
                                            mon,
                                            MON_DATA_FRIENDSHIP,
                                            &raw mut friendship as *mut c_void,
                                        );
                                        retVal = 0;
                                    }
                                    itemEffectParam += 1;
                                }
                                6 => {
                                    if GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut()) >= 100
                                        && GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut()) < 200
                                        && (retVal == 0 || friendshipOnly != 0)
                                        && ShouldSkipFriendshipChange() == 0
                                        && friendshipChange == 0
                                    {
                                        friendshipChange = *itemEffect.at(itemEffectParam) as i8;
                                        friendship =
                                            GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut())
                                                as i32;
                                        if friendshipChange > 0 && holdEffect == 27 {
                                            friendship += 150 * friendshipChange as i32 / 100;
                                        } else {
                                            friendship += friendshipChange as i32;
                                        }
                                        if friendshipChange > 0 {
                                            if GetMonData3(mon, MON_DATA_POKEBALL, null_mut())
                                                == ITEM_LUXURY_BALL
                                            {
                                                friendship += 1;
                                            }
                                            if GetMonData3(mon, MON_DATA_MET_LOCATION, null_mut())
                                                == GetCurrentRegionMapSectionId() as u32
                                            {
                                                friendship += 1;
                                            }
                                        }
                                        if friendship < 0 {
                                            friendship = 0;
                                        }
                                        if friendship > 255 {
                                            friendship = 255;
                                        }
                                        SetMonData(
                                            mon,
                                            MON_DATA_FRIENDSHIP,
                                            &raw mut friendship as *mut c_void,
                                        );
                                        retVal = 0;
                                    }
                                    itemEffectParam += 1;
                                }
                                7 => {
                                    if GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut()) >= 200
                                        && (retVal == 0 || friendshipOnly != 0)
                                        && ShouldSkipFriendshipChange() == 0
                                        && friendshipChange == 0
                                    {
                                        friendshipChange = *itemEffect.at(itemEffectParam) as i8;
                                        friendship =
                                            GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut())
                                                as i32;
                                        if friendshipChange > 0 && holdEffect == 27 {
                                            friendship += 150 * friendshipChange as i32 / 100;
                                        } else {
                                            friendship += friendshipChange as i32;
                                        }
                                        if friendshipChange > 0 {
                                            if GetMonData3(mon, MON_DATA_POKEBALL, null_mut())
                                                == ITEM_LUXURY_BALL
                                            {
                                                friendship += 1;
                                            }
                                            if GetMonData3(mon, MON_DATA_MET_LOCATION, null_mut())
                                                == GetCurrentRegionMapSectionId() as u32
                                            {
                                                friendship += 1;
                                            }
                                        }
                                        if friendship < 0 {
                                            friendship = 0;
                                        }
                                        if friendship > 255 {
                                            friendship = 255;
                                        }
                                        SetMonData(
                                            mon,
                                            MON_DATA_FRIENDSHIP,
                                            &raw mut friendship as *mut c_void,
                                        );
                                        retVal = 0;
                                    }
                                    itemEffectParam += 1;
                                }
                                _ => {}
                            }
                        }
                    }
                    temp1 += 1;
                    effectFlags >>= 1;
                }
            }
            _ => {}
        }
    }
    retVal
}
pub unsafe fn HealStatusConditions(
    mon: *mut Pokemon,
    battlePartyId: u32,
    healMask: u32,
    battler: u8,
) -> u8 {
    let mut status: u32 = GetMonData3(mon, MON_DATA_STATUS, null_mut());
    if status & healMask != 0 {
        status &= !healMask;
        SetMonData(mon, MON_DATA_STATUS, &raw mut status as *mut c_void);
        if gMain.inBattle() != 0 && battler != MAX_BATTLERS_COUNT {
            gBattleMons[battler].status1 &= !healMask;
        }
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetItemEffectParamOffset(itemId: u16, effectByte: u8, mut effectBit: u8) -> u8 {
    let mut itemEffect: *mut u8 = null_mut();
    let mut j: u8 = 0;
    let mut effectFlags: u8 = 0;
    let mut offset: u8 = ITEM_EFFECT_ARG_START;
    let mut temp: *mut u8 = gItemEffectTable[itemId as i32 - ITEM_POTION];
    if temp.is_null() && itemId != ITEM_ENIGMA_BERRY {
        return 0;
    }
    if itemId == ITEM_ENIGMA_BERRY {
        temp = gEnigmaBerries[gActiveBattler].itemEffect.as_mut_ptr();
    }
    itemEffect = temp;
    for i in 0..(ITEM_EFFECT_ARG_START as i32) {
        match i {
            0..=3 => {
                if i == effectByte as i32 {
                    return 0;
                }
            }
            4 => {
                effectFlags = *itemEffect.at(4);
                if effectFlags as i32 & ITEM4_PP_UP != 0 {
                    effectFlags &= 223;
                }
                j = 0;
                while effectFlags != 0 {
                    if effectFlags as i32 & 1 != 0 {
                        'l4: {
                            let sw1: u8 = j;
                            let mut fall = false;
                            if sw1 == 2 {
                                fall = true;
                                if effectFlags as i32 & 16 != 0 {
                                    effectFlags &= 239;
                                }
                            }
                            if fall || sw1 == 0 {
                                if i == effectByte as i32
                                    && effectFlags as i32 & effectBit as i32 != 0
                                {
                                    return offset;
                                }
                                offset += 1;
                                break 'l4;
                            }
                            if sw1 == 1 {
                                if i == effectByte as i32
                                    && effectFlags as i32 & effectBit as i32 != 0
                                {
                                    return offset;
                                }
                                offset += 1;
                                break 'l4;
                            }
                            if sw1 == 3 {
                                if i == effectByte as i32
                                    && effectFlags as i32 & effectBit as i32 != 0
                                {
                                    return offset;
                                }
                                offset += 1;
                                break 'l4;
                            }
                            if sw1 == 7 {
                                if i == effectByte as i32 {
                                    return 0;
                                }
                                break 'l4;
                            }
                        }
                    }
                    j += 1;
                    effectFlags >>= 1;
                    if i == effectByte as i32 {
                        effectBit >>= 1;
                    }
                }
            }
            5 => {
                effectFlags = *itemEffect.at(5);
                j = 0;
                while effectFlags != 0 {
                    if effectFlags as i32 & 1 != 0 {
                        match j {
                            0..=6 => {
                                if i == effectByte as i32
                                    && effectFlags as i32 & effectBit as i32 != 0
                                {
                                    return offset;
                                }
                                offset += 1;
                            }
                            7 if i == effectByte as i32 => {
                                return 0;
                            }
                            _ => {}
                        }
                    }
                    j += 1;
                    effectFlags >>= 1;
                    if i == effectByte as i32 {
                        effectBit >>= 1;
                    }
                }
            }
            _ => {}
        }
    }
    offset
}
unsafe fn BufferStatRoseMessage(statIdx: i32) {
    gBattlerTarget = gBattlerInMenuId;
    StringCopy(
        gBattleTextBuff1.as_mut_ptr(),
        (*(&raw const crate::data::battle_message::gStatNamesTable).cast::<CArray<*mut u8, 0>>())
            [sStatsToRaise[statIdx]],
    );
    StringCopy(
        gBattleTextBuff2.as_mut_ptr(),
        (*(&raw const crate::data::battle_message::gText_StatRose).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    BattleStringExpandPlaceholdersToDisplayedString(
        (*(&raw const crate::data::battle_message::gText_DefendersStatRose)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut(),
    );
}
pub unsafe fn UseStatIncreaseItem(itemId: u16) -> *mut u8 {
    let mut itemEffect: *mut u8 = null_mut();
    if itemId == ITEM_ENIGMA_BERRY {
        if gMain.inBattle() != 0 {
            itemEffect = gEnigmaBerries[gBattlerInMenuId].itemEffect.as_mut_ptr();
        } else {
            itemEffect = (*gSaveBlock1Ptr).enigmaBerry.itemEffect.as_mut_ptr();
        }
    } else {
        itemEffect = gItemEffectTable[itemId as i32 - ITEM_POTION];
    }
    gPotentialItemEffectBattler = gBattlerInMenuId;
    for i in 0..3i32 {
        if *itemEffect.at(i) as i32 & 15 != 0 {
            BufferStatRoseMessage(i * 2);
        }
        if *itemEffect.at(i) as i32 & 240 != 0 {
            if i != 0 {
                BufferStatRoseMessage(i * 2 + 1);
            } else {
                gBattlerAttacker = gBattlerInMenuId;
                BattleStringExpandPlaceholdersToDisplayedString(
                    (*(&raw const crate::data::battle_message::gText_PkmnGettingPumped)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            }
        }
    }
    if *itemEffect.at(3) as i32 & ITEM3_GUARD_SPEC != 0 {
        gBattlerAttacker = gBattlerInMenuId;
        BattleStringExpandPlaceholdersToDisplayedString(
            (*(&raw const crate::data::battle_message::gText_PkmnShroudedInMist)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    }
    gDisplayedStringBattle.as_mut_ptr()
}
pub unsafe fn GetNature(mon: *mut Pokemon) -> u8 {
    (GetMonData3(mon, MON_DATA_PERSONALITY, null_mut()) % 25) as u8
}
pub unsafe fn GetNatureFromPersonality(personality: u32) -> u8 {
    (personality % 25) as u8
}
pub unsafe fn GetEvolutionTargetSpecies(mon: *mut Pokemon, mode: u8, evolutionItem: u16) -> u16 {
    let mut targetSpecies: u16 = 0;
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut heldItem: u16 = GetMonData3(mon, MON_DATA_HELD_ITEM, null_mut()) as u16;
    let personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    let mut level: u8 = 0;
    let mut friendship: u16 = 0;
    let beauty: u8 = GetMonData3(mon, MON_DATA_BEAUTY, null_mut()) as u8;
    let upperPersonality: u16 = (personality >> 16) as u16;
    let mut holdEffect: u8 = 0;
    if heldItem == ITEM_ENIGMA_BERRY {
        holdEffect = (*gSaveBlock1Ptr).enigmaBerry.holdEffect;
    } else {
        holdEffect = GetItemHoldEffect(heldItem);
    }
    if holdEffect == HOLD_EFFECT_PREVENT_EVOLVE && mode != EVO_MODE_ITEM_CHECK {
        return SPECIES_NONE;
    }
    match mode {
        EVO_MODE_NORMAL => {
            level = GetMonData3(mon, MON_DATA_LEVEL, null_mut()) as u8;
            friendship = GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut()) as u16;
            for i in 0..EVOS_PER_MON {
                match gEvolutionTable[species][i].method {
                    EVO_FRIENDSHIP => {
                        if friendship >= FRIENDSHIP_EVO_THRESHOLD {
                            targetSpecies = gEvolutionTable[species][i].targetSpecies;
                        }
                    }
                    EVO_FRIENDSHIP_DAY => {
                        RtcCalcLocalTime();
                        if gLocalTime.hours >= DAY_EVO_HOUR_BEGIN
                            && gLocalTime.hours < DAY_EVO_HOUR_END
                            && friendship >= FRIENDSHIP_EVO_THRESHOLD
                        {
                            targetSpecies = gEvolutionTable[species][i].targetSpecies;
                        }
                    }
                    EVO_FRIENDSHIP_NIGHT => {
                        RtcCalcLocalTime();
                        if gLocalTime.hours >= NIGHT_EVO_HOUR_BEGIN
                            && gLocalTime.hours < NIGHT_EVO_HOUR_END
                            && friendship >= FRIENDSHIP_EVO_THRESHOLD
                        {
                            targetSpecies = gEvolutionTable[species][i].targetSpecies;
                        }
                    }
                    EVO_LEVEL => {
                        if gEvolutionTable[species][i].param <= level as u16 {
                            targetSpecies = gEvolutionTable[species][i].targetSpecies;
                        }
                    }
                    EVO_LEVEL_ATK_GT_DEF => {
                        if gEvolutionTable[species][i].param <= level as u16
                            && GetMonData3(mon, MON_DATA_ATK, null_mut())
                                > GetMonData3(mon, MON_DATA_DEF, null_mut())
                        {
                            targetSpecies = gEvolutionTable[species][i].targetSpecies;
                        }
                    }
                    EVO_LEVEL_ATK_EQ_DEF => {
                        if gEvolutionTable[species][i].param <= level as u16
                            && GetMonData3(mon, MON_DATA_ATK, null_mut())
                                == GetMonData3(mon, MON_DATA_DEF, null_mut())
                        {
                            targetSpecies = gEvolutionTable[species][i].targetSpecies;
                        }
                    }
                    EVO_LEVEL_ATK_LT_DEF => {
                        if gEvolutionTable[species][i].param <= level as u16
                            && GetMonData3(mon, MON_DATA_ATK, null_mut())
                                < GetMonData3(mon, MON_DATA_DEF, null_mut())
                        {
                            targetSpecies = gEvolutionTable[species][i].targetSpecies;
                        }
                    }
                    EVO_LEVEL_SILCOON => {
                        if gEvolutionTable[species][i].param <= level as u16
                            && upperPersonality as i32 % 10 <= 4
                        {
                            targetSpecies = gEvolutionTable[species][i].targetSpecies;
                        }
                    }
                    EVO_LEVEL_CASCOON => {
                        if gEvolutionTable[species][i].param <= level as u16
                            && upperPersonality as i32 % 10 > 4
                        {
                            targetSpecies = gEvolutionTable[species][i].targetSpecies;
                        }
                    }
                    EVO_LEVEL_NINJASK => {
                        if gEvolutionTable[species][i].param <= level as u16 {
                            targetSpecies = gEvolutionTable[species][i].targetSpecies;
                        }
                    }
                    EVO_BEAUTY if gEvolutionTable[species][i].param <= beauty as u16 => {
                        targetSpecies = gEvolutionTable[species][i].targetSpecies;
                    }
                    _ => {}
                }
            }
        }
        EVO_MODE_TRADE => {
            for i in 0..EVOS_PER_MON {
                match gEvolutionTable[species][i].method {
                    EVO_TRADE => {
                        targetSpecies = gEvolutionTable[species][i].targetSpecies;
                    }
                    EVO_TRADE_ITEM if gEvolutionTable[species][i].param == heldItem => {
                        heldItem = ITEM_NONE;
                        SetMonData(mon, MON_DATA_HELD_ITEM, &raw mut heldItem as *mut c_void);
                        targetSpecies = gEvolutionTable[species][i].targetSpecies;
                    }
                    _ => {}
                }
            }
        }
        EVO_MODE_ITEM_USE | EVO_MODE_ITEM_CHECK => {
            for i in 0..EVOS_PER_MON {
                if gEvolutionTable[species][i].method == EVO_ITEM
                    && gEvolutionTable[species][i].param == evolutionItem
                {
                    targetSpecies = gEvolutionTable[species][i].targetSpecies;
                    break;
                }
            }
        }
        _ => {}
    }
    targetSpecies
}
pub fn HoennPokedexNumToSpecies(hoennNum: u16) -> u16 {
    if hoennNum == 0 {
        return 0;
    }
    let mut species: u16 = 0;
    while species < 411 && sSpeciesToHoennPokedexNum[species] != hoennNum {
        species += 1;
    }
    if species == 411 {
        return 0;
    }
    species + 1
}
pub unsafe fn NationalPokedexNumToSpecies(nationalNum: u16) -> u16 {
    if nationalNum == 0 {
        return 0;
    }
    let mut species: u16 = 0;
    while species < 411 && sSpeciesToNationalPokedexNum[species] != nationalNum {
        species += 1;
    }
    if species == 411 {
        return 0;
    }
    species + 1
}
pub fn NationalToHoennOrder(nationalNum: u16) -> u16 {
    if nationalNum == 0 {
        return 0;
    }
    let mut hoennNum: u16 = 0;
    while hoennNum < 411 && sHoennToNationalOrder[hoennNum] != nationalNum {
        hoennNum += 1;
    }
    if hoennNum == 411 {
        return 0;
    }
    hoennNum + 1
}
#[unsafe(no_mangle)]
pub unsafe fn SpeciesToNationalPokedexNum(species: u16) -> u16 {
    if species == 0 {
        return 0;
    }
    sSpeciesToNationalPokedexNum[species as i32 - 1]
}
pub fn SpeciesToHoennPokedexNum(species: u16) -> u16 {
    if species == 0 {
        return 0;
    }
    sSpeciesToHoennPokedexNum[species as i32 - 1]
}
pub fn HoennToNationalOrder(hoennNum: u16) -> u16 {
    if hoennNum == 0 {
        return 0;
    }
    sHoennToNationalOrder[hoennNum as i32 - 1]
}
pub fn SpeciesToCryId(species: u16) -> u16 {
    if species <= 250 {
        return species;
    }
    if species < 276 {
        return 200;
    }
    gSpeciesIdToCryId[species as i32 - 276]
}
unsafe fn DrawSpindaSpotsUnused(species: u16, mut personality: u32, dest: *mut u8) {
    if species == SPECIES_SPINDA
        && !core::ptr::addr_eq(dest, (*gMonSpritesGfxPtr).sprites.ptr[0])
        && !core::ptr::addr_eq(dest, (*gMonSpritesGfxPtr).sprites.ptr[2])
    {
        for i in 0..4i32 {
            let x: u8 = gSpindaSpotGraphics[i].x + ((personality as u8 & 0x0F) - 8);
            let mut y: u8 = gSpindaSpotGraphics[i].y + (((personality & 0xF0) >> 4) as u8 - 8);
            for row in 0..16i32 {
                let mut spotPixelRow: i32 = gSpindaSpotGraphics[i].image[row] as i32;
                for column in (x as i32)..(x as i32 + 16) {
                    let destPixels: *mut u8 = dest
                        .at(column / 8 * 32)
                        .at(column % 8 / 2)
                        .at(y as i32 / 8 * 32 * 8)
                        .at(y as i32 % 8 * 4);
                    if spotPixelRow & 1 != 0 {
                        if column & 1 != 0 {
                            if *destPixels as i32 & 240 >= 16 && *destPixels as i32 & 240 <= 48 {
                                *destPixels += 64;
                            }
                        } else {
                            if *destPixels as i32 & 15 >= 1 && *destPixels as i32 & 15 <= 3 {
                                *destPixels += 4;
                            }
                        }
                    }
                    spotPixelRow >>= 1;
                }
                y += 1;
            }
            personality >>= 8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn DrawSpindaSpots(species: u16, mut personality: u32, dest: *mut u8, isFrontPic: u8) {
    if species == SPECIES_SPINDA && isFrontPic != 0 {
        for i in 0..4i32 {
            let x: u8 = gSpindaSpotGraphics[i].x + ((personality as u8 & 0x0F) - 8);
            let mut y: u8 = gSpindaSpotGraphics[i].y + (((personality & 0xF0) >> 4) as u8 - 8);
            for row in 0..16i32 {
                let mut spotPixelRow: i32 = gSpindaSpotGraphics[i].image[row] as i32;
                for column in (x as i32)..(x as i32 + 16) {
                    let destPixels: *mut u8 = dest
                        .at(column / 8 * 32)
                        .at(column % 8 / 2)
                        .at(y as i32 / 8 * 32 * 8)
                        .at(y as i32 % 8 * 4);
                    if spotPixelRow & 1 != 0 {
                        if column & 1 != 0 {
                            if *destPixels as i32 & 240 >= 16 && *destPixels as i32 & 240 <= 48 {
                                *destPixels += 64;
                            }
                        } else {
                            if *destPixels as i32 & 15 >= 1 && *destPixels as i32 & 15 <= 3 {
                                *destPixels += 4;
                            }
                        }
                    }
                    spotPixelRow >>= 1;
                }
                y += 1;
            }
            personality >>= 8;
        }
    }
}
pub unsafe fn EvolutionRenameMon(mon: *mut Pokemon, oldSpecies: u16, newSpecies: u16) {
    let mut language: u8 = 0;
    GetMonData3(mon, MON_DATA_NICKNAME, gStringVar1.as_mut_ptr());
    language = GetMonData3(mon, MON_DATA_LANGUAGE, &raw mut language) as u8;
    if language == GAME_LANGUAGE
        && StringCompare(
            (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[oldSpecies]
                .as_ptr()
                .cast_mut(),
            gStringVar1.as_mut_ptr(),
        ) == 0
    {
        SetMonData(
            mon,
            MON_DATA_NICKNAME,
            (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[newSpecies]
                .as_ptr()
                .cast_mut() as *mut c_void,
        );
    }
}
pub unsafe fn GetPlayerFlankId() -> u8 {
    let mut flankId: u8 = 0;
    match gLinkPlayers[GetMultiplayerId()].id {
        0 | 3 => {
            flankId = 0;
        }
        1 | 2 => {
            flankId = 1;
        }
        _ => {}
    }
    flankId
}
pub unsafe fn GetLinkTrainerFlankId(linkPlayerId: u8) -> u16 {
    let mut flankId: u16 = 0;
    match gLinkPlayers[linkPlayerId].id {
        0 | 3 => {
            flankId = 0;
        }
        1 | 2 => {
            flankId = 1;
        }
        _ => {}
    }
    flankId
}
pub unsafe fn GetBattlerMultiplayerId(id: u16) -> i32 {
    let mut multiplayerId: i32 = 0;
    while multiplayerId < MAX_LINK_PLAYERS {
        if gLinkPlayers[multiplayerId].id == id {
            break;
        }
        multiplayerId += 1;
    }
    multiplayerId
}
pub unsafe fn GetTrainerEncounterMusicId(trainerOpponentId: u16) -> u8 {
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        return GetTrainerEncounterMusicIdInBattlePyramid(trainerOpponentId);
    } else if InTrainerHillChallenge() != 0 {
        return GetTrainerEncounterMusicIdInTrainerHill(trainerOpponentId);
    } else {
        return (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
            [trainerOpponentId]
            .encounterMusic_gender
            & 0x7F;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ModifyStatByNature(nature: u8, stat: u16, statIndex: u8) -> u16 {
    let mut retVal: u16 = 0;
    if statIndex <= STAT_HP || statIndex > NUM_NATURE_STATS as u8 {
        return stat;
    }
    match gNatureStatTable[nature][statIndex as i32 - 1] {
        1 => {
            retVal = stat * 110;
            retVal = (retVal as i32 / 100) as u16;
        }
        -1 => {
            retVal = stat * 90;
            retVal = (retVal as i32 / 100) as u16;
        }
        _ => {
            retVal = stat;
        }
    }
    retVal
}
#[unsafe(no_mangle)]
pub unsafe fn AdjustFriendship(mon: *mut Pokemon, event: u8) {
    let mut holdEffect: u8 = 0;
    let mut r#mod: i8 = 0;
    if ShouldSkipFriendshipChange() != 0 {
        return;
    }
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
    let heldItem: u16 = GetMonData3(mon, MON_DATA_HELD_ITEM, null_mut()) as u16;
    if heldItem == ITEM_ENIGMA_BERRY {
        if gMain.inBattle() != 0 {
            holdEffect = gEnigmaBerries[0].holdEffect;
        } else {
            holdEffect = (*gSaveBlock1Ptr).enigmaBerry.holdEffect;
        }
    } else {
        holdEffect = GetItemHoldEffect(heldItem);
    }
    if species != 0 && species != SPECIES_EGG as u16 {
        let mut friendshipLevel: u8 = 0;
        let mut friendship: i16 = GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut()) as i16;
        if friendship > 99 {
            friendshipLevel += 1;
        }
        if friendship > 199 {
            friendshipLevel += 1;
        }
        if event == FRIENDSHIP_EVENT_WALKING && Random() as i32 & 1 != 0 {
            return;
        }
        if event == FRIENDSHIP_EVENT_LEAGUE_BATTLE {
            if gBattleTypeFlags & BATTLE_TYPE_TRAINER == 0 {
                return;
            }
            if !((*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
                [gTrainerBattleOpponent_A]
                .trainerClass
                == TRAINER_CLASS_LEADER
                || (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
                    [gTrainerBattleOpponent_A]
                    .trainerClass
                    == TRAINER_CLASS_ELITE_FOUR
                || (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
                    [gTrainerBattleOpponent_A]
                    .trainerClass
                    == TRAINER_CLASS_CHAMPION)
            {
                return;
            }
        }
        r#mod = sFriendshipEventModifiers[event][friendshipLevel];
        if r#mod > 0 && holdEffect == HOLD_EFFECT_FRIENDSHIP_UP {
            r#mod = (150 * r#mod as i32 / 100) as i8;
        }
        friendship += r#mod as i16;
        if r#mod > 0 {
            if GetMonData3(mon, MON_DATA_POKEBALL, null_mut()) == ITEM_LUXURY_BALL {
                friendship += 1;
            }
            if GetMonData3(mon, MON_DATA_MET_LOCATION, null_mut())
                == GetCurrentRegionMapSectionId() as u32
            {
                friendship += 1;
            }
        }
        if friendship < 0 {
            friendship = 0;
        }
        if friendship > MAX_FRIENDSHIP as i16 {
            friendship = MAX_FRIENDSHIP as i16;
        }
        SetMonData(mon, MON_DATA_FRIENDSHIP, &raw mut friendship as *mut c_void);
    }
}
pub unsafe fn MonGainEVs(mon: *mut Pokemon, defeatedSpecies: u16) {
    let mut evs: CArray<u8, 6> = zeroed();
    let mut evIncrease: u16 = 0;
    let mut totalEVs: u16 = 0;
    let mut heldItem: u16 = 0;
    let mut holdEffect: u8 = 0;
    let mut multiplier: i32 = 0;
    let mut i: i32 = 0;
    while i < NUM_STATS {
        evs[i] = GetMonData3(mon, MON_DATA_HP_EV + i, null_mut()) as u8;
        totalEVs += evs[i] as u16;
        i += 1;
    }
    for i in 0..NUM_STATS {
        if totalEVs >= MAX_TOTAL_EVS as u16 {
            break;
        }
        if CheckPartyHasHadPokerus(mon, 0) != 0 {
            multiplier = 2;
        } else {
            multiplier = 1;
        }
        match i {
            0 => {
                evIncrease = gSpeciesInfo[defeatedSpecies].evYield_HP() * multiplier as u16;
            }
            1 => {
                evIncrease = gSpeciesInfo[defeatedSpecies].evYield_Attack() * multiplier as u16;
            }
            STAT_DEF => {
                evIncrease = gSpeciesInfo[defeatedSpecies].evYield_Defense() * multiplier as u16;
            }
            3 => {
                evIncrease = gSpeciesInfo[defeatedSpecies].evYield_Speed() * multiplier as u16;
            }
            4 => {
                evIncrease = gSpeciesInfo[defeatedSpecies].evYield_SpAttack() * multiplier as u16;
            }
            STAT_SPDEF => {
                evIncrease = gSpeciesInfo[defeatedSpecies].evYield_SpDefense() * multiplier as u16;
            }
            _ => {}
        }
        heldItem = GetMonData3(mon, MON_DATA_HELD_ITEM, null_mut()) as u16;
        if heldItem == ITEM_ENIGMA_BERRY {
            if gMain.inBattle() != 0 {
                holdEffect = gEnigmaBerries[0].holdEffect;
            } else {
                holdEffect = (*gSaveBlock1Ptr).enigmaBerry.holdEffect;
            }
        } else {
            holdEffect = GetItemHoldEffect(heldItem);
        }
        if holdEffect == HOLD_EFFECT_MACHO_BRACE {
            evIncrease *= 2;
        }
        if totalEVs as i32 + evIncrease as i16 as i32 > MAX_TOTAL_EVS {
            evIncrease = evIncrease as i16 as u16 + MAX_TOTAL_EVS as u16 - (totalEVs + evIncrease);
        }
        if evs[i] as i32 + evIncrease as i16 as i32 > MAX_PER_STAT_EVS {
            let val1: i32 = evIncrease as i16 as i32 + MAX_PER_STAT_EVS;
            let val2: i32 = evs[i] as i32 + evIncrease as i32;
            evIncrease = val1 as u16 - val2 as u16;
        }
        evs[i] += evIncrease as u8;
        totalEVs += evIncrease;
        SetMonData(mon, MON_DATA_HP_EV + i, &raw mut evs[i] as *mut c_void);
    }
}
pub unsafe fn GetMonEVCount(mon: *mut Pokemon) -> u16 {
    let mut count: u16 = 0;
    for i in 0..NUM_STATS {
        count += GetMonData3(mon, MON_DATA_HP_EV + i, null_mut()) as u16;
    }
    count
}
pub unsafe fn RandomlyGivePartyPokerus(party: *mut Pokemon) {
    let mut rnd: u16 = Random();
    if rnd == 0x4000 || rnd == 0x8000 || rnd == 0xC000 {
        let mut mon: *mut Pokemon = null_mut();
        loop {
            rnd = (Random() as i32 % 6) as u16;
            mon = party.at(rnd);
            if !(GetMonData3(mon, MON_DATA_SPECIES, null_mut()) == 0
                || GetMonData3(mon, MON_DATA_IS_EGG, null_mut()) != 0)
            {
                break;
            }
        }
        if CheckPartyHasHadPokerus(party, gBitTable[rnd] as u8) == 0 {
            let mut rnd2: u8 = 0;
            loop {
                rnd2 = Random() as u8;
                if rnd2 as i32 & 0x7 != 0 {
                    break;
                }
            }
            if rnd2 as i32 & 0xF0 != 0 {
                rnd2 &= 0x7;
            }
            rnd2 |= rnd2 << 4;
            rnd2 &= 0xF3;
            rnd2 += 1;
            SetMonData(
                party.at(rnd),
                MON_DATA_POKERUS,
                &raw mut rnd2 as *mut c_void,
            );
        }
    }
}
pub unsafe fn CheckPartyPokerus(party: *mut Pokemon, mut selection: u8) -> u8 {
    let mut partyIndex: i32 = 0;
    let mut curBit: u32 = 1;
    let mut retVal: u8 = 0;
    if selection != 0 {
        loop {
            if selection as i32 & 1 != 0
                && GetMonData3(party.at(partyIndex), MON_DATA_POKERUS, null_mut()) & 0xF != 0
            {
                retVal |= curBit as u8;
            }
            partyIndex += 1;
            curBit <<= 1;
            selection >>= 1;
            if selection == 0 {
                break;
            }
        }
    } else if GetMonData3(party, MON_DATA_POKERUS, null_mut()) & 0xF != 0 {
        retVal = 1;
    }
    retVal
}
pub unsafe fn CheckPartyHasHadPokerus(party: *mut Pokemon, mut selection: u8) -> u8 {
    let mut partyIndex: i32 = 0;
    let mut curBit: u32 = 1;
    let mut retVal: u8 = 0;
    if selection != 0 {
        loop {
            if selection as i32 & 1 != 0
                && GetMonData3(party.at(partyIndex), MON_DATA_POKERUS, null_mut()) != 0
            {
                retVal |= curBit as u8;
            }
            partyIndex += 1;
            curBit <<= 1;
            selection >>= 1;
            if selection == 0 {
                break;
            }
        }
    } else if GetMonData3(party, MON_DATA_POKERUS, null_mut()) != 0 {
        retVal = 1;
    }
    retVal
}
#[unsafe(no_mangle)]
pub unsafe fn UpdatePartyPokerusTime(days: u16) {
    for i in 0..PARTY_SIZE {
        if GetMonData3(&raw mut gPlayerParty[i], MON_DATA_SPECIES, null_mut()) != 0 {
            let mut pokerus: u8 =
                GetMonData3(&raw mut gPlayerParty[i], MON_DATA_POKERUS, null_mut()) as u8;
            if pokerus as i32 & 0xF != 0 {
                if pokerus as i32 & 0xF < days as i32 || days > 4 {
                    pokerus &= 0xF0;
                } else {
                    pokerus -= days as u8;
                }
                if pokerus == 0 {
                    pokerus = 0x10;
                }
                SetMonData(
                    &raw mut gPlayerParty[i],
                    MON_DATA_POKERUS,
                    &raw mut pokerus as *mut c_void,
                );
            }
        }
    }
}
pub unsafe fn PartySpreadPokerus(party: *mut Pokemon) {
    if Random() as i32 % 3 == 0 {
        let mut i: i32 = 0;
        while i < PARTY_SIZE {
            if GetMonData3(party.at(i), MON_DATA_SPECIES, null_mut()) != 0 {
                let pokerus: u8 = GetMonData3(party.at(i), MON_DATA_POKERUS, null_mut()) as u8;
                let mut curPokerus: u8 = pokerus;
                if pokerus != 0 && pokerus as i32 & 0xF != 0 {
                    if i != 0
                        && GetMonData3(party.at(i - 1), MON_DATA_POKERUS, null_mut()) & 0xF0 == 0
                    {
                        SetMonData(
                            party.at(i - 1),
                            MON_DATA_POKERUS,
                            &raw mut curPokerus as *mut c_void,
                        );
                    }
                    if i != 5
                        && GetMonData3(party.at(i + 1), MON_DATA_POKERUS, null_mut()) & 0xF0 == 0
                    {
                        SetMonData(
                            party.at(i + 1),
                            MON_DATA_POKERUS,
                            &raw mut curPokerus as *mut c_void,
                        );
                        i += 1;
                    }
                }
            }
            i += 1;
        }
    }
}
pub unsafe fn TryIncrementMonLevel(mon: *mut Pokemon) -> u8 {
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut nextLevel: u8 = GetMonData3(mon, MON_DATA_LEVEL, null_mut()) as u8 + 1;
    let mut expPoints: u32 = GetMonData3(mon, MON_DATA_EXP, null_mut());
    if expPoints > gExperienceTables[gSpeciesInfo[species].growthRate][100] {
        expPoints = gExperienceTables[gSpeciesInfo[species].growthRate][100];
        SetMonData(mon, MON_DATA_EXP, &raw mut expPoints as *mut c_void);
    }
    if nextLevel > MAX_LEVEL as u8
        || expPoints < gExperienceTables[gSpeciesInfo[species].growthRate][nextLevel]
    {
        return FALSE;
    } else {
        SetMonData(mon, MON_DATA_LEVEL, &raw mut nextLevel as *mut c_void);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn CanMonLearnTMHM(mon: *mut Pokemon, tm: u8) -> u32 {
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
    if species == SPECIES_EGG as u16 {
        return 0;
    }
    if 8 <= 8 {
        if tm < 32 {
            let mask: u32 = shl_i32(1, tm as u32) as u32;
            return gTMHMLearnsets[species].as_u32s[0] & mask;
        } else {
            let mask: u32 = shl_i32(1, tm as u32 - 32) as u32;
            return gTMHMLearnsets[species].as_u32s[1] & mask;
        }
    } else {
        let index: u32 = (tm as i32 / 32) as u32;
        let mask: u32 = shl_i32(1, (tm as i32 % 32) as u32) as u32;
        return gTMHMLearnsets[species].as_u32s[index] & mask;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn CanSpeciesLearnTMHM(species: u16, tm: u8) -> u32 {
    if species == SPECIES_EGG as u16 {
        return 0;
    }
    if 8 <= 8 {
        if tm < 32 {
            let mask: u32 = shl_i32(1, tm as u32) as u32;
            return gTMHMLearnsets[species].as_u32s[0] & mask;
        } else {
            let mask: u32 = shl_i32(1, tm as u32 - 32) as u32;
            return gTMHMLearnsets[species].as_u32s[1] & mask;
        }
    } else {
        let index: u32 = (tm as i32 / 32) as u32;
        let mask: u32 = shl_i32(1, (tm as i32 % 32) as u32) as u32;
        return gTMHMLearnsets[species].as_u32s[index] & mask;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetMoveRelearnerMoves(mon: *mut Pokemon, moves: *mut u16) -> u8 {
    let mut learnedMoves: CArray<u16, 4> = zeroed();
    let mut numMoves: u8 = 0;
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let level: u8 = GetMonData3(mon, MON_DATA_LEVEL, null_mut()) as u8;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        learnedMoves[i] = GetMonData3(mon, MON_DATA_MOVE1 + i, null_mut()) as u16;
        i += 1;
    }
    for i in 0..MAX_LEVEL_UP_MOVES {
        if *gLevelUpLearnsets[species].at(i) == LEVEL_UP_END {
            break;
        }
        let moveLevel: u16 = *gLevelUpLearnsets[species].at(i) & LEVEL_UP_MOVE_LV as u16;
        if moveLevel as i32 <= (level as i32) << 9 {
            j = 0;
            while j < MAX_MON_MOVES
                && learnedMoves[j] as i32
                    != *gLevelUpLearnsets[species].at(i) as i32 & LEVEL_UP_MOVE_ID as i32
            {
                j += 1;
            }
            if j == MAX_MON_MOVES {
                k = 0;
                while k < numMoves as i32
                    && *moves.at(k) as i32
                        != *gLevelUpLearnsets[species].at(i) as i32 & LEVEL_UP_MOVE_ID as i32
                {
                    k += 1;
                }
                if k == numMoves as i32 {
                    *moves.at({
                        let t1 = numMoves;
                        numMoves += 1;
                        t1
                    }) = *gLevelUpLearnsets[species].at(i) & LEVEL_UP_MOVE_ID;
                }
            }
        }
    }
    numMoves
}
pub unsafe fn GetLevelUpMovesBySpecies(species: u16, moves: *mut u16) -> u8 {
    let mut numMoves: u8 = 0;
    let mut i: i32 = 0;
    while i < MAX_LEVEL_UP_MOVES && *gLevelUpLearnsets[species].at(i) != LEVEL_UP_END {
        *moves.at({
            let t1 = numMoves;
            numMoves += 1;
            t1
        }) = *gLevelUpLearnsets[species].at(i) & LEVEL_UP_MOVE_ID;
        i += 1;
    }
    numMoves
}
pub unsafe fn GetNumberOfRelearnableMoves(mon: *mut Pokemon) -> u8 {
    let mut learnedMoves: CArray<u16, 4> = zeroed();
    let mut moves: CArray<u16, 20> = zeroed();
    let mut numMoves: u8 = 0;
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
    let level: u8 = GetMonData3(mon, MON_DATA_LEVEL, null_mut()) as u8;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    if species == SPECIES_EGG as u16 {
        return 0;
    }
    let mut i: i32 = 0;
    while i < MAX_MON_MOVES {
        learnedMoves[i] = GetMonData3(mon, MON_DATA_MOVE1 + i, null_mut()) as u16;
        i += 1;
    }
    for i in 0..MAX_LEVEL_UP_MOVES {
        if *gLevelUpLearnsets[species].at(i) == LEVEL_UP_END {
            break;
        }
        let moveLevel: u16 = *gLevelUpLearnsets[species].at(i) & LEVEL_UP_MOVE_LV as u16;
        if moveLevel as i32 <= (level as i32) << 9 {
            j = 0;
            while j < MAX_MON_MOVES
                && learnedMoves[j] as i32
                    != *gLevelUpLearnsets[species].at(i) as i32 & LEVEL_UP_MOVE_ID as i32
            {
                j += 1;
            }
            if j == MAX_MON_MOVES {
                k = 0;
                while k < numMoves as i32
                    && moves[k] as i32
                        != *gLevelUpLearnsets[species].at(i) as i32 & LEVEL_UP_MOVE_ID as i32
                {
                    k += 1;
                }
                if k == numMoves as i32 {
                    moves[{
                        let t1 = numMoves;
                        numMoves += 1;
                        t1
                    }] = *gLevelUpLearnsets[species].at(i) & LEVEL_UP_MOVE_ID;
                }
            }
        }
    }
    numMoves
}
pub unsafe fn SpeciesToPokedexNum(mut species: u16) -> u16 {
    if IsNationalPokedexEnabled() != 0 {
        return SpeciesToNationalPokedexNum(species);
    } else {
        species = SpeciesToHoennPokedexNum(species);
        if species <= HOENN_DEX_DEOXYS {
            return species;
        }
        return 0xFFFF;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn IsSpeciesInHoennDex(species: u16) -> u32 {
    if SpeciesToHoennPokedexNum(species) > HOENN_DEX_DEOXYS {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ClearBattleMonForms() {
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        gBattleMonForms[i] = 0;
    }
}
pub unsafe fn GetBattleBGM() -> u16 {
    if gBattleTypeFlags & BATTLE_TYPE_KYOGRE_GROUDON != 0 {
        return MUS_VS_KYOGRE_GROUDON;
    } else if gBattleTypeFlags & BATTLE_TYPE_REGI != 0 {
        return MUS_VS_REGI;
    } else if gBattleTypeFlags & 0x2000002 != 0 {
        return MUS_VS_TRAINER;
    } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0 {
        let mut trainerClass: u8 = 0;
        if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
            trainerClass = GetFrontierOpponentClass(gTrainerBattleOpponent_A);
        } else if gBattleTypeFlags & BATTLE_TYPE_TRAINER_HILL != 0 {
            trainerClass = TRAINER_CLASS_EXPERT;
        } else {
            trainerClass = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A]
                .trainerClass;
        }
        match trainerClass {
            TRAINER_CLASS_AQUA_LEADER | TRAINER_CLASS_MAGMA_LEADER => {
                return MUS_VS_AQUA_MAGMA_LEADER;
            }
            TRAINER_CLASS_TEAM_AQUA
            | TRAINER_CLASS_TEAM_MAGMA
            | TRAINER_CLASS_AQUA_ADMIN
            | TRAINER_CLASS_MAGMA_ADMIN => {
                return MUS_VS_AQUA_MAGMA;
            }
            TRAINER_CLASS_LEADER => {
                return MUS_VS_GYM_LEADER;
            }
            TRAINER_CLASS_CHAMPION => {
                return MUS_VS_CHAMPION;
            }
            TRAINER_CLASS_RIVAL => {
                if gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
                    return MUS_VS_RIVAL;
                }
                if StringCompare(
                    (*(&raw const crate::data::data_tables::gTrainers)
                        .cast::<CArray<Trainer, 0>>())[gTrainerBattleOpponent_A]
                        .trainerName
                        .as_ptr()
                        .cast_mut(),
                    (*(&raw const crate::data::battle_message::gText_BattleWallyName)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                ) == 0
                {
                    return MUS_VS_TRAINER;
                }
                return MUS_VS_RIVAL;
            }
            TRAINER_CLASS_ELITE_FOUR => {
                return MUS_VS_ELITE_FOUR;
            }
            TRAINER_CLASS_SALON_MAIDEN
            | TRAINER_CLASS_DOME_ACE
            | TRAINER_CLASS_PALACE_MAVEN
            | TRAINER_CLASS_ARENA_TYCOON
            | TRAINER_CLASS_FACTORY_HEAD
            | TRAINER_CLASS_PIKE_QUEEN
            | TRAINER_CLASS_PYRAMID_KING => {
                return MUS_VS_FRONTIER_BRAIN;
            }
            _ => {
                return MUS_VS_TRAINER;
            }
        }
    } else {
        return MUS_VS_WILD;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn PlayBattleBGM() {
    ResetMapMusic();
    m4aMPlayAllStop();
    PlayBGM(GetBattleBGM());
}
pub unsafe fn PlayMapChosenOrBattleBGM(songId: u16) {
    ResetMapMusic();
    m4aMPlayAllStop();
    if songId != 0 {
        PlayNewMapMusic(songId);
    } else {
        PlayNewMapMusic(GetBattleBGM());
    }
}
pub unsafe fn CreateTask_PlayMapChosenOrBattleBGM(songId: u16) {
    ResetMapMusic();
    m4aMPlayAllStop();
    let taskId: u8 = CreateTask(Some(Task_PlayMapChosenOrBattleBGM), 0);
    task_set(taskId, tSongId, songId as i16);
}
pub(crate) unsafe fn Task_PlayMapChosenOrBattleBGM(taskId: u8) {
    if task_get(taskId, tSongId) != 0 {
        PlayNewMapMusic(task_get(taskId, tSongId) as u16);
    } else {
        PlayNewMapMusic(GetBattleBGM());
    }
    DestroyTask(taskId);
}
pub unsafe fn GetMonFrontSpritePal(mon: *mut Pokemon) -> *mut u32 {
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
    let otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    let personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    GetMonSpritePalFromSpeciesAndPersonality(species, otId, personality)
}
#[unsafe(no_mangle)]
pub unsafe fn GetMonSpritePalFromSpeciesAndPersonality(
    species: u16,
    otId: u32,
    personality: u32,
) -> *mut u32 {
    if species > NUM_SPECIES {
        return (*(&raw const crate::data::data_tables::gMonPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[0]
            .data;
    }
    let shinyValue: u32 = (otId & 0xFFFF0000) >> 16
        ^ otId & 0xFFFF
        ^ (personality & 0xFFFF0000) >> 16
        ^ personality & 0xFFFF;
    if shinyValue < SHINY_ODDS {
        return (*(&raw const crate::data::data_tables::gMonShinyPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[species]
            .data;
    } else {
        return (*(&raw const crate::data::data_tables::gMonPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[species]
            .data;
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
pub unsafe fn GetMonSpritePalStruct(mon: *mut Pokemon) -> *mut CompressedSpritePalette {
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
    let otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    let personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    GetMonSpritePalStructFromOtIdPersonality(species, otId, personality)
}
#[unsafe(no_mangle)]
pub unsafe fn GetMonSpritePalStructFromOtIdPersonality(
    species: u16,
    otId: u32,
    personality: u32,
) -> *mut CompressedSpritePalette {
    let shinyValue: u32 = (otId & 0xFFFF0000) >> 16
        ^ otId & 0xFFFF
        ^ (personality & 0xFFFF0000) >> 16
        ^ personality & 0xFFFF;
    if shinyValue < SHINY_ODDS {
        return (&raw const (*(&raw const crate::data::data_tables::gMonShinyPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[species])
            .cast_mut();
    } else {
        return (&raw const (*(&raw const crate::data::data_tables::gMonPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[species])
            .cast_mut();
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
pub fn IsHMMove2(r#move: u16) -> u32 {
    let mut i: i32 = 0;
    while sHMMoves[i] != HM_MOVES_END {
        if sHMMoves[{
            let t1 = i;
            i += 1;
            t1
        }] == r#move
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
pub unsafe fn IsMonSpriteNotFlipped(species: u16) -> u8 {
    gSpeciesInfo[species].noFlip()
}
pub unsafe fn GetMonFlavorRelation(mon: *mut Pokemon, flavor: u8) -> i8 {
    let nature: u8 = GetNature(mon);
    (*(&raw const crate::data::pokeblock::gPokeblockFlavorCompatibilityTable)
        .cast::<CArray<i8, 125>>())[nature as i32 * FLAVOR_COUNT + flavor as i32]
}
pub unsafe fn GetFlavorRelationByPersonality(personality: u32, flavor: u8) -> i8 {
    let nature: u8 = GetNatureFromPersonality(personality);
    (*(&raw const crate::data::pokeblock::gPokeblockFlavorCompatibilityTable)
        .cast::<CArray<i8, 125>>())[nature as i32 * FLAVOR_COUNT + flavor as i32]
}
pub unsafe fn IsTradedMon(mon: *mut Pokemon) -> u8 {
    let mut otName: CArray<u8, 8> = zeroed();
    GetMonData3(mon, MON_DATA_OT_NAME, otName.as_mut_ptr());
    let otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    IsOtherTrainer(otId, otName.as_mut_ptr())
}
pub unsafe fn IsOtherTrainer(otId: u32, otName: *mut u8) -> u8 {
    if otId
        == (*gSaveBlock2Ptr).playerTrainerId[0] as u32
            | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
            | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
            | ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24
    {
        let mut i: i32 = 0;
        while *otName.at(i) != EOS {
            if *otName.at(i) != (*gSaveBlock2Ptr).playerName[i] {
                return TRUE;
            }
            i += 1;
        }
        return FALSE;
    }
    TRUE
}
pub unsafe fn MonRestorePP(mon: *mut Pokemon) {
    BoxMonRestorePP(&raw mut (*mon).r#box);
}
pub unsafe fn BoxMonRestorePP(boxMon: *mut BoxPokemon) {
    for i in 0..MAX_MON_MOVES {
        if GetBoxMonData3(boxMon, MON_DATA_MOVE1 + i, null_mut()) != 0 {
            let r#move: u16 = GetBoxMonData3(boxMon, MON_DATA_MOVE1 + i, null_mut()) as u16;
            let bonus: u16 = GetBoxMonData3(boxMon, MON_DATA_PP_BONUSES, null_mut()) as u16;
            let mut pp: u8 = CalculatePPWithBonus(r#move, bonus as u8, i as u8);
            SetBoxMonData(boxMon, MON_DATA_PP1 + i, &raw mut pp as *mut c_void);
        }
    }
}
pub unsafe fn SetMonPreventsSwitchingString() {
    gLastUsedAbility = (*gBattleStruct).abilityPreventingSwitchout;
    gBattleTextBuff1[0] = B_BUFF_PLACEHOLDER_BEGIN;
    gBattleTextBuff1[1] = B_BUFF_MON_NICK_WITH_PREFIX;
    gBattleTextBuff1[2] = (*gBattleStruct).battlerPreventingSwitchout;
    gBattleTextBuff1[4] = B_BUFF_EOS;
    if GetBattlerSide((*gBattleStruct).battlerPreventingSwitchout) == B_SIDE_PLAYER {
        gBattleTextBuff1[3] = GetPartyIdFromBattlePartyId(
            gBattlerPartyIndexes[(*gBattleStruct).battlerPreventingSwitchout] as u8,
        );
    } else {
        gBattleTextBuff1[3] =
            gBattlerPartyIndexes[(*gBattleStruct).battlerPreventingSwitchout] as u8;
    }
    gBattleTextBuff2[0] = 0xFD;
    gBattleTextBuff2[1] = 4;
    gBattleTextBuff2[2] = gBattlerInMenuId;
    gBattleTextBuff2[3] = GetPartyIdFromBattlePartyId(gBattlerPartyIndexes[gBattlerInMenuId] as u8);
    gBattleTextBuff2[4] = 0xFF;
    BattleStringExpandPlaceholders(
        (*(&raw const crate::data::battle_message::gText_PkmnsXPreventsSwitching)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut(),
        gStringVar4.as_mut_ptr(),
    );
}
fn GetWildMonTableIdInAlteringCave(species: u16) -> i32 {
    for i in 0..9i32 {
        if sAlteringCaveWildMonHeldItems[i].species == species {
            return i;
        }
    }
    0
}
pub unsafe fn SetWildMonHeldItem() {
    if gBattleTypeFlags & 0x302008 == 0 {
        let rnd: u16 = (Random() as i32 % 100) as u16;
        let species: u16 =
            GetMonData3(&raw mut gEnemyParty[0], MON_DATA_SPECIES, null_mut()) as u16;
        let mut chanceNoItem: u16 = 45;
        let mut chanceNotRare: u16 = 95;
        if GetMonData3(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG, null_mut()) == 0
            && GetMonAbility(&raw mut gPlayerParty[0]) == ABILITY_COMPOUND_EYES
        {
            chanceNoItem = 20;
            chanceNotRare = 80;
        }
        if gMapHeader.mapLayoutId == LAYOUT_ALTERING_CAVE {
            let alteringCaveId: i32 = GetWildMonTableIdInAlteringCave(species);
            if alteringCaveId != 0 {
                if rnd < chanceNotRare {
                    return;
                }
                SetMonData(
                    &raw mut gEnemyParty[0],
                    MON_DATA_HELD_ITEM,
                    (&raw const sAlteringCaveWildMonHeldItems[alteringCaveId].item).cast_mut()
                        as *mut c_void,
                );
            } else {
                if rnd < chanceNoItem {
                    return;
                }
                if rnd < chanceNotRare {
                    SetMonData(
                        &raw mut gEnemyParty[0],
                        MON_DATA_HELD_ITEM,
                        (&raw const gSpeciesInfo[species].itemCommon).cast_mut() as *mut c_void,
                    );
                } else {
                    SetMonData(
                        &raw mut gEnemyParty[0],
                        MON_DATA_HELD_ITEM,
                        (&raw const gSpeciesInfo[species].itemRare).cast_mut() as *mut c_void,
                    );
                }
            }
        } else {
            if gSpeciesInfo[species].itemCommon == gSpeciesInfo[species].itemRare
                && gSpeciesInfo[species].itemCommon != ITEM_NONE
            {
                SetMonData(
                    &raw mut gEnemyParty[0],
                    MON_DATA_HELD_ITEM,
                    (&raw const gSpeciesInfo[species].itemCommon).cast_mut() as *mut c_void,
                );
            } else {
                if rnd < chanceNoItem {
                    return;
                }
                if rnd < chanceNotRare {
                    SetMonData(
                        &raw mut gEnemyParty[0],
                        MON_DATA_HELD_ITEM,
                        (&raw const gSpeciesInfo[species].itemCommon).cast_mut() as *mut c_void,
                    );
                } else {
                    SetMonData(
                        &raw mut gEnemyParty[0],
                        MON_DATA_HELD_ITEM,
                        (&raw const gSpeciesInfo[species].itemRare).cast_mut() as *mut c_void,
                    );
                }
            }
        }
    }
}
pub unsafe fn IsMonShiny(mon: *mut Pokemon) -> u8 {
    let otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    let personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    IsShinyOtIdPersonality(otId, personality)
}
pub fn IsShinyOtIdPersonality(otId: u32, personality: u32) -> u8 {
    let mut retVal: u8 = FALSE;
    let shinyValue: u32 = (otId & 0xFFFF0000) >> 16
        ^ otId & 0xFFFF
        ^ (personality & 0xFFFF0000) >> 16
        ^ personality & 0xFFFF;
    if shinyValue < SHINY_ODDS {
        retVal = TRUE;
    }
    retVal
}
pub unsafe fn GetTrainerPartnerName() -> *mut u8 {
    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 {
        if gPartnerTrainerId == TRAINER_STEVEN_PARTNER {
            return (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[804]
                .trainerName
                .as_ptr()
                .cast_mut();
        } else {
            GetFrontierTrainerName(gStringVar1.as_mut_ptr(), gPartnerTrainerId);
            return gStringVar1.as_mut_ptr();
        }
    } else {
        let id: u8 = GetMultiplayerId();
        return gLinkPlayers[GetBattlerMultiplayerId(gLinkPlayers[id].id ^ 2)]
            .name
            .as_mut_ptr();
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
pub(crate) unsafe fn Task_AnimateAfterDelay(taskId: u8) {
    if ({
        task_set(taskId, sAnimDelay, task_get(taskId, sAnimDelay) - 1);
        task_get(taskId, sAnimDelay)
    }) == 0
    {
        LaunchAnimationTaskForFrontSprite(
            (task_get(taskId, 0) as u16 as i32 | (task_get(taskId, 1) as u16 as i32) << 16) as usize
                as *mut c_void as *mut Sprite,
            task_get(taskId, sAnimId) as u8,
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_PokemonSummaryAnimateAfterDelay(taskId: u8) {
    if ({
        task_set(taskId, sAnimDelay, task_get(taskId, sAnimDelay) - 1);
        task_get(taskId, sAnimDelay)
    }) == 0
    {
        StartMonSummaryAnimation(
            (task_get(taskId, 0) as u16 as i32 | (task_get(taskId, 1) as u16 as i32) << 16) as usize
                as *mut c_void as *mut Sprite,
            task_get(taskId, sAnimId) as u8,
        );
        SummaryScreen_SetAnimDelayTaskId(TASK_NONE);
        DestroyTask(taskId);
    }
}
pub unsafe fn BattleAnimateFrontSprite(sprite: *mut Sprite, species: u16, noCry: u8, panMode: u8) {
    if gHitMarker & HITMARKER_NO_ANIMATIONS != 0 && gBattleTypeFlags & 0x2000002 == 0 {
        DoMonFrontSpriteAnimation(sprite, species, noCry, panMode | SKIP_FRONT_ANIM);
    } else {
        DoMonFrontSpriteAnimation(sprite, species, noCry, panMode);
    }
}
pub unsafe fn DoMonFrontSpriteAnimation(
    sprite: *mut Sprite,
    species: u16,
    noCry: u8,
    panModeAnimFlag: u8,
) {
    let mut pan: i8 = 0;
    match panModeAnimFlag as i32 & 127 {
        0 => {
            pan = -25;
        }
        1 => {
            pan = 25;
        }
        _ => {
            pan = 0;
        }
    }
    if panModeAnimFlag as i32 & SKIP_FRONT_ANIM as i32 != 0 {
        if noCry == 0 {
            PlayCry_Normal(species, pan);
        }
        (*sprite).callback = Some(SpriteCallbackDummy);
    } else {
        if noCry == 0 {
            PlayCry_Normal(species, pan);
            if HasTwoFramesAnimation(species) != 0 {
                StartSpriteAnim(sprite, 1);
            }
        }
        if sMonAnimationDelayTable[species as i32 - 1] != 0 {
            let taskId: u8 = CreateTask(Some(Task_AnimateAfterDelay), 0);
            task_set(taskId, 0, sprite as usize as u32 as i16);
            task_set(taskId, 1, (sprite as usize as u32 >> 16) as i16);
            task_set(
                taskId,
                sAnimId,
                sMonFrontAnimIdsTable[species as i32 - 1] as i16,
            );
            task_set(
                taskId,
                sAnimDelay,
                sMonAnimationDelayTable[species as i32 - 1] as i16,
            );
        } else {
            LaunchAnimationTaskForFrontSprite(sprite, sMonFrontAnimIdsTable[species as i32 - 1]);
        }
        (*sprite).callback = Some(SpriteCallbackDummy_2);
    }
}
pub unsafe fn PokemonSummaryDoMonAnimation(sprite: *mut Sprite, species: u16, oneFrame: u8) {
    if oneFrame == 0 && HasTwoFramesAnimation(species) != 0 {
        StartSpriteAnim(sprite, 1);
    }
    if sMonAnimationDelayTable[species as i32 - 1] != 0 {
        let taskId: u8 = CreateTask(Some(Task_PokemonSummaryAnimateAfterDelay), 0);
        task_set(taskId, 0, sprite as usize as u32 as i16);
        task_set(taskId, 1, (sprite as usize as u32 >> 16) as i16);
        task_set(
            taskId,
            sAnimId,
            sMonFrontAnimIdsTable[species as i32 - 1] as i16,
        );
        task_set(
            taskId,
            sAnimDelay,
            sMonAnimationDelayTable[species as i32 - 1] as i16,
        );
        SummaryScreen_SetAnimDelayTaskId(taskId);
        SetSpriteCB_MonAnimDummy(sprite);
    } else {
        StartMonSummaryAnimation(sprite, sMonFrontAnimIdsTable[species as i32 - 1]);
    }
}
pub unsafe fn StopPokemonAnimationDelayTask() {
    let delayTaskId: u8 = FindTaskIdByFunc(Some(Task_PokemonSummaryAnimateAfterDelay));
    if delayTaskId != TASK_NONE {
        DestroyTask(delayTaskId);
    }
}
pub unsafe fn BattleAnimateBackSprite(sprite: *mut Sprite, species: u16) {
    if gHitMarker & HITMARKER_NO_ANIMATIONS != 0 && gBattleTypeFlags & 0x2000002 == 0 {
        (*sprite).callback = Some(SpriteCallbackDummy);
    } else {
        LaunchAnimationTaskForBackSprite(sprite, GetSpeciesBackAnimSet(species));
        (*sprite).callback = Some(SpriteCallbackDummy_2);
    }
}
unsafe fn GetOwnOpposingLinkMultiBattlerId(rightSide: u8) -> u8 {
    let mut battler: i32 = 0;
    let multiplayerId: u8 = GetMultiplayerId();
    match gLinkPlayers[multiplayerId].id {
        0 | 2 => {
            battler = if rightSide != 0 { 1 } else { 3 };
        }
        1 | 3 => {
            battler = if rightSide != 0 { 2 } else { 0 };
        }
        _ => {}
    }
    let mut i: i32 = 0;
    while i < MAX_LINK_PLAYERS {
        if gLinkPlayers[i].id as i32 == battler as i16 as i32 {
            break;
        }
        i += 1;
    }
    i as u8
}
pub unsafe fn GetOpposingLinkMultiBattlerId(rightSide: u8, multiplayerId: u8) -> u8 {
    let mut battler: i32 = 0;
    match gLinkPlayers[multiplayerId].id {
        0 | 2 => {
            battler = if rightSide != 0 { 1 } else { 3 };
        }
        1 | 3 => {
            battler = if rightSide != 0 { 2 } else { 0 };
        }
        _ => {}
    }
    let mut i: i32 = 0;
    while i < MAX_LINK_PLAYERS {
        if gLinkPlayers[i].id as i32 == battler as i16 as i32 {
            break;
        }
        i += 1;
    }
    i as u8
}
pub unsafe fn FacilityClassToPicIndex(facilityClass: u16) -> u16 {
    gFacilityClassToPicIndex[facilityClass] as u16
}
pub unsafe fn PlayerGenderToFrontTrainerPicId(playerGender: u8) -> u16 {
    if playerGender != MALE {
        return FacilityClassToPicIndex(FACILITY_CLASS_MAY);
    } else {
        return FacilityClassToPicIndex(FACILITY_CLASS_BRENDAN);
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn HandleSetPokedexFlag(nationalNum: u16, caseId: u8, personality: u32) {
    let getFlagCaseId: u8 = (if caseId == FLAG_SET_SEEN {
        FLAG_GET_SEEN as i32
    } else {
        FLAG_GET_CAUGHT as i32
    }) as u8;
    if GetSetPokedexFlag(nationalNum, getFlagCaseId) == 0 {
        GetSetPokedexFlag(nationalNum, caseId);
        if NationalPokedexNumToSpecies(nationalNum) == SPECIES_UNOWN {
            (*gSaveBlock2Ptr).pokedex.unownPersonality = personality;
        }
        if NationalPokedexNumToSpecies(nationalNum) == SPECIES_SPINDA {
            (*gSaveBlock2Ptr).pokedex.spindaPersonality = personality;
        }
    }
}
pub unsafe fn GetTrainerClassNameFromId(mut trainerId: u16) -> *mut u8 {
    if trainerId >= TRAINERS_COUNT {
        trainerId = TRAINER_NONE;
    }
    (*(&raw const crate::data::data_tables::gTrainerClassNames).cast::<CArray<CArray<u8, 13>, 0>>())
        [(*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
            [trainerId]
            .trainerClass]
        .as_ptr()
        .cast_mut()
}
pub unsafe fn GetTrainerNameFromId(mut trainerId: u16) -> *mut u8 {
    if trainerId >= TRAINERS_COUNT {
        trainerId = TRAINER_NONE;
    }
    (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())[trainerId]
        .trainerName
        .as_ptr()
        .cast_mut()
}
pub unsafe fn HasTwoFramesAnimation(species: u16) -> u8 {
    (species != SPECIES_CASTFORM
        && species != SPECIES_DEOXYS as u16
        && species != SPECIES_SPINDA
        && species != SPECIES_UNOWN) as u8
}
unsafe fn ShouldSkipFriendshipChange() -> u8 {
    if gMain.inBattle() != 0 && gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
        return TRUE;
    }
    if gMain.inBattle() == 0
        && (InBattlePike() != 0 || CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE)
    {
        return TRUE;
    }
    FALSE
}
unsafe fn InitMonSpritesGfx_Battle(gfx: *mut MonSpritesGfxManager) {
    let mut j: u16 = 0;
    let mut i: u16 = 0;
    while (i as u32) < (*gfx).numSprites() {
        *(*gfx).templates.at(i) = gBattlerSpriteTemplates[i];
        j = 0;
        while (j as u32) < (*gfx).numFrames() {
            (*(*gfx)
                .frameImages
                .at(i as u32 * (*gfx).numFrames() + j as u32))
            .data =
                (*(*gfx).spritePointers.at(i)).at(j as i32 * MON_PIC_SIZE as i32) as *mut c_void;
            j += 1;
        }
        (*(*gfx).templates.at(i)).images = (*gfx).frameImages.at(i as u32 * (*gfx).numFrames());
        i += 1;
    }
}
unsafe fn InitMonSpritesGfx_FullParty(gfx: *mut MonSpritesGfxManager) {
    let mut j: u16 = 0;
    let mut i: u16 = 0;
    while (i as u32) < (*gfx).numSprites() {
        *(*gfx).templates.at(i) = *sSpriteTemplate_64x64;
        j = 0;
        while (j as u32) < (*gfx).numFrames() {
            (*(*gfx)
                .frameImages
                .at(i as u32 * (*gfx).numSprites() + j as u32))
            .data =
                (*(*gfx).spritePointers.at(i)).at(j as i32 * MON_PIC_SIZE as i32) as *mut c_void;
            j += 1;
        }
        (*(*gfx).templates.at(i)).images = (*gfx).frameImages.at(i as u32 * (*gfx).numSprites());
        (*(*gfx).templates.at(i)).anims = (*(&raw const crate::data::data_tables::gAnims_MonPic)
            .cast::<CArray<*mut AnimCmd, 0>>())
        .as_ptr()
        .cast_mut();
        (*(*gfx).templates.at(i)).paletteTag = i;
        i += 1;
    }
}
pub unsafe fn CreateMonSpritesGfxManager(mut managerId: u8, mode: u8) -> *mut MonSpritesGfxManager {
    let mut i: u8 = 0;
    let mut failureFlags: u8 = 0;
    managerId = (managerId as i32 % 2) as u8;
    let gfx: *mut MonSpritesGfxManager = AllocZeroed(20) as *mut MonSpritesGfxManager;
    if gfx.is_null() {
        return null_mut();
    }
    match mode {
        2 => {
            (*gfx).set_numSprites(7);
            (*gfx).set_numSprites2(7);
            (*gfx).set_numFrames(MAX_MON_PIC_FRAMES);
            (*gfx).set_dataSize(1);
            (*gfx).set_mode(MON_SPR_GFX_MODE_FULL_PARTY);
        }
        _ => {
            (*gfx).set_numSprites(MAX_BATTLERS_COUNT as u32);
            (*gfx).set_numSprites2(MAX_BATTLERS_COUNT as u32);
            (*gfx).set_numFrames(MAX_MON_PIC_FRAMES);
            (*gfx).set_dataSize(1);
            (*gfx).set_mode(MON_SPR_GFX_MODE_NORMAL);
        }
    }
    (*gfx).spriteBuffer = AllocZeroed(
        (*gfx).dataSize() * MON_PIC_SIZE as u32 * MAX_MON_PIC_FRAMES * (*gfx).numSprites(),
    );
    (*gfx).spritePointers = AllocZeroed((*gfx).numSprites() * 32) as *mut *mut u8;
    if (*gfx).spriteBuffer.is_null() || (*gfx).spritePointers.is_null() {
        failureFlags |= ALLOC_FAIL_BUFFER;
    } else {
        i = 0;
        while (i as u32) < (*gfx).numSprites() {
            *(*gfx).spritePointers.at(i) = ((*gfx).spriteBuffer as *mut u8)
                .at((*gfx).dataSize() * MON_PIC_SIZE as u32 * MAX_MON_PIC_FRAMES * i as u32)
                as *mut c_void as *mut u8;
            i += 1;
        }
    }
    (*gfx).templates = AllocZeroed(24 * (*gfx).numSprites()) as *mut SpriteTemplate;
    (*gfx).frameImages =
        AllocZeroed(8 * (*gfx).numSprites() * (*gfx).numFrames()) as *mut SpriteFrameImage;
    if (*gfx).templates.is_null() || (*gfx).frameImages.is_null() {
        failureFlags |= ALLOC_FAIL_STRUCT;
    } else {
        i = 0;
        while (i as u32) < (*gfx).numFrames() * (*gfx).numSprites() {
            (*(*gfx).frameImages.at(i)).size = MON_PIC_SIZE;
            i += 1;
        }
        match (*gfx).mode() {
            MON_SPR_GFX_MODE_FULL_PARTY => {
                InitMonSpritesGfx_FullParty(gfx);
            }
            _ => {
                InitMonSpritesGfx_Battle(gfx);
            }
        }
    }
    if failureFlags as i32 & ALLOC_FAIL_STRUCT as i32 != 0 {
        if !(*gfx).frameImages.is_null() {
            Free((*gfx).frameImages as *mut c_void);
            (*gfx).frameImages = null_mut();
        }
        if !(*gfx).templates.is_null() {
            Free((*gfx).templates as *mut c_void);
            (*gfx).templates = null_mut();
        }
    }
    if failureFlags as i32 & ALLOC_FAIL_BUFFER as i32 != 0 {
        if !(*gfx).spritePointers.is_null() {
            Free((*gfx).spritePointers as *mut c_void);
            (*gfx).spritePointers = null_mut();
        }
        if !(*gfx).spriteBuffer.is_null() {
            Free((*gfx).spriteBuffer);
            (*gfx).spriteBuffer = null_mut();
        }
    }
    if failureFlags != 0 {
        memset(gfx as *mut u8, 0, 20);
        Free(gfx as *mut c_void);
    } else {
        (*gfx).set_active(GFX_MANAGER_ACTIVE);
        sMonSpritesGfxManagers[managerId] = gfx;
    }
    sMonSpritesGfxManagers[managerId]
}
pub unsafe fn DestroyMonSpritesGfxManager(mut managerId: u8) {
    managerId = (managerId as i32 % 2) as u8;
    let gfx: *mut MonSpritesGfxManager = sMonSpritesGfxManagers[managerId];
    if gfx.is_null() {
        return;
    }
    if (*gfx).active() != GFX_MANAGER_ACTIVE {
        memset(gfx as *mut u8, 0, 20);
    } else {
        if !(*gfx).frameImages.is_null() {
            Free((*gfx).frameImages as *mut c_void);
            (*gfx).frameImages = null_mut();
        }
        if !(*gfx).templates.is_null() {
            Free((*gfx).templates as *mut c_void);
            (*gfx).templates = null_mut();
        }
        if !(*gfx).spritePointers.is_null() {
            Free((*gfx).spritePointers as *mut c_void);
            (*gfx).spritePointers = null_mut();
        }
        if !(*gfx).spriteBuffer.is_null() {
            Free((*gfx).spriteBuffer);
            (*gfx).spriteBuffer = null_mut();
        }
        memset(gfx as *mut u8, 0, 20);
        Free(gfx as *mut c_void);
    }
}
pub unsafe fn MonSpritesGfxManager_GetSpritePtr(managerId: u8, mut spriteNum: u8) -> *mut u8 {
    let gfx: *mut MonSpritesGfxManager = sMonSpritesGfxManagers[managerId as i32 % 2];
    if (*gfx).active() != GFX_MANAGER_ACTIVE {
        return null_mut();
    } else {
        if spriteNum as u32 >= (*gfx).numSprites() {
            spriteNum = 0;
        }
        return *(*gfx).spritePointers.at(spriteNum);
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
