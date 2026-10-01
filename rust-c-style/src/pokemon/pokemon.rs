//! Translated from `src/pokemon.c` by tools/rustport/c2rs.py.
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
        ((self.bits_0 as u32 >> 0) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_FOCUS_PUNCH(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn DRAGON_CLAW(&self) -> u32 {
        ((self.bits_0 as u32 >> 1) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_DRAGON_CLAW(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn WATER_PULSE(&self) -> u32 {
        ((self.bits_0 as u32 >> 2) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_WATER_PULSE(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn CALM_MIND(&self) -> u32 {
        ((self.bits_0 as u32 >> 3) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_CALM_MIND(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn ROAR(&self) -> u32 {
        ((self.bits_0 as u32 >> 4) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_ROAR(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn TOXIC(&self) -> u32 {
        ((self.bits_0 as u32 >> 5) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_TOXIC(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn HAIL(&self) -> u32 {
        ((self.bits_0 as u32 >> 6) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_HAIL(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn BULK_UP(&self) -> u32 {
        ((self.bits_0 as u32 >> 7) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_BULK_UP(&mut self, v: u32) {
        self.bits_0 = (self.bits_0 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn BULLET_SEED(&self) -> u32 {
        ((self.bits_1 as u32 >> 0) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_BULLET_SEED(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn HIDDEN_POWER(&self) -> u32 {
        ((self.bits_1 as u32 >> 1) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_HIDDEN_POWER(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn SUNNY_DAY(&self) -> u32 {
        ((self.bits_1 as u32 >> 2) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_SUNNY_DAY(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn TAUNT(&self) -> u32 {
        ((self.bits_1 as u32 >> 3) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_TAUNT(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn ICE_BEAM(&self) -> u32 {
        ((self.bits_1 as u32 >> 4) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_ICE_BEAM(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn BLIZZARD(&self) -> u32 {
        ((self.bits_1 as u32 >> 5) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_BLIZZARD(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn HYPER_BEAM(&self) -> u32 {
        ((self.bits_1 as u32 >> 6) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_HYPER_BEAM(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn LIGHT_SCREEN(&self) -> u32 {
        ((self.bits_1 as u32 >> 7) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_LIGHT_SCREEN(&mut self, v: u32) {
        self.bits_1 = (self.bits_1 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn PROTECT(&self) -> u32 {
        ((self.bits_2 as u32 >> 0) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_PROTECT(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn RAIN_DANCE(&self) -> u32 {
        ((self.bits_2 as u32 >> 1) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_RAIN_DANCE(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn GIGA_DRAIN(&self) -> u32 {
        ((self.bits_2 as u32 >> 2) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_GIGA_DRAIN(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn SAFEGUARD(&self) -> u32 {
        ((self.bits_2 as u32 >> 3) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_SAFEGUARD(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn FRUSTRATION(&self) -> u32 {
        ((self.bits_2 as u32 >> 4) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_FRUSTRATION(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn SOLAR_BEAM(&self) -> u32 {
        ((self.bits_2 as u32 >> 5) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_SOLAR_BEAM(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn IRON_TAIL(&self) -> u32 {
        ((self.bits_2 as u32 >> 6) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_IRON_TAIL(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn THUNDERBOLT(&self) -> u32 {
        ((self.bits_2 as u32 >> 7) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_THUNDERBOLT(&mut self, v: u32) {
        self.bits_2 = (self.bits_2 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn THUNDER(&self) -> u32 {
        ((self.bits_3 as u32 >> 0) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_THUNDER(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn EARTHQUAKE(&self) -> u32 {
        ((self.bits_3 as u32 >> 1) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_EARTHQUAKE(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn RETURN(&self) -> u32 {
        ((self.bits_3 as u32 >> 2) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_RETURN(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn DIG(&self) -> u32 {
        ((self.bits_3 as u32 >> 3) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_DIG(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn PSYCHIC(&self) -> u32 {
        ((self.bits_3 as u32 >> 4) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_PSYCHIC(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn SHADOW_BALL(&self) -> u32 {
        ((self.bits_3 as u32 >> 5) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_SHADOW_BALL(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn BRICK_BREAK(&self) -> u32 {
        ((self.bits_3 as u32 >> 6) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_BRICK_BREAK(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn DOUBLE_TEAM(&self) -> u32 {
        ((self.bits_3 as u32 >> 7) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_DOUBLE_TEAM(&mut self, v: u32) {
        self.bits_3 = (self.bits_3 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn REFLECT(&self) -> u32 {
        ((self.bits_4 as u32 >> 0) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_REFLECT(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn SHOCK_WAVE(&self) -> u32 {
        ((self.bits_4 as u32 >> 1) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_SHOCK_WAVE(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn FLAMETHROWER(&self) -> u32 {
        ((self.bits_4 as u32 >> 2) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_FLAMETHROWER(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn SLUDGE_BOMB(&self) -> u32 {
        ((self.bits_4 as u32 >> 3) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_SLUDGE_BOMB(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn SANDSTORM(&self) -> u32 {
        ((self.bits_4 as u32 >> 4) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_SANDSTORM(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn FIRE_BLAST(&self) -> u32 {
        ((self.bits_4 as u32 >> 5) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_FIRE_BLAST(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn ROCK_TOMB(&self) -> u32 {
        ((self.bits_4 as u32 >> 6) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_ROCK_TOMB(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn AERIAL_ACE(&self) -> u32 {
        ((self.bits_4 as u32 >> 7) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_AERIAL_ACE(&mut self, v: u32) {
        self.bits_4 = (self.bits_4 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn TORMENT(&self) -> u32 {
        ((self.bits_5 as u32 >> 0) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_TORMENT(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn FACADE(&self) -> u32 {
        ((self.bits_5 as u32 >> 1) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_FACADE(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn SECRET_POWER(&self) -> u32 {
        ((self.bits_5 as u32 >> 2) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_SECRET_POWER(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn REST(&self) -> u32 {
        ((self.bits_5 as u32 >> 3) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_REST(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn ATTRACT(&self) -> u32 {
        ((self.bits_5 as u32 >> 4) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_ATTRACT(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn THIEF(&self) -> u32 {
        ((self.bits_5 as u32 >> 5) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_THIEF(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn STEEL_WING(&self) -> u32 {
        ((self.bits_5 as u32 >> 6) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_STEEL_WING(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn SKILL_SWAP(&self) -> u32 {
        ((self.bits_5 as u32 >> 7) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_SKILL_SWAP(&mut self, v: u32) {
        self.bits_5 = (self.bits_5 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn SNATCH(&self) -> u32 {
        ((self.bits_6 as u32 >> 0) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_SNATCH(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn OVERHEAT(&self) -> u32 {
        ((self.bits_6 as u32 >> 1) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_OVERHEAT(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn CUT(&self) -> u32 {
        ((self.bits_6 as u32 >> 2) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_CUT(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn FLY(&self) -> u32 {
        ((self.bits_6 as u32 >> 3) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_FLY(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
    #[inline(always)]
    pub fn SURF(&self) -> u32 {
        ((self.bits_6 as u32 >> 4) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_SURF(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 4)) | ((v as u8 & 0x1) << 4);
    }
    #[inline(always)]
    pub fn STRENGTH(&self) -> u32 {
        ((self.bits_6 as u32 >> 5) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_STRENGTH(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 5)) | ((v as u8 & 0x1) << 5);
    }
    #[inline(always)]
    pub fn FLASH(&self) -> u32 {
        ((self.bits_6 as u32 >> 6) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_FLASH(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 6)) | ((v as u8 & 0x1) << 6);
    }
    #[inline(always)]
    pub fn ROCK_SMASH(&self) -> u32 {
        ((self.bits_6 as u32 >> 7) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_ROCK_SMASH(&mut self, v: u32) {
        self.bits_6 = (self.bits_6 & !(0x1 << 7)) | ((v as u8 & 0x1) << 7);
    }
    #[inline(always)]
    pub fn WATERFALL(&self) -> u32 {
        ((self.bits_7 as u32 >> 0) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_WATERFALL(&mut self, v: u32) {
        self.bits_7 = (self.bits_7 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn DIVE(&self) -> u32 {
        ((self.bits_7 as u32 >> 1) & 0x1) as u32
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
#[unsafe(no_mangle)]
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

unsafe extern "C" {
    static mut gAbsentBattlerFlags: u8;
    static mut gActiveBattler: u8;
    static gAnims_MonPic: CArray<*mut AnimCmd, 0>;
    static gApprentices: CArray<ApprenticeTrainer, 0>;
    static mut gBattleMonForms: CArray<u8, 4>;
    static mut gBattleMons: CArray<BattlePokemon, 4>;
    static mut gBattleMoveDamage: i32;
    static mut gBattleMovePower: u16;
    static mut gBattleResources: *mut BattleResources;
    static mut gBattleResults: BattleResults;
    static mut gBattleScripting: BattleScripting;
    static mut gBattleStruct: *mut BattleStruct;
    static mut gBattleTextBuff1: CArray<u8, 16>;
    static mut gBattleTextBuff2: CArray<u8, 16>;
    static mut gBattleTypeFlags: u32;
    static mut gBattleWeather: u16;
    static mut gBattlerAttacker: u8;
    static mut gBattlerInMenuId: u8;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gBattlerTarget: u8;
    static mut gBattlersCount: u8;
    static gBitTable: CArray<u32, 0>;
    static mut gCritMultiplier: u8;
    static mut gCurrentMove: u16;
    static mut gDisableStructs: CArray<DisableStruct, 4>;
    static mut gDisplayedStringBattle: CArray<u8, 300>;
    static mut gEnigmaBerries: CArray<BattleEnigmaBerry, 4>;
    static gGameLanguage: u8;
    static gGameVersion: u8;
    static mut gHitMarker: u32;
    static mut gLastUsedAbility: u8;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gLocalTime: Time;
    static mut gMain: Main;
    static mut gMapHeader: MapHeader;
    static gMonFrontAnimsPtrTable: CArray<*mut *mut AnimCmd, 0>;
    static gMonPaletteTable: CArray<CompressedSpritePalette, 0>;
    static gMonShinyPaletteTable: CArray<CompressedSpritePalette, 0>;
    static mut gMonSpritesGfxPtr: *mut MonSpritesGfx;
    static mut gMoveToLearn: u16;
    static mut gPartnerTrainerId: u16;
    static gPokeblockFlavorCompatibilityTable: CArray<i8, 125>;
    static mut gPotentialItemEffectBattler: u8;
    static mut gRecordedBattleMultiplayerId: u8;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSideTimers: CArray<SideTimer, 2>;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_MonBoxId: u16;
    static mut gSpecialVar_MonBoxPos: u16;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static gStatNamesTable: CArray<*mut u8, 0>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_BadEgg: CArray<u8, 0>;
    static gText_BattleWallyName: CArray<u8, 0>;
    static gText_DefendersStatRose: CArray<u8, 0>;
    static gText_EggNickname: CArray<u8, 0>;
    static gText_PkmnGettingPumped: CArray<u8, 0>;
    static gText_PkmnShroudedInMist: CArray<u8, 0>;
    static gText_PkmnsXPreventsSwitching: CArray<u8, 0>;
    static gText_StatRose: CArray<u8, 0>;
    static gTrainerBackAnimsPtrTable: CArray<*mut *mut AnimCmd, 0>;
    static mut gTrainerBattleOpponent_A: u16;
    static gTrainerClassNames: CArray<CArray<u8, 13>, 0>;
    static gTrainerFrontAnimsPtrTable: CArray<*mut *mut AnimCmd, 0>;
    static gTrainers: CArray<Trainer, 0>;
    fn AbilityBattleEffects(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn BattleStringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> u32;
    fn BattleStringExpandPlaceholdersToDisplayedString(a0: *mut u8) -> u32;
    fn BeginEvolutionScene(a0: *mut Pokemon, a1: u16, a2: u8, a3: u8);
    fn BtlController_EmitGetMonData(a0: u8, a1: u8, a2: u8);
    fn ClearTemporarySpeciesSpriteData(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn DestroyTask(a0: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn GetApprenticeNameInLanguage(a0: u32, a1: i32) -> *mut u8;
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBoxMonDataAt(a0: u8, a1: u8, a2: i32) -> u32;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut BoxPokemon;
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
    fn LaunchAnimationTaskForBackSprite(a0: *mut Sprite, a1: u8);
    fn LaunchAnimationTaskForFrontSprite(a0: *mut Sprite, a1: u8);
    fn MarkBattlerForControllerExec(a0: u8);
    fn PlayBGM(a0: u16);
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayNewMapMusic(a0: u16);
    fn Random() -> u16;
    fn ResetMapMusic();
    fn RtcCalcLocalTime();
    fn SetPCBoxToSendMon(a0: u8);
    fn SetSpriteCB_MonAnimDummy(a0: *mut Sprite);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn SpriteCallbackDummy_2(a0: *mut Sprite);
    fn StartMonSummaryAnimation(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
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
pub unsafe extern "C" fn ZeroBoxMonData(boxMon: *mut BoxPokemon) {
    let mut raw: *mut u8 = boxMon as *mut u8;
    let mut i: u32 = 0;
    i = 0;
    while i < 80 {
        *raw.at(i) = 0;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ZeroMonData(mon: *mut Pokemon) {
    let mut arg: u32 = 0;
    ZeroBoxMonData(&raw mut (*mon).r#box);
    arg = 0;
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
pub unsafe extern "C" fn ZeroPlayerPartyMons() {
    let mut i: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
        ZeroMonData(&raw mut gPlayerParty[i]);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ZeroEnemyPartyMons() {
    let mut i: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
        ZeroMonData(&raw mut gEnemyParty[i]);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMon(
    mon: *mut Pokemon,
    species: u16,
    mut level: u8,
    fixedIV: u8,
    hasFixedPersonality: u8,
    fixedPersonality: u32,
    otIdType: u8,
    fixedOtId: u32,
) {
    let mut mail: u32 = 0;
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
    mail = MAIL_NONE;
    SetMonData(mon, MON_DATA_MAIL, &raw mut mail as *mut c_void);
    CalculateMonStats(mon);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBoxMon(
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
    let mut checksum: u16 = 0;
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
    checksum = CalculateBoxMonChecksum(boxMon);
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
        let mut iv: u32 = 0;
        value = Random() as u32;
        iv = value & MAX_IV_MASK;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonWithNature(
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonWithGenderNatureLetter(
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
                | (personality & 0x00000003) >> 0)
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMaleMon(mon: *mut Pokemon, species: u16, level: u8) {
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
pub unsafe extern "C" fn CreateMonWithIVsPersonality(
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonWithIVsOTID(
    mon: *mut Pokemon,
    species: u16,
    level: u8,
    mut ivs: *mut u8,
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonWithEVSpread(
    mon: *mut Pokemon,
    species: u16,
    level: u8,
    fixedIV: u8,
    evSpread: u8,
) {
    let mut i: i32 = 0;
    let mut statCount: i32 = 0;
    let mut evAmount: u16 = 0;
    let mut evsBits: u8 = 0;
    CreateMon(mon, species, level, fixedIV, 0, 0, 0, 0);
    evsBits = evSpread;
    i = 0;
    while i < NUM_STATS {
        if evsBits as i32 & 1 != 0 {
            statCount += 1;
        }
        evsBits >>= 1;
        i += 1;
    }
    evAmount = div_i32(MAX_TOTAL_EVS, statCount) as u16;
    evsBits = 1;
    i = 0;
    while i < NUM_STATS {
        if evSpread as i32 & evsBits as i32 != 0 {
            SetMonData(mon, MON_DATA_HP_EV + i, &raw mut evAmount as *mut c_void);
        }
        evsBits <<= 1;
        i += 1;
    }
    CalculateMonStats(mon);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBattleTowerMon(mon: *mut Pokemon, src: *mut BattleTowerPokemon) {
    let mut i: i32 = 0;
    let mut nickname: CArray<u8, 32> = zeroed();
    let mut language: u8 = 0;
    let mut value: u8 = 0;
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
    i = 0;
    while i < MAX_MON_MOVES {
        SetMonMoveSlot(mon, (*src).moves[i], i as u8);
        i += 1;
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
    value = (*src).abilityNum() as u8;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBattleTowerMon_HandleLevel(
    mon: *mut Pokemon,
    src: *mut BattleTowerPokemon,
    lvl50: u8,
) {
    let mut i: i32 = 0;
    let mut nickname: CArray<u8, 32> = zeroed();
    let mut level: u8 = 0;
    let mut language: u8 = 0;
    let mut value: u8 = 0;
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
    i = 0;
    while i < MAX_MON_MOVES {
        SetMonMoveSlot(mon, (*src).moves[i], i as u8);
        i += 1;
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
    value = (*src).abilityNum() as u8;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateApprenticeMon(mon: *mut Pokemon, src: *mut Apprentice, monId: u8) {
    let mut i: i32 = 0;
    let mut evAmount: u16 = 0;
    let mut language: u8 = 0;
    let mut otId: u32 = gApprentices[(*src).id()].otId as u32;
    let mut personality: u32 = ((gApprentices[(*src).id()].otId >> 8) as u32
        | (gApprentices[(*src).id()].otId as u32 & 0xFF) << 8)
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
    i = 0;
    while i < MAX_MON_MOVES {
        SetMonMoveSlot(mon, (*src).party[monId].moves[i], i as u8);
        i += 1;
    }
    evAmount = 85;
    i = 0;
    while i < NUM_STATS {
        SetMonData(mon, MON_DATA_HP_EV + i, &raw mut evAmount as *mut c_void);
        i += 1;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonWithEVSpreadNatureOTID(
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
    let mut evsBits: u8 = 0;
    let mut evAmount: u16 = 0;
    loop {
        i = Random() as i32 | (Random() as i32) << 16;
        if nature == GetNatureFromPersonality(i as u32) {
            break;
        }
    }
    CreateMon(mon, species, level, fixedIV, 1, i as u32, 1, otId);
    evsBits = evSpread;
    i = 0;
    while i < NUM_STATS {
        if evsBits as i32 & 1 != 0 {
            statCount += 1;
        }
        evsBits >>= 1;
        i += 1;
    }
    evAmount = div_i32(MAX_TOTAL_EVS, statCount) as u16;
    evsBits = 1;
    i = 0;
    while i < NUM_STATS {
        if evSpread as i32 & evsBits as i32 != 0 {
            SetMonData(mon, MON_DATA_HP_EV + i, &raw mut evAmount as *mut c_void);
        }
        evsBits <<= 1;
        i += 1;
    }
    CalculateMonStats(mon);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertPokemonToBattleTowerPokemon(
    mon: *mut Pokemon,
    dest: *mut BattleTowerPokemon,
) {
    let mut i: i32 = 0;
    let mut heldItem: u16 = 0;
    (*dest).species = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    heldItem = GetMonData3(mon, MON_DATA_HELD_ITEM, null_mut()) as u16;
    if heldItem == ITEM_ENIGMA_BERRY {
        heldItem = ITEM_NONE;
    }
    (*dest).heldItem = heldItem;
    i = 0;
    while i < MAX_MON_MOVES {
        (*dest).moves[i] = GetMonData3(mon, MON_DATA_MOVE1 + i, null_mut()) as u16;
        i += 1;
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
pub(crate) unsafe extern "C" fn CreateEventMon(
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldIgnoreDeoxysForm(caseId: u8, battler: u8) -> u8 {
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
    return TRUE;
}
pub(crate) unsafe extern "C" fn GetDeoxysStat(mon: *mut Pokemon, statId: i32) -> u16 {
    let mut ivVal: i32 = 0;
    let mut evVal: i32 = 0;
    let mut statValue: u16 = 0;
    let mut nature: u8 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_LINK_IN_BATTLE != 0
        || GetMonData3(mon, MON_DATA_SPECIES, null_mut()) != SPECIES_DEOXYS
    {
        return 0;
    }
    ivVal = GetMonData3(mon, MON_DATA_HP_IV + statId, null_mut()) as i32;
    evVal = GetMonData3(mon, MON_DATA_HP_EV + statId, null_mut()) as i32;
    statValue = ((sDeoxysBaseStats[statId] as i32 * 2 + ivVal + evVal / 4) * (*mon).level as i32
        / 100) as u16
        + 5;
    nature = GetNature(mon);
    statValue = ModifyStatByNature(nature, statValue, statId as u8);
    return statValue;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDeoxysStats() {
    let mut i: i32 = 0;
    let mut value: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
        'l1: {
            let mut mon: *mut Pokemon = &raw mut gPlayerParty[i];
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
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUnionRoomTrainerPic() -> u16 {
    let mut linkId: u8 = 0;
    let mut arrId: u32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
        linkId = gRecordedBattleMultiplayerId ^ 1;
    } else {
        linkId = GetMultiplayerId() ^ 1;
    }
    arrId = gLinkPlayers[linkId].trainerId % 8;
    arrId |= gLinkPlayers[linkId].gender as u32 * NUM_UNION_ROOM_CLASSES;
    return FacilityClassToPicIndex(gUnionRoomFacilityClasses[arrId]);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUnionRoomTrainerClass() -> u16 {
    let mut linkId: u8 = 0;
    let mut arrId: u32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
        linkId = gRecordedBattleMultiplayerId ^ 1;
    } else {
        linkId = GetMultiplayerId() ^ 1;
    }
    arrId = gLinkPlayers[linkId].trainerId % 8;
    arrId |= gLinkPlayers[linkId].gender as u32 * NUM_UNION_ROOM_CLASSES;
    return gFacilityClassToTrainerClass[gUnionRoomFacilityClasses[arrId]] as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateEnemyEventMon() {
    let mut species: i32 = gSpecialVar_0x8004 as i32;
    let mut level: i32 = gSpecialVar_0x8005 as i32;
    let mut itemId: i32 = gSpecialVar_0x8006 as i32;
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
pub(crate) unsafe extern "C" fn CalculateBoxMonChecksum(boxMon: *mut BoxPokemon) -> u16 {
    let mut checksum: u16 = 0;
    let mut substruct0: *mut PokemonSubstruct = GetSubstruct(boxMon, (*boxMon).personality, 0);
    let mut substruct1: *mut PokemonSubstruct = GetSubstruct(boxMon, (*boxMon).personality, 1);
    let mut substruct2: *mut PokemonSubstruct = GetSubstruct(boxMon, (*boxMon).personality, 2);
    let mut substruct3: *mut PokemonSubstruct = GetSubstruct(boxMon, (*boxMon).personality, 3);
    let mut i: i32 = 0;
    i = 0;
    while i < 6 {
        checksum += (*substruct0).raw[i];
        i += 1;
    }
    i = 0;
    while i < 6 {
        checksum += (*substruct1).raw[i];
        i += 1;
    }
    i = 0;
    while i < 6 {
        checksum += (*substruct2).raw[i];
        i += 1;
    }
    i = 0;
    while i < 6 {
        checksum += (*substruct3).raw[i];
        i += 1;
    }
    return checksum;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculateMonStats(mon: *mut Pokemon) {
    let mut oldMaxHP: i32 = GetMonData3(mon, MON_DATA_MAX_HP, null_mut()) as i32;
    let mut currentHP: i32 = GetMonData3(mon, MON_DATA_HP, null_mut()) as i32;
    let mut hpIV: i32 = GetMonData3(mon, MON_DATA_HP_IV, null_mut()) as i32;
    let mut hpEV: i32 = GetMonData3(mon, MON_DATA_HP_EV, null_mut()) as i32;
    let mut attackIV: i32 = GetMonData3(mon, MON_DATA_ATK_IV, null_mut()) as i32;
    let mut attackEV: i32 = GetMonData3(mon, MON_DATA_ATK_EV, null_mut()) as i32;
    let mut defenseIV: i32 = GetMonData3(mon, MON_DATA_DEF_IV, null_mut()) as i32;
    let mut defenseEV: i32 = GetMonData3(mon, MON_DATA_DEF_EV, null_mut()) as i32;
    let mut speedIV: i32 = GetMonData3(mon, MON_DATA_SPEED_IV, null_mut()) as i32;
    let mut speedEV: i32 = GetMonData3(mon, MON_DATA_SPEED_EV, null_mut()) as i32;
    let mut spAttackIV: i32 = GetMonData3(mon, MON_DATA_SPATK_IV, null_mut()) as i32;
    let mut spAttackEV: i32 = GetMonData3(mon, MON_DATA_SPATK_EV, null_mut()) as i32;
    let mut spDefenseIV: i32 = GetMonData3(mon, MON_DATA_SPDEF_IV, null_mut()) as i32;
    let mut spDefenseEV: i32 = GetMonData3(mon, MON_DATA_SPDEF_EV, null_mut()) as i32;
    let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut level: i32 = GetLevelFromMonExp(mon) as i32;
    let mut newMaxHP: i32 = 0;
    SetMonData(mon, MON_DATA_LEVEL, &raw mut level as *mut c_void);
    if species == SPECIES_SHEDINJA {
        newMaxHP = 1;
    } else {
        let mut n: i32 = 2 * gSpeciesInfo[species].baseHP as i32 + hpIV;
        newMaxHP = (n + hpEV / 4) * level / 100 + level + 10;
    }
    gBattleScripting.levelUpHP = newMaxHP as u8 - oldMaxHP as u8;
    if gBattleScripting.levelUpHP == 0 {
        gBattleScripting.levelUpHP = 1;
    }
    SetMonData(mon, MON_DATA_MAX_HP, &raw mut newMaxHP as *mut c_void);
    {
        let mut baseStat: u8 = gSpeciesInfo[species].baseAttack;
        let mut n: i32 = (2 * baseStat as i32 + attackIV + attackEV / 4) * level / 100 + 5;
        let mut nature: u8 = GetNature(mon);
        n = ModifyStatByNature(nature, n as u16, STAT_ATK) as i32;
        SetMonData(mon, MON_DATA_ATK, &raw mut n as *mut c_void);
    }
    {
        let mut baseStat: u8 = gSpeciesInfo[species].baseDefense;
        let mut n: i32 = (STAT_DEF * baseStat as i32 + defenseIV + defenseEV / 4) * level / 100 + 5;
        let mut nature: u8 = GetNature(mon);
        n = ModifyStatByNature(nature, n as u16, STAT_DEF as u8) as i32;
        SetMonData(mon, MON_DATA_DEF, &raw mut n as *mut c_void);
    }
    {
        let mut baseStat: u8 = gSpeciesInfo[species].baseSpeed;
        let mut n: i32 = (2 * baseStat as i32 + speedIV + speedEV / 4) * level / 100 + 5;
        let mut nature: u8 = GetNature(mon);
        n = ModifyStatByNature(nature, n as u16, STAT_SPEED) as i32;
        SetMonData(mon, MON_DATA_SPEED, &raw mut n as *mut c_void);
    }
    {
        let mut baseStat: u8 = gSpeciesInfo[species].baseSpAttack;
        let mut n: i32 = (2 * baseStat as i32 + spAttackIV + spAttackEV / 4) * level / 100 + 5;
        let mut nature: u8 = GetNature(mon);
        n = ModifyStatByNature(nature, n as u16, STAT_SPATK) as i32;
        SetMonData(mon, MON_DATA_SPATK, &raw mut n as *mut c_void);
    }
    {
        let mut baseStat: u8 = gSpeciesInfo[species].baseSpDefense;
        let mut n: i32 =
            (2 * baseStat as i32 + spDefenseIV + spDefenseEV / 4) * level / 100 + STAT_SPDEF;
        let mut nature: u8 = GetNature(mon);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BoxMonToMon(src: *mut BoxPokemon, dest: *mut Pokemon) {
    let mut value: u32 = 0;
    (*dest).r#box = *src;
    SetMonData(dest, MON_DATA_STATUS, &raw mut value as *mut c_void);
    SetMonData(dest, MON_DATA_HP, &raw mut value as *mut c_void);
    SetMonData(dest, MON_DATA_MAX_HP, &raw mut value as *mut c_void);
    value = MAIL_NONE;
    SetMonData(dest, MON_DATA_MAIL, &raw mut value as *mut c_void);
    CalculateMonStats(dest);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLevelFromMonExp(mon: *mut Pokemon) -> u8 {
    let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut exp: u32 = GetMonData3(mon, MON_DATA_EXP, null_mut());
    let mut level: i32 = 1;
    while level <= MAX_LEVEL as i32
        && gExperienceTables[gSpeciesInfo[species].growthRate][level] <= exp
    {
        level += 1;
    }
    return level as u8 - 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLevelFromBoxMonExp(boxMon: *mut BoxPokemon) -> u8 {
    let mut species: u16 = GetBoxMonData3(boxMon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut exp: u32 = GetBoxMonData3(boxMon, MON_DATA_EXP, null_mut());
    let mut level: i32 = 1;
    while level <= MAX_LEVEL as i32
        && gExperienceTables[gSpeciesInfo[species].growthRate][level] <= exp
    {
        level += 1;
    }
    return level as u8 - 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveMoveToMon(mon: *mut Pokemon, r#move: u16) -> u16 {
    return GiveMoveToBoxMon(&raw mut (*mon).r#box, r#move);
}
pub(crate) unsafe extern "C" fn GiveMoveToBoxMon(boxMon: *mut BoxPokemon, mut r#move: u16) -> u16 {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        let mut existingMove: u16 = GetBoxMonData3(boxMon, MON_DATA_MOVE1 + i, null_mut()) as u16;
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
        i += 1;
    }
    return MON_HAS_MAX_MOVES;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveMoveToBattleMon(mon: *mut BattlePokemon, r#move: u16) -> u16 {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        if (*mon).moves[i] == MOVE_NONE {
            (*mon).moves[i] = r#move;
            (*mon).pp[i] = gBattleMoves[r#move].pp;
            return r#move;
        }
        i += 1;
    }
    return MON_HAS_MAX_MOVES;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMonMoveSlot(mon: *mut Pokemon, mut r#move: u16, slot: u8) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattleMonMoveSlot(mon: *mut BattlePokemon, r#move: u16, slot: u8) {
    (*mon).moves[slot] = r#move;
    (*mon).pp[slot] = gBattleMoves[r#move].pp;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveMonInitialMoveset(mon: *mut Pokemon) {
    GiveBoxMonInitialMoveset(&raw mut (*mon).r#box);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveBoxMonInitialMoveset(boxMon: *mut BoxPokemon) {
    let mut species: u16 = GetBoxMonData3(boxMon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut level: i32 = GetLevelFromBoxMonExp(boxMon) as i32;
    let mut i: i32 = 0;
    i = 0;
    while *gLevelUpLearnsets[species].at(i) != LEVEL_UP_END {
        let mut moveLevel: u16 = 0;
        let mut r#move: u16 = 0;
        moveLevel = *gLevelUpLearnsets[species].at(i) & LEVEL_UP_MOVE_LV as u16;
        if moveLevel as i32 > level << 9 {
            break;
        }
        r#move = *gLevelUpLearnsets[species].at(i) & LEVEL_UP_MOVE_ID;
        if GiveMoveToBoxMon(boxMon, r#move) == MON_HAS_MAX_MOVES {
            DeleteFirstMoveAndGiveMoveToBoxMon(boxMon, r#move);
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonTryLearningNewMove(mon: *mut Pokemon, firstMove: u8) -> u16 {
    let mut retVal: u32 = MOVE_NONE as u32;
    let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut level: u8 = GetMonData3(mon, MON_DATA_LEVEL, null_mut()) as u8;
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
    return retVal as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DeleteFirstMoveAndGiveMoveToMon(mon: *mut Pokemon, r#move: u16) {
    let mut i: i32 = 0;
    let mut moves: CArray<u16, 4> = zeroed();
    let mut pp: CArray<u8, 4> = zeroed();
    let mut ppBonuses: u8 = 0;
    i = 0;
    while i < 3 {
        moves[i] = GetMonData3(mon, MON_DATA_MOVE2 + i, null_mut()) as u16;
        pp[i] = GetMonData3(mon, MON_DATA_PP2 + i, null_mut()) as u8;
        i += 1;
    }
    ppBonuses = GetMonData3(mon, MON_DATA_PP_BONUSES, null_mut()) as u8;
    ppBonuses >>= 2;
    moves[3] = r#move;
    pp[3] = gBattleMoves[r#move].pp;
    i = 0;
    while i < MAX_MON_MOVES {
        SetMonData(mon, MON_DATA_MOVE1 + i, &raw mut moves[i] as *mut c_void);
        SetMonData(mon, MON_DATA_PP1 + i, &raw mut pp[i] as *mut c_void);
        i += 1;
    }
    SetMonData(mon, MON_DATA_PP_BONUSES, &raw mut ppBonuses as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DeleteFirstMoveAndGiveMoveToBoxMon(boxMon: *mut BoxPokemon, r#move: u16) {
    let mut i: i32 = 0;
    let mut moves: CArray<u16, 4> = zeroed();
    let mut pp: CArray<u8, 4> = zeroed();
    let mut ppBonuses: u8 = 0;
    i = 0;
    while i < 3 {
        moves[i] = GetBoxMonData3(boxMon, MON_DATA_MOVE2 + i, null_mut()) as u16;
        pp[i] = GetBoxMonData3(boxMon, MON_DATA_PP2 + i, null_mut()) as u8;
        i += 1;
    }
    ppBonuses = GetBoxMonData3(boxMon, MON_DATA_PP_BONUSES, null_mut()) as u8;
    ppBonuses >>= 2;
    moves[3] = r#move;
    pp[3] = gBattleMoves[r#move].pp;
    i = 0;
    while i < MAX_MON_MOVES {
        SetBoxMonData(boxMon, MON_DATA_MOVE1 + i, &raw mut moves[i] as *mut c_void);
        SetBoxMonData(boxMon, MON_DATA_PP1 + i, &raw mut pp[i] as *mut c_void);
        i += 1;
    }
    SetBoxMonData(
        boxMon,
        MON_DATA_PP_BONUSES,
        &raw mut ppBonuses as *mut c_void,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculateBaseDamage(
    attacker: *mut BattlePokemon,
    defender: *mut BattlePokemon,
    r#move: u32,
    sideStatus: u16,
    powerOverride: u16,
    typeOverride: u8,
    battlerIdAtk: u8,
    battlerIdDef: u8,
) -> i32 {
    let mut i: u32 = 0;
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
    i = 0;
    while i < 17 {
        if attackerHoldEffect == sHoldEffectToType[i][0] && r#type == sHoldEffectToType[i][1] {
            if r#type < 9 {
                attack = (attack as i32 * (attackerHoldEffectParam as i32 + 100) / 100) as u16;
            } else {
                spAttack = (spAttack as i32 * (attackerHoldEffectParam as i32 + 100) / 100) as u16;
            }
            break;
        }
        i += 1;
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
        damage = damage * gBattleMovePower as i32;
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
        damage = damage / 50;
        if (*attacker).status1 & STATUS1_BURN != 0 && (*attacker).ability != ABILITY_GUTS {
            damage = damage / 2;
        }
        if sideStatus as i32 & 1 != 0 && gCritMultiplier == 1 {
            if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 && CountAliveMonsInBattle(2) == 2 {
                damage = 2 * (damage / 3);
            } else {
                damage = damage / 2;
            }
        }
        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
            && gBattleMoves[r#move].target == MOVE_TARGET_BOTH
            && CountAliveMonsInBattle(2) == 2
        {
            damage = damage / 2;
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
        damage = damage * gBattleMovePower as i32;
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
        damage = damage / 50;
        if sideStatus as i32 & SIDE_STATUS_LIGHTSCREEN != 0 && gCritMultiplier == 1 {
            if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 && CountAliveMonsInBattle(2) == 2 {
                damage = 2 * (damage / 3);
            } else {
                damage = damage / 2;
            }
        }
        if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
            && gBattleMoves[r#move].target == MOVE_TARGET_BOTH
            && CountAliveMonsInBattle(2) == 2
        {
            damage = damage / 2;
        }
        if AbilityBattleEffects(14, 0, 13, 0, 0) == 0 && AbilityBattleEffects(14, 0, 77, 0, 0) == 0
        {
            if gBattleWeather as i32 & B_WEATHER_RAIN_TEMPORARY as i32 != 0 {
                match r#type {
                    TYPE_FIRE => {
                        damage = damage / 2;
                    }
                    TYPE_WATER => {
                        damage = 15 * damage / 10;
                    }
                    _ => {}
                }
            }
            if gBattleWeather as i32 & 159 != 0 && gCurrentMove == MOVE_SOLAR_BEAM {
                damage = damage / 2;
            }
            if gBattleWeather as i32 & B_WEATHER_SUN != 0 {
                match r#type {
                    TYPE_FIRE => {
                        damage = 15 * damage / 10;
                    }
                    TYPE_WATER => {
                        damage = damage / 2;
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
    return damage + 2;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountAliveMonsInBattle(caseId: u8) -> u8 {
    let mut i: i32 = 0;
    let mut retVal: u8 = 0;
    match caseId {
        BATTLE_ALIVE_EXCEPT_ACTIVE => {
            i = 0;
            while i < MAX_BATTLERS_COUNT as i32 {
                if i != gActiveBattler as i32 && gAbsentBattlerFlags as u32 & gBitTable[i] == 0 {
                    retVal += 1;
                }
                i += 1;
            }
        }
        BATTLE_ALIVE_ATK_SIDE => {
            i = 0;
            while i < MAX_BATTLERS_COUNT as i32 {
                if GetBattlerSide(i as u8) == GetBattlerSide(gBattlerAttacker)
                    && gAbsentBattlerFlags as u32 & gBitTable[i] == 0
                {
                    retVal += 1;
                }
                i += 1;
            }
        }
        BATTLE_ALIVE_DEF_SIDE => {
            i = 0;
            while i < MAX_BATTLERS_COUNT as i32 {
                if GetBattlerSide(i as u8) == GetBattlerSide(gBattlerTarget)
                    && gAbsentBattlerFlags as u32 & gBitTable[i] == 0
                {
                    retVal += 1;
                }
                i += 1;
            }
        }
        _ => {}
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn ShouldGetStatBadgeBoost(badgeFlag: u16, battler: u8) -> u8 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDefaultMoveTarget(battler: u8) -> u8 {
    let mut opposing: u8 = GetBattlerPosition(battler) & 1 ^ 1;
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
        if gAbsentBattlerFlags as u32 & gBitTable[opposing] != 0 {
            return GetBattlerAtPosition(opposing ^ 2);
        } else {
            return GetBattlerAtPosition(opposing);
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonGender(mon: *mut Pokemon) -> u8 {
    return GetBoxMonGender(&raw mut (*mon).r#box);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonGender(boxMon: *mut BoxPokemon) -> u8 {
    let mut species: u16 = GetBoxMonData3(boxMon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut personality: u32 = GetBoxMonData3(boxMon, MON_DATA_PERSONALITY, null_mut());
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGenderFromSpeciesAndPersonality(species: u16, personality: u32) -> u8 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMultiuseSpriteTemplateToPokemon(speciesTag: u16, battlerPosition: u8) {
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
        gMultiuseSpriteTemplate.anims = gAnims_MonPic.as_ptr().cast_mut();
    } else if speciesTag > SPECIES_SHINY_TAG {
        gMultiuseSpriteTemplate.anims =
            gMonFrontAnimsPtrTable[speciesTag as i32 - SPECIES_SHINY_TAG as i32];
    } else {
        gMultiuseSpriteTemplate.anims = gMonFrontAnimsPtrTable[speciesTag];
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMultiuseSpriteTemplateToTrainerBack(
    trainerPicId: u16,
    battlerPosition: u8,
) {
    gMultiuseSpriteTemplate.paletteTag = trainerPicId;
    if battlerPosition == B_POSITION_PLAYER_LEFT || battlerPosition == B_POSITION_PLAYER_RIGHT {
        gMultiuseSpriteTemplate = sTrainerBackSpriteTemplates[trainerPicId];
        gMultiuseSpriteTemplate.anims = gTrainerBackAnimsPtrTable[trainerPicId];
    } else {
        if !gMonSpritesGfxPtr.is_null() {
            gMultiuseSpriteTemplate = (*gMonSpritesGfxPtr).templates[battlerPosition];
        } else {
            gMultiuseSpriteTemplate = gBattlerSpriteTemplates[battlerPosition];
        }
        gMultiuseSpriteTemplate.anims = gTrainerFrontAnimsPtrTable[trainerPicId];
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMultiuseSpriteTemplateToTrainerFront(
    trainerPicId: u16,
    battlerPosition: u8,
) {
    if !gMonSpritesGfxPtr.is_null() {
        gMultiuseSpriteTemplate = (*gMonSpritesGfxPtr).templates[battlerPosition];
    } else {
        gMultiuseSpriteTemplate = gBattlerSpriteTemplates[battlerPosition];
    }
    gMultiuseSpriteTemplate.paletteTag = trainerPicId;
    gMultiuseSpriteTemplate.anims = gTrainerFrontAnimsPtrTable[trainerPicId];
}
pub(crate) unsafe extern "C" fn EncryptBoxMon(boxMon: *mut BoxPokemon) {
    let mut i: u32 = 0;
    i = 0;
    while i < 12 {
        (*boxMon).secure.raw[i] ^= (*boxMon).personality;
        (*boxMon).secure.raw[i] ^= (*boxMon).otId;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DecryptBoxMon(boxMon: *mut BoxPokemon) {
    let mut i: u32 = 0;
    i = 0;
    while i < 12 {
        (*boxMon).secure.raw[i] ^= (*boxMon).otId;
        (*boxMon).secure.raw[i] ^= (*boxMon).personality;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetSubstruct(
    boxMon: *mut BoxPokemon,
    personality: u32,
    substructType: u8,
) -> *mut PokemonSubstruct {
    let mut substruct: *mut PokemonSubstruct = null_mut();
    'l1: {
        match personality % 24 {
            0 => {
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
                let mut substructs0: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs1: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs2: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs3: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs4: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs5: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs6: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs7: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs8: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs9: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs10: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs11: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs12: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs13: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs14: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs15: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs16: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs17: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs18: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs19: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs20: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs21: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs22: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
                let mut substructs23: *mut PokemonSubstruct =
                    (*boxMon).secure.substructs.as_mut_ptr();
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
    return substruct;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonData3(mon: *mut Pokemon, field: i32, data: *mut u8) -> u32 {
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
    return ret;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonData2(mon: *mut Pokemon, field: i32) -> u32 {
    return GetMonData3(mon, field, null_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonData3(
    boxMon: *mut BoxPokemon,
    field: i32,
    mut data: *mut u8,
) -> u32 {
    let mut i: i32 = 0;
    let mut retVal: u32 = 0;
    let mut substruct0: *mut PokemonSubstruct0 = null_mut();
    let mut substruct1: *mut PokemonSubstruct1 = null_mut();
    let mut substruct2: *mut PokemonSubstruct2 = null_mut();
    let mut substruct3: *mut PokemonSubstruct3 = null_mut();
    if field > MON_DATA_ENCRYPT_SEPARATOR {
        substruct0 = &raw mut (*GetSubstruct(boxMon, (*boxMon).personality, 0)).type0;
        substruct1 = &raw mut (*GetSubstruct(boxMon, (*boxMon).personality, 1)).type1;
        substruct2 = &raw mut (*GetSubstruct(boxMon, (*boxMon).personality, 2)).type2;
        substruct3 = &raw mut (*GetSubstruct(boxMon, (*boxMon).personality, 3)).type3;
        DecryptBoxMon(boxMon);
        if CalculateBoxMonChecksum(boxMon) != (*boxMon).checksum {
            (*boxMon).set_isBadEgg(TRUE);
            (*boxMon).set_isEgg(TRUE);
            (*substruct3).set_isEgg(TRUE as u32);
        }
    }
    'l1: {
        match field {
            MON_DATA_PERSONALITY => {
                retVal = (*boxMon).personality;
            }
            MON_DATA_OT_ID => {
                retVal = (*boxMon).otId;
            }
            MON_DATA_NICKNAME => {
                if (*boxMon).isBadEgg() != 0 {
                    retVal = 0;
                    while retVal < POKEMON_NAME_LENGTH && gText_BadEgg[retVal] != EOS {
                        *data.at(retVal) = gText_BadEgg[retVal];
                        retVal += 1;
                    }
                    *data.at(retVal) = EOS;
                } else if (*boxMon).isEgg() != 0 {
                    StringCopy(data, gText_EggNickname.as_ptr().cast_mut());
                    retVal = StringLength(data) as u32;
                } else if (*boxMon).language == LANGUAGE_JAPANESE {
                    *data = EXT_CTRL_CODE_BEGIN;
                    *data.at(1) = EXT_CTRL_CODE_JPN;
                    retVal = 2;
                    i = 0;
                    while i < 5 && (*boxMon).nickname[i] != EOS {
                        *data.at(retVal) = (*boxMon).nickname[i];
                        retVal += 1;
                        i += 1;
                    }
                    *data.at({
                        let t1 = retVal;
                        retVal += 1;
                        t1
                    }) = EXT_CTRL_CODE_BEGIN;
                    *data.at({
                        let t2 = retVal;
                        retVal += 1;
                        t2
                    }) = EXT_CTRL_CODE_ENG;
                    *data.at(retVal) = EOS;
                } else {
                    retVal = 0;
                    while retVal < POKEMON_NAME_LENGTH {
                        *data.at(retVal) = (*boxMon).nickname[retVal];
                        retVal += 1;
                    }
                    *data.at(retVal) = EOS;
                }
                break 'l1;
            }
            MON_DATA_LANGUAGE => {
                retVal = (*boxMon).language as u32;
            }
            MON_DATA_SANITY_IS_BAD_EGG => {
                retVal = (*boxMon).isBadEgg() as u32;
            }
            MON_DATA_SANITY_HAS_SPECIES => {
                retVal = (*boxMon).hasSpecies() as u32;
            }
            MON_DATA_SANITY_IS_EGG => {
                retVal = (*boxMon).isEgg() as u32;
            }
            MON_DATA_OT_NAME => {
                retVal = 0;
                while retVal < PLAYER_NAME_LENGTH as u32 {
                    *data.at(retVal) = (*boxMon).otName[retVal];
                    retVal += 1;
                }
                *data.at(retVal) = EOS;
                break 'l1;
            }
            MON_DATA_MARKINGS => {
                retVal = (*boxMon).markings as u32;
            }
            MON_DATA_CHECKSUM => {
                retVal = (*boxMon).checksum as u32;
            }
            MON_DATA_ENCRYPT_SEPARATOR => {
                retVal = (*boxMon).unknown as u32;
            }
            MON_DATA_SPECIES => {
                retVal = (if (*boxMon).isBadEgg() != 0 {
                    SPECIES_EGG as i32
                } else {
                    (*substruct0).species as i32
                }) as u32;
            }
            MON_DATA_HELD_ITEM => {
                retVal = (*substruct0).heldItem as u32;
            }
            MON_DATA_EXP => {
                retVal = (*substruct0).experience;
            }
            MON_DATA_PP_BONUSES => {
                retVal = (*substruct0).ppBonuses as u32;
            }
            MON_DATA_FRIENDSHIP => {
                retVal = (*substruct0).friendship as u32;
            }
            MON_DATA_MOVE1 | MON_DATA_MOVE2 | MON_DATA_MOVE3 | MON_DATA_MOVE4 => {
                retVal = (*substruct1).moves[field - MON_DATA_MOVE1] as u32;
            }
            MON_DATA_PP1 | MON_DATA_PP2 | MON_DATA_PP3 | MON_DATA_PP4 => {
                retVal = (*substruct1).pp[field - MON_DATA_PP1] as u32;
            }
            MON_DATA_HP_EV => {
                retVal = (*substruct2).hpEV as u32;
            }
            MON_DATA_ATK_EV => {
                retVal = (*substruct2).attackEV as u32;
            }
            MON_DATA_DEF_EV => {
                retVal = (*substruct2).defenseEV as u32;
            }
            MON_DATA_SPEED_EV => {
                retVal = (*substruct2).speedEV as u32;
            }
            MON_DATA_SPATK_EV => {
                retVal = (*substruct2).spAttackEV as u32;
            }
            MON_DATA_SPDEF_EV => {
                retVal = (*substruct2).spDefenseEV as u32;
            }
            MON_DATA_COOL => {
                retVal = (*substruct2).cool as u32;
            }
            MON_DATA_BEAUTY => {
                retVal = (*substruct2).beauty as u32;
            }
            MON_DATA_CUTE => {
                retVal = (*substruct2).cute as u32;
            }
            MON_DATA_SMART => {
                retVal = (*substruct2).smart as u32;
            }
            MON_DATA_TOUGH => {
                retVal = (*substruct2).tough as u32;
            }
            MON_DATA_SHEEN => {
                retVal = (*substruct2).sheen as u32;
            }
            MON_DATA_POKERUS => {
                retVal = (*substruct3).pokerus as u32;
            }
            MON_DATA_MET_LOCATION => {
                retVal = (*substruct3).metLocation as u32;
            }
            MON_DATA_MET_LEVEL => {
                retVal = (*substruct3).metLevel() as u32;
            }
            MON_DATA_MET_GAME => {
                retVal = (*substruct3).metGame() as u32;
            }
            MON_DATA_POKEBALL => {
                retVal = (*substruct3).pokeball() as u32;
            }
            MON_DATA_OT_GENDER => {
                retVal = (*substruct3).otGender() as u32;
            }
            MON_DATA_HP_IV => {
                retVal = (*substruct3).hpIV();
            }
            MON_DATA_ATK_IV => {
                retVal = (*substruct3).attackIV();
            }
            MON_DATA_DEF_IV => {
                retVal = (*substruct3).defenseIV();
            }
            MON_DATA_SPEED_IV => {
                retVal = (*substruct3).speedIV();
            }
            MON_DATA_SPATK_IV => {
                retVal = (*substruct3).spAttackIV();
            }
            MON_DATA_SPDEF_IV => {
                retVal = (*substruct3).spDefenseIV();
            }
            MON_DATA_IS_EGG => {
                retVal = (*substruct3).isEgg();
            }
            MON_DATA_ABILITY_NUM => {
                retVal = (*substruct3).abilityNum();
            }
            MON_DATA_COOL_RIBBON => {
                retVal = (*substruct3).coolRibbon();
            }
            MON_DATA_BEAUTY_RIBBON => {
                retVal = (*substruct3).beautyRibbon();
            }
            MON_DATA_CUTE_RIBBON => {
                retVal = (*substruct3).cuteRibbon();
            }
            MON_DATA_SMART_RIBBON => {
                retVal = (*substruct3).smartRibbon();
            }
            MON_DATA_TOUGH_RIBBON => {
                retVal = (*substruct3).toughRibbon();
            }
            MON_DATA_CHAMPION_RIBBON => {
                retVal = (*substruct3).championRibbon();
            }
            MON_DATA_WINNING_RIBBON => {
                retVal = (*substruct3).winningRibbon();
            }
            MON_DATA_VICTORY_RIBBON => {
                retVal = (*substruct3).victoryRibbon();
            }
            MON_DATA_ARTIST_RIBBON => {
                retVal = (*substruct3).artistRibbon();
            }
            MON_DATA_EFFORT_RIBBON => {
                retVal = (*substruct3).effortRibbon();
            }
            MON_DATA_MARINE_RIBBON => {
                retVal = (*substruct3).marineRibbon();
            }
            MON_DATA_LAND_RIBBON => {
                retVal = (*substruct3).landRibbon();
            }
            MON_DATA_SKY_RIBBON => {
                retVal = (*substruct3).skyRibbon();
            }
            MON_DATA_COUNTRY_RIBBON => {
                retVal = (*substruct3).countryRibbon();
            }
            MON_DATA_NATIONAL_RIBBON => {
                retVal = (*substruct3).nationalRibbon();
            }
            MON_DATA_EARTH_RIBBON => {
                retVal = (*substruct3).earthRibbon();
            }
            MON_DATA_WORLD_RIBBON => {
                retVal = (*substruct3).worldRibbon();
            }
            MON_DATA_UNUSED_RIBBONS => {
                retVal = (*substruct3).unusedRibbons();
            }
            MON_DATA_MODERN_FATEFUL_ENCOUNTER => {
                retVal = (*substruct3).modernFatefulEncounter();
            }
            MON_DATA_SPECIES_OR_EGG => {
                retVal = (*substruct0).species as u32;
                if (*substruct0).species != 0
                    && ((*substruct3).isEgg() != 0 || (*boxMon).isBadEgg() != 0)
                {
                    retVal = SPECIES_EGG;
                }
            }
            MON_DATA_IVS => {
                retVal = (*substruct3).hpIV()
                    | (*substruct3).attackIV() << 5
                    | (*substruct3).defenseIV() << 10
                    | (*substruct3).speedIV() << 15
                    | (*substruct3).spAttackIV() << 20
                    | (*substruct3).spDefenseIV() << 25;
            }
            MON_DATA_KNOWN_MOVES => {
                if (*substruct0).species != 0 && (*substruct3).isEgg() == 0 {
                    let mut moves: *mut u16 = data as *mut u16;
                    let mut i: i32 = 0;
                    while *moves.at(i) != MOVES_COUNT {
                        let mut r#move: u16 = *moves.at(i);
                        if (*substruct1).moves[0] == r#move
                            || (*substruct1).moves[1] == r#move
                            || (*substruct1).moves[2] == r#move
                            || (*substruct1).moves[3] == r#move
                        {
                            retVal |= gBitTable[i];
                        }
                        i += 1;
                    }
                }
            }
            MON_DATA_RIBBON_COUNT => {
                retVal = 0;
                if (*substruct0).species != 0 && (*substruct3).isEgg() == 0 {
                    retVal += (*substruct3).coolRibbon();
                    retVal += (*substruct3).beautyRibbon();
                    retVal += (*substruct3).cuteRibbon();
                    retVal += (*substruct3).smartRibbon();
                    retVal += (*substruct3).toughRibbon();
                    retVal += (*substruct3).championRibbon();
                    retVal += (*substruct3).winningRibbon();
                    retVal += (*substruct3).victoryRibbon();
                    retVal += (*substruct3).artistRibbon();
                    retVal += (*substruct3).effortRibbon();
                    retVal += (*substruct3).marineRibbon();
                    retVal += (*substruct3).landRibbon();
                    retVal += (*substruct3).skyRibbon();
                    retVal += (*substruct3).countryRibbon();
                    retVal += (*substruct3).nationalRibbon();
                    retVal += (*substruct3).earthRibbon();
                    retVal += (*substruct3).worldRibbon();
                }
            }
            MON_DATA_RIBBONS => {
                retVal = 0;
                if (*substruct0).species != 0 && (*substruct3).isEgg() == 0 {
                    retVal = (*substruct3).championRibbon()
                        | (*substruct3).coolRibbon() << 1
                        | (*substruct3).beautyRibbon() << 4
                        | (*substruct3).cuteRibbon() << 7
                        | (*substruct3).smartRibbon() << 10
                        | (*substruct3).toughRibbon() << 13
                        | (*substruct3).winningRibbon() << 16
                        | (*substruct3).victoryRibbon() << 17
                        | (*substruct3).artistRibbon() << 18
                        | (*substruct3).effortRibbon() << 19
                        | (*substruct3).marineRibbon() << 20
                        | (*substruct3).landRibbon() << 21
                        | (*substruct3).skyRibbon() << 22
                        | (*substruct3).countryRibbon() << 23
                        | (*substruct3).nationalRibbon() << 24
                        | (*substruct3).earthRibbon() << 25
                        | (*substruct3).worldRibbon() << 26;
                }
            }
            _ => {}
        }
    }
    if field > MON_DATA_ENCRYPT_SEPARATOR {
        EncryptBoxMon(boxMon);
    }
    return retVal;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonData2(boxMon: *mut BoxPokemon, field: i32) -> u32 {
    return GetBoxMonData3(boxMon, field, null_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMonData(mon: *mut Pokemon, field: i32, dataArg: *mut c_void) {
    let mut data: *mut u8 = dataArg as *mut u8;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBoxMonData(boxMon: *mut BoxPokemon, field: i32, dataArg: *mut c_void) {
    let mut data: *mut u8 = dataArg as *mut u8;
    let mut substruct0: *mut PokemonSubstruct0 = null_mut();
    let mut substruct1: *mut PokemonSubstruct1 = null_mut();
    let mut substruct2: *mut PokemonSubstruct2 = null_mut();
    let mut substruct3: *mut PokemonSubstruct3 = null_mut();
    if field > MON_DATA_ENCRYPT_SEPARATOR {
        substruct0 = &raw mut (*GetSubstruct(boxMon, (*boxMon).personality, 0)).type0;
        substruct1 = &raw mut (*GetSubstruct(boxMon, (*boxMon).personality, 1)).type1;
        substruct2 = &raw mut (*GetSubstruct(boxMon, (*boxMon).personality, 2)).type2;
        substruct3 = &raw mut (*GetSubstruct(boxMon, (*boxMon).personality, 3)).type3;
        DecryptBoxMon(boxMon);
        if CalculateBoxMonChecksum(boxMon) != (*boxMon).checksum {
            (*boxMon).set_isBadEgg(TRUE);
            (*boxMon).set_isEgg(TRUE);
            (*substruct3).set_isEgg(TRUE as u32);
            EncryptBoxMon(boxMon);
            return;
        }
    }
    'l1: {
        match field {
            MON_DATA_PERSONALITY => {
                (*boxMon).personality = *data as u32
                    + ((*data.at(1) as u32) << 8)
                    + ((*data.at(2) as u32) << 16)
                    + ((*data.at(3) as u32) << 24);
            }
            MON_DATA_OT_ID => {
                (*boxMon).otId = *data as u32
                    + ((*data.at(1) as u32) << 8)
                    + ((*data.at(2) as u32) << 16)
                    + ((*data.at(3) as u32) << 24);
            }
            MON_DATA_NICKNAME => {
                let mut i: i32 = 0;
                i = 0;
                while i < POKEMON_NAME_LENGTH as i32 {
                    (*boxMon).nickname[i] = *data.at(i);
                    i += 1;
                }
                break 'l1;
            }
            MON_DATA_LANGUAGE => {
                (*boxMon).language = *data;
            }
            MON_DATA_SANITY_IS_BAD_EGG => {
                (*boxMon).set_isBadEgg(*data);
            }
            MON_DATA_SANITY_HAS_SPECIES => {
                (*boxMon).set_hasSpecies(*data);
            }
            MON_DATA_SANITY_IS_EGG => {
                (*boxMon).set_isEgg(*data);
            }
            MON_DATA_OT_NAME => {
                let mut i: i32 = 0;
                i = 0;
                while i < PLAYER_NAME_LENGTH {
                    (*boxMon).otName[i] = *data.at(i);
                    i += 1;
                }
                break 'l1;
            }
            MON_DATA_MARKINGS => {
                (*boxMon).markings = *data;
            }
            MON_DATA_CHECKSUM => {
                (*boxMon).checksum = *data as u16 + ((*data.at(1) as u16) << 8);
            }
            MON_DATA_ENCRYPT_SEPARATOR => {
                (*boxMon).unknown = *data as u16 + ((*data.at(1) as u16) << 8);
            }
            MON_DATA_SPECIES => {
                (*substruct0).species = *data as u16 + ((*data.at(1) as u16) << 8);
                if (*substruct0).species != 0 {
                    (*boxMon).set_hasSpecies(TRUE);
                } else {
                    (*boxMon).set_hasSpecies(FALSE);
                }
                break 'l1;
            }
            MON_DATA_HELD_ITEM => {
                (*substruct0).heldItem = *data as u16 + ((*data.at(1) as u16) << 8);
            }
            MON_DATA_EXP => {
                (*substruct0).experience = *data as u32
                    + ((*data.at(1) as u32) << 8)
                    + ((*data.at(2) as u32) << 16)
                    + ((*data.at(3) as u32) << 24);
            }
            MON_DATA_PP_BONUSES => {
                (*substruct0).ppBonuses = *data;
            }
            MON_DATA_FRIENDSHIP => {
                (*substruct0).friendship = *data;
            }
            MON_DATA_MOVE1 | MON_DATA_MOVE2 | MON_DATA_MOVE3 | MON_DATA_MOVE4 => {
                (*substruct1).moves[field - MON_DATA_MOVE1] =
                    *data as u16 + ((*data.at(1) as u16) << 8);
            }
            MON_DATA_PP1 | MON_DATA_PP2 | MON_DATA_PP3 | MON_DATA_PP4 => {
                (*substruct1).pp[field - MON_DATA_PP1] = *data;
            }
            MON_DATA_HP_EV => {
                (*substruct2).hpEV = *data;
            }
            MON_DATA_ATK_EV => {
                (*substruct2).attackEV = *data;
            }
            MON_DATA_DEF_EV => {
                (*substruct2).defenseEV = *data;
            }
            MON_DATA_SPEED_EV => {
                (*substruct2).speedEV = *data;
            }
            MON_DATA_SPATK_EV => {
                (*substruct2).spAttackEV = *data;
            }
            MON_DATA_SPDEF_EV => {
                (*substruct2).spDefenseEV = *data;
            }
            MON_DATA_COOL => {
                (*substruct2).cool = *data;
            }
            MON_DATA_BEAUTY => {
                (*substruct2).beauty = *data;
            }
            MON_DATA_CUTE => {
                (*substruct2).cute = *data;
            }
            MON_DATA_SMART => {
                (*substruct2).smart = *data;
            }
            MON_DATA_TOUGH => {
                (*substruct2).tough = *data;
            }
            MON_DATA_SHEEN => {
                (*substruct2).sheen = *data;
            }
            MON_DATA_POKERUS => {
                (*substruct3).pokerus = *data;
            }
            MON_DATA_MET_LOCATION => {
                if 1 == 1 {
                    (*substruct3).metLocation = *data;
                } else if 1 == 2 {
                    (*substruct3).metLocation = *data + (*data.at(1) << 8);
                } else if 1 == 4 {
                    (*substruct3).metLocation =
                        *data + (*data.at(1) << 8) + (*data.at(2) << 16) + (*data.at(3) << 24);
                }
            }
            MON_DATA_MET_LEVEL => {
                let mut metLevel: u8 = *data;
                (*substruct3).set_metLevel(metLevel as u16);
                break 'l1;
            }
            MON_DATA_MET_GAME => {
                (*substruct3).set_metGame(*data as u16);
            }
            MON_DATA_POKEBALL => {
                let mut pokeball: u8 = *data;
                (*substruct3).set_pokeball(pokeball as u16);
                break 'l1;
            }
            MON_DATA_OT_GENDER => {
                (*substruct3).set_otGender(*data as u16);
            }
            MON_DATA_HP_IV => {
                (*substruct3).set_hpIV(*data as u32);
            }
            MON_DATA_ATK_IV => {
                (*substruct3).set_attackIV(*data as u32);
            }
            MON_DATA_DEF_IV => {
                (*substruct3).set_defenseIV(*data as u32);
            }
            MON_DATA_SPEED_IV => {
                (*substruct3).set_speedIV(*data as u32);
            }
            MON_DATA_SPATK_IV => {
                (*substruct3).set_spAttackIV(*data as u32);
            }
            MON_DATA_SPDEF_IV => {
                (*substruct3).set_spDefenseIV(*data as u32);
            }
            MON_DATA_IS_EGG => {
                (*substruct3).set_isEgg(*data as u32);
                if (*substruct3).isEgg() != 0 {
                    (*boxMon).set_isEgg(TRUE);
                } else {
                    (*boxMon).set_isEgg(FALSE);
                }
            }
            MON_DATA_ABILITY_NUM => {
                (*substruct3).set_abilityNum(*data as u32);
            }
            MON_DATA_COOL_RIBBON => {
                (*substruct3).set_coolRibbon(*data as u32);
            }
            MON_DATA_BEAUTY_RIBBON => {
                (*substruct3).set_beautyRibbon(*data as u32);
            }
            MON_DATA_CUTE_RIBBON => {
                (*substruct3).set_cuteRibbon(*data as u32);
            }
            MON_DATA_SMART_RIBBON => {
                (*substruct3).set_smartRibbon(*data as u32);
            }
            MON_DATA_TOUGH_RIBBON => {
                (*substruct3).set_toughRibbon(*data as u32);
            }
            MON_DATA_CHAMPION_RIBBON => {
                (*substruct3).set_championRibbon(*data as u32);
            }
            MON_DATA_WINNING_RIBBON => {
                (*substruct3).set_winningRibbon(*data as u32);
            }
            MON_DATA_VICTORY_RIBBON => {
                (*substruct3).set_victoryRibbon(*data as u32);
            }
            MON_DATA_ARTIST_RIBBON => {
                (*substruct3).set_artistRibbon(*data as u32);
            }
            MON_DATA_EFFORT_RIBBON => {
                (*substruct3).set_effortRibbon(*data as u32);
            }
            MON_DATA_MARINE_RIBBON => {
                (*substruct3).set_marineRibbon(*data as u32);
            }
            MON_DATA_LAND_RIBBON => {
                (*substruct3).set_landRibbon(*data as u32);
            }
            MON_DATA_SKY_RIBBON => {
                (*substruct3).set_skyRibbon(*data as u32);
            }
            MON_DATA_COUNTRY_RIBBON => {
                (*substruct3).set_countryRibbon(*data as u32);
            }
            MON_DATA_NATIONAL_RIBBON => {
                (*substruct3).set_nationalRibbon(*data as u32);
            }
            MON_DATA_EARTH_RIBBON => {
                (*substruct3).set_earthRibbon(*data as u32);
            }
            MON_DATA_WORLD_RIBBON => {
                (*substruct3).set_worldRibbon(*data as u32);
            }
            MON_DATA_UNUSED_RIBBONS => {
                (*substruct3).set_unusedRibbons(*data as u32);
            }
            MON_DATA_MODERN_FATEFUL_ENCOUNTER => {
                (*substruct3).set_modernFatefulEncounter(*data as u32);
            }
            MON_DATA_IVS => {
                let mut ivs: u32 = *data as u32
                    | (*data.at(1) as u32) << 8
                    | (*data.at(2) as u32) << 16
                    | (*data.at(3) as u32) << 24;
                (*substruct3).set_hpIV(ivs & MAX_IV_MASK);
                (*substruct3).set_attackIV(ivs >> 5 & MAX_IV_MASK);
                (*substruct3).set_defenseIV(ivs >> 10 & MAX_IV_MASK);
                (*substruct3).set_speedIV(ivs >> 15 & MAX_IV_MASK);
                (*substruct3).set_spAttackIV(ivs >> 20 & MAX_IV_MASK);
                (*substruct3).set_spDefenseIV(ivs >> 25 & MAX_IV_MASK);
                break 'l1;
            }
            _ => {}
        }
    }
    if field > MON_DATA_ENCRYPT_SEPARATOR {
        (*boxMon).checksum = CalculateBoxMonChecksum(boxMon);
        EncryptBoxMon(boxMon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyMon(dest: *mut c_void, src: *mut c_void, size: u32) {
    memcpy(dest as *mut u8, src as *mut u8, size);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveMonToPlayer(mon: *mut Pokemon) -> u8 {
    let mut i: i32 = 0;
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
    i = 0;
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
    return MON_GIVEN_TO_PARTY;
}
pub(crate) unsafe extern "C" fn CopyMonToPC(mon: *mut Pokemon) -> u8 {
    let mut boxNo: i32 = 0;
    let mut boxPos: i32 = 0;
    SetPCBoxToSendMon(VarGet(VAR_PC_BOX_TO_SEND_MON) as u8);
    boxNo = StorageGetCurrentBox() as i32;
    loop {
        boxPos = 0;
        while boxPos < IN_BOX_COUNT {
            let mut checkingMon: *mut BoxPokemon = GetBoxedMonPtr(boxNo as u8, boxPos as u8);
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
            boxPos += 1;
        }
        boxNo += 1;
        if boxNo == TOTAL_BOXES_COUNT as i32 {
            boxNo = 0;
        }
        if boxNo == StorageGetCurrentBox() as i32 {
            break;
        }
    }
    return MON_CANT_GIVE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculatePlayerPartyCount() -> u8 {
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
    return gPlayerPartyCount;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculateEnemyPartyCount() -> u8 {
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
    return gEnemyPartyCount;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonsStateToDoubles() -> u8 {
    let mut aliveCount: i32 = 0;
    let mut i: i32 = 0;
    CalculatePlayerPartyCount();
    if gPlayerPartyCount == 1 {
        return gPlayerPartyCount;
    }
    i = 0;
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
    return (if aliveCount > 1 {
        PLAYER_HAS_TWO_USABLE_MONS
    } else {
        PLAYER_HAS_ONE_USABLE_MON
    }) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonsStateToDoubles_2() -> u8 {
    let mut aliveCount: i32 = 0;
    let mut i: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
        let mut species: u32 = GetMonData3(
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
        i += 1;
    }
    if aliveCount == 1 {
        return PLAYER_HAS_ONE_MON;
    }
    return (if aliveCount > 1 {
        PLAYER_HAS_TWO_USABLE_MONS
    } else {
        PLAYER_HAS_ONE_USABLE_MON
    }) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAbilityBySpecies(species: u16, abilityNum: u8) -> u8 {
    if abilityNum != 0 {
        gLastUsedAbility = gSpeciesInfo[species].abilities[1];
    } else {
        gLastUsedAbility = gSpeciesInfo[species].abilities[0];
    }
    return gLastUsedAbility;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonAbility(mon: *mut Pokemon) -> u8 {
    let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut abilityNum: u8 = GetMonData3(mon, MON_DATA_ABILITY_NUM, null_mut()) as u8;
    return GetAbilityBySpecies(species, abilityNum);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateSecretBaseEnemyParty(secretBaseRecord: *mut SecretBase) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    ZeroEnemyPartyMons();
    *(*gBattleResources).secretBase = *secretBaseRecord;
    i = 0;
    while i < PARTY_SIZE {
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
            j = 0;
            while j < MAX_MON_MOVES {
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
                j += 1;
            }
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseTrainerPicIndex() -> u8 {
    let mut facilityClass: u8 = sSecretBaseFacilityClasses
        [(*(*gBattleResources).secretBase).gender()]
        [(*(*gBattleResources).secretBase).trainerId[0] as i32 % 5];
    return gFacilityClassToPicIndex[facilityClass];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseTrainerClass() -> u8 {
    let mut facilityClass: u8 = sSecretBaseFacilityClasses
        [(*(*gBattleResources).secretBase).gender()]
        [(*(*gBattleResources).secretBase).trainerId[0] as i32 % 5];
    return gFacilityClassToTrainerClass[facilityClass];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPlayerPartyAndPokemonStorageFull() -> u8 {
    let mut i: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
        if GetMonData3(&raw mut gPlayerParty[i], MON_DATA_SPECIES, null_mut())
            == SPECIES_NONE as u32
        {
            return FALSE;
        }
        i += 1;
    }
    return IsPokemonStorageFull();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPokemonStorageFull() -> u8 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < TOTAL_BOXES_COUNT as i32 {
        j = 0;
        while j < IN_BOX_COUNT {
            if GetBoxMonDataAt(i as u8, j as u8, MON_DATA_SPECIES) == SPECIES_NONE as u32 {
                return FALSE;
            }
            j += 1;
        }
        i += 1;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSpeciesName(mut name: *mut u8, species: u16) {
    let mut i: i32 = 0;
    i = 0;
    while i <= POKEMON_NAME_LENGTH as i32 {
        if species > NUM_SPECIES {
            *name.at(i) = gSpeciesNames[0][i];
        } else {
            *name.at(i) = gSpeciesNames[species][i];
        }
        if *name.at(i) == EOS {
            break;
        }
        i += 1;
    }
    *name.at(i) = EOS;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalculatePPWithBonus(r#move: u16, ppBonuses: u8, moveIndex: u8) -> u8 {
    let mut basePP: u8 = gBattleMoves[r#move].pp;
    return basePP
        + (basePP as i32
            * 20
            * shr_i32(
                gPPUpGetMask[moveIndex] as i32 & ppBonuses as i32,
                2 * moveIndex as u32,
            )
            / 100) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveMonPPBonus(mon: *mut Pokemon, moveIndex: u8) {
    let mut ppBonuses: u8 = GetMonData3(mon, MON_DATA_PP_BONUSES, null_mut()) as u8;
    ppBonuses &= gPPUpClearMask[moveIndex];
    SetMonData(mon, MON_DATA_PP_BONUSES, &raw mut ppBonuses as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveBattleMonPPBonus(mon: *mut BattlePokemon, moveIndex: u8) {
    (*mon).ppBonuses &= gPPUpClearMask[moveIndex];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyPlayerPartyMonToBattleData(battler: u8, partyIndex: u8) {
    let mut hpSwitchout: *mut u16 = null_mut();
    let mut i: i32 = 0;
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
    i = 0;
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
    hpSwitchout = &raw mut (*gBattleStruct).hpOnSwitchout[GetBattlerSide(battler)];
    *hpSwitchout = gBattleMons[battler].hp;
    i = 0;
    while i < NUM_BATTLE_STATS {
        gBattleMons[battler].statStages[i] = DEFAULT_STAT_STAGE;
        i += 1;
    }
    gBattleMons[battler].status2 = 0;
    UpdateSentPokesToOpponentValue(battler);
    ClearTemporarySpeciesSpriteData(battler, FALSE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ExecuteTableBasedItemEffect(
    mon: *mut Pokemon,
    item: u16,
    partyIndex: u8,
    moveIndex: u8,
) -> u8 {
    return PokemonUseItemEffects(mon, item, partyIndex, moveIndex, FALSE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokemonUseItemEffects(
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
    let mut heldItem: u16 = 0;
    let mut effectFlags: u8 = 0;
    let mut evChange: i8 = 0;
    let mut evCount: u16 = 0;
    heldItem = GetMonData3(mon, MON_DATA_HELD_ITEM, null_mut()) as u16;
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
    i = 0;
    while i < ITEM_EFFECT_ARG_START as i32 {
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
                        dataUnsigned =
                            GetMonData3(mon, MON_DATA_PP1 + moveIndex as i32, null_mut())
                                + dataUnsigned;
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
                                            dataUnsigned =
                                                GetMonData3(mon, MON_DATA_HP, null_mut())
                                                    + dataUnsigned;
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
                                                    if gBattleResults.numHealingItemsUsed < 255 {
                                                        gBattleResults.numHealingItemsUsed += 1;
                                                    }
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
                                            let mut r#move: u16 = 0;
                                            dataUnsigned = GetMonData3(
                                                mon,
                                                MON_DATA_PP1 + temp2 as i32,
                                                null_mut(),
                                            );
                                            r#move = GetMonData3(
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
                                                            & gBitTable[temp2]
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
                                        let mut r#move: u16 = 0;
                                        dataUnsigned = GetMonData3(
                                            mon,
                                            MON_DATA_PP1 + moveIndex as i32,
                                            null_mut(),
                                        );
                                        r#move = GetMonData3(
                                            mon,
                                            MON_DATA_MOVE1 + moveIndex as i32,
                                            null_mut(),
                                        ) as u16;
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
                                                        & gBitTable[moveIndex]
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
                                    let mut targetSpecies: u16 =
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
                                0 | 1 | 2 | 3 => {
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
                                        dataUnsigned = GetMonData3(
                                            mon,
                                            MON_DATA_PP1 + moveIndex as i32,
                                            null_mut(),
                                        ) + dataUnsigned;
                                        SetMonData(
                                            mon,
                                            MON_DATA_PP1 + moveIndex as i32,
                                            &raw mut dataUnsigned as *mut c_void,
                                        );
                                        retVal = FALSE;
                                    }
                                }
                                5 => {
                                    if GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut()) < 100 {
                                        if (retVal == 0 || friendshipOnly != 0)
                                            && ShouldSkipFriendshipChange() == 0
                                            && friendshipChange == 0
                                        {
                                            friendshipChange =
                                                *itemEffect.at(itemEffectParam) as i8;
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
                                                if GetMonData3(
                                                    mon,
                                                    MON_DATA_MET_LOCATION,
                                                    null_mut(),
                                                ) == GetCurrentRegionMapSectionId() as u32
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
                                    }
                                    itemEffectParam += 1;
                                }
                                6 => {
                                    if GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut()) >= 100
                                        && GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut()) < 200
                                    {
                                        if (retVal == 0 || friendshipOnly != 0)
                                            && ShouldSkipFriendshipChange() == 0
                                            && friendshipChange == 0
                                        {
                                            friendshipChange =
                                                *itemEffect.at(itemEffectParam) as i8;
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
                                                if GetMonData3(
                                                    mon,
                                                    MON_DATA_MET_LOCATION,
                                                    null_mut(),
                                                ) == GetCurrentRegionMapSectionId() as u32
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
                                    }
                                    itemEffectParam += 1;
                                }
                                7 => {
                                    if GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut()) >= 200 {
                                        if (retVal == 0 || friendshipOnly != 0)
                                            && ShouldSkipFriendshipChange() == 0
                                            && friendshipChange == 0
                                        {
                                            friendshipChange =
                                                *itemEffect.at(itemEffectParam) as i8;
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
                                                if GetMonData3(
                                                    mon,
                                                    MON_DATA_MET_LOCATION,
                                                    null_mut(),
                                                ) == GetCurrentRegionMapSectionId() as u32
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
        i += 1;
    }
    return retVal;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HealStatusConditions(
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemEffectParamOffset(
    itemId: u16,
    effectByte: u8,
    mut effectBit: u8,
) -> u8 {
    let mut temp: *mut u8 = null_mut();
    let mut itemEffect: *mut u8 = null_mut();
    let mut offset: u8 = 0;
    let mut i: i32 = 0;
    let mut j: u8 = 0;
    let mut effectFlags: u8 = 0;
    offset = ITEM_EFFECT_ARG_START;
    temp = gItemEffectTable[itemId as i32 - ITEM_POTION];
    if temp.is_null() && itemId != ITEM_ENIGMA_BERRY {
        return 0;
    }
    if itemId == ITEM_ENIGMA_BERRY {
        temp = gEnigmaBerries[gActiveBattler].itemEffect.as_mut_ptr();
    }
    itemEffect = temp;
    i = 0;
    while i < ITEM_EFFECT_ARG_START as i32 {
        match i {
            0 | 1 | 2 | 3 => {
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
                                fall = true;
                                if i == effectByte as i32
                                    && effectFlags as i32 & effectBit as i32 != 0
                                {
                                    return offset;
                                }
                                offset += 1;
                                break 'l4;
                            }
                            if sw1 == 1 {
                                fall = true;
                                if i == effectByte as i32
                                    && effectFlags as i32 & effectBit as i32 != 0
                                {
                                    return offset;
                                }
                                offset += 1;
                                break 'l4;
                            }
                            if sw1 == 3 {
                                fall = true;
                                if i == effectByte as i32
                                    && effectFlags as i32 & effectBit as i32 != 0
                                {
                                    return offset;
                                }
                                offset += 1;
                                break 'l4;
                            }
                            if sw1 == 7 {
                                fall = true;
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
                            0 | 1 | 2 | 3 | 4 | 5 | 6 => {
                                if i == effectByte as i32
                                    && effectFlags as i32 & effectBit as i32 != 0
                                {
                                    return offset;
                                }
                                offset += 1;
                            }
                            7 => {
                                if i == effectByte as i32 {
                                    return 0;
                                }
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
        i += 1;
    }
    return offset;
}
pub(crate) unsafe extern "C" fn BufferStatRoseMessage(statIdx: i32) {
    gBattlerTarget = gBattlerInMenuId;
    StringCopy(
        gBattleTextBuff1.as_mut_ptr(),
        gStatNamesTable[sStatsToRaise[statIdx]],
    );
    StringCopy(
        gBattleTextBuff2.as_mut_ptr(),
        gText_StatRose.as_ptr().cast_mut(),
    );
    BattleStringExpandPlaceholdersToDisplayedString(gText_DefendersStatRose.as_ptr().cast_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UseStatIncreaseItem(itemId: u16) -> *mut u8 {
    let mut i: i32 = 0;
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
    i = 0;
    while i < 3 {
        if *itemEffect.at(i) as i32 & 15 != 0 {
            BufferStatRoseMessage(i * 2);
        }
        if *itemEffect.at(i) as i32 & 240 != 0 {
            if i != 0 {
                BufferStatRoseMessage(i * 2 + 1);
            } else {
                gBattlerAttacker = gBattlerInMenuId;
                BattleStringExpandPlaceholdersToDisplayedString(
                    gText_PkmnGettingPumped.as_ptr().cast_mut(),
                );
            }
        }
        i += 1;
    }
    if *itemEffect.at(3) as i32 & ITEM3_GUARD_SPEC != 0 {
        gBattlerAttacker = gBattlerInMenuId;
        BattleStringExpandPlaceholdersToDisplayedString(
            gText_PkmnShroudedInMist.as_ptr().cast_mut(),
        );
    }
    return gDisplayedStringBattle.as_mut_ptr();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNature(mon: *mut Pokemon) -> u8 {
    return (GetMonData3(mon, MON_DATA_PERSONALITY, null_mut()) % 25) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNatureFromPersonality(personality: u32) -> u8 {
    return (personality % 25) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetEvolutionTargetSpecies(
    mon: *mut Pokemon,
    mode: u8,
    evolutionItem: u16,
) -> u16 {
    let mut i: i32 = 0;
    let mut targetSpecies: u16 = 0;
    let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut heldItem: u16 = GetMonData3(mon, MON_DATA_HELD_ITEM, null_mut()) as u16;
    let mut personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    let mut level: u8 = 0;
    let mut friendship: u16 = 0;
    let mut beauty: u8 = GetMonData3(mon, MON_DATA_BEAUTY, null_mut()) as u8;
    let mut upperPersonality: u16 = (personality >> 16) as u16;
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
            i = 0;
            while i < EVOS_PER_MON {
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
                        if gEvolutionTable[species][i].param <= level as u16 {
                            if GetMonData3(mon, MON_DATA_ATK, null_mut())
                                > GetMonData3(mon, MON_DATA_DEF, null_mut())
                            {
                                targetSpecies = gEvolutionTable[species][i].targetSpecies;
                            }
                        }
                    }
                    EVO_LEVEL_ATK_EQ_DEF => {
                        if gEvolutionTable[species][i].param <= level as u16 {
                            if GetMonData3(mon, MON_DATA_ATK, null_mut())
                                == GetMonData3(mon, MON_DATA_DEF, null_mut())
                            {
                                targetSpecies = gEvolutionTable[species][i].targetSpecies;
                            }
                        }
                    }
                    EVO_LEVEL_ATK_LT_DEF => {
                        if gEvolutionTable[species][i].param <= level as u16 {
                            if GetMonData3(mon, MON_DATA_ATK, null_mut())
                                < GetMonData3(mon, MON_DATA_DEF, null_mut())
                            {
                                targetSpecies = gEvolutionTable[species][i].targetSpecies;
                            }
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
                    EVO_BEAUTY => {
                        if gEvolutionTable[species][i].param <= beauty as u16 {
                            targetSpecies = gEvolutionTable[species][i].targetSpecies;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        EVO_MODE_TRADE => {
            i = 0;
            while i < EVOS_PER_MON {
                match gEvolutionTable[species][i].method {
                    EVO_TRADE => {
                        targetSpecies = gEvolutionTable[species][i].targetSpecies;
                    }
                    EVO_TRADE_ITEM => {
                        if gEvolutionTable[species][i].param == heldItem {
                            heldItem = ITEM_NONE;
                            SetMonData(mon, MON_DATA_HELD_ITEM, &raw mut heldItem as *mut c_void);
                            targetSpecies = gEvolutionTable[species][i].targetSpecies;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        EVO_MODE_ITEM_USE | EVO_MODE_ITEM_CHECK => {
            i = 0;
            while i < EVOS_PER_MON {
                if gEvolutionTable[species][i].method == EVO_ITEM
                    && gEvolutionTable[species][i].param == evolutionItem
                {
                    targetSpecies = gEvolutionTable[species][i].targetSpecies;
                    break;
                }
                i += 1;
            }
        }
        _ => {}
    }
    return targetSpecies;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HoennPokedexNumToSpecies(hoennNum: u16) -> u16 {
    let mut species: u16 = 0;
    if hoennNum == 0 {
        return 0;
    }
    species = 0;
    while species < 411 && sSpeciesToHoennPokedexNum[species] != hoennNum {
        species += 1;
    }
    if species == 411 {
        return 0;
    }
    return species + 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn NationalPokedexNumToSpecies(nationalNum: u16) -> u16 {
    let mut species: u16 = 0;
    if nationalNum == 0 {
        return 0;
    }
    species = 0;
    while species < 411 && sSpeciesToNationalPokedexNum[species] != nationalNum {
        species += 1;
    }
    if species == 411 {
        return 0;
    }
    return species + 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn NationalToHoennOrder(nationalNum: u16) -> u16 {
    let mut hoennNum: u16 = 0;
    if nationalNum == 0 {
        return 0;
    }
    hoennNum = 0;
    while hoennNum < 411 && sHoennToNationalOrder[hoennNum] != nationalNum {
        hoennNum += 1;
    }
    if hoennNum == 411 {
        return 0;
    }
    return hoennNum + 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpeciesToNationalPokedexNum(species: u16) -> u16 {
    if species == 0 {
        return 0;
    }
    return sSpeciesToNationalPokedexNum[species as i32 - 1];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpeciesToHoennPokedexNum(species: u16) -> u16 {
    if species == 0 {
        return 0;
    }
    return sSpeciesToHoennPokedexNum[species as i32 - 1];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HoennToNationalOrder(hoennNum: u16) -> u16 {
    if hoennNum == 0 {
        return 0;
    }
    return sHoennToNationalOrder[hoennNum as i32 - 1];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpeciesToCryId(species: u16) -> u16 {
    if species <= 250 {
        return species;
    }
    if species < 276 {
        return 200;
    }
    return gSpeciesIdToCryId[species as i32 - 276];
}
pub(crate) unsafe extern "C" fn DrawSpindaSpotsUnused(
    species: u16,
    mut personality: u32,
    dest: *mut u8,
) {
    if species == SPECIES_SPINDA
        && (dest as usize) != ((*gMonSpritesGfxPtr).sprites.ptr[0] as usize)
        && (dest as usize) != ((*gMonSpritesGfxPtr).sprites.ptr[2] as usize)
    {
        let mut i: i32 = 0;
        i = 0;
        while i < 4 {
            let mut row: i32 = 0;
            let mut x: u8 = gSpindaSpotGraphics[i].x + ((personality as u8 & 0x0F) - 8);
            let mut y: u8 = gSpindaSpotGraphics[i].y + (((personality & 0xF0) >> 4) as u8 - 8);
            row = 0;
            while row < 16 {
                let mut column: i32 = 0;
                let mut spotPixelRow: i32 = gSpindaSpotGraphics[i].image[row] as i32;
                column = x as i32;
                while column < x as i32 + 16 {
                    let mut destPixels: *mut u8 = dest
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
                    column += 1;
                }
                y += 1;
                row += 1;
            }
            personality >>= 8;
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawSpindaSpots(
    species: u16,
    mut personality: u32,
    dest: *mut u8,
    isFrontPic: u8,
) {
    if species == SPECIES_SPINDA && isFrontPic != 0 {
        let mut i: i32 = 0;
        i = 0;
        while i < 4 {
            let mut row: i32 = 0;
            let mut x: u8 = gSpindaSpotGraphics[i].x + ((personality as u8 & 0x0F) - 8);
            let mut y: u8 = gSpindaSpotGraphics[i].y + (((personality & 0xF0) >> 4) as u8 - 8);
            row = 0;
            while row < 16 {
                let mut column: i32 = 0;
                let mut spotPixelRow: i32 = gSpindaSpotGraphics[i].image[row] as i32;
                column = x as i32;
                while column < x as i32 + 16 {
                    let mut destPixels: *mut u8 = dest
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
                    column += 1;
                }
                y += 1;
                row += 1;
            }
            personality >>= 8;
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionRenameMon(mon: *mut Pokemon, oldSpecies: u16, newSpecies: u16) {
    let mut language: u8 = 0;
    GetMonData3(mon, MON_DATA_NICKNAME, gStringVar1.as_mut_ptr());
    language = GetMonData3(mon, MON_DATA_LANGUAGE, &raw mut language) as u8;
    if language == GAME_LANGUAGE
        && StringCompare(
            gSpeciesNames[oldSpecies].as_ptr().cast_mut(),
            gStringVar1.as_mut_ptr(),
        ) == 0
    {
        SetMonData(
            mon,
            MON_DATA_NICKNAME,
            gSpeciesNames[newSpecies].as_ptr().cast_mut() as *mut c_void,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerFlankId() -> u8 {
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
    return flankId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkTrainerFlankId(linkPlayerId: u8) -> u16 {
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
    return flankId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerMultiplayerId(id: u16) -> i32 {
    let mut multiplayerId: i32 = 0;
    multiplayerId = 0;
    while multiplayerId < MAX_LINK_PLAYERS {
        if gLinkPlayers[multiplayerId].id == id {
            break;
        }
        multiplayerId += 1;
    }
    return multiplayerId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerEncounterMusicId(trainerOpponentId: u16) -> u8 {
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        return GetTrainerEncounterMusicIdInBattlePyramid(trainerOpponentId);
    } else if InTrainerHillChallenge() != 0 {
        return GetTrainerEncounterMusicIdInTrainerHill(trainerOpponentId);
    } else {
        return gTrainers[trainerOpponentId].encounterMusic_gender & 0x7F;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ModifyStatByNature(nature: u8, stat: u16, statIndex: u8) -> u16 {
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
    return retVal;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AdjustFriendship(mon: *mut Pokemon, event: u8) {
    let mut species: u16 = 0;
    let mut heldItem: u16 = 0;
    let mut holdEffect: u8 = 0;
    let mut r#mod: i8 = 0;
    if ShouldSkipFriendshipChange() != 0 {
        return;
    }
    species = GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
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
    if species != 0 && species != SPECIES_EGG as u16 {
        let mut friendshipLevel: u8 = 0;
        let mut friendship: i16 = GetMonData3(mon, MON_DATA_FRIENDSHIP, null_mut()) as i16;
        if friendship > 99 {
            friendshipLevel += 1;
        }
        if friendship > 199 {
            friendshipLevel += 1;
        }
        if event == FRIENDSHIP_EVENT_WALKING {
            if Random() as i32 & 1 != 0 {
                return;
            }
        }
        if event == FRIENDSHIP_EVENT_LEAGUE_BATTLE {
            if gBattleTypeFlags & BATTLE_TYPE_TRAINER == 0 {
                return;
            }
            if !(gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_LEADER
                || gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_ELITE_FOUR
                || gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_CHAMPION)
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonGainEVs(mon: *mut Pokemon, defeatedSpecies: u16) {
    let mut evs: CArray<u8, 6> = zeroed();
    let mut evIncrease: u16 = 0;
    let mut totalEVs: u16 = 0;
    let mut heldItem: u16 = 0;
    let mut holdEffect: u8 = 0;
    let mut i: i32 = 0;
    let mut multiplier: i32 = 0;
    i = 0;
    while i < NUM_STATS {
        evs[i] = GetMonData3(mon, MON_DATA_HP_EV + i, null_mut()) as u8;
        totalEVs += evs[i] as u16;
        i += 1;
    }
    i = 0;
    while i < NUM_STATS {
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
            let mut val1: i32 = evIncrease as i16 as i32 + MAX_PER_STAT_EVS;
            let mut val2: i32 = evs[i] as i32 + evIncrease as i32;
            evIncrease = val1 as u16 - val2 as u16;
        }
        evs[i] += evIncrease as u8;
        totalEVs += evIncrease;
        SetMonData(mon, MON_DATA_HP_EV + i, &raw mut evs[i] as *mut c_void);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonEVCount(mon: *mut Pokemon) -> u16 {
    let mut i: i32 = 0;
    let mut count: u16 = 0;
    i = 0;
    while i < NUM_STATS {
        count += GetMonData3(mon, MON_DATA_HP_EV + i, null_mut()) as u16;
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RandomlyGivePartyPokerus(mut party: *mut Pokemon) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckPartyPokerus(mut party: *mut Pokemon, mut selection: u8) -> u8 {
    let mut retVal: u8 = 0;
    let mut partyIndex: i32 = 0;
    let mut curBit: u32 = 1;
    retVal = 0;
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
    return retVal;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckPartyHasHadPokerus(mut party: *mut Pokemon, mut selection: u8) -> u8 {
    let mut retVal: u8 = 0;
    let mut partyIndex: i32 = 0;
    let mut curBit: u32 = 1;
    retVal = 0;
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
    return retVal;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdatePartyPokerusTime(days: u16) {
    let mut i: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
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
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PartySpreadPokerus(mut party: *mut Pokemon) {
    if Random() as i32 % 3 == 0 {
        let mut i: i32 = 0;
        i = 0;
        while i < PARTY_SIZE {
            if GetMonData3(party.at(i), MON_DATA_SPECIES, null_mut()) != 0 {
                let mut pokerus: u8 = GetMonData3(party.at(i), MON_DATA_POKERUS, null_mut()) as u8;
                let mut curPokerus: u8 = pokerus;
                if pokerus != 0 {
                    if pokerus as i32 & 0xF != 0 {
                        if i != 0
                            && GetMonData3(party.at(i - 1), MON_DATA_POKERUS, null_mut()) & 0xF0
                                == 0
                        {
                            SetMonData(
                                party.at(i - 1),
                                MON_DATA_POKERUS,
                                &raw mut curPokerus as *mut c_void,
                            );
                        }
                        if i != 5
                            && GetMonData3(party.at(i + 1), MON_DATA_POKERUS, null_mut()) & 0xF0
                                == 0
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
            }
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryIncrementMonLevel(mon: *mut Pokemon) -> u8 {
    let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanMonLearnTMHM(mon: *mut Pokemon, tm: u8) -> u32 {
    let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
    if species == SPECIES_EGG as u16 {
        return 0;
    }
    if 8 <= 8 {
        if tm < 32 {
            let mut mask: u32 = shl_i32(1, tm as u32) as u32;
            return gTMHMLearnsets[species].as_u32s[0] & mask;
        } else {
            let mut mask: u32 = shl_i32(1, tm as u32 - 32) as u32;
            return gTMHMLearnsets[species].as_u32s[1] & mask;
        }
    } else {
        let mut index: u32 = (tm as i32 / 32) as u32;
        let mut mask: u32 = shl_i32(1, (tm as i32 % 32) as u32) as u32;
        return gTMHMLearnsets[species].as_u32s[index] & mask;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanSpeciesLearnTMHM(species: u16, tm: u8) -> u32 {
    if species == SPECIES_EGG as u16 {
        return 0;
    }
    if 8 <= 8 {
        if tm < 32 {
            let mut mask: u32 = shl_i32(1, tm as u32) as u32;
            return gTMHMLearnsets[species].as_u32s[0] & mask;
        } else {
            let mut mask: u32 = shl_i32(1, tm as u32 - 32) as u32;
            return gTMHMLearnsets[species].as_u32s[1] & mask;
        }
    } else {
        let mut index: u32 = (tm as i32 / 32) as u32;
        let mut mask: u32 = shl_i32(1, (tm as i32 % 32) as u32) as u32;
        return gTMHMLearnsets[species].as_u32s[index] & mask;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMoveRelearnerMoves(mon: *mut Pokemon, mut moves: *mut u16) -> u8 {
    let mut learnedMoves: CArray<u16, 4> = zeroed();
    let mut numMoves: u8 = 0;
    let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut level: u8 = GetMonData3(mon, MON_DATA_LEVEL, null_mut()) as u8;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        learnedMoves[i] = GetMonData3(mon, MON_DATA_MOVE1 + i, null_mut()) as u16;
        i += 1;
    }
    i = 0;
    while i < MAX_LEVEL_UP_MOVES {
        let mut moveLevel: u16 = 0;
        if *gLevelUpLearnsets[species].at(i) == LEVEL_UP_END {
            break;
        }
        moveLevel = *gLevelUpLearnsets[species].at(i) & LEVEL_UP_MOVE_LV as u16;
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
        i += 1;
    }
    return numMoves;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLevelUpMovesBySpecies(species: u16, mut moves: *mut u16) -> u8 {
    let mut numMoves: u8 = 0;
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_LEVEL_UP_MOVES && *gLevelUpLearnsets[species].at(i) != LEVEL_UP_END {
        *moves.at({
            let t1 = numMoves;
            numMoves += 1;
            t1
        }) = *gLevelUpLearnsets[species].at(i) & LEVEL_UP_MOVE_ID;
        i += 1;
    }
    return numMoves;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumberOfRelearnableMoves(mon: *mut Pokemon) -> u8 {
    let mut learnedMoves: CArray<u16, 4> = zeroed();
    let mut moves: CArray<u16, 20> = zeroed();
    let mut numMoves: u8 = 0;
    let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
    let mut level: u8 = GetMonData3(mon, MON_DATA_LEVEL, null_mut()) as u8;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    if species == SPECIES_EGG as u16 {
        return 0;
    }
    i = 0;
    while i < MAX_MON_MOVES {
        learnedMoves[i] = GetMonData3(mon, MON_DATA_MOVE1 + i, null_mut()) as u16;
        i += 1;
    }
    i = 0;
    while i < MAX_LEVEL_UP_MOVES {
        let mut moveLevel: u16 = 0;
        if *gLevelUpLearnsets[species].at(i) == LEVEL_UP_END {
            break;
        }
        moveLevel = *gLevelUpLearnsets[species].at(i) & LEVEL_UP_MOVE_LV as u16;
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
        i += 1;
    }
    return numMoves;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpeciesToPokedexNum(mut species: u16) -> u16 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSpeciesInHoennDex(species: u16) -> u32 {
    if SpeciesToHoennPokedexNum(species) > HOENN_DEX_DEOXYS {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBattleMonForms() {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_BATTLERS_COUNT as i32 {
        gBattleMonForms[i] = 0;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleBGM() -> u16 {
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
            trainerClass = gTrainers[gTrainerBattleOpponent_A].trainerClass;
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
                    gTrainers[gTrainerBattleOpponent_A]
                        .trainerName
                        .as_ptr()
                        .cast_mut(),
                    gText_BattleWallyName.as_ptr().cast_mut(),
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayBattleBGM() {
    ResetMapMusic();
    m4aMPlayAllStop();
    PlayBGM(GetBattleBGM());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayMapChosenOrBattleBGM(songId: u16) {
    ResetMapMusic();
    m4aMPlayAllStop();
    if songId != 0 {
        PlayNewMapMusic(songId);
    } else {
        PlayNewMapMusic(GetBattleBGM());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_PlayMapChosenOrBattleBGM(songId: u16) {
    let mut taskId: u8 = 0;
    ResetMapMusic();
    m4aMPlayAllStop();
    taskId = CreateTask(Some(Task_PlayMapChosenOrBattleBGM), 0);
    gTasks[taskId].data[0] = songId as i16;
}
pub(crate) unsafe extern "C" fn Task_PlayMapChosenOrBattleBGM(taskId: u8) {
    if gTasks[taskId].data[0] != 0 {
        PlayNewMapMusic(gTasks[taskId].data[0] as u16);
    } else {
        PlayNewMapMusic(GetBattleBGM());
    }
    DestroyTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonFrontSpritePal(mon: *mut Pokemon) -> *mut u32 {
    let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
    let mut otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    let mut personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    return GetMonSpritePalFromSpeciesAndPersonality(species, otId, personality);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonSpritePalFromSpeciesAndPersonality(
    species: u16,
    otId: u32,
    personality: u32,
) -> *mut u32 {
    let mut shinyValue: u32 = 0;
    if species > NUM_SPECIES {
        return gMonPaletteTable[0].data;
    }
    shinyValue = (otId & 0xFFFF0000) >> 16
        ^ otId & 0xFFFF
        ^ (personality & 0xFFFF0000) >> 16
        ^ personality & 0xFFFF;
    if shinyValue < SHINY_ODDS {
        return gMonShinyPaletteTable[species].data;
    } else {
        return gMonPaletteTable[species].data;
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonSpritePalStruct(mon: *mut Pokemon) -> *mut CompressedSpritePalette {
    let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
    let mut otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    let mut personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    return GetMonSpritePalStructFromOtIdPersonality(species, otId, personality);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonSpritePalStructFromOtIdPersonality(
    species: u16,
    otId: u32,
    personality: u32,
) -> *mut CompressedSpritePalette {
    let mut shinyValue: u32 = 0;
    shinyValue = (otId & 0xFFFF0000) >> 16
        ^ otId & 0xFFFF
        ^ (personality & 0xFFFF0000) >> 16
        ^ personality & 0xFFFF;
    if shinyValue < SHINY_ODDS {
        return (&raw const gMonShinyPaletteTable[species]).cast_mut();
    } else {
        return (&raw const gMonPaletteTable[species]).cast_mut();
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsHMMove2(r#move: u16) -> u32 {
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
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMonSpriteNotFlipped(species: u16) -> u8 {
    return gSpeciesInfo[species].noFlip();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonFlavorRelation(mon: *mut Pokemon, flavor: u8) -> i8 {
    let mut nature: u8 = GetNature(mon);
    return gPokeblockFlavorCompatibilityTable[nature as i32 * FLAVOR_COUNT + flavor as i32];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFlavorRelationByPersonality(personality: u32, flavor: u8) -> i8 {
    let mut nature: u8 = GetNatureFromPersonality(personality);
    return gPokeblockFlavorCompatibilityTable[nature as i32 * FLAVOR_COUNT + flavor as i32];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsTradedMon(mon: *mut Pokemon) -> u8 {
    let mut otName: CArray<u8, 8> = zeroed();
    let mut otId: u32 = 0;
    GetMonData3(mon, MON_DATA_OT_NAME, otName.as_mut_ptr());
    otId = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    return IsOtherTrainer(otId, otName.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsOtherTrainer(otId: u32, otName: *mut u8) -> u8 {
    if otId
        == (*gSaveBlock2Ptr).playerTrainerId[0] as u32
            | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
            | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
            | ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24
    {
        let mut i: i32 = 0;
        i = 0;
        while *otName.at(i) != EOS {
            if *otName.at(i) != (*gSaveBlock2Ptr).playerName[i] {
                return TRUE;
            }
            i += 1;
        }
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonRestorePP(mon: *mut Pokemon) {
    BoxMonRestorePP(&raw mut (*mon).r#box);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BoxMonRestorePP(boxMon: *mut BoxPokemon) {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_MON_MOVES {
        if GetBoxMonData3(boxMon, MON_DATA_MOVE1 + i, null_mut()) != 0 {
            let mut r#move: u16 = GetBoxMonData3(boxMon, MON_DATA_MOVE1 + i, null_mut()) as u16;
            let mut bonus: u16 = GetBoxMonData3(boxMon, MON_DATA_PP_BONUSES, null_mut()) as u16;
            let mut pp: u8 = CalculatePPWithBonus(r#move, bonus as u8, i as u8);
            SetBoxMonData(boxMon, MON_DATA_PP1 + i, &raw mut pp as *mut c_void);
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMonPreventsSwitchingString() {
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
        gText_PkmnsXPreventsSwitching.as_ptr().cast_mut(),
        gStringVar4.as_mut_ptr(),
    );
}
pub(crate) unsafe extern "C" fn GetWildMonTableIdInAlteringCave(species: u16) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while i < 9 {
        if sAlteringCaveWildMonHeldItems[i].species == species {
            return i;
        }
        i += 1;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWildMonHeldItem() {
    if gBattleTypeFlags & 0x302008 == 0 {
        let mut rnd: u16 = (Random() as i32 % 100) as u16;
        let mut species: u16 =
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
            let mut alteringCaveId: i32 = GetWildMonTableIdInAlteringCave(species);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMonShiny(mon: *mut Pokemon) -> u8 {
    let mut otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    let mut personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    return IsShinyOtIdPersonality(otId, personality);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsShinyOtIdPersonality(otId: u32, personality: u32) -> u8 {
    let mut retVal: u8 = FALSE;
    let mut shinyValue: u32 = (otId & 0xFFFF0000) >> 16
        ^ otId & 0xFFFF
        ^ (personality & 0xFFFF0000) >> 16
        ^ personality & 0xFFFF;
    if shinyValue < SHINY_ODDS {
        retVal = TRUE;
    }
    return retVal;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerPartnerName() -> *mut u8 {
    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 {
        if gPartnerTrainerId == TRAINER_STEVEN_PARTNER {
            return gTrainers[804].trainerName.as_ptr().cast_mut();
        } else {
            GetFrontierTrainerName(gStringVar1.as_mut_ptr(), gPartnerTrainerId);
            return gStringVar1.as_mut_ptr();
        }
    } else {
        let mut id: u8 = GetMultiplayerId();
        return gLinkPlayers[GetBattlerMultiplayerId(gLinkPlayers[id].id ^ 2)]
            .name
            .as_mut_ptr();
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
pub(crate) unsafe extern "C" fn Task_AnimateAfterDelay(taskId: u8) {
    if ({
        gTasks[taskId].data[3] -= 1;
        gTasks[taskId].data[3]
    }) == 0
    {
        LaunchAnimationTaskForFrontSprite(
            (gTasks[taskId].data[0] as u16 as i32 | (gTasks[taskId].data[1] as u16 as i32) << 16)
                as usize as *mut c_void as *mut Sprite,
            gTasks[taskId].data[2] as u8,
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_PokemonSummaryAnimateAfterDelay(taskId: u8) {
    if ({
        gTasks[taskId].data[3] -= 1;
        gTasks[taskId].data[3]
    }) == 0
    {
        StartMonSummaryAnimation(
            (gTasks[taskId].data[0] as u16 as i32 | (gTasks[taskId].data[1] as u16 as i32) << 16)
                as usize as *mut c_void as *mut Sprite,
            gTasks[taskId].data[2] as u8,
        );
        SummaryScreen_SetAnimDelayTaskId(TASK_NONE);
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAnimateFrontSprite(
    sprite: *mut Sprite,
    species: u16,
    noCry: u8,
    panMode: u8,
) {
    if gHitMarker & HITMARKER_NO_ANIMATIONS != 0 && gBattleTypeFlags & 0x2000002 == 0 {
        DoMonFrontSpriteAnimation(sprite, species, noCry, panMode | SKIP_FRONT_ANIM);
    } else {
        DoMonFrontSpriteAnimation(sprite, species, noCry, panMode);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoMonFrontSpriteAnimation(
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
            let mut taskId: u8 = CreateTask(Some(Task_AnimateAfterDelay), 0);
            gTasks[taskId].data[0] = sprite as usize as u32 as i16;
            gTasks[taskId].data[1] = (sprite as usize as u32 >> 16) as i16;
            gTasks[taskId].data[2] = sMonFrontAnimIdsTable[species as i32 - 1] as i16;
            gTasks[taskId].data[3] = sMonAnimationDelayTable[species as i32 - 1] as i16;
        } else {
            LaunchAnimationTaskForFrontSprite(sprite, sMonFrontAnimIdsTable[species as i32 - 1]);
        }
        (*sprite).callback = Some(SpriteCallbackDummy_2);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokemonSummaryDoMonAnimation(
    sprite: *mut Sprite,
    species: u16,
    oneFrame: u8,
) {
    if oneFrame == 0 && HasTwoFramesAnimation(species) != 0 {
        StartSpriteAnim(sprite, 1);
    }
    if sMonAnimationDelayTable[species as i32 - 1] != 0 {
        let mut taskId: u8 = CreateTask(Some(Task_PokemonSummaryAnimateAfterDelay), 0);
        gTasks[taskId].data[0] = sprite as usize as u32 as i16;
        gTasks[taskId].data[1] = (sprite as usize as u32 >> 16) as i16;
        gTasks[taskId].data[2] = sMonFrontAnimIdsTable[species as i32 - 1] as i16;
        gTasks[taskId].data[3] = sMonAnimationDelayTable[species as i32 - 1] as i16;
        SummaryScreen_SetAnimDelayTaskId(taskId);
        SetSpriteCB_MonAnimDummy(sprite);
    } else {
        StartMonSummaryAnimation(sprite, sMonFrontAnimIdsTable[species as i32 - 1]);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopPokemonAnimationDelayTask() {
    let mut delayTaskId: u8 = FindTaskIdByFunc(Some(Task_PokemonSummaryAnimateAfterDelay));
    if delayTaskId != TASK_NONE {
        DestroyTask(delayTaskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleAnimateBackSprite(sprite: *mut Sprite, species: u16) {
    if gHitMarker & HITMARKER_NO_ANIMATIONS != 0 && gBattleTypeFlags & 0x2000002 == 0 {
        (*sprite).callback = Some(SpriteCallbackDummy);
    } else {
        LaunchAnimationTaskForBackSprite(sprite, GetSpeciesBackAnimSet(species));
        (*sprite).callback = Some(SpriteCallbackDummy_2);
    }
}
pub(crate) unsafe extern "C" fn GetOwnOpposingLinkMultiBattlerId(rightSide: u8) -> u8 {
    let mut i: i32 = 0;
    let mut battler: i32 = 0;
    let mut multiplayerId: u8 = GetMultiplayerId();
    match gLinkPlayers[multiplayerId].id {
        0 | 2 => {
            battler = if rightSide != 0 { 1 } else { 3 };
        }
        1 | 3 => {
            battler = if rightSide != 0 { 2 } else { 0 };
        }
        _ => {}
    }
    i = 0;
    while i < MAX_LINK_PLAYERS {
        if gLinkPlayers[i].id as i32 == battler as i16 as i32 {
            break;
        }
        i += 1;
    }
    return i as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetOpposingLinkMultiBattlerId(rightSide: u8, multiplayerId: u8) -> u8 {
    let mut i: i32 = 0;
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
    i = 0;
    while i < MAX_LINK_PLAYERS {
        if gLinkPlayers[i].id as i32 == battler as i16 as i32 {
            break;
        }
        i += 1;
    }
    return i as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FacilityClassToPicIndex(facilityClass: u16) -> u16 {
    return gFacilityClassToPicIndex[facilityClass] as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerGenderToFrontTrainerPicId(playerGender: u8) -> u16 {
    if playerGender != MALE {
        return FacilityClassToPicIndex(FACILITY_CLASS_MAY);
    } else {
        return FacilityClassToPicIndex(FACILITY_CLASS_BRENDAN);
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleSetPokedexFlag(nationalNum: u16, caseId: u8, personality: u32) {
    let mut getFlagCaseId: u8 = (if caseId == FLAG_SET_SEEN {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerClassNameFromId(mut trainerId: u16) -> *mut u8 {
    if trainerId >= TRAINERS_COUNT {
        trainerId = TRAINER_NONE;
    }
    return gTrainerClassNames[gTrainers[trainerId].trainerClass]
        .as_ptr()
        .cast_mut();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerNameFromId(mut trainerId: u16) -> *mut u8 {
    if trainerId >= TRAINERS_COUNT {
        trainerId = TRAINER_NONE;
    }
    return gTrainers[trainerId].trainerName.as_ptr().cast_mut();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasTwoFramesAnimation(species: u16) -> u8 {
    return (species != SPECIES_CASTFORM
        && species != SPECIES_DEOXYS as u16
        && species != SPECIES_SPINDA
        && species != SPECIES_UNOWN) as u8;
}
pub(crate) unsafe extern "C" fn ShouldSkipFriendshipChange() -> u8 {
    if gMain.inBattle() != 0 && gBattleTypeFlags & BATTLE_TYPE_FRONTIER != 0 {
        return TRUE;
    }
    if gMain.inBattle() == 0
        && (InBattlePike() != 0 || CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE)
    {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn InitMonSpritesGfx_Battle(gfx: *mut MonSpritesGfxManager) {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    i = 0;
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
pub(crate) unsafe extern "C" fn InitMonSpritesGfx_FullParty(gfx: *mut MonSpritesGfxManager) {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    i = 0;
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
        (*(*gfx).templates.at(i)).anims = gAnims_MonPic.as_ptr().cast_mut();
        (*(*gfx).templates.at(i)).paletteTag = i;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonSpritesGfxManager(
    mut managerId: u8,
    mode: u8,
) -> *mut MonSpritesGfxManager {
    let mut i: u8 = 0;
    let mut failureFlags: u8 = 0;
    let mut gfx: *mut MonSpritesGfxManager = null_mut();
    failureFlags = 0;
    managerId = (managerId as i32 % 2) as u8;
    gfx = AllocZeroed(20) as *mut MonSpritesGfxManager;
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
    return sMonSpritesGfxManagers[managerId];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyMonSpritesGfxManager(mut managerId: u8) {
    let mut gfx: *mut MonSpritesGfxManager = null_mut();
    managerId = (managerId as i32 % 2) as u8;
    gfx = sMonSpritesGfxManagers[managerId];
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonSpritesGfxManager_GetSpritePtr(
    managerId: u8,
    mut spriteNum: u8,
) -> *mut u8 {
    let mut gfx: *mut MonSpritesGfxManager = sMonSpritesGfxManagers[managerId as i32 % 2];
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
        return null_mut();
    }
}
